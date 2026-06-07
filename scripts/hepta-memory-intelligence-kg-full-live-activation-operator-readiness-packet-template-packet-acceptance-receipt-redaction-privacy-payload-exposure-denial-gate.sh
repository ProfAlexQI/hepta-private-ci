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

VIEW_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-export-query-observability-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-export-query-observability-denial-gate.sh
)"

view_report_sha256="$(sha256_text "$VIEW_JSON")"
redaction_privacy_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-redaction-privacy-payload-exposure-denial:$view_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"

jq -n -e \
  --argjson source "$VIEW_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_ready == true
    and $source.export_query_observability_surface_count == 16
    and $source.export_query_observability_attempt_count == 16
    and $source.query_registered_count == 0
    and $source.query_executed_count == 0
    and $source.query_result_recorded_count == 0
    and $source.export_requested_count == 0
    and $source.export_snapshot_recorded_count == 0
    and $source.export_file_written_count == 0
    and $source.observability_metric_recorded_count == 0
    and $source.observability_event_recorded_count == 0
    and $source.dashboard_panel_recorded_count == 0
    and $source.operator_summary_recorded_count == 0
    and $source.readback_surface_recorded_count == 0
    and $source.external_delivery_performed_count == 0
    and $source.export_query_observability_acceptance_recorded_count == 0
    and $source.export_query_observability_operator_approval_derived_count == 0
    and $source.export_query_observability_activation_authority_derived_count == 0
    and $source.export_query_observability_activation_command_derived_count == 0
    and $source.export_query_observability_live_execution_allowed_count == 0
    and $source.operator_acceptance_recorded == false
    and $source.operator_approval_recorded == false
    and $source.activation_authority_derived == false
    and $source.activation_command_derived == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

