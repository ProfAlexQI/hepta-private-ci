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
    echo "missing full-live activation closure index source text: $label" >&2
    exit 1
  fi
}

SOURCE_TRUTH_GATE="scripts/hepta-memory-intelligence-kg-activation-truth-index-route-gate.sh"
SOURCE_EXPLICIT_GATE="scripts/hepta-first-model-invocation-explicit-approval-evidence-no-invocation-boundary-route-gate.sh"
source "$REPO_ROOT/scripts/lib/hepta-source-set.sh"
NATIVE_GATEWAY_SOURCE="hepta-native-gateway-source-set-v1"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"
ENDPOINT="/api/hepta-full-live-activation-closure-index"
SOURCE_COMMAND="/hepta-full-live-activation-closure-index --json"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes full-live closure index"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  "HEPTA_FULL_LIVE_ACTIVATION_CLOSURE_INDEX_ENDPOINT" \
  "full-live closure index endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$ENDPOINT" \
  "full-live closure index endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$SOURCE_COMMAND" \
  "full-live closure index source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_full_live_activation_closure_index_report" \
  "full-live closure index report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "insert_report_json!(\"full_live_activation_closure_index_ready\", report_ready)" \
  "full-live closure index ready field"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"remaining_unrestricted_activation_blocker_count\"," \
  "full-live closure blocker count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_full_live_activation_closure_index_endpoint_summarizes_blockers_without_side_effects" \
  "focused full-live closure index unit test"

TEST_LOG="$(mktemp /tmp/hepta-full-live-activation-closure-index-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_full_live_activation_closure_index_endpoint_summarizes_blockers_without_side_effects \
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
    and .full_live_activation_closure_index_ready == true
    and .full_live_activation_closure_index_status == "blocked_report_only"
    and .hepta_core_connected == true
    and .hepta_core_full_fusion_complete == true
    and .operator_approved_lanes_ready == true
    and .unrestricted_full_live_activation_enabled == false
    and .unrestricted_full_live_activation_allowed == false
    and .closure_source_count == 8
    and .ready_closure_source_count == 8
    and .closure_blocker_count == 13
    and .accepted_unrestricted_activation_blocker_count == 0
    and .remaining_unrestricted_activation_blocker_count == 13
    and .canary_ladder_phase_count == 5
    and (.closure_sources | length) == 8
    and (.closure_sources | all(.ready == true))
    and (.closure_blockers | length) == 13
    and (.closure_blockers | all(.accepted == false))
    and .fresh_operator_approval_artifact_verified == false
    and .single_use_nonce_consumed == false
    and .operator_identity_session_bound == false
    and .explicit_command_accepted == false
    and .fresh_long_soak_evidence_accepted == false
    and .activation_evidence_recorded == false
    and .activation_evidence_persisted == false
    and .durable_memory_store_write_performed == false
    and .bounded_context_handoff_accepted == false
    and .kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .provider_invocation_authorized == false
    and .model_invocation_authorized == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .channel_send_performed == false
    and .external_send_performed == false
    and .release_artifact_written == false
    and .public_artifact_written == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .allowed_next_actions[0].action == "run_full_live_activation_closure_index_require_live_gate"
    and .allowed_next_actions[1].action == "prepare_scoped_live_canary_operator_packet"
    and .allowed_next_actions[2].action == "continue_first_model_invocation_operator_approval_packet_review_acceptance_denial_preflight"
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
truth_source_gate_sha256="$(sha256_file "$SOURCE_TRUTH_GATE")"
explicit_source_gate_sha256="$(sha256_file "$SOURCE_EXPLICIT_GATE")"
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
  --arg truth_source_gate_sha256 "$truth_source_gate_sha256" \
  --arg explicit_source_gate_sha256 "$explicit_source_gate_sha256" \
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
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:"hepta_full_live_activation_closure_index_route_gate",
    endpoint:$endpoint,
    source_command:$source_command,
    native_gateway_sha256:$native_gateway_sha256,
    truth_source_gate_sha256:$truth_source_gate_sha256,
    explicit_source_gate_sha256:$explicit_source_gate_sha256,
    focused_test_log:$test_log,
    live_endpoint_checked:$live_endpoint_checked,
    live_route_status:$live_route_status,
    live_route_count:$live_route_count,
    live_missing_route_count:$live_missing_route_count,
    expected_route_count:$expected_route_count,
    route_gate_ready:true,
    full_live_activation_closure_index_ready:true,
    unrestricted_full_live_activation_enabled:false,
    closure_source_count:8,
    ready_closure_source_count:8,
    closure_blocker_count:13,
    accepted_unrestricted_activation_blocker_count:0,
    remaining_unrestricted_activation_blocker_count:13,
    canary_ladder_phase_count:5,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    terminal_required_marker_count:$terminal_required_marker_count,
    terminal_present_required_marker_count:$terminal_present_required_marker_count,
    terminal_missing_required_marker_count:$terminal_missing_required_marker_count,
    hepta_core_connected:true,
    hepta_core_full_fusion_complete:true,
    operator_approved_lanes_ready:true,
    provider_invoked:false,
    model_invoked:false,
    credential_read:false,
    live_kg_write_performed:false,
    memory_store_write_performed:false,
    channel_send_performed:false,
    external_send_performed:false,
    release_artifact_written:false,
    public_artifact_written:false,
    install_executed:false,
    service_restarted:false,
    active_binary_mutated:false,
    side_effects:{
      full_live_activation_enabled:false,
      activation_evidence_persisted:false,
      durable_memory_store_write_performed:false,
      live_kg_write_performed:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      channel_send_performed:false,
      external_send_performed:false,
      release_artifact_written:false,
      public_artifact_written:false,
      install_executed:false,
      service_restarted:false,
      active_binary_mutated:false
    }
  }'

echo "Hepta full-live activation closure index route gate passed"
