# Hepta Core Activation Terminal Closure Operator Packet Authority Replay Matrix Gate

This gate is the report-only replay matrix above the operator packet dry-run
validator.

It consumes
`scripts/hepta-core-activation-terminal-closure-operator-packet-dry-run-validator-gate.sh`
through the shared JSON report capture helper. The source validator has already
proved that packet-shaped fixtures stay blocked, including complete-shape,
missing-field, stale evidence, cross-request, receipt-ledger-ack, and
public-claim fixtures.

This matrix goes one layer higher: it routes those packet-shaped fixtures toward
the semantic entry points that a future activation path would care about:
terminal closure, receipt acceptance, ledger recording, index delivery,
completion acknowledgement, and public release governance. Every replay stays
blocked.

## Replay Fixtures

The matrix must expose exactly 10 fixtures:

- `template-fixture-replayed-to-terminal-closure`
- `complete-shape-replayed-to-terminal-closure`
- `complete-shape-replayed-to-receipt-acceptance`
- `complete-shape-replayed-to-ledger-record`
- `cross-request-packet-replay-to-terminal-closure`
- `stale-evidence-packet-replay-to-receipt-ledger`
- `receipt-ledger-ack-replay-to-terminal-closure`
- `public-claim-packet-replay-to-release-governance`
- `delivered-index-without-accepted-packet-replay`
- `superseded-packet-pair-replay-to-current-request`

All fixtures stay `validation_status=blocked`, `dry_run_only=true`,
`report_only=true`, and `matrix_only=true`. Complete field shape is not
authority. Delivery shape is not acknowledgement. Receipt, ledger, and terminal
closure shapes cannot revive cross-request, stale, or superseded authority.

## Entry Points

The matrix covers six non-authorizing entry points:

- terminal closure
- receipt acceptance
- ledger record
- index delivery
- completion acknowledgement
- public release claim / release artifact write

Each entry point requires real accepted authority and exposes
`replay_authority_allowed=false`.

## Non-Authority Boundary

The gate is intentionally stdout-only. It does not:

- record or accept operator approval
- record activation request state
- accept fresh evidence
- approve filesystem persistence
- enable or execute receipt persistence
- accept receipts
- record ledger, index, delivery, or completion acknowledgement state
- persist, materialize, deliver, or accept a packet replay
- record, persist, materialize, or accept terminal closure
- activate, install, restart, or mutate active binaries
- invoke providers or models
- send Telegram/channel output
- fetch or merge upstream code
- write release artifacts
- make public release or GA claims
- read credentials or secret values

The replay matrix is a guardrail for future operator packet authority. It proves
that packet-looking evidence cannot become activation authority by being routed
through a downstream closure, receipt, ledger, delivery, or release surface.
