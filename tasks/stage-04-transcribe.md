# Stage 04 — transcribe

Status: PENDING

Domain: modules/transcribe
Branch: feature/transcribe

Goal:
Send Vec<AudioChunk> to OpenAI Whisper API. Return TranscriptResult { text, language }.

Assigned To: —
Started At: —
Completed At: —

---

Checklist:
- [ ] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md
- [ ] Implement modules/transcribe
- [ ] Call OpenAI Whisper API (model: whisper-1)
- [ ] Thai speech returns correct Thai text
- [ ] English speech returns correct English text
- [ ] text is trimmed (no leading/trailing whitespace)
- [ ] API timeout returns structured error, not panic
- [ ] Unit tests pass (mock API allowed)
- [ ] Build passes
- [ ] Create gate-outs/stage-04-transcribe.md

---

Gate-Out: gate-outs/stage-04-transcribe.md
Next Stage: stage-05-clipboard
