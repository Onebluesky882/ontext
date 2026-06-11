# Agent Rules

## Before Starting Any Task

If you are a worker agent assigned to a PIPELINE.md stage:
1. Read START_HERE.md first
2. Read AGENT_RULES.md
3. Confirm your assigned stage's `tasks/stage-XX-<name>.md` has Status: READY/IN_PROGRESS before beginning work

## ADR Numbering

**MANDATORY — do this before creating any ADR, no exceptions:**

```bash
ls docs/adrs/ | sort | tail -3
```

Take the highest number shown, add 1, use that as your ADR number.

Rules:
- Never guess or assume the next number
- Never create a file without running the check above first
- File name: `NNN-short-slug.md` (zero-padded to 3 digits)
- Header inside the file must match the filename exactly:
  ```
  # ADR NNN — Title
  ```
- This rule applies in every worktree — the `docs/adrs/` directory is shared via git

**Current highest:** 008 — next new ADR must be 009
