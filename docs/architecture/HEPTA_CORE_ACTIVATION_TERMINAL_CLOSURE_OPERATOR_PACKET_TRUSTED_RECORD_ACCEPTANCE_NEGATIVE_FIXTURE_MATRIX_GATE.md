# Hepta Core Activation Terminal Closure Operator Packet Trusted-Record Acceptance Negative-Fixture Matrix Gate

This gate is the report-only negative-fixture matrix above the
trusted-record acceptance skeleton.

It consumes
`scripts/hepta-core-activation-terminal-closure-operator-packet-trusted-record-acceptance-skeleton-gate.sh`
through the shared JSON report capture helper. The source skeleton has already
declared 8 trusted-record shapes, 7 precondition families, and 56 required
precondition checks while accepting none of them.

The matrix proves that malformed, stale, mismatched, or out-of-order
trusted-record-shaped inputs still cannot become accepted records, terminal
closure, receipt/ledger authority, delivery authority, public release authority,
or activation authority.

## Negative Fixtures

The gate must expose exactly 12 negative fixtures:

- `missing-required-record-shape-field`
- `unknown-trusted-record-shape`
- `operator-identity-hash-mismatch`
- `operator-identity-binding-method-mismatch`
- `activation-request-nonce-replay`
- `activation-request-generation-mismatch`
- `trusted-evidence-set-hash-mismatch`
- `receipt-payload-ledger-hash-mismatch`
- `freshness-window-expired`
- `receipt-accepted-without-persistence`
- `ledger-record-before-receipt-acceptance`
- `delivery-before-completion-ack`

All fixtures stay `validation_status=blocked`, `negative_fixture_only=true`,
`dry_run_only=true`, and `report_only=true`. No fixture can record, persist,
accept, deliver, or authorize a trusted record.

## Covered Preconditions

The fixture matrix covers the same 7 precondition families declared by the
skeleton:

- record shape
- operator identity binding
- activation request nonce binding
- hash binding
- freshness window
- receipt ledger precondition
- delivery completion acknowledgement precondition

The matrix reports 12 attempted acceptance fixtures, 12 blocked fixtures, and 0
allowed fixtures. It also keeps the source skeleton facts visible: 8 skeleton
records, 56 required checks, and 0 satisfied checks.

## Non-Acceptance Boundary

The gate is intentionally stdout-only. It does not:

- record, persist, accept, or deliver a trusted record
- record or accept operator approval
- record activation request state
- accept fresh evidence
- approve filesystem persistence
- enable or execute receipt persistence
- accept receipts
- record ledger, index, delivery, or completion acknowledgement state
- record, persist, materialize, or accept terminal closure
- activate, install, restart, or mutate active binaries
- invoke providers or models
- send Telegram/channel output
- fetch or merge upstream code
- write release artifacts
- make public release or GA claims
- read credentials or secret values

The matrix is a fail-closed guard for future trusted-record acceptance work. It
keeps the future acceptance shape testable while proving that negative fixtures
and packet-shaped inputs are not authority.
