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
  echo "jq is required to build the artifact signing receipt signing receipt public status denial report" >&2
  exit 1
fi

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

SOURCE_GATE="scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-terminal-status-denial-gate.sh"
[[ -x "$SOURCE_GATE" ]] || {
  echo "missing executable terminal public claim delivery receipt artifact signing receipt signing receipt terminal decision/status denial gate: $SOURCE_GATE" >&2
  exit 1
}

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    "$SOURCE_GATE"
)"

source_report_sha256="$(sha256_text "$SOURCE_JSON")"
signing_receipt_public_status_contract_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-artifact-signing-receipt-signing-receipt-terminal-public-claim-status-exposure-denial:$source_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
signing_receipt_public_status_policy_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-signing-receipt-signing-receipt-public-status:no-public-claim:no-status-exposure:no-release:no-channel:no-telegram:no-install:no-live"
)"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_promotion_denial_ready == true
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_surface_count == 18
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_denied_count == 18
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_recorded_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_status_recorded_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_status_promotion_recorded_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_public_status_exposed_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_external_decision_sent_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_telegram_decision_sent_count == 0
    and $source.release_publication_authority_from_signing_receipt_terminal_status_derived_count == 0
    and $source.activation_authority_from_signing_receipt_terminal_status_derived_count == 0
    and $source.install_from_signing_receipt_terminal_status_executed_count == 0
    and $source.active_binary_from_signing_receipt_terminal_status_mutated_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_recorded == false
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_status_recorded == false
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_status_promotion_recorded == false
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_public_status_exposed == false
    and $source.operator_approval_recorded == false
    and $source.release_publication_authority_from_signing_receipt_terminal_status_derived == false
    and $source.activation_authority_from_signing_receipt_terminal_status_derived == false
    and $source.provider_invoked == false
    and $source.credential_read == false
    and ($source.allowed_next_actions | any(
      .action == "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_denial_gate"
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
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

surfaces_json="$(
  jq -n '[
    "source_signing_receipt_terminal_decision_status_report_required",
    "signing_receipt_terminal_public_claim_attempt",
    "signing_receipt_terminal_public_status_exposure",
    "signing_receipt_public_release_claim_attempt",
    "signing_receipt_public_ga_claim_attempt",
    "signing_receipt_release_status_exposure",
    "signing_receipt_publication_status_exposure",
    "signing_receipt_dashboard_status_exposure",
    "signing_receipt_public_badge_exposure",
    "signing_receipt_status_endpoint_exposure",
    "signing_receipt_query_status_exposure",
    "signing_receipt_export_status_exposure",
    "signing_receipt_observability_status_exposure",
    "signing_receipt_artifact_availability_status_exposure",
    "signing_receipt_distribution_queue_status_exposure",
    "signing_receipt_channel_external_telegram_status_exposure",
    "signing_receipt_release_publication_authority_status_exposure",
    "signing_receipt_activation_install_status_exposure"
  ] | map({
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_surface: .,
    source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_ready: true,
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_attempted: true,
    public_claim_requested: (. | test("claim")),
    status_exposure_requested: (. | test("status|exposure")),
    public_release_claim_requested: (. | test("public_release")),
    public_ga_claim_requested: (. | test("public_ga")),
    channel_or_external_requested: (. | test("channel|external|telegram")),
    authority_status_requested: (. | test("authority")),
    install_status_requested: (. | test("install")),
    public_claim_allowed: false,
    status_exposure_allowed: false,
    public_release_claim_allowed: false,
    public_ga_claim_allowed: false,
    public_claim_recorded: false,
    public_claim_persisted: false,
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
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_noop_confirmed: true,
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_status: "artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_denied"
  })'
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_denial_gate" \
    --arg source_report_sha256 "$source_report_sha256" \
    --arg signing_receipt_public_status_contract_hash_sha256 "$signing_receipt_public_status_contract_hash_sha256" \
    --arg signing_receipt_public_status_policy_hash_sha256 "$signing_receipt_public_status_policy_hash_sha256" \
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
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_schema_version: "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_denial_v1",
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_mode: "denied_signing_receipt_terminal_status_cannot_create_public_claim_status_exposure_release_channel_telegram_or_live_install",
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_gate: $source.gate,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_ready: $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_promotion_denial_ready,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_report_sha256: $source_report_sha256,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_contract_hash_sha256: $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_contract_hash_sha256,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_contract_hash_sha256: $signing_receipt_public_status_contract_hash_sha256,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_policy_hash_sha256: $signing_receipt_public_status_policy_hash_sha256,
        minimum_required_samples: $min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_denial_ready: true,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_surface_count: $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_surface_count,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_denied_count: $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_denied_count,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_surface_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_attempt_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_denied_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_surfaces: $surfaces,
        denied_by_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure: ($surfaces | map(.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_surface)),
        allowed_next_actions: [
          {
            action: "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_package_release_channel_status_denial_gate",
            status: "allowed_report_only_next_slice",
            records_public_claim: false,
            records_status_exposure: false,
            records_package_release_channel_status: false,
            derives_operator_approval: false,
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
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_recorded_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_persisted_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_public_claim_recorded_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_public_status_exposed_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_public_ga_status_exposed_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_public_release_status_exposed_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_release_status_exposed_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_publication_status_exposed_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_dashboard_status_exposed_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_status_endpoint_exposed_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_query_status_exposed_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_export_status_exposed_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_observability_status_exposed_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_channel_status_delivered_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_external_status_sent_count",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_telegram_status_sent_count",
        "operator_approval_from_public_status_derived_count",
        "release_publication_authority_from_public_status_derived_count",
        "activation_authority_from_public_status_derived_count",
        "install_from_public_status_executed_count",
        "active_binary_from_public_status_mutated_count",
        "provider_invoked_count",
        "credential_read_count",
        "external_send_performed_count"
      ])
      + false_object([
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_recorded",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_public_claim_recorded",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_public_status_exposed",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_public_ga_status_exposed",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_public_release_status_exposed",
        "public_status_claimed",
        "public_release_claimed",
        "public_ga_claimed",
        "public_ga_claim_allowed",
        "public_release_published",
        "operator_approval_recorded",
        "operator_approval_from_public_status_derived",
        "release_publication_authority_from_public_status_derived",
        "activation_authority_from_public_status_derived",
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
        "install_from_public_status_executed",
        "service_restarted",
        "active_binary_mutated",
        "active_binary_from_public_status_mutated",
        "external_send_performed",
        "telegram_send_performed"
      ])
      + {
        side_effects: false_object([
          "public_claim_recorded",
          "public_claim_persisted",
          "public_status_exposed",
          "public_ga_status_exposed",
          "public_release_status_exposed",
          "release_status_exposed",
          "publication_status_exposed",
          "package_release_channel_status_exposed",
          "dashboard_status_exposed",
          "status_endpoint_exposed",
          "query_status_exposed",
          "export_status_exposed",
          "observability_status_exposed",
          "channel_status_delivered",
          "external_status_sent",
          "telegram_status_sent",
          "operator_approval_from_public_status_derived",
          "release_publication_authority_from_public_status_derived",
          "activation_authority_from_public_status_derived",
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

printf '%s\n' "$report"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_denial_ready == true
  and .source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_decision_status_ready == true
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_surface_count == 18
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_denied_count == 18
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_public_claim_recorded_count == 0
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_public_status_exposed_count == 0
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_public_ga_status_exposed_count == 0
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_public_release_status_exposed_count == 0
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_external_status_sent_count == 0
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_telegram_status_sent_count == 0
  and .release_publication_authority_from_public_status_derived_count == 0
  and .activation_authority_from_public_status_derived_count == 0
  and .install_from_public_status_executed_count == 0
  and .active_binary_from_public_status_mutated_count == 0
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_recorded == false
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_public_claim_recorded == false
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_public_status_exposed == false
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_public_ga_status_exposed == false
  and .public_status_claimed == false
  and .public_ga_claimed == false
  and .public_release_claimed == false
  and .operator_approval_recorded == false
  and .release_publication_authority_from_public_status_derived == false
  and .activation_authority_from_public_status_derived == false
  and .provider_invoked == false
  and .credential_read == false
  and .install_executed == false
  and .active_binary_mutated == false
  and .external_send_performed == false
  and (.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_surfaces | length) == 18
  and (.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_surfaces | all(
    .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_attempted == true
    and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_noop_confirmed == true
    and .public_claim_allowed == false
    and .status_exposure_allowed == false
    and .public_claim_recorded == false
    and .public_status_exposed == false
    and .public_ga_status_exposed == false
    and .public_release_status_exposed == false
    and .external_status_sent == false
    and .telegram_status_sent == false
    and .release_publication_authority_from_public_status_derived == false
    and .activation_authority_from_public_status_derived == false
    and .install_from_public_status_executed == false
    and .active_binary_from_public_status_mutated == false
    and .credential_read == false
  ))
  and (.allowed_next_actions | any(
    .action == "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_package_release_channel_status_denial_gate"
    and .records_public_claim == false
    and .records_status_exposure == false
    and .records_package_release_channel_status == false
    and .derives_release_publication_authority == false
    and .derives_activation_authority == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .reads_credentials == false
    and .sends_externally == false
  ))
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

echo "Hepta memory/intelligence/KG artifact signing terminal public claim delivery receipt artifact signing receipt signing receipt terminal public claim/status exposure denial gate passed" >&2
