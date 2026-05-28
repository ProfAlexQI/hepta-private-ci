# Hepta KG Prompt-Preview Preflight Gate

This gate binds the Rust-native KG prompt-preview preflight report into the
local preflight script family without executing prompt preview, context
injection, model invocation, external KG reads, network calls, or live KG
writes.

## Contract

- Script: `scripts/hepta-kg-prompt-preview-preflight-gate.sh`
- Gate id: `hepta_kg_prompt_preview_preflight_gate`
- Source report contract:
  `hepta-intelligence-memory-kg-prompt-preview-preflight-v0`
- Context handoff contract:
  `hepta-intelligence-memory-kg-prompt-preview-context-handoff-v0`
- Source command: `memory-kg-prompt-preview-preflight`
- Runtime summary: `knowledge_graph_prompt_preview_preflight_summary`

## Source Checks

The script runs focused Rust tests for the two source surfaces:

- `hepta-intelligence`
  `memory_kg_prompt_preview_preflight_blocks_ci_promotion_until_gate_chain_closes`
- `hepta-runtime`
  `knowledge_graph_prompt_preview_preflight_summary_renders_blocked_ci_gate`

The script then emits a JSON report and validates it with `jq -e`. The report
requires the five source gates to be linked, ready, blocked, and report-only:

- approval packet
- operator evidence
- redaction diff
- rollback/kill-switch
- context handoff

## Deliberate Blockers

The gate is ready only as a CI/report gate. It keeps the source preflight report
blocked until all final requirements exist:

- 7 operator evidence records
- 4 safety controls
- 6 context handoff requirements
- redacted diff review
- context handoff approval

The resulting total remains 19 required and 19 missing preflight requirements.

## Side-Effect Boundary

The gate must remain side-effect free:

- no preflight execution
- no prompt preview rendering
- no prompt payload materialization
- no context injection
- no model invocation
- no external KG adapter read
- no Graphiti, Neo4j, or CocoIndex client construction
- no network call
- no external DB write
- no live KG write
- no native gateway route or source-command migration
- no install, restart, or active binary mutation
- no credential read

Any future promotion must keep this script as the report-only CI gate and add a
separate explicitly approved execution path.
