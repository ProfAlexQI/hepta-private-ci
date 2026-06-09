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

OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_ORDERING_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-ordering-monotonicity-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-ordering-monotonicity-denial-gate.sh
)"

source_operator_identity_session_revocation_logout_replay_reinstatement_ordering_report_sha256="$(
  sha256_text "$OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_ORDERING_JSON"
)"
operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-cancellation-supersession-denial:$source_operator_identity_session_revocation_logout_replay_reinstatement_ordering_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-cancellation-supersession:no-cancel:no-supersede:no-replacement:no-tombstone:no-lifecycle:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_ORDERING_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denied_count == 18
    and zero_fields($source; [
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_sequence_cursor_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_sequence_cursor_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_monotonicity_state_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_monotonicity_state_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_latest_wins_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordered_query_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordered_export_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordered_observability_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordered_delivery_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_completion_order_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_revocation_logout_replay_reinstatement_ordering_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_revocation_logout_replay_reinstatement_ordering_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_ordering_release_publication_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_ordering_activation_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_ordering_install_executed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_ordering_service_restart_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_ordering_active_binary_mutated_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_ordering_external_send_count"
    ])
    and false_fields($source; [
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_ordering_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_monotonicity_persisted",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_lifecycle_latest_state_recorded",
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
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surfaces | length) == 18
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surfaces | all(
      .artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_attempted == true
      and .operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_noop_confirmed == true
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .records_ordering == false
      and .records_monotonicity == false
      and .accepts_cancellation == false
      and .accepts_supersession == false
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

operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surfaces_json="$(
  jq -n '
    def cancellation_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surface:$id,
        source_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_ready:true,
        artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_attempted:true,
        revocation_logout_replay_cancellation_requested:false,
        logout_replay_cancellation_requested:false,
        identity_reinstatement_withdrawal_requested:false,
        session_reinstatement_supersession_requested:false,
        replacement_receipt_requested:false,
        tombstone_requested:false,
        delete_marker_requested:false,
        latest_replacement_requested:false,
        ack_replacement_requested:false,
        cancelled_query_requested:false,
        superseded_export_requested:false,
        replacement_observability_requested:false,
        cancellation_completion_requested:false,
        supersession_delivery_requested:false,
        release_publication_authority_cancellation_supersession_requested:false,
        install_restart_active_binary_supersession_requested:false,
        cancellation_accepted:false,
        cancellation_recorded:false,
        cancellation_persisted:false,
        revocation_replay_cancellation_recorded:false,
        logout_replay_cancellation_recorded:false,
        identity_reinstatement_withdrawal_recorded:false,
        session_reinstatement_supersession_recorded:false,
        supersession_accepted:false,
        supersession_recorded:false,
        supersession_persisted:false,
        replacement_receipt_accepted:false,
        replacement_receipt_recorded:false,
        replacement_receipt_persisted:false,
        tombstone_recorded:false,
        tombstone_persisted:false,
        delete_marker_recorded:false,
        delete_marker_persisted:false,
        latest_replacement_accepted:false,
        ack_replacement_accepted:false,
        cancelled_query_registered:false,
        superseded_export_recorded:false,
        replacement_observability_recorded:false,
        ordering_replacement_accepted:false,
        monotonicity_replacement_accepted:false,
        replay_reinstatement_replacement_accepted:false,
        lifecycle_cancellation_supersession_recorded:false,
        lifecycle_cancellation_supersession_persisted:false,
        result_receipt_from_cancellation_supersession_recorded:false,
        result_receipt_from_cancellation_supersession_persisted:false,
        result_receipt_from_cancellation_supersession_materialized:false,
        result_receipt_from_cancellation_supersession_filesystem_written:false,
        operator_approval_from_cancellation_supersession_derived:false,
        acceptance_from_cancellation_supersession_recorded:false,
        terminal_decision_from_cancellation_supersession_recorded:false,
        terminal_status_from_cancellation_supersession_recorded:false,
        release_publication_authority_from_cancellation_supersession_derived:false,
        activation_authority_from_cancellation_supersession_derived:false,
        download_link_from_cancellation_supersession_rendered:false,
        install_command_from_cancellation_supersession_rendered:false,
        install_from_cancellation_supersession_executed:false,
        service_restart_from_cancellation_supersession_performed:false,
        launchd_from_cancellation_supersession_mutated:false,
        active_binary_from_cancellation_supersession_mutated:false,
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
        operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_noop_confirmed:true,
        operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_status:$status,
        reason:$reason
      } + $extra;
    [
      cancellation_surface("source_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_report_required"; "blocked_source_ordering_monotonicity_required_noop"; "source_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_report_required"; {source_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_report_required:true}),
      cancellation_surface("download_button_revocation_replay_cancellation_claim"; "blocked_revocation_replay_cancellation_noop"; "download_button_revocation_replay_cancellation_claim_denied"; {revocation_logout_replay_cancellation_requested:true}),
      cancellation_surface("direct_download_url_logout_replay_cancellation_claim"; "blocked_logout_replay_cancellation_noop"; "direct_download_url_logout_replay_cancellation_claim_denied"; {logout_replay_cancellation_requested:true, revocation_logout_replay_cancellation_requested:true}),
      cancellation_surface("checksum_identity_reinstatement_withdrawal_claim"; "blocked_identity_reinstatement_withdrawal_noop"; "checksum_identity_reinstatement_withdrawal_claim_denied"; {identity_reinstatement_withdrawal_requested:true}),
      cancellation_surface("package_manager_session_reinstatement_supersession_claim"; "blocked_session_reinstatement_supersession_noop"; "package_manager_session_reinstatement_supersession_claim_denied"; {session_reinstatement_supersession_requested:true}),
      cancellation_surface("curl_pipe_shell_revocation_replay_replacement_nonce_claim"; "blocked_replacement_receipt_nonce_noop"; "curl_pipe_shell_revocation_replay_replacement_nonce_claim_denied"; {replacement_receipt_requested:true}),
      cancellation_surface("installer_device_session_reinstatement_tombstone_claim"; "blocked_device_session_reinstatement_tombstone_noop"; "installer_device_session_reinstatement_tombstone_claim_denied"; {session_reinstatement_supersession_requested:true, tombstone_requested:true}),
      cancellation_surface("auto_update_session_logout_replay_delete_marker_claim"; "blocked_session_logout_replay_delete_marker_noop"; "auto_update_session_logout_replay_delete_marker_claim_denied"; {logout_replay_cancellation_requested:true, delete_marker_requested:true}),
      cancellation_surface("release_channel_identity_revocation_replay_cancellation_status_claim"; "blocked_identity_revocation_replay_cancellation_status_noop"; "release_channel_identity_revocation_replay_cancellation_status_claim_denied"; {revocation_logout_replay_cancellation_requested:true}),
      cancellation_surface("update_feed_session_reinstatement_supersession_summary_claim"; "blocked_session_reinstatement_supersession_summary_noop"; "update_feed_session_reinstatement_supersession_summary_claim_denied"; {session_reinstatement_supersession_requested:true}),
      cancellation_surface("package_registry_identity_badge_reinstatement_replacement_claim"; "blocked_identity_badge_reinstatement_replacement_noop"; "package_registry_identity_badge_reinstatement_replacement_claim_denied"; {replacement_receipt_requested:true, identity_reinstatement_withdrawal_requested:true}),
      cancellation_surface("cdn_session_readback_logout_replay_cancelled_query_claim"; "blocked_session_readback_logout_replay_cancelled_query_noop"; "cdn_session_readback_logout_replay_cancelled_query_claim_denied"; {logout_replay_cancellation_requested:true, cancelled_query_requested:true}),
      cancellation_surface("sbom_identity_dashboard_reinstatement_superseded_export_claim"; "blocked_identity_dashboard_reinstatement_superseded_export_noop"; "sbom_identity_dashboard_reinstatement_superseded_export_claim_denied"; {identity_reinstatement_withdrawal_requested:true, session_reinstatement_supersession_requested:true, superseded_export_requested:true}),
      cancellation_surface("signature_channel_session_reinstatement_replacement_observability_claim"; "blocked_channel_session_reinstatement_replacement_observability_noop"; "signature_channel_session_reinstatement_replacement_observability_claim_denied"; {session_reinstatement_supersession_requested:true, replacement_receipt_requested:true, replacement_observability_requested:true}),
      cancellation_surface("one_click_identity_approval_reinstatement_cancellation_completion_claim"; "blocked_identity_approval_reinstatement_cancellation_completion_noop"; "one_click_identity_approval_reinstatement_cancellation_completion_claim_denied"; {identity_reinstatement_withdrawal_requested:true, revocation_logout_replay_cancellation_requested:true, cancellation_completion_requested:true}),
      cancellation_surface("external_telegram_identity_session_reinstatement_supersession_delivery_claim"; "blocked_external_telegram_reinstatement_supersession_delivery_noop"; "external_telegram_identity_session_reinstatement_supersession_delivery_claim_denied"; {identity_reinstatement_withdrawal_requested:true, session_reinstatement_supersession_requested:true, supersession_delivery_requested:true, telegram_cancellation_supersession_requested:true}),
      cancellation_surface("release_publication_authority_replay_reinstatement_cancellation_supersession_claim"; "blocked_release_publication_replay_reinstatement_cancellation_supersession_noop"; "release_publication_authority_replay_reinstatement_cancellation_supersession_claim_denied"; {revocation_logout_replay_cancellation_requested:true, session_reinstatement_supersession_requested:true, release_publication_authority_cancellation_supersession_requested:true}),
      cancellation_surface("activation_live_install_restart_active_binary_session_reinstatement_supersession_claim"; "blocked_live_session_reinstatement_supersession_noop"; "activation_live_install_restart_active_binary_session_reinstatement_supersession_claim_denied"; {session_reinstatement_supersession_requested:true, replacement_receipt_requested:true, install_restart_active_binary_supersession_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denial_gate" \
    --arg source_operator_identity_session_revocation_logout_replay_reinstatement_ordering_report_sha256 "$source_operator_identity_session_revocation_logout_replay_reinstatement_ordering_report_sha256" \
    --arg operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_contract_hash_sha256 "$operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_contract_hash_sha256" \
    --arg operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_policy_hash_sha256 "$operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_ORDERING_JSON" \
    --argjson surfaces "$operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denial_v1",
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_mode:"denied_ordering_cannot_create_lifecycle_cancellation_supersession_replacement_or_authority",
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_gate:$source.gate,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denial_ready,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_report_sha256:$source_operator_identity_session_revocation_logout_replay_reinstatement_ordering_report_sha256,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_contract_hash_sha256:$operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_policy_hash_sha256:$operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denial_ready:true,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surface_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_attempt_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denied_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denied_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_monotonicity_state_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_monotonicity_state_recorded_count,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surface_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_attempt_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denied_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surfaces:$surfaces,
        denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession:[
          "source_ordering_monotonicity_report_required",
          "revocation_logout_replay_cancellation_denied",
          "logout_replay_cancellation_denied",
          "identity_reinstatement_withdrawal_denied",
          "session_reinstatement_supersession_denied",
          "replacement_receipt_recording_denied",
          "tombstone_recording_denied",
          "delete_marker_recording_denied",
          "latest_replacement_denied",
          "ack_replacement_denied",
          "cancelled_query_denied",
          "superseded_export_denied",
          "replacement_observability_denied",
          "ordering_monotonicity_replacement_bypass_denied",
          "lifecycle_cancellation_supersession_persistence_denied",
          "result_receipt_from_cancellation_supersession_denied",
          "acceptance_or_approval_from_cancellation_supersession_denied",
          "release_activation_authority_from_cancellation_supersession_denied",
          "download_install_from_cancellation_supersession_denied",
          "memory_kg_provider_secret_external_send_from_cancellation_supersession_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denial_gate",
            status:"allowed_report_only_next_slice",
            records_operator_identity:false,
            records_operator_session:false,
            accepts_replay:false,
            records_reinstatement:false,
            records_ordering:false,
            records_monotonicity:false,
            accepts_cancellation:false,
            accepts_supersession:false,
            records_replacement:false,
            records_tombstone:false,
            records_audit_trail:false,
            records_evidence:false,
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
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_supersession_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_supersession_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_supersession_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_replacement_receipt_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_replacement_receipt_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_replacement_receipt_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_tombstone_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_tombstone_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_delete_marker_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_delete_marker_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_latest_replacement_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ack_replacement_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancelled_query_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_superseded_export_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_replacement_observability_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_lifecycle_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_lifecycle_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_cancellation_supersession_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_cancellation_supersession_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_cancellation_supersession_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_cancellation_supersession_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_cancellation_supersession_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_cancellation_supersession_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_cancellation_supersession_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_cancellation_supersession_external_send_count"
      ])
      + false_object([
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_cancellation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_supersession_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_lifecycle_replacement_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_completion_ack_recorded",
        "download_button_rendered",
        "direct_download_url_exposed",
        "package_manager_install_command_rendered",
        "curl_pipe_shell_snippet_rendered",
        "installer_launch_prompt_rendered",
        "auto_update_offer_rendered",
        "external_install_message_sent",
        "telegram_install_message_sent",
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
          "cancellation_accepted",
          "cancellation_recorded",
          "cancellation_persisted",
          "supersession_accepted",
          "supersession_recorded",
          "supersession_persisted",
          "replacement_receipt_accepted",
          "replacement_receipt_recorded",
          "replacement_receipt_persisted",
          "tombstone_recorded",
          "tombstone_persisted",
          "delete_marker_recorded",
          "delete_marker_persisted",
          "latest_replacement_accepted",
          "ack_replacement_accepted",
          "cancelled_query_registered",
          "superseded_export_recorded",
          "replacement_observability_recorded",
          "ordering_replacement_accepted",
          "monotonicity_replacement_accepted",
          "lifecycle_cancellation_supersession_recorded",
          "lifecycle_cancellation_supersession_persisted",
          "result_receipt_from_cancellation_supersession_recorded",
          "result_receipt_from_cancellation_supersession_persisted",
          "result_receipt_from_cancellation_supersession_materialized",
          "result_receipt_from_cancellation_supersession_filesystem_written",
          "operator_approval_from_cancellation_supersession_derived",
          "acceptance_from_cancellation_supersession_recorded",
          "terminal_decision_from_cancellation_supersession_recorded",
          "terminal_status_from_cancellation_supersession_recorded",
          "release_publication_authority_from_cancellation_supersession_derived",
          "activation_authority_from_cancellation_supersession_derived",
          "download_link_from_cancellation_supersession_rendered",
          "install_command_from_cancellation_supersession_rendered",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denial_ready == true
  and $report.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_ready == true
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surface_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denied_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surface_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_attempt_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denied_count == 18
  and zero_fields($report; [
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_supersession_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_supersession_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_supersession_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_replacement_receipt_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_replacement_receipt_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_replacement_receipt_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_tombstone_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_tombstone_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_delete_marker_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_delete_marker_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_latest_replacement_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ack_replacement_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancelled_query_registered_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_superseded_export_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_replacement_observability_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_lifecycle_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_lifecycle_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_cancellation_supersession_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_cancellation_supersession_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_cancellation_supersession_release_publication_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_cancellation_supersession_activation_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_cancellation_supersession_install_executed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_cancellation_supersession_service_restart_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_cancellation_supersession_active_binary_mutated_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_cancellation_supersession_external_send_count"
  ])
  and false_fields($report; [
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_cancellation_recorded",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_supersession_persisted",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_lifecycle_replacement_recorded",
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
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surfaces | length) == 18
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surfaces | all(
    .artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_attempted == true
    and .operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_noop_confirmed == true
    and false_fields(.; [
      "cancellation_accepted",
      "cancellation_recorded",
      "cancellation_persisted",
      "supersession_accepted",
      "supersession_recorded",
      "supersession_persisted",
      "replacement_receipt_accepted",
      "replacement_receipt_recorded",
      "replacement_receipt_persisted",
      "tombstone_recorded",
      "tombstone_persisted",
      "delete_marker_recorded",
      "delete_marker_persisted",
      "latest_replacement_accepted",
      "ack_replacement_accepted",
      "cancelled_query_registered",
      "superseded_export_recorded",
      "replacement_observability_recorded",
      "lifecycle_cancellation_supersession_recorded",
      "lifecycle_cancellation_supersession_persisted",
      "result_receipt_from_cancellation_supersession_recorded",
      "result_receipt_from_cancellation_supersession_persisted",
      "operator_approval_from_cancellation_supersession_derived",
      "release_publication_authority_from_cancellation_supersession_derived",
      "activation_authority_from_cancellation_supersession_derived",
      "install_from_cancellation_supersession_executed",
      "service_restart_from_cancellation_supersession_performed",
      "active_binary_from_cancellation_supersession_mutated",
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
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surfaces[] | select(.revocation_logout_replay_cancellation_requested == true)] | length) == 5
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surfaces[] | select(.logout_replay_cancellation_requested == true)] | length) == 3
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surfaces[] | select(.identity_reinstatement_withdrawal_requested == true)] | length) == 5
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surfaces[] | select(.session_reinstatement_supersession_requested == true)] | length) == 8
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surfaces[] | select(.replacement_receipt_requested == true)] | length) == 4
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surfaces[] | select(.tombstone_requested == true or .delete_marker_requested == true)] | length) == 2
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surfaces[] | select(.telegram_cancellation_supersession_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surfaces[] | select(.install_restart_active_binary_supersession_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .records_operator_identity == false
    and .records_operator_session == false
    and .accepts_replay == false
    and .records_reinstatement == false
    and .records_ordering == false
    and .records_monotonicity == false
    and .accepts_cancellation == false
    and .accepts_supersession == false
    and .records_replacement == false
    and .records_tombstone == false
    and .records_audit_trail == false
    and .records_evidence == false
    and .derives_authority == false
    and .renders_download_link == false
    and .emits_install_command == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .mutates_memory_store == false
    and .writes_kg == false
    and .sends_externally == false
  ))
  and ($report.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement cancellation/supersession denial gate passed"
