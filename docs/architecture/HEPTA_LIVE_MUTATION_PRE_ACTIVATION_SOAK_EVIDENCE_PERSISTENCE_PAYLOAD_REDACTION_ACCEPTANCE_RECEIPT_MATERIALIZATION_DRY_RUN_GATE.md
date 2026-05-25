# Hepta Live Mutation Pre-Activation Soak Evidence Persistence Payload Redaction Acceptance Receipt Materialization Dry-Run Gate

Date: 2026-05-25

This gate sits after the payload redaction acceptance receipt write-enable
fixture gate. It models deterministic redacted receipt materialization plans for
future persistence while keeping command execution, materialization execution,
receipt persistence, filesystem writes, and live mutation disabled.

The gate does not invoke or execute a command, materialize a receipt, persist a
receipt, write a file, inspect payload plaintext, run a live secret scan, send a
channel message, invoke a provider or model, mutate Gateway/runtime stores, or
enable live mutation.

## Contract

The gate consumes:

- `scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-write-enable-fixture-gate.sh`
- the source payload redaction acceptance receipt write-enable fixture report hash
- the source payload redaction acceptance receipt no-write sink contract report hash
- the source payload redaction acceptance receipt invocation dry-run report hash
- the source payload redaction acceptance receipt command contract report hash
- the source payload redaction acceptance matrix report hash
- the source payload redaction proof report hash
- the source no-secret payload review report hash
- the source operator-scope report hash
- the source approval-packet report hash
- the source approval receipt payload hash
- the source pre-activation soak report hash
- the source persistence-denial report hash
- the minimum 24-sample pre-activation soak requirement

It requires the source write-enable fixture to be `ready`, but keeps these
values false or zero:

- `payload_redaction_acceptance_receipt_command_recorded`
- `payload_redaction_acceptance_receipt_command_enabled_by_default`
- `payload_redaction_acceptance_receipt_command_invocation_performed_count`
- `payload_redaction_acceptance_receipt_command_execution_performed_count`
- `payload_redaction_acceptance_receipt_recorded`
- `payload_redaction_acceptance_receipt_materialized`
- `payload_redaction_acceptance_receipt_persisted`
- `payload_redaction_acceptance_matrix_recorded`
- `payload_redaction_acceptance_matrix_persisted`
- `payload_redaction_proof_recorded`
- `payload_redaction_proof_accepted`
- `command_invocation_performed_count`
- `command_execution_performed_count`
- `receipt_persistence_execution_performed_count`
- `materialization_execution_performed_count`
- `materialization_executed_count`
- `filesystem_write_performed_count`
- `workspace_write_performed_count`
- `receipt_materialized_count`
- `receipt_persisted_count`
- `raw_payload_plaintext_recorded`
- `raw_payload_plaintext_persisted`
- `live_secret_scan_performed`
- `receipt_persistence_enabled`
- `receipt_persisted`
- `activation_allowed`
- `live_mutation_execution_ready`

## Materialization Dry-Run Fixture Status

- Required materialization fixture count: `5`
- Materialization fixture count: `5`
- Blocked materialization fixture count: `5`
- Allowed materialization fixture count: `0`
- Explicit write-enable requested fixture count: `5`
- Materialization denied without operator scope count: `1`
- Materialization denied command disabled count: `1`
- Materialization denied persistence disabled count: `1`
- Materialization denied plaintext payload count: `1`
- Materialization denied public artifact count: `1`
- Deterministic materialization plan count: `3`
- Deterministic materialization plan persisted count: `0`
- Payload hash planned fixture count: `5`
- Redacted payload summary hash bound fixture count: `5`
- Redacted output path planned fixture count: `4`
- Redacted receipt shape fixture count: `3`
- Accepted redaction proof bound fixture count: `4`
- Operator scope bound fixture count: `4`
- Plaintext payload attempt count: `1`
- Public claim attempt count: `1`
- Release artifact write attempt count: `1`
- Filesystem persistence allowed count: `0`
- Command invocation attempt count: `5`
- Command invocation performed count: `0`
- Command execution performed count: `0`
- Receipt persistence execution performed count: `0`
- Materialization execution performed count: `0`
- Materialization executed count: `0`
- Filesystem write performed count: `0`
- Workspace write performed count: `0`
- Receipt materialized count: `0`
- Receipt persisted count: `0`
- Materialization dry-run ready: `true`
- Activation allowed: `false`
- Live mutation execution ready: `false`

## Fixtures

All fixtures explicitly request write enablement and materialization, then stay
blocked as dry-run plans:

- `materialization-without-operator-scope`
- `operator-scoped-but-command-disabled`
- `accepted-proof-but-persistence-disabled`
- `plaintext-materialization-attempt`
- `public-artifact-materialization-attempt`

Every fixture keeps:

- `write_enable_requested = true`
- `materialization_requested = true`
- `materialization_status = blocked_dry_run`
- `command_invocation_requested = true`
- `command_invocation_performed = false`
- `command_execution_performed = false`
- `receipt_persistence_execution_performed = false`
- `materialization_execution_performed = false`
- `filesystem_write_requested = true`
- `filesystem_write_performed = false`
- `receipt_materialized = false`
- `receipt_persisted = false`
- `activation_allowed = false`

Only the redacted non-public fixture shapes can carry a deterministic
materialization plan. Plaintext and public-artifact attempts are represented but
remain denied.

## Required Before Execution

Any future execution path must provide all of the following before persistence
can be considered:

- `operator_approval_id`
- `operator_identity_hash`
- `single_surface_activation_scope`
- `accepted_redaction_proof_ids`
- `redacted_payload_summary_sha256`
- `receipt_output_path_redacted`
- `receipt_materialization_plan_id`
- `receipt_persistence_approval`
- `fresh_pre_activation_soak_evidence`
- `rollback_plan_id`
- `public_claim_and_artifact_decision`

## Safety Boundary

The gate must not:

- write memory, skills, plugin registries, or capability registries
- invoke providers or models
- send channel messages
- mutate runtime stores or Gateway queues
- invoke or execute a receipt command
- execute materialization
- write approval packets, operator-scope records, payload reviews, redaction
  proofs, acceptance matrices, no-write sink records, write-enable fixture
  records, materialization plans, receipt files, evidence files, or release
  artifacts
- persist plaintext payloads
- inspect raw payload plaintext
- run live secret scans
- persist pre-activation soak evidence
- restart launchd
- execute rollback
- read credentials or secret files

The next safe step is implemented by
`docs/architecture/HEPTA_LIVE_MUTATION_PRE_ACTIVATION_SOAK_EVIDENCE_PERSISTENCE_PAYLOAD_REDACTION_ACCEPTANCE_RECEIPT_FILESYSTEM_PERSISTENCE_APPROVAL_PACKET_GATE.md`
and
`scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-approval-packet-gate.sh`.
It adds a schema-only redaction acceptance receipt filesystem persistence
approval-packet gate, still without command execution, materialization
execution, receipt persistence, filesystem writes, or live mutation.
