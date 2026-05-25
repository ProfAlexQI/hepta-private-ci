# Hepta Live Mutation Pre-Activation Soak Evidence Persistence Payload Redaction Acceptance Receipt Filesystem Sink Write Preview Gate

Date: 2026-05-25

This gate sits after the payload redaction acceptance receipt filesystem
output-path evidence binding gate. It models deterministic sink-write previews
for the three report-only receipt roots, while keeping every write,
persistence, command, materialization, Gateway, provider, channel, and live
mutation path disabled.

The gate is still report-only. It does not select an output path, invoke or
execute a command, execute materialization, execute filesystem persistence,
persist a receipt, write a file, inspect payload plaintext, run a live secret
scan, send a channel message, invoke a provider or model, mutate
Gateway/runtime stores, or enable live mutation.

## Contract

The gate consumes:

- `scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-output-path-evidence-binding-gate.sh`
- the source payload redaction acceptance receipt filesystem output-path
  evidence binding report hash
- the source output-path allowlist report hash
- the source filesystem persistence approval-packet report hash
- the source materialization dry-run report hash
- the source write-enable fixture report hash
- the source no-write sink contract report hash
- the source invocation dry-run report hash
- the source receipt command contract report hash
- the source payload redaction acceptance matrix report hash
- the source payload redaction proof report hash
- the source no-secret payload review report hash
- the source operator-scope report hash
- the source approval-packet report hash
- the source approval receipt payload hash
- the source pre-activation soak report hash
- the source persistence-denial report hash
- the minimum 24-sample pre-activation soak requirement

It requires the source output-path evidence binding gate to be `ready`, but keeps
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
- `output_path_allowlist_recorded`
- `output_path_allowlist_persisted`
- `output_path_evidence_binding_recorded`
- `output_path_evidence_binding_persisted`
- `filesystem_sink_write_preview_recorded`
- `filesystem_sink_write_preview_persisted`
- `payload_redaction_acceptance_matrix_recorded`
- `payload_redaction_acceptance_matrix_persisted`
- `payload_redaction_proof_recorded`
- `payload_redaction_proof_accepted`
- `accepted_redaction_proof_count`
- `fresh_pre_activation_soak_evidence_bound_fixture_count`
- `active_binary_sha_bound_fixture_count`
- `operator_scope_bound_fixture_count`
- `accepted_redaction_proof_bound_fixture_count`
- `trusted_source_bound_fixture_count`
- `selected_output_path_count`
- `recorded_output_path_count`
- `recorded_path_binding_count`
- `receipt_output_path_selected`
- `receipt_output_path_recorded`
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

## Preview Status

- Required preview fixture count: `3`
- Preview fixture count: `3`
- Allowed output path entry count: `3`
- Blocked output path entry count: `3`
- Previewed output path count: `3`
- Report-only root preview count: `3`
- Mutating root preview count: `0`
- Deterministic payload hash count: `3`
- Redacted output path preview count: `3`
- Fresh pre-activation soak evidence bound fixture count: `0`
- Active binary SHA bound fixture count: `0`
- Operator scope bound fixture count: `0`
- Accepted redaction proof bound fixture count: `0`
- Trusted source bound fixture count: `0`
- Blocked preview fixture count: `3`
- Allowed preview fixture count: `0`
- Source tree path preview allowed: `false`
- Home directory path preview allowed: `false`
- Release artifact path preview allowed: `false`
- Public artifact path preview allowed: `false`
- Default selected output path count: `0`
- Selected output path count: `0`
- Recorded output path count: `0`
- Recorded path binding count: `0`
- Filesystem persistence allowed count: `0`
- Filesystem write requested count: `3`
- Filesystem write performed count: `0`
- Workspace write performed count: `0`
- Receipt materialized count: `0`
- Receipt persisted count: `0`
- Activation blocked by sink write preview: `true`
- Activation allowed by sink write preview: `false`
- Live mutation execution ready: `false`

## Preview Fixtures

The three preview fixtures are deterministic, redacted, and blocked:

- `payload-redaction-acceptance-receipts-root-sink-write-preview`
- `payload-redaction-acceptance-receipt-dry-run-root-sink-write-preview`
- `payload-redaction-acceptance-receipt-operator-packet-root-sink-write-preview`

Each fixture binds the source output-path evidence binding report hash and a
deterministic preview payload hash. Each fixture requests a hypothetical
filesystem write shape, but keeps command invocation, command execution,
materialization execution, filesystem persistence execution, filesystem write,
receipt persistence, and activation all false.

Blocked roots remain denied for sink write preview:

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
- select a filesystem output path
- write sink previews, output-path bindings, output-path allowlists, approval
  packets, operator-scope records, payload reviews, redaction proofs,
  acceptance matrices, no-write sink records, write-enable fixture records,
  materialization plans, receipt files, evidence files, or release artifacts
- persist plaintext payloads
- inspect raw payload plaintext
- run live secret scans
- persist pre-activation soak evidence
- restart launchd
- execute rollback
- read credentials or secret files

The next safe step is a redaction acceptance receipt filesystem persistence
execution-denial matrix, still without command execution, materialization
execution, filesystem persistence execution, output-path selection, receipt
persistence, filesystem writes, or live mutation.
