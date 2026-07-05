# Public GA Operator Identity/Session Reinstatement Denial Attachment

This attachment consumes the Public GA operator identity/session revocation logout final index and source-probes the existing revocation/logout replay/reinstatement denial gate.

Status: ready-but-blocked.

The attachment does not invoke the revocation/logout replay/reinstatement gate, the revocation/logout gate, replay/cross-binding gates, identity/session binding gates, operator intent/consent gates, long soak gates, Public GA readiness gates, or terminal live gates.

It keeps identity reinstatement, session reinstatement, revocation/logout replay acceptance, reinstatement tokens, reinstatement nonces, device-session reinstatement, session lifecycle promotion, approval, release authority, activation authority, install authority, and Public GA claims blocked.

The attachment carries the canonical terminal closure backfeed from the source
revocation/logout final index: 17 release/live blockers across 4 ready
categories, with runner selector blockers=2 and dirty worktree owner-freeze
blockers=2. This is read-model context only; the local reinstatement blocker
count remains 30.

The only next step is a static readback/final-index closure for the same non-reinstatement facts.
