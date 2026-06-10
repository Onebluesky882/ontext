# Stage 06 — focus-paste

Status: DONE

Domain: app/ontext/src-tauri
Branch: wansing

Goal:
Make transcribed text appear at the user's cursor in whatever app was
focused when recording started — in real time per segment, not only
after Stop is pressed.

Background:
Manual testing (2026-06-10) found two issues with the current
button-driven pipeline (record_and_transcribe in
app/ontext/src-tauri/src/lib.rs):

1. Clicking Start/Stop in the ontext window gives ontext window focus.
   When paste() simulates Cmd+V at the end, it pastes into ontext's own
   window instead of the app the user had focused (e.g. Notes,
   TextEdit) — text only ends up in the clipboard, not at the cursor.
2. All segment text is buffered in `all_text` and pasted as one block
   only after Stop is clicked — not real-time.

Assigned To: claude-sonnet-4-6 (orchestrator — protected files)
Started At: 2026-06-10
Completed At: 2026-06-10

---

Checklist:
- [x] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md
- [x] Capture the frontmost app (NSWorkspace.frontmostApplication,
      bundle identifier) when start_pipeline begins recording
      (objc2 / objc2-app-kit are already in the dependency tree via
      tauri — no new heavyweight deps needed)
- [x] Before each paste, reactivate that app
      (NSRunningApplication.activateWithOptions) so Cmd+V lands in the
      correct window
- [x] Paste each transcribed segment as soon as it's ready (streaming
      loop in record_and_transcribe), instead of buffering into
      all_text and pasting once at Stop
- [x] Add a short delay after activation before sending Cmd+V if needed
      for focus to settle (measure, don't guess)
- [x] Windows/Linux: keep existing behavior (no frontmost-app capture
      available via this API) — guard with #[cfg(target_os = "macos")]
- [ ] Manual test: cursor in TextEdit, click Start in ontext, speak,
      text appears in TextEdit per segment without clicking Stop first
- [x] Unit tests pass
- [x] Build passes
- [x] Create gate-outs/stage-06-focus-paste.md

---

Gate-Out: gate-outs/stage-06-focus-paste.md
Next Stage: — (pipeline complete)
