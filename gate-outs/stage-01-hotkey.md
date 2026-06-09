---
status: PASS
stage: 01
domain: modules/hotkey
branch: feature/hotkey
assigned_to: claude-sonnet-4-6
completed_at: 2026-06-09
ready_for_next: YES
---

summary: implemented global hotkey detection using rdev; emits HotkeyEvent::Start on combo press and HotkeyEvent::Stop on release

modified_files:
  - modules/hotkey/src/lib.rs
  - modules/hotkey/Cargo.toml

dependencies_added:
  - rdev@0.5.3 — cross-platform global key listener (see docs/adrs/004-use-rdev.md)
  - thiserror@1.0.69 — structured error type for HotkeyError

tests:
  - test_hotkey_start_emits_event
  - test_hotkey_stop_emits_event
  - test_no_event_without_full_combo
  - test_start_only_emitted_once_while_held
  - test_stop_only_emitted_once_after_release
  - test_right_modifier_keys_work

acceptance_criteria:
  - PASS: Hotkey press emits HotkeyEvent::Start
  - PASS: Hotkey release emits HotkeyEvent::Stop
  - PASS: Works while another app is in focus (rdev uses CGEventTap on macOS / SetWindowsHookEx on Windows)

known_issues:
  - rdev transitive dep block@0.1.6 has a future-incompatibility warning in Rust; not actionable at this stage
  - macOS requires Accessibility / Input Monitoring permission at runtime; must be requested by Tauri layer

recommendations:
  - lib.rs needs to register hotkey module (orchestrator action)
  - Tauri layer must request Accessibility permission on macOS before calling HotkeyListener::start
