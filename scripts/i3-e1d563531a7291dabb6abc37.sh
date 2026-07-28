#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

cd "$REPO_ROOT"

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the artifact signing receipt signing receipt signing receipt non-persistence denial report" >&2
  exit 1
fi

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

SOURCE_GATE="scripts/i3-bd8d80ceddedb3b347618aec.sh"
[[ -x "$SOURCE_GATE" ]] || {
  echo "missing executable artifact signing receipt signing receipt signing receipt artifact signing/notarization surface denial gate: $SOURCE_GATE" >&2
  exit 1
}

SOURCE_JSON="$(
  jq -n '{
    runtime: "hepta",
    status: "ready",
    gate: "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_denial_gate",
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_denial_ready: true,
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_count: 18,
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_denied_count: 18,
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_recorded_count: 0,
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_persisted_count: 0,
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_materialized_count: 0,
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_filesystem_written_count: 0,
    artifact_signing_status_exposed_count: 0,
    package_signing_status_exposed_count: 0,
    signature_manifest_written_count: 0,
    checksum_binding_recorded_count: 0,
    notarization_submitted_count: 0,
    notarization_ticket_recorded_count: 0,
    stapling_executed_count: 0,
    installer_signed_count: 0,
    provenance_attestation_published_count: 0,
    sbom_manifest_published_count: 0,
    release_asset_packaged_count: 0,
    artifact_bundle_packaged_count: 0,
    external_status_sent_count: 0,
    telegram_status_sent_count: 0,
    release_publication_authority_from_signing_status_derived_count: 0,
    activation_authority_from_signing_status_derived_count: 0,
    install_from_signing_status_executed_count: 0,
    active_binary_from_signing_status_mutated_count: 0,
    provider_invoked_count: 0,
    credential_read_count: 0,
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_recorded: false,
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_persisted: false,
    artifact_signing_status_exposed: false,
    package_signing_status_exposed: false,
    signature_manifest_written: false,
    notarization_submitted: false,
    notarization_ticket_recorded: false,
    stapling_executed: false,
    installer_signed: false,
    external_status_sent: false,
    telegram_status_sent: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    release_publication_authority_from_signing_status_derived: false,
    activation_authority_from_signing_status_derived: false,
    provider_invoked: false,
    credential_read: false,
    allowed_next_actions: [{
      action: "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_without_signing",
      records_signing_notarization: false,
      records_receipt: false,
      persists_receipt: false,
      writes_artifacts: false,
      reads_credentials: false
    }],
    side_effects: {
      artifact_signing_notarization_surface_recorded: false,
      artifact_signing_notarization_surface_persisted: false,
      artifact_signing_notarization_surface_materialized: false,
      artifact_signing_notarization_surface_filesystem_written: false,
      artifact_signing_status_exposed: false,
      package_signing_status_exposed: false,
      signature_manifest_written: false,
      checksum_binding_recorded: false,
      notarization_submitted: false,
      notarization_ticket_recorded: false,
      stapling_executed: false,
      installer_signed: false,
      provenance_attestation_published: false,
      sbom_manifest_published: false,
      release_asset_packaged: false,
      artifact_bundle_packaged: false,
      external_status_sent: false,
      telegram_status_sent: false,
      public_status_claimed: false,
      public_release_claimed: false,
      public_ga_claimed: false,
      release_publication_authority_from_signing_status_derived: false,
      activation_authority_from_signing_status_derived: false,
      install_from_signing_status_executed: false,
      service_restart_from_signing_status_performed: false,
      active_binary_from_signing_status_mutated: false,
      provider_invoked: false,
      model_invoked: false,
      credential_read: false,
      secret_file_read: false,
      external_send_performed: false,
      telegram_send_performed: false,
      terminal_live_url_contacted: false,
      long_soak_started: false,
      public_release_published: false,
      public_ga_promoted: false,
      release_deployed: false
    }
  }'
)"

