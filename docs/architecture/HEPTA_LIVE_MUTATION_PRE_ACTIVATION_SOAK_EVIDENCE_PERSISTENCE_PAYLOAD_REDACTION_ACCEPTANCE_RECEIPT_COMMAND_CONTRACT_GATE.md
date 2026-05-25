# Hepta Live Mutation Pre-Activation Soak Evidence Persistence Payload Redaction Acceptance Receipt Command Contract Gate

Date: 2026-05-25

This gate sits after the payload redaction acceptance matrix gate. It defines the
dry-run command contract for a future redaction acceptance receipt before any
receipt command can be invoked.

The gate does not record a command, invoke a command, accept a redaction proof,
persist a receipt, inspect payload plaintext, run a live secret scan, write a
file, send a channel message, invoke a provider or model, or enable live
mutation.

## Contract

The gate consumes:

- `scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-matrix-gate.sh`
- the source payload redaction acceptance matrix report hash
- the source payload redaction proof report hash
- the source no-secret payload review report hash
- the source operator-scope report hash
- the source approval-packet report hash
- the source approval receipt payload hash
- the source pre-activation soak report hash
- the source persistence-denial report hash
- the minimum 24-sample pre-activation soak requirement

It requires the source payload redaction acceptance matrix gate to be `ready`,
but keeps these values false:

- `payload_redaction_acceptance_receipt_command_recorded`
- `payload_redaction_acceptance_receipt_command_enabled_by_default`
- `payload_redaction_acceptance_receipt_command_invoked`
- `payload_redaction_acceptance_receipt_command_execution_performed`
- `payload_redaction_acceptance_receipt_recorded`
- `payload_redaction_acceptance_receipt_persisted`
- `payload_redaction_acceptance_matrix_recorded`
- `payload_redaction_acceptance_matrix_persisted`
- `payload_redaction_proof_recorded`
- `payload_redaction_proof_accepted`
- `raw_payload_plaintext_recorded`
- `raw_payload_plaintext_persisted`
- `live_secret_scan_performed`
- `receipt_persistence_enabled`
- `receipt_persisted`
- `activation_allowed`
- `live_mutation_execution_ready`

## Command Fields

The gate models twelve fields required before a future acceptance receipt command
can be invoked:

- `payload_redaction_acceptance_receipt_command_id`
- `payload_redaction_acceptance_matrix_id`
- `payload_redaction_proof_id`
- `operator_approval_id`
- `operator_identity_hash`
- `single_surface_activation_scope`
- `source_payload_redaction_acceptance_matrix_report_sha256`
- `accepted_redaction_proof_ids`
- `redacted_payload_summary_sha256`
- `receipt_output_path_redacted`
- `rollback_plan_id`
- `public_claim_and_artifact_decision`

The current dry-run records zero command fields, invokes zero commands, and
persists zero receipts.

## Denial Fixtures

The gate denies six command shapes:

- schema-only no command
- command without an accepted proof
- command without operator identity or single-surface scope
- command with plaintext payload
- command without a redacted output path
- public artifact command attempt

All fixtures keep `command_accepted = false`, `command_invocation_performed =
false`, `receipt_persisted = false`, and `activation_allowed = false`.

## Safety Boundary

The gate must not:

- write memory, skills, plugin registries, or capability registries
- invoke providers or models
- send channel messages
- mutate runtime stores or Gateway queues
- execute a receipt command
- write approval packets, operator-scope records, payload reviews, redaction
  proofs, acceptance matrices, receipt commands, receipt files, evidence files,
  or release artifacts
- persist plaintext payloads
- inspect raw payload plaintext
- run live secret scans
- persist pre-activation soak evidence
- restart launchd
- execute rollback
- read credentials or secret files

The next safe step is a redaction acceptance receipt invocation dry-run gate,
still without command execution, receipt persistence, or live mutation.
