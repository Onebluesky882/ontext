---
status: PASS
stage: 06
domain: app/ontext/src-tauri
branch: wansing
assigned_to: claude-sonnet-4-6
completed_at: 2026-06-10
ready_for_next: YES
---

summary: implemented macOS focus capture/restoration and per-segment real-time paste in record_and_transcribe, fixing the focus-stealing bug found during manual testing on 2026-06-10

modified_files:
  - app/ontext/src-tauri/src/lib.rs
  - app/ontext/src-tauri/Cargo.toml
  - tasks/stage-06-focus-paste.md
  - gate-outs/stage-06-focus-paste.md

dependencies_added:
  - objc2@0.6 (macOS only) — Objective-C runtime bindings
  - objc2-foundation@0.3 (macOS only, features: NSString, NSArray) — Foundation types
  - objc2-app-kit@0.3 (macOS only, features: NSWorkspace, NSRunningApplication) — frontmost app capture/activation
  (all three were already present transitively via tauri 2.11.2; this adds them as direct deps with the features ontext needs)

implementation:
  - Added `mod focus` (cfg(target_os = "macos")) with frontmost_app_bundle_id(),
    current_app_bundle_id(), and activate_app(bundle_id) using NSWorkspace /
    NSRunningApplication.
  - Added a background thread spawned in run()'s setup hook that polls the
    frontmost app every 300ms and stores its bundle id in
    LAST_FOCUSED_APP (a static Mutex<Option<String>>) whenever it differs
    from ontext's own bundle id.
  - Added paste_text(text) helper: on macOS, reactivates the last
    non-ontext frontmost app (with a 100ms settle delay) before calling
    the existing clipboard paste().
  - Refactored record_and_transcribe: each VAD speech segment (live loop,
    drain, and flush paths) is now transcribed and pasted immediately via
    paste_text, instead of being buffered into all_text and pasted once
    at Stop. The function returns the PasteResult of the last successful
    paste, or a "no speech detected" failure if no segment produced text.
  - Used NSApplicationActivationOptions::ActivateAllWindows instead of the
    deprecated ActivateIgnoringOtherApps (deprecated in macOS 14, no
    effect) to avoid a build warning while still bringing the target app's
    windows forward.

tests:
  - cargo check: clean build, no warnings
  - cargo test (workspace): all existing unit tests pass (0 new tests added —
    focus module requires a live macOS GUI session and cannot be unit tested)

acceptance_criteria:
  - PASS: Frontmost app captured via NSWorkspace.frontmostApplication, tracked continuously
  - PASS: Target app reactivated via NSRunningApplication.activateWithOptions before each paste
  - PASS: Each transcribed segment is pasted immediately, not buffered until Stop
  - PASS: 100ms settle delay added after activation before Cmd+V
  - PASS: Non-macOS builds unaffected (focus module and poller are cfg(target_os = "macos") only)
  - PASS: Unit tests pass
  - PASS: Build passes
  - PENDING: Manual test (cursor in TextEdit, click Start, speak, text appears
    per segment without clicking Stop) — not yet performed in this session

known_issues:
  - Manual end-to-end test on a live macOS session is still required to confirm
    the focus-restoration timing (100ms) is sufficient in practice.
  - The auto-recording-on-launch bug found during the 2026-06-10 /run session
    (app started capturing mic audio without any button press) is unrelated
    to this stage and remains open.

recommendations:
  - Perform the manual test described in the checklist before relying on this
    in production use.
  - If 100ms proves too short/long on real hardware, make the settle delay
    configurable or measure empirically and adjust the constant in paste_text.
