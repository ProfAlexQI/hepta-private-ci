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

ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_DENIAL_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-denial-gate" \
    scripts/i3-8695f13a365f7ff86105233c.sh
)"

source_artifact_download_install_affordance_denial_report_sha256="$(
  sha256_text "$ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_DENIAL_JSON"
)"
artifact_download_install_affordance_result_receipt_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-no-persistence-denial:$source_artifact_download_install_affordance_denial_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
artifact_download_install_affordance_result_receipt_policy_hash_sha256="$(
  sha256_text "release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-no-persistence-denial:no-receipt-record:no-receipt-persist:no-ledger:no-index:no-export:no-query:no-observability:no-idempotency:no-install:no-live"
)"

jq -n -e \
  --argjson source "$ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_DENIAL_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_attempt_count == 18
    and zero_fields($source; [
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_allowed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_request_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_materialized_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_filesystem_written_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_delivered_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_exposed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_published_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_executed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_download_button_rendered_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_direct_download_url_exposed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manager_install_command_rendered_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_curl_pipe_shell_snippet_rendered_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_launch_prompt_rendered_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_auto_update_offer_rendered_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_external_install_message_sent_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_install_message_sent_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_acceptance_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_approval_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_command_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_execution_allowed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_install_executed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_service_restarted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_active_binary_mutated_count"
    ])
    and false_fields($source; [
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_accepted",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_persisted",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_materialized",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_filesystem_written",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_delivered",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_exposed",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_published",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_executed",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_download_button_rendered",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_direct_download_url_exposed",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manager_install_command_rendered",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_curl_pipe_shell_snippet_rendered",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_launch_prompt_rendered",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_auto_update_offer_rendered",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_install_message_sent",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_install_message_sent",
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
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_surfaces | length) == 18
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_surfaces | all(
      .artifact_download_install_affordance_attempted == true
      and false_fields(.; [
        "artifact_download_install_affordance_allowed",
        "artifact_download_install_affordance_accepted",
        "artifact_download_install_affordance_recorded",
        "artifact_download_install_affordance_persisted",
        "artifact_download_install_affordance_materialized",
        "artifact_download_install_affordance_filesystem_written",
        "artifact_download_install_affordance_delivered",
        "artifact_download_install_affordance_exposed",
        "artifact_download_install_affordance_published",
        "artifact_download_install_affordance_executed",
        "download_button_rendered",
        "direct_download_url_exposed",
        "package_manager_install_command_rendered",
        "curl_pipe_shell_snippet_rendered",
        "installer_launch_prompt_rendered",
        "auto_update_offer_rendered",
        "external_install_message_sent",
        "telegram_install_message_sent",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_command_derived",
        "live_execution_allowed",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked"
      ])
      and .artifact_download_install_affordance_noop_confirmed == true
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_gate"
      and .status == "allowed_report_only_next_slice"
      and .renders_download_link == false
      and .emits_install_command == false
      and .prompts_installer == false
      and .publishes_update_offer == false
      and .sends_external_install_message == false
      and .records_operator_acceptance == false
      and .derives_release_publication_authority == false
      and .derives_activation_authority == false
      and .activates_live == false
      and .installs_or_restarts == false
      and .mutates_active_binary == false
      and .mutates_memory_store == false
      and .writes_kg == false
      and .sends_externally == false
    ))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

