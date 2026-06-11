# Stage 12 — focus-paste (Go) + cutover

Status: DONE

Domain: `app/ontext-wails` (Wails Go backend), repo root (`Cargo.toml`,
`app/ontext/`)
Branch: `feature/stage-12-focus-paste-cutover`

Goal:
Port the macOS focus capture/restoration logic from stage 6
(`gate-outs/stage-06-focus-paste.md`) to Go via `cgo` + AppKit/
CoreFoundation shims. Wire stages 07-10 (audio/vad/transcribe/clipboard)
and stage 11 (frontend bindings) together into the `pipeline.Pipeline` used
by `app.go`, matching the per-segment real-time paste behavior from
stage 6. On PASS, remove `app/ontext/src-tauri`, the `app/ontext/` Tauri
frontend, the root `Cargo.toml` workspace, and Rust `modules/*` crates.

Assigned To: claude-sonnet-4-6
Started At: 2026-06-11
Completed At: 2026-06-11

---

Checklist:
- [x] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md (Stage 12), ADR 009, gate-outs/stage-06-focus-paste.md
- [x] Confirm stages 07-11 gate-outs are PASS / ready_for_next: YES
- [x] Implement cgo + AppKit/CoreFoundation focus capture/restore
      (frontmost app tracking, reactivate before paste, settle delay),
      behind macOS build tags; non-macOS builds unaffected
- [x] Wire `audio` (07), `vad` (08), `transcribe` (09), `clipboard` (10)
      adapters into `pipeline.Pipeline` in `app.go` (already wired prior to
      this stage; added `focus.Manager` wiring)
- [x] Each transcribed segment pasted immediately (no buffering until Stop)
- [x] Unit tests pass; build passes on macOS and Windows
- [x] Remove `app/ontext/src-tauri/`, `app/ontext/` Tauri frontend
      remnants, root `Cargo.toml` workspace members, and Rust `modules/*`
      crates no longer needed
- [x] Create gate-outs/stage-12-focus-paste-cutover.md

---

Gate-Out: gate-outs/stage-12-focus-paste-cutover.md
Next Stage: 13 — hotkey (Go) (if confirmed) — otherwise migration complete
