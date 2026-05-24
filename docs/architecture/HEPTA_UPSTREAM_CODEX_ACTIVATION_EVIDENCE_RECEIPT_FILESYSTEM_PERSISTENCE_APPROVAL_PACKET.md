# Hepta Upstream Codex Activation Evidence Receipt Filesystem Persistence Approval Packet

Gate id: `upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet`

Source materialization dry-run gate:
`scripts/hepta-upstream-codex-activation-evidence-receipt-materialization-dry-run.sh`

Filesystem persistence approval packet gate:
`scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet.sh`

This gate defines the approval packet required before any future upstream Codex
activation evidence receipt can be persisted to the workspace. It is still a
report-only contract: the packet schema is present, but no approval fields are
recorded by default and no filesystem persistence is allowed.

Current contract:

- Source materialization dry-run ready: `true`
- Required approval field count: `12`
- Approval field count: `12`
- Recorded approval field count: `0`
- Redacted or hashed field count: `10`
- Required for filesystem persistence field count: `12`
- Operator approval required: `true`
- Operator approval recorded: `false`
- Activation request required: `true`
- Activation request recorded: `false`
- Materialization plan required: `true`
- Materialization plan recorded: `false`
- Fresh trusted records required: `true`
- Fresh trusted records recorded: `false`
- Active binary SHA required: `true`
- Active binary SHA recorded: `false`
- Public artifact policy required: `true`
- Public artifact policy recorded: `false`
- Filesystem persistence approval packet ready: `true`
- Filesystem persistence allowed: `false`
- Filesystem persistence execution performed: `false`
- Workspace write performed: `false`
- Evidence receipt persisted: `false`
- Activation blocked by filesystem persistence approval: `true`
- Activation allowed by filesystem persistence approval: `false`
- Active wiring allowed: `false`

Required approval fields:

- `filesystem_persistence_approval_id`
- `activation_request_id`
- `operator_approval_id`
- `operator_identity_hash`
- `materialization_plan_id`
- `receipt_payload_hash`
- `redacted_output_path`
- `accepted_trusted_record_ids`
- `fresh_trusted_record_ids`
- `active_binary_sha256`
- `rollback_plan_id`
- `public_claim_and_artifact_decision`

Side-effect boundary:

- No upstream fetch/merge/checkout
- No command invocation performed
- No receipt persistence execution
- No materialization execution
- No filesystem persistence execution
- No workspace write
- No evidence receipt persistence
- No active service restart
- No credential or secret read
- No provider or model invocation
- No channel delivery
- No gateway RPC
- No public release publication
- No public GA claim
- No release artifact write

This packet makes the write boundary explicit: deterministic materialization
plans are not execution authority, and filesystem persistence still requires a
complete approval packet, fresh trusted evidence, active binary SHA binding,
and separate public artifact policy approval.

Next gate:
`scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist.sh`

The next gate adds a redacted output-path allowlist before any future receipt
write can target a filesystem sink. It keeps source-tree, home-directory,
release-artifact, and public-artifact paths blocked from receipt persistence.
