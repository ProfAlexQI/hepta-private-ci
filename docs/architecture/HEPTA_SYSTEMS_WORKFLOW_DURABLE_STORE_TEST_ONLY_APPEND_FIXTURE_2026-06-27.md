# Workflow Durable Store Test-Only Append Fixture

This note records the Phase 7 local-only workflow durable-store fixture for the
Hepta systems lane. It consumes the Temporal-lite durable-store adapter and
materializes a test-only in-memory fixture for append-only validation.

## Scope

The fixture covers the nine existing durable workflow event contracts:

- `plan_step_event_intake`
- `agent_spawn_event_intake`
- `mailbox_delivery_event_intake`
- `agent_job_item_event_intake`
- `worker_task_event_intake`
- `scheduler_run_event_intake`
- `artifact_event_intake`
- `approval_event_intake`
- `task_result_event_intake`

For each contract it validates fixture metadata for:

- append-only sequence order
- idempotency key shape
- duplicate append denial
- checkpoint metadata
- replay validation metadata
- rollback anchor metadata

The fixture is `ready` as a test-only read model. Runtime durable-store writes
remain closed.

## Boundary

This is a test-only in-memory fixture. It deliberately performs no runtime
event-log write, SQLite write, fixture file write, lease acquisition,
idempotency index mutation, checkpoint write, workflow execution, replay
execution, rollback execution, provider invocation, model invocation,
Gateway/Auth mutation, Native POST mutation, Telegram transport mutation,
channel send, package, release, Public GA promotion, or live execution.

Closed runtime boundary: no runtime event-log write, SQLite write, fixture file write, lease acquisition, idempotency index mutation, checkpoint write, workflow execution, replay execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, or live execution.

## Expected Counts

- event contracts: 9
- fixture entries: 9
- append-only sequence validations: 9
- idempotency fixture validations: 9
- checkpoint fixture validations: 9
- replay validation fixture validations: 9
- rollback fixture validations: 9
- duplicate append denials: 9
- runtime event-log writes: 0
- runtime SQLite writes: 0
- live executions: 0

## Verification

The local gate validates:

- the upstream Temporal-lite durable-store adapter is ready
- all nine event contracts have test-only append fixture entries
- every entry has stable append, idempotency, checkpoint, replay validation,
  and rollback metadata keys
- duplicate append denial is represented for each entry
- runtime feature gate, event-log writes, SQLite writes, fixture persistence,
  workflow execution, replay, rollback, and live execution remain disabled
- targeted hepta-runtime Rust tests pass

## Next Move

Phase 8 should add
`phase8_internal_read_only_hepta_system_status_invocation_without_external_network_or_mutation`.
It should open only the thinnest internal read-only `hepta-system status`
invocation path, with no credential read, no external network, no tool mutation,
no ledger write, no Gateway/Auth mutation, no Native POST mutation, no Telegram
transport mutation, no package/release writes, no Public GA, and no live
cutover.
