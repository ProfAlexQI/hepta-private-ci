# Hepta Activation Command Result Receipt Retention Expiry Garbage Collection Denial Route Gate

This gate promotes the existing activation-command result-receipt retention/expiry/garbage-collection denial evidence into a native gateway route while preserving the report-only boundary.

## Route

- Endpoint: `/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-retention-expiry-garbage-collection-denial`
- Source command: `/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-retention-expiry-garbage-collection-denial --json`
- Source gate: `scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-retention-expiry-garbage-collection-denial-gate.sh`
- Route gate: `scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-retention-expiry-garbage-collection-denial-route-gate.sh`

## Contract

The route requires the activation-command result-receipt audit-trail/immutable-evidence denial evidence to be ready first. It then exposes ten retention/expiry/garbage-collection fixtures as blocked/no-op evidence:

- missing source audit-trail/immutable-evidence report
- retention policy write request
- retention index recording request
- expiry scheduler and timer request
- TTL update and extension request
- garbage-collection scan request
- delete, tombstone, and sweep request
- archive and compaction request
- activation, provider, model, Memory, KG, and readback retention/GC attempt
- rollback, secret, external send, public claim, install, restart, active-binary, and upstream retention/GC attempt

All fixtures keep retention policy acceptance/recording/persistence, retention index recording, expiry scheduling, timer start, TTL updates, garbage-collection scan/candidate/decision recording, delete, tombstone, sweep, archive, compaction, ledger/index/delivery retention records, activation authority, provider/model invocation, Memory/KG mutation, credential/secret read, channel send, install/restart, active-binary mutation, upstream fetch/merge, and public release claims disabled.

## Side-Effect Boundary

The route is stdout/report only. It does not accept retention policy requests, register expiry schedulers, start timers, update TTL, scan garbage-collection candidates, delete receipts, tombstone receipts, sweep receipts, archive receipts, compact ledgers, record lifecycle evidence, derive operator approval or activation authority, invoke providers/models, write Memory or KG, read credentials or secrets, deliver channels, install or restart services, mutate the active binary, or make public release or GA claims.

## Validation

The route gate validates the source retention/expiry/garbage-collection denial gate, native gateway route wiring, source command count `104`, the focused codex-cli native unit test, optional live endpoint parity, and terminal coverage through `scripts/hepta-preflight.sh`.
