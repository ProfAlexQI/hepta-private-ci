# Hepta Activation Command Result Receipt Replay Idempotency Denial Route Gate

This gate promotes the existing activation-command result-receipt replay/idempotency denial evidence into a native gateway route while preserving the report-only boundary.

## Route

- Endpoint: `/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial`
- Source command: `/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial --json`
- Source gate: `scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial-gate.sh`
- Route gate: `scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial-route-gate.sh`

## Contract

The route requires the activation-command result-receipt no-persistence route to be ready first. It then exposes ten replay/idempotency fixtures as blocked/no-op evidence:

- missing source result receipt no-persistence report
- duplicate result receipt identity replay
- replay acceptance attempt
- idempotency key recording attempt
- idempotency state persistence/materialization attempt
- cross-scope result receipt reuse
- stale nonce and out-of-order replay
- completion acknowledgement, ledger, index, and delivery replay
- activation, provider, Memory, and KG replay
- external send, public claim, install, restart, upstream, credential, and secret replay

All fixtures keep replay acceptance, duplicate acceptance, idempotency key/state recording, idempotency persistence, cross-scope reuse, nonce acceptance, status upgrade, completion acknowledgement, activation authority, provider/model invocation, Memory/KG mutation, credential/secret read, channel send, install/restart, active-binary mutation, and upstream fetch/merge disabled.

## Side-Effect Boundary

The route is stdout/report only. It does not record or persist receipts, does not create idempotency state, does not derive operator approval or activation authority, does not invoke providers/models, does not write Memory or KG, does not read credentials or secrets, does not deliver channels, does not install or restart services, does not mutate the active binary, and does not make public release or GA claims.

## Validation

The route gate validates the source replay/idempotency denial gate, native gateway route wiring, source command count `101`, the focused codex-cli native unit test, optional live endpoint parity, and terminal coverage through `scripts/hepta-preflight.sh`.
