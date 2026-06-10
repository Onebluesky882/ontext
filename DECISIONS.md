# DECISIONS.md

This file records all technology decisions for ontext.
These decisions are authoritative. Do not switch technologies without updating this file and getting orchestrator approval.

---

## Runtime: Tauri 2

Decision: Use Tauri 2 as the application runtime.

Reason:
- Cross-platform: macOS, Windows, iOS, Android
- Rust backend — safe, fast, low memory
- Smaller binary than Electron
- Native OS integration for hotkey, audio, clipboard

Do not switch to: Electron, Flutter, React Native

---

## Frontend: React 19 + TypeScript

Decision: Use React 19 with TypeScript and Vite.

Reason:
- Already initialized in project
- Type safety reduces agent errors across stages

Do not switch to: Vue, Svelte, plain JS

---

## Transcription: Groq Whisper API

Decision: Use Groq's OpenAI-compatible API as the default transcription backend (switched from OpenAI Whisper API).

Reason:
- OpenAI-compatible API — minimal code changes from prior OpenAI Whisper integration
- Faster inference than OpenAI Whisper API
- `whisper-large-v3-turbo` model optimized for low-latency, near real-time transcription
- High accuracy across languages including Thai
- No local model setup required
- Easy to swap for local whisper.cpp if needed

API base: `https://api.groq.com/openai/v1`
Endpoint: `https://api.groq.com/openai/v1/audio/transcriptions`
Model: `whisper-large-v3-turbo`

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

## Rust Edition: 2021

Decision: Use Rust edition 2021.

Reason:
- Already set in Cargo.toml
- Latest stable edition at project init

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

## Branch Strategy

Decision: feature branches only. Never commit directly to `main` or `dev`.

Branch format: `feature/<module-name>`

Examples:
- `feature/hotkey`
- `feature/audio`
- `feature/vad`
- `feature/transcribe`
- `feature/clipboard`