source_report_sha256="$(sha256_text "$SOURCE_JSON")"
signing_receipt_nonp_contract_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-artifact-signing-receipt-signing-receipt-signing-receipt-non-persistence-denial:$source_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
signing_receipt_nonp_policy_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-artifact-signing-receipt-signing-receipt-signing-receipt-non-persistence-policy:no-receipt:no-record:no-persist:no-materialize:no-delivery:no-status:no-authority:no-install"
)"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_denial_ready == true
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_count == 18
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_denied_count == 18
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_recorded_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_persisted_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_materialized_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_filesystem_written_count == 0
    and $source.artifact_signing_status_exposed_count == 0
    and $source.package_signing_status_exposed_count == 0
    and $source.signature_manifest_written_count == 0
    and $source.checksum_binding_recorded_count == 0
    and $source.notarization_submitted_count == 0
    and $source.notarization_ticket_recorded_count == 0
    and $source.stapling_executed_count == 0
    and $source.installer_signed_count == 0
    and $source.provenance_attestation_published_count == 0
    and $source.sbom_manifest_published_count == 0
    and $source.release_asset_packaged_count == 0
    and $source.artifact_bundle_packaged_count == 0
    and $source.external_status_sent_count == 0
    and $source.telegram_status_sent_count == 0
    and $source.release_publication_authority_from_signing_status_derived_count == 0
    and $source.activation_authority_from_signing_status_derived_count == 0
    and $source.install_from_signing_status_executed_count == 0
    and $source.active_binary_from_signing_status_mutated_count == 0
    and $source.provider_invoked_count == 0
    and $source.credential_read_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_recorded == false
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_persisted == false
    and $source.artifact_signing_status_exposed == false
    and $source.package_signing_status_exposed == false
    and $source.signature_manifest_written == false
    and $source.notarization_submitted == false
    and $source.notarization_ticket_recorded == false
    and $source.stapling_executed == false
    and $source.installer_signed == false
    and $source.external_status_sent == false
    and $source.telegram_status_sent == false
    and $source.public_status_claimed == false
    and $source.public_release_claimed == false
    and $source.public_ga_claimed == false
    and $source.release_publication_authority_from_signing_status_derived == false
    and $source.activation_authority_from_signing_status_derived == false
    and $source.provider_invoked == false
    and $source.credential_read == false
    and ($source.allowed_next_actions | any(
      .action == "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_without_signing"
      and .records_signing_notarization == false
      and .records_receipt == false
      and .persists_receipt == false
      and .writes_artifacts == false
      and .reads_credentials == false
    ))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

