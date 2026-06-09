# Stage 02 — audio

Status: PENDING

Domain: modules/audio
Branch: feature/audio

Goal:
Open microphone on HotkeyEvent::Start. Stream PCM audio at 16kHz mono f32. Stop on HotkeyEvent::Stop.

Assigned To: —
Started At: —
Completed At: —

---

Checklist:
- [ ] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md
- [ ] Implement modules/audio using cpal
- [ ] Microphone opens on HotkeyEvent::Start
- [ ] Microphone closes on HotkeyEvent::Stop
- [ ] Output is AudioBuffer { samples: Vec<f32>, sample_rate: 16000 }
- [ ] Unit tests pass
- [ ] Build passes
- [ ] Create gate-outs/stage-02-audio.md

---

Gate-Out: gate-outs/stage-02-audio.md
Next Stage: stage-03-vad
