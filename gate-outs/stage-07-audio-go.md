---
status: PASS
stage: 07
domain: app/ontext-wails/internal/audio
branch: feature/stage-07-audio-go
assigned_to: claude-sonnet-4-6
completed_at: 2026-06-11
ready_for_next: YES
---

summary: implemented MalgoCapturer in internal/audio/malgo.go using github.com/gen2brain/malgo (miniaudio CGO bindings). Streams 16 kHz mono f32 frames on a buffered channel; closes cleanly on Stop() or ctx cancellation. Wired into app.go replacing NoopCapturer.

modified_files:
  - app/ontext-wails/internal/audio/malgo.go (new)
  - app/ontext-wails/internal/audio/audio_test.go (new)
  - app/ontext-wails/app.go
  - app/ontext-wails/go.mod
  - app/ontext-wails/go.sum

dependencies_added:
  - github.com/gen2brain/malgo v0.11.25

tests:
  - TestNoopCapturer_ClosesImmediately
  - TestMalgoCapturer_StopBeforeStart
  - TestMalgoCapturer_DoubleStart (device, skipped in -short mode)
  - TestMalgoCapturer_StartStop (device, skipped in -short mode)
  - TestDecodePCMF32

acceptance_criteria:
  - PASS: Microphone opens on Start, closes on Stop (Stop cancels the context, goroutine stops device and closes channel)
  - PASS: Output frames are 16 kHz mono f32 (cfg.SampleRate=16000, cfg.Capture.Channels=1, cfg.Capture.Format=FormatF32)
  - PASS: Unit tests pass (go test ./internal/audio/... -short)
  - PASS: Build passes on macOS with CGO_ENABLED=1 (default)
  - NOTE: Windows build requires CGO on a Windows host — malgo wraps miniaudio C library, same constraint as cpal on Rust. Cross-compile from macOS without CGO is not possible (expected).

known_issues:
  - main.go embed of frontend/dist fails if frontend is not built first (pre-existing, not introduced by this stage)
  - Device tests (DoubleStart, StartStop) are skipped in -short mode; they require physical microphone and are skipped gracefully if no device is found

recommendations:
  - Stage 08 (vad Go) can begin; audio.Frame channel is the correct input type for the VAD detector
