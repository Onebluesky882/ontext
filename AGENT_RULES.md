AGENT_RULES.md

Purpose

This repository uses a multi-agent workflow.

Every agent is a worker, not the owner of the project.

The orchestrator controls:

* PROJECT.md
* PIPELINE.md
* DECISIONS.md
* ARCHITECTURE.md
* CONTRACTS.md

Agents implement assigned work only.

⸻

Required Reading

Before starting any task, read:

1. PROJECT.md
2. PIPELINE.md
3. ARCHITECTURE.md
4. CONTRACTS.md
5. DECISIONS.md

Do not begin implementation before understanding these files.

⸻

Domain Ownership

Each stage owns only its assigned domain.

Examples:

Stage 1
Domain:
modules/hotkey

Stage 2
Domain:
modules/audio

Stage 3
Domain:
modules/vad

Stage 4
Domain:
modules/transcribe

Stage 5
Domain:
modules/clipboard

Do not modify another stage’s implementation.

⸻

Allowed Changes

You may:

* Create files inside your assigned domain
* Modify files inside your assigned domain
* Add tests for your assigned domain
* Update documentation for your assigned domain

You may NOT:

* Modify confirmed stages
* Rewrite architecture
* Change contracts
* Change database schema without explicit instruction
* Change public APIs without approval

⸻

Contract Compliance

CONTRACTS.md is the source of truth.

Input types and output types must match contracts exactly.

If a contract appears incorrect:

STOP

Report the issue.

Do not invent a new contract.

⸻

Dependency Rules

Prefer existing dependencies.

Do not add new dependencies unless required.

If adding a dependency:

Document:

* package name
* version
* reason

inside gate-out.md

⸻

File Ownership

Agent must report all modified files.

Example:

Modified Files:

* modules/transcribe/mod.rs
* modules/transcribe/client.rs
* tests/transcribe_test.rs

⸻

Branch Rules

Agent branches:

feature/

Examples:

feature/hotkey
feature/audio
feature/vad
feature/transcribe
feature/clipboard

Never merge directly into:

* dev
* main

Create PR only.

⸻

Testing Rules

Run relevant tests before completion.

Required:

* unit tests
* build verification

If tests cannot run:

Explain why.

Never claim tests passed without execution.

⸻

Error Handling

Never panic intentionally.

Return structured errors.

Example:

{
“error”: “timeout contacting whisper api”
}

Applications must fail gracefully.

⸻

Architecture Rules

Follow ARCHITECTURE.md.

Do not:

* move modules
* rename domains
* redesign workflow

unless explicitly instructed.

⸻

Protected Files (Orchestrator Only)

Do NOT modify these files under any circumstances:

* app/ontext/src-tauri/src/lib.rs
* app/ontext/src-tauri/src/main.rs
* Cargo.toml (workspace root)

These files are owned by the orchestrator.
The orchestrator wires all modules together after all stages complete.

If your module needs to be registered in lib.rs:

STOP

Report in gate-out.md under Recommendations.
Do not modify lib.rs yourself.

⸻

Cargo Rules

Each module has its own Cargo.toml:

modules/<name>/Cargo.toml

Add dependencies only to your module's Cargo.toml.

Do NOT modify:

* Cargo.toml (workspace root)
* app/ontext/src-tauri/Cargo.toml

If you need a shared type from another module, import it as a path dependency:

[dependencies]
ontext-audio = { path = "../../modules/audio" }

⸻

Decision Rules

DECISIONS.md is authoritative.

If DECISIONS.md says:

Use Drizzle

Do not switch to Prisma.

If DECISIONS.md says:

Use Better Auth

Do not switch to Auth.js.

⸻

Stage Completion

When work is complete, create:

gate-outs/stage-0X-<name>.md

Use this exact format (conductor parses these fields):

---
status: PASS
stage: 01
domain: modules/hotkey
branch: feature/hotkey
assigned_to: <model name>
completed_at: <YYYY-MM-DD>
ready_for_next: YES
---

summary: one line description of what was implemented

modified_files:
  - modules/hotkey/src/lib.rs
  - modules/hotkey/Cargo.toml

dependencies_added:
  - none

tests:
  - test_hotkey_start_emits_event
  - test_hotkey_stop_emits_event

acceptance_criteria:
  - PASS: Hotkey press emits HotkeyEvent::Start
  - PASS: Hotkey release emits HotkeyEvent::Stop

known_issues:
  - none

recommendations:
  - none

Rules:
- status must be exactly PASS or FAIL
- ready_for_next must be exactly YES or NO
- Do not add extra fields
- Do not use bullet points (*) — use hyphens (-) only
- Do not leave fields empty — use "none" if nothing to report

⸻

Stop Condition

After completing assigned work:

STOP

Do not continue to the next stage.

Do not implement future stages.

Wait for orchestrator confirmation.

⸻

Multi-Model Compatibility

Assume future contributors may include:

* GPT
* Claude
* Gemini
* Codex
* Other agents

Write code and documentation that is:

* deterministic
* explicit
* easy to merge
* easy to review

Avoid hidden assumptions.

⸻

Merge Optimization

Prefer:

small focused commits

Avoid:

large refactors

One stage should produce one logical PR.

Keep changes isolated to the assigned domain.

This reduces merge conflicts and improves review quality.

⸻

Task Tracking

When starting a stage, update the assigned task file:

tasks/stage-XX-<name>.md

Set:

Status: IN_PROGRESS
Assigned To: <model name>
Started At: <date>

When stage completes, update the same file:

Status: DONE
Completed At: <date>

⸻

ADR Rules

If you add a new dependency not listed in DECISIONS.md:

1. Create docs/adrs/00X-<reason>.md
2. Follow the ADR format:
   - Status
   - Context
   - Decision
   - Reasons
   - Consequences
   - Alternatives Considered
3. Record the dependency in gate-out.md under Dependencies Added

Do not add dependencies without an ADR.

⸻

Types

Frontend TypeScript types are in:

app/ontext/src/types/

Do not redefine types inline in components.
Import from types/ instead.

If a new shared type is needed, add it to the appropriate file:

* audio.ts    — AudioBuffer, AudioChunk
* transcript.ts — TranscriptResult, PasteResult
* events.ts   — HotkeyEvent, AppStatus, StatusEvent

⸻

Git & Build Artifacts

This rule applies to every agent, every stage, no exceptions.

Never commit or push build artifacts.

Forbidden paths — do NOT track or push:

- target/
- node_modules/
- dist/
- build/
- .next/

Pre-push checklist

Before every push, verify:

1. Check .gitignore exists and covers the paths above
2. Run: git status — confirm no artifact directories are listed as staged or tracked
3. If any forbidden path appears in git status, stop and fix before pushing

If artifacts are already tracked

Remove them from git without deleting local files:

git rm -r --cached target/
git rm -r --cached node_modules/
git commit -m "chore: remove tracked build artifacts"

Then ensure .gitignore covers those paths before pushing again.

Minimum .gitignore for Rust projects

/target/
Cargo.lock (only for library crates — keep for binary/app crates)
*.rs.bk
.DS_Store

For workspaces that also have a frontend (Node/React):

node_modules/
dist/
build/
.next/
.env
.env.local
