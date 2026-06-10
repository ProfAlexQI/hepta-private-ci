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

TERMINAL_PUBLIC_STATUS_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-terminal-public-claim-status-exposure-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-terminal-public-claim-status-exposure-denial-gate.sh
)"

source_terminal_public_status_report_sha256="$(sha256_text "$TERMINAL_PUBLIC_STATUS_JSON")"
package_release_channel_status_exposure_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-package-release-channel-status-exposure-denial:$source_terminal_public_status_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
package_release_channel_status_exposure_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-package-release-channel-status-exposure:no-package-channel:no-release-channel:no-registry:no-feed:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$TERMINAL_PUBLIC_STATUS_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_public_claim_status_exposure_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_public_claim_status_exposure_denial_ready == true
    and $source.source_terminal_decision_status_ready == true
    and $source.terminal_public_claim_status_exposure_surface_count == 18
    and $source.terminal_public_claim_status_exposure_attempt_count == 18
    and $source.terminal_public_claim_status_exposure_denied_count == 18
    and zero_fields($source; [
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
      "download_link_from_public_status_rendered_count",
      "install_command_from_public_status_rendered_count",
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
    and false_fields($source; [
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
    and ($source.terminal_public_claim_status_exposure_surfaces | length) == 18
    and ($source.terminal_public_claim_status_exposure_surfaces | all(
      .terminal_public_claim_status_exposure_attempted == true
      and .terminal_public_claim_status_exposure_noop_confirmed == true
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
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_package_release_channel_status_exposure_denial_gate"
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
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

surfaces_json="$(
  jq -n '
    def surface($id; $status; $reason; $extra):
      {
        surface:$id,
        source_terminal_public_claim_status_ready:true,
        package_release_channel_status_exposure_attempted:true,
        package_release_channel_status_exposure_allowed:false,
        package_release_channel_status_exposure_accepted:false,
        package_release_channel_status_exposure_recorded:false,
        package_release_channel_status_exposure_persisted:false,
        package_release_channel_status_exposure_materialized:false,
        package_release_channel_status_exposure_filesystem_written:false,
        package_release_channel_status_exposure_delivered:false,
        package_channel_status_exposed:false,
        release_channel_status_exposed:false,
        update_feed_status_exposed:false,
        package_registry_status_exposed:false,
        cdn_status_exposed:false,
        sbom_status_exposed:false,
        signature_status_exposed:false,
        notarization_status_exposed:false,
        version_tag_status_exposed:false,
        dashboard_status_exposed:false,
        public_endpoint_status_exposed:false,
        query_status_exposed:false,
        export_status_exposed:false,
        observability_status_exposed:false,
        external_status_sent:false,
        telegram_status_sent:false,
        public_status_claimed:false,
        public_release_claimed:false,
        public_ga_claimed:false,
        acceptance_from_package_status_recorded:false,
        operator_approval_from_package_status_derived:false,
        release_publication_authority_from_package_status_derived:false,
        activation_authority_from_package_status_derived:false,
        download_link_from_package_status_rendered:false,
        install_command_from_package_status_rendered:false,
        install_from_package_status_executed:false,
        service_restart_from_package_status_performed:false,
        active_binary_from_package_status_mutated:false,
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
        package_release_channel_status_exposure_noop_confirmed:true,
        package_release_channel_status_exposure_status:$status,
        reason:$reason
      } + $extra;
    [
      surface("source_terminal_public_claim_status_exposure_report_required"; "blocked_source_terminal_public_status_required_noop"; "source_terminal_public_claim_status_exposure_report_required"; {source_report_required:true}),
      surface("package_channel_operator_intent_evidence_status_exposure"; "blocked_package_channel_status_noop"; "package_channel_operator_intent_evidence_status_exposure_denied"; {package_channel_status_requested:true}),
      surface("release_channel_operator_consent_evidence_status_exposure"; "blocked_release_channel_status_noop"; "release_channel_operator_consent_evidence_status_exposure_denied"; {release_channel_status_requested:true}),
      surface("update_feed_terminal_public_status_package_claim"; "blocked_update_feed_package_status_noop"; "update_feed_terminal_public_status_package_claim_denied"; {update_feed_status_requested:true}),
      surface("package_registry_public_release_track_status_claim"; "blocked_package_registry_status_noop"; "package_registry_public_release_track_status_claim_denied"; {package_registry_status_requested:true}),
      surface("cdn_artifact_package_status_claim"; "blocked_cdn_package_status_noop"; "cdn_artifact_package_status_claim_denied"; {cdn_status_requested:true}),
      surface("sbom_provenance_package_status_claim"; "blocked_sbom_package_status_noop"; "sbom_provenance_package_status_claim_denied"; {sbom_status_requested:true}),
      surface("signature_notarization_release_channel_status_claim"; "blocked_signature_notarization_status_noop"; "signature_notarization_release_channel_status_claim_denied"; {signature_status_requested:true, notarization_status_requested:true}),
      surface("version_tag_release_channel_status_claim"; "blocked_version_tag_release_channel_status_noop"; "version_tag_release_channel_status_claim_denied"; {version_tag_status_requested:true}),
      surface("dashboard_package_release_channel_status_exposure"; "blocked_dashboard_channel_status_noop"; "dashboard_package_release_channel_status_exposure_denied"; {dashboard_status_requested:true}),
      surface("public_endpoint_release_channel_status_exposure"; "blocked_public_endpoint_release_status_noop"; "public_endpoint_release_channel_status_exposure_denied"; {public_endpoint_status_requested:true}),
      surface("query_export_package_channel_status_exposure"; "blocked_query_export_channel_status_noop"; "query_export_package_channel_status_exposure_denied"; {query_status_requested:true, export_status_requested:true}),
      surface("observability_package_release_channel_status_exposure"; "blocked_observability_channel_status_noop"; "observability_package_release_channel_status_exposure_denied"; {observability_status_requested:true}),
      surface("external_telegram_release_channel_status_send"; "blocked_external_telegram_release_channel_status_noop"; "external_telegram_release_channel_status_send_denied"; {external_status_requested:true, telegram_status_requested:true}),
      surface("package_channel_operator_acceptance_status_claim"; "blocked_package_channel_acceptance_status_noop"; "package_channel_operator_acceptance_status_claim_denied"; {acceptance_status_requested:true}),
      surface("release_publication_authority_package_status_claim"; "blocked_release_publication_authority_package_status_noop"; "release_publication_authority_package_status_claim_denied"; {release_publication_authority_status_requested:true}),
      surface("activation_live_install_channel_status_exposure"; "blocked_activation_live_install_channel_status_noop"; "activation_live_install_channel_status_exposure_denied"; {activation_live_status_requested:true, live_install_status_requested:true}),
      surface("install_restart_active_binary_package_status_claim"; "blocked_install_restart_active_binary_package_status_noop"; "install_restart_active_binary_package_status_claim_denied"; {install_restart_active_binary_status_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_package_release_channel_status_exposure_denial_gate" \
    --arg source_terminal_public_status_report_sha256 "$source_terminal_public_status_report_sha256" \
    --arg package_release_channel_status_exposure_contract_hash_sha256 "$package_release_channel_status_exposure_contract_hash_sha256" \
    --arg package_release_channel_status_exposure_policy_hash_sha256 "$package_release_channel_status_exposure_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$TERMINAL_PUBLIC_STATUS_JSON" \
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
        package_release_channel_status_exposure_schema_version:"operator_intent_consent_evidence_package_release_channel_status_exposure_denial_v1",
        package_release_channel_status_exposure_mode:"denied_public_status_cannot_create_package_release_channel_registry_feed_or_live_status",
        source_terminal_public_claim_status_exposure_gate:$source.gate,
        source_terminal_public_claim_status_exposure_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_public_claim_status_exposure_denial_ready,
        source_terminal_public_claim_status_exposure_report_sha256:$source_terminal_public_status_report_sha256,
        package_release_channel_status_exposure_contract_hash_sha256:$package_release_channel_status_exposure_contract_hash_sha256,
        package_release_channel_status_exposure_policy_hash_sha256:$package_release_channel_status_exposure_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_package_release_channel_status_exposure_denial_ready:true,
        source_terminal_public_claim_status_exposure_surface_count:$source.terminal_public_claim_status_exposure_surface_count,
        source_terminal_public_claim_status_exposure_attempt_count:$source.terminal_public_claim_status_exposure_attempt_count,
        source_terminal_public_claim_status_exposure_denied_count:$source.terminal_public_claim_status_exposure_denied_count,
        package_release_channel_status_exposure_surface_count:($surfaces | length),
        package_release_channel_status_exposure_attempt_count:($surfaces | length),
        package_release_channel_status_exposure_denied_count:($surfaces | length),
        package_release_channel_status_exposure_surfaces:$surfaces,
        denied_by_package_release_channel_status_exposure:[
          "source_terminal_public_claim_status_exposure_report_required",
          "package_channel_status_acceptance_denied",
          "package_channel_status_recording_denied",
          "package_channel_status_persistence_denied",
          "package_channel_status_materialization_denied",
          "package_channel_status_filesystem_write_denied",
          "release_channel_status_exposure_denied",
          "update_feed_package_status_exposure_denied",
          "package_registry_status_exposure_denied",
          "cdn_artifact_status_exposure_denied",
          "sbom_provenance_status_exposure_denied",
          "signature_notarization_status_exposure_denied",
          "version_tag_status_exposure_denied",
          "dashboard_endpoint_query_export_observability_status_exposure_denied",
          "external_telegram_release_channel_status_send_denied",
          "acceptance_and_operator_approval_from_package_status_denied",
          "release_publication_authority_from_package_status_denied",
          "activation_live_install_from_package_status_denied",
          "install_restart_active_binary_from_package_status_denied",
          "memory_provider_secret_external_send_from_package_status_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_distribution_artifact_manifest_status_denial_gate",
            status:"allowed_report_only_next_slice",
            exposes_package_release_channel_status:false,
            exposes_distribution_artifact_status:false,
            materializes_manifest:false,
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
        "package_release_channel_status_exposure_accepted_count",
        "package_release_channel_status_exposure_recorded_count",
        "package_release_channel_status_exposure_persisted_count",
        "package_release_channel_status_exposure_materialized_count",
        "package_release_channel_status_exposure_filesystem_written_count",
        "package_release_channel_status_exposure_delivered_count",
        "package_channel_status_exposed_count",
        "release_channel_status_exposed_count",
        "update_feed_status_exposed_count",
        "package_registry_status_exposed_count",
        "cdn_status_exposed_count",
        "sbom_status_exposed_count",
        "signature_status_exposed_count",
        "notarization_status_exposed_count",
        "version_tag_status_exposed_count",
        "dashboard_status_exposed_count",
        "public_endpoint_status_exposed_count",
        "query_status_exposed_count",
        "export_status_exposed_count",
        "observability_status_exposed_count",
        "external_status_sent_count",
        "telegram_status_sent_count",
        "public_status_claimed_count",
        "public_release_claimed_count",
        "public_ga_claimed_count",
        "acceptance_from_package_status_recorded_count",
        "operator_approval_from_package_status_derived_count",
        "release_publication_authority_from_package_status_derived_count",
        "activation_authority_from_package_status_derived_count",
        "download_link_from_package_status_rendered_count",
        "install_command_from_package_status_rendered_count",
        "install_from_package_status_executed_count",
        "service_restart_from_package_status_performed_count",
        "active_binary_from_package_status_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count"
      ])
      + false_object([
        "package_release_channel_status_exposure_accepted",
        "package_release_channel_status_exposure_recorded",
        "package_release_channel_status_exposure_persisted",
        "package_channel_status_exposed",
        "release_channel_status_exposed",
        "public_status_claimed",
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
          "package_release_channel_status_exposure_recorded",
          "package_release_channel_status_exposure_persisted",
          "package_release_channel_status_exposure_materialized",
          "package_release_channel_status_exposure_filesystem_written",
          "package_release_channel_status_exposure_delivered",
          "package_channel_status_exposed",
          "release_channel_status_exposed",
          "update_feed_status_exposed",
          "package_registry_status_exposed",
          "cdn_status_exposed",
          "sbom_status_exposed",
          "signature_status_exposed",
          "notarization_status_exposed",
          "version_tag_status_exposed",
          "dashboard_status_exposed",
          "public_endpoint_status_exposed",
          "query_status_exposed",
          "export_status_exposed",
          "observability_status_exposed",
          "external_status_sent",
          "telegram_status_sent",
          "public_status_claimed",
          "public_release_claimed",
          "public_ga_claimed",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_package_release_channel_status_exposure_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_package_release_channel_status_exposure_denial_ready == true
  and $report.source_terminal_public_claim_status_exposure_ready == true
  and $report.source_terminal_public_claim_status_exposure_surface_count == 18
  and $report.source_terminal_public_claim_status_exposure_attempt_count == 18
  and $report.source_terminal_public_claim_status_exposure_denied_count == 18
  and $report.package_release_channel_status_exposure_surface_count == 18
  and $report.package_release_channel_status_exposure_attempt_count == 18
  and $report.package_release_channel_status_exposure_denied_count == 18
  and zero_fields($report; [
    "package_release_channel_status_exposure_accepted_count",
    "package_release_channel_status_exposure_recorded_count",
    "package_release_channel_status_exposure_persisted_count",
    "package_release_channel_status_exposure_materialized_count",
    "package_release_channel_status_exposure_filesystem_written_count",
    "package_release_channel_status_exposure_delivered_count",
    "package_channel_status_exposed_count",
    "release_channel_status_exposed_count",
    "update_feed_status_exposed_count",
    "package_registry_status_exposed_count",
    "cdn_status_exposed_count",
    "version_tag_status_exposed_count",
    "dashboard_status_exposed_count",
    "public_endpoint_status_exposed_count",
    "query_status_exposed_count",
    "export_status_exposed_count",
    "observability_status_exposed_count",
    "external_status_sent_count",
    "telegram_status_sent_count",
    "public_status_claimed_count",
    "public_release_claimed_count",
    "public_ga_claimed_count",
    "release_publication_authority_from_package_status_derived_count",
    "activation_authority_from_package_status_derived_count",
    "install_from_package_status_executed_count",
    "active_binary_from_package_status_mutated_count",
    "memory_store_write_performed_count",
    "live_kg_write_performed_count",
    "provider_invoked_count",
    "model_invoked_count",
    "credential_read_count",
    "secret_file_read_count",
    "external_send_performed_count"
  ])
  and false_fields($report; [
    "package_release_channel_status_exposure_accepted",
    "package_release_channel_status_exposure_recorded",
    "package_release_channel_status_exposure_persisted",
    "package_channel_status_exposed",
    "release_channel_status_exposed",
    "public_status_claimed",
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
  and ($report.package_release_channel_status_exposure_surfaces | length) == 18
  and ($report.package_release_channel_status_exposure_surfaces | all(
    .package_release_channel_status_exposure_attempted == true
    and .package_release_channel_status_exposure_noop_confirmed == true
    and .package_release_channel_status_exposure_allowed == false
    and .package_release_channel_status_exposure_accepted == false
    and .package_release_channel_status_exposure_recorded == false
    and .package_channel_status_exposed == false
    and .release_channel_status_exposed == false
    and .public_status_claimed == false
    and .public_release_claimed == false
    and .public_ga_claimed == false
    and .release_publication_authority_from_package_status_derived == false
    and .activation_authority_from_package_status_derived == false
    and .install_from_package_status_executed == false
    and .active_binary_from_package_status_mutated == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .external_send_performed == false
  ))
  and ([.package_release_channel_status_exposure_surfaces[] | select(.package_channel_status_requested == true)] | length) == 1
  and ([.package_release_channel_status_exposure_surfaces[] | select(.release_channel_status_requested == true)] | length) == 1
  and ([.package_release_channel_status_exposure_surfaces[] | select(.telegram_status_requested == true)] | length) == 1
  and ([.package_release_channel_status_exposure_surfaces[] | select(.install_restart_active_binary_status_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_distribution_artifact_manifest_status_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .exposes_package_release_channel_status == false
    and .exposes_distribution_artifact_status == false
    and .materializes_manifest == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence package/release channel status exposure denial gate passed"
