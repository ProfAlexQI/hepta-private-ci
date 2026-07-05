# Hepta Systems Controlled Live Readiness Denial Readback Index - 2026-06-27

This note records Phase 5a of the Hepta systems convergence plan. The
Controlled Live Readiness Denial Readback Index makes every current
controlled-live blocker queryable and operator-facing without accepting,
waiving, persisting, or resolving any blocker.

## Source

The index reads:

- `scripts/hepta-systems-controlled-live-readiness-audit-report.sh`
- `codex-rs/hepta-runtime/src/controlled_live_readiness_audit.rs`
- `codex-rs/hepta-runtime/src/controlled_live_readiness_denial_readback_index.rs`

The source audit must remain `ready_blocked`: controlled-live audit ready,
controlled-live cutover not ready, and seven blockers retained.

## Index Entries

The index exposes seven stable query keys and readback routes:

- `controlled_live.blockers.dirty_worktree_boundary`
- `controlled_live.blockers.operator_live_approval_missing`
- `controlled_live.blockers.fresh_soak_readback_missing`
- `controlled_live.blockers.credential_boundary_attestation_missing`
- `controlled_live.blockers.gateway_native_telegram_post_boundary_approval_missing`
- `controlled_live.blockers.rollback_rehearsal_missing`
- `controlled_live.blockers.kill_switch_rehearsal_missing`

Each entry is operator-facing, operator-recoverable, and blocks cutover. Each
entry records the required evidence in human-readable form while keeping the
current state as `missing`.

## Boundary

This is a readback index only. It performs no waiver, acceptance, approval request, readback persistence, live execution, Native POST mutation, Telegram transport mutation, gateway/auth mutation, replay, rollback, package, release, or Public GA promotion.

It also does not:

- record operator approval
- mutate plugin or tool registries
- invoke tools
- write ledgers
- write workflow event logs or SQLite state
- acquire leases
- mutate idempotency indexes
- write checkpoints
- read credentials
- send channels
- invoke providers or models
- mutate a kill-switch

## Next Move

Phase 5b should produce a controlled-live operator packet preview without
sending an approval request. The packet can assemble scope, payload hash,
rollback owner, blocker readbacks, and required evidence while keeping approval
request, approval recording, persistence, Gateway/Auth mutation, Native POST
mutation, Telegram transport mutation, package/release writes, Public GA, and
live execution disabled.
