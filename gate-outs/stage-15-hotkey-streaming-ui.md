---
status: PASS
stage: 15
domain: app/ontext-wails/frontend/src, app/ontext-wails/internal/pipeline
branch: stage-15-hotkey-streaming-ui
assigned_to: claude-sonnet-4-6
completed_at: 2026-06-12
ready_for_next: YES
---

summary: added pipeline.OnPartialTranscript callback wired to a new "transcript:partial" Wails event, and restructured the frontend main flow into three step-by-step pages (permissions, hotkey status, live transcript) that consume the existing status/hotkey:unavailable/transcript:partial events

modified_files:
  - app/ontext-wails/app.go
  - app/ontext-wails/internal/pipeline/pipeline.go
  - app/ontext-wails/frontend/src/App.tsx
  - app/ontext-wails/frontend/src/App.css
  - app/ontext-wails/frontend/src/hooks/usePipeline.ts
  - app/ontext-wails/frontend/src/store/appStore.ts
  - app/ontext-wails/frontend/src/types/events.ts
  - app/ontext-wails/frontend/src/pages/flow/FlowPage.tsx
  - app/ontext-wails/frontend/src/pages/flow/PermissionsPage.tsx
  - app/ontext-wails/frontend/src/pages/flow/HotkeyStatusPage.tsx
  - app/ontext-wails/frontend/src/pages/flow/LiveTranscriptPage.tsx
  - app/ontext-wails/frontend/src/pages/MainPage.tsx (deleted, replaced by pages/flow/*)
  - tasks/stage-15-hotkey-streaming-ui.md

dependencies_added:
  - none

tests:
  - go build ./... (app/ontext-wails)
  - go vet ./... (app/ontext-wails)
  - go test ./internal/pipeline/... ./internal/hotkey/...
  - tsc --noEmit (frontend)
  - vite build (frontend)

acceptance_criteria:
  - PASS: pipeline.Pipeline.OnPartialTranscript is invoked with the cumulative transcript text after each VAD segment is transcribed; app.go emits it via runtime.EventsEmit(ctx, "transcript:partial", text) - no new IPC pattern, reuses the Stage 11 EventsEmit/EventsOn bridge
  - PASS: FlowPage.tsx renders three ordered pages (PermissionsPage -> HotkeyStatusPage -> LiveTranscriptPage) on first run, gated by a localStorage flag ("ontext:flow-done") similar to the existing onboarding flow
  - PASS: PermissionsPage checks/prompts for microphone (getUserMedia, frontend-only) and Accessibility (existing RequestAccessibilityPermission), with the Stage 13 "open System Settings" fallback shown if Accessibility is denied
  - PASS: HotkeyStatusPage and LiveTranscriptPage subscribe to the existing "status" and "hotkey:unavailable" events (Stage 11/13) and the new "transcript:partial" event; live transcript text updates onChange-style as partial events arrive
  - PASS: tsc --noEmit and vite build both pass with no errors
  - PASS: Stage 13 hotkey start/stop logic (internal/hotkey) untouched - go test ./internal/hotkey/... passes unchanged

known_issues:
  - "processing" is not a distinct pipeline.Status value (pipeline.go only emits idle/running/done/error); HotkeyStatusPage/StatusBadge therefore show "running" while transcribing rather than a separate "processing" state. A true processing state would require a pipeline.go status change, which is outside this stage's wiring-only scope
  - OnPartialTranscript emits the cumulative transcript only after each VAD segment completes (not true token-by-token streaming), since transcribe.Transcriber returns whole-segment results; this matches the granularity available from Stage 4/9's TranscriptResult contract
  - wails dev GUI smoke test not run in this headless environment; verified via tsc/vite build/go build/vet/test as in prior stages

recommendations:
  - if a true "processing" UI state is desired, a future stage could add pipeline.StatusProcessing (emitted between segment capture and transcribe completion) - flagging here rather than changing pipeline.go under this wiring-only stage
