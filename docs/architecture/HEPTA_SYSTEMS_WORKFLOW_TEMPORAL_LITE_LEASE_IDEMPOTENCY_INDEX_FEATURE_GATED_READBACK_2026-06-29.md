# Hepta Systems Workflow Temporal-Lite Lease Idempotency Index Feature-Gated Readback - 2026-06-29

This note closes
`workflow_temporal_lite_lease_idempotency_index_feature_gated_readback`.
It consumes the Temporal-Lite checkpoint and rollback anchor readback and projects
the lease and idempotency index boundary needed before any durable event-log or
SQLite adapter work.

This is a test-only lease and idempotency readback. It does not acquire a lease,
does not write an idempotency index, and does not persist any runtime state.

## Source Boundary

- Source report:
  `scripts/hepta-systems-workflow-temporal-lite-checkpoint-and-rollback-anchor-feature-gated-readback-report.sh`
- Source surface:
  `workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback`
- Source entries: 9
- Source state: checkpoint and rollback anchors are paired, mismatch-free, and readback-only
- Source write state: no checkpoint write, rollback anchor write, anchor persistence, workflow execution, replay execution, rollback execution, event-log write, SQLite write, or live execution

## Readback Projection

The lease/idempotency readback projects one lease boundary and one idempotency
boundary for each of the nine Temporal-Lite event contracts:

- lease key:
  `temporal-lite.lease.readback.<sequence>.<event_contract_id>`
- lease token:
  `lease-token.v1.<sequence>.<event_contract_id>.<checkpoint_anchor_key_length>`
- idempotency index key:
  `temporal-lite.idempotency-index.readback.<sequence>.<event_contract_id>`
- idempotency key:
  `idempotency-key.v1.<sequence>.<event_contract_id>.<event_id_length>`
- duplicate guard key:
  `temporal-lite.duplicate-guard.readback.<sequence>.<event_contract_id>`

Projected counts:

- 9 lease readbacks
- 9 idempotency index readbacks
- 9 lease tokens
- 9 idempotency keys
- 9 duplicate guards
- 0 leases acquired
- 0 leases persisted
- 0 idempotency index writes
- 0 idempotency index persistences

## Closed Boundary

This slice has no lease acquisition, lease persistence, idempotency index write, idempotency index persistence, runtime event-log write, SQLite write, workflow execution, replay execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, or live execution.

The report and Rust read model keep these paths false:

- `runtime_feature_gate_enabled`
- `runtime_event_log_write_allowed`
- `runtime_sqlite_write_allowed`
- `lease_acquisition_allowed`
- `lease_persistence_allowed`
- `idempotency_index_write_allowed`
- `idempotency_index_persistence_allowed`
- `workflow_execution_allowed`
- `replay_execution_allowed`
- `rollback_execution_allowed`
- `live_execution_allowed`

## Local Gates

- Report:
  `scripts/hepta-systems-workflow-temporal-lite-lease-idempotency-index-feature-gated-readback-report.sh`
- Gate:
  `scripts/hepta-systems-workflow-temporal-lite-lease-idempotency-index-feature-gated-readback-gate.sh`
- Rust module:
  `codex-rs/hepta-runtime/src/workflow_temporal_lite_lease_idempotency_index_feature_gated_readback.rs`

## Next Move

The next local systems gate is:

`temporal_lite_event_log_sqlite_adapter_feature_gated_readback`

That slice should remain feature-gated and readback-first: no runtime event-log
write, SQLite write, workflow execution, replay execution, rollback execution,
provider invocation, model invocation, transport mutation, release, canary
activation, or live execution.
