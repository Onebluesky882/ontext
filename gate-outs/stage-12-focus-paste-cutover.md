---
status: PASS
stage: 12
domain: app/ontext-wails
branch: feature/stage-12-focus-paste-cutover
assigned_to: claude-sonnet-4-6
completed_at: 2026-06-11
ready_for_next: YES
---

summary: ported macOS focus capture/restore to Go via cgo+AppKit/CoreFoundation, switched the pipeline to per-segment real-time paste, and removed the old Tauri/Rust app and workspace

modified_files:
  - app/ontext-wails/internal/focus/focus.go
  - app/ontext-wails/internal/focus/focus_darwin.go
  - app/ontext-wails/internal/focus/focus_other.go
  - app/ontext-wails/internal/pipeline/pipeline.go
  - app/ontext-wails/app.go
  - app/ontext-wails/main.go
  - PROJECT.md
  - ARCHITECTURE.md
  - CONTRACTS.md
  - DECISIONS.md
  - PIPELINE.md
  - .gitignore

dependencies_added:
  - none (cgo + AppKit/ApplicationServices frameworks, already available via Xcode toolchain on macOS)

removed:
  - app/ontext/ (Tauri frontend + src-tauri Rust backend)
  - modules/hotkey, modules/audio, modules/vad, modules/transcribe, modules/clipboard (Rust crates)
  - Cargo.toml (workspace root)

tests:
  - go build ./... (app/ontext-wails, macOS)
  - go vet ./internal/...
  - go test ./internal/...
  - go build ./internal/focus/... (CGO_ENABLED=0 GOOS=windows GOARCH=amd64) — focus package cross-compiles to a no-op on Windows
  - tsc / vite build (frontend)

acceptance_criteria:
  - PASS: Frontmost app captured and tracked continuously (internal/focus.Manager polls NSWorkspace.frontmostApplication every 300ms via cgo, macOS only)
  - PASS: Target app reactivated before each paste, with settle delay (focus.Manager.Activate uses NSRunningApplication.activateWithOptions + focus.SettleDelay = 100ms)
  - PASS: Each transcribed segment pasted immediately (pipeline.Pipeline.Start now pastes res.Text per segment instead of buffering until Stop)
  - PASS: Non-macOS builds unaffected (focus_darwin.go behind //go:build darwin; focus_other.go provides no-op stubs for other platforms)
  - PASS: Unit tests pass (go test ./internal/...)
  - PASS: Build passes on macOS (go build ./...); Windows build of internal/focus verified via cross-compile (audio/clipboard packages already require cgo+native toolchains for Windows, unchanged by this stage)
  - PASS: Old Tauri/Rust app and Cargo workspace removed (app/ontext/, modules/*, Cargo.toml)

known_issues:
  - wails dev / interactive GUI smoke test not run in this headless environment; verified via go build/vet/test + tsc/vite build instead, consistent with stage 11
  - RequestAccessibilityPermission now calls real AXIsProcessTrusted/AXIsProcessTrustedWithOptions via cgo, but has not been exercised on a live macOS session with the Accessibility prompt
  - focus.Manager.Activate errors are swallowed in the pipeline (best-effort reactivation; paste still proceeds even if activation fails) — matches stage 6 behavior where activation failure did not block paste

recommendations:
  - perform a manual end-to-end test on macOS (cursor in another app, click Start, speak, confirm text is pasted into that app per segment without clicking Stop, and that the Accessibility prompt appears if permission is not yet granted)
  - migration from Tauri/Rust to Wails/Go (ADR 009) is now complete; stage 13 (hotkey, Go) remains PROPOSED and requires orchestrator confirmation before starting
