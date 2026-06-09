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

Each stage must produce `gate-outs/<stage>.md` before the next stage starts.

```
Status: PASS | FAIL
Stage:
Domain:
Summary:
Modified Files:
  - file1
  - file2
Dependencies Added:
  - none
Tests:
  - test_name
Acceptance Criteria:
  - Requirement 1
Known Issues:
  - none
Ready For Next Stage: YES | NO
```
