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
    echo "missing operator canary activation command result receipt cancellation/supersession route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

CANCELLATION_SUPERSESSION_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_gate"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_status == "blocked"
  and .source_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_gate"
  and .source_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_status == "blocked"
  and .source_ordering_monotonicity_fixture_count == 10
  and .source_accepted_ordering_monotonicity_fixture_count == 0
  and .source_ordering_monotonicity_performed_count == 0
  and .source_sequence_cursor_recorded_count == 0
  and .source_monotonicity_state_recorded_count == 0
  and .cancellation_supersession_fixture_count == 10
  and .blocked_cancellation_supersession_fixture_count == 10
  and .noop_cancellation_supersession_fixture_count == 10
  and .allowed_cancellation_supersession_fixture_count == 0
  and .accepted_cancellation_supersession_fixture_count == 0
  and .cancellation_fixture_count == 6
  and .supersession_fixture_count == 5
  and .cancellation_performed_count == 0
  and .supersession_performed_count == 0
  and .replacement_receipt_accepted_count == 0
  and .replacement_receipt_recorded_count == 0
  and .replacement_receipt_persisted_count == 0
  and .tombstone_recorded_count == 0
  and .delete_marker_recorded_count == 0
  and .activation_command_result_receipt_cancellation_allowed == false
  and .activation_command_result_receipt_cancellation_recorded == false
  and .activation_command_result_receipt_cancellation_persisted == false
  and .activation_command_result_receipt_cancellation_request_accepted == false
  and .activation_command_result_receipt_supersession_allowed == false
  and .activation_command_result_receipt_supersession_recorded == false
  and .activation_command_result_receipt_supersession_persisted == false
  and .activation_command_result_receipt_supersession_request_accepted == false
  and .activation_command_result_receipt_replacement_receipt_accepted == false
  and .activation_command_result_receipt_replacement_receipt_recorded == false
  and .activation_command_result_receipt_replacement_receipt_persisted == false
  and .activation_command_result_receipt_tombstone_recorded == false
  and .activation_command_result_receipt_delete_marker_recorded == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_completion_ack_recorded == false
  and .operator_approval_from_cancellation_accepted == false
  and .operator_approval_from_supersession_accepted == false
  and .activation_from_cancellation_allowed == false
  and .activation_from_supersession_allowed == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_request_accepted == false
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
  and (.cancellation_supersession_fixtures | length) == 10
  and (.cancellation_supersession_fixtures | all(
    (.cancellation_supersession_status | startswith("blocked_"))
    and .activation_command_result_receipt_cancellation_allowed == false
    and .activation_command_result_receipt_cancellation_recorded == false
    and .activation_command_result_receipt_supersession_allowed == false
    and .activation_command_result_receipt_supersession_recorded == false
    and .activation_command_result_receipt_replacement_receipt_accepted == false
    and .activation_command_result_receipt_replacement_receipt_recorded == false
    and .activation_command_result_receipt_tombstone_recorded == false
    and .activation_command_result_receipt_delete_marker_recorded == false
    and .operator_approval_from_cancellation_accepted == false
    and .operator_approval_from_supersession_accepted == false
    and .activation_from_cancellation_allowed == false
    and .activation_from_supersession_allowed == false
    and .activation_command_enabled == false
    and .activation_command_invoked == false
    and .activation_command_dispatched == false
    and .activation_request_accepted == false
    and .activation_request_executed == false
    and .dispatch_performed == false
    and .execution_performed == false
    and .context_injection_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .external_kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .channel_send_performed == false
    and .install_performed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .upstream_fetch_performed == false
    and .upstream_merge_performed == false
    and .receipt_noop_confirmed == true
  ))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$CANCELLATION_SUPERSESSION_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 111;' \
  "native gateway route/source command count includes activation command result receipt cancellation/supersession denial route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT' \
  "native gateway activation command result receipt cancellation/supersession endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial' \
  "native gateway activation command result receipt cancellation/supersession endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial --json' \
  "native gateway activation command result receipt cancellation/supersession source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_report' \
  "native gateway activation command result receipt cancellation/supersession report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_route_enabled": true' \
  "activation command result receipt cancellation/supersession route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_command_result_receipt_cancellation_recorded"' \
  "activation command result receipt cancellation recording denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_command_result_receipt_supersession_recorded"' \
  "activation command result receipt supersession recording denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_command_result_receipt_replacement_receipt_recorded"' \
  "activation command result receipt replacement recording denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_from_cancellation_allowed"' \
  "activation from result receipt cancellation denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-canary-activation-command-result-receipt-cancellation-supersession-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_endpoint_blocks_cancellation \
  -- --nocapture >"$TEST_LOG"

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial"
  )"
  jq -e '
    .status == "ready"
    and .route_count == 111
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .source_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_route_ready == true
    and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_route_enabled == true
    and .cancellation_supersession_fixture_count == 10
    and .blocked_cancellation_supersession_fixture_count == 10
    and .allowed_cancellation_supersession_fixture_count == 0
    and .accepted_cancellation_supersession_fixture_count == 0
    and .cancellation_performed_count == 0
    and .supersession_performed_count == 0
    and .replacement_receipt_recorded_count == 0
    and .tombstone_recorded_count == 0
    and .delete_marker_recorded_count == 0
    and .activation_command_result_receipt_cancellation_recorded == false
    and .activation_command_result_receipt_supersession_recorded == false
    and .activation_command_result_receipt_replacement_receipt_recorded == false
    and .activation_from_cancellation_allowed == false
    and .activation_from_supersession_allowed == false
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
    and .current_live_enabled_lane_count == 21
    and .enablement_lane_count == 24
    and .ready_enablement_lane_count == 24
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
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_route_gate" \
    --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial" \
    --arg source_command "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial --json" \
    --arg native_gateway_source "$NATIVE_GATEWAY_SOURCE" \
    --arg native_gateway_sha256 "$(sha256_file "$NATIVE_GATEWAY_SOURCE")" \
    --arg test_log "$TEST_LOG" \
    --argjson cancellation_supersession "$CANCELLATION_SUPERSESSION_JSON" \
    --argjson live_route "$LIVE_ROUTE_JSON" \
    '{
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      source_command:$source_command,
      activation_mode:"operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_native_route_status",
      source_activation_command_result_receipt_cancellation_supersession_denial_gate:$cancellation_supersession.gate,
      source_activation_command_result_receipt_cancellation_supersession_denial_gate_ready:$cancellation_supersession.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_ready,
      source_activation_command_result_receipt_cancellation_supersession_denial_gate_status:$cancellation_supersession.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_status,
      source_ordering_monotonicity_fixture_count:$cancellation_supersession.source_ordering_monotonicity_fixture_count,
      source_accepted_ordering_monotonicity_fixture_count:$cancellation_supersession.source_accepted_ordering_monotonicity_fixture_count,
      source_route_wired:true,
      source_route_count_expected:105,
      source_route_tested_by_native_gateway_unit_test:true,
      native_gateway_source:$native_gateway_source,
      native_gateway_sha256:$native_gateway_sha256,
      native_gateway_unit_test_log:$test_log,
      live_endpoint_required:($live_route != null),
      live_endpoint_ready:(if $live_route == null then false else true end),
      live_endpoint_status:(if $live_route == null then "not_required" else $live_route.status end),
      live_route_count:(if $live_route == null then null else $live_route.route_count end),
      live_missing_route_count:(if $live_route == null then null else $live_route.missing_route_count end),
      operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_route_enabled:true,
      cancellation_supersession_fixture_count:$cancellation_supersession.cancellation_supersession_fixture_count,
      blocked_cancellation_supersession_fixture_count:$cancellation_supersession.blocked_cancellation_supersession_fixture_count,
      noop_cancellation_supersession_fixture_count:$cancellation_supersession.noop_cancellation_supersession_fixture_count,
      accepted_cancellation_supersession_fixture_count:$cancellation_supersession.accepted_cancellation_supersession_fixture_count,
      cancellation_performed_count:$cancellation_supersession.cancellation_performed_count,
      supersession_performed_count:$cancellation_supersession.supersession_performed_count,
      replacement_receipt_recorded_count:$cancellation_supersession.replacement_receipt_recorded_count,
      tombstone_recorded_count:$cancellation_supersession.tombstone_recorded_count,
      delete_marker_recorded_count:$cancellation_supersession.delete_marker_recorded_count,
      activation_command_result_receipt_cancellation_recorded:$cancellation_supersession.activation_command_result_receipt_cancellation_recorded,
      activation_command_result_receipt_supersession_recorded:$cancellation_supersession.activation_command_result_receipt_supersession_recorded,
      activation_command_result_receipt_replacement_receipt_recorded:$cancellation_supersession.activation_command_result_receipt_replacement_receipt_recorded,
      activation_from_cancellation_allowed:$cancellation_supersession.activation_from_cancellation_allowed,
      activation_from_supersession_allowed:$cancellation_supersession.activation_from_supersession_allowed,
      activation_command_enabled:$cancellation_supersession.activation_command_enabled,
      activation_command_invoked:$cancellation_supersession.activation_command_invoked,
      activation_command_dispatched:$cancellation_supersession.activation_command_dispatched,
      activation_request_recorded:$cancellation_supersession.activation_request_recorded,
      activation_request_executed:$cancellation_supersession.activation_request_executed,
      dispatch_performed_count:$cancellation_supersession.dispatch_performed_count,
      execution_performed_count:$cancellation_supersession.execution_performed_count,
      provider_invoked_count:$cancellation_supersession.provider_invoked_count,
      model_invoked_count:$cancellation_supersession.model_invoked_count,
      memory_store_write_performed_count:$cancellation_supersession.memory_store_write_performed_count,
      external_kg_adapter_read_performed_count:$cancellation_supersession.external_kg_adapter_read_performed_count,
      live_kg_write_performed_count:$cancellation_supersession.live_kg_write_performed_count,
      credential_read_count:$cancellation_supersession.credential_read_count,
      secret_file_read_count:$cancellation_supersession.secret_file_read_count,
      channel_send_performed_count:$cancellation_supersession.channel_send_performed_count,
      install_performed_count:$cancellation_supersession.install_performed_count,
      service_restarted_count:$cancellation_supersession.service_restarted_count,
      active_binary_mutated_count:$cancellation_supersession.active_binary_mutated_count,
      current_live_enabled_lane_count:21,
      enablement_lane_count:24,
      ready_enablement_lane_count:24,
      side_effects:{
        activation_command_result_receipt_cancellation_recorded:false,
        activation_command_result_receipt_supersession_recorded:false,
        activation_command_result_receipt_replacement_receipt_recorded:false,
        activation_command_result_receipt_tombstone_recorded:false,
        activation_from_cancellation_allowed:false,
        activation_from_supersession_allowed:false,
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

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_route_gate"
  and .source_activation_command_result_receipt_cancellation_supersession_denial_gate_ready == true
  and .source_activation_command_result_receipt_cancellation_supersession_denial_gate_status == "blocked"
  and .source_route_wired == true
  and .source_route_count_expected == 105
  and .source_route_tested_by_native_gateway_unit_test == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_route_enabled == true
  and .cancellation_supersession_fixture_count == 10
  and .blocked_cancellation_supersession_fixture_count == 10
  and .noop_cancellation_supersession_fixture_count == 10
  and .accepted_cancellation_supersession_fixture_count == 0
  and .cancellation_performed_count == 0
  and .supersession_performed_count == 0
  and .replacement_receipt_recorded_count == 0
  and .tombstone_recorded_count == 0
  and .delete_marker_recorded_count == 0
  and .activation_command_result_receipt_cancellation_recorded == false
  and .activation_command_result_receipt_supersession_recorded == false
  and .activation_command_result_receipt_replacement_receipt_recorded == false
  and .activation_from_cancellation_allowed == false
  and .activation_from_supersession_allowed == false
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
  and .current_live_enabled_lane_count == 21
  and .enablement_lane_count == 24
  and .ready_enablement_lane_count == 24
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG operator canary controlled request harness operator review acknowledgement activation command result receipt cancellation/supersession denial route gate passed"
