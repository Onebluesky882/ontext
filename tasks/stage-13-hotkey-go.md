# Stage 13 — hotkey (Go)

Status: PROPOSED — needs orchestrator decision before starting

Domain: `app/ontext-wails/internal/hotkey` (new Go package)
Branch: `feature/stage-13-hotkey-go`

Background / Conflict:
PIPELINE.md previously stated `modules/hotkey` is dropped (dead code, not
ported) — see PROJECT.md: hotkey-driven recording was replaced by
button-driven start/stop after `rdev` caused crashes on macOS when
Accessibility permission was not granted.

This stage proposes reintroducing a global hotkey for the Wails app. Do
NOT start implementation until:
- [ ] Orchestrator confirms global-hotkey start/stop should be reintroduced
- [ ] DECISIONS.md and PROJECT.md updated to record the reversal and the
      chosen Go hotkey library (e.g. `golang.design/x/hotkey`), with
      rationale for why it avoids the prior `rdev`/Accessibility crash
- [ ] If orchestrator declines: mark this stage DROPPED in this file and in
      PIPELINE.md — no further action

Goal (if confirmed):
Implement a global hotkey listener in Go that emits Start/Stop signals into
`pipeline.Pipeline`. Must degrade gracefully (no crash) if Accessibility
permission is missing on macOS — fall back to button-only start/stop with a
status message.

Assigned To: (unassigned)
Started At: (unset)

---

Checklist (only after orchestrator confirmation above):
- [ ] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md (Stage 13)
- [ ] Add chosen hotkey dependency, document in gate-out per AGENT_RULES.md
      Dependency Rules + ADR if new
- [ ] Implement listener in `internal/hotkey/`, emitting Start/Stop into
      `pipeline.Pipeline`
- [ ] Missing Accessibility permission does not crash — falls back
      gracefully
- [ ] Unit tests pass; build passes on macOS and Windows
- [ ] Create gate-outs/stage-13-hotkey-go.md

---

Gate-Out: gate-outs/stage-13-hotkey-go.md
Next Stage: none (final stage)
