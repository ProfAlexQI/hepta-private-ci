# Hepta Core Activation Terminal Closure Operator Packet Template Gate

This gate is the report-only template layer above the terminal closure gap
evidence index.

It consumes
`scripts/hepta-core-activation-terminal-closure-gap-evidence-index-gate.sh`
through the shared JSON report capture helper. The source gap index has already
proved that terminal closure remains blocked with 12 indexed gaps, 6 source
gates, 6 doc anchors, and 12 witness hashes.

This gate does not record or accept an operator packet. It turns the indexed
gaps into a deterministic operator-facing packet template:

- one template section per terminal closure gap
- required packet fields for that section
- source gate and source field for the missing record
- denied reason and doc anchor
- per-gap witness hash carried from the gap evidence index
- operator instruction text for future manual packet construction

## Template Sections

The template must expose exactly 12 sections:

- operator authority
- operator identity
- activation request
- fresh long-soak evidence
- trusted evidence set
- filesystem persistence approval
- receipt persistence command
- receipt persistence execution
- receipt acceptance
- ledger record
- index delivery
- completion acknowledgement

All sections stay `status=missing`, `operator_input_required=true`,
`template_only=true`, `report_only=true`, `recorded=false`, `persisted=false`,
`accepted=false`, and `delivered=false`.

## Non-Approval Boundary

The gate is intentionally stdout-only. Rendering the template is not approval
and does not authorize terminal closure. It does not:

- record or accept operator approval
- record activation request state
- accept fresh evidence
- approve filesystem persistence
- enable or execute receipt persistence
- accept receipts
- record ledger, index, delivery, or completion acknowledgement state
- persist, materialize, deliver, or accept the operator packet template
- record, persist, materialize, or accept terminal closure
- activate, install, restart, or mutate active binaries
- invoke providers or models
- send Telegram/channel output
- fetch or merge upstream code
- write release artifacts
- make public release or GA claims
- read credentials or secret values

The template is an entrance checklist for a future explicit operator packet. It
keeps the current runtime blocked while making the missing packet shape
auditable.
