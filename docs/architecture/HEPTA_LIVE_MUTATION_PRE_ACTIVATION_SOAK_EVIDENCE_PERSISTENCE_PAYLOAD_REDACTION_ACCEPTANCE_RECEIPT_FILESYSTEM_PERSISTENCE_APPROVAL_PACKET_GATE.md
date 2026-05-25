# Hepta Live Mutation Pre-Activation Soak Evidence Persistence Payload Redaction Acceptance Receipt Filesystem Persistence Approval Packet Gate

Date: 2026-05-25

This gate sits after the payload redaction acceptance receipt materialization
dry-run gate. It defines the schema-only approval packet required before any
future redacted receipt filesystem persistence can be considered.

The gate does not record an approval packet, execute a command, execute
materialization, persist a receipt, write a file, inspect payload plaintext, run
a live secret scan, send a channel message, invoke a provider or model, mutate
Gateway/runtime stores, or enable live mutation.

## Contract

The gate consumes:

- `scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-materialization-dry-run-gate.sh`
- the source payload redaction acceptance receipt materialization dry-run report
  hash
- the source payload redaction acceptance receipt write-enable fixture report
  hash
- the source payload redaction acceptance receipt no-write sink contract report
  hash
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

It requires the source materialization dry-run gate to be `ready`, but keeps
these values false or zero:

- `payload_redaction_acceptance_receipt_command_recorded`
- `payload_redaction_acceptance_receipt_command_enabled_by_default`
- `payload_redaction_acceptance_receipt_command_invocation_performed_count`
- `payload_redaction_acceptance_receipt_command_execution_performed_count`
- `payload_redaction_acceptance_receipt_recorded`
- `payload_redaction_acceptance_receipt_materialized`
- `payload_redaction_acceptance_receipt_persisted`
- `filesystem_persistence_approval_packet_recorded`
- `filesystem_persistence_approval_packet_persisted`
- `payload_redaction_acceptance_matrix_recorded`
- `payload_redaction_acceptance_matrix_persisted`
- `payload_redaction_proof_recorded`
- `payload_redaction_proof_accepted`
- `accepted_redaction_proof_count`
- `recorded_approval_field_count`
- `deterministic_materialization_plan_persisted_count`
- `materialization_plan_recorded`
- `materialization_plan_persisted`
- `operator_approval_recorded`
- `operator_identity_hash_recorded`
- `single_surface_activation_scope_recorded`
- `receipt_payload_hash_recorded`
- `redacted_payload_summary_hash_recorded`
- `receipt_output_path_redacted_recorded`
- `accepted_redaction_proof_ids_recorded`
- `fresh_pre_activation_soak_evidence_recorded`
- `active_binary_sha_recorded`
- `rollback_plan_recorded`
- `public_artifact_policy_recorded`
- `filesystem_persistence_allowed_count`
- `command_invocation_performed_count`
- `command_execution_performed_count`
- `receipt_persistence_execution_performed_count`
- `materialization_execution_performed_count`
- `filesystem_persistence_execution_performed_count`
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

## Approval Packet Status

- Required approval field count: `13`
- Approval field count: `13`
- Recorded approval field count: `0`
- Redacted or hashed field count: `11`
- Required for filesystem persistence field count: `13`
- Required filesystem persistence approval fixture count: `5`
- Filesystem persistence approval fixture count: `5`
- Blocked filesystem persistence approval fixture count: `5`
- Allowed filesystem persistence approval fixture count: `0`
- Explicit filesystem persistence approval requested fixture count: `5`
- Approval denied without operator scope count: `1`
- Approval denied command disabled count: `1`
- Approval denied persistence disabled count: `1`
- Approval denied plaintext payload count: `1`
- Approval denied public artifact count: `1`
- Deterministic materialization plan count: `3`
- Deterministic materialization plan persisted count: `0`
- Materialization plan required: `true`
- Materialization plan recorded: `false`
- Operator approval required: `true`
- Operator approval recorded: `false`
- Operator identity hash required: `true`
- Operator identity hash recorded: `false`
- Single surface activation scope required: `true`
- Single surface activation scope recorded: `false`
- Receipt payload hash required: `true`
- Receipt payload hash recorded: `false`
- Redacted payload summary hash required: `true`
- Redacted payload summary hash recorded: `false`
- Receipt output path redacted required: `true`
- Receipt output path redacted recorded: `false`
- Accepted redaction proof IDs required: `true`
- Accepted redaction proof IDs recorded: `false`
- Fresh pre-activation soak evidence required: `true`
- Fresh pre-activation soak evidence recorded: `false`
- Active binary SHA required: `true`
- Active binary SHA recorded: `false`
- Rollback plan required: `true`
- Rollback plan recorded: `false`
- Public artifact policy required: `true`
- Public artifact policy recorded: `false`
- Filesystem persistence allowed count: `0`
- Filesystem persistence execution performed count: `0`
- Filesystem write performed count: `0`
- Workspace write performed count: `0`
- Receipt materialized count: `0`
- Receipt persisted count: `0`
- Filesystem persistence approval packet ready: `true`
- Activation allowed: `false`
- Live mutation execution ready: `false`

## Required Approval Fields

- `filesystem_persistence_approval_id`
- `operator_approval_id`
- `operator_identity_hash`
- `single_surface_activation_scope`
- `receipt_materialization_plan_id`
- `receipt_payload_hash`
- `redacted_payload_summary_sha256`
- `receipt_output_path_redacted`
- `accepted_redaction_proof_ids`
- `fresh_pre_activation_soak_evidence_id`
- `active_binary_sha256`
- `rollback_plan_id`
- `public_claim_and_artifact_decision`

## Fixtures

All fixtures explicitly request a filesystem persistence approval packet, then
stay blocked as schema-only plans:

- `approval-without-operator-scope`
- `approval-with-command-disabled`
- `approval-with-persistence-disabled`
- `approval-for-plaintext-payload-attempt`
- `approval-for-public-artifact-attempt`

Every fixture keeps:

- `filesystem_persistence_approval_requested = true`
- `approval_status = blocked_schema_only`
- `recorded_approval_field_count = 0`
- `materialization_plan_recorded = false`
- `operator_identity_hash_recorded = false`
- `single_surface_activation_scope_recorded = false`
- `accepted_redaction_proof_ids_recorded = false`
- `fresh_pre_activation_soak_evidence_recorded = false`
- `active_binary_sha_recorded = false`
- `rollback_plan_recorded = false`
- `public_artifact_policy_recorded = false`
- `command_invocation_performed = false`
- `command_execution_performed = false`
- `receipt_persistence_execution_performed = false`
- `materialization_execution_performed = false`
- `filesystem_persistence_execution_performed = false`
- `filesystem_write_performed = false`
- `workspace_write_performed = false`
- `receipt_materialized = false`
- `receipt_persisted = false`
- `activation_allowed = false`

Only the redacted non-public fixture shapes can carry a deterministic
materialization plan. Plaintext and public-artifact attempts are represented but
remain denied.

## Safety Boundary

The gate must not:

- write memory, skills, plugin registries, or capability registries
- invoke providers or models
- send channel messages
- mutate runtime stores or Gateway queues
- invoke or execute a receipt command
- execute materialization
- execute filesystem persistence
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

The next safe step is:

`scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-output-path-allowlist-gate.sh`

It defines the redacted report-only output-path allowlist, still without
command execution, materialization execution, filesystem persistence execution,
output-path selection, receipt persistence, filesystem writes, or live mutation.
