# Hepta Live Mutation Pre-Activation Soak Evidence Persistence Payload Redaction Acceptance Receipt Filesystem Persistence Ledger Persistence Rehearsal Denial Gate

Date: 2026-05-25

This gate sits after the payload redaction acceptance receipt filesystem
persistence ledger shape approval gate. It rehearses the denial policy for
future ledger persistence attempts while keeping the current gate report-only
and side-effect free.

The gate does not record a ledger shape approval, persist a ledger, select a
write path, invoke or execute a command, execute materialization, execute
filesystem persistence, persist a receipt, write a file, inspect payload
plaintext, run a live secret scan, send a channel message, invoke a provider or
model, mutate Gateway/runtime stores, restart launchd, execute rollback, or
enable live mutation.

## Contract

The gate consumes:

- `scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-shape-approval-gate.sh`
- the source ledger-shape approval report hash
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

It requires the source ledger-shape approval gate to be `ready`, but keeps these
values false or zero:

- `ledger_persistence_rehearsal_performed_count`
- `ledger_persistence_allowed`
- `ledger_persistence_allowed_count`
- `ledger_persistence_execution_performed`
- `ledger_persistence_execution_performed_count`
- `ledger_recorded`
- `ledger_persisted`
- `ledger_materialized`
- `ledger_filesystem_written`
- `ledger_write_path_selected`
- `ledger_write_path_recorded`
- `ledger_shape_approval_recorded`
- `ledger_shape_approval_persisted`
- `dry_run_ledger_recorded`
- `dry_run_ledger_persisted`
- `dry_run_ledger_materialized`
- `dry_run_ledger_filesystem_written`
- `receipt_persistence_allowed_count`
- `receipt_persistence_execution_performed_count`
- `receipt_materialized_count`
- `receipt_persisted_count`
- `filesystem_persistence_allowed`
- `filesystem_persistence_allowed_count`
- `filesystem_persistence_execution_performed`
- `filesystem_persistence_execution_performed_count`
- `filesystem_write_performed`
- `filesystem_write_performed_count`
- `workspace_write_performed`
- `workspace_write_performed_count`
- `command_invocation_requested_count`
- `command_invocation_performed_count`
- `command_execution_requested_count`
- `command_execution_performed_count`
- `materialization_execution_requested_count`
- `materialization_execution_performed_count`
- `selected_output_path_count`
- `recorded_output_path_count`
- `recorded_path_binding_count`
- `active_binary_sha_bound_count`
- `trusted_source_bound_count`
- `operator_scope_bound_count`
- `accepted_redaction_proof_bound_count`
- `rollback_rehearsal_evidence_recorded_count`
- `public_claim_allowed`
- `release_artifact_write_allowed`
- `raw_payload_plaintext_recorded`
- `raw_payload_plaintext_persisted`
- `raw_payload_inspected`
- `live_secret_scan_performed`
- `receipt_persistence_enabled`
- `activation_allowed`
- `live_mutation_execution_ready`

## Rehearsal Status

- Required ledger persistence rehearsal fixture count: `4`
- Ledger persistence rehearsal fixture count: `4`
- Blocked ledger persistence rehearsal fixture count: `4`
- Allowed ledger persistence rehearsal fixture count: `0`
- Ledger persistence rehearsal requested count: `4`
- Ledger persistence rehearsal performed count: `0`
- Ledger persistence execution requested count: `4`
- Ledger persistence execution performed count: `0`
- Filesystem persistence execution requested count: `4`
- Filesystem persistence execution performed count: `0`
- Filesystem write requested count: `4`
- Filesystem write performed count: `0`
- Future rollback rehearsal evidence slot count: `4`
- Rollback rehearsal evidence recorded count: `0`
- Activation blocked by ledger persistence rehearsal denial: `true`
- Activation allowed by ledger persistence rehearsal denial: `false`
- Live mutation execution ready: `false`

## Required Before Any Ledger Persistence Rehearsal

The future rehearsal shape is explicit and stable:

- `explicit_operator_enablement_for_ledger_persistence`
- `ledger_shape_approval_id`
- `ledger_shape_approval_report_sha256`
- `filesystem_persistence_approval_id`
- `operator_approval_id`
- `operator_identity_hash`
- `single_surface_activation_scope`
- `accepted_redaction_proof_ids`
- `fresh_pre_activation_soak_evidence`
- `active_binary_sha256`
- `trusted_source_binding`
- `receipt_payload_hash`
- `redacted_payload_summary_sha256`
- `receipt_output_path_redacted`
- `rollback_plan_id`
- `rollback_rehearsal_evidence_id`
- `public_claim_and_artifact_decision`
- `ledger_persistence_rehearsal_denial_report_sha256`

## Rehearsal Fixtures

The four ledger-persistence rehearsal fixtures are deterministic, redacted, and
blocked:

- `missing-ledger-shape-approval-ledger-persistence-rehearsal`
- `stale-pre-activation-soak-ledger-persistence-rehearsal`
- `workspace-path-ledger-persistence-rehearsal`
- `public-artifact-ledger-persistence-rehearsal`

Each fixture references one source ledger-shape approval fixture and a
deterministic rehearsal hash. Each fixture requests a hypothetical future ledger
persistence rehearsal, but keeps command invocation, command execution,
materialization execution, ledger persistence execution, receipt persistence,
filesystem persistence execution, filesystem write, workspace write, activation,
and live mutation all false.

The rehearsal-denial gate explicitly denies:

- missing ledger shape approval
- missing filesystem persistence approval id
- stale or missing fresh pre-activation soak evidence
- missing rollback rehearsal evidence
- source-tree and workspace output-path attempts
- public claim and release-artifact write attempts
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
- record, materialize, persist, or write ledger persistence rehearsals
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
ledger persistence rehearsal receipt contract gate, still without command
execution, materialization execution, filesystem persistence execution,
output-path selection, ledger materialization, receipt persistence, filesystem
writes, or live mutation.
