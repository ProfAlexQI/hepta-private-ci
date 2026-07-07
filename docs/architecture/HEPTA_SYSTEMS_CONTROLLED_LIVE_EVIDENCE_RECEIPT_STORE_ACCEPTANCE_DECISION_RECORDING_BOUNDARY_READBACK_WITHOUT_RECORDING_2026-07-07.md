# Controlled Live Evidence Receipt Store Acceptance Decision Recording Boundary Readback Without Recording

This note records a local-only controlled-live readback surface for the Hepta
systems lane. It consumes the operator acceptance packet readback and projects
the boundary that would govern acceptance decision recording, without recording
or persisting a decision.

## Scope

The controlled live evidence receipt store acceptance decision recording
boundary readback without recording is `ready_blocked`. It is ready as a
recording-boundary contract and blocked as approval, acceptance, persistence,
and live cutover.

The boundary covers the same seven unchanged-missing controlled-live evidence
gaps:

- `dirty_worktree_boundary`
- `operator_live_approval_missing`
- `fresh_soak_readback_missing`
- `credential_boundary_attestation_missing`
- `gateway_native_telegram_post_boundary_approval_missing`
- `rollback_rehearsal_missing`
- `kill_switch_rehearsal_missing`

Each entry keeps the source acceptance-decision request from the operator
acceptance packet and adds a recording-boundary route:

- `readback://controlled-live/evidence-receipt-store/acceptance-decision-recording-boundary/<blocker-id>`

The shared recording boundary identity is:

- boundary id: `controlled-live-evidence-receipt-store-acceptance-decision-recording-boundary`
- boundary route: `readback://controlled-live/evidence-receipt-store/acceptance-decision-recording-boundary`
- decision record schema: `controlled_live_evidence_receipt_store_acceptance_decision_record_v1`

## Recording Contract

Each entry projects:

- the source packet entry id and source acceptance-decision request route
- the future acceptance decision record id
- the acceptance decision idempotency key
- the post-record readback route
- the rollback anchor for a future recording append
- the denial receipt id for this still-closed recording boundary

The contract makes operator acceptance and evidence acceptance explicit
preconditions. Both remain absent for all seven entries, so decision recording
is denied for all seven entries.

## Boundary

This is a controlled live evidence receipt store acceptance decision recording
boundary readback without recording. It deliberately performs no acceptance
decision recording, acceptance decision persistence, denial receipt
persistence, evidence recording, evidence persistence, receipt store write,
receipt persistence, ledger write, event-log write, SQLite write, credential
read, operator packet send, operator packet persistence, approval request,
approval acceptance, Native POST mutation, Telegram transport mutation,
gateway/auth mutation, channel send, provider call, model call, replay,
rollback, kill-switch rehearsal execution, kill-switch mutation, package,
release, Public GA promotion, or live execution.

Gate phrase: controlled live evidence receipt store acceptance decision recording boundary readback without recording.

Closed boundary: no acceptance decision recording, acceptance decision persistence, denial receipt persistence, evidence recording, evidence persistence, receipt store write, receipt persistence, ledger write, event-log write, SQLite write, credential read, operator packet send, operator packet persistence, approval request, approval acceptance, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution.

## Expected Counts

- boundary entries: 7
- projected boundaries: 7
- ready boundaries: 7
- projected decision record schemas: 7
- attached acceptance-decision requests: 7
- required operator acceptances: 7
- present operator acceptances: 0
- required evidence acceptances: 7
- present evidence acceptances: 0
- missing recording preconditions: 7
- allowed decision recordings: 0
- recorded acceptance decisions: 0
- persisted acceptance decisions: 0
- projected idempotency keys: 7
- unique idempotency keys: 7
- projected post-record readbacks: 7
- projected rollback anchors: 7
- projected denial receipts: 7
- persisted denial receipts: 0
- receipt store writes: 0
- persisted receipts: 0
- ledger writes: 0
- event-log writes: 0
- SQLite writes: 0
- live mutations: 0

## Verification

The local gate validates:

- The operator acceptance packet source remains `ready_blocked`.
- Seven acceptance decision recording boundary entries are projected from the
  source packet.
- Every entry remains `missing -> missing` and `blocked_missing_evidence`.
- Operator acceptance and evidence acceptance are required and absent.
- The decision record schema, idempotency key, post-record readback route, and
  rollback anchor are projected for each entry.
- Acceptance decision recording, decision persistence, denial receipt
  persistence, evidence recording, receipt-store write, receipt persistence,
  ledger write, event-log write, SQLite write, credential read, transport
  mutation, replay, rollback, kill-switch mutation, and live execution remain
  disabled.
- The targeted hepta-runtime Rust tests pass.

## Next Move

The next local-only gate should be
`controlled_live_evidence_receipt_store_recording_denial_receipt_readback_without_persistence`.
It should make the refusal to record queryable as a denial receipt projection,
while still refusing acceptance, evidence recording, receipt persistence,
ledger writes, SQLite writes, transport mutation, canary, and live execution.
