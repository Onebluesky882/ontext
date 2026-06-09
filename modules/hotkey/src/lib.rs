use rdev::{listen, EventType, Key};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HotkeyEvent {
    Start,
    Stop,
}

#[derive(Debug, Error)]
pub enum HotkeyError {
    #[error("failed to start global key listener: {0}")]
    ListenError(String),
}

/// Tracks key state and detects the hotkey combo.
/// macOS:   Cmd  + Shift + Space
/// Windows: Ctrl + Shift + Space
pub struct HotkeyDetector {
    pressed: HashSet<String>,
    combo_active: bool,
}

impl HotkeyDetector {
    pub fn new() -> Self {
        Self {
            pressed: HashSet::new(),
            combo_active: false,
        }
    }

    pub fn key_down(&mut self, key: &Key) -> Option<HotkeyEvent> {
        self.pressed.insert(key_name(key));
        if !self.combo_active && self.combo_held() {
            self.combo_active = true;
            Some(HotkeyEvent::Start)
        } else {
            None
        }
    }

    pub fn key_up(&mut self, key: &Key) -> Option<HotkeyEvent> {
        let was_active = self.combo_active;
        self.pressed.remove(&key_name(key));
        if was_active && !self.combo_held() {
            self.combo_active = false;
            Some(HotkeyEvent::Stop)
        } else {
            None
        }
    }

    #[cfg(target_os = "macos")]
    fn combo_held(&self) -> bool {
        let meta = self.pressed.contains("MetaLeft") || self.pressed.contains("MetaRight");
        let shift = self.pressed.contains("ShiftLeft") || self.pressed.contains("ShiftRight");
        let space = self.pressed.contains("Space");
        meta && shift && space
    }

    #[cfg(not(target_os = "macos"))]
    fn combo_held(&self) -> bool {
        let ctrl = self.pressed.contains("ControlLeft") || self.pressed.contains("ControlRight");
        let shift = self.pressed.contains("ShiftLeft") || self.pressed.contains("ShiftRight");
        let space = self.pressed.contains("Space");
        ctrl && shift && space
    }
}

impl Default for HotkeyDetector {
    fn default() -> Self {
        Self::new()
    }
}

fn key_name(key: &Key) -> String {
    format!("{:?}", key)
}

/// Listens for the global hotkey in a background thread.
/// Calls `callback` with `HotkeyEvent::Start` on press and `HotkeyEvent::Stop` on release.
/// The listener runs until the process exits (rdev does not support graceful stop).
pub struct HotkeyListener {
    _handle: thread::JoinHandle<()>,
}

