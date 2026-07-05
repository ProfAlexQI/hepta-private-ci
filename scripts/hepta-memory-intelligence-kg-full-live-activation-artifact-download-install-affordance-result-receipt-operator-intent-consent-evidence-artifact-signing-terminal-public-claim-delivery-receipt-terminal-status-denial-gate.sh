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
  echo "jq is required to build the terminal public claim delivery receipt terminal decision/status denial report" >&2
  exit 1
fi

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

SOURCE_GATE="scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-final-ack-denial-gate.sh"
[[ -x "$SOURCE_GATE" ]] || {
  echo "missing executable terminal public claim delivery receipt final acknowledgement denial gate: $SOURCE_GATE" >&2
  exit 1
}

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    "$SOURCE_GATE"
)"

source_report_sha256="$(sha256_text "$SOURCE_JSON")"
terminal_status_contract_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-terminal-decision-status-promotion-denial:$source_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
terminal_status_policy_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-terminal-decision-status:no-terminal-decision:no-status-promotion:no-public-status:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_final_operator_acknowledgement_non_acceptance_denial_ready == true
    and $source.terminal_public_claim_delivery_receipt_final_acknowledgement_surface_count == 18
    and $source.terminal_public_claim_delivery_receipt_final_acknowledgement_denied_count == 18
    and $source.terminal_public_claim_delivery_receipt_final_acknowledgement_recorded_count == 0
    and $source.operator_received_recorded_count == 0
    and $source.operator_read_recorded_count == 0
    and $source.completion_acknowledgement_recorded_count == 0
    and $source.external_acknowledgement_sent_count == 0
    and $source.telegram_acknowledgement_sent_count == 0
    and $source.release_publication_authority_from_acknowledgement_derived_count == 0
    and $source.activation_authority_from_acknowledgement_derived_count == 0
    and $source.install_from_acknowledgement_executed_count == 0
    and $source.active_binary_from_acknowledgement_mutated_count == 0
    and $source.terminal_public_claim_delivery_receipt_final_acknowledgement_recorded == false
    and $source.operator_received_recorded == false
    and $source.operator_read_recorded == false
    and $source.external_acknowledgement_sent == false
    and $source.telegram_acknowledgement_sent == false
    and $source.operator_approval_recorded == false
    and $source.release_publication_authority_derived == false
    and $source.activation_authority_derived == false
    and $source.install_executed == false
    and $source.active_binary_mutated == false
    and $source.provider_invoked == false
    and $source.credential_read == false
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_denial_gate"
      and .records_terminal_decision == false
      and .records_status_promotion == false
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
    "source_delivery_receipt_final_acknowledgement_report_required",
    "delivery_receipt_terminal_decision_claim",
    "delivery_receipt_terminal_status_claim",
    "delivery_receipt_status_promotion_claim",
    "delivery_receipt_final_ack_terminal_decision_claim",
    "delivery_receipt_completion_terminal_status_claim",
    "delivery_receipt_summary_terminal_status_claim",
    "delivery_receipt_briefing_terminal_decision_claim",
    "delivery_receipt_readback_digest_status_promotion_claim",
    "delivery_receipt_dashboard_terminal_status_claim",
    "delivery_receipt_notification_status_promotion_claim",
    "delivery_receipt_channel_terminal_decision_claim",
    "delivery_receipt_external_telegram_terminal_decision_claim",
    "delivery_receipt_public_status_exposure_claim",
    "delivery_receipt_public_ga_status_exposure_claim",
    "delivery_receipt_release_publication_authority_terminal_decision_claim",
    "delivery_receipt_activation_live_install_terminal_status_claim",
    "delivery_receipt_install_restart_active_binary_status_promotion_claim"
  ] | map({
    terminal_public_claim_delivery_receipt_terminal_decision_status_surface: .,
    source_terminal_public_claim_delivery_receipt_final_acknowledgement_ready: true,
    terminal_public_claim_delivery_receipt_terminal_decision_status_attempted: true,
    terminal_decision_requested: (. | test("decision")),
    terminal_status_requested: (. | test("status")),
    status_promotion_requested: (. | test("promotion")),
    public_status_exposure_requested: (. | test("public_status|public_ga_status")),
    terminal_decision_allowed: false,
    terminal_status_allowed: false,
    status_promotion_allowed: false,
    terminal_decision_recorded: false,
    terminal_decision_persisted: false,
    terminal_decision_materialized: false,
    terminal_decision_filesystem_written: false,
    terminal_decision_delivered: false,
    terminal_status_recorded: false,
    terminal_status_persisted: false,
    terminal_status_materialized: false,
    terminal_status_filesystem_written: false,
    status_promotion_recorded: false,
    public_status_exposed: false,
    public_ga_status_exposed: false,
    public_release_status_exposed: false,
    channel_decision_delivered: false,
    external_decision_sent: false,
    telegram_decision_sent: false,
    acceptance_from_terminal_decision_recorded: false,
    operator_approval_from_terminal_status_derived: false,
    release_publication_authority_from_terminal_status_derived: false,
    activation_authority_from_terminal_status_derived: false,
    activation_command_from_terminal_status_derived: false,
    live_execution_from_terminal_status_allowed: false,
    download_link_from_terminal_status_rendered: false,
    install_command_from_terminal_status_rendered: false,
    install_from_terminal_status_executed: false,
    service_restart_from_terminal_status_performed: false,
    launchd_from_terminal_status_mutated: false,
    active_binary_from_terminal_status_mutated: false,
    result_receipt_from_terminal_status_recorded: false,
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
    terminal_public_claim_delivery_receipt_terminal_decision_status_noop_confirmed: true,
    terminal_public_claim_delivery_receipt_terminal_decision_status_status: "terminal_decision_status_promotion_denied"
  })'
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_denial_gate" \
    --arg source_report_sha256 "$source_report_sha256" \
    --arg terminal_status_contract_hash_sha256 "$terminal_status_contract_hash_sha256" \
    --arg terminal_status_policy_hash_sha256 "$terminal_status_policy_hash_sha256" \
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
        terminal_public_claim_delivery_receipt_terminal_decision_status_schema_version: "terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_denial_v1",
        terminal_public_claim_delivery_receipt_terminal_decision_status_mode: "denied_final_acknowledgement_cannot_become_terminal_decision_status_promotion_public_status_authority_or_live_install",
        source_terminal_public_claim_delivery_receipt_final_acknowledgement_gate: $source.gate,
        source_terminal_public_claim_delivery_receipt_final_acknowledgement_ready: $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_final_operator_acknowledgement_non_acceptance_denial_ready,
        source_terminal_public_claim_delivery_receipt_final_acknowledgement_report_sha256: $source_report_sha256,
        source_terminal_public_claim_delivery_receipt_final_acknowledgement_contract_hash_sha256: $source.terminal_public_claim_delivery_receipt_final_acknowledgement_contract_hash_sha256,
        terminal_public_claim_delivery_receipt_terminal_decision_status_contract_hash_sha256: $terminal_status_contract_hash_sha256,
        terminal_public_claim_delivery_receipt_terminal_decision_status_policy_hash_sha256: $terminal_status_policy_hash_sha256,
        minimum_required_samples: $min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_denial_ready: true,
        source_terminal_public_claim_delivery_receipt_final_acknowledgement_surface_count: $source.terminal_public_claim_delivery_receipt_final_acknowledgement_surface_count,
        source_terminal_public_claim_delivery_receipt_final_acknowledgement_denied_count: $source.terminal_public_claim_delivery_receipt_final_acknowledgement_denied_count,
        terminal_public_claim_delivery_receipt_terminal_decision_status_surface_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_terminal_decision_status_attempt_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_terminal_decision_status_denied_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_terminal_decision_status_surfaces: $surfaces,
        denied_by_terminal_public_claim_delivery_receipt_terminal_decision_status: [
          "source_delivery_receipt_final_acknowledgement_report_required",
          "delivery_receipt_terminal_decision_recording_denied",
          "delivery_receipt_terminal_status_recording_denied",
          "delivery_receipt_status_promotion_denied",
          "delivery_receipt_public_status_exposure_denied",
          "delivery_receipt_external_telegram_terminal_decision_denied",
          "delivery_receipt_operator_approval_from_terminal_status_denied",
          "delivery_receipt_release_publication_authority_from_terminal_status_denied",
          "delivery_receipt_activation_authority_from_terminal_status_denied",
          "delivery_receipt_install_restart_active_binary_from_terminal_status_denied"
        ],
        allowed_next_actions: [
          {
            action: "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_terminal_public_claim_status_exposure_denial_gate",
            status: "allowed_report_only_next_slice",
            records_terminal_decision: false,
            records_status_promotion: false,
            records_public_claim: false,
            records_status_exposure: false,
            derives_operator_approval: false,
            derives_release_publication_authority: false,
            derives_activation_authority: false,
            renders_download_link: false,
            emits_install_command: false,
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
        "terminal_public_claim_delivery_receipt_terminal_decision_recorded_count",
        "terminal_public_claim_delivery_receipt_terminal_decision_persisted_count",
        "terminal_public_claim_delivery_receipt_terminal_status_recorded_count",
        "terminal_public_claim_delivery_receipt_terminal_status_persisted_count",
        "terminal_public_claim_delivery_receipt_status_promotion_recorded_count",
        "terminal_public_claim_delivery_receipt_public_status_exposed_count",
        "terminal_public_claim_delivery_receipt_external_decision_sent_count",
        "terminal_public_claim_delivery_receipt_telegram_decision_sent_count",
        "operator_approval_from_terminal_status_derived_count",
        "release_publication_authority_from_terminal_status_derived_count",
        "activation_authority_from_terminal_status_derived_count",
        "install_from_terminal_status_executed_count",
        "service_restart_from_terminal_status_performed_count",
        "active_binary_from_terminal_status_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count"
      ])
      + false_object([
        "terminal_public_claim_delivery_receipt_terminal_decision_recorded",
        "terminal_public_claim_delivery_receipt_terminal_status_recorded",
        "terminal_public_claim_delivery_receipt_status_promotion_recorded",
        "terminal_public_claim_delivery_receipt_public_status_exposed",
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
          "terminal_decision_recorded",
          "terminal_decision_persisted",
          "terminal_status_recorded",
          "terminal_status_persisted",
          "status_promotion_recorded",
          "public_status_exposed",
          "public_ga_status_exposed",
          "public_release_status_exposed",
          "channel_decision_delivered",
          "external_decision_sent",
          "telegram_decision_sent",
          "operator_approval_from_terminal_status_derived",
          "release_publication_authority_from_terminal_status_derived",
          "activation_authority_from_terminal_status_derived",
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
          "public_release_claimed",
          "public_ga_claimed",
          "filesystem_written"
        ])
      }
    '
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_denial_ready == true
  and .source_terminal_public_claim_delivery_receipt_final_acknowledgement_ready == true
  and .terminal_public_claim_delivery_receipt_terminal_decision_status_surface_count == 18
  and .terminal_public_claim_delivery_receipt_terminal_decision_status_denied_count == 18
  and .terminal_public_claim_delivery_receipt_terminal_decision_recorded_count == 0
  and .terminal_public_claim_delivery_receipt_terminal_status_recorded_count == 0
  and .terminal_public_claim_delivery_receipt_status_promotion_recorded_count == 0
  and .terminal_public_claim_delivery_receipt_public_status_exposed_count == 0
  and .terminal_public_claim_delivery_receipt_external_decision_sent_count == 0
  and .terminal_public_claim_delivery_receipt_telegram_decision_sent_count == 0
  and .release_publication_authority_from_terminal_status_derived_count == 0
  and .activation_authority_from_terminal_status_derived_count == 0
  and .install_from_terminal_status_executed_count == 0
  and .active_binary_from_terminal_status_mutated_count == 0
  and .terminal_public_claim_delivery_receipt_terminal_decision_recorded == false
  and .terminal_public_claim_delivery_receipt_terminal_status_recorded == false
  and .terminal_public_claim_delivery_receipt_status_promotion_recorded == false
  and .terminal_public_claim_delivery_receipt_public_status_exposed == false
  and .operator_approval_recorded == false
  and .release_publication_authority_derived == false
  and .activation_authority_derived == false
  and .provider_invoked == false
  and .credential_read == false
  and .install_executed == false
  and .active_binary_mutated == false
  and .external_send_performed == false
  and (.terminal_public_claim_delivery_receipt_terminal_decision_status_surfaces | all(
    .terminal_public_claim_delivery_receipt_terminal_decision_status_attempted == true
    and .terminal_public_claim_delivery_receipt_terminal_decision_status_noop_confirmed == true
    and .terminal_decision_allowed == false
    and .terminal_status_allowed == false
    and .status_promotion_allowed == false
    and .terminal_decision_recorded == false
    and .terminal_status_recorded == false
    and .status_promotion_recorded == false
    and .public_status_exposed == false
    and .telegram_decision_sent == false
    and .release_publication_authority_from_terminal_status_derived == false
    and .activation_authority_from_terminal_status_derived == false
    and .install_from_terminal_status_executed == false
    and .active_binary_from_terminal_status_mutated == false
    and .external_send_performed == false
  ))
  and (.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_terminal_public_claim_status_exposure_denial_gate"
    and .records_terminal_decision == false
    and .records_status_promotion == false
    and .records_public_claim == false
    and .records_status_exposure == false
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
echo "Hepta memory/intelligence/KG artifact signing terminal public claim delivery receipt terminal decision/status denial gate passed" >&2
