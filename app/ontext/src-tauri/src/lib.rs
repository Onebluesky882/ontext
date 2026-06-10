use ontext_audio::AudioCapture;
use ontext_clipboard::{paste, PasteResult};
use ontext_hotkey::{HotkeyEvent, HotkeyListener};
use ontext_transcribe::transcribe;
use ontext_vad::process as vad_process;
use std::sync::OnceLock;
use tokio::sync::broadcast;

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
            Ok(HotkeyEvent::Stop) => continue,
            Err(broadcast::error::RecvError::Lagged(_)) => continue,
            Err(broadcast::error::RecvError::Closed) => {
                return PasteResult {
                    success: false,
                    error: Some("hotkey channel closed unexpectedly".to_string()),
                }
            }
        }
    }

    // AudioCapture holds cpal::Stream (!Send), so keep it entirely inside
    // spawn_blocking — it never crosses an .await boundary this way.
    let (stop_tx, stop_rx) = std::sync::mpsc::channel::<()>();

    let audio_task = tokio::task::spawn_blocking(move || {
        let mut audio = AudioCapture::new();
        if let Err(e) = audio.handle(HotkeyEvent::Start) {
            return Err(format!("audio capture failed to start: {e}"));
        }
        let _ = stop_rx.recv(); // block until hotkey release
        match audio.handle(HotkeyEvent::Stop) {
            Ok(Some(b)) => Ok(b),
            Ok(None) => Err("audio capture returned no buffer".to_string()),
            Err(e) => Err(format!("audio capture failed to stop: {e}")),
        }
    });

    // Wait for hotkey release
    loop {
        match rx.recv().await {
            Ok(HotkeyEvent::Stop) => break,
            Ok(HotkeyEvent::Start) => continue,
            Err(broadcast::error::RecvError::Lagged(_)) => continue,
            Err(broadcast::error::RecvError::Closed) => {
                return PasteResult {
                    success: false,
                    error: Some("hotkey channel closed unexpectedly".to_string()),
                }
            }
        }
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

    // ontext_transcribe::TranscriptResult and ontext_clipboard::TranscriptResult are separate types
    paste(ontext_clipboard::TranscriptResult {
        text: transcript.text,
        language: transcript.language,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = dotenvy::dotenv(); // load .env if present, ignore if missing
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, run_pipeline, request_accessibility_permission])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
