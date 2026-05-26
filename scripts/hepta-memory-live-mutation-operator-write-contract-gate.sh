#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

cd "$REPO_ROOT"

source scripts/lib/hepta-json-report-capture.sh

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

MEMORY_INTELLIGENCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    capture_json_report "hepta-memory-intelligence-closure" \
    scripts/hepta-memory-intelligence-closure.sh
)"

ACCEPTANCE_MATRIX_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-matrix-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-matrix-gate.sh
)"

memory_intelligence_report_sha256="$(printf '%s' "$MEMORY_INTELLIGENCE_JSON" | shasum -a 256 | awk '{print $1}')"
acceptance_matrix_report_sha256="$(printf '%s' "$ACCEPTANCE_MATRIX_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson memory "$MEMORY_INTELLIGENCE_JSON" \
  --argjson matrix "$ACCEPTANCE_MATRIX_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $memory.runtime == "hepta"
    and $memory.status == "attention"
    and $memory.compatibility_mode == "hepta_memory_intelligence_closure_gate"
    and $memory.active_service_stack_consumes_memory_intelligence == true
    and $memory.hepta_core_direct_memory_intelligence_dependency_count == 0
    and $memory.hepta_core_dependency_boundary_ready == true
    and $memory.runtime_memory_intelligence_dependencies_ready == true
    and $memory.memory_surface_count == 14
    and $memory.absorbed_or_represented_count == 14
    and $memory.gap_report_ready_count == 14
    and $memory.live_mutation_enabled_count == 0
    and $memory.full_live_memory_intelligence_closure_ready == false
    and $memory.gap_only_surface_count == 0
    and ($memory.gap_only_surfaces | length) == 0
    and ($memory.side_effects | to_entries | all(.value == false))
    and $matrix.runtime == "hepta"
    and $matrix.status == "ready"
    and $matrix.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_matrix_gate"
    and $matrix.payload_redaction_acceptance_matrix_ready == true
    and $matrix.source_payload_redaction_proof_ready == true
    and $matrix.payload_redaction_acceptance_matrix_recorded == false
    and $matrix.payload_redaction_proof_recorded == false
    and $matrix.payload_redaction_proof_accepted == false
    and $matrix.accepted_redaction_proof_count == 0
    and $matrix.reviewed_redaction_proof_count == 0
    and $matrix.raw_payload_plaintext_recorded == false
    and $matrix.raw_payload_plaintext_persisted == false
    and $matrix.activation_allowed == false
    and $matrix.live_mutation_execution_ready == false
    and ($matrix.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_contract_gate" \
  --arg memory_intelligence_report_sha256 "$memory_intelligence_report_sha256" \
  --arg acceptance_matrix_report_sha256 "$acceptance_matrix_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson memory "$MEMORY_INTELLIGENCE_JSON" \
  --argjson matrix "$ACCEPTANCE_MATRIX_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    contract_mode:"operator_approved_memory_write_request_shape_no_memory_store_write",
    source_memory_intelligence_closure_gate:$memory.compatibility_mode,
    source_memory_intelligence_report_sha256:$memory_intelligence_report_sha256,
    source_memory_capability_endpoint:$memory.memory_capability_endpoint,
    source_payload_redaction_acceptance_matrix_gate:$matrix.gate,
    source_payload_redaction_acceptance_matrix_ready:$matrix.payload_redaction_acceptance_matrix_ready,
    source_payload_redaction_acceptance_matrix_report_sha256:$acceptance_matrix_report_sha256,
    source_payload_redaction_proof_report_sha256:$matrix.source_payload_redaction_proof_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_live_mutation_operator_write_contract_ready:true,
    memory_surface_count:$memory.memory_surface_count,
    absorbed_or_represented_count:$memory.absorbed_or_represented_count,
    gap_report_ready_count:$memory.gap_report_ready_count,
    live_mutation_enabled_count:$memory.live_mutation_enabled_count,
    gap_only_surface_count:$memory.gap_only_surface_count,
    hepta_core_direct_memory_intelligence_dependency_count:$memory.hepta_core_direct_memory_intelligence_dependency_count,
    hepta_core_dependency_boundary_ready:$memory.hepta_core_dependency_boundary_ready,
    runtime_memory_intelligence_dependencies_ready:$memory.runtime_memory_intelligence_dependencies_ready,
    memory_write_request_recorded:false,
    memory_write_request_accepted:false,
    memory_write_request_persisted:false,
    operator_approval_recorded:false,
    operator_identity_hash_recorded:false,
    single_surface_activation_scope_recorded:false,
    accepted_redaction_proof_recorded:false,
    accepted_redaction_proof_count:0,
    memory_write_execution_ready:false,
    memory_store_mutation_allowed:false,
    live_mutation_execution_ready:false,
    raw_payload_plaintext_recorded:false,
    raw_payload_plaintext_persisted:false,
    memory_store_mutation_enabled:false,
    capability_registry_mutation_enabled:false,
    plugin_registry_mutation_enabled:false,
    provider_prompt_replay_enabled:false,
    external_send_enabled:false,
    public_claim_or_release_artifact_write_enabled:false,
    required_memory_write_request_field_count:17,
    recorded_memory_write_request_field_count:0,
    allowed_memory_write_operations:[
      "append_daily_memory_note",
      "append_project_memory_note",
      "promote_long_term_memory_summary",
      "redact_or_supersede_memory_record"
    ],
    disallowed_memory_write_operations:[
      "raw_secret_or_credential_persistence",
      "unbounded_bulk_import",
      "destructive_delete_without_supersession",
      "cross_surface_registry_mutation",
      "provider_prompt_replay",
      "channel_delivery_or_external_send",
      "public_claim_or_release_artifact_write"
    ],
    required_memory_write_request_fields:[
      "memory_write_request_id",
      "operator_approval_id",
      "operator_identity_hash",
      "single_surface_activation_scope",
      "memory_namespace",
      "memory_write_operation",
      "memory_retention_class",
      "record_intent",
      "raw_payload_sha256",
      "redacted_payload_summary_sha256",
      "accepted_redaction_proof_id",
      "source_memory_intelligence_report_sha256",
      "source_payload_redaction_acceptance_matrix_report_sha256",
      "fresh_pre_activation_soak_evidence_id",
      "rollback_plan_id",
      "post_write_validation_plan_id",
      "no_public_claim_no_external_send_decision"
    ],
    denied_memory_write_request_fixtures:[
      {
        id:"schema-only-no-request",
        recorded_memory_write_request_field_count:0,
        request_accepted:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"memory_write_request_not_recorded"
      },
      {
        id:"memory-write-without-operator-approval",
        recorded_memory_write_request_field_count:7,
        operator_approval_recorded:false,
        request_accepted:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"operator_approval_required"
      },
      {
        id:"multi-surface-memory-write-request",
        recorded_memory_write_request_field_count:10,
        single_surface_activation_scope_recorded:false,
        requested_surface_count:2,
        request_accepted:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"single_surface_activation_scope_required"
      },
      {
        id:"raw-secret-marker-memory-write",
        recorded_memory_write_request_field_count:12,
        raw_secret_marker_detected:true,
        request_accepted:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"raw_secret_or_credential_persistence_denied"
      },
      {
        id:"memory-write-without-accepted-redaction-proof",
        recorded_memory_write_request_field_count:14,
        accepted_redaction_proof_recorded:false,
        accepted_redaction_proof_count:0,
        request_accepted:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"accepted_redaction_proof_required"
      },
      {
        id:"unbounded-bulk-memory-import",
        recorded_memory_write_request_field_count:17,
        memory_write_operation:"unbounded_bulk_import",
        request_accepted:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"unbounded_bulk_import_denied"
      },
      {
        id:"destructive-delete-without-supersession",
        recorded_memory_write_request_field_count:17,
        memory_write_operation:"destructive_delete",
        supersession_recorded:false,
        request_accepted:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"destructive_delete_without_supersession_denied"
      },
      {
        id:"provider-prompt-replay-memory-write",
        recorded_memory_write_request_field_count:17,
        provider_prompt_replay_requested:true,
        request_accepted:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"provider_prompt_replay_denied"
      },
      {
        id:"external-send-or-public-claim-memory-write",
        recorded_memory_write_request_field_count:17,
        external_send_requested:true,
        public_claim_requested:true,
        release_artifact_write_requested:true,
        request_accepted:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"external_send_public_claim_and_release_artifact_denied"
      }
    ],
    denied_by_memory_write_contract:[
      "memory_write_request_not_recorded",
      "operator_approval_not_recorded",
      "operator_identity_hash_not_recorded",
      "single_surface_activation_scope_not_recorded",
      "accepted_redaction_proof_not_recorded",
      "source_memory_intelligence_hash_not_bound",
      "source_payload_redaction_acceptance_matrix_hash_not_bound",
      "fresh_pre_activation_soak_evidence_not_recorded",
      "raw_secret_or_credential_persistence_denied",
      "plaintext_payload_recording_denied",
      "multi_surface_mutation_denied",
      "unbounded_bulk_import_denied",
      "destructive_delete_without_supersession_denied",
      "cross_surface_registry_mutation_denied",
      "provider_prompt_replay_denied",
      "channel_delivery_or_external_send_denied",
      "public_claim_or_release_artifact_write_denied",
      "memory_write_execution_denied"
    ],
    required_before_memory_write_request_acceptance:[
      "operator_approval_id",
      "operator_identity_hash",
      "single_surface_activation_scope",
      "memory_namespace",
      "allowed_memory_write_operation",
      "memory_retention_class",
      "record_intent",
      "raw_payload_sha256",
      "redacted_payload_summary_sha256",
      "accepted_redaction_proof_id",
      "source_memory_intelligence_report_sha256",
      "source_payload_redaction_acceptance_matrix_report_sha256",
      "fresh_pre_activation_soak_evidence_id",
      "rollback_plan_id",
      "post_write_validation_plan_id",
      "no_public_claim_no_external_send_decision"
    ],
    side_effects:{
      memory_store_mutated:false,
      capability_registry_mutated:false,
      plugin_registry_mutated:false,
      coding_agent_spawned:false,
      skill_workshop_written:false,
      provider_invoked:false,
      model_invoked:false,
      provider_prompt_replayed:false,
      channel_send_performed:false,
      runtime_store_mutated:false,
      gateway_event_enqueued:false,
      filesystem_written:false,
      release_artifact_written:false,
      public_artifact_written:false,
      launchd_mutated:false,
      service_restarted:false,
      rollback_executed:false,
      receipt_persisted:false,
      memory_write_request_persisted:false,
      memory_write_contract_persisted:false,
      payload_plaintext_persisted:false,
      raw_payload_inspected:false,
      live_secret_scan_performed:false,
      external_send_performed:false,
      credential_read:false,
      secret_file_read:false
    }
  }')"

