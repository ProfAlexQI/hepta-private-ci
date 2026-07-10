#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"
EXPECTED_ROUTE_COUNT="${HEPTA_EXPECTED_ROUTE_COUNT:-$(bash "$REPO_ROOT/scripts/lib/hepta-native-route-count.sh")}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

require_source_text() {
  local source_file="$1"
  local source_text="$2"
  local label="$3"

  if ! rg -Fq "$source_text" "$source_file"; then
    echo "missing first-model positive approval packet boundary source text: $label" >&2
    exit 1
  fi
}

SOURCE_ARTIFACT_GATE="scripts/hepta-artifact-signing-receipt-release-public-artifact-publication-denial-route-gate.sh"
SOURCE_FIRST_MODEL_GATE="scripts/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-route-gate.sh"
NATIVE_GATEWAY_SOURCE="codex-rs/hepta-native-gateway/src/native_gateway.rs"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"
ENDPOINT="/api/hepta-first-model-positive-approval-packet-boundary"
SOURCE_COMMAND="/hepta-first-model-positive-approval-packet-boundary --json"

ARTIFACT_SOURCE_JSON="$(
  capture_json_report \
    "hepta-artifact-signing-receipt-release-public-artifact-publication-denial-route-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_EXPECTED_ROUTE_COUNT="$EXPECTED_ROUTE_COUNT" \
      "$SOURCE_ARTIFACT_GATE"
)"

FIRST_MODEL_SOURCE_JSON="$(
  capture_json_report \
    "hepta-first-model-terminal-operator-decision-public-claim-route-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_EXPECTED_ROUTE_COUNT="$EXPECTED_ROUTE_COUNT" \
      "$SOURCE_FIRST_MODEL_GATE"
)"

jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
  .runtime == "hepta"
  and .status == "ready"
  and .expected_route_count == $expected
  and .release_public_artifact_publication_surface_count == 18
  and .release_public_artifact_publication_denied_count == 18
  and .denied_by_artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_count == 17
  and .side_effects.release_artifact_written == false
  and .side_effects.public_artifact_written == false
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.credential_read == false
  and .side_effects.external_send_performed == false
' >/dev/null <<<"$ARTIFACT_SOURCE_JSON"

jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
  .runtime == "hepta"
  and .status == "ready"
  and .expected_route_count == $expected
  and .terminal_operator_decision_public_claim_fixture_count == 10
  and .blocked_terminal_operator_decision_public_claim_fixture_count == 10
  and .accepted_terminal_operator_decision_public_claim_fixture_count == 0
  and .terminal_operator_decision_performed_count == 0
  and .public_claim_promotion_performed_count == 0
  and .terminal_operator_decision_recorded_count == 0
  and .terminal_operator_decision_persisted_count == 0
  and .public_claim_recorded_count == 0
  and .public_claim_promoted_count == 0
  and .public_release_published_count == 0
  and .release_artifact_written_count == 0
  and .provider_invocation_authorized == false
  and .model_invocation_authorized == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .live_kg_write_performed == false
  and .memory_store_write_performed == false
  and .external_send_performed == false
  and .install_executed == false
  and .service_restarted == false
  and .active_binary_mutated == false
' >/dev/null <<<"$FIRST_MODEL_SOURCE_JSON"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes first-model positive approval packet boundary"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  "HEPTA_FIRST_MODEL_POSITIVE_APPROVAL_PACKET_BOUNDARY_ENDPOINT" \
  "first-model positive approval packet boundary endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$ENDPOINT" \
  "first-model positive approval packet boundary endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$SOURCE_COMMAND" \
  "first-model positive approval packet boundary source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_first_model_positive_approval_packet_boundary_report" \
  "first-model positive approval packet boundary report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"first_model_positive_approval_packet_boundary_route_enabled\": true" \
  "first-model positive approval packet boundary route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"positive_approval_packet_item_count\": packet_item_count" \
  "first-model positive approval packet boundary item count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_first_model_positive_approval_packet_boundary_endpoint_blocks_approval_and_invocation_authority" \
  "focused first-model positive approval packet boundary unit test"

