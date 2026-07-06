#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-retention-expiry-gc-final-index-report.sh"
OBSERVABILITY_GATE="$ROOT/scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-observability-denial-gate.sh"
OBSERVABILITY_DOC="$ROOT/docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_GATE.md"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable terminal public claim delivery receipt retention/expiry/GC final index report: $SOURCE_REPORT" >&2
  exit 1
}
[[ -f "$OBSERVABILITY_GATE" ]] || {
  echo "missing terminal public claim delivery receipt export/query/observability denial gate: $OBSERVABILITY_GATE" >&2
  exit 1
}
[[ -f "$OBSERVABILITY_DOC" ]] || {
  echo "missing terminal public claim delivery receipt export/query/observability denial doc: $OBSERVABILITY_DOC" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the terminal public claim delivery receipt export/query/observability report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_final_index"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_final_index_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_final_index_blocked == true
  and .final_blocker_count == 102
  and .retention_policy_recorded == false
  and .expiry_timer_started == false
  and .garbage_collection_executed == false
  and .release_publication_authority_from_delivery_receipt_retention_derived == false
  and .activation_authority_from_delivery_receipt_retention_derived == false
  and .public_ga_claimed == false
' <<<"$source_json" >/dev/null

observability_static_mention_count="$(
  grep -Eci 'query|export|observability|metric|log|trace|event|dashboard|alert|slo|readback|audit.view|ledger|index|delivery|summary|receipt|ack|authority|download|install|restart|active-binary|credential|secret|provider|model|telegram|external|live' "$OBSERVABILITY_GATE" || true
)"

jq -n \
  --argjson source "$source_json" \
  --argjson observability_static_mention_count "$observability_static_mention_count" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_attachment",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_final_index_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_final_index_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_final_index_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_final_index_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_final_index_blocked,
    source_final_blocker_count: $source.final_blocker_count,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_final_index_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_attachment_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_attachment_blocked: true,
    artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_denial_gate_present: true,
    artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_denial_doc_present: true,
    artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_static_mention_count: $observability_static_mention_count,
    artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_denial_gate_invoked: false,
    artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_denial_gate_invoked: false,
    long_soak_started: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    terminal_public_claim_delivery_receipt_export_query_observability_allowed: false,
    terminal_public_claim_delivery_receipt_export_query_observability_accepted: false,
    terminal_public_claim_delivery_receipt_export_query_observability_recorded: false,
    terminal_public_claim_delivery_receipt_export_query_observability_persisted: false,
    terminal_public_claim_delivery_receipt_export_query_observability_materialized: false,
    terminal_public_claim_delivery_receipt_export_query_observability_filesystem_written: false,
    query_registered: false,
    query_executed: false,
    query_result_recorded: false,
    query_result_persisted: false,
    search_index_recorded: false,
    search_index_persisted: false,
    export_accepted: false,
    export_snapshot_recorded: false,
    export_snapshot_persisted: false,
    export_file_written: false,
    export_stream_opened: false,
    observability_metric_recorded: false,
    observability_log_recorded: false,
    observability_trace_recorded: false,
    observability_event_recorded: false,
    dashboard_panel_recorded: false,
    alert_registered: false,
    slo_recorded: false,
    operator_summary_recorded: false,
    readback_surface_recorded: false,
    audit_view_recorded: false,
    ledger_observability_recorded: false,
    index_observability_recorded: false,
    delivery_observability_recorded: false,
    result_receipt_recorded: false,
    result_receipt_persisted: false,
    result_receipt_exported: false,
    result_receipt_query_registered: false,
    result_receipt_observability_recorded: false,
    completion_ack_recorded: false,
    operator_acceptance_from_export_query_observability_recorded: false,
    operator_approval_from_export_query_observability_derived: false,
    release_publication_authority_from_export_query_observability_derived: false,
    activation_authority_from_export_query_observability_derived: false,
    download_link_from_export_query_observability_rendered: false,
    install_command_from_export_query_observability_rendered: false,
    install_from_export_query_observability_executed: false,
    service_restart_from_export_query_observability_performed: false,
    launchd_from_export_query_observability_mutated: false,
    active_binary_from_export_query_observability_mutated: false,
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
    attachment_blocker_count: 104,
    manual_operator_live_cutover_approval_required: true,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_readback_without_retention",
    local_gate: "scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-retention-expiry-gc-final-index-artifact-signing-terminal-public-claim-delivery-receipt-export-query-observability-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_RETENTION_EXPIRY_GC_FINAL_INDEX_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_EXPORT_QUERY_OBSERVABILITY_2026-06-21.md",
    source_files: {
      terminal_public_claim_delivery_receipt_retention_expiry_gc_final_index_report: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-retention-expiry-gc-final-index-report.sh",
      terminal_public_claim_delivery_receipt_export_query_observability_denial_gate: "scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-observability-denial-gate.sh",
      terminal_public_claim_delivery_receipt_export_query_observability_denial_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_GATE.md"
    },
    side_effect_free: true,
    side_effects: {
      report_written: false,
      git_index_mutated: false,
      artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_denial_gate_invoked: false,
      artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_denial_gate_invoked: false,
      query_registered: false,
      query_executed: false,
      query_result_recorded: false,
      query_result_persisted: false,
      search_index_recorded: false,
      search_index_persisted: false,
      export_accepted: false,
      export_snapshot_recorded: false,
      export_snapshot_persisted: false,
      export_file_written: false,
      export_stream_opened: false,
      observability_metric_recorded: false,
      observability_log_recorded: false,
      observability_trace_recorded: false,
      observability_event_recorded: false,
      dashboard_panel_recorded: false,
      alert_registered: false,
      slo_recorded: false,
      operator_summary_recorded: false,
      readback_surface_recorded: false,
      audit_view_recorded: false,
      ledger_observability_recorded: false,
      index_observability_recorded: false,
      delivery_observability_recorded: false,
      result_receipt_recorded: false,
      result_receipt_persisted: false,
      completion_ack_recorded: false,
      operator_acceptance_from_export_query_observability_recorded: false,
      operator_approval_from_export_query_observability_derived: false,
      release_publication_authority_from_export_query_observability_derived: false,
      activation_authority_from_export_query_observability_derived: false,
      install_from_export_query_observability_executed: false,
      service_restart_from_export_query_observability_performed: false,
      active_binary_from_export_query_observability_mutated: false,
      memory_store_write_performed: false,
      live_kg_write_performed: false,
      provider_invoked: false,
      model_invoked: false,
      credential_read: false,
      secret_file_read: false,
      telegram_send_performed: false,
      external_send_performed: false,
      long_soak_started: false,
      terminal_live_gate_invoked: false,
      terminal_live_url_contacted: false,
      public_ga_readiness_script_invoked: false,
      public_claim_non_promotion_denial_gate_invoked: false,
      public_ga_claim_recorded: false,
      public_ga_promoted: false,
      public_release_published: false,
      rollback_executed: false,
      external_network_read: false,
      release_artifact_written: false,
      public_artifact_written: false,
      filesystem_written: false
    }
  }'
