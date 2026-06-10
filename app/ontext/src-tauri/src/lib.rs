use ontext_audio::AudioCapture;
use ontext_clipboard::{paste, PasteResult};
use ontext_transcribe::transcribe;
use ontext_vad::{AudioChunk, StreamingVad};
use std::sync::{Arc, OnceLock};
use tokio::sync::Notify;

#[cfg(target_os = "macos")]
mod ax_permission {
    use std::ffi::c_void;
    use std::os::raw::c_int;

    type CFTypeRef = *const c_void;
    type CFStringRef = *const c_void;
    type CFBooleanRef = *const c_void;
    type CFDictionaryRef = *const c_void;
    type CFIndex = c_int;

    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        static kCFBooleanTrue: CFBooleanRef;
        static kCFTypeDictionaryKeyCallBacks: *const c_void;
        static kCFTypeDictionaryValueCallBacks: *const c_void;
        fn CFDictionaryCreate(
            allocator: *const c_void,
            keys: *mut CFTypeRef,
            values: *mut CFTypeRef,
            num_values: CFIndex,
            key_callbacks: *const c_void,
            value_callbacks: *const c_void,
        ) -> CFDictionaryRef;
        fn CFRelease(cf: CFTypeRef);
    }

    #[link(name = "ApplicationServices", kind = "framework")]
    extern "C" {
        static kAXTrustedCheckOptionPrompt: CFStringRef;
        fn AXIsProcessTrustedWithOptions(options: CFDictionaryRef) -> bool;
    }

    pub fn request_with_prompt() -> bool {
        unsafe {
            let mut keys: [CFTypeRef; 1] = [kAXTrustedCheckOptionPrompt];
            let mut values: [CFTypeRef; 1] = [kCFBooleanTrue];
            let dict = CFDictionaryCreate(
                std::ptr::null(),
                keys.as_mut_ptr(),
                values.as_mut_ptr(),
                1,
                kCFTypeDictionaryKeyCallBacks,
                kCFTypeDictionaryValueCallBacks,
            );
            let trusted = AXIsProcessTrustedWithOptions(dict);
            CFRelease(dict as CFTypeRef);
            trusted
        }
    }
}

// Notified by `stop_recording` to signal an active pipeline to stop.
// Uses notify_one so the permit persists even if nobody is waiting yet.
static STOP_NOTIFY: OnceLock<Arc<Notify>> = OnceLock::new();

