#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-terminal-status-final-index-report.sh"
PUBLIC_STATUS_GATE="$ROOT/scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-public-status-denial-gate.sh"
PUBLIC_STATUS_DOC="$ROOT/docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_SIGNING_RECEIPT_PUBLIC_STATUS_DENIAL_GATE.md"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable signing receipt terminal decision/status final index report: $SOURCE_REPORT" >&2
  exit 1
}
[[ -f "$PUBLIC_STATUS_GATE" ]] || {
  echo "missing signing receipt public status denial gate: $PUBLIC_STATUS_GATE" >&2
  exit 1
}
[[ -f "$PUBLIC_STATUS_DOC" ]] || {
  echo "missing signing receipt public status denial doc: $PUBLIC_STATUS_DOC" >&2
  exit 1
}

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_promotion_final_index"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_promotion_final_index_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_promotion_final_index_blocked == true
  and .final_blocker_count == 166
  and .terminal_decision_recorded == false
  and .terminal_status_recorded == false
  and .status_promotion_recorded == false
  and .public_status_exposed == false
  and .public_ga_status_exposed == false
  and .public_release_status_exposed == false
  and .external_decision_sent == false
  and .telegram_decision_sent == false
  and .release_publication_authority_from_terminal_status_derived == false
  and .activation_authority_from_terminal_status_derived == false
  and .install_from_terminal_status_executed == false
  and .active_binary_from_terminal_status_mutated == false
  and .public_ga_claim_allowed == false
  and .public_ga_claimed == false
  and .public_release_published == false
' <<<"$source_json" >/dev/null

public_status_static_mention_count="$(
  grep -Eci 'public|claim|status|exposure|release|channel|dashboard|endpoint|query|export|observability|telegram|external|authority|install|restart|active-binary|live' "$PUBLIC_STATUS_GATE" || true
)"

jq -n \
  --argjson source "$source_json" \
  --argjson public_status_static_mention_count "$public_status_static_mention_count" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_attachment",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_promotion_final_index_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_promotion_final_index_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_promotion_final_index_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_promotion_final_index_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_promotion_final_index_blocked,
    source_final_blocker_count: $source.final_blocker_count,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_promotion_final_index_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_attachment_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_attachment_blocked: true,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_denial_gate_present: true,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_denial_doc_present: true,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_static_mention_count: $public_status_static_mention_count,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_denial_gate_invoked: false,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_promotion_denial_gate_invoked: false,
    long_soak_started: false,
    terminal_decision_recorded: false,
    terminal_status_recorded: false,
    status_promotion_recorded: false,
    terminal_public_claim_status_exposure_requested: false,
    terminal_public_claim_status_exposure_allowed: false,
    terminal_public_claim_status_exposure_accepted: false,
    terminal_public_claim_status_exposure_recorded: false,
    terminal_public_claim_status_exposure_persisted: false,
    terminal_public_claim_status_exposure_materialized: false,
    terminal_public_claim_status_exposure_filesystem_written: false,
    terminal_public_claim_status_exposure_delivered: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    public_status_exposed: false,
    public_ga_status_exposed: false,
    public_release_status_exposed: false,
    release_status_exposed: false,
    publication_status_exposed: false,
    package_release_channel_status_exposed: false,
    dashboard_status_exposed: false,
    public_badge_exposed: false,
    status_endpoint_exposed: false,
    query_status_exposed: false,
    export_status_exposed: false,
    observability_status_exposed: false,
    artifact_availability_status_exposed: false,
    distribution_queue_status_exposed: false,
    channel_status_delivered: false,
    external_status_sent: false,
    telegram_status_sent: false,
    acceptance_from_public_status_recorded: false,
    operator_approval_from_public_status_derived: false,
    release_publication_authority_from_public_status_derived: false,
    activation_authority_from_public_status_derived: false,
    activation_command_from_public_status_derived: false,
    live_execution_from_public_status_allowed: false,
    download_link_from_public_status_rendered: false,
    install_command_from_public_status_rendered: false,
    install_from_public_status_executed: false,
    service_restart_from_public_status_performed: false,
    active_binary_from_public_status_mutated: false,
    memory_store_write_performed: false,
    live_kg_write_performed: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    telegram_send_performed: false,
    external_send_performed: false,
    public_ga_readiness_script_invoked: false,
    public_claim_non_promotion_denial_gate_invoked: false,
    terminal_live_gates_invoked: false,
    attachment_blocker_count: 168,
    manual_operator_live_cutover_approval_required: true,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_readback_without_status_promotion",
    local_gate: "scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-terminal-status-final-index-delivery-receipt-signing-receipt-public-status-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_SIGNING_RECEIPT_TERMINAL_STATUS_FINAL_INDEX_DELIVERY_RECEIPT_SIGNING_RECEIPT_PUBLIC_STATUS_2026-06-21.md",
    source_files: {
      signing_receipt_terminal_decision_status_final_index_report: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-terminal-status-final-index-report.sh",
      signing_receipt_public_status_denial_gate: "scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-public-status-denial-gate.sh",
      signing_receipt_public_status_denial_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_SIGNING_RECEIPT_PUBLIC_STATUS_DENIAL_GATE.md"
    },
    side_effect_free: true,
    side_effects: {
      report_written: false,
      git_index_mutated: false,
      public_claim_status_exposure_denial_gate_invoked: false,
      terminal_decision_status_promotion_denial_gate_invoked: false,
      public_claim_recorded: false,
      public_claim_persisted: false,
      public_status_exposed: false,
      public_ga_status_exposed: false,
      public_release_status_exposed: false,
      release_status_exposed: false,
      publication_status_exposed: false,
      package_release_channel_status_exposed: false,
      dashboard_status_exposed: false,
      status_endpoint_exposed: false,
      query_status_exposed: false,
      export_status_exposed: false,
      observability_status_exposed: false,
      channel_status_delivered: false,
      external_status_sent: false,
      telegram_status_sent: false,
      operator_approval_from_public_status_derived: false,
      release_publication_authority_from_public_status_derived: false,
      activation_authority_from_public_status_derived: false,
      install_from_public_status_executed: false,
      service_restart_from_public_status_performed: false,
      active_binary_from_public_status_mutated: false,
      provider_invoked: false,
      model_invoked: false,
      credential_read: false,
      secret_file_read: false,
      external_send_performed: false,
      telegram_send_performed: false,
      long_soak_started: false,
      terminal_live_gate_invoked: false,
      terminal_live_url_contacted: false,
      public_ga_readiness_script_invoked: false,
      public_claim_non_promotion_denial_gate_invoked: false,
      public_ga_claim_recorded: false,
      public_ga_promoted: false,
      public_release_published: false,
      rollback_executed: false,
      external_network_read: false
    }
  }'
