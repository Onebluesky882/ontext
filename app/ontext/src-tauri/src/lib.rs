use ontext_audio::AudioCapture;
use ontext_clipboard::{paste, PasteResult};
use ontext_hotkey::{HotkeyEvent, HotkeyListener};
use ontext_transcribe::transcribe;
use ontext_vad::process as vad_process;
use std::sync::{Arc, OnceLock};
use tokio::sync::{broadcast, Notify};

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

// One rdev::listen thread for the lifetime of the process.
// rdev has no stop API, so starting a new thread per pipeline run would
// accumulate zombie threads. Instead every run subscribes to this broadcaster.
static HOTKEY_TX: OnceLock<broadcast::Sender<HotkeyEvent>> = OnceLock::new();

// Notified by `stop_recording` to signal an active pipeline to stop.
static STOP_NOTIFY: OnceLock<Arc<Notify>> = OnceLock::new();

fn get_stop_notify() -> Arc<Notify> {
    STOP_NOTIFY.get_or_init(|| Arc::new(Notify::new())).clone()
}

fn subscribe_hotkeys() -> Result<broadcast::Receiver<HotkeyEvent>, String> {
    // get_or_init cannot return an error, so we initialise with a sentinel and
    // detect failure after the fact.
    static HOTKEY_INIT_FAILED: std::sync::atomic::AtomicBool =
        std::sync::atomic::AtomicBool::new(false);

    let tx = HOTKEY_TX.get_or_init(|| {
        let (tx, _) = broadcast::channel::<HotkeyEvent>(16);
        let tx2 = tx.clone();
        match HotkeyListener::start(move |event| {
            let _ = tx2.send(event);
        }) {
            Ok(_listener) => {
                // listener runs for the process lifetime; intentionally leaked
            }
            Err(e) => {
                eprintln!("ontext: failed to start hotkey listener: {e}");
                HOTKEY_INIT_FAILED.store(true, std::sync::atomic::Ordering::SeqCst);
            }
        }
        tx
    });

    if HOTKEY_INIT_FAILED.load(std::sync::atomic::Ordering::SeqCst) {
        return Err(
            "global hotkey listener failed to start — check Accessibility permission".to_string(),
        );
    }

    Ok(tx.subscribe())
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

/// Record audio until a stop signal arrives (hotkey release OR `stop_recording` command),
/// then transcribe and paste. Audio capture must start on a blocking thread because
/// `cpal::Stream` is `!Send` and must not cross `.await` points.
async fn record_and_transcribe(api_key: String) -> PasteResult {
    // Optional hotkey receiver — if the listener failed we continue UI-only.
    let hotkey_rx = subscribe_hotkeys().ok();
    let stop_notify = get_stop_notify();

    let (stop_tx, stop_rx) = std::sync::mpsc::channel::<()>();

    // AudioCapture holds cpal::Stream (!Send); keep entirely inside spawn_blocking.
    let audio_task = tokio::task::spawn_blocking(move || {
        let mut audio = AudioCapture::new();
        if let Err(e) = audio.handle(HotkeyEvent::Start) {
            return Err(format!("audio capture failed to start: {e}"));
        }
        let _ = stop_rx.recv();
        match audio.handle(HotkeyEvent::Stop) {
            Ok(Some(b)) => Ok(b),
            Ok(None) => Err("audio capture returned no buffer".to_string()),
            Err(e) => Err(format!("audio capture failed to stop: {e}")),
        }
    });

    // Wait for stop signal from hotkey release OR the stop_recording command.
    let hotkey_stop = async {
        if let Some(mut rx) = hotkey_rx {
            loop {
                match rx.recv().await {
                    Ok(HotkeyEvent::Stop) => break,
                    Ok(HotkeyEvent::Start) | Err(broadcast::error::RecvError::Lagged(_)) => {
                        continue
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        } else {
            std::future::pending::<()>().await
        }
    };

    tokio::select! {
        _ = hotkey_stop => {}
        _ = stop_notify.notified() => {}
    }

    let _ = stop_tx.send(());

    let buffer = match audio_task.await {
        Ok(Ok(b)) => b,
        Ok(Err(e)) => return PasteResult { success: false, error: Some(e) },
        Err(e) => {
            return PasteResult {
                success: false,
                error: Some(format!("audio task panicked: {e}")),
            }
        }
    };

    let chunks = vad_process(&buffer);
    if chunks.is_empty() {
        return PasteResult {
            success: false,
            error: Some("no speech detected in recording".to_string()),
        };
    }

    let transcript = match transcribe(chunks, &api_key).await {
        Ok(t) => t,
        Err(e) => {
            return PasteResult {
                success: false,
                error: Some(format!("transcription failed: {e}")),
            }
        }
    };

    paste(ontext_clipboard::TranscriptResult {
        text: transcript.text,
        language: transcript.language,
    })
}

/// Hotkey-driven pipeline: arms the listener and waits for Ctrl+Space before recording.
#[tauri::command]
async fn run_pipeline() -> PasteResult {
    let api_key = match std::env::var("VITE_GROQ") {
        Ok(k) => k,
        Err(_) => {
            return PasteResult {
                success: false,
                error: Some("VITE_GROQ not set in environment".to_string()),
            }
        }
    };

    let mut rx = match subscribe_hotkeys() {
        Ok(r) => r,
        Err(e) => {
            return PasteResult {
                success: false,
                error: Some(format!("hotkey listener failed: {e}")),
            }
        }
    };

    // Wait for hotkey press
    loop {
        match rx.recv().await {
            Ok(HotkeyEvent::Start) => break,
            Ok(HotkeyEvent::Stop) | Err(broadcast::error::RecvError::Lagged(_)) => continue,
            Err(broadcast::error::RecvError::Closed) => {
                return PasteResult {
                    success: false,
                    error: Some("hotkey channel closed unexpectedly".to_string()),
                }
            }
        }
    }

    record_and_transcribe(api_key).await
}

/// Button-driven pipeline: starts recording immediately without waiting for hotkey.
/// Call `stop_recording` to end the recording.
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

/// Signal an active `start_pipeline` or `run_pipeline` recording to stop.
#[tauri::command]
fn stop_recording() {
    get_stop_notify().notify_waiters();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = dotenvy::dotenv(); // load .env if present, ignore if missing
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
                        toggle_window(tray.app_handle());
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            run_pipeline,
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
