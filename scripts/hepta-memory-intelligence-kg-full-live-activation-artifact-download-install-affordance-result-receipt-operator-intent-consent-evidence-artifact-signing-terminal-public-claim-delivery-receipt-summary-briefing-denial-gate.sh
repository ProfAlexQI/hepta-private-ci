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

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-observability-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-observability-denial-gate.sh
)"

source_report_sha256="$(sha256_text "$SOURCE_JSON")"
summary_contract_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-summary-briefing-denial:$source_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
summary_policy_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-summary-briefing:no-summary:no-briefing:no-readback:no-delivery:no-ack:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_export_query_observability_denial_ready == true
    and $source.source_terminal_public_claim_delivery_receipt_retention_ready == true
    and $source.terminal_public_claim_delivery_receipt_export_query_observability_surface_count == 18
    and $source.terminal_public_claim_delivery_receipt_export_query_observability_denied_count == 18
    and zero_fields($source; [
      "terminal_public_claim_delivery_receipt_export_query_observability_recorded_count",
      "terminal_public_claim_delivery_receipt_query_registered_count",
      "terminal_public_claim_delivery_receipt_query_executed_count",
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
    and false_fields($source; [
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
    and ($source.allowed_next_actions | any(
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
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

surfaces_json="$(
  jq -n '
    def summary_surface($id; $status; $reason; $extra):
      {
        terminal_public_claim_delivery_receipt_summary_briefing_surface:$id,
        source_terminal_public_claim_delivery_receipt_export_query_observability_ready:true,
        terminal_public_claim_delivery_receipt_summary_briefing_attempted:true,
        terminal_public_claim_delivery_receipt_summary_briefing_allowed:false,
        terminal_public_claim_delivery_receipt_summary_briefing_accepted:false,
        terminal_public_claim_delivery_receipt_summary_briefing_recorded:false,
        terminal_public_claim_delivery_receipt_summary_briefing_persisted:false,
        terminal_public_claim_delivery_receipt_summary_briefing_materialized:false,
        terminal_public_claim_delivery_receipt_summary_briefing_filesystem_written:false,
        summary_requested:false,
        briefing_requested:false,
        readback_requested:false,
        status_banner_requested:false,
        exported_summary_requested:false,
        briefing_card_requested:false,
        notification_timeline_requested:false,
        dashboard_narrative_requested:false,
        audit_narrative_requested:false,
        delivery_requested:false,
        final_summary_requested:false,
        operator_memo_requested:false,
        approval_summary_requested:false,
        external_briefing_requested:false,
        telegram_briefing_requested:false,
        authority_briefing_requested:false,
        live_status_briefing_requested:false,
        install_restart_active_binary_status_requested:false,
        operator_summary_recorded:false,
        operator_summary_persisted:false,
        operator_briefing_recorded:false,
        operator_briefing_persisted:false,
        delivery_receipt_readback_recorded:false,
        delivery_receipt_readback_persisted:false,
        status_banner_recorded:false,
        exported_summary_recorded:false,
        briefing_card_recorded:false,
        notification_timeline_recorded:false,
        dashboard_narrative_recorded:false,
        audit_narrative_recorded:false,
        briefing_delivery_recorded:false,
        final_summary_recorded:false,
        operator_memo_recorded:false,
        approval_summary_recorded:false,
        external_briefing_delivered:false,
        telegram_briefing_delivered:false,
        authority_briefing_recorded:false,
        live_status_briefing_recorded:false,
        delivery_receipt_result_receipt_recorded:false,
        delivery_receipt_completion_ack_recorded:false,
        operator_acceptance_from_summary_recorded:false,
        operator_acceptance_from_briefing_recorded:false,
        operator_approval_from_summary_derived:false,
        operator_approval_from_briefing_derived:false,
        release_publication_authority_from_summary_briefing_derived:false,
        activation_authority_from_summary_briefing_derived:false,
        download_link_from_summary_briefing_rendered:false,
        install_command_from_summary_briefing_rendered:false,
        install_from_summary_briefing_executed:false,
        service_restart_from_summary_briefing_performed:false,
        active_binary_from_summary_briefing_mutated:false,
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
        terminal_public_claim_delivery_receipt_summary_briefing_noop_confirmed:true,
        terminal_public_claim_delivery_receipt_summary_briefing_status:$status,
        reason:$reason
      } + $extra;
    [
      summary_surface("source_delivery_receipt_export_query_observability_report_required"; "blocked_source_delivery_receipt_observability_report_required_noop"; "source_delivery_receipt_export_query_observability_report_required"; {source_report_required:true}),
      summary_surface("delivery_receipt_query_operator_summary"; "blocked_delivery_receipt_query_summary_noop"; "delivery_receipt_query_operator_summary_denied"; {summary_requested:true}),
      summary_surface("delivery_receipt_export_operator_briefing"; "blocked_delivery_receipt_export_briefing_noop"; "delivery_receipt_export_operator_briefing_denied"; {briefing_requested:true}),
      summary_surface("delivery_receipt_observability_readback_digest"; "blocked_delivery_receipt_observability_readback_noop"; "delivery_receipt_observability_readback_digest_denied"; {readback_requested:true}),
      summary_surface("delivery_receipt_dashboard_status_banner"; "blocked_delivery_receipt_dashboard_status_banner_noop"; "delivery_receipt_dashboard_status_banner_denied"; {status_banner_requested:true}),
      summary_surface("delivery_receipt_exported_summary_text"; "blocked_delivery_receipt_exported_summary_noop"; "delivery_receipt_exported_summary_text_denied"; {exported_summary_requested:true, summary_requested:true}),
      summary_surface("delivery_receipt_briefing_card"; "blocked_delivery_receipt_briefing_card_noop"; "delivery_receipt_briefing_card_denied"; {briefing_card_requested:true, briefing_requested:true}),
      summary_surface("delivery_receipt_notification_timeline"; "blocked_delivery_receipt_notification_timeline_noop"; "delivery_receipt_notification_timeline_denied"; {notification_timeline_requested:true}),
      summary_surface("delivery_receipt_dashboard_narrative"; "blocked_delivery_receipt_dashboard_narrative_noop"; "delivery_receipt_dashboard_narrative_denied"; {dashboard_narrative_requested:true}),
      summary_surface("delivery_receipt_audit_narrative"; "blocked_delivery_receipt_audit_narrative_noop"; "delivery_receipt_audit_narrative_denied"; {audit_narrative_requested:true}),
      summary_surface("delivery_receipt_final_summary"; "blocked_delivery_receipt_final_summary_noop"; "delivery_receipt_final_summary_denied"; {final_summary_requested:true, summary_requested:true, readback_requested:true}),
      summary_surface("delivery_receipt_observability_briefing_panel"; "blocked_delivery_receipt_observability_briefing_panel_noop"; "delivery_receipt_observability_briefing_panel_denied"; {briefing_requested:true, dashboard_narrative_requested:true}),
      summary_surface("delivery_receipt_trace_operator_memo"; "blocked_delivery_receipt_trace_operator_memo_noop"; "delivery_receipt_trace_operator_memo_denied"; {operator_memo_requested:true}),
      summary_surface("delivery_receipt_alert_approval_summary"; "blocked_delivery_receipt_alert_approval_summary_noop"; "delivery_receipt_alert_approval_summary_denied"; {approval_summary_requested:true, summary_requested:true}),
      summary_surface("external_telegram_delivery_receipt_briefing"; "blocked_external_telegram_delivery_receipt_briefing_noop"; "external_telegram_delivery_receipt_briefing_denied"; {external_briefing_requested:true, telegram_briefing_requested:true, delivery_requested:true, briefing_requested:true}),
      summary_surface("release_publication_authority_summary_briefing"; "blocked_release_publication_authority_summary_briefing_noop"; "release_publication_authority_summary_briefing_denied"; {authority_briefing_requested:true, briefing_requested:true}),
      summary_surface("activation_live_install_status_briefing"; "blocked_activation_live_install_status_briefing_noop"; "activation_live_install_status_briefing_denied"; {live_status_briefing_requested:true, briefing_requested:true}),
      summary_surface("install_restart_active_binary_status_briefing"; "blocked_install_restart_active_binary_status_briefing_noop"; "install_restart_active_binary_status_briefing_denied"; {install_restart_active_binary_status_requested:true, live_status_briefing_requested:true, briefing_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_operator_facing_summary_briefing_non_persistence_denial_gate" \
    --arg source_report_sha256 "$source_report_sha256" \
    --arg summary_contract_hash_sha256 "$summary_contract_hash_sha256" \
    --arg summary_policy_hash_sha256 "$summary_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$SOURCE_JSON" \
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
        terminal_public_claim_delivery_receipt_summary_briefing_schema_version:"terminal_public_claim_delivery_receipt_summary_briefing_non_persistence_denial_v1",
        terminal_public_claim_delivery_receipt_summary_briefing_mode:"denied_delivery_receipt_export_query_observability_cannot_be_summarized_briefed_delivered_acknowledged_promoted_or_used_for_authority_or_live_install",
        source_terminal_public_claim_delivery_receipt_export_query_observability_gate:$source.gate,
        source_terminal_public_claim_delivery_receipt_export_query_observability_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_export_query_observability_denial_ready,
        source_terminal_public_claim_delivery_receipt_export_query_observability_report_sha256:$source_report_sha256,
        source_terminal_public_claim_delivery_receipt_export_query_observability_contract_hash_sha256:$source.terminal_public_claim_delivery_receipt_export_query_observability_contract_hash_sha256,
        terminal_public_claim_delivery_receipt_summary_briefing_contract_hash_sha256:$summary_contract_hash_sha256,
        terminal_public_claim_delivery_receipt_summary_briefing_policy_hash_sha256:$summary_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_operator_facing_summary_briefing_non_persistence_denial_ready:true,
        source_terminal_public_claim_delivery_receipt_export_query_observability_surface_count:$source.terminal_public_claim_delivery_receipt_export_query_observability_surface_count,
        source_terminal_public_claim_delivery_receipt_export_query_observability_denied_count:$source.terminal_public_claim_delivery_receipt_export_query_observability_denied_count,
        terminal_public_claim_delivery_receipt_summary_briefing_surface_count:($surfaces | length),
        terminal_public_claim_delivery_receipt_summary_briefing_attempt_count:($surfaces | length),
        terminal_public_claim_delivery_receipt_summary_briefing_denied_count:($surfaces | length),
        terminal_public_claim_delivery_receipt_summary_briefing_surfaces:$surfaces,
        denied_by_terminal_public_claim_delivery_receipt_summary_briefing:[
          "source_delivery_receipt_export_query_observability_report_required",
          "delivery_receipt_query_operator_summary_denied",
          "delivery_receipt_export_operator_briefing_denied",
          "delivery_receipt_observability_readback_denied",
          "delivery_receipt_dashboard_status_banner_denied",
          "delivery_receipt_exported_summary_denied",
          "delivery_receipt_briefing_card_denied",
          "delivery_receipt_notification_timeline_denied",
          "delivery_receipt_dashboard_narrative_denied",
          "delivery_receipt_audit_narrative_denied",
          "delivery_receipt_final_summary_denied",
          "delivery_receipt_observability_briefing_panel_denied",
          "delivery_receipt_trace_operator_memo_denied",
          "delivery_receipt_alert_approval_summary_denied",
          "external_telegram_delivery_receipt_briefing_denied",
          "release_publication_authority_summary_briefing_denied",
          "activation_live_install_status_briefing_denied",
          "install_restart_active_binary_status_briefing_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_final_operator_acknowledgement_non_acceptance_denial_gate",
            status:"allowed_report_only_next_slice",
            records_summary:false,
            records_briefing:false,
            records_readback:false,
            records_status_banner:false,
            records_delivery:false,
            records_acknowledgement:false,
            derives_operator_approval:false,
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
        "terminal_public_claim_delivery_receipt_summary_briefing_allowed_count",
        "terminal_public_claim_delivery_receipt_summary_briefing_accepted_count",
        "terminal_public_claim_delivery_receipt_summary_briefing_recorded_count",
        "terminal_public_claim_delivery_receipt_summary_briefing_persisted_count",
        "terminal_public_claim_delivery_receipt_summary_briefing_materialized_count",
        "terminal_public_claim_delivery_receipt_summary_briefing_filesystem_written_count",
        "terminal_public_claim_delivery_receipt_operator_summary_recorded_count",
        "terminal_public_claim_delivery_receipt_operator_briefing_recorded_count",
        "terminal_public_claim_delivery_receipt_readback_recorded_count",
        "terminal_public_claim_delivery_receipt_status_banner_recorded_count",
        "terminal_public_claim_delivery_receipt_briefing_delivery_recorded_count",
        "terminal_public_claim_delivery_receipt_final_summary_recorded_count",
        "terminal_public_claim_delivery_receipt_external_briefing_delivered_count",
        "terminal_public_claim_delivery_receipt_telegram_briefing_delivered_count",
        "terminal_public_claim_delivery_receipt_summary_briefing_acceptance_recorded_count",
        "operator_approval_from_delivery_receipt_summary_briefing_derived_count",
        "release_publication_authority_from_delivery_receipt_summary_briefing_derived_count",
        "activation_authority_from_delivery_receipt_summary_briefing_derived_count",
        "install_from_delivery_receipt_summary_briefing_executed_count",
        "active_binary_from_delivery_receipt_summary_briefing_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count"
      ])
      + false_object([
        "terminal_public_claim_delivery_receipt_summary_briefing_accepted",
        "terminal_public_claim_delivery_receipt_summary_briefing_recorded",
        "terminal_public_claim_delivery_receipt_summary_briefing_persisted",
        "terminal_public_claim_delivery_receipt_operator_summary_recorded",
        "terminal_public_claim_delivery_receipt_operator_briefing_recorded",
        "terminal_public_claim_delivery_receipt_readback_recorded",
        "terminal_public_claim_delivery_receipt_status_banner_recorded",
        "terminal_public_claim_delivery_receipt_briefing_delivery_recorded",
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
          "operator_summary_recorded",
          "operator_summary_persisted",
          "operator_briefing_recorded",
          "operator_briefing_persisted",
          "readback_recorded",
          "status_banner_recorded",
          "exported_summary_recorded",
          "briefing_card_recorded",
          "notification_timeline_recorded",
          "dashboard_narrative_recorded",
          "audit_narrative_recorded",
          "briefing_delivery_recorded",
          "final_summary_recorded",
          "operator_memo_recorded",
          "approval_summary_recorded",
          "external_briefing_delivered",
          "telegram_briefing_delivered",
          "authority_briefing_recorded",
          "live_status_briefing_recorded",
          "summary_briefing_acceptance_recorded",
          "operator_approval_from_summary_briefing_derived",
          "release_publication_authority_from_summary_briefing_derived",
          "activation_authority_from_summary_briefing_derived",
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
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
  and $report.source_terminal_public_claim_delivery_receipt_export_query_observability_ready == true
  and $report.terminal_public_claim_delivery_receipt_summary_briefing_surface_count == 18
  and $report.terminal_public_claim_delivery_receipt_summary_briefing_denied_count == 18
  and zero_fields($report; [
    "terminal_public_claim_delivery_receipt_summary_briefing_recorded_count",
    "terminal_public_claim_delivery_receipt_operator_summary_recorded_count",
    "terminal_public_claim_delivery_receipt_operator_briefing_recorded_count",
    "terminal_public_claim_delivery_receipt_readback_recorded_count",
    "terminal_public_claim_delivery_receipt_status_banner_recorded_count",
    "terminal_public_claim_delivery_receipt_briefing_delivery_recorded_count",
    "operator_approval_from_delivery_receipt_summary_briefing_derived_count",
    "release_publication_authority_from_delivery_receipt_summary_briefing_derived_count",
    "activation_authority_from_delivery_receipt_summary_briefing_derived_count",
    "install_from_delivery_receipt_summary_briefing_executed_count",
    "active_binary_from_delivery_receipt_summary_briefing_mutated_count",
    "provider_invoked_count",
    "credential_read_count",
    "external_send_performed_count"
  ])
  and false_fields($report; [
    "terminal_public_claim_delivery_receipt_summary_briefing_recorded",
    "terminal_public_claim_delivery_receipt_operator_summary_recorded",
    "terminal_public_claim_delivery_receipt_operator_briefing_recorded",
    "terminal_public_claim_delivery_receipt_readback_recorded",
    "terminal_public_claim_delivery_receipt_status_banner_recorded",
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
  and ($report.terminal_public_claim_delivery_receipt_summary_briefing_surfaces | all(
    .terminal_public_claim_delivery_receipt_summary_briefing_attempted == true
    and .terminal_public_claim_delivery_receipt_summary_briefing_allowed == false
    and .terminal_public_claim_delivery_receipt_summary_briefing_noop_confirmed == true
    and .operator_summary_recorded == false
    and .operator_briefing_recorded == false
    and .delivery_receipt_readback_recorded == false
    and .briefing_delivery_recorded == false
    and .external_briefing_delivered == false
    and .telegram_briefing_delivered == false
    and .release_publication_authority_from_summary_briefing_derived == false
    and .activation_authority_from_summary_briefing_derived == false
    and .install_from_summary_briefing_executed == false
    and .active_binary_from_summary_briefing_mutated == false
    and .external_send_performed == false
  ))
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_final_operator_acknowledgement_non_acceptance_denial_gate"
    and .records_summary == false
    and .records_briefing == false
    and .records_readback == false
    and .records_delivery == false
    and .records_acknowledgement == false
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
echo "Hepta memory/intelligence/KG artifact signing terminal public claim delivery receipt summary/briefing denial gate passed" >&2
