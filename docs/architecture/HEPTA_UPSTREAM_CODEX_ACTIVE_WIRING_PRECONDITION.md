# Hepta Upstream Codex Active Wiring Precondition

## Scope

This packet defines the preconditions for any future upstream Codex active
wiring path. It is not an activation packet and does not allow active runtime
code wiring by itself.

- Precondition id: `upstream-codex-active-wiring-precondition`
- Source closure gate: `scripts/hepta-upstream-codex-promotion-closure.sh`
- Precondition gate: `scripts/hepta-upstream-codex-active-wiring-precondition.sh`
- Active dependency gate: `scripts/hepta-active-service-dependency-isolation.sh`
- Candidate diff range:
  `108234b5ebe6941764a6b8edbb37b2aa04369f07..7d47056ea42636271ac020b86347fbbef49490aa`

## Source Closure

- Promotion closure ready: `true`
- All surface promotion packets complete: `true`
- Active promotion denial ready: `true`

## Required Preconditions

- Explicit operator approval required: `true`
- Operator approval recorded: `false`
- Activation request id required: `true`
- Activation request id present: `false`
- Live dependency isolation required: `true`
- Watchdog required: `true`
- Browser smoke required: `true`
- Long soak required: `true`
- Active wiring precondition ready: `true`
- Active wiring allowed: `false`

## Denied Active Decisions

- Active runtime code wiring allowed: `false`
- Active runtime dependency allowed: `false`
- Active runtime auto-rebase allowed: `false`
- Active Codex engine dependency allowed: `false`
- Public release claim allowed: `false`
- Public GA claim allowed: `false`
- Release artifact write allowed: `false`

## Preconditions

- Promotion closure gate is ready.
- All four required surface promotion packets are complete.
- Active promotion denial remains ready.
- Explicit operator approval record is required and not yet recorded.
- Activation request id is required and not yet present.
- Live dependency isolation, watchdog, browser smoke, and long soak must be
  fresh.

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

- Record an operator approval packet before any active wiring.
- Bind any activation request to a concrete `activation_request_id`.
- Rerun live active-service dependency isolation at activation time.
- Rerun watchdog, browser smoke, and long soak at activation time.
- Keep public release and public GA claims false until a separate release gate.