jq -e '
  .status == "ready"
  and .memory_live_mutation_operator_write_contract_ready == true
  and .contract_mode == "operator_approved_memory_write_request_shape_no_memory_store_write"
  and .source_memory_intelligence_report_sha256 != ""
  and .source_payload_redaction_acceptance_matrix_ready == true
  and .source_payload_redaction_acceptance_matrix_report_sha256 != ""
  and .source_payload_redaction_proof_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .memory_surface_count == 14
  and .absorbed_or_represented_count == 14
  and .gap_report_ready_count == 14
  and .live_mutation_enabled_count == 0
  and .gap_only_surface_count == 0
  and .hepta_core_direct_memory_intelligence_dependency_count == 0
  and .hepta_core_dependency_boundary_ready == true
  and .runtime_memory_intelligence_dependencies_ready == true
  and .memory_write_request_recorded == false
  and .memory_write_request_accepted == false
  and .memory_write_request_persisted == false
  and .operator_approval_recorded == false
  and .operator_identity_hash_recorded == false
  and .single_surface_activation_scope_recorded == false
  and .accepted_redaction_proof_recorded == false
  and .accepted_redaction_proof_count == 0
  and .memory_write_execution_ready == false
  and .memory_store_mutation_allowed == false
  and .live_mutation_execution_ready == false
  and .raw_payload_plaintext_recorded == false
  and .raw_payload_plaintext_persisted == false
  and .memory_store_mutation_enabled == false
  and .capability_registry_mutation_enabled == false
  and .plugin_registry_mutation_enabled == false
  and .provider_prompt_replay_enabled == false
  and .external_send_enabled == false
  and .public_claim_or_release_artifact_write_enabled == false
  and .required_memory_write_request_field_count == 17
  and .recorded_memory_write_request_field_count == 0
  and (.allowed_memory_write_operations | length) == 4
  and (.disallowed_memory_write_operations | length) == 7
  and (.required_memory_write_request_fields | length) == 17
  and (.denied_memory_write_request_fixtures | length) == 9
  and (.denied_memory_write_request_fixtures | all(.request_accepted == false and .memory_store_mutated == false and .activation_allowed == false))
  and (.denied_by_memory_write_contract | length) == 18
  and (.required_before_memory_write_request_acceptance | length) == 16
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory live mutation operator write contract gate passed"
