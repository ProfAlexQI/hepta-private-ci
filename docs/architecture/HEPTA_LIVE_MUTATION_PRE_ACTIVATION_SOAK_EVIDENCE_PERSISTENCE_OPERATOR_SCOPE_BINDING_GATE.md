# Hepta Live Mutation Pre-Activation Soak Evidence Persistence Operator Scope Binding Gate

Date: 2026-05-25

This gate sits after the pre-activation soak evidence persistence approval
packet gate. It defines how a future operator approval must bind a redacted
operator identity to exactly one live-mutation surface before any persistence or
activation can be considered.

The gate is still a dry-run denial contract. It does not record an approval,
persist an approval packet, persist soak evidence, write a receipt, or enable
live mutation.

## Contract

The gate consumes:

- `scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-approval-packet-gate.sh`
- the source approval packet report hash
- the source approval receipt payload hash
- the source pre-activation soak report hash
- the source persistence-denial report hash
- the minimum 24-sample pre-activation soak requirement

It requires the source approval-packet shape gate to be `ready`, but keeps these
values false:

- `operator_identity_binding_recorded`
- `operator_identity_hash_recorded`
- `operator_approval_id_recorded`
- `single_surface_activation_scope_recorded`
- `single_surface_scope_validated`
- `approval_packet_recorded`
- `approval_packet_persisted`
- `approval_packet_accepted`
- `pre_activation_soak_evidence_persistence_allowed`
- `receipt_persistence_enabled`
- `receipt_persisted`
- `activation_allowed`
- `live_mutation_execution_ready`

## Operator Scope Shape

The gate models twelve fields required before an operator/scope binding can be
accepted:

- `operator_approval_id`
- `operator_identity_hash`
- `operator_approval_signature_hash`
- `operator_approval_captured_at_unix`
- `single_surface_activation_scope`
- `single_surface_scope_reason`
- `source_approval_packet_report_sha256`
- `source_receipt_payload_sha256`
- `source_pre_activation_soak_report_sha256`
- `fresh_soak_evidence_record_id`
- `rollback_plan_id`
- `no_secret_payload_review_id`

The scope must select exactly one allowed live-mutation surface. The current
dry-run records zero fields and accepts zero scopes.

## Scope Allowlist

The allowed single-surface activation scopes are:

- `memory_store_mutation`
- `capability_registry_mutation`
- `plugin_registry_mutation`
- `coding_agent_spawn`
- `search_provider_live_query`
- `skill_workshop_write`
- `provider_model_invocation`
- `channel_delivery`
- `runtime_store_mutation`
- `gateway_event_enqueue`

Selecting multiple surfaces, an unsupported surface, or a public
claim/release-artifact path keeps activation blocked.

## Denial Fixtures

The gate denies five operator/scope shapes:

- missing operator identity
- multiple activation surfaces
- unsupported activation surface
- operator scope without fresh soak evidence or rollback
- public-claim or release-artifact attempt with an operator scope

All fixtures keep `binding_accepted = false` and `persistence_allowed = false`.

## Safety Boundary

The gate must not:

- write memory, skills, plugin registries, or capability registries
- invoke providers or models
- send channel messages
- mutate runtime stores or Gateway queues
- write approval packets, receipt files, evidence files, or release artifacts
- persist pre-activation soak evidence
- restart launchd
- execute rollback
- read credentials

The next safe step is a no-secret payload review dry-run gate, still without
persistence or live mutation.
