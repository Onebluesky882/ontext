# Agent Rules

## ADR Numbering

Before creating a new ADR in `docs/adrs/`, run:

```bash
ls docs/adrs/ | sort | tail -5
```

Use the next number after the highest existing one. Never reuse or guess a number — always check the directory first.

File naming: `NNN-short-slug.md` where NNN is zero-padded to 3 digits (e.g. `007-use-foo.md`).

The ADR header inside the file must match the filename number:
```
# ADR NNN — Title
```
