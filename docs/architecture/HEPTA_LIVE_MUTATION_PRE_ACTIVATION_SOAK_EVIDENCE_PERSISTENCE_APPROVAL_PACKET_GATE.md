# Hepta Live Mutation Pre-Activation Soak Evidence Persistence Approval Packet Gate

Date: 2026-05-25

This gate sits after the pre-activation soak evidence persistence-denial gate. It
defines the packet shape that a future operator-approved persistence record must
carry, while still refusing to persist the packet, persist soak evidence, or
enable live mutation.

## Contract

The gate consumes:

- `scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-denial-gate.sh`
- the source approval receipt payload hash
- the source pre-activation soak report hash
- the minimum 24-sample pre-activation soak requirement

It requires the source denial gate to be `ready`, but keeps these values false:

- `approval_packet_recorded`
- `approval_packet_persisted`
- `approval_packet_accepted`
- `pre_activation_soak_evidence_persistence_allowed`
- `receipt_persistence_enabled`
- `receipt_persisted`
- `operator_approval_recorded`
- `activation_allowed`
- `live_mutation_execution_ready`

## Approval Packet Shape

The gate models fourteen fields that must exist before a pre-activation soak
evidence receipt can be persisted:

- `approval_packet_id`
- `operator_approval_id`
- `operator_identity_hash`
- `single_surface_activation_scope`
- `source_receipt_payload_sha256`
- `source_pre_activation_soak_report_sha256`
- `fresh_soak_evidence_record_id`
- `fresh_soak_evidence_report_sha256`
- `fresh_soak_evidence_sample_count`
- `fresh_soak_evidence_captured_at_unix`
- `installed_binary_sha256_after_approval`
- `rollback_plan_id`
- `no_secret_payload_review_id`
- `public_claim_and_artifact_decision`

In the default path, zero of those fields are recorded. This gate is a schema
and denial contract, not a persistence implementation.

## Denial Fixtures

The gate denies four approval packet shapes:

- empty approval packet
- operator-approved packet without a fresh soak evidence record
- fresh-soak packet without a rollback plan
- public-claim or release-artifact write attempt

All fixtures keep `packet_accepted = false` and `persistence_allowed = false`.

## Safety Boundary

The gate must not:

- write memory, skills, plugin registries, or capability registries
- invoke providers or models
- send channel messages
- mutate runtime stores or Gateway queues
- write approval packets, receipt files, or release artifacts
- persist pre-activation soak evidence
- restart launchd
- execute rollback
- read credentials

The next safe step is an operator-approval identity and scope dry-run binding
gate, still without persistence or live mutation.
