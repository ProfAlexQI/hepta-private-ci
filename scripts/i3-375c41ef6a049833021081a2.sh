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

TERMINAL_DECISION_STATUS_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-terminal-decision-status-promotion-denial-gate" \
    scripts/i3-ee1f0cd8da68b96f743161d3.sh
)"

source_terminal_decision_status_report_sha256="$(sha256_text "$TERMINAL_DECISION_STATUS_JSON")"
terminal_public_claim_status_exposure_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-terminal-public-claim-status-exposure-denial:$source_terminal_decision_status_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
terminal_public_claim_status_exposure_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-terminal-public-claim-status-exposure:no-public-claim:no-status-exposure:no-package-channel:no-ga:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$TERMINAL_DECISION_STATUS_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_decision_status_promotion_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_decision_status_promotion_denial_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_decision_status_promotion_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_decision_status_promotion_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_decision_status_promotion_denied_count == 18
    and zero_fields($source; [
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_decision_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_decision_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_decision_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_status_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_status_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_status_promotion_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_channel_decision_delivered_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_external_decision_sent_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_telegram_decision_sent_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_decision_release_publication_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_status_activation_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_status_install_executed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_status_active_binary_mutated_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_status_external_send_count"
    ])
    and false_fields($source; [
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
      "external_send_performed"
    ])
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_public_claim_status_exposure_denial_gate"
      and .records_public_claim == false
      and .records_status_exposure == false
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

