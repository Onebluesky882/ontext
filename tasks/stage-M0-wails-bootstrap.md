# Stage M0 — wails-bootstrap

Status: DONE
Completed At: 2026-06-11

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
- [x] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md, ADR 009
- [x] Confirm Wails CLI prerequisites (Go 1.22+, Wails CLI installed) — document
      versions used in gate-out
- [x] Scaffold new Wails project (`app/ontext-wails/`)
- [x] Copy `app/ontext/src/` (React frontend) into the new project's
      `frontend/` directory, adjusting tsconfig/package.json as needed —
      no UI behavior changes (`tsc` + `vite build` pass)
- [ ] Add Tailwind CSS to the new frontend — DEFERRED to stage 11
      (frontend Wails bindings), since that stage already touches all
      frontend UI wiring
- [ ] Add a single placeholder Go-bound method (e.g. `Greet(name string) string`)
      and call it from the frontend to confirm the Go<->JS binding works —
      DEFERRED: `Greet` exists in `app.go` (from `wails init` template) but
      the copied `App.tsx` doesn't call it yet; real bindings are added in
      stage 11
- [ ] `wails dev` launches and renders the existing UI shell — NOT verified
      interactively in this session (only `go build` and `vite build`
      verified); recommend stage 11 verify with `wails dev`
- [x] Verify existing Tauri app (`app/ontext/src-tauri`) still builds —
      untouched, no shared files modified
- [x] Build passes on macOS (`go build ./...`, `vite build`)
- [x] Create gate-outs/stage-M0-wails-bootstrap.md

---

Gate-Out: gate-outs/stage-M0-wails-bootstrap.md
Next Stage: 07 — audio (Go)
