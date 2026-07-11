#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"
EXPECTED_ROUTE_COUNT="${HEPTA_EXPECTED_ROUTE_COUNT:-$(bash "$REPO_ROOT/scripts/lib/hepta-native-route-count.sh")}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

require_unsigned_integer() {
  local name="$1"
  local value="$2"

  case "$value" in
    ''|*[!0-9]*)
      echo "$name must be an unsigned integer" >&2
      exit 2
      ;;
  esac
}

require_source_text() {
  local source_file="$1"
  local source_text="$2"
  local label="$3"

  if ! rg -Fq "$source_text" "$source_file"; then
    echo "missing artifact signing receipt final operator acknowledgement route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

SOURCE_FINAL_ACK_GATE="scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-final-ack-denial-gate.sh"
source "$REPO_ROOT/scripts/lib/hepta-source-set.sh"
NATIVE_GATEWAY_SOURCE="hepta-native-gateway-source-set-v1"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"
ENDPOINT="/api/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-final-operator-acknowledgement-non-acceptance-denial"
SOURCE_COMMAND="/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-final-operator-acknowledgement-non-acceptance-denial --json"

SOURCE_GATE_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-final-ack-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      "$SOURCE_FINAL_ACK_GATE"
)"

jq -e '
  def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
  def false_fields($o; $fields): all($fields[]; $o[.] == false);

  .runtime == "hepta"
  and .status == "ready"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_non_acceptance_denial_ready == true
  and .source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_ready == true
  and .source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surface_count == 18
  and .source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_denied_count == 18
  and .artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surface_count == 18
  and .artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_attempt_count == 18
  and .artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_denied_count == 18
  and (.denied_by_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement | length) == 19
  and zero_fields(.; [
    "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_allowed_count",
    "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_accepted_count",
    "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_recorded_count",
    "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_persisted_count",
    "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_delivered_count",
    "artifact_distribution_signing_notarization_receipt_operator_received_recorded_count",
    "artifact_distribution_signing_notarization_receipt_operator_confirmed_recorded_count",
    "artifact_distribution_signing_notarization_receipt_operator_read_recorded_count",
    "artifact_distribution_signing_notarization_receipt_operator_seen_recorded_count",
    "artifact_distribution_signing_notarization_receipt_final_response_recorded_count",
    "artifact_distribution_signing_notarization_receipt_completion_acknowledgement_recorded_count",
    "artifact_distribution_signing_notarization_receipt_status_acknowledgement_recorded_count",
    "artifact_distribution_signing_notarization_receipt_external_acknowledgement_sent_count",
    "artifact_distribution_signing_notarization_receipt_telegram_acknowledgement_sent_count",
    "operator_approval_from_signing_receipt_acknowledgement_derived_count",
    "release_publication_authority_from_signing_receipt_acknowledgement_derived_count",
    "activation_authority_from_signing_receipt_acknowledgement_derived_count",
    "install_from_signing_receipt_acknowledgement_executed_count",
    "service_restart_from_signing_receipt_acknowledgement_performed_count",
    "active_binary_from_signing_receipt_acknowledgement_mutated_count",
    "memory_store_write_performed_count",
    "live_kg_write_performed_count",
    "provider_invoked_count",
    "model_invoked_count",
    "credential_read_count",
    "secret_file_read_count",
    "external_send_performed_count"
  ])
  and false_fields(.; [
    "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_accepted",
    "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_recorded",
    "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_persisted",
    "artifact_distribution_signing_notarization_receipt_operator_received_recorded",
    "artifact_distribution_signing_notarization_receipt_operator_confirmed_recorded",
    "artifact_distribution_signing_notarization_receipt_operator_read_recorded",
    "artifact_distribution_signing_notarization_receipt_operator_seen_recorded",
    "artifact_distribution_signing_notarization_receipt_final_response_recorded",
    "artifact_distribution_signing_notarization_receipt_completion_acknowledgement_recorded",
    "artifact_distribution_signing_notarization_receipt_status_acknowledgement_recorded",
    "operator_approval_recorded",
    "release_publication_authority_derived",
    "activation_authority_derived",
    "install_executed",
    "service_restarted",
    "active_binary_mutated",
    "memory_store_write_performed",
    "live_kg_write_performed",
    "provider_invoked",
    "model_invoked",
    "credential_read",
    "secret_file_read",
    "external_send_performed"
  ])
  and (.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surfaces | length) == 18
  and (.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surfaces | all(
    .artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_attempted == true
    and .artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_allowed == false
    and .artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_noop_confirmed == true
    and .final_operator_acknowledgement_accepted == false
    and .final_operator_acknowledgement_recorded == false
    and .operator_received_recorded == false
    and .operator_confirmed_recorded == false
    and .operator_read_recorded == false
    and .operator_seen_recorded == false
    and .final_response_recorded == false
    and .completion_acknowledgement_recorded == false
    and .status_acknowledgement_recorded == false
    and .external_acknowledgement_sent == false
    and .telegram_acknowledgement_sent == false
    and .release_publication_authority_from_acknowledgement_derived == false
    and .activation_authority_from_acknowledgement_derived == false
    and .install_from_acknowledgement_executed == false
    and .service_restart_from_acknowledgement_performed == false
    and .active_binary_from_acknowledgement_mutated == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .external_send_performed == false
  ))
  and ([.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surfaces[] | select(.final_operator_acknowledgement_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surfaces[] | select(.telegram_acknowledgement_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surfaces[] | select(.authority_acknowledgement_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surfaces[] | select(.live_acknowledgement_requested == true)] | length) == 2
  and ([.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surfaces[] | select(.install_restart_active_binary_acknowledgement_requested == true)] | length) == 1
  and (.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .records_final_acknowledgement == false
    and .records_received_confirmed_read_seen == false
    and .records_terminal_decision == false
    and .records_status_promotion == false
    and .derives_release_publication_authority == false
    and .derives_activation_authority == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .mutates_memory_store == false
    and .writes_kg == false
    and .invokes_provider == false
    and .reads_credentials == false
    and .sends_externally == false
  ))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$SOURCE_GATE_JSON"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes artifact signing receipt final operator acknowledgement route"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  "HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT" \
  "artifact signing receipt final operator acknowledgement endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$ENDPOINT" \
  "artifact signing receipt final operator acknowledgement endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$SOURCE_COMMAND" \
  "artifact signing receipt final operator acknowledgement source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_final_operator_acknowledgement_non_acceptance_denial_report" \
  "artifact signing receipt final operator acknowledgement report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_final_operator_acknowledgement_non_acceptance_denial_route_enabled\": true" \
  "artifact signing receipt final operator acknowledgement route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surface_count\": surface_count" \
  "artifact signing receipt final operator acknowledgement surface count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_final_operator_acknowledgement_endpoint_blocks_acceptance_and_authority" \
  "artifact signing receipt final operator acknowledgement focused endpoint test"

TEST_LOG="$(mktemp /tmp/hepta-artifact-signing-receipt-final-operator-acknowledgement-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_final_operator_acknowledgement_endpoint_blocks_acceptance_and_authority \
  -- --nocapture >"$TEST_LOG"

LIVE_ROUTE_JSON='{}'
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(curl -fsS "$BASE_URL$ENDPOINT")"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .implemented_route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_final_operator_acknowledgement_non_acceptance_denial_route_enabled == true
    and .memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_final_operator_acknowledgement_non_acceptance_denial_ready == true
    and .source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_ready == true
    and .artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surface_count == 18
    and .artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_attempt_count == 18
    and .artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_denied_count == 18
    and (.denied_by_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement | length) == 19
    and .artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_recorded_count == 0
    and .artifact_distribution_signing_notarization_receipt_operator_received_recorded_count == 0
    and .artifact_distribution_signing_notarization_receipt_status_acknowledgement_recorded_count == 0
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .secret_file_read == false
    and .external_send_performed == false
    and (.side_effects | to_entries | all(.value == false))
  ' >/dev/null <<<"$LIVE_ROUTE_JSON"
fi

TERMINAL_COVERAGE_JSON="$(
  capture_json_report \
    "hepta-preflight-terminal-coverage-inventory-gate" \
    scripts/hepta-preflight-terminal-coverage-inventory-gate.sh
)"

jq -e '
  .status == "ready"
  and .preflight_terminal_coverage_inventory_ready == true
  and .required_marker_count >= 300
  and .present_required_marker_count == .required_marker_count
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
source_gate_sha256="$(sha256_file "$SOURCE_FINAL_ACK_GATE")"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"
terminal_required_marker_count="$(jq -r '.required_marker_count // 0' <<<"$TERMINAL_COVERAGE_JSON")"
terminal_present_required_marker_count="$(jq -r '.present_required_marker_count // 0' <<<"$TERMINAL_COVERAGE_JSON")"
terminal_missing_required_marker_count="$(jq -r '.missing_required_marker_count // 0' <<<"$TERMINAL_COVERAGE_JSON")"
live_route_status="$(jq -r '.status // "skipped"' <<<"$LIVE_ROUTE_JSON")"
live_route_count="$(jq -r '.route_count // 0' <<<"$LIVE_ROUTE_JSON")"
live_missing_route_count="$(jq -r '.missing_route_count // 0' <<<"$LIVE_ROUTE_JSON")"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg endpoint "$ENDPOINT" \
  --arg source_command "$SOURCE_COMMAND" \
  --arg source_gate_sha256 "$source_gate_sha256" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg test_log "$TEST_LOG" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --arg live_route_status "$live_route_status" \
  --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
  --argjson source_surface_count "$(jq -r '.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surface_count' <<<"$SOURCE_GATE_JSON")" \
  --argjson source_denied_count "$(jq -r '.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_denied_count' <<<"$SOURCE_GATE_JSON")" \
  --argjson denied_by_count "$(jq -r '.denied_by_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement | length' <<<"$SOURCE_GATE_JSON")" \
  --argjson terminal_required_marker_count "$terminal_required_marker_count" \
  --argjson terminal_present_required_marker_count "$terminal_present_required_marker_count" \
  --argjson terminal_missing_required_marker_count "$terminal_missing_required_marker_count" \
  --argjson live_endpoint_checked "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && echo true || echo false)" \
  --argjson live_route_count "$live_route_count" \
  --argjson live_missing_route_count "$live_missing_route_count" \
  '{
    product: $product,
    runtime: $runtime,
    status: "ready",
    base_url: $base_url,
    gate: "hepta_artifact_signing_receipt_final_operator_acknowledgement_denial_route_gate",
    endpoint: $endpoint,
    source_command: $source_command,
    native_route: true,
    side_effect_free: true,
    expected_route_count: $expected_route_count,
    source_final_operator_acknowledgement_surface_count: $source_surface_count,
    source_final_operator_acknowledgement_denied_count: $source_denied_count,
    denied_by_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_count: $denied_by_count,
    route_source_text_verified: true,
    focused_endpoint_test_passed: true,
    test_log: $test_log,
    source_gate_sha256: $source_gate_sha256,
    native_gateway_sha256: $native_gateway_sha256,
    terminal_required_marker_count: $terminal_required_marker_count,
    terminal_present_required_marker_count: $terminal_present_required_marker_count,
    terminal_missing_required_marker_count: $terminal_missing_required_marker_count,
    terminal_coverage_sha256: $terminal_coverage_sha256,
    live_endpoint_checked: $live_endpoint_checked,
    live_route_status: $live_route_status,
    live_route_count: $live_route_count,
    live_missing_route_count: $live_missing_route_count,
    side_effects: {
      final_operator_acknowledgement_recorded: false,
      operator_received_recorded: false,
      operator_confirmed_recorded: false,
      operator_read_recorded: false,
      operator_seen_recorded: false,
      terminal_decision_recorded: false,
      status_promotion_recorded: false,
      release_publication_authority_derived: false,
      activation_authority_derived: false,
      install_executed: false,
      service_restarted: false,
      active_binary_mutated: false,
      memory_store_write_performed: false,
      live_kg_write_performed: false,
      provider_invoked: false,
      model_invoked: false,
      credential_read: false,
      external_send_performed: false
    }
  }'

echo "Hepta artifact signing receipt final operator acknowledgement non-acceptance denial route gate passed"