surfaces_json="$(
  jq -n '
    def surface($id; $status; $reason; $extra):
      {
        surface:$id,
        source_terminal_decision_status_ready:true,
        terminal_public_claim_status_exposure_attempted:true,
        public_claim_status_exposure_allowed:false,
        public_claim_status_exposure_accepted:false,
        public_claim_status_exposure_recorded:false,
        public_claim_status_exposure_persisted:false,
        public_claim_status_exposure_materialized:false,
        public_claim_status_exposure_filesystem_written:false,
        public_claim_status_exposure_delivered:false,
        public_status_claimed:false,
        public_release_claimed:false,
        public_ga_claimed:false,
        release_status_exposed:false,
        publication_status_exposed:false,
        package_release_channel_status_exposed:false,
        dashboard_status_exposed:false,
        public_badge_exposed:false,
        status_endpoint_exposed:false,
        query_status_exposed:false,
        export_status_exposed:false,
        observability_status_exposed:false,
        release_notes_status_exposed:false,
        changelog_status_exposed:false,
        version_tag_status_exposed:false,
        artifact_availability_status_exposed:false,
        distribution_queue_status_exposed:false,
        live_install_status_exposed:false,
        channel_status_delivered:false,
        external_status_sent:false,
        telegram_status_sent:false,
        acceptance_from_public_status_recorded:false,
        operator_approval_from_public_status_derived:false,
        release_publication_authority_from_public_status_derived:false,
        activation_authority_from_public_status_derived:false,
        download_link_from_public_status_rendered:false,
        install_command_from_public_status_rendered:false,
        install_from_public_status_executed:false,
        service_restart_from_public_status_performed:false,
        active_binary_from_public_status_mutated:false,
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
        terminal_public_claim_status_exposure_noop_confirmed:true,
        terminal_public_claim_status_exposure_status:$status,
        reason:$reason
      } + $extra;
    [
      surface("source_operator_intent_consent_evidence_terminal_decision_status_report_required"; "blocked_source_terminal_status_required_noop"; "source_operator_intent_consent_evidence_terminal_decision_status_report_required"; {source_report_required:true}),
      surface("revocation_replay_terminal_decision_public_claim"; "blocked_terminal_decision_public_claim_noop"; "revocation_replay_terminal_decision_public_claim_denied"; {public_claim_requested:true}),
      surface("logout_replay_terminal_status_public_status_claim"; "blocked_terminal_status_public_status_noop"; "logout_replay_terminal_status_public_status_claim_denied"; {public_status_claim_requested:true}),
      surface("identity_reinstatement_status_promotion_public_release_claim"; "blocked_public_release_claim_noop"; "identity_reinstatement_status_promotion_public_release_claim_denied"; {public_release_claim_requested:true}),
      surface("session_reinstatement_decision_summary_public_ga_claim"; "blocked_public_ga_claim_noop"; "session_reinstatement_decision_summary_public_ga_claim_denied"; {public_ga_claim_requested:true}),
      surface("identity_badge_package_release_channel_status_exposure"; "blocked_package_release_channel_status_noop"; "identity_badge_package_release_channel_status_exposure_denied"; {package_release_channel_status_requested:true}),
      surface("logout_replay_terminal_readback_status_endpoint_exposure"; "blocked_status_endpoint_noop"; "logout_replay_terminal_readback_status_endpoint_exposure_denied"; {status_endpoint_requested:true}),
      surface("identity_dashboard_terminal_status_dashboard_exposure"; "blocked_dashboard_status_noop"; "identity_dashboard_terminal_status_dashboard_exposure_denied"; {dashboard_status_requested:true}),
      surface("session_channel_terminal_decision_channel_delivery"; "blocked_channel_status_delivery_noop"; "session_channel_terminal_decision_channel_delivery_denied"; {channel_status_delivery_requested:true}),
      surface("external_telegram_terminal_decision_status_send"; "blocked_external_telegram_status_noop"; "external_telegram_terminal_decision_status_send_denied"; {external_status_requested:true, telegram_status_requested:true}),
      surface("decision_summary_query_status_exposure"; "blocked_query_status_noop"; "decision_summary_query_status_exposure_denied"; {query_status_requested:true}),
      surface("status_badge_export_status_exposure"; "blocked_export_status_noop"; "status_badge_export_status_exposure_denied"; {export_status_requested:true}),
      surface("notification_observability_status_exposure"; "blocked_observability_status_noop"; "notification_observability_status_exposure_denied"; {observability_status_requested:true}),
      surface("release_notes_changelog_version_tag_status_exposure"; "blocked_release_notes_changelog_version_tag_noop"; "release_notes_changelog_version_tag_status_exposure_denied"; {release_notes_status_requested:true, changelog_status_requested:true, version_tag_status_requested:true}),
      surface("artifact_availability_distribution_queue_status_exposure"; "blocked_artifact_availability_distribution_queue_noop"; "artifact_availability_distribution_queue_status_exposure_denied"; {artifact_availability_status_requested:true, distribution_queue_status_requested:true}),
      surface("release_publication_authority_public_status_claim"; "blocked_release_publication_authority_public_status_noop"; "release_publication_authority_public_status_claim_denied"; {release_publication_authority_public_status_requested:true}),
      surface("activation_live_install_status_exposure"; "blocked_activation_live_install_status_noop"; "activation_live_install_status_exposure_denied"; {activation_live_status_requested:true, live_install_status_requested:true}),
      surface("install_restart_active_binary_public_status_claim"; "blocked_install_restart_active_binary_public_status_noop"; "install_restart_active_binary_public_status_claim_denied"; {install_restart_active_binary_status_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_public_claim_status_exposure_denial_gate" \
    --arg source_terminal_decision_status_report_sha256 "$source_terminal_decision_status_report_sha256" \
    --arg terminal_public_claim_status_exposure_contract_hash_sha256 "$terminal_public_claim_status_exposure_contract_hash_sha256" \
    --arg terminal_public_claim_status_exposure_policy_hash_sha256 "$terminal_public_claim_status_exposure_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$TERMINAL_DECISION_STATUS_JSON" \
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
        terminal_public_claim_status_exposure_schema_version:"operator_intent_consent_evidence_terminal_public_claim_status_exposure_denial_v1",
        terminal_public_claim_status_exposure_mode:"denied_terminal_status_cannot_create_public_claim_status_exposure_authority_install_or_live_status",
        source_terminal_decision_status_gate:$source.gate,
        source_terminal_decision_status_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_decision_status_promotion_denial_ready,
        source_terminal_decision_status_report_sha256:$source_terminal_decision_status_report_sha256,
        terminal_public_claim_status_exposure_contract_hash_sha256:$terminal_public_claim_status_exposure_contract_hash_sha256,
        terminal_public_claim_status_exposure_policy_hash_sha256:$terminal_public_claim_status_exposure_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_public_claim_status_exposure_denial_ready:true,
        source_terminal_decision_status_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_decision_status_promotion_surface_count,
        source_terminal_decision_status_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_decision_status_promotion_attempt_count,
        source_terminal_decision_status_denied_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_decision_status_promotion_denied_count,
        terminal_public_claim_status_exposure_surface_count:($surfaces | length),
        terminal_public_claim_status_exposure_attempt_count:($surfaces | length),
        terminal_public_claim_status_exposure_denied_count:($surfaces | length),
        terminal_public_claim_status_exposure_surfaces:$surfaces,
        denied_by_terminal_public_claim_status_exposure:[
          "source_operator_intent_consent_evidence_terminal_decision_status_report_required",
          "terminal_public_claim_acceptance_denied",
          "terminal_public_claim_recording_denied",
          "terminal_public_claim_persistence_denied",
          "terminal_public_claim_materialization_denied",
          "terminal_public_claim_filesystem_write_denied",
          "terminal_public_claim_delivery_denied",
          "public_status_claim_denied",
          "public_release_claim_denied",
          "public_ga_claim_denied",
          "release_publication_package_channel_status_exposure_denied",
          "dashboard_badge_endpoint_status_exposure_denied",
          "query_export_observability_status_exposure_denied",
          "release_notes_changelog_version_tag_status_exposure_denied",
          "artifact_availability_distribution_queue_status_exposure_denied",
          "channel_external_telegram_status_delivery_denied",
          "acceptance_and_operator_approval_from_public_status_denied",
          "release_publication_authority_from_public_status_denied",
          "activation_authority_from_public_status_denied",
          "download_install_affordance_from_public_status_denied",
          "install_restart_active_binary_from_public_status_denied",
          "memory_provider_secret_external_send_from_public_status_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_package_release_channel_status_exposure_denial_gate",
            status:"allowed_report_only_next_slice",
            records_public_claim:false,
            exposes_public_status:false,
            exposes_package_release_channel_status:false,
            claims_public_release:false,
            claims_public_ga:false,
            records_operator_acceptance:false,
            derives_release_publication_authority:false,
            derives_activation_authority:false,
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
        "terminal_public_claim_status_exposure_accepted_count",
        "terminal_public_claim_status_exposure_recorded_count",
        "terminal_public_claim_status_exposure_persisted_count",
        "terminal_public_claim_status_exposure_materialized_count",
        "terminal_public_claim_status_exposure_filesystem_written_count",
        "terminal_public_claim_status_exposure_delivered_count",
        "public_status_claimed_count",
        "public_release_claimed_count",
        "public_ga_claimed_count",
        "release_status_exposed_count",
        "publication_status_exposed_count",
        "package_release_channel_status_exposed_count",
        "dashboard_status_exposed_count",
        "status_endpoint_exposed_count",
        "query_status_exposed_count",
        "export_status_exposed_count",
        "observability_status_exposed_count",
        "artifact_availability_status_exposed_count",
        "distribution_queue_status_exposed_count",
        "live_install_status_exposed_count",
        "channel_status_delivered_count",
        "external_status_sent_count",
        "telegram_status_sent_count",
        "acceptance_from_public_status_recorded_count",
        "operator_approval_from_public_status_derived_count",
        "release_publication_authority_from_public_status_derived_count",
        "activation_authority_from_public_status_derived_count",
        "download_link_from_public_status_rendered_count",
        "install_command_from_public_status_rendered_count",
        "install_from_public_status_executed_count",
        "service_restart_from_public_status_performed_count",
        "active_binary_from_public_status_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count"
      ])
      + false_object([
        "terminal_public_claim_status_exposure_accepted",
        "terminal_public_claim_status_exposure_recorded",
        "terminal_public_claim_status_exposure_persisted",
        "public_status_claimed",
        "public_release_claimed",
        "public_ga_claimed",
        "package_release_channel_status_exposed",
        "live_install_status_exposed",
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
        "external_send_performed"
      ])
      + {
        side_effects:false_object([
          "terminal_public_claim_status_exposure_recorded",
          "terminal_public_claim_status_exposure_persisted",
          "terminal_public_claim_status_exposure_materialized",
          "terminal_public_claim_status_exposure_filesystem_written",
          "terminal_public_claim_status_exposure_delivered",
          "public_status_claimed",
          "public_release_claimed",
          "public_ga_claimed",
          "release_status_exposed",
          "publication_status_exposed",
          "package_release_channel_status_exposed",
          "dashboard_status_exposed",
          "status_endpoint_exposed",
          "query_status_exposed",
          "export_status_exposed",
          "observability_status_exposed",
          "artifact_availability_status_exposed",
          "distribution_queue_status_exposed",
          "live_install_status_exposed",
          "channel_status_delivered",
          "external_status_sent",
          "telegram_status_sent",
          "operator_acceptance_recorded",
          "operator_approval_recorded",
          "release_publication_authority_derived",
          "activation_authority_derived",
          "download_link_rendered",
          "install_command_rendered",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_public_claim_status_exposure_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_public_claim_status_exposure_denial_ready == true
  and $report.source_terminal_decision_status_ready == true
  and $report.source_terminal_decision_status_surface_count == 18
  and $report.source_terminal_decision_status_attempt_count == 18
  and $report.source_terminal_decision_status_denied_count == 18
  and $report.terminal_public_claim_status_exposure_surface_count == 18
  and $report.terminal_public_claim_status_exposure_attempt_count == 18
  and $report.terminal_public_claim_status_exposure_denied_count == 18
  and zero_fields($report; [
    "terminal_public_claim_status_exposure_accepted_count",
    "terminal_public_claim_status_exposure_recorded_count",
    "terminal_public_claim_status_exposure_persisted_count",
    "terminal_public_claim_status_exposure_materialized_count",
    "terminal_public_claim_status_exposure_filesystem_written_count",
    "terminal_public_claim_status_exposure_delivered_count",
    "public_status_claimed_count",
    "public_release_claimed_count",
    "public_ga_claimed_count",
    "package_release_channel_status_exposed_count",
    "live_install_status_exposed_count",
    "channel_status_delivered_count",
    "external_status_sent_count",
    "telegram_status_sent_count",
    "release_publication_authority_from_public_status_derived_count",
    "activation_authority_from_public_status_derived_count",
    "install_from_public_status_executed_count",
    "active_binary_from_public_status_mutated_count",
    "memory_store_write_performed_count",
    "live_kg_write_performed_count",
    "provider_invoked_count",
    "model_invoked_count",
    "credential_read_count",
    "secret_file_read_count",
    "external_send_performed_count"
  ])
  and false_fields($report; [
    "terminal_public_claim_status_exposure_accepted",
    "terminal_public_claim_status_exposure_recorded",
    "terminal_public_claim_status_exposure_persisted",
    "public_status_claimed",
    "public_release_claimed",
    "public_ga_claimed",
    "package_release_channel_status_exposed",
    "live_install_status_exposed",
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
    "external_send_performed"
  ])
  and ($report.terminal_public_claim_status_exposure_surfaces | length) == 18
  and ($report.terminal_public_claim_status_exposure_surfaces | all(
    .terminal_public_claim_status_exposure_attempted == true
    and .terminal_public_claim_status_exposure_noop_confirmed == true
    and .public_claim_status_exposure_allowed == false
    and .public_claim_status_exposure_accepted == false
    and .public_claim_status_exposure_recorded == false
    and .public_status_claimed == false
    and .public_release_claimed == false
    and .public_ga_claimed == false
    and .package_release_channel_status_exposed == false
    and .live_install_status_exposed == false
    and .channel_status_delivered == false
    and .external_status_sent == false
    and .telegram_status_sent == false
    and .release_publication_authority_from_public_status_derived == false
    and .activation_authority_from_public_status_derived == false
    and .install_from_public_status_executed == false
    and .active_binary_from_public_status_mutated == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .external_send_performed == false
  ))
  and ([.terminal_public_claim_status_exposure_surfaces[] | select(.public_claim_requested == true)] | length) == 1
  and ([.terminal_public_claim_status_exposure_surfaces[] | select(.public_release_claim_requested == true)] | length) == 1
  and ([.terminal_public_claim_status_exposure_surfaces[] | select(.telegram_status_requested == true)] | length) == 1
  and ([.terminal_public_claim_status_exposure_surfaces[] | select(.install_restart_active_binary_status_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_package_release_channel_status_exposure_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .records_public_claim == false
    and .exposes_public_status == false
    and .exposes_package_release_channel_status == false
    and .claims_public_release == false
    and .claims_public_ga == false
    and .derives_release_publication_authority == false
    and .derives_activation_authority == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .mutates_memory_store == false
    and .writes_kg == false
    and .sends_externally == false
  ))
  and ($report.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence terminal public claim/status exposure denial gate passed"
