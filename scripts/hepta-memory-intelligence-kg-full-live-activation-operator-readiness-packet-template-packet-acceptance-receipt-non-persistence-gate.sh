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

PACKET_ASSEMBLY_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-assembly-non-acceptance-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-assembly-non-acceptance-gate.sh
)"

packet_assembly_report_sha256="$(sha256_text "$PACKET_ASSEMBLY_JSON")"
receipt_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-non-persistence:$packet_assembly_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"

jq -n -e \
  --argjson source "$PACKET_ASSEMBLY_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_assembly_non_acceptance_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_assembly_non_acceptance_ready == true
    and $source.source_section_completion_ready == true
    and $source.source_operator_packet_section_count == 10
    and $source.source_operator_packet_required_field_count == 43
    and $source.source_missing_field_count == 43
    and $source.source_section_completion_matrix_count == 10
    and $source.source_section_complete_count == 0
    and $source.source_section_ready_count == 0
    and $source.packet_assembly_attempt_count == 4
    and $source.packet_assembled_count == 0
    and $source.packet_complete_count == 0
    and $source.packet_ready_count == 0
    and $source.packet_recorded_count == 0
    and $source.packet_persisted_count == 0
    and $source.packet_accepted_count == 0
    and $source.packet_operator_approval_derived_count == 0
    and $source.packet_activation_authority_derived_count == 0
    and $source.packet_activation_command_derived_count == 0
    and $source.packet_live_execution_allowed_count == 0
    and $source.operator_acceptance_recorded == false
    and $source.operator_approval_recorded == false
    and $source.activation_authority_derived == false
    and $source.activation_command_derived == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

