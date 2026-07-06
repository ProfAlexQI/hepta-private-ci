#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-package-channel-final-index-report.sh"
DISTRIBUTION_MANIFEST_GATE="$ROOT/scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-distribution-manifest-denial-gate.sh"
DISTRIBUTION_MANIFEST_DOC="$ROOT/docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_SIGNING_DISTRIBUTION_MANIFEST_DENIAL_GATE.md"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable artifact signing receipt package/release/channel status final index report: $SOURCE_REPORT" >&2
  exit 1
}
[[ -f "$DISTRIBUTION_MANIFEST_GATE" ]] || {
  echo "missing artifact signing receipt distribution artifact/manifest status denial gate: $DISTRIBUTION_MANIFEST_GATE" >&2
  exit 1
}
[[ -f "$DISTRIBUTION_MANIFEST_DOC" ]] || {
  echo "missing artifact signing receipt distribution artifact/manifest status denial doc: $DISTRIBUTION_MANIFEST_DOC" >&2
  exit 1
}

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_final_index"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_final_index_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_final_index_blocked == true
  and .final_blocker_count == 142
  and .package_release_channel_status_recorded == false
  and .package_channel_status_exposed == false
  and .release_channel_status_exposed == false
  and .external_status_sent == false
  and .telegram_status_sent == false
  and .release_publication_authority_from_package_channel_derived == false
  and .activation_authority_from_package_channel_derived == false
  and .install_from_package_channel_executed == false
  and .active_binary_from_package_channel_mutated == false
  and .public_ga_claim_allowed == false
  and .public_release_published == false
' <<<"$source_json" >/dev/null

distribution_manifest_static_mention_count="$(
  grep -Eci 'distribution|artifact|manifest|index|catalog|checksum|provenance|signature|dashboard|endpoint|query|export|observability|telegram|external|authority|install|restart|active-binary|credential' "$DISTRIBUTION_MANIFEST_GATE" || true
)"

jq -n \
  --argjson source "$source_json" \
  --argjson distribution_manifest_static_mention_count "$distribution_manifest_static_mention_count" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_distribution_artifact_manifest_status_attachment",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_final_index_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_final_index_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_final_index_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_final_index_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_final_index_blocked,
    source_final_blocker_count: $source.final_blocker_count,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_final_index_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_distribution_artifact_manifest_status_attachment_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_distribution_artifact_manifest_status_attachment_blocked: true,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_distribution_artifact_manifest_status_denial_gate_present: true,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_distribution_artifact_manifest_status_denial_doc_present: true,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_distribution_manifest_static_mention_count: $distribution_manifest_static_mention_count,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_distribution_artifact_manifest_status_denial_gate_invoked: false,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_denial_gate_invoked: false,
    distribution_artifact_manifest_status_recorded: false,
    distribution_artifact_manifest_status_persisted: false,
    distribution_artifact_manifest_status_materialized: false,
    distribution_artifact_manifest_status_filesystem_written: false,
    distribution_artifact_manifest_status_delivered: false,
    distribution_artifact_status_exposed: false,
    manifest_status_exposed: false,
    artifact_index_status_exposed: false,
    package_manifest_materialized: false,
    release_manifest_published: false,
    artifact_catalog_status_exposed: false,
    manifest_checksum_status_exposed: false,
    artifact_provenance_status_exposed: false,
    manifest_signature_status_exposed: false,
    dashboard_status_exposed: false,
    public_endpoint_status_exposed: false,
    query_status_exposed: false,
    export_status_exposed: false,
    observability_status_exposed: false,
    external_status_sent: false,
    telegram_status_sent: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    operator_approval_from_manifest_status_derived: false,
    release_publication_authority_from_manifest_status_derived: false,
    activation_authority_from_manifest_status_derived: false,
    install_from_manifest_status_executed: false,
    service_restart_from_manifest_status_performed: false,
    active_binary_from_manifest_status_mutated: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    terminal_live_gates_invoked: false,
    attachment_blocker_count: 144,
    manual_operator_live_cutover_approval_required: true,
    terminal_live_url_required: false,
    long_soak_required: false,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_distribution_artifact_manifest_status_readback_without_package_channel",
    local_gate: "scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-package-channel-final-index-delivery-receipt-signing-distribution-manifest-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_SIGNING_PACKAGE_CHANNEL_FINAL_INDEX_DELIVERY_RECEIPT_SIGNING_DISTRIBUTION_MANIFEST_2026-06-21.md",
    side_effect_free: true,
    side_effects: {
      report_written: false,
      git_index_mutated: false,
      distribution_artifact_manifest_status_denial_gate_invoked: false,
      package_release_channel_status_denial_gate_invoked: false,
      distribution_artifact_manifest_status_recorded: false,
      distribution_artifact_manifest_status_persisted: false,
      distribution_artifact_manifest_status_materialized: false,
      distribution_artifact_manifest_status_filesystem_written: false,
      distribution_artifact_status_exposed: false,
      manifest_status_exposed: false,
      package_manifest_materialized: false,
      release_manifest_published: false,
      query_status_exposed: false,
      export_status_exposed: false,
      observability_status_exposed: false,
      external_status_sent: false,
      telegram_status_sent: false,
      operator_approval_from_manifest_status_derived: false,
      release_publication_authority_from_manifest_status_derived: false,
      activation_authority_from_manifest_status_derived: false,
      install_from_manifest_status_executed: false,
      service_restart_from_manifest_status_performed: false,
      active_binary_from_manifest_status_mutated: false,
      provider_invoked: false,
      model_invoked: false,
      credential_read: false,
      secret_file_read: false,
      terminal_live_gate_invoked: false,
      terminal_live_url_contacted: false,
      long_soak_started: false,
      public_ga_promoted: false,
      public_release_published: false,
      release_deployed: false
    }
  }'
