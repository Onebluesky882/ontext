# PROJECT.md

## Project Name

ontext

## Description

ontext is a cross-platform speech-to-text application.
Click Start in the app → speak → click Stop → text is pasted into the active input field.

Note: the original design used a global hotkey (Ctrl+Space) to start/stop
recording. The hotkey listener (rdev) caused crashes when macOS Accessibility
permission was not granted, so it was removed in favor of a button-driven
start/stop in the UI. `modules/hotkey` still exists but is unused/dead code —
see DECISIONS.md before re-enabling it.

## Platforms

- macOS
- Windows
- iOS
- Android

## Tech Stack

- Runtime: Tauri 2
- Frontend: React 19 + TypeScript + Vite
- Backend: Rust (Tauri core)
- Transcription: Groq Whisper API (whisper-large-v3-turbo, language=th)

## Core Flow

```
User clicks Start (UI button)
  → Audio capture starts, streams mic chunks
  → Streaming RMS-VAD detects speech segments
  → Each segment sent to Groq Whisper for transcription
  → User clicks Stop
  → Combined transcript copied to clipboard
  → Text pasted into active input (Cmd+V / Ctrl+V simulation)
```

## Modules

| Stage | Module              | Responsibility                                    |
|-------|---------------------|----------------------------------------------------|
| 1     | modules/hotkey      | Global hotkey detection (unused — see note above) |
| 2     | modules/audio       | Microphone capture, PCM buffer                    |
| 3     | modules/vad         | Voice activity detection (streaming RMS-VAD)      |
| 4     | modules/transcribe  | Audio → text via Groq Whisper                     |
| 5     | modules/clipboard   | Write text to clipboard, paste                    |

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

Stages 1-5 complete (see tasks/ and gate-outs/). Hotkey-driven recording
(stage 1) was replaced by button-driven start/stop — see note in Description.
