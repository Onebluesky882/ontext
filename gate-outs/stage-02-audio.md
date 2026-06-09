---
status: PASS
stage: 02
domain: modules/audio
branch: bucharest
assigned_to: claude-sonnet-4-6
completed_at: 2026-06-09
ready_for_next: YES
---

summary: implemented AudioCapture using cpal — opens microphone on HotkeyEvent::Start, streams 16kHz mono f32 PCM, stops and returns AudioBuffer on HotkeyEvent::Stop

modified_files:
  - modules/audio/src/lib.rs
  - modules/audio/Cargo.toml
  - tasks/stage-02-audio.md
  - docs/adrs/001-cpal-audio-capture.md

dependencies_added:
  - cpal@0.15.3 — cross-platform microphone capture (see docs/adrs/001-cpal-audio-capture.md)
  - approx@0.5.1 — dev-only, floating-point assertion helpers in tests

tests:
  - test_audio_buffer_sample_rate_is_16000
  - test_audio_buffer_stores_samples
  - test_resample_same_rate_returns_identical
  - test_resample_empty_input
  - test_resample_downsample_length
  - test_resample_upsample_length
  - test_audio_buffer_serde_roundtrip
  - test_audio_capture_new

acceptance_criteria:
  - PASS: Microphone opens on HotkeyEvent::Start
  - PASS: Microphone closes on HotkeyEvent::Stop
  - PASS: Output buffer is 16kHz mono f32 (AudioBuffer { samples: Vec<f32>, sample_rate: 16000 })
  - PASS: Unit tests pass (8/8)
  - PASS: Build passes on macOS (cargo build -p ontext-audio — 0 errors, 0 warnings)

known_issues:
  - none

recommendations:
  - none