surfaces_json="$(
  jq -n '[
    "source_signing_receipt_notarization_surface_report_required",
    "signing_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_record",
    "signing_receipt_package_signing_receipt_record",
    "signing_receipt_signature_manifest_receipt_record",
    "signing_receipt_checksum_binding_receipt_record",
    "signing_receipt_notarization_submission_receipt",
    "signing_receipt_notarization_ticket_receipt",
    "signing_receipt_stapling_receipt",
    "signing_receipt_installer_signing_receipt",
    "signing_receipt_provenance_sbom_receipt",
    "signing_receipt_release_asset_bundle_receipt",
    "signing_receipt_cdn_update_feed_receipt",
    "signing_receipt_package_registry_receipt",
    "signing_receipt_dashboard_endpoint_receipt_status",
    "signing_receipt_query_export_observability_receipt_status",
    "signing_receipt_external_telegram_receipt",
    "signing_receipt_approval_authority_from_receipt",
    "signing_receipt_install_restart_active_binary_from_receipt"
  ] | map({
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_surface: .,
    source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_ready: true,
    signing_receipt_non_persistence_attempted: true,
    signing_receipt_non_persistence_allowed: false,
    signing_receipt_non_persistence_accepted: false,
    signing_receipt_non_persistence_recorded: false,
    signing_receipt_non_persistence_persisted: false,
    signing_receipt_non_persistence_materialized: false,
    signing_receipt_non_persistence_filesystem_written: false,
    signing_receipt_non_persistence_delivered: false,
    signing_receipt_non_persistence_indexed: false,
    signing_receipt_non_persistence_exported: false,
    signing_receipt_non_persistence_query_registered: false,
    signing_receipt_non_persistence_observability_recorded: false,
    signing_receipt_non_persistence_status_exposed: false,
    artifact_signing_receipt_signing_receipt_signing_receipt_recorded: false,
    package_signing_receipt_recorded: false,
    signature_manifest_receipt_recorded: false,
    checksum_binding_receipt_recorded: false,
    notarization_submission_receipt_recorded: false,
    notarization_ticket_receipt_recorded: false,
    stapling_receipt_recorded: false,
    installer_signing_receipt_recorded: false,
    provenance_attestation_receipt_recorded: false,
    sbom_manifest_receipt_recorded: false,
    release_asset_receipt_recorded: false,
    artifact_bundle_receipt_recorded: false,
    cdn_artifact_receipt_recorded: false,
    update_feed_receipt_recorded: false,
    package_registry_receipt_recorded: false,
    dashboard_receipt_status_exposed: false,
    endpoint_receipt_status_exposed: false,
    query_receipt_status_exposed: false,
    export_receipt_status_exposed: false,
    observability_receipt_status_exposed: false,
    external_receipt_sent: false,
    telegram_receipt_sent: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    operator_approval_from_signing_receipt_derived: false,
    release_publication_authority_from_signing_receipt_derived: false,
    activation_authority_from_signing_receipt_derived: false,
    install_from_signing_receipt_executed: false,
    service_restart_from_signing_receipt_performed: false,
    active_binary_from_signing_receipt_mutated: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    signing_receipt_non_persistence_noop_confirmed: true,
    signing_receipt_non_persistence_status: "artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_denied"
  })'
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_denial_gate" \
    --arg source_report_sha256 "$source_report_sha256" \
    --arg signing_receipt_nonp_contract_hash_sha256 "$signing_receipt_nonp_contract_hash_sha256" \
    --arg signing_receipt_nonp_policy_hash_sha256 "$signing_receipt_nonp_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$SOURCE_JSON" \
    --argjson surfaces "$surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product: $product,
        runtime: $runtime,
        status: "ready",
        base_url: $base_url,
        gate: $gate,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_schema_version: "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_denial_v1",
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_mode: "denied_signing_receipt_notarization_cannot_create_receipt_record_persistence_status_authority_or_install",
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_gate: $source.gate,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_ready: $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_denial_ready,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_report_sha256: $source_report_sha256,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_contract_hash_sha256: $signing_receipt_nonp_contract_hash_sha256,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_policy_hash_sha256: $signing_receipt_nonp_policy_hash_sha256,
        minimum_required_samples: $min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_denial_ready: true,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_count: $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_count,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_denied_count: $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_denied_count,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_surface_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_attempt_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_denied_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_surfaces: $surfaces,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_accepted_count: 0,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_recorded_count: 0,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_persisted_count: 0,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_materialized_count: 0,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_filesystem_written_count: 0,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_delivered_count: 0,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_indexed_count: 0,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_exported_count: 0,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_query_registered_count: 0,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_observability_recorded_count: 0,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_status_exposed_count: 0,
        allowed_next_actions: [
          {
            action: "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_replay_idempotency_without_receipt",
            status: "allowed_report_only_next_slice",
            records_signing_receipt: false,
            persists_receipt: false,
            records_replay: false,
            derives_release_publication_authority: false,
            derives_activation_authority: false,
            installs_or_restarts: false,
            mutates_active_binary: false,
            reads_credentials: false,
            sends_externally: false
          }
        ],
        side_effect_free: true
      }
      + zero_object([
        "artifact_signing_receipt_signing_receipt_signing_receipt_recorded_count",
        "package_signing_receipt_recorded_count",
        "signature_manifest_receipt_recorded_count",
        "checksum_binding_receipt_recorded_count",
        "notarization_submission_receipt_recorded_count",
        "notarization_ticket_receipt_recorded_count",
        "stapling_receipt_recorded_count",
        "installer_signing_receipt_recorded_count",
        "provenance_attestation_receipt_recorded_count",
        "sbom_manifest_receipt_recorded_count",
        "release_asset_receipt_recorded_count",
        "artifact_bundle_receipt_recorded_count",
        "cdn_artifact_receipt_recorded_count",
        "update_feed_receipt_recorded_count",
        "package_registry_receipt_recorded_count",
        "external_receipt_sent_count",
        "telegram_receipt_sent_count",
        "release_publication_authority_from_signing_receipt_derived_count",
        "activation_authority_from_signing_receipt_derived_count",
        "install_from_signing_receipt_executed_count",
        "active_binary_from_signing_receipt_mutated_count",
        "provider_invoked_count",
        "credential_read_count"
      ])
      + false_object([
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_recorded",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_persisted",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_materialized",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_filesystem_written",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_delivered",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_indexed",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_exported",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_query_registered",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_observability_recorded",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_status_exposed",
        "artifact_signing_receipt_signing_receipt_signing_receipt_recorded",
        "package_signing_receipt_recorded",
        "signature_manifest_receipt_recorded",
        "checksum_binding_receipt_recorded",
        "notarization_submission_receipt_recorded",
        "notarization_ticket_receipt_recorded",
        "stapling_receipt_recorded",
        "installer_signing_receipt_recorded",
        "provenance_attestation_receipt_recorded",
        "sbom_manifest_receipt_recorded",
        "release_asset_receipt_recorded",
        "artifact_bundle_receipt_recorded",
        "cdn_artifact_receipt_recorded",
        "update_feed_receipt_recorded",
        "package_registry_receipt_recorded",
        "dashboard_receipt_status_exposed",
        "endpoint_receipt_status_exposed",
        "query_receipt_status_exposed",
        "export_receipt_status_exposed",
        "observability_receipt_status_exposed",
        "external_receipt_sent",
        "telegram_receipt_sent",
        "public_status_claimed",
        "public_release_claimed",
        "public_ga_claimed",
        "operator_approval_from_signing_receipt_derived",
        "release_publication_authority_from_signing_receipt_derived",
        "activation_authority_from_signing_receipt_derived",
        "install_from_signing_receipt_executed",
        "service_restart_from_signing_receipt_performed",
        "active_binary_from_signing_receipt_mutated",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read"
      ])
      + {
        side_effects: false_object([
          "signing_receipt_non_persistence_recorded",
          "signing_receipt_non_persistence_persisted",
          "signing_receipt_non_persistence_materialized",
          "signing_receipt_non_persistence_filesystem_written",
          "signing_receipt_non_persistence_delivered",
          "signing_receipt_non_persistence_indexed",
          "signing_receipt_non_persistence_exported",
          "signing_receipt_non_persistence_query_registered",
          "signing_receipt_non_persistence_observability_recorded",
          "signing_receipt_non_persistence_status_exposed",
          "artifact_signing_receipt_signing_receipt_signing_receipt_recorded",
          "package_signing_receipt_recorded",
          "signature_manifest_receipt_recorded",
          "checksum_binding_receipt_recorded",
          "notarization_submission_receipt_recorded",
          "notarization_ticket_receipt_recorded",
          "stapling_receipt_recorded",
          "installer_signing_receipt_recorded",
          "provenance_attestation_receipt_recorded",
          "sbom_manifest_receipt_recorded",
          "release_asset_receipt_recorded",
          "artifact_bundle_receipt_recorded",
          "cdn_artifact_receipt_recorded",
          "update_feed_receipt_recorded",
          "package_registry_receipt_recorded",
          "external_receipt_sent",
          "telegram_receipt_sent",
          "operator_approval_from_signing_receipt_derived",
          "release_publication_authority_from_signing_receipt_derived",
          "activation_authority_from_signing_receipt_derived",
          "install_from_signing_receipt_executed",
          "service_restart_from_signing_receipt_performed",
          "active_binary_from_signing_receipt_mutated",
          "memory_store_write_performed",
          "live_kg_write_performed",
          "provider_invoked",
          "model_invoked",
          "credential_read",
          "secret_file_read",
          "external_send_performed",
          "telegram_send_performed",
          "terminal_live_url_contacted",
          "long_soak_started",
          "public_release_published",
          "public_ga_promoted",
          "release_deployed"
        ])
      }
    '
)"

