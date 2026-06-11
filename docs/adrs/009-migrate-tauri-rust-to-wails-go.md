# ADR 009 — Migrate Runtime from Tauri (Rust) to Wails (Go)

## Status

Accepted

## Date

2026-06-11

## Context

ontext currently runs on Tauri 2 with a Rust backend (`app/ontext/src-tauri`)
and five Rust workspace modules (`modules/hotkey`, `modules/audio`,
`modules/vad`, `modules/transcribe`, `modules/clipboard`). Stages 1-6 are
complete and gated (see `gate-outs/`).

The team has decided to move the desktop runtime to Wails, with Go replacing
Rust as the backend language. The React 19 + TypeScript + Vite frontend is
retained — Wails, like Tauri, embeds a system webview and binds frontend JS
calls to backend methods, so the existing `app/ontext/src/` UI can be reused
largely as-is.

This is a full backend rewrite. ~1,700 lines of Rust across `src-tauri/lib.rs`
and the five `modules/*` crates need Go equivalents, including macOS-specific
AppKit/CoreFoundation bindings (focus tracking, Accessibility permission
prompt) currently implemented via `objc2`/`objc2-app-kit`/`objc2-foundation`.

## Decision

Replace the Rust/Tauri backend with a Go/Wails backend.

**Runtime:** Wails v2
**Backend language:** Go (1.22+)
**Frontend:** unchanged — React 19 + TypeScript + Vite, served via Wails' embedded webview

**Go library mapping for ported modules:**

| Rust (current)                          | Go (replacement)                                  | Module          |
|------------------------------------------|----------------------------------------------------|-----------------|
| `cpal`                                    | `github.com/gen2brain/malgo` (miniaudio bindings)  | audio           |
| `webrtc-vad` / streaming RMS-VAD (custom) | port RMS-VAD logic directly to Go (no new dep)     | vad             |
| `reqwest` → Groq Whisper HTTP API         | `net/http` (stdlib)                                 | transcribe      |
| `arboard` (clipboard write)               | `github.com/atotto/clipboard`                       | clipboard       |
| `enigo` (Cmd+V / Ctrl+V simulation)       | `github.com/go-vgo/robotgo`                         | clipboard       |
| `objc2` / `objc2-app-kit` (focus capture, AX permission) | `cgo` + AppKit/CoreFoundation, or `robotgo` where coverage exists | clipboard / focus |
| `tauri-plugin-global-shortcut` (unused, dead code) | not ported — hotkey module remains retired, see PROJECT.md |

**Repository layout (target):**

```
ontext/
├── app/ontext/           # Wails app
│   ├── frontend/         # React frontend (moved from src/, same code)
│   └── (Go backend at module root: app.go, main.go, wails.json)
├── modules/              # Go packages, one per stage
│   ├── audio/
│   ├── vad/
│   ├── transcribe/
│   └── clipboard/
├── docs/
├── tasks/
└── gate-outs/
```

`modules/hotkey` is dropped from the workspace (already dead code per
PROJECT.md / DECISIONS.md — not ported).

**Migration pipeline:** see `PIPELINE.md` — new stages M1-M5 added, mapping
1:1 to the original stages 2-6 (audio, vad, transcribe, clipboard,
focus/paste), plus M0 for Wails project bootstrap.

## Consequences

- `Cargo.toml` workspace, `modules/*` Rust crates, and `app/ontext/src-tauri`
  are removed once each corresponding Go module passes its gate-out.
- `DECISIONS.md` is updated: Tauri/Rust runtime decision marked superseded by
  this ADR; new entries added for Go library choices above.
- CI/build tooling changes from `cargo`/`tauri build` to `go build` +
  `wails build`.
- macOS AX-permission and focus-restoration logic (stage 6) is the highest-risk
  port — no mature Go equivalent exists for `objc2-app-kit`'s
  `NSWorkspace`/`NSRunningApplication` bindings; will require direct `cgo` +
  Objective-C shims.
- Existing gate-outs for stages 1-6 remain as historical record; new gate-outs
  use the `stage-M0`..`stage-M5` naming to distinguish migration stages.
