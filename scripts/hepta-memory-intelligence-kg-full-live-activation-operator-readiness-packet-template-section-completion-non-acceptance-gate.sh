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

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

FIELD_VALIDATION_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-field-validation-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-field-validation-denial-gate.sh
)"

field_validation_report_sha256="$(sha256_text "$FIELD_VALIDATION_JSON")"
section_completion_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-section-completion-non-acceptance:$field_validation_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"

jq -n -e \
  --argjson source "$FIELD_VALIDATION_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_field_validation_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_field_validation_denial_ready == true
    and $source.source_template_non_acceptance_ready == true
    and $source.source_operator_packet_section_count == 10
    and $source.source_operator_packet_required_field_count == 43
    and $source.required_field_count == 43
    and $source.field_validation_matrix_count == 43
    and $source.missing_field_count == 43
    and $source.present_field_count == 0
    and $source.captured_field_value_count == 0
    and $source.recorded_field_hash_count == 0
    and $source.shape_validated_field_count == 0
    and $source.accepted_field_count == 0
    and $source.authority_derived_field_count == 0
    and $source.live_execution_allowed_field_count == 0
    and $source.section_validation_count == 10
    and $source.operator_acceptance_recorded == false
    and $source.operator_approval_recorded == false
    and $source.activation_authority_derived == false
    and $source.activation_command_derived == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

