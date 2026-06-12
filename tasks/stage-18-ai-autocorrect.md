# Stage 18 — AI text autocorrect (post-transcribe recheck)

Status: READY

Domain: `app/ontext-wails/internal/autocorrect`, `app/ontext-wails/internal/pipeline`
Branch: `feature/stage-18-ai-autocorrect`

Goal:
Add a new `internal/autocorrect` module implementing the
`autocorrect.Corrector` contract (CONTRACTS.md, ADR 011) that sends
`transcribe.Result.Text` to the Groq chat-completions API to fix
spelling/grammar/punctuation only, with a fail-open `NoopCorrector` fallback.
Wire it into `internal/pipeline` between `transcribe` and `clipboard`.

Assigned To: (unassigned)
Started At:
Completed At:

---

Checklist:
- [ ] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md
      (Stage 18), ADR 011, Stage 09 (transcribe) and Stage 12
      (focus-paste-cutover/pipeline) gate-outs
- [ ] Implement `autocorrect.Corrector` interface in
      `app/ontext-wails/internal/autocorrect`
- [ ] Implement Groq-backed corrector (chat-completions endpoint, small/fast
      instruction model, e.g. `llama-3.1-8b-instant`), mirroring the
      `transcribe` package's Groq client structure
- [ ] Implement `autocorrect.NoopCorrector` (returns input unchanged),
      mirroring `transcribe.NoopTranscriber`
- [ ] Skip the Groq call entirely for empty/whitespace-only input
- [ ] On API error/timeout/empty response, fall back to original text
      (fail-open) — never block or drop the segment
- [ ] Wire `autocorrect.Corrector` into `internal/pipeline` between
      `transcribe` and `clipboard`, so `clipboard.Writer.Paste` receives the
      corrected (or fallback) text
- [ ] Unit tests: corrector fixes sample spelling/grammar/punctuation errors
      without changing meaning; API error/timeout falls back correctly;
      empty input produces no API call
- [ ] `go test ./internal/autocorrect/... ./internal/pipeline/...` passes
- [ ] Create gate-outs/stage-18-ai-autocorrect.md

---

Gate-Out: gate-outs/stage-18-ai-autocorrect.md
Next Stage: TBD (orchestrator decides after gate-in)
