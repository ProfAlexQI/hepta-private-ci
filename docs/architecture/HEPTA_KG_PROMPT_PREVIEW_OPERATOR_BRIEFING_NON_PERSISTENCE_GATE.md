# Hepta KG Prompt-Preview Operator Briefing Non-Persistence Gate

`scripts/hepta-kg-prompt-preview-operator-briefing-non-persistence-gate.sh`
turns the KG prompt-preview terminal summary into an operator-briefing shaped
report without accepting, saving, delivering, or approving that briefing.

## Contract

- Script:
  `scripts/hepta-kg-prompt-preview-operator-briefing-non-persistence-gate.sh`
- Gate id: `hepta_kg_prompt_preview_operator_briefing_non_persistence_gate`
- Source gate: `hepta_kg_prompt_preview_terminal_summary_gate`
- Source preflight gate: `hepta_kg_prompt_preview_preflight_gate`
- Briefing schema: `kg_prompt_preview_operator_briefing_non_persistence_v1`

## Briefing Shape

The gate emits five redacted, blocked, non-persisted sections:

- source gate status
- missing requirements
- approval state
- execution boundary
- publication boundary

These sections are stdout-only schema fields. They are not a durable operator
approval, not a persisted briefing, and not a prompt-preview execution record.

## Required Blocked State

The source terminal summary must remain ready but blocked:

- 5 source gates linked, checked, blocked, and report-only
- 7 missing operator evidence records
- 4 missing safety controls
- 6 missing context handoff requirements
- 2 missing final review/approval records
- 19 total missing preflight requirements

The briefing gate then keeps final operator approval, operator identity, scope,
and activation plan acceptance all false.

## Denied Surfaces

The gate denies:

- operator briefing persistence
- operator briefing filesystem write
- operator briefing delivery
- operator approval recording
- operator approval acceptance
- prompt-preview execution through the briefing
- context injection through the briefing
- model invocation through the briefing
- public release claims through the briefing
- active runtime mutation through the briefing

The previous terminal summary denied-action list is carried forward, so the
combined denial count is `32`.

## Side-Effect Boundary

This gate must not:

- render prompt preview
- materialize prompt payloads
- inject context
- invoke a model
- read an external KG adapter
- construct Graphiti, Neo4j, or CocoIndex clients
- perform network calls
- write external DB or live KG data
- persist or deliver an operator briefing
- record or accept final operator approval
- migrate a gateway route or source command
- wire an active runtime path
- promote CI
- install, restart, or mutate the active binary
- read credentials

`scripts/hepta-preflight.sh` runs this gate immediately after the KG
prompt-preview terminal summary gate.
