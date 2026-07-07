# Controlled Live Evidence Receipt Store Local Evidence Acceptance Source Readback Without Recording

This note documents the controlled live evidence receipt store local evidence acceptance source readback without recording.

## Purpose

The readback takes the local evidence/receipt-store open-preconditions catalog and projects the dev-only evidence acceptance source that would eventually feed controlled-live evidence acceptance. It makes the source id, route, schema, policy, redaction policy, idempotency key, and recording-boundary route queryable while preserving the no-recording boundary.

For each of the seven controlled-live evidence blockers, the readback projects:

- the source local evidence/receipt-store open-precondition set and route
- the source dev evidence acceptance source id and required evidence acceptance key
- the source operator local-store approval, receipt-store feature gate, and append-only store path grant
- a local evidence acceptance source id, route, kind, scope, schema, policy, and redaction policy
- a source idempotency key, readback route, and future recording-boundary route
- explicit no-recording, no-persistence, no-write, and no-live booleans

## Closed Boundary

This is a metadata-only readback gate. It performs no dev evidence acceptance source recording, evidence acceptance recording, evidence record, local receipt-store feature-gate opening, append-only store path grant, atomic append, post-append readback persistence, rollback anchor verification, retention policy commit, replay idempotency guard enablement, receipt-store write attempt record, receipt store write, receipt persistence, ledger write, event-log write, SQLite write, credential read, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay execution, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution.

## Source Cache

The shell gate renders the local evidence/receipt-store open-preconditions source report once into a temporary file, then passes it through `HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_OPEN_PRECONDITIONS_JSON`. The target report exposes `source_cache_mode=provided_source_json`, `source_report_render_count=0`, and `target_source_reuse_count=1` under gate mode.

This cache is temporary and read-only. It does not create persistent cache artifacts, approvals, evidence acceptance sources, evidence records, receipts, ledger rows, SQLite rows, workflow events, local receipt-store feature gates, append-only grants, or local store writes.

## Next

The next reversible slice is `controlled_live_evidence_receipt_store_local_evidence_acceptance_recording_boundary_readback_without_recording`, which should make the source recording boundary queryable while still keeping evidence acceptance, receipt-store writes, and live execution disabled.