artifact_download_install_affordance_result_receipt_surfaces_json="$(
  jq -n '
    def result_receipt_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt:$id,
        source_artifact_download_install_affordance_denial_ready:true,
        source_artifact_download_install_affordance_noop_confirmed:true,
        artifact_download_install_affordance_result_receipt_attempted:true,
        artifact_download_install_affordance_result_receipt_allowed:false,
        artifact_download_install_affordance_result_receipt_schema_accepted:false,
        artifact_download_install_affordance_result_receipt_accepted:false,
        artifact_download_install_affordance_result_receipt_recorded:false,
        artifact_download_install_affordance_result_receipt_persisted:false,
        artifact_download_install_affordance_result_receipt_materialized:false,
        artifact_download_install_affordance_result_receipt_filesystem_written:false,
        artifact_download_install_affordance_result_receipt_ledger_written:false,
        artifact_download_install_affordance_result_receipt_indexed:false,
        artifact_download_install_affordance_result_receipt_enqueued:false,
        artifact_download_install_affordance_result_receipt_delivered:false,
        artifact_download_install_affordance_result_receipt_exported:false,
        artifact_download_install_affordance_result_receipt_query_registered:false,
        artifact_download_install_affordance_result_receipt_observability_recorded:false,
        artifact_download_install_affordance_result_receipt_hash_bound:false,
        artifact_download_install_affordance_result_receipt_status_accepted:false,
        artifact_download_install_affordance_completion_ack_recorded:false,
        artifact_download_install_affordance_completion_ack_persisted:false,
        artifact_download_install_affordance_completion_ack_accepted:false,
        artifact_download_install_affordance_completion_ack_materialized:false,
        artifact_download_install_affordance_completion_ack_delivered:false,
        download_button_rendered:false,
        direct_download_url_exposed:false,
        package_manager_install_command_rendered:false,
        curl_pipe_shell_snippet_rendered:false,
        installer_launch_prompt_rendered:false,
        auto_update_offer_rendered:false,
        external_install_message_sent:false,
        telegram_install_message_sent:false,
        acceptance_recorded:false,
        operator_approval_from_receipt_accepted:false,
        release_publication_authority_from_receipt_derived:false,
        activation_authority_from_receipt_derived:false,
        activation_command_from_receipt_derived:false,
        live_execution_from_receipt_allowed:false,
        activation_from_receipt_performed:false,
        install_from_receipt_executed:false,
        service_restart_from_receipt_performed:false,
        launchd_from_receipt_mutated:false,
        active_binary_from_receipt_mutated:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        live_kg_write_performed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_read:false,
        secret_file_read:false,
        receipt_noop_confirmed:true,
        artifact_download_install_affordance_result_receipt_status:$status,
        reason:$reason
      } + $extra;
    [
      result_receipt_surface("source_artifact_download_install_affordance_report_required"; "blocked_source_report_required_noop"; "source_artifact_download_install_affordance_report_required"; {source_artifact_download_install_affordance_report_required:true}),
      result_receipt_surface("download_button_result_receipt_recording"; "blocked_download_button_result_receipt_recording_noop"; "download_button_result_receipt_recording_denied"; {download_button_result_receipt_record_requested:true}),
      result_receipt_surface("direct_download_url_result_receipt_persistence"; "blocked_direct_download_url_result_receipt_persistence_noop"; "direct_download_url_result_receipt_persistence_denied"; {direct_download_url_result_receipt_persist_requested:true}),
      result_receipt_surface("checksum_prompt_result_receipt_materialization"; "blocked_checksum_prompt_result_receipt_materialization_noop"; "checksum_prompt_result_receipt_materialization_denied"; {checksum_prompt_result_receipt_materialize_requested:true}),
      result_receipt_surface("package_manager_install_command_result_receipt"; "blocked_package_manager_install_command_result_receipt_noop"; "package_manager_install_command_result_receipt_denied"; {package_manager_install_command_result_receipt_requested:true}),
      result_receipt_surface("curl_pipe_shell_result_receipt"; "blocked_curl_pipe_shell_result_receipt_noop"; "curl_pipe_shell_result_receipt_denied"; {curl_pipe_shell_result_receipt_requested:true}),
      result_receipt_surface("installer_launch_prompt_result_receipt"; "blocked_installer_launch_prompt_result_receipt_noop"; "installer_launch_prompt_result_receipt_denied"; {installer_launch_prompt_result_receipt_requested:true}),
      result_receipt_surface("auto_update_offer_result_receipt"; "blocked_auto_update_offer_result_receipt_noop"; "auto_update_offer_result_receipt_denied"; {auto_update_offer_result_receipt_requested:true}),
      result_receipt_surface("release_channel_subscription_result_receipt"; "blocked_release_channel_subscription_result_receipt_noop"; "release_channel_subscription_result_receipt_denied"; {release_channel_subscription_result_receipt_requested:true}),
      result_receipt_surface("update_feed_hint_result_receipt"; "blocked_update_feed_hint_result_receipt_noop"; "update_feed_hint_result_receipt_denied"; {update_feed_hint_result_receipt_requested:true}),
      result_receipt_surface("package_registry_badge_result_receipt"; "blocked_package_registry_badge_result_receipt_noop"; "package_registry_badge_result_receipt_denied"; {package_registry_badge_result_receipt_requested:true}),
      result_receipt_surface("cdn_mirror_download_result_receipt"; "blocked_cdn_mirror_download_result_receipt_noop"; "cdn_mirror_download_result_receipt_denied"; {cdn_mirror_download_result_receipt_requested:true}),
      result_receipt_surface("sbom_provenance_notarization_result_receipt"; "blocked_sbom_provenance_notarization_result_receipt_noop"; "sbom_provenance_notarization_result_receipt_denied"; {sbom_provenance_notarization_result_receipt_requested:true}),
      result_receipt_surface("signature_verification_command_result_receipt"; "blocked_signature_verification_command_result_receipt_noop"; "signature_verification_command_result_receipt_denied"; {signature_verification_command_result_receipt_requested:true}),
      result_receipt_surface("one_click_install_deep_link_result_receipt"; "blocked_one_click_install_deep_link_result_receipt_noop"; "one_click_install_deep_link_result_receipt_denied"; {one_click_install_deep_link_result_receipt_requested:true}),
      result_receipt_surface("external_telegram_install_message_result_receipt"; "blocked_external_telegram_install_message_result_receipt_noop"; "external_telegram_install_message_result_receipt_denied"; {external_install_message_result_receipt_requested:true, telegram_install_message_result_receipt_requested:true}),
      result_receipt_surface("release_publication_authority_install_affordance_result_receipt"; "blocked_release_publication_authority_install_affordance_result_receipt_noop"; "release_publication_authority_install_affordance_result_receipt_denied"; {release_publication_authority_install_affordance_result_receipt_requested:true}),
      result_receipt_surface("activation_live_install_restart_active_binary_result_receipt"; "blocked_activation_live_install_restart_active_binary_result_receipt_noop"; "activation_live_install_restart_active_binary_result_receipt_denied"; {activation_live_install_result_receipt_requested:true, install_restart_active_binary_result_receipt_requested:true})
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_denial_gate" \
  --arg source_artifact_download_install_affordance_denial_report_sha256 "$source_artifact_download_install_affordance_denial_report_sha256" \
  --arg artifact_download_install_affordance_result_receipt_contract_hash_sha256 "$artifact_download_install_affordance_result_receipt_contract_hash_sha256" \
  --arg artifact_download_install_affordance_result_receipt_policy_hash_sha256 "$artifact_download_install_affordance_result_receipt_policy_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_DENIAL_JSON" \
  --argjson surfaces "$artifact_download_install_affordance_result_receipt_surfaces_json" \
  '
    def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
    def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

    {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_denial_v1",
      receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_mode:"denied_download_install_affordance_cannot_emit_or_persist_a_result_receipt_or_install_activation_evidence",
      source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_gate:$source.gate,
      source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_ready,
      source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_report_sha256:$source_artifact_download_install_affordance_denial_report_sha256,
      source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_contract_hash_sha256,
      release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_contract_hash_sha256:$artifact_download_install_affordance_result_receipt_contract_hash_sha256,
      release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_policy_hash_sha256:$artifact_download_install_affordance_result_receipt_policy_hash_sha256,
      minimum_required_samples:$min_long_soak_samples,
      memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_denial_ready:true,
      source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_count,
      source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_attempt_count,
      source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_allowed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_allowed_count,
      source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_recorded_count,
      source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_persisted_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_persisted_count,
      source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_executed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_executed_count,
      release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_surface_count:($surfaces | length),
      release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_attempt_count:($surfaces | length),
      release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_surfaces:$surfaces,
      denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt:[
        "source_artifact_download_install_affordance_report_required",
        "download_button_result_receipt_recording_denied",
        "direct_download_url_result_receipt_persistence_denied",
        "checksum_prompt_result_receipt_materialization_denied",
        "package_manager_install_command_result_receipt_denied",
        "curl_pipe_shell_result_receipt_denied",
        "installer_launch_prompt_result_receipt_denied",
        "auto_update_offer_result_receipt_denied",
        "release_channel_subscription_result_receipt_denied",
        "update_feed_hint_result_receipt_denied",
        "package_registry_badge_result_receipt_denied",
        "cdn_mirror_download_result_receipt_denied",
        "sbom_provenance_notarization_result_receipt_denied",
        "signature_verification_command_result_receipt_denied",
        "one_click_install_deep_link_result_receipt_denied",
        "external_telegram_install_message_result_receipt_denied",
        "release_publication_authority_install_affordance_result_receipt_denied",
        "activation_live_install_restart_active_binary_result_receipt_denied",
        "result_receipt_schema_acceptance_denied",
        "result_receipt_recording_denied",
        "result_receipt_persistence_denied",
        "result_receipt_ledger_index_queue_delivery_denied",
        "result_receipt_export_query_observability_denied",
        "result_receipt_hash_status_identity_binding_denied",
        "completion_ack_from_result_receipt_denied",
        "operator_approval_from_result_receipt_denied",
        "release_publication_authority_from_result_receipt_denied",
        "activation_authority_from_result_receipt_denied",
        "install_restart_active_binary_from_result_receipt_denied",
        "memory_provider_kg_from_result_receipt_denied"
      ],
      allowed_next_actions:[
        {
          action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_gate",
          status:"allowed_report_only_next_slice",
          records_result_receipt:false,
          persists_result_receipt:false,
          records_idempotency:false,
          accepts_duplicate_receipt:false,
          renders_download_link:false,
          emits_install_command:false,
          prompts_installer:false,
          publishes_update_offer:false,
          sends_external_install_message:false,
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
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_allowed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_schema_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_materialized_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_filesystem_written_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_written_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_indexed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_enqueued_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivered_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_exported_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_bound_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_completion_ack_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_completion_ack_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_acceptance_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_approval_from_receipt_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_from_receipt_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_from_receipt_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_command_from_receipt_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_execution_from_receipt_allowed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_install_from_receipt_executed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_service_restart_from_receipt_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_active_binary_from_receipt_mutated_count"
    ])
    + false_object([
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_allowed",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_schema_accepted",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_accepted",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_persisted",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_materialized",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_filesystem_written",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_written",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_indexed",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_enqueued",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivered",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_exported",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_bound",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_accepted",
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
        "artifact_download_install_affordance_result_receipt_allowed",
        "artifact_download_install_affordance_result_receipt_schema_accepted",
        "artifact_download_install_affordance_result_receipt_accepted",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_materialized",
        "artifact_download_install_affordance_result_receipt_filesystem_written",
        "artifact_download_install_affordance_result_receipt_ledger_written",
        "artifact_download_install_affordance_result_receipt_indexed",
        "artifact_download_install_affordance_result_receipt_enqueued",
        "artifact_download_install_affordance_result_receipt_delivered",
        "artifact_download_install_affordance_result_receipt_exported",
        "artifact_download_install_affordance_result_receipt_query_registered",
        "artifact_download_install_affordance_result_receipt_observability_recorded",
        "artifact_download_install_affordance_result_receipt_hash_bound",
        "artifact_download_install_affordance_result_receipt_status_accepted",
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
        "external_send_performed",
        "filesystem_written"
      ])
    }')"

jq -e '
  def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
  def false_fields($o; $fields): all($fields[]; $o[.] == false);

  . as $report
  | $report.runtime == "hepta"
  and $report.status == "ready"
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_denial_ready == true
  and $report.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_ready == true
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_attempt_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_allowed_count == 0
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_recorded_count == 0
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_persisted_count == 0
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_executed_count == 0
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_surface_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_attempt_count == 18
  and zero_fields($report; [
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_allowed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_schema_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_materialized_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_filesystem_written_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_written_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_indexed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_enqueued_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivered_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_exported_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_bound_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_completion_ack_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_completion_ack_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_acceptance_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_approval_from_receipt_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_from_receipt_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_from_receipt_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_command_from_receipt_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_execution_from_receipt_allowed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_install_from_receipt_executed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_service_restart_from_receipt_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_active_binary_from_receipt_mutated_count"
  ])
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_surfaces | length) == 18
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_surfaces | all(
    .artifact_download_install_affordance_result_receipt_attempted == true
    and false_fields(.; [
      "artifact_download_install_affordance_result_receipt_allowed",
      "artifact_download_install_affordance_result_receipt_schema_accepted",
      "artifact_download_install_affordance_result_receipt_accepted",
      "artifact_download_install_affordance_result_receipt_recorded",
      "artifact_download_install_affordance_result_receipt_persisted",
      "artifact_download_install_affordance_result_receipt_materialized",
      "artifact_download_install_affordance_result_receipt_filesystem_written",
      "artifact_download_install_affordance_result_receipt_ledger_written",
      "artifact_download_install_affordance_result_receipt_indexed",
      "artifact_download_install_affordance_result_receipt_enqueued",
      "artifact_download_install_affordance_result_receipt_delivered",
      "artifact_download_install_affordance_result_receipt_exported",
      "artifact_download_install_affordance_result_receipt_query_registered",
      "artifact_download_install_affordance_result_receipt_observability_recorded",
      "artifact_download_install_affordance_result_receipt_hash_bound",
      "artifact_download_install_affordance_result_receipt_status_accepted",
      "artifact_download_install_affordance_completion_ack_recorded",
      "artifact_download_install_affordance_completion_ack_accepted",
      "operator_approval_from_receipt_accepted",
      "release_publication_authority_from_receipt_derived",
      "activation_authority_from_receipt_derived",
      "activation_command_from_receipt_derived",
      "live_execution_from_receipt_allowed",
      "install_from_receipt_executed",
      "service_restart_from_receipt_performed",
      "active_binary_from_receipt_mutated",
      "memory_store_write_performed",
      "memory_store_mutated",
      "live_kg_write_performed",
      "provider_invoked",
      "model_invoked",
      "credential_read",
      "secret_file_read"
    ])
    and .receipt_noop_confirmed == true
  ))
  and ($report.allowed_next_actions | all(
    .status == "allowed_report_only_next_slice"
    and .records_result_receipt == false
    and .persists_result_receipt == false
    and .records_idempotency == false
    and .accepts_duplicate_receipt == false
    and .renders_download_link == false
    and .emits_install_command == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .mutates_memory_store == false
    and .writes_kg == false
    and .sends_externally == false
  ))
  and false_fields($report; [
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_allowed",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_schema_accepted",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_accepted",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_recorded",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_persisted",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_materialized",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_filesystem_written",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_written",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_indexed",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_enqueued",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivered",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_exported",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_recorded",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_bound",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_accepted",
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
  and ($report.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt no-persistence denial gate passed"
