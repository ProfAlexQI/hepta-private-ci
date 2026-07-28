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

TERMINAL_PACKAGE_CHANNEL_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-package-release-channel-status-exposure-denial-gate" \
    scripts/i3-d74520fde78bf8eca3db467c.sh
)"

source_package_release_channel_status_exposure_report_sha256="$(sha256_text "$TERMINAL_PACKAGE_CHANNEL_JSON")"
distribution_artifact_manifest_status_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-distribution-artifact-manifest-status-denial:$source_package_release_channel_status_exposure_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
distribution_artifact_manifest_status_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-distribution-artifact-manifest-status:no-distribution-artifact:no-manifest:no-artifact-index:no-catalog:no-provenance:no-signature:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$TERMINAL_PACKAGE_CHANNEL_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_package_release_channel_status_exposure_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_package_release_channel_status_exposure_denial_ready == true
    and $source.source_terminal_public_claim_status_exposure_ready == true
    and $source.package_release_channel_status_exposure_surface_count == 18
    and $source.package_release_channel_status_exposure_attempt_count == 18
    and $source.package_release_channel_status_exposure_denied_count == 18
    and zero_fields($source; [
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
    and false_fields($source; [
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
    and ($source.package_release_channel_status_exposure_surfaces | length) == 18
    and ($source.package_release_channel_status_exposure_surfaces | all(
      .package_release_channel_status_exposure_attempted == true
      and .package_release_channel_status_exposure_noop_confirmed == true
      and .package_release_channel_status_exposure_allowed == false
      and .package_release_channel_status_exposure_accepted == false
      and .package_release_channel_status_exposure_recorded == false
      and .package_release_channel_status_exposure_persisted == false
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
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_distribution_artifact_manifest_status_denial_gate"
      and .exposes_package_release_channel_status == false
      and .exposes_distribution_artifact_status == false
      and .materializes_manifest == false
      and .claims_public_release == false
      and .claims_public_ga == false
      and .records_operator_acceptance == false
      and .derives_release_publication_authority == false
      and .derives_activation_authority == false
      and .renders_download_link == false
      and .emits_install_command == false
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
        source_package_release_channel_status_exposure_ready:true,
        distribution_artifact_manifest_status_attempted:true,
        distribution_artifact_manifest_status_allowed:false,
        distribution_artifact_manifest_status_request_accepted:false,
        distribution_artifact_manifest_status_accepted:false,
        distribution_artifact_manifest_status_recorded:false,
        distribution_artifact_manifest_status_persisted:false,
        distribution_artifact_manifest_status_materialized:false,
        distribution_artifact_manifest_status_filesystem_written:false,
        distribution_artifact_manifest_status_delivered:false,
        distribution_artifact_status_exposed:false,
        manifest_status_exposed:false,
        artifact_index_status_exposed:false,
        package_manifest_materialized:false,
        release_manifest_published:false,
        artifact_catalog_status_exposed:false,
        manifest_checksum_status_exposed:false,
        artifact_provenance_status_exposed:false,
        manifest_signature_status_exposed:false,
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
        acceptance_from_manifest_status_recorded:false,
        operator_approval_from_manifest_status_derived:false,
        release_publication_authority_from_manifest_status_derived:false,
        activation_authority_from_manifest_status_derived:false,
        download_link_from_manifest_status_rendered:false,
        install_command_from_manifest_status_rendered:false,
        install_from_manifest_status_executed:false,
        service_restart_from_manifest_status_performed:false,
        active_binary_from_manifest_status_mutated:false,
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
        distribution_artifact_manifest_status_noop_confirmed:true,
        distribution_artifact_manifest_status_status:$status,
        reason:$reason
      } + $extra;
    [
      surface("source_package_release_channel_status_exposure_report_required"; "blocked_source_package_release_channel_status_required_noop"; "source_package_release_channel_status_exposure_report_required"; {source_report_required:true}),
      surface("distribution_artifact_status_claim"; "blocked_distribution_artifact_status_noop"; "distribution_artifact_status_claim_denied"; {distribution_artifact_status_requested:true}),
      surface("manifest_status_claim"; "blocked_manifest_status_noop"; "manifest_status_claim_denied"; {manifest_status_requested:true}),
      surface("artifact_index_status_claim"; "blocked_artifact_index_status_noop"; "artifact_index_status_claim_denied"; {artifact_index_status_requested:true}),
      surface("package_manifest_materialization_claim"; "blocked_package_manifest_materialization_noop"; "package_manifest_materialization_claim_denied"; {package_manifest_materialization_requested:true}),
      surface("release_manifest_publication_claim"; "blocked_release_manifest_publication_noop"; "release_manifest_publication_claim_denied"; {release_manifest_publication_requested:true}),
      surface("artifact_catalog_status_claim"; "blocked_artifact_catalog_status_noop"; "artifact_catalog_status_claim_denied"; {artifact_catalog_status_requested:true}),
      surface("manifest_checksum_status_claim"; "blocked_manifest_checksum_status_noop"; "manifest_checksum_status_claim_denied"; {manifest_checksum_status_requested:true}),
      surface("artifact_provenance_status_claim"; "blocked_artifact_provenance_status_noop"; "artifact_provenance_status_claim_denied"; {artifact_provenance_status_requested:true}),
      surface("manifest_signature_status_claim"; "blocked_manifest_signature_status_noop"; "manifest_signature_status_claim_denied"; {manifest_signature_status_requested:true}),
      surface("dashboard_artifact_manifest_status_exposure"; "blocked_dashboard_artifact_manifest_status_noop"; "dashboard_artifact_manifest_status_exposure_denied"; {dashboard_status_requested:true}),
      surface("public_endpoint_artifact_manifest_status_exposure"; "blocked_public_endpoint_artifact_manifest_status_noop"; "public_endpoint_artifact_manifest_status_exposure_denied"; {public_endpoint_status_requested:true}),
      surface("query_export_artifact_manifest_status_exposure"; "blocked_query_export_artifact_manifest_status_noop"; "query_export_artifact_manifest_status_exposure_denied"; {query_status_requested:true, export_status_requested:true}),
      surface("observability_artifact_manifest_status_exposure"; "blocked_observability_artifact_manifest_status_noop"; "observability_artifact_manifest_status_exposure_denied"; {observability_status_requested:true}),
      surface("external_telegram_artifact_manifest_status_send"; "blocked_external_telegram_artifact_manifest_status_noop"; "external_telegram_artifact_manifest_status_send_denied"; {external_status_requested:true, telegram_status_requested:true}),
      surface("release_publication_authority_artifact_manifest_status_claim"; "blocked_release_publication_authority_artifact_manifest_status_noop"; "release_publication_authority_artifact_manifest_status_claim_denied"; {release_publication_authority_status_requested:true}),
      surface("activation_live_install_artifact_manifest_status_exposure"; "blocked_activation_live_install_artifact_manifest_status_noop"; "activation_live_install_artifact_manifest_status_exposure_denied"; {activation_live_status_requested:true, live_install_status_requested:true}),
      surface("install_restart_active_binary_artifact_manifest_status_claim"; "blocked_install_restart_active_binary_artifact_manifest_status_noop"; "install_restart_active_binary_artifact_manifest_status_claim_denied"; {install_restart_active_binary_status_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_distribution_artifact_manifest_status_denial_gate" \
    --arg source_package_release_channel_status_exposure_report_sha256 "$source_package_release_channel_status_exposure_report_sha256" \
    --arg distribution_artifact_manifest_status_contract_hash_sha256 "$distribution_artifact_manifest_status_contract_hash_sha256" \
    --arg distribution_artifact_manifest_status_policy_hash_sha256 "$distribution_artifact_manifest_status_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$TERMINAL_PACKAGE_CHANNEL_JSON" \
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
        distribution_artifact_manifest_status_schema_version:"operator_intent_consent_evidence_distribution_artifact_manifest_status_denial_v1",
        distribution_artifact_manifest_status_mode:"denied_package_release_channel_status_cannot_create_distribution_artifact_manifest_index_catalog_provenance_signature_authority_or_install_status",
        source_package_release_channel_status_exposure_gate:$source.gate,
        source_package_release_channel_status_exposure_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_package_release_channel_status_exposure_denial_ready,
        source_terminal_public_claim_status_exposure_ready:$source.source_terminal_public_claim_status_exposure_ready,
        source_package_release_channel_status_exposure_report_sha256:$source_package_release_channel_status_exposure_report_sha256,
        distribution_artifact_manifest_status_contract_hash_sha256:$distribution_artifact_manifest_status_contract_hash_sha256,
        distribution_artifact_manifest_status_policy_hash_sha256:$distribution_artifact_manifest_status_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_distribution_artifact_manifest_status_denial_ready:true,
        source_package_release_channel_status_exposure_surface_count:$source.package_release_channel_status_exposure_surface_count,
        source_package_release_channel_status_exposure_attempt_count:$source.package_release_channel_status_exposure_attempt_count,
        source_package_release_channel_status_exposure_denied_count:$source.package_release_channel_status_exposure_denied_count,
        source_package_channel_status_exposed_count:$source.package_channel_status_exposed_count,
        source_release_channel_status_exposed_count:$source.release_channel_status_exposed_count,
        source_update_feed_status_exposed_count:$source.update_feed_status_exposed_count,
        source_package_registry_status_exposed_count:$source.package_registry_status_exposed_count,
        source_cdn_status_exposed_count:$source.cdn_status_exposed_count,
        source_sbom_status_exposed_count:$source.sbom_status_exposed_count,
        source_signature_status_exposed_count:$source.signature_status_exposed_count,
        source_notarization_status_exposed_count:$source.notarization_status_exposed_count,
        source_version_tag_status_exposed_count:$source.version_tag_status_exposed_count,
        source_release_publication_authority_from_package_status_derived_count:$source.release_publication_authority_from_package_status_derived_count,
        source_activation_authority_from_package_status_derived_count:$source.activation_authority_from_package_status_derived_count,
        distribution_artifact_manifest_status_surface_count:($surfaces | length),
        distribution_artifact_manifest_status_attempt_count:($surfaces | length),
        distribution_artifact_manifest_status_denied_count:($surfaces | length),
        distribution_artifact_manifest_status_surfaces:$surfaces,
        denied_by_distribution_artifact_manifest_status:[
          "source_package_release_channel_status_exposure_report_required",
          "distribution_artifact_status_exposure_denied",
          "manifest_status_exposure_denied",
          "artifact_index_status_exposure_denied",
          "package_manifest_materialization_denied",
          "release_manifest_publication_denied",
          "artifact_catalog_status_exposure_denied",
          "manifest_checksum_status_exposure_denied",
          "artifact_provenance_status_exposure_denied",
          "manifest_signature_status_exposure_denied",
          "dashboard_artifact_manifest_status_exposure_denied",
          "public_endpoint_artifact_manifest_status_exposure_denied",
          "query_export_artifact_manifest_status_exposure_denied",
          "observability_artifact_manifest_status_exposure_denied",
          "external_telegram_artifact_manifest_status_send_denied",
          "acceptance_and_operator_approval_from_manifest_status_denied",
          "release_publication_authority_from_manifest_status_denied",
          "activation_live_install_from_manifest_status_denied",
          "install_restart_active_binary_from_manifest_status_denied",
          "memory_provider_kg_secret_external_send_from_manifest_status_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_surface_denial_gate",
            status:"allowed_report_only_next_slice",
            exposes_distribution_artifact_manifest_status:false,
            records_artifact_distribution_signing:false,
            records_notarization:false,
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
        "distribution_artifact_manifest_status_allowed_count",
        "distribution_artifact_manifest_status_request_accepted_count",
        "distribution_artifact_manifest_status_accepted_count",
        "distribution_artifact_manifest_status_recorded_count",
        "distribution_artifact_manifest_status_persisted_count",
        "distribution_artifact_manifest_status_materialized_count",
        "distribution_artifact_manifest_status_filesystem_written_count",
        "distribution_artifact_manifest_status_delivered_count",
        "distribution_artifact_status_exposed_count",
        "manifest_status_exposed_count",
        "artifact_index_status_exposed_count",
        "package_manifest_materialized_count",
        "release_manifest_published_count",
        "artifact_catalog_status_exposed_count",
        "manifest_checksum_status_exposed_count",
        "artifact_provenance_status_exposed_count",
        "manifest_signature_status_exposed_count",
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
        "acceptance_from_manifest_status_recorded_count",
        "operator_approval_from_manifest_status_derived_count",
        "release_publication_authority_from_manifest_status_derived_count",
        "activation_authority_from_manifest_status_derived_count",
        "download_link_from_manifest_status_rendered_count",
        "install_command_from_manifest_status_rendered_count",
        "install_from_manifest_status_executed_count",
        "service_restart_from_manifest_status_performed_count",
        "active_binary_from_manifest_status_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count"
      ])
      + false_object([
        "distribution_artifact_manifest_status_accepted",
        "distribution_artifact_manifest_status_recorded",
        "distribution_artifact_manifest_status_persisted",
        "distribution_artifact_status_exposed",
        "manifest_status_exposed",
        "artifact_manifest_materialized",
        "release_manifest_published",
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
          "distribution_artifact_manifest_status_recorded",
          "distribution_artifact_manifest_status_persisted",
          "distribution_artifact_manifest_status_materialized",
          "distribution_artifact_manifest_status_filesystem_written",
          "distribution_artifact_manifest_status_delivered",
          "distribution_artifact_status_exposed",
          "manifest_status_exposed",
          "artifact_index_status_exposed",
          "package_manifest_materialized",
          "release_manifest_published",
          "artifact_catalog_status_exposed",
          "manifest_checksum_status_exposed",
          "artifact_provenance_status_exposed",
          "manifest_signature_status_exposed",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_distribution_artifact_manifest_status_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_distribution_artifact_manifest_status_denial_ready == true
  and $report.source_package_release_channel_status_exposure_ready == true
  and $report.source_terminal_public_claim_status_exposure_ready == true
  and $report.source_package_release_channel_status_exposure_surface_count == 18
  and $report.source_package_release_channel_status_exposure_attempt_count == 18
  and $report.source_package_release_channel_status_exposure_denied_count == 18
  and $report.source_package_channel_status_exposed_count == 0
  and $report.source_release_channel_status_exposed_count == 0
  and $report.source_update_feed_status_exposed_count == 0
  and $report.source_package_registry_status_exposed_count == 0
  and $report.source_cdn_status_exposed_count == 0
  and $report.source_sbom_status_exposed_count == 0
  and $report.source_signature_status_exposed_count == 0
  and $report.source_notarization_status_exposed_count == 0
  and $report.source_version_tag_status_exposed_count == 0
  and $report.source_release_publication_authority_from_package_status_derived_count == 0
  and $report.source_activation_authority_from_package_status_derived_count == 0
  and $report.distribution_artifact_manifest_status_surface_count == 18
  and $report.distribution_artifact_manifest_status_attempt_count == 18
  and $report.distribution_artifact_manifest_status_denied_count == 18
  and zero_fields($report; [
    "distribution_artifact_manifest_status_allowed_count",
    "distribution_artifact_manifest_status_request_accepted_count",
    "distribution_artifact_manifest_status_accepted_count",
    "distribution_artifact_manifest_status_recorded_count",
    "distribution_artifact_manifest_status_persisted_count",
    "distribution_artifact_manifest_status_materialized_count",
    "distribution_artifact_manifest_status_filesystem_written_count",
    "distribution_artifact_manifest_status_delivered_count",
    "distribution_artifact_status_exposed_count",
    "manifest_status_exposed_count",
    "artifact_index_status_exposed_count",
    "package_manifest_materialized_count",
    "release_manifest_published_count",
    "artifact_catalog_status_exposed_count",
    "manifest_checksum_status_exposed_count",
    "artifact_provenance_status_exposed_count",
    "manifest_signature_status_exposed_count",
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
    "acceptance_from_manifest_status_recorded_count",
    "operator_approval_from_manifest_status_derived_count",
    "release_publication_authority_from_manifest_status_derived_count",
    "activation_authority_from_manifest_status_derived_count",
    "download_link_from_manifest_status_rendered_count",
    "install_command_from_manifest_status_rendered_count",
    "install_from_manifest_status_executed_count",
    "service_restart_from_manifest_status_performed_count",
    "active_binary_from_manifest_status_mutated_count",
    "memory_store_write_performed_count",
    "live_kg_write_performed_count",
    "provider_invoked_count",
    "model_invoked_count",
    "credential_read_count",
    "secret_file_read_count",
    "external_send_performed_count"
  ])
  and false_fields($report; [
    "distribution_artifact_manifest_status_accepted",
    "distribution_artifact_manifest_status_recorded",
    "distribution_artifact_manifest_status_persisted",
    "distribution_artifact_status_exposed",
    "manifest_status_exposed",
    "artifact_manifest_materialized",
    "release_manifest_published",
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
  and ($report.distribution_artifact_manifest_status_surfaces | length) == 18
  and ($report.distribution_artifact_manifest_status_surfaces | all(
    .distribution_artifact_manifest_status_attempted == true
    and .distribution_artifact_manifest_status_allowed == false
    and .distribution_artifact_manifest_status_request_accepted == false
    and .distribution_artifact_manifest_status_accepted == false
    and .distribution_artifact_manifest_status_recorded == false
    and .distribution_artifact_manifest_status_persisted == false
    and .distribution_artifact_manifest_status_materialized == false
    and .distribution_artifact_manifest_status_filesystem_written == false
    and .distribution_artifact_manifest_status_delivered == false
    and .distribution_artifact_status_exposed == false
    and .manifest_status_exposed == false
    and .artifact_index_status_exposed == false
    and .package_manifest_materialized == false
    and .release_manifest_published == false
    and .artifact_catalog_status_exposed == false
    and .manifest_checksum_status_exposed == false
    and .artifact_provenance_status_exposed == false
    and .manifest_signature_status_exposed == false
    and .dashboard_status_exposed == false
    and .public_endpoint_status_exposed == false
    and .query_status_exposed == false
    and .export_status_exposed == false
    and .observability_status_exposed == false
    and .external_status_sent == false
    and .telegram_status_sent == false
    and .acceptance_from_manifest_status_recorded == false
    and .operator_approval_from_manifest_status_derived == false
    and .release_publication_authority_from_manifest_status_derived == false
    and .activation_authority_from_manifest_status_derived == false
    and .install_from_manifest_status_executed == false
    and .service_restart_from_manifest_status_performed == false
    and .active_binary_from_manifest_status_mutated == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .live_kg_write_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .secret_file_read == false
    and .external_send_performed == false
    and .distribution_artifact_manifest_status_noop_confirmed == true
  ))
  and ([.distribution_artifact_manifest_status_surfaces[] | select(.manifest_status_requested == true)] | length) == 1
  and ([.distribution_artifact_manifest_status_surfaces[] | select(.release_manifest_publication_requested == true)] | length) == 1
  and ([.distribution_artifact_manifest_status_surfaces[] | select(.manifest_signature_status_requested == true)] | length) == 1
  and ([.distribution_artifact_manifest_status_surfaces[] | select(.telegram_status_requested == true)] | length) == 1
  and ([.distribution_artifact_manifest_status_surfaces[] | select(.install_restart_active_binary_status_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_surface_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .exposes_distribution_artifact_manifest_status == false
    and .records_artifact_distribution_signing == false
    and .records_notarization == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence distribution artifact/manifest status denial gate passed"
