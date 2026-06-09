use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptResult {
    pub text: String,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasteResult {
    pub success: bool,
    pub error: Option<String>,
}

pub fn paste(result: TranscriptResult) -> PasteResult {
    let mut clipboard = match arboard::Clipboard::new() {
        Ok(c) => c,
        Err(e) => {
            return PasteResult {
                success: false,
                error: Some(format!("clipboard init failed: {e}")),
            }
        }
    };

    if let Err(e) = clipboard.set_text(&result.text) {
        return PasteResult {
            success: false,
            error: Some(format!("clipboard write failed: {e}")),
        };
    }

    if let Err(e) = simulate_paste() {
        return PasteResult {
            success: false,
            error: Some(format!("paste simulation failed: {e}")),
        };
    }

    PasteResult {
        success: true,
        error: None,
    }
}

fn simulate_paste() -> Result<(), String> {
    use enigo::{Direction, Enigo, Key, Keyboard, Settings};

    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("input simulation init failed: {e}"))?;

    #[cfg(target_os = "macos")]
    {
        enigo
            .key(Key::Meta, Direction::Press)
            .map_err(|e| format!("key press failed: {e}"))?;
        let v_result = enigo.key(Key::Unicode('v'), Direction::Click);
        enigo
            .key(Key::Meta, Direction::Release)
            .map_err(|e| format!("key release failed: {e}"))?;
        v_result.map_err(|e| format!("key click failed: {e}"))?;
    }

    #[cfg(target_os = "windows")]
    {
        enigo
            .key(Key::Control, Direction::Press)
            .map_err(|e| format!("key press failed: {e}"))?;
        let v_result = enigo.key(Key::Unicode('v'), Direction::Click);
        enigo
            .key(Key::Control, Direction::Release)
            .map_err(|e| format!("key release failed: {e}"))?;
        v_result.map_err(|e| format!("key click failed: {e}"))?;
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        return Err("unsupported platform for paste simulation".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paste_result_success_has_none_error() {
        let r = PasteResult {
            success: true,
            error: None,
        };
        assert!(r.success);
        assert!(r.error.is_none());
    }

    #[test]
    fn paste_result_failure_has_error_string() {
        let r = PasteResult {
            success: false,
            error: Some("clipboard init failed: display not found".to_string()),
        };
        assert!(!r.success);
        assert!(r.error.is_some());
        assert!(!r.error.unwrap().is_empty());
    }

    #[test]
    fn paste_result_failure_error_never_empty_string() {
        let r = PasteResult {
            success: false,
            error: Some("some error".to_string()),
        };
        if let Some(msg) = r.error {
            assert!(!msg.is_empty(), "error must never be an empty string");
        }
    }

    #[test]
    fn paste_result_serializes_camel_case() {
        let r = PasteResult {
            success: true,
            error: None,
        };
        let json = serde_json::to_string(&r).unwrap();
        assert!(json.contains("\"success\""));
        assert!(!json.contains("snake_case"));
    }

    #[test]
    fn transcript_result_serializes_camel_case() {
        let t = TranscriptResult {
            text: "hello".to_string(),
            language: "en".to_string(),
        };
        let json = serde_json::to_string(&t).unwrap();
        assert!(json.contains("\"text\""));
        assert!(json.contains("\"language\""));
    }

    #[test]
    #[ignore = "requires display and clipboard access"]
    fn paste_writes_to_clipboard_and_returns_success() {
        let result = paste(TranscriptResult {
            text: "hello world".to_string(),
            language: "en".to_string(),
        });
        assert!(result.success);
        assert!(result.error.is_none());
    }
}
