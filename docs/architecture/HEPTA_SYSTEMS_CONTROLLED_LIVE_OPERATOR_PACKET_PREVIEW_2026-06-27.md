# Hepta Systems Controlled Live Operator Packet Preview - 2026-06-27

This note records Phase 5b for the Hepta systems lane: Controlled Live Operator
Packet Preview without sending an approval request.

## Current Facts

The packet preview consumes the Phase 5a controlled-live denial readback index
and assembles a local, side-effect-free operator packet with six sections:

- scope
- payload hash
- rollback owner
- blocker readbacks
- required evidence
- closed boundary

The preview keeps the packet stable enough for operator-facing readback without
promoting it into an approval request. It uses:

- packet id: `controlled-live-operator-packet-preview`
- scope id: `hepta-system-controlled-live-read-only-chain`
- payload hash: `sha256:controlled-live-operator-packet-preview-no-live-payload`
- rollback owner: `operator-explicit-before-live`

All seven Phase 5a blockers are included in the packet as readback entries:

- `dirty_worktree_boundary`
- `operator_live_approval_missing`
- `fresh_soak_readback_missing`
- `credential_boundary_attestation_missing`
- `gateway_native_telegram_post_boundary_approval_missing`
- `rollback_rehearsal_missing`
- `kill_switch_rehearsal_missing`

Each blocker remains non-accepted and non-waived. The packet is ready as a
preview and blocked as a live cutover.

## Sources

- `codex-rs/hepta-runtime/src/controlled_live_operator_packet_preview.rs`
- `scripts/hepta-systems-controlled-live-operator-packet-preview-report.sh`
- `scripts/hepta-systems-controlled-live-operator-packet-preview-gate.sh`
- `scripts/hepta-systems-controlled-live-readiness-denial-readback-index-report.sh`

## Boundaries

This surface is report-only and side-effect-free. It performs no approval
request, approval recording, packet persistence, readback persistence, blocker
waiver, denial acceptance, live execution, Native POST mutation, Telegram
transport mutation, gateway/auth mutation, replay, rollback, package, release,
or Public GA promotion.

The exact closed boundary is: no approval request, approval recording, packet persistence, readback persistence, blocker waiver, denial acceptance, live execution, Native POST mutation, Telegram transport mutation, gateway/auth mutation, replay, rollback, package, release, or Public GA promotion.

## Next Move

Phase 5c should add a non-send readback around this operator packet preview.
That next surface can prove the packet remains visible and unsent, but it must
continue to avoid approval requests, approval recording, persistence, Gateway or
Auth mutation, Native POST mutation, Telegram transport mutation, package or
release writes, Public GA promotion, and live execution.
