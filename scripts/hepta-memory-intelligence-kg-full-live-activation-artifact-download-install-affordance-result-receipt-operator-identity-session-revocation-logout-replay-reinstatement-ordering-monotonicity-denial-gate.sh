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

OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-denial-gate.sh
)"

source_operator_identity_session_revocation_logout_replay_reinstatement_report_sha256="$(
  sha256_text "$OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_JSON"
)"
operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-ordering-monotonicity-denial:$source_operator_identity_session_revocation_logout_replay_reinstatement_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-ordering-monotonicity:no-ordering:no-monotonicity:no-latest-lifecycle:no-cursor:no-approval:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denied_count == 18
    and zero_fields($source; [
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_logout_replay_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_logout_replay_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_logout_replay_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_logout_replay_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_logout_replay_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_logout_replay_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_reinstatement_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_reinstatement_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_reinstatement_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_reinstatement_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_reinstatement_token_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_reinstatement_nonce_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_device_session_reinstatement_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_logout_replay_refresh_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_revocation_replay_status_promoted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_reinstatement_summary_promoted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_revocation_logout_replay_reinstatement_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_revocation_logout_replay_reinstatement_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_release_publication_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_activation_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_install_executed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_service_restart_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_active_binary_mutated_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_external_send_count"
    ])
    and false_fields($source; [
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_revocation_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_logout_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_lifecycle_recorded",
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
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surfaces | length) == 18
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surfaces | all(
      .artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_attempted == true
      and .operator_identity_session_revocation_logout_replay_reinstatement_noop_confirmed == true
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .records_ordering == false
      and .records_monotonicity == false
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

operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surfaces_json="$(
  jq -n '
    def ordering_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surface:$id,
        source_operator_identity_session_revocation_logout_replay_reinstatement_ready:true,
        artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_attempted:true,
        revocation_logout_replay_ordering_requested:false,
        logout_replay_sequence_requested:false,
        identity_reinstatement_ordering_requested:false,
        session_reinstatement_ordering_requested:false,
        sequence_claim_requested:false,
        monotonicity_claim_requested:false,
        timestamp_rollback_requested:false,
        epoch_rollback_requested:false,
        same_sequence_different_nonce_requested:false,
        late_arrival_requested:false,
        future_sequence_gap_requested:false,
        latest_wins_requested:false,
        monotonic_cursor_requested:false,
        ordered_query_requested:false,
        ordered_export_requested:false,
        ordered_observability_requested:false,
        ordered_delivery_requested:false,
        completion_order_requested:false,
        telegram_ordered_delivery_requested:false,
        release_publication_authority_ordering_requested:false,
        install_restart_active_binary_ordering_requested:false,
        revocation_logout_replay_ordering_accepted:false,
        logout_replay_sequence_accepted:false,
        identity_reinstatement_ordering_accepted:false,
        session_reinstatement_ordering_accepted:false,
        ordering_recorded:false,
        ordering_persisted:false,
        ordering_materialized:false,
        ordering_filesystem_written:false,
        sequence_cursor_recorded:false,
        sequence_cursor_persisted:false,
        monotonicity_state_recorded:false,
        monotonicity_state_persisted:false,
        monotonicity_state_materialized:false,
        timestamp_rollback_accepted:false,
        epoch_rollback_accepted:false,
        same_sequence_different_nonce_accepted:false,
        late_arrival_accepted:false,
        future_sequence_gap_accepted:false,
        latest_wins_accepted:false,
        monotonic_cursor_accepted:false,
        ordered_query_accepted:false,
        ordered_export_accepted:false,
        ordered_observability_accepted:false,
        ordered_delivery_accepted:false,
        completion_order_recorded:false,
        operator_approval_from_revocation_logout_replay_reinstatement_ordering_derived:false,
        acceptance_from_revocation_logout_replay_reinstatement_ordering_recorded:false,
        terminal_decision_from_revocation_logout_replay_reinstatement_ordering_recorded:false,
        terminal_status_from_revocation_logout_replay_reinstatement_ordering_recorded:false,
        release_publication_authority_from_revocation_logout_replay_reinstatement_ordering_derived:false,
        activation_authority_from_revocation_logout_replay_reinstatement_ordering_derived:false,
        download_link_from_revocation_logout_replay_reinstatement_ordering_rendered:false,
        install_command_from_revocation_logout_replay_reinstatement_ordering_rendered:false,
        install_from_revocation_logout_replay_reinstatement_ordering_executed:false,
        service_restart_from_revocation_logout_replay_reinstatement_ordering_performed:false,
        launchd_from_revocation_logout_replay_reinstatement_ordering_mutated:false,
        active_binary_from_revocation_logout_replay_reinstatement_ordering_mutated:false,
        result_receipt_from_revocation_logout_replay_reinstatement_ordering_recorded:false,
        result_receipt_from_revocation_logout_replay_reinstatement_ordering_persisted:false,
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
        operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_noop_confirmed:true,
        operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_status:$status,
        reason:$reason
      } + $extra;
    [
      ordering_surface("source_operator_identity_session_revocation_logout_replay_reinstatement_report_required"; "blocked_source_replay_reinstatement_required_noop"; "source_operator_identity_session_revocation_logout_replay_reinstatement_report_required"; {source_operator_identity_session_revocation_logout_replay_reinstatement_report_required:true}),
      ordering_surface("download_button_revocation_replay_sequence_claim"; "blocked_revocation_replay_sequence_noop"; "download_button_revocation_replay_sequence_claim_denied"; {revocation_logout_replay_ordering_requested:true, sequence_claim_requested:true}),
      ordering_surface("direct_download_url_logout_replay_sequence_claim"; "blocked_logout_replay_sequence_noop"; "direct_download_url_logout_replay_sequence_claim_denied"; {logout_replay_sequence_requested:true, sequence_claim_requested:true}),
      ordering_surface("checksum_identity_reinstatement_timestamp_rollback_claim"; "blocked_identity_reinstatement_timestamp_rollback_noop"; "checksum_identity_reinstatement_timestamp_rollback_claim_denied"; {identity_reinstatement_ordering_requested:true, monotonicity_claim_requested:true, timestamp_rollback_requested:true}),
      ordering_surface("package_manager_session_reinstatement_epoch_rollback_claim"; "blocked_session_reinstatement_epoch_rollback_noop"; "package_manager_session_reinstatement_epoch_rollback_claim_denied"; {session_reinstatement_ordering_requested:true, monotonicity_claim_requested:true, epoch_rollback_requested:true}),
      ordering_surface("curl_pipe_shell_revocation_replay_same_sequence_different_nonce_claim"; "blocked_same_sequence_different_nonce_noop"; "curl_pipe_shell_revocation_replay_same_sequence_different_nonce_claim_denied"; {revocation_logout_replay_ordering_requested:true, sequence_claim_requested:true, same_sequence_different_nonce_requested:true}),
      ordering_surface("installer_device_session_reinstatement_late_arrival_claim"; "blocked_device_session_reinstatement_late_arrival_noop"; "installer_device_session_reinstatement_late_arrival_claim_denied"; {session_reinstatement_ordering_requested:true, monotonicity_claim_requested:true, late_arrival_requested:true}),
      ordering_surface("auto_update_session_logout_replay_future_sequence_gap_claim"; "blocked_logout_replay_future_sequence_gap_noop"; "auto_update_session_logout_replay_future_sequence_gap_claim_denied"; {revocation_logout_replay_ordering_requested:true, logout_replay_sequence_requested:true, sequence_claim_requested:true, future_sequence_gap_requested:true}),
      ordering_surface("release_channel_identity_revocation_replay_latest_wins_claim"; "blocked_identity_revocation_replay_latest_wins_noop"; "release_channel_identity_revocation_replay_latest_wins_claim_denied"; {revocation_logout_replay_ordering_requested:true, sequence_claim_requested:true, latest_wins_requested:true}),
      ordering_surface("update_feed_session_reinstatement_monotonic_cursor_claim"; "blocked_session_reinstatement_monotonic_cursor_noop"; "update_feed_session_reinstatement_monotonic_cursor_claim_denied"; {session_reinstatement_ordering_requested:true, monotonicity_claim_requested:true, monotonic_cursor_requested:true}),
      ordering_surface("package_registry_identity_badge_reinstatement_ordered_status_claim"; "blocked_identity_badge_reinstatement_ordered_status_noop"; "package_registry_identity_badge_reinstatement_ordered_status_claim_denied"; {identity_reinstatement_ordering_requested:true, monotonicity_claim_requested:true}),
      ordering_surface("cdn_session_readback_logout_replay_ordered_query_claim"; "blocked_session_readback_logout_replay_ordered_query_noop"; "cdn_session_readback_logout_replay_ordered_query_claim_denied"; {logout_replay_sequence_requested:true, sequence_claim_requested:true, ordered_query_requested:true}),
      ordering_surface("sbom_identity_dashboard_reinstatement_ordered_export_claim"; "blocked_identity_dashboard_reinstatement_ordered_export_noop"; "sbom_identity_dashboard_reinstatement_ordered_export_claim_denied"; {identity_reinstatement_ordering_requested:true, monotonicity_claim_requested:true, ordered_export_requested:true}),
      ordering_surface("signature_channel_session_reinstatement_ordered_observability_claim"; "blocked_channel_session_reinstatement_ordered_observability_noop"; "signature_channel_session_reinstatement_ordered_observability_claim_denied"; {session_reinstatement_ordering_requested:true, monotonicity_claim_requested:true, ordered_observability_requested:true}),
      ordering_surface("one_click_identity_approval_reinstatement_completion_order_claim"; "blocked_identity_approval_reinstatement_completion_order_noop"; "one_click_identity_approval_reinstatement_completion_order_claim_denied"; {identity_reinstatement_ordering_requested:true, monotonicity_claim_requested:true, completion_order_requested:true}),
      ordering_surface("external_telegram_identity_session_reinstatement_ordered_delivery_claim"; "blocked_external_telegram_reinstatement_ordered_delivery_noop"; "external_telegram_identity_session_reinstatement_ordered_delivery_claim_denied"; {logout_replay_sequence_requested:true, identity_reinstatement_ordering_requested:true, session_reinstatement_ordering_requested:true, sequence_claim_requested:true, ordered_delivery_requested:true, telegram_ordered_delivery_requested:true}),
      ordering_surface("release_publication_authority_replay_reinstatement_ordering_claim"; "blocked_release_publication_replay_reinstatement_ordering_noop"; "release_publication_authority_replay_reinstatement_ordering_claim_denied"; {revocation_logout_replay_ordering_requested:true, session_reinstatement_ordering_requested:true, sequence_claim_requested:true, release_publication_authority_ordering_requested:true}),
      ordering_surface("activation_live_install_restart_active_binary_session_reinstatement_ordering_claim"; "blocked_live_session_reinstatement_ordering_noop"; "activation_live_install_restart_active_binary_session_reinstatement_ordering_claim_denied"; {session_reinstatement_ordering_requested:true, monotonicity_claim_requested:true, install_restart_active_binary_ordering_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denial_gate" \
    --arg source_operator_identity_session_revocation_logout_replay_reinstatement_report_sha256 "$source_operator_identity_session_revocation_logout_replay_reinstatement_report_sha256" \
    --arg operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_contract_hash_sha256 "$operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_contract_hash_sha256" \
    --arg operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_policy_hash_sha256 "$operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_JSON" \
    --argjson surfaces "$operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denial_v1",
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_mode:"denied_replay_reinstatement_cannot_create_lifecycle_ordering_monotonicity_or_latest_state_authority",
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_gate:$source.gate,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denial_ready,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_report_sha256:$source_operator_identity_session_revocation_logout_replay_reinstatement_report_sha256,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_contract_hash_sha256:$operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_policy_hash_sha256:$operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denial_ready:true,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surface_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_attempt_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denied_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denied_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_reinstatement_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_reinstatement_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_reinstatement_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_reinstatement_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_logout_replay_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_logout_replay_recorded_count,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surface_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_attempt_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denied_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surfaces:$surfaces,
        denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity:[
          "source_revocation_logout_replay_reinstatement_report_required",
          "revocation_logout_replay_ordering_denied",
          "logout_replay_sequence_acceptance_denied",
          "identity_reinstatement_ordering_denied",
          "session_reinstatement_ordering_denied",
          "sequence_cursor_recording_denied",
          "sequence_cursor_persistence_denied",
          "monotonicity_state_recording_denied",
          "monotonicity_state_persistence_denied",
          "timestamp_rollback_denied",
          "epoch_rollback_denied",
          "same_sequence_different_nonce_denied",
          "late_arrival_denied",
          "future_sequence_gap_denied",
          "latest_wins_lifecycle_state_denied",
          "monotonic_cursor_denied",
          "ordered_query_export_observability_denied",
          "ordered_delivery_and_completion_order_denied",
          "acceptance_or_approval_from_ordering_denied",
          "release_activation_authority_from_ordering_denied",
          "download_install_from_ordering_denied",
          "memory_kg_provider_secret_external_send_from_ordering_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denial_gate",
            status:"allowed_report_only_next_slice",
            records_operator_identity:false,
            records_operator_session:false,
            records_session_binding:false,
            accepts_replay:false,
            accepts_cross_session_binding:false,
            records_revocation:false,
            records_logout:false,
            accepts_revocation_replay:false,
            records_reinstatement:false,
            accepts_reinstatement_replay:false,
            records_ordering:false,
            records_monotonicity:false,
            accepts_cancellation:false,
            accepts_supersession:false,
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
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_sequence_cursor_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_sequence_cursor_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_monotonicity_state_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_monotonicity_state_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_monotonicity_state_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_timestamp_rollback_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_epoch_rollback_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_same_sequence_different_nonce_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_late_arrival_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_future_sequence_gap_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_latest_wins_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_monotonic_cursor_accepted_count",
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
      + false_object([
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_ordering_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_monotonicity_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_lifecycle_latest_state_recorded",
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
          "revocation_logout_replay_ordering_accepted",
          "logout_replay_sequence_accepted",
          "identity_reinstatement_ordering_accepted",
          "session_reinstatement_ordering_accepted",
          "ordering_recorded",
          "ordering_persisted",
          "ordering_materialized",
          "ordering_filesystem_written",
          "sequence_cursor_recorded",
          "sequence_cursor_persisted",
          "monotonicity_state_recorded",
          "monotonicity_state_persisted",
          "monotonicity_state_materialized",
          "timestamp_rollback_accepted",
          "epoch_rollback_accepted",
          "same_sequence_different_nonce_accepted",
          "late_arrival_accepted",
          "future_sequence_gap_accepted",
          "latest_wins_accepted",
          "monotonic_cursor_accepted",
          "ordered_query_accepted",
          "ordered_export_accepted",
          "ordered_observability_accepted",
          "ordered_delivery_accepted",
          "completion_order_recorded",
          "operator_approval_from_revocation_logout_replay_reinstatement_ordering_derived",
          "acceptance_from_revocation_logout_replay_reinstatement_ordering_recorded",
          "terminal_decision_from_revocation_logout_replay_reinstatement_ordering_recorded",
          "terminal_status_from_revocation_logout_replay_reinstatement_ordering_recorded",
          "release_publication_authority_from_revocation_logout_replay_reinstatement_ordering_derived",
          "activation_authority_from_revocation_logout_replay_reinstatement_ordering_derived",
          "download_link_from_revocation_logout_replay_reinstatement_ordering_rendered",
          "install_command_from_revocation_logout_replay_reinstatement_ordering_rendered",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denial_ready == true
  and $report.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ready == true
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surface_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denied_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surface_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_attempt_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denied_count == 18
  and zero_fields($report; [
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_materialized_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_filesystem_written_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_sequence_cursor_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_sequence_cursor_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_monotonicity_state_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_monotonicity_state_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_monotonicity_state_materialized_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_timestamp_rollback_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_epoch_rollback_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_same_sequence_different_nonce_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_late_arrival_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_future_sequence_gap_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_latest_wins_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_monotonic_cursor_accepted_count",
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
  and false_fields($report; [
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
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surfaces | length) == 18
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surfaces | all(
    .artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_attempted == true
    and .operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_noop_confirmed == true
    and false_fields(.; [
      "revocation_logout_replay_ordering_accepted",
      "logout_replay_sequence_accepted",
      "identity_reinstatement_ordering_accepted",
      "session_reinstatement_ordering_accepted",
      "ordering_recorded",
      "ordering_persisted",
      "ordering_materialized",
      "ordering_filesystem_written",
      "sequence_cursor_recorded",
      "sequence_cursor_persisted",
      "monotonicity_state_recorded",
      "monotonicity_state_persisted",
      "monotonicity_state_materialized",
      "timestamp_rollback_accepted",
      "epoch_rollback_accepted",
      "same_sequence_different_nonce_accepted",
      "late_arrival_accepted",
      "future_sequence_gap_accepted",
      "latest_wins_accepted",
      "monotonic_cursor_accepted",
      "ordered_query_accepted",
      "ordered_export_accepted",
      "ordered_observability_accepted",
      "ordered_delivery_accepted",
      "completion_order_recorded",
      "operator_approval_from_revocation_logout_replay_reinstatement_ordering_derived",
      "release_publication_authority_from_revocation_logout_replay_reinstatement_ordering_derived",
      "activation_authority_from_revocation_logout_replay_reinstatement_ordering_derived",
      "install_from_revocation_logout_replay_reinstatement_ordering_executed",
      "service_restart_from_revocation_logout_replay_reinstatement_ordering_performed",
      "active_binary_from_revocation_logout_replay_reinstatement_ordering_mutated",
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
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surfaces[] | select(.revocation_logout_replay_ordering_requested == true)] | length) == 5
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surfaces[] | select(.logout_replay_sequence_requested == true)] | length) == 4
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surfaces[] | select(.identity_reinstatement_ordering_requested == true)] | length) == 5
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surfaces[] | select(.session_reinstatement_ordering_requested == true)] | length) == 7
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surfaces[] | select(.telegram_ordered_delivery_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surfaces[] | select(.install_restart_active_binary_ordering_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .records_operator_identity == false
    and .records_operator_session == false
    and .records_session_binding == false
    and .accepts_replay == false
    and .accepts_cross_session_binding == false
    and .records_revocation == false
    and .records_logout == false
    and .accepts_revocation_replay == false
    and .records_reinstatement == false
    and .accepts_reinstatement_replay == false
    and .records_ordering == false
    and .records_monotonicity == false
    and .accepts_cancellation == false
    and .accepts_supersession == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement ordering/monotonicity denial gate passed"
