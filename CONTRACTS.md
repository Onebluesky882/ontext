# CONTRACTS.md

This file is the source of truth for all module interfaces.
Input and output types must match exactly.
Do not change contracts without orchestrator approval.

---

## HotkeyEvent

```rust
pub enum HotkeyEvent {
    Start,
    Stop,
}
```

---

## AudioBuffer

```rust
pub struct AudioBuffer {
    pub samples: Vec<f32>,
    pub sample_rate: u32,  // always 16000
}
```

---

## AudioChunk

```rust
pub struct AudioChunk {
    pub samples: Vec<f32>,
    pub start_ms: u64,
    pub end_ms: u64,
}
```

---

## TranscriptResult

```rust
pub struct TranscriptResult {
    pub text: String,
    pub language: String,
}
```

---

## PasteResult

```rust
pub struct PasteResult {
    pub success: bool,
    pub error: Option<String>,
}
```

---

## Pipeline Contracts

| Module      | Input               | Output                  |
|-------------|---------------------|-------------------------|
| hotkey      | —                   | `HotkeyEvent`           |
| audio       | `HotkeyEvent`       | `AudioBuffer`           |
| vad         | `AudioBuffer`       | `Vec<AudioChunk>`       |
| transcribe  | `Vec<AudioChunk>`   | `TranscriptResult`      |
| clipboard   | `TranscriptResult`  | `PasteResult`           |

---

## Serde / Tauri IPC Serialization

All Rust structs passed through Tauri IPC must derive Serialize and Deserialize
and use `rename_all = "camelCase"` so field names match the TypeScript types.

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioBuffer {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
}
```

Field name mapping (Rust → TypeScript):

| Rust (snake_case) | TypeScript (camelCase) |
|-------------------|------------------------|
| `sample_rate`     | `sampleRate`           |
| `start_ms`        | `startMs`              |
| `end_ms`          | `endMs`                |

Do not skip `#[serde(rename_all = "camelCase")]`. Missing it will break Tauri IPC silently.

---

## Rules

- `sample_rate` is always `16000`. Do not change.
- `text` in `TranscriptResult` must be trimmed (no leading/trailing whitespace).
- `PasteResult.error` must be `None` on success, never an empty string.
- All structs sent over Tauri IPC must use `#[serde(rename_all = "camelCase")]`.
- If a contract appears incorrect: STOP and report. Do not invent a new contract.
