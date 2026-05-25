# Hepta Live Mutation Pre-Activation Soak Evidence Persistence No-Secret Payload Review Gate

Date: 2026-05-25

This gate sits after the operator-scope binding gate. It defines the dry-run
shape for reviewing a future single-surface live-mutation payload before any
payload, approval, receipt, or soak evidence can be persisted.

The gate is still a no-write denial contract. It does not inspect real
credentials, record a payload review, persist a payload, send a channel
message, invoke a provider or model, or enable live mutation.

## Contract

The gate consumes:

- `scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-operator-scope-binding-gate.sh`
- the source operator-scope report hash
- the source approval-packet report hash
- the source approval receipt payload hash
- the source pre-activation soak report hash
- the source persistence-denial report hash
- the minimum 24-sample pre-activation soak requirement

It requires the source operator-scope gate to be `ready`, but keeps these values
false:

- `no_secret_payload_review_recorded`
- `no_secret_payload_review_id_recorded`
- `payload_manifest_recorded`
- `payload_hash_recorded`
- `payload_plaintext_recorded`
- `live_payload_review_performed`
- `approval_packet_recorded`
- `approval_packet_persisted`
- `operator_scope_binding_recorded`
- `operator_scope_binding_persisted`
- `pre_activation_soak_evidence_persistence_allowed`
- `receipt_persistence_enabled`
- `receipt_persisted`
- `activation_allowed`
- `live_mutation_execution_ready`

## Review Shape

The gate models fourteen fields required before a future payload review can be
accepted:

- `no_secret_payload_review_id`
- `reviewer_identity_hash`
- `reviewed_payload_kind`
- `single_surface_activation_scope`
- `reviewed_payload_sha256`
- `reviewed_payload_redacted_summary_sha256`
- `secret_scanner_policy_id`
- `external_send_policy_id`
- `path_redaction_policy_id`
- `credential_read_denial_policy_id`
- `source_operator_scope_report_sha256`
- `source_approval_packet_report_sha256`
- `rollback_plan_id`
- `review_captured_at_unix`

The current dry-run records zero review fields and approves zero payloads.

## Payload Kinds

The review model covers one payload kind for each allowed single-surface live
mutation scope:

- `memory_store_mutation_request`
- `capability_registry_update`
- `plugin_registry_update`
- `coding_agent_spawn_request`
- `search_provider_live_query`
- `skill_workshop_patch`
- `provider_model_prompt`
- `channel_delivery_payload`
- `runtime_store_patch`
- `gateway_event_payload`

Each future payload must bind to exactly one surface and must have a redacted
summary hash before it can be considered for persistence.

## Denial Fixtures

The gate denies six payload shapes:

- raw credential marker
- unredacted home or workspace path
- channel delivery recipient payload
- public artifact output path
- multi-surface payload
- provider prompt with hidden context export

All fixtures keep `review_accepted = false`, `persistence_allowed = false`, and
`activation_allowed = false`.

## Safety Boundary

The gate must not:

- write memory, skills, plugin registries, or capability registries
- invoke providers or models
- send channel messages
- mutate runtime stores or Gateway queues
- write approval packets, operator-scope records, payload reviews, receipt
  files, evidence files, or release artifacts
- persist plaintext payloads
- persist pre-activation soak evidence
- restart launchd
- execute rollback
- read credentials or secret files

The next safe step is a payload redaction proof dry-run gate, still without
persistence or live mutation.
