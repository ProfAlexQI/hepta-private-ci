# Hepta Live Mutation Pre-Activation Soak Evidence Persistence Payload Redaction Acceptance Matrix Gate

Date: 2026-05-25

This gate sits after the payload redaction proof gate. It defines the dry-run
acceptance matrix for a future single-surface redaction proof before that proof
can become actionable.

The gate does not record a proof, accept a proof, inspect payload plaintext, run
a live secret scan, persist an acceptance matrix, write a receipt, invoke a
provider or model, send a channel message, or enable live mutation.

## Contract

The gate consumes:

- `scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-proof-gate.sh`
- the source payload redaction proof report hash
- the source no-secret payload review report hash
- the source operator-scope report hash
- the source approval-packet report hash
- the source approval receipt payload hash
- the source pre-activation soak report hash
- the source persistence-denial report hash
- the minimum 24-sample pre-activation soak requirement

It requires the source payload redaction proof gate to be `ready`, but keeps
these values false:

- `payload_redaction_acceptance_matrix_recorded`
- `payload_redaction_acceptance_matrix_id_recorded`
- `payload_redaction_proof_recorded`
- `payload_redaction_proof_accepted`
- `payload_review_persisted`
- `payload_redaction_proof_persisted`
- `payload_redaction_acceptance_matrix_persisted`
- `raw_payload_plaintext_recorded`
- `raw_payload_plaintext_persisted`
- `live_secret_scan_performed`
- `receipt_persistence_enabled`
- `receipt_persisted`
- `activation_allowed`
- `live_mutation_execution_ready`

## Acceptance Matrix

The gate models eight checks required before a future redaction proof can be
accepted:

- `source_no_secret_payload_review_hash_bound`
- `source_operator_scope_hash_bound`
- `single_surface_scope_bound`
- `raw_payload_sha256_present`
- `redacted_payload_summary_sha256_present`
- `redacted_summary_differs_from_raw_payload`
- `redaction_policies_recorded`
- `plaintext_payload_absent_from_record`

The current dry-run satisfies zero acceptance checks and accepts zero redaction
proofs.

## Denial Fixtures

The gate denies six acceptance shapes:

- schema-only no proof
- raw hash without redacted summary
- redacted summary matching raw payload
- policyless redaction proof
- source review unbound
- plaintext retention attempt

All fixtures keep `proof_accepted = false`, `persistence_allowed = false`, and
`activation_allowed = false`.

## Safety Boundary

The gate must not:

- write memory, skills, plugin registries, or capability registries
- invoke providers or models
- send channel messages
- mutate runtime stores or Gateway queues
- write approval packets, operator-scope records, payload reviews, redaction
  proofs, acceptance matrices, receipt files, evidence files, or release
  artifacts
- persist plaintext payloads
- inspect raw payload plaintext
- run live secret scans
- persist pre-activation soak evidence
- restart launchd
- execute rollback
- read credentials or secret files

The next safe step is a redaction proof acceptance receipt command contract
dry-run gate, still without persistence or live mutation.
