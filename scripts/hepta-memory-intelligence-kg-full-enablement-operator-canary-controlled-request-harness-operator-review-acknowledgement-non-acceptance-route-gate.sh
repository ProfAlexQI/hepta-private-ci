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
    echo "missing operator canary operator-review acknowledgement non-acceptance route source text: $label" >&2
    exit 1
  fi
}

OPERATOR_REVIEW_INDEX_ROUTE_JSON="$(
  curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-readback-index-no-persistence"
)"

jq -e '
  .status == "ready"
  and .operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_route_enabled == true
  and .operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_ready == true
  and .operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_status == "blocked"
  and .operator_review_required_count == 8
  and .operator_review_supplied_count == 0
  and .operator_review_recorded_count == 0
  and .operator_review_persisted_count == 0
  and .operator_review_delivered_count == 0
  and .operator_review_accepted_count == 0
  and .readback_index_declared_count == 1
  and .readback_index_recorded_count == 0
  and .readback_index_persisted_count == 0
  and .readback_index_materialized_count == 0
  and .readback_index_filesystem_written_count == 0
  and .review_authorizes_dispatch_count == 0
  and .review_authorizes_execution_count == 0
  and .review_authorizes_live_count == 0
  and .dispatch_performed_count == 0
  and .execution_performed_count == 0
  and .context_injection_performed_count == 0
  and .provider_invoked_count == 0
  and .model_invoked_count == 0
  and .memory_store_write_performed_count == 0
  and .external_kg_adapter_read_performed_count == 0
  and .live_kg_write_performed_count == 0
  and .credential_read_count == 0
  and .secret_file_read_count == 0
  and .channel_send_performed_count == 0
  and .canary_harness_armed == false
  and .canary_live_enabled == false
  and .current_live_enabled_lane_count == 14
  and .enablement_lane_count == 17
  and .ready_enablement_lane_count == 17
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$OPERATOR_REVIEW_INDEX_ROUTE_JSON"