receipt_surfaces_json="$(
  jq -n '[
    "packet_assembly_denial_receipt",
    "packet_acceptance_attempt_receipt",
    "operator_summary_receipt",
    "packet_query_receipt",
    "packet_export_receipt",
    "packet_observability_receipt",
    "packet_completion_ack_receipt",
    "packet_authority_derivation_receipt"
  ] | map({
    receipt_surface: .,
    receipt_generated: true,
    receipt_recorded: false,
    receipt_persisted: false,
    receipt_materialized: false,
    receipt_indexed: false,
    receipt_queryable: false,
    receipt_exportable: false,
    receipt_observable: false,
    receipt_delivered: false,
    receipt_acceptance_recorded: false,
    receipt_operator_approval_derived: false,
    receipt_activation_authority_derived: false,
    receipt_activation_command_derived: false,
    receipt_live_execution_allowed: false,
    receipt_status: "non_persistent_report_only"
  })'
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_gate" \
  --arg packet_assembly_report_sha256 "$packet_assembly_report_sha256" \
  --arg receipt_contract_hash_sha256 "$receipt_contract_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$PACKET_ASSEMBLY_JSON" \
  --argjson receipts "$receipt_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    packet_acceptance_receipt_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_v1",
    packet_acceptance_receipt_mode:"denied_packet_assembly_receipts_are_report_only_no_persistence_no_acceptance_no_authority",
    source_packet_assembly_gate:$source.gate,
    source_packet_assembly_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_assembly_non_acceptance_ready,
    source_packet_assembly_report_sha256:$packet_assembly_report_sha256,
    receipt_contract_hash_sha256:$receipt_contract_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_ready:true,
    source_packet_assembly_attempt_count:$source.packet_assembly_attempt_count,
    source_packet_assembled_count:$source.packet_assembled_count,
    source_packet_accepted_count:$source.packet_accepted_count,
    source_packet_activation_authority_derived_count:$source.packet_activation_authority_derived_count,
    receipt_surface_count:($receipts | length),
    receipt_generated_count:($receipts | length),
    receipt_recorded_count:0,
    receipt_persisted_count:0,
    receipt_materialized_count:0,
    receipt_indexed_count:0,
    receipt_queryable_count:0,
    receipt_exportable_count:0,
    receipt_observable_count:0,
    receipt_delivered_count:0,
    receipt_acceptance_recorded_count:0,
    receipt_operator_approval_derived_count:0,
    receipt_activation_authority_derived_count:0,
    receipt_activation_command_derived_count:0,
    receipt_live_execution_allowed_count:0,
    receipt_surfaces:$receipts,
    denied_by_packet_acceptance_receipt:[
      "operator_readiness_packet_template_packet_receipt_recording_denied",
      "operator_readiness_packet_template_packet_receipt_persistence_denied",
      "operator_readiness_packet_template_packet_receipt_materialization_denied",
      "operator_readiness_packet_template_packet_receipt_indexing_denied",
      "operator_readiness_packet_template_packet_receipt_query_export_denied",
      "operator_readiness_packet_template_packet_receipt_observability_denied",
      "operator_readiness_packet_template_packet_receipt_delivery_denied",
      "operator_readiness_packet_template_packet_receipt_acceptance_denied",
      "operator_readiness_packet_template_packet_receipt_authority_derivation_denied",
      "operator_readiness_packet_template_packet_receipt_live_execution_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_replay_idempotency_denial_gate",
        status:"allowed_report_only_next_slice",
        persists_receipt:false,
        records_operator_acceptance:false,
        derives_activation_authority:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false
      }
    ],
    packet_template_recorded:false,
    packet_template_persisted:false,
    packet_assembly_performed:false,
    packet_assembly_recorded:false,
    packet_assembly_persisted:false,
    packet_complete:false,
    packet_ready:false,
    packet_accepted:false,
    packet_acceptance_receipt_recorded:false,
    packet_acceptance_receipt_persisted:false,
    packet_acceptance_receipt_materialized:false,
    packet_acceptance_receipt_indexed:false,
    packet_acceptance_receipt_delivered:false,
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
      packet_acceptance_receipt_recorded:false,
      packet_acceptance_receipt_persisted:false,
      packet_acceptance_receipt_materialized:false,
      packet_acceptance_receipt_indexed:false,
      packet_acceptance_receipt_queryable:false,
      packet_acceptance_receipt_exportable:false,
      packet_acceptance_receipt_observable:false,
      packet_acceptance_receipt_delivered:false,
      packet_acceptance_receipt_acceptance_recorded:false,
      packet_acceptance_receipt_authority_derived:false,
      packet_acceptance_receipt_live_execution_allowed:false,
      packet_template_recorded:false,
      packet_template_persisted:false,
      packet_assembly_performed:false,
      packet_assembly_recorded:false,
      packet_assembly_persisted:false,
      packet_ready_promoted:false,
      packet_acceptance_recorded:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_ready == true
  and .source_packet_assembly_ready == true
  and .source_packet_assembly_attempt_count == 4
  and .source_packet_assembled_count == 0
  and .source_packet_accepted_count == 0
  and .source_packet_activation_authority_derived_count == 0
  and .receipt_surface_count == 8
  and .receipt_generated_count == 8
  and .receipt_recorded_count == 0
  and .receipt_persisted_count == 0
  and .receipt_materialized_count == 0
  and .receipt_indexed_count == 0
  and .receipt_queryable_count == 0
  and .receipt_exportable_count == 0
  and .receipt_observable_count == 0
  and .receipt_delivered_count == 0
  and .receipt_acceptance_recorded_count == 0
  and .receipt_operator_approval_derived_count == 0
  and .receipt_activation_authority_derived_count == 0
  and .receipt_activation_command_derived_count == 0
  and .receipt_live_execution_allowed_count == 0
  and (.receipt_surfaces | all(
    .receipt_generated == true
    and .receipt_recorded == false
    and .receipt_persisted == false
    and .receipt_materialized == false
    and .receipt_indexed == false
    and .receipt_queryable == false
    and .receipt_exportable == false
    and .receipt_observable == false
    and .receipt_delivered == false
    and .receipt_acceptance_recorded == false
    and .receipt_operator_approval_derived == false
    and .receipt_activation_authority_derived == false
    and .receipt_activation_command_derived == false
    and .receipt_live_execution_allowed == false
    and .receipt_status == "non_persistent_report_only"
  ))
  and (.denied_by_packet_acceptance_receipt | length) == 10
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_template_recorded == false
  and .packet_template_persisted == false
  and .packet_assembly_performed == false
  and .packet_assembly_recorded == false
  and .packet_assembly_persisted == false
  and .packet_complete == false
  and .packet_ready == false
  and .packet_accepted == false
  and .packet_acceptance_receipt_recorded == false
  and .packet_acceptance_receipt_persisted == false
  and .packet_acceptance_receipt_materialized == false
  and .packet_acceptance_receipt_indexed == false
  and .packet_acceptance_receipt_delivered == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt non-persistence gate passed"
