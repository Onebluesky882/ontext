# ADR 001 — cpal for microphone capture

Status: Accepted

Context:
Stage 2 requires opening the default microphone, streaming PCM samples, and stopping on demand. A cross-platform audio I/O crate is needed that supports macOS and Windows without OS-specific code in the module.

Decision:
Use `cpal` v0.15 for microphone capture.

Reasons:
- Cross-platform: supports macOS (CoreAudio), Windows (WASAPI), Linux (ALSA/JACK)
- Provides low-level stream control (start/stop) needed for hotkey-gated recording
- Stable API used widely in Rust audio projects
- No heavy ML or native SDK dependency required

Consequences:
- Adds `cpal` and its platform backends (coreaudio-rs on macOS, wasapi on Windows) to the build
- Build time increases slightly due to bindgen (coreaudio-sys)

Alternatives Considered:
- `rodio`: higher-level, designed for playback; no direct stream control
- `portaudio`: C binding, more complex cross-compile setup
- Direct CoreAudio / WASAPI bindings: platform-specific, breaks cross-platform goal
