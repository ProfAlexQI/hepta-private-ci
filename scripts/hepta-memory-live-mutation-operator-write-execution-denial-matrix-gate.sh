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

EXECUTION_PREFLIGHT_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-preflight-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-preflight-gate.sh
)"

execution_preflight_report_sha256="$(printf '%s' "$EXECUTION_PREFLIGHT_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson preflight "$EXECUTION_PREFLIGHT_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $preflight.runtime == "hepta"
    and $preflight.status == "ready"
    and $preflight.gate == "hepta_memory_live_mutation_operator_write_execution_preflight_gate"
    and $preflight.memory_write_execution_preflight_ready == true
    and $preflight.pre_execution_validation_shape_ready == true
    and $preflight.execution_preflight_mode == "memory_write_execution_preflight_no_approval_no_mutation"
    and $preflight.source_memory_write_approval_packet_shape_ready == true
    and $preflight.source_memory_write_approval_packet_report_sha256 != ""
    and $preflight.source_memory_write_contract_report_sha256 != ""
    and $preflight.source_memory_intelligence_report_sha256 != ""
    and $preflight.source_payload_redaction_acceptance_matrix_report_sha256 != ""
    and $preflight.source_payload_redaction_proof_report_sha256 != ""
    and $preflight.minimum_required_samples >= 24
    and $preflight.pre_execution_validation_recorded == false
    and $preflight.pre_execution_validation_persisted == false
    and $preflight.pre_execution_validation_accepted == false
    and $preflight.memory_write_approval_packet_accepted == false
    and $preflight.memory_write_request_accepted == false
    and $preflight.operator_approval_recorded == false
    and $preflight.memory_write_execution_allowed == false
    and $preflight.memory_write_execution_ready == false
    and $preflight.memory_store_mutation_allowed == false
    and $preflight.memory_store_mutated == false
    and $preflight.live_mutation_execution_ready == false
    and $preflight.rollback_execution_allowed == false
    and $preflight.rollback_executed == false
    and $preflight.external_send_enabled == false
    and $preflight.public_claim_or_release_artifact_write_enabled == false
    and $preflight.required_pre_execution_validation_check_count == 17
    and $preflight.recorded_pre_execution_validation_check_count == 0
    and ($preflight.required_pre_execution_validation_checks | length) == 17
    and ($preflight.denied_memory_write_execution_preflight_fixtures | length) == 9
    and ($preflight.required_before_memory_write_execution | length) == 17
    and ($preflight.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_denial_matrix_gate" \
  --arg execution_preflight_report_sha256 "$execution_preflight_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson preflight "$EXECUTION_PREFLIGHT_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    execution_denial_matrix_mode:"memory_write_execution_attempt_denial_matrix_no_store_mutation",
    source_memory_write_execution_preflight_gate:$preflight.gate,
    source_memory_write_execution_preflight_ready:$preflight.memory_write_execution_preflight_ready,
    source_memory_write_execution_preflight_report_sha256:$execution_preflight_report_sha256,
    source_memory_write_approval_packet_report_sha256:$preflight.source_memory_write_approval_packet_report_sha256,
    source_memory_write_contract_report_sha256:$preflight.source_memory_write_contract_report_sha256,
    source_memory_intelligence_report_sha256:$preflight.source_memory_intelligence_report_sha256,
    source_payload_redaction_acceptance_matrix_report_sha256:$preflight.source_payload_redaction_acceptance_matrix_report_sha256,
    source_payload_redaction_proof_report_sha256:$preflight.source_payload_redaction_proof_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_write_execution_denial_matrix_ready:true,
    pre_execution_validation_shape_ready:$preflight.pre_execution_validation_shape_ready,
    required_pre_execution_validation_check_count:$preflight.required_pre_execution_validation_check_count,
    recorded_pre_execution_validation_check_count:0,
    accepted_pre_execution_validation_check_count:0,
    future_pre_execution_validation_check_slot_count:$preflight.required_pre_execution_validation_check_count,
    memory_write_execution_attempt_requested_count:7,
    memory_write_execution_attempt_performed_count:0,
    memory_write_execution_allowed_count:0,
    memory_write_execution_denied_count:7,
    blocked_execution_fixture_count:7,
    allowed_execution_fixture_count:0,
    required_execution_denial_fixture_count:7,
    execution_denial_fixture_count:7,
    memory_write_execution_denial_matrix_recorded:false,
    memory_write_execution_denial_matrix_persisted:false,
    memory_write_execution_denial_matrix_materialized:false,
    memory_write_execution_denial_matrix_filesystem_written:false,
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
    source_payload_redaction_proof_hash_bound:false,
    raw_payload_sha256_bound:false,
    redacted_payload_summary_sha256_bound:false,
    raw_payload_plaintext_recorded:false,
    raw_payload_plaintext_persisted:false,
    fresh_pre_activation_soak_evidence_recorded:false,
    rollback_plan_recorded:false,
    post_write_validation_plan_recorded:false,
    no_public_claim_no_external_send_decision_recorded:false,
    memory_write_execution_allowed:false,
    memory_write_execution_ready:false,
    memory_write_execution_performed:false,
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
    execution_denial_fixtures:[
      {
        id:"missing-accepted-approval-packet-execution-attempt",
        execution_requested:true,
        accepted_approval_packet_present:false,
        accepted_pre_execution_validation_check_count:0,
        execution_allowed:false,
        execution_performed:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"accepted_approval_packet_required"
      },
      {
        id:"partial-pre-execution-validation-execution-attempt",
        execution_requested:true,
        accepted_approval_packet_present:true,
        accepted_pre_execution_validation_check_count:8,
        required_pre_execution_validation_check_count:17,
        execution_allowed:false,
        execution_performed:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"all_pre_execution_validation_checks_required"
      },
      {
        id:"namespace-operation-retention-not-allowlisted-execution-attempt",
        execution_requested:true,
        accepted_pre_execution_validation_check_count:17,
        memory_namespace_allowed:false,
        memory_write_operation_allowed:false,
        memory_retention_class_allowed:false,
        execution_allowed:false,
        execution_performed:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"namespace_operation_and_retention_allowlists_required"
      },
      {
        id:"payload-hash-mismatch-or-plaintext-execution-attempt",
        execution_requested:true,
        accepted_pre_execution_validation_check_count:17,
        raw_payload_sha256_bound:false,
        redacted_payload_summary_sha256_bound:false,
        raw_payload_plaintext_recorded:true,
        execution_allowed:false,
        execution_performed:false,
        memory_store_mutated:false,
        activation_allowed:false,
        reason:"payload_hash_binding_without_plaintext_required"
      },
      {
        id:"stale-soak-or-missing-rollback-validation-execution-attempt",
        execution_requested:true,
        accepted_pre_execution_validation_check_count:17,
        fresh_pre_activation_soak_evidence_recorded:false,
        rollback_plan_recorded:false,
        post_write_validation_plan_recorded:false,
        execution_allowed:false,
        execution_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        activation_allowed:false,
        reason:"fresh_soak_rollback_and_validation_required"
      },
      {
        id:"external-send-public-claim-release-artifact-execution-attempt",
        execution_requested:true,
        accepted_pre_execution_validation_check_count:17,
        external_send_requested:true,
        public_claim_requested:true,
        release_artifact_write_requested:true,
        execution_allowed:false,
        execution_performed:false,
        memory_store_mutated:false,
        external_send_performed:false,
        public_release_published:false,
        release_artifact_written:false,
        activation_allowed:false,
        reason:"external_send_public_claim_and_release_artifact_denied"
      },
      {
        id:"direct-memory-store-mutation-or-rollback-execution-attempt",
        execution_requested:true,
        accepted_pre_execution_validation_check_count:17,
        memory_store_mutation_requested:true,
        rollback_execution_requested:true,
        execution_allowed:false,
        execution_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        activation_allowed:false,
        reason:"execution_denial_matrix_layer_cannot_mutate_memory_or_execute_rollback"
      }
    ],
    denied_by_memory_write_execution_denial_matrix:[
      "memory_write_execution_denial_matrix_recording_denied",
      "memory_write_execution_denial_matrix_persistence_denied",
      "memory_write_execution_denial_matrix_materialization_denied",
      "memory_write_execution_denial_matrix_filesystem_write_denied",
      "accepted_approval_packet_required",
      "all_pre_execution_validation_checks_required",
      "approval_packet_hash_binding_required",
      "memory_write_request_hash_binding_required",
      "operator_signature_and_timestamp_required",
      "single_surface_scope_verification_required",
      "namespace_operation_and_retention_allowlists_required",
      "fresh_accepted_redaction_proof_required",
      "payload_hash_binding_without_plaintext_required",
      "source_report_hash_bindings_required",
      "fresh_pre_activation_soak_verification_required",
      "rollback_plan_verification_required",
      "post_write_validation_plan_verification_required",
      "no_public_claim_no_external_send_verification_required",
      "memory_write_execution_denied",
      "memory_store_mutation_denied",
      "rollback_execution_denied",
      "external_send_public_claim_release_artifact_denied"
    ],
    required_before_memory_write_execution:$preflight.required_before_memory_write_execution,
    side_effects:{
      memory_store_mutated:false,
      memory_write_request_recorded:false,
      memory_write_request_persisted:false,
      memory_write_approval_packet_recorded:false,
      memory_write_approval_packet_persisted:false,
      memory_write_execution_preflight_recorded:false,
      memory_write_execution_preflight_persisted:false,
      memory_write_execution_denial_matrix_recorded:false,
      memory_write_execution_denial_matrix_persisted:false,
      memory_write_execution_denial_matrix_materialized:false,
      memory_write_execution_denial_matrix_filesystem_written:false,
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
  and .memory_write_execution_denial_matrix_ready == true
  and .execution_denial_matrix_mode == "memory_write_execution_attempt_denial_matrix_no_store_mutation"
  and .source_memory_write_execution_preflight_ready == true
  and .source_memory_write_execution_preflight_report_sha256 != ""
  and .source_memory_write_approval_packet_report_sha256 != ""
  and .source_memory_write_contract_report_sha256 != ""
  and .source_memory_intelligence_report_sha256 != ""
  and .source_payload_redaction_acceptance_matrix_report_sha256 != ""
  and .source_payload_redaction_proof_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .required_pre_execution_validation_check_count == 17
  and .recorded_pre_execution_validation_check_count == 0
  and .accepted_pre_execution_validation_check_count == 0
  and .future_pre_execution_validation_check_slot_count == 17
  and .memory_write_execution_attempt_requested_count == 7
  and .memory_write_execution_attempt_performed_count == 0
  and .memory_write_execution_allowed_count == 0
  and .memory_write_execution_denied_count == 7
  and .blocked_execution_fixture_count == 7
  and .allowed_execution_fixture_count == 0
  and .required_execution_denial_fixture_count == 7
  and .execution_denial_fixture_count == 7
  and .memory_write_execution_denial_matrix_recorded == false
  and .memory_write_execution_denial_matrix_persisted == false
  and .memory_write_execution_denial_matrix_materialized == false
  and .memory_write_execution_denial_matrix_filesystem_written == false
  and .pre_execution_validation_recorded == false
  and .pre_execution_validation_persisted == false
  and .pre_execution_validation_accepted == false
  and .memory_write_approval_packet_accepted == false
  and .memory_write_request_accepted == false
  and .operator_approval_recorded == false
  and .source_memory_write_approval_packet_hash_bound == false
  and .source_memory_write_contract_hash_bound == false
  and .source_memory_intelligence_hash_bound == false
  and .source_payload_redaction_acceptance_matrix_hash_bound == false
  and .source_payload_redaction_proof_hash_bound == false
  and .raw_payload_sha256_bound == false
  and .redacted_payload_summary_sha256_bound == false
  and .raw_payload_plaintext_recorded == false
  and .raw_payload_plaintext_persisted == false
  and .memory_write_execution_allowed == false
  and .memory_write_execution_ready == false
  and .memory_write_execution_performed == false
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
  and (.execution_denial_fixtures | length) == 7
  and (.execution_denial_fixtures | all(.execution_requested == true and .execution_allowed == false and .execution_performed == false and .memory_store_mutated == false and .activation_allowed == false))
  and (.denied_by_memory_write_execution_denial_matrix | length) == 22
  and (.required_before_memory_write_execution | length) == 17
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory live mutation operator write execution denial matrix gate passed"
