# Hepta Memory Live Mutation Operator Write Approval Packet Gate

Date: 2026-05-26

This gate sits after the memory live mutation operator write contract. It
defines the approval packet shape for a future operator-approved memory write
request, but still refuses to record, persist, accept, or execute that request.

## Source Gate

The gate consumes:

- `scripts/hepta-memory-live-mutation-operator-write-contract-gate.sh`

The required source state is:

- the memory write contract is `ready`
- no memory write request is recorded or accepted
- no operator approval, identity hash, single-surface scope, or accepted
  redaction proof is recorded
- memory store mutation and live mutation remain disabled
- raw payload plaintext is not recorded or persisted
- all source side-effect fields are false

## Approval Packet Shape

A future memory write approval packet must include these fields before it can be
accepted by a later gate:

- `approval_packet_id`
- `memory_write_request_id`
- `operator_approval_id`
- `operator_identity_hash`
- `operator_approval_signature_hash`
- `operator_approval_captured_at_unix`
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
- `source_memory_write_contract_report_sha256`
- `fresh_pre_activation_soak_evidence_id`
- `rollback_plan_id`
- `post_write_validation_plan_id`
- `no_public_claim_no_external_send_decision`

The default path records zero of these fields. This is deliberate: the gate is
only an approval-packet schema and denial matrix.

## Denial Fixtures

The gate denies these fixture families:

- empty approval packet
- operator approval without identity and signature hash
- disallowed memory write operation
- missing accepted redaction proof
- missing fresh soak, rollback, or post-write validation evidence
- raw secret or plaintext payload attempt
- external send, public claim, or release artifact write attempt
- direct memory store mutation at the approval-packet layer

Every fixture keeps packet acceptance, request acceptance, memory store mutation,
and activation false.

## Side-Effect Boundary

The gate must not:

- mutate the memory store
- record or persist memory write requests
- record or persist approval packets
- inspect or persist raw payload plaintext
- mutate capability, plugin, skill, runtime, or Gateway registries
- invoke providers or models
- send to any channel
- write release or public artifacts
- read credentials or secret files
- restart services or execute rollback

The output intentionally reports:

- `memory_write_approval_packet_recorded = false`
- `memory_write_approval_packet_persisted = false`
- `memory_write_approval_packet_accepted = false`
- `memory_write_request_accepted = false`
- `memory_store_mutation_allowed = false`
- `memory_write_execution_ready = false`
- `live_mutation_execution_ready = false`
