use ontext_audio::AudioCapture;
use ontext_clipboard::{paste, PasteResult};
use ontext_hotkey::{HotkeyEvent, HotkeyListener};
use ontext_transcribe::transcribe;
use ontext_vad::process as vad_process;
use tokio::sync::mpsc;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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

    let (tx, mut rx) = mpsc::channel::<HotkeyEvent>(8);

    let _listener = match HotkeyListener::start(move |event| {
        let _ = tx.blocking_send(event);
    }) {
        Ok(l) => l,
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
            Some(HotkeyEvent::Start) => break,
            Some(HotkeyEvent::Stop) => continue,
            None => {
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
            Some(HotkeyEvent::Stop) => break,
            Some(HotkeyEvent::Start) => continue,
            None => {
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
        .invoke_handler(tauri::generate_handler![greet, run_pipeline])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
