# 004 — Use rdev for Global Hotkey Detection

Status: Accepted
Date: 2026-06-09

---

## Context

ontext needs to detect a global hotkey (Cmd+Shift+Space on macOS, Ctrl+Shift+Space on Windows)
while any other application is in focus. This requires a system-level key listener,
not just in-app key handling.

The `modules/hotkey` crate is a pure Rust library (no Tauri runtime in scope at this layer),
so a self-contained Rust crate is required.

## Decision

Use the `rdev` crate for cross-platform global keyboard event listening.

```toml
rdev = "0.5"
```

## Reasons

- Cross-platform: macOS (CGEventTap), Windows (SetWindowsHookEx), Linux (X11/evdev)
- Pure Rust — no external runtime or C++ dependency
- Captures global key events regardless of focused application
- Actively maintained, widely used in Tauri/desktop Rust projects
- Provides key press and key release events needed for Start/Stop detection

## Consequences

- macOS requires Accessibility permission at runtime (Input Monitoring or Accessibility in System Preferences)
  — the Tauri layer must request this permission; the module itself does not handle it
- rdev's `listen` function is blocking and runs for the process lifetime; graceful stop is not supported
  — acceptable for a Tauri desktop app where the listener runs until quit
- `block v0.1.6` (transitive dep) has a future-incompatibility warning in Rust; not actionable now

## Alternatives Considered

| Library                       | Reason Rejected                                                        |
|-------------------------------|------------------------------------------------------------------------|
| tauri-plugin-global-shortcut  | Requires Tauri AppHandle context; not usable from a pure library crate |
| inputbot                      | Primarily Windows/Linux; macOS support is limited                      |
| platform APIs directly        | Per-platform boilerplate; poor cross-platform maintainability          |
| device_query                  | Polling-based, not event-driven; wastes CPU and misses fast keystrokes |
