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

CONTRACT_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-contract-gate" \
    scripts/hepta-memory-live-mutation-operator-write-contract-gate.sh
)"

contract_report_sha256="$(printf '%s' "$CONTRACT_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson contract "$CONTRACT_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $contract.runtime == "hepta"
    and $contract.status == "ready"
    and $contract.gate == "hepta_memory_live_mutation_operator_write_contract_gate"
    and $contract.memory_live_mutation_operator_write_contract_ready == true
    and $contract.contract_mode == "operator_approved_memory_write_request_shape_no_memory_store_write"
    and $contract.source_memory_intelligence_report_sha256 != ""
    and $contract.source_payload_redaction_acceptance_matrix_ready == true
    and $contract.source_payload_redaction_acceptance_matrix_report_sha256 != ""
    and $contract.source_payload_redaction_proof_report_sha256 != ""
    and $contract.minimum_required_samples >= 24
    and $contract.memory_write_request_recorded == false
    and $contract.memory_write_request_accepted == false
    and $contract.memory_write_request_persisted == false
    and $contract.operator_approval_recorded == false
    and $contract.operator_identity_hash_recorded == false
    and $contract.single_surface_activation_scope_recorded == false
    and $contract.accepted_redaction_proof_recorded == false
    and $contract.accepted_redaction_proof_count == 0
    and $contract.memory_write_execution_ready == false
    and $contract.memory_store_mutation_allowed == false
    and $contract.live_mutation_execution_ready == false
    and $contract.raw_payload_plaintext_recorded == false
    and $contract.raw_payload_plaintext_persisted == false
    and $contract.memory_store_mutation_enabled == false
    and $contract.provider_prompt_replay_enabled == false
    and $contract.external_send_enabled == false
    and $contract.public_claim_or_release_artifact_write_enabled == false
    and $contract.required_memory_write_request_field_count == 17
    and ($contract.allowed_memory_write_operations | length) == 4
    and ($contract.required_memory_write_request_fields | length) == 17
    and ($contract.denied_memory_write_request_fixtures | length) == 9
    and ($contract.denied_memory_write_request_fixtures | all(.request_accepted == false and .memory_store_mutated == false and .activation_allowed == false))
    and ($contract.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_approval_packet_gate" \
  --arg contract_report_sha256 "$contract_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson contract "$CONTRACT_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    approval_packet_mode:"memory_write_operator_approval_packet_shape_no_recording_no_execution",
    source_memory_write_contract_gate:$contract.gate,
    source_memory_write_contract_ready:$contract.memory_live_mutation_operator_write_contract_ready,
    source_memory_write_contract_report_sha256:$contract_report_sha256,
    source_memory_intelligence_report_sha256:$contract.source_memory_intelligence_report_sha256,
    source_payload_redaction_acceptance_matrix_report_sha256:$contract.source_payload_redaction_acceptance_matrix_report_sha256,
    source_payload_redaction_proof_report_sha256:$contract.source_payload_redaction_proof_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_write_approval_packet_shape_ready:true,
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
    record_intent_recorded:false,
    raw_payload_sha256_recorded:false,
    redacted_payload_summary_sha256_recorded:false,
    accepted_redaction_proof_recorded:false,
    accepted_redaction_proof_count:0,
    source_memory_intelligence_hash_bound:false,
    source_payload_redaction_acceptance_matrix_hash_bound:false,
    source_memory_write_contract_hash_bound:false,
    fresh_pre_activation_soak_evidence_recorded:false,
    rollback_plan_recorded:false,
    post_write_validation_plan_recorded:false,
    no_public_claim_no_external_send_decision_recorded:false,
    raw_payload_plaintext_recorded:false,
    raw_payload_plaintext_persisted:false,
    memory_store_mutation_allowed:false,
    memory_store_mutated:false,
    memory_write_execution_ready:false,
    live_mutation_execution_ready:false,
    provider_prompt_replay_enabled:false,
    external_send_enabled:false,
    public_claim_or_release_artifact_write_enabled:false,
    required_memory_write_approval_packet_field_count:21,
    recorded_memory_write_approval_packet_field_count:0,
    inherited_required_memory_write_request_field_count:$contract.required_memory_write_request_field_count,
    inherited_allowed_memory_write_operations:$contract.allowed_memory_write_operations,
    required_memory_write_approval_packet_fields:[
      "approval_packet_id",
      "memory_write_request_id",
      "operator_approval_id",
      "operator_identity_hash",
      "operator_approval_signature_hash",
      "operator_approval_captured_at_unix",
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
      "source_memory_write_contract_report_sha256",
      "fresh_pre_activation_soak_evidence_id",
      "rollback_plan_id",
      "post_write_validation_plan_id",
      "no_public_claim_no_external_send_decision"
    ],
    denied_memory_write_approval_packet_fixtures:[
      {
        id:"empty-memory-write-approval-packet",
        recorded_memory_write_approval_packet_field_count:0,
        packet_accepted:false,
        memory_write_request_accepted:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"approval_packet_not_recorded"
      },
      {
        id:"operator-approval-without-identity-signature",
        recorded_memory_write_approval_packet_field_count:7,
        operator_approval_recorded:true,
        operator_identity_hash_recorded:false,
        operator_approval_signature_hash_recorded:false,
        packet_accepted:false,
        memory_write_request_accepted:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"operator_identity_and_signature_required"
      },
      {
        id:"disallowed-memory-write-operation",
        recorded_memory_write_approval_packet_field_count:12,
        memory_write_operation:"raw_secret_or_credential_persistence",
        memory_write_operation_allowed:false,
        packet_accepted:false,
        memory_write_request_accepted:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"memory_write_operation_not_in_allowlist"
      },
      {
        id:"memory-write-without-accepted-redaction-proof",
        recorded_memory_write_approval_packet_field_count:15,
        accepted_redaction_proof_recorded:false,
        accepted_redaction_proof_count:0,
        packet_accepted:false,
        memory_write_request_accepted:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"accepted_redaction_proof_required"
      },
      {
        id:"memory-write-without-fresh-soak-rollback-or-validation",
        recorded_memory_write_approval_packet_field_count:18,
        fresh_pre_activation_soak_evidence_recorded:false,
        rollback_plan_recorded:false,
        post_write_validation_plan_recorded:false,
        packet_accepted:false,
        memory_write_request_accepted:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"fresh_soak_rollback_and_validation_required"
      },
      {
        id:"raw-secret-marker-memory-approval-packet",
        recorded_memory_write_approval_packet_field_count:21,
        raw_secret_marker_detected:true,
        raw_payload_plaintext_recorded:true,
        packet_accepted:false,
        memory_write_request_accepted:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"raw_secret_or_plaintext_payload_denied"
      },
      {
        id:"external-send-public-claim-release-artifact-memory-packet",
        recorded_memory_write_approval_packet_field_count:21,
        external_send_requested:true,
        public_claim_requested:true,
        release_artifact_write_requested:true,
        packet_accepted:false,
        memory_write_request_accepted:false,
        memory_store_mutated:false,
        activation_allowed:false,
        external_send_allowed:false,
        public_claim_allowed:false,
        release_artifact_write_allowed:false,
        reason:"external_send_public_claim_and_release_artifact_denied"
      },
      {
        id:"direct-memory-store-mutation-at-approval-packet-layer",
        recorded_memory_write_approval_packet_field_count:21,
        memory_store_mutation_requested:true,
        packet_accepted:false,
        memory_write_request_accepted:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"approval_packet_layer_cannot_execute_memory_store_mutation"
      }
    ],
    denied_by_memory_write_approval_packet_gate:[
      "memory_write_approval_packet_not_recorded",
      "memory_write_approval_packet_not_persisted",
      "memory_write_request_not_recorded",
      "operator_approval_not_recorded",
      "operator_identity_hash_not_recorded",
      "operator_approval_signature_hash_not_recorded",
      "single_surface_activation_scope_not_recorded",
      "memory_namespace_not_recorded",
      "memory_write_operation_not_recorded",
      "memory_write_operation_not_allowed",
      "accepted_redaction_proof_not_recorded",
      "source_memory_intelligence_hash_not_bound",
      "source_payload_redaction_acceptance_matrix_hash_not_bound",
      "source_memory_write_contract_hash_not_bound",
      "fresh_pre_activation_soak_evidence_not_recorded",
      "rollback_plan_not_recorded",
      "post_write_validation_plan_not_recorded",
      "no_public_claim_no_external_send_decision_not_recorded",
      "raw_payload_plaintext_recording_denied",
      "memory_store_mutation_denied",
      "external_send_public_claim_release_artifact_denied"
    ],
    required_before_memory_write_approval_packet_acceptance:[
      "operator_approval_id",
      "operator_identity_hash",
      "operator_approval_signature_hash",
      "operator_approval_timestamp",
      "single_surface_activation_scope",
      "allowed_memory_write_operation",
      "accepted_redaction_proof_id",
      "source_memory_intelligence_hash_binding",
      "source_payload_redaction_acceptance_matrix_hash_binding",
      "source_memory_write_contract_hash_binding",
      "fresh_pre_activation_soak_evidence_id",
      "rollback_plan_id",
      "post_write_validation_plan_id",
      "no_public_claim_no_external_send_decision"
    ],
    side_effects:{
      memory_store_mutated:false,
      memory_write_request_recorded:false,
      memory_write_request_persisted:false,
      memory_write_approval_packet_recorded:false,
      memory_write_approval_packet_persisted:false,
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
  and .memory_write_approval_packet_shape_ready == true
  and .approval_packet_mode == "memory_write_operator_approval_packet_shape_no_recording_no_execution"
  and .source_memory_write_contract_ready == true
  and .source_memory_write_contract_report_sha256 != ""
  and .source_memory_intelligence_report_sha256 != ""
  and .source_payload_redaction_acceptance_matrix_report_sha256 != ""
  and .source_payload_redaction_proof_report_sha256 != ""
  and .minimum_required_samples >= 24
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
  and .accepted_redaction_proof_recorded == false
  and .accepted_redaction_proof_count == 0
  and .source_memory_write_contract_hash_bound == false
  and .fresh_pre_activation_soak_evidence_recorded == false
  and .rollback_plan_recorded == false
  and .post_write_validation_plan_recorded == false
  and .no_public_claim_no_external_send_decision_recorded == false
  and .raw_payload_plaintext_recorded == false
  and .raw_payload_plaintext_persisted == false
  and .memory_store_mutation_allowed == false
  and .memory_store_mutated == false
  and .memory_write_execution_ready == false
  and .live_mutation_execution_ready == false
  and .provider_prompt_replay_enabled == false
  and .external_send_enabled == false
  and .public_claim_or_release_artifact_write_enabled == false
  and .required_memory_write_approval_packet_field_count == 21
  and .recorded_memory_write_approval_packet_field_count == 0
  and .inherited_required_memory_write_request_field_count == 17
  and (.inherited_allowed_memory_write_operations | length) == 4
  and (.required_memory_write_approval_packet_fields | length) == 21
  and (.denied_memory_write_approval_packet_fixtures | length) == 8
  and (.denied_memory_write_approval_packet_fixtures | all(.packet_accepted == false and .memory_write_request_accepted == false and .memory_store_mutated == false and .activation_allowed == false))
  and (.denied_by_memory_write_approval_packet_gate | length) == 21
  and (.required_before_memory_write_approval_packet_acceptance | length) == 14
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory live mutation operator write approval packet gate passed"
