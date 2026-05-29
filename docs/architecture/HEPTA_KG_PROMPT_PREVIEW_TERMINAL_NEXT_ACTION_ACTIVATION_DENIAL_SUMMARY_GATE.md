# Hepta KG Prompt-Preview Terminal Next-Action Activation Denial Summary Gate

`scripts/hepta-kg-prompt-preview-terminal-next-action-activation-denial-summary-gate.sh`
turns the context handoff checklist into a terminal next-action and activation
denial summary. It is a stdout-only report over the closed KG prompt-preview
checklist chain. It is not an activation record, not an approval packet, not a
prompt-preview renderer, and not a runtime wiring mechanism.

## Contract

- Script:
  `scripts/hepta-kg-prompt-preview-terminal-next-action-activation-denial-summary-gate.sh`
- Gate id:
  `hepta_kg_prompt_preview_terminal_next_action_activation_denial_summary_gate`
- Source gate:
  `hepta_kg_prompt_preview_context_handoff_checklist_gate`
- Source context handoff checklist schema:
  `kg_prompt_preview_context_handoff_checklist_v1`
- Summary schema:
  `kg_prompt_preview_terminal_next_action_activation_denial_summary_v1`

## Required Source State

The source context handoff checklist must remain ready but blocked:

- 5 source gates linked, checked, blocked, and report-only
- 5 operator briefing sections redacted, blocked, and not persisted
- 7 operator evidence records missing
- 4 safety controls missing
- 6 context handoff requirements missing
- 2 final review or approval records missing
- 19 total preflight requirements missing
- 6 context handoff checklist items missing, redacted, and not persisted
- raw prompt diff, prompt text, and payload text counts remain zero

The terminal summary preserves the source chain and keeps all operator approval,
rollback, kill-switch, redacted diff review, handoff, prompt-preview,
context-injection, model, KG, migration, install, release, and public-claim
flags false.

## Terminal Next Actions

Only report-only or verification-only terminal actions are allowed:

- inspect the terminal activation-denial summary
- maintain the report-only evidence index
- rerun full light preflight as verification

These actions do not permit prompt preview execution, prompt payload
materialization, context injection, model invocation, external KG reads, live KG
writes, gateway/source-command migration, CI promotion, active runtime wiring,
install/restart, release artifact writes, credential reads, or public release
claims.

## Activation Denial Boundary

The gate reports `activation_allowed = false` and keeps the terminal activation
decision blocked until operator evidence, safety evidence, redacted diff review,
context handoff scope, monitoring, and final approval are all provided,
reviewed, and explicitly accepted by a future approved path.

The denied terminal action set includes prompt preview execution, context
injection, model invocation, external KG adapter reads, live KG writes,
checklist persistence or delivery, CI promotion, install/restart, active binary
mutation, release artifact writes, public release claims, and credential reads.

## Side-Effect Boundary

This gate must not:

- render prompt preview
- materialize prompt payloads
- expose raw prompt diffs, prompt text, or payload text
- inject context
- accept context handoff evidence
- persist or deliver the terminal summary
- persist or deliver context handoff checklists
- accept redacted diff review evidence
- accept rollback or kill-switch evidence
- record or accept operator approval
- invoke a model
- read an external KG adapter
- construct Graphiti, Neo4j, or CocoIndex clients
- perform network calls
- write external DB or live KG data
- persist or deliver readiness indexes or operator briefings
- send Telegram, channel, or external messages
- migrate gateway routes or source commands
- promote CI
- wire active runtime paths
- install, restart, or mutate the active binary
- write release artifacts
- make public release or public GA claims
- read credentials

`scripts/hepta-preflight.sh` runs this gate immediately after the KG
prompt-preview context handoff checklist gate and before the live mutation
governance gate.
