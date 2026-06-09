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

gate-out.md

Format:

Status:
PASS | FAIL

Stage:
Domain:
Summary:
Modified Files:

* file1
* file2

Dependencies Added:

* none

Tests:

* test_a
* test_b

Acceptance Criteria:

* Requirement 1
* Requirement 2

Known Issues:

* none

Recommendations:

* optional

Ready For Next Stage:
YES | NO

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
