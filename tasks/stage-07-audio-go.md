# Stage 07 — audio (Go)

Status: DONE

Domain: `app/ontext-wails/internal/audio` (Go package)
Branch: `feature/stage-07-audio-go`

Goal:
Port `modules/audio` (cpal-based capture) to Go using `malgo`. Same
input/output contract as original Stage 2 (CONTRACTS.md). The package
skeleton already exists:
- `audio.go` defines `Frame { Samples []float32, SampleRate int }` and the
  `Capturer` interface (`Start(ctx) (<-chan Frame, error)`, `Stop() error`)
- `noop.go` has `NoopCapturer` (placeholder, currently wired into
  `app.go`)

Implement a `malgo`-backed `Capturer` to replace `NoopCapturer`.

Assigned To: claude-sonnet-4-6
Started At: 2026-06-11
Completed At: 2026-06-11

---

Checklist:
- [x] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md (Stage 07), ADR 009
- [x] Add `github.com/gen2brain/malgo` dependency to `app/ontext-wails/go.mod`
- [x] Implement `MalgoCapturer` in `internal/audio/malgo.go` satisfying `Capturer`
- [x] Stream 16kHz mono f32 frames on the returned channel; close cleanly on `Stop`/ctx cancel
- [x] Wire `MalgoCapturer` into `app.go` (replacing `audio.NoopCapturer{}`)
- [x] Unit tests pass
- [x] Build passes on macOS and Windows (cross-compile check at minimum)
- [x] Create gate-outs/stage-07-audio-go.md

---

Gate-Out: gate-outs/stage-07-audio-go.md
Next Stage: 08 — vad (Go)
