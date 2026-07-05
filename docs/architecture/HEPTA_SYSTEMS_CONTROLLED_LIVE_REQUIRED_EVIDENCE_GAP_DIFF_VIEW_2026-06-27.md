# Hepta Systems Controlled Live Required Evidence Gap Diff View - 2026-06-27

This note records Phase 5g for the Hepta systems lane: Controlled Live Required
Evidence Gap Diff View without acceptance.

## Current Facts

The gap diff view consumes the Phase 5f required evidence gap summary and makes
all seven missing controlled-live evidence gaps comparable across readbacks
without accepting evidence.

This surface is comparable across readbacks without accepting evidence.

Each entry carries:

- stable gap key
- diff view key
- comparison anchor
- owner
- risk bucket
- previous state `missing`
- current state `missing`
- state delta `unchanged_missing`
- source query key, readback route, diff key, and fingerprint

The diff view covers the seven controlled-live blockers:

- dirty worktree boundary
- operator live approval
- fresh soak/readback evidence
- credential boundary attestation
- Gateway/Native/Telegram POST boundary approval
- rollback rehearsal evidence
- kill-switch rehearsal evidence

The diff view is operator-facing, queryable, and comparable, but it is not
evidence. It does not accept approval, record evidence, waive blockers, read
credentials, persist readbacks, mutate transports, or start live execution.

## Sources

- `codex-rs/hepta-runtime/src/controlled_live_required_evidence_gap_diff_view.rs`
- `scripts/hepta-systems-controlled-live-required-evidence-gap-diff-view-report.sh`
- `scripts/hepta-systems-controlled-live-required-evidence-gap-diff-view-gate.sh`
- `scripts/hepta-systems-controlled-live-required-evidence-gap-summary-report.sh`

## Boundaries

This surface is report-only and side-effect-free. It performs no approval
request, approval acceptance, approval recording, evidence recording, evidence
persistence, blocker waiver, credential read, readback persistence, ledger
write, event-log write, SQLite write, Native POST mutation, Telegram transport
mutation, gateway/auth mutation, channel send, replay, rollback, package,
release, Public GA promotion, or live execution.

The exact closed boundary is: no approval request, approval acceptance, approval recording, evidence recording, evidence persistence, blocker waiver, credential read, readback persistence, ledger write, event-log write, SQLite write, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, replay, rollback, package, release, Public GA promotion, or live execution.

## Next Move

Phase 5h should produce a controlled-live required evidence gap operator
readback without acceptance. That next surface can present the diff view in a
stable operator-facing form while still avoiding approval acceptance, blocker
waiver, credential access, evidence recording, persistence, transport mutation,
and live execution.
