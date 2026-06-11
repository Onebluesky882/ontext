# Stage 12 — focus-paste (Go) + cutover

Status: TODO (blocked on stages 07-11)

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

Assigned To: (unassigned)
Started At: (unset)

---

Checklist:
- [ ] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md (Stage 12), ADR 009, gate-outs/stage-06-focus-paste.md
- [ ] Confirm stages 07-11 gate-outs are PASS / ready_for_next: YES
- [ ] Implement cgo + AppKit/CoreFoundation focus capture/restore
      (frontmost app tracking, reactivate before paste, settle delay),
      behind macOS build tags; non-macOS builds unaffected
- [ ] Wire `audio` (07), `vad` (08), `transcribe` (09), `clipboard` (10)
      adapters into `pipeline.Pipeline` in `app.go` (replacing remaining
      no-ops)
- [ ] Each transcribed segment pasted immediately (no buffering until Stop)
- [ ] Unit tests pass; build passes on macOS and Windows
- [ ] Remove `app/ontext/src-tauri/`, `app/ontext/` Tauri frontend
      remnants, root `Cargo.toml` workspace members, and Rust `modules/*`
      crates no longer needed
- [ ] Create gate-outs/stage-12-focus-paste-cutover.md

---

Gate-Out: gate-outs/stage-12-focus-paste-cutover.md
Next Stage: 13 — hotkey (Go) (if confirmed) — otherwise migration complete
