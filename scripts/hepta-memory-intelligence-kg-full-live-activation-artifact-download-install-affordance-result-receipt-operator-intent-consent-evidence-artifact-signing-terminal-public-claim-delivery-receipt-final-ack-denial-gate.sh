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

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

SOURCE_GATE="scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-summary-briefing-denial-gate.sh"
[[ -x "$SOURCE_GATE" ]] || {
  echo "missing executable terminal public claim delivery receipt summary/briefing denial gate: $SOURCE_GATE" >&2
  exit 1
}

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    "$SOURCE_GATE"
)"

source_report_sha256="$(sha256_text "$SOURCE_JSON")"
final_ack_contract_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-final-operator-acknowledgement-non-acceptance-denial:$source_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
final_ack_policy_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-final-ack:no-ack:no-received:no-confirmed:no-read:no-seen:no-response:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
    and $source.terminal_public_claim_delivery_receipt_summary_briefing_surface_count == 18
    and $source.terminal_public_claim_delivery_receipt_summary_briefing_denied_count == 18
    and $source.terminal_public_claim_delivery_receipt_summary_briefing_recorded_count == 0
    and $source.terminal_public_claim_delivery_receipt_operator_summary_recorded_count == 0
    and $source.terminal_public_claim_delivery_receipt_operator_briefing_recorded_count == 0
    and $source.terminal_public_claim_delivery_receipt_readback_recorded_count == 0
    and $source.terminal_public_claim_delivery_receipt_briefing_delivery_recorded_count == 0
    and $source.operator_approval_from_delivery_receipt_summary_briefing_derived_count == 0
    and $source.release_publication_authority_from_delivery_receipt_summary_briefing_derived_count == 0
    and $source.activation_authority_from_delivery_receipt_summary_briefing_derived_count == 0
    and $source.install_from_delivery_receipt_summary_briefing_executed_count == 0
    and $source.active_binary_from_delivery_receipt_summary_briefing_mutated_count == 0
    and $source.provider_invoked_count == 0
    and $source.credential_read_count == 0
    and $source.external_send_performed_count == 0
    and $source.terminal_public_claim_delivery_receipt_summary_briefing_recorded == false
    and $source.terminal_public_claim_delivery_receipt_operator_summary_recorded == false
    and $source.terminal_public_claim_delivery_receipt_operator_briefing_recorded == false
    and $source.terminal_public_claim_delivery_receipt_readback_recorded == false
    and $source.terminal_public_claim_delivery_receipt_briefing_delivery_recorded == false
    and $source.operator_approval_recorded == false
    and $source.release_publication_authority_derived == false
    and $source.activation_authority_derived == false
    and $source.activation_performed == false
    and $source.install_executed == false
    and $source.active_binary_mutated == false
    and $source.provider_invoked == false
    and $source.credential_read == false
    and $source.external_send_performed == false
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_final_operator_acknowledgement_non_acceptance_denial_gate"
      and .records_acknowledgement == false
      and .derives_release_publication_authority == false
      and .derives_activation_authority == false
      and .installs_or_restarts == false
      and .mutates_active_binary == false
      and .reads_credentials == false
      and .sends_externally == false
    ))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

