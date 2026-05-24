# Hepta Upstream Codex Activation Evidence Receipt No-Write Sink Adapter Contract

## Scope

This contract sits after the receipt persistence invocation dry-run. It defines
a no-write sink adapter that can accept redacted invocation shapes as validation
input while refusing filesystem persistence by default. It does not execute a
receipt persistence command, write the workspace, persist evidence, activate
runtime wiring, or permit public release claims.

- No-write sink adapter id:
  `upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract`
- Source invocation dry-run gate:
  `scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run.sh`
- No-write sink adapter contract gate:
  `scripts/hepta-upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract.sh`
- Active dependency gate: `scripts/hepta-active-service-dependency-isolation.sh`
- Candidate diff range:
  `108234b5ebe6941764a6b8edbb37b2aa04369f07..7d47056ea42636271ac020b86347fbbef49490aa`

## No-Write Sink Status

- Source invocation dry-run ready: `true`
- Required sink surface count: `6`
- Ready sink surface count: `6`
- Side-effect-free surface count: `6`
- Accepted invocation fixture count: `3`
- Rejected write fixture count: `3`
- Rejected public claim fixture count: `1`
- Persisted receipt count: `0`
- Workspace write performed count: `0`
- Sink write path enabled by default: `false`
- Sink accepts redacted payload hash: `true`
- Sink accepts redacted output path: `true`
- Sink requires operator approval: `true`
- Sink requires fresh trusted records: `true`
- Sink rejects public claim artifact write: `true`
- No-write sink adapter ready: `true`
- Activation blocked by no-write sink adapter: `true`
- Activation allowed by no-write sink adapter: `false`
- Active wiring allowed: `false`

## Required Sink Surfaces

All surfaces are required, ready, and side-effect-free:

- `redacted_invocation_acceptance`
- `payload_hash_binding`
- `redacted_output_path_binding`
- `operator_approval_requirement`
- `fresh_trusted_record_requirement`
- `public_claim_artifact_rejection`

## Denied Active Decisions

- Active runtime code wiring allowed: `false`
- Active runtime dependency allowed: `false`
- Active runtime auto-rebase allowed: `false`
- Active Codex engine dependency allowed: `false`
- Public release claim allowed: `false`
- Public GA claim allowed: `false`
- Release artifact write allowed: `false`

## No-Write Sink Adapter Invariants

- No-write sink adapter accepts redacted invocation shapes without persisting
  them.
- Filesystem persistence remains disabled by default.
- Public-claim and release-artifact requests are rejected by the no-write sink.
- Sink readiness does not permit active runtime wiring or public claims.

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

- Run the operator-approved write-enable fixture before any filesystem
  persistence.
- Bind sink acceptance to fresh live gate evidence and active binary SHA.
- Require release-governance approval before any public artifact path is opened.
