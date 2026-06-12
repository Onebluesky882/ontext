# DECISIONS.md

This file records all technology decisions for ontext.
These decisions are authoritative. Do not switch technologies without updating this file and getting orchestrator approval.

---

## Runtime: Tauri 2 (REMOVED — see ADR 009)

Decision: Use Tauri 2 as the application runtime.

Status: Superseded 2026-06-11 by [ADR 009](docs/adrs/009-migrate-tauri-rust-to-wails-go.md).
The runtime has been migrated to Wails (Go backend) as of stage 12's
cutover — `app/ontext/src-tauri`, `app/ontext/` Tauri frontend, the root
`Cargo.toml` workspace, and Rust `modules/*` crates have all been removed.
This entry is kept for historical context only.

Reason (original):
- Cross-platform: macOS, Windows, iOS, Android
- Rust backend — safe, fast, low memory
- Smaller binary than Electron
- Native OS integration for hotkey, audio, clipboard

---

## Runtime: Wails v2 + Go

Decision: Use Wails v2 as the application runtime, with Go (1.22+) as the
backend language. Replaces Tauri 2 / Rust (see ADR 009).

Reason:
- Cross-platform: macOS, Windows (iOS/Android dropped from target platforms —
  Wails does not support mobile)
- Embeds a system webview, same as Tauri — existing React/Vite frontend
  reused as-is
- Go backend — simpler concurrency model for the streaming audio/VAD/paste
  pipeline than was achieved in Rust

Go library choices for ported modules:
- Audio capture: `github.com/gen2brain/malgo` (replaces `cpal`)
- VAD: streaming RMS-VAD ported directly to Go (replaces `webrtc-vad` crate)
- Transcription HTTP: stdlib `net/http` (replaces `reqwest`)
- Clipboard write: `github.com/atotto/clipboard` (replaces `arboard`)
- Paste simulation (Cmd+V/Ctrl+V): `github.com/go-vgo/robotgo` (replaces `enigo`)
- macOS focus capture/AX permission: `cgo` + AppKit/CoreFoundation shims
  (replaces `objc2`/`objc2-app-kit`/`objc2-foundation`)

Do not switch to: Electron, Flutter, React Native

---

## Frontend: React 19 + TypeScript

Decision: Use React 19 with TypeScript and Vite.

Reason:
- Already initialized in project
- Type safety reduces agent errors across stages

Do not switch to: Vue, Svelte, plain JS

---

## Styling: Tailwind CSS

Decision: Add Tailwind CSS to the frontend, starting at stage M0
(Wails bootstrap).

Reason:
- Already used in `app/web` (Next.js portal, ADR 008) — keeps styling
  approach consistent across the desktop app and web portal
- Utility-first classes reduce one-off CSS files as the Wails frontend is
  rebuilt

Existing components keep their current styling; Tailwind is available for
new/changed UI from M0 onward.

Do not switch to: CSS Modules, styled-components, Sass

---

## Transcription: Groq Whisper API

Decision: Use Groq's OpenAI-compatible API as the default transcription backend (switched from OpenAI Whisper API).

Reason:
- OpenAI-compatible API — minimal code changes from prior OpenAI Whisper integration
- Faster inference than OpenAI Whisper API
- `whisper-large-v3` model used as the default for best accuracy
- High accuracy across languages including Thai
- No local model setup required
- Easy to swap for local whisper.cpp if needed

API base: `https://api.groq.com/openai/v1`
Endpoint: `https://api.groq.com/openai/v1/audio/transcriptions`
Model: `whisper-large-v3`

Alternative (future): whisper.cpp local model — document in DECISIONS.md when switching.

Do not use: Google Speech-to-Text, Azure Speech, DeepSpeech

---

## VAD: webrtc-vad

Decision: Use `webrtc-vad` Rust crate for voice activity detection.

Reason:
- Lightweight, no ML dependency
- Fast, deterministic
- Proven in production (used in WebRTC)

Do not switch to: silero-vad (requires ONNX runtime, heavier)

---

## Audio Format: 16kHz mono f32 PCM

Decision: All audio in the pipeline is 16kHz, mono channel, f32 samples.

Reason:
- Whisper requires 16kHz mono
- Standardized format prevents conversion errors between modules

Do not change sample rate or channel count without updating all modules and CONTRACTS.md.

---

## Package Manager: pnpm

Decision: Use pnpm for Node/frontend dependencies.

Reason:
- Already initialized in project (`pnpm-lock.yaml` exists)

Do not switch to: npm, yarn, bun

---

## Rust Edition: 2021 (REMOVED — see ADR 009)

Status: Superseded 2026-06-11. The `Cargo.toml` workspace and `modules/*`
Rust crates were removed in stage 12's cutover. This entry is kept for
historical context only.

