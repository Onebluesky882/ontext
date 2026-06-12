# ARCHITECTURE.md

## Overview

ontext uses a pipeline architecture.
Each module is a discrete Go package under `app/ontext-wails/internal/`.
Modules communicate through defined contracts (see CONTRACTS.md).
The Wails Go backend (`app.go` + `internal/pipeline`) orchestrates the pipeline.
The React frontend handles UI state only.

## System Diagram

```
┌─────────────────────────────────────────────────┐
│              Wails Backend (Go)                  │
│                                                  │
│  ┌──────────┐    ┌───────────┐                  │
│  │  audio   │───▶│    vad    │                  │
│  └──────────┘    └─────┬─────┘                  │
│                        │                         │
│               ┌────────▼──────┐                  │
│               │  transcribe   │                  │
│               └────────┬──────┘                  │
│                        │                         │
│               ┌────────▼──────┐    ┌──────────┐  │
│               │  clipboard    │◀───│  focus   │  │
│               └───────────────┘    └──────────┘  │
└─────────────────────────────────────────────────┘
         ▲                              │
         │ Wails bindings/events        │
         ▼                              ▼
┌─────────────────────┐     ┌──────────────────────┐
│  React Frontend     │     │  Active App (paste)  │
│  (UI / status only) │     │  (macOS/Windows)     │
└─────────────────────┘     └──────────────────────┘
```

## Module Responsibilities

### internal/audio
- Open microphone on `StartPipeline`
- Stream PCM audio (16kHz, mono, f32)
- Stop on `StopRecording`
- Output: `audio.Frame { Samples []float32, SampleRate int }`
- Library: `gen2brain/malgo`

### internal/vad
- Input: `<-chan audio.Frame`
- Detect speech segments, discard silence
- Output: `<-chan vad.Segment` (non-silent segments only)
- Streaming RMS-VAD (ported from Rust, no external dependency)

### internal/transcribe
- Input: `vad.Segment`
- Send to Groq Whisper API
- Output: `transcribe.Result { Text, Language, NoSpeechProb, AvgLogprob, CompressionRatio }`
- Filters likely hallucinations via `Result.IsLikelyHallucination()`

### internal/autocorrect
- Input: `transcribe.Result.Text` (string)
- Send to Groq chat-completion API (small/fast instruction model) to fix
  spelling/grammar/punctuation only — no rephrasing
- Output: corrected text (string)
- Fail-open: on error/timeout/empty response, falls back to the original
  text unchanged

### internal/focus
- Tracks the frontmost application (excluding ontext itself) via cgo +
  AppKit/CoreFoundation (macOS only; no-op on other platforms)
- Reactivates that app before each paste, with a settle delay
  (`focus.SettleDelay`)
- Exposes Accessibility-permission check/prompt
  (`AXIsProcessTrusted` / `AXIsProcessTrustedWithOptions`)

### internal/clipboard
- Input: text (string) from `transcribe.Result`
- Write `text` to system clipboard (`atotto/clipboard`)
- Simulate paste (`Cmd+V` / `Ctrl+V`) into active window (`go-vgo/robotgo`)
- Output: `error` (nil on success)

### internal/pipeline
- Wires audio -> vad -> transcribe -> autocorrect -> focus -> clipboard together
- Each transcribed segment is pasted immediately (real-time, not buffered
  until Stop)
- Emits `Status` (idle/running/done/error) via `OnStatus` callback, wired to
  `runtime.EventsEmit` in `app.go`

## Frontend (React)

- Shows recording status (idle / recording / transcribing)
- Shows last transcript
- Settings: Whisper API key, language
- Does NOT implement any pipeline logic
- Calls bound Go methods (`App.StartPipeline`, `App.StopRecording`,
  `App.RequestAccessibilityPermission`) and subscribes to `status` events via
  `wailsjs/runtime` `EventsOn`

## Platform Notes

| Platform | Audio API | Paste API        | Focus tracking            |
|----------|-----------|-------------------|----------------------------|
| macOS    | CoreAudio (malgo) | CGEvent (robotgo) | cgo + AppKit/CoreFoundation |
| Windows  | WASAPI (malgo)    | SendInput (robotgo) | no-op (not needed)        |

## Data Flow (Sequence)

```
1. User clicks Start → App.StartPipeline()
2. audio.Capturer.Start() → streams audio.Frame on a channel
3. vad.Detector.Detect(frames) → streams vad.Segment on a channel
4. For each segment:
   a. transcribe.Transcriber.Transcribe(segment) → transcribe.Result
   b. if not empty/hallucination:
      i.  autocorrect.Corrector.Correct(result.Text) → corrected text
          (falls back to result.Text on error)
      ii. focus.Manager.Activate(lastFocusedApp)
   c. clipboard.Writer.Paste(correctedText) → pastes into active app
5. Pipeline emits Status via OnStatus → runtime.EventsEmit("status", ...)
6. User clicks Stop → App.StopRecording() cancels the session
```
