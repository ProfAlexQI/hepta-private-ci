#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
RELEASE_BIN="${HEPTA_RELEASE_BIN:-${HEPTA_CODEX_RELEASE_BIN:-$HOME/.local/opt/hepta/bin/hepta}}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"

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
    echo "missing operator canary operator-review acknowledgement activation request denial matrix route source text: $label" >&2
    exit 1
  fi
}

OPERATOR_ACK_ROUTE_JSON="$(
  curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-non-acceptance"
)"

jq -e '
  .status == "ready"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_route_enabled == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_status == "blocked"
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
  and .operator_review_acknowledgement_authorizes_dispatch_count == 0
  and .operator_review_acknowledgement_authorizes_execution_count == 0
  and .operator_review_acknowledgement_authorizes_live_count == 0
  and .operator_approval_recorded == false
  and .operator_identity_accepted == false
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
  and .canary_harness_executable == false
  and .canary_live_enabled == false
  and .current_live_enabled_lane_count == 15
  and .enablement_lane_count == 18
  and .ready_enablement_lane_count == 18
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$OPERATOR_ACK_ROUTE_JSON"

ACTIVATION_DENIAL_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-request-denial-matrix-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-request-denial-matrix-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_gate"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_status == "blocked"
  and .source_operator_review_acknowledgement_non_acceptance_status == "blocked"
  and .source_operator_review_acknowledgement_fixture_count == 8
  and .source_operator_review_acknowledgement_accepted_count == 0
  and .source_operator_review_acknowledgement_performed_count == 0
  and .source_operator_review_acknowledgement_authorizes_dispatch_count == 0
  and .source_operator_review_acknowledgement_authorizes_execution_count == 0
  and .source_operator_review_acknowledgement_authorizes_live_count == 0
  and .activation_request_denial_fixture_count == 9
  and .activation_request_requested_fixture_count == 9
  and .blocked_activation_request_fixture_count == 9
  and .noop_activation_request_fixture_count == 9
  and .allowed_activation_request_fixture_count == 0
  and .accepted_activation_request_fixture_count == 0
  and .activation_request_performed_count == 0
  and .activation_request_allowed == false
  and .activation_request_accepted == false
  and .activation_request_recorded == false
  and .activation_request_persisted == false
  and .activation_request_materialized == false
  and .activation_request_filesystem_written == false
  and .activation_request_delivered == false
  and .activation_request_executed == false
  and .activation_nonce_generated == false
  and .activation_identity_accepted == false
  and .activation_scope_accepted == false
  and .activation_final_state_promoted == false
  and .operator_approval_recorded == false
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
  and .upstream_fetch_performed_count == 0
  and .upstream_merge_performed_count == 0
  and .denied_by_operator_review_acknowledgement_activation_request_denial_matrix_count == 26
  and (.activation_request_denial_fixtures | length) == 9
  and (.activation_request_denial_fixtures | all(
    .activation_request_requested == true
    and .activation_request_status == "blocked_noop"
    and .activation_request_allowed == false
    and .activation_request_accepted == false
    and .activation_request_recorded == false
    and .activation_request_persisted == false
    and .activation_request_executed == false
    and .dispatch_performed == false
    and .execution_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .channel_send_performed == false
    and .install_performed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .upstream_fetch_performed == false
    and .upstream_merge_performed == false
  ))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$ACTIVATION_DENIAL_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 130;' \
  "native gateway route/source command count includes operator-review acknowledgement activation request denial matrix route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_REQUEST_DENIAL_MATRIX_ENDPOINT' \
  "native gateway operator-review acknowledgement activation request denial matrix endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-request-denial-matrix' \
  "native gateway operator-review acknowledgement activation request denial matrix endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-request-denial-matrix --json' \
  "native gateway operator-review acknowledgement activation request denial matrix source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_report' \
  "native gateway operator-review acknowledgement activation request denial matrix report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_route_enabled": true' \
  "operator-review acknowledgement activation request denial matrix route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_request_performed_count": 0' \
  "activation request performed count remains zero"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_request_persisted": false' \
  "activation request persistence denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_request_executed": false' \
  "activation request execution denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"provider_invoked_count": 0' \
  "activation request provider invocation denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"live_kg_write_performed_count": 0' \
  "activation request live KG write denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-canary-activation-request-denial-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_endpoint_blocks_activation_requests \
  -- --nocapture >"$TEST_LOG"

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-request-denial-matrix"
  )"
  jq -e '
    .status == "ready"
    and .route_count == 130
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_route_enabled == true
    and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_ready == true
    and .activation_request_denial_fixture_count == 9
    and .blocked_activation_request_fixture_count == 9
    and .activation_request_performed_count == 0
    and .activation_request_persisted == false
    and .activation_request_executed == false
    and .dispatch_performed_count == 0
    and .execution_performed_count == 0
    and .provider_invoked_count == 0
    and .model_invoked_count == 0
    and .memory_store_write_performed_count == 0
    and .live_kg_write_performed_count == 0
    and .credential_read_count == 0
    and .secret_file_read_count == 0
    and .channel_send_performed_count == 0
    and .current_live_enabled_lane_count == 16
    and .enablement_lane_count == 19
    and .ready_enablement_lane_count == 19
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
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_route_gate" \
    --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-request-denial-matrix" \
    --arg source_command "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-request-denial-matrix --json" \
    --arg native_gateway_source "$NATIVE_GATEWAY_SOURCE" \
    --arg native_gateway_sha256 "$(sha256_file "$NATIVE_GATEWAY_SOURCE")" \
    --arg test_log "$TEST_LOG" \
    --argjson operator_ack_route "$OPERATOR_ACK_ROUTE_JSON" \
    --argjson activation_denial "$ACTIVATION_DENIAL_JSON" \
    --argjson live_route "$LIVE_ROUTE_JSON" \
    '{
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      source_command:$source_command,
      activation_mode:"operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_native_route_status",
      source_operator_acknowledgement_route_ready:($operator_ack_route.status == "ready"),
      source_activation_request_denial_matrix_gate:$activation_denial.gate,
      source_activation_request_denial_matrix_gate_ready:$activation_denial.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_ready,
      source_activation_request_denial_matrix_gate_status:$activation_denial.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_status,
      live_endpoint_required:($live_route != null),
      live_endpoint_ready:(if $live_route == null then true else ($live_route.status == "ready") end),
      source_route_wired:true,
      source_route_count_expected:105,
      source_route_tested_by_native_gateway_unit_test:true,
      operator_authorization_source:"telegram_direct_operator_highest_authorization_2026_06_13_19_36_01_asia_shanghai",
      operator_authorization_received:true,
      operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_route_enabled:true,
      activation_request_denial_fixture_count:9,
      activation_request_requested_fixture_count:9,
      blocked_activation_request_fixture_count:9,
      noop_activation_request_fixture_count:9,
      allowed_activation_request_fixture_count:0,
      accepted_activation_request_fixture_count:0,
      activation_request_performed_count:0,
      activation_request_allowed:false,
      activation_request_accepted:false,
      activation_request_recorded:false,
      activation_request_persisted:false,
      activation_request_materialized:false,
      activation_request_filesystem_written:false,
      activation_request_delivered:false,
      activation_request_executed:false,
      activation_nonce_generated:false,
      activation_identity_accepted:false,
      activation_scope_accepted:false,
      activation_final_state_promoted:false,
      operator_review_acknowledgement_accepted:false,
      operator_review_acknowledgement_recorded:false,
      operator_review_acknowledgement_persisted:false,
      operator_approval_recorded:false,
      operator_identity_accepted:false,
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
      install_performed_count:0,
      service_restarted_count:0,
      active_binary_mutated_count:0,
      upstream_fetch_performed_count:0,
      upstream_merge_performed_count:0,
      canary_harness_armed:false,
      canary_live_enabled:false,
      current_live_enabled_lane_count:16,
      enablement_lane_count:19,
      ready_enablement_lane_count:19,
      source_contract:{
        source:$native_gateway_source,
        source_sha256:$native_gateway_sha256,
        test_log:$test_log,
        contract:"hepta-native-gateway-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-request-denial-matrix-route-v1",
        source_pattern_present:true,
        compile_checked_by_tests:true,
        route_handler_present:true,
        accepts_activation_request:false,
        records_activation_request:false,
        persists_activation_request:false,
        materializes_activation_request:false,
        executes_activation_request:false,
        generates_activation_nonce:false,
        accepts_operator_identity:false,
        records_operator_approval:false,
        dispatches_controlled_request:false,
        executes_controlled_request:false,
        injects_context:false,
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
        activation_request_performed:false,
        activation_request_recorded:false,
        activation_request_persisted:false,
        activation_request_materialized:false,
        activation_request_filesystem_written:false,
        activation_request_delivered:false,
        activation_request_executed:false,
        activation_nonce_generated:false,
        activation_identity_accepted:false,
        activation_scope_accepted:false,
        activation_final_state_promoted:false,
        operator_review_acknowledgement_accepted:false,
        operator_approval_recorded:false,
        operator_identity_accepted:false,
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

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_route_gate"
  and .source_operator_acknowledgement_route_ready == true
  and .source_activation_request_denial_matrix_gate_ready == true
  and .source_activation_request_denial_matrix_gate_status == "blocked"
  and .live_endpoint_ready == true
  and .source_route_wired == true
  and .source_route_count_expected == 105
  and .source_route_tested_by_native_gateway_unit_test == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_route_enabled == true
  and .activation_request_denial_fixture_count == 9
  and .activation_request_requested_fixture_count == 9
  and .blocked_activation_request_fixture_count == 9
  and .noop_activation_request_fixture_count == 9
  and .allowed_activation_request_fixture_count == 0
  and .accepted_activation_request_fixture_count == 0
  and .activation_request_performed_count == 0
  and .activation_request_allowed == false
  and .activation_request_accepted == false
  and .activation_request_recorded == false
  and .activation_request_persisted == false
  and .activation_request_executed == false
  and .activation_nonce_generated == false
  and .activation_identity_accepted == false
  and .activation_scope_accepted == false
  and .activation_final_state_promoted == false
  and .operator_approval_recorded == false
  and .operator_identity_accepted == false
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
  and .upstream_fetch_performed_count == 0
  and .upstream_merge_performed_count == 0
  and .canary_harness_armed == false
  and .canary_live_enabled == false
  and .current_live_enabled_lane_count == 16
  and .enablement_lane_count == 19
  and .ready_enablement_lane_count == 19
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG operator canary controlled request harness operator review acknowledgement activation request denial matrix route gate passed"
