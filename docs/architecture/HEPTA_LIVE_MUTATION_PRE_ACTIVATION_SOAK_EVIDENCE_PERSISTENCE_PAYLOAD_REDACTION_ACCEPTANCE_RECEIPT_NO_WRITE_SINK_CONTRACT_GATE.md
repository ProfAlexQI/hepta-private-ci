# Hepta Live Mutation Pre-Activation Soak Evidence Persistence Payload Redaction Acceptance Receipt No-Write Sink Contract Gate

Date: 2026-05-25

This gate sits after the payload redaction acceptance receipt invocation dry-run
gate. It models the future receipt sink boundary while keeping the sink in
no-write mode.

The gate does not invoke a command, execute a receipt command, persist a
receipt, write a file, inspect payload plaintext, run a live secret scan, send a
channel message, invoke a provider or model, or enable live mutation.

## Contract

The gate consumes:

- `scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-invocation-dry-run-gate.sh`
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

It requires the source invocation dry-run gate to be `ready`, but keeps these
values false or zero:

- `payload_redaction_acceptance_receipt_command_recorded`
- `payload_redaction_acceptance_receipt_command_enabled_by_default`
- `payload_redaction_acceptance_receipt_command_invocation_performed_count`
- `payload_redaction_acceptance_receipt_command_execution_performed_count`
- `payload_redaction_acceptance_receipt_recorded`
- `payload_redaction_acceptance_receipt_persisted`
- `payload_redaction_acceptance_matrix_recorded`
- `payload_redaction_acceptance_matrix_persisted`
- `payload_redaction_proof_recorded`
- `payload_redaction_proof_accepted`
- `command_invocation_performed_count`
- `command_execution_performed_count`
- `receipt_persistence_execution_performed_count`
- `filesystem_write_performed_count`
- `workspace_write_performed_count`
- `receipt_persisted_count`
- `raw_payload_plaintext_recorded`
- `raw_payload_plaintext_persisted`
- `live_secret_scan_performed`
- `receipt_persistence_enabled`
- `receipt_persisted`
- `activation_allowed`
- `live_mutation_execution_ready`

## Sink Surfaces

The no-write sink contract defines eight side-effect-free surfaces:

- `redacted_invocation_shape_acceptance`
- `redacted_payload_summary_hash_binding`
- `redacted_output_path_binding`
- `accepted_redaction_proof_binding`
- `operator_scope_binding`
- `plaintext_payload_rejection`
- `public_claim_artifact_rejection`
- `filesystem_write_rejection`

## Sink Status

- Required sink surface count: `8`
- Ready sink surface count: `8`
- Side-effect-free sink surface count: `8`
- Source invocation fixture count: `5`
- No-write sink fixture count: `5`
- No-write sink accepted redacted fixture count: `3`
- No-write sink rejected plaintext fixture count: `1`
- No-write sink rejected public artifact fixture count: `1`
- No-write sink write request fixture count: `5`
- No-write sink rejected write fixture count: `5`
- No-write sink allowed write fixture count: `0`
- No-write sink redacted output path fixture count: `4`
- No-write sink payload summary hash bound fixture count: `5`
- No-write sink accepted redaction proof bound fixture count: `4`
- No-write sink operator scope bound fixture count: `4`
- No-write sink accepts redacted payload summary hash: `true`
- No-write sink accepts redacted output path: `true`
- No-write sink requires accepted redaction proof: `true`
- No-write sink requires operator scope: `true`
- No-write sink rejects plaintext payload: `true`
- No-write sink rejects public claim artifact: `true`
- No-write sink rejects filesystem write: `true`
- No-write sink write path enabled by default: `false`
- Command invocation attempt count: `5`
- Command invocation performed count: `0`
- Command execution performed count: `0`
- Receipt persistence execution performed count: `0`
- Filesystem write performed count: `0`
- Workspace write performed count: `0`
- Receipt persisted count: `0`
- No-write sink contract ready: `true`
- Activation allowed: `false`
- Live mutation execution ready: `false`

## Sink Fixtures

The gate carries forward the five invocation fixtures and classifies them at
the sink boundary:

- `redacted-command-shape`: accepted for no-write validation only
- `accepted-proof-but-command-disabled`: accepted for no-write validation only
- `persistence-disabled-invocation-attempt`: accepted for no-write validation only
- `plaintext-payload-invocation-attempt`: rejected
- `public-artifact-invocation-attempt`: rejected

Every fixture keeps:

- `command_invocation_performed = false`
- `command_execution_performed = false`
- `receipt_persistence_execution_performed = false`
- `filesystem_write_requested = true`
- `filesystem_write_performed = false`
- `receipt_persisted = false`
- `activation_allowed = false`

## Safety Boundary

The gate must not:

- write memory, skills, plugin registries, or capability registries
- invoke providers or models
- send channel messages
- mutate runtime stores or Gateway queues
- invoke or execute a receipt command
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

The next safe step is a redaction acceptance receipt write-enable fixture gate,
still without command execution, receipt persistence, filesystem writes, or live
mutation.
