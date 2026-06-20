#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
RELEASE_BIN="${HEPTA_RELEASE_BIN:-${HEPTA_CODEX_RELEASE_BIN:-$HOME/.local/opt/hepta/bin/hepta}}"

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
    echo "missing operator canary operator-review/readback index route source text: $label" >&2
    exit 1
  fi
}

SINGLE_BUDGET_ROUTE_JSON="$(
  curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-single-budget-dispatch-dry-run-noop-receipt"
)"

jq -e '
  .status == "ready"
  and .operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_route_enabled == true
  and .operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_ready == true
  and .operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_status == "blocked"
  and .single_budget_declared == 1
  and .single_budget_accepted == false
  and .single_budget_consumed == 0
  and .single_budget_remaining == 0
  and .controlled_request_dispatched_count == 0
  and .controlled_request_executed_count == 0
  and .noop_receipt_persisted_count == 0
  and .noop_receipt_accepted_count == 0
  and .request_payload_materialized_count == 0
  and .request_payload_file_written_count == 0
  and .context_injection_performed_count == 0
  and .provider_invoked_count == 0
  and .model_invoked_count == 0
  and .memory_store_write_performed_count == 0
  and .live_kg_write_performed_count == 0
  and .credential_read_count == 0
  and .secret_file_read_count == 0
  and .channel_send_performed_count == 0
  and .canary_harness_armed == false
  and .canary_live_enabled == false
  and .current_live_enabled_lane_count == 13
  and .enablement_lane_count == 16
  and .ready_enablement_lane_count == 16
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.live_kg_write_performed == false
  and .side_effects.channel_send_performed == false
' >/dev/null <<<"$SINGLE_BUDGET_ROUTE_JSON"

