# Hepta Systems Current Reality Canonical Frontier Index

## Purpose

The current systems lane now has enough readback layers that downstream gates should not keep rendering the full capability matrix and long status-canary chains directly. This frontier index is a short, source-cache oriented readback over the current blocking edge.

The 114-row matrix remains the source of truth. This index does not replace the matrix, does not add a new matrix row, and does not write a cache artifact. It narrows the next frontier to eight source facts:

- current 114-row capability matrix
- read-only status tool registration-denial readback
- Dirty worktree owner-freeze source-cache
- Temporal-lite minimal local persistence frontier
- Workflow v1 source-of-truth source-cache
- Plugins v1 contract source-cache
- Tools v1 read-only status tool source-cache
- Controlled-live status canary source-cache

## Contract

The report must prove:

- the matrix is still 114 rows / 113 ready / 1 blocked with live enabled count 0
- the read-only status tool registration-denial readback is queryable for two candidates
- the Dirty worktree owner-freeze source-cache groups current dirty state by lane/owner/risk and keeps clean scoped worktree, git mutation, cleanup, deletion, release, canary, live, and Public GA blocked
- the Temporal-lite event store has local test append/replay coverage while runtime writes and workflow execution stay disabled
- the Workflow v1 source-of-truth source-cache locks the single append-only event store, lease, idempotency, checkpoint, replay validator, WorkGraph projection, and WAL denial path without runtime event-log writes, SQLite writes, WAL writes, lease acquire, idempotency-index writes, checkpoint writes, replay execution, WorkGraph store writes, or live
- the Plugins v1 contract source-cache locks manifest/permission/activation/toolPolicy schema v1, schema migration, signature/trust, install-cache test path, and sandbox enforcement design without install, cache mutation, activation, registration, invocation, or live
- the Tools v1 read-only status tool source-cache locks registration-denial, shadow lookup, feature-gated dry-run, structured status result, approval/ledger/receipt projection, and local append-only store boundary without registration, lookup execution, invocation, ledger/receipt writes, network, credentials, external POST, or live
- the Controlled-live status canary source-cache locks clean worktree, fresh soak, operator approval, credential boundary, transport boundary, rollback rehearsal, and kill-switch rehearsal prerequisites without canary start, live execution, Public GA, credential read, transport mutation, evidence recording, approval acceptance, ledger write, runtime event-log write, SQLite write, or receipt persistence
- direct long-chain expansion is denied for downstream consumers
- registration-denial remains source-cache/frontier evidence instead of becoming another current-reality matrix row

## Side-Effect Boundary

This frontier is report-only. It does not write a cache artifact, mutate the current-reality matrix, persist a canonical index, register tools, execute registry lookup, invoke tools, write ledgers or receipts, write SQLite/runtime event logs, mutate git state, read credentials, mutate transports, start canary, or open live execution.
