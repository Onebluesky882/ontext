# Stage 13 — hotkey (Go)

Status: READY

Domain: `app/ontext-wails/internal/hotkey` (new Go package)
Branch: `feature/stage-13-hotkey-go`

Background / Resolution:
PIPELINE.md previously stated `modules/hotkey` is dropped (dead code, not
ported) — see PROJECT.md: hotkey-driven recording was replaced by
button-driven start/stop after `rdev` caused crashes on macOS when
Accessibility permission was not granted.

Orchestrator confirmed (2026-06-11) that the global hotkey should be
reintroduced — as a hold-to-talk control that doubles as the usage-session
timer for billing (ADR 010, Accepted). DECISIONS.md ("Hotkey Reintroduction
(Stage 13, Go)") and PROJECT.md have been updated to record the reversal and
the chosen library, `golang.design/x/hotkey` (replaces `rdev`).

Goal:
Implement a global hold-to-talk hotkey listener in Go that emits Start (on
key-down) / Stop (on key-up) signals into `pipeline.Pipeline`, and captures
`startedAt`/`endedAt` timestamps for usage-session reporting per ADR 010
(`POST /usage/events`, `durationMs = endedAt - startedAt`). Must degrade
gracefully (no crash) if Accessibility permission is missing on macOS — fall
back to button-only start/stop with a status message.

Assigned To: (unassigned)
Started At: (unset)

---

Checklist:
- [ ] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md (Stage 13), ADR 010
- [ ] Add `golang.design/x/hotkey` dependency, document in gate-out per
      AGENT_RULES.md Dependency Rules
- [ ] Implement listener in `internal/hotkey/`, emitting Start/Stop into
      `pipeline.Pipeline`
- [ ] Capture `startedAt` (key-down) / `endedAt` (key-up) timestamps and
      expose them for the usage-metering reporter (ADR 010)
- [ ] Missing Accessibility permission does not crash — falls back
      gracefully
- [ ] Unit tests pass; build passes on macOS and Windows
- [ ] Create gate-outs/stage-13-hotkey-go.md

---

Gate-Out: gate-outs/stage-13-hotkey-go.md
Next Stage: none (final stage)
