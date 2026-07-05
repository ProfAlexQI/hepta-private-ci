# Hepta Systems Workflow Temporal-Lite Durable Store Adapter - 2026-06-27

This note records Phase 3 of the Hepta systems convergence plan. The
Temporal-Lite Durable Store Adapter restores the workflow durable-store adapter
surfaces as a local read-model behind a feature gate.

## Sources

The adapter consumes the current append-only intake preview:

- `scripts/hepta-systems-work-graph-append-only-event-intake-preview-report.sh`
- `codex-rs/hepta-runtime/src/work_graph_append_only_event_intake_preview.rs`

It restores the three current filesystem surfaces that had been memory drift:

- `codex-rs/hepta-runtime/src/workflow_durable_store_append_plan.rs`
- `codex-rs/hepta-runtime/src/workflow_durable_store_adapter_harness.rs`
- `codex-rs/hepta-runtime/src/workflow_durable_store_adapter.rs`

The restored contract carries the 9 current append-only event contracts into a
Temporal-lite plan:

- append-plan metadata
- lease metadata
- idempotency-key metadata
- checkpoint metadata
- replay validation metadata
- rollback anchor metadata
- no-op harness receipts

## Boundary

This is a feature gate contract with no event-log writes. It does not:

- write workflow event logs
- write SQLite state
- acquire leases
- mutate idempotency indexes
- write checkpoints
- perform readback
- execute workflow steps
- execute replay
- execute rollback
- start live execution
- invoke providers or models
- mutate gateway/auth or Native POST routing
- send channels
- package, release, or promote Public GA

## Next Move

Phase 4 should thread a thin read-only E2E chain:

`hepta-system status plugin -> ToolRegistry read-only dispatch preflight -> workflow durable store adapter receipt -> Native read-only console`

The next phase should still avoid registration, invocation, ledger writes,
approval requests, receipt persistence, workflow event-log writes, SQLite
writes, replay/rollback execution, gateway/auth mutation, Native POST mutation,
provider/model invocation, channel sends, and live execution.
