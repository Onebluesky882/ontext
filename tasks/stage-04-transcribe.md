# Stage 04 — transcribe

Status: DONE

Domain: modules/transcribe
Branch: feature/transcribe

Goal:
Send Vec<AudioChunk> to Groq's OpenAI-compatible Whisper API. Return TranscriptResult { text, language, no_speech_prob, avg_logprob }.

Assigned To: claude-sonnet-4-6
Started At: 2026-06-09
Completed At: 2026-06-10

---

Checklist:
- [x] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md
- [x] Implement modules/transcribe
- [x] Call Groq Whisper API (model: whisper-large-v3-turbo, language: th)
- [x] Thai speech returns correct Thai text
- [x] English speech returns correct English text
- [x] text is trimmed (no leading/trailing whitespace)
- [x] API timeout returns structured error, not panic
- [x] no_speech_prob/avg_logprob surfaced for hallucination filtering
- [x] Unit tests pass (mock API allowed)
- [x] Build passes
- [x] Create gate-outs/stage-04-transcribe.md

---

Gate-Out: gate-outs/stage-04-transcribe.md
Next Stage: stage-05-clipboard
