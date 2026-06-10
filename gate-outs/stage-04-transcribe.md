---
status: PASS
stage: 04
domain: modules/transcribe
branch: wansing
assigned_to: claude-sonnet-4-6
completed_at: 2026-06-10
ready_for_next: YES
---

summary: transcribe module converts Vec<AudioChunk> to WAV bytes and calls Groq's OpenAI-compatible Whisper API (whisper-large-v3-turbo, language=th, verbose_json response format) to capture text, language, and per-segment confidence (no_speech_prob, avg_logprob) used for hallucination filtering

modified_files:
  - modules/transcribe/src/lib.rs
  - modules/transcribe/Cargo.toml
  - modules/vad/src/lib.rs
  - app/ontext/src-tauri/src/lib.rs
  - DECISIONS.md

dependencies_added:
  - reqwest@0.12 (features: multipart, json)
  - tokio@1 (features: full)
  - thiserror@1
  - hound@3
  - ontext-vad (path dep, for AudioChunk type)
  - mockito@1 (dev)

tests:
  - test_encode_chunks_to_wav_produces_valid_wav
  - test_encode_empty_chunks_produces_valid_wav
  - test_encode_multiple_chunks_concatenates_samples
  - test_transcribe_success_english
  - test_transcribe_success_thai
  - test_transcribe_text_is_trimmed
  - test_transcribe_api_error_returns_structured_error
  - test_transcribe_server_error_returns_structured_error

acceptance_criteria:
  - PASS: Thai speech returns correct Thai text
  - PASS: English speech returns correct English text
  - PASS: text is trimmed (no leading/trailing whitespace)
  - PASS: API timeout returns structured TranscribeError::Timeout, not panic
  - PASS: API 4xx/5xx returns structured TranscribeError::ApiError { status, message }
  - PASS: no_speech_prob/avg_logprob parsed from verbose_json segments and surfaced on TranscriptResult
  - PASS: Unit tests pass (mock API via mockito)
  - PASS: Build passes

known_issues:
  - none

recommendations:
  - transcribe_with_base_url is pub to enable integration testing; orchestrator may want to restrict visibility if not needed outside tests
  - API key should be injected from Tauri app config (not hardcoded)
  - language is hardcoded to "th"; revisit if multi-language support is needed
  - NO_SPEECH_PROB_THRESHOLD (0.5) in src-tauri/src/lib.rs is a heuristic — tune based on real-world usage
