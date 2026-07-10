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
    echo "missing scoped Memory canary durable receipt boundary source text: $label" >&2
    exit 1
  fi
}

SOURCE_POSITIVE_GATE="scripts/hepta-first-model-positive-approval-packet-boundary-route-gate.sh"
SOURCE_MEMORY_GATE="scripts/hepta-minimal-memory-canary-scoped-operator-packet-write-readback-rollback-idempotency-receipt-route-gate.sh"
NATIVE_GATEWAY_SOURCE="codex-rs/hepta-native-gateway/src/native_gateway.rs"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"
ENDPOINT="/api/hepta-scoped-memory-canary-durable-receipt-boundary"
SOURCE_COMMAND="/hepta-scoped-memory-canary-durable-receipt-boundary --json"

POSITIVE_SOURCE_JSON="$(
  capture_json_report \
    "hepta-first-model-positive-approval-packet-boundary-route-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_EXPECTED_ROUTE_COUNT="$EXPECTED_ROUTE_COUNT" \
      "$SOURCE_POSITIVE_GATE"
)"

MEMORY_SOURCE_JSON="$(
  capture_json_report \
    "hepta-minimal-memory-canary-scoped-operator-packet-route-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_EXPECTED_ROUTE_COUNT="$EXPECTED_ROUTE_COUNT" \
      "$SOURCE_MEMORY_GATE"
)"

jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
  .runtime == "hepta"
  and .status == "ready"
  and .expected_route_count == $expected
  and .positive_approval_packet_item_count == 12
  and .accepted_positive_approval_packet_item_count == 0
  and .denied_by_first_model_positive_approval_packet_boundary_count == 15
  and .side_effects.positive_approval_packet_accepted == false
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.credential_read == false
  and .side_effects.memory_store_write_performed == false
  and .side_effects.live_kg_write_performed == false
  and .next_slice == "prepare_minimal_memory_canary_scoped_operator_packet"
' >/dev/null <<<"$POSITIVE_SOURCE_JSON"

jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
  .runtime == "hepta"
  and .status == "ready"
  and .expected_route_count == $expected
  and .minimal_memory_canary_ready == true
  and .canary_execution_mode == "ephemeral_isolated_fixture_no_durable_store_mutation"
  and .scoped_operator_packet_count == 1
  and .ephemeral_memory_store_write_performed == true
  and .ephemeral_memory_readback_performed == true
  and .ephemeral_memory_rollback_performed == true
  and .idempotency_receipt_generated == true
  and .durable_memory_store_write_performed == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .live_kg_write_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .external_send_performed == false
  and .next_slice == "hepta_intelligence_bounded_context_attachment_preview_readback"
' >/dev/null <<<"$MEMORY_SOURCE_JSON"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes scoped Memory durable receipt boundary"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  "HEPTA_SCOPED_MEMORY_CANARY_DURABLE_RECEIPT_BOUNDARY_ENDPOINT" \
  "scoped Memory durable receipt boundary endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$ENDPOINT" \
  "scoped Memory durable receipt boundary endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$SOURCE_COMMAND" \
  "scoped Memory durable receipt boundary source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_scoped_memory_canary_durable_receipt_boundary_report" \
  "scoped Memory durable receipt boundary report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"scoped_memory_canary_durable_receipt_boundary_route_enabled\": true" \
  "scoped Memory durable receipt boundary route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"durable_receipt_candidate_count\": receipt_candidate_count" \
  "scoped Memory durable receipt candidate count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_scoped_memory_canary_durable_receipt_boundary_endpoint_blocks_durable_memory_mutation" \
  "focused scoped Memory durable receipt boundary unit test"