OPERATOR_ACK_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-non-acceptance-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-non-acceptance-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_gate"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_status == "blocked"
  and .source_operator_review_readback_index_gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_gate"
  and .source_operator_review_readback_index_status == "blocked"
  and .source_operator_review_required_count == 8
  and .source_operator_review_accepted_count == 0
  and .source_readback_index_declared_count == 1
  and .source_readback_index_persisted_count == 0
  and .source_review_authorizes_dispatch_count == 0
  and .source_review_authorizes_execution_count == 0
  and .source_review_authorizes_live_count == 0
  and .operator_review_acknowledgement_fixture_count == 8
  and .operator_review_acknowledgement_requested_fixture_count == 8
  and .blocked_operator_review_acknowledgement_fixture_count == 8
  and .noop_operator_review_acknowledgement_fixture_count == 8
  and .allowed_operator_review_acknowledgement_fixture_count == 0
  and .accepted_operator_review_acknowledgement_fixture_count == 0
  and .operator_review_acknowledgement_performed_count == 0
  and .operator_review_acknowledgement_allowed == false
  and .operator_review_acknowledgement_accepted == false
  and .operator_review_acknowledgement_recorded == false
  and .operator_review_acknowledgement_persisted == false
  and .operator_review_acknowledgement_materialized == false
  and .operator_review_acknowledgement_filesystem_written == false
  and .operator_review_acknowledgement_delivered == false
  and .operator_review_acknowledgement_identity_accepted == false
  and .operator_review_acknowledgement_signature_accepted == false
  and .operator_review_acknowledgement_final_state_promoted == false
  and .operator_review_acknowledgement_completion_promoted == false
  and .operator_review_acknowledgement_authorizes_dispatch_count == 0
  and .operator_review_acknowledgement_authorizes_execution_count == 0
  and .operator_review_acknowledgement_authorizes_live_count == 0
  and .operator_approval_recorded == false
  and .operator_identity_accepted == false
  and .readback_index_recorded_count == 0
  and .readback_index_persisted_count == 0
  and .readback_index_materialized_count == 0
  and .readback_index_filesystem_written_count == 0
  and .dispatch_allowed_count == 0
  and .dispatch_performed_count == 0
  and .execution_allowed_count == 0
  and .execution_performed_count == 0
  and .context_injection_performed_count == 0
  and .provider_invoked_count == 0
  and .model_invoked_count == 0
  and .memory_store_write_performed_count == 0
  and .external_kg_adapter_read_performed_count == 0
  and .live_kg_write_performed_count == 0
  and .credential_read_count == 0
  and .secret_file_read_count == 0
  and .channel_send_performed_count == 0
  and .canary_harness_armed == false
  and .canary_harness_executable == false
  and .canary_live_enabled == false
  and .denied_by_operator_review_acknowledgement_non_acceptance_count == 19
  and (.operator_review_acknowledgement_fixtures | all(
    .acknowledgement_requested == true
    and .acknowledgement_status == "blocked_noop"
    and .acknowledgement_performed == false
    and .acknowledgement_accepted == false
    and .acknowledgement_recorded == false
    and .acknowledgement_persisted == false
    and .acknowledgement_materialized == false
    and .acknowledgement_filesystem_written == false
    and .acknowledgement_delivered == false
    and .dispatch_performed == false
    and .execution_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .external_kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .channel_send_performed == false
  ))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$OPERATOR_ACK_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 108;' \
  "native gateway route/source command count includes operator-review acknowledgement non-acceptance route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_NON_ACCEPTANCE_ENDPOINT' \
  "native gateway operator-review acknowledgement endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-non-acceptance' \
  "native gateway operator-review acknowledgement endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-non-acceptance --json' \
  "native gateway operator-review acknowledgement source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_report' \
  "native gateway operator-review acknowledgement report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_route_enabled": true' \
  "operator-review acknowledgement route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"operator_review_acknowledgement_performed_count": 0' \
  "operator-review acknowledgement performed count remains zero"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"operator_review_acknowledgement_persisted": false' \
  "operator-review acknowledgement persistence denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"operator_review_acknowledgement_authorizes_dispatch_count": 0' \
  "operator-review acknowledgement dispatch authority denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"provider_invoked_count": 0' \
  "operator-review acknowledgement provider invocation denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"live_kg_write_performed_count": 0' \
  "operator-review acknowledgement live KG write denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-canary-operator-review-ack-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_endpoint_reports_noop_only \
  -- --nocapture >"$TEST_LOG"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_route_gate" \
    --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-non-acceptance" \
    --arg source_command "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-non-acceptance --json" \
    --arg native_gateway_source "$NATIVE_GATEWAY_SOURCE" \
    --arg native_gateway_sha256 "$(sha256_file "$NATIVE_GATEWAY_SOURCE")" \
    --arg test_log "$TEST_LOG" \
    --argjson operator_review_index_route "$OPERATOR_REVIEW_INDEX_ROUTE_JSON" \
    --argjson operator_ack "$OPERATOR_ACK_JSON" \
    '{
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      source_command:$source_command,
      activation_mode:"operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_native_route_status",
      source_operator_review_index_route_ready:($operator_review_index_route.status == "ready"),
      source_operator_acknowledgement_gate:$operator_ack.gate,
      source_operator_acknowledgement_gate_ready:$operator_ack.operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_ready,
      source_operator_acknowledgement_gate_status:$operator_ack.operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_status,
      source_route_wired:true,
      source_route_count_expected:105,
      source_route_tested_by_native_gateway_unit_test:true,
      operator_authorization_source:"telegram_direct_operator_highest_authorization_2026_06_13_16_27_10_asia_shanghai",
      operator_authorization_received:true,
      operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_route_enabled:true,
      operator_review_acknowledgement_fixture_count:8,
      operator_review_acknowledgement_requested_fixture_count:8,
      blocked_operator_review_acknowledgement_fixture_count:8,
      noop_operator_review_acknowledgement_fixture_count:8,
      allowed_operator_review_acknowledgement_fixture_count:0,
      accepted_operator_review_acknowledgement_fixture_count:0,
      operator_review_acknowledgement_performed_count:0,
      operator_review_acknowledgement_allowed:false,
      operator_review_acknowledgement_accepted:false,
      operator_review_acknowledgement_recorded:false,
      operator_review_acknowledgement_persisted:false,
      operator_review_acknowledgement_materialized:false,
      operator_review_acknowledgement_filesystem_written:false,
      operator_review_acknowledgement_delivered:false,
      operator_review_acknowledgement_identity_accepted:false,
      operator_review_acknowledgement_signature_accepted:false,
      operator_review_acknowledgement_authorizes_dispatch_count:0,
      operator_review_acknowledgement_authorizes_execution_count:0,
      operator_review_acknowledgement_authorizes_live_count:0,
      operator_approval_recorded:false,
      operator_identity_accepted:false,
      readback_index_recorded_count:0,
      readback_index_persisted_count:0,
      readback_index_materialized_count:0,
      dispatch_allowed_count:0,
      dispatch_performed_count:0,
      execution_allowed_count:0,
      execution_performed_count:0,
      context_injection_performed_count:0,
      provider_invoked_count:0,
      model_invoked_count:0,
      memory_store_write_performed_count:0,
      external_kg_adapter_read_performed_count:0,
      live_kg_write_performed_count:0,
      credential_read_count:0,
      secret_file_read_count:0,
      channel_send_performed_count:0,
      canary_harness_armed:false,
      canary_live_enabled:false,
      current_live_enabled_lane_count:15,
      enablement_lane_count:18,
      ready_enablement_lane_count:18,
      source_contract:{
        source:$native_gateway_source,
        source_sha256:$native_gateway_sha256,
        test_log:$test_log,
        contract:"hepta-native-gateway-operator-canary-controlled-request-harness-operator-review-acknowledgement-non-acceptance-route-v1",
        source_pattern_present:true,
        compile_checked_by_tests:true,
        route_handler_present:true,
        accepts_operator_review_acknowledgement:false,
        records_operator_review_acknowledgement:false,
        persists_operator_review_acknowledgement:false,
        accepts_operator_identity:false,
        accepts_operator_signature:false,
        records_operator_approval:false,
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
        operator_review_acknowledgement_performed:false,
        operator_review_acknowledgement_recorded:false,
        operator_review_acknowledgement_persisted:false,
        operator_review_acknowledgement_materialized:false,
        operator_review_acknowledgement_filesystem_written:false,
        operator_review_acknowledgement_delivered:false,
        operator_review_acknowledgement_accepted:false,
        operator_approval_recorded:false,
        operator_identity_accepted:false,
        readback_index_recorded:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_route_gate"
  and .source_operator_review_index_route_ready == true
  and .source_operator_acknowledgement_gate_ready == true
  and .source_operator_acknowledgement_gate_status == "blocked"
  and .source_route_wired == true
  and .source_route_count_expected == 105
  and .source_route_tested_by_native_gateway_unit_test == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_route_enabled == true
  and .operator_review_acknowledgement_fixture_count == 8
  and .operator_review_acknowledgement_requested_fixture_count == 8
  and .blocked_operator_review_acknowledgement_fixture_count == 8
  and .noop_operator_review_acknowledgement_fixture_count == 8
  and .allowed_operator_review_acknowledgement_fixture_count == 0
  and .accepted_operator_review_acknowledgement_fixture_count == 0
  and .operator_review_acknowledgement_performed_count == 0
  and .operator_review_acknowledgement_allowed == false
  and .operator_review_acknowledgement_accepted == false
  and .operator_review_acknowledgement_recorded == false
  and .operator_review_acknowledgement_persisted == false
  and .operator_review_acknowledgement_materialized == false
  and .operator_review_acknowledgement_filesystem_written == false
  and .operator_review_acknowledgement_delivered == false
  and .operator_review_acknowledgement_identity_accepted == false
  and .operator_review_acknowledgement_signature_accepted == false
  and .operator_review_acknowledgement_authorizes_dispatch_count == 0
  and .operator_review_acknowledgement_authorizes_execution_count == 0
  and .operator_review_acknowledgement_authorizes_live_count == 0
  and .operator_approval_recorded == false
  and .operator_identity_accepted == false
  and .readback_index_persisted_count == 0
  and .dispatch_performed_count == 0
  and .execution_performed_count == 0
  and .provider_invoked_count == 0
  and .model_invoked_count == 0
  and .memory_store_write_performed_count == 0
  and .external_kg_adapter_read_performed_count == 0
  and .live_kg_write_performed_count == 0
  and .credential_read_count == 0
  and .secret_file_read_count == 0
  and .channel_send_performed_count == 0
  and .canary_harness_armed == false
  and .canary_live_enabled == false
  and .current_live_enabled_lane_count == 15
  and .enablement_lane_count == 18
  and .ready_enablement_lane_count == 18
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG operator canary controlled request harness operator review acknowledgement non-acceptance route gate passed"
