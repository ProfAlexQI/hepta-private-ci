# Hepta KG Prompt-Preview Context Handoff Checklist Gate

`scripts/hepta-kg-prompt-preview-context-handoff-checklist-gate.sh`
turns the redacted diff review checklist into a stdout-only context handoff
checklist. It mirrors the Rust-side context handoff contract and keeps every
handoff requirement missing, redacted, non-persisted, and blocking until a
future explicitly approved path exists.

## Contract

- Script:
  `scripts/hepta-kg-prompt-preview-context-handoff-checklist-gate.sh`
- Gate id:
  `hepta_kg_prompt_preview_context_handoff_checklist_gate`
- Source gate:
  `hepta_kg_prompt_preview_redacted_diff_review_checklist_gate`
- Source rollback/kill-switch gate:
  `hepta_kg_prompt_preview_rollback_kill_switch_evidence_checklist_gate`
- Source readiness index gate:
  `hepta_kg_prompt_preview_readiness_next_action_index_gate`
- Checklist schema:
  `kg_prompt_preview_context_handoff_checklist_v1`
- Source context handoff contract:
  `hepta-intelligence-memory-kg-prompt-preview-context-handoff-v0`

## Required Source State

The source redacted diff review checklist must remain ready but blocked:

- the source checklist explicitly allowed this follow-up as report-only
- 5 source gates linked, checked, blocked, and report-only
- 5 operator briefing sections redacted, blocked, and not persisted
- 7 operator evidence records missing
- 4 safety controls missing
- 6 context handoff requirements missing
- 2 final review or approval records missing
- 19 total preflight requirements missing
- 2 redacted diff review items missing, redacted, and not persisted
- raw prompt diff, prompt text, and payload text counts remain zero

The context handoff checklist preserves the source chain and keeps all operator
approval, rollback, kill-switch, review, prompt-preview, context-injection,
model, KG, migration, install, and public-claim flags false.

## Handoff Checklist Items

The schema declares six required, missing, redacted, non-persisted handoff
items:

- operator evidence packet
- rollback/kill-switch safety packet
- redacted diff review receipt
- context handoff operator approval
- context injection scope record
- post-handoff monitoring plan

Every item blocks prompt preview and context injection until real evidence is
provided, reviewed, and explicitly accepted by a future gate.

## Redaction Boundary

This gate emits only redacted references and fixed absence counts:

- `raw_prompt_diff_count = 0`
- `prompt_text_included_count = 0`
- `payload_text_included_count = 0`

It must not print raw prompt diffs, prompt text, prompt payload text, context
payload text, or any context injection material.

## Allowed Follow-Up

After this gate, only report-only or verification-only work remains allowed:

- maintain a report-only evidence index
- rerun full light preflight as verification

These actions do not permit handoff acceptance, checklist persistence or
delivery, context injection, prompt preview execution, prompt payload
materialization, model invocation, external KG reads, live KG writes,
gateway/source-command migration, CI promotion, active runtime wiring,
install/restart, or public release claims.

## Side-Effect Boundary

This gate must not:

- accept context handoff evidence
- persist or deliver the checklist
- expose raw prompt diffs
- expose prompt, payload, or context text
- accept redacted diff review evidence
- accept rollback or kill-switch evidence
- record or accept operator approval
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
prompt-preview redacted diff review checklist gate.
