# ADR 011 — AI Text Autocorrect (Post-Transcribe Recheck)

## Status

Accepted

## Date

2026-06-12

## Context

Whisper transcription (Stage 09/Stage 04) produces raw text that often
contains minor spelling, grammar, capitalization, and punctuation errors,
especially for technical terms, names, and homophones. These errors are
pasted verbatim into the active app (Stage 06/Stage 12), requiring manual
correction by the user.

Stage 17 confirmed the VAD/hallucination pipeline reliably filters
non-speech, but does not address quality of the transcribed text itself.

## Decision

Introduce a new `internal/autocorrect` module that runs as an additional
pipeline step between `transcribe` and `clipboard`:

```
transcribe.Result.Text -> autocorrect.Corrector.Correct(text) -> corrected text -> clipboard.Writer.Paste(correctedText)
```

`autocorrect.Corrector` is implemented via a Groq chat-completion call
(consistent with Stage 09's existing Groq usage for transcription), using a
small/fast instruction model (e.g. `llama-3.1-8b-instant`), with a prompt
constrained to fix spelling/grammar/punctuation only — no rephrasing, no
added/removed meaning, no added commentary.

On API error, timeout, or empty/unchanged response, the corrector returns the
original `transcribe.Result.Text` unchanged (fail-open) so the pipeline never
blocks or drops a transcript due to autocorrect failures.

## Consequences

- New contract: `autocorrect.Corrector` interface (see CONTRACTS.md)
- New external dependency: one additional Groq API call per pasted segment
  (chat completions endpoint), adding latency and STT-adjacent cost. Per
  ADR 010, this is a candidate for inclusion in the same usage-based billing
  model if cost becomes material — out of scope for this ADR.
- Pipeline (`internal/pipeline`) gains one more step; `PasteResult` is
  unaffected (still pastes a single string).
- A no-op `autocorrect.NoopCorrector` (returns input unchanged) must exist
  for tests and for environments without a Groq key, mirroring
  `transcribe.NoopTranscriber` (Stage 09).
