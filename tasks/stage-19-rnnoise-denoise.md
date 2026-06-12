# Stage 19 — RNNoise denoise (pre-VAD noise suppression)

Status: READY

Domain: `app/ontext-wails/internal/denoise`, `app/ontext-wails/internal/audio`, `app/ontext-wails/internal/vad`, `app/ontext-wails/internal/pipeline`
Branch: `feature/stage-19-rnnoise-denoise`

Goal:
Add a new `internal/denoise` module implementing the `denoise.Denoiser`
contract (CONTRACTS.md, ADR 012) that applies RNNoise-based noise suppression
to each `audio.Frame` via a cgo binding to `librnnoise`, with a fail-open
`denoise.NoopDenoiser` fallback. Wire it into `internal/pipeline` between
`audio` and `vad`.

Assigned To: (unassigned)
Started At:
Completed At:

---

Checklist:
- [ ] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md
      (Stage 19), ADR 012, Stage 07 (audio) and Stage 08/17 (vad) gate-outs
- [ ] Implement `denoise.Denoiser` interface in
      `app/ontext-wails/internal/denoise`
- [ ] Implement RNNoise-backed denoiser via cgo binding to `librnnoise`,
      re-chunking `audio.Frame.Samples` to RNNoise's native frame size as
      needed while returning frames of unchanged length/`SampleRate`
- [ ] Implement `denoise.NoopDenoiser` (returns input unchanged), mirroring
      `transcribe.NoopTranscriber` / `autocorrect.NoopCorrector`
- [ ] On RNNoise init/runtime failure, fall back to passing the frame through
      unchanged (fail-open) — never block or drop frames
- [ ] Wire `denoise.Denoiser` into `internal/pipeline` between `audio` and
      `vad`, so `vad.Detector.Detect` receives denoised frames
- [ ] If RMS-VAD thresholds (Stage 08/17) need adjustment for pre-denoised
      input, document rationale in DECISIONS.md before changing
- [ ] Unit tests: denoiser reduces noise on sample noisy fixtures (e.g.
      Stage 17 fixtures) while preserving frame length/sample rate; simulated
      RNNoise init/runtime failure falls back correctly
- [ ] `go test ./internal/denoise/... ./internal/vad/... ./internal/pipeline/...` passes
- [ ] If cross-platform cgo/librnnoise build cannot be achieved, STOP and
      report in gate-out.md under Known Issues/Recommendations
- [ ] Create gate-outs/stage-19-rnnoise-denoise.md

---

Gate-Out: gate-outs/stage-19-rnnoise-denoise.md
Next Stage: TBD (orchestrator decides after gate-in)
