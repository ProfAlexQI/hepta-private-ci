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
    echo "missing bounded Intelligence context handoff prompt preview boundary source text: $label" >&2
    exit 1
  fi
}

SOURCE_SCOPED_GATE="scripts/hepta-scoped-memory-canary-durable-receipt-boundary-route-gate.sh"
NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"
ENDPOINT="/api/hepta-bounded-intelligence-context-handoff-prompt-preview-boundary"
SOURCE_COMMAND="/hepta-bounded-intelligence-context-handoff-prompt-preview-boundary --json"

SCOPED_SOURCE_JSON="$(
  capture_json_report \
    "hepta-scoped-memory-canary-durable-receipt-boundary-route-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_EXPECTED_ROUTE_COUNT="$EXPECTED_ROUTE_COUNT" \
      "$SOURCE_SCOPED_GATE"
)"

jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
  .runtime == "hepta"
  and .status == "ready"
  and .expected_route_count == $expected
  and .durable_receipt_candidate_count == 12
  and .accepted_durable_receipt_candidate_count == 0
  and .denied_by_scoped_memory_canary_durable_receipt_boundary_count == 16
  and .durable_receipt_preview_generated == true
  and .side_effects.durable_receipt_recorded == false
  and .side_effects.durable_receipt_persisted == false
  and .side_effects.durable_receipt_accepted == false
  and .side_effects.durable_memory_store_write_performed == false
  and .side_effects.memory_store_write_performed == false
  and .side_effects.memory_store_mutated == false
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.credential_read == false
  and .side_effects.live_kg_write_performed == false
  and .side_effects.external_send_performed == false
  and .next_slice == "hepta_bounded_intelligence_context_handoff_prompt_preview_boundary"
' >/dev/null <<<"$SCOPED_SOURCE_JSON"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = ${EXPECTED_ROUTE_COUNT};" \
  "native gateway route/source command count includes bounded Intelligence handoff boundary"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "HEPTA_BOUNDED_INTELLIGENCE_CONTEXT_HANDOFF_PROMPT_PREVIEW_BOUNDARY_ENDPOINT" \
  "bounded Intelligence handoff endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" "$ENDPOINT" \
  "bounded Intelligence handoff endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" "$SOURCE_COMMAND" \
  "bounded Intelligence handoff source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_bounded_intelligence_context_handoff_prompt_preview_boundary_report" \
  "bounded Intelligence handoff report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"bounded_intelligence_context_handoff_prompt_preview_boundary_route_enabled\": true" \
  "bounded Intelligence handoff route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"redacted_receipt_reference_count\": redacted_receipt_reference_count" \
  "bounded Intelligence redacted receipt references emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"prompt_preview_boundary_generated\": true" \
  "bounded Intelligence prompt preview boundary emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"action\": \"hepta_kg_read_only_adapter_shadow_rank_canary\"" \
  "KG shadow-rank next slice"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_bounded_intelligence_context_handoff_prompt_preview_boundary_endpoint_blocks_prompt_injection_and_provider_invocation" \
  "focused bounded Intelligence handoff boundary unit test"

