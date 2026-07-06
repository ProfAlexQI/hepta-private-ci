# Public GA Operator Identity/Session Intent/Consent Evidence Artifact Signing Terminal Public Claim Delivery Receipt Replay/Idempotency Readback

This readback consumes the artifact signing terminal public claim delivery
receipt replay/idempotency attachment and records a static readback snapshot.

The readback is ready-but-blocked. It does not invoke the replay/idempotency
denial gate, delivery receipt non-persistence denial gate, Public GA readiness,
terminal live gates, live URL reads, long soak, release publication, artifact
writes, approval requests, external delivery, Telegram delivery, install,
restart, or active binary mutation.

The expected local state is:

- attachment attached: true
- replay/idempotency denial gate present: true
- replay/idempotency denial gate invoked: false
- delivery receipt replay recorded/persisted/performed: false
- duplicate receipt and idempotency key/state recorded: false
- replay nonce, cross-scope reuse, status upgrade, acknowledgement replay, and
  hash-status rebind accepted: false
- replay-derived approval or authority: false
- install/restart/active-binary mutation from replay: false
- readback blocker count: 94

Next local step: derive the final index without receipt replay or idempotency
acceptance.
