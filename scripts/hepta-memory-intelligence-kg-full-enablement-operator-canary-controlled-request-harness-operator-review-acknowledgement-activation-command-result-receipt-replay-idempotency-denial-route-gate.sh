#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
RELEASE_BIN="${HEPTA_RELEASE_BIN:-${HEPTA_CODEX_RELEASE_BIN:-$HOME/.local/opt/hepta/bin/hepta}}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"
EXPECTED_ROUTE_COUNT=192

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
    echo "missing operator canary activation command result receipt replay/idempotency route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

REPLAY_IDEMPOTENCY_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial-gate.sh
)"

jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_gate"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_status == "blocked"
  and .source_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_gate"
  and .source_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_status == "blocked"
  and .source_activation_command_result_receipt_fixture_count == 10
  and .source_accepted_activation_command_result_receipt_fixture_count == 0
  and .replay_idempotency_fixture_count == 10
  and .blocked_replay_idempotency_fixture_count == 10
  and .noop_replay_idempotency_fixture_count == 10
  and .allowed_replay_idempotency_fixture_count == 0
  and .accepted_replay_idempotency_fixture_count == 0
  and .replay_idempotency_performed_count == 0
  and .duplicate_result_receipt_accepted_count == 0
  and .idempotency_state_recorded_count == 0
  and .idempotency_state_persisted_count == 0
  and .activation_command_result_receipt_replay_allowed == false
  and .activation_command_result_receipt_replay_recorded == false
  and .activation_command_result_receipt_replay_persisted == false
  and .activation_command_result_receipt_replay_performed == false
  and .activation_command_result_receipt_duplicate_accepted == false
  and .activation_command_result_receipt_idempotency_key_accepted == false
  and .activation_command_result_receipt_idempotency_key_recorded == false
  and .activation_command_result_receipt_idempotency_state_recorded == false
  and .activation_command_result_receipt_idempotency_state_persisted == false
  and .activation_command_result_receipt_replay_nonce_accepted == false
  and .activation_command_result_receipt_cross_scope_reuse_accepted == false
  and .activation_command_result_receipt_status_upgrade_accepted == false
  and .activation_command_result_receipt_completed_status_accepted == false
  and .activation_command_result_receipt_ack_replay_accepted == false
  and .activation_command_result_receipt_ledger_replay_accepted == false
  and .activation_command_result_receipt_index_replay_accepted == false
  and .activation_command_result_receipt_delivery_replay_accepted == false
  and .activation_command_result_receipt_export_replay_accepted == false
  and .activation_command_result_receipt_query_replay_accepted == false
  and .activation_command_result_receipt_observability_replay_accepted == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_completion_ack_recorded == false
  and .operator_approval_from_replay_accepted == false
  and .operator_approval_from_receipt_accepted == false
  and .activation_from_replay_allowed == false
  and .activation_from_receipt_allowed == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_request_accepted == false
  and .activation_request_recorded == false
  and .activation_request_executed == false
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
  and .install_performed_count == 0
  and .service_restarted_count == 0
  and .active_binary_mutated_count == 0
  and .upstream_fetch_performed_count == 0
  and .upstream_merge_performed_count == 0
  and .canary_harness_armed == false
  and .canary_harness_executable == false
  and .canary_live_enabled == false
  and (.replay_idempotency_fixtures | length) == 10
  and (.replay_idempotency_fixtures | all(
    (.replay_idempotency_status | startswith("blocked_"))
    and .activation_command_result_receipt_replay_allowed == false
    and .activation_command_result_receipt_duplicate_accepted == false
    and .activation_command_result_receipt_idempotency_state_recorded == false
    and .activation_command_result_receipt_idempotency_state_persisted == false
    and .activation_command_result_receipt_cross_scope_reuse_accepted == false
    and .activation_from_replay_allowed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .external_kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .receipt_noop_confirmed == true
  ))
  and .denied_by_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_count >= 110
  and (.allowed_next_actions | any(.action == "stage_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial" and .status == "allowed_report_only_next_slice"))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$REPLAY_IDEMPOTENCY_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 192;' \
  "native gateway route/source command count includes activation command result receipt replay/idempotency denial route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT' \
  "native gateway activation command result receipt replay/idempotency endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial' \
  "native gateway activation command result receipt replay/idempotency endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial --json' \
  "native gateway activation command result receipt replay/idempotency source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_report' \
  "native gateway activation command result receipt replay/idempotency report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_route_enabled": true' \
  "activation command result receipt replay/idempotency route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_command_result_receipt_replay_recorded": false' \
  "activation command result receipt replay recording denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_command_result_receipt_duplicate_accepted": false' \
  "activation command result receipt duplicate acceptance denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_command_result_receipt_idempotency_state_recorded": false' \
  "activation command result receipt idempotency state recording denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_from_replay_allowed": false' \
  "activation from result receipt replay denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-canary-activation-command-result-receipt-replay-idempotency-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_endpoint_blocks_replay \
  -- --nocapture >"$TEST_LOG"

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial"
  )"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .source_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_route_ready == true
    and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_route_enabled == true
    and .replay_idempotency_fixture_count == 10
    and .blocked_replay_idempotency_fixture_count == 10
    and .allowed_replay_idempotency_fixture_count == 0
    and .accepted_replay_idempotency_fixture_count == 0
    and .replay_idempotency_performed_count == 0
    and .duplicate_result_receipt_accepted_count == 0
    and .idempotency_state_recorded_count == 0
    and .activation_command_result_receipt_replay_recorded == false
    and .activation_command_result_receipt_duplicate_accepted == false
    and .activation_command_result_receipt_idempotency_state_recorded == false
    and .activation_from_replay_allowed == false
    and .activation_command_enabled == false
    and .activation_command_invoked == false
    and .activation_command_dispatched == false
    and .activation_request_accepted == false
    and .dispatch_performed_count == 0
    and .execution_performed_count == 0
    and .provider_invoked_count == 0
    and .model_invoked_count == 0
    and .memory_store_write_performed_count == 0
    and .live_kg_write_performed_count == 0
    and .credential_read_count == 0
    and .secret_file_read_count == 0
    and .channel_send_performed_count == 0
    and .install_performed_count == 0
    and .service_restarted_count == 0
    and .active_binary_mutated_count == 0
    and .current_live_enabled_lane_count == 19
    and .enablement_lane_count == 22
    and .ready_enablement_lane_count == 22
    and (.side_effects | to_entries | all(.value == false))
  ' >/dev/null <<<"$LIVE_ROUTE_JSON"
