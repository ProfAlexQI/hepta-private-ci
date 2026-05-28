# Hepta KG Prompt-Preview Redacted Diff Review Checklist Gate

`scripts/hepta-kg-prompt-preview-redacted-diff-review-checklist-gate.sh`
turns the rollback/kill-switch evidence checklist into a stdout-only redacted
diff review checklist. It defines the final review evidence shape required
before any future prompt-preview path can advance, while proving this report
does not expose raw prompt diffs, prompt text, or payload text.

## Contract

- Script:
  `scripts/hepta-kg-prompt-preview-redacted-diff-review-checklist-gate.sh`
- Gate id:
  `hepta_kg_prompt_preview_redacted_diff_review_checklist_gate`
- Source gate:
  `hepta_kg_prompt_preview_rollback_kill_switch_evidence_checklist_gate`
- Source approval gate:
  `hepta_kg_prompt_preview_operator_approval_checklist_schema_gate`
- Source readiness index gate:
  `hepta_kg_prompt_preview_readiness_next_action_index_gate`
- Checklist schema:
  `kg_prompt_preview_redacted_diff_review_checklist_v1`

## Required Source State

The source rollback/kill-switch checklist must remain ready but blocked:

- the source checklist explicitly allowed this follow-up as report-only
- 5 source gates linked, checked, blocked, and report-only
- 5 operator briefing sections redacted, blocked, and not persisted
- 7 operator approval evidence records missing
- 4 safety controls missing
- 6 context handoff requirements missing
- 2 final review or approval records missing
- 19 total preflight requirements missing
- 4 rollback/kill-switch checklist items missing, redacted, and not persisted

The redacted diff review checklist preserves the source chain and keeps all
operator approval, rollback, kill-switch, prompt-preview, context-injection,
model, KG, migration, install, and public-claim flags false.

## Review Checklist Items

The schema declares two required, missing, redacted, non-persisted review items:

- redacted diff review record
- redacted diff review approval record

Both items block prompt preview and context injection until real review evidence
is provided, reviewed, and explicitly accepted by a future gate.

## Redaction Boundary

This gate emits only redacted references and fixed absence counts:

- `raw_prompt_diff_count = 0`
- `prompt_text_included_count = 0`
- `payload_text_included_count = 0`

It must not print raw prompt diffs, prompt text, prompt payload text, or context
payload text.

## Allowed Follow-Up

After this gate, only report-only or verification-only work remains allowed:

- maintain a report-only evidence index
- add context handoff checklist
- rerun full light preflight as verification

These actions do not permit review acceptance, checklist persistence or
delivery, raw prompt diff exposure, prompt text exposure, prompt preview
execution, prompt payload materialization, context injection, model invocation,
external KG reads, live KG writes, gateway/source-command migration, CI
promotion, active runtime wiring, install/restart, or public release claims.

## Side-Effect Boundary

This gate must not:

- accept redacted diff review evidence
- persist or deliver the checklist
- expose raw prompt diffs
- expose prompt or payload text
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
prompt-preview rollback/kill-switch evidence checklist gate.
