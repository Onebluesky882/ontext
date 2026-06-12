# ADR 012 — RNNoise Denoise (Pre-VAD Noise Suppression)

## Status

Accepted

## Date

2026-06-12

## Context

Stage 17 verified that the streaming RMS-VAD (Stage 08) filters out
continuous low-level background noise (fan hum, keyboard clicks) using
energy-based thresholds, while preserving speech segments. RMS-based VAD is
amplitude-based only — it cannot distinguish speech from louder, non-stationary
noise (e.g. background voices, music, traffic) that sits in the same energy
range as the user's speech. That noise passes through to `transcribe.Result.Text`
and is corrected (but not removed) by Stage 18's autocorrect step, which only
fixes spelling/grammar/punctuation, not transcription of noise as bogus words.

RNNoise (Xiph.Org) is a small, real-time, RNN-based noise suppression filter
designed for 16kHz mono PCM — matching `audio.Frame`'s existing format
(Samples []float32, SampleRate 16000) exactly, requiring no resampling.

## Decision

Introduce a new `internal/denoise` module that runs as a pipeline step
between `audio` and `vad`:

```
audio.Frame -> denoise.Denoiser.Denoise(frame) -> denoised audio.Frame -> vad.Detector.Detect(...)
```

`denoise.Denoiser` is implemented via a cgo binding to `librnnoise`
(Xiph.Org RNNoise), processing `audio.Frame.Samples` in RNNoise's native
480-sample (10ms @ 48kHz-derived / 480 @ 16kHz-equivalent per RNNoise's
48kHz-internal resampling) frame size, buffering/re-chunking
`audio.Frame.Samples` as needed so callers continue to receive
`audio.Frame` of unchanged length and `SampleRate`.

On initialization failure (e.g. missing/incompatible librnnoise at runtime),
the denoiser falls back to passing frames through unchanged (fail-open), so
the pipeline never blocks or drops audio due to denoise failures.

A `denoise.NoopDenoiser` (returns the input `audio.Frame` unchanged) must
exist for tests and platforms without a working RNNoise build, mirroring
`transcribe.NoopTranscriber` (Stage 09) and `autocorrect.NoopCorrector`
(Stage 18).

## Consequences

- New contract: `denoise.Denoiser` interface (see CONTRACTS.md)
- New external dependency: cgo binding to `librnnoise` (or a Go wrapper
  package around it) — requires librnnoise to be available at build time on
  each target platform (macOS, Windows per ARCHITECTURE.md's platform table).
  If a suitable maintained Go wrapper does not exist, vendor a minimal cgo
  binding inside `internal/denoise` per Stage 19's task.
- Pipeline (`internal/pipeline`) gains one step before `vad`; `vad.Segment`
  and downstream contracts (`transcribe.Result`, `autocorrect.Corrector`,
  `clipboard.Writer`) are unaffected.
- RMS-VAD thresholds (Stage 08/17) may need re-tuning once input audio is
  pre-denoised (denoised silence has lower residual RMS than raw silence);
  any threshold change must be documented in DECISIONS.md per AGENT_RULES.
- If cgo/librnnoise cannot be made to work cross-platform within Stage 19,
  the worker must STOP and report this in gate-out.md under Known
  Issues/Recommendations rather than inventing an alternative algorithm.
