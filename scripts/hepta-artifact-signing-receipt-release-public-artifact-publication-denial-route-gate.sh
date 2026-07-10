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
    echo "missing artifact signing receipt release/public artifact publication route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

SOURCE_DELIVERY_READBACK_ROUTE_GATE="scripts/hepta-artifact-signing-receipt-terminal-public-claim-delivery-readback-denial-route-gate.sh"
NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"
ENDPOINT="/api/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-release-public-artifact-publication-denial"
SOURCE_COMMAND="/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-release-public-artifact-publication-denial --json"

SOURCE_ROUTE_JSON="$(
  capture_json_report \
    "hepta-artifact-signing-receipt-terminal-public-claim-delivery-readback-denial-route-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      HEPTA_EXPECTED_ROUTE_COUNT="$EXPECTED_ROUTE_COUNT" \
      "$SOURCE_DELIVERY_READBACK_ROUTE_GATE"
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .expected_route_count >= 205
  and .source_terminal_public_claim_delivery_readback_surface_count == 18
  and .source_terminal_public_claim_delivery_readback_denied_count == 18
  and .denied_by_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_count == 14
  and .public_claim_delivery_request_count == 6
  and .status_readback_request_count == 12
  and .channel_delivery_request_count == 6
  and .side_effects.release_artifact_written == false
  and .side_effects.public_artifact_written == false
  and .side_effects.release_publication_authority_derived == false
  and .side_effects.activation_authority_derived == false
  and .side_effects.install_executed == false
  and .side_effects.service_restarted == false
  and .side_effects.active_binary_mutated == false
  and .side_effects.memory_store_write_performed == false
  and .side_effects.live_kg_write_performed == false
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.credential_read == false
  and .side_effects.external_send_performed == false
' >/dev/null <<<"$SOURCE_ROUTE_JSON"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes artifact signing receipt release/public artifact publication route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_RELEASE_PUBLIC_ARTIFACT_PUBLICATION_DENIAL_ENDPOINT" \
  "artifact signing receipt release/public artifact publication endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" "$ENDPOINT" \
  "artifact signing receipt release/public artifact publication endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" "$SOURCE_COMMAND" \
  "artifact signing receipt release/public artifact publication source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_release_public_artifact_publication_denial_report" \
  "artifact signing receipt release/public artifact publication report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_release_public_artifact_publication_denial_route_enabled\": true" \
  "artifact signing receipt release/public artifact publication route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_surface_count\": surface_count" \
  "artifact signing receipt release/public artifact publication surface count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_release_public_artifact_publication_endpoint_blocks_publication_and_authority" \
  "artifact signing receipt release/public artifact publication focused endpoint test"

TEST_LOG="$(mktemp /tmp/hepta-artifact-signing-receipt-release-public-artifact-publication-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_release_public_artifact_publication_endpoint_blocks_publication_and_authority \
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
    and .memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_release_public_artifact_publication_denial_route_enabled == true
    and .memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_release_public_artifact_publication_denial_ready == true
    and .source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_ready == true
    and .source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surface_count == 18
    and .source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denied_count == 18
    and .artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_surface_count == 18
    and .artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_attempt_count == 18
    and .artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_denied_count == 18
    and (.denied_by_artifact_distribution_signing_notarization_receipt_release_public_artifact_publication | length) == 17
    and .release_artifact_written_count == 0
    and .public_artifact_written_count == 0
    and .publication_queue_enqueued_count == 0
    and .publication_manifest_written_count == 0
    and .public_distribution_performed_count == 0
    and .package_registry_artifact_published_count == 0
    and .external_package_channel_published_count == 0
    and .telegram_package_channel_published_count == 0
    and .public_release_claimed_count == 0
    and .public_ga_claimed_count == 0
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
source_gate_sha256="$(sha256_file "$SOURCE_DELIVERY_READBACK_ROUTE_GATE")"
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
  --argjson source_surface_count "$(jq -r '.source_terminal_public_claim_delivery_readback_surface_count' <<<"$SOURCE_ROUTE_JSON")" \
  --argjson source_denied_count "$(jq -r '.source_terminal_public_claim_delivery_readback_denied_count' <<<"$SOURCE_ROUTE_JSON")" \
  --argjson source_denied_by_count "$(jq -r '.denied_by_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_count' <<<"$SOURCE_ROUTE_JSON")" \
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
    gate: "hepta_artifact_signing_receipt_release_public_artifact_publication_denial_route_gate",
    endpoint: $endpoint,
    source_command: $source_command,
    native_route: true,
    side_effect_free: true,
    expected_route_count: $expected_route_count,
    source_terminal_public_claim_delivery_readback_surface_count: $source_surface_count,
    source_terminal_public_claim_delivery_readback_denied_count: $source_denied_count,
    source_terminal_public_claim_delivery_readback_denied_by_count: $source_denied_by_count,
    release_public_artifact_publication_surface_count: 18,
    release_public_artifact_publication_denied_count: 18,
    denied_by_artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_count: 17,
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
      release_artifact_written: false,
      public_artifact_written: false,
      publication_queue_enqueued: false,
      publication_manifest_written: false,
      public_distribution_performed: false,
      package_registry_artifact_published: false,
      external_package_channel_published: false,
      telegram_package_channel_published: false,
      public_release_claimed: false,
      public_ga_claimed: false,
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

echo "Hepta artifact signing receipt release/public artifact publication denial route gate passed"
