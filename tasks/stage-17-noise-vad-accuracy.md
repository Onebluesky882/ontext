# Stage 17 — noise filtering / VAD accuracy verification

Status: DONE

Domain: `app/ontext-wails/internal/audio`, `app/ontext-wails/internal/vad`
Branch: `feature/stage-17-noise-vad-accuracy`

Goal:
Verify the streaming RMS-VAD (Stage 08) correctly removes background noise
(fan noise, keyboard typing, etc.) without dropping speech segments, and that
`IsLikelyHallucination` thresholds (Stage 09) remain appropriate for noisy
input. Test end-to-end transcription accuracy against real noisy-environment
audio fixtures.

Assigned To: claude-sonnet-4-6
Started At: 2026-06-12
Completed At: 2026-06-12

---

Checklist:
- [x] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md (Stage 17), Stage 08 + Stage 09 gate-outs
- [x] Gather/prepare noisy audio fixtures (fan noise, keyboard, ambient speech)
- [x] Run streaming RMS-VAD against fixtures — verify noise filtered,
      speech segments preserved with correct timestamps
- [x] Review `IsLikelyHallucination` thresholds against noisy-input results —
      if adjustment needed, document rationale in DECISIONS.md before changing
- [x] Run end-to-end transcription on noisy fixtures — measure accuracy vs.
      expected text
- [x] Confirm silence-only input yields no API calls and no panic
- [x] Unit tests pass (existing + any new fixtures)
- [x] Create gate-outs/stage-17-noise-vad-accuracy.md

---

Gate-Out: gate-outs/stage-17-noise-vad-accuracy.md
Next Stage: none (independent verification stage)
