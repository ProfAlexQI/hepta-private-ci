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
  echo "jq is required to build the terminal public claim delivery receipt artifact signing receipt signing receipt signing receipt export/query/observability denial report" >&2
  exit 1
fi

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

SOURCE_GATE="scripts/i3-805519f77fce19f768f30d6d.sh"
[[ -x "$SOURCE_GATE" ]] || {
  echo "missing executable terminal public claim delivery receipt artifact signing receipt signing receipt signing receipt retention/expiry/GC denial gate: $SOURCE_GATE" >&2
  exit 1
}

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    "$SOURCE_GATE"
)"

source_report_sha256="$(sha256_text "$SOURCE_JSON")"
signing_receipt_export_contract_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-artifact-signing-receipt-signing-receipt-export-query-observability-denial:$source_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
signing_receipt_export_policy_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-signing-receipt-signing-receipt-export-query-observability:no-query:no-export:no-observability:no-dashboard:no-readback:no-authority:no-install"
)"

surfaces_json="$(
  jq -n '[
    "source_signing_receipt_retention_report_required",
    "signing_receipt_query_registration",
    "signing_receipt_query_execution",
    "signing_receipt_query_result",
    "signing_receipt_search_index",
    "signing_receipt_export_request",
    "signing_receipt_export_snapshot",
    "signing_receipt_export_file",
    "signing_receipt_export_stream",
    "signing_receipt_observability_metric",
    "signing_receipt_observability_log",
    "signing_receipt_observability_trace",
    "signing_receipt_observability_event",
    "signing_receipt_dashboard_alert_slo",
    "signing_receipt_operator_readback_audit_view",
    "artifact_package_signature_receipt_observability",
    "notarization_release_registry_receipt_observability",
    "external_telegram_authority_install_observability"
  ] | map({
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_surface: .,
    source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_ready: true,
    signing_receipt_export_query_observability_attempted: true,
    signing_receipt_export_query_observability_allowed: false,
    signing_receipt_export_query_observability_accepted: false,
    signing_receipt_export_query_observability_recorded: false,
    signing_receipt_export_query_observability_persisted: false,
    signing_receipt_export_query_observability_materialized: false,
    signing_receipt_export_query_observability_filesystem_written: false,
    signing_receipt_query_registered: false,
    signing_receipt_query_executed: false,
    signing_receipt_query_result_recorded: false,
    signing_receipt_query_result_persisted: false,
    signing_receipt_search_index_recorded: false,
    signing_receipt_search_index_persisted: false,
    signing_receipt_export_accepted: false,
    signing_receipt_export_snapshot_recorded: false,
    signing_receipt_export_snapshot_persisted: false,
    signing_receipt_export_file_written: false,
    signing_receipt_export_stream_opened: false,
    signing_receipt_observability_metric_recorded: false,
    signing_receipt_observability_log_recorded: false,
    signing_receipt_observability_trace_recorded: false,
    signing_receipt_observability_event_recorded: false,
    signing_receipt_dashboard_panel_recorded: false,
    signing_receipt_alert_registered: false,
    signing_receipt_slo_recorded: false,
    signing_receipt_operator_summary_recorded: false,
    signing_receipt_readback_surface_recorded: false,
    signing_receipt_audit_view_recorded: false,
    signing_receipt_ledger_observability_recorded: false,
    signing_receipt_index_observability_recorded: false,
    signing_receipt_delivery_observability_recorded: false,
    signing_receipt_status_observability_recorded: false,
    artifact_signing_receipt_observability_recorded: false,
    package_signing_receipt_observability_recorded: false,
    signature_manifest_receipt_observability_recorded: false,
    notarization_ticket_receipt_observability_recorded: false,
    release_asset_receipt_observability_recorded: false,
    cdn_update_feed_receipt_observability_recorded: false,
    package_registry_receipt_observability_recorded: false,
    external_signing_receipt_observability_recorded: false,
    telegram_signing_receipt_observability_recorded: false,
    operator_acceptance_from_signing_receipt_export_query_observability_recorded: false,
    operator_approval_from_signing_receipt_export_query_observability_derived: false,
    release_publication_authority_from_signing_receipt_export_query_observability_derived: false,
    activation_authority_from_signing_receipt_export_query_observability_derived: false,
    install_from_signing_receipt_export_query_observability_executed: false,
    service_restart_from_signing_receipt_export_query_observability_performed: false,
    active_binary_from_signing_receipt_export_query_observability_mutated: false,
    memory_store_write_performed: false,
    live_kg_write_performed: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    external_send_performed: false,
    telegram_send_performed: false,
    signing_receipt_export_query_observability_noop_confirmed: true,
    signing_receipt_export_query_observability_status: "artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_denied"
  })'
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_denial_gate" \
    --arg source_report_sha256 "$source_report_sha256" \
    --arg signing_receipt_export_contract_hash_sha256 "$signing_receipt_export_contract_hash_sha256" \
    --arg signing_receipt_export_policy_hash_sha256 "$signing_receipt_export_policy_hash_sha256" \
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
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_schema_version: "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_denial_v1",
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_mode: "denied_signing_receipt_retention_cannot_export_query_observe_or_derive_authority_or_install",
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_gate: $source.gate,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_ready: $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_denial_ready,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_report_sha256: $source_report_sha256,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_contract_hash_sha256: $signing_receipt_export_contract_hash_sha256,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_policy_hash_sha256: $signing_receipt_export_policy_hash_sha256,
        minimum_required_samples: $min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_denial_ready: true,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_surface_count: $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_surface_count,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_denied_count: $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_denied_count,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_surface_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_attempt_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_denied_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_surfaces: $surfaces,
        denied_by_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability: ($surfaces | map(.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_surface)),
        allowed_next_actions: [
          {
            action: "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_denial_gate",
            status: "allowed_report_only_next_slice",
            registers_query: false,
            executes_query: false,
            records_export: false,
            writes_export: false,
            records_observability: false,
            exposes_dashboard: false,
            records_operator_summary: false,
            records_readback: false,
            derives_release_publication_authority: false,
            derives_activation_authority: false,
            installs_or_restarts: false,
            mutates_active_binary: false,
            invokes_provider: false,
            reads_credentials: false,
            sends_externally: false
          }
        ]
      }
      + zero_object([
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_allowed_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_accepted_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_recorded_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_persisted_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_materialized_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_filesystem_written_count",
        "signing_receipt_query_registered_count",
        "signing_receipt_query_executed_count",
        "signing_receipt_query_result_recorded_count",
        "signing_receipt_search_index_recorded_count",
        "signing_receipt_export_accepted_count",
        "signing_receipt_export_snapshot_recorded_count",
        "signing_receipt_export_file_written_count",
        "signing_receipt_export_stream_opened_count",
        "signing_receipt_observability_metric_recorded_count",
        "signing_receipt_observability_log_recorded_count",
        "signing_receipt_observability_trace_recorded_count",
        "signing_receipt_observability_event_recorded_count",
        "signing_receipt_dashboard_panel_recorded_count",
        "signing_receipt_alert_registered_count",
        "signing_receipt_slo_recorded_count",
        "signing_receipt_readback_surface_recorded_count",
        "signing_receipt_audit_view_recorded_count",
        "signing_receipt_ledger_observability_recorded_count",
        "signing_receipt_index_observability_recorded_count",
        "signing_receipt_delivery_observability_recorded_count",
        "external_signing_receipt_observability_recorded_count",
        "telegram_signing_receipt_observability_recorded_count",
        "release_publication_authority_from_signing_receipt_export_query_observability_derived_count",
        "activation_authority_from_signing_receipt_export_query_observability_derived_count",
        "install_from_signing_receipt_export_query_observability_executed_count",
        "service_restart_from_signing_receipt_export_query_observability_performed_count",
        "active_binary_from_signing_receipt_export_query_observability_mutated_count",
        "provider_invoked_count",
        "credential_read_count",
        "external_send_performed_count",
        "telegram_send_performed_count"
      ])
      + false_object([
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_allowed",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_accepted",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_recorded",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_persisted",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_materialized",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_filesystem_written",
        "signing_receipt_query_registered",
        "signing_receipt_query_executed",
        "signing_receipt_query_result_recorded",
        "signing_receipt_search_index_recorded",
        "signing_receipt_export_accepted",
        "signing_receipt_export_snapshot_recorded",
        "signing_receipt_export_file_written",
        "signing_receipt_export_stream_opened",
        "signing_receipt_observability_metric_recorded",
        "signing_receipt_observability_log_recorded",
        "signing_receipt_observability_trace_recorded",
        "signing_receipt_observability_event_recorded",
        "signing_receipt_dashboard_panel_recorded",
        "signing_receipt_alert_registered",
        "signing_receipt_slo_recorded",
        "signing_receipt_readback_surface_recorded",
        "signing_receipt_audit_view_recorded",
        "signing_receipt_ledger_observability_recorded",
        "signing_receipt_index_observability_recorded",
        "signing_receipt_delivery_observability_recorded",
        "external_signing_receipt_observability_recorded",
        "telegram_signing_receipt_observability_recorded",
        "operator_approval_from_signing_receipt_export_query_observability_derived",
        "release_publication_authority_from_signing_receipt_export_query_observability_derived",
        "activation_authority_from_signing_receipt_export_query_observability_derived",
        "install_from_signing_receipt_export_query_observability_executed",
        "service_restart_from_signing_receipt_export_query_observability_performed",
        "active_binary_from_signing_receipt_export_query_observability_mutated",
        "memory_store_write_performed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "external_send_performed",
        "telegram_send_performed",
        "public_status_claimed",
        "public_release_claimed",
        "public_ga_claimed"
      ])
      + {
        side_effects: false_object([
          "query_registered",
          "query_executed",
          "query_result_recorded",
          "query_result_persisted",
          "search_index_recorded",
          "search_index_persisted",
          "export_accepted",
          "export_snapshot_recorded",
          "export_snapshot_persisted",
          "export_file_written",
          "export_stream_opened",
          "observability_metric_recorded",
          "observability_log_recorded",
          "observability_trace_recorded",
          "observability_event_recorded",
          "dashboard_panel_recorded",
          "alert_registered",
          "slo_recorded",
          "operator_summary_recorded",
          "readback_surface_recorded",
          "audit_view_recorded",
          "ledger_observability_recorded",
          "index_observability_recorded",
          "delivery_observability_recorded",
          "external_signing_receipt_observability_recorded",
          "telegram_signing_receipt_observability_recorded",
          "operator_approval_from_signing_receipt_export_query_observability_derived",
          "release_publication_authority_from_signing_receipt_export_query_observability_derived",
          "activation_authority_from_signing_receipt_export_query_observability_derived",
          "install_from_signing_receipt_export_query_observability_executed",
          "service_restart_from_signing_receipt_export_query_observability_performed",
          "active_binary_from_signing_receipt_export_query_observability_mutated",
          "memory_store_write_performed",
          "live_kg_write_performed",
          "provider_invoked",
          "model_invoked",
          "credential_read",
          "secret_file_read",
          "telegram_send_performed",
          "external_send_performed",
          "public_status_claimed",
          "public_release_claimed",
          "public_ga_claimed",
          "filesystem_written"
        ])
      }
    '
)"

