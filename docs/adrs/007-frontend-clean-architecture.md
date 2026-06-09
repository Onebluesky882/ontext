# ADR 007 — Frontend Clean Architecture

**Status:** Accepted  
**Date:** 2026-06-09

## Context

Stage 6 introduces the macOS Tauri UI. The React codebase needs a clear layer boundary so components stay thin, business logic is testable in isolation, and state is predictable.

## Decision

Adopt a four-layer structure inside `app/ontext/src/`:

| Layer | Path | Responsibility |
|-------|------|----------------|
| **Store** | `store/appStore.ts` | Global reactive state (zustand). Single source of truth for pipeline status and last result. |
| **Hooks** | `hooks/` | Bridge between UI and Tauri commands / localStorage. No JSX. |
| **Components** | `components/` | Purely presentational; receive props, emit callbacks, no direct store reads unless display-only. |
| **Pages** | `pages/` | Compose components and hooks into full screens; own layout. |

**State management:** Zustand 5 — tiny, no boilerplate, works with React 19 Strict Mode.

**Styling:** CSS custom properties for light/dark theming. System font stack (`-apple-system`) for macOS-native feel.

**Routing:** Simple `useState<'main' | 'settings'>` in `App.tsx` — no router library needed for two screens.

## Consequences

- Components can be rendered in isolation (Storybook-ready) without store setup.
- `usePipeline` owns the `invoke('run_pipeline')` call; pages never call `invoke` directly.
- Adding new pages or swapping zustand for another store requires changes only in the store/hooks layer.
