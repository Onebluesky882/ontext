# 003 — Use OpenAI Whisper API for Transcription

Status: Accepted
Date: 2026-06-09

---

## Context

ontext needs to convert speech to text accurately.
Must support Thai and English at minimum.
Must work cross-platform without local model setup.

## Decision

Use the OpenAI Whisper API as the default transcription backend.

```
Endpoint: https://api.openai.com/v1/audio/transcriptions
Model: whisper-1
Format: audio/wav (16kHz mono f32 PCM)
```

## Reasons

- High accuracy for Thai and English
- No local model required — works on all platforms including mobile
- Simple REST API — easy to implement in Rust with reqwest
- Supports language detection automatically

## Consequences

- Requires internet connection
- Requires OPENAI_API_KEY — stored in app settings, never hardcoded
- API cost per request (~$0.006/min audio)
- Latency ~1–3 seconds depending on audio length

## Future Option: Local Model

If offline support is needed, switch to `whisper.cpp` with a local `.bin` model.
Document the switch as a new ADR when that decision is made.
Local model files go in: `models/whisper/`

## Alternatives Considered

| Option              | Reason Rejected                                         |
|---------------------|---------------------------------------------------------|
| Google Speech-to-Text | Lower Thai accuracy, more complex auth               |
| Azure Speech        | Higher cost, more setup                                  |
| whisper.cpp (local) | Large model files, not suitable for mobile default      |
| DeepSpeech          | Thai not supported                                       |
