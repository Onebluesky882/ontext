# PROJECT.md

## Project Name

ontext

## Description

ontext is a cross-platform speech-to-text application.
Press a hotkey → speak → release → text is pasted into the active input field.

## Platforms

- macOS
- Windows
- iOS
- Android

## Tech Stack

- Runtime: Tauri 2
- Frontend: React 19 + TypeScript + Vite
- Backend: Rust (Tauri core)
- Transcription: Whisper (via API or local model)

## Core Flow

```
Hotkey pressed
  → Audio capture starts
  → VAD detects speech
  → Audio recorded
  → Sent to Whisper for transcription
  → Text copied to clipboard
  → Text pasted into active input
```

## Modules

| Stage | Module              | Responsibility                        |
|-------|---------------------|---------------------------------------|
| 1     | modules/hotkey      | Global hotkey detection               |
| 2     | modules/audio       | Microphone capture, PCM buffer        |
| 3     | modules/vad         | Voice activity detection              |
| 4     | modules/transcribe  | Audio → text via Whisper              |
| 5     | modules/clipboard   | Write text to clipboard, paste        |

## Repository Structure

```
ontext/
├── app/ontext/           # Tauri app (frontend + Rust backend)
│   ├── src/              # React frontend
│   └── src-tauri/        # Rust backend
├── modules/              # Core Rust modules
│   ├── hotkey/
│   ├── audio/
│   ├── vad/
│   ├── transcribe/
│   └── clipboard/
├── docs/                 # Additional documentation
├── tasks/                # Agent task assignments
└── gate-outs/            # Stage completion reports
```

## Status

Stage 1 — Not started
