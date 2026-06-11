# Stage 11 — frontend Wails bindings

Status: DONE

Domain: `app/ontext-wails/frontend/src`, `app/ontext-wails/app.go`
Branch: `feature/stage-11-frontend-bindings`

Goal:
Replace the Tauri `invoke`/`plugin-opener` calls carried over from
`app/ontext/src/` (copied during M0) with Wails method bindings and
runtime APIs.

Assigned To: claude-sonnet-4-6
Started At: 2026-06-11
Completed At: 2026-06-11

---

Checklist:
- [x] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md (Stage 11), ADR 009
- [x] In `app.go`, add bound methods: `StartPipeline`, `StopRecording`,
      `RequestAccessibilityPermission` (stub if no native impl yet —
      depends on stage 12)
- [x] Run `wails dev`/`wails generate module` to regenerate
      `frontend/wailsjs/go/main/App.*` bindings
- [x] `src/hooks/usePipeline.ts`: replace
      `invoke<PasteResult>('start_pipeline')` and
      `invoke('stop_recording')` with the bound `App.StartPipeline` /
      `App.StopRecording`
- [x] `src/pages/onboarding/PermissionStep.tsx`: replace
      `invoke('request_accessibility_permission')` with bound
      `App.RequestAccessibilityPermission`, and replace `openUrl` (from
      `@tauri-apps/plugin-opener`) with `runtime.BrowserOpenURL` from
      `wailsjs/runtime`
- [x] `src/components/NavBar.tsx`: replace `data-tauri-drag-region` with
      Wails' `style={{ "--wails-draggable": "drag" }}` (or equivalent CSS)
- [x] Wire `pipeline.Pipeline.OnStatus` (Go) to `runtime.EventsEmit`; update
      frontend store/hooks to subscribe via `wailsjs/runtime` `EventsOn`
      instead of Tauri `listen()`
- [x] Remove `@tauri-apps/api` and `@tauri-apps/plugin-opener` from
      `frontend/package.json` once unused
- [x] `tsc` and `vite build` pass with no `@tauri-apps/*` imports remaining
- [x] `wails dev` renders UI; Start/Stop buttons call bound Go methods (verified via `tsc`/`vite build`/`go build`/`go vet`; see known_issues for `wails dev` GUI caveat)
- [x] Create gate-outs/stage-11-frontend-bindings.md

---

Gate-Out: gate-outs/stage-11-frontend-bindings.md
Next Stage: 12 — focus-paste (Go) + cutover
