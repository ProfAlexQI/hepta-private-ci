# Hepta Upstream Codex Activation Evidence Receipt Materialization Dry Run

This gate sits after the write-enable fixture gate. It models a future receipt
materialization plan that can bind a payload hash and redacted output path while
still refusing actual filesystem persistence by default.

The point is narrow: prove Hepta can describe what would be materialized before
any workspace write path exists. This is still a report-only contract.

## Gate

- Gate id:
  `upstream-codex-activation-evidence-receipt-materialization-dry-run`
- Source write-enable fixture gate:
  `scripts/hepta-upstream-codex-activation-evidence-receipt-write-enable-fixture.sh`
- Materialization dry-run gate:
  `scripts/hepta-upstream-codex-activation-evidence-receipt-materialization-dry-run.sh`
- Active dependency isolation gate:
  `scripts/hepta-active-service-dependency-isolation.sh`

## Current Truth

- Source write-enable fixture ready: `true`
- Required materialization fixture count: `3`
- Materialization fixture count: `3`
- Blocked materialization fixture count: `3`
- Allowed materialization fixture count: `0`
- Explicit write-enable requested fixture count: `3`
- Operator-approved fixture count: `2`
- Activation request bound fixture count: `3`
- Fresh trusted record fixture count: `2`
- Active binary SHA bound fixture count: `3`
- Payload hash planned fixture count: `3`
- Redacted output path planned fixture count: `3`
- Deterministic materialization plan count: `3`
- Public claim attempt fixture count: `1`
- Release artifact write attempt fixture count: `1`
- Public artifact policy satisfied fixture count: `2`
- Filesystem persistence allowed count: `0`
- Materialization executed count: `0`
- Workspace write performed count: `0`
- Evidence receipt persisted count: `0`
- Materialization dry-run ready: `true`
- Activation blocked by materialization dry run: `true`
- Activation allowed by materialization dry run: `false`
- Active wiring allowed: `false`

## Fixtures

All fixtures explicitly request write enablement and include deterministic dry-run
materialization fields. They still remain blocked:

- `materialization-without-operator-approval`
  - Blocks because operator approval is absent.
  - Payload hash and redacted output path are planned.
  - No filesystem persistence is allowed.
- `operator-approved-stale-materialization`
  - Blocks because trusted records are not fresh.
  - Payload hash and redacted output path are planned.
  - No filesystem persistence is allowed.
- `public-artifact-materialization-attempt`
  - Blocks because public claim and release artifact write requests require
    separate release-governance approval.
  - Payload hash and redacted output path are planned.
  - No filesystem persistence is allowed.

## Side-Effect Boundary

This gate performs no live materialization.

- No upstream fetch/merge/checkout
- No command invocation performed
- No receipt persistence execution
- No filesystem persistence
- No materialization execution
- No workspace write
- No evidence receipt persistence
- No active service restart
- No credential or secret read
- No provider/model invocation
- No channel delivery
- No gateway RPC
- No public release publication
- No release artifact write

## Required Next Gates

- Bind materialization dry runs to fresh live evidence records.
- Add a filesystem persistence approval packet before any workspace write.
- Require release-governance approval before public artifact persistence.
