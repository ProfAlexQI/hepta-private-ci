# Hepta Live Mutation Pre-Activation Soak Evidence Persistence Payload Redaction Acceptance Receipt Invocation Dry-Run Gate

Date: 2026-05-25

This gate sits after the payload redaction acceptance receipt command contract
gate. It models future receipt command invocation requests while keeping the
command disabled by default.

The gate does not invoke a command, execute a receipt command, persist a
receipt, write a file, inspect payload plaintext, run a live secret scan, send a
channel message, invoke a provider or model, or enable live mutation.

## Contract

The gate consumes:

- `scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-command-contract-gate.sh`
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

It requires the source receipt command contract gate to be `ready`, but keeps
these values false or zero:

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

## Invocation Fixtures

The gate models five dry-run invocation fixtures:

- `redacted-command-shape`
- `accepted-proof-but-command-disabled`
- `persistence-disabled-invocation-attempt`
- `plaintext-payload-invocation-attempt`
- `public-artifact-invocation-attempt`

Each fixture may set `command_invocation_requested = true`, but all fixtures keep:

- `command_invocation_performed = false`
- `command_execution_performed = false`
- `receipt_persistence_execution_performed = false`
- `filesystem_write_performed = false`
- `receipt_persisted = false`
- `activation_allowed = false`

## Invocation Status

- Required invocation fixture count: `5`
- Blocked invocation fixture count: `5`
- Allowed invocation fixture count: `0`
- Command invocation attempt count: `5`
- Command invocation performed count: `0`
- Command execution performed count: `0`
- Receipt persistence execution performed count: `0`
- Filesystem write performed count: `0`
- Workspace write performed count: `0`
- Receipt persisted count: `0`
- Redacted output path fixture count: `4`
- Redacted payload summary hash bound fixture count: `5`
- Accepted redaction proof bound fixture count: `4`
- Operator scope bound fixture count: `4`
- Public claim attempt count: `1`
- Release artifact write attempt count: `1`
- Plaintext payload attempt count: `1`
- Invocation dry-run ready: `true`
- Activation allowed: `false`
- Live mutation execution ready: `false`

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

The next safe step is a redaction acceptance receipt no-write sink contract,
still without command execution, receipt persistence, filesystem writes, or live
mutation.
