# Controlled Live Evidence Receipt Store Local Evidence Acceptance Recording Denial Receipt Retention Replay Readback Without Persistence

This note documents the controlled live evidence receipt-store local evidence acceptance recording denial receipt retention replay readback without persistence gate.

The surface is `controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_retention_replay_readback_without_persistence`.

This is the controlled live evidence receipt store local evidence acceptance recording denial receipt retention replay readback without persistence gate.

## Intent

The previous readback layer projected queryable local evidence acceptance recording denial receipts for the seven controlled-live evidence blockers. This layer adds the next reversible envelope around those denial receipts: retention policy, expiry guard, replay key, replay idempotency key, retention readback route, replay readback route, garbage-collection denial, supersession guard, and zero-effect digest.

The layer is still deliberately readback-only. It proves that each denial receipt can be described with retention and replay metadata without opening any persistence, replay, cleanup, local receipt-store write, live execution, provider, model, native gateway, Telegram, or credential boundary.

## Source

The source report is:

`scripts/hepta-systems-controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial-receipt-readback-without-persistence-report.sh`

The gate renders that source once and passes it to the target report via:

`HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_RECORDING_DENIAL_RECEIPT_JSON`

The target report must expose:

- `source_cache_mode=provided_source_json`
- `source_report_render_count=0`
- `target_source_reuse_count=1`

This preserves the source-cache readback pattern and prevents the retention-replay layer from deriving from an independently rerendered upstream snapshot.

## Projection

For each of the seven controlled-live evidence blockers, the readback projects:

- source denial receipt id, route, digest, and idempotency key
- source acceptance-source record id and idempotency key
- retention policy id and route
- expiry guard id
- replay key and replay idempotency key
- replay readback route
- retention readback route
- garbage-collection denial id
- supersession guard id
- zero-effect digest
- retention state `projected_not_persisted`
- replay state `projected_not_written`

The retention replay collection is:

`controlled-live-evidence-receipt-store-local-evidence-acceptance-recording-denial-retention-replay`

The retention replay collection route is:

`readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/recording-denial-receipts/retention-replay`

## Closed Boundary

This gate is intentionally not a retention commit and not a replay index write. It only proves that retention and replay metadata can be derived from the denial receipt readback while all mutation paths remain closed.

Required closed boundary: no retention policy persistence, replay index write, expiry enforcement, garbage collection, local evidence acceptance source recording, acceptance source persistence, denial receipt persistence, evidence acceptance recording, evidence recording, receipt-store write attempt record, receipt store write, receipt persistence, ledger write, event-log write, SQLite write, credential read, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay execution, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution.

## Expected Counts

- `retention_replay_entry_count=7`
- `retention_policy_projected_count=7`
- `expiry_guard_projected_count=7`
- `replay_key_projected_count=7`
- `replay_idempotency_key_projected_count=7`
- `replay_idempotency_key_unique_count=7`
- `retention_readback_route_projected_count=7`
- `replay_readback_route_projected_count=7`
- `garbage_collection_denial_projected_count=7`
- `supersession_guard_projected_count=7`
- `zero_effect_digest_projected_count=7`
- `source_denial_receipt_attached_count=7`
- `source_acceptance_source_record_attached_count=7`

The following must remain zero:

- `retention_policy_persisted_count`
- `replay_index_written_count`
- `expiry_enforced_count`
- `garbage_collection_performed_count`
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

The next reversible slice is `controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_positive_preconditions_readback_without_persistence`, which should make the positive preconditions explicit while still refusing persistence and live execution.
