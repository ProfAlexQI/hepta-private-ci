# Hepta Systems Controlled Live Required Evidence Collection Plan - 2026-06-27

This note records Phase 5d for the Hepta systems lane: Controlled Live Required
Evidence Collection Plan without recording evidence.

## Current Facts

The collection plan consumes two read-only sources:

- Phase 5a controlled-live denial readback index
- Phase 5c controlled-live operator packet non-send readback

It lists required evidence for all seven controlled-live blockers:

- dirty worktree boundary
- operator live approval
- fresh soak/readback evidence
- credential boundary attestation
- Gateway/Native/Telegram POST boundary approval
- rollback rehearsal evidence
- kill-switch rehearsal evidence

Every entry is queryable and operator-visible, but the plan remains
`plan_only_no_recording`. It does not accept approvals, read credentials, record
evidence, persist readbacks, waive blockers, or promote cutover.

## Sources

- `codex-rs/hepta-runtime/src/controlled_live_required_evidence_collection_plan.rs`
- `scripts/hepta-systems-controlled-live-required-evidence-collection-plan-report.sh`
- `scripts/hepta-systems-controlled-live-required-evidence-collection-plan-gate.sh`
- `scripts/hepta-systems-controlled-live-readiness-denial-readback-index-report.sh`
- `scripts/hepta-systems-controlled-live-operator-packet-non-send-readback-report.sh`

## Boundaries

This surface is report-only and side-effect-free. It performs no approval
request, approval acceptance, approval recording, evidence recording, evidence
persistence, blocker waiver, credential read, packet send, readback persistence,
ledger write, event-log write, SQLite write, Native POST mutation, Telegram
transport mutation, gateway/auth mutation, channel send, replay, rollback,
package, release, Public GA promotion, or live execution.

The exact closed boundary is: no approval request, approval acceptance, approval recording, evidence recording, evidence persistence, blocker waiver, credential read, packet send, readback persistence, ledger write, event-log write, SQLite write, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, replay, rollback, package, release, Public GA promotion, or live execution.

## Next Move

Phase 5e should build a controlled-live required evidence readback index without
recording. That next surface can make the evidence requirements easier to query
and diff, while still avoiding evidence persistence, credential access,
approval acceptance, blocker waiver, transport mutation, and live execution.
