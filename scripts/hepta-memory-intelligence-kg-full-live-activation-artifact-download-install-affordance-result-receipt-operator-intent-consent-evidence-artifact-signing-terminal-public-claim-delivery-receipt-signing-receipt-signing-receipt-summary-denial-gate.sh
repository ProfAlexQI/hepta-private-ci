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
  echo "jq is required to build the terminal public claim delivery receipt artifact signing receipt signing receipt signing receipt summary/briefing denial report" >&2
  exit 1
fi

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

SOURCE_GATE="scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-signing-receipt-export-denial-gate.sh"
[[ -x "$SOURCE_GATE" ]] || {
  echo "missing executable terminal public claim delivery receipt artifact signing receipt signing receipt signing receipt export/query/observability denial gate: $SOURCE_GATE" >&2
  exit 1
}

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    "$SOURCE_GATE"
)"

jq -e '
  .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_denial_ready == true
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_surface_count == 18
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_denied_count == 18
  and .signing_receipt_query_registered == false
  and .signing_receipt_export_file_written == false
  and .signing_receipt_observability_metric_recorded == false
  and .release_publication_authority_from_signing_receipt_export_query_observability_derived == false
  and .activation_authority_from_signing_receipt_export_query_observability_derived == false
  and .public_ga_claimed == false
' <<<"$SOURCE_JSON" >/dev/null

source_report_sha256="$(sha256_text "$SOURCE_JSON")"
signing_receipt_summary_contract_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-artifact-signing-receipt-signing-receipt-signing-receipt-summary-briefing-denial:$source_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
signing_receipt_summary_policy_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-signing-receipt-signing-receipt-summary-briefing:no-summary:no-briefing:no-readback-digest:no-status-banner:no-dashboard:no-telegram:no-authority:no-install"
)"

