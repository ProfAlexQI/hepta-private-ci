# Hepta Systems Workflow v1 Source Of Truth Source Cache - 2026-07-08

This source-cache keeps Workflow v1 short and queryable without adding
current-reality matrix rows or extending WorkGraph suffix chains. It states that
the single append-only event store is the source of truth for Workflow v1, while
all runtime writes and live execution remain disabled.

Stable path anchor: lease -> idempotency -> checkpoint -> replay validator -> WorkGraph projection.
The source-cache now checks the single append-only event store interface
provenance through the same path, including event-log/SQLite adapter and
WorkGraph projection derivation.

## Sources

- `scripts/hepta-systems-workflow-temporal-lite-append-only-event-store-minimal-local-persistence-report.sh`
- `scripts/hepta-systems-workflow-temporal-lite-deterministic-replay-validator-local-persistence-readback-report.sh`
- `scripts/hepta-systems-workflow-temporal-lite-checkpoint-and-rollback-anchor-local-persistence-readback-report.sh`
- `scripts/hepta-systems-workflow-temporal-lite-lease-idempotency-index-local-persistence-readback-report.sh`
- `scripts/hepta-systems-workflow-temporal-lite-event-log-sqlite-adapter-local-persistence-readback-report.sh`
- `scripts/hepta-systems-workflow-temporal-lite-work-graph-projection-local-persistence-readback-report.sh`
- `scripts/hepta-systems-workflow-temporal-lite-work-graph-projection-replay-alignment-local-persistence-readback-report.sh`
- `scripts/hepta-systems-workflow-temporal-lite-replay-alignment-checkpoint-consistency-local-persistence-readback-report.sh`
- `scripts/hepta-systems-work-graph-canonical-schema-types-report-only-report.sh`
- `scripts/hepta-systems-work-graph-canonical-schema-fixture-report-generation-report.sh`
- `scripts/hepta-systems-work-graph-append-only-event-store-feature-gated-wal-precondition-report.sh`
- `scripts/hepta-systems-work-graph-append-only-event-store-feature-gated-wal-no-write-plan-report.sh`
- `scripts/hepta-systems-work-graph-append-only-event-store-feature-gated-wal-replay-diff-plan-report.sh`
- `scripts/hepta-systems-work-graph-append-only-event-store-feature-gated-wal-denial-closeout-report.sh`

## Contract

Workflow v1 is constrained to a visible source-cache path:

- canonical schema contract: 9 schema collections, 82 fields, and 36 join keys
  are visible before runtime persistence.
- append-only event store: 9 event contracts append in local tests, duplicate
  appends are denied, and deterministic replay is validated after reopen.
- lease: lease readback is visible, but lease acquisition and persistence are
  disabled.
- idempotency: idempotency index and duplicate guards are visible, but
  idempotency-index writes are disabled.
- checkpoint: checkpoint and rollback anchors are derived from event history,
  but checkpoint and rollback-anchor writes are disabled.
- replay validator: deterministic replay and checkpoint consistency validate
  event history, but replay execution is disabled.
- WorkGraph projection: node and edge projections are derived from event
  history, but WorkGraph projection/store writes are disabled.
- interface provenance: replay validator, checkpoint/rollback anchors,
  lease/idempotency, event-log/SQLite adapter, and WorkGraph projection all
  report derivation from the append-only event store interface before the
  source-cache can be ready.
- feature-gated WAL: WAL preconditions, no-write plans, replay-diff plans, and
  denial closeout are visible while WAL/checkpoint/replay/idempotency writes
  remain denied.

## Closed Boundary

Stable closed-boundary anchor: no runtime event-log write, runtime SQLite write, WAL write, lease acquire, idempotency-index write, checkpoint write, replay execution, WorkGraph store write, scheduler enforcement, task-result enforcement, role-manifest enforcement, canary, live, or Public GA.

This source-cache performs no filesystem write, runtime event-log write, runtime
SQLite write, runtime store persistence, WAL open/create/write/fsync, lease
acquire, lease persistence, idempotency-index write, checkpoint write, rollback
anchor write, replay execution, replay-diff persistence, rollback execution,
WorkGraph projection write, WorkGraph store write, scheduler admission
enforcement, task-result enforcement, role-manifest enforcement, workflow
execution, provider/model invocation, Gateway/Auth/Native/channel mutation,
canary, cutover, live, or Public GA.
