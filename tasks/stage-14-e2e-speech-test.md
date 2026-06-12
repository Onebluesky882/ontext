# Stage 14 — e2e speech-to-text verification

Status: READY

Domain: `app/ontext-wails` (integration test only — production code changes
only if a real bug is found in `internal/transcribe`, must be reported)
Branch: `feature/stage-14-e2e-speech-test`

Goal:
Run the full pipeline end-to-end via `wails dev`: Start → speak → Stop →
text pasted into the focused input. Verify Thai and English transcription
accuracy against the Stage 09 contract (`whisper-large-v3-turbo`,
`language=th`), and confirm API timeout/error paths return structured errors
(no panic).

Assigned To: (unassigned)
Started At: -
Completed At: -

---

Checklist:
- [ ] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md (Stage 14)
- [ ] `wails dev` → run full Start/speak/Stop/paste flow, record result
- [ ] Test Thai speech sample(s) — record actual transcript vs. spoken text
- [ ] Test English speech sample(s) — record actual transcript vs. spoken text
- [ ] Simulate API timeout/error (e.g. mock/disable network) — confirm
      structured error returned, app does not panic
- [ ] Document all results (transcripts, pass/fail) in gate-out
- [ ] Create gate-outs/stage-14-e2e-speech-test.md

---

Gate-Out: gate-outs/stage-14-e2e-speech-test.md
Next Stage: none (independent verification stage)