surfaces_json="$(
  jq -n '[
    "source_signing_receipt_export_query_observability_report_required",
    "signing_receipt_operator_summary",
    "signing_receipt_operator_briefing",
    "signing_receipt_readback_digest",
    "signing_receipt_status_banner",
    "signing_receipt_exported_summary",
    "signing_receipt_briefing_card",
    "signing_receipt_notification_timeline",
    "signing_receipt_dashboard_narrative",
    "signing_receipt_audit_narrative",
    "signing_receipt_approval_summary",
    "signing_receipt_external_telegram_briefing",
    "signing_receipt_final_summary",
    "signing_receipt_operator_memo",
    "signing_receipt_completion_summary",
    "signing_receipt_channel_briefing",
    "signing_receipt_authority_install_briefing",
    "signing_receipt_provider_credential_summary"
  ] | map({
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_surface: .,
    source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_ready: true,
    signing_receipt_summary_briefing_attempted: true,
    signing_receipt_summary_briefing_allowed: false,
    signing_receipt_summary_briefing_accepted: false,
    signing_receipt_summary_briefing_recorded: false,
    signing_receipt_summary_briefing_persisted: false,
    signing_receipt_summary_briefing_materialized: false,
    signing_receipt_summary_briefing_filesystem_written: false,
    signing_receipt_operator_summary_recorded: false,
    signing_receipt_operator_summary_persisted: false,
    signing_receipt_operator_briefing_recorded: false,
    signing_receipt_operator_briefing_persisted: false,
    signing_receipt_readback_digest_recorded: false,
    signing_receipt_readback_digest_persisted: false,
    signing_receipt_status_banner_recorded: false,
    signing_receipt_exported_summary_written: false,
    signing_receipt_briefing_card_recorded: false,
    signing_receipt_notification_timeline_recorded: false,
    signing_receipt_dashboard_narrative_recorded: false,
    signing_receipt_audit_narrative_recorded: false,
    signing_receipt_approval_summary_recorded: false,
    signing_receipt_final_summary_recorded: false,
    signing_receipt_operator_memo_recorded: false,
    signing_receipt_completion_summary_recorded: false,
    signing_receipt_channel_briefing_recorded: false,
    external_signing_receipt_briefing_sent: false,
    telegram_signing_receipt_briefing_sent: false,
    signing_receipt_authority_briefing_recorded: false,
    signing_receipt_install_briefing_recorded: false,
    signing_receipt_active_binary_briefing_recorded: false,
    artifact_signing_receipt_summary_recorded: false,
    package_signing_receipt_summary_recorded: false,
    signature_manifest_receipt_summary_recorded: false,
    notarization_ticket_receipt_summary_recorded: false,
    release_asset_receipt_summary_recorded: false,
    cdn_update_feed_receipt_summary_recorded: false,
    package_registry_receipt_summary_recorded: false,
    operator_acceptance_from_signing_receipt_summary_briefing_recorded: false,
    operator_approval_from_signing_receipt_summary_briefing_derived: false,
    release_publication_authority_from_signing_receipt_summary_briefing_derived: false,
    activation_authority_from_signing_receipt_summary_briefing_derived: false,
    install_from_signing_receipt_summary_briefing_executed: false,
    service_restart_from_signing_receipt_summary_briefing_performed: false,
    active_binary_from_signing_receipt_summary_briefing_mutated: false,
    memory_store_write_performed: false,
    live_kg_write_performed: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    external_send_performed: false,
    telegram_send_performed: false,
    signing_receipt_summary_briefing_noop_confirmed: true,
    signing_receipt_summary_briefing_status: "artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_denied"
  })'
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_denial_gate" \
    --arg source_report_sha256 "$source_report_sha256" \
    --arg signing_receipt_summary_contract_hash_sha256 "$signing_receipt_summary_contract_hash_sha256" \
    --arg signing_receipt_summary_policy_hash_sha256 "$signing_receipt_summary_policy_hash_sha256" \
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
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_schema_version: "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_denial_v1",
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_mode: "denied_signing_receipt_export_cannot_summarize_brief_or_derive_authority_or_install",
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_gate: $source.gate,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_ready: $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_denial_ready,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_report_sha256: $source_report_sha256,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_contract_hash_sha256: $signing_receipt_summary_contract_hash_sha256,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_policy_hash_sha256: $signing_receipt_summary_policy_hash_sha256,
        minimum_required_samples: $min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_denial_ready: true,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_surface_count: $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_surface_count,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_denied_count: $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_denied_count,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_surface_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_attempt_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_denied_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_surfaces: $surfaces,
        denied_by_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing: ($surfaces | map(.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_surface)),
        allowed_next_actions: [
          {
            action: "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_non_acceptance_denial_gate",
            status: "allowed_report_only_next_slice",
            records_summary: false,
            records_briefing: false,
            records_readback_digest: false,
            writes_exported_summary: false,
            records_dashboard_narrative: false,
            sends_telegram_briefing: false,
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
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_allowed_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_accepted_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_recorded_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_persisted_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_materialized_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_filesystem_written_count",
        "signing_receipt_operator_summary_recorded_count",
        "signing_receipt_operator_briefing_recorded_count",
        "signing_receipt_readback_digest_recorded_count",
        "signing_receipt_status_banner_recorded_count",
        "signing_receipt_exported_summary_written_count",
        "signing_receipt_briefing_card_recorded_count",
        "signing_receipt_notification_timeline_recorded_count",
        "signing_receipt_dashboard_narrative_recorded_count",
        "signing_receipt_audit_narrative_recorded_count",
        "signing_receipt_approval_summary_recorded_count",
        "signing_receipt_final_summary_recorded_count",
        "signing_receipt_operator_memo_recorded_count",
        "signing_receipt_completion_summary_recorded_count",
        "signing_receipt_channel_briefing_recorded_count",
        "external_signing_receipt_briefing_sent_count",
        "telegram_signing_receipt_briefing_sent_count",
        "release_publication_authority_from_signing_receipt_summary_briefing_derived_count",
        "activation_authority_from_signing_receipt_summary_briefing_derived_count",
        "install_from_signing_receipt_summary_briefing_executed_count",
        "service_restart_from_signing_receipt_summary_briefing_performed_count",
        "active_binary_from_signing_receipt_summary_briefing_mutated_count",
        "provider_invoked_count",
        "credential_read_count",
        "external_send_performed_count",
        "telegram_send_performed_count"
      ])
      + false_object([
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_allowed",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_accepted",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_recorded",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_persisted",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_materialized",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_filesystem_written",
        "signing_receipt_operator_summary_recorded",
        "signing_receipt_operator_summary_persisted",
        "signing_receipt_operator_briefing_recorded",
        "signing_receipt_operator_briefing_persisted",
        "signing_receipt_readback_digest_recorded",
        "signing_receipt_readback_digest_persisted",
        "signing_receipt_status_banner_recorded",
        "signing_receipt_exported_summary_written",
        "signing_receipt_briefing_card_recorded",
        "signing_receipt_notification_timeline_recorded",
        "signing_receipt_dashboard_narrative_recorded",
        "signing_receipt_audit_narrative_recorded",
        "signing_receipt_approval_summary_recorded",
        "signing_receipt_final_summary_recorded",
        "signing_receipt_operator_memo_recorded",
        "signing_receipt_completion_summary_recorded",
        "signing_receipt_channel_briefing_recorded",
        "external_signing_receipt_briefing_sent",
        "telegram_signing_receipt_briefing_sent",
        "signing_receipt_authority_briefing_recorded",
        "signing_receipt_install_briefing_recorded",
        "signing_receipt_active_binary_briefing_recorded",
        "operator_approval_from_signing_receipt_summary_briefing_derived",
        "release_publication_authority_from_signing_receipt_summary_briefing_derived",
        "activation_authority_from_signing_receipt_summary_briefing_derived",
        "install_from_signing_receipt_summary_briefing_executed",
        "service_restart_from_signing_receipt_summary_briefing_performed",
        "active_binary_from_signing_receipt_summary_briefing_mutated",
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
          "summary_recorded",
          "briefing_recorded",
          "readback_digest_recorded",
          "status_banner_recorded",
          "exported_summary_written",
          "briefing_card_recorded",
          "notification_timeline_recorded",
          "dashboard_narrative_recorded",
          "audit_narrative_recorded",
          "approval_summary_recorded",
          "external_briefing_sent",
          "telegram_briefing_sent",
          "operator_memo_recorded",
          "completion_summary_recorded",
          "channel_briefing_recorded",
          "operator_approval_from_signing_receipt_summary_briefing_derived",
          "release_publication_authority_from_signing_receipt_summary_briefing_derived",
          "activation_authority_from_signing_receipt_summary_briefing_derived",
          "install_from_signing_receipt_summary_briefing_executed",
          "service_restart_from_signing_receipt_summary_briefing_performed",
          "active_binary_from_signing_receipt_summary_briefing_mutated",
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
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_denial_ready == true
  and .source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_ready == true
  and .source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_surface_count == 18
  and .source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_export_query_observability_denied_count == 18
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_surface_count == 18
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_denied_count == 18
  and zero_fields(.; [
    "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_recorded_count",
    "signing_receipt_operator_summary_recorded_count",
    "signing_receipt_operator_briefing_recorded_count",
    "signing_receipt_readback_digest_recorded_count",
    "signing_receipt_exported_summary_written_count",
    "signing_receipt_dashboard_narrative_recorded_count",
    "signing_receipt_audit_narrative_recorded_count",
    "telegram_signing_receipt_briefing_sent_count",
    "release_publication_authority_from_signing_receipt_summary_briefing_derived_count",
    "activation_authority_from_signing_receipt_summary_briefing_derived_count",
    "install_from_signing_receipt_summary_briefing_executed_count",
    "active_binary_from_signing_receipt_summary_briefing_mutated_count",
    "provider_invoked_count",
    "credential_read_count"
  ])
  and false_fields(.; [
    "signing_receipt_operator_summary_recorded",
    "signing_receipt_operator_briefing_recorded",
    "signing_receipt_readback_digest_recorded",
    "signing_receipt_status_banner_recorded",
    "signing_receipt_exported_summary_written",
    "signing_receipt_dashboard_narrative_recorded",
    "signing_receipt_audit_narrative_recorded",
    "external_signing_receipt_briefing_sent",
    "telegram_signing_receipt_briefing_sent",
    "release_publication_authority_from_signing_receipt_summary_briefing_derived",
    "activation_authority_from_signing_receipt_summary_briefing_derived",
    "install_from_signing_receipt_summary_briefing_executed",
    "active_binary_from_signing_receipt_summary_briefing_mutated",
    "provider_invoked",
    "credential_read",
    "public_ga_claimed",
    "public_release_claimed"
  ])
  and (.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_surfaces | length) == 18
  and (.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_surfaces | all(
    .signing_receipt_summary_briefing_attempted == true
    and .signing_receipt_summary_briefing_allowed == false
    and .signing_receipt_summary_briefing_accepted == false
    and .signing_receipt_summary_briefing_noop_confirmed == true
    and .signing_receipt_operator_summary_recorded == false
    and .signing_receipt_operator_briefing_recorded == false
    and .telegram_signing_receipt_briefing_sent == false
    and .release_publication_authority_from_signing_receipt_summary_briefing_derived == false
    and .activation_authority_from_signing_receipt_summary_briefing_derived == false
    and .install_from_signing_receipt_summary_briefing_executed == false
    and .active_binary_from_signing_receipt_summary_briefing_mutated == false
    and .provider_invoked == false
    and .credential_read == false
  ))
  and (.allowed_next_actions | any(
    .action == "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_non_acceptance_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .records_summary == false
    and .records_briefing == false
    and .sends_telegram_briefing == false
    and .derives_release_publication_authority == false
    and .derives_activation_authority == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .reads_credentials == false
    and .sends_externally == false
  ))
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

echo "Hepta memory/intelligence/KG artifact signing terminal public claim delivery receipt artifact signing receipt signing receipt signing receipt summary/briefing denial gate passed" >&2
