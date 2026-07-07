# Controlled Live Evidence Receipt Store Local Evidence Acceptance Authority Decision Recording Denial Receipt Retention Replay Readback Without Persistence

This note documents the controlled live evidence receipt store local evidence acceptance authority decision recording denial receipt retention replay readback without persistence.

The readback consumes `controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_readback_without_persistence` and adds queryable retention/replay invariants for projected denial receipts. It does not persist retention policy, write a replay index, persist the denial receipt, record an authority decision, send the operator packet, open local evidence acceptance, or open live execution.

## Source

Source report:
`scripts/hepta-systems-cler-store-local-evidence-acceptance-authority-decision-recording-denial-receipt-report.sh`

Source surface:
`controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_readback_without_persistence`

The target gate pre-renders the source JSON once and passes it with `HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_DECISION_RECORDING_DENIAL_RECEIPT_JSON`. The target report must then show `source_cache_mode=provided_source_json`, `source_report_render_count=0`, and `target_source_reuse_count=1`.

## Retention Replay Projection

The readback projects 7 retention/replay rows:

- source denial receipt id, route, digest, schema version, and idempotency key
- source recording boundary, terminal closeout, persistence-denial, packet persistence-denial receipt, non-send readback, authority packet, authority decision request, non-authority receipt, and authority decision record binding
- retention policy id and readback route
- expiry guard id
- replay key and replay idempotency key
- replay and retention readback routes
- garbage-collection denial id
- supersession guard id
- zero-effect digest
- projected-not-persisted retention state and projected-not-written replay state

Expected gate counts:

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
- `source_recording_boundary_attached_entry_count=7`
- `source_authority_packet_attached_entry_count=7`
- `source_authority_decision_request_attached_entry_count=7`

Expected closed-path counts:

- `retention_policy_persisted_count=0`
- `replay_index_written_count=0`
- `expiry_enforced_count=0`
- `garbage_collection_performed_count=0`
- `authority_decision_recorded_count=0`
- `authority_decision_persisted_count=0`
- `denial_receipt_persisted_count=0`
- `operator_packet_sent_count=0`
- `operator_packet_persisted_count=0`
- `non_authority_receipt_persisted_count=0`
- `local_evidence_acceptance_authority_present_count=0`
- `local_evidence_acceptance_allowed_count=0`
- `local_evidence_acceptance_recorded_count=0`
- `evidence_acceptance_recorded_count=0`
- `evidence_recorded_count=0`
- `receipt_store_write_attempt_recorded_count=0`
- `receipt_store_written_count=0`
- `receipt_persisted_count=0`
- `ledger_written_count=0`
- `workflow_event_log_written_count=0`
- `sqlite_written_count=0`
- `live_mutation_allowed_count=0`

## Closed Boundary

This surface intentionally performs no retention policy persistence, replay index write, expiry enforcement, garbage collection, authority decision recording, authority decision persistence, denial receipt persistence, operator packet send, operator packet persistence, non-authority receipt persistence, local evidence acceptance authority, local evidence acceptance, local evidence acceptance recording, evidence acceptance recording, evidence recording, receipt-store write-attempt recording, receipt store write, receipt persistence, ledger write, event-log write, SQLite write, credential read, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay execution, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution.

The side-effect map must remain all false. The dashboard should remain blocked with evidence missing and live closed.

## Next

The next reversible slice is `controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_positive_preconditions_readback_without_persistence`, which should stay query-only until the local evidence authority and receipt-store persistence boundaries are separately opened by scoped evidence and explicit command.
