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
    echo "missing artifact signing receipt operator-facing summary/briefing route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

SOURCE_SUMMARY_BRIEFING_GATE="scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-summary-briefing-denial-gate.sh"
NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"
ENDPOINT="/api/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-operator-facing-summary-briefing-non-persistence-denial"
SOURCE_COMMAND="/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-operator-facing-summary-briefing-non-persistence-denial --json"

SOURCE_GATE_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-summary-briefing-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      "$SOURCE_SUMMARY_BRIEFING_GATE"
)"

jq -e '
  def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
  def false_fields($o; $fields): all($fields[]; $o[.] == false);

  .runtime == "hepta"
  and .status == "ready"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
  and .source_artifact_distribution_signing_notarization_receipt_export_query_observability_ready == true
  and .source_artifact_distribution_signing_notarization_receipt_export_query_observability_surface_count == 18
  and .source_artifact_distribution_signing_notarization_receipt_export_query_observability_denied_count == 18
  and .artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surface_count == 18
  and .artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_attempt_count == 18
  and .artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_denied_count == 18
  and (.denied_by_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing | length) == 19
  and zero_fields(.; [
    "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_allowed_count",
    "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_accepted_count",
    "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_recorded_count",
    "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_persisted_count",
    "artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count",
    "artifact_distribution_signing_notarization_receipt_operator_briefing_recorded_count",
    "artifact_distribution_signing_notarization_receipt_readback_recorded_count",
    "artifact_distribution_signing_notarization_receipt_status_banner_recorded_count",
    "artifact_distribution_signing_notarization_receipt_briefing_delivery_recorded_count",
    "artifact_distribution_signing_notarization_receipt_summary_briefing_acceptance_recorded_count",
    "operator_approval_from_signing_receipt_summary_briefing_derived_count",
    "release_publication_authority_from_signing_receipt_summary_briefing_derived_count",
    "activation_authority_from_signing_receipt_summary_briefing_derived_count",
    "install_from_signing_receipt_summary_briefing_executed_count",
    "service_restart_from_signing_receipt_summary_briefing_performed_count",
    "active_binary_from_signing_receipt_summary_briefing_mutated_count",
    "memory_store_write_performed_count",
    "live_kg_write_performed_count",
    "provider_invoked_count",
    "model_invoked_count",
    "credential_read_count",
    "secret_file_read_count",
    "external_send_performed_count"
  ])
  and false_fields(.; [
    "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_accepted",
    "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_recorded",
    "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_persisted",
    "artifact_distribution_signing_notarization_receipt_operator_summary_recorded",
    "artifact_distribution_signing_notarization_receipt_operator_briefing_recorded",
    "artifact_distribution_signing_notarization_receipt_readback_recorded",
    "artifact_distribution_signing_notarization_receipt_status_banner_recorded",
    "artifact_distribution_signing_notarization_receipt_briefing_delivery_recorded",
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
  and (.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surfaces | length) == 18
  and (.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surfaces | all(
    .artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_attempted == true
    and .artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_allowed == false
    and .artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_noop_confirmed == true
    and .operator_summary_recorded == false
    and .operator_briefing_recorded == false
    and .signing_receipt_readback_recorded == false
    and .briefing_delivery_recorded == false
    and .external_briefing_delivered == false
    and .telegram_briefing_delivered == false
    and .release_publication_authority_from_summary_briefing_derived == false
    and .activation_authority_from_summary_briefing_derived == false
    and .install_from_summary_briefing_executed == false
    and .service_restart_from_summary_briefing_performed == false
    and .active_binary_from_summary_briefing_mutated == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .external_send_performed == false
  ))
  and ([.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surfaces[] | select(.summary_requested == true)] | length) == 4
  and ([.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surfaces[] | select(.briefing_requested == true)] | length) == 7
  and ([.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surfaces[] | select(.external_briefing_requested == true and .telegram_briefing_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surfaces[] | select(.authority_briefing_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surfaces[] | select(.live_status_briefing_requested == true)] | length) == 2
  and ([.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surfaces[] | select(.install_restart_active_binary_status_requested == true)] | length) == 1
  and (.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_non_acceptance_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .records_summary == false
    and .records_briefing == false
    and .records_readback == false
    and .records_delivery == false
    and .records_acknowledgement == false
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
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = ${EXPECTED_ROUTE_COUNT};" \
  "native gateway route/source command count includes artifact signing receipt operator-facing summary/briefing route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT" \
  "artifact signing receipt operator-facing summary/briefing endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" "$ENDPOINT" \
  "artifact signing receipt operator-facing summary/briefing endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" "$SOURCE_COMMAND" \
  "artifact signing receipt operator-facing summary/briefing source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_operator_facing_summary_briefing_non_persistence_denial_report" \
  "artifact signing receipt operator-facing summary/briefing report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_operator_facing_summary_briefing_non_persistence_denial_route_enabled\": true" \
  "artifact signing receipt operator-facing summary/briefing route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surface_count\": surface_count" \
  "artifact signing receipt operator-facing summary/briefing surface count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_operator_summary_briefing_endpoint_blocks_delivery_and_authority" \
  "artifact signing receipt operator-facing summary/briefing focused endpoint test"

TEST_LOG="$(mktemp /tmp/hepta-artifact-signing-receipt-operator-summary-briefing-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_operator_summary_briefing_endpoint_blocks_delivery_and_authority \
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
    and .memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_operator_facing_summary_briefing_non_persistence_denial_route_enabled == true
    and .memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
    and .source_artifact_distribution_signing_notarization_receipt_export_query_observability_ready == true
    and .artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surface_count == 18
    and .artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_attempt_count == 18
    and .artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_denied_count == 18
    and (.denied_by_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing | length) == 19
    and .artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count == 0
    and .artifact_distribution_signing_notarization_receipt_operator_briefing_recorded_count == 0
    and .artifact_distribution_signing_notarization_receipt_briefing_delivery_recorded_count == 0
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
source_gate_sha256="$(sha256_file "$SOURCE_SUMMARY_BRIEFING_GATE")"
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
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_operator_facing_summary_briefing_denial_route_gate" \
  --arg endpoint "$ENDPOINT" \
  --arg source_command "$SOURCE_COMMAND" \
  --arg source_gate_sha256 "$source_gate_sha256" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg focused_test_log "$TEST_LOG" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --arg live_route_status "$live_route_status" \
  --argjson live_endpoint_checked "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && echo true || echo false)" \
  --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
  --argjson terminal_required_marker_count "$terminal_required_marker_count" \
  --argjson terminal_present_required_marker_count "$terminal_present_required_marker_count" \
  --argjson terminal_missing_required_marker_count "$terminal_missing_required_marker_count" \
  --argjson live_route_count "$live_route_count" \
  --argjson live_missing_route_count "$live_missing_route_count" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    endpoint:$endpoint,
    source_command:$source_command,
    activation_mode:"artifact_signing_receipt_operator_facing_summary_briefing_native_route_status",
    side_effect_free:true,
    native_route:true,
    route_enabled:true,
    source_operator_facing_summary_briefing_gate_ready:true,
    source_gate_sha256:$source_gate_sha256,
    native_gateway_sha256:$native_gateway_sha256,
    focused_test_log:$focused_test_log,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    live_endpoint_checked:$live_endpoint_checked,
    expected_route_count:$expected_route_count,
    terminal_required_marker_count:$terminal_required_marker_count,
    terminal_present_required_marker_count:$terminal_present_required_marker_count,
    terminal_missing_required_marker_count:$terminal_missing_required_marker_count,
    artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surface_count:18,
    artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_denied_count:18,
    denied_by_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_count:19,
    artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count:0,
    artifact_distribution_signing_notarization_receipt_operator_briefing_recorded_count:0,
    artifact_distribution_signing_notarization_receipt_briefing_delivery_recorded_count:0,
    live_route_status:$live_route_status,
    live_route_count:$live_route_count,
    live_missing_route_count:$live_missing_route_count
  }'
