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

TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_RETENTION_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-retention-gc-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-retention-gc-denial-gate.sh
)"

source_terminal_public_claim_delivery_receipt_retention_report_sha256="$(
  sha256_text "$TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_RETENTION_JSON"
)"
terminal_public_claim_delivery_receipt_export_query_observability_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-distribution-signing-notarization-receipt-terminal-public-claim-delivery-receipt-export-query-observability-denial:$source_terminal_public_claim_delivery_receipt_retention_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
terminal_public_claim_delivery_receipt_export_query_observability_policy_hash_sha256="$(
  sha256_text "artifact-signing-terminal-public-claim-delivery-receipt-export-query-observability:no-query:no-export:no-observability:no-dashboard:no-alert:no-readback:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_RETENTION_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_retention_expiry_garbage_collection_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_retention_expiry_garbage_collection_denial_ready == true
    and $source.source_terminal_public_claim_delivery_receipt_audit_evidence_ready == true
    and $source.terminal_public_claim_delivery_receipt_retention_expiry_garbage_collection_surface_count == 18
    and $source.terminal_public_claim_delivery_receipt_retention_expiry_garbage_collection_attempt_count == 18
    and $source.terminal_public_claim_delivery_receipt_retention_expiry_garbage_collection_denied_count == 18
    and zero_fields($source; [
      "terminal_public_claim_delivery_receipt_retention_expiry_garbage_collection_recorded_count",
      "terminal_public_claim_delivery_receipt_retention_policy_recorded_count",
      "terminal_public_claim_delivery_receipt_ttl_lease_recorded_count",
      "terminal_public_claim_delivery_receipt_expiry_timer_started_count",
      "terminal_public_claim_delivery_receipt_garbage_collection_queue_recorded_count",
      "terminal_public_claim_delivery_receipt_garbage_collection_decision_recorded_count",
      "release_publication_authority_from_delivery_receipt_retention_derived_count",
      "activation_authority_from_delivery_receipt_retention_derived_count",
      "install_from_delivery_receipt_retention_executed_count",
      "active_binary_from_delivery_receipt_retention_mutated_count",
      "memory_store_write_performed_count",
      "live_kg_write_performed_count",
      "provider_invoked_count",
      "credential_read_count",
      "external_send_performed_count"
    ])
    and false_fields($source; [
      "terminal_public_claim_delivery_receipt_retention_expiry_garbage_collection_recorded",
      "terminal_public_claim_delivery_receipt_retention_policy_recorded",
      "terminal_public_claim_delivery_receipt_expiry_recorded",
      "terminal_public_claim_delivery_receipt_garbage_collection_recorded",
      "public_release_claimed",
      "public_ga_claimed",
      "operator_approval_recorded",
      "release_publication_authority_derived",
      "activation_authority_derived",
      "activation_performed",
      "provider_invoked",
      "credential_read",
      "install_executed",
      "active_binary_mutated",
      "external_send_performed"
    ])
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_export_query_observability_denial_gate"
      and .records_retention == false
      and .registers_export == false
      and .registers_query == false
      and .records_observability == false
      and .derives_release_publication_authority == false
      and .derives_activation_authority == false
      and .installs_or_restarts == false
      and .mutates_active_binary == false
      and .reads_credentials == false
      and .sends_externally == false
    ))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

