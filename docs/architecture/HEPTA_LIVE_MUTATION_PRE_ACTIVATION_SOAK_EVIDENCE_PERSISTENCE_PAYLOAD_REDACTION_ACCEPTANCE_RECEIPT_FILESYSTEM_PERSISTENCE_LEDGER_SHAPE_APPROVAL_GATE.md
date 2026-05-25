# Hepta Live Mutation Pre-Activation Soak Evidence Persistence Payload Redaction Acceptance Receipt Filesystem Persistence Ledger Shape Approval Gate

Date: 2026-05-25

This gate sits after the payload redaction acceptance receipt filesystem
persistence dry-run ledger gate. It defines the required future ledger shape and
approval fields before any dry-run ledger entry could ever be recorded or
persisted, while keeping the current gate schema-only and report-only.

The gate does not record an approval, materialize a ledger shape, persist a
ledger, select an output path, invoke or execute a command, execute
materialization, execute filesystem persistence, persist a receipt, write a
file, inspect payload plaintext, run a live secret scan, send a channel message,
invoke a provider or model, mutate Gateway/runtime stores, restart launchd,
execute rollback, or enable live mutation.

## Contract

The gate consumes:

- `scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-dry-run-ledger-gate.sh`
- the source dry-run ledger report hash
- the source execution-denial matrix report hash
- the source sink write preview report hash
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

It requires the source dry-run ledger to be `ready`, but keeps these values false
or zero:

- `recorded_ledger_shape_field_count`
- `ledger_shape_approval_performed_count`
- `ledger_shape_approval_recorded`
- `ledger_shape_approval_persisted`
- `ledger_shape_materialized`
- `ledger_shape_filesystem_written`
- `dry_run_ledger_recorded`
- `dry_run_ledger_persisted`
- `dry_run_ledger_materialized`
- `dry_run_ledger_filesystem_written`
- `ledger_persistence_allowed`
- `ledger_persistence_allowed_count`
- `ledger_persistence_execution_requested_count`
- `ledger_persistence_execution_performed`
- `ledger_persistence_execution_performed_count`
- `receipt_persistence_allowed_count`
- `receipt_persistence_execution_performed_count`
- `filesystem_persistence_execution_performed`
- `filesystem_persistence_execution_performed_count`
- `filesystem_write_performed`
- `filesystem_write_performed_count`
- `workspace_write_performed`
- `workspace_write_performed_count`
- `command_invocation_performed_count`
- `command_execution_performed_count`
- `materialization_execution_performed_count`
- `receipt_materialized_count`
- `receipt_persisted_count`
- `selected_output_path_count`
- `recorded_output_path_count`
- `recorded_path_binding_count`
- `active_binary_sha_bound_count`
- `trusted_source_bound_count`
- `operator_scope_bound_count`
- `accepted_redaction_proof_bound_count`
- `public_claim_allowed`
- `release_artifact_write_allowed`
- `raw_payload_plaintext_recorded`
- `raw_payload_plaintext_persisted`
- `raw_payload_inspected`
- `live_secret_scan_performed`
- `receipt_persistence_enabled`
- `activation_allowed`
- `live_mutation_execution_ready`

## Ledger Shape Status

- Required ledger shape field count: `16`
- Ledger shape field count: `16`
- Recorded ledger shape field count: `0`
- Redacted or hashed ledger shape field count: `15`
- Required before any ledger persistence count: `16`
- Source dry-run ledger entry count: `4`
- Source dry-run ledger entry hash count: `4`
- Source denied dry-run ledger entry count: `4`
- Source allowed dry-run ledger entry count: `0`
- Required ledger shape fixture count: `4`
- Ledger shape fixture count: `4`
- Blocked ledger shape fixture count: `4`
- Allowed ledger shape fixture count: `0`
- Ledger shape approval requested count: `4`
- Ledger shape approval performed count: `0`
- Ledger persistence allowed count: `0`
- Ledger persistence execution requested count: `0`
- Ledger persistence execution performed count: `0`
- Activation blocked by ledger shape approval: `true`
- Activation allowed by ledger shape approval: `false`
- Live mutation execution ready: `false`

## Required Ledger Shape Fields

The future approval shape is explicit and stable:

- `ledger_shape_approval_id`
- `ledger_schema_version`
- `source_dry_run_ledger_gate_report_sha256`
- `source_execution_denial_matrix_report_sha256`
- `dry_run_ledger_entry_hashes`
- `filesystem_persistence_approval_ids`
- `operator_approval_id`
- `operator_identity_hash`
- `single_surface_activation_scope`
- `fresh_pre_activation_soak_evidence_id`
- `active_binary_sha256`
- `trusted_source_binding`
- `accepted_redaction_proof_ids`
- `redacted_receipt_output_path`
- `rollback_plan_id`
- `public_claim_and_artifact_decision`

The gate also requires `rollback_rehearsal_evidence_id` before any future ledger
persistence can execute.

## Shape Fixtures

The four shape-approval fixtures are deterministic, redacted, and blocked:

- `missing-persistence-approval-id-ledger-shape-approval`
- `stale-pre-activation-soak-evidence-ledger-shape-approval`
- `workspace-path-ledger-shape-approval`
- `public-artifact-ledger-shape-approval`

Each fixture references one source dry-run ledger entry and a deterministic
ledger-shape hash. Each fixture requests a hypothetical ledger shape approval,
but keeps approval recording, approval persistence, ledger persistence,
filesystem persistence execution, filesystem write, workspace write, activation,
and live mutation all false.

The ledger shape approval gate explicitly denies:

- missing persistence approval id
- stale or missing fresh pre-activation soak evidence
- source-tree and workspace output-path attempts
- public claim and release-artifact write attempts
- ledger shape approval recording
- ledger persistence execution
- receipt persistence
- filesystem persistence execution
- filesystem writes
- workspace writes
- public release claims
- release artifact writes
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
- record, materialize, persist, or write ledger shape approvals
- record, materialize, persist, or write dry-run ledger entries
- select a filesystem output path
- write sink previews, execution-denial matrices, output-path bindings,
  output-path allowlists, approval packets, operator-scope records, payload
  reviews, redaction proofs, acceptance matrices, no-write sink records,
  write-enable fixture records, materialization plans, receipt files, evidence
  files, ledgers, or release artifacts
- persist plaintext payloads
- inspect raw payload plaintext
- run live secret scans
- persist pre-activation soak evidence
- restart launchd
- execute rollback
- read credentials or secret files

The next safe step is a redaction acceptance receipt filesystem persistence
ledger persistence rehearsal denial gate, still without command execution,
materialization execution, filesystem persistence execution, output-path
selection, ledger materialization, receipt persistence, filesystem writes, or
live mutation.
