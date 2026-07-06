# Public GA Operator Identity/Session Replay Denial Readback

This static readback consumes the Public GA Operator Identity/Session Replay
Denial Attachment. It does not invoke replay gates, replay session tokens,
replay identity hashes, or accept cross-session binding.

The readback is ready-but-blocked. It records that the replay/cross-binding
denial gate and doc are present while every operator approval, identity,
session, replay, cross-binding, live, release, and Public GA side effect remains
false.

It also carries the canonical terminal closure backfeed unchanged: 17
release/live blockers, 4 ready categories, 17 category blockers, runner
selector blockers=2, and dirty worktree owner-freeze blockers=2. The readback
check count is 32, while the local replay denial blocker count remains 26.
