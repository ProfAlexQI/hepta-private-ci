# Public GA Operator Identity/Session Retention Expiry GC Readback

This readback consumes the Public GA operator identity/session retention expiry GC attachment as a static snapshot.

Status: ready-but-blocked.

The readback carries the canonical terminal closure backfeed from the audit evidence final index through the retention expiry GC attachment: 17 release/live blockers across 4 ready categories, with 17 categorized blockers. The `runner_selector` and `dirty_worktree_owner_freeze` categories each remain queryable with 2 blockers. This expands the static readback check count from 38 to 44 without changing the local blocker count.

The readback does not invoke the retention/expiry/garbage-collection gate, the audit/evidence gate, long soak gates, Public GA readiness gates, or terminal live gates.

It confirms retention policies, TTL leases, expiry timestamps, expiry timers, garbage-collection queues, GC scans, GC decisions, archives, compaction, retention authority, release publication, rollback execution, and Public GA claims remain false.

The only next step is a final index that preserves the same no-retention boundary.
