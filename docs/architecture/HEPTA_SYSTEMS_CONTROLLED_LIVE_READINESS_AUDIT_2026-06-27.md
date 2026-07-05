# Hepta Systems Controlled Live Readiness Audit - 2026-06-27

This note records Phase 5 of the Hepta systems convergence plan. The Controlled
Live Readiness Audit is audit ready, cutover blocked. It makes the live cutover
preconditions explicit without opening a mutation path.

## Sources

The audit reads the current local source-of-truth chain:

- `scripts/hepta-systems-plugin-lifecycle-state-machine-report.sh`
- `scripts/hepta-systems-tool-registry-read-only-dispatch-preflight-report.sh`
- `scripts/hepta-systems-workflow-durable-store-adapter-report.sh`
- `scripts/hepta-systems-hepta-system-status-read-only-e2e-report.sh`
- `codex-rs/hepta-runtime/src/live_readiness.rs`
- `codex-rs/hepta-runtime/src/controlled_live_readiness_audit.rs`

The first five preconditions are satisfied only as read-only evidence:

- single source-of-truth is present
- hepta-system status read-only E2E is ready
- Temporal-lite adapter is ready behind a closed feature gate
- replay validation metadata is present while replay execution is disabled
- rollback metadata is present while rollback execution is disabled

## Blockers

The audit keeps controlled live blocked on seven conditions:

- dirty worktree boundary
- missing explicit operator live approval packet
- missing fresh soak/readback evidence
- missing credential boundary attestation
- missing Gateway, Native POST, and Telegram live mutation boundary approval
- missing rollback rehearsal evidence
- missing kill-switch rehearsal evidence

These blockers are operator-recoverable, but none is waived by this gate.

## Boundary

This is a report-only audit. It performs no live execution, Native POST mutation, Telegram transport mutation, gateway/auth mutation, tool invocation, ledger write, approval request, receipt persistence, event-log write, SQLite write, replay, rollback, package, release, or Public GA promotion.

It also does not:

- install plugins
- mutate plugin cache
- register tools
- acquire leases
- mutate idempotency indexes
- write checkpoints
- read credentials
- send channels
- invoke providers or models
- record approval or rollback rehearsal evidence
- mutate a kill-switch

## Next Move

Phase 5a should produce a controlled-live readiness denial readback index
without cutover. That readback should make every blocker queryable and
operator-facing while keeping live execution, registration, invocation,
persistence, Gateway/Auth mutation, Native POST mutation, Telegram transport
mutation, package/release writes, and Public GA disabled.
