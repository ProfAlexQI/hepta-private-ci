# Hepta Live Mutation Pre-Activation Soak Evidence Persistence Payload Redaction Acceptance Receipt Filesystem Persistence Ledger Persistence Rehearsal Receipt Contract Gate

Date: 2026-05-25

This gate sits after the payload redaction acceptance receipt filesystem
persistence ledger persistence rehearsal denial gate. It defines the future
receipt contract for ledger-persistence rehearsal outcomes while keeping the
current system report-only and side-effect free.

The gate does not record a receipt contract, materialize a rehearsal receipt,
persist a ledger or receipt, select a write path, invoke or execute a command,
execute materialization, execute filesystem persistence, write a file, inspect
payload plaintext, run a live secret scan, send a channel message, invoke a
provider or model, mutate Gateway/runtime stores, restart launchd, execute
rollback, or enable live mutation.

## Contract

The gate consumes:

- `scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-denial-gate.sh`
- the source rehearsal-denial report hash
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

It requires the source rehearsal-denial gate to be `ready`, but keeps these
values false or zero:

- `recorded_rehearsal_receipt_contract_field_count`
- `rehearsal_receipt_contract_recorded_count`
- `rehearsal_receipt_contract_persisted_count`
- `rehearsal_receipt_contract_materialized_count`
- `rehearsal_receipt_contract_filesystem_written_count`
- `rehearsal_receipt_materialized_count`
- `rehearsal_receipt_persisted_count`
- `ledger_persistence_rehearsal_performed_count`
- `ledger_persistence_allowed`
- `ledger_persistence_allowed_count`
- `ledger_persistence_execution_requested_count`
- `ledger_persistence_execution_performed`
- `ledger_persistence_execution_performed_count`
- `ledger_recorded`
- `ledger_persisted`
- `ledger_materialized`
- `ledger_filesystem_written`
- `ledger_write_path_selected`
- `ledger_write_path_recorded`
- `receipt_persistence_allowed_count`
- `receipt_persistence_execution_performed_count`
- `receipt_materialized_count`
- `receipt_persisted_count`
- `filesystem_persistence_allowed`
- `filesystem_persistence_allowed_count`
- `filesystem_persistence_execution_requested_count`
- `filesystem_persistence_execution_performed`
- `filesystem_persistence_execution_performed_count`
- `filesystem_write_requested_count`
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

## Receipt Contract Status

- Required rehearsal receipt contract field count: `22`
- Rehearsal receipt contract field count: `22`
- Recorded rehearsal receipt contract field count: `0`
- Redacted or hashed rehearsal receipt contract field count: `20`
- Required rehearsal receipt contract fixture count: `4`
- Rehearsal receipt contract fixture count: `4`
- Blocked rehearsal receipt contract fixture count: `4`
- Allowed rehearsal receipt contract fixture count: `0`
- Rehearsal receipt contract hash count: `4`
- Rehearsal receipt contract requested count: `4`
- Rehearsal receipt contract recorded count: `0`
- Rehearsal receipt contract persisted count: `0`
- Rehearsal receipt contract materialized count: `0`
- Rehearsal receipt contract filesystem written count: `0`
- Rehearsal receipt requested count: `4`
- Rehearsal receipt materialized count: `0`
- Rehearsal receipt persisted count: `0`
- Activation blocked by rehearsal receipt contract: `true`
- Activation allowed by rehearsal receipt contract: `false`
- Live mutation execution ready: `false`

## Required Rehearsal Receipt Contract Fields

The future rehearsal receipt contract shape is explicit and stable:

- `ledger_persistence_rehearsal_receipt_id`
- `receipt_schema_version`
- `source_rehearsal_denial_report_sha256`
- `source_ledger_shape_approval_report_sha256`
- `source_dry_run_ledger_report_sha256`
- `rehearsal_fixture_id`
- `deterministic_rehearsal_sha256`
- `denial_reason_codes`
- `operator_approval_id`
- `operator_identity_hash`
- `single_surface_activation_scope`
- `filesystem_persistence_approval_id`
- `fresh_pre_activation_soak_evidence_id`
- `active_binary_sha256`
- `trusted_source_binding`
- `accepted_redaction_proof_ids`
- `receipt_payload_hash`
- `redacted_payload_summary_sha256`
- `receipt_output_path_redacted`
- `rollback_plan_id`
- `rollback_rehearsal_evidence_id`
- `public_claim_and_artifact_decision`

## Receipt Contract Fixtures

The four rehearsal receipt contract fixtures are deterministic, redacted, and
blocked:

- `missing-ledger-shape-approval-rehearsal-receipt-contract`
- `stale-pre-activation-soak-rehearsal-receipt-contract`
- `workspace-path-rehearsal-receipt-contract`
- `public-artifact-rehearsal-receipt-contract`

Each fixture references one source rehearsal-denial fixture and a deterministic
receipt-contract hash. Each fixture requests a hypothetical future rehearsal
receipt, but keeps contract recording, contract persistence, contract
materialization, receipt materialization, receipt persistence, ledger
persistence rehearsal, ledger persistence execution, filesystem persistence
execution, filesystem write, workspace write, activation, and live mutation all
false.

The receipt-contract gate explicitly denies:

- missing ledger shape approval
- missing filesystem persistence approval id
- stale or missing fresh pre-activation soak evidence
- missing rollback rehearsal evidence
- source-tree and workspace output-path attempts
- public claim and release-artifact write attempts
- rehearsal receipt contract recording
- rehearsal receipt materialization
- rehearsal receipt persistence
- ledger persistence execution
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
- record, materialize, persist, or write rehearsal receipt contracts
- record, materialize, persist, or write rehearsal receipts
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
ledger persistence rehearsal receipt review gate, still without command
execution, materialization execution, filesystem persistence execution,
output-path selection, ledger materialization, receipt persistence, filesystem
writes, or live mutation.