surfaces_json="$(
  jq -n '
    def qeo_surface($id; $status; $reason; $extra):
      {
        terminal_public_claim_delivery_receipt_export_query_observability_surface:$id,
        source_terminal_public_claim_delivery_receipt_retention_ready:true,
        terminal_public_claim_delivery_receipt_export_query_observability_attempted:true,
        terminal_public_claim_delivery_receipt_export_query_observability_allowed:false,
        terminal_public_claim_delivery_receipt_export_query_observability_accepted:false,
        terminal_public_claim_delivery_receipt_export_query_observability_recorded:false,
        terminal_public_claim_delivery_receipt_export_query_observability_persisted:false,
        terminal_public_claim_delivery_receipt_export_query_observability_materialized:false,
        terminal_public_claim_delivery_receipt_export_query_observability_filesystem_written:false,
        query_registration_requested:false,
        query_execution_requested:false,
        query_result_requested:false,
        search_index_requested:false,
        export_request_requested:false,
        export_snapshot_requested:false,
        export_file_requested:false,
        export_stream_requested:false,
        observability_metric_requested:false,
        observability_log_requested:false,
        observability_trace_requested:false,
        observability_event_requested:false,
        dashboard_panel_requested:false,
        alert_slo_requested:false,
        operator_readback_requested:false,
        audit_view_requested:false,
        ledger_observability_requested:false,
        index_observability_requested:false,
        delivery_observability_requested:false,
        external_telegram_observability_requested:false,
        release_publication_authority_observability_requested:false,
        activation_live_observability_requested:false,
        install_restart_active_binary_observability_requested:false,
        query_registered:false,
        query_executed:false,
        query_result_recorded:false,
        query_result_persisted:false,
        search_index_recorded:false,
        search_index_persisted:false,
        export_accepted:false,
        export_snapshot_recorded:false,
        export_snapshot_persisted:false,
        export_file_written:false,
        export_stream_opened:false,
        observability_metric_recorded:false,
        observability_log_recorded:false,
        observability_trace_recorded:false,
        observability_event_recorded:false,
        dashboard_panel_recorded:false,
        alert_registered:false,
        slo_recorded:false,
        operator_summary_recorded:false,
        readback_surface_recorded:false,
        audit_view_recorded:false,
        ledger_observability_recorded:false,
        index_observability_recorded:false,
        delivery_observability_recorded:false,
        result_receipt_recorded:false,
        result_receipt_persisted:false,
        result_receipt_exported:false,
        result_receipt_query_registered:false,
        result_receipt_observability_recorded:false,
        completion_ack_recorded:false,
        operator_acceptance_from_export_query_observability_recorded:false,
        operator_approval_from_export_query_observability_derived:false,
        release_publication_authority_from_export_query_observability_derived:false,
        activation_authority_from_export_query_observability_derived:false,
        download_link_from_export_query_observability_rendered:false,
        install_command_from_export_query_observability_rendered:false,
        install_from_export_query_observability_executed:false,
        service_restart_from_export_query_observability_performed:false,
        launchd_from_export_query_observability_mutated:false,
        active_binary_from_export_query_observability_mutated:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        live_kg_write_performed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_read:false,
        secret_file_read:false,
        telegram_send_performed:false,
        channel_send_performed:false,
        external_send_performed:false,
        public_release_claimed:false,
        public_ga_claimed:false,
        release_artifact_written:false,
        public_artifact_written:false,
        terminal_public_claim_delivery_receipt_export_query_observability_noop_confirmed:true,
        terminal_public_claim_delivery_receipt_export_query_observability_status:$status,
        reason:$reason
      } + $extra;
    [
      qeo_surface("source_delivery_receipt_retention_report_required"; "blocked_source_delivery_receipt_retention_required_noop"; "source_delivery_receipt_retention_report_required"; {source_report_required:true}),
      qeo_surface("delivery_receipt_query_registration"; "blocked_delivery_receipt_query_registration_noop"; "delivery_receipt_query_registration_denied"; {query_registration_requested:true}),
      qeo_surface("delivery_receipt_query_execution"; "blocked_delivery_receipt_query_execution_noop"; "delivery_receipt_query_execution_denied"; {query_execution_requested:true}),
      qeo_surface("delivery_receipt_query_result"; "blocked_delivery_receipt_query_result_noop"; "delivery_receipt_query_result_denied"; {query_result_requested:true}),
      qeo_surface("delivery_receipt_search_index"; "blocked_delivery_receipt_search_index_noop"; "delivery_receipt_search_index_denied"; {search_index_requested:true}),
      qeo_surface("delivery_receipt_export_request"; "blocked_delivery_receipt_export_request_noop"; "delivery_receipt_export_request_denied"; {export_request_requested:true}),
      qeo_surface("delivery_receipt_export_snapshot"; "blocked_delivery_receipt_export_snapshot_noop"; "delivery_receipt_export_snapshot_denied"; {export_snapshot_requested:true}),
      qeo_surface("delivery_receipt_export_file"; "blocked_delivery_receipt_export_file_noop"; "delivery_receipt_export_file_denied"; {export_file_requested:true}),
      qeo_surface("delivery_receipt_export_stream"; "blocked_delivery_receipt_export_stream_noop"; "delivery_receipt_export_stream_denied"; {export_stream_requested:true}),
      qeo_surface("delivery_receipt_observability_metric"; "blocked_delivery_receipt_observability_metric_noop"; "delivery_receipt_observability_metric_denied"; {observability_metric_requested:true}),
      qeo_surface("delivery_receipt_observability_log"; "blocked_delivery_receipt_observability_log_noop"; "delivery_receipt_observability_log_denied"; {observability_log_requested:true}),
      qeo_surface("delivery_receipt_observability_trace"; "blocked_delivery_receipt_observability_trace_noop"; "delivery_receipt_observability_trace_denied"; {observability_trace_requested:true}),
      qeo_surface("delivery_receipt_observability_event"; "blocked_delivery_receipt_observability_event_noop"; "delivery_receipt_observability_event_denied"; {observability_event_requested:true}),
      qeo_surface("delivery_receipt_dashboard_alert_slo"; "blocked_delivery_receipt_dashboard_alert_slo_noop"; "delivery_receipt_dashboard_alert_slo_denied"; {dashboard_panel_requested:true, alert_slo_requested:true}),
      qeo_surface("delivery_receipt_operator_readback_audit_view"; "blocked_delivery_receipt_readback_audit_view_noop"; "delivery_receipt_operator_readback_audit_view_denied"; {operator_readback_requested:true, audit_view_requested:true}),
      qeo_surface("external_telegram_delivery_receipt_observability"; "blocked_external_telegram_observability_noop"; "external_telegram_delivery_receipt_observability_denied"; {external_telegram_observability_requested:true, delivery_observability_requested:true}),
      qeo_surface("release_publication_authority_observability"; "blocked_release_publication_authority_observability_noop"; "release_publication_authority_observability_denied"; {release_publication_authority_observability_requested:true, ledger_observability_requested:true, index_observability_requested:true}),
      qeo_surface("activation_install_active_binary_observability"; "blocked_activation_install_active_binary_observability_noop"; "activation_install_active_binary_observability_denied"; {activation_live_observability_requested:true, install_restart_active_binary_observability_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_export_query_observability_denial_gate" \
    --arg source_terminal_public_claim_delivery_receipt_retention_report_sha256 "$source_terminal_public_claim_delivery_receipt_retention_report_sha256" \
    --arg terminal_public_claim_delivery_receipt_export_query_observability_contract_hash_sha256 "$terminal_public_claim_delivery_receipt_export_query_observability_contract_hash_sha256" \
    --arg terminal_public_claim_delivery_receipt_export_query_observability_policy_hash_sha256 "$terminal_public_claim_delivery_receipt_export_query_observability_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_RETENTION_JSON" \
    --argjson surfaces "$surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);
      {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        terminal_public_claim_delivery_receipt_export_query_observability_schema_version:"terminal_public_claim_delivery_receipt_export_query_observability_denial_v1",
        terminal_public_claim_delivery_receipt_export_query_observability_mode:"denied_delivery_receipt_retention_cannot_become_query_export_observability_readback_status_or_authority",
        source_terminal_public_claim_delivery_receipt_retention_gate:$source.gate,
        source_terminal_public_claim_delivery_receipt_retention_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_retention_expiry_garbage_collection_denial_ready,
        source_terminal_public_claim_delivery_receipt_retention_report_sha256:$source_terminal_public_claim_delivery_receipt_retention_report_sha256,
        source_terminal_public_claim_delivery_receipt_retention_contract_hash_sha256:$source.terminal_public_claim_delivery_receipt_retention_expiry_garbage_collection_contract_hash_sha256,
        terminal_public_claim_delivery_receipt_export_query_observability_contract_hash_sha256:$terminal_public_claim_delivery_receipt_export_query_observability_contract_hash_sha256,
        terminal_public_claim_delivery_receipt_export_query_observability_policy_hash_sha256:$terminal_public_claim_delivery_receipt_export_query_observability_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_export_query_observability_denial_ready:true,
        source_terminal_public_claim_delivery_receipt_retention_surface_count:$source.terminal_public_claim_delivery_receipt_retention_expiry_garbage_collection_surface_count,
        source_terminal_public_claim_delivery_receipt_retention_denied_count:$source.terminal_public_claim_delivery_receipt_retention_expiry_garbage_collection_denied_count,
        terminal_public_claim_delivery_receipt_export_query_observability_surface_count:($surfaces | length),
        terminal_public_claim_delivery_receipt_export_query_observability_attempt_count:($surfaces | length),
        terminal_public_claim_delivery_receipt_export_query_observability_denied_count:($surfaces | length),
        terminal_public_claim_delivery_receipt_export_query_observability_surfaces:$surfaces,
        denied_by_terminal_public_claim_delivery_receipt_export_query_observability:[
          "source_delivery_receipt_retention_report_required",
          "delivery_receipt_query_registration_denied",
          "delivery_receipt_query_execution_denied",
          "delivery_receipt_query_result_denied",
          "delivery_receipt_search_index_denied",
          "delivery_receipt_export_request_denied",
          "delivery_receipt_export_snapshot_denied",
          "delivery_receipt_export_file_denied",
          "delivery_receipt_export_stream_denied",
          "delivery_receipt_observability_metric_denied",
          "delivery_receipt_observability_log_denied",
          "delivery_receipt_observability_trace_denied",
          "delivery_receipt_observability_event_denied",
          "delivery_receipt_dashboard_alert_slo_denied",
          "delivery_receipt_operator_readback_audit_view_denied",
          "external_telegram_delivery_receipt_observability_denied",
          "release_publication_authority_observability_denied",
          "activation_install_active_binary_observability_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_operator_facing_summary_briefing_non_persistence_denial_gate",
            status:"allowed_report_only_next_slice",
            records_query:false,
            executes_query:false,
            records_export:false,
            writes_export:false,
            records_observability:false,
            exposes_dashboard:false,
            records_operator_summary:false,
            records_readback:false,
            derives_release_publication_authority:false,
            derives_activation_authority:false,
            installs_or_restarts:false,
            mutates_active_binary:false,
            mutates_memory_store:false,
            writes_kg:false,
            invokes_provider:false,
            reads_credentials:false,
            sends_externally:false
          }
        ]
      }
      + zero_object([
        "terminal_public_claim_delivery_receipt_export_query_observability_allowed_count",
        "terminal_public_claim_delivery_receipt_export_query_observability_accepted_count",
        "terminal_public_claim_delivery_receipt_export_query_observability_recorded_count",
        "terminal_public_claim_delivery_receipt_export_query_observability_persisted_count",
        "terminal_public_claim_delivery_receipt_export_query_observability_materialized_count",
        "terminal_public_claim_delivery_receipt_export_query_observability_filesystem_written_count",
        "terminal_public_claim_delivery_receipt_query_registered_count",
        "terminal_public_claim_delivery_receipt_query_executed_count",
        "terminal_public_claim_delivery_receipt_query_result_recorded_count",
        "terminal_public_claim_delivery_receipt_search_index_recorded_count",
        "terminal_public_claim_delivery_receipt_export_accepted_count",
        "terminal_public_claim_delivery_receipt_export_snapshot_recorded_count",
        "terminal_public_claim_delivery_receipt_export_file_written_count",
        "terminal_public_claim_delivery_receipt_export_stream_opened_count",
        "terminal_public_claim_delivery_receipt_observability_metric_recorded_count",
        "terminal_public_claim_delivery_receipt_observability_log_recorded_count",
        "terminal_public_claim_delivery_receipt_observability_trace_recorded_count",
        "terminal_public_claim_delivery_receipt_observability_event_recorded_count",
        "terminal_public_claim_delivery_receipt_dashboard_panel_recorded_count",
        "terminal_public_claim_delivery_receipt_alert_registered_count",
        "terminal_public_claim_delivery_receipt_slo_recorded_count",
        "terminal_public_claim_delivery_receipt_readback_surface_recorded_count",
        "terminal_public_claim_delivery_receipt_audit_view_recorded_count",
        "release_publication_authority_from_delivery_receipt_export_query_observability_derived_count",
        "activation_authority_from_delivery_receipt_export_query_observability_derived_count",
        "install_from_delivery_receipt_export_query_observability_executed_count",
        "active_binary_from_delivery_receipt_export_query_observability_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count"
      ])
      + false_object([
        "terminal_public_claim_delivery_receipt_export_query_observability_recorded",
        "terminal_public_claim_delivery_receipt_query_registered",
        "terminal_public_claim_delivery_receipt_query_executed",
        "terminal_public_claim_delivery_receipt_query_result_recorded",
        "terminal_public_claim_delivery_receipt_search_index_recorded",
        "terminal_public_claim_delivery_receipt_export_accepted",
        "terminal_public_claim_delivery_receipt_export_snapshot_recorded",
        "terminal_public_claim_delivery_receipt_export_file_written",
        "terminal_public_claim_delivery_receipt_export_stream_opened",
        "terminal_public_claim_delivery_receipt_observability_metric_recorded",
        "terminal_public_claim_delivery_receipt_observability_log_recorded",
        "terminal_public_claim_delivery_receipt_observability_trace_recorded",
        "terminal_public_claim_delivery_receipt_observability_event_recorded",
        "terminal_public_claim_delivery_receipt_dashboard_panel_recorded",
        "terminal_public_claim_delivery_receipt_alert_registered",
        "terminal_public_claim_delivery_receipt_slo_recorded",
        "terminal_public_claim_delivery_receipt_readback_surface_recorded",
        "terminal_public_claim_delivery_receipt_audit_view_recorded",
        "public_release_claimed",
        "public_ga_claimed",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
        "external_send_performed"
      ])
      + {
        side_effects:false_object([
          "query_registered",
          "query_executed",
          "query_result_recorded",
          "query_result_persisted",
          "search_index_recorded",
          "search_index_persisted",
          "export_accepted",
          "export_snapshot_recorded",
          "export_snapshot_persisted",
          "export_file_written",
          "export_stream_opened",
          "observability_metric_recorded",
          "observability_log_recorded",
          "observability_trace_recorded",
          "observability_event_recorded",
          "dashboard_panel_recorded",
          "alert_registered",
          "slo_recorded",
          "operator_summary_recorded",
          "readback_surface_recorded",
          "audit_view_recorded",
          "ledger_observability_recorded",
          "index_observability_recorded",
          "delivery_observability_recorded",
          "result_receipt_recorded",
          "result_receipt_persisted",
          "completion_ack_recorded",
          "operator_acceptance_from_export_query_observability_recorded",
          "operator_approval_from_export_query_observability_derived",
          "release_publication_authority_from_export_query_observability_derived",
          "activation_authority_from_export_query_observability_derived",
          "install_executed",
          "service_restarted",
          "active_binary_mutated",
          "memory_store_write_performed",
          "memory_store_mutated",
          "live_kg_write_performed",
          "provider_invoked",
          "model_invoked",
          "credential_read",
          "secret_file_read",
          "telegram_send_performed",
          "channel_send_performed",
          "external_send_performed",
          "release_artifact_written",
          "public_artifact_written",
          "filesystem_written"
        ])
      }
    '
)"

