# Hepta Live Mutation Pre-Activation Soak Evidence Persistence Denial Gate

Date: 2026-05-25

This gate sits after the pre-activation soak evidence gate. It proves that the
24-sample soak evidence candidate cannot be persisted or used as activation
evidence until a full approval packet exists.

## Contract

The gate consumes:

- `scripts/hepta-live-mutation-pre-activation-soak-evidence-gate.sh`
- the source approval receipt payload hash
- the minimum 24-sample pre-activation soak requirement

It requires the source gate to be `ready`, but keeps these values false:

- `pre_activation_soak_evidence_persistence_allowed`
- `fresh_soak_evidence_recorded`
- `fresh_soak_evidence_bound`
- `long_soak_evidence_persisted`
- `receipt_persistence_enabled`
- `receipt_persisted`
- `operator_approval_recorded`
- `activation_allowed`
- `live_mutation_execution_ready`

## Persistence Boundary

The gate models ten fields that must exist before a soak evidence receipt can be
persisted:

- `operator_approval_id`
- `single_surface_activation_scope`
- `source_receipt_payload_sha256`
- `pre_activation_soak_report_sha256`
- `fresh_soak_evidence_record_id`
- `fresh_soak_evidence_captured_at_unix`
- `fresh_soak_evidence_sample_count`
- `fresh_soak_evidence_report_sha256`
- `installed_binary_sha256_after_approval`
- `rollback_plan_id`

In the default path, zero of those fields are recorded. That is intentional:
this is a denial gate, not a persistence implementation.

## Denial Fixtures

The gate denies four persistence shapes:

- missing operator approval
- missing fresh 24-sample soak evidence record
- missing or mismatched source receipt payload hash
- filesystem persistence request before the approval packet exists

All fixtures keep `persistence_allowed = false`.

## Safety Boundary

The gate must not:

- write memory, skills, plugin registries, or capability registries
- invoke providers or models
- send channel messages
- mutate runtime stores or Gateway queues
- write receipt files or release artifacts
- persist pre-activation soak evidence
- restart launchd
- execute rollback
- read credentials

The follow-on gate
`scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-approval-packet-gate.sh`
defines the bounded approval packet shape for a future persistence record, still
without executing filesystem writes or enabling live mutation.
