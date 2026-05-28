# Hepta KG Prompt-Preview Readiness Next-Action Index Gate

`scripts/hepta-kg-prompt-preview-readiness-next-action-index-gate.sh`
turns the KG prompt-preview operator briefing into a stdout-only readiness and
next-action index. It is an index over blocked evidence, not an approval packet
and not a runtime activation mechanism.

## Contract

- Script:
  `scripts/hepta-kg-prompt-preview-readiness-next-action-index-gate.sh`
- Gate id: `hepta_kg_prompt_preview_readiness_next_action_index_gate`
- Source gate:
  `hepta_kg_prompt_preview_operator_briefing_non_persistence_gate`
- Source terminal summary gate:
  `hepta_kg_prompt_preview_terminal_summary_gate`
- Source preflight gate:
  `hepta_kg_prompt_preview_preflight_gate`
- Index schema:
  `kg_prompt_preview_readiness_next_action_index_v1`

## Required Source State

The source operator briefing must remain ready but blocked:

- 5 source gates linked, checked, blocked, and report-only
- 5 redacted, blocked, non-persisted briefing sections
- 7 missing operator evidence records
- 4 missing safety controls
- 6 missing context handoff requirements
- 2 missing final review/approval records
- 19 total missing preflight requirements
- 32 denied source actions

The readiness index preserves these values and keeps final operator approval,
operator identity, operator scope, and activation plan acceptance false.

## Allowed Next Actions

The index allows only report-only or verification-only follow-up:

- maintain a report-only evidence index
- add an operator approval checklist schema
- add a rollback/kill-switch evidence checklist
- add a redacted diff review checklist
- add a context handoff checklist
- rerun full light preflight as verification

These actions do not permit prompt preview execution, prompt payload
materialization, context injection, model invocation, external KG reads, live KG
writes, gateway/source-command migration, CI promotion, active runtime wiring,
install/restart, or public release claims.

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
- persist or deliver the readiness index
- persist or deliver an operator briefing
- send Telegram, channel, or external messages
- record or accept final operator approval
- migrate gateway routes or source commands
- wire active runtime paths
- promote CI
- install, restart, or mutate the active binary
- read credentials

`scripts/hepta-preflight.sh` runs this gate immediately after the KG
prompt-preview operator briefing non-persistence gate.
