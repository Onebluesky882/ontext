# CONTRACTS.md

This file is the source of truth for all module interfaces.
Input and output types must match exactly.
Do not change contracts without orchestrator approval.

---

## audio.Frame

```go
type Frame struct {
    Samples    []float32
    SampleRate int // always 16000
}
```

---

## denoise.Denoiser

```go
type Denoiser interface {
    Denoise(frame audio.Frame) audio.Frame
}
```

`Denoise` returns an `audio.Frame` with the same `len(Samples)` and
`SampleRate` as the input — only sample values may change. Implementations
must not panic; on RNNoise init/runtime failure, return the input `frame`
unchanged (fail-open).

---

## vad.Segment

```go
type Segment struct {
    Samples    []float32
    SampleRate int
}
```

---

## transcribe.Result

```go
type Result struct {
    Text             string
    Language         string
    NoSpeechProb     float32
    AvgLogprob       float32
    CompressionRatio float32
}
```

---

## autocorrect.Corrector

```go
type Corrector interface {
    Correct(ctx context.Context, text string) (string, error)
}
```

`Correct` fixes spelling/grammar/punctuation only — no rephrasing, no
added/removed meaning, no commentary. On error, timeout, or empty response,
callers must fall back to the original text (fail-open); `Correct` itself
returns the error so the pipeline can decide, but must never panic.

---

## clipboard.Writer

```go
type Writer interface {
    Paste(ctx context.Context, text string) error
}
```

`Paste` returns `nil` on success, never panics. On failure it returns a
descriptive error (`fmt.Errorf`), never an empty-string error.

---

## Pipeline Contracts

| Module      | Input               | Output                                            |
|-------------|---------------------|----------------------------------------------------|
| audio       | Start/Stop signal   | `<-chan audio.Frame`                               |
| denoise     | `audio.Frame`       | `audio.Frame` (denoised, same shape)               |
| vad         | `<-chan audio.Frame`| `<-chan vad.Segment`                               |
| transcribe  | `vad.Segment`       | `transcribe.Result`                                |
| autocorrect | `transcribe.Result.Text` (string) | corrected text (string)              |
| focus       | —                   | last focused app bundle id; `Activate(bundleID)`   |
| clipboard   | text (string)       | `error`                                            |

---

## Wails Bindings (Go <-> TypeScript)

`app.go` exposes bound methods called from the frontend via the generated
`wailsjs/go/main/App` bindings:

- `StartPipeline() PasteResult`
- `StopRecording() error`
- `RequestAccessibilityPermission() error`

```go
type PasteResult struct {
    Success bool   `json:"success"`
    Error   string `json:"error,omitempty"`
}
```

Status updates are pushed via `runtime.EventsEmit(ctx, "status", string(status))`
and consumed in the frontend via `wailsjs/runtime` `EventsOn("status", ...)`.

Field name mapping (Go -> TypeScript):

| Go              | TypeScript      |
|-----------------|------------------|
| `SampleRate`    | `sampleRate`     |
| `Success`       | `success`        |
| `Error`         | `error`          |

---

## Rules

- `SampleRate` is always `16000`. Do not change.
- `transcribe.Result.Text` must be trimmed (no leading/trailing whitespace).
- `PasteResult.Error` must be omitted on success (`json:",omitempty"`), never
  present as an empty string.
- All JSON fields exposed to the frontend use `lowerCamelCase` via Go struct
  tags (`json:"..."`).
- If a contract appears incorrect: STOP and report. Do not invent a new
  contract.
