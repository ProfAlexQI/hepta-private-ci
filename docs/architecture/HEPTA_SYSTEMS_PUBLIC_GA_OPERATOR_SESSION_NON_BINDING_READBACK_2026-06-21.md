# Public GA Operator Session Non-Binding Readback

This static readback consumes the Public GA Operator Session Non-Binding
Attachment. It does not invoke session gates, does not bind a session, does not
replay a session token, and does not accept cross-session binding.

The readback is ready-but-blocked. It records that the replay/cross-binding
denial gate and doc are present while every operator approval, identity,
session, replay, live, release, and Public GA side effect remains false.

It also carries the canonical terminal closure backfeed unchanged: 17
release/live blockers, 4 ready categories, 17 category blockers, runner
selector blockers=2, and dirty worktree owner-freeze blockers=2. The readback
check count is 30, while the local session non-binding blocker count remains
24.
