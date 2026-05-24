# Hepta Upstream Codex Activation Evidence Recording Denial Matrix

## Scope

This matrix gate sits after the evidence recording dry-run receipt. It proves
that partially populated, stale, or public-claim-shaped receipt attempts are
routed into a no-write sink by default. It does not record evidence, persist
receipts, enable active wiring, or allow release artifact writes.

- Matrix id:
  `upstream-codex-activation-evidence-recording-denial-matrix`
- Source evidence recording dry-run receipt gate:
  `scripts/hepta-upstream-codex-activation-evidence-recording-dry-run-receipt.sh`
- Evidence recording denial matrix gate:
  `scripts/hepta-upstream-codex-activation-evidence-recording-denial-matrix.sh`
- Active dependency gate: `scripts/hepta-active-service-dependency-isolation.sh`
- Candidate diff range:
  `108234b5ebe6941764a6b8edbb37b2aa04369f07..7d47056ea42636271ac020b86347fbbef49490aa`

## Matrix Status

- Source receipt gate ready: `true`
- Required denied attempt count: `3`
- Denied receipt attempt count: `3`
- Allowed receipt attempt count: `0`
- Max recorded receipt field count: `12`
- Max accepted trusted record count: `8`
- Max fresh trusted record count: `8`
- Public claim attempt count: `1`
- Release artifact write attempt count: `1`
- Receipt sink write performed: `false`
- Evidence receipt persisted: `false`
- Trusted record materialized: `false`
- No-write sink ready: `true`
- Activation blocked by no-write sink: `true`
- Activation allowed by no-write sink: `false`
- Active wiring allowed: `false`

## Denied Receipt Attempts

All attempts are blocked and non-persistent:

- `partial-receipt-fields`
  - Kind: `partial_receipt_fields`
  - Recorded receipt field count: `5`
  - Accepted trusted record count: `3`
  - Fresh trusted record count: `0`
  - Denial reason:
    `partial receipt fields and stale trusted records cannot be persisted`
- `operator-approved-but-stale-evidence`
  - Kind: `operator_approved_stale_evidence`
  - Recorded receipt field count: `12`
  - Accepted trusted record count: `8`
  - Fresh trusted record count: `0`
  - Denial reason:
    `operator approval alone cannot bypass stale trusted evidence`
- `public-claim-release-artifact-attempt`
  - Kind: `public_claim_release_artifact_attempt`
  - Recorded receipt field count: `12`
  - Accepted trusted record count: `8`
  - Fresh trusted record count: `8`
  - Public claim requested: `true`
  - Release artifact write requested: `true`
  - Denial reason:
    `public release claim and artifact writes require a separate explicit release path`

## Denied Active Decisions

- Active runtime code wiring allowed: `false`
- Active runtime dependency allowed: `false`
- Active runtime auto-rebase allowed: `false`
- Active Codex engine dependency allowed: `false`
- Public release claim allowed: `false`
- Public GA claim allowed: `false`
- Release artifact write allowed: `false`

## No-Write Sink Invariants

- Denied receipt attempts can be fully shaped without being persisted.
- Receipt sink writes remain false until an explicit operator-approved recording
  path is opened.
- Public-claim-shaped receipt attempts stay blocked by default.
- No denied fixture can enable active runtime wiring or release artifact writes.

## Side-Effect Boundary

- No upstream fetch
- No upstream merge
- No upstream checkout
- No workspace mutation by default
- No receipt persistence
- No active service restart
- No credential value read
- No secret file read
- No provider invocation
- No channel delivery
- No gateway RPC
- No public release publication

## Required Next Gates

- Define an operator-approved receipt persistence command before any workspace
  write.
- Bind persisted receipts to fresh trusted record ids and live SHA evidence.
- Rerun denial matrix before accepting any public-claim-shaped receipt.
