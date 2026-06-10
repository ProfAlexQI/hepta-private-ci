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

ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RESULT_RECEIPT_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-no-persistence-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-no-persistence-denial-gate.sh
)"

source_artifact_distribution_signing_notarization_result_receipt_report_sha256="$(
  sha256_text "$ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RESULT_RECEIPT_JSON"
)"
artifact_distribution_signing_notarization_receipt_replay_idempotency_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-distribution-signing-notarization-receipt-replay-idempotency-denial:$source_artifact_distribution_signing_notarization_result_receipt_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
artifact_distribution_signing_notarization_receipt_replay_idempotency_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-distribution-signing-notarization-receipt-replay-idempotency:no-duplicate:no-replay:no-idempotency:no-status-upgrade:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RESULT_RECEIPT_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_result_receipt_no_persistence_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_result_receipt_no_persistence_denial_ready == true
    and $source.source_artifact_distribution_signing_notarization_surface_ready == true
    and $source.artifact_distribution_signing_notarization_result_receipt_surface_count == 18
    and $source.artifact_distribution_signing_notarization_result_receipt_surface_attempt_count == 18
    and $source.artifact_distribution_signing_notarization_result_receipt_surface_denied_count == 18
    and zero_fields($source; [
      "artifact_distribution_signing_notarization_result_receipt_surface_allowed_count",
      "artifact_distribution_signing_notarization_result_receipt_surface_request_accepted_count",
      "artifact_distribution_signing_notarization_result_receipt_surface_accepted_count",
      "artifact_distribution_signing_notarization_result_receipt_surface_recorded_count",
      "artifact_distribution_signing_notarization_result_receipt_surface_persisted_count",
      "artifact_distribution_signing_notarization_result_receipt_surface_materialized_count",
      "artifact_distribution_signing_notarization_result_receipt_surface_filesystem_written_count",
      "artifact_distribution_signing_notarization_result_receipt_surface_delivered_count",
      "artifact_distribution_signing_notarization_result_receipt_surface_indexed_count",
      "artifact_distribution_signing_notarization_result_receipt_surface_exported_count",
      "artifact_distribution_signing_notarization_result_receipt_surface_query_registered_count",
      "artifact_distribution_signing_notarization_result_receipt_surface_observability_recorded_count",
      "artifact_distribution_signing_notarization_result_receipt_surface_status_exposed_count",
      "artifact_signing_receipt_accepted_count",
      "package_signing_receipt_accepted_count",
      "signature_manifest_receipt_recorded_count",
      "notarization_submission_receipt_persisted_count",
      "notarization_ticket_receipt_materialized_count",
      "stapling_receipt_filesystem_written_count",
      "installer_signing_receipt_delivered_count",
      "provenance_attestation_receipt_indexed_count",
      "sbom_manifest_receipt_exported_count",
      "release_asset_bundle_receipt_query_registered_count",
      "cdn_update_feed_receipt_observability_recorded_count",
      "package_registry_receipt_status_exposed_count",
      "dashboard_endpoint_receipt_status_exposed_count",
      "external_signing_receipt_delivered_count",
      "telegram_signing_receipt_delivered_count",
      "acceptance_from_signing_receipt_recorded_count",
      "operator_approval_from_signing_receipt_derived_count",
      "release_publication_authority_from_signing_receipt_derived_count",
      "activation_authority_from_signing_receipt_derived_count",
      "download_link_from_signing_receipt_rendered_count",
      "install_command_from_signing_receipt_rendered_count",
      "install_from_signing_receipt_executed_count",
      "service_restart_from_signing_receipt_performed_count",
      "active_binary_from_signing_receipt_mutated_count",
      "memory_store_write_performed_count",
      "live_kg_write_performed_count",
      "provider_invoked_count",
      "model_invoked_count",
      "credential_read_count",
      "secret_file_read_count",
      "external_send_performed_count"
    ])
    and false_fields($source; [
      "artifact_distribution_signing_notarization_result_receipt_accepted",
      "artifact_distribution_signing_notarization_result_receipt_recorded",
      "artifact_distribution_signing_notarization_result_receipt_persisted",
      "artifact_distribution_signing_notarization_result_receipt_materialized",
      "artifact_distribution_signing_notarization_result_receipt_delivered",
      "artifact_distribution_signing_notarization_result_receipt_status_exposed",
      "artifact_signing_receipt_accepted",
      "package_signing_receipt_accepted",
      "signature_manifest_receipt_recorded",
      "notarization_submission_receipt_persisted",
      "notarization_ticket_receipt_materialized",
      "stapling_receipt_filesystem_written",
      "installer_signing_receipt_delivered",
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
      "launchd_mutated",
      "service_restarted",
      "active_binary_mutated",
      "external_send_performed"
    ])
    and ($source.artifact_distribution_signing_notarization_result_receipt_surfaces | length) == 18
    and ($source.artifact_distribution_signing_notarization_result_receipt_surfaces | all(
      .artifact_distribution_signing_notarization_result_receipt_surface_attempted == true
      and .artifact_distribution_signing_notarization_result_receipt_surface_allowed == false
      and .artifact_distribution_signing_notarization_result_receipt_surface_accepted == false
      and .artifact_distribution_signing_notarization_result_receipt_surface_recorded == false
      and .artifact_distribution_signing_notarization_result_receipt_surface_persisted == false
      and .artifact_distribution_signing_notarization_result_receipt_surface_status_exposed == false
      and .artifact_distribution_signing_notarization_result_receipt_surface_noop_confirmed == true
      and .release_publication_authority_derived == false
      and .activation_authority_derived == false
      and .install_executed == false
      and .service_restarted == false
      and .active_binary_mutated == false
      and .memory_store_write_performed == false
      and .live_kg_write_performed == false
      and .external_send_performed == false
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_replay_idempotency_denial_gate"
      and .records_signing_receipt == false
      and .persists_signing_receipt == false
      and .materializes_signing_receipt == false
      and .delivers_signing_receipt == false
      and .exposes_signing_receipt_status == false
      and .replays_signing_receipt == false
      and .accepts_idempotency_key == false
      and .records_operator_acceptance == false
      and .derives_release_publication_authority == false
      and .derives_activation_authority == false
      and .renders_download_link == false
      and .emits_install_command == false
      and .installs_or_restarts == false
      and .mutates_active_binary == false
      and .mutates_memory_store == false
      and .writes_kg == false
      and .invokes_provider == false
      and .reads_credentials == false
      and .sends_externally == false
    ))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

