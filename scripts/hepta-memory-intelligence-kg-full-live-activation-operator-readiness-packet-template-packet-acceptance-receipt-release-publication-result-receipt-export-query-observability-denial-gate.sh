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

RESULT_RECEIPT_RETENTION_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-retention-expiry-garbage-collection-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-retention-expiry-garbage-collection-denial-gate.sh
)"

result_receipt_retention_report_sha256="$(sha256_text "$RESULT_RECEIPT_RETENTION_JSON")"
result_receipt_export_query_observability_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-export-query-observability-denial:$result_receipt_retention_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"

jq -n -e \
  --argjson source "$RESULT_RECEIPT_RETENTION_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_audit_evidence_ready == true
    and $source.release_publication_result_receipt_retention_surface_count == 18
    and $source.release_publication_result_receipt_retention_attempt_count == 18
    and $source.release_publication_result_receipt_retention_policy_recorded_count == 0
    and $source.release_publication_result_receipt_retention_policy_persisted_count == 0
    and $source.release_publication_result_receipt_retention_index_recorded_count == 0
    and $source.release_publication_result_receipt_retention_ledger_recorded_count == 0
    and $source.release_publication_result_receipt_ttl_update_recorded_count == 0
    and $source.release_publication_result_receipt_ttl_extension_recorded_count == 0
    and $source.release_publication_result_receipt_expiry_recorded_count == 0
    and $source.release_publication_result_receipt_expiry_scheduler_registered_count == 0
    and $source.release_publication_result_receipt_expiry_timer_started_count == 0
    and $source.release_publication_result_receipt_expiry_ack_recorded_count == 0
    and $source.release_publication_result_receipt_garbage_collection_scan_performed_count == 0
    and $source.release_publication_result_receipt_garbage_collection_candidate_recorded_count == 0
    and $source.release_publication_result_receipt_garbage_collection_decision_recorded_count == 0
    and $source.release_publication_result_receipt_delete_performed_count == 0
    and $source.release_publication_result_receipt_tombstone_recorded_count == 0
    and $source.release_publication_result_receipt_sweep_performed_count == 0
    and $source.release_publication_result_receipt_archive_written_count == 0
    and $source.release_publication_result_receipt_compaction_performed_count == 0
    and $source.release_publication_result_receipt_compaction_artifact_written_count == 0
    and $source.release_publication_result_receipt_ledger_retention_recorded_count == 0
    and $source.release_publication_result_receipt_index_retention_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_retention_recorded_count == 0
    and $source.release_publication_result_receipt_retention_acceptance_recorded_count == 0
    and $source.release_publication_result_receipt_retention_release_publication_authority_derived_count == 0
    and $source.release_publication_result_receipt_retention_activation_authority_derived_count == 0
    and $source.release_publication_result_receipt_retention_activation_command_derived_count == 0
    and $source.release_publication_result_receipt_retention_live_execution_allowed_count == 0
    and $source.packet_acceptance_receipt_release_publication_result_receipt_retention_policy_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_retention_index_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_retention_ledger_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_expiry_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_expiry_scheduler_registered == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_expiry_timer_started == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_garbage_collection_scan_performed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delete_marker_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delete_performed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_tombstone_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_sweep_performed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_archive_written == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_compaction_performed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_audit_trail_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_immutable_evidence_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_hash_chain_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_readback_evidence_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_exported == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_query_registered == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_observability_recorded == false
    and $source.packet_acceptance_receipt_release_publication_recorded == false
    and $source.operator_acceptance_recorded == false
    and $source.operator_approval_recorded == false
    and $source.release_publication_authority_derived == false
    and $source.activation_authority_derived == false
    and $source.activation_command_derived == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and $source.memory_store_write_performed == false
    and $source.memory_store_mutated == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.credential_read == false
    and $source.secret_file_read == false
    and $source.install_executed == false
    and $source.launchd_mutated == false
    and $source.service_restarted == false
    and $source.active_binary_mutated == false
    and $source.public_release_claimed == false
    and $source.public_ga_claimed == false
    and $source.release_artifact_written == false
    and $source.public_artifact_written == false
    and $source.external_send_performed == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

