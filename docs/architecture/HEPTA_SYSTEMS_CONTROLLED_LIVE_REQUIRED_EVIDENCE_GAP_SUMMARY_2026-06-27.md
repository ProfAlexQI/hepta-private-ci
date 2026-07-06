# Hepta Systems Controlled Live Required Evidence Gap Summary - 2026-06-27

This note records Phase 5f for the Hepta systems lane: Controlled Live Required
Evidence Gap Summary without acceptance.

## Current Facts

The gap summary consumes the Phase 5e required evidence readback index and
groups all seven missing controlled-live evidence requirements by owner and
cutover risk without accepting evidence.

This surface summarizes by owner and cutover risk without accepting evidence.

Each entry carries:

- stable gap key
- owner
- risk bucket
- cutover risk text
- source query key, readback route, diff key, and fingerprint
- evidence state `missing`

The summary covers the seven controlled-live blockers:

- dirty worktree boundary
- operator live approval
- fresh soak/readback evidence
- credential boundary attestation
- Gateway/Native/Telegram POST boundary approval
- rollback rehearsal evidence
- kill-switch rehearsal evidence

The gap summary is operator-facing and queryable, but it is not evidence. It
does not accept approval, record evidence, waive blockers, read credentials,
persist readbacks, mutate transports, or start live execution.

## Sources

- `codex-rs/hepta-runtime/src/controlled_live_required_evidence_gap_summary.rs`
- `scripts/hepta-systems-controlled-live-required-evidence-gap-summary-report.sh`
- `scripts/hepta-systems-controlled-live-required-evidence-gap-summary-gate.sh`
- `scripts/hepta-systems-controlled-live-required-evidence-readback-index-report.sh`

## Boundaries

This surface is report-only and side-effect-free. It performs no approval
request, approval acceptance, approval recording, evidence recording, evidence
persistence, blocker waiver, credential read, readback persistence, ledger
write, event-log write, SQLite write, Native POST mutation, Telegram transport
mutation, gateway/auth mutation, channel send, replay, rollback, package,
release, Public GA promotion, or live execution.

The exact closed boundary is: no approval request, approval acceptance, approval recording, evidence recording, evidence persistence, blocker waiver, credential read, readback persistence, ledger write, event-log write, SQLite write, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, replay, rollback, package, release, Public GA promotion, or live execution.

## Next Move

Phase 5g should produce a controlled-live required evidence gap diff view
without acceptance. That next surface can make the owner/risk gaps easier to
compare across readbacks while still avoiding approval acceptance, blocker
waiver, credential access, evidence recording, persistence, transport mutation,
and live execution.
