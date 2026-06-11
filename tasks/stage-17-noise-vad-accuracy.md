# Stage 17 — noise filtering / VAD accuracy verification

Status: READY

Domain: `app/ontext-wails/internal/audio`, `app/ontext-wails/internal/vad`
Branch: `feature/stage-17-noise-vad-accuracy`

Goal:
Verify the streaming RMS-VAD (Stage 08) correctly removes background noise
(fan noise, keyboard typing, etc.) without dropping speech segments, and that
`IsLikelyHallucination` thresholds (Stage 09) remain appropriate for noisy
input. Test end-to-end transcription accuracy against real noisy-environment
audio fixtures.

Assigned To: (unassigned)
Started At: -
Completed At: -

---

Checklist:
- [ ] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md (Stage 17), Stage 08 + Stage 09 gate-outs
- [ ] Gather/prepare noisy audio fixtures (fan noise, keyboard, ambient speech)
- [ ] Run streaming RMS-VAD against fixtures — verify noise filtered,
      speech segments preserved with correct timestamps
- [ ] Review `IsLikelyHallucination` thresholds against noisy-input results —
      if adjustment needed, document rationale in DECISIONS.md before changing
- [ ] Run end-to-end transcription on noisy fixtures — measure accuracy vs.
      expected text
- [ ] Confirm silence-only input yields no API calls and no panic
- [ ] Unit tests pass (existing + any new fixtures)
- [ ] Create gate-outs/stage-17-noise-vad-accuracy.md

---

Gate-Out: gate-outs/stage-17-noise-vad-accuracy.md
Next Stage: none (independent verification stage)
