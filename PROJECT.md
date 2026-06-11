# PROJECT.md

## Project Name

ontext

## Description

ontext is a cross-platform speech-to-text application.
Click Start in the app → speak → click Stop → text is pasted into the active input field.

Note: the original design used a global hotkey (Ctrl+Space) to start/stop
recording. The hotkey listener (rdev) caused crashes when macOS Accessibility
permission was not granted, so it was removed in favor of a button-driven
start/stop in the UI.

Update (2026-06-11): the global hotkey is being reintroduced as a
hold-to-talk control — hotkey-down starts recording, hotkey-up stops it — and
doubles as the usage-session timer for billing (ADR 010): `startedAt`/
`endedAt` from the hold are reported to the usage-metering backend as
`durationMs`. Stage 13 (`internal/hotkey`, Go) is READY, using
`golang.design/x/hotkey` instead of `rdev` (see DECISIONS.md for why this
avoids the prior Accessibility crash). The button-driven start/stop remains
as a fallback when Accessibility permission is missing.

## Platforms

- macOS
- Windows

## Tech Stack

- Runtime: Wails v2
- Frontend: React 19 + TypeScript + Vite
- Backend: Go (1.22+)
- Transcription: Groq Whisper API (whisper-large-v3-turbo, language=th)

## Core Flow

```
User clicks Start (UI button)
  → Audio capture starts, streams mic chunks
  → Streaming RMS-VAD detects speech segments
  → Each segment sent to Groq Whisper for transcription, then pasted
    immediately into the previously focused app (real-time, per segment)
  → User clicks Stop
```

## Modules

| Stage | Module                          | Responsibility                                    |
|-------|----------------------------------|----------------------------------------------------|
| 07    | internal/audio                   | Microphone capture, PCM buffer (malgo)            |
| 08    | internal/vad                     | Voice activity detection (streaming RMS-VAD)      |
| 09    | internal/transcribe               | Audio → text via Groq Whisper                     |
| 10    | internal/clipboard                | Write text to clipboard, paste (atotto/robotgo)   |
| 11    | frontend / app.go                 | Wails bindings, status events                     |
| 12    | internal/focus, internal/pipeline | Focus capture/restore (cgo+AppKit), pipeline wiring |
| 13    | internal/hotkey                   | Global hold-to-talk hotkey, usage-session timing (golang.design/x/hotkey) |

## Repository Structure

```
ontext/
├── app/ontext-wails/     # Wails app (Go backend + React frontend)
│   ├── frontend/         # React frontend
│   └── internal/         # Go packages: audio, vad, transcribe, clipboard, focus, pipeline
├── docs/                 # Additional documentation
├── tasks/                # Agent task assignments
└── gate-outs/            # Stage completion reports
```

## Status

**Migration complete:** the runtime has been migrated from Tauri/Rust to
Wails/Go (see [ADR 009](docs/adrs/009-migrate-tauri-rust-to-wails-go.md) and
DECISIONS.md). Stages M0 and 07-12 are done — the old Tauri app
(`app/ontext/`), root `Cargo.toml` workspace, and Rust `modules/*` crates
have been removed as of stage 12's cutover.

Stage 13 (hotkey, Go) is READY — see PIPELINE.md and ADR 010.

iOS/Android were dropped from target platforms during the Wails migration
(Wails does not support mobile) — see DECISIONS.md.
