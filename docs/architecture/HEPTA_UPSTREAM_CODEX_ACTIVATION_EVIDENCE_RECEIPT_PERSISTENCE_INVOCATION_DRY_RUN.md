# Hepta Upstream Codex Activation Evidence Receipt Persistence Invocation Dry Run

## Scope

This dry-run fixture sits after the receipt persistence command contract. It
models redacted command invocation attempts while keeping persistence disabled by
default. The fixtures may request invocation, but no command is invoked, no
receipt persistence execution runs, no workspace write occurs, and no activation
or public claim is allowed.

- Invocation dry-run id:
  `upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run`
- Source command contract gate:
  `scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-command-contract.sh`
- Receipt persistence invocation dry-run gate:
  `scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run.sh`
- Active dependency gate: `scripts/hepta-active-service-dependency-isolation.sh`
- Candidate diff range:
  `108234b5ebe6941764a6b8edbb37b2aa04369f07..7d47056ea42636271ac020b86347fbbef49490aa`

## Invocation Dry-Run Status

- Source command contract ready: `true`
- Required invocation fixture count: `3`
- Command invocation attempt count: `3`
- Command invocation performed count: `0`
- Receipt persistence execution performed count: `0`
- Workspace write performed count: `0`
- Evidence receipt persisted count: `0`
- Redacted output path fixture count: `3`
- Payload hash bound fixture count: `3`
- Operator approved fixture count: `3`
- Activation request bound fixture count: `3`
- Max recorded command field count: `10`
- Max accepted trusted record count: `8`
- Max fresh trusted record count: `8`
- Public claim attempt count: `1`
- Release artifact write attempt count: `1`
- Receipt persistence command enabled by default: `false`
- Invocation dry-run no-op ready: `true`
- Activation blocked by invocation dry-run: `true`
- Activation allowed by invocation dry-run: `false`
- Active wiring allowed: `false`

## Dry-Run Fixtures

All fixtures are redacted, command-shaped, and blocked as `blocked_noop`:

- `redacted-command-shape`: all `10` command fields present, `8` accepted
  trusted records, `8` fresh trusted records, no public claim request.
- `stale-evidence-invocation-attempt`: all `10` command fields present, `8`
  accepted trusted records, `0` fresh trusted records.
- `public-claim-artifact-invocation-attempt`: all `10` command fields present,
  `8` accepted trusted records, `8` fresh trusted records, public claim and
  release artifact write requested.

## Denied Active Decisions

- Active runtime code wiring allowed: `false`
- Active runtime dependency allowed: `false`
- Active runtime auto-rebase allowed: `false`
- Active Codex engine dependency allowed: `false`
- Public release claim allowed: `false`
- Public GA claim allowed: `false`
- Release artifact write allowed: `false`

## Invocation Dry-Run Invariants

- Redacted invocation fixtures can request persistence without executing it.
- Command invocation remains unperformed while the command is disabled by
  default.
- Receipt persistence execution and workspace writes stay false for every
  fixture.
- Public-claim-shaped invocation fixtures stay blocked by default.

## Side-Effect Boundary

- No upstream fetch
- No upstream merge
- No upstream checkout
- No command invocation performed
- No receipt persistence execution
- No workspace mutation by default
- No evidence receipt persistence
- No active service restart
- No credential value read
- No secret file read
- No provider invocation
- No channel delivery
- No gateway RPC
- No public release publication

## Required Next Gates

- Bind a no-write receipt sink adapter before any persisted receipt path.
- Require fresh live gate evidence for every invocation fixture.
- Require operator approval before enabling any receipt persistence command.