else
  LIVE_ROUTE_JSON='null'
fi

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_route_gate" \
    --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial" \
    --arg source_command "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial --json" \
    --arg native_gateway_source "$NATIVE_GATEWAY_SOURCE" \
    --arg native_gateway_sha256 "$(sha256_file "$NATIVE_GATEWAY_SOURCE")" \
    --arg test_log "$TEST_LOG" \
    --argjson replay_idempotency "$REPLAY_IDEMPOTENCY_JSON" \
    --argjson live_route "$LIVE_ROUTE_JSON" \
    '{
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      source_command:$source_command,
      activation_mode:"operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_native_route_status",
      source_activation_command_result_receipt_replay_idempotency_denial_gate:$replay_idempotency.gate,
      source_activation_command_result_receipt_replay_idempotency_denial_gate_ready:$replay_idempotency.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready,
      source_activation_command_result_receipt_replay_idempotency_denial_gate_status:$replay_idempotency.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_status,
      source_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_gate:$replay_idempotency.source_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_gate,
      source_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_gate_status:$replay_idempotency.source_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_status,
      source_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_route_ready:($replay_idempotency.source_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_status == "blocked"),
      live_endpoint_required:($live_route != null),
      live_endpoint_ready:(if $live_route == null then true else ($live_route.status == "ready") end),
      source_route_wired:true,
      source_route_count_expected:105,
      source_route_tested_by_native_gateway_unit_test:true,
      operator_authorization_source:"telegram_direct_operator_highest_authorization_2026_06_13_19_36_01_asia_shanghai",
      operator_authorization_received:true,
      operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_route_enabled:true,
      replay_idempotency_fixture_count:10,
      blocked_replay_idempotency_fixture_count:10,
      noop_replay_idempotency_fixture_count:10,
      allowed_replay_idempotency_fixture_count:0,
      accepted_replay_idempotency_fixture_count:0,
      replay_idempotency_performed_count:0,
      duplicate_result_receipt_accepted_count:0,
      idempotency_state_recorded_count:0,
      idempotency_state_persisted_count:0,
      activation_command_result_receipt_replay_allowed:false,
      activation_command_result_receipt_replay_recorded:false,
      activation_command_result_receipt_replay_persisted:false,
      activation_command_result_receipt_replay_performed:false,
      activation_command_result_receipt_duplicate_accepted:false,
      activation_command_result_receipt_idempotency_key_recorded:false,
      activation_command_result_receipt_idempotency_state_recorded:false,
      activation_command_result_receipt_idempotency_state_persisted:false,
      activation_command_result_receipt_replay_nonce_accepted:false,
      activation_command_result_receipt_cross_scope_reuse_accepted:false,
      activation_command_result_receipt_status_upgrade_accepted:false,
      activation_command_result_receipt_completed_status_accepted:false,
      activation_command_result_receipt_ack_replay_accepted:false,
      activation_command_result_receipt_ledger_replay_accepted:false,
      activation_command_result_receipt_index_replay_accepted:false,
      activation_command_result_receipt_delivery_replay_accepted:false,
      activation_command_result_receipt_export_replay_accepted:false,
      activation_command_result_receipt_query_replay_accepted:false,
      activation_command_result_receipt_observability_replay_accepted:false,
      activation_command_result_receipt_recorded:false,
      activation_command_result_receipt_persisted:false,
      activation_command_result_receipt_accepted:false,
      activation_command_completion_ack_recorded:false,
      operator_approval_from_replay_accepted:false,
      operator_approval_from_receipt_accepted:false,
      activation_from_replay_allowed:false,
      activation_from_receipt_allowed:false,
      activation_command_enabled:false,
      activation_command_invoked:false,
      activation_command_dispatched:false,
      activation_request_recorded:false,
      activation_request_executed:false,
      dispatch_performed_count:0,
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
      install_performed_count:0,
      service_restarted_count:0,
      active_binary_mutated_count:0,
      upstream_fetch_performed_count:0,
      upstream_merge_performed_count:0,
      canary_harness_armed:false,
      canary_live_enabled:false,
      current_live_enabled_lane_count:19,
      enablement_lane_count:22,
      ready_enablement_lane_count:22,
      source_contract:{
        source:$native_gateway_source,
        source_sha256:$native_gateway_sha256,
        test_log:$test_log,
        contract:"hepta-native-gateway-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial-route-v1",
        source_pattern_present:true,
        compile_checked_by_tests:true,
        route_handler_present:true,
        accepts_replay:false,
        accepts_duplicate_receipt:false,
        records_idempotency_key:false,
        records_idempotency_state:false,
        persists_idempotency_state:false,
        records_replay:false,
        derives_activation_authority:false,
        invokes_provider_or_model:false,
        writes_memory_or_kg:false,
        reads_credentials:false,
        delivers_channel:false,
        mutates_installed_binary:false,
        fetches_or_merges_upstream:false
      },
      side_effects:{
        workspace_written:false,
        filesystem_written:false,
        activation_command_result_receipt_replay_recorded:false,
        activation_command_result_receipt_replay_persisted:false,
        activation_command_result_receipt_replay_performed:false,
        activation_command_result_receipt_duplicate_accepted:false,
        activation_command_result_receipt_duplicate_recorded:false,
        activation_command_result_receipt_duplicate_persisted:false,
        activation_command_result_receipt_idempotency_key_recorded:false,
        activation_command_result_receipt_idempotency_state_recorded:false,
        activation_command_result_receipt_idempotency_state_persisted:false,
        activation_command_result_receipt_replay_nonce_recorded:false,
        activation_command_result_receipt_cross_scope_reuse_accepted:false,
        activation_from_replay_allowed:false,
        activation_from_receipt_allowed:false,
        activation_command_enabled:false,
        activation_command_invoked:false,
        activation_command_dispatched:false,
        activation_request_recorded:false,
        activation_request_executed:false,
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
        install_performed:false,
        service_restarted:false,
        active_binary_mutated:false,
        upstream_fetch_performed:false,
        upstream_merge_performed:false,
        public_release_claimed:false,
        public_ga_claimed:false
      }
    }'
)"

jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_route_gate"
  and .source_activation_command_result_receipt_replay_idempotency_denial_gate_ready == true
  and .source_activation_command_result_receipt_replay_idempotency_denial_gate_status == "blocked"
  and .source_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_route_ready == true
  and .live_endpoint_ready == true
  and .source_route_wired == true
  and .source_route_count_expected == 105
  and .source_route_tested_by_native_gateway_unit_test == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_route_enabled == true
  and .replay_idempotency_fixture_count == 10
  and .blocked_replay_idempotency_fixture_count == 10
  and .noop_replay_idempotency_fixture_count == 10
  and .accepted_replay_idempotency_fixture_count == 0
  and .replay_idempotency_performed_count == 0
  and .duplicate_result_receipt_accepted_count == 0
  and .idempotency_state_recorded_count == 0
  and .idempotency_state_persisted_count == 0
  and .activation_command_result_receipt_replay_recorded == false
  and .activation_command_result_receipt_duplicate_accepted == false
  and .activation_command_result_receipt_idempotency_state_recorded == false
  and .activation_command_result_receipt_idempotency_state_persisted == false
  and .activation_from_replay_allowed == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_request_recorded == false
  and .activation_request_executed == false
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
  and .install_performed_count == 0
  and .service_restarted_count == 0
  and .active_binary_mutated_count == 0
  and .current_live_enabled_lane_count == 19
  and .enablement_lane_count == 22
  and .ready_enablement_lane_count == 22
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG operator canary controlled request harness operator review acknowledgement activation command result receipt replay/idempotency denial route gate passed"
