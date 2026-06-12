# Gate-Out: Stage 14 — e2e speech-to-text verification

status: BLOCKED
ready_for_next: NO (independent verification stage — no downstream stage gated on this)

## Summary

A full `wails dev` end-to-end run (Start → speak → Stop → paste, with real
Thai/English audio against the live Groq API) could not be performed in this
environment:

- No microphone/audio input device available to the sandbox.
- No `GROQ` API key configured (`.env` absent, env var unset) — the live API
  test (`TestGroqTranscriber_RealAPI`) skips for this reason.
- Building the desktop binary (`go build -tags desktop,production`) fails at
  the link step in this sandbox:
  ```
  "_OBJC_CLASS_$_UTType", referenced from: ... in 000046.o
  ld: symbol(s) not found for architecture arm64
  ```
  This is a toolchain/SDK linking issue in the sandbox (missing
  UniformTypeIdentifiers framework symbols for the Wails/webview bindings),
  unrelated to `internal/transcribe`. A plain `go build .` (no desktop tags)
  succeeds but produces a non-functional binary ("Wails applications will not
  build without the correct build tags").

Given these constraints, verification was scoped to what is testable without
a GUI/mic/live API: automated tests, frontend build, and code review of the
error-handling path.

## What was verified

1. **Build**
   - `frontend`: `npm install && npm run build` — succeeds, produces
     `frontend/dist`.
   - `go build -o ontext-wails .` — succeeds (non-desktop build).
   - `go build -tags desktop,production .` — fails (sandbox linker issue,
     see above; not a code issue).

2. **Automated test suite** (`go test ./...`)
   - `internal/transcribe`: PASS (all 11 tests, including hallucination
     diagnostics, trimming, and error/timeout cases).
   - `internal/audio`, `internal/clipboard`, `internal/hotkey`,
     `internal/vad`: PASS.
   - `internal/pipeline`, `internal/httpapi`, `internal/focus`: no test
     files.
   - root package (`ontext-wails`): build fails for `go test` only because
     `main.go` embeds `all:frontend/dist`, which doesn't exist until the
     frontend is built (expected; resolved by running the frontend build
     first, see above).

3. **API timeout / error paths — confirmed via existing tests + code review**
   - `TestGroqTranscriber_APIError401` / `_ServerError500`: non-2xx Groq
     responses return a wrapped `error`, no panic.
   - `TestGroqTranscriber_ContextTimeout`: a canceled/expired context returns
     an `error` from `Transcribe`, no panic.
   - `internal/pipeline/pipeline.go` `Pipeline.Start`: on
     `Transcriber.Transcribe` error, sets status `StatusError` and returns
     `Result{Error: ...}` — no panic, no partial paste.
   - `app.go` `StartPipeline`: wraps `Result.Error` into
     `PasteResult{Success: false, Error: err.Error()}` per contract
     (`Error` omitted on success via `json:",omitempty"`).
   - Conclusion: the timeout/error → structured-error contract holds at the
     code level. **Not exercised through the live UI/wails runtime** (no
     display in this sandbox).

4. **Thai/English transcription accuracy — NOT TESTED**
   - No live audio input or Groq API credentials available. The mock tests
     (`TestGroqTranscriber_SuccessThai`, `_SuccessEnglish`) confirm response
     parsing/trimming for both languages, but do not exercise real model
     output.

## Bug found in `internal/transcribe` (reported, fixed)

`internal/transcribe/groq.go`: `defaultModel` was `"whisper-large-v3"`, but
DECISIONS.md (Transcription: Groq Whisper API) and PROJECT.md / the Stage 09
contract specify `whisper-large-v3-turbo`. Since `app.go`'s
`NewGroqTranscriber` relies on this default (no override), production calls
were using the wrong model.

Fix applied (in-domain, `internal/transcribe`):
- `groq.go`: `defaultModel = "whisper-large-v3-turbo"`.
- `groq_test.go`: updated `TestGroqTranscriber_RealAPI`'s assertion to expect
  `whisper-large-v3-turbo`.

All `internal/transcribe` tests pass after this change.

## Acceptance criteria

| Criterion | Result |
|---|---|
| Thai speech produces correct Thai text, pasted into focused input | NOT TESTED — no mic/display/API key in sandbox |
| English speech produces correct English text, pasted into focused input | NOT TESTED — no mic/display/API key in sandbox |
| Simulated API timeout/error returns structured error, not panic | PASS (verified via unit tests + code review; not via live UI) |
| Results documented with actual transcript samples | N/A — no live samples could be captured |

## Recommendation to conductor

The live-audio portions of this stage require an environment with a
microphone, a display capable of running the Wails desktop build, and a real
`GROQ` API key. None of these are available in this sandboxed worker
environment. Suggest either:
- Running this stage manually on a developer machine with mic + `GROQ` set
  in `.env`, using this gate-out's automated-test results as a baseline, or
- Descoping Stage 14 to the automated/unit-level checks already covered here
  (which now pass, including the model-name bug fix) and tracking the live
  manual run separately.
