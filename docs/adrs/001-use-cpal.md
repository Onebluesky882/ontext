# 001 — Use cpal for Audio Capture

Status: Accepted
Date: 2026-06-09

---

## Context

ontext needs to capture microphone audio on macOS, Windows, iOS, and Android.
The audio must be 16kHz mono f32 PCM (Whisper requirement).
The solution must work from Rust (Tauri backend).

## Decision

Use the `cpal` crate for cross-platform audio capture.

```toml
cpal = "0.15"
```

## Reasons

- Cross-platform: macOS (CoreAudio), Windows (WASAPI), iOS (AVAudioSession), Android (AAudio/OpenSLES)
- Pure Rust — no C++ bindings required
- Supports f32 sample format natively
- Actively maintained

## Consequences

- Must handle platform-specific stream config (sample rate resampling if device default != 16kHz)
- iOS/Android require runtime microphone permission — handled by Tauri plugin layer

## Alternatives Considered

| Library     | Reason Rejected                          |
|-------------|------------------------------------------|
| rodio       | Playback-focused, capture support limited |
| portaudio   | C binding, cross-platform issues on mobile |
| platform APIs directly | Too much per-platform boilerplate |
