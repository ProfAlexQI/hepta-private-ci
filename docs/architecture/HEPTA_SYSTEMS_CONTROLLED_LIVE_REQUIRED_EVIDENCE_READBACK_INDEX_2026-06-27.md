# Hepta Systems Controlled Live Required Evidence Readback Index - 2026-06-27

This note records Phase 5e for the Hepta systems lane: Controlled Live Required
Evidence Readback Index without recording evidence.

## Current Facts

The readback index consumes the Phase 5d required evidence collection plan and
makes all seven evidence requirements queryable and diffable without recording evidence.

Each entry carries:

- stable query key
- stable readback route
- stable diff key
- deterministic local fingerprint
- evidence state `missing`

The index covers the seven controlled-live blockers:

- dirty worktree boundary
- operator live approval
- fresh soak/readback evidence
- credential boundary attestation
- Gateway/Native/Telegram POST boundary approval
- rollback rehearsal evidence
- kill-switch rehearsal evidence

The fingerprints identify required-evidence metadata only. They are not evidence
receipts and do not imply that any evidence has been recorded.

## Sources

- `codex-rs/hepta-runtime/src/controlled_live_required_evidence_readback_index.rs`
- `scripts/hepta-systems-controlled-live-required-evidence-readback-index-report.sh`
- `scripts/hepta-systems-controlled-live-required-evidence-readback-index-gate.sh`
- `scripts/hepta-systems-controlled-live-required-evidence-collection-plan-report.sh`

## Boundaries

This surface is report-only and side-effect-free. It performs no approval
request, approval acceptance, approval recording, evidence recording, evidence
persistence, blocker waiver, credential read, readback persistence, ledger
write, event-log write, SQLite write, Native POST mutation, Telegram transport
mutation, gateway/auth mutation, channel send, replay, rollback, package,
release, Public GA promotion, or live execution.

The exact closed boundary is: no approval request, approval acceptance, approval recording, evidence recording, evidence persistence, blocker waiver, credential read, readback persistence, ledger write, event-log write, SQLite write, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, replay, rollback, package, release, Public GA promotion, or live execution.

## Next Move

Phase 5f should produce a controlled-live required evidence gap summary without
acceptance. That next surface can group the seven missing requirements by owner
and cutover risk, while still avoiding approval acceptance, blocker waiver,
credential access, evidence recording, persistence, transport mutation, and live
execution.
