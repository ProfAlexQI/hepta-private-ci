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

RETENTION_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-retention-expiry-garbage-collection-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-retention-expiry-garbage-collection-denial-gate.sh
)"

retention_report_sha256="$(sha256_text "$RETENTION_JSON")"
export_query_observability_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-export-query-observability-denial:$retention_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"

jq -n -e \
  --argjson source "$RETENTION_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_retention_expiry_garbage_collection_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_retention_expiry_garbage_collection_denial_ready == true
    and $source.retention_expiry_gc_surface_count == 17
    and $source.retention_expiry_gc_attempt_count == 17
    and $source.retention_policy_recorded_count == 0
    and $source.retention_policy_persisted_count == 0
    and $source.retention_index_recorded_count == 0
    and $source.ttl_update_recorded_count == 0
    and $source.ttl_extension_recorded_count == 0
    and $source.expiry_recorded_count == 0
    and $source.expiry_persisted_count == 0
    and $source.expiry_scheduler_registered_count == 0
    and $source.expiry_timer_started_count == 0
    and $source.garbage_collection_scan_performed_count == 0
    and $source.garbage_collection_candidate_recorded_count == 0
    and $source.garbage_collection_decision_recorded_count == 0
    and $source.delete_performed_count == 0
    and $source.tombstone_recorded_count == 0
    and $source.archive_written_count == 0
    and $source.compaction_performed_count == 0
    and $source.retention_gc_acceptance_recorded_count == 0
    and $source.retention_gc_operator_approval_derived_count == 0
    and $source.retention_gc_activation_authority_derived_count == 0
    and $source.retention_gc_activation_command_derived_count == 0
    and $source.retention_gc_live_execution_allowed_count == 0
    and $source.operator_acceptance_recorded == false
    and $source.operator_approval_recorded == false
    and $source.activation_authority_derived == false
    and $source.activation_command_derived == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

