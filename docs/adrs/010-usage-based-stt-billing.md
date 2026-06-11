# ADR 010 — Usage-Based STT Billing (Included Minutes + Overage)

## Status

Proposed

## Date

2026-06-11

## Context

The current pricing in `app/web` (ADR 008) is flat-fee, unlimited-usage per plan
(Free / Pro $9 / Team $29). STT cost scales directly with audio minutes
processed, so an unlimited plan has no cost ceiling — a heavy user can make a
plan unprofitable.

Cost basis (ontext-internal estimate, excludes server/storage/bandwidth/VAT/dev cost):

- STT vendor cost ≈ 0.075 บาท/นาที (≈ 4.5 บาท/ชม.)
- Target sell price at 300% markup ≈ 0.30 บาท/นาที (≈ 18 บาท/ชม.)

Server/storage/bandwidth/VAT/dev margin do not scale per-minute the same way
(they scale with concurrent users, GB stored, GB transferred). They should be
recovered through the recurring base fee, not folded into the per-minute rate.

## Decision

Each plan gets an **included minute quota per billing period** plus a
**metered overage rate** for usage beyond the quota. The STT button records a
session (start/stop), the desktop app reports session duration to a new
usage-metering backend, and overage is billed through Stripe metered billing.

### Plan shape (illustrative — finance to confirm against real infra cost)

| Plan | Base fee / mo | Included minutes | Overage rate |
|------|---------------|-------------------|--------------|
| Free | 0 | 30 min | blocked at quota (no overage) |
| Pro | 199 บาท | 600 min (10 hr) | 0.35 บาท/min |
| Team | 599 บาท + 199/seat | 3000 min pooled | 0.35 บาท/min |

Overage rate is set above the 0.30 บาท/min STT-only target to leave headroom
for storage/bandwidth on overage usage.

### Pay-as-you-go top-up packs

In addition to subscription overage, users (any plan, including Free) can buy
one-off minute/hour credit packs that top up their quota balance directly —
no subscription change required.

| Pack | Price | Rate |
|------|-------|------|
| 1 hr | ~25 บาท | 0.42 บาท/min |
| 5 hr | ~110 บาท | 0.37 บาท/min |
| 10 hr | ~200 บาท | 0.33 บาท/min |

- Purchased via Stripe Checkout (one-time payment, not a subscription item).
- On webhook success, `usage_periods.topup_seconds` is credited for that user.
- Quota check (`GET /usage/quota`) consumes `includedSeconds` first, then
  `topupSeconds`, before falling into subscription overage (or blocking, on
  Free).
- Top-up credit does not expire at period rollover (it's a purchased balance,
  not a recurring allowance).

### Architecture

Separate concerns per ADR 008: `app/web` keeps owning auth, plans, and Stripe
checkout/webhooks. A new Go service in `backend/` owns usage metering (matches
the Go stack from ADR 009).

```
Desktop app (ontext-wails)
  → POST /usage/events  { sessionId, startedAt, endedAt, durationMs }
  → GET  /usage/quota    -> { includedSeconds, usedSeconds, remainingSeconds, plan }

backend/ (Go, new)
  - usage_events table: append-only log of STT sessions per user
  - usage_periods table: per-user aggregate for the current billing period
  - cron: nightly aggregation + push overage usage records to Stripe
    (Subscription Item, usage_type=metered)

app/web (Next.js)
  - Stripe Product per plan: 1 flat recurring price (base fee, included
    minutes) + 1 metered price (overage), both on the same Subscription
  - /api/subscription returns plan + quota fields for the dashboard
```

### Data model additions

`Subscription` type (`app/web/store/subscriptionStore.ts`) gains:

```ts
type Subscription = {
  planId: PlanId;
  status: SubscriptionStatus;
  currentPeriodEnd: string | null;
  stripeCustomerId: string | null;
  includedMinutes: number;
  usedSeconds: number;
  topupSeconds: number;
  overageRatePerMinute: number; // บาท
};
```

### Session timing

A usage session = one hotkey hold. `startedAt` is recorded on hotkey-down,
`endedAt` on hotkey-up; `durationMs = endedAt - startedAt` is what gets
reported to `POST /usage/events` and deducted from the user's balance
(included minutes → top-up credit → overage/blocked).

### Quota enforcement

- Desktop app calls `GET /usage/quota` before enabling the STT hotkey.
- Free plan: included quota + top-up credit exhausted → STT disabled,
  deep-link to `/pricing` (top-up or upgrade).
- Pro/Team: quota + top-up exhausted → STT stays enabled (overage billed), UI
  shows a soft warning.

## Consequences

- New `backend/` Go service required (currently empty) — owns
  `usage_events` / `usage_periods` and the Stripe usage-record cron.
- `app/web` Stripe integration changes: each paid plan becomes a Subscription
  with two Subscription Items (flat + metered), not a single flat price.
- Desktop app (`app/ontext-wails`) needs a new API client call on STT
  start/stop to report session duration, plus a quota check on hotkey press.
- Free plan users get a hard stop instead of unlimited usage — UX needs a
  clear "quota exhausted" state.
- Illustrative prices above are placeholders; must be revisited once real
  server/storage/bandwidth costs are known.
- `app/web` needs a one-time-payment Checkout flow (separate from the
  subscription Checkout) for top-up packs, plus webhook handling that credits
  `topup_seconds` instead of changing subscription status.
- Hotkey hold/release timing (`modules/hotkey`) must report precise
  start/stop timestamps to the desktop app's usage reporter — this is the
  source of truth for billed duration.
