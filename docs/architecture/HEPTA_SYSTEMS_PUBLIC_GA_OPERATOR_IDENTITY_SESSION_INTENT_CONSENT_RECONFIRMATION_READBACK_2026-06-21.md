# Public GA Operator Identity/Session Operator Intent/Consent Reconfirmation Readback

This readback records a static, local-only snapshot for the Public GA operator
identity/session operator intent/consent reconfirmation attachment.

The readback is ready-but-blocked. It keeps operator intent, operator consent,
consent reconfirmation, identity signatures, session consent tokens, nonces,
consent refresh, approval derivation, acceptance, terminal decision/status
promotion, release authority, activation authority, install actions, service
restarts, active-binary mutation, Public GA, public release, and rollback
execution blocked.

It also carries the canonical terminal closure release/live backfeed as
read-model context: 17 release/live blockers across 4 categories with 17
category blockers, including runner_selector=2 and
dirty_worktree_owner_freeze=2. That backfeed is operator-visible only and does
not change this readback's local blocker semantics.

It does not invoke the intent/consent gate, terminal decision/status gate,
final acknowledgement gate, any Public GA approval packet, terminal live gate,
long soak, live URL, or external send. It is only a static readback for the
local intent/consent denial attachment.
