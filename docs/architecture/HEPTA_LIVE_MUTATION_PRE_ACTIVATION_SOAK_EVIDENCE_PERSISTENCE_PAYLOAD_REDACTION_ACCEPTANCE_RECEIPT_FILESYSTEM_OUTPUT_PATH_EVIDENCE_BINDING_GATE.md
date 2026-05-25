# Hepta Live Mutation Pre-Activation Soak Evidence Persistence Payload Redaction Acceptance Receipt Filesystem Output Path Evidence Binding Gate

Date: 2026-05-25

This gate sits after the payload redaction acceptance receipt filesystem
output-path allowlist gate. It binds every future receipt output destination to
the evidence fields that must be present before a filesystem sink may select a
path.

The gate is still report-only. It does not record an output-path binding, select
an output path, record fresh evidence, record active binary SHA, invoke or
execute a command, execute materialization, execute filesystem persistence,
persist a receipt, write a file, inspect payload plaintext, run a live secret
scan, send a channel message, invoke a provider or model, mutate
Gateway/runtime stores, or enable live mutation.

## Contract

The gate consumes:

- `scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-output-path-allowlist-gate.sh`
- the source payload redaction acceptance receipt filesystem output-path
  allowlist report hash
- the source payload redaction acceptance receipt filesystem persistence
  approval-packet report hash
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

It requires the source output-path allowlist gate to be `ready`, but keeps these
values false or zero:

- `payload_redaction_acceptance_receipt_command_recorded`
- `payload_redaction_acceptance_receipt_command_enabled_by_default`
- `payload_redaction_acceptance_receipt_command_invocation_performed_count`
- `payload_redaction_acceptance_receipt_command_execution_performed_count`
- `payload_redaction_acceptance_receipt_recorded`
- `payload_redaction_acceptance_receipt_materialized`
- `payload_redaction_acceptance_receipt_persisted`
- `filesystem_persistence_approval_packet_recorded`
- `filesystem_persistence_approval_packet_persisted`
- `output_path_allowlist_recorded`
- `output_path_allowlist_persisted`
- `output_path_evidence_binding_recorded`
- `output_path_evidence_binding_persisted`
- `payload_redaction_acceptance_matrix_recorded`
- `payload_redaction_acceptance_matrix_persisted`
- `payload_redaction_proof_recorded`
- `payload_redaction_proof_accepted`
- `accepted_redaction_proof_count`
- `default_selected_output_path_count`
- `selected_output_path_count`
- `recorded_output_path_count`
- `recorded_path_binding_count`
- `fresh_pre_activation_soak_evidence_bound_count`
- `active_binary_sha_bound_count`
- `operator_scope_bound_count`
- `accepted_redaction_proof_bound_count`
- `trusted_source_bound_count`
- `source_tree_path_binding_allowed`
- `home_directory_path_binding_allowed`
- `release_artifact_path_binding_allowed`
- `public_artifact_path_binding_allowed`
- `receipt_output_path_selected`
- `receipt_output_path_recorded`
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
- `filesystem_persistence_allowed`
- `filesystem_persistence_allowed_count`
- `command_invocation_performed_count`
- `command_execution_performed_count`
- `receipt_persistence_execution_performed_count`
- `materialization_execution_performed_count`
- `filesystem_persistence_execution_performed`
- `filesystem_persistence_execution_performed_count`
- `filesystem_write_performed`
- `filesystem_write_performed_count`
- `workspace_write_performed`
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

## Binding Status

- Required path binding count: `11`
- Path binding count: `11`
- Required allowlist entry count: `6`
- Allowlist entry count: `6`
- Allowed output path entry count: `3`
- Blocked output path entry count: `3`
- Redacted output path entry count: `6`
- Default selected output path count: `0`
- Selected output path count: `0`
- Recorded output path count: `0`
- Recorded path binding count: `0`
- Eligible report-only root count: `3`
- Blocked mutating root count: `3`
- Fresh pre-activation soak evidence bound count: `0`
- Active binary SHA bound count: `0`
- Operator scope bound count: `0`
- Accepted redaction proof bound count: `0`
- Redacted or hashed binding count: `11`
- Trusted source bound count: `0`
- Source tree path binding allowed: `false`
- Home directory path binding allowed: `false`
- Release artifact path binding allowed: `false`
- Public artifact path binding allowed: `false`
- Receipt output path allowlist ready: `true`
- Receipt output path evidence binding ready: `true`
- Receipt output path selected: `false`
- Receipt output path recorded: `false`
- Filesystem persistence allowed: `false`
- Filesystem persistence allowed count: `0`
- Filesystem persistence execution performed: `false`
- Filesystem persistence execution performed count: `0`
- Filesystem write performed: `false`
- Filesystem write performed count: `0`
- Workspace write performed: `false`
- Workspace write performed count: `0`
- Receipt materialized count: `0`
- Receipt persisted count: `0`
- Activation blocked by output path evidence binding: `true`
- Activation allowed by output path evidence binding: `false`
- Live mutation execution ready: `false`

## Required Evidence Bindings

- `activation_request_id`
- `operator_approval_id`
- `operator_identity_hash`
- `single_surface_activation_scope`
- `receipt_payload_hash`
- `redacted_payload_summary_hash`
- `accepted_redaction_proof_ids`
- `fresh_pre_activation_soak_evidence`
- `active_binary_sha256`
- `rollback_plan_id`
- `public_artifact_policy`

## Allowed Output Path Bindings

Allowed only as future report-only receipt persistence candidates, never
selected by default:

- `payload_redaction_acceptance_receipts_root`
- `payload_redaction_acceptance_receipt_dry_run_root`
- `payload_redaction_acceptance_receipt_operator_packet_root`

Blocked roots remain denied for path binding:

- `source_tree_root`
- `home_directory_root`
- `release_artifact_root`

The release artifact root also covers public artifact attempts; public release
or public GA claims remain denied.

## Safety Boundary

The gate must not:

- write memory, skills, plugin registries, or capability registries
- invoke providers or models
- send channel messages
- mutate runtime stores or Gateway queues
- invoke or execute a receipt command
- execute materialization
- execute filesystem persistence
- record an output-path binding
- select a filesystem output path
- write output-path bindings, output-path allowlists, approval packets,
  operator-scope records, payload reviews, redaction proofs, acceptance
  matrices, no-write sink records, write-enable fixture records,
  materialization plans, receipt files, evidence files, or release artifacts
- persist plaintext payloads
- inspect raw payload plaintext
- run live secret scans
- persist pre-activation soak evidence
- restart launchd
- execute rollback
- read credentials or secret files

The next safe step is a redaction acceptance receipt filesystem sink write
preview gate, still without command execution, materialization execution,
filesystem persistence execution, output-path selection, receipt persistence,
filesystem writes, or live mutation.