impl HotkeyListener {
    pub fn start<F>(callback: F) -> Result<Self, HotkeyError>
    where
        F: Fn(HotkeyEvent) + Send + 'static,
    {
        let detector = Arc::new(Mutex::new(HotkeyDetector::new()));

        let handle = thread::spawn(move || {
            let detector = detector.clone();
            let result = listen(move |event| {
                let mut det = detector.lock().expect("detector mutex poisoned");
                let hotkey_event = match event.event_type {
                    EventType::KeyPress(key) => det.key_down(&key),
                    EventType::KeyRelease(key) => det.key_up(&key),
                    _ => None,
                };
                if let Some(ev) = hotkey_event {
                    callback(ev);
                }
            });

            if let Err(e) = result {
                eprintln!("ontext-hotkey: rdev listen error: {:?}", e);
            }
        });

        Ok(Self { _handle: handle })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "macos")]
    mod platform_tests {
        use super::*;

        #[test]
        fn test_hotkey_start_emits_event() {
            let mut det = HotkeyDetector::new();
            assert_eq!(det.key_down(&Key::MetaLeft), None);
            assert_eq!(det.key_down(&Key::ShiftLeft), None);
            assert_eq!(det.key_down(&Key::Space), Some(HotkeyEvent::Start));
        }

        #[test]
        fn test_hotkey_stop_emits_event() {
            let mut det = HotkeyDetector::new();
            det.key_down(&Key::MetaLeft);
            det.key_down(&Key::ShiftLeft);
            det.key_down(&Key::Space);
            assert_eq!(det.key_up(&Key::Space), Some(HotkeyEvent::Stop));
        }

        #[test]
        fn test_no_event_without_full_combo() {
            let mut det = HotkeyDetector::new();
            assert_eq!(det.key_down(&Key::ShiftLeft), None);
            assert_eq!(det.key_down(&Key::Space), None);
        }

        #[test]
        fn test_start_only_emitted_once_while_held() {
            let mut det = HotkeyDetector::new();
            det.key_down(&Key::MetaLeft);
            det.key_down(&Key::ShiftLeft);
            let first = det.key_down(&Key::Space);
            let second = det.key_down(&Key::Space);
            assert_eq!(first, Some(HotkeyEvent::Start));
            assert_eq!(second, None);
        }

        #[test]
        fn test_stop_only_emitted_once_after_release() {
            let mut det = HotkeyDetector::new();
            det.key_down(&Key::MetaLeft);
            det.key_down(&Key::ShiftLeft);
            det.key_down(&Key::Space);
            let first = det.key_up(&Key::Space);
            let second = det.key_up(&Key::Space);
            assert_eq!(first, Some(HotkeyEvent::Stop));
            assert_eq!(second, None);
        }

        #[test]
        fn test_right_modifier_keys_work() {
            let mut det = HotkeyDetector::new();
            det.key_down(&Key::MetaRight);
            det.key_down(&Key::ShiftRight);
            assert_eq!(det.key_down(&Key::Space), Some(HotkeyEvent::Start));
        }
    }

    #[cfg(not(target_os = "macos"))]
    mod platform_tests {
        use super::*;

        #[test]
        fn test_hotkey_start_emits_event() {
            let mut det = HotkeyDetector::new();
            assert_eq!(det.key_down(&Key::ControlLeft), None);
            assert_eq!(det.key_down(&Key::ShiftLeft), None);
            assert_eq!(det.key_down(&Key::Space), Some(HotkeyEvent::Start));
        }

        #[test]
        fn test_hotkey_stop_emits_event() {
            let mut det = HotkeyDetector::new();
            det.key_down(&Key::ControlLeft);
            det.key_down(&Key::ShiftLeft);
            det.key_down(&Key::Space);
            assert_eq!(det.key_up(&Key::Space), Some(HotkeyEvent::Stop));
        }

        #[test]
        fn test_no_event_without_full_combo() {
            let mut det = HotkeyDetector::new();
            assert_eq!(det.key_down(&Key::ShiftLeft), None);
            assert_eq!(det.key_down(&Key::Space), None);
        }

        #[test]
        fn test_start_only_emitted_once_while_held() {
            let mut det = HotkeyDetector::new();
            det.key_down(&Key::ControlLeft);
            det.key_down(&Key::ShiftLeft);
            let first = det.key_down(&Key::Space);
            let second = det.key_down(&Key::Space);
            assert_eq!(first, Some(HotkeyEvent::Start));
            assert_eq!(second, None);
        }

        #[test]
        fn test_stop_only_emitted_once_after_release() {
            let mut det = HotkeyDetector::new();
            det.key_down(&Key::ControlLeft);
            det.key_down(&Key::ShiftLeft);
            det.key_down(&Key::Space);
            let first = det.key_up(&Key::Space);
            let second = det.key_up(&Key::Space);
            assert_eq!(first, Some(HotkeyEvent::Stop));
            assert_eq!(second, None);
        }

        #[test]
        fn test_right_modifier_keys_work() {
            let mut det = HotkeyDetector::new();
            det.key_down(&Key::ControlRight);
            det.key_down(&Key::ShiftRight);
            assert_eq!(det.key_down(&Key::Space), Some(HotkeyEvent::Start));
        }
    }
}
