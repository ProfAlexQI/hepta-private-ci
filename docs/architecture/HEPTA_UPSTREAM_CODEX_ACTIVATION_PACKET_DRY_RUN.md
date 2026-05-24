# Hepta Upstream Codex Activation Packet Dry-Run

## Scope

This validator exercises representative activation packet fixtures without
recording a real activation packet. It proves that placeholder or incomplete
packets remain blocked and cannot open active runtime wiring, public release
claims, or release artifact writes.

- Validator id: `upstream-codex-activation-packet-dry-run-validator`
- Source packet gate: `scripts/hepta-upstream-codex-activation-request-packet.sh`
- Dry-run validator gate: `scripts/hepta-upstream-codex-activation-packet-dry-run.sh`
- Active dependency gate: `scripts/hepta-active-service-dependency-isolation.sh`
- Candidate diff range:
  `108234b5ebe6941764a6b8edbb37b2aa04369f07..7d47056ea42636271ac020b86347fbbef49490aa`

## Schema Status

- Activation packet schema ready: `true`
- Activation packet recorded: `false`
- Required schema field count: `14`
- Fixture count: `3`
- Blocked fixture count: `3`
- Allowed fixture count: `0`
- Dry-run validator ready: `true`
- Active wiring allowed: `false`

## Dry-Run Fixtures

- `empty-placeholder`: records `0 / 14` required fields and is blocked because
  all required activation fields are missing.
- `operator-only-placeholder`: records `2 / 14` required fields and is blocked
  because activation request id, live evidence, and rollback plan are missing.
- `public-claim-attempt-without-evidence`: records `6 / 14` required fields and
  requests public-release/artifact decisions, but remains blocked because full
  evidence and separate release governance approval are absent.

## Denied Active Decisions

- Active runtime code wiring allowed: `false`
- Active runtime dependency allowed: `false`
- Active runtime auto-rebase allowed: `false`
- Active Codex engine dependency allowed: `false`
- Public release claim allowed: `false`
- Public GA claim allowed: `false`
- Release artifact write allowed: `false`

## Validation Invariants

- Dry-run fixtures cannot activate wiring without all required fields.
- Operator approval and activation request id must both be recorded.
- Live evidence and rollback fields must be present before activation.
- Public release and artifact-write requests are denied by default.
- The dry-run validator performs no upstream or runtime side effects.

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

- Replace placeholder fixtures with a concrete activation packet only after
  operator approval.
- Bind the activation packet to fresh dependency-isolation, watchdog,
  browser-smoke, long-soak, and rollback evidence ids.
- Keep active Codex engine dependency false unless a separate dependency-change
  review approves it.
- Keep public release and artifact-write decisions false until release
  governance approves them.
- Rerun clean preflight and live gates after any future concrete activation
  packet is recorded.
