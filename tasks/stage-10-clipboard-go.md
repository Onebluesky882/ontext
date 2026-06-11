# Stage 10 — clipboard (Go)

Status: TODO

Domain: `app/ontext-wails/internal/clipboard` (Go package)
Branch: `feature/stage-10-clipboard-go`

Goal:
Port clipboard write + paste simulation to Go using `atotto/clipboard` +
`go-vgo/robotgo`. Same input/output contract as original Stage 5
(CONTRACTS.md). The package skeleton already exists:
- `clipboard.go` defines the `Writer` interface
  (`Paste(ctx, text string) error`)
- `noop.go` has `NoopWriter` (placeholder, currently wired into `app.go`)

Implement a real `Writer` to replace `NoopWriter`.

Assigned To: (unassigned)
Started At: (unset)

---

Checklist:
- [ ] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md (Stage 10), ADR 009
- [ ] Add `github.com/atotto/clipboard` and `github.com/go-vgo/robotgo`
      dependencies to `app/ontext-wails/go.mod`
- [ ] Implement a `Writer` in `internal/clipboard/` that writes text to the
      system clipboard and simulates Cmd+V (macOS) / Ctrl+V (Windows)
- [ ] Never panic — return descriptive `error` on failure
- [ ] Wire the new `Writer` into `app.go` (replacing `clipboard.NoopWriter{}`)
- [ ] Unit tests pass
- [ ] Build passes on macOS and Windows
- [ ] Create gate-outs/stage-10-clipboard-go.md

---

Gate-Out: gate-outs/stage-10-clipboard-go.md
Next Stage: 11 — frontend Wails bindings
