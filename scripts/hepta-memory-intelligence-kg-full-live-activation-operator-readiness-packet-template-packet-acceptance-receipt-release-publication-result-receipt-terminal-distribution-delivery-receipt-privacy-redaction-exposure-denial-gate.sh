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

TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_QUERY_EXPORT_OBSERVABILITY_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-query-export-observability-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-query-export-observability-denial-gate.sh
)"

delivery_receipt_query_export_observability_report_sha256="$(
  sha256_text "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_QUERY_EXPORT_OBSERVABILITY_JSON"
)"
delivery_receipt_privacy_redaction_exposure_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-privacy-redaction-exposure-denial:$delivery_receipt_query_export_observability_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
delivery_receipt_privacy_redaction_exposure_policy_hash_sha256="$(
  sha256_text "release-publication-result-receipt-terminal-distribution-delivery-receipt-privacy-redaction-exposure-denial:no-redacted-preview:no-payload-hash:no-summary:no-privacy-review:no-scan:no-authority"
)"

jq -n -e \
  --argjson source "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_QUERY_EXPORT_OBSERVABILITY_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_allowed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_request_accepted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_accepted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_recorded_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_persisted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_materialized_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_filesystem_written_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_delivered_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_exposed_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_query_registered_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_query_executed_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_query_result_exposed_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_search_index_written_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_export_requested_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_export_snapshot_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_export_file_written_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_export_stream_opened_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_observability_metric_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_observability_log_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_observability_trace_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_observability_event_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_dashboard_panel_exposed_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_alert_slo_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_operator_readback_exposed_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_audit_view_exposed_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_status_evidence_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_release_publication_authority_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_activation_authority_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_activation_command_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_live_execution_allowed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_install_executed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_service_restarted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_active_binary_mutated_count == 0
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_accepted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_persisted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_materialized == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_filesystem_written == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_delivered == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_exposed == false
    and $source.operator_acceptance_recorded == false
    and $source.operator_approval_recorded == false
    and $source.release_publication_authority_derived == false
    and $source.activation_authority_derived == false
    and $source.activation_command_derived == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and $source.memory_store_write_performed == false
    and $source.memory_store_mutated == false
    and $source.live_kg_write_performed == false
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
    and ($source.allowed_next_actions | any(.action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_denial_gate" and .status == "allowed_report_only_next_slice" and .exposes_delivery_receipt_query == false and .exports_delivery_receipt == false and .records_observability == false and .mutates_memory_store == false and .writes_kg == false and .sends_externally == false))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

delivery_receipt_privacy_redaction_exposure_surfaces_json="$(
  jq -n '
    def privacy_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_surface:$id,
        source_terminal_distribution_delivery_receipt_query_export_observability_ready:true,
        privacy_redaction_exposure_attempted:true,
        privacy_redaction_exposure_allowed:false,
        privacy_redaction_exposure_request_accepted:false,
        privacy_redaction_exposure_accepted:false,
        privacy_redaction_exposure_recorded:false,
        privacy_redaction_exposure_persisted:false,
        privacy_redaction_exposure_materialized:false,
        privacy_redaction_exposure_filesystem_written:false,
        privacy_redaction_exposure_delivered:false,
        privacy_redaction_exposure_exposed:false,
        redacted_payload_preview_rendered:false,
        payload_hash_preview_rendered:false,
        payload_diff_rendered:false,
        payload_summary_rendered:false,
        operator_readback_text_rendered:false,
        privacy_review_recorded:false,
        secret_scan_recorded:false,
        pii_scan_recorded:false,
        raw_payload_inspected:false,
        plaintext_materialized:false,
        redaction_bypass_performed:false,
        hash_to_payload_linked:false,
        export_redacted_payload_written:false,
        observability_redacted_payload_recorded:false,
        dashboard_redaction_badge_exposed:false,
        audit_redaction_view_exposed:false,
        delivery_receipt_payload_exposure_evidence_exposed:false,
        acceptance_recorded:false,
        operator_approval_derived:false,
        release_publication_authority_derived:false,
        activation_authority_derived:false,
        activation_command_derived:false,
        live_execution_allowed:false,
        activation_performed:false,
        install_executed:false,
        service_restarted:false,
        launchd_mutated:false,
        active_binary_mutated:false,
        release_artifact_written:false,
        public_artifact_written:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        live_kg_write_performed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_read:false,
        secret_file_read:false,
        external_send_performed:false,
        privacy_redaction_exposure_noop_confirmed:true,
        privacy_redaction_exposure_status:$status,
        reason:$reason
      } + $extra;
    [
      privacy_surface("publication_result_receipt_delivery_receipt_redacted_payload_preview"; "blocked_redacted_payload_preview_noop"; "redacted_payload_preview_denied"; {redacted_payload_preview_requested:true}),
      privacy_surface("publication_result_receipt_delivery_receipt_payload_hash_preview"; "blocked_payload_hash_preview_noop"; "payload_hash_preview_denied"; {payload_hash_preview_requested:true}),
      privacy_surface("publication_result_receipt_delivery_receipt_payload_diff_preview"; "blocked_payload_diff_preview_noop"; "payload_diff_preview_denied"; {payload_diff_preview_requested:true}),
      privacy_surface("publication_result_receipt_delivery_receipt_payload_summary"; "blocked_payload_summary_noop"; "payload_summary_denied"; {payload_summary_requested:true}),
      privacy_surface("publication_result_receipt_delivery_receipt_operator_readback_text"; "blocked_operator_readback_text_noop"; "operator_readback_text_denied"; {operator_readback_text_requested:true}),
      privacy_surface("publication_result_receipt_delivery_receipt_privacy_review"; "blocked_privacy_review_noop"; "privacy_review_denied"; {privacy_review_requested:true}),
      privacy_surface("publication_result_receipt_delivery_receipt_secret_scan"; "blocked_secret_scan_noop"; "secret_scan_denied"; {secret_scan_requested:true}),
      privacy_surface("publication_result_receipt_delivery_receipt_pii_scan"; "blocked_pii_scan_noop"; "pii_scan_denied"; {pii_scan_requested:true}),
      privacy_surface("publication_result_receipt_delivery_receipt_raw_payload_inspection"; "blocked_raw_payload_inspection_noop"; "raw_payload_inspection_denied"; {raw_payload_inspection_requested:true}),
      privacy_surface("publication_result_receipt_delivery_receipt_plaintext_materialization"; "blocked_plaintext_materialization_noop"; "plaintext_materialization_denied"; {plaintext_materialization_requested:true}),
      privacy_surface("publication_result_receipt_delivery_receipt_redaction_bypass"; "blocked_redaction_bypass_noop"; "redaction_bypass_denied"; {redaction_bypass_requested:true}),
      privacy_surface("publication_result_receipt_delivery_receipt_hash_to_payload_link"; "blocked_hash_to_payload_link_noop"; "hash_to_payload_link_denied"; {hash_to_payload_link_requested:true}),
      privacy_surface("publication_result_receipt_delivery_receipt_export_redacted_payload"; "blocked_export_redacted_payload_noop"; "export_redacted_payload_denied"; {export_redacted_payload_requested:true}),
      privacy_surface("publication_result_receipt_delivery_receipt_observability_redacted_payload"; "blocked_observability_redacted_payload_noop"; "observability_redacted_payload_denied"; {observability_redacted_payload_requested:true}),
      privacy_surface("publication_result_receipt_delivery_receipt_dashboard_redaction_badge"; "blocked_dashboard_redaction_badge_noop"; "dashboard_redaction_badge_denied"; {dashboard_redaction_badge_requested:true}),
      privacy_surface("publication_result_receipt_delivery_receipt_audit_redaction_view"; "blocked_audit_redaction_view_noop"; "audit_redaction_view_denied"; {audit_redaction_view_requested:true}),
      privacy_surface("publication_result_receipt_release_publication_authority_payload_exposure"; "blocked_release_publication_authority_payload_exposure_noop"; "release_publication_authority_from_payload_exposure_denied"; {release_publication_authority_payload_exposure_requested:true}),
      privacy_surface("publication_result_receipt_activation_live_active_binary_payload_exposure"; "blocked_activation_live_active_binary_payload_exposure_noop"; "activation_live_active_binary_from_payload_exposure_denied"; {activation_live_payload_exposure_requested:true, install_restart_active_binary_payload_exposure_requested:true})
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_denial_gate" \
  --arg delivery_receipt_query_export_observability_report_sha256 "$delivery_receipt_query_export_observability_report_sha256" \
  --arg delivery_receipt_privacy_redaction_exposure_contract_hash_sha256 "$delivery_receipt_privacy_redaction_exposure_contract_hash_sha256" \
  --arg delivery_receipt_privacy_redaction_exposure_policy_hash_sha256 "$delivery_receipt_privacy_redaction_exposure_policy_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_QUERY_EXPORT_OBSERVABILITY_JSON" \
  --argjson surfaces "$delivery_receipt_privacy_redaction_exposure_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_denial_v1",
    receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_mode:"denied_delivery_receipt_views_cannot_become_redacted_payload_privacy_or_payload_exposure_surfaces",
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_gate:$source.gate,
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_denial_ready,
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_report_sha256:$delivery_receipt_query_export_observability_report_sha256,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_contract_hash_sha256,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_contract_hash_sha256:$delivery_receipt_privacy_redaction_exposure_contract_hash_sha256,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_policy_hash_sha256:$delivery_receipt_privacy_redaction_exposure_policy_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_denial_ready:true,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_surface_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_attempt_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_recorded_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_persisted_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_persisted_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_exposed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_exposed_count,
    source_release_publication_result_receipt_delivery_receipt_query_result_exposed_count:$source.release_publication_result_receipt_delivery_receipt_query_result_exposed_count,
    source_release_publication_result_receipt_delivery_receipt_export_file_written_count:$source.release_publication_result_receipt_delivery_receipt_export_file_written_count,
    source_release_publication_result_receipt_delivery_receipt_observability_log_recorded_count:$source.release_publication_result_receipt_delivery_receipt_observability_log_recorded_count,
    source_release_publication_result_receipt_delivery_receipt_dashboard_panel_exposed_count:$source.release_publication_result_receipt_delivery_receipt_dashboard_panel_exposed_count,
    source_release_publication_result_receipt_delivery_receipt_operator_readback_exposed_count:$source.release_publication_result_receipt_delivery_receipt_operator_readback_exposed_count,
    source_release_publication_result_receipt_delivery_receipt_audit_view_exposed_count:$source.release_publication_result_receipt_delivery_receipt_audit_view_exposed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_release_publication_authority_derived_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_release_publication_authority_derived_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_activation_authority_derived_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_activation_authority_derived_count,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_surface_count:($surfaces | length),
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_attempt_count:($surfaces | length),
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_allowed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_request_accepted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_accepted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_persisted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_materialized_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_filesystem_written_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_delivered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_exposed_count:0,
    release_publication_result_receipt_delivery_receipt_redacted_payload_preview_rendered_count:0,
    release_publication_result_receipt_delivery_receipt_payload_hash_preview_rendered_count:0,
    release_publication_result_receipt_delivery_receipt_payload_diff_rendered_count:0,
    release_publication_result_receipt_delivery_receipt_payload_summary_rendered_count:0,
    release_publication_result_receipt_delivery_receipt_operator_readback_text_rendered_count:0,
    release_publication_result_receipt_delivery_receipt_privacy_review_recorded_count:0,
    release_publication_result_receipt_delivery_receipt_secret_scan_recorded_count:0,
    release_publication_result_receipt_delivery_receipt_pii_scan_recorded_count:0,
    release_publication_result_receipt_delivery_receipt_raw_payload_inspected_count:0,
    release_publication_result_receipt_delivery_receipt_plaintext_materialized_count:0,
    release_publication_result_receipt_delivery_receipt_redaction_bypass_performed_count:0,
    release_publication_result_receipt_delivery_receipt_hash_to_payload_linked_count:0,
    release_publication_result_receipt_delivery_receipt_export_redacted_payload_written_count:0,
    release_publication_result_receipt_delivery_receipt_observability_redacted_payload_recorded_count:0,
    release_publication_result_receipt_delivery_receipt_dashboard_redaction_badge_exposed_count:0,
    release_publication_result_receipt_delivery_receipt_audit_redaction_view_exposed_count:0,
    release_publication_result_receipt_delivery_receipt_payload_exposure_evidence_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_acceptance_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_operator_approval_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_release_publication_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_activation_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_activation_command_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_live_execution_allowed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_install_executed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_service_restarted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_active_binary_mutated_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_release_artifact_written_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_public_artifact_written_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_surfaces:$surfaces,
    denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure:[
      "source_terminal_distribution_delivery_receipt_query_export_observability_report_required",
      "privacy_redaction_exposure_request_acceptance_denied",
      "privacy_redaction_exposure_acceptance_denied",
      "privacy_redaction_exposure_recording_denied",
      "privacy_redaction_exposure_persistence_denied",
      "privacy_redaction_exposure_materialization_denied",
      "privacy_redaction_exposure_filesystem_write_denied",
      "privacy_redaction_exposure_delivery_denied",
      "privacy_redaction_exposure_denied",
      "redacted_payload_preview_denied",
      "payload_hash_preview_denied",
      "payload_diff_preview_denied",
      "payload_summary_denied",
      "operator_readback_text_denied",
      "privacy_review_denied",
      "secret_scan_denied",
      "pii_scan_denied",
      "raw_payload_inspection_denied",
      "plaintext_materialization_denied",
      "redaction_bypass_denied",
      "hash_to_payload_link_denied",
      "export_redacted_payload_denied",
      "observability_redacted_payload_denied",
      "dashboard_redaction_badge_denied",
      "audit_redaction_view_denied",
      "payload_exposure_evidence_denied",
      "acceptance_from_payload_exposure_denied",
      "operator_approval_from_payload_exposure_denied",
      "release_publication_authority_from_payload_exposure_denied",
      "activation_live_from_payload_exposure_denied",
      "install_restart_active_binary_from_payload_exposure_denied",
      "release_artifact_write_denied",
      "public_artifact_write_denied",
      "memory_provider_kg_from_payload_exposure_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_non_persistence_denial_gate",
        status:"allowed_report_only_next_slice",
        renders_payload:false,
        exposes_redacted_payload:false,
        records_privacy_review:false,
        records_operator_acceptance:false,
        derives_release_publication_authority:false,
        derives_activation_authority:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false,
        sends_externally:false
      }
    ],
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_filesystem_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_query_registered:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_query_executed:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_query_result_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_search_index_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_requested:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_snapshot_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_file_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_stream_opened:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_metric_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_log_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_trace_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_event_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_dashboard_panel_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_alert_slo_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_readback_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_audit_view_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_status_evidence_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_allowed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_request_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_filesystem_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_redacted_payload_preview_rendered:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_payload_hash_preview_rendered:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_payload_diff_rendered:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_payload_summary_rendered:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_readback_text_rendered:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_privacy_review_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_secret_scan_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_pii_scan_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_raw_payload_inspected:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_plaintext_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_redaction_bypass_performed:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_hash_to_payload_linked:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_redacted_payload_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_redacted_payload_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_dashboard_redaction_badge_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_audit_redaction_view_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_payload_exposure_evidence_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_persisted:false,
    packet_acceptance_receipt_release_publication_recorded:false,
    operator_acceptance_recorded:false,
    operator_approval_recorded:false,
    release_publication_authority_derived:false,
    activation_authority_derived:false,
    activation_command_derived:false,
    activation_allowed:false,
    activation_performed:false,
    memory_store_write_performed:false,
    memory_store_mutated:false,
    live_kg_write_performed:false,
    provider_invoked:false,
    model_invoked:false,
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
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_accepted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_materialized:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_filesystem_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_redacted_payload_preview_rendered:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_payload_hash_preview_rendered:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_payload_diff_rendered:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_payload_summary_rendered:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_readback_text_rendered:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_privacy_review_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_secret_scan_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_pii_scan_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_raw_payload_inspected:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_plaintext_materialized:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_redaction_bypass_performed:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_hash_to_payload_linked:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_redacted_payload_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_redacted_payload_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_dashboard_redaction_badge_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_audit_redaction_view_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_payload_exposure_evidence_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_query_result_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_file_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_log_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_dashboard_panel_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_readback_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_audit_view_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_persisted:false,
      operator_acceptance_recorded:false,
      operator_approval_recorded:false,
      release_publication_authority_derived:false,
      activation_authority_derived:false,
      activation_command_derived:false,
      activation_allowed:false,
      activation_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      live_kg_write_performed:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      secret_file_read:false,
      install_executed:false,
      launchd_mutated:false,
      service_restarted:false,
      active_binary_mutated:false,
      release_artifact_written:false,
      public_artifact_written:false,
      public_release_claimed:false,
      public_ga_claimed:false,
      external_send_performed:false,
      filesystem_written:false
    }
  }')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_ready == true
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_surface_count == 18
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_attempt_count == 18
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_recorded_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_persisted_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_exposed_count == 0
  and .source_release_publication_result_receipt_delivery_receipt_query_result_exposed_count == 0
  and .source_release_publication_result_receipt_delivery_receipt_export_file_written_count == 0
  and .source_release_publication_result_receipt_delivery_receipt_observability_log_recorded_count == 0
  and .source_release_publication_result_receipt_delivery_receipt_dashboard_panel_exposed_count == 0
  and .source_release_publication_result_receipt_delivery_receipt_operator_readback_exposed_count == 0
  and .source_release_publication_result_receipt_delivery_receipt_audit_view_exposed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_release_publication_authority_derived_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_surface_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_attempt_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_request_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_persisted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_materialized_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_filesystem_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_delivered_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_exposed_count == 0
  and .release_publication_result_receipt_delivery_receipt_redacted_payload_preview_rendered_count == 0
  and .release_publication_result_receipt_delivery_receipt_payload_hash_preview_rendered_count == 0
  and .release_publication_result_receipt_delivery_receipt_payload_diff_rendered_count == 0
  and .release_publication_result_receipt_delivery_receipt_payload_summary_rendered_count == 0
  and .release_publication_result_receipt_delivery_receipt_operator_readback_text_rendered_count == 0
  and .release_publication_result_receipt_delivery_receipt_privacy_review_recorded_count == 0
  and .release_publication_result_receipt_delivery_receipt_secret_scan_recorded_count == 0
  and .release_publication_result_receipt_delivery_receipt_pii_scan_recorded_count == 0
  and .release_publication_result_receipt_delivery_receipt_raw_payload_inspected_count == 0
  and .release_publication_result_receipt_delivery_receipt_plaintext_materialized_count == 0
  and .release_publication_result_receipt_delivery_receipt_redaction_bypass_performed_count == 0
  and .release_publication_result_receipt_delivery_receipt_hash_to_payload_linked_count == 0
  and .release_publication_result_receipt_delivery_receipt_export_redacted_payload_written_count == 0
  and .release_publication_result_receipt_delivery_receipt_observability_redacted_payload_recorded_count == 0
  and .release_publication_result_receipt_delivery_receipt_dashboard_redaction_badge_exposed_count == 0
  and .release_publication_result_receipt_delivery_receipt_audit_redaction_view_exposed_count == 0
  and .release_publication_result_receipt_delivery_receipt_payload_exposure_evidence_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_acceptance_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_operator_approval_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_release_publication_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_activation_command_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_live_execution_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_install_executed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_service_restarted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_active_binary_mutated_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_release_artifact_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_public_artifact_written_count == 0
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_surfaces | length) == 18
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_surfaces | all(
    .privacy_redaction_exposure_attempted == true
    and .privacy_redaction_exposure_allowed == false
    and .privacy_redaction_exposure_request_accepted == false
    and .privacy_redaction_exposure_accepted == false
    and .privacy_redaction_exposure_recorded == false
    and .privacy_redaction_exposure_persisted == false
    and .privacy_redaction_exposure_materialized == false
    and .privacy_redaction_exposure_filesystem_written == false
    and .privacy_redaction_exposure_delivered == false
    and .privacy_redaction_exposure_exposed == false
    and .redacted_payload_preview_rendered == false
    and .payload_hash_preview_rendered == false
    and .payload_diff_rendered == false
    and .payload_summary_rendered == false
    and .operator_readback_text_rendered == false
    and .privacy_review_recorded == false
    and .secret_scan_recorded == false
    and .pii_scan_recorded == false
    and .raw_payload_inspected == false
    and .plaintext_materialized == false
    and .redaction_bypass_performed == false
    and .hash_to_payload_linked == false
    and .export_redacted_payload_written == false
    and .observability_redacted_payload_recorded == false
    and .dashboard_redaction_badge_exposed == false
    and .audit_redaction_view_exposed == false
    and .delivery_receipt_payload_exposure_evidence_exposed == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .live_execution_allowed == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .release_artifact_written == false
    and .public_artifact_written == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .live_kg_write_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .external_send_performed == false
    and .privacy_redaction_exposure_noop_confirmed == true
  ))
  and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure | length) == 34
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_persisted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_materialized == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_filesystem_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_query_result_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_file_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_log_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_dashboard_panel_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_readback_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_audit_view_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_persisted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_materialized == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_filesystem_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_redacted_payload_preview_rendered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_payload_hash_preview_rendered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_payload_diff_rendered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_payload_summary_rendered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_readback_text_rendered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_privacy_review_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_secret_scan_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_pii_scan_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_raw_payload_inspected == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_plaintext_materialized == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_redaction_bypass_performed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_hash_to_payload_linked == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_redacted_payload_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_redacted_payload_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_dashboard_redaction_badge_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_audit_redaction_view_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_payload_exposure_evidence_exposed == false
  and .operator_acceptance_recorded == false
  and .operator_approval_recorded == false
  and .release_publication_authority_derived == false
  and .activation_authority_derived == false
  and .activation_command_derived == false
  and .activation_allowed == false
  and .activation_performed == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .live_kg_write_performed == false
  and .provider_invoked == false
  and .model_invoked == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt privacy/redaction/payload-exposure denial gate passed"
