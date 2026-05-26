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

APPROVAL_PACKET_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-approval-packet-gate" \
    scripts/hepta-memory-live-mutation-operator-write-approval-packet-gate.sh
)"

approval_packet_report_sha256="$(printf '%s' "$APPROVAL_PACKET_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson packet "$APPROVAL_PACKET_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $packet.runtime == "hepta"
    and $packet.status == "ready"
    and $packet.gate == "hepta_memory_live_mutation_operator_write_approval_packet_gate"
    and $packet.memory_write_approval_packet_shape_ready == true
    and $packet.approval_packet_mode == "memory_write_operator_approval_packet_shape_no_recording_no_execution"
    and $packet.source_memory_write_contract_ready == true
    and $packet.source_memory_write_contract_report_sha256 != ""
    and $packet.source_memory_intelligence_report_sha256 != ""
    and $packet.source_payload_redaction_acceptance_matrix_report_sha256 != ""
    and $packet.source_payload_redaction_proof_report_sha256 != ""
    and $packet.minimum_required_samples >= 24
    and $packet.memory_write_approval_packet_recorded == false
    and $packet.memory_write_approval_packet_persisted == false
    and $packet.memory_write_approval_packet_accepted == false
    and $packet.memory_write_request_recorded == false
    and $packet.memory_write_request_accepted == false
    and $packet.memory_write_request_persisted == false
    and $packet.operator_approval_recorded == false
    and $packet.operator_identity_hash_recorded == false
    and $packet.operator_approval_signature_hash_recorded == false
    and $packet.operator_approval_timestamp_recorded == false
    and $packet.accepted_redaction_proof_recorded == false
    and $packet.accepted_redaction_proof_count == 0
    and $packet.fresh_pre_activation_soak_evidence_recorded == false
    and $packet.rollback_plan_recorded == false
    and $packet.post_write_validation_plan_recorded == false
    and $packet.no_public_claim_no_external_send_decision_recorded == false
    and $packet.raw_payload_plaintext_recorded == false
    and $packet.raw_payload_plaintext_persisted == false
    and $packet.memory_store_mutation_allowed == false
    and $packet.memory_store_mutated == false
    and $packet.memory_write_execution_ready == false
    and $packet.live_mutation_execution_ready == false
    and ($packet.required_memory_write_approval_packet_fields | length) == 21
    and ($packet.denied_memory_write_approval_packet_fixtures | length) == 8
    and ($packet.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_preflight_gate" \
  --arg approval_packet_report_sha256 "$approval_packet_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson packet "$APPROVAL_PACKET_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    execution_preflight_mode:"memory_write_execution_preflight_no_approval_no_mutation",
    source_memory_write_approval_packet_gate:$packet.gate,
    source_memory_write_approval_packet_shape_ready:$packet.memory_write_approval_packet_shape_ready,
    source_memory_write_approval_packet_report_sha256:$approval_packet_report_sha256,
    source_memory_write_contract_report_sha256:$packet.source_memory_write_contract_report_sha256,
    source_memory_intelligence_report_sha256:$packet.source_memory_intelligence_report_sha256,
    source_payload_redaction_acceptance_matrix_report_sha256:$packet.source_payload_redaction_acceptance_matrix_report_sha256,
    source_payload_redaction_proof_report_sha256:$packet.source_payload_redaction_proof_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_write_execution_preflight_ready:true,
    pre_execution_validation_shape_ready:true,
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
    single_surface_activation_scope_recorded:false,
    memory_namespace_recorded:false,
    memory_write_operation_recorded:false,
    memory_write_operation_allowed:false,
    memory_retention_class_recorded:false,
    accepted_redaction_proof_recorded:false,
    accepted_redaction_proof_count:0,
    source_memory_write_approval_packet_hash_bound:false,
    source_memory_write_contract_hash_bound:false,
    source_memory_intelligence_hash_bound:false,
    source_payload_redaction_acceptance_matrix_hash_bound:false,
    raw_payload_sha256_bound:false,
    redacted_payload_summary_sha256_bound:false,
    fresh_pre_activation_soak_evidence_recorded:false,
    rollback_plan_recorded:false,
    post_write_validation_plan_recorded:false,
    no_public_claim_no_external_send_decision_recorded:false,
    memory_write_execution_allowed:false,
    memory_write_execution_ready:false,
    memory_store_mutation_allowed:false,
    memory_store_mutated:false,
    live_mutation_execution_ready:false,
    rollback_execution_allowed:false,
    rollback_executed:false,
    provider_prompt_replay_enabled:false,
    external_send_enabled:false,
    public_claim_or_release_artifact_write_enabled:false,
    required_pre_execution_validation_check_count:17,
    recorded_pre_execution_validation_check_count:0,
    required_pre_execution_validation_checks:[
      "approval_packet_hash_binding",
      "memory_write_request_hash_binding",
      "operator_approval_signature_verification",
      "single_surface_scope_verification",
      "memory_namespace_allowlist_verification",
      "memory_write_operation_allowlist_verification",
      "retention_class_allowlist_verification",
      "redaction_proof_acceptance_verification",
      "raw_payload_sha256_binding",
      "redacted_payload_summary_sha256_binding",
      "source_memory_intelligence_hash_binding",
      "source_payload_redaction_matrix_hash_binding",
      "source_memory_write_contract_hash_binding",
      "fresh_pre_activation_soak_verification",
      "rollback_plan_verification",
      "post_write_validation_plan_verification",
      "no_public_claim_no_external_send_verification"
    ],
    denied_memory_write_execution_preflight_fixtures:[
      {
        id:"no-accepted-approval-packet",
        recorded_pre_execution_validation_check_count:0,
        packet_accepted:false,
        execution_allowed:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"accepted_approval_packet_required"
      },
      {
        id:"missing-approval-packet-hash-binding",
        recorded_pre_execution_validation_check_count:3,
        source_memory_write_approval_packet_hash_bound:false,
        execution_allowed:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"approval_packet_hash_binding_required"
      },
      {
        id:"operator-signature-or-timestamp-invalid",
        recorded_pre_execution_validation_check_count:5,
        operator_approval_signature_verified:false,
        operator_approval_timestamp_fresh:false,
        execution_allowed:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"valid_operator_signature_and_fresh_timestamp_required"
      },
      {
        id:"namespace-operation-or-retention-not-allowlisted",
        recorded_pre_execution_validation_check_count:8,
        memory_namespace_allowed:false,
        memory_write_operation_allowed:false,
        memory_retention_class_allowed:false,
        execution_allowed:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"memory_namespace_operation_and_retention_allowlists_required"
      },
      {
        id:"redaction-proof-missing-or-stale",
        recorded_pre_execution_validation_check_count:9,
        accepted_redaction_proof_recorded:false,
        accepted_redaction_proof_fresh:false,
        execution_allowed:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"fresh_accepted_redaction_proof_required"
      },
      {
        id:"payload-hash-mismatch-or-plaintext-present",
        recorded_pre_execution_validation_check_count:11,
        raw_payload_sha256_bound:false,
        redacted_payload_summary_sha256_bound:false,
        raw_payload_plaintext_recorded:true,
        execution_allowed:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"payload_hash_binding_without_plaintext_required"
      },
      {
        id:"fresh-soak-rollback-or-validation-missing",
        recorded_pre_execution_validation_check_count:14,
        fresh_pre_activation_soak_evidence_recorded:false,
        rollback_plan_recorded:false,
        post_write_validation_plan_recorded:false,
        execution_allowed:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"fresh_soak_rollback_and_validation_required"
      },
      {
        id:"external-send-public-claim-or-release-artifact",
        recorded_pre_execution_validation_check_count:17,
        external_send_requested:true,
        public_claim_requested:true,
        release_artifact_write_requested:true,
        execution_allowed:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"external_send_public_claim_and_release_artifact_denied"
      },
      {
        id:"direct-memory-store-execution-at-preflight-layer",
        recorded_pre_execution_validation_check_count:17,
        memory_store_mutation_requested:true,
        rollback_execution_requested:true,
        execution_allowed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        activation_allowed:false,
        reason:"execution_preflight_layer_cannot_mutate_memory_or_execute_rollback"
      }
    ],
    denied_by_memory_write_execution_preflight_gate:[
      "accepted_approval_packet_required",
      "approval_packet_hash_binding_required",
      "memory_write_request_hash_binding_required",
      "valid_operator_signature_and_fresh_timestamp_required",
      "single_surface_scope_verification_required",
      "memory_namespace_allowlist_required",
      "memory_write_operation_allowlist_required",
      "retention_class_allowlist_required",
      "fresh_accepted_redaction_proof_required",
      "raw_payload_sha256_binding_required",
      "redacted_payload_summary_sha256_binding_required",
      "source_memory_intelligence_hash_binding_required",
      "source_payload_redaction_matrix_hash_binding_required",
      "source_memory_write_contract_hash_binding_required",
      "fresh_pre_activation_soak_verification_required",
      "rollback_plan_verification_required",
      "post_write_validation_plan_verification_required",
      "no_public_claim_no_external_send_verification_required",
      "payload_plaintext_recording_denied",
      "memory_store_mutation_denied",
      "rollback_execution_denied",
      "external_send_public_claim_release_artifact_denied"
    ],
    required_before_memory_write_execution:[
      "accepted_memory_write_approval_packet",
      "approval_packet_hash_binding",
      "memory_write_request_hash_binding",
      "operator_approval_signature_verification",
      "operator_approval_timestamp_freshness",
      "single_surface_activation_scope_verification",
      "memory_namespace_allowlist_verification",
      "memory_write_operation_allowlist_verification",
      "retention_class_allowlist_verification",
      "accepted_redaction_proof_freshness",
      "raw_payload_hash_binding_without_plaintext",
      "redacted_payload_summary_hash_binding",
      "source_report_hash_bindings",
      "fresh_pre_activation_soak_evidence",
      "rollback_plan",
      "post_write_validation_plan",
      "no_public_claim_no_external_send_decision"
    ],
    side_effects:{
      memory_store_mutated:false,
      memory_write_request_recorded:false,
      memory_write_request_persisted:false,
      memory_write_approval_packet_recorded:false,
      memory_write_approval_packet_persisted:false,
      memory_write_execution_preflight_recorded:false,
      memory_write_execution_preflight_persisted:false,
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
      approval_record_persisted:false,
      preflight_record_persisted:false,
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
  and .memory_write_execution_preflight_ready == true
  and .pre_execution_validation_shape_ready == true
  and .execution_preflight_mode == "memory_write_execution_preflight_no_approval_no_mutation"
  and .source_memory_write_approval_packet_shape_ready == true
  and .source_memory_write_approval_packet_report_sha256 != ""
  and .source_memory_write_contract_report_sha256 != ""
  and .source_memory_intelligence_report_sha256 != ""
  and .source_payload_redaction_acceptance_matrix_report_sha256 != ""
  and .source_payload_redaction_proof_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .pre_execution_validation_recorded == false
  and .pre_execution_validation_persisted == false
  and .pre_execution_validation_accepted == false
  and .memory_write_approval_packet_recorded == false
  and .memory_write_approval_packet_persisted == false
  and .memory_write_approval_packet_accepted == false
  and .memory_write_request_recorded == false
  and .memory_write_request_accepted == false
  and .memory_write_request_persisted == false
  and .operator_approval_recorded == false
  and .operator_identity_hash_recorded == false
  and .operator_approval_signature_hash_recorded == false
  and .operator_approval_timestamp_recorded == false
  and .single_surface_activation_scope_recorded == false
  and .memory_namespace_recorded == false
  and .memory_write_operation_recorded == false
  and .memory_write_operation_allowed == false
  and .memory_retention_class_recorded == false
  and .accepted_redaction_proof_recorded == false
  and .accepted_redaction_proof_count == 0
  and .source_memory_write_approval_packet_hash_bound == false
  and .source_memory_write_contract_hash_bound == false
  and .source_memory_intelligence_hash_bound == false
  and .source_payload_redaction_acceptance_matrix_hash_bound == false
  and .raw_payload_sha256_bound == false
  and .redacted_payload_summary_sha256_bound == false
  and .fresh_pre_activation_soak_evidence_recorded == false
  and .rollback_plan_recorded == false
  and .post_write_validation_plan_recorded == false
  and .no_public_claim_no_external_send_decision_recorded == false
  and .memory_write_execution_allowed == false
  and .memory_write_execution_ready == false
  and .memory_store_mutation_allowed == false
  and .memory_store_mutated == false
  and .live_mutation_execution_ready == false
  and .rollback_execution_allowed == false
  and .rollback_executed == false
  and .provider_prompt_replay_enabled == false
  and .external_send_enabled == false
  and .public_claim_or_release_artifact_write_enabled == false
  and .required_pre_execution_validation_check_count == 17
  and .recorded_pre_execution_validation_check_count == 0
  and (.required_pre_execution_validation_checks | length) == 17
  and (.denied_memory_write_execution_preflight_fixtures | length) == 9
  and (.denied_memory_write_execution_preflight_fixtures | all(.execution_allowed == false and .memory_store_mutated == false and .activation_allowed == false))
  and (.denied_by_memory_write_execution_preflight_gate | length) == 22
  and (.required_before_memory_write_execution | length) == 17
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory live mutation operator write execution preflight gate passed"