---

## Go Version: 1.22+

Decision: Use Go 1.22 or later for the Wails backend.

Reason:
- Required by Wails v2 and `gen2brain/malgo`
- Generics support used for shared pipeline types (audio buffers, chunks)

---

## Web Portal: Next.js (Pages Router)

Decision: `app/web/` is a standalone Next.js app using the Pages Router.

Reason:
- Pages Router maps naturally to the clean architecture layers required (components/, hooks/, store/, pages/)
- Simpler mental model for state-7 scope — no need for Server Components complexity
- Compatible with next-auth v4 and Stripe redirect flow

Do not switch to: App Router (requires significant restructuring), Remix, SvelteKit

---

## Payment: Stripe

Decision: Use Stripe Checkout Sessions + Webhooks for subscription billing.

Reason:
- Industry standard, PCI compliant
- Hosted checkout removes card data from our servers
- Webhook events drive subscription lifecycle (created, updated, canceled)

Price IDs: configured in `.env` (`STRIPE_PRO_PRICE_ID`, `STRIPE_TEAM_PRICE_ID`).

Do not switch to: LemonSqueezy, Paddle (can be revisited if Stripe unavailable in target market)

---

## Auth (Web): NextAuth.js v4

Decision: Use NextAuth.js v4 with JWT strategy for the web portal.

Reason:
- Integrates with Next.js API routes with minimal config
- JWT strategy avoids requiring a session DB for MVP
- Credentials provider is placeholder — swap for OAuth (Google/GitHub) when ready

Do not switch to: Clerk, Auth0, Supabase Auth (evaluate post-MVP)

---

## Client State (Web): Zustand

Decision: Use Zustand for auth and subscription state in the web portal.

Reason:
- Minimal boilerplate, works well with Next.js hydration
- `persist` middleware handles localStorage rehydration

Do not switch to: Redux, Jotai

---

## Hotkey Reintroduction (Stage 13, Go)

Decision: reintroduce a global hotkey for the Wails app, implemented with
`golang.design/x/hotkey` (replaces the dropped `rdev`-based
`modules/hotkey`).

Reason for the original removal: `rdev`'s global listener crashed on macOS
when Accessibility permission was not granted, so hotkey-driven start/stop
was replaced with button-driven start/stop in the UI.

Reason for reversal:
- ADR 010 (usage-based STT billing) defines a usage session as one hotkey
  hold: `startedAt` on hotkey-down, `endedAt` on hotkey-up,
  `durationMs = endedAt - startedAt` reported to `POST /usage/events`. A
  hold-to-talk hotkey is the natural UX for this and is cheaper/faster than
  requiring two clicks (Start/Stop) per session.
- `golang.design/x/hotkey` registers a global shortcut via OS APIs without
  requiring the same broad input-monitoring/Accessibility grant that
  `rdev`'s low-level event tap needed; if registration fails or
  Accessibility permission is missing, it returns an error instead of
  crashing, so `internal/hotkey` can fall back to button-only start/stop with
  a status message (per Stage 13 acceptance criteria).
- The button-driven Start/Stop UI from the Wails migration remains as the
  fallback path — this is additive, not a regression.

---

## AI Text Autocorrect (Stage 18)

Decision: add a new `internal/autocorrect` module (see ADR 011), inserted
into the pipeline between `transcribe` and `clipboard`. It calls the Groq
chat-completions API with a small/fast instruction model to fix
spelling/grammar/punctuation only, and fails open (returns the original text)
on any error, timeout, or empty response. A `NoopCorrector` mirrors
`transcribe.NoopTranscriber` for tests/environments without a Groq key.

See ADR 011 for full context and consequences (added latency/cost per
segment, candidate for future usage-billing inclusion per ADR 010).

---

## RNNoise Denoise (Stage 19)

Decision: add a new `internal/denoise` module (see ADR 012), inserted into
the pipeline between `audio` and `vad`. It applies RNNoise-based noise
suppression to each `audio.Frame`, returning a denoised frame of the same
length and sample rate. Implemented via a cgo binding to `librnnoise`
(Xiph.Org), and fails open (returns the original frame) on init/runtime
error. A `NoopDenoiser` mirrors `transcribe.NoopTranscriber` and
`autocorrect.NoopCorrector` for tests/platforms without a working RNNoise
build.

See ADR 012 for full context and consequences (cross-platform cgo build
requirement, possible RMS-VAD threshold re-tuning).

---

## Branch Strategy

Decision: feature branches only. Never commit directly to `main` or `dev`.

Branch format: `feature/<module-name>`

Examples:
- `feature/hotkey`
- `feature/audio`
- `feature/vad`
- `feature/transcribe`
- `feature/clipboard`