surfaces_json="$(
  jq -n '
    def replay_surface($id; $status; $reason; $extra):
      {
        surface:$id,
        source_artifact_distribution_signing_notarization_result_receipt_no_persistence_ready:true,
        canonical_noop_signing_receipt_identity_required:true,
        artifact_distribution_signing_notarization_receipt_replay_idempotency_attempted:true,
        artifact_distribution_signing_notarization_receipt_replay_idempotency_allowed:false,
        artifact_distribution_signing_notarization_receipt_replay_allowed:false,
        artifact_distribution_signing_notarization_receipt_replay_accepted:false,
        artifact_distribution_signing_notarization_receipt_replay_recorded:false,
        artifact_distribution_signing_notarization_receipt_replay_persisted:false,
        artifact_distribution_signing_notarization_receipt_replay_performed:false,
        artifact_distribution_signing_notarization_receipt_duplicate_accepted:false,
        artifact_distribution_signing_notarization_receipt_duplicate_recorded:false,
        artifact_distribution_signing_notarization_receipt_duplicate_persisted:false,
        artifact_distribution_signing_notarization_receipt_idempotency_key_accepted:false,
        artifact_distribution_signing_notarization_receipt_idempotency_key_recorded:false,
        artifact_distribution_signing_notarization_receipt_idempotency_state_recorded:false,
        artifact_distribution_signing_notarization_receipt_idempotency_state_persisted:false,
        artifact_distribution_signing_notarization_receipt_idempotency_state_materialized:false,
        artifact_distribution_signing_notarization_receipt_idempotency_filesystem_written:false,
        artifact_distribution_signing_notarization_receipt_replay_nonce_accepted:false,
        artifact_distribution_signing_notarization_receipt_replay_nonce_recorded:false,
        artifact_distribution_signing_notarization_receipt_cross_scope_reuse_accepted:false,
        artifact_distribution_signing_notarization_receipt_status_upgrade_accepted:false,
        artifact_distribution_signing_notarization_receipt_completed_status_accepted:false,
        artifact_distribution_signing_notarization_receipt_ack_replay_accepted:false,
        artifact_distribution_signing_notarization_receipt_ledger_replay_accepted:false,
        artifact_distribution_signing_notarization_receipt_index_replay_accepted:false,
        artifact_distribution_signing_notarization_receipt_delivery_replay_accepted:false,
        artifact_distribution_signing_notarization_receipt_query_replay_accepted:false,
        artifact_distribution_signing_notarization_receipt_export_replay_accepted:false,
        artifact_distribution_signing_notarization_receipt_observability_replay_accepted:false,
        artifact_distribution_signing_notarization_receipt_hash_status_rebind_accepted:false,
        artifact_signing_receipt_replay_accepted:false,
        package_signing_receipt_replay_accepted:false,
        signature_manifest_receipt_idempotency_recorded:false,
        notarization_submission_receipt_idempotency_persisted:false,
        notarization_ticket_receipt_nonce_recorded:false,
        stapling_receipt_cross_scope_reuse_accepted:false,
        installer_signing_receipt_out_of_order_accepted:false,
        provenance_attestation_receipt_ack_replay_accepted:false,
        sbom_manifest_receipt_ledger_index_replay_accepted:false,
        release_asset_bundle_receipt_export_query_replay_accepted:false,
        cdn_update_feed_receipt_observability_replay_accepted:false,
        package_registry_receipt_status_rebind_accepted:false,
        dashboard_endpoint_receipt_hash_status_replay_accepted:false,
        external_signing_receipt_delivery_replay_accepted:false,
        telegram_signing_receipt_delivery_replay_accepted:false,
        public_release_claimed:false,
        public_ga_claimed:false,
        acceptance_recorded:false,
        operator_approval_derived:false,
        release_publication_authority_derived:false,
        activation_authority_derived:false,
        download_link_rendered:false,
        install_command_rendered:false,
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
        telegram_send_performed:false,
        channel_send_performed:false,
        external_send_performed:false,
        artifact_distribution_signing_notarization_receipt_replay_idempotency_noop_confirmed:true,
        artifact_distribution_signing_notarization_receipt_replay_idempotency_status:$status,
        reason:$reason
      } + $extra;
    [
      replay_surface("source_signing_notarization_result_receipt_no_persistence_report_required"; "blocked_source_signing_receipt_no_persistence_required_noop"; "source_signing_notarization_result_receipt_no_persistence_report_required"; {source_report_required:true}),
      replay_surface("duplicate_artifact_signing_receipt_identity"; "blocked_duplicate_artifact_signing_receipt_identity_noop"; "duplicate_artifact_signing_receipt_identity_denied"; {duplicate_artifact_signing_receipt_identity_requested:true}),
      replay_surface("package_signing_receipt_replay_acceptance"; "blocked_package_signing_receipt_replay_acceptance_noop"; "package_signing_receipt_replay_acceptance_denied"; {package_signing_receipt_replay_acceptance_requested:true}),
      replay_surface("signature_manifest_receipt_idempotency_key"; "blocked_signature_manifest_receipt_idempotency_key_noop"; "signature_manifest_receipt_idempotency_key_denied"; {signature_manifest_receipt_idempotency_key_requested:true}),
      replay_surface("notarization_submission_receipt_idempotency_state"; "blocked_notarization_submission_receipt_idempotency_state_noop"; "notarization_submission_receipt_idempotency_state_denied"; {notarization_submission_receipt_idempotency_state_requested:true}),
      replay_surface("notarization_ticket_stale_nonce_replay"; "blocked_notarization_ticket_stale_nonce_replay_noop"; "notarization_ticket_stale_nonce_replay_denied"; {notarization_ticket_stale_nonce_replay_requested:true}),
      replay_surface("stapling_receipt_cross_scope_reuse"; "blocked_stapling_receipt_cross_scope_reuse_noop"; "stapling_receipt_cross_scope_reuse_denied"; {stapling_receipt_cross_scope_reuse_requested:true}),
      replay_surface("installer_signing_receipt_out_of_order_replay"; "blocked_installer_signing_receipt_out_of_order_replay_noop"; "installer_signing_receipt_out_of_order_replay_denied"; {installer_signing_receipt_out_of_order_replay_requested:true}),
      replay_surface("provenance_receipt_completion_ack_replay"; "blocked_provenance_receipt_completion_ack_replay_noop"; "provenance_receipt_completion_ack_replay_denied"; {provenance_receipt_completion_ack_replay_requested:true}),
      replay_surface("sbom_receipt_ledger_index_replay"; "blocked_sbom_receipt_ledger_index_replay_noop"; "sbom_receipt_ledger_index_replay_denied"; {sbom_receipt_ledger_index_replay_requested:true}),
      replay_surface("release_asset_bundle_receipt_export_query_replay"; "blocked_release_asset_bundle_receipt_export_query_replay_noop"; "release_asset_bundle_receipt_export_query_replay_denied"; {release_asset_bundle_receipt_export_query_replay_requested:true}),
      replay_surface("cdn_update_feed_receipt_observability_replay"; "blocked_cdn_update_feed_receipt_observability_replay_noop"; "cdn_update_feed_receipt_observability_replay_denied"; {cdn_update_feed_receipt_observability_replay_requested:true}),
      replay_surface("package_registry_receipt_status_rebind"; "blocked_package_registry_receipt_status_rebind_noop"; "package_registry_receipt_status_rebind_denied"; {package_registry_receipt_status_rebind_requested:true}),
      replay_surface("dashboard_endpoint_receipt_hash_status_replay"; "blocked_dashboard_endpoint_receipt_hash_status_replay_noop"; "dashboard_endpoint_receipt_hash_status_replay_denied"; {dashboard_endpoint_receipt_hash_status_replay_requested:true}),
      replay_surface("external_telegram_receipt_delivery_replay"; "blocked_external_telegram_receipt_delivery_replay_noop"; "external_telegram_receipt_delivery_replay_denied"; {external_signing_receipt_delivery_replay_requested:true, telegram_signing_receipt_delivery_replay_requested:true}),
      replay_surface("release_publication_authority_replay_from_signing_receipt"; "blocked_release_publication_authority_replay_from_signing_receipt_noop"; "release_publication_authority_replay_from_signing_receipt_denied"; {release_publication_authority_replay_from_signing_receipt_requested:true}),
      replay_surface("activation_live_install_replay_from_signing_receipt"; "blocked_activation_live_install_replay_from_signing_receipt_noop"; "activation_live_install_replay_from_signing_receipt_denied"; {activation_live_install_replay_from_signing_receipt_requested:true}),
      replay_surface("install_restart_active_binary_replay_path"; "blocked_install_restart_active_binary_replay_path_noop"; "install_restart_active_binary_replay_path_denied"; {install_restart_active_binary_replay_path_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_replay_idempotency_denial_gate" \
    --arg source_artifact_distribution_signing_notarization_result_receipt_report_sha256 "$source_artifact_distribution_signing_notarization_result_receipt_report_sha256" \
    --arg artifact_distribution_signing_notarization_receipt_replay_idempotency_contract_hash_sha256 "$artifact_distribution_signing_notarization_receipt_replay_idempotency_contract_hash_sha256" \
    --arg artifact_distribution_signing_notarization_receipt_replay_idempotency_policy_hash_sha256 "$artifact_distribution_signing_notarization_receipt_replay_idempotency_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RESULT_RECEIPT_JSON" \
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
        artifact_distribution_signing_notarization_receipt_replay_idempotency_schema_version:"operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_replay_idempotency_denial_v1",
        artifact_distribution_signing_notarization_receipt_replay_idempotency_mode:"denied_signing_receipt_no_persistence_cannot_be_replayed_deduplicated_cached_rebound_promoted_or_used_for_authority_or_live_install",
        source_artifact_distribution_signing_notarization_result_receipt_gate:$source.gate,
        source_artifact_distribution_signing_notarization_result_receipt_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_result_receipt_no_persistence_denial_ready,
        source_artifact_distribution_signing_notarization_result_receipt_report_sha256:$source_artifact_distribution_signing_notarization_result_receipt_report_sha256,
        source_artifact_distribution_signing_notarization_result_receipt_contract_hash_sha256:$source.artifact_distribution_signing_notarization_result_receipt_contract_hash_sha256,
        artifact_distribution_signing_notarization_receipt_replay_idempotency_contract_hash_sha256:$artifact_distribution_signing_notarization_receipt_replay_idempotency_contract_hash_sha256,
        artifact_distribution_signing_notarization_receipt_replay_idempotency_policy_hash_sha256:$artifact_distribution_signing_notarization_receipt_replay_idempotency_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_replay_idempotency_denial_ready:true,
        source_artifact_distribution_signing_notarization_result_receipt_surface_count:$source.artifact_distribution_signing_notarization_result_receipt_surface_count,
        source_artifact_distribution_signing_notarization_result_receipt_surface_attempt_count:$source.artifact_distribution_signing_notarization_result_receipt_surface_attempt_count,
        source_artifact_distribution_signing_notarization_result_receipt_surface_denied_count:$source.artifact_distribution_signing_notarization_result_receipt_surface_denied_count,
        source_artifact_distribution_signing_notarization_result_receipt_surface_recorded_count:$source.artifact_distribution_signing_notarization_result_receipt_surface_recorded_count,
        source_artifact_distribution_signing_notarization_result_receipt_surface_persisted_count:$source.artifact_distribution_signing_notarization_result_receipt_surface_persisted_count,
        source_artifact_distribution_signing_notarization_result_receipt_surface_status_exposed_count:$source.artifact_distribution_signing_notarization_result_receipt_surface_status_exposed_count,
        source_release_publication_authority_from_signing_receipt_derived_count:$source.release_publication_authority_from_signing_receipt_derived_count,
        source_activation_authority_from_signing_receipt_derived_count:$source.activation_authority_from_signing_receipt_derived_count,
        artifact_distribution_signing_notarization_receipt_replay_idempotency_surface_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_replay_idempotency_attempt_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_replay_idempotency_denied_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_replay_idempotency_surfaces:$surfaces,
        denied_by_artifact_distribution_signing_notarization_receipt_replay_idempotency:[
          "source_artifact_distribution_signing_notarization_result_receipt_no_persistence_report_required",
          "signing_receipt_replay_denied",
          "signing_receipt_duplicate_identity_denied",
          "signing_receipt_idempotency_key_denied",
          "signing_receipt_idempotency_state_denied",
          "signing_receipt_nonce_replay_denied",
          "signing_receipt_cross_scope_reuse_denied",
          "signing_receipt_status_upgrade_denied",
          "signing_receipt_completed_status_denied",
          "signing_receipt_completion_ack_replay_denied",
          "signing_receipt_ledger_index_delivery_replay_denied",
          "signing_receipt_export_query_observability_replay_denied",
          "signing_receipt_hash_status_rebind_denied",
          "external_telegram_signing_receipt_replay_denied",
          "release_publication_authority_from_signing_receipt_replay_denied",
          "activation_live_install_from_signing_receipt_replay_denied",
          "install_restart_active_binary_from_signing_receipt_replay_denied",
          "memory_provider_kg_secret_external_send_from_signing_receipt_replay_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denial_gate",
            status:"allowed_report_only_next_slice",
            replays_signing_receipt:false,
            records_duplicate_receipt:false,
            records_idempotency_key:false,
            persists_idempotency_state:false,
            accepts_cross_scope_reuse:false,
            accepts_status_upgrade:false,
            records_completion_ack:false,
            rebinds_hash_status:false,
            records_operator_acceptance:false,
            derives_release_publication_authority:false,
            derives_activation_authority:false,
            renders_download_link:false,
            emits_install_command:false,
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
        "artifact_distribution_signing_notarization_receipt_replay_idempotency_allowed_count",
        "artifact_distribution_signing_notarization_receipt_replay_allowed_count",
        "artifact_distribution_signing_notarization_receipt_replay_accepted_count",
        "artifact_distribution_signing_notarization_receipt_replay_recorded_count",
        "artifact_distribution_signing_notarization_receipt_replay_persisted_count",
        "artifact_distribution_signing_notarization_receipt_replay_performed_count",
        "artifact_distribution_signing_notarization_receipt_duplicate_accepted_count",
        "artifact_distribution_signing_notarization_receipt_duplicate_recorded_count",
        "artifact_distribution_signing_notarization_receipt_duplicate_persisted_count",
        "artifact_distribution_signing_notarization_receipt_idempotency_key_accepted_count",
        "artifact_distribution_signing_notarization_receipt_idempotency_key_recorded_count",
        "artifact_distribution_signing_notarization_receipt_idempotency_state_recorded_count",
        "artifact_distribution_signing_notarization_receipt_idempotency_state_persisted_count",
        "artifact_distribution_signing_notarization_receipt_idempotency_state_materialized_count",
        "artifact_distribution_signing_notarization_receipt_idempotency_filesystem_written_count",
        "artifact_distribution_signing_notarization_receipt_replay_nonce_accepted_count",
        "artifact_distribution_signing_notarization_receipt_replay_nonce_recorded_count",
        "artifact_distribution_signing_notarization_receipt_cross_scope_reuse_accepted_count",
        "artifact_distribution_signing_notarization_receipt_status_upgrade_accepted_count",
        "artifact_distribution_signing_notarization_receipt_completed_status_accepted_count",
        "artifact_distribution_signing_notarization_receipt_ack_replay_accepted_count",
        "artifact_distribution_signing_notarization_receipt_ledger_replay_accepted_count",
        "artifact_distribution_signing_notarization_receipt_index_replay_accepted_count",
        "artifact_distribution_signing_notarization_receipt_delivery_replay_accepted_count",
        "artifact_distribution_signing_notarization_receipt_query_replay_accepted_count",
        "artifact_distribution_signing_notarization_receipt_export_replay_accepted_count",
        "artifact_distribution_signing_notarization_receipt_observability_replay_accepted_count",
        "artifact_distribution_signing_notarization_receipt_hash_status_rebind_accepted_count",
        "artifact_signing_receipt_replay_accepted_count",
        "package_signing_receipt_replay_accepted_count",
        "signature_manifest_receipt_idempotency_recorded_count",
        "notarization_submission_receipt_idempotency_persisted_count",
        "notarization_ticket_receipt_nonce_recorded_count",
        "stapling_receipt_cross_scope_reuse_accepted_count",
        "installer_signing_receipt_out_of_order_accepted_count",
        "provenance_attestation_receipt_ack_replay_accepted_count",
        "sbom_manifest_receipt_ledger_index_replay_accepted_count",
        "release_asset_bundle_receipt_export_query_replay_accepted_count",
        "cdn_update_feed_receipt_observability_replay_accepted_count",
        "package_registry_receipt_status_rebind_accepted_count",
        "dashboard_endpoint_receipt_hash_status_replay_accepted_count",
        "external_signing_receipt_delivery_replay_accepted_count",
        "telegram_signing_receipt_delivery_replay_accepted_count",
        "acceptance_from_signing_receipt_replay_recorded_count",
        "operator_approval_from_signing_receipt_replay_derived_count",
        "release_publication_authority_from_signing_receipt_replay_derived_count",
        "activation_authority_from_signing_receipt_replay_derived_count",
        "download_link_from_signing_receipt_replay_rendered_count",
        "install_command_from_signing_receipt_replay_rendered_count",
        "install_from_signing_receipt_replay_executed_count",
        "service_restart_from_signing_receipt_replay_performed_count",
        "active_binary_from_signing_receipt_replay_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count"
      ])
      + false_object([
        "artifact_distribution_signing_notarization_receipt_replay_allowed",
        "artifact_distribution_signing_notarization_receipt_replay_accepted",
        "artifact_distribution_signing_notarization_receipt_replay_recorded",
        "artifact_distribution_signing_notarization_receipt_replay_persisted",
        "artifact_distribution_signing_notarization_receipt_replay_performed",
        "artifact_distribution_signing_notarization_receipt_duplicate_accepted",
        "artifact_distribution_signing_notarization_receipt_idempotency_key_accepted",
        "artifact_distribution_signing_notarization_receipt_idempotency_state_persisted",
        "artifact_distribution_signing_notarization_receipt_cross_scope_reuse_accepted",
        "artifact_distribution_signing_notarization_receipt_status_upgrade_accepted",
        "artifact_distribution_signing_notarization_receipt_completed_status_accepted",
        "artifact_distribution_signing_notarization_receipt_hash_status_rebind_accepted",
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
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "external_send_performed"
      ])
      + {
        side_effects:false_object([
          "artifact_distribution_signing_notarization_receipt_replay_recorded",
          "artifact_distribution_signing_notarization_receipt_replay_persisted",
          "artifact_distribution_signing_notarization_receipt_replay_performed",
          "artifact_distribution_signing_notarization_receipt_duplicate_recorded",
          "artifact_distribution_signing_notarization_receipt_duplicate_persisted",
          "artifact_distribution_signing_notarization_receipt_idempotency_key_recorded",
          "artifact_distribution_signing_notarization_receipt_idempotency_state_recorded",
          "artifact_distribution_signing_notarization_receipt_idempotency_state_persisted",
          "artifact_distribution_signing_notarization_receipt_idempotency_state_materialized",
          "artifact_distribution_signing_notarization_receipt_idempotency_filesystem_written",
          "artifact_distribution_signing_notarization_receipt_replay_nonce_recorded",
          "artifact_distribution_signing_notarization_receipt_completed_status_accepted",
          "artifact_distribution_signing_notarization_receipt_ack_replay_accepted",
          "artifact_distribution_signing_notarization_receipt_ledger_replay_accepted",
          "artifact_distribution_signing_notarization_receipt_index_replay_accepted",
          "artifact_distribution_signing_notarization_receipt_delivery_replay_accepted",
          "artifact_distribution_signing_notarization_receipt_query_replay_accepted",
          "artifact_distribution_signing_notarization_receipt_export_replay_accepted",
          "artifact_distribution_signing_notarization_receipt_observability_replay_accepted",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_replay_idempotency_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_replay_idempotency_denial_ready == true
  and $report.source_artifact_distribution_signing_notarization_result_receipt_ready == true
  and $report.source_artifact_distribution_signing_notarization_result_receipt_surface_count == 18
  and $report.source_artifact_distribution_signing_notarization_result_receipt_surface_attempt_count == 18
  and $report.source_artifact_distribution_signing_notarization_result_receipt_surface_denied_count == 18
  and $report.source_artifact_distribution_signing_notarization_result_receipt_surface_recorded_count == 0
  and $report.source_artifact_distribution_signing_notarization_result_receipt_surface_persisted_count == 0
  and $report.source_artifact_distribution_signing_notarization_result_receipt_surface_status_exposed_count == 0
  and $report.source_release_publication_authority_from_signing_receipt_derived_count == 0
  and $report.source_activation_authority_from_signing_receipt_derived_count == 0
  and $report.artifact_distribution_signing_notarization_receipt_replay_idempotency_surface_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_replay_idempotency_attempt_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_replay_idempotency_denied_count == 18
  and zero_fields($report; [
    "artifact_distribution_signing_notarization_receipt_replay_idempotency_allowed_count",
    "artifact_distribution_signing_notarization_receipt_replay_allowed_count",
    "artifact_distribution_signing_notarization_receipt_replay_accepted_count",
    "artifact_distribution_signing_notarization_receipt_replay_recorded_count",
    "artifact_distribution_signing_notarization_receipt_replay_persisted_count",
    "artifact_distribution_signing_notarization_receipt_replay_performed_count",
    "artifact_distribution_signing_notarization_receipt_duplicate_accepted_count",
    "artifact_distribution_signing_notarization_receipt_duplicate_recorded_count",
    "artifact_distribution_signing_notarization_receipt_duplicate_persisted_count",
    "artifact_distribution_signing_notarization_receipt_idempotency_key_accepted_count",
    "artifact_distribution_signing_notarization_receipt_idempotency_key_recorded_count",
    "artifact_distribution_signing_notarization_receipt_idempotency_state_recorded_count",
    "artifact_distribution_signing_notarization_receipt_idempotency_state_persisted_count",
    "artifact_distribution_signing_notarization_receipt_idempotency_state_materialized_count",
    "artifact_distribution_signing_notarization_receipt_idempotency_filesystem_written_count",
    "artifact_distribution_signing_notarization_receipt_replay_nonce_accepted_count",
    "artifact_distribution_signing_notarization_receipt_replay_nonce_recorded_count",
    "artifact_distribution_signing_notarization_receipt_cross_scope_reuse_accepted_count",
    "artifact_distribution_signing_notarization_receipt_status_upgrade_accepted_count",
    "artifact_distribution_signing_notarization_receipt_completed_status_accepted_count",
    "artifact_distribution_signing_notarization_receipt_ack_replay_accepted_count",
    "artifact_distribution_signing_notarization_receipt_ledger_replay_accepted_count",
    "artifact_distribution_signing_notarization_receipt_index_replay_accepted_count",
    "artifact_distribution_signing_notarization_receipt_delivery_replay_accepted_count",
    "artifact_distribution_signing_notarization_receipt_query_replay_accepted_count",
    "artifact_distribution_signing_notarization_receipt_export_replay_accepted_count",
    "artifact_distribution_signing_notarization_receipt_observability_replay_accepted_count",
    "artifact_distribution_signing_notarization_receipt_hash_status_rebind_accepted_count",
    "artifact_signing_receipt_replay_accepted_count",
    "package_signing_receipt_replay_accepted_count",
    "signature_manifest_receipt_idempotency_recorded_count",
    "notarization_submission_receipt_idempotency_persisted_count",
    "notarization_ticket_receipt_nonce_recorded_count",
    "stapling_receipt_cross_scope_reuse_accepted_count",
    "installer_signing_receipt_out_of_order_accepted_count",
    "provenance_attestation_receipt_ack_replay_accepted_count",
    "sbom_manifest_receipt_ledger_index_replay_accepted_count",
    "release_asset_bundle_receipt_export_query_replay_accepted_count",
    "cdn_update_feed_receipt_observability_replay_accepted_count",
    "package_registry_receipt_status_rebind_accepted_count",
    "dashboard_endpoint_receipt_hash_status_replay_accepted_count",
    "external_signing_receipt_delivery_replay_accepted_count",
    "telegram_signing_receipt_delivery_replay_accepted_count",
    "acceptance_from_signing_receipt_replay_recorded_count",
    "operator_approval_from_signing_receipt_replay_derived_count",
    "release_publication_authority_from_signing_receipt_replay_derived_count",
    "activation_authority_from_signing_receipt_replay_derived_count",
    "download_link_from_signing_receipt_replay_rendered_count",
    "install_command_from_signing_receipt_replay_rendered_count",
    "install_from_signing_receipt_replay_executed_count",
    "service_restart_from_signing_receipt_replay_performed_count",
    "active_binary_from_signing_receipt_replay_mutated_count",
    "memory_store_write_performed_count",
    "live_kg_write_performed_count",
    "provider_invoked_count",
    "model_invoked_count",
    "credential_read_count",
    "secret_file_read_count",
    "external_send_performed_count"
  ])
  and false_fields($report; [
    "artifact_distribution_signing_notarization_receipt_replay_allowed",
    "artifact_distribution_signing_notarization_receipt_replay_accepted",
    "artifact_distribution_signing_notarization_receipt_replay_recorded",
    "artifact_distribution_signing_notarization_receipt_replay_persisted",
    "artifact_distribution_signing_notarization_receipt_replay_performed",
    "artifact_distribution_signing_notarization_receipt_duplicate_accepted",
    "artifact_distribution_signing_notarization_receipt_idempotency_key_accepted",
    "artifact_distribution_signing_notarization_receipt_idempotency_state_persisted",
    "artifact_distribution_signing_notarization_receipt_cross_scope_reuse_accepted",
    "artifact_distribution_signing_notarization_receipt_status_upgrade_accepted",
    "artifact_distribution_signing_notarization_receipt_completed_status_accepted",
    "artifact_distribution_signing_notarization_receipt_hash_status_rebind_accepted",
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
    "launchd_mutated",
    "service_restarted",
    "active_binary_mutated",
    "external_send_performed"
  ])
  and ($report.artifact_distribution_signing_notarization_receipt_replay_idempotency_surfaces | length) == 18
  and ($report.artifact_distribution_signing_notarization_receipt_replay_idempotency_surfaces | all(
    .artifact_distribution_signing_notarization_receipt_replay_idempotency_attempted == true
    and .artifact_distribution_signing_notarization_receipt_replay_idempotency_allowed == false
    and .artifact_distribution_signing_notarization_receipt_replay_allowed == false
    and .artifact_distribution_signing_notarization_receipt_replay_accepted == false
    and .artifact_distribution_signing_notarization_receipt_replay_recorded == false
    and .artifact_distribution_signing_notarization_receipt_replay_persisted == false
    and .artifact_distribution_signing_notarization_receipt_duplicate_accepted == false
    and .artifact_distribution_signing_notarization_receipt_idempotency_key_accepted == false
    and .artifact_distribution_signing_notarization_receipt_idempotency_state_persisted == false
    and .artifact_distribution_signing_notarization_receipt_replay_nonce_accepted == false
    and .artifact_distribution_signing_notarization_receipt_cross_scope_reuse_accepted == false
    and .artifact_distribution_signing_notarization_receipt_status_upgrade_accepted == false
    and .artifact_distribution_signing_notarization_receipt_hash_status_rebind_accepted == false
    and .artifact_distribution_signing_notarization_receipt_replay_idempotency_noop_confirmed == true
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
    and .download_link_rendered == false
    and .install_command_rendered == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .live_kg_write_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .secret_file_read == false
    and .external_send_performed == false
  ))
  and ([.artifact_distribution_signing_notarization_receipt_replay_idempotency_surfaces[] | select(.signature_manifest_receipt_idempotency_key_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_replay_idempotency_surfaces[] | select(.notarization_ticket_stale_nonce_replay_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_replay_idempotency_surfaces[] | select(.stapling_receipt_cross_scope_reuse_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_replay_idempotency_surfaces[] | select(.telegram_signing_receipt_delivery_replay_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_replay_idempotency_surfaces[] | select(.install_restart_active_binary_replay_path_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .replays_signing_receipt == false
    and .records_duplicate_receipt == false
    and .records_idempotency_key == false
    and .persists_idempotency_state == false
    and .accepts_cross_scope_reuse == false
    and .accepts_status_upgrade == false
    and .records_completion_ack == false
    and .rebinds_hash_status == false
    and .records_operator_acceptance == false
    and .derives_release_publication_authority == false
    and .derives_activation_authority == false
    and .renders_download_link == false
    and .emits_install_command == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .mutates_memory_store == false
    and .writes_kg == false
    and .invokes_provider == false
    and .reads_credentials == false
    and .sends_externally == false
  ))
  and ($report.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt replay/idempotency denial gate passed"
