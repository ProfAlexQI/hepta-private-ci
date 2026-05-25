# Hepta Live Mutation Pre-Activation Soak Evidence Persistence Payload Redaction Proof Gate

Date: 2026-05-25

This gate sits after the no-secret payload review gate. It defines the dry-run
proof shape for showing that a future single-surface live-mutation payload has a
redacted summary and hash bindings before any payload review can become
actionable.

The gate does not inspect real payload plaintext, run a live secret scan, persist
a proof, write a receipt, send a channel message, invoke a provider or model, or
enable live mutation.

## Contract

The gate consumes:

- `scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-no-secret-payload-review-gate.sh`
- the source no-secret payload review report hash
- the source operator-scope report hash
- the source approval-packet report hash
- the source approval receipt payload hash
- the source pre-activation soak report hash
- the source persistence-denial report hash
- the minimum 24-sample pre-activation soak requirement

It requires the source no-secret payload review gate to be `ready`, but keeps
these values false:

- `payload_redaction_proof_recorded`
- `payload_redaction_proof_id_recorded`
- `redaction_policy_recorded`
- `redacted_summary_hash_recorded`
- `raw_payload_hash_recorded`
- `raw_payload_plaintext_recorded`
- `raw_payload_plaintext_persisted`
- `payload_review_persisted`
- `payload_redaction_proof_persisted`
- `receipt_persistence_enabled`
- `receipt_persisted`
- `activation_allowed`
- `live_mutation_execution_ready`

## Proof Shape

The gate models fourteen fields required before a future redaction proof can be
accepted:

- `payload_redaction_proof_id`
- `no_secret_payload_review_id`
- `reviewed_payload_kind`
- `single_surface_activation_scope`
- `raw_payload_sha256`
- `redacted_payload_summary_sha256`
- `redaction_policy_id`
- `secret_scanner_policy_id`
- `path_redaction_policy_id`
- `external_recipient_redaction_policy_id`
- `source_no_secret_payload_review_report_sha256`
- `source_operator_scope_report_sha256`
- `reviewer_identity_hash`
- `proof_captured_at_unix`

The current dry-run records zero proof fields and approves zero redaction
proofs. Plaintext payload is explicitly absent from the record.

## Denial Fixtures

The gate denies six redaction proof shapes:

- missing redacted summary hash
- redacted summary equal to raw payload
- raw secret marker after redaction
- unredacted path after redaction
- channel recipient after redaction
- public artifact path after redaction

All fixtures keep `proof_accepted = false`, `persistence_allowed = false`, and
`activation_allowed = false`.

## Safety Boundary

The gate must not:

- write memory, skills, plugin registries, or capability registries
- invoke providers or models
- send channel messages
- mutate runtime stores or Gateway queues
- write approval packets, operator-scope records, payload reviews, redaction
  proofs, receipt files, evidence files, or release artifacts
- persist plaintext payloads
- inspect raw payload plaintext
- run live secret scans
- persist pre-activation soak evidence
- restart launchd
- execute rollback
- read credentials or secret files

The next safe step is a redaction proof acceptance-matrix dry-run gate, still
without persistence or live mutation.
