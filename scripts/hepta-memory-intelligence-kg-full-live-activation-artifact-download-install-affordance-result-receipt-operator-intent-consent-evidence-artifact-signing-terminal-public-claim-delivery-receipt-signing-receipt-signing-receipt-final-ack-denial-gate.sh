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
  echo "jq is required to build the terminal public claim delivery receipt artifact signing receipt signing receipt signing receipt final acknowledgement denial report" >&2
  exit 1
fi

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

SOURCE_GATE="scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-signing-receipt-summary-denial-gate.sh"
[[ -x "$SOURCE_GATE" ]] || {
  echo "missing executable terminal public claim delivery receipt artifact signing receipt signing receipt signing receipt summary/briefing denial gate: $SOURCE_GATE" >&2
  exit 1
}

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    "$SOURCE_GATE"
)"

jq -e '
  .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_denial_ready == true
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_surface_count == 18
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_denied_count == 18
  and .signing_receipt_operator_summary_recorded == false
  and .signing_receipt_operator_briefing_recorded == false
  and .telegram_signing_receipt_briefing_sent == false
  and .release_publication_authority_from_signing_receipt_summary_briefing_derived == false
  and .activation_authority_from_signing_receipt_summary_briefing_derived == false
  and .public_ga_claimed == false
' <<<"$SOURCE_JSON" >/dev/null

source_report_sha256="$(sha256_text "$SOURCE_JSON")"
signing_receipt_final_ack_contract_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-artifact-signing-receipt-signing-receipt-signing-receipt-final-acknowledgement-non-acceptance-denial:$source_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
signing_receipt_final_ack_policy_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-signing-receipt-signing-receipt-final-ack:no-final-ack:no-received:no-confirmed:no-read:no-seen:no-response:no-authority:no-install:no-public-status"
)"

