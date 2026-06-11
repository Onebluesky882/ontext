# Stage 09 — transcribe (Go)

Status: IN_PROGRESS (largely implemented — needs review + tests + gate-out)

Domain: `app/ontext-wails/internal/transcribe` (Go package)
Branch: `feature/stage-09-transcribe-go`

Background:
A working Groq Whisper client was implemented during the M0 scaffolding
session:
- `groq.go` — `GroqTranscriber` (stdlib `net/http`, model
  `whisper-large-v3`, reads `GROQ` env var)
- `wav.go` — PCM float32 → 16-bit WAV encoding
- `transcribe.go` — `Result` type with `NoSpeechProb`/`AvgLogprob`/
  `CompressionRatio` and `IsLikelyHallucination()`, ported thresholds from
  `modules/transcribe` (Rust)
- `groq_test.go` — live API test gated on `GROQ` env var (passes)
- `noop.go` — `NoopTranscriber`, used as fallback in `app.go` when `GROQ`
  is unset

Goal:
Review/finish this implementation against the original Stage 4 contract
(CONTRACTS.md), add unit tests that don't require network access (mock
HTTP server for success/error/timeout cases), and confirm the model choice
(`whisper-large-v3` vs `-turbo` — see DECISIONS.md, which currently
specifies `whisper-large-v3-turbo`) and document/update DECISIONS.md if the
model differs intentionally.

Assigned To: (unassigned)
Started At: (unset)

---

Checklist:
- [ ] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md (Stage 09), ADR 009
- [ ] Confirm model choice (`whisper-large-v3` vs DECISIONS.md's
      `whisper-large-v3-turbo`) — update DECISIONS.md or `groq.go` to match
- [ ] Add mock-HTTP-server unit tests for: success response, API error
      status, timeout (no live `GROQ` key required)
- [ ] Confirm `Transcribe` never panics on API failure — returns `error`
- [ ] Confirm hallucination thresholds match `modules/transcribe` (Rust):
      `NoSpeechProbThreshold=0.5`, `AvgLogprobThreshold=-1.0`,
      `CompressionRatioThreshold=2.4`
- [ ] Build passes
- [ ] Create gate-outs/stage-09-transcribe-go.md

---

Gate-Out: gate-outs/stage-09-transcribe-go.md
Next Stage: 10 — clipboard (Go)