redaction_privacy_surfaces_json="$(
  jq -n '[
    "packet_receipt_redacted_payload_preview_claim",
    "packet_receipt_payload_hash_preview_claim",
    "packet_receipt_payload_diff_claim",
    "packet_receipt_readback_text_claim",
    "packet_receipt_operator_summary_text_claim",
    "packet_receipt_privacy_review_claim",
    "packet_receipt_secret_scan_claim",
    "packet_receipt_pii_scan_claim",
    "packet_receipt_raw_payload_inspection_claim",
    "packet_receipt_plaintext_materialization_claim",
    "packet_receipt_redaction_bypass_claim",
    "packet_receipt_hash_to_payload_link_claim",
    "packet_receipt_external_redaction_review_claim",
    "packet_receipt_privacy_acceptance_claim",
    "packet_receipt_authority_from_redaction_claim",
    "packet_receipt_live_from_privacy_claim"
  ] | map({
    redaction_privacy_surface: .,
    redaction_privacy_or_payload_exposure_attempted: true,
    redacted_payload_preview_recorded: false,
    payload_hash_preview_recorded: false,
    payload_diff_recorded: false,
    readback_text_recorded: false,
    operator_summary_text_recorded: false,
    privacy_review_recorded: false,
    privacy_review_persisted: false,
    secret_scan_performed: false,
    pii_scan_performed: false,
    raw_payload_inspected: false,
    plaintext_materialized: false,
    redaction_bypass_allowed: false,
    hash_to_payload_link_recorded: false,
    external_redaction_review_performed: false,
    privacy_acceptance_recorded: false,
    acceptance_recorded: false,
    operator_approval_derived: false,
    activation_authority_derived: false,
    activation_command_derived: false,
    live_execution_allowed: false,
    redaction_privacy_status: "redaction_privacy_payload_exposure_denied"
  })'
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_gate" \
  --arg view_report_sha256 "$view_report_sha256" \
  --arg redaction_privacy_contract_hash_sha256 "$redaction_privacy_contract_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$VIEW_JSON" \
  --argjson surfaces "$redaction_privacy_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_redaction_privacy_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_v1",
    receipt_redaction_privacy_mode:"non_persistent_receipts_cannot_expose_payload_or_create_privacy_authority",
    source_packet_acceptance_receipt_export_query_observability_gate:$source.gate,
    source_packet_acceptance_receipt_export_query_observability_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_ready,
    source_view_report_sha256:$view_report_sha256,
    source_export_query_observability_contract_hash_sha256:$source.export_query_observability_contract_hash_sha256,
    redaction_privacy_payload_exposure_contract_hash_sha256:$redaction_privacy_contract_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_ready:true,
    source_export_query_observability_surface_count:$source.export_query_observability_surface_count,
    source_query_registered_count:$source.query_registered_count,
    source_export_snapshot_recorded_count:$source.export_snapshot_recorded_count,
    source_observability_metric_recorded_count:$source.observability_metric_recorded_count,
    source_operator_summary_recorded_count:$source.operator_summary_recorded_count,
    source_readback_surface_recorded_count:$source.readback_surface_recorded_count,
    source_export_query_observability_activation_authority_derived_count:$source.export_query_observability_activation_authority_derived_count,
    redaction_privacy_surface_count:($surfaces | length),
    redaction_privacy_attempt_count:($surfaces | length),
    redacted_payload_preview_recorded_count:0,
    payload_hash_preview_recorded_count:0,
    payload_diff_recorded_count:0,
    readback_text_recorded_count:0,
    operator_summary_text_recorded_count:0,
    privacy_review_recorded_count:0,
    privacy_review_persisted_count:0,
    secret_scan_performed_count:0,
    pii_scan_performed_count:0,
    raw_payload_inspected_count:0,
    plaintext_materialized_count:0,
    redaction_bypass_allowed_count:0,
    hash_to_payload_link_recorded_count:0,
    external_redaction_review_performed_count:0,
    privacy_acceptance_recorded_count:0,
    redaction_privacy_acceptance_recorded_count:0,
    redaction_privacy_operator_approval_derived_count:0,
    redaction_privacy_activation_authority_derived_count:0,
    redaction_privacy_activation_command_derived_count:0,
    redaction_privacy_live_execution_allowed_count:0,
    redaction_privacy_surfaces:$surfaces,
    denied_by_packet_receipt_redaction_privacy:[
      "operator_readiness_packet_template_packet_receipt_redacted_payload_preview_denied",
      "operator_readiness_packet_template_packet_receipt_payload_hash_preview_denied",
      "operator_readiness_packet_template_packet_receipt_payload_diff_denied",
      "operator_readiness_packet_template_packet_receipt_readback_text_denied",
      "operator_readiness_packet_template_packet_receipt_operator_summary_text_denied",
      "operator_readiness_packet_template_packet_receipt_privacy_review_recording_denied",
      "operator_readiness_packet_template_packet_receipt_privacy_review_persistence_denied",
      "operator_readiness_packet_template_packet_receipt_secret_scan_denied",
      "operator_readiness_packet_template_packet_receipt_pii_scan_denied",
      "operator_readiness_packet_template_packet_receipt_raw_payload_inspection_denied",
      "operator_readiness_packet_template_packet_receipt_plaintext_materialization_denied",
      "operator_readiness_packet_template_packet_receipt_redaction_bypass_denied",
      "operator_readiness_packet_template_packet_receipt_hash_to_payload_link_denied",
      "operator_readiness_packet_template_packet_receipt_external_redaction_review_denied",
      "operator_readiness_packet_template_packet_receipt_privacy_acceptance_denied",
      "operator_readiness_packet_template_packet_receipt_authority_from_redaction_denied",
      "operator_readiness_packet_template_packet_receipt_live_execution_from_privacy_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_gate",
        status:"allowed_report_only_next_slice",
        exposes_payload:false,
        records_privacy_review:false,
        performs_secret_scan:false,
        records_operator_acceptance:false,
        derives_activation_authority:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false
      }
    ],
    packet_acceptance_receipt_recorded:false,
    packet_acceptance_receipt_persisted:false,
    packet_acceptance_receipt_query_registered:false,
    packet_acceptance_receipt_export_file_written:false,
    packet_acceptance_receipt_observability_metric_recorded:false,
    packet_acceptance_receipt_redacted_payload_preview_recorded:false,
    packet_acceptance_receipt_payload_hash_preview_recorded:false,
    packet_acceptance_receipt_readback_text_recorded:false,
    packet_acceptance_receipt_operator_summary_text_recorded:false,
    packet_acceptance_receipt_privacy_review_recorded:false,
    packet_acceptance_receipt_secret_scan_performed:false,
    packet_acceptance_receipt_pii_scan_performed:false,
    packet_acceptance_receipt_raw_payload_inspected:false,
    packet_acceptance_receipt_plaintext_materialized:false,
    packet_acceptance_receipt_redaction_bypass_allowed:false,
    packet_acceptance_receipt_hash_to_payload_link_recorded:false,
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
      packet_acceptance_receipt_redacted_payload_preview_recorded:false,
      packet_acceptance_receipt_payload_hash_preview_recorded:false,
      packet_acceptance_receipt_payload_diff_recorded:false,
      packet_acceptance_receipt_readback_text_recorded:false,
      packet_acceptance_receipt_operator_summary_text_recorded:false,
      packet_acceptance_receipt_privacy_review_recorded:false,
      packet_acceptance_receipt_privacy_review_persisted:false,
      packet_acceptance_receipt_secret_scan_performed:false,
      packet_acceptance_receipt_pii_scan_performed:false,
      packet_acceptance_receipt_raw_payload_inspected:false,
      packet_acceptance_receipt_plaintext_materialized:false,
      packet_acceptance_receipt_redaction_bypass_allowed:false,
      packet_acceptance_receipt_hash_to_payload_link_recorded:false,
      packet_acceptance_receipt_external_redaction_review_performed:false,
      packet_acceptance_receipt_privacy_acceptance_recorded:false,
      packet_acceptance_receipt_acceptance_recorded:false,
      packet_acceptance_receipt_authority_derived:false,
      packet_acceptance_receipt_live_execution_allowed:false,
      packet_acceptance_receipt_query_registered:false,
      packet_acceptance_receipt_export_file_written:false,
      packet_acceptance_receipt_observability_metric_recorded:false,
      packet_acceptance_receipt_recorded:false,
      packet_acceptance_receipt_persisted:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_ready == true
  and .source_packet_acceptance_receipt_export_query_observability_ready == true
  and .source_export_query_observability_surface_count == 16
  and .source_query_registered_count == 0
  and .source_export_snapshot_recorded_count == 0
  and .source_observability_metric_recorded_count == 0
  and .source_operator_summary_recorded_count == 0
  and .source_readback_surface_recorded_count == 0
  and .source_export_query_observability_activation_authority_derived_count == 0
  and .redaction_privacy_surface_count == 16
  and .redaction_privacy_attempt_count == 16
  and .redacted_payload_preview_recorded_count == 0
  and .payload_hash_preview_recorded_count == 0
  and .payload_diff_recorded_count == 0
  and .readback_text_recorded_count == 0
  and .operator_summary_text_recorded_count == 0
  and .privacy_review_recorded_count == 0
  and .privacy_review_persisted_count == 0
  and .secret_scan_performed_count == 0
  and .pii_scan_performed_count == 0
  and .raw_payload_inspected_count == 0
  and .plaintext_materialized_count == 0
  and .redaction_bypass_allowed_count == 0
  and .hash_to_payload_link_recorded_count == 0
  and .external_redaction_review_performed_count == 0
  and .privacy_acceptance_recorded_count == 0
  and .redaction_privacy_acceptance_recorded_count == 0
  and .redaction_privacy_operator_approval_derived_count == 0
  and .redaction_privacy_activation_authority_derived_count == 0
  and .redaction_privacy_activation_command_derived_count == 0
  and .redaction_privacy_live_execution_allowed_count == 0
  and (.redaction_privacy_surfaces | all(
    .redaction_privacy_or_payload_exposure_attempted == true
    and .redacted_payload_preview_recorded == false
    and .payload_hash_preview_recorded == false
    and .payload_diff_recorded == false
    and .readback_text_recorded == false
    and .operator_summary_text_recorded == false
    and .privacy_review_recorded == false
    and .secret_scan_performed == false
    and .pii_scan_performed == false
    and .raw_payload_inspected == false
    and .plaintext_materialized == false
    and .redaction_bypass_allowed == false
    and .hash_to_payload_link_recorded == false
    and .external_redaction_review_performed == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .live_execution_allowed == false
    and .redaction_privacy_status == "redaction_privacy_payload_exposure_denied"
  ))
  and (.denied_by_packet_receipt_redaction_privacy | length) == 17
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_redacted_payload_preview_recorded == false
  and .packet_acceptance_receipt_payload_hash_preview_recorded == false
  and .packet_acceptance_receipt_readback_text_recorded == false
  and .packet_acceptance_receipt_operator_summary_text_recorded == false
  and .packet_acceptance_receipt_privacy_review_recorded == false
  and .packet_acceptance_receipt_secret_scan_performed == false
  and .packet_acceptance_receipt_raw_payload_inspected == false
  and .packet_acceptance_receipt_plaintext_materialized == false
  and .packet_acceptance_receipt_hash_to_payload_link_recorded == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt redaction/privacy/payload-exposure denial gate passed"
