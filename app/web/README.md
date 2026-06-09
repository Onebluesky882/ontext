# ontext-web

Next.js web portal for ontext subscription management.

## Stack

- **Next.js** (Pages Router) — routing and API
- **NextAuth.js** — authentication (JWT, credentials)
- **Stripe** — subscription billing via Checkout Sessions
- **Zustand** — client-side auth/subscription state
- **Tailwind CSS** — styling

## Architecture

```
components/   pure UI, props only, no hooks/store access
hooks/        logic: useAuth, useSubscription, useCheckout
store/        Zustand: authStore, subscriptionStore
pages/        routes: compose hooks + components only
pages/api/    API routes: auth, checkout, webhook, subscription
```

## Setup

```bash
cp .env.example .env.local
# fill in NEXTAUTH_SECRET, Stripe keys, and Price IDs
pnpm install
pnpm dev
```

## Stripe Webhook (local)

```bash
stripe listen --forward-to localhost:3000/api/webhook
```

## Routes

| Route | Access |
|-------|--------|
| `/` | Public — landing |
| `/pricing` | Public — plan selection |
| `/login` | Public — sign in |
| `/dashboard` | Auth required (middleware) |

## TODO (post-MVP)

- [ ] Replace credentials provider with Google/GitHub OAuth
- [ ] Add database (Prisma + Postgres) for user + subscription records
- [ ] Populate `/api/subscription` from DB instead of placeholder
- [ ] Populate webhook handlers to write subscription state to DB
- [ ] Add Stripe Customer Portal for self-serve plan changes
