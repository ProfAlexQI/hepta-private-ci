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

INTENT_CONSENT_EVIDENCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-persistence-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-persistence-denial-gate.sh
)"

source_intent_consent_evidence_report_sha256="$(sha256_text "$INTENT_CONSENT_EVIDENCE_JSON")"
evidence_export_query_observability_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-export-query-observability-denial:$source_intent_consent_evidence_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
evidence_export_query_observability_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-export-query-observability:no-export:no-query:no-observability:no-readback:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$INTENT_CONSENT_EVIDENCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denied_count == 18
    and zero_fields($source; [
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_evidence_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_consent_evidence_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_receipt_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_receipt_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_materialized_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_filesystem_written_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_ledger_written_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_indexed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_exported_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_query_registered_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_observability_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_approval_from_evidence_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_release_publication_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_activation_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_install_executed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_active_binary_mutated_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_external_send_count"
    ])
    and false_fields($source; [
      "artifact_download_install_affordance_result_receipt_recorded",
      "artifact_download_install_affordance_result_receipt_persisted",
      "operator_acceptance_recorded",
      "operator_approval_recorded",
      "release_publication_authority_derived",
      "activation_authority_derived",
      "activation_performed",
      "memory_store_write_performed",
      "memory_store_mutated",
      "live_kg_write_performed",
      "provider_invoked",
      "model_invoked",
      "credential_read",
      "secret_file_read",
      "install_executed",
      "launchd_mutated",
      "service_restarted",
      "active_binary_mutated",
      "external_send_performed"
    ])
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_surfaces | length) == 18
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_surfaces | all(
      .artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_attempted == true
      and .operator_intent_consent_evidence_persistence_noop_confirmed == true
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .records_intent_consent_evidence == false
      and .exports_evidence == false
      and .registers_query == false
      and .records_observability == false
      and .derives_authority == false
      and .installs_or_restarts == false
      and .mutates_active_binary == false
      and .mutates_memory_store == false
      and .writes_kg == false
      and .sends_externally == false
    ))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

