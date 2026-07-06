# Public GA Operator Identity/Session Retention Expiry GC Attachment

This attachment consumes the Public GA operator identity/session audit evidence final index and source-probes the existing retention/expiry/garbage-collection denial gate.

Status: ready-but-blocked.

The attachment carries the canonical terminal closure backfeed from the audit evidence final index: 17 release/live blockers across 4 ready categories, with 17 categorized blockers. The `runner_selector` and `dirty_worktree_owner_freeze` categories each remain queryable with 2 blockers.

The attachment does not invoke the retention/expiry/garbage-collection gate, the audit/evidence gate, long soak gates, Public GA readiness gates, or terminal live gates.

It keeps retention policies, TTL leases, expiry timestamps, schedulers, timers, expiry acknowledgements, garbage-collection queues, GC scans, GC decisions, archives, compaction, retention authority, result receipts, release authority, activation authority, and Public GA claims blocked.

The only next step is a static readback/final-index closure for the same no-retention facts.
