# Stage M0 — wails-bootstrap

Status: TODO

Domain: app/ontext (new Wails project)
Branch: feature/wails-bootstrap

Goal:
Scaffold a Wails v2 project alongside the existing Tauri app, and move the
existing React frontend into it without behavior changes. This is the first
stage of the Tauri/Rust → Wails/Go migration (see ADR 009, PIPELINE.md
Migration Pipeline section).

Background:
ontext stages 1-6 are complete on Tauri 2 + Rust. The team decided to migrate
the runtime to Wails v2 + Go (DECISIONS.md, ADR 009). The React 19 +
TypeScript + Vite frontend is reused as-is. This stage only sets up the new
project skeleton — no pipeline logic (audio/vad/transcribe/clipboard) is
ported yet.

Assigned To: claude-sonnet-4-6
Started At: 2026-06-11

---

Checklist:
- [ ] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md, ADR 009
- [ ] Confirm Wails CLI prerequisites (Go 1.22+, Wails CLI installed) — document
      versions used in gate-out
- [ ] Scaffold new Wails project (suggest: `app/ontext-wails/` to avoid
      colliding with the existing Tauri app during migration; final path
      decided at cutover in stage M5)
- [ ] Copy `app/ontext/src/` (React frontend) into the new project's
      `frontend/` directory, adjusting Vite config/imports as needed —
      no UI behavior changes
- [ ] Add Tailwind CSS to the new frontend (not present in current Tauri
      frontend — see DECISIONS.md "Styling: Tailwind CSS"). Existing
      components keep current inline/CSS styling for now; Tailwind is
      available for new UI work from M0 onward
- [ ] Add a single placeholder Go-bound method (e.g. `Greet(name string) string`)
      and call it from the frontend to confirm the Go<->JS binding works
- [ ] `wails dev` launches and renders the existing UI shell
- [ ] Verify existing Tauri app (`app/ontext/src-tauri`) still builds —
      no regressions, no shared files modified
- [ ] Build passes on macOS
- [ ] Create gate-outs/stage-M0-wails-bootstrap.md

---

Gate-Out: gate-outs/stage-M0-wails-bootstrap.md
Next Stage: M1 — audio (Go)
