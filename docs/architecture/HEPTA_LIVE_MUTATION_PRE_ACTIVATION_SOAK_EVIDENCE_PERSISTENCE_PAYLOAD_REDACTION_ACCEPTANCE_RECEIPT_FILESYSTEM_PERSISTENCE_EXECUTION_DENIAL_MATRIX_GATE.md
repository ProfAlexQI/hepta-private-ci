# Hepta Live Mutation Pre-Activation Soak Evidence Persistence Payload Redaction Acceptance Receipt Filesystem Persistence Execution Denial Matrix Gate

Date: 2026-05-25

This gate sits after the payload redaction acceptance receipt filesystem sink
write preview gate. It models four explicit filesystem persistence execution
attempts and keeps all of them blocked. The purpose is to make the next
boundary precise: a sink write preview is not a receipt persistence action, and
future filesystem persistence still requires an explicit persistence approval
id, fresh evidence, safe output roots, and no public artifact write request.

The gate is still report-only. It does not select an output path, invoke or
execute a command, execute materialization, execute filesystem persistence,
persist a receipt, write a file, inspect payload plaintext, run a live secret
scan, send a channel message, invoke a provider or model, mutate
Gateway/runtime stores, or enable live mutation.

## Contract

The gate consumes:

- `scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-sink-write-preview-gate.sh`
- the source payload redaction acceptance receipt filesystem sink write preview
  report hash
- the source output-path evidence binding report hash
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

It requires the source sink write preview gate to be `ready`, but keeps these
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
- `filesystem_sink_write_preview_recorded`
- `filesystem_sink_write_preview_persisted`
- `execution_denial_matrix_recorded`
- `execution_denial_matrix_persisted`
- `payload_redaction_acceptance_matrix_recorded`
- `payload_redaction_acceptance_matrix_persisted`
- `payload_redaction_proof_recorded`
- `payload_redaction_proof_accepted`
- `accepted_redaction_proof_count`
- `active_binary_sha_bound_fixture_count`
- `trusted_source_bound_fixture_count`
- `operator_scope_bound_fixture_count`
- `accepted_redaction_proof_bound_fixture_count`
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

## Denial Matrix Status

- Source preview fixture count: `3`
- Required denial fixture count: `4`
- Denial fixture count: `4`
- Execution requested fixture count: `4`
- Future persistence approval slot count: `4`
- Explicit persistence approval id present count: `3`
- Explicit persistence approval id missing count: `1`
- Stale or missing fresh pre-activation soak evidence fixture count: `1`
- Stale or missing fresh evidence fixture count: `1`
- Future active binary SHA bound fixture count: `4`
- Future trusted source bound fixture count: `4`
- Future operator scope bound fixture count: `3`
- Future accepted redaction proof bound fixture count: `3`
- Active binary SHA bound fixture count: `0`
- Trusted source bound fixture count: `0`
- Operator scope bound fixture count: `0`
- Accepted redaction proof bound fixture count: `0`
- Source tree path attempt fixture count: `1`
- Workspace path attempt fixture count: `1`
- Public claim attempt fixture count: `1`
- Release artifact write attempt fixture count: `1`
- Blocked execution fixture count: `4`
- Allowed execution fixture count: `0`
- Filesystem persistence allowed count: `0`
- Filesystem persistence execution requested count: `4`
- Filesystem persistence execution performed count: `0`
- Filesystem write requested count: `4`
- Filesystem write performed count: `0`
- Workspace write performed count: `0`
- Receipt materialized count: `0`
- Receipt persisted count: `0`
- Activation blocked by execution denial matrix: `true`
- Activation allowed by execution denial matrix: `false`
- Live mutation execution ready: `false`

## Denial Fixtures

The four execution attempts are deterministic, redacted, and blocked:

- `missing-persistence-approval-id-execution-attempt`
- `stale-pre-activation-soak-evidence-execution-attempt`
- `workspace-path-execution-attempt`
- `public-artifact-execution-attempt`

Each fixture requests a hypothetical filesystem persistence execution shape, but
keeps command invocation, command execution, materialization execution,
filesystem persistence execution, filesystem write, workspace write, receipt
persistence, and activation all false.

The matrix explicitly denies:

- missing persistence approval id
- stale or missing fresh pre-activation soak evidence
- source-tree and workspace output-path attempts
- public claim and release-artifact write attempts
- command invocation and execution
- materialization execution
- filesystem persistence execution
- filesystem writes
- workspace writes
- receipt persistence
- live mutation execution

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
- write sink previews, execution-denial matrices, output-path bindings,
  output-path allowlists, approval packets, operator-scope records, payload
  reviews, redaction proofs, acceptance matrices, no-write sink records,
  write-enable fixture records, materialization plans, receipt files, evidence
  files, or release artifacts
- persist plaintext payloads
- inspect raw payload plaintext
- run live secret scans
- persist pre-activation soak evidence
- restart launchd
- execute rollback
- read credentials or secret files

The next safe step is a redaction acceptance receipt filesystem persistence
receipt dry-run ledger, still without command execution, materialization
execution, filesystem persistence execution, output-path selection, receipt
persistence, filesystem writes, or live mutation.
