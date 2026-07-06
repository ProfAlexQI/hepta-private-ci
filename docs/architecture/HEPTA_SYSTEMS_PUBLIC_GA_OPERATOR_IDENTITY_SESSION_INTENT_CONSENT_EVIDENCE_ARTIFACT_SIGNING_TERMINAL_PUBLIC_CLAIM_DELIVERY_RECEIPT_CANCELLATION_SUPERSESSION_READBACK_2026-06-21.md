# Public GA Operator Identity/Session Intent/Consent Evidence Artifact Signing Terminal Public Claim Delivery Receipt Cancellation/Supersession Readback

This readback consumes the artifact signing terminal public claim delivery
receipt cancellation/supersession attachment and projects its denial state into
the identity/session readback layer.

The readback is ready-but-blocked. It does not invoke the
cancellation/supersession target gate, the ordering/monotonicity target gate,
Public GA readiness, public-claim gates, terminal live gates, transport
delivery, or external sends.

Readback facts:

- source attachment ready=true and blocked=true
- cancellation/supersession denial gate present=true
- cancellation/supersession denial doc present=true
- cancellation/supersession denial gate invoked=false
- cancellation, supersession, withdrawal, replacement receipt, tombstone, delete
  marker, lifecycle state, result receipt, external/Telegram supersession, and
  readback receipt backfill all false
- release-publication authority, activation authority, install/restart, and
  active-binary mutation all false
- `readback_check_count=98`
- `public_ga_claim_allowed=false`
- `public_release_published=false`

The next local step is the final index projection. It must stay report-only and
must not record, persist, cancel, replace, or supersede a delivery receipt.
