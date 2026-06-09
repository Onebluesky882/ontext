# ADR 008 — Next.js Payment Portal with Stripe

## Status

Accepted

## Date

2026-06-09

## Context

ontext needs a subscription model to monetize the SaaS offering. The Tauri desktop app is the core product, but billing, plan management, and subscriber auth must live in a web portal that is accessible from any browser and can be linked from within the desktop app.

Separate concerns: the Tauri app handles the recording pipeline; the web portal handles identity and billing.

## Decision

Create `app/web/` — a Next.js (Pages Router) application for:
- Pricing and subscription plan display
- Stripe Checkout payment flow
- Stripe Webhooks for subscription lifecycle
- Auth guard restricting dashboard to active subscribers

**Framework:** Next.js (Pages Router)
**Payment:** Stripe — Checkout Sessions + Webhooks
**Auth:** NextAuth.js (JWT strategy, credentials provider as base)
**State:** Zustand for client-side auth/subscription state
**Styling:** Tailwind CSS
**Package manager:** pnpm (matches project standard)

**Clean architecture layers:**

| Layer | Location | Rule |
|-------|----------|------|
| UI | `components/` | Pure props, no fetch, no store access |
| Logic | `hooks/` | useStore, fetch, side effects |
| Global state | `store/` | Zustand slices |
| Routes | `pages/` | Compose hooks + components only |

## Consequences

- Subscribers identified by email; Stripe Customer ID stored server-side
- Webhook endpoint verifies Stripe signature before mutating any state
- Dashboard route (`/dashboard`) is protected by Next.js middleware
- The Tauri app can deep-link to `app/web` for upgrade prompts
- Free tier can be gated inside the Tauri app by checking subscription status via a lightweight API call to `app/web`
