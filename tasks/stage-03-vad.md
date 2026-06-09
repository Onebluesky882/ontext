# Stage 03 — vad

Status: DONE

Domain: modules/vad
Branch: feature/vad

Goal:
Remove silence from AudioBuffer. Return only speech segments as Vec<AudioChunk>.

Assigned To: claude-sonnet-4-6
Started At: 2026-06-09
Completed At: 2026-06-09

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
