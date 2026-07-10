# Hepta Systems Tools v1 Read-Only Status Tool Source Cache - 2026-07-08

This source-cache keeps the Tools v1 boundary short and queryable without
adding current-reality matrix rows. It composes the existing registration
precondition, registration-denial, ToolRegistry shadow, read-only dispatch, and
receipt no-persistence reports into one visible contract.

Stable path anchor: registration -> lookup -> internal call -> structured result -> approval/ledger/receipt -> local append-only store.

## Sources

- `scripts/hepta-systems-plugin-v1-contract-source-cache-report.sh`
- `scripts/hepta-systems-tool-registry-shadow-registration-lookup-readback-report.sh`
- `scripts/hepta-systems-tool-registry-registration-lookup-cutover-preflight-report.sh`
- `scripts/hepta-systems-tool-registry-read-only-dispatch-preflight-report.sh`
- `scripts/hepta-systems-tool-registry-minimal-read-only-invocation-ledger-receipt-readback-report.sh`
- `scripts/hepta-systems-plugin-tool-invocation-feature-gated-read-only-status-dry-run-readback-report.sh`
- `scripts/hepta-systems-plugin-tool-invocation-read-only-status-tool-registration-preconditions-readback-without-registration-report.sh`
- `scripts/hepta-systems-plugin-tool-invocation-read-only-status-tool-registration-denial-readback-without-registration-report.sh`
- `scripts/hepta-systems-plugin-tool-invocation-read-only-status-tool-registration-denial-receipt-readback-without-persistence-report.sh`
- `scripts/hepta-systems-plugin-tool-invocation-read-only-status-tool-registration-denial-receipt-retention-replay-readback-without-persistence-report.sh`
- `scripts/hepta-systems-plugin-tool-invocation-read-only-status-tool-registration-denial-receipt-positive-preconditions-readback-without-persistence-report.sh`
- `scripts/hepta-systems-plugin-tool-invocation-read-only-status-tool-registration-denial-receipt-persistence-denial-readback-without-persistence-report.sh`
- `scripts/hepta-systems-workflow-temporal-lite-append-only-event-store-minimal-local-persistence-report.sh`

## Contract

Tools v1 has one selected read-only status tool and one non-selected
preflight-only connector candidate. The selected path is visible as a sequence:

- registration: denied/readback-only until registration authority, operator
  approval, schema digest, read-only policy, sandbox, idempotency, and rollback
  anchors exist. The denial readback is queryable in memory by candidate tool id,
  registration denial key, and registration denial route; missing tool ids miss
  without recording an attempt.
- lookup: shadow lookup result projection exists, but registry lookup execution
  remains disabled.
- internal call: a feature-gated dry-run payload and result projection exist,
  but the feature gate stays closed and no call executes. The feature-gated
  dry-run report now proves the path through registration-denial query hit,
  shadow lookup projection, internal status payload projection, structured
  result projection, approval/ledger/receipt projection, and local append-only
  store projection.
- structured result: the status payload projection is typed and queryable, but
  not persisted.
- approval/ledger/receipt: approval preflight, ledger preview, receipt
  projection, and in-memory result receipt are visible but unwritten.
- local append-only store: the Temporal-lite event-store contract is available
  as the future local store boundary; runtime event-log, workflow event-log, and
  SQLite writes stay disabled.

Registration-denial receipt persistence remains explicitly denied. Retention,
replay, positive preconditions, and persistence-denial reports are source-cache
facts only; they do not authorize receipt persistence.

## Closed Boundary

Stable closed-boundary anchor: no network, credentials, external POST, ToolRegistry mutation, registry lookup execution, tool invocation, ledger write, receipt persistence, runtime event-log write, workflow event-log write, SQLite write, transport mutation, canary, live, or Public GA.

This source-cache performs no feature gate open, no ToolRegistry registration,
no ToolRegistry mutation, no registry lookup execution, no tool invocation, no
internal call execution, no structured-result persistence, no approval request,
no approval acceptance or recording, no ledger write, no receipt persistence, no
result receipt write, no runtime event-log write, no workflow event-log write,
no SQLite write, no append-only store write, no credential read, no external
network or external POST, no Gateway, Native, Telegram, or channel mutation, no
provider/model invocation, no canary, no live, and no Public GA.
