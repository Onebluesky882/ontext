# Stage 05 — clipboard

Status: DONE

Domain: modules/clipboard
Branch: feature/clipboard

Goal:
Write TranscriptResult.text to clipboard and paste into the active input field.

Assigned To: claude-sonnet-4-6
Started At: 2026-06-09
Completed At: 2026-06-09

---

Checklist:
- [ ] Read PROJECT.md, ARCHITECTURE.md, CONTRACTS.md, DECISIONS.md, PIPELINE.md
- [ ] Implement modules/clipboard
- [ ] Text written to system clipboard
- [ ] Paste simulated into active app (Cmd+V / Ctrl+V)
- [ ] PasteResult.success is true on success
- [ ] PasteResult.error is None on success (never empty string)
- [ ] Failure returns descriptive error string
- [ ] Unit tests pass
- [ ] Build passes
- [ ] Create gate-outs/stage-05-clipboard.md

---

Gate-Out: gate-outs/stage-05-clipboard.md
Next Stage: — (pipeline complete)
