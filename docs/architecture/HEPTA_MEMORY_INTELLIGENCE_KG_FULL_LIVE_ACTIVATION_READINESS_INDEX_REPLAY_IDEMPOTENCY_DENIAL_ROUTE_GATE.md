# Hepta Full Live Activation Readiness Index Replay Idempotency Denial Route Gate

This route exposes the report-only readiness-index replay/idempotency denial slice through the native Hepta gateway:

`/api/hepta-memory-intelligence-kg-full-live-activation-readiness-index-replay-idempotency-denial`

The route is intentionally non-mutating. It confirms that replaying the full live activation readiness index cannot register idempotency keys, write replay caches, register query results, write index entries, export records, record observability, derive operator acceptance, derive activation authority, invoke providers/models, write Memory/KG, read credentials, install/restart, mutate the active binary, publish artifacts, or send externally.

Validation is handled by:

`scripts/hepta-memory-intelligence-kg-full-live-activation-readiness-index-replay-idempotency-denial-route-gate.sh`

The gate checks source report shape, native route wiring, route parity, focused unit coverage, optional live endpoint readiness, and terminal preflight coverage.
