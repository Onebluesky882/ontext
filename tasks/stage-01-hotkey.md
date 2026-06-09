# Stage 01 — hotkey

Status: PENDING

Domain: modules/hotkey
Branch: feature/hotkey

Goal:
Detect global hotkey press and release. Emit HotkeyEvent::Start on press, HotkeyEvent::Stop on release.

Assigned To: —
Started At: —
Completed At: —

---

Checklist:
- [ ] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md
- [ ] Implement modules/hotkey
- [ ] Hotkey press emits HotkeyEvent::Start
- [ ] Hotkey release emits HotkeyEvent::Stop
- [ ] Works while another app is in focus
- [ ] Unit tests pass
- [ ] Build passes
- [ ] Create gate-outs/stage-01-hotkey.md

---

Gate-Out: gate-outs/stage-01-hotkey.md
Next Stage: stage-02-audio