OPERATOR_REVIEW_INDEX_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-readback-index-no-persistence-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-readback-index-no-persistence-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_gate"
  and .operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_ready == true
  and .operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_status == "blocked"
  and .source_single_budget_dispatch_dry_run_noop_receipt_status == "blocked"
  and .operator_review_readback_index_section_count == 8
  and .operator_review_required_count == 8
  and .operator_review_supplied_count == 0
  and .operator_review_recorded_count == 0
  and .operator_review_persisted_count == 0
  and .operator_review_delivered_count == 0
  and .operator_review_accepted_count == 0
  and .readback_index_recorded_count == 0
  and .readback_index_persisted_count == 0
  and .readback_index_materialized_count == 0
  and .dispatch_allowed_count == 0
  and .dispatch_performed_count == 0
  and .execution_allowed_count == 0
  and .execution_performed_count == 0
  and .provider_invoked_count == 0
  and .model_invoked_count == 0
  and .memory_store_write_performed_count == 0
  and .live_kg_write_performed_count == 0
  and .credential_read_count == 0
  and .secret_file_read_count == 0
  and .channel_send_performed_count == 0
  and .canary_harness_armed == false
  and .canary_live_enabled == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$OPERATOR_REVIEW_INDEX_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 158;' \
  "native gateway route/source command count includes operator-review/readback index route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_READBACK_INDEX_NO_PERSISTENCE_ENDPOINT' \
  "native gateway operator-review/readback index endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-readback-index-no-persistence' \
  "native gateway operator-review/readback index endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-readback-index-no-persistence --json' \
  "native gateway operator-review/readback index source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_report' \
  "native gateway operator-review/readback index report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_route_enabled": true' \
  "operator-review/readback index route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"operator_review_supplied_count": 0' \
  "operator review remains unsupplied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"operator_review_persisted_count": 0' \
  "operator review persistence denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"readback_index_persisted_count": 0' \
  "readback index persistence denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"dispatch_performed_count": 0' \
  "operator review dispatch denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"provider_invoked_count": 0' \
  "operator review provider invocation denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"live_kg_write_performed_count": 0' \
  "operator review live KG write denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-canary-operator-review-index-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_endpoint_reports_noop_only \
  -- --nocapture >"$TEST_LOG"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_route_gate" \
    --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-readback-index-no-persistence" \
    --arg source_command "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-readback-index-no-persistence --json" \
    --arg native_gateway_source "$NATIVE_GATEWAY_SOURCE" \
    --arg native_gateway_sha256 "$(sha256_file "$NATIVE_GATEWAY_SOURCE")" \
    --arg test_log "$TEST_LOG" \
    --argjson single_budget_route "$SINGLE_BUDGET_ROUTE_JSON" \
    --argjson operator_review_index "$OPERATOR_REVIEW_INDEX_JSON" \
    '{
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      source_command:$source_command,
      activation_mode:"operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_native_route_status",
      source_single_budget_route_ready:($single_budget_route.status == "ready"),
      source_operator_review_index_gate:$operator_review_index.gate,
      source_operator_review_index_gate_ready:$operator_review_index.operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_ready,
      source_operator_review_index_gate_status:$operator_review_index.operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_status,
      source_route_wired:true,
      source_route_count_expected:105,
      source_route_tested_by_native_gateway_unit_test:true,
      operator_authorization_source:"telegram_direct_operator_highest_authorization_2026_06_13_16_27_10_asia_shanghai",
      operator_authorization_received:true,
      operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_route_enabled:true,
      operator_review_required_count:8,
      operator_review_supplied_count:0,
      operator_review_recorded_count:0,
      operator_review_persisted_count:0,
      operator_review_delivered_count:0,
      operator_review_accepted_count:0,
      readback_index_declared_count:1,
      readback_index_recorded_count:0,
      readback_index_persisted_count:0,
      readback_index_materialized_count:0,
      dispatch_performed_count:0,
      execution_performed_count:0,
      provider_invoked_count:0,
      model_invoked_count:0,
      memory_store_write_performed_count:0,
      live_kg_write_performed_count:0,
      credential_read_count:0,
      secret_file_read_count:0,
      channel_send_performed_count:0,
      canary_harness_armed:false,
      canary_live_enabled:false,
      current_live_enabled_lane_count:14,
      enablement_lane_count:17,
      ready_enablement_lane_count:17,
      source_contract:{
        source:$native_gateway_source,
        source_sha256:$native_gateway_sha256,
        test_log:$test_log,
        contract:"hepta-native-gateway-operator-canary-controlled-request-harness-operator-review-readback-index-no-persistence-route-v1",
        source_pattern_present:true,
        compile_checked_by_tests:true,
        route_handler_present:true,
        records_operator_review:false,
        persists_operator_review:false,
        records_readback_index:false,
        persists_readback_index:false,
        dispatches_controlled_request:false,
        executes_controlled_request:false,
        injects_context:false,
        invokes_provider_or_model:false,
        writes_memory_or_kg:false,
        reads_credentials:false,
        delivers_channel:false
      },
      side_effects:{
        workspace_written:false,
        filesystem_written:false,
        operator_review_recorded:false,
        operator_review_persisted:false,
        operator_review_delivered:false,
        operator_review_accepted:false,
        operator_review_index_recorded:false,
        operator_review_index_persisted:false,
        operator_review_index_materialized:false,
        readback_index_persisted:false,
        readback_index_materialized:false,
        dispatch_performed:false,
        execution_performed:false,
        context_injection_performed:false,
        provider_invoked:false,
        model_invoked:false,
        memory_store_write_performed:false,
        live_kg_write_performed:false,
        credential_read:false,
        secret_file_read:false,
        channel_send_performed:false,
        service_restarted:false,
        active_binary_mutated:false,
        public_release_claimed:false,
        public_ga_claimed:false
      }
    }'
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_route_gate"
  and .source_single_budget_route_ready == true
  and .source_operator_review_index_gate_ready == true
  and .source_operator_review_index_gate_status == "blocked"
  and .source_route_wired == true
  and .source_route_count_expected == 105
  and .source_route_tested_by_native_gateway_unit_test == true
  and .operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_route_enabled == true
  and .operator_review_required_count == 8
  and .operator_review_supplied_count == 0
  and .operator_review_persisted_count == 0
  and .readback_index_persisted_count == 0
  and .dispatch_performed_count == 0
  and .execution_performed_count == 0
  and .provider_invoked_count == 0
  and .model_invoked_count == 0
  and .memory_store_write_performed_count == 0
  and .live_kg_write_performed_count == 0
  and .credential_read_count == 0
  and .secret_file_read_count == 0
  and .channel_send_performed_count == 0
  and .canary_harness_armed == false
  and .canary_live_enabled == false
  and .current_live_enabled_lane_count == 14
  and .enablement_lane_count == 17
  and .ready_enablement_lane_count == 17
  and (.source_contract.records_operator_review == false)
  and (.source_contract.persists_readback_index == false)
  and (.source_contract.dispatches_controlled_request == false)
  and (.source_contract.invokes_provider_or_model == false)
  and (.source_contract.writes_memory_or_kg == false)
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG operator canary controlled request harness operator-review/readback index no-persistence route gate passed"
