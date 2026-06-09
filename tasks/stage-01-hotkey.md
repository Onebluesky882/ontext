# Stage 01 — hotkey

Status: DONE

Domain: modules/hotkey
Branch: feature/hotkey

Goal:
Detect global hotkey press and release. Emit HotkeyEvent::Start on press, HotkeyEvent::Stop on release.

Assigned To: claude-sonnet-4-6
Started At: 2026-06-09
Completed At: 2026-06-09

---

Checklist:
- [x] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md
- [x] Implement modules/hotkey
- [x] Hotkey press emits HotkeyEvent::Start
- [x] Hotkey release emits HotkeyEvent::Stop
- [x] Works while another app is in focus
- [x] Unit tests pass
- [x] Build passes
- [x] Create gate-outs/stage-01-hotkey.md

---

Gate-Out: gate-outs/stage-01-hotkey.md
Next Stage: stage-02-audio
