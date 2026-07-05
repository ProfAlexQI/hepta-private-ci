# Hepta Systems Controlled Live Required Evidence Gap Operator Readback - 2026-06-27

This note records Phase 5h for the Hepta systems lane: Controlled Live Required
Evidence Gap Operator Readback without acceptance.

## Current Facts

The operator readback consumes the Phase 5g required evidence gap diff view and
presents all seven unchanged missing controlled-live evidence gaps as a stable
operator-facing readback without accepting evidence.

This surface is an operator-facing readback without accepting evidence.

Each entry carries:

- stable operator readback key
- stable operator readback route
- operator display order
- operator status `blocked_missing_evidence`
- operator action `collect_required_evidence_before_live_cutover`
- source gap key, diff view key, comparison anchor, readback route, diff key, and fingerprint
- previous state `missing`
- current state `missing`
- state delta `unchanged_missing`

The operator readback covers the seven controlled-live blockers:

- dirty worktree boundary
- operator live approval
- fresh soak/readback evidence
- credential boundary attestation
- Gateway/Native/Telegram POST boundary approval
- rollback rehearsal evidence
- kill-switch rehearsal evidence

The operator readback is visible, queryable, and comparable, but it is not
evidence and it is not persisted. It does not accept approval, record evidence,
waive blockers, read credentials, persist readbacks, mutate transports, or start
live execution.

## Sources

- `codex-rs/hepta-runtime/src/controlled_live_required_evidence_gap_operator_readback.rs`
- `scripts/hepta-systems-controlled-live-required-evidence-gap-operator-readback-report.sh`
- `scripts/hepta-systems-controlled-live-required-evidence-gap-operator-readback-gate.sh`
- `scripts/hepta-systems-controlled-live-required-evidence-gap-diff-view-report.sh`

## Boundaries

This surface is report-only and side-effect-free. It performs no approval
request, approval acceptance, approval recording, evidence recording, evidence
persistence, blocker waiver, credential read, readback persistence, ledger
write, event-log write, SQLite write, Native POST mutation, Telegram transport
mutation, gateway/auth mutation, channel send, replay, rollback, package,
release, Public GA promotion, or live execution.

The exact closed boundary is: no approval request, approval acceptance, approval recording, evidence recording, evidence persistence, blocker waiver, credential read, readback persistence, ledger write, event-log write, SQLite write, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, replay, rollback, package, release, Public GA promotion, or live execution.

## Next Move

Phase 5i should produce a controlled-live required evidence gap operator packet
attachment without acceptance. That next surface can attach the operator
readback to the local operator packet preview while still avoiding approval
acceptance, blocker waiver, credential access, evidence recording, persistence,
transport mutation, and live execution.