TEST_LOG="$(mktemp /tmp/hepta-first-model-positive-approval-packet-boundary-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_first_model_positive_approval_packet_boundary_endpoint_blocks_approval_and_invocation_authority \
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
    and .first_model_positive_approval_packet_boundary_route_enabled == true
    and .first_model_positive_approval_packet_boundary_ready == true
    and .first_model_positive_approval_packet_boundary_status == "blocked"
    and .source_artifact_publication_denial_ready == true
    and .source_first_model_terminal_decision_ready == true
    and .positive_approval_packet_item_count == 12
    and .accepted_positive_approval_packet_item_count == 0
    and (.positive_approval_packet_items | length) == 12
    and (.positive_approval_packet_items | all(.accepted == false))
    and (.denied_by_first_model_positive_approval_packet_boundary | length) == 15
    and .positive_approval_packet_recorded == false
    and .positive_approval_packet_persisted == false
    and .positive_approval_packet_accepted == false
    and .fresh_operator_approval_artifact_present == false
    and .single_use_nonce_consumed == false
    and .operator_identity_session_bound == false
    and .explicit_invocation_command_accepted == false
    and .approval_authority_derived == false
    and .activation_authority_derived == false
    and .provider_invocation_authorized == false
    and .model_invocation_authorized == false
    and .provider_invocation_budget == 0
    and .model_invocation_budget == 0
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .secret_file_read == false
    and .provider_prompt_injection_performed == false
    and .context_injection_performed == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .live_kg_write_performed == false
    and .channel_send_performed == false
    and .telegram_send_performed == false
    and .external_send_performed == false
    and .release_artifact_written == false
    and .public_artifact_written == false
    and .public_release_claimed == false
    and .public_ga_claimed == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .allowed_next_actions[0].action == "prepare_minimal_memory_canary_scoped_operator_packet"
    and .allowed_next_actions[0].accepts_positive_approval_packet == false
    and .allowed_next_actions[0].invokes_provider == false
    and .allowed_next_actions[0].invokes_model == false
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
  and .present_required_marker_count == .required_marker_count
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
artifact_source_gate_sha256="$(sha256_file "$SOURCE_ARTIFACT_GATE")"
first_model_source_gate_sha256="$(sha256_file "$SOURCE_FIRST_MODEL_GATE")"
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
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg artifact_source_gate_sha256 "$artifact_source_gate_sha256" \
  --arg first_model_source_gate_sha256 "$first_model_source_gate_sha256" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --arg test_log "$TEST_LOG" \
  --arg live_route_status "$live_route_status" \
  --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
  --argjson live_endpoint_checked "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && echo true || echo false)" \
  --argjson live_route_count "$live_route_count" \
  --argjson live_missing_route_count "$live_missing_route_count" \
  --argjson terminal_required_marker_count "$terminal_required_marker_count" \
  --argjson terminal_present_required_marker_count "$terminal_present_required_marker_count" \
  --argjson terminal_missing_required_marker_count "$terminal_missing_required_marker_count" \
  '{
    product: $product,
    runtime: $runtime,
    status: "ready",
    base_url: $base_url,
    gate: "hepta_first_model_positive_approval_packet_boundary_route_gate",
    endpoint: $endpoint,
    source_command: $source_command,
    native_route: true,
    side_effect_free: true,
    expected_route_count: $expected_route_count,
    route_source_text_verified: true,
    focused_endpoint_test_passed: true,
    test_log: $test_log,
    native_gateway_sha256: $native_gateway_sha256,
    artifact_source_gate_sha256: $artifact_source_gate_sha256,
    first_model_source_gate_sha256: $first_model_source_gate_sha256,
    source_artifact_publication_denial_ready: true,
    source_first_model_terminal_decision_ready: true,
    positive_approval_packet_item_count: 12,
    accepted_positive_approval_packet_item_count: 0,
    denied_by_first_model_positive_approval_packet_boundary_count: 15,
    terminal_required_marker_count: $terminal_required_marker_count,
    terminal_present_required_marker_count: $terminal_present_required_marker_count,
    terminal_missing_required_marker_count: $terminal_missing_required_marker_count,
    terminal_coverage_sha256: $terminal_coverage_sha256,
    live_endpoint_checked: $live_endpoint_checked,
    live_route_status: $live_route_status,
    live_route_count: $live_route_count,
    live_missing_route_count: $live_missing_route_count,
    side_effects: {
      positive_approval_packet_recorded: false,
      positive_approval_packet_persisted: false,
      positive_approval_packet_accepted: false,
      operator_approval_recorded: false,
      approval_authority_derived: false,
      activation_authority_derived: false,
      provider_invoked: false,
      model_invoked: false,
      credential_read: false,
      memory_store_write_performed: false,
      live_kg_write_performed: false,
      channel_send_performed: false,
      telegram_send_performed: false,
      external_send_performed: false,
      release_artifact_written: false,
      public_artifact_written: false,
      public_release_claimed: false,
      install_executed: false,
      service_restarted: false,
      active_binary_mutated: false
    },
    next_slice: "prepare_minimal_memory_canary_scoped_operator_packet"
  }'

echo "Hepta first-model positive approval packet boundary route gate passed"
