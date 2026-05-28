# Hepta KG Prompt-Preview Terminal Summary Gate

`scripts/hepta-kg-prompt-preview-terminal-summary-gate.sh` is the
operator-readable terminal summary for the KG prompt-preview preflight chain. It
consumes the JSON report from
`scripts/hepta-kg-prompt-preview-preflight-gate.sh`, validates the blocked source
state, and emits a schema-only summary that can be read by operators or CI.

## Contract

- Script: `scripts/hepta-kg-prompt-preview-terminal-summary-gate.sh`
- Gate id: `hepta_kg_prompt_preview_terminal_summary_gate`
- Source gate: `hepta_kg_prompt_preview_preflight_gate`
- Source report contract:
  `hepta-intelligence-memory-kg-prompt-preview-preflight-v0`
- Context handoff contract:
  `hepta-intelligence-memory-kg-prompt-preview-context-handoff-v0`
- Summary schema: `kg_prompt_preview_terminal_summary_v1`

## Summary Semantics

The summary is ready only because the source preflight gate is linked, checked,
blocked, and report-only. It preserves the deliberate blocked state:

- 5 linked source gates
- 5 ready source checks
- 5 blocked source gates
- 5 report-only source gates
- 19 required preflight requirements
- 19 missing preflight requirements

The terminal decision remains:

`blocked_until_source_preflight_requirements_final_review_context_handoff_approval_and_explicit_operator_approval_exist`

## Operator Boundary

The summary is operator-readable, not operator-approval. It records no final
approval and does not persist a briefing, ledger, receipt, or summary file. It
also keeps CI promotion and active runtime wiring denied.

The source chain remains blocked until these requirements are added through a
separate explicitly approved path:

- operator evidence
- safety controls
- context handoff evidence
- redacted diff review
- context handoff approval
- final operator approval

## Side-Effect Boundary

The gate must remain side-effect free:

- no prompt preview rendering
- no prompt payload materialization
- no context injection
- no model invocation
- no external KG adapter read
- no Graphiti, Neo4j, or CocoIndex client construction
- no network call
- no external DB write
- no live KG write
- no terminal summary persistence
- no operator briefing persistence
- no native gateway route or source-command migration
- no active runtime wiring
- no CI promotion
- no install, restart, or active binary mutation
- no credential read

`scripts/hepta-preflight.sh` runs this terminal summary immediately after the KG
prompt-preview preflight gate and before live mutation governance gates.