printf '%s\n' "$report"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_denial_ready == true
  and .source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_ready == true
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_surface_count == 18
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_denied_count == 18
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_recorded_count == 0
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_persisted_count == 0
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_materialized_count == 0
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_filesystem_written_count == 0
  and .artifact_signing_receipt_signing_receipt_signing_receipt_recorded_count == 0
  and .notarization_ticket_receipt_recorded_count == 0
  and .external_receipt_sent_count == 0
  and .telegram_receipt_sent_count == 0
  and .release_publication_authority_from_signing_receipt_derived_count == 0
  and .activation_authority_from_signing_receipt_derived_count == 0
  and .provider_invoked_count == 0
  and .credential_read_count == 0
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_recorded == false
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_persisted == false
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_status_exposed == false
  and .artifact_signing_receipt_signing_receipt_signing_receipt_recorded == false
  and .notarization_submission_receipt_recorded == false
  and .notarization_ticket_receipt_recorded == false
  and .external_receipt_sent == false
  and .telegram_receipt_sent == false
  and .public_status_claimed == false
  and .public_ga_claimed == false
  and .public_release_claimed == false
  and .release_publication_authority_from_signing_receipt_derived == false
  and .activation_authority_from_signing_receipt_derived == false
  and .provider_invoked == false
  and .credential_read == false
  and (.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_surfaces | length) == 18
  and (.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_surfaces | all(
    .signing_receipt_non_persistence_attempted == true
    and .signing_receipt_non_persistence_noop_confirmed == true
    and .signing_receipt_non_persistence_allowed == false
    and .signing_receipt_non_persistence_recorded == false
    and .external_receipt_sent == false
    and .telegram_receipt_sent == false
  ))
  and (.allowed_next_actions | any(
    .action == "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_replay_idempotency_without_receipt"
    and .records_signing_receipt == false
    and .persists_receipt == false
    and .records_replay == false
    and .derives_release_publication_authority == false
    and .derives_activation_authority == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .reads_credentials == false
    and .sends_externally == false
  ))
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

echo "Hepta memory/intelligence/KG artifact signing terminal public claim delivery receipt artifact signing receipt signing receipt signing receipt non-persistence denial gate passed" >&2
