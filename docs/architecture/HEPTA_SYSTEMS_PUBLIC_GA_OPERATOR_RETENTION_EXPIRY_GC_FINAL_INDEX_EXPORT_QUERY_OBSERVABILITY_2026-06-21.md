# Public GA Operator Identity/Session Export Query Observability Attachment

This attachment consumes the Public GA operator identity/session retention expiry GC final index and source-probes the operator identity/session export/query/observability denial gate.

Status: ready-but-blocked.

The attachment carries the canonical terminal closure backfeed from the retention expiry GC final index: 17 release/live blockers across 4 ready categories, with 17 categorized blockers. The `runner_selector` and `dirty_worktree_owner_freeze` categories each remain queryable with 2 blockers.

The attachment does not invoke the export/query/observability denial gate, the retention/expiry/garbage-collection gate, audit/evidence gates, long soak gates, Public GA readiness gates, or terminal live gates.

It confirms query registration, query execution, query results, search indexes, export acceptance, export snapshots, export files, export streams, observability metrics, logs, traces, events, dashboards, alerts, SLOs, operator summaries, readback surfaces, audit views, ledger/index/delivery observability, export/query/observability acceptance, release authority, activation authority, install/restart actions, active binary mutation, external sends, release publication, rollback execution, and Public GA claims remain false.

The next migration step is `derive_public_ga_operator_identity_session_export_query_observability_readback_without_retention`.
