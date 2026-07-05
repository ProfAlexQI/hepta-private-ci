# Public GA Operator Identity/Session Revocation Logout Attachment

This attachment consumes the Public GA operator identity/session replay denial
final index and source-probes the operator identity/session revocation logout
denial gate and note. It does not invoke revocation/logout gates,
replay/cross-binding gates, identity/session binding gates, intent/consent
gates, Public GA operator packet, live endpoints, or long soak.

The attachment is ready-but-blocked. It keeps identity revocation, session
logout, session revocation, lifecycle status promotion, revocation token,
logout nonce, revocation/logout authority, public release, and Public GA
promotion false.

The attachment carries the canonical terminal closure backfeed from the source
replay denial final index: 17 release/live blockers across 4 ready categories,
with runner selector blockers=2 and dirty worktree owner-freeze blockers=2.
This is read-model context only; the local revocation/logout blocker count
remains 28.