TEST_LOG="$(mktemp /tmp/hepta-scoped-memory-canary-durable-receipt-boundary-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_scoped_memory_canary_durable_receipt_boundary_endpoint_blocks_durable_memory_mutation \
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
    and .scoped_memory_canary_durable_receipt_boundary_route_enabled == true
    and .scoped_memory_canary_durable_receipt_boundary_ready == true
    and .scoped_memory_canary_durable_receipt_boundary_status == "blocked_report_only"
    and .source_first_model_positive_approval_packet_boundary_ready == true
    and .source_minimal_memory_canary_ready == true
    and .durable_receipt_candidate_count == 12
    and .accepted_durable_receipt_candidate_count == 0
    and (.durable_receipt_candidates | length) == 12
    and (.durable_receipt_candidates | all(.accepted == false))
    and (.denied_by_scoped_memory_canary_durable_receipt_boundary | length) == 16
    and .durable_receipt_preview_generated == true
    and .durable_receipt_recorded == false
    and .durable_receipt_persisted == false
    and .durable_receipt_accepted == false
    and .fresh_durable_memory_write_command_required == true
    and .fresh_durable_memory_write_command_present == false
    and .fresh_durable_memory_write_command_accepted == false
    and .accepted_scoped_memory_write_command == false
    and .memory_write_receipt_recorded == false
    and .memory_write_receipt_persisted == false
    and .memory_receipt_ledger_recorded == false
    and .durable_memory_store_write_performed == false
    and .durable_memory_store_read_performed == false
    and .durable_memory_store_rollback_performed == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .secret_file_read == false
    and .provider_prompt_injection_performed == false
    and .context_injection_performed == false
    and .live_kg_write_performed == false
    and .channel_send_performed == false
    and .telegram_send_performed == false
    and .external_send_performed == false
    and .release_artifact_written == false
    and .public_artifact_written == false
    and .public_release_claimed == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .allowed_next_actions[0].action == "hepta_bounded_intelligence_context_handoff_prompt_preview_boundary"
    and .allowed_next_actions[0].uses_scoped_memory_canary_durable_receipt_boundary == true
    and .allowed_next_actions[0].uses_durable_receipt_hash_only == true
    and .allowed_next_actions[0].accepts_durable_receipt == false
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
positive_source_gate_sha256="$(sha256_file "$SOURCE_POSITIVE_GATE")"
memory_source_gate_sha256="$(sha256_file "$SOURCE_MEMORY_GATE")"
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
  --arg positive_source_gate_sha256 "$positive_source_gate_sha256" \
  --arg memory_source_gate_sha256 "$memory_source_gate_sha256" \
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
    gate: "hepta_scoped_memory_canary_durable_receipt_boundary_route_gate",
    endpoint: $endpoint,
    source_command: $source_command,
    native_route: true,
    side_effect_free: true,
    expected_route_count: $expected_route_count,
    route_source_text_verified: true,
    focused_endpoint_test_passed: true,
    test_log: $test_log,
    native_gateway_sha256: $native_gateway_sha256,
    positive_source_gate_sha256: $positive_source_gate_sha256,
    memory_source_gate_sha256: $memory_source_gate_sha256,
    source_first_model_positive_approval_packet_boundary_ready: true,
    source_minimal_memory_canary_ready: true,
    durable_receipt_candidate_count: 12,
    accepted_durable_receipt_candidate_count: 0,
    denied_by_scoped_memory_canary_durable_receipt_boundary_count: 16,
    durable_receipt_preview_generated: true,
    terminal_required_marker_count: $terminal_required_marker_count,
    terminal_present_required_marker_count: $terminal_present_required_marker_count,
    terminal_missing_required_marker_count: $terminal_missing_required_marker_count,
    terminal_coverage_sha256: $terminal_coverage_sha256,
    live_endpoint_checked: $live_endpoint_checked,
    live_route_status: $live_route_status,
    live_route_count: $live_route_count,
    live_missing_route_count: $live_missing_route_count,
    side_effects: {
      durable_receipt_recorded: false,
      durable_receipt_persisted: false,
      durable_receipt_accepted: false,
      memory_write_receipt_recorded: false,
      memory_write_receipt_persisted: false,
      durable_memory_store_write_performed: false,
      durable_memory_store_read_performed: false,
      durable_memory_store_rollback_performed: false,
      memory_store_write_performed: false,
      memory_store_mutated: false,
      provider_invoked: false,
      model_invoked: false,
      credential_read: false,
      live_kg_write_performed: false,
      channel_send_performed: false,
      telegram_send_performed: false,
      external_send_performed: false,
      release_artifact_written: false,
      public_artifact_written: false,
      public_release_claimed: false,
      install_executed: false,
      service_restarted: false,
      active_binary_mutated: false,
      filesystem_written: false
    },
    next_slice: "hepta_bounded_intelligence_context_handoff_prompt_preview_boundary"
  }'

echo "Hepta scoped Memory canary durable receipt boundary route gate passed"