printf '%s\n' "$report"

jq -e '
  def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
  def false_fields($o; $fields): all($fields[]; $o[.] == false);

  .runtime == "hepta"
  and .status == "ready"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_denial_ready == true
  and .source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_ready == true
  and .source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_surface_count == 18
  and .source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_retention_expiry_gc_denied_count == 18
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_surface_count == 18
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_denied_count == 18
  and zero_fields(.; [
    "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_recorded_count",
    "signing_receipt_query_registered_count",
    "signing_receipt_query_executed_count",
    "signing_receipt_query_result_recorded_count",
    "signing_receipt_export_file_written_count",
    "signing_receipt_observability_metric_recorded_count",
    "signing_receipt_dashboard_panel_recorded_count",
    "release_publication_authority_from_signing_receipt_export_query_observability_derived_count",
    "activation_authority_from_signing_receipt_export_query_observability_derived_count",
    "install_from_signing_receipt_export_query_observability_executed_count",
    "active_binary_from_signing_receipt_export_query_observability_mutated_count",
    "provider_invoked_count",
    "credential_read_count"
  ])
  and false_fields(.; [
    "signing_receipt_query_registered",
    "signing_receipt_query_executed",
    "signing_receipt_export_file_written",
    "signing_receipt_observability_metric_recorded",
    "signing_receipt_dashboard_panel_recorded",
    "external_signing_receipt_observability_recorded",
    "telegram_signing_receipt_observability_recorded",
    "release_publication_authority_from_signing_receipt_export_query_observability_derived",
    "activation_authority_from_signing_receipt_export_query_observability_derived",
    "install_from_signing_receipt_export_query_observability_executed",
    "active_binary_from_signing_receipt_export_query_observability_mutated",
    "provider_invoked",
    "credential_read",
    "public_ga_claimed",
    "public_release_claimed"
  ])
  and (.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_surfaces | length) == 18
  and (.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_surfaces | all(
    .signing_receipt_export_query_observability_attempted == true
    and .signing_receipt_export_query_observability_allowed == false
    and .signing_receipt_export_query_observability_accepted == false
    and .signing_receipt_export_query_observability_noop_confirmed == true
    and .signing_receipt_query_registered == false
    and .signing_receipt_export_file_written == false
    and .signing_receipt_observability_metric_recorded == false
    and .release_publication_authority_from_signing_receipt_export_query_observability_derived == false
    and .activation_authority_from_signing_receipt_export_query_observability_derived == false
    and .install_from_signing_receipt_export_query_observability_executed == false
    and .active_binary_from_signing_receipt_export_query_observability_mutated == false
    and .provider_invoked == false
    and .credential_read == false
  ))
  and (.allowed_next_actions | any(
    .action == "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .registers_query == false
    and .records_export == false
    and .records_observability == false
    and .derives_release_publication_authority == false
    and .derives_activation_authority == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .reads_credentials == false
    and .sends_externally == false
  ))
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

echo "Hepta memory/intelligence/KG artifact signing terminal public claim delivery receipt artifact signing receipt signing receipt signing receipt export/query/observability denial gate passed" >&2
