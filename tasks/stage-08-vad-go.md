# Stage 08 — vad (Go)

Status: TODO

Domain: `app/ontext-wails/internal/vad` (Go package)
Branch: `feature/stage-08-vad-go`

Goal:
Port the streaming RMS-VAD implementation from `modules/vad` (Rust) to Go.
Same input/output contract as original Stage 3 (CONTRACTS.md). The package
skeleton already exists:
- `vad.go` defines `Segment { Samples []float32, SampleRate int }` and the
  `Detector` interface (`Detect(ctx, <-chan audio.Frame) <-chan Segment`)
- `noop.go` has `NoopDetector` (placeholder, currently wired into
  `app.go`)

Implement an RMS-based `Detector` to replace `NoopDetector`.

Assigned To: (unassigned)
Started At: (unset)

---

Checklist:
- [ ] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md (Stage 08), ADR 009
- [ ] Implement `RMSDetector` in `internal/vad/rms.go` satisfying `Detector`,
      porting thresholds/timing constants from `modules/vad` (Rust)
- [ ] Silent segments removed, speech segments preserved with correct
      start/end timing
- [ ] Closed input channel yields a closed output channel (no panic) on
      empty input
- [ ] Wire `RMSDetector` into `app.go` (replacing `vad.NoopDetector{}`)
- [ ] Unit tests pass, using the same fixtures as the Rust version where
      possible (port fixtures if needed)
- [ ] Create gate-outs/stage-08-vad-go.md

---

Gate-Out: gate-outs/stage-08-vad-go.md
Next Stage: 09 — transcribe (Go)
