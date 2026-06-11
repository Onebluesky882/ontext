# Stage 15 — hold-hotkey streaming UI (step-by-step Wails pages)

Status: READY

Domain: `app/ontext-wails/frontend/src`, `app/ontext-wails/internal/hotkey`
(wiring/event-bridge only — do not modify Stage 13 hotkey core start/stop
logic; if a change there is needed, STOP and report in gate-out)
Branch: `feature/stage-15-hotkey-streaming-ui`

Goal:
While the hold-hotkey is active, stream partial transcript updates to the UI
live (like an `onChange` text input) using the existing Wails event bridge
(`runtime.EventsEmit` / `wailsjs/runtime` `EventsOn`, wired in Stage 11).
Restructure the frontend into step-by-step pages:
1. Permission request page (microphone + accessibility)
2. Hotkey status page (idle / recording / processing)
3. Live transcript page (real-time streamed text)

Assigned To: (unassigned)
Started At: -
Completed At: -

---

Checklist:
- [ ] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md (Stage 15), Stage 13 + Stage 11 gate-outs
- [ ] Confirm hold-hotkey Start/Stop (Stage 13) still works unmodified
- [ ] Wire partial transcript events to live-update the UI text (onChange-style)
- [ ] Build page 1: permission request (mic + accessibility), with fallback
      message if Accessibility is missing (per Stage 13)
- [ ] Build page 2: hotkey status (idle / recording / processing)
- [ ] Build page 3: live transcript view
- [ ] Wire page navigation (step-by-step flow on first run)
- [ ] `tsc` and `vite build` pass
- [ ] Create gate-outs/stage-15-hotkey-streaming-ui.md

---

Gate-Out: gate-outs/stage-15-hotkey-streaming-ui.md
Next Stage: none (independent UI stage)
