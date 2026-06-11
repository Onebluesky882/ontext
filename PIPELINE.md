# PIPELINE.md

Each stage must be completed and gate-out submitted before the next stage begins.
The orchestrator controls stage progression.

---

## Stage 1 — hotkey

Domain: `modules/hotkey`
Branch: `feature/hotkey`

Goal:
Detect global hotkey press and release. Emit events to the pipeline.

Input: none

Output: `HotkeyEvent` (Start | Stop)

Default hotkey:
- macOS: `Cmd+Shift+Space`
- Windows: `Ctrl+Shift+Space`

Acceptance Criteria:
- Hotkey press emits `HotkeyEvent::Start`
- Hotkey release emits `HotkeyEvent::Stop`
- Works while another app is in focus
- Unit tests pass
- Build passes on macOS and Windows

---

## Stage 2 — audio

Domain: `modules/audio`
Branch: `feature/audio`

Goal:
Open microphone on `HotkeyEvent::Start`. Stream PCM audio. Stop on `HotkeyEvent::Stop`.

Input: `HotkeyEvent`

Output: `AudioBuffer { samples: Vec<f32>, sample_rate: u32 }`

Constraints:
- Sample rate: 16000 Hz
- Channels: mono
- Format: f32

Acceptance Criteria:
- Microphone opens on Start event
- Microphone closes on Stop event
- Output buffer is 16kHz mono f32
- Unit tests pass
- Build passes on macOS and Windows

---

## Stage 3 — vad

Domain: `modules/vad`
Branch: `feature/vad`

Goal:
Remove silence from `AudioBuffer`. Return only speech segments.

Input: `AudioBuffer`

Output: `Vec<AudioChunk>`

Acceptance Criteria:
- Silent segments are removed
- Speech segments are preserved with correct timestamps
- Empty input returns empty Vec (no panic)
- Unit tests pass with sample audio fixtures

---

## Stage 4 — transcribe

Domain: `modules/transcribe`
Branch: `feature/transcribe`

Goal:
Send audio chunks to Whisper API. Return transcript text.

Input: `Vec<AudioChunk>`

Output: `TranscriptResult { text: String, language: String }`

Constraints:
- API: OpenAI Whisper (`whisper-1`)
- text must be trimmed
- Return structured error on API failure (do not panic)

Acceptance Criteria:
- Thai speech returns correct Thai text
- English speech returns correct English text
- API timeout returns error, not panic
- Unit tests pass (mock API allowed for unit tests)

---

## Stage 5 — clipboard

Domain: `modules/clipboard`
Branch: `feature/clipboard`

Goal:
Write transcript text to clipboard and paste into the active input field.

Input: `TranscriptResult`

Output: `PasteResult { success: bool, error: Option<String> }`

Constraints:
- macOS: use `Cmd+V` simulation
- Windows: use `Ctrl+V` simulation
- `error` must be `None` on success (never empty string)

Acceptance Criteria:
- Text appears in focused input after pipeline completes
- `PasteResult.success` is `true` on success
- Failure returns descriptive error string
- Unit tests pass

---

## Migration Pipeline — Tauri/Rust → Wails/Go

See [ADR 009](docs/adrs/009-migrate-tauri-rust-to-wails-go.md) and the
updated `DECISIONS.md`. Each migration stage must produce
`gate-outs/stage-MX-<name>.md` before the next stage begins, same as stages
1-5 above. `modules/hotkey` is dropped (dead code, not ported).

---

### Stage M0 — wails-bootstrap

Domain: `app/ontext` (new Wails project)
Branch: `feature/wails-bootstrap`

Goal:
Scaffold a Wails v2 project alongside the existing Tauri app. Move the React
+ Vite frontend (`app/ontext/src/`) into the Wails project's `frontend/`
without behavior changes, and add Tailwind CSS (new — see DECISIONS.md).
Verify the frontend renders inside the Wails webview with a single
placeholder Go-bound method (e.g. `Greet(name string) string`).

Input: none

Output: running Wails dev build showing the existing UI shell

Acceptance Criteria:
- `wails dev` launches and renders the existing React UI
- One Go method is bound and callable from the frontend
- Existing Tauri app (`src-tauri`) untouched and still builds (no regressions
  until cutover)
- Build passes on macOS

---

### Stage M1 — audio (Go)

Domain: `modules/audio` (Go package)
Branch: `feature/audio-go`

Goal:
Port `modules/audio` (cpal-based capture) to Go using `malgo`. Same
input/output contract as original Stage 2.

