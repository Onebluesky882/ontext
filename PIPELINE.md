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
`gate-outs/stage-0X-<name>.md` before the next stage begins, same as stages
1-6 above. Migration stages continue the same numbering sequence
(stage-07 onward) — only stage M0 (already in progress) keeps its `M0` name.

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

### Stage 07 — audio (Go)

Domain: `app/ontext-wails/internal/audio` (Go package)
Branch: `feature/stage-07-audio-go`

Goal:
Port `modules/audio` (cpal-based capture) to Go using `malgo`. Same
input/output contract as original Stage 2. The package skeleton already
exists (`audio.go` defines the `Capturer` interface and `Frame` type,
`noop.go` is a placeholder) — implement a `malgo`-backed `Capturer` to
replace `NoopCapturer`.

Input: Start/Stop signal from UI (Wails-bound method call)

Output: `audio.Frame { Samples []float32, SampleRate int }` streamed on a channel

Constraints:
- Sample rate: 16000 Hz, mono, f32 — unchanged (CONTRACTS.md)

Acceptance Criteria:
- Microphone opens on Start, closes on Stop
- Output frames are 16kHz mono f32
- Unit tests pass
- Build passes on macOS and Windows

---

### Stage 08 — vad (Go)

Domain: `app/ontext-wails/internal/vad` (Go package)
Branch: `feature/stage-08-vad-go`

Goal:
Port the streaming RMS-VAD implementation from `modules/vad` to Go. Same
input/output contract as original Stage 3. The package skeleton already
exists (`vad.go` defines the `Detector` interface and `Segment` type,
`noop.go` is a placeholder) — implement an RMS-based `Detector` to replace
`NoopDetector`.

Input: `<-chan audio.Frame`

Output: `<-chan vad.Segment`

Acceptance Criteria:
- Silent segments removed, speech segments preserved with correct timestamps
- Empty/closed input channel yields a closed output channel (no panic)
- Unit tests pass with the same fixtures used in the Rust version

---

### Stage 09 — transcribe (Go)

Domain: `app/ontext-wails/internal/transcribe` (Go package)
Branch: `feature/stage-09-transcribe-go`

Status: Largely implemented already — `groq.go` (GroqTranscriber using
`net/http`, model `whisper-large-v3`), `wav.go` (PCM→WAV encoding), and
`groq_test.go` (live API test against `GROQ` env var) exist and pass.
This stage is to review/finish that implementation and produce its
gate-out.

Goal:
Confirm the Groq Whisper API integration meets the original Stage 4
contract, add any missing unit tests (mock HTTP server for error/timeout
cases), and verify hallucination-filtering thresholds
(`IsLikelyHallucination`) match `modules/transcribe` (Rust).

Input: `vad.Segment`

Output: `transcribe.Result { Text string, Language string, NoSpeechProb, AvgLogprob, CompressionRatio float32 }`

Constraints:
- API base/model: Groq `whisper-large-v3` (confirm vs `-turbo`; see
  DECISIONS.md and gate-out for any model change rationale), language=th
- Return Go `error`, never panic, on API failure

Acceptance Criteria:
- Thai and English speech return correct text
- API timeout returns error, not panic
- Unit tests pass (mock HTTP server allowed; live test gated on `GROQ` env var)

---

### Stage 10 — clipboard (Go)

Domain: `app/ontext-wails/internal/clipboard` (Go package)
Branch: `feature/stage-10-clipboard-go`

Goal:
Port clipboard write + paste simulation to Go using `atotto/clipboard` +
`go-vgo/robotgo`. Same input/output contract as original Stage 5. The
package skeleton already exists (`clipboard.go` defines the `Writer`
interface, `noop.go` is a placeholder) — implement a real `Writer` to
replace `NoopWriter`.

Input: text (string) from `transcribe.Result`

Output: `error` (nil on success)

Constraints:
- macOS: Cmd+V simulation; Windows: Ctrl+V simulation
- Never panic; return descriptive error on failure

Acceptance Criteria:
- Text appears in focused input after paste
- Failure returns descriptive error, not panic
- Unit tests pass

---

### Stage 11 — frontend Wails bindings

Domain: `app/ontext-wails/frontend/src`, `app/ontext-wails/app.go`
Branch: `feature/stage-11-frontend-bindings`

