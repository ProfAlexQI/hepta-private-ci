# Public GA Operator Identity/Session Retention Expiry GC Final Index

This final index consumes the Public GA operator identity/session retention expiry GC readback and exposes a stable ready-but-blocked terminal surface for the no-retention/no-expiry/no-GC boundary.

Status: ready-but-blocked.

The final index carries the canonical terminal closure backfeed from the audit evidence final index through the retention expiry GC attachment/readback: 17 release/live blockers across 4 ready categories, with 17 categorized blockers. The `runner_selector` and `dirty_worktree_owner_freeze` categories each remain queryable with 2 blockers. The local final blocker count remains 38.

The final index does not invoke the retention/expiry/garbage-collection gate, the audit/evidence gate, long soak gates, Public GA readiness gates, or terminal live gates.

It confirms retention policies, TTL leases, expiry timestamps, expiry timers, expiry acknowledgements, garbage-collection queues, GC scans, GC decisions, tombstone/delete-marker GC, archives, compaction, retention authority, result receipts, release authority derivation, activation authority derivation, release publication, rollback execution, and Public GA claims remain false.

The next migration step is `attach_public_ga_operator_identity_session_retention_expiry_gc_final_index_to_public_ga_operator_identity_session_export_query_observability_without_retention`.