surfaces_json="$(
  jq -n '[
    "source_signing_receipt_summary_briefing_report_required",
    "signing_receipt_final_acknowledgement_claim",
    "signing_receipt_operator_received_claim",
    "signing_receipt_operator_confirmed_claim",
    "signing_receipt_operator_read_claim",
    "signing_receipt_operator_seen_claim",
    "signing_receipt_final_response_claim",
    "signing_receipt_completion_acknowledgement_claim",
    "signing_receipt_status_acknowledgement_claim",
    "signing_receipt_summary_acknowledgement_claim",
    "signing_receipt_briefing_acknowledgement_claim",
    "signing_receipt_readback_digest_acknowledgement_claim",
    "signing_receipt_dashboard_acknowledgement_claim",
    "signing_receipt_notification_acknowledgement_claim",
    "signing_receipt_external_telegram_acknowledgement_claim",
    "signing_receipt_release_publication_authority_acknowledgement_claim",
    "signing_receipt_activation_live_install_acknowledgement_claim",
    "signing_receipt_install_restart_active_binary_acknowledgement_claim"
  ] | map({
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_surface: .,
    source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_ready: true,
    signing_receipt_final_acknowledgement_attempted: true,
    signing_receipt_final_acknowledgement_allowed: false,
    signing_receipt_final_acknowledgement_accepted: false,
    signing_receipt_final_acknowledgement_recorded: false,
    signing_receipt_final_acknowledgement_persisted: false,
    signing_receipt_final_acknowledgement_materialized: false,
    signing_receipt_final_acknowledgement_filesystem_written: false,
    signing_receipt_final_acknowledgement_delivered: false,
    signing_receipt_operator_received_recorded: false,
    signing_receipt_operator_confirmed_recorded: false,
    signing_receipt_operator_read_recorded: false,
    signing_receipt_operator_seen_recorded: false,
    signing_receipt_final_response_recorded: false,
    signing_receipt_completion_acknowledgement_recorded: false,
    signing_receipt_status_acknowledgement_recorded: false,
    signing_receipt_summary_acknowledgement_recorded: false,
    signing_receipt_briefing_acknowledgement_recorded: false,
    signing_receipt_readback_digest_acknowledgement_recorded: false,
    signing_receipt_dashboard_acknowledgement_recorded: false,
    signing_receipt_notification_acknowledgement_recorded: false,
    signing_receipt_channel_acknowledgement_delivered: false,
    external_signing_receipt_acknowledgement_sent: false,
    telegram_signing_receipt_acknowledgement_sent: false,
    signing_receipt_acknowledgement_acceptance_recorded: false,
    operator_acceptance_from_signing_receipt_acknowledgement_recorded: false,
    operator_approval_from_signing_receipt_acknowledgement_derived: false,
    release_publication_authority_from_signing_receipt_acknowledgement_derived: false,
    activation_authority_from_signing_receipt_acknowledgement_derived: false,
    activation_command_from_signing_receipt_acknowledgement_derived: false,
    live_execution_from_signing_receipt_acknowledgement_allowed: false,
    install_from_signing_receipt_acknowledgement_executed: false,
    service_restart_from_signing_receipt_acknowledgement_performed: false,
    active_binary_from_signing_receipt_acknowledgement_mutated: false,
    result_receipt_from_signing_receipt_acknowledgement_recorded: false,
    memory_store_write_performed: false,
    live_kg_write_performed: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    telegram_send_performed: false,
    channel_send_performed: false,
    external_send_performed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    release_artifact_written: false,
    public_artifact_written: false,
    signing_receipt_final_acknowledgement_noop_confirmed: true,
    signing_receipt_final_acknowledgement_status: "artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_non_acceptance_denied"
  })'
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_non_acceptance_denial_gate" \
    --arg source_report_sha256 "$source_report_sha256" \
    --arg signing_receipt_final_ack_contract_hash_sha256 "$signing_receipt_final_ack_contract_hash_sha256" \
    --arg signing_receipt_final_ack_policy_hash_sha256 "$signing_receipt_final_ack_policy_hash_sha256" \
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
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_schema_version: "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_non_acceptance_denial_v1",
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_mode: "denied_signing_receipt_summary_cannot_become_final_acknowledgement_acceptance_status_authority_or_install",
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_gate: $source.gate,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_ready: $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_denial_ready,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_report_sha256: $source_report_sha256,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_contract_hash_sha256: $signing_receipt_final_ack_contract_hash_sha256,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_policy_hash_sha256: $signing_receipt_final_ack_policy_hash_sha256,
        minimum_required_samples: $min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_non_acceptance_denial_ready: true,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_surface_count: $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_surface_count,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_denied_count: $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_denied_count,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_surface_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_attempt_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_denied_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_surfaces: $surfaces,
        denied_by_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement: ($surfaces | map(.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_surface)),
        allowed_next_actions: [
          {
            action: "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_decision_status_promotion_denial_gate",
            status: "allowed_report_only_next_slice",
            records_final_acknowledgement: false,
            records_operator_received: false,
            records_operator_read: false,
            records_terminal_decision: false,
            records_status_promotion: false,
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
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_allowed_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_accepted_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_recorded_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_persisted_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_materialized_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_filesystem_written_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_delivered_count",
        "signing_receipt_operator_received_recorded_count",
        "signing_receipt_operator_confirmed_recorded_count",
        "signing_receipt_operator_read_recorded_count",
        "signing_receipt_operator_seen_recorded_count",
        "signing_receipt_final_response_recorded_count",
        "signing_receipt_completion_acknowledgement_recorded_count",
        "signing_receipt_status_acknowledgement_recorded_count",
        "signing_receipt_summary_acknowledgement_recorded_count",
        "signing_receipt_briefing_acknowledgement_recorded_count",
        "signing_receipt_readback_digest_acknowledgement_recorded_count",
        "signing_receipt_dashboard_acknowledgement_recorded_count",
        "signing_receipt_notification_acknowledgement_recorded_count",
        "external_signing_receipt_acknowledgement_sent_count",
        "telegram_signing_receipt_acknowledgement_sent_count",
        "signing_receipt_acknowledgement_acceptance_recorded_count",
        "operator_approval_from_signing_receipt_acknowledgement_derived_count",
        "release_publication_authority_from_signing_receipt_acknowledgement_derived_count",
        "activation_authority_from_signing_receipt_acknowledgement_derived_count",
        "install_from_signing_receipt_acknowledgement_executed_count",
        "active_binary_from_signing_receipt_acknowledgement_mutated_count",
        "provider_invoked_count",
        "credential_read_count",
        "external_send_performed_count"
      ])
      + false_object([
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_accepted",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_recorded",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_persisted",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_materialized",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_filesystem_written",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_delivered",
        "signing_receipt_operator_received_recorded",
        "signing_receipt_operator_confirmed_recorded",
        "signing_receipt_operator_read_recorded",
        "signing_receipt_operator_seen_recorded",
        "signing_receipt_final_response_recorded",
        "signing_receipt_completion_acknowledgement_recorded",
        "signing_receipt_status_acknowledgement_recorded",
        "signing_receipt_summary_acknowledgement_recorded",
        "signing_receipt_briefing_acknowledgement_recorded",
        "signing_receipt_readback_digest_acknowledgement_recorded",
        "signing_receipt_dashboard_acknowledgement_recorded",
        "signing_receipt_notification_acknowledgement_recorded",
        "external_signing_receipt_acknowledgement_sent",
        "telegram_signing_receipt_acknowledgement_sent",
        "signing_receipt_acknowledgement_acceptance_recorded",
        "operator_approval_from_signing_receipt_acknowledgement_derived",
        "release_publication_authority_from_signing_receipt_acknowledgement_derived",
        "activation_authority_from_signing_receipt_acknowledgement_derived",
        "activation_command_from_signing_receipt_acknowledgement_derived",
        "live_execution_from_signing_receipt_acknowledgement_allowed",
        "install_from_signing_receipt_acknowledgement_executed",
        "service_restart_from_signing_receipt_acknowledgement_performed",
        "active_binary_from_signing_receipt_acknowledgement_mutated",
        "result_receipt_from_signing_receipt_acknowledgement_recorded",
        "public_release_claimed",
        "public_ga_claimed",
        "memory_store_write_performed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "external_send_performed",
        "telegram_send_performed"
      ])
      + {
        side_effects: false_object([
          "final_acknowledgement_recorded",
          "operator_received_recorded",
          "operator_confirmed_recorded",
          "operator_read_recorded",
          "operator_seen_recorded",
          "final_response_recorded",
          "completion_acknowledgement_recorded",
          "status_acknowledgement_recorded",
          "summary_acknowledgement_recorded",
          "briefing_acknowledgement_recorded",
          "readback_digest_acknowledgement_recorded",
          "dashboard_acknowledgement_recorded",
          "notification_acknowledgement_recorded",
          "external_acknowledgement_sent",
          "telegram_acknowledgement_sent",
          "release_publication_authority_from_signing_receipt_acknowledgement_derived",
          "activation_authority_from_signing_receipt_acknowledgement_derived",
          "install_from_signing_receipt_acknowledgement_executed",
          "active_binary_from_signing_receipt_acknowledgement_mutated",
          "memory_store_write_performed",
          "live_kg_write_performed",
          "provider_invoked",
          "model_invoked",
          "credential_read",
          "secret_file_read",
          "telegram_send_performed",
          "external_send_performed",
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
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_non_acceptance_denial_ready == true
  and .source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_ready == true
  and .source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_surface_count == 18
  and .source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_summary_briefing_denied_count == 18
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_surface_count == 18
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_denied_count == 18
  and zero_fields(.; [
    "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_recorded_count",
    "signing_receipt_operator_received_recorded_count",
    "signing_receipt_operator_read_recorded_count",
    "signing_receipt_completion_acknowledgement_recorded_count",
    "telegram_signing_receipt_acknowledgement_sent_count",
    "release_publication_authority_from_signing_receipt_acknowledgement_derived_count",
    "activation_authority_from_signing_receipt_acknowledgement_derived_count",
    "install_from_signing_receipt_acknowledgement_executed_count",
    "active_binary_from_signing_receipt_acknowledgement_mutated_count",
    "provider_invoked_count",
    "credential_read_count"
  ])
  and false_fields(.; [
    "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_recorded",
    "signing_receipt_operator_received_recorded",
    "signing_receipt_operator_read_recorded",
    "signing_receipt_completion_acknowledgement_recorded",
    "external_signing_receipt_acknowledgement_sent",
    "telegram_signing_receipt_acknowledgement_sent",
    "release_publication_authority_from_signing_receipt_acknowledgement_derived",
    "activation_authority_from_signing_receipt_acknowledgement_derived",
    "install_from_signing_receipt_acknowledgement_executed",
    "active_binary_from_signing_receipt_acknowledgement_mutated",
    "provider_invoked",
    "credential_read",
    "public_ga_claimed",
    "public_release_claimed"
  ])
  and (.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_surfaces | length) == 18
  and (.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_final_acknowledgement_surfaces | all(
    .signing_receipt_final_acknowledgement_attempted == true
    and .signing_receipt_final_acknowledgement_allowed == false
    and .signing_receipt_final_acknowledgement_recorded == false
    and .signing_receipt_final_acknowledgement_noop_confirmed == true
    and .signing_receipt_operator_received_recorded == false
    and .signing_receipt_operator_read_recorded == false
    and .telegram_signing_receipt_acknowledgement_sent == false
    and .release_publication_authority_from_signing_receipt_acknowledgement_derived == false
    and .activation_authority_from_signing_receipt_acknowledgement_derived == false
    and .install_from_signing_receipt_acknowledgement_executed == false
    and .active_binary_from_signing_receipt_acknowledgement_mutated == false
    and .credential_read == false
  ))
  and (.allowed_next_actions | any(
    .action == "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_decision_status_promotion_denial_gate"
    and .records_final_acknowledgement == false
    and .records_terminal_decision == false
    and .records_status_promotion == false
    and .derives_release_publication_authority == false
    and .derives_activation_authority == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .reads_credentials == false
    and .sends_externally == false
  ))
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

echo "Hepta memory/intelligence/KG artifact signing terminal public claim delivery receipt artifact signing receipt signing receipt signing receipt final acknowledgement denial gate passed" >&2
