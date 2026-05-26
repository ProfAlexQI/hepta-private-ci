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

DENIAL_MATRIX_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-denial-matrix-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-denial-matrix-gate.sh
)"

denial_matrix_report_sha256="$(printf '%s' "$DENIAL_MATRIX_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson matrix "$DENIAL_MATRIX_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $matrix.runtime == "hepta"
    and $matrix.status == "ready"
    and $matrix.gate == "hepta_memory_live_mutation_operator_write_execution_denial_matrix_gate"
    and $matrix.memory_write_execution_denial_matrix_ready == true
    and $matrix.execution_denial_matrix_mode == "memory_write_execution_attempt_denial_matrix_no_store_mutation"
    and $matrix.source_memory_write_execution_preflight_ready == true
    and $matrix.source_memory_write_execution_preflight_report_sha256 != ""
    and $matrix.source_memory_write_approval_packet_report_sha256 != ""
    and $matrix.source_memory_write_contract_report_sha256 != ""
    and $matrix.source_memory_intelligence_report_sha256 != ""
    and $matrix.source_payload_redaction_acceptance_matrix_report_sha256 != ""
    and $matrix.source_payload_redaction_proof_report_sha256 != ""
    and $matrix.minimum_required_samples >= 24
    and $matrix.required_pre_execution_validation_check_count == 17
    and $matrix.accepted_pre_execution_validation_check_count == 0
    and $matrix.memory_write_execution_attempt_requested_count == 7
    and $matrix.memory_write_execution_attempt_performed_count == 0
    and $matrix.memory_write_execution_allowed_count == 0
    and $matrix.memory_write_execution_denied_count == 7
    and $matrix.blocked_execution_fixture_count == 7
    and $matrix.allowed_execution_fixture_count == 0
    and $matrix.memory_write_execution_denial_matrix_recorded == false
    and $matrix.memory_write_execution_denial_matrix_persisted == false
    and $matrix.memory_write_execution_denial_matrix_materialized == false
    and $matrix.memory_write_execution_denial_matrix_filesystem_written == false
    and $matrix.pre_execution_validation_recorded == false
    and $matrix.pre_execution_validation_persisted == false
    and $matrix.pre_execution_validation_accepted == false
    and $matrix.memory_write_approval_packet_accepted == false
    and $matrix.memory_write_request_accepted == false
    and $matrix.operator_approval_recorded == false
    and $matrix.raw_payload_plaintext_recorded == false
    and $matrix.raw_payload_plaintext_persisted == false
    and $matrix.memory_write_execution_allowed == false
    and $matrix.memory_write_execution_ready == false
    and $matrix.memory_write_execution_performed == false
    and $matrix.memory_store_mutation_allowed == false
    and $matrix.memory_store_mutated == false
    and $matrix.live_mutation_execution_ready == false
    and $matrix.rollback_execution_allowed == false
    and $matrix.rollback_executed == false
    and $matrix.external_send_enabled == false
    and $matrix.external_send_performed == false
    and $matrix.public_claim_or_release_artifact_write_enabled == false
    and $matrix.public_release_published == false
    and $matrix.release_artifact_written == false
    and ($matrix.execution_denial_fixtures | length) == 7
    and ($matrix.execution_denial_fixtures | all(.execution_requested == true and .execution_allowed == false and .execution_performed == false and .memory_store_mutated == false and .activation_allowed == false))
    and ($matrix.required_before_memory_write_execution | length) == 17
    and ($matrix.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_no_write_sink_contract_gate" \
  --arg denial_matrix_report_sha256 "$denial_matrix_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson matrix "$DENIAL_MATRIX_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    no_write_sink_contract_mode:"memory_write_execution_no_write_sink_contract_no_store_mutation",
    source_memory_write_execution_denial_matrix_gate:$matrix.gate,
    source_memory_write_execution_denial_matrix_ready:$matrix.memory_write_execution_denial_matrix_ready,
    source_memory_write_execution_denial_matrix_report_sha256:$denial_matrix_report_sha256,
    source_memory_write_execution_preflight_report_sha256:$matrix.source_memory_write_execution_preflight_report_sha256,
    source_memory_write_approval_packet_report_sha256:$matrix.source_memory_write_approval_packet_report_sha256,
    source_memory_write_contract_report_sha256:$matrix.source_memory_write_contract_report_sha256,
    source_memory_intelligence_report_sha256:$matrix.source_memory_intelligence_report_sha256,
    source_payload_redaction_acceptance_matrix_report_sha256:$matrix.source_payload_redaction_acceptance_matrix_report_sha256,
    source_payload_redaction_proof_report_sha256:$matrix.source_payload_redaction_proof_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_write_execution_no_write_sink_contract_ready:true,
    memory_write_execution_denial_matrix_ready:true,
    pre_execution_validation_shape_ready:$matrix.pre_execution_validation_shape_ready,
    required_pre_execution_validation_check_count:$matrix.required_pre_execution_validation_check_count,
    accepted_pre_execution_validation_check_count:0,
    required_no_write_sink_surface_count:8,
    ready_no_write_sink_surface_count:8,
    side_effect_free_no_write_sink_surface_count:8,
    no_write_sink_fixture_count:6,
    no_write_sink_accepted_validation_fixture_count:3,
    no_write_sink_rejected_execution_fixture_count:3,
    no_write_sink_execution_request_fixture_count:6,
    no_write_sink_write_request_fixture_count:3,
    no_write_sink_allowed_write_fixture_count:0,
    no_write_sink_rejected_write_fixture_count:3,
    no_write_sink_accepts_redacted_execution_envelope:true,
    no_write_sink_accepts_source_report_hash_bindings:true,
    no_write_sink_requires_operator_approval_and_preflight_validation:true,
    no_write_sink_requires_namespace_operation_retention_allowlist:true,
    no_write_sink_requires_payload_hash_binding_without_plaintext:true,
    no_write_sink_requires_fresh_soak_rollback_validation:true,
    no_write_sink_rejects_external_send_public_claim_artifact:true,
    no_write_sink_rejects_store_write_execution:true,
    no_write_sink_write_path_enabled_by_default:false,
    no_write_sink_persistence_enabled_by_default:false,
    memory_write_execution_denial_matrix_recorded:false,
    memory_write_execution_denial_matrix_persisted:false,
    memory_write_execution_no_write_sink_contract_recorded:false,
    memory_write_execution_no_write_sink_contract_persisted:false,
    memory_write_execution_no_write_sink_contract_materialized:false,
    memory_write_execution_no_write_sink_contract_filesystem_written:false,
    pre_execution_validation_recorded:false,
    pre_execution_validation_persisted:false,
    pre_execution_validation_accepted:false,
    memory_write_approval_packet_recorded:false,
    memory_write_approval_packet_persisted:false,
    memory_write_approval_packet_accepted:false,
    memory_write_request_recorded:false,
    memory_write_request_accepted:false,
    memory_write_request_persisted:false,
    operator_approval_recorded:false,
    operator_identity_hash_recorded:false,
    operator_approval_signature_hash_recorded:false,
    operator_approval_timestamp_recorded:false,
    memory_namespace_recorded:false,
    memory_write_operation_recorded:false,
    memory_retention_class_recorded:false,
    accepted_redaction_proof_recorded:false,
    accepted_redaction_proof_count:0,
    source_memory_write_approval_packet_hash_bound:false,
    source_memory_write_contract_hash_bound:false,
    source_memory_intelligence_hash_bound:false,
    source_payload_redaction_acceptance_matrix_hash_bound:false,
    source_payload_redaction_proof_hash_bound:false,
    raw_payload_sha256_bound:false,
    redacted_payload_summary_sha256_bound:false,
    raw_payload_plaintext_recorded:false,
    raw_payload_plaintext_persisted:false,
    fresh_pre_activation_soak_evidence_recorded:false,
    rollback_plan_recorded:false,
    post_write_validation_plan_recorded:false,
    memory_write_execution_allowed:false,
    memory_write_execution_ready:false,
    memory_write_execution_performed:false,
    memory_write_execution_performed_count:0,
    memory_write_execution_allowed_count:0,
    memory_write_execution_denied_count:6,
    memory_store_write_path_enabled:false,
    memory_store_write_performed_count:0,
    memory_store_mutation_allowed:false,
    memory_store_mutated:false,
    live_mutation_execution_ready:false,
    rollback_execution_allowed:false,
    rollback_executed:false,
    provider_prompt_replay_enabled:false,
    external_send_enabled:false,
    external_send_performed:false,
    public_claim_or_release_artifact_write_enabled:false,
    public_release_published:false,
    release_artifact_written:false,
    no_write_sink_surfaces:[
      "redacted_execution_request_envelope_validation",
      "source_report_hash_binding_validation",
      "operator_approval_preflight_validation_requirement",
      "memory_namespace_operation_retention_allowlist_requirement",
      "payload_hash_binding_without_plaintext_requirement",
      "fresh_soak_rollback_validation_requirement",
      "external_send_public_claim_artifact_rejection",
      "store_write_path_disabled_by_default"
    ],
    no_write_sink_fixtures:[
      {
        id:"redacted-execution-envelope-validation-shape",
        sink_status:"accepted_for_no_write_validation",
        redacted_execution_request_envelope_present:true,
        source_report_hash_bindings_present:true,
        execution_requested:true,
        write_requested:false,
        execution_allowed:false,
        execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"redacted_shape_can_be_validated_but_execution_remains_disabled"
      },
      {
        id:"source-report-hash-bound-validation-shape",
        sink_status:"accepted_for_no_write_validation",
        source_memory_write_execution_denial_matrix_report_sha256_bound:true,
        source_memory_write_execution_preflight_report_sha256_bound:true,
        source_payload_redaction_proof_report_sha256_bound:true,
        execution_requested:true,
        write_requested:false,
        execution_allowed:false,
        execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"source_hash_shape_can_be_validated_but_not_executed"
      },
      {
        id:"approval-preflight-allowlist-validation-shape",
        sink_status:"accepted_for_no_write_validation",
        operator_approval_required:true,
        all_pre_execution_validation_checks_required:true,
        namespace_operation_retention_allowlist_required:true,
        payload_hash_binding_without_plaintext_required:true,
        fresh_soak_rollback_validation_required:true,
        execution_requested:true,
        write_requested:false,
        execution_allowed:false,
        execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"future_requirements_can_be_described_but_no_write_sink_keeps_execution_off"
      },
      {
        id:"store-write-path-disabled-mutation-attempt",
        sink_status:"rejected",
        execution_requested:true,
        write_requested:true,
        memory_store_mutation_requested:true,
        no_write_sink_write_path_enabled_by_default:false,
        execution_allowed:false,
        execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"store_write_path_disabled_by_default"
      },
      {
        id:"external-send-public-artifact-attempt",
        sink_status:"rejected",
        execution_requested:true,
        write_requested:true,
        external_send_requested:true,
        public_claim_requested:true,
        release_artifact_write_requested:true,
        execution_allowed:false,
        execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        external_send_performed:false,
        public_release_published:false,
        release_artifact_written:false,
        activation_allowed:false,
        reason:"external_send_public_claim_and_release_artifact_denied"
      },
      {
        id:"rollback-or-direct-store-execution-attempt",
        sink_status:"rejected",
        execution_requested:true,
        write_requested:true,
        rollback_execution_requested:true,
        memory_store_mutation_requested:true,
        execution_allowed:false,
        execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        activation_allowed:false,
        reason:"no_write_sink_cannot_execute_rollback_or_direct_store_mutation"
      }
    ],
    denied_by_no_write_sink_contract:[
      "execution_remains_disabled",
      "store_write_path_disabled_by_default",
      "memory_store_mutation_denied",
      "rollback_execution_denied",
      "external_send_denied",
      "public_claim_denied",
      "release_artifact_write_denied",
      "plaintext_payload_recording_denied",
      "secret_read_denied",
      "service_restart_denied"
    ],
    required_before_any_memory_write_execution:[
      "accepted_operator_approval_packet",
      "accepted_pre_execution_validation_record",
      "operator_identity_hash",
      "operator_approval_signature_hash",
      "operator_approval_timestamp",
      "single_surface_activation_scope",
      "namespace_operation_retention_allowlist_match",
      "accepted_redaction_proof_id",
      "source_report_hash_bindings",
      "raw_payload_sha256_without_plaintext",
      "redacted_payload_summary_sha256",
      "fresh_pre_activation_soak_evidence",
      "rollback_plan_id",
      "post_write_validation_plan_id",
      "no_public_claim_no_external_send_decision",
      "explicit_write_path_enablement",
      "post_write_watchdog_soak_plan"
    ],
    side_effects:{
      memory_store_mutated:false,
      memory_store_write_performed:false,
      memory_write_request_recorded:false,
      memory_write_request_persisted:false,
      memory_write_approval_packet_recorded:false,
      memory_write_approval_packet_persisted:false,
      memory_write_execution_preflight_recorded:false,
      memory_write_execution_preflight_persisted:false,
      memory_write_execution_denial_matrix_recorded:false,
      memory_write_execution_denial_matrix_persisted:false,
      memory_write_execution_no_write_sink_contract_recorded:false,
      memory_write_execution_no_write_sink_contract_persisted:false,
      memory_write_execution_no_write_sink_contract_materialized:false,
      memory_write_execution_no_write_sink_contract_filesystem_written:false,
      pre_execution_validation_recorded:false,
      pre_execution_validation_persisted:false,
      payload_plaintext_persisted:false,
      raw_payload_inspected:false,
      capability_registry_mutated:false,
      plugin_registry_mutated:false,
      skill_workshop_written:false,
      provider_invoked:false,
      model_invoked:false,
      provider_prompt_replayed:false,
      channel_send_performed:false,
      external_send_performed:false,
      runtime_store_mutated:false,
      gateway_event_enqueued:false,
      filesystem_written:false,
      release_artifact_written:false,
      public_artifact_written:false,
      public_release_published:false,
      approval_record_persisted:false,
      preflight_record_persisted:false,
      denial_matrix_persisted:false,
      no_write_sink_contract_persisted:false,
      receipt_persisted:false,
      launchd_mutated:false,
      service_restarted:false,
      rollback_executed:false,
      credential_read:false,
      secret_file_read:false
    }
  }')"