export_query_observability_surfaces_json="$(
  jq -n '[
    "packet_receipt_query_registration_claim",
    "packet_receipt_query_result_claim",
    "packet_receipt_search_index_claim",
    "packet_receipt_export_snapshot_claim",
    "packet_receipt_export_file_claim",
    "packet_receipt_observability_metric_claim",
    "packet_receipt_observability_event_claim",
    "packet_receipt_dashboard_panel_claim",
    "packet_receipt_operator_summary_claim",
    "packet_receipt_readback_surface_claim",
    "packet_receipt_audit_view_claim",
    "packet_receipt_external_delivery_claim",
    "packet_receipt_completion_ack_view_claim",
    "packet_receipt_acceptance_view_claim",
    "packet_receipt_authority_view_claim",
    "packet_receipt_live_view_claim"
  ] | map({
    export_query_observability_surface: .,
    export_query_or_observability_attempted: true,
    query_registered: false,
    query_executed: false,
    query_result_recorded: false,
    query_result_persisted: false,
    search_index_recorded: false,
    search_index_persisted: false,
    export_requested: false,
    export_snapshot_recorded: false,
    export_snapshot_persisted: false,
    export_file_written: false,
    observability_metric_recorded: false,
    observability_event_recorded: false,
    dashboard_panel_recorded: false,
    operator_summary_recorded: false,
    readback_surface_recorded: false,
    audit_view_recorded: false,
    external_delivery_performed: false,
    completion_ack_recorded: false,
    acceptance_recorded: false,
    operator_approval_derived: false,
    activation_authority_derived: false,
    activation_command_derived: false,
    live_execution_allowed: false,
    export_query_observability_status: "export_query_observability_denied"
  })'
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_gate" \
  --arg retention_report_sha256 "$retention_report_sha256" \
  --arg export_query_observability_contract_hash_sha256 "$export_query_observability_contract_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$RETENTION_JSON" \
  --argjson surfaces "$export_query_observability_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_export_query_observability_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_v1",
    receipt_export_query_observability_mode:"non_persistent_receipts_cannot_create_query_export_observability_or_authority",
    source_packet_acceptance_receipt_retention_expiry_gc_gate:$source.gate,
    source_packet_acceptance_receipt_retention_expiry_gc_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_retention_expiry_garbage_collection_denial_ready,
    source_retention_report_sha256:$retention_report_sha256,
    source_retention_expiry_garbage_collection_contract_hash_sha256:$source.retention_expiry_garbage_collection_contract_hash_sha256,
    export_query_observability_contract_hash_sha256:$export_query_observability_contract_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_ready:true,
    source_retention_expiry_gc_surface_count:$source.retention_expiry_gc_surface_count,
    source_retention_expiry_gc_attempt_count:$source.retention_expiry_gc_attempt_count,
    source_retention_policy_recorded_count:$source.retention_policy_recorded_count,
    source_expiry_recorded_count:$source.expiry_recorded_count,
    source_garbage_collection_scan_performed_count:$source.garbage_collection_scan_performed_count,
    source_archive_written_count:$source.archive_written_count,
    source_compaction_performed_count:$source.compaction_performed_count,
    source_retention_gc_activation_authority_derived_count:$source.retention_gc_activation_authority_derived_count,
    export_query_observability_surface_count:($surfaces | length),
    export_query_observability_attempt_count:($surfaces | length),
    query_registered_count:0,
    query_executed_count:0,
    query_result_recorded_count:0,
    query_result_persisted_count:0,
    search_index_recorded_count:0,
    search_index_persisted_count:0,
    export_requested_count:0,
    export_snapshot_recorded_count:0,
    export_snapshot_persisted_count:0,
    export_file_written_count:0,
    observability_metric_recorded_count:0,
    observability_event_recorded_count:0,
    dashboard_panel_recorded_count:0,
    operator_summary_recorded_count:0,
    readback_surface_recorded_count:0,
    audit_view_recorded_count:0,
    external_delivery_performed_count:0,
    completion_ack_recorded_count:0,
    export_query_observability_acceptance_recorded_count:0,
    export_query_observability_operator_approval_derived_count:0,
    export_query_observability_activation_authority_derived_count:0,
    export_query_observability_activation_command_derived_count:0,
    export_query_observability_live_execution_allowed_count:0,
    export_query_observability_surfaces:$surfaces,
    denied_by_packet_receipt_export_query_observability:[
      "operator_readiness_packet_template_packet_receipt_query_registration_denied",
      "operator_readiness_packet_template_packet_receipt_query_execution_denied",
      "operator_readiness_packet_template_packet_receipt_query_result_recording_denied",
      "operator_readiness_packet_template_packet_receipt_search_index_recording_denied",
      "operator_readiness_packet_template_packet_receipt_export_request_denied",
      "operator_readiness_packet_template_packet_receipt_export_snapshot_recording_denied",
      "operator_readiness_packet_template_packet_receipt_export_file_write_denied",
      "operator_readiness_packet_template_packet_receipt_observability_metric_denied",
      "operator_readiness_packet_template_packet_receipt_observability_event_denied",
      "operator_readiness_packet_template_packet_receipt_dashboard_panel_denied",
      "operator_readiness_packet_template_packet_receipt_operator_summary_denied",
      "operator_readiness_packet_template_packet_receipt_readback_surface_denied",
      "operator_readiness_packet_template_packet_receipt_audit_view_denied",
      "operator_readiness_packet_template_packet_receipt_external_delivery_denied",
      "operator_readiness_packet_template_packet_receipt_completion_ack_view_denied",
      "operator_readiness_packet_template_packet_receipt_acceptance_from_view_denied",
      "operator_readiness_packet_template_packet_receipt_authority_from_view_denied",
      "operator_readiness_packet_template_packet_receipt_live_execution_from_view_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_denial_gate",
        status:"allowed_report_only_next_slice",
        persists_receipt:false,
        records_operator_acceptance:false,
        derives_activation_authority:false,
        queries_receipt:false,
        exports_receipt:false,
        records_observability:false,
        delivers_externally:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false
      }
    ],
    packet_template_recorded:false,
    packet_template_persisted:false,
    packet_assembly_performed:false,
    packet_accepted:false,
    packet_acceptance_receipt_recorded:false,
    packet_acceptance_receipt_persisted:false,
    packet_acceptance_receipt_replayed:false,
    packet_acceptance_receipt_ordering_recorded:false,
    packet_acceptance_receipt_cancellation_recorded:false,
    packet_acceptance_receipt_supersession_recorded:false,
    packet_acceptance_receipt_audit_trail_recorded:false,
    packet_acceptance_receipt_immutable_evidence_recorded:false,
    packet_acceptance_receipt_retention_policy_recorded:false,
    packet_acceptance_receipt_expiry_recorded:false,
    packet_acceptance_receipt_garbage_collection_scan_performed:false,
    packet_acceptance_receipt_query_registered:false,
    packet_acceptance_receipt_query_executed:false,
    packet_acceptance_receipt_query_result_recorded:false,
    packet_acceptance_receipt_search_index_recorded:false,
    packet_acceptance_receipt_export_snapshot_recorded:false,
    packet_acceptance_receipt_export_file_written:false,
    packet_acceptance_receipt_observability_metric_recorded:false,
    packet_acceptance_receipt_observability_event_recorded:false,
    packet_acceptance_receipt_dashboard_panel_recorded:false,
    packet_acceptance_receipt_operator_summary_recorded:false,
    packet_acceptance_receipt_readback_surface_recorded:false,
    packet_acceptance_receipt_external_delivery_performed:false,
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
      packet_acceptance_receipt_query_registered:false,
      packet_acceptance_receipt_query_executed:false,
      packet_acceptance_receipt_query_result_recorded:false,
      packet_acceptance_receipt_query_result_persisted:false,
      packet_acceptance_receipt_search_index_recorded:false,
      packet_acceptance_receipt_search_index_persisted:false,
      packet_acceptance_receipt_export_requested:false,
      packet_acceptance_receipt_export_snapshot_recorded:false,
      packet_acceptance_receipt_export_snapshot_persisted:false,
      packet_acceptance_receipt_export_file_written:false,
      packet_acceptance_receipt_observability_metric_recorded:false,
      packet_acceptance_receipt_observability_event_recorded:false,
      packet_acceptance_receipt_dashboard_panel_recorded:false,
      packet_acceptance_receipt_operator_summary_recorded:false,
      packet_acceptance_receipt_readback_surface_recorded:false,
      packet_acceptance_receipt_audit_view_recorded:false,
      packet_acceptance_receipt_external_delivery_performed:false,
      packet_acceptance_receipt_completion_ack_recorded:false,
      packet_acceptance_receipt_acceptance_recorded:false,
      packet_acceptance_receipt_authority_derived:false,
      packet_acceptance_receipt_live_execution_allowed:false,
      packet_acceptance_receipt_retention_policy_recorded:false,
      packet_acceptance_receipt_expiry_recorded:false,
      packet_acceptance_receipt_garbage_collection_scan_performed:false,
      packet_acceptance_receipt_audit_trail_recorded:false,
      packet_acceptance_receipt_immutable_evidence_recorded:false,
      packet_acceptance_receipt_recorded:false,
      packet_acceptance_receipt_persisted:false,
      packet_template_recorded:false,
      packet_template_persisted:false,
      packet_assembly_performed:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_ready == true
  and .source_packet_acceptance_receipt_retention_expiry_gc_ready == true
  and .source_retention_expiry_gc_surface_count == 17
  and .source_retention_policy_recorded_count == 0
  and .source_expiry_recorded_count == 0
  and .source_garbage_collection_scan_performed_count == 0
  and .source_archive_written_count == 0
  and .source_compaction_performed_count == 0
  and .source_retention_gc_activation_authority_derived_count == 0
  and .export_query_observability_surface_count == 16
  and .export_query_observability_attempt_count == 16
  and .query_registered_count == 0
  and .query_executed_count == 0
  and .query_result_recorded_count == 0
  and .query_result_persisted_count == 0
  and .search_index_recorded_count == 0
  and .search_index_persisted_count == 0
  and .export_requested_count == 0
  and .export_snapshot_recorded_count == 0
  and .export_snapshot_persisted_count == 0
  and .export_file_written_count == 0
  and .observability_metric_recorded_count == 0
  and .observability_event_recorded_count == 0
  and .dashboard_panel_recorded_count == 0
  and .operator_summary_recorded_count == 0
  and .readback_surface_recorded_count == 0
  and .audit_view_recorded_count == 0
  and .external_delivery_performed_count == 0
  and .completion_ack_recorded_count == 0
  and .export_query_observability_acceptance_recorded_count == 0
  and .export_query_observability_operator_approval_derived_count == 0
  and .export_query_observability_activation_authority_derived_count == 0
  and .export_query_observability_activation_command_derived_count == 0
  and .export_query_observability_live_execution_allowed_count == 0
  and (.export_query_observability_surfaces | all(
    .export_query_or_observability_attempted == true
    and .query_registered == false
    and .query_executed == false
    and .query_result_recorded == false
    and .search_index_recorded == false
    and .export_requested == false
    and .export_snapshot_recorded == false
    and .export_file_written == false
    and .observability_metric_recorded == false
    and .observability_event_recorded == false
    and .dashboard_panel_recorded == false
    and .operator_summary_recorded == false
    and .readback_surface_recorded == false
    and .audit_view_recorded == false
    and .external_delivery_performed == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .live_execution_allowed == false
    and .export_query_observability_status == "export_query_observability_denied"
  ))
  and (.denied_by_packet_receipt_export_query_observability | length) == 18
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_query_registered == false
  and .packet_acceptance_receipt_query_executed == false
  and .packet_acceptance_receipt_query_result_recorded == false
  and .packet_acceptance_receipt_export_snapshot_recorded == false
  and .packet_acceptance_receipt_export_file_written == false
  and .packet_acceptance_receipt_observability_metric_recorded == false
  and .packet_acceptance_receipt_observability_event_recorded == false
  and .packet_acceptance_receipt_dashboard_panel_recorded == false
  and .packet_acceptance_receipt_operator_summary_recorded == false
  and .packet_acceptance_receipt_readback_surface_recorded == false
  and .packet_acceptance_receipt_external_delivery_performed == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt export/query/observability denial gate passed"
