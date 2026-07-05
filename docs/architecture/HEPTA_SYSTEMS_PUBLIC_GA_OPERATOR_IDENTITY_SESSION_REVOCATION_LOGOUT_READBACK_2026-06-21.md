# Public GA Operator Identity/Session Revocation Logout Readback

This static readback consumes the Public GA Operator Identity/Session
Revocation Logout Attachment. It does not invoke revocation/logout gates,
revoke identity, log out a session, record revocation tokens, or derive
authority.

The readback is ready-but-blocked. It records that the revocation/logout denial
gate and doc are present while every operator approval, identity, session,
revocation, logout, live, release, and Public GA side effect remains false.

It also carries the canonical terminal closure backfeed unchanged: 17
release/live blockers, 4 ready categories, 17 category blockers, runner
selector blockers=2, and dirty worktree owner-freeze blockers=2. The readback
check count is 34, while the local revocation/logout blocker count remains 28.