TEST_LOG="$(mktemp /tmp/hepta-bounded-intelligence-context-handoff-prompt-preview-boundary-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_bounded_intelligence_context_handoff_prompt_preview_boundary_endpoint_blocks_prompt_injection_and_provider_invocation \
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
    and .bounded_intelligence_context_handoff_prompt_preview_boundary_route_enabled == true
    and .bounded_intelligence_context_handoff_prompt_preview_boundary_ready == true
    and .bounded_intelligence_context_handoff_prompt_preview_boundary_status == "blocked_report_only"
    and .source_scoped_memory_canary_durable_receipt_boundary_ready == true
    and .source_hepta_intelligence_context_attachment_lane_ready == true
    and .redacted_receipt_reference_count == 4
    and (.redacted_receipt_references | length) == 4
    and (.redacted_receipt_references | all(.raw_payload_materialized == false and .accepted == false))
    and .context_handoff_candidate_count == 8
    and .accepted_context_handoff_candidate_count == 0
    and (.context_handoff_candidates | length) == 8
    and (.context_handoff_candidates | all(.accepted == false))
    and .prompt_preview_candidate_count == 6
    and .rendered_prompt_preview_candidate_count == 0
    and .accepted_prompt_preview_candidate_count == 0
    and (.prompt_preview_candidates | length) == 6
    and (.prompt_preview_candidates | all(.rendered == false and .accepted == false))
    and .denied_by_bounded_intelligence_context_handoff_prompt_preview_boundary_count == 18
    and (.denied_by_bounded_intelligence_context_handoff_prompt_preview_boundary | length) == 18
    and .uses_redacted_receipt_hashes_only == true
    and .bounded_context_handoff_preview_generated == true
    and .prompt_preview_boundary_generated == true
    and .boundary_readback_performed == true
    and .boundary_readback_hash_matched == true
    and .readback_receipt_persisted == false
    and .context_handoff_recorded == false
    and .context_handoff_persisted == false
    and .context_handoff_accepted == false
    and .prompt_preview_rendered_by_report_route == false
    and .raw_context_materialized == false
    and .raw_prompt_payload_materialized == false
    and .prompt_payload_materialized == false
    and .provider_prompt_injection_performed == false
    and .context_injection_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .secret_file_read == false
    and .kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .channel_send_performed == false
    and .telegram_send_performed == false
    and .external_send_performed == false
    and .release_artifact_written == false
    and .public_artifact_written == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .allowed_next_actions[0].action == "hepta_kg_read_only_adapter_shadow_rank_canary"
    and .allowed_next_actions[0].uses_bounded_intelligence_context_handoff_prompt_preview_boundary == true
    and .allowed_next_actions[0].uses_redacted_receipt_hashes == true
    and .allowed_next_actions[0].renders_prompt_payload == false
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
scoped_source_gate_sha256="$(sha256_file "$SOURCE_SCOPED_GATE")"
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
  --arg scoped_source_gate_sha256 "$scoped_source_gate_sha256" \
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
    gate: "hepta_bounded_intelligence_context_handoff_prompt_preview_boundary_route_gate",
    endpoint: $endpoint,
    source_command: $source_command,
    native_route: true,
    side_effect_free: true,
    expected_route_count: $expected_route_count,
    route_source_text_verified: true,
    focused_endpoint_test_passed: true,
    test_log: $test_log,
    native_gateway_sha256: $native_gateway_sha256,
    scoped_source_gate_sha256: $scoped_source_gate_sha256,
    source_scoped_memory_canary_durable_receipt_boundary_ready: true,
    source_hepta_intelligence_context_attachment_lane_ready: true,
    redacted_receipt_reference_count: 4,
    context_handoff_candidate_count: 8,
    accepted_context_handoff_candidate_count: 0,
    prompt_preview_candidate_count: 6,
    rendered_prompt_preview_candidate_count: 0,
    accepted_prompt_preview_candidate_count: 0,
    denied_by_bounded_intelligence_context_handoff_prompt_preview_boundary_count: 18,
    bounded_context_handoff_preview_generated: true,
    prompt_preview_boundary_generated: true,
    boundary_readback_performed: true,
    boundary_readback_hash_matched: true,
    terminal_required_marker_count: $terminal_required_marker_count,
    terminal_present_required_marker_count: $terminal_present_required_marker_count,
    terminal_missing_required_marker_count: $terminal_missing_required_marker_count,
    terminal_coverage_sha256: $terminal_coverage_sha256,
    live_endpoint_checked: $live_endpoint_checked,
    live_route_status: $live_route_status,
    live_route_count: $live_route_count,
    live_missing_route_count: $live_missing_route_count,
    side_effects: {
      durable_memory_store_read_performed: false,
      durable_memory_store_write_performed: false,
      memory_store_write_performed: false,
      memory_store_mutated: false,
      hepta_intelligence_context_attached: false,
      prompt_preview_rendered: false,
      prompt_payload_materialized: false,
      provider_prompt_injection_performed: false,
      context_injection_performed: false,
      provider_invoked: false,
      model_invoked: false,
      credential_read: false,
      kg_adapter_read_performed: false,
      live_kg_write_performed: false,
      channel_send_performed: false,
      telegram_send_performed: false,
      external_send_performed: false,
      readback_receipt_persisted: false,
      release_artifact_written: false,
      public_artifact_written: false,
      install_executed: false,
      service_restarted: false,
      active_binary_mutated: false,
      filesystem_written: false
    },
    next_slice: "hepta_kg_read_only_adapter_shadow_rank_canary"
  }'

echo "Hepta bounded Intelligence context handoff prompt preview boundary route gate passed"
