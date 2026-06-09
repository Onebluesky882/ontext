# 002 — Use webrtc-vad for Voice Activity Detection

Status: Accepted
Date: 2026-06-09

---

## Context

ontext records audio while the hotkey is held.
Raw audio includes silence before and after speech.
Sending silence to Whisper wastes API calls and increases latency.
A VAD (Voice Activity Detection) layer is needed to strip silence.

## Decision

Use the `webrtc-vad` crate for voice activity detection.

```toml
webrtc-vad = "0.4"
```

## Reasons

- Lightweight — no ML model, no ONNX runtime
- Fast — deterministic signal processing
- Proven — same algorithm used in WebRTC (Chrome, Firefox)
- Works on 16kHz audio natively (matches our audio format)

## Consequences

- Less accurate than ML-based VAD on noisy environments
- Aggressiveness level (0–3) must be tuned — default: 2
- Frames must be exactly 10ms, 20ms, or 30ms — audio module must chunk correctly

## Alternatives Considered

| Library       | Reason Rejected                                      |
|---------------|------------------------------------------------------|
| silero-vad    | Requires ONNX runtime — heavy dependency             |
| py-webrtcvad  | Python only                                          |
| no VAD        | Sends silence to API — wastes cost, slower response  |
