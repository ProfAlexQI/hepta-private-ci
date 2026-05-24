# Hepta Upstream Codex Activation Denied Sample

## Scope

This gate defines a full-shaped sample activation packet and proves it remains
blocked by default. The sample records all required schema fields as present,
but it does not record real operator approval and has no fresh activation
evidence. Packet shape alone is not an activation grant.

- Sample id: `upstream-codex-activation-denied-sample-packet`
- Source readiness closure gate: `scripts/hepta-upstream-codex-activation-readiness-closure.sh`
- Denied sample gate: `scripts/hepta-upstream-codex-activation-denied-sample.sh`
- Active dependency gate: `scripts/hepta-active-service-dependency-isolation.sh`
- Candidate diff range:
  `108234b5ebe6941764a6b8edbb37b2aa04369f07..7d47056ea42636271ac020b86347fbbef49490aa`

## Sample Status

- Activation readiness closure ready: `true`
- Sample packet shape complete: `true`
- Sample required schema field count: `14`
- Sample recorded schema field count: `14`
- Sample required evidence count: `8`
- Sample fresh evidence count: `0`
- Sample operator approval field present: `true`
- Sample operator approval recorded: `false`
- Sample public release claim requested: `true`
- Sample release artifact write requested: `true`
- Sample validation status: `blocked`
- Sample blocked reason: `operator approval is not recorded and activation
  evidence is not fresh`
- Active wiring allowed: `false`

## Sample Invariants

- Full-shaped activation samples are not approvals.
- Operator approval must be recorded separately from packet shape.
- All eight evidence slots must be fresh before activation can be reconsidered.
- Public release claims and artifact writes remain denied for the denied sample.

## Denied Active Decisions

- Active runtime code wiring allowed: `false`
- Active runtime dependency allowed: `false`
- Active runtime auto-rebase allowed: `false`
- Active Codex engine dependency allowed: `false`
- Public release claim allowed: `false`
- Public GA claim allowed: `false`
- Release artifact write allowed: `false`

## Side-Effect Boundary

- No upstream fetch
- No upstream merge
- No upstream checkout
- No workspace mutation by default
- No active service restart
- No credential value read
- No secret file read
- No provider invocation
- No channel delivery
- No gateway RPC
- No public release publication

## Required Next Gates

- Replace the denied sample with a concrete operator-approved activation packet.
- Bind every evidence slot to fresh live dependency, watchdog, browser, soak,
  and rollback evidence.
- Rerun activation readiness closure after concrete approval and evidence.
- Run clean preflight and live gates before any active wiring decision.