fn get_stop_notify() -> Arc<Notify> {
    STOP_NOTIFY.get_or_init(|| Arc::new(Notify::new())).clone()
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn request_accessibility_permission() -> bool {
    #[cfg(target_os = "macos")]
    {
        ax_permission::request_with_prompt()
    }
    #[cfg(not(target_os = "macos"))]
    {
        true
    }
}

/// Streaming pipeline: mic chunks → RMS-VAD → transcribe each speech segment → paste all.
async fn record_and_transcribe(api_key: String) -> PasteResult {
    let stop_notify = get_stop_notify();
    let (stop_tx, stop_rx) = std::sync::mpsc::channel::<()>();
    let (chunk_tx, chunk_rx) = std::sync::mpsc::channel::<Vec<f32>>();

    eprintln!("[ontext] recording started");

    // Audio capture runs on a dedicated blocking thread (cpal::Stream is !Send).
    let audio_join = tokio::task::spawn_blocking(move || -> Result<(), String> {
        let mut capture = AudioCapture::new();
        let tx = chunk_tx;
        capture
            .start_with_callback(move |samples| {
                let _ = tx.send(samples);
            })
            .map_err(|e| e.to_string())?;
        eprintln!("[ontext] audio stream running");
        let _ = stop_rx.recv(); // blocks until stop_recording is called
        eprintln!("[ontext] audio stream stopping");
        Ok(()) // dropping capture here stops the cpal stream
    });

    let mut vad = StreamingVad::new();
    let mut all_text: Vec<String> = Vec::new();
    let mut log_tick = 0u32;

    'running: loop {
        // Drain all buffered audio chunks before sleeping
        loop {
            match chunk_rx.try_recv() {
                Ok(samples) => {
                    log_tick += 1;
                    if log_tick % 100 == 0 {
                        let rms = rms_of(&samples);
                        eprintln!(
                            "[ontext] RMS={:.4}  speaking={}",
                            rms,
                            vad.is_speaking()
                        );
                    }

                    let (final_chunk, _partial) = vad.process(&samples);
                    if let Some(speech) = final_chunk {
                        let ms = (speech.len() as f32 / 16000.0 * 1000.0) as u32;
                        eprintln!("[ontext] VAD: speech segment {ms}ms → transcribing...");
                        match transcribe_samples(&speech, &api_key).await {
                            Ok(t) if !t.is_empty() => {
                                eprintln!("[ontext] transcript: {:?}", t);
                                all_text.push(t);
                            }
                            Ok(_) => eprintln!("[ontext] transcript: empty"),
                            Err(e) => eprintln!("[ontext] transcription error: {e}"),
                        }
                    }
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => break,
                Err(std::sync::mpsc::TryRecvError::Disconnected) => break 'running,
            }
        }

        tokio::select! {
            _ = stop_notify.notified() => {
                let _ = stop_tx.send(());
                break 'running;
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(20)) => {}
        }
    }

    // Wait for the blocking thread to finish (ensures chunk_tx is dropped).
    let _ = audio_join.await;
    eprintln!("[ontext] audio thread done — draining queue");

    // Drain any chunks that arrived before the stream shut down.
    while let Ok(samples) = chunk_rx.try_recv() {
        let (final_chunk, _) = vad.process(&samples);
        if let Some(speech) = final_chunk {
            let ms = (speech.len() as f32 / 16000.0 * 1000.0) as u32;
            eprintln!("[ontext] VAD (drain): {ms}ms → transcribing...");
            if let Ok(t) = transcribe_samples(&speech, &api_key).await {
                if !t.is_empty() {
                    eprintln!("[ontext] transcript (drain): {:?}", t);
                    all_text.push(t);
                }
            }
        }
    }

    // Flush any in-progress speech that ended when recording stopped.
    if let Some(speech) = vad.flush() {
        let ms = (speech.len() as f32 / 16000.0 * 1000.0) as u32;
        eprintln!("[ontext] VAD (flush): {ms}ms → transcribing...");
        if let Ok(t) = transcribe_samples(&speech, &api_key).await {
            if !t.is_empty() {
                eprintln!("[ontext] transcript (flush): {:?}", t);
                all_text.push(t);
            }
        }
    }

    eprintln!("[ontext] all segments: {:?}", all_text);

    if all_text.is_empty() {
        return PasteResult {
            success: false,
            error: Some("no speech detected in recording".to_string()),
        };
    }

    paste(ontext_clipboard::TranscriptResult {
        text: all_text.join(" "),
        language: String::new(),
    })
}

async fn transcribe_samples(samples: &[f32], api_key: &str) -> Result<String, String> {
    let end_ms = (samples.len() as f64 / 16000.0 * 1000.0) as u64;
    let chunk = AudioChunk {
        samples: samples.to_vec(),
        start_ms: 0,
        end_ms,
    };
    transcribe(vec![chunk], api_key)
        .await
        .map(|r| r.text)
        .map_err(|e| e.to_string())
}

fn rms_of(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    let sq: f32 = samples.iter().map(|s| s * s).sum();
    (sq / samples.len() as f32).sqrt()
}

/// Start recording immediately. Call `stop_recording` to stop.
#[tauri::command]
async fn start_pipeline() -> PasteResult {
    let api_key = match std::env::var("VITE_GROQ") {
        Ok(k) => k,
        Err(_) => {
            return PasteResult {
                success: false,
                error: Some("VITE_GROQ not set in environment".to_string()),
            }
        }
    };
    record_and_transcribe(api_key).await
}

/// Signal the active recording to stop and trigger transcription.
#[tauri::command]
fn stop_recording() {
    eprintln!("[ontext] stop_recording called");
    get_stop_notify().notify_one() // notify_one stores a permit; notify_waiters does not
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = dotenvy::dotenv();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            use tauri::{
                image::Image,
                menu::{MenuBuilder, MenuItemBuilder},
                tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
                Manager,
            };

            let show_item = MenuItemBuilder::new("Show / Hide").id("toggle").build(app)?;
            let quit_item = MenuItemBuilder::new("Quit").id("quit").build(app)?;
            let menu = MenuBuilder::new(app)
                .items(&[&show_item, &quit_item])
                .build()?;

            let icon = Image::from_path(
                app.path()
                    .resource_dir()
                    .unwrap_or_default()
                    .join("icons/icon.png"),
            )
            .unwrap_or_else(|_| app.default_window_icon().unwrap().clone());

            TrayIconBuilder::new()
                .icon(icon)
                .menu(&menu)
                .tooltip("ontext")
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "toggle" => toggle_window(app),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        show_window(tray.app_handle());
                    }
                })
                .build(app)?;

            if let Some(window) = app.get_webview_window("main") {
                let w = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = w.hide();
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            start_pipeline,
            stop_recording,
            request_accessibility_permission
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn toggle_window(app: &tauri::AppHandle) {
    use tauri::Manager;
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

fn show_window(app: &tauri::AppHandle) {
    use tauri::Manager;
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}
