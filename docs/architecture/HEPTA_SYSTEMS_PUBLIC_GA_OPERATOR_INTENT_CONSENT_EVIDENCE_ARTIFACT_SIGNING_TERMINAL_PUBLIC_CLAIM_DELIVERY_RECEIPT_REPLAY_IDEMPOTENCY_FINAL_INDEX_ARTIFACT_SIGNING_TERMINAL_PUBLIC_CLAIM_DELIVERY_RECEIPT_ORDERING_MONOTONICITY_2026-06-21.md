# Public GA Operator Intent/Consent Evidence Artifact Signing Terminal Public Claim Delivery Receipt Replay/Idempotency Final Index Attachment

This attachment consumes the artifact signing terminal public claim delivery
receipt replay/idempotency final index and exposes the next local denial target:
artifact signing terminal public claim delivery receipt ordering/monotonicity.

The attachment is ready-but-blocked. It source-probes the target denial gate and
architecture note but does not invoke the ordering/monotonicity gate, the
replay/idempotency denial gate, Public GA readiness, public-claim gates,
terminal live gates, transport delivery, or any external send path.

Current contract:

- source replay/idempotency final index ready=true and blocked=true
- target ordering/monotonicity denial gate present=true
- target ordering/monotonicity denial doc present=true
- target denial gate invoked=false
- replay/idempotency denial gate invoked=false
- ordering, sequence cursor, monotonicity state, latest-wins overwrite, ordered
  status, ordered acknowledgement, ordered ledger/index, query/export,
  observability, and hash/status acceptance all false
- release-publication authority, activation authority, install/restart, and
  active-binary mutation all false
- `public_ga_claim_allowed=false`
- `public_release_published=false`
- `attachment_blocker_count=96`

The next local step is the readback projection. It must remain report-only and
must not record, persist, or replay a delivery receipt order.