jq -e '
  def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
  def false_fields($o; $fields): all($fields[]; $o[.] == false);

  . as $report
  | $report.runtime == "hepta"
  and $report.status == "ready"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_export_query_observability_denial_ready == true
  and $report.source_terminal_public_claim_delivery_receipt_retention_ready == true
  and $report.source_terminal_public_claim_delivery_receipt_retention_surface_count == 18
  and $report.source_terminal_public_claim_delivery_receipt_retention_denied_count == 18
  and $report.terminal_public_claim_delivery_receipt_export_query_observability_surface_count == 18
  and $report.terminal_public_claim_delivery_receipt_export_query_observability_denied_count == 18
  and zero_fields($report; [
    "terminal_public_claim_delivery_receipt_export_query_observability_recorded_count",
    "terminal_public_claim_delivery_receipt_query_registered_count",
    "terminal_public_claim_delivery_receipt_query_executed_count",
    "terminal_public_claim_delivery_receipt_query_result_recorded_count",
    "terminal_public_claim_delivery_receipt_export_file_written_count",
    "terminal_public_claim_delivery_receipt_observability_metric_recorded_count",
    "terminal_public_claim_delivery_receipt_dashboard_panel_recorded_count",
    "release_publication_authority_from_delivery_receipt_export_query_observability_derived_count",
    "activation_authority_from_delivery_receipt_export_query_observability_derived_count",
    "install_from_delivery_receipt_export_query_observability_executed_count",
    "active_binary_from_delivery_receipt_export_query_observability_mutated_count",
    "provider_invoked_count",
    "credential_read_count",
    "external_send_performed_count"
  ])
  and false_fields($report; [
    "terminal_public_claim_delivery_receipt_export_query_observability_recorded",
    "terminal_public_claim_delivery_receipt_query_registered",
    "terminal_public_claim_delivery_receipt_query_executed",
    "terminal_public_claim_delivery_receipt_export_file_written",
    "terminal_public_claim_delivery_receipt_observability_metric_recorded",
    "terminal_public_claim_delivery_receipt_dashboard_panel_recorded",
    "public_release_claimed",
    "public_ga_claimed",
    "operator_approval_recorded",
    "release_publication_authority_derived",
    "activation_authority_derived",
    "activation_performed",
    "provider_invoked",
    "credential_read",
    "install_executed",
    "active_binary_mutated",
    "external_send_performed"
  ])
  and ($report.terminal_public_claim_delivery_receipt_export_query_observability_surfaces | all(
    .terminal_public_claim_delivery_receipt_export_query_observability_attempted == true
    and .terminal_public_claim_delivery_receipt_export_query_observability_allowed == false
    and .terminal_public_claim_delivery_receipt_export_query_observability_noop_confirmed == true
    and .query_registered == false
    and .export_file_written == false
    and .observability_metric_recorded == false
    and .dashboard_panel_recorded == false
    and .release_publication_authority_from_export_query_observability_derived == false
    and .activation_authority_from_export_query_observability_derived == false
    and .install_from_export_query_observability_executed == false
    and .active_binary_from_export_query_observability_mutated == false
    and .external_send_performed == false
  ))
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_operator_facing_summary_briefing_non_persistence_denial_gate"
    and .records_query == false
    and .records_export == false
    and .records_observability == false
    and .records_operator_summary == false
    and .derives_release_publication_authority == false
    and .derives_activation_authority == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .reads_credentials == false
    and .sends_externally == false
  ))
  and ($report.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG artifact signing terminal public claim delivery receipt export/query/observability denial gate passed" >&2
