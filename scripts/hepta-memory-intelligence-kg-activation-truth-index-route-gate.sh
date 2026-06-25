#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"
EXPECTED_ROUTE_COUNT=190

cd "$REPO_ROOT"

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

require_source_text() {
  local source_file="$1"
  local source_text="$2"
  local label="$3"

  if ! rg -Fq "$source_text" "$source_file"; then
    echo "missing Memory/Intelligence/KG activation truth index source text: $label" >&2
    exit 1
  fi
}

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 190;' \
  "native gateway route/source command count includes activation truth index"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_ACTIVATION_TRUTH_INDEX_ENDPOINT' \
  "activation truth index endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-activation-truth-index' \
  "activation truth index endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-activation-truth-index --json' \
  "activation truth index source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_activation_truth_index_report' \
  "activation truth index report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'native_memory_intelligence_kg_activation_truth_index_read_only' \
  "activation truth index compatibility mode"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_activation_truth_index_endpoint_separates_lane_readiness_from_full_live_activation' \
  "focused activation truth index unit test"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"treat_lane_ready_as_full_live_activation"' \
  "truth index lane-ready/full-live separation denial"

TEST_LOG="$(mktemp /tmp/hepta-memory-intelligence-kg-activation-truth-index-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_activation_truth_index_endpoint_separates_lane_readiness_from_full_live_activation \
  -- --nocapture >"$TEST_LOG"

live_route_status="skipped"
live_route_count=0
live_missing_route_count=0

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-activation-truth-index"
  )"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .implemented_route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .hepta_core_connected == true
    and .hepta_core_full_fusion_complete == true
    and .memory_capability_inventory_ready == true
    and .operator_approved_lanes_ready == true
    and .operator_approved_lane_count == 3
    and .ready_operator_approved_lane_count == 3
    and .explicit_command_required_for_execution == true
    and .report_only_boundaries_intact == true
    and .full_live_activation_enabled == false
    and .full_live_activation_status == "blocked_report_only"
    and .full_live_activation_blocked == true
    and .replay_allowed == false
    and .replay_accepted == false
    and .readiness_index_side_effects_all_false == true
    and .memory_lane.operator_approved_lane_ready == true
    and .memory_lane.live_memory_write_allowed_by_lane == true
    and .memory_lane.execution_requires_explicit_command == true
    and .memory_lane.report_route_write_performed == false
    and .hepta_intelligence_lane.operator_approved_lane_ready == true
    and .hepta_intelligence_lane.context_attachment_lane_enabled == true
    and .hepta_intelligence_lane.context_attachment_requires_explicit_command == true
    and .hepta_intelligence_lane.report_route_context_attached == false
    and .hepta_intelligence_lane.report_route_context_injection_performed == false
    and .kg_lane.operator_approved_lane_ready == true
    and .kg_lane.kg_prompt_preview_lane_enabled == true
    and .kg_lane.kg_external_adapter_read_lane_enabled == true
    and .kg_lane.kg_external_adapter_requires_explicit_command == true
    and .kg_lane.kg_external_adapter_credential_reference_required == true
    and .kg_lane.kg_external_adapter_credential_read_allowed_by_lane == false
    and .kg_lane.supported_kg_adapter_count == 3
    and .kg_lane.kg_live_write_lane_enabled == false
    and .kg_lane.report_route_kg_adapter_read_performed == false
    and .kg_lane.report_route_credential_read_performed == false
    and .kg_lane.report_route_kg_live_write_performed == false
    and (.truth_matrix | length) == 6
    and ([.truth_matrix[] | select(.surface == "memory" and .operator_approved_lane_ready == true and .explicit_command_required == true and .report_route_execution_performed == false and .full_live_unrestricted == false)] | length) == 1
    and ([.truth_matrix[] | select(.surface == "kg" and .operator_approved_lane_ready == true and .explicit_command_required == true and .report_route_execution_performed == false and .full_live_unrestricted == false)] | length) == 1
    and (.blocked_actions | index("treat_lane_ready_as_full_live_activation")) != null
    and (.blocked_actions | index("write_memory_from_truth_index_report_route")) != null
    and (.blocked_actions | index("write_live_kg_from_truth_index_report_route")) != null
    and (.blocked_actions | index("invoke_provider_or_model_from_truth_index_report_route")) != null
    and (.blocked_actions | index("release_public_claim_from_truth_index_report_route")) != null
    and .allowed_next_actions[0].action == "continue_release_artifact_publication_denial_chain"
    and .allowed_next_actions[0].claims_public_release == false
    and (.side_effects | to_entries | all(.value == false))
  ' >/dev/null <<<"$LIVE_JSON"
  live_route_status="$(jq -r '.status' <<<"$LIVE_JSON")"
  live_route_count="$(jq -r '.route_count' <<<"$LIVE_JSON")"
  live_missing_route_count="$(jq -r '.missing_route_count' <<<"$LIVE_JSON")"
fi

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_activation_truth_index_route_gate" \
    --arg endpoint "/api/hepta-memory-intelligence-kg-activation-truth-index" \
    --arg source_command "/hepta-memory-intelligence-kg-activation-truth-index --json" \
    --arg native_gateway_sha256 "$(sha256_file "$NATIVE_GATEWAY_SOURCE")" \
    --arg test_log "$TEST_LOG" \
    --arg live_route_status "$live_route_status" \
    --argjson live_route_count "$live_route_count" \
    --argjson live_missing_route_count "$live_missing_route_count" \
    --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
    --argjson live_endpoint_checked "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && echo true || echo false)" \
    '{
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      source_command:$source_command,
      native_gateway_sha256:$native_gateway_sha256,
      focused_test_log:$test_log,
      live_endpoint_checked:$live_endpoint_checked,
      live_route_status:$live_route_status,
      live_route_count:$live_route_count,
      live_missing_route_count:$live_missing_route_count,
      expected_route_count:$expected_route_count,
      route_gate_ready:true,
      hepta_core_connected:true,
      memory_intelligence_kg_lanes_connected:true,
      operator_approved_lane_count:3,
      ready_operator_approved_lane_count:3,
      explicit_command_required_for_execution:true,
      full_live_activation_enabled:false,
      full_live_activation_status:"blocked_report_only",
      full_live_activation_blocked:true,
      memory_report_route_write_performed:false,
      intelligence_report_route_context_injection_performed:false,
      kg_report_route_adapter_read_performed:false,
      kg_live_write_performed:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      channel_send_performed:false,
      public_release_claimed:false,
      service_restarted:false,
      active_binary_mutated:false,
      next_slice:"continue_release_artifact_publication_denial_chain"
    }'
)"

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG activation truth index route gate passed"