result_receipt_export_query_observability_surfaces_json="$(
  jq -n '[
    "publication_result_receipt_query_registration_claim",
    "publication_result_receipt_query_execution_claim",
    "publication_result_receipt_query_result_claim",
    "publication_result_receipt_search_index_claim",
    "publication_result_receipt_export_request_claim",
    "publication_result_receipt_export_snapshot_claim",
    "publication_result_receipt_export_file_claim",
    "publication_result_receipt_export_stream_claim",
    "publication_result_receipt_observability_metric_claim",
    "publication_result_receipt_observability_log_claim",
    "publication_result_receipt_observability_trace_claim",
    "publication_result_receipt_observability_event_claim",
    "publication_result_receipt_dashboard_panel_claim",
    "publication_result_receipt_alert_slo_claim",
    "publication_result_receipt_operator_summary_readback_claim",
    "publication_result_receipt_audit_view_evidence_claim",
    "publication_result_receipt_release_publication_authority_view_claim",
    "publication_result_receipt_activation_live_install_restart_active_binary_view_claim"
  ] | map({
    release_publication_result_receipt_export_query_observability_surface: .,
    source_release_publication_result_receipt_retention_ready: true,
    export_query_or_observability_attempted: true,
    query_registered: false,
    query_executed: false,
    query_result_recorded: false,
    query_result_persisted: false,
    search_index_recorded: false,
    search_index_persisted: false,
    export_requested: false,
    export_accepted: false,
    export_snapshot_recorded: false,
    export_snapshot_persisted: false,
    export_file_written: false,
    export_stream_opened: false,
    observability_metric_recorded: false,
    observability_log_recorded: false,
    observability_trace_recorded: false,
    observability_event_recorded: false,
    dashboard_panel_recorded: false,
    alert_registered: false,
    slo_recorded: false,
    operator_summary_recorded: false,
    readback_surface_recorded: false,
    audit_view_recorded: false,
    ledger_observability_recorded: false,
    index_observability_recorded: false,
    delivery_observability_recorded: false,
    retention_policy_recorded: false,
    expiry_recorded: false,
    garbage_collection_scan_performed: false,
    audit_trail_recorded: false,
    immutable_evidence_recorded: false,
    hash_chain_recorded: false,
    publication_completion_ack_recorded: false,
    release_publication_recorded: false,
    release_artifact_written: false,
    public_artifact_written: false,
    public_distribution_performed: false,
    channel_delivery_performed: false,
    external_send_performed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    acceptance_recorded: false,
    operator_approval_derived: false,
    release_publication_authority_derived: false,
    activation_authority_derived: false,
    activation_command_derived: false,
    live_execution_allowed: false,
    activation_performed: false,
    memory_store_write_performed: false,
    memory_store_mutated: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    install_executed: false,
    launchd_mutated: false,
    service_restarted: false,
    active_binary_mutated: false,
    export_query_observability_noop_confirmed: true,
    release_publication_result_receipt_export_query_observability_status: "release_publication_result_receipt_export_query_observability_denied"
  })'
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_gate" \
  --arg result_receipt_retention_report_sha256 "$result_receipt_retention_report_sha256" \
  --arg result_receipt_export_query_observability_contract_hash_sha256 "$result_receipt_export_query_observability_contract_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$RESULT_RECEIPT_RETENTION_JSON" \
  --argjson surfaces "$result_receipt_export_query_observability_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_release_publication_result_receipt_export_query_observability_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_v1",
    receipt_release_publication_result_receipt_export_query_observability_mode:"denied_release_publication_result_receipt_cannot_create_export_query_observability_views_or_authority",
    source_packet_acceptance_receipt_release_publication_result_receipt_retention_gate:$source.gate,
    source_packet_acceptance_receipt_release_publication_result_receipt_retention_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_ready,
    source_packet_acceptance_receipt_release_publication_result_receipt_retention_report_sha256:$result_receipt_retention_report_sha256,
    source_release_publication_result_receipt_retention_contract_hash_sha256:$source.release_publication_result_receipt_retention_expiry_garbage_collection_contract_hash_sha256,
    release_publication_result_receipt_export_query_observability_contract_hash_sha256:$result_receipt_export_query_observability_contract_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_ready:true,
    source_release_publication_result_receipt_retention_surface_count:$source.release_publication_result_receipt_retention_surface_count,
    source_release_publication_result_receipt_retention_attempt_count:$source.release_publication_result_receipt_retention_attempt_count,
    source_release_publication_result_receipt_retention_policy_recorded_count:$source.release_publication_result_receipt_retention_policy_recorded_count,
    source_release_publication_result_receipt_expiry_recorded_count:$source.release_publication_result_receipt_expiry_recorded_count,
    source_release_publication_result_receipt_garbage_collection_scan_performed_count:$source.release_publication_result_receipt_garbage_collection_scan_performed_count,
    source_release_publication_result_receipt_delete_performed_count:$source.release_publication_result_receipt_delete_performed_count,
    source_release_publication_result_receipt_archive_written_count:$source.release_publication_result_receipt_archive_written_count,
    source_release_publication_result_receipt_compaction_artifact_written_count:$source.release_publication_result_receipt_compaction_artifact_written_count,
    source_release_publication_result_receipt_retention_release_publication_authority_derived_count:$source.release_publication_result_receipt_retention_release_publication_authority_derived_count,
    source_release_publication_result_receipt_retention_activation_authority_derived_count:$source.release_publication_result_receipt_retention_activation_authority_derived_count,
    release_publication_result_receipt_export_query_observability_surface_count:($surfaces | length),
    release_publication_result_receipt_export_query_observability_attempt_count:($surfaces | length),
    release_publication_result_receipt_query_registered_count:0,
    release_publication_result_receipt_query_executed_count:0,
    release_publication_result_receipt_query_result_recorded_count:0,
    release_publication_result_receipt_query_result_persisted_count:0,
    release_publication_result_receipt_search_index_recorded_count:0,
    release_publication_result_receipt_search_index_persisted_count:0,
    release_publication_result_receipt_export_requested_count:0,
    release_publication_result_receipt_export_accepted_count:0,
    release_publication_result_receipt_export_snapshot_recorded_count:0,
    release_publication_result_receipt_export_snapshot_persisted_count:0,
    release_publication_result_receipt_export_file_written_count:0,
    release_publication_result_receipt_export_stream_opened_count:0,
    release_publication_result_receipt_observability_metric_recorded_count:0,
    release_publication_result_receipt_observability_log_recorded_count:0,
    release_publication_result_receipt_observability_trace_recorded_count:0,
    release_publication_result_receipt_observability_event_recorded_count:0,
    release_publication_result_receipt_dashboard_panel_recorded_count:0,
    release_publication_result_receipt_alert_registered_count:0,
    release_publication_result_receipt_slo_recorded_count:0,
    release_publication_result_receipt_operator_summary_recorded_count:0,
    release_publication_result_receipt_readback_surface_recorded_count:0,
    release_publication_result_receipt_audit_view_recorded_count:0,
    release_publication_result_receipt_ledger_observability_recorded_count:0,
    release_publication_result_receipt_index_observability_recorded_count:0,
    release_publication_result_receipt_delivery_observability_recorded_count:0,
    release_publication_result_receipt_export_query_observability_acceptance_recorded_count:0,
    release_publication_result_receipt_export_query_observability_operator_approval_derived_count:0,
    release_publication_result_receipt_export_query_observability_release_publication_authority_derived_count:0,
    release_publication_result_receipt_export_query_observability_activation_authority_derived_count:0,
    release_publication_result_receipt_export_query_observability_activation_command_derived_count:0,
    release_publication_result_receipt_export_query_observability_live_execution_allowed_count:0,
    release_publication_result_receipt_export_query_observability_surfaces:$surfaces,
    denied_by_packet_receipt_release_publication_result_receipt_export_query_observability:[
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_query_registration_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_query_execution_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_query_result_recording_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_query_result_persistence_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_search_index_recording_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_search_index_persistence_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_export_request_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_export_acceptance_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_export_snapshot_recording_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_export_snapshot_persistence_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_export_file_write_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_export_stream_open_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_observability_metric_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_observability_log_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_observability_trace_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_observability_event_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_dashboard_panel_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_alert_registration_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_slo_recording_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_operator_summary_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_readback_surface_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_audit_view_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_ledger_index_delivery_observability_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_completion_ack_from_view_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_acceptance_from_view_denied",
      "operator_readiness_packet_template_packet_receipt_release_publication_authority_from_result_receipt_view_denied",
      "operator_readiness_packet_template_packet_receipt_activation_live_from_result_receipt_view_denied",
      "operator_readiness_packet_template_packet_receipt_install_restart_active_binary_from_result_receipt_view_denied",
      "operator_readiness_packet_template_packet_receipt_memory_provider_external_send_from_result_receipt_view_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_gate",
        status:"allowed_report_only_next_slice",
        exports_receipt:false,
        registers_query:false,
        records_observability:false,
        records_summary:false,
        records_briefing:false,
        derives_release_publication_authority:false,
        derives_activation_authority:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false,
        sends_externally:false
      }
    ],
    packet_acceptance_receipt_release_publication_result_receipt_query_registered:false,
    packet_acceptance_receipt_release_publication_result_receipt_query_executed:false,
    packet_acceptance_receipt_release_publication_result_receipt_query_result_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_query_result_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_search_index_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_search_index_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_export_requested:false,
    packet_acceptance_receipt_release_publication_result_receipt_export_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_export_snapshot_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_export_snapshot_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_export_file_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_export_stream_opened:false,
    packet_acceptance_receipt_release_publication_result_receipt_observability_metric_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_observability_log_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_observability_trace_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_observability_event_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_dashboard_panel_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_alert_registered:false,
    packet_acceptance_receipt_release_publication_result_receipt_slo_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_operator_summary_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_readback_surface_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_audit_view_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_ledger_observability_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_index_observability_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_observability_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_retention_policy_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_retention_index_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_retention_ledger_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_expiry_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_garbage_collection_scan_performed:false,
    packet_acceptance_receipt_release_publication_result_receipt_archive_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_compaction_performed:false,
    packet_acceptance_receipt_release_publication_result_receipt_audit_trail_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_immutable_evidence_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_hash_chain_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_readback_evidence_recorded:false,
    packet_acceptance_receipt_publication_completion_ack_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_cancellation_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_ordering_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_replayed:false,
    packet_acceptance_receipt_release_publication_result_receipt_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_exported:false,
    packet_acceptance_receipt_release_publication_result_receipt_observability_recorded:false,
    packet_acceptance_receipt_release_publication_recorded:false,
    packet_acceptance_receipt_release_artifact_written:false,
    packet_acceptance_receipt_public_artifact_written:false,
    packet_acceptance_receipt_public_distribution_performed:false,
    packet_acceptance_receipt_channel_delivery_performed:false,
    packet_acceptance_receipt_external_publication_sent:false,
    packet_acceptance_receipt_public_release_claimed:false,
    packet_acceptance_receipt_public_ga_claimed:false,
    operator_acceptance_recorded:false,
    operator_approval_recorded:false,
    release_publication_authority_derived:false,
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
      packet_acceptance_receipt_release_publication_result_receipt_query_registered:false,
      packet_acceptance_receipt_release_publication_result_receipt_query_executed:false,
      packet_acceptance_receipt_release_publication_result_receipt_query_result_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_query_result_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_search_index_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_search_index_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_export_requested:false,
      packet_acceptance_receipt_release_publication_result_receipt_export_accepted:false,
      packet_acceptance_receipt_release_publication_result_receipt_export_snapshot_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_export_snapshot_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_export_file_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_export_stream_opened:false,
      packet_acceptance_receipt_release_publication_result_receipt_observability_metric_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_observability_log_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_observability_trace_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_observability_event_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_dashboard_panel_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_alert_registered:false,
      packet_acceptance_receipt_release_publication_result_receipt_slo_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_operator_summary_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_readback_surface_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_audit_view_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_ledger_observability_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_index_observability_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_observability_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_retention_policy_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_retention_index_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_retention_ledger_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_expiry_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_garbage_collection_scan_performed:false,
      packet_acceptance_receipt_release_publication_result_receipt_archive_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_compaction_performed:false,
      packet_acceptance_receipt_release_publication_result_receipt_audit_trail_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_immutable_evidence_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_hash_chain_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_readback_evidence_recorded:false,
      packet_acceptance_receipt_publication_completion_ack_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_cancellation_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_ordering_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_replayed:false,
      packet_acceptance_receipt_release_publication_result_receipt_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_materialized:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_exported:false,
      packet_acceptance_receipt_release_publication_result_receipt_observability_recorded:false,
      packet_acceptance_receipt_release_publication_recorded:false,
      packet_acceptance_receipt_release_artifact_written:false,
      packet_acceptance_receipt_public_artifact_written:false,
      packet_acceptance_receipt_public_distribution_performed:false,
      packet_acceptance_receipt_channel_delivery_performed:false,
      packet_acceptance_receipt_external_publication_sent:false,
      packet_acceptance_receipt_public_release_claimed:false,
      packet_acceptance_receipt_public_ga_claimed:false,
      operator_acceptance_recorded:false,
      operator_approval_recorded:false,
      release_publication_authority_derived:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_retention_ready == true
  and .source_release_publication_result_receipt_retention_surface_count == 18
  and .source_release_publication_result_receipt_retention_attempt_count == 18
  and .source_release_publication_result_receipt_retention_policy_recorded_count == 0
  and .source_release_publication_result_receipt_expiry_recorded_count == 0
  and .source_release_publication_result_receipt_garbage_collection_scan_performed_count == 0
  and .source_release_publication_result_receipt_delete_performed_count == 0
  and .source_release_publication_result_receipt_archive_written_count == 0
  and .source_release_publication_result_receipt_compaction_artifact_written_count == 0
  and .source_release_publication_result_receipt_retention_release_publication_authority_derived_count == 0
  and .source_release_publication_result_receipt_retention_activation_authority_derived_count == 0
  and .release_publication_result_receipt_export_query_observability_surface_count == 18
  and .release_publication_result_receipt_export_query_observability_attempt_count == 18
  and .release_publication_result_receipt_query_registered_count == 0
  and .release_publication_result_receipt_query_executed_count == 0
  and .release_publication_result_receipt_query_result_recorded_count == 0
  and .release_publication_result_receipt_query_result_persisted_count == 0
  and .release_publication_result_receipt_search_index_recorded_count == 0
  and .release_publication_result_receipt_search_index_persisted_count == 0
  and .release_publication_result_receipt_export_requested_count == 0
  and .release_publication_result_receipt_export_accepted_count == 0
  and .release_publication_result_receipt_export_snapshot_recorded_count == 0
  and .release_publication_result_receipt_export_snapshot_persisted_count == 0
  and .release_publication_result_receipt_export_file_written_count == 0
  and .release_publication_result_receipt_export_stream_opened_count == 0
  and .release_publication_result_receipt_observability_metric_recorded_count == 0
  and .release_publication_result_receipt_observability_log_recorded_count == 0
  and .release_publication_result_receipt_observability_trace_recorded_count == 0
  and .release_publication_result_receipt_observability_event_recorded_count == 0
  and .release_publication_result_receipt_dashboard_panel_recorded_count == 0
  and .release_publication_result_receipt_alert_registered_count == 0
  and .release_publication_result_receipt_slo_recorded_count == 0
  and .release_publication_result_receipt_operator_summary_recorded_count == 0
  and .release_publication_result_receipt_readback_surface_recorded_count == 0
  and .release_publication_result_receipt_audit_view_recorded_count == 0
  and .release_publication_result_receipt_ledger_observability_recorded_count == 0
  and .release_publication_result_receipt_index_observability_recorded_count == 0
  and .release_publication_result_receipt_delivery_observability_recorded_count == 0
  and .release_publication_result_receipt_export_query_observability_acceptance_recorded_count == 0
  and .release_publication_result_receipt_export_query_observability_release_publication_authority_derived_count == 0
  and .release_publication_result_receipt_export_query_observability_activation_authority_derived_count == 0
  and .release_publication_result_receipt_export_query_observability_activation_command_derived_count == 0
  and .release_publication_result_receipt_export_query_observability_live_execution_allowed_count == 0
  and (.release_publication_result_receipt_export_query_observability_surfaces | all(
    .export_query_or_observability_attempted == true
    and .query_registered == false
    and .query_executed == false
    and .query_result_recorded == false
    and .query_result_persisted == false
    and .search_index_recorded == false
    and .export_requested == false
    and .export_accepted == false
    and .export_snapshot_recorded == false
    and .export_file_written == false
    and .export_stream_opened == false
    and .observability_metric_recorded == false
    and .observability_log_recorded == false
    and .observability_trace_recorded == false
    and .observability_event_recorded == false
    and .dashboard_panel_recorded == false
    and .alert_registered == false
    and .slo_recorded == false
    and .operator_summary_recorded == false
    and .readback_surface_recorded == false
    and .audit_view_recorded == false
    and .ledger_observability_recorded == false
    and .index_observability_recorded == false
    and .delivery_observability_recorded == false
    and .retention_policy_recorded == false
    and .expiry_recorded == false
    and .garbage_collection_scan_performed == false
    and .audit_trail_recorded == false
    and .immutable_evidence_recorded == false
    and .hash_chain_recorded == false
    and .publication_completion_ack_recorded == false
    and .release_publication_recorded == false
    and .release_artifact_written == false
    and .public_artifact_written == false
    and .external_send_performed == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .live_execution_allowed == false
    and .activation_performed == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .provider_invoked == false
    and .model_invoked == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .export_query_observability_noop_confirmed == true
    and .release_publication_result_receipt_export_query_observability_status == "release_publication_result_receipt_export_query_observability_denied"
  ))
  and (.denied_by_packet_receipt_release_publication_result_receipt_export_query_observability | length) == 29
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_release_publication_result_receipt_query_registered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_query_executed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_query_result_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_search_index_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_export_requested == false
  and .packet_acceptance_receipt_release_publication_result_receipt_export_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_export_snapshot_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_export_file_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_export_stream_opened == false
  and .packet_acceptance_receipt_release_publication_result_receipt_observability_metric_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_observability_log_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_observability_trace_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_observability_event_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_dashboard_panel_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_operator_summary_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_readback_surface_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_audit_view_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_ledger_observability_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_index_observability_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_observability_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_retention_policy_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_expiry_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_garbage_collection_scan_performed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_audit_trail_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_immutable_evidence_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_hash_chain_recorded == false
  and .packet_acceptance_receipt_publication_completion_ack_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_cancellation_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_ordering_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_replayed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_persisted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_exported == false
  and .packet_acceptance_receipt_release_publication_result_receipt_observability_recorded == false
  and .packet_acceptance_receipt_release_publication_recorded == false
  and .packet_acceptance_receipt_release_artifact_written == false
  and .packet_acceptance_receipt_public_artifact_written == false
  and .packet_acceptance_receipt_public_distribution_performed == false
  and .operator_acceptance_recorded == false
  and .operator_approval_recorded == false
  and .release_publication_authority_derived == false
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

printf '%s\n' "$report" | jq .
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt export/query/observability denial gate passed"
