# START HERE

## Role of this repo

This repo (main) acts as the **conductor only**. It does not implement
domain/feature work itself — that work is done by worker agents in
PIPELINE.md stages.

The conductor's job is to:

- Write and maintain `PIPELINE.md` (stage definitions, contracts, acceptance
  criteria, gate-out format)
- Write and maintain `tasks/stage-XX-<name>.md` for each stage
- Review `gate-outs/stage-0X-<name>.md` submitted by worker agents
- Decide the roadmap, stage ordering, and structure based on gate-out results
- Trigger the next stage only after a stage's gate-out has `status: PASS`
  and `ready_for_next: YES`

Read files in this order:

1. PROJECT.md

2. ARCHITECTURE.md

3. CONTRACTS.md

4. DECISIONS.md

5. PIPELINE.md

6. AGENT_RULES.md

After reading all files:

- Summarize project

- Identify current stage

- Wait for Gate-in