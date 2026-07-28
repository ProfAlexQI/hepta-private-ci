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

ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_CANCELLATION_SUPERSESSION_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-cancel-supersession-denial-gate" \
    scripts/i3-71bd59c6d099c54edc1a3553.sh
)"

source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_report_sha256="$(
  sha256_text "$ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_CANCELLATION_SUPERSESSION_JSON"
)"
artifact_distribution_signing_notarization_receipt_audit_evidence_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-distribution-signing-notarization-receipt-audit-evidence-denial:$source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
artifact_distribution_signing_notarization_receipt_audit_evidence_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-distribution-signing-notarization-receipt-audit-evidence:no-audit:no-immutable-evidence:no-hash-chain:no-attestation:no-ledger:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_CANCELLATION_SUPERSESSION_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_cancellation_supersession_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_cancellation_supersession_denial_ready == true
    and $source.source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_ready == true
    and $source.artifact_distribution_signing_notarization_receipt_cancellation_supersession_surface_count == 18
    and $source.artifact_distribution_signing_notarization_receipt_cancellation_supersession_attempt_count == 18
    and $source.artifact_distribution_signing_notarization_receipt_cancellation_supersession_denied_count == 18
    and zero_fields($source; [
      "artifact_distribution_signing_notarization_receipt_cancellation_supersession_allowed_count",
      "artifact_distribution_signing_notarization_receipt_cancellation_supersession_accepted_count",
      "artifact_distribution_signing_notarization_receipt_cancellation_supersession_recorded_count",
      "artifact_distribution_signing_notarization_receipt_cancellation_supersession_persisted_count",
      "artifact_distribution_signing_notarization_receipt_cancellation_supersession_materialized_count",
      "artifact_distribution_signing_notarization_receipt_cancellation_supersession_filesystem_written_count",
      "artifact_distribution_signing_notarization_receipt_cancellation_accepted_count",
      "artifact_distribution_signing_notarization_receipt_cancellation_recorded_count",
      "artifact_distribution_signing_notarization_receipt_cancellation_persisted_count",
      "artifact_distribution_signing_notarization_receipt_withdrawal_accepted_count",
      "artifact_distribution_signing_notarization_receipt_supersession_accepted_count",
      "artifact_distribution_signing_notarization_receipt_supersession_recorded_count",
      "artifact_distribution_signing_notarization_receipt_supersession_persisted_count",
      "artifact_distribution_signing_notarization_receipt_replacement_receipt_accepted_count",
      "artifact_distribution_signing_notarization_receipt_replacement_receipt_recorded_count",
      "artifact_distribution_signing_notarization_receipt_replacement_receipt_persisted_count",
      "artifact_distribution_signing_notarization_receipt_tombstone_recorded_count",
      "artifact_distribution_signing_notarization_receipt_tombstone_persisted_count",
      "artifact_distribution_signing_notarization_receipt_delete_marker_recorded_count",
      "artifact_distribution_signing_notarization_receipt_delete_marker_persisted_count",
      "artifact_distribution_signing_notarization_receipt_lifecycle_cancellation_supersession_persisted_count",
      "release_publication_authority_from_signing_receipt_cancellation_derived_count",
      "activation_authority_from_signing_receipt_supersession_derived_count",
      "install_from_signing_receipt_cancellation_executed_count",
      "service_restart_from_signing_receipt_supersession_performed_count",
      "active_binary_from_signing_receipt_cancellation_mutated_count",
      "memory_store_write_performed_count",
      "live_kg_write_performed_count",
      "provider_invoked_count",
      "model_invoked_count",
      "credential_read_count",
      "secret_file_read_count",
      "external_send_performed_count"
    ])
    and false_fields($source; [
      "artifact_distribution_signing_notarization_receipt_cancellation_supersession_accepted",
      "artifact_distribution_signing_notarization_receipt_cancellation_supersession_recorded",
      "artifact_distribution_signing_notarization_receipt_cancellation_supersession_persisted",
      "artifact_distribution_signing_notarization_receipt_cancellation_accepted",
      "artifact_distribution_signing_notarization_receipt_cancellation_recorded",
      "artifact_distribution_signing_notarization_receipt_cancellation_persisted",
      "artifact_distribution_signing_notarization_receipt_withdrawal_accepted",
      "artifact_distribution_signing_notarization_receipt_supersession_accepted",
      "artifact_distribution_signing_notarization_receipt_supersession_recorded",
      "artifact_distribution_signing_notarization_receipt_supersession_persisted",
      "artifact_distribution_signing_notarization_receipt_replacement_receipt_accepted",
      "artifact_distribution_signing_notarization_receipt_tombstone_recorded",
      "artifact_distribution_signing_notarization_receipt_delete_marker_recorded",
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
    and ($source.artifact_distribution_signing_notarization_receipt_cancellation_supersession_surfaces | length) == 18
    and ($source.artifact_distribution_signing_notarization_receipt_cancellation_supersession_surfaces | all(
      .artifact_distribution_signing_notarization_receipt_cancellation_supersession_attempted == true
      and .artifact_distribution_signing_notarization_receipt_cancellation_supersession_allowed == false
      and .artifact_distribution_signing_notarization_receipt_cancellation_supersession_accepted == false
      and .artifact_distribution_signing_notarization_receipt_cancellation_accepted == false
      and .artifact_distribution_signing_notarization_receipt_supersession_accepted == false
      and .artifact_distribution_signing_notarization_receipt_replacement_receipt_accepted == false
      and .artifact_distribution_signing_notarization_receipt_tombstone_recorded == false
      and .artifact_distribution_signing_notarization_receipt_delete_marker_recorded == false
      and .artifact_distribution_signing_notarization_receipt_cancellation_supersession_noop_confirmed == true
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
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_audit_evidence_denial_gate"
      and .accepts_cancellation == false
      and .accepts_supersession == false
      and .records_replacement_receipt == false
      and .records_tombstone == false
      and .records_delete_marker == false
      and .persists_lifecycle_state == false
      and .records_audit_evidence == false
      and .records_operator_acceptance == false
      and .derives_release_publication_authority == false
      and .derives_activation_authority == false
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
    def audit_surface($id; $status; $reason; $extra):
      {
        surface:$id,
        source_signing_receipt_cancellation_supersession_denial_ready:true,
        canonical_noop_signing_receipt_identity_required:true,
        artifact_distribution_signing_notarization_receipt_audit_evidence_attempted:true,
        artifact_distribution_signing_notarization_receipt_audit_evidence_allowed:false,
        artifact_distribution_signing_notarization_receipt_audit_evidence_accepted:false,
        artifact_distribution_signing_notarization_receipt_audit_evidence_recorded:false,
        artifact_distribution_signing_notarization_receipt_audit_evidence_persisted:false,
        artifact_distribution_signing_notarization_receipt_audit_evidence_materialized:false,
        artifact_distribution_signing_notarization_receipt_audit_evidence_filesystem_written:false,
        artifact_distribution_signing_notarization_receipt_audit_trail_recorded:false,
        artifact_distribution_signing_notarization_receipt_audit_trail_persisted:false,
        artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded:false,
        artifact_distribution_signing_notarization_receipt_immutable_evidence_persisted:false,
        artifact_distribution_signing_notarization_receipt_hash_chain_recorded:false,
        artifact_distribution_signing_notarization_receipt_merkle_root_recorded:false,
        artifact_distribution_signing_notarization_receipt_attestation_recorded:false,
        artifact_distribution_signing_notarization_receipt_witness_recorded:false,
        artifact_distribution_signing_notarization_receipt_notary_recorded:false,
        artifact_distribution_signing_notarization_receipt_ledger_recorded:false,
        artifact_distribution_signing_notarization_receipt_ledger_persisted:false,
        artifact_distribution_signing_notarization_receipt_index_recorded:false,
        artifact_distribution_signing_notarization_receipt_index_persisted:false,
        artifact_distribution_signing_notarization_receipt_delivery_evidence_recorded:false,
        artifact_distribution_signing_notarization_receipt_delivery_evidence_delivered:false,
        artifact_distribution_signing_notarization_receipt_query_export_evidence_recorded:false,
        artifact_distribution_signing_notarization_receipt_observability_evidence_recorded:false,
        artifact_distribution_signing_notarization_receipt_readback_evidence_recorded:false,
        artifact_distribution_signing_notarization_receipt_status_evidence_recorded:false,
        artifact_distribution_signing_notarization_receipt_hash_status_evidence_recorded:false,
        artifact_signing_cancellation_audit_trail_recorded:false,
        package_signing_supersession_immutable_evidence_recorded:false,
        signature_manifest_withdrawal_hash_chain_recorded:false,
        notarization_submission_cancellation_attestation_recorded:false,
        notarization_ticket_supersession_witness_recorded:false,
        stapling_tombstone_ledger_index_recorded:false,
        installer_replacement_evidence_materialized:false,
        provenance_latest_replacement_immutable_evidence_recorded:false,
        sbom_supersession_evidence_exported:false,
        release_asset_cancelled_query_evidence_recorded:false,
        cdn_superseded_observability_evidence_recorded:false,
        package_registry_replacement_status_evidence_recorded:false,
        dashboard_endpoint_tombstone_hash_status_evidence_recorded:false,
        external_audit_evidence_delivered:false,
        telegram_audit_evidence_delivered:false,
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
        artifact_distribution_signing_notarization_receipt_audit_evidence_noop_confirmed:true,
        artifact_distribution_signing_notarization_receipt_audit_evidence_status:$status,
        reason:$reason
      } + $extra;
    [
      audit_surface("source_signing_receipt_cancellation_supersession_report_required"; "blocked_source_signing_receipt_cancellation_supersession_required_noop"; "source_signing_receipt_cancellation_supersession_report_required"; {source_report_required:true}),
      audit_surface("artifact_signing_cancellation_audit_trail_append"; "blocked_artifact_signing_cancellation_audit_trail_append_noop"; "artifact_signing_cancellation_audit_trail_append_denied"; {artifact_signing_cancellation_audit_trail_append_requested:true}),
      audit_surface("package_signing_supersession_immutable_evidence_packet"; "blocked_package_signing_supersession_immutable_evidence_packet_noop"; "package_signing_supersession_immutable_evidence_packet_denied"; {package_signing_supersession_immutable_evidence_packet_requested:true}),
      audit_surface("signature_manifest_withdrawal_hash_chain"; "blocked_signature_manifest_withdrawal_hash_chain_noop"; "signature_manifest_withdrawal_hash_chain_denied"; {signature_manifest_withdrawal_hash_chain_requested:true}),
      audit_surface("notarization_submission_cancellation_attestation"; "blocked_notarization_submission_cancellation_attestation_noop"; "notarization_submission_cancellation_attestation_denied"; {notarization_submission_cancellation_attestation_requested:true}),
      audit_surface("notarization_ticket_supersession_witness_notary"; "blocked_notarization_ticket_supersession_witness_notary_noop"; "notarization_ticket_supersession_witness_notary_denied"; {notarization_ticket_supersession_witness_notary_requested:true}),
      audit_surface("stapling_tombstone_ledger_index"; "blocked_stapling_tombstone_ledger_index_noop"; "stapling_tombstone_ledger_index_denied"; {stapling_tombstone_ledger_index_requested:true}),
      audit_surface("installer_replacement_evidence_materialization"; "blocked_installer_replacement_evidence_materialization_noop"; "installer_replacement_evidence_materialization_denied"; {installer_replacement_evidence_materialization_requested:true}),
      audit_surface("provenance_latest_replacement_immutable_evidence"; "blocked_provenance_latest_replacement_immutable_evidence_noop"; "provenance_latest_replacement_immutable_evidence_denied"; {provenance_latest_replacement_immutable_evidence_requested:true}),
      audit_surface("sbom_supersession_evidence_export"; "blocked_sbom_supersession_evidence_export_noop"; "sbom_supersession_evidence_export_denied"; {sbom_supersession_evidence_export_requested:true}),
      audit_surface("release_asset_cancelled_query_evidence"; "blocked_release_asset_cancelled_query_evidence_noop"; "release_asset_cancelled_query_evidence_denied"; {release_asset_cancelled_query_evidence_requested:true}),
      audit_surface("cdn_superseded_observability_evidence"; "blocked_cdn_superseded_observability_evidence_noop"; "cdn_superseded_observability_evidence_denied"; {cdn_superseded_observability_evidence_requested:true}),
      audit_surface("package_registry_replacement_status_evidence"; "blocked_package_registry_replacement_status_evidence_noop"; "package_registry_replacement_status_evidence_denied"; {package_registry_replacement_status_evidence_requested:true}),
      audit_surface("dashboard_endpoint_tombstone_hash_status_evidence"; "blocked_dashboard_endpoint_tombstone_hash_status_evidence_noop"; "dashboard_endpoint_tombstone_hash_status_evidence_denied"; {dashboard_endpoint_tombstone_hash_status_evidence_requested:true}),
      audit_surface("external_telegram_audit_evidence_delivery"; "blocked_external_telegram_audit_evidence_delivery_noop"; "external_telegram_audit_evidence_delivery_denied"; {external_audit_evidence_delivery_requested:true, telegram_audit_evidence_delivery_requested:true}),
      audit_surface("release_publication_authority_audit_evidence"; "blocked_release_publication_authority_audit_evidence_noop"; "release_publication_authority_audit_evidence_denied"; {release_publication_authority_audit_evidence_requested:true}),
      audit_surface("activation_live_install_audit_evidence"; "blocked_activation_live_install_audit_evidence_noop"; "activation_live_install_audit_evidence_denied"; {activation_live_install_audit_evidence_requested:true}),
      audit_surface("install_restart_active_binary_audit_path"; "blocked_install_restart_active_binary_audit_path_noop"; "install_restart_active_binary_audit_path_denied"; {install_restart_active_binary_audit_path_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_audit_evidence_denial_gate" \
    --arg source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_report_sha256 "$source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_report_sha256" \
    --arg artifact_distribution_signing_notarization_receipt_audit_evidence_contract_hash_sha256 "$artifact_distribution_signing_notarization_receipt_audit_evidence_contract_hash_sha256" \
    --arg artifact_distribution_signing_notarization_receipt_audit_evidence_policy_hash_sha256 "$artifact_distribution_signing_notarization_receipt_audit_evidence_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_CANCELLATION_SUPERSESSION_JSON" \
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
        artifact_distribution_signing_notarization_receipt_audit_evidence_schema_version:"operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_audit_evidence_denial_v1",
        artifact_distribution_signing_notarization_receipt_audit_evidence_mode:"denied_signing_receipt_cancellation_supersession_cannot_be_wrapped_in_audit_immutable_evidence_hash_chain_attestation_ledger_or_used_for_authority_or_live_install",
        source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_gate:$source.gate,
        source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_cancellation_supersession_denial_ready,
        source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_report_sha256:$source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_report_sha256,
        source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_contract_hash_sha256:$source.artifact_distribution_signing_notarization_receipt_cancellation_supersession_contract_hash_sha256,
        artifact_distribution_signing_notarization_receipt_audit_evidence_contract_hash_sha256:$artifact_distribution_signing_notarization_receipt_audit_evidence_contract_hash_sha256,
        artifact_distribution_signing_notarization_receipt_audit_evidence_policy_hash_sha256:$artifact_distribution_signing_notarization_receipt_audit_evidence_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_audit_evidence_denial_ready:true,
        source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_surface_count:$source.artifact_distribution_signing_notarization_receipt_cancellation_supersession_surface_count,
        source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_attempt_count:$source.artifact_distribution_signing_notarization_receipt_cancellation_supersession_attempt_count,
        source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_denied_count:$source.artifact_distribution_signing_notarization_receipt_cancellation_supersession_denied_count,
        source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_accepted_count:$source.artifact_distribution_signing_notarization_receipt_cancellation_supersession_accepted_count,
        source_artifact_distribution_signing_notarization_receipt_lifecycle_cancellation_supersession_persisted_count:$source.artifact_distribution_signing_notarization_receipt_lifecycle_cancellation_supersession_persisted_count,
        source_release_publication_authority_from_signing_receipt_cancellation_derived_count:$source.release_publication_authority_from_signing_receipt_cancellation_derived_count,
        source_activation_authority_from_signing_receipt_supersession_derived_count:$source.activation_authority_from_signing_receipt_supersession_derived_count,
        artifact_distribution_signing_notarization_receipt_audit_evidence_surface_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_audit_evidence_attempt_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_audit_evidence_denied_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_audit_evidence_surfaces:$surfaces,
        denied_by_artifact_distribution_signing_notarization_receipt_audit_evidence:[
          "source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_report_required",
          "signing_receipt_audit_trail_denied",
          "signing_receipt_immutable_evidence_denied",
          "signing_receipt_hash_chain_merkle_root_denied",
          "signing_receipt_attestation_witness_notary_denied",
          "signing_receipt_ledger_index_denied",
          "signing_receipt_materialized_evidence_denied",
          "signing_receipt_export_query_observability_evidence_denied",
          "signing_receipt_readback_status_hash_evidence_denied",
          "external_telegram_signing_receipt_audit_evidence_delivery_denied",
          "release_publication_authority_from_signing_receipt_audit_evidence_denied",
          "activation_live_install_from_signing_receipt_audit_evidence_denied",
          "install_restart_active_binary_from_signing_receipt_audit_evidence_denied",
          "memory_provider_kg_secret_external_send_from_signing_receipt_audit_evidence_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denial_gate",
            status:"allowed_report_only_next_slice",
            records_audit_evidence:false,
            records_immutable_evidence:false,
            records_hash_chain:false,
            records_attestation:false,
            records_witness:false,
            records_notary:false,
            records_ledger:false,
            persists_audit_evidence:false,
            accepts_retention:false,
            accepts_expiry:false,
            performs_garbage_collection:false,
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
        "artifact_distribution_signing_notarization_receipt_audit_evidence_allowed_count",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_accepted_count",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_recorded_count",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_persisted_count",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_materialized_count",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_filesystem_written_count",
        "artifact_distribution_signing_notarization_receipt_audit_trail_recorded_count",
        "artifact_distribution_signing_notarization_receipt_audit_trail_persisted_count",
        "artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded_count",
        "artifact_distribution_signing_notarization_receipt_immutable_evidence_persisted_count",
        "artifact_distribution_signing_notarization_receipt_hash_chain_recorded_count",
        "artifact_distribution_signing_notarization_receipt_merkle_root_recorded_count",
        "artifact_distribution_signing_notarization_receipt_attestation_recorded_count",
        "artifact_distribution_signing_notarization_receipt_witness_recorded_count",
        "artifact_distribution_signing_notarization_receipt_notary_recorded_count",
        "artifact_distribution_signing_notarization_receipt_ledger_recorded_count",
        "artifact_distribution_signing_notarization_receipt_ledger_persisted_count",
        "artifact_distribution_signing_notarization_receipt_index_recorded_count",
        "artifact_distribution_signing_notarization_receipt_index_persisted_count",
        "artifact_distribution_signing_notarization_receipt_delivery_evidence_recorded_count",
        "artifact_distribution_signing_notarization_receipt_delivery_evidence_delivered_count",
        "artifact_distribution_signing_notarization_receipt_query_export_evidence_recorded_count",
        "artifact_distribution_signing_notarization_receipt_observability_evidence_recorded_count",
        "artifact_distribution_signing_notarization_receipt_readback_evidence_recorded_count",
        "artifact_distribution_signing_notarization_receipt_status_evidence_recorded_count",
        "artifact_distribution_signing_notarization_receipt_hash_status_evidence_recorded_count",
        "artifact_signing_cancellation_audit_trail_recorded_count",
        "package_signing_supersession_immutable_evidence_recorded_count",
        "signature_manifest_withdrawal_hash_chain_recorded_count",
        "notarization_submission_cancellation_attestation_recorded_count",
        "notarization_ticket_supersession_witness_recorded_count",
        "stapling_tombstone_ledger_index_recorded_count",
        "installer_replacement_evidence_materialized_count",
        "provenance_latest_replacement_immutable_evidence_recorded_count",
        "sbom_supersession_evidence_exported_count",
        "release_asset_cancelled_query_evidence_recorded_count",
        "cdn_superseded_observability_evidence_recorded_count",
        "package_registry_replacement_status_evidence_recorded_count",
        "dashboard_endpoint_tombstone_hash_status_evidence_recorded_count",
        "external_audit_evidence_delivered_count",
        "telegram_audit_evidence_delivered_count",
        "acceptance_from_signing_receipt_audit_evidence_recorded_count",
        "operator_approval_from_signing_receipt_audit_evidence_derived_count",
        "release_publication_authority_from_signing_receipt_audit_evidence_derived_count",
        "activation_authority_from_signing_receipt_audit_evidence_derived_count",
        "download_link_from_signing_receipt_audit_evidence_rendered_count",
        "install_command_from_signing_receipt_audit_evidence_rendered_count",
        "install_from_signing_receipt_audit_evidence_executed_count",
        "service_restart_from_signing_receipt_audit_evidence_performed_count",
        "active_binary_from_signing_receipt_audit_evidence_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count"
      ])
      + false_object([
        "artifact_distribution_signing_notarization_receipt_audit_evidence_accepted",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_recorded",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_persisted",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_materialized",
        "artifact_distribution_signing_notarization_receipt_audit_trail_recorded",
        "artifact_distribution_signing_notarization_receipt_audit_trail_persisted",
        "artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded",
        "artifact_distribution_signing_notarization_receipt_immutable_evidence_persisted",
        "artifact_distribution_signing_notarization_receipt_hash_chain_recorded",
        "artifact_distribution_signing_notarization_receipt_attestation_recorded",
        "artifact_distribution_signing_notarization_receipt_ledger_recorded",
        "artifact_distribution_signing_notarization_receipt_delivery_evidence_delivered",
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
          "artifact_distribution_signing_notarization_receipt_audit_evidence_recorded",
          "artifact_distribution_signing_notarization_receipt_audit_evidence_persisted",
          "artifact_distribution_signing_notarization_receipt_audit_evidence_materialized",
          "artifact_distribution_signing_notarization_receipt_audit_evidence_filesystem_written",
          "artifact_distribution_signing_notarization_receipt_audit_trail_recorded",
          "artifact_distribution_signing_notarization_receipt_audit_trail_persisted",
          "artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded",
          "artifact_distribution_signing_notarization_receipt_immutable_evidence_persisted",
          "artifact_distribution_signing_notarization_receipt_hash_chain_recorded",
          "artifact_distribution_signing_notarization_receipt_merkle_root_recorded",
          "artifact_distribution_signing_notarization_receipt_attestation_recorded",
          "artifact_distribution_signing_notarization_receipt_witness_recorded",
          "artifact_distribution_signing_notarization_receipt_notary_recorded",
          "artifact_distribution_signing_notarization_receipt_ledger_recorded",
          "artifact_distribution_signing_notarization_receipt_ledger_persisted",
          "artifact_distribution_signing_notarization_receipt_index_recorded",
          "artifact_distribution_signing_notarization_receipt_index_persisted",
          "artifact_distribution_signing_notarization_receipt_delivery_evidence_recorded",
          "artifact_distribution_signing_notarization_receipt_delivery_evidence_delivered",
          "artifact_distribution_signing_notarization_receipt_query_export_evidence_recorded",
          "artifact_distribution_signing_notarization_receipt_observability_evidence_recorded",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_audit_evidence_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_audit_evidence_denial_ready == true
  and $report.source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_ready == true
  and $report.source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_surface_count == 18
  and $report.source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_denied_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_audit_evidence_surface_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_audit_evidence_attempt_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_audit_evidence_denied_count == 18
  and zero_fields($report; [
    "artifact_distribution_signing_notarization_receipt_audit_evidence_allowed_count",
    "artifact_distribution_signing_notarization_receipt_audit_evidence_accepted_count",
    "artifact_distribution_signing_notarization_receipt_audit_evidence_recorded_count",
    "artifact_distribution_signing_notarization_receipt_audit_evidence_persisted_count",
    "artifact_distribution_signing_notarization_receipt_audit_evidence_materialized_count",
    "artifact_distribution_signing_notarization_receipt_audit_evidence_filesystem_written_count",
    "artifact_distribution_signing_notarization_receipt_audit_trail_recorded_count",
    "artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded_count",
    "artifact_distribution_signing_notarization_receipt_hash_chain_recorded_count",
    "artifact_distribution_signing_notarization_receipt_attestation_recorded_count",
    "artifact_distribution_signing_notarization_receipt_ledger_recorded_count",
    "artifact_distribution_signing_notarization_receipt_delivery_evidence_delivered_count",
    "release_publication_authority_from_signing_receipt_audit_evidence_derived_count",
    "activation_authority_from_signing_receipt_audit_evidence_derived_count",
    "install_from_signing_receipt_audit_evidence_executed_count",
    "service_restart_from_signing_receipt_audit_evidence_performed_count",
    "active_binary_from_signing_receipt_audit_evidence_mutated_count",
    "memory_store_write_performed_count",
    "live_kg_write_performed_count",
    "provider_invoked_count",
    "model_invoked_count",
    "credential_read_count",
    "secret_file_read_count",
    "external_send_performed_count"
  ])
  and false_fields($report; [
    "artifact_distribution_signing_notarization_receipt_audit_evidence_accepted",
    "artifact_distribution_signing_notarization_receipt_audit_evidence_recorded",
    "artifact_distribution_signing_notarization_receipt_audit_evidence_persisted",
    "artifact_distribution_signing_notarization_receipt_audit_trail_recorded",
    "artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded",
    "artifact_distribution_signing_notarization_receipt_hash_chain_recorded",
    "artifact_distribution_signing_notarization_receipt_attestation_recorded",
    "artifact_distribution_signing_notarization_receipt_ledger_recorded",
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
  and ($report.artifact_distribution_signing_notarization_receipt_audit_evidence_surfaces | length) == 18
  and ($report.artifact_distribution_signing_notarization_receipt_audit_evidence_surfaces | all(
    .artifact_distribution_signing_notarization_receipt_audit_evidence_attempted == true
    and .artifact_distribution_signing_notarization_receipt_audit_evidence_allowed == false
    and .artifact_distribution_signing_notarization_receipt_audit_evidence_accepted == false
    and .artifact_distribution_signing_notarization_receipt_audit_trail_recorded == false
    and .artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded == false
    and .artifact_distribution_signing_notarization_receipt_ledger_recorded == false
    and .artifact_distribution_signing_notarization_receipt_audit_evidence_noop_confirmed == true
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
  and ([.artifact_distribution_signing_notarization_receipt_audit_evidence_surfaces[] | select(.signature_manifest_withdrawal_hash_chain_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_audit_evidence_surfaces[] | select(.notarization_ticket_supersession_witness_notary_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_audit_evidence_surfaces[] | select(.telegram_audit_evidence_delivery_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_audit_evidence_surfaces[] | select(.install_restart_active_binary_audit_path_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .records_audit_evidence == false
    and .records_immutable_evidence == false
    and .records_hash_chain == false
    and .records_attestation == false
    and .records_witness == false
    and .records_notary == false
    and .records_ledger == false
    and .persists_audit_evidence == false
    and .accepts_retention == false
    and .accepts_expiry == false
    and .performs_garbage_collection == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt audit/evidence denial gate passed"
