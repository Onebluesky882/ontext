---
status: PASS
stage: 17
domain: app/ontext-wails/internal/audio, app/ontext-wails/internal/vad
branch: stage-17-noise-vad-accuracy
assigned_to: claude-sonnet-4-6
completed_at: 2026-06-12
ready_for_next: YES
---

summary: Added synthetic noise fixtures (fan hum, keyboard click, speech-with-background-noise) to internal/vad tests; verified streaming RMS-VAD filters continuous low-level noise and short noise bursts while preserving speech segments, and reviewed IsLikelyHallucination thresholds against this behavior — no changes needed.

modified_files:
  - app/ontext-wails/internal/vad/vad_test.go
  - tasks/stage-17-noise-vad-accuracy.md

dependencies_added:
  - none

tests:
  - TestRMSDetector_FanNoiseFiltered
  - TestRMSDetector_KeyboardClickDiscarded
  - TestRMSDetector_SpeechWithBackgroundNoisePreserved
  - all pre-existing internal/vad and internal/transcribe tests (go test ./internal/vad/... ./internal/transcribe/... -> ok)

acceptance_criteria:
  - PASS: Continuous low-amplitude fan/AC noise (RMS 0.005, below 0.02 threshold) produces zero segments and therefore zero transcription calls (FanNoiseFiltered)
  - PASS: Short keyboard-click noise burst (150ms, above threshold but below minChunkMs=500ms) is discarded (KeyboardClickDiscarded)
  - PASS: Speech segment with continuous background noise mixed in is still detected and its samples preserved (SpeechWithBackgroundNoisePreserved)
  - PASS: Silence-only input (existing SilenceOnly test, and new FanNoiseFiltered) yields zero segments, zero downstream calls, no panic
  - PASS: go build ./internal/... and go vet ./internal/vad/... clean
  - PASS: go test ./internal/vad/... ./internal/transcribe/... -> ok

known_issues:
  - No real microphone-captured noisy audio fixtures (fan/keyboard/ambient WAV recordings) were available in this environment; fixtures are synthetic (deterministic pseudo-random noise mixed with sine-wave "speech" tones). This exercises the RMS-VAD threshold/timing logic correctly but does not validate behavior against real-world spectral characteristics of fan or keyboard noise.
  - End-to-end transcription accuracy against noisy fixtures was not run: TestGroqTranscriber_RealAPI is gated on the GROQ env var, which is not set in this environment. IsLikelyHallucination thresholds (NoSpeechProbThreshold=0.5, AvgLogprobThreshold=-1.0, CompressionRatioThreshold=2.4, set in Stage 09) were reviewed against the VAD output behavior; since the VAD already discards low-RMS noise and short bursts before they reach transcribe, no threshold changes are warranted and none were made.
  - Stage 08's known issue (vad.Segment lacks start_ms/end_ms timestamps) is unchanged; "correct timestamps" verification in this stage's goal was therefore limited to sample-count/order correctness, not wall-clock timestamps.

recommendations:
  - If real noisy-environment audio fixtures (fan, keyboard, ambient speech WAV files) become available, add them under a testdata/ directory and extend internal/vad tests to load them directly rather than relying on synthetic noise.
  - To validate end-to-end accuracy, run TestGroqTranscriber_RealAPI with GROQ set against recordings of known expected text in a noisy room.