section_completion_matrix_json="$(
  jq -n \
    --argjson fields "$FIELD_VALIDATION_JSON" \
    '
      $fields.required_field_validation_matrix
      | group_by(.section_id)
      | map({
          section_id: .[0].section_id,
          required_field_count: length,
          missing_field_count: (map(select(.field_missing == true)) | length),
          present_field_count: (map(select(.field_present == true)) | length),
          recorded_field_count: (map(select(.field_recorded == true)) | length),
          accepted_field_count: (map(select(.field_accepted == true)) | length),
          authority_derived_field_count: (map(select(.field_authority_derived == true)) | length),
          live_execution_allowed_field_count: (map(select(.field_live_execution_allowed == true)) | length),
          section_completion_attempted: true,
          section_complete: false,
          section_ready: false,
          section_recorded: false,
          section_persisted: false,
          section_accepted: false,
          section_operator_approval_derived: false,
          section_activation_authority_derived: false,
          section_live_execution_allowed: false,
          completion_status: "completion_denied_missing_required_fields",
          denial_reason: "operator_readiness_packet_template_section_completion_cannot_bypass_missing_fields"
        })
    '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_gate" \
  --arg field_validation_report_sha256 "$field_validation_report_sha256" \
  --arg section_completion_contract_hash_sha256 "$section_completion_contract_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$FIELD_VALIDATION_JSON" \
  --argjson sections "$section_completion_matrix_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    section_completion_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_v1",
    section_completion_mode:"section_completion_attempts_denied_no_acceptance_no_authority_no_live",
    source_field_validation_gate:$source.gate,
    source_field_validation_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_field_validation_denial_ready,
    source_field_validation_report_sha256:$field_validation_report_sha256,
    section_completion_contract_hash_sha256:$section_completion_contract_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_ready:true,
    source_operator_packet_section_count:$source.source_operator_packet_section_count,
    source_operator_packet_required_field_count:$source.source_operator_packet_required_field_count,
    source_required_field_count:$source.required_field_count,
    source_missing_field_count:$source.missing_field_count,
    section_completion_matrix_count:($sections | length),
    section_completion_attempt_count:($sections | length),
    section_complete_count:0,
    section_ready_count:0,
    section_recorded_count:0,
    section_persisted_count:0,
    section_accepted_count:0,
    section_operator_approval_derived_count:0,
    section_activation_authority_derived_count:0,
    section_live_execution_allowed_count:0,
    section_completion_matrix:$sections,
    denied_by_section_completion:[
      "operator_readiness_packet_template_section_completion_bypass_denied",
      "operator_readiness_packet_template_section_ready_promotion_denied",
      "operator_readiness_packet_template_section_recording_denied",
      "operator_readiness_packet_template_section_persistence_denied",
      "operator_readiness_packet_template_section_operator_acceptance_denied",
      "operator_readiness_packet_template_section_operator_approval_derivation_denied",
      "operator_readiness_packet_template_section_activation_authority_derivation_denied",
      "operator_readiness_packet_template_section_live_execution_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_assembly_non_acceptance_gate",
        status:"allowed_report_only_next_slice",
        records_operator_acceptance:false,
        derives_activation_authority:false,
        activates_live:false,
        persists_section_completion:false,
        mutates_memory_store:false,
        writes_kg:false
      }
    ],
    packet_template_recorded:false,
    packet_template_persisted:false,
    packet_template_materialized:false,
    packet_template_delivered:false,
    section_completion_recorded:false,
    section_completion_persisted:false,
    section_completion_materialized:false,
    section_completion_accepted:false,
    operator_acceptance_recorded:false,
    operator_approval_recorded:false,
    activation_authority_derived:false,
    activation_command_derived:false,
    activation_allowed:false,
    activation_performed:false,
    memory_store_write_performed:false,
    memory_store_mutated:false,
    hepta_intelligence_context_attached:false,
    prompt_preview_rendered:false,
    context_injection_performed:false,
    provider_invoked:false,
    model_invoked:false,
    external_kg_adapter_read_performed:false,
    external_adapter_client_constructed:false,
    network_call_performed:false,
    external_db_write_performed:false,
    live_kg_write_performed:false,
    credential_read:false,
    secret_file_read:false,
    install_executed:false,
    launchd_mutated:false,
    service_restarted:false,
    active_binary_mutated:false,
    public_release_claimed:false,
    public_ga_claimed:false,
    release_artifact_written:false,
    public_artifact_written:false,
    external_send_performed:false,
    side_effects:{
      section_completion_recorded:false,
      section_completion_persisted:false,
      section_completion_materialized:false,
      section_completion_accepted:false,
      section_ready_promoted:false,
      section_operator_approval_derived:false,
      section_activation_authority_derived:false,
      section_live_execution_allowed:false,
      packet_template_recorded:false,
      packet_template_persisted:false,
      packet_template_materialized:false,
      packet_template_delivered:false,
      operator_acceptance_recorded:false,
      operator_approval_recorded:false,
      activation_authority_derived:false,
      activation_command_derived:false,
      activation_allowed:false,
      activation_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      hepta_intelligence_context_attached:false,
      prompt_preview_rendered:false,
      context_injection_performed:false,
      provider_invoked:false,
      model_invoked:false,
      external_kg_adapter_read_performed:false,
      external_adapter_client_constructed:false,
      network_call_performed:false,
      external_db_write_performed:false,
      live_kg_write_performed:false,
      credential_read:false,
      secret_file_read:false,
      install_executed:false,
      launchd_mutated:false,
      service_restarted:false,
      active_binary_mutated:false,
      public_release_claimed:false,
      public_ga_claimed:false,
      release_artifact_written:false,
      public_artifact_written:false,
      external_send_performed:false,
      filesystem_written:false
    }
  }')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_ready == true
  and .source_field_validation_ready == true
  and .source_operator_packet_section_count == 10
  and .source_operator_packet_required_field_count == 43
  and .source_required_field_count == 43
  and .source_missing_field_count == 43
  and .section_completion_matrix_count == 10
  and .section_completion_attempt_count == 10
  and .section_complete_count == 0
  and .section_ready_count == 0
  and .section_recorded_count == 0
  and .section_persisted_count == 0
  and .section_accepted_count == 0
  and .section_operator_approval_derived_count == 0
  and .section_activation_authority_derived_count == 0
  and .section_live_execution_allowed_count == 0
  and (.section_completion_matrix | all(
    .required_field_count > 0
    and .missing_field_count == .required_field_count
    and .present_field_count == 0
    and .recorded_field_count == 0
    and .accepted_field_count == 0
    and .authority_derived_field_count == 0
    and .live_execution_allowed_field_count == 0
    and .section_completion_attempted == true
    and .section_complete == false
    and .section_ready == false
    and .section_recorded == false
    and .section_persisted == false
    and .section_accepted == false
    and .section_operator_approval_derived == false
    and .section_activation_authority_derived == false
    and .section_live_execution_allowed == false
    and .completion_status == "completion_denied_missing_required_fields"
  ))
  and (.denied_by_section_completion | length) == 8
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_template_recorded == false
  and .packet_template_persisted == false
  and .section_completion_recorded == false
  and .section_completion_persisted == false
  and .section_completion_accepted == false
  and .operator_acceptance_recorded == false
  and .operator_approval_recorded == false
  and .activation_authority_derived == false
  and .activation_command_derived == false
  and .activation_allowed == false
  and .activation_performed == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .hepta_intelligence_context_attached == false
  and .prompt_preview_rendered == false
  and .context_injection_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .external_kg_adapter_read_performed == false
  and .external_adapter_client_constructed == false
  and .network_call_performed == false
  and .external_db_write_performed == false
  and .live_kg_write_performed == false
  and .credential_read == false
  and .secret_file_read == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and .public_release_claimed == false
  and .public_ga_claimed == false
  and .release_artifact_written == false
  and .public_artifact_written == false
  and .external_send_performed == false
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template section completion non-acceptance gate passed"
