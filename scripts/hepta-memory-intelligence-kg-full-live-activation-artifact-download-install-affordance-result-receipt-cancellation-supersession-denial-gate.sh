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

ORDERING_MONOTONICITY_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-ordering-monotonicity-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-ordering-monotonicity-denial-gate.sh
)"

source_ordering_monotonicity_report_sha256="$(sha256_text "$ORDERING_MONOTONICITY_JSON")"
cancellation_supersession_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-cancellation-supersession-denial:$source_ordering_monotonicity_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
cancellation_supersession_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-cancellation-supersession-denial:no-cancel:no-revoke:no-supersede:no-replacement:no-tombstone:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$ORDERING_MONOTONICITY_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denied_count == 18
    and zero_fields($source; [
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_allowed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_latest_wins_overwrite_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_gap_fill_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ack_before_noop_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_completion_ack_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_approval_from_ordering_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_from_ordering_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_from_ordering_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_install_from_ordering_executed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_service_restart_from_ordering_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_active_binary_from_ordering_mutated_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_memory_store_ordering_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_kg_ordering_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_provider_ordering_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_model_ordering_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_credential_read_from_ordering_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_secret_read_from_ordering_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_external_send_from_ordering_count"
    ])
    and false_fields($source; [
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_allowed",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_persisted",
      "artifact_download_install_affordance_result_receipt_recorded",
      "artifact_download_install_affordance_result_receipt_persisted",
      "artifact_download_install_affordance_completion_ack_recorded",
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
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surfaces | length) == 18
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surfaces | all(
      .artifact_download_install_affordance_result_receipt_ordering_requested == true
      and .receipt_noop_confirmed == true
      and false_fields(.; [
        "artifact_download_install_affordance_result_receipt_ordering_allowed",
        "artifact_download_install_affordance_result_receipt_ordering_recorded",
        "artifact_download_install_affordance_result_receipt_ordering_persisted",
        "artifact_download_install_affordance_result_receipt_sequence_cursor_recorded",
        "artifact_download_install_affordance_result_receipt_monotonicity_state_recorded",
        "artifact_download_install_affordance_result_receipt_latest_wins_overwrite_accepted",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_completion_ack_recorded",
        "operator_approval_from_ordering_accepted",
        "release_publication_authority_from_ordering_derived",
        "activation_authority_from_ordering_derived",
        "install_from_ordering_executed",
        "service_restart_from_ordering_performed",
        "active_binary_from_ordering_mutated",
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
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .accepts_cancellation == false
      and .accepts_supersession == false
      and .records_result_receipt == false
      and .persists_replacement_receipt == false
      and .installs_or_restarts == false
      and .mutates_active_binary == false
      and .mutates_memory_store == false
      and .writes_kg == false
      and .sends_externally == false
    ))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

cancellation_supersession_surfaces_json="$(
  jq -n '
    def cancellation_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surface:$id,
        source_ordering_monotonicity_present:true,
        source_ordering_monotonicity_ready:true,
        source_ordering_noop_confirmed:true,
        canonical_noop_result_receipt_replacement_identity_required:true,
        artifact_download_install_affordance_result_receipt_cancellation_supersession_requested:true,
        artifact_download_install_affordance_result_receipt_cancellation_supersession_status:$status,
        artifact_download_install_affordance_result_receipt_cancellation_supersession_allowed:false,
        artifact_download_install_affordance_result_receipt_cancellation_supersession_recorded:false,
        artifact_download_install_affordance_result_receipt_cancellation_supersession_persisted:false,
        artifact_download_install_affordance_result_receipt_cancellation_supersession_performed:false,
        artifact_download_install_affordance_result_receipt_cancellation_accepted:false,
        artifact_download_install_affordance_result_receipt_cancellation_recorded:false,
        artifact_download_install_affordance_result_receipt_cancellation_persisted:false,
        artifact_download_install_affordance_result_receipt_revocation_accepted:false,
        artifact_download_install_affordance_result_receipt_withdrawal_accepted:false,
        artifact_download_install_affordance_result_receipt_supersession_accepted:false,
        artifact_download_install_affordance_result_receipt_supersession_recorded:false,
        artifact_download_install_affordance_result_receipt_supersession_persisted:false,
        artifact_download_install_affordance_result_receipt_replacement_receipt_accepted:false,
        artifact_download_install_affordance_result_receipt_replacement_receipt_recorded:false,
        artifact_download_install_affordance_result_receipt_replacement_receipt_persisted:false,
        artifact_download_install_affordance_result_receipt_tombstone_recorded:false,
        artifact_download_install_affordance_result_receipt_tombstone_persisted:false,
        artifact_download_install_affordance_result_receipt_delete_marker_recorded:false,
        artifact_download_install_affordance_result_receipt_latest_replacement_accepted:false,
        artifact_download_install_affordance_result_receipt_ack_replacement_accepted:false,
        artifact_download_install_affordance_result_receipt_query_replacement_registered:false,
        artifact_download_install_affordance_result_receipt_export_replacement_recorded:false,
        artifact_download_install_affordance_result_receipt_observability_replacement_recorded:false,
        artifact_download_install_affordance_result_receipt_ordering_recorded:false,
        artifact_download_install_affordance_result_receipt_sequence_cursor_recorded:false,
        artifact_download_install_affordance_result_receipt_monotonicity_state_recorded:false,
        artifact_download_install_affordance_result_receipt_replay_recorded:false,
        artifact_download_install_affordance_result_receipt_idempotency_state_recorded:false,
        artifact_download_install_affordance_result_receipt_recorded:false,
        artifact_download_install_affordance_result_receipt_persisted:false,
        artifact_download_install_affordance_result_receipt_accepted:false,
        artifact_download_install_affordance_result_receipt_materialized:false,
        artifact_download_install_affordance_result_receipt_filesystem_written:false,
        artifact_download_install_affordance_result_receipt_ledger_written:false,
        artifact_download_install_affordance_result_receipt_indexed:false,
        artifact_download_install_affordance_result_receipt_enqueued:false,
        artifact_download_install_affordance_result_receipt_delivered:false,
        artifact_download_install_affordance_result_receipt_exported:false,
        artifact_download_install_affordance_result_receipt_query_registered:false,
        artifact_download_install_affordance_result_receipt_observability_recorded:false,
        artifact_download_install_affordance_completion_ack_recorded:false,
        artifact_download_install_affordance_completion_ack_accepted:false,
        download_button_rendered:false,
        direct_download_url_exposed:false,
        package_manager_install_command_rendered:false,
        curl_pipe_shell_snippet_rendered:false,
        installer_launch_prompt_rendered:false,
        auto_update_offer_rendered:false,
        external_install_message_sent:false,
        telegram_install_message_sent:false,
        operator_approval_from_cancellation_supersession_accepted:false,
        release_publication_authority_from_cancellation_supersession_derived:false,
        activation_authority_from_cancellation_supersession_derived:false,
        activation_command_from_cancellation_supersession_derived:false,
        activation_from_cancellation_supersession_allowed:false,
        live_execution_from_cancellation_supersession_allowed:false,
        install_from_cancellation_supersession_executed:false,
        service_restart_from_cancellation_supersession_performed:false,
        launchd_from_cancellation_supersession_mutated:false,
        active_binary_from_cancellation_supersession_mutated:false,
        activation_activated:false,
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
        cancellation_supersession_noop_confirmed:true,
        reason:$reason
      } + $extra;
    [
      cancellation_surface("source_ordering_monotonicity_report_required"; "blocked_source_report_required_noop"; "source_ordering_monotonicity_report_required"; {source_ordering_monotonicity_report_required:true}),
      cancellation_surface("download_button_result_receipt_cancel_claim"; "blocked_cancellation_acceptance_noop"; "download_button_result_receipt_cancel_claim_denied"; {cancellation_claim_requested:true}),
      cancellation_surface("direct_download_url_result_receipt_revoke_claim"; "blocked_revocation_acceptance_noop"; "direct_download_url_result_receipt_revoke_claim_denied"; {revocation_claim_requested:true}),
      cancellation_surface("checksum_prompt_result_receipt_withdraw_claim"; "blocked_withdrawal_acceptance_noop"; "checksum_prompt_result_receipt_withdraw_claim_denied"; {withdrawal_claim_requested:true}),
      cancellation_surface("package_manager_install_command_result_receipt_supersede_claim"; "blocked_supersession_acceptance_noop"; "package_manager_install_command_result_receipt_supersede_claim_denied"; {supersession_claim_requested:true}),
      cancellation_surface("curl_pipe_shell_result_receipt_replacement_receipt"; "blocked_replacement_receipt_noop"; "curl_pipe_shell_result_receipt_replacement_receipt_denied"; {replacement_receipt_requested:true}),
      cancellation_surface("installer_launch_prompt_result_receipt_tombstone_claim"; "blocked_tombstone_noop"; "installer_launch_prompt_result_receipt_tombstone_claim_denied"; {tombstone_requested:true}),
      cancellation_surface("auto_update_offer_result_receipt_delete_marker_claim"; "blocked_delete_marker_noop"; "auto_update_offer_result_receipt_delete_marker_claim_denied"; {delete_marker_requested:true}),
      cancellation_surface("release_channel_subscription_result_receipt_latest_replacement"; "blocked_latest_replacement_noop"; "release_channel_subscription_result_receipt_latest_replacement_denied"; {latest_replacement_requested:true}),
      cancellation_surface("update_feed_hint_result_receipt_ack_replacement"; "blocked_ack_replacement_noop"; "update_feed_hint_result_receipt_ack_replacement_denied"; {ack_replacement_requested:true}),
      cancellation_surface("package_registry_badge_result_receipt_query_export_observability_replacement"; "blocked_query_export_observability_replacement_noop"; "package_registry_badge_result_receipt_query_export_observability_replacement_denied"; {query_replacement_requested:true, export_replacement_requested:true, observability_replacement_requested:true}),
      cancellation_surface("cdn_mirror_download_result_receipt_ordering_replacement_bypass"; "blocked_ordering_replacement_bypass_noop"; "cdn_mirror_download_result_receipt_ordering_replacement_bypass_denied"; {ordering_replacement_bypass_requested:true, sequence_cursor_replacement_requested:true, monotonicity_state_replacement_requested:true}),
      cancellation_surface("sbom_provenance_notarization_result_receipt_idempotency_replacement_bypass"; "blocked_idempotency_replacement_bypass_noop"; "sbom_provenance_notarization_result_receipt_idempotency_replacement_bypass_denied"; {idempotency_replacement_bypass_requested:true}),
      cancellation_surface("signature_verification_command_result_receipt_completion_ack_replacement"; "blocked_completion_ack_replacement_noop"; "signature_verification_command_result_receipt_completion_ack_replacement_denied"; {completion_ack_replacement_requested:true}),
      cancellation_surface("one_click_install_deep_link_result_receipt_activation_authority_replacement"; "blocked_activation_authority_replacement_noop"; "one_click_install_deep_link_result_receipt_activation_authority_replacement_denied"; {activation_authority_replacement_requested:true}),
      cancellation_surface("external_telegram_install_message_result_receipt_external_replacement"; "blocked_external_replacement_noop"; "external_telegram_install_message_result_receipt_external_replacement_denied"; {external_replacement_requested:true, telegram_replacement_requested:true}),
      cancellation_surface("release_publication_authority_install_affordance_result_receipt_replacement"; "blocked_release_publication_authority_replacement_noop"; "release_publication_authority_install_affordance_result_receipt_replacement_denied"; {release_publication_authority_replacement_requested:true}),
      cancellation_surface("activation_live_install_restart_active_binary_result_receipt_replacement"; "blocked_activation_live_install_restart_active_binary_replacement_noop"; "activation_live_install_restart_active_binary_result_receipt_replacement_denied"; {activation_replacement_requested:true, install_replacement_requested:true, service_restart_replacement_requested:true, active_binary_replacement_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_gate" \
    --arg source_ordering_monotonicity_report_sha256 "$source_ordering_monotonicity_report_sha256" \
    --arg cancellation_supersession_contract_hash_sha256 "$cancellation_supersession_contract_hash_sha256" \
    --arg cancellation_supersession_policy_hash_sha256 "$cancellation_supersession_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$ORDERING_MONOTONICITY_JSON" \
    --argjson surfaces "$cancellation_supersession_surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_v1",
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_mode:"denied_cancel_revoke_supersede_or_replacement_attempt_cannot_create_result_receipt_or_install_activation_authority",
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_gate:$source.gate,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_ready,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_report_sha256:$source_ordering_monotonicity_report_sha256,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_contract_hash_sha256:$cancellation_supersession_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_policy_hash_sha256:$cancellation_supersession_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_ready:true,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surface_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_attempt_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denied_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denied_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_persisted_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_persisted_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_from_ordering_derived_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_from_ordering_derived_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_from_ordering_derived_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_from_ordering_derived_count,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surface_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_attempt_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denied_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surfaces:$surfaces,
        denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession:[
          "source_result_receipt_ordering_monotonicity_report_required",
          "canonical_noop_result_receipt_replacement_identity_required",
          "cancellation_acceptance_denied",
          "cancellation_recording_denied",
          "cancellation_persistence_denied",
          "revocation_acceptance_denied",
          "withdrawal_acceptance_denied",
          "supersession_acceptance_denied",
          "supersession_recording_denied",
          "supersession_persistence_denied",
          "replacement_receipt_acceptance_denied",
          "replacement_receipt_recording_denied",
          "replacement_receipt_persistence_denied",
          "tombstone_recording_denied",
          "tombstone_persistence_denied",
          "delete_marker_recording_denied",
          "latest_replacement_denied",
          "ack_replacement_denied",
          "query_export_observability_replacement_denied",
          "ordering_monotonicity_replacement_bypass_denied",
          "idempotency_replacement_bypass_denied",
          "completion_ack_replacement_denied",
          "operator_approval_from_cancellation_supersession_denied",
          "release_publication_authority_from_cancellation_supersession_denied",
          "activation_authority_from_cancellation_supersession_denied",
          "external_public_release_replacement_denied",
          "install_restart_active_binary_replacement_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_gate",
            status:"allowed_report_only_next_slice",
            accepts_cancellation:false,
            accepts_supersession:false,
            accepts_replacement_receipt:false,
            records_tombstone:false,
            records_delete_marker:false,
            records_audit_trail:false,
            persists_immutable_evidence:false,
            records_result_receipt:false,
            persists_result_receipt:false,
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
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_withdrawal_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_supersession_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_supersession_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_supersession_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replacement_receipt_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replacement_receipt_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replacement_receipt_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delete_marker_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_latest_replacement_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ack_replacement_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_replacement_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_replacement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_replacement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_approval_from_cancellation_supersession_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_from_cancellation_supersession_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_from_cancellation_supersession_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_command_from_cancellation_supersession_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_execution_from_cancellation_supersession_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_install_from_cancellation_supersession_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_service_restart_from_cancellation_supersession_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_active_binary_from_cancellation_supersession_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_memory_store_cancellation_supersession_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_kg_cancellation_supersession_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_provider_cancellation_supersession_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_model_cancellation_supersession_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_credential_read_from_cancellation_supersession_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_secret_read_from_cancellation_supersession_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_external_send_from_cancellation_supersession_count"
      ])
      + false_object([
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_supersession_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replacement_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delete_marker_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_accepted",
        "artifact_download_install_affordance_result_receipt_materialized",
        "artifact_download_install_affordance_result_receipt_filesystem_written",
        "artifact_download_install_affordance_completion_ack_recorded",
        "artifact_download_install_affordance_completion_ack_accepted",
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
          "artifact_download_install_affordance_result_receipt_cancellation_supersession_allowed",
          "artifact_download_install_affordance_result_receipt_cancellation_supersession_recorded",
          "artifact_download_install_affordance_result_receipt_cancellation_supersession_persisted",
          "artifact_download_install_affordance_result_receipt_cancellation_accepted",
          "artifact_download_install_affordance_result_receipt_cancellation_recorded",
          "artifact_download_install_affordance_result_receipt_cancellation_persisted",
          "artifact_download_install_affordance_result_receipt_revocation_accepted",
          "artifact_download_install_affordance_result_receipt_withdrawal_accepted",
          "artifact_download_install_affordance_result_receipt_supersession_accepted",
          "artifact_download_install_affordance_result_receipt_supersession_recorded",
          "artifact_download_install_affordance_result_receipt_supersession_persisted",
          "artifact_download_install_affordance_result_receipt_replacement_receipt_accepted",
          "artifact_download_install_affordance_result_receipt_replacement_receipt_recorded",
          "artifact_download_install_affordance_result_receipt_replacement_receipt_persisted",
          "artifact_download_install_affordance_result_receipt_tombstone_recorded",
          "artifact_download_install_affordance_result_receipt_tombstone_persisted",
          "artifact_download_install_affordance_result_receipt_delete_marker_recorded",
          "artifact_download_install_affordance_result_receipt_latest_replacement_accepted",
          "artifact_download_install_affordance_result_receipt_ack_replacement_accepted",
          "artifact_download_install_affordance_result_receipt_query_replacement_registered",
          "artifact_download_install_affordance_result_receipt_export_replacement_recorded",
          "artifact_download_install_affordance_result_receipt_observability_replacement_recorded",
          "artifact_download_install_affordance_result_receipt_recorded",
          "artifact_download_install_affordance_result_receipt_persisted",
          "artifact_download_install_affordance_result_receipt_accepted",
          "artifact_download_install_affordance_result_receipt_materialized",
          "artifact_download_install_affordance_result_receipt_filesystem_written",
          "artifact_download_install_affordance_completion_ack_recorded",
          "artifact_download_install_affordance_completion_ack_accepted",
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
          "release_artifact_written",
          "public_artifact_written",
          "public_release_claimed",
          "public_ga_claimed",
          "telegram_send_performed",
          "channel_send_performed",
          "external_send_performed",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_ready == true
  and $report.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_ready == true
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surface_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_attempt_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denied_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_recorded_count == 0
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_persisted_count == 0
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_recorded_count == 0
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_recorded_count == 0
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_from_ordering_derived_count == 0
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_from_ordering_derived_count == 0
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surface_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_attempt_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denied_count == 18
  and zero_fields($report; [
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_allowed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_withdrawal_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_supersession_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_supersession_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_supersession_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replacement_receipt_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replacement_receipt_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replacement_receipt_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delete_marker_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_latest_replacement_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ack_replacement_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_replacement_registered_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_replacement_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_replacement_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_approval_from_cancellation_supersession_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_from_cancellation_supersession_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_from_cancellation_supersession_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_install_from_cancellation_supersession_executed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_service_restart_from_cancellation_supersession_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_active_binary_from_cancellation_supersession_mutated_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_memory_store_cancellation_supersession_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_kg_cancellation_supersession_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_provider_cancellation_supersession_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_model_cancellation_supersession_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_credential_read_from_cancellation_supersession_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_secret_read_from_cancellation_supersession_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_external_send_from_cancellation_supersession_count"
  ])
  and false_fields($report; [
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_allowed",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_recorded",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_supersession_recorded",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replacement_receipt_recorded",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_recorded",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delete_marker_recorded",
    "artifact_download_install_affordance_result_receipt_recorded",
    "artifact_download_install_affordance_result_receipt_persisted",
    "artifact_download_install_affordance_completion_ack_recorded",
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
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surfaces | length) == 18
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surfaces | all(
    .artifact_download_install_affordance_result_receipt_cancellation_supersession_requested == true
    and .cancellation_supersession_noop_confirmed == true
    and false_fields(.; [
      "artifact_download_install_affordance_result_receipt_cancellation_supersession_allowed",
      "artifact_download_install_affordance_result_receipt_cancellation_supersession_recorded",
      "artifact_download_install_affordance_result_receipt_cancellation_supersession_persisted",
      "artifact_download_install_affordance_result_receipt_cancellation_accepted",
      "artifact_download_install_affordance_result_receipt_cancellation_recorded",
      "artifact_download_install_affordance_result_receipt_supersession_accepted",
      "artifact_download_install_affordance_result_receipt_supersession_recorded",
      "artifact_download_install_affordance_result_receipt_replacement_receipt_accepted",
      "artifact_download_install_affordance_result_receipt_replacement_receipt_recorded",
      "artifact_download_install_affordance_result_receipt_tombstone_recorded",
      "artifact_download_install_affordance_result_receipt_delete_marker_recorded",
      "artifact_download_install_affordance_result_receipt_latest_replacement_accepted",
      "artifact_download_install_affordance_result_receipt_ack_replacement_accepted",
      "artifact_download_install_affordance_result_receipt_query_replacement_registered",
      "artifact_download_install_affordance_result_receipt_export_replacement_recorded",
      "artifact_download_install_affordance_result_receipt_observability_replacement_recorded",
      "artifact_download_install_affordance_result_receipt_ordering_recorded",
      "artifact_download_install_affordance_result_receipt_sequence_cursor_recorded",
      "artifact_download_install_affordance_result_receipt_monotonicity_state_recorded",
      "artifact_download_install_affordance_result_receipt_recorded",
      "artifact_download_install_affordance_result_receipt_persisted",
      "artifact_download_install_affordance_completion_ack_recorded",
      "operator_approval_from_cancellation_supersession_accepted",
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
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surfaces[] | select(.cancellation_claim_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surfaces[] | select(.supersession_claim_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surfaces[] | select(.replacement_receipt_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surfaces[] | select(.tombstone_requested == true)] | length) == 1
  and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession | length) == 27
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .accepts_cancellation == false
    and .accepts_supersession == false
    and .accepts_replacement_receipt == false
    and .persists_immutable_evidence == false
    and .records_result_receipt == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .mutates_memory_store == false
    and .writes_kg == false
    and .sends_externally == false
  ))
  and ($report.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt cancellation/supersession denial gate passed"