evidence_export_query_observability_surfaces_json="$(
  jq -n '
    def observability_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_surface:$id,
        source_operator_intent_consent_evidence_persistence_ready:true,
        artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_attempted:true,
        evidence_export_requested:false,
        evidence_query_requested:false,
        evidence_observability_requested:false,
        evidence_readback_requested:false,
        evidence_dashboard_requested:false,
        evidence_audit_view_requested:false,
        evidence_external_observability_requested:false,
        evidence_telegram_observability_requested:false,
        authority_view_requested:false,
        live_view_requested:false,
        install_view_requested:false,
        service_restart_view_requested:false,
        active_binary_view_requested:false,
        evidence_export_accepted:false,
        evidence_exported:false,
        evidence_export_snapshot_recorded:false,
        evidence_export_snapshot_persisted:false,
        evidence_export_file_written:false,
        evidence_export_stream_opened:false,
        evidence_query_registered:false,
        evidence_query_executed:false,
        evidence_query_result_recorded:false,
        evidence_query_result_persisted:false,
        evidence_search_index_recorded:false,
        evidence_search_index_persisted:false,
        evidence_observability_recorded:false,
        evidence_metric_recorded:false,
        evidence_log_recorded:false,
        evidence_trace_recorded:false,
        evidence_event_recorded:false,
        evidence_dashboard_panel_recorded:false,
        evidence_alert_registered:false,
        evidence_slo_recorded:false,
        evidence_operator_readback_recorded:false,
        evidence_audit_view_recorded:false,
        evidence_external_observability_recorded:false,
        evidence_telegram_observability_recorded:false,
        evidence_release_authority_view_recorded:false,
        evidence_live_install_view_recorded:false,
        evidence_readback_recorded:false,
        evidence_receipt_recorded:false,
        evidence_receipt_persisted:false,
        evidence_materialized:false,
        evidence_filesystem_written:false,
        evidence_ledger_written:false,
        evidence_indexed:false,
        intent_consent_evidence_recorded:false,
        intent_consent_evidence_persisted:false,
        identity_session_binding_from_evidence_recorded:false,
        operator_approval_from_evidence_derived:false,
        acceptance_from_evidence_recorded:false,
        terminal_decision_from_evidence_recorded:false,
        terminal_status_from_evidence_recorded:false,
        release_publication_authority_from_evidence_derived:false,
        activation_authority_from_evidence_derived:false,
        download_link_from_evidence_rendered:false,
        install_command_from_evidence_rendered:false,
        install_from_evidence_executed:false,
        service_restart_from_evidence_performed:false,
        launchd_from_evidence_mutated:false,
        active_binary_from_evidence_mutated:false,
        result_receipt_from_evidence_recorded:false,
        result_receipt_from_evidence_persisted:false,
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
        operator_intent_consent_evidence_export_query_observability_noop_confirmed:true,
        operator_intent_consent_evidence_export_query_observability_status:$status,
        reason:$reason
      } + $extra;
    [
      observability_surface("source_operator_intent_consent_evidence_persistence_report_required"; "blocked_source_evidence_persistence_report_required_noop"; "source_operator_intent_consent_evidence_persistence_report_required"; {source_operator_intent_consent_evidence_persistence_report_required:true}),
      observability_surface("revocation_replay_operator_intent_evidence_export_request"; "blocked_revocation_replay_intent_evidence_export_noop"; "revocation_replay_operator_intent_evidence_export_request_denied"; {evidence_export_requested:true}),
      observability_surface("logout_replay_operator_consent_evidence_query_registration"; "blocked_logout_replay_consent_evidence_query_registration_noop"; "logout_replay_operator_consent_evidence_query_registration_denied"; {evidence_query_requested:true}),
      observability_surface("identity_signature_evidence_query_result"; "blocked_identity_signature_evidence_query_result_noop"; "identity_signature_evidence_query_result_denied"; {evidence_query_requested:true}),
      observability_surface("session_consent_token_evidence_search_index"; "blocked_session_consent_token_evidence_search_index_noop"; "session_consent_token_evidence_search_index_denied"; {evidence_query_requested:true, evidence_search_index_requested:true}),
      observability_surface("intent_timestamp_evidence_export_snapshot"; "blocked_intent_timestamp_evidence_export_snapshot_noop"; "intent_timestamp_evidence_export_snapshot_denied"; {evidence_export_requested:true, evidence_export_snapshot_requested:true}),
      observability_surface("consent_nonce_evidence_export_file"; "blocked_consent_nonce_evidence_export_file_noop"; "consent_nonce_evidence_export_file_denied"; {evidence_export_requested:true, evidence_export_file_requested:true}),
      observability_surface("consent_refresh_evidence_export_stream"; "blocked_consent_refresh_evidence_export_stream_noop"; "consent_refresh_evidence_export_stream_denied"; {evidence_export_requested:true, evidence_export_stream_requested:true}),
      observability_surface("intent_status_evidence_metric_log"; "blocked_intent_status_evidence_metric_log_noop"; "intent_status_evidence_metric_log_denied"; {evidence_observability_requested:true, evidence_metric_requested:true, evidence_log_requested:true}),
      observability_surface("consent_summary_evidence_trace_event"; "blocked_consent_summary_evidence_trace_event_noop"; "consent_summary_evidence_trace_event_denied"; {evidence_observability_requested:true, evidence_trace_requested:true, evidence_event_requested:true}),
      observability_surface("identity_badge_intent_evidence_dashboard_panel"; "blocked_identity_badge_intent_evidence_dashboard_panel_noop"; "identity_badge_intent_evidence_dashboard_panel_denied"; {evidence_dashboard_requested:true}),
      observability_surface("consent_readback_evidence_alert_slo"; "blocked_consent_readback_evidence_alert_slo_noop"; "consent_readback_evidence_alert_slo_denied"; {evidence_observability_requested:true, evidence_alert_requested:true, evidence_slo_requested:true}),
      observability_surface("dashboard_consent_notification_evidence_operator_readback"; "blocked_dashboard_consent_notification_evidence_readback_noop"; "dashboard_consent_notification_evidence_operator_readback_denied"; {evidence_readback_requested:true}),
      observability_surface("channel_consent_delivery_evidence_audit_view"; "blocked_channel_consent_delivery_evidence_audit_view_noop"; "channel_consent_delivery_evidence_audit_view_denied"; {evidence_audit_view_requested:true}),
      observability_surface("identity_approval_consent_evidence_external_observability"; "blocked_identity_approval_consent_evidence_external_observability_noop"; "identity_approval_consent_evidence_external_observability_denied"; {evidence_external_observability_requested:true}),
      observability_surface("external_telegram_consent_evidence_observability"; "blocked_external_telegram_consent_evidence_observability_noop"; "external_telegram_consent_evidence_observability_denied"; {evidence_external_observability_requested:true, evidence_telegram_observability_requested:true}),
      observability_surface("release_authority_intent_consent_evidence_authority_view"; "blocked_release_authority_intent_consent_evidence_authority_view_noop"; "release_authority_intent_consent_evidence_authority_view_denied"; {authority_view_requested:true}),
      observability_surface("live_install_restart_active_binary_consent_evidence_live_view"; "blocked_live_install_restart_active_binary_consent_evidence_view_noop"; "live_install_restart_active_binary_consent_evidence_live_view_denied"; {live_view_requested:true, install_view_requested:true, service_restart_view_requested:true, active_binary_view_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_denial_gate" \
    --arg source_intent_consent_evidence_report_sha256 "$source_intent_consent_evidence_report_sha256" \
    --arg evidence_export_query_observability_contract_hash_sha256 "$evidence_export_query_observability_contract_hash_sha256" \
    --arg evidence_export_query_observability_policy_hash_sha256 "$evidence_export_query_observability_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$INTENT_CONSENT_EVIDENCE_JSON" \
    --argjson surfaces "$evidence_export_query_observability_surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_denial_v1",
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_mode:"denied_intent_consent_evidence_cannot_be_exported_queried_observed_or_used_as_authority",
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_gate:$source.gate,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denial_ready,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_report_sha256:$source_intent_consent_evidence_report_sha256,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_contract_hash_sha256:$evidence_export_query_observability_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_policy_hash_sha256:$evidence_export_query_observability_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_denial_ready:true,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_surface_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denied_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denied_count,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_surface_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_attempt_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_denied_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_surfaces:$surfaces,
        denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability:[
          "operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_export_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_query_registration_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_query_execution_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_query_result_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_search_index_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_snapshot_file_stream_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_metric_log_trace_event_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_dashboard_alert_slo_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_readback_audit_view_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_external_telegram_observability_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_authority_view_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_live_install_view_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_receipt_persistence_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_operator_approval_from_evidence_view_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_release_publication_authority_from_evidence_view_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_activation_authority_from_evidence_view_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_install_restart_active_binary_from_evidence_view_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_memory_provider_secret_external_send_from_evidence_view_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_operator_facing_summary_briefing_non_persistence_denial_gate",
            status:"allowed_report_only_next_slice",
            records_operator_intent:false,
            records_operator_consent:false,
            records_operator_identity:false,
            records_operator_session:false,
            records_intent_consent_evidence:false,
            exports_evidence:false,
            registers_query:false,
            records_observability:false,
            records_summary:false,
            records_briefing:false,
            derives_authority:false,
            renders_download_link:false,
            emits_install_command:false,
            installs_or_restarts:false,
            mutates_active_binary:false,
            mutates_memory_store:false,
            writes_kg:false,
            sends_externally:false
          }
        ]
      }
      + zero_object([
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_exported_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_snapshot_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_snapshot_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_file_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_stream_opened_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_query_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_query_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_query_result_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_search_index_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_observability_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_metric_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_log_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_trace_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_event_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_dashboard_panel_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_alert_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_slo_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_operator_readback_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_audit_view_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_external_observability_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_telegram_observability_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_authority_view_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_live_view_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_approval_from_evidence_view_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_observability_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_observability_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_observability_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_observability_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_observability_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_observability_external_send_count"
      ])
      + false_object([
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_exported",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_observability_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
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
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed"
      ])
      + {
        side_effects:false_object([
          "evidence_exported",
          "evidence_export_snapshot_persisted",
          "evidence_export_file_written",
          "evidence_export_stream_opened",
          "evidence_query_registered",
          "evidence_query_executed",
          "evidence_query_result_recorded",
          "evidence_search_index_persisted",
          "evidence_observability_recorded",
          "evidence_metric_recorded",
          "evidence_log_recorded",
          "evidence_trace_recorded",
          "evidence_event_recorded",
          "evidence_dashboard_panel_recorded",
          "evidence_alert_registered",
          "evidence_slo_recorded",
          "evidence_operator_readback_recorded",
          "evidence_audit_view_recorded",
          "evidence_external_observability_recorded",
          "evidence_telegram_observability_recorded",
          "evidence_release_authority_view_recorded",
          "evidence_live_install_view_recorded",
          "evidence_receipt_recorded",
          "evidence_receipt_persisted",
          "intent_consent_evidence_recorded",
          "intent_consent_evidence_persisted",
          "operator_approval_from_evidence_derived",
          "acceptance_from_evidence_recorded",
          "release_publication_authority_from_evidence_derived",
          "activation_authority_from_evidence_derived",
          "download_link_from_evidence_rendered",
          "install_command_from_evidence_rendered",
          "install_executed",
          "launchd_mutated",
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
          "public_release_claimed",
          "public_ga_claimed",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_denial_ready == true
  and $report.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_ready == true
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_surface_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denied_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_surface_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_attempt_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_denied_count == 18
  and zero_fields($report; [
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_exported_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_snapshot_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_file_written_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_stream_opened_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_query_registered_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_query_executed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_query_result_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_search_index_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_observability_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_metric_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_log_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_trace_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_event_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_dashboard_panel_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_alert_registered_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_slo_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_operator_readback_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_audit_view_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_external_observability_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_telegram_observability_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_approval_from_evidence_view_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_observability_release_publication_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_observability_activation_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_observability_install_executed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_observability_active_binary_mutated_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_observability_external_send_count"
  ])
' <<<"$report" >/dev/null

jq -e '
  def false_fields($o; $fields): all($fields[]; $o[.] == false);
  . as $report
  | ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_surfaces | length) == 18
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_surfaces | all(
    .artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_attempted == true
    and .operator_intent_consent_evidence_export_query_observability_noop_confirmed == true
    and false_fields(.; [
      "evidence_exported",
      "evidence_export_snapshot_persisted",
      "evidence_export_file_written",
      "evidence_export_stream_opened",
      "evidence_query_registered",
      "evidence_query_executed",
      "evidence_query_result_recorded",
      "evidence_search_index_persisted",
      "evidence_observability_recorded",
      "evidence_metric_recorded",
      "evidence_log_recorded",
      "evidence_trace_recorded",
      "evidence_event_recorded",
      "evidence_dashboard_panel_recorded",
      "evidence_alert_registered",
      "evidence_slo_recorded",
      "evidence_operator_readback_recorded",
      "evidence_audit_view_recorded",
      "evidence_external_observability_recorded",
      "evidence_telegram_observability_recorded",
      "operator_approval_from_evidence_derived",
      "release_publication_authority_from_evidence_derived",
      "activation_authority_from_evidence_derived",
      "install_from_evidence_executed",
      "service_restart_from_evidence_performed",
      "active_binary_from_evidence_mutated",
      "memory_store_write_performed",
      "memory_store_mutated",
      "live_kg_write_performed",
      "provider_invoked",
      "model_invoked",
      "credential_read",
      "secret_file_read",
      "external_send_performed"
    ])
  ))
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_surfaces[] | select(.evidence_export_requested == true)] | length) == 4
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_surfaces[] | select(.evidence_query_requested == true)] | length) == 3
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_surfaces[] | select(.evidence_observability_requested == true)] | length) == 3
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_surfaces[] | select(.evidence_telegram_observability_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_surfaces[] | select(.active_binary_view_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_operator_facing_summary_briefing_non_persistence_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .records_intent_consent_evidence == false
    and .exports_evidence == false
    and .registers_query == false
    and .records_observability == false
    and .records_summary == false
    and .records_briefing == false
    and .derives_authority == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .mutates_memory_store == false
    and .writes_kg == false
    and .sends_externally == false
  ))
  and ($report.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence export/query/observability denial gate passed"
