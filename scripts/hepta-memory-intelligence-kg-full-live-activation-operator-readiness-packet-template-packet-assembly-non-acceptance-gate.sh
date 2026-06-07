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

SECTION_COMPLETION_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-section-completion-non-acceptance-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-section-completion-non-acceptance-gate.sh
)"

section_completion_report_sha256="$(sha256_text "$SECTION_COMPLETION_JSON")"
packet_assembly_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-assembly-non-acceptance:$section_completion_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"

jq -n -e \
  --argjson source "$SECTION_COMPLETION_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_ready == true
    and $source.source_field_validation_ready == true
    and $source.source_operator_packet_section_count == 10
    and $source.source_operator_packet_required_field_count == 43
    and $source.source_required_field_count == 43
    and $source.source_missing_field_count == 43
    and $source.section_completion_matrix_count == 10
    and $source.section_completion_attempt_count == 10
    and $source.section_complete_count == 0
    and $source.section_ready_count == 0
    and $source.section_recorded_count == 0
    and $source.section_persisted_count == 0
    and $source.section_accepted_count == 0
    and $source.section_operator_approval_derived_count == 0
    and $source.section_activation_authority_derived_count == 0
    and $source.section_live_execution_allowed_count == 0
    and $source.operator_acceptance_recorded == false
    and $source.operator_approval_recorded == false
    and $source.activation_authority_derived == false
    and $source.activation_command_derived == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

packet_assembly_attempts_json="$(
  jq -n \
    --argjson source "$SECTION_COMPLETION_JSON" \
    '[
      {
        attempt_id:"assemble_all_sections_incomplete_packet",
        attempted_section_count:$source.section_completion_matrix_count,
        complete_section_count:$source.section_complete_count,
        missing_section_count:$source.section_completion_matrix_count,
        assembled:false,
        accepted:false,
        operator_approval_derived:false,
        activation_authority_derived:false,
        live_execution_allowed:false,
        status:"assembly_denied_incomplete_sections"
      },
      {
        attempt_id:"assemble_ready_sections_packet",
        attempted_section_count:$source.section_ready_count,
        complete_section_count:0,
        missing_section_count:$source.section_completion_matrix_count,
        assembled:false,
        accepted:false,
        operator_approval_derived:false,
        activation_authority_derived:false,
        live_execution_allowed:false,
        status:"assembly_denied_no_ready_sections"
      },
      {
        attempt_id:"assemble_recorded_sections_packet",
        attempted_section_count:$source.section_recorded_count,
        complete_section_count:0,
        missing_section_count:$source.section_completion_matrix_count,
        assembled:false,
        accepted:false,
        operator_approval_derived:false,
        activation_authority_derived:false,
        live_execution_allowed:false,
        status:"assembly_denied_no_recorded_sections"
      },
      {
        attempt_id:"assemble_accepted_sections_packet",
        attempted_section_count:$source.section_accepted_count,
        complete_section_count:0,
        missing_section_count:$source.section_completion_matrix_count,
        assembled:false,
        accepted:false,
        operator_approval_derived:false,
        activation_authority_derived:false,
        live_execution_allowed:false,
        status:"assembly_denied_no_accepted_sections"
      }
    ]'
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_assembly_non_acceptance_gate" \
  --arg section_completion_report_sha256 "$section_completion_report_sha256" \
  --arg packet_assembly_contract_hash_sha256 "$packet_assembly_contract_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$SECTION_COMPLETION_JSON" \
  --argjson assembly_attempts "$packet_assembly_attempts_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    packet_assembly_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_assembly_non_acceptance_v1",
    packet_assembly_mode:"incomplete_sections_cannot_assemble_accept_or_authorize_live",
    source_section_completion_gate:$source.gate,
    source_section_completion_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_ready,
    source_section_completion_report_sha256:$section_completion_report_sha256,
    packet_assembly_contract_hash_sha256:$packet_assembly_contract_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_assembly_non_acceptance_ready:true,
    source_operator_packet_section_count:$source.source_operator_packet_section_count,
    source_operator_packet_required_field_count:$source.source_operator_packet_required_field_count,
    source_missing_field_count:$source.source_missing_field_count,
    source_section_completion_matrix_count:$source.section_completion_matrix_count,
    source_section_complete_count:$source.section_complete_count,
    source_section_ready_count:$source.section_ready_count,
    packet_assembly_attempt_count:($assembly_attempts | length),
    packet_assembled_count:0,
    packet_complete_count:0,
    packet_ready_count:0,
    packet_recorded_count:0,
    packet_persisted_count:0,
    packet_accepted_count:0,
    packet_operator_approval_derived_count:0,
    packet_activation_authority_derived_count:0,
    packet_activation_command_derived_count:0,
    packet_live_execution_allowed_count:0,
    packet_assembly_attempts:$assembly_attempts,
    denied_by_packet_assembly:[
      "operator_readiness_packet_template_incomplete_section_assembly_denied",
      "operator_readiness_packet_template_packet_ready_promotion_denied",
      "operator_readiness_packet_template_packet_recording_denied",
      "operator_readiness_packet_template_packet_persistence_denied",
      "operator_readiness_packet_template_packet_acceptance_denied",
      "operator_readiness_packet_template_packet_operator_approval_derivation_denied",
      "operator_readiness_packet_template_packet_activation_authority_derivation_denied",
      "operator_readiness_packet_template_packet_activation_command_derivation_denied",
      "operator_readiness_packet_template_packet_live_execution_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_gate",
        status:"allowed_report_only_next_slice",
        records_operator_acceptance:false,
        persists_packet:false,
        derives_activation_authority:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false
      }
    ],
    packet_template_recorded:false,
    packet_template_persisted:false,
    packet_template_materialized:false,
    packet_template_delivered:false,
    packet_assembly_performed:false,
    packet_assembly_recorded:false,
    packet_assembly_persisted:false,
    packet_complete:false,
    packet_ready:false,
    packet_accepted:false,
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
      packet_assembly_performed:false,
      packet_assembly_recorded:false,
      packet_assembly_persisted:false,
      packet_ready_promoted:false,
      packet_acceptance_recorded:false,
      packet_operator_approval_derived:false,
      packet_activation_authority_derived:false,
      packet_activation_command_derived:false,
      packet_live_execution_allowed:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_assembly_non_acceptance_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_assembly_non_acceptance_ready == true
  and .source_section_completion_ready == true
  and .source_operator_packet_section_count == 10
  and .source_operator_packet_required_field_count == 43
  and .source_missing_field_count == 43
  and .source_section_completion_matrix_count == 10
  and .source_section_complete_count == 0
  and .source_section_ready_count == 0
  and .packet_assembly_attempt_count == 4
  and .packet_assembled_count == 0
  and .packet_complete_count == 0
  and .packet_ready_count == 0
  and .packet_recorded_count == 0
  and .packet_persisted_count == 0
  and .packet_accepted_count == 0
  and .packet_operator_approval_derived_count == 0
  and .packet_activation_authority_derived_count == 0
  and .packet_activation_command_derived_count == 0
  and .packet_live_execution_allowed_count == 0
  and (.packet_assembly_attempts | all(
    .assembled == false
    and .accepted == false
    and .operator_approval_derived == false
    and .activation_authority_derived == false
    and .live_execution_allowed == false
  ))
  and (.denied_by_packet_assembly | length) == 9
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_template_recorded == false
  and .packet_template_persisted == false
  and .packet_assembly_performed == false
  and .packet_assembly_recorded == false
  and .packet_assembly_persisted == false
  and .packet_complete == false
  and .packet_ready == false
  and .packet_accepted == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet assembly non-acceptance gate passed"
