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
    capture_json_report \
    "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-terminal-decision-status-promotion-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-terminal-decision-status-promotion-denial-gate.sh
)"

source_terminal_decision_status_report_sha256="$(sha256_text "$TERMINAL_DECISION_STATUS_JSON")"
terminal_public_claim_status_exposure_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-terminal-public-claim-status-exposure-denial:$source_terminal_decision_status_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
terminal_public_claim_status_exposure_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-terminal-public-claim-status-exposure:no-public-claim:no-status-exposure:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$TERMINAL_DECISION_STATUS_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denied_count == 18
    and zero_fields($source; [
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_status_promotion_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_channel_decision_delivered_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_external_decision_sent_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_telegram_decision_sent_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_release_publication_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_activation_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_install_executed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_service_restart_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_active_binary_mutated_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_external_send_count"
    ])
    and false_fields($source; [
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_status_promotion_recorded",
      "operator_acceptance_recorded",
      "operator_approval_recorded",
      "release_publication_authority_derived",
      "activation_authority_derived",
      "activation_performed",
      "memory_store_write_performed",
      "live_kg_write_performed",
      "provider_invoked",
      "credential_read",
      "install_executed",
      "service_restarted",
      "active_binary_mutated",
      "external_send_performed"
    ])
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .records_public_claim == false
      and .exposes_status == false
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
    def public_claim_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_surface:$id,
        source_terminal_decision_status_promotion_ready:true,
        public_claim_status_exposure_attempted:true,
        public_claim_status_exposure_allowed:false,
        public_claim_status_exposure_request_accepted:false,
        public_claim_status_exposure_accepted:false,
        public_claim_status_exposure_recorded:false,
        public_claim_status_exposure_persisted:false,
        public_claim_status_exposure_materialized:false,
        public_claim_status_exposure_filesystem_written:false,
        public_claim_status_exposure_delivered:false,
        public_claim_status_exposed:false,
        public_status_claimed:false,
        public_release_claimed:false,
        public_ga_claimed:false,
        release_status_exposed:false,
        publication_status_exposed:false,
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
        channel_status_delivered:false,
        external_status_sent:false,
        telegram_status_sent:false,
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
        memory_store_write_performed:false,
        memory_store_mutated:false,
        live_kg_write_performed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_read:false,
        secret_file_read:false,
        public_claim_status_exposure_noop_confirmed:true,
        public_claim_status_exposure_status:$status,
        reason:$reason
      } + $extra;
    [
      public_claim_surface("revocation_replay_public_claim_status_claim"; "blocked_revocation_replay_public_claim_noop"; "revocation_replay_public_claim_status_denied"; {public_claim_requested:true}),
      public_claim_surface("logout_replay_release_claim_status_claim"; "blocked_logout_replay_release_claim_noop"; "logout_replay_release_claim_status_denied"; {release_claim_requested:true}),
      public_claim_surface("identity_reinstatement_publication_claim_status_claim"; "blocked_identity_reinstatement_publication_claim_noop"; "identity_reinstatement_publication_claim_status_denied"; {publication_claim_requested:true}),
      public_claim_surface("session_reinstatement_ga_stable_claim_status_claim"; "blocked_session_reinstatement_ga_stable_claim_noop"; "session_reinstatement_ga_stable_claim_status_denied"; {ga_stable_claim_requested:true}),
      public_claim_surface("terminal_decision_dashboard_public_badge_status_claim"; "blocked_dashboard_public_badge_noop"; "terminal_decision_dashboard_public_badge_denied"; {dashboard_public_badge_requested:true}),
      public_claim_surface("terminal_status_endpoint_claim"; "blocked_status_endpoint_noop"; "terminal_status_endpoint_exposure_denied"; {status_endpoint_requested:true}),
      public_claim_surface("terminal_query_status_claim"; "blocked_query_status_noop"; "terminal_query_status_exposure_denied"; {query_status_requested:true}),
      public_claim_surface("terminal_export_status_claim"; "blocked_export_status_noop"; "terminal_export_status_exposure_denied"; {export_status_requested:true}),
      public_claim_surface("terminal_observability_status_claim"; "blocked_observability_status_noop"; "terminal_observability_status_exposure_denied"; {observability_status_requested:true}),
      public_claim_surface("terminal_release_notes_status_claim"; "blocked_release_notes_status_noop"; "terminal_release_notes_status_exposure_denied"; {release_notes_status_requested:true}),
      public_claim_surface("terminal_changelog_status_claim"; "blocked_changelog_status_noop"; "terminal_changelog_status_exposure_denied"; {changelog_status_requested:true}),
      public_claim_surface("terminal_version_tag_status_claim"; "blocked_version_tag_status_noop"; "terminal_version_tag_status_exposure_denied"; {version_tag_status_requested:true}),
      public_claim_surface("terminal_artifact_availability_status_claim"; "blocked_artifact_availability_status_noop"; "terminal_artifact_availability_status_exposure_denied"; {artifact_availability_status_requested:true}),
      public_claim_surface("terminal_distribution_queue_status_claim"; "blocked_distribution_queue_status_noop"; "terminal_distribution_queue_status_exposure_denied"; {distribution_queue_status_requested:true}),
      public_claim_surface("terminal_channel_external_telegram_public_status_claim"; "blocked_channel_external_telegram_public_status_noop"; "terminal_channel_external_telegram_public_status_denied"; {channel_status_requested:true, external_status_requested:true, telegram_status_requested:true}),
      public_claim_surface("terminal_release_publication_authority_public_status_claim"; "blocked_release_publication_authority_public_status_noop"; "terminal_release_publication_authority_from_public_status_denied"; {release_publication_authority_public_status_requested:true}),
      public_claim_surface("terminal_activation_live_public_status_claim"; "blocked_activation_live_public_status_noop"; "terminal_activation_live_from_public_status_denied"; {activation_live_public_status_requested:true}),
      public_claim_surface("terminal_install_restart_active_binary_public_status_claim"; "blocked_active_binary_public_status_noop"; "terminal_install_restart_active_binary_from_public_status_denied"; {install_restart_active_binary_public_status_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_denial_gate" \
    --arg source_terminal_decision_status_report_sha256 "$source_terminal_decision_status_report_sha256" \
    --arg terminal_public_claim_status_exposure_contract_hash_sha256 "$terminal_public_claim_status_exposure_contract_hash_sha256" \
    --arg terminal_public_claim_status_exposure_policy_hash_sha256 "$terminal_public_claim_status_exposure_policy_hash_sha256" \
    --argjson source "$TERMINAL_DECISION_STATUS_JSON" \
    --argjson surfaces "$surfaces_json" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_denial_v1",
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_mode:"denied_terminal_decision_status_cannot_be_exposed_as_public_release_publication_or_activation_status",
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_gate:$source.gate,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denial_ready,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_report_sha256:$source_terminal_decision_status_report_sha256,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_contract_hash_sha256:$terminal_public_claim_status_exposure_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_policy_hash_sha256:$terminal_public_claim_status_exposure_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_denial_ready:true,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_surface_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_attempt_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denied_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denied_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_status_promotion_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_status_promotion_recorded_count,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_surface_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_attempt_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_surfaces:$surfaces,
        denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure:[
          "source_terminal_decision_status_promotion_report_required",
          "public_claim_status_request_acceptance_denied",
          "public_claim_status_acceptance_denied",
          "public_claim_status_recording_denied",
          "public_claim_status_persistence_denied",
          "public_claim_status_materialization_denied",
          "public_claim_status_filesystem_write_denied",
          "public_claim_status_delivery_denied",
          "public_claim_status_exposure_denied",
          "public_status_claim_denied",
          "public_release_claim_denied",
          "public_ga_claim_denied",
          "release_status_exposure_denied",
          "publication_status_exposure_denied",
          "dashboard_status_exposure_denied",
          "public_badge_exposure_denied",
          "status_endpoint_exposure_denied",
          "query_status_exposure_denied",
          "export_status_exposure_denied",
          "observability_status_exposure_denied",
          "release_notes_status_exposure_denied",
          "changelog_status_exposure_denied",
          "version_tag_status_exposure_denied",
          "artifact_availability_status_exposure_denied",
          "distribution_queue_status_exposure_denied",
          "channel_status_delivery_denied",
          "external_status_send_denied",
          "telegram_status_send_denied",
          "acceptance_from_public_status_denied",
          "operator_approval_from_public_status_denied",
          "release_publication_authority_from_public_status_denied",
          "activation_live_from_public_status_denied",
          "install_restart_active_binary_from_public_status_denied",
          "memory_provider_kg_from_public_status_denied"
        ],
        denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_count:34,
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denial_gate",
            status:"allowed_report_only_next_slice",
            exposes_public_status:false,
            claims_public_release:false,
            claims_public_ga:false,
            records_operator_acceptance:false,
            derives_release_publication_authority:false,
            derives_activation_authority:false,
            activates_live:false,
            installs_or_restarts:false,
            mutates_active_binary:false,
            mutates_memory_store:false,
            writes_kg:false,
            sends_externally:false
          }
        ]
      }
      + zero_object([
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_request_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposed_count",
        "release_publication_result_receipt_public_status_claimed_count",
        "release_publication_result_receipt_public_release_claimed_count",
        "release_publication_result_receipt_public_ga_claimed_count",
        "release_publication_result_receipt_release_status_exposed_count",
        "release_publication_result_receipt_publication_status_exposed_count",
        "release_publication_result_receipt_dashboard_status_exposed_count",
        "release_publication_result_receipt_public_badge_exposed_count",
        "release_publication_result_receipt_status_endpoint_exposed_count",
        "release_publication_result_receipt_query_status_exposed_count",
        "release_publication_result_receipt_export_status_exposed_count",
        "release_publication_result_receipt_observability_status_exposed_count",
        "release_publication_result_receipt_release_notes_status_exposed_count",
        "release_publication_result_receipt_changelog_status_exposed_count",
        "release_publication_result_receipt_version_tag_status_exposed_count",
        "release_publication_result_receipt_artifact_availability_status_exposed_count",
        "release_publication_result_receipt_distribution_queue_status_exposed_count",
        "release_publication_result_receipt_channel_status_delivered_count",
        "release_publication_result_receipt_external_status_sent_count",
        "release_publication_result_receipt_telegram_status_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_operator_approval_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_activation_command_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_live_execution_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_service_restarted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_active_binary_mutated_count"
      ])
      + false_object([
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_status_endpoint_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_query_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_export_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_release_notes_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_changelog_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_version_tag_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_artifact_availability_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_distribution_queue_status_exposed",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_command_derived",
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
          "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_accepted",
          "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposed",
          "packet_acceptance_receipt_release_publication_result_receipt_status_endpoint_exposed",
          "packet_acceptance_receipt_release_publication_result_receipt_query_status_exposed",
          "packet_acceptance_receipt_release_publication_result_receipt_export_status_exposed",
          "packet_acceptance_receipt_release_publication_result_receipt_observability_status_exposed",
          "operator_acceptance_recorded",
          "operator_approval_recorded",
          "release_publication_authority_derived",
          "activation_authority_derived",
          "activation_command_derived",
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
          "external_send_performed",
          "filesystem_written"
        ])
      }
    '
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_ready == true
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_surface_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_surface_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_attempt_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposed_count == 0
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_surfaces | length) == 18
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_surfaces | all(
    .public_claim_status_exposure_attempted == true
    and .public_claim_status_exposure_allowed == false
    and .public_claim_status_exposure_recorded == false
    and .public_claim_status_exposed == false
    and .public_status_claimed == false
    and .public_release_claimed == false
    and .public_ga_claimed == false
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
    and .live_execution_allowed == false
    and .public_claim_status_exposure_noop_confirmed == true
  ))
  and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure | length) == 34
  and .denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_count == 34
  and .operator_approval_recorded == false
  and .release_publication_authority_derived == false
  and .activation_authority_derived == false
  and .activation_performed == false
  and .memory_store_write_performed == false
  and .live_kg_write_performed == false
  and .provider_invoked == false
  and .credential_read == false
  and .install_executed == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and .external_send_performed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

echo "$report"
echo "Hepta memory/intelligence/KG full live activation artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement terminal public claim/status exposure denial gate passed"
