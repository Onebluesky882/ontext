# ARCHITECTURE.md

## Overview

ontext uses a pipeline architecture.
Each module is a discrete Rust crate under `modules/`.
Modules communicate through defined contracts (see CONTRACTS.md).
The Tauri backend orchestrates the pipeline.
The React frontend handles UI state only.

## System Diagram

```
┌─────────────────────────────────────────────────┐
│                  Tauri Backend (Rust)            │
│                                                  │
│  ┌──────────┐    ┌──────────┐    ┌───────────┐  │
│  │  hotkey  │───▶│  audio   │───▶│    vad    │  │
│  └──────────┘    └──────────┘    └─────┬─────┘  │
│                                        │         │
│                               ┌────────▼──────┐  │
│                               │  transcribe   │  │
│                               └────────┬──────┘  │
│                                        │         │
│                               ┌────────▼──────┐  │
│                               │  clipboard    │  │
│                               └───────────────┘  │
└─────────────────────────────────────────────────┘
         ▲                              │
         │ Tauri IPC (invoke/event)     │
         ▼                              ▼
┌─────────────────────┐     ┌──────────────────────┐
│  React Frontend     │     │  Active App (paste)  │
│  (UI / status only) │     │  (macOS/Win/iOS/And) │
└─────────────────────┘     └──────────────────────┘
```

## Module Responsibilities

### modules/hotkey
- Listen for global hotkey (default: `Ctrl+Shift+Space` / `Cmd+Shift+Space`)
- Emit `hotkey:start` event on press
- Emit `hotkey:stop` event on release
- Plugin: `tauri-plugin-global-shortcut`

### modules/audio
- Open microphone on `hotkey:start`
- Stream PCM audio (16kHz, mono, f32)
- Stop on `hotkey:stop`
- Output: `AudioBuffer { samples: Vec<f32>, sample_rate: u32 }`

### modules/vad
- Input: `AudioBuffer`
- Detect speech segments, discard silence
- Output: `Vec<AudioChunk>` (non-silent segments only)
- Library: `webrtc-vad` or `silero-vad`

### modules/transcribe
- Input: `Vec<AudioChunk>`
- Send to Whisper API (or local model)
- Output: `TranscriptResult { text: String, language: String }`

### modules/clipboard
- Input: `TranscriptResult`
- Write `text` to system clipboard
- Simulate paste (`Cmd+V` / `Ctrl+V`) into active window
- Output: `PasteResult { success: bool }`

## Frontend (React)

- Shows recording status (idle / recording / transcribing)
- Shows last transcript
- Settings: hotkey config, Whisper API key, language
- Does NOT implement any pipeline logic

## Platform Notes

| Platform | Hotkey API              | Audio API      | Paste API         |
|----------|-------------------------|----------------|-------------------|
| macOS    | CGEventTap / tauri      | CoreAudio      | CGEvent           |
| Windows  | RegisterHotKey / tauri  | WASAPI         | SendInput         |
| iOS      | Long press button       | AVAudioEngine  | UIPasteboard      |
| Android  | Floating button         | AudioRecord    | ClipboardManager  |

## Data Flow (Sequence)

```
1. User holds hotkey
2. hotkey::listen() → emits HotkeyEvent::Start
3. audio::start_capture() → streams AudioBuffer
4. vad::process(buffer) → returns Vec<AudioChunk>
5. transcribe::run(chunks) → returns TranscriptResult
6. clipboard::paste(result.text) → pastes into active app
7. Frontend receives status updates via Tauri events
```
