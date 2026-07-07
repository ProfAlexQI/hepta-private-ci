# Controlled Live Evidence Receipt Store Local Evidence Acceptance Recording Denial Receipt Readback Without Persistence

This note documents the controlled live evidence receipt-store local evidence acceptance recording denial receipt readback without persistence gate.

The surface is `controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_readback_without_persistence`.

This is the controlled live evidence receipt store local evidence acceptance recording denial receipt readback without persistence gate.

## Intent

The previous readback layer projected the future local evidence acceptance source recording boundary for the seven controlled-live evidence blockers. This layer turns the refused source-recording outcome into a queryable denial receipt projection.

The denial receipts are metadata-only. They are derived from the local evidence acceptance recording boundary readback and remain deliberately unpersisted until the local evidence/receipt-store open prerequisites, evidence acceptance, append-only write grant, atomic append, post-write readback, rollback anchor, retention commit, and replay idempotency guard are satisfied.

## Source

The source report is:

`scripts/hepta-systems-controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-boundary-readback-without-recording-report.sh`

The gate renders that source once and passes it to the target report via:

`HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_RECORDING_BOUNDARY_JSON`

The target report must expose:

- `source_cache_mode=provided_source_json`
- `source_report_render_count=0`
- `target_source_reuse_count=1`

This keeps the receipt-store readback chain in the source-cache pattern introduced earlier in the sprint.

## Projection

For each of the seven controlled-live evidence blockers, the readback projects:

- source recording boundary id and route
- source local evidence acceptance source record id
- source recording idempotency key
- source boundary denial receipt id
- local evidence acceptance recording denial receipt id
- denial receipt readback route
- denial receipt digest
- denial receipt schema version
- denial receipt idempotency key
- denial reason and denied-not-persisted state

The required denial reason is:

`local_evidence_acceptance_source_recording_disabled_open_preconditions_missing`

The denial receipt schema is:

`controlled_live_local_evidence_acceptance_source_recording_denial_receipt_v1`

## Closed Boundary

This gate is intentionally not an opening of the local evidence store. It proves a queryable denial receipt exists in the read model while keeping all effects closed.

Required closed boundary: no local evidence acceptance source recording, acceptance source persistence, denial receipt persistence, evidence acceptance recording, evidence recording, receipt-store write attempt record, receipt store write, receipt persistence, ledger write, event-log write, SQLite write, credential read, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay execution, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution.

## Expected Counts

- `denial_receipt_entry_count=7`
- `denial_receipt_projected_count=7`
- `denial_receipt_digest_projected_count=7`
- `denial_receipt_readback_route_projected_count=7`
- `denial_receipt_idempotency_key_projected_count=7`
- `denial_receipt_idempotency_key_unique_count=7`
- `source_recording_boundary_attached_count=7`
- `source_acceptance_source_record_id_attached_count=7`
- `source_denial_receipt_id_attached_count=7`
- `recording_denial_reason_projected_count=7`
- `recording_precondition_missing_count=7`
- `acceptance_source_recording_disabled_count=7`

The following must remain zero:

- `acceptance_source_recorded_count`
- `acceptance_source_persisted_count`
- `denial_receipt_persisted_count`
- `evidence_acceptance_recorded_count`
- `evidence_recorded_count`
- `receipt_store_write_attempt_recorded_count`
- `receipt_store_written_count`
- `receipt_persisted_count`
- `ledger_written_count`
- `workflow_event_log_written_count`
- `sqlite_written_count`
- `live_mutation_allowed_count`

## Next Step

The next reversible slice is `controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_retention_replay_readback_without_persistence`, which should add retention, expiry, replay, garbage-collection denial, supersession guard, and zero-effect digest invariants while still avoiding persistence and live execution.
