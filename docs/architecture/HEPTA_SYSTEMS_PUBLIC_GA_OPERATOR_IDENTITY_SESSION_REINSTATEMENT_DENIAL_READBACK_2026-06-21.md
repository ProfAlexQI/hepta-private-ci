# Public GA Operator Identity/Session Reinstatement Denial Readback

This readback consumes the Public GA operator identity/session reinstatement denial attachment as a static snapshot.

Status: ready-but-blocked.

The readback does not invoke the revocation/logout replay/reinstatement gate, the revocation/logout gate, replay/cross-binding gates, identity/session binding gates, operator intent/consent gates, long soak gates, Public GA readiness gates, or terminal live gates.

It confirms identity reinstatement, session reinstatement, revocation/logout replay acceptance, reinstatement token recording, reinstatement nonce recording, device-session reinstatement, session lifecycle promotion, reinstatement authority derivation, release publication, rollback execution, and Public GA claims remain false.

It also carries the canonical terminal closure backfeed unchanged: 17
release/live blockers, 4 ready categories, 17 category blockers, runner
selector blockers=2, and dirty worktree owner-freeze blockers=2. The readback
check count is 36, while the local reinstatement blocker count remains 30.

The only next step is a final index that preserves the same non-reinstatement boundary.
