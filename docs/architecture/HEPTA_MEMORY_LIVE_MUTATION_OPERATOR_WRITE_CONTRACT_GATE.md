# Hepta Memory Live Mutation Operator Write Contract Gate

Date: 2026-05-26

This gate defines the first memory-specific write request contract for live
mutation. It does not enable memory writes. It records the shape an operator
approved request must satisfy before any later gate can consider a memory store
mutation.

## Source Gates

The contract consumes two already bounded gates:

- `scripts/hepta-memory-intelligence-closure.sh`
- `scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-matrix-gate.sh`

The required source state is:

- memory capability surfaces are fully represented: `14 / 14`
- live memory mutation remains disabled: `live_mutation_enabled_count = 0`
- `hepta-core` has no direct memory/intelligence dependency
- no payload redaction proof is accepted yet
- all upstream side-effect fields are false

## Request Shape

A future memory write request must include these fields before it can be
accepted:

- `memory_write_request_id`
- `operator_approval_id`
- `operator_identity_hash`
- `single_surface_activation_scope`
- `memory_namespace`
- `memory_write_operation`
- `memory_retention_class`
- `record_intent`
- `raw_payload_sha256`
- `redacted_payload_summary_sha256`
- `accepted_redaction_proof_id`
- `source_memory_intelligence_report_sha256`
- `source_payload_redaction_acceptance_matrix_report_sha256`
- `fresh_pre_activation_soak_evidence_id`
- `rollback_plan_id`
- `post_write_validation_plan_id`
- `no_public_claim_no_external_send_decision`

Allowed operation names are limited to:

- `append_daily_memory_note`
- `append_project_memory_note`
- `promote_long_term_memory_summary`
- `redact_or_supersede_memory_record`

## Hard Denials

The contract rejects these request families:

- missing operator approval
- missing operator identity hash
- missing single-surface activation scope
- missing accepted redaction proof
- raw secret or credential persistence
- plaintext payload recording
- multi-surface mutation
- unbounded bulk import
- destructive delete without supersession
- cross-surface registry mutation
- provider prompt replay
- channel delivery or external send
- public claim or release artifact write

## Side-Effect Boundary

The gate is a shape and denial matrix only. It must not:

- mutate the memory store
- mutate capability, plugin, or runtime registries
- inspect or persist raw payload plaintext
- read credentials or secret files
- invoke a provider or model
- send to a channel
- write a release or public artifact
- restart services

The output intentionally reports:

- `memory_write_request_recorded = false`
- `memory_write_request_accepted = false`
- `memory_store_mutation_allowed = false`
- `memory_write_execution_ready = false`
- `live_mutation_execution_ready = false`

This is the correct state until an explicit operator approval record, accepted
redaction proof, fresh soak evidence, rollback plan, and post-write validation
plan all exist.