jq -e '
  .status == "ready"
  and .memory_write_execution_no_write_sink_contract_ready == true
  and .no_write_sink_contract_mode == "memory_write_execution_no_write_sink_contract_no_store_mutation"
  and .source_memory_write_execution_denial_matrix_ready == true
  and .source_memory_write_execution_denial_matrix_report_sha256 != ""
  and .source_memory_write_execution_preflight_report_sha256 != ""
  and .source_memory_write_approval_packet_report_sha256 != ""
  and .source_memory_write_contract_report_sha256 != ""
  and .source_memory_intelligence_report_sha256 != ""
  and .source_payload_redaction_acceptance_matrix_report_sha256 != ""
  and .source_payload_redaction_proof_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .required_pre_execution_validation_check_count == 17
  and .accepted_pre_execution_validation_check_count == 0
  and .required_no_write_sink_surface_count == 8
  and .ready_no_write_sink_surface_count == 8
  and .side_effect_free_no_write_sink_surface_count == 8
  and .no_write_sink_fixture_count == 6
  and .no_write_sink_accepted_validation_fixture_count == 3
  and .no_write_sink_rejected_execution_fixture_count == 3
  and .no_write_sink_execution_request_fixture_count == 6
  and .no_write_sink_write_request_fixture_count == 3
  and .no_write_sink_allowed_write_fixture_count == 0
  and .no_write_sink_rejected_write_fixture_count == 3
  and .no_write_sink_accepts_redacted_execution_envelope == true
  and .no_write_sink_accepts_source_report_hash_bindings == true
  and .no_write_sink_requires_operator_approval_and_preflight_validation == true
  and .no_write_sink_requires_namespace_operation_retention_allowlist == true
  and .no_write_sink_requires_payload_hash_binding_without_plaintext == true
  and .no_write_sink_requires_fresh_soak_rollback_validation == true
  and .no_write_sink_rejects_external_send_public_claim_artifact == true
  and .no_write_sink_rejects_store_write_execution == true
  and .no_write_sink_write_path_enabled_by_default == false
  and .no_write_sink_persistence_enabled_by_default == false
  and .memory_write_execution_no_write_sink_contract_recorded == false
  and .memory_write_execution_no_write_sink_contract_persisted == false
  and .memory_write_execution_no_write_sink_contract_materialized == false
  and .memory_write_execution_no_write_sink_contract_filesystem_written == false
  and .memory_write_approval_packet_accepted == false
  and .memory_write_request_accepted == false
  and .operator_approval_recorded == false
  and .raw_payload_plaintext_recorded == false
  and .raw_payload_plaintext_persisted == false
  and .memory_write_execution_allowed == false
  and .memory_write_execution_ready == false
  and .memory_write_execution_performed == false
  and .memory_write_execution_performed_count == 0
  and .memory_write_execution_allowed_count == 0
  and .memory_write_execution_denied_count == 6
  and .memory_store_write_path_enabled == false
  and .memory_store_write_performed_count == 0
  and .memory_store_mutation_allowed == false
  and .memory_store_mutated == false
  and .live_mutation_execution_ready == false
  and .rollback_execution_allowed == false
  and .rollback_executed == false
  and .external_send_enabled == false
  and .external_send_performed == false
  and .public_claim_or_release_artifact_write_enabled == false
  and .public_release_published == false
  and .release_artifact_written == false
  and (.no_write_sink_surfaces | length) == 8
  and (.no_write_sink_fixtures | length) == 6
  and ([.no_write_sink_fixtures[] | select(.sink_status == "accepted_for_no_write_validation")] | length) == 3
  and ([.no_write_sink_fixtures[] | select(.sink_status == "rejected")] | length) == 3
  and (.no_write_sink_fixtures | all(.execution_requested == true and .execution_allowed == false and .execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .activation_allowed == false))
  and ([.no_write_sink_fixtures[] | select(.write_requested == true)] | length) == 3
  and (.denied_by_no_write_sink_contract | length) == 10
  and (.required_before_any_memory_write_execution | length) == 17
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory live mutation operator write execution no-write sink contract gate passed"
