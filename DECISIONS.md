# DECISIONS.md

This file records all technology decisions for ontext.
These decisions are authoritative. Do not switch technologies without updating this file and getting orchestrator approval.

---

## Runtime: Tauri 2

Decision: Use Tauri 2 as the application runtime.

Reason:
- Cross-platform: macOS, Windows, iOS, Android
- Rust backend — safe, fast, low memory
- Smaller binary than Electron
- Native OS integration for hotkey, audio, clipboard

Do not switch to: Electron, Flutter, React Native

---

## Frontend: React 19 + TypeScript

Decision: Use React 19 with TypeScript and Vite.

Reason:
- Already initialized in project
- Type safety reduces agent errors across stages

Do not switch to: Vue, Svelte, plain JS

---

## Transcription: OpenAI Whisper API

Decision: Use OpenAI Whisper API as the default transcription backend.

Reason:
- High accuracy across languages including Thai
- No local model setup required
- Easy to swap for local whisper.cpp if needed

API endpoint: `https://api.openai.com/v1/audio/transcriptions`
Model: `whisper-1`

Alternative (future): whisper.cpp local model — document in DECISIONS.md when switching.

Do not use: Google Speech-to-Text, Azure Speech, DeepSpeech

---

## VAD: webrtc-vad

Decision: Use `webrtc-vad` Rust crate for voice activity detection.

Reason:
- Lightweight, no ML dependency
- Fast, deterministic
- Proven in production (used in WebRTC)

Do not switch to: silero-vad (requires ONNX runtime, heavier)

---

## Audio Format: 16kHz mono f32 PCM

Decision: All audio in the pipeline is 16kHz, mono channel, f32 samples.

Reason:
- Whisper requires 16kHz mono
- Standardized format prevents conversion errors between modules

Do not change sample rate or channel count without updating all modules and CONTRACTS.md.

---

## Package Manager: pnpm

Decision: Use pnpm for Node/frontend dependencies.

Reason:
- Already initialized in project (`pnpm-lock.yaml` exists)

Do not switch to: npm, yarn, bun

---

## Rust Edition: 2021

Decision: Use Rust edition 2021.

Reason:
- Already set in Cargo.toml
- Latest stable edition at project init

---

## Branch Strategy

Decision: feature branches only. Never commit directly to `main` or `dev`.

Branch format: `feature/<module-name>`

Examples:
- `feature/hotkey`
- `feature/audio`
- `feature/vad`
- `feature/transcribe`
- `feature/clipboard`
