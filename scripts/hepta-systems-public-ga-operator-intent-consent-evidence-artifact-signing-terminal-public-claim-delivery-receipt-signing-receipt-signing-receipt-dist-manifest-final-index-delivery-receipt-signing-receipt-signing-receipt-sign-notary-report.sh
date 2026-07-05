#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-signing-receipt-dist-manifest-final-index-report.sh"
NOTARIZATION_GATE="$ROOT/scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-signing-receipt-sign-notary-denial-gate.sh"
NOTARIZATION_DOC="$ROOT/docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_SIGNING_RECEIPT_SIGNING_RECEIPT_SIGN_NOTARY_DENIAL_GATE.md"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable artifact signing receipt signing receipt signing receipt distribution/manifest final index report: $SOURCE_REPORT" >&2
  exit 1
}
[[ -f "$NOTARIZATION_GATE" ]] || {
  echo "missing artifact signing receipt signing receipt signing receipt artifact signing/notarization surface denial gate: $NOTARIZATION_GATE" >&2
  exit 1
}
[[ -f "$NOTARIZATION_DOC" ]] || {
  echo "missing artifact signing receipt signing receipt signing receipt artifact signing/notarization surface denial doc: $NOTARIZATION_DOC" >&2
  exit 1
}

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_distribution_artifact_manifest_status_final_index"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_distribution_artifact_manifest_status_final_index_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_distribution_artifact_manifest_status_final_index_blocked == true
  and .final_blocker_count == 256
  and .distribution_artifact_manifest_status_recorded == false
  and .distribution_artifact_status_exposed == false
  and .manifest_status_exposed == false
  and .external_status_sent == false
  and .telegram_status_sent == false
  and .release_publication_authority_from_manifest_status_derived == false
  and .activation_authority_from_manifest_status_derived == false
  and .install_from_manifest_status_executed == false
  and .active_binary_from_manifest_status_mutated == false
  and .public_ga_claim_allowed == false
  and .public_release_published == false
' <<<"$source_json" >/dev/null

notarization_static_mention_count="$(
  grep -Eci 'signing|signature|checksum|notarization|ticket|stapling|installer|provenance|sbom|release|bundle|cdn|feed|registry|dashboard|endpoint|query|export|observability|telegram|external|authority|install|restart|active-binary|credential' "$NOTARIZATION_GATE" || true
)"

jq -n \
  --argjson source "$source_json" \
  --argjson notarization_static_mention_count "$notarization_static_mention_count" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_attachment",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_distribution_artifact_manifest_status_final_index_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_distribution_artifact_manifest_status_final_index_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_distribution_artifact_manifest_status_final_index_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_distribution_artifact_manifest_status_final_index_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_distribution_artifact_manifest_status_final_index_blocked,
    source_final_blocker_count: $source.final_blocker_count,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_distribution_artifact_manifest_status_final_index_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_attachment_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_attachment_blocked: true,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_denial_gate_present: true,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_denial_doc_present: true,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_notarization_static_mention_count: $notarization_static_mention_count,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_denial_gate_invoked: false,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_distribution_artifact_manifest_status_denial_gate_invoked: false,
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
    cdn_artifact_written: false,
    update_feed_artifact_written: false,
    package_registry_artifact_published: false,
    dashboard_signing_status_exposed: false,
    endpoint_signing_status_exposed: false,
    query_status_exposed: false,
    export_status_exposed: false,
    observability_status_exposed: false,
    external_status_sent: false,
    telegram_status_sent: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    operator_approval_from_signing_status_derived: false,
    release_publication_authority_from_signing_status_derived: false,
    activation_authority_from_signing_status_derived: false,
    install_from_signing_status_executed: false,
    service_restart_from_signing_status_performed: false,
    active_binary_from_signing_status_mutated: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    terminal_live_gates_invoked: false,
    attachment_blocker_count: 258,
    manual_operator_live_cutover_approval_required: true,
    terminal_live_url_required: false,
    long_soak_required: false,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_artifact_signing_notarization_surface_readback_without_manifest_status",
    local_gate: "scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-signing-receipt-dist-manifest-final-index-delivery-receipt-signing-receipt-signing-receipt-sign-notary-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_SIGNING_RECEIPT_SIGNING_RECEIPT_DIST_MANIFEST_FINAL_INDEX_DELIVERY_RECEIPT_SIGNING_RECEIPT_SIGNING_RECEIPT_SIGN_NOTARY_2026-06-21.md",
    side_effect_free: true,
    side_effects: {
      report_written: false,
      git_index_mutated: false,
      signing_notarization_surface_denial_gate_invoked: false,
      distribution_artifact_manifest_status_denial_gate_invoked: false,
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
      cdn_artifact_written: false,
      update_feed_artifact_written: false,
      package_registry_artifact_published: false,
      query_status_exposed: false,
      export_status_exposed: false,
      observability_status_exposed: false,
      external_status_sent: false,
      telegram_status_sent: false,
      operator_approval_from_signing_status_derived: false,
      release_publication_authority_from_signing_status_derived: false,
      activation_authority_from_signing_status_derived: false,
      install_from_signing_status_executed: false,
      service_restart_from_signing_status_performed: false,
      active_binary_from_signing_status_mutated: false,
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