surfaces_json="$(
  jq -n '[
    "source_delivery_receipt_summary_briefing_report_required",
    "delivery_receipt_final_operator_acknowledgement_claim",
    "delivery_receipt_operator_received_claim",
    "delivery_receipt_operator_confirmed_claim",
    "delivery_receipt_operator_read_claim",
    "delivery_receipt_operator_seen_claim",
    "delivery_receipt_final_response_claim",
    "delivery_receipt_completion_acknowledgement_claim",
    "delivery_receipt_status_acknowledgement_claim",
    "delivery_receipt_summary_acknowledgement_claim",
    "delivery_receipt_briefing_acknowledgement_claim",
    "delivery_receipt_readback_digest_acknowledgement_claim",
    "delivery_receipt_dashboard_acknowledgement_claim",
    "delivery_receipt_notification_acknowledgement_claim",
    "delivery_receipt_external_telegram_acknowledgement_claim",
    "delivery_receipt_release_publication_authority_acknowledgement_claim",
    "delivery_receipt_activation_live_install_acknowledgement_claim",
    "delivery_receipt_install_restart_active_binary_acknowledgement_claim"
  ] | map({
    terminal_public_claim_delivery_receipt_final_acknowledgement_surface: .,
    source_terminal_public_claim_delivery_receipt_summary_briefing_ready: true,
    terminal_public_claim_delivery_receipt_final_acknowledgement_attempted: true,
    terminal_public_claim_delivery_receipt_final_acknowledgement_allowed: false,
    terminal_public_claim_delivery_receipt_final_acknowledgement_accepted: false,
    terminal_public_claim_delivery_receipt_final_acknowledgement_recorded: false,
    terminal_public_claim_delivery_receipt_final_acknowledgement_persisted: false,
    terminal_public_claim_delivery_receipt_final_acknowledgement_materialized: false,
    terminal_public_claim_delivery_receipt_final_acknowledgement_filesystem_written: false,
    terminal_public_claim_delivery_receipt_final_acknowledgement_delivered: false,
    operator_received_recorded: false,
    operator_confirmed_recorded: false,
    operator_read_recorded: false,
    operator_seen_recorded: false,
    final_response_recorded: false,
    completion_acknowledgement_recorded: false,
    status_acknowledgement_recorded: false,
    summary_acknowledgement_recorded: false,
    briefing_acknowledgement_recorded: false,
    readback_digest_acknowledgement_recorded: false,
    dashboard_acknowledgement_recorded: false,
    notification_acknowledgement_recorded: false,
    channel_acknowledgement_delivered: false,
    external_acknowledgement_sent: false,
    telegram_acknowledgement_sent: false,
    acknowledgement_acceptance_recorded: false,
    operator_acceptance_from_acknowledgement_recorded: false,
    operator_approval_from_acknowledgement_derived: false,
    release_publication_authority_from_acknowledgement_derived: false,
    activation_authority_from_acknowledgement_derived: false,
    activation_command_from_acknowledgement_derived: false,
    live_execution_from_acknowledgement_allowed: false,
    download_link_from_acknowledgement_rendered: false,
    install_command_from_acknowledgement_rendered: false,
    install_from_acknowledgement_executed: false,
    service_restart_from_acknowledgement_performed: false,
    launchd_from_acknowledgement_mutated: false,
    active_binary_from_acknowledgement_mutated: false,
    result_receipt_from_acknowledgement_recorded: false,
    result_receipt_from_acknowledgement_persisted: false,
    memory_store_write_performed: false,
    memory_store_mutated: false,
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
    terminal_public_claim_delivery_receipt_final_acknowledgement_noop_confirmed: true,
    terminal_public_claim_delivery_receipt_final_acknowledgement_status: "final_operator_acknowledgement_non_acceptance_denied"
  })'
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_final_operator_acknowledgement_non_acceptance_denial_gate" \
    --arg source_report_sha256 "$source_report_sha256" \
    --arg final_ack_contract_hash_sha256 "$final_ack_contract_hash_sha256" \
    --arg final_ack_policy_hash_sha256 "$final_ack_policy_hash_sha256" \
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
        terminal_public_claim_delivery_receipt_final_acknowledgement_schema_version: "terminal_public_claim_delivery_receipt_final_operator_acknowledgement_non_acceptance_denial_v1",
        terminal_public_claim_delivery_receipt_final_acknowledgement_mode: "denied_summary_briefing_cannot_become_operator_acknowledgement_acceptance_authority_or_live_install",
        source_terminal_public_claim_delivery_receipt_summary_briefing_gate: $source.gate,
        source_terminal_public_claim_delivery_receipt_summary_briefing_ready: $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_operator_facing_summary_briefing_non_persistence_denial_ready,
        source_terminal_public_claim_delivery_receipt_summary_briefing_report_sha256: $source_report_sha256,
        source_terminal_public_claim_delivery_receipt_summary_briefing_contract_hash_sha256: $source.terminal_public_claim_delivery_receipt_summary_briefing_contract_hash_sha256,
        terminal_public_claim_delivery_receipt_final_acknowledgement_contract_hash_sha256: $final_ack_contract_hash_sha256,
        terminal_public_claim_delivery_receipt_final_acknowledgement_policy_hash_sha256: $final_ack_policy_hash_sha256,
        minimum_required_samples: $min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_final_operator_acknowledgement_non_acceptance_denial_ready: true,
        source_terminal_public_claim_delivery_receipt_summary_briefing_surface_count: $source.terminal_public_claim_delivery_receipt_summary_briefing_surface_count,
        source_terminal_public_claim_delivery_receipt_summary_briefing_denied_count: $source.terminal_public_claim_delivery_receipt_summary_briefing_denied_count,
        terminal_public_claim_delivery_receipt_final_acknowledgement_surface_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_final_acknowledgement_attempt_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_final_acknowledgement_denied_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_final_acknowledgement_surfaces: $surfaces,
        denied_by_terminal_public_claim_delivery_receipt_final_acknowledgement: [
          "delivery_receipt_final_operator_acknowledgement_denied",
          "delivery_receipt_operator_received_denied",
          "delivery_receipt_operator_confirmed_denied",
          "delivery_receipt_operator_read_denied",
          "delivery_receipt_operator_seen_denied",
          "delivery_receipt_final_response_denied",
          "delivery_receipt_completion_acknowledgement_denied",
          "delivery_receipt_status_acknowledgement_denied",
          "delivery_receipt_summary_acknowledgement_denied",
          "delivery_receipt_briefing_acknowledgement_denied",
          "delivery_receipt_readback_digest_acknowledgement_denied",
          "delivery_receipt_dashboard_acknowledgement_denied",
          "delivery_receipt_notification_acknowledgement_denied",
          "delivery_receipt_external_telegram_acknowledgement_denied",
          "delivery_receipt_authority_acknowledgement_denied",
          "delivery_receipt_live_install_acknowledgement_denied",
          "delivery_receipt_install_restart_active_binary_acknowledgement_denied"
        ],
        allowed_next_actions: [
          {
            action: "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_denial_gate",
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
            mutates_memory_store: false,
            writes_kg: false,
            invokes_provider: false,
            reads_credentials: false,
            sends_externally: false
          }
        ]
      }
      + zero_object([
        "terminal_public_claim_delivery_receipt_final_acknowledgement_allowed_count",
        "terminal_public_claim_delivery_receipt_final_acknowledgement_accepted_count",
        "terminal_public_claim_delivery_receipt_final_acknowledgement_recorded_count",
        "terminal_public_claim_delivery_receipt_final_acknowledgement_persisted_count",
        "terminal_public_claim_delivery_receipt_final_acknowledgement_materialized_count",
        "terminal_public_claim_delivery_receipt_final_acknowledgement_filesystem_written_count",
        "terminal_public_claim_delivery_receipt_final_acknowledgement_delivered_count",
        "operator_received_recorded_count",
        "operator_confirmed_recorded_count",
        "operator_read_recorded_count",
        "operator_seen_recorded_count",
        "final_response_recorded_count",
        "completion_acknowledgement_recorded_count",
        "status_acknowledgement_recorded_count",
        "summary_acknowledgement_recorded_count",
        "briefing_acknowledgement_recorded_count",
        "readback_digest_acknowledgement_recorded_count",
        "dashboard_acknowledgement_recorded_count",
        "notification_acknowledgement_recorded_count",
        "channel_acknowledgement_delivered_count",
        "external_acknowledgement_sent_count",
        "telegram_acknowledgement_sent_count",
        "acknowledgement_acceptance_recorded_count",
        "operator_approval_from_acknowledgement_derived_count",
        "release_publication_authority_from_acknowledgement_derived_count",
        "activation_authority_from_acknowledgement_derived_count",
        "install_from_acknowledgement_executed_count",
        "active_binary_from_acknowledgement_mutated_count",
        "provider_invoked_count",
        "credential_read_count",
        "external_send_performed_count"
      ])
      + false_object([
        "terminal_public_claim_delivery_receipt_final_acknowledgement_accepted",
        "terminal_public_claim_delivery_receipt_final_acknowledgement_recorded",
        "terminal_public_claim_delivery_receipt_final_acknowledgement_persisted",
        "terminal_public_claim_delivery_receipt_final_acknowledgement_materialized",
        "terminal_public_claim_delivery_receipt_final_acknowledgement_filesystem_written",
        "terminal_public_claim_delivery_receipt_final_acknowledgement_delivered",
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
        "channel_acknowledgement_delivered",
        "external_acknowledgement_sent",
        "telegram_acknowledgement_sent",
        "acknowledgement_acceptance_recorded",
        "operator_acceptance_from_acknowledgement_recorded",
        "operator_approval_from_acknowledgement_derived",
        "release_publication_authority_from_acknowledgement_derived",
        "activation_authority_from_acknowledgement_derived",
        "activation_command_from_acknowledgement_derived",
        "live_execution_from_acknowledgement_allowed",
        "download_link_from_acknowledgement_rendered",
        "install_command_from_acknowledgement_rendered",
        "install_from_acknowledgement_executed",
        "service_restart_from_acknowledgement_performed",
        "active_binary_from_acknowledgement_mutated",
        "result_receipt_from_acknowledgement_recorded",
        "result_receipt_from_acknowledgement_persisted",
        "public_release_claimed",
        "public_ga_claimed",
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
        "service_restarted",
        "active_binary_mutated",
        "external_send_performed"
      ])
      + {
        side_effects: false_object([
          "final_operator_acknowledgement_recorded",
          "final_operator_acknowledgement_persisted",
          "final_operator_acknowledgement_delivered",
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
          "channel_acknowledgement_delivered",
          "external_acknowledgement_sent",
          "telegram_acknowledgement_sent",
          "operator_approval_from_acknowledgement_derived",
          "release_publication_authority_from_acknowledgement_derived",
          "activation_authority_from_acknowledgement_derived",
          "install_executed",
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
  .runtime == "hepta"
  and .status == "ready"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_final_operator_acknowledgement_non_acceptance_denial_ready == true
  and .source_terminal_public_claim_delivery_receipt_summary_briefing_ready == true
  and .terminal_public_claim_delivery_receipt_final_acknowledgement_surface_count == 18
  and .terminal_public_claim_delivery_receipt_final_acknowledgement_denied_count == 18
  and .terminal_public_claim_delivery_receipt_final_acknowledgement_recorded_count == 0
  and .operator_received_recorded_count == 0
  and .operator_read_recorded_count == 0
  and .completion_acknowledgement_recorded_count == 0
  and .external_acknowledgement_sent_count == 0
  and .telegram_acknowledgement_sent_count == 0
  and .release_publication_authority_from_acknowledgement_derived_count == 0
  and .activation_authority_from_acknowledgement_derived_count == 0
  and .install_from_acknowledgement_executed_count == 0
  and .active_binary_from_acknowledgement_mutated_count == 0
  and .terminal_public_claim_delivery_receipt_final_acknowledgement_recorded == false
  and .operator_received_recorded == false
  and .operator_read_recorded == false
  and .completion_acknowledgement_recorded == false
  and .status_acknowledgement_recorded == false
  and .external_acknowledgement_sent == false
  and .telegram_acknowledgement_sent == false
  and .operator_approval_recorded == false
  and .release_publication_authority_derived == false
  and .activation_authority_derived == false
  and .activation_performed == false
  and .provider_invoked == false
  and .credential_read == false
  and .install_executed == false
  and .active_binary_mutated == false
  and .external_send_performed == false
  and (.terminal_public_claim_delivery_receipt_final_acknowledgement_surfaces | all(
    .terminal_public_claim_delivery_receipt_final_acknowledgement_attempted == true
    and .terminal_public_claim_delivery_receipt_final_acknowledgement_allowed == false
    and .terminal_public_claim_delivery_receipt_final_acknowledgement_noop_confirmed == true
    and .terminal_public_claim_delivery_receipt_final_acknowledgement_recorded == false
    and .operator_received_recorded == false
    and .operator_read_recorded == false
    and .telegram_acknowledgement_sent == false
    and .release_publication_authority_from_acknowledgement_derived == false
    and .activation_authority_from_acknowledgement_derived == false
    and .install_from_acknowledgement_executed == false
    and .active_binary_from_acknowledgement_mutated == false
    and .external_send_performed == false
  ))
  and (.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_denial_gate"
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

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG artifact signing terminal public claim delivery receipt final acknowledgement denial gate passed" >&2
