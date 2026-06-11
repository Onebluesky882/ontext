---
status: PASS
stage: 13
domain: app/ontext-wails/internal/hotkey
branch: feature/stage-13-hotkey-go
assigned_to: claude-sonnet-4-6
completed_at: 2026-06-11
ready_for_next: YES
---

summary: implemented global hold-to-talk hotkey using golang.design/x/hotkey, emitting Start/Stop into pipeline.Pipeline and startedAt/endedAt/durationMs session timestamps for ADR 010 usage reporting, with graceful fallback to button-only operation on registration failure

modified_files:
  - app/ontext-wails/internal/hotkey/hotkey.go
  - app/ontext-wails/internal/hotkey/xhotkey.go
  - app/ontext-wails/internal/hotkey/controller.go
  - app/ontext-wails/internal/hotkey/controller_test.go
  - app/ontext-wails/app.go
  - app/ontext-wails/go.mod
  - app/ontext-wails/go.sum
  - tasks/stage-13-hotkey-go.md

dependencies_added:
  - golang.design/x/hotkey v0.6.1 (see DECISIONS.md "Hotkey Reintroduction (Stage 13, Go)" and ADR 010)

tests:
  - TestControllerHoldToTalkStartsAndStops
  - TestControllerIgnoresRepeatedKeyDown
  - TestControllerRegisterErrorFallsBack
  - TestControllerCloseUnregisters

acceptance_criteria:
  - PASS: Hotkey press/release starts/stops the pipeline (hold-to-talk, Cmd+Shift+Space / Ctrl+Shift+Space)
  - PASS: Hotkey-down/up timestamps captured as hotkey.Session{StartedAt,EndedAt}; app.go emits "usage:session" with startedAt/endedAt/durationMs for the future ADR 010 usage-metering reporter (POST /usage/events)
  - PASS: Registration failure does not crash the app - falls back to button-driven start/stop with a hotkey:unavailable status event
  - PASS: Unit tests pass (go test ./internal/hotkey/...)
  - PASS: go vet/go build pass for all internal packages on macOS

known_issues:
  - main package (app/ontext-wails) cannot be built/vetted in this workspace because frontend/dist is not present (frontend node_modules not installed) - this is a pre-existing condition unrelated to this stage; internal/... packages build and test cleanly
  - Windows build not verified in this workspace (no Windows host available); golang.design/x/hotkey supports Windows via RegisterHotKey
  - The actual POST /usage/events call to backend/ (ADR 010) is not implemented here - backend/ does not exist yet. app.go emits a "usage:session" Wails event with the session timestamps/duration so a future reporter can consume it

recommendations:
  - none
