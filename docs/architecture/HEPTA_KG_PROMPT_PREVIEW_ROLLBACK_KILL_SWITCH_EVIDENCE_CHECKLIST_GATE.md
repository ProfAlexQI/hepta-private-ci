# Hepta KG Prompt-Preview Rollback/Kill-Switch Evidence Checklist Gate

`scripts/hepta-kg-prompt-preview-rollback-kill-switch-evidence-checklist-gate.sh`
turns the KG prompt-preview operator approval checklist schema into a
stdout-only rollback and kill-switch evidence checklist. It defines the safety
evidence shape required before any future prompt-preview path can advance, but
it does not accept, persist, or deliver safety evidence.

## Contract

- Script:
  `scripts/hepta-kg-prompt-preview-rollback-kill-switch-evidence-checklist-gate.sh`
- Gate id:
  `hepta_kg_prompt_preview_rollback_kill_switch_evidence_checklist_gate`
- Source gate:
  `hepta_kg_prompt_preview_operator_approval_checklist_schema_gate`
- Source readiness index gate:
  `hepta_kg_prompt_preview_readiness_next_action_index_gate`
- Source preflight gate:
  `hepta_kg_prompt_preview_preflight_gate`
- Checklist schema:
  `kg_prompt_preview_rollback_kill_switch_evidence_checklist_v1`

## Required Source State

The source operator approval checklist schema must remain ready but blocked:

- the source next-action index explicitly allowed this checklist as report-only
- 5 source gates linked, checked, blocked, and report-only
- 5 operator briefing sections redacted, blocked, and not persisted
- 7 operator approval checklist items missing
- 4 safety controls missing
- 6 context handoff requirements missing
- 2 final review or approval records missing
- 19 total preflight requirements missing

The rollback/kill-switch checklist preserves these source counts and keeps all
approval, identity, scope, activation plan, digest, and bounded prompt-preview
scope acceptance false.

## Safety Checklist Items

The schema declares four required, missing, redacted, non-persisted safety
items:

- rollback plan record
- rollback dry-run evidence
- kill-switch record
- kill-switch dry-run evidence

Every item blocks prompt preview and context injection until real evidence is
provided, reviewed, and explicitly accepted by a future gate.

## Allowed Follow-Up

After this gate, only report-only or verification-only work remains allowed:

- maintain a report-only evidence index
- add redacted diff review checklist
- add context handoff checklist
- rerun full light preflight as verification

These actions do not permit safety evidence acceptance, approval recording,
prompt preview execution, prompt payload materialization, context injection,
model invocation, external KG reads, live KG writes, gateway/source-command
migration, CI promotion, active runtime wiring, install/restart, or public
release claims.

## Side-Effect Boundary

This gate must not:

- accept rollback or kill-switch evidence
- persist or deliver the checklist
- record or accept operator approval
- accept operator identity, scope, activation plan, digest, or bounded scope
- render prompt preview
- materialize prompt payloads
- inject context
- invoke a model
- read an external KG adapter
- construct Graphiti, Neo4j, or CocoIndex clients
- perform network calls
- write external DB or live KG data
- persist or deliver operator approval checklist or operator briefing artifacts
- send Telegram, channel, or external messages
- migrate gateway routes or source commands
- wire active runtime paths
- promote CI
- install, restart, or mutate the active binary
- read credentials

`scripts/hepta-preflight.sh` runs this gate immediately after the KG
prompt-preview operator approval checklist schema gate.
