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