Input: Start/Stop signal from UI (Wails-bound method call)

Output: `AudioBuffer { Samples []float32, SampleRate uint32 }`

Constraints:
- Sample rate: 16000 Hz, mono, f32 — unchanged (CONTRACTS.md)

Acceptance Criteria:
- Microphone opens on Start, closes on Stop
- Output buffer is 16kHz mono f32
- Unit tests pass
- Build passes on macOS and Windows

---

### Stage M2 — vad (Go)

Domain: `modules/vad` (Go package)
Branch: `feature/vad-go`

Goal:
Port the streaming RMS-VAD implementation from `modules/vad` to Go. Same
input/output contract as original Stage 3.

Input: `AudioBuffer`

Output: `[]AudioChunk`

Acceptance Criteria:
- Silent segments removed, speech segments preserved with correct timestamps
- Empty input returns empty slice (no panic)
- Unit tests pass with the same fixtures used in the Rust version

---

### Stage M3 — transcribe (Go)

Domain: `modules/transcribe` (Go package)
Branch: `feature/transcribe-go`

Goal:
Port Groq Whisper API integration to Go using `net/http`. Same
input/output contract as original Stage 4.

Input: `[]AudioChunk`

Output: `TranscriptResult { Text string, Language string }`

Constraints:
- API/model unchanged: Groq `whisper-large-v3-turbo`, language=th
- Return Go `error`, never panic, on API failure

Acceptance Criteria:
- Thai and English speech return correct text
- API timeout returns error, not panic
- Unit tests pass (mock HTTP server allowed)

---

### Stage M4 — clipboard (Go)

Domain: `modules/clipboard` (Go package)
Branch: `feature/clipboard-go`

Goal:
Port clipboard write + paste simulation to Go using `atotto/clipboard` +
`go-vgo/robotgo`. Same input/output contract as original Stage 5.

Input: `TranscriptResult`

Output: `PasteResult { Success bool, Error string }`

Constraints:
- macOS: Cmd+V simulation; Windows: Ctrl+V simulation
- `Error` empty string on success

Acceptance Criteria:
- Text appears in focused input after paste
- `PasteResult.Success` is `true` on success
- Failure returns descriptive error string
- Unit tests pass

---

### Stage M5 — focus-paste (Go) + cutover

Domain: `app/ontext` (Wails Go backend)
Branch: `feature/focus-paste-go`

Goal:
Port the macOS focus capture/restoration logic from stage 6
(`gate-outs/stage-06-focus-paste.md`) to Go via `cgo` + AppKit/CoreFoundation
shims. Wire M1-M4 modules into the Wails app's record/transcribe flow,
matching the per-segment real-time paste behavior from stage 6. On PASS,
remove `app/ontext/src-tauri`, `Cargo.toml` workspace, and Rust
`modules/*` crates.

Input: `HotkeyEvent`-equivalent UI Start/Stop calls

Output: `PasteResult` per segment, real-time

Acceptance Criteria:
- Frontmost app captured and tracked continuously (cgo AppKit equivalent of
  `NSWorkspace.frontmostApplication`)
- Target app reactivated before each paste, with settle delay
- Each transcribed segment pasted immediately (no buffering until Stop)
- Non-macOS builds unaffected (focus code behind Go build tags)
- Unit tests pass; build passes
- Old Tauri/Rust app and Cargo workspace removed

---

## Gate-Out Format

Each stage must produce `gate-outs/stage-0X-<name>.md` before the next stage starts.
The conductor parses this file to verify completion and trigger the next stage.

```
---
status: PASS
stage: 01
domain: modules/hotkey
branch: feature/hotkey
assigned_to: claude-sonnet-4-6
completed_at: 2026-06-09
ready_for_next: YES
---

summary: implemented global hotkey detection using tauri-plugin-global-shortcut

modified_files:
  - modules/hotkey/src/lib.rs
  - modules/hotkey/Cargo.toml

dependencies_added:
  - tauri-plugin-global-shortcut@2.0.0

tests:
  - test_hotkey_start_emits_event
  - test_hotkey_stop_emits_event

acceptance_criteria:
  - PASS: Hotkey press emits HotkeyEvent::Start
  - PASS: Hotkey release emits HotkeyEvent::Stop
  - PASS: Works while another app is in focus

known_issues:
  - none

recommendations:
  - lib.rs needs to register hotkey module (orchestrator action)
```

Field rules:
- `status`: PASS or FAIL only
- `ready_for_next`: YES or NO only
- All list items use `-` prefix
- Empty fields must say `none`
