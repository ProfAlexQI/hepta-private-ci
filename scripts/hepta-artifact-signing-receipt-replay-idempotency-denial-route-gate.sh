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
    echo "missing artifact signing receipt replay/idempotency route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

SOURCE_REPLAY_IDEMPOTENCY_GATE="scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-replay-idempotency-denial-gate.sh"
NATIVE_GATEWAY_SOURCE="codex-rs/hepta-native-gateway/src/native_gateway.rs"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"

SOURCE_GATE_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-replay-idempotency-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      "$SOURCE_REPLAY_IDEMPOTENCY_GATE"
)"

jq -e '
  def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
  def false_fields($o; $fields): all($fields[]; $o[.] == false);

  .runtime == "hepta"
  and .status == "ready"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_replay_idempotency_denial_ready == true
  and .source_artifact_distribution_signing_notarization_result_receipt_ready == true
  and .source_artifact_distribution_signing_notarization_result_receipt_surface_count == 18
  and .source_artifact_distribution_signing_notarization_result_receipt_surface_attempt_count == 18
  and .source_artifact_distribution_signing_notarization_result_receipt_surface_denied_count == 18
  and .artifact_distribution_signing_notarization_receipt_replay_idempotency_surface_count == 18
  and .artifact_distribution_signing_notarization_receipt_replay_idempotency_attempt_count == 18
  and .artifact_distribution_signing_notarization_receipt_replay_idempotency_denied_count == 18
  and (.denied_by_artifact_distribution_signing_notarization_receipt_replay_idempotency | length) == 18
  and zero_fields(.; [
    "artifact_distribution_signing_notarization_receipt_replay_idempotency_allowed_count",
    "artifact_distribution_signing_notarization_receipt_replay_allowed_count",
    "artifact_distribution_signing_notarization_receipt_replay_accepted_count",
    "artifact_distribution_signing_notarization_receipt_replay_recorded_count",
    "artifact_distribution_signing_notarization_receipt_replay_persisted_count",
    "artifact_distribution_signing_notarization_receipt_duplicate_accepted_count",
    "artifact_distribution_signing_notarization_receipt_idempotency_key_accepted_count",
    "artifact_distribution_signing_notarization_receipt_idempotency_state_recorded_count",
    "artifact_distribution_signing_notarization_receipt_idempotency_state_persisted_count",
    "artifact_distribution_signing_notarization_receipt_hash_status_rebind_accepted_count",
    "release_publication_authority_from_signing_receipt_replay_derived_count",
    "activation_authority_from_signing_receipt_replay_derived_count",
    "install_from_signing_receipt_replay_executed_count",
    "service_restart_from_signing_receipt_replay_performed_count",
    "active_binary_from_signing_receipt_replay_mutated_count",
    "memory_store_write_performed_count",
    "live_kg_write_performed_count",
    "provider_invoked_count",
    "model_invoked_count",
    "credential_read_count",
    "secret_file_read_count",
    "external_send_performed_count"
  ])
  and false_fields(.; [
    "artifact_distribution_signing_notarization_receipt_replay_allowed",
    "artifact_distribution_signing_notarization_receipt_replay_accepted",
    "artifact_distribution_signing_notarization_receipt_replay_recorded",
    "artifact_distribution_signing_notarization_receipt_replay_persisted",
    "artifact_distribution_signing_notarization_receipt_duplicate_accepted",
    "artifact_distribution_signing_notarization_receipt_idempotency_key_accepted",
    "artifact_distribution_signing_notarization_receipt_idempotency_state_persisted",
    "artifact_distribution_signing_notarization_receipt_cross_scope_reuse_accepted",
    "artifact_distribution_signing_notarization_receipt_status_upgrade_accepted",
    "artifact_distribution_signing_notarization_receipt_completed_status_accepted",
    "artifact_distribution_signing_notarization_receipt_hash_status_rebind_accepted",
    "release_publication_authority_derived",
    "activation_authority_derived",
    "memory_store_write_performed",
    "live_kg_write_performed",
    "provider_invoked",
    "model_invoked",
    "credential_read",
    "secret_file_read",
    "install_executed",
    "launchd_mutated",
    "service_restarted",
    "active_binary_mutated",
    "external_send_performed"
  ])
  and (.artifact_distribution_signing_notarization_receipt_replay_idempotency_surfaces | length) == 18
  and (.artifact_distribution_signing_notarization_receipt_replay_idempotency_surfaces | all(
    .artifact_distribution_signing_notarization_receipt_replay_idempotency_attempted == true
    and .artifact_distribution_signing_notarization_receipt_replay_idempotency_allowed == false
    and .artifact_distribution_signing_notarization_receipt_replay_accepted == false
    and .artifact_distribution_signing_notarization_receipt_idempotency_key_accepted == false
    and .artifact_distribution_signing_notarization_receipt_idempotency_state_persisted == false
    and .artifact_distribution_signing_notarization_receipt_hash_status_rebind_accepted == false
    and .artifact_distribution_signing_notarization_receipt_replay_idempotency_noop_confirmed == true
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
  ))
  and ([.artifact_distribution_signing_notarization_receipt_replay_idempotency_surfaces[] | select(.signature_manifest_receipt_idempotency_key_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_replay_idempotency_surfaces[] | select(.notarization_ticket_stale_nonce_replay_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_replay_idempotency_surfaces[] | select(.telegram_signing_receipt_delivery_replay_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_replay_idempotency_surfaces[] | select(.install_restart_active_binary_replay_path_requested == true)] | length) == 1
  and (.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .replays_signing_receipt == false
    and .persists_idempotency_state == false
    and .derives_release_publication_authority == false
    and .derives_activation_authority == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .writes_kg == false
    and .invokes_provider == false
    and .reads_credentials == false
    and .sends_externally == false
  ))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$SOURCE_GATE_JSON"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes artifact signing receipt replay/idempotency route"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  "HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT" \
  "artifact signing receipt replay/idempotency endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  "/api/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-replay-idempotency-denial" \
  "artifact signing receipt replay/idempotency endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-replay-idempotency-denial --json" \
  "artifact signing receipt replay/idempotency source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_replay_idempotency_denial_report" \
  "artifact signing receipt replay/idempotency report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_replay_idempotency_denial_route_enabled\": true" \
  "artifact signing receipt replay/idempotency route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"artifact_distribution_signing_notarization_receipt_replay_idempotency_surface_count\": surface_count" \
  "artifact signing receipt replay/idempotency surface count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_replay_idempotency_endpoint_blocks_replay_and_authority" \
  "artifact signing receipt replay/idempotency focused endpoint test"

TEST_LOG="$(mktemp /tmp/hepta-artifact-signing-receipt-replay-idempotency-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_replay_idempotency_endpoint_blocks_replay_and_authority \
  -- --nocapture >"$TEST_LOG"

LIVE_ROUTE_JSON='{}'
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-replay-idempotency-denial"
  )"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .implemented_route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_replay_idempotency_denial_route_enabled == true
    and .memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_replay_idempotency_denial_ready == true
    and .source_artifact_distribution_signing_notarization_result_receipt_ready == true
    and .artifact_distribution_signing_notarization_receipt_replay_idempotency_surface_count == 18
    and .artifact_distribution_signing_notarization_receipt_replay_idempotency_attempt_count == 18
    and .artifact_distribution_signing_notarization_receipt_replay_idempotency_denied_count == 18
    and (.denied_by_artifact_distribution_signing_notarization_receipt_replay_idempotency | length) == 18
    and .artifact_distribution_signing_notarization_receipt_replay_allowed_count == 0
    and .artifact_distribution_signing_notarization_receipt_duplicate_accepted_count == 0
    and .artifact_distribution_signing_notarization_receipt_idempotency_key_accepted_count == 0
    and .artifact_distribution_signing_notarization_receipt_idempotency_state_persisted_count == 0
    and .artifact_distribution_signing_notarization_receipt_hash_status_rebind_accepted_count == 0
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
source_gate_sha256="$(sha256_file "$SOURCE_REPLAY_IDEMPOTENCY_GATE")"
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
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_replay_idempotency_denial_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-replay-idempotency-denial" \
  --arg source_command "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-replay-idempotency-denial --json" \
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
    activation_mode:"artifact_signing_receipt_replay_idempotency_native_route_status",
    side_effect_free:true,
    native_route:true,
    route_enabled:true,
    source_replay_idempotency_gate_ready:true,
    source_gate_sha256:$source_gate_sha256,
    native_gateway_sha256:$native_gateway_sha256,
    focused_test_log:$focused_test_log,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    live_endpoint_checked:$live_endpoint_checked,
    expected_route_count:$expected_route_count,
    terminal_required_marker_count:$terminal_required_marker_count,
    terminal_present_required_marker_count:$terminal_present_required_marker_count,
    terminal_missing_required_marker_count:$terminal_missing_required_marker_count,
    artifact_distribution_signing_notarization_receipt_replay_idempotency_surface_count:18,
    artifact_distribution_signing_notarization_receipt_replay_idempotency_denied_count:18,
    artifact_distribution_signing_notarization_receipt_replay_allowed_count:0,
    artifact_distribution_signing_notarization_receipt_duplicate_accepted_count:0,
    artifact_distribution_signing_notarization_receipt_idempotency_key_accepted_count:0,
    artifact_distribution_signing_notarization_receipt_idempotency_state_persisted_count:0,
    live_route_status:$live_route_status,
    live_route_count:$live_route_count,
    live_missing_route_count:$live_missing_route_count,
    side_effects:{
      signing_receipt_replayed:false,
      duplicate_receipt_accepted:false,
      idempotency_key_recorded:false,
      idempotency_state_persisted:false,
      replay_authority_derived:false,
      install_executed:false,
      active_binary_mutated:false,
      memory_store_write_performed:false,
      live_kg_write_performed:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      external_send_performed:false
    }
  }'

echo "Hepta artifact signing receipt replay/idempotency route gate passed"
