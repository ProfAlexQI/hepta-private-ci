# Hepta Upstream Codex Activation Evidence Receipt Persistence Command Contract

## Scope

This command contract sits after the evidence recording denial matrix. It
defines the operator-approved command shape required before any future receipt
persistence path may write the workspace. The command is disabled and no-op by
default: it records no fields, invokes no command, persists no receipt, and does
not allow active wiring or public claims.

- Command contract id:
  `upstream-codex-activation-evidence-receipt-persistence-command-contract`
- Source evidence recording denial matrix gate:
  `scripts/hepta-upstream-codex-activation-evidence-recording-denial-matrix.sh`
- Receipt persistence command contract gate:
  `scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-command-contract.sh`
- Active dependency gate: `scripts/hepta-active-service-dependency-isolation.sh`
- Candidate diff range:
  `108234b5ebe6941764a6b8edbb37b2aa04369f07..7d47056ea42636271ac020b86347fbbef49490aa`

## Command Status

- Source denial matrix ready: `true`
- Required command field count: `10`
- Recorded command field count: `0`
- Redacted or hashed field count: `9`
- Operator approval required: `true`
- Operator approval recorded: `false`
- Activation request required: `true`
- Activation request recorded: `false`
- Trusted record materialized: `false`
- Receipt persistence command enabled by default: `false`
- Receipt persistence command invoked: `false`
- Receipt persistence execution performed: `false`
- Receipt persistence no-op ready: `true`
- Workspace write performed: `false`
- Evidence receipt persisted: `false`
- Activation blocked by persistence contract: `true`
- Activation allowed by persistence contract: `false`
- Active wiring allowed: `false`

## Required Command Fields

All fields are required and absent by default:

- `receipt_persistence_command_id`
- `activation_request_id`
- `operator_approval_id`
- `operator_identity_hash`
- `accepted_trusted_record_ids`
- `fresh_trusted_record_ids`
- `receipt_payload_hash`
- `receipt_output_path_redacted`
- `rollback_plan_id`
- `public_claim_and_artifact_decision`

The command contract requires redacted or hashed values for command id,
activation request id, operator approval id, operator identity, trusted record
ids, receipt payload hash, output path, and rollback plan id.

## Denied Active Decisions

- Active runtime code wiring allowed: `false`
- Active runtime dependency allowed: `false`
- Active runtime auto-rebase allowed: `false`
- Active Codex engine dependency allowed: `false`
- Public release claim allowed: `false`
- Public GA claim allowed: `false`
- Release artifact write allowed: `false`

## Command Contract Invariants

- Receipt persistence command contract is present but disabled by default.
- No command invocation can write the workspace without operator approval.
- Activation request and trusted evidence ids are required before persistence.
- Persistence command readiness does not permit active wiring or release claims.

## Side-Effect Boundary

- No upstream fetch
- No upstream merge
- No upstream checkout
- No command invocation
- No receipt persistence execution
- No workspace mutation by default
- No active service restart
- No credential value read
- No secret file read
- No provider invocation
- No channel delivery
- No gateway RPC
- No public release publication

## Required Next Gates

- Add a redacted persistence dry-run fixture before any real write path.
- Bind command invocation to a fresh activation request and trusted evidence
  ids.
- Require live SHA, watchdog, browser smoke, and soak evidence before
  persistence.
