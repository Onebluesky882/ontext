# ADR 004 — Use arboard and enigo for clipboard and paste simulation

Status: Accepted

## Context

Stage 5 (modules/clipboard) must write transcript text to the system clipboard and simulate a paste keystroke (Cmd+V on macOS, Ctrl+V on Windows) into the active application window.

## Decision

Use `arboard` (v3) for clipboard access and `enigo` (v0.2) for keyboard input simulation.

## Reasons

- `arboard` is cross-platform (macOS, Windows, Linux), actively maintained, and provides a simple API for reading/writing clipboard text.
- `enigo` is cross-platform, supports key press/release/click actions, and has first-class support for modifier+key combos required for Cmd+V and Ctrl+V simulation.
- Both crates are pure Rust with no unsafe FFI required from the module level.

## Consequences

- Clipboard and paste work on macOS and Windows without platform-specific code beyond `#[cfg(target_os)]` guards.
- On unsupported platforms, `simulate_paste` returns a descriptive error rather than panicking.
- Both crates are added only to `modules/clipboard/Cargo.toml` — no other module is affected.

## Alternatives Considered

- `clipboard` crate: older API, less active maintenance.
- `x11-clipboard`: Linux-only, out of scope for current platform targets.
- Raw CGEvent / SendInput via FFI: more control but significantly more unsafe code; enigo already wraps this correctly.
