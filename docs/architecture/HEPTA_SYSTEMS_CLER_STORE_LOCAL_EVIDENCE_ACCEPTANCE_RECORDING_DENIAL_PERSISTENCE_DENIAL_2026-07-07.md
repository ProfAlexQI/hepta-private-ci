# Controlled Live Evidence Receipt Store Local Evidence Acceptance Recording Denial Receipt Persistence Denial Readback Without Persistence

Date: 2026-07-07

## Purpose

This note documents the controlled-live local evidence acceptance recording denial receipt persistence-denial readback. The surface consumes the positive-preconditions readback and turns the missing persistence authority, operator approval, evidence acceptance, grant, append, readback, rollback, retention, and replay guards into an explicit queryable denial.

The canonical surface remains `controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_persistence_denial_readback_without_persistence`. The local implementation uses short script and Rust filenames to keep this lane stable while retaining the public gate and schema names.

## Readback Contract

For each of the seven controlled-live blockers, the readback binds:

- source positive precondition set id and route
- source local evidence acceptance recording denial receipt id, route, and digest
- source acceptance-source record id
- source retention policy id
- source replay idempotency key
- source denial receipt persistence grant precondition id
- derived persistence-denial id, route, and reason

The denial reason is `local_evidence_acceptance_recording_denial_receipt_persistence_disabled_positive_preconditions_missing`.

## Closed Boundary

This readback performs no persistence authority recording, operator persistence approval, evidence acceptance, denial receipt persistence grant, persistence attempt recording, denial receipt persistence, atomic append, post-persist readback persistence, rollback anchor verification, retention policy commit, replay idempotency guard enablement, acceptance-source recording, acceptance-source persistence, evidence acceptance recording, evidence recording, receipt-store write-attempt recording, receipt-store write, receipt persistence, ledger write, event-log write, SQLite write, credential read, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay execution, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution.

## Expected Counts

- `persistence_denial_entry_count=7`
- `persistence_denial_projected_count=7`
- `source_positive_preconditions_attached_count=7`
- `source_denial_receipt_attached_count=7`
- `source_acceptance_source_record_attached_count=7`
- `denial_receipt_persistence_denied_count=7`
- `denial_receipt_persistence_disabled_count=7`
- all nine missing positive-precondition counts equal `7`

The following must remain zero:

- `denial_receipt_persistence_allowed_count`
- `denial_receipt_persistence_attempt_recorded_count`
- `denial_receipt_persisted_count`
- `acceptance_source_recorded_count`
- `acceptance_source_persisted_count`
- `evidence_acceptance_recorded_count`
- `evidence_recorded_count`
- `receipt_store_write_attempt_recorded_count`
- `receipt_store_written_count`
- `receipt_persisted_count`
- `ledger_written_count`
- `workflow_event_log_written_count`
- `sqlite_written_count`
- `live_mutation_allowed_count`

## Source Cache

The gate renders the positive-preconditions source report once and passes it to the target report with `HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_RECORDING_DENIAL_RECEIPT_POSITIVE_PRECONDITIONS_JSON`.

This keeps the target readback queryable without writing cache artifacts, denial receipts, evidence records, receipt-store entries, ledgers, event logs, SQLite rows, or live execution state.

## Next Step

The next reversible slice is `controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_denial_receipt_persistence_denial_retention_replay_readback_without_persistence`, which should add retention/replay invariants for this persistence-denial branch while preserving the same no-persistence boundary.
