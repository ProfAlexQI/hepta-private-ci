# Hepta Systems Controlled Canary Readiness Plan

Date: 2026-06-27

## Intent

Phase 10 plans the controlled canary boundary for the internal read-only
`hepta-system status` path. It does not activate the canary and does not open
Gateway/Auth, Native POST, Telegram transport, channel send, credential reads,
persistence, Public GA, or live execution.

This phase exists to turn the Phase 9 approval protocol and the existing
controlled-live blocker readbacks into a single canary readiness plan.

## Sources

The plan consumes three side-effect-free sources:

- Phase 9 operator approval protocol:
  `scripts/hepta-systems-hepta-system-status-operator-approval-protocol-report.sh`
- Phase 5n kill-switch rehearsal boundary readback:
  `scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-kill-switch-rehearsal-boundary-readback-report.sh`
- Status canary start guard:
  `status-canary-start-guard/hepta-system-status/v1`

It intentionally avoids consuming the operator readiness dashboard report so the
current reality matrix can consume this report without report recursion.

## Scope

The canary scope is:

- canary id: `controlled-canary.hepta-system-status.internal-read-only.v1`
- route: `canary://hepta-system/status/internal-read-only/readiness-plan`
- allowed surface: `internal_read_only_status_payload`
- activation mode: `plan_only_no_activation`

Gate phrase: plan_only_no_activation.

The plan carries seven blocker references into canary readiness:

- `dirty_worktree_boundary`
- `operator_live_approval_missing`
- `fresh_soak_readback_missing`
- `credential_boundary_attestation_missing`
- `gateway_native_telegram_post_boundary_approval_missing`
- `rollback_rehearsal_missing`
- `kill_switch_rehearsal_missing`

All seven entries remain `missing_required_evidence` and continue to block
canary activation.

## Start Guard

The controlled canary readiness plan is now bound to
`StatusCanaryStartGuard`. The guard consumes the status-canary evidence packet
and keeps the actual start path closed unless both of these are true:

- the seven-item evidence packet is complete
- an independent status canary start switch is enabled

Current state:

- `status_canary_start_guard_bound=true`
- `status_canary_start_guard_route=status_canary_start_blocked_missing_evidence_packet`
- `status_canary_start_guard_switch_enabled=false`
- `status_canary_start_guard_evidence_complete=false`
- `status_canary_start_guard_missing_evidence_count=7`
- `status_canary_start_guard_blocked=true`
- `status_canary_start_guard_allowed=false`

## Boundary

This is a local read-model only. There is no Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, credential read, persistence, Public GA, or live activation.

The plan also keeps approval request, approval acceptance, approval recording,
approval broker writes, evidence recording, evidence persistence, blocker
waiver, workflow event-log writes, SQLite writes, provider invocation, model
invocation, package/release writes, and live execution disabled.

## Gate

Local gate:

```bash
scripts/hepta-systems-controlled-canary-readiness-plan-gate.sh
```

The gate verifies:

- Phase 9 approval protocol is ready-blocked and has one packet.
- Phase 5n kill-switch boundary readback is ready-blocked with seven missing
  evidence entries.
- Seven canary plan entries are operator-visible and queryable.
- Dirty worktree, soak/readback, rollback rehearsal, kill-switch rehearsal,
  credential boundary, and Gateway/Native/Telegram boundary blockers remain
  preserved.
- Status canary start guard is bound, side-effect-free, and blocked by the
  missing evidence packet while the independent start switch remains closed.
- No approval, credential, transport, persistence, Public GA, live activation,
  or live execution path is opened.
- Targeted hepta-runtime Rust tests pass.

## Next Step

Next migration step:
`phase11_dirty_worktree_release_boundary_inventory_without_git_mutation`.

Phase 11 should inventory the dirty worktree/release boundary without staging,
committing, reverting, deleting, or otherwise mutating user or sibling-agent
work.