Goal:
Replace the Tauri `invoke`/`plugin-opener` calls carried over from
`app/ontext/src/` with Wails method bindings and runtime APIs:
- `src/hooks/usePipeline.ts`: replace `invoke('start_pipeline')` /
  `invoke('stop_recording')` with bound `App.StartPipeline` /
  `App.StopRecording` methods (add these to `app.go`, regenerate
  `wailsjs` bindings)
- `src/pages/onboarding/PermissionStep.tsx`: replace
  `invoke('request_accessibility_permission')` and `openUrl` (from
  `@tauri-apps/plugin-opener`) with a bound Go method +
  `runtime.BrowserOpenURL`
- `src/components/NavBar.tsx`: replace `data-tauri-drag-region` with
  Wails' `--wails-draggable: drag` CSS
- Remove `@tauri-apps/api` and `@tauri-apps/plugin-opener` from
  `frontend/package.json` once no longer referenced
- Wire `pipeline.Pipeline.OnStatus` to `runtime.EventsEmit`, and update
  the frontend store/hooks to subscribe via `wailsjs/runtime` `EventsOn`
  instead of Tauri's `listen()`

Input: none (frontend refactor)

Output: frontend builds against Wails bindings only, no `@tauri-apps/*` imports

Acceptance Criteria:
- `tsc` and `vite build` pass with no `@tauri-apps/*` dependencies
- `wails dev` renders UI; Start/Stop buttons call bound Go methods
- Status updates received via Wails events update the UI
- Unit/build checks pass

---

### Stage 12 — focus-paste (Go) + cutover

Domain: `app/ontext-wails` (Wails Go backend), repo root (`Cargo.toml`,
`app/ontext/`)
Branch: `feature/stage-12-focus-paste-cutover`

Goal:
Port the macOS focus capture/restoration logic from stage 6
(`gate-outs/stage-06-focus-paste.md`) to Go via `cgo` + AppKit/CoreFoundation
shims. Wire stages 07-10 (audio/vad/transcribe/clipboard) and stage 11
(frontend bindings) together into the `pipeline.Pipeline` used by
`app.go`, matching the per-segment real-time paste behavior from stage 6.
On PASS, remove `app/ontext/src-tauri`, `app/ontext/` Tauri frontend
remnants, the root `Cargo.toml` workspace, and Rust `modules/*` crates.

Input: UI Start/Stop calls (via stage 11 bindings)

Output: `PasteResult`-equivalent per segment, real-time

Acceptance Criteria:
- Frontmost app captured and tracked continuously (cgo AppKit equivalent of
  `NSWorkspace.frontmostApplication`)
- Target app reactivated before each paste, with settle delay
- Each transcribed segment pasted immediately (no buffering until Stop)
- Non-macOS builds unaffected (focus code behind Go build tags)
- Unit tests pass; build passes on macOS and Windows
- Old Tauri/Rust app and Cargo workspace removed

---

### Stage 13 — hotkey (Go)

Domain: `app/ontext-wails/internal/hotkey` (new Go package)
Branch: `feature/stage-13-hotkey-go`

Status: PROPOSED — conflicts with the existing decision recorded earlier in
this file ("`modules/hotkey` is dropped, dead code, not ported") and with
PROJECT.md's note that hotkey-driven recording was replaced by
button-driven start/stop after `rdev` crashes on macOS without
Accessibility permission. Before starting this stage:
- Confirm with the orchestrator whether global-hotkey start/stop should be
  reintroduced for the Wails app
- If yes, update DECISIONS.md and PROJECT.md to reflect the reversal and
  document the chosen Go hotkey library (e.g. `golang.design/x/hotkey`) and
  how it avoids the prior `rdev`/Accessibility crash
- If no, this stage is dropped — do not implement

Goal (if confirmed):
Implement a global hotkey listener in Go that emits Start/Stop signals to
`pipeline.Pipeline`, gracefully handling missing Accessibility permission
on macOS (no crash — degrade to button-only start/stop).

Input: none

Output: Start/Stop signal into `pipeline.Pipeline`

Acceptance Criteria:
- Hotkey press/release starts/stops the pipeline
- Missing Accessibility permission does not crash the app — falls back to
  button-driven start/stop with a status message
- Unit tests pass; build passes on macOS and Windows

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
