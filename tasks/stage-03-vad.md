# Stage 03 — vad

Status: PENDING

Domain: modules/vad
Branch: feature/vad

Goal:
Remove silence from AudioBuffer. Return only speech segments as Vec<AudioChunk>.

Assigned To: —
Started At: —
Completed At: —

---

Checklist:
- [ ] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md
- [ ] Implement modules/vad using webrtc-vad
- [ ] Silent segments are removed
- [ ] Speech segments preserved with correct start_ms / end_ms
- [ ] Empty input returns empty Vec (no panic)
- [ ] Unit tests pass with audio fixtures
- [ ] Build passes
- [ ] Create gate-outs/stage-03-vad.md

---

Gate-Out: gate-outs/stage-03-vad.md
Next Stage: stage-04-transcribe
