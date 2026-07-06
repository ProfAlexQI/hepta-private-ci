# Hepta Systems Controlled Live Required Evidence Gap Operator Packet Attachment - 2026-06-27

This note records Phase 5i for the Hepta systems lane: Controlled Live Required
Evidence Gap Operator Packet Attachment without acceptance.

## Current Facts

The operator packet attachment consumes the Phase 5b controlled-live operator
packet preview and the Phase 5h required evidence gap operator readback. It
attaches all seven unchanged missing evidence gap readbacks to the local packet
preview as an operator packet attachment without accepting evidence.

This surface is an operator packet attachment without accepting evidence.

Each entry carries:

- source packet id
- source packet payload hash
- stable attachment key
- stable attachment route
- source operator readback key and route
- operator display order
- operator status `blocked_missing_evidence`
- previous state `missing`
- current state `missing`
- state delta `unchanged_missing`

The attachment covers the seven controlled-live blockers:

- dirty worktree boundary
- operator live approval
- fresh soak/readback evidence
- credential boundary attestation
- Gateway/Native/Telegram POST boundary approval
- rollback rehearsal evidence
- kill-switch rehearsal evidence

The attachment is visible, queryable, and comparable, but it is not evidence, it
is not an approval request, it is not sent, and it is not persisted. It does not
accept approval, record evidence, waive blockers, read credentials, persist the
packet, persist attachments, persist readbacks, mutate transports, or start live
execution.

## Sources

- `codex-rs/hepta-runtime/src/controlled_live_required_evidence_gap_operator_packet_attachment.rs`
- `scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-report.sh`
- `scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-gate.sh`
- `scripts/hepta-systems-controlled-live-operator-packet-preview-report.sh`
- `scripts/hepta-systems-controlled-live-required-evidence-gap-operator-readback-report.sh`

## Boundaries

This surface is report-only and side-effect-free. It performs no approval
request, approval acceptance, approval recording, evidence recording, evidence
persistence, blocker waiver, credential read, packet persistence, attachment
persistence, readback persistence, ledger write, event-log write, SQLite write,
Native POST mutation, Telegram transport mutation, gateway/auth mutation,
channel send, replay, rollback, package, release, Public GA promotion, or live
execution.

The exact closed boundary is: no approval request, approval acceptance, approval recording, evidence recording, evidence persistence, blocker waiver, credential read, packet persistence, attachment persistence, readback persistence, ledger write, event-log write, SQLite write, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, replay, rollback, package, release, Public GA promotion, or live execution.

## Next Move

Phase 5j should produce a controlled-live required evidence gap operator packet
attachment non-send readback without acceptance. That next surface can prove the
attachment is visible, unsent, unpersisted, and still not an approval request,
while still avoiding approval acceptance, blocker waiver, credential access,
evidence recording, persistence, transport mutation, and live execution.
