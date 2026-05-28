# Hepta KG Prompt-Preview Operator Approval Checklist Schema Gate

`scripts/hepta-kg-prompt-preview-operator-approval-checklist-schema-gate.sh`
turns the KG prompt-preview readiness next-action index into a stdout-only
operator approval checklist schema. It defines the evidence shape needed for a
future approval review, but it does not record, accept, persist, or deliver
operator approval.

## Contract

- Script:
  `scripts/hepta-kg-prompt-preview-operator-approval-checklist-schema-gate.sh`
- Gate id:
  `hepta_kg_prompt_preview_operator_approval_checklist_schema_gate`
- Source gate:
  `hepta_kg_prompt_preview_readiness_next_action_index_gate`
- Source operator briefing gate:
  `hepta_kg_prompt_preview_operator_briefing_non_persistence_gate`
- Source preflight gate:
  `hepta_kg_prompt_preview_preflight_gate`
- Checklist schema:
  `kg_prompt_preview_operator_approval_checklist_schema_v1`

## Required Source State

The source readiness index must remain ready but blocked:

- the source next-action index explicitly allows this checklist as report-only
- 5 source gates linked, checked, blocked, and report-only
- 5 operator briefing sections redacted, blocked, and not persisted
- 7 missing operator evidence records
- 4 missing safety controls
- 6 missing context handoff requirements
- 2 missing final review or approval records
- 19 total missing preflight requirements

The checklist schema preserves these source counts and keeps final operator
approval, operator identity, operator scope, activation plan, approval digest,
and bounded prompt-preview scope acceptance false.

## Checklist Items

The schema declares seven required, missing, redacted, non-persisted checklist
items:

- operator approval record
- rollback plan record
- kill-switch record
- reviewer identity record
- approval timestamp record
- signed approval digest
- bounded prompt-preview scope

Every item blocks prompt preview until real evidence is provided, reviewed,
signed where appropriate, scoped, and explicitly accepted by a future gate.

## Allowed Follow-Up

After this gate, only report-only or verification-only work remains allowed:

- maintain a report-only evidence index
- add rollback/kill-switch evidence checklist
- add redacted diff review checklist
- add context handoff checklist
- rerun full light preflight as verification

These actions do not permit approval recording, prompt preview execution,
prompt payload materialization, context injection, model invocation, external KG
reads, live KG writes, gateway/source-command migration, CI promotion, active
runtime wiring, install/restart, or public release claims.

## Side-Effect Boundary

This gate must not:

- record or accept operator approval
- accept operator identity, scope, activation plan, digest, or bounded scope
- persist or deliver the checklist
- render prompt preview
- materialize prompt payloads
- inject context
- invoke a model
- read an external KG adapter
- construct Graphiti, Neo4j, or CocoIndex clients
- perform network calls
- write external DB or live KG data
- persist or deliver the readiness index or operator briefing
- send Telegram, channel, or external messages
- migrate gateway routes or source commands
- wire active runtime paths
- promote CI
- install, restart, or mutate the active binary
- read credentials

`scripts/hepta-preflight.sh` runs this gate immediately after the KG
prompt-preview readiness next-action index gate.
