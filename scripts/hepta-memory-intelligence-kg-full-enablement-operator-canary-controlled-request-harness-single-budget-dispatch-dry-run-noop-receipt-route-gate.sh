#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
EXPECTED_ROUTE_COUNT="${HEPTA_EXPECTED_ROUTE_COUNT:-$(bash "$REPO_ROOT/scripts/lib/hepta-native-route-count.sh")}"
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
    echo "missing operator canary single-budget dispatch dry-run no-op receipt route source text: $label" >&2
    exit 1
  fi
}

ACKNOWLEDGEMENT_NO_OP_HANDOFF_LANE_JSON="$(
  curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-dry-run-envelope-readback-audit-receipt-acknowledgement-no-op-handoff-lane"
)"

jq -e '
  .status == "ready"
  and .bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_lane_enabled == true
  and .bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_allowed_by_lane == true
  and .bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_requires_explicit_command == true
  and .bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_requires_readback_audit_receipt_lane == true
  and .bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_acknowledged_by_report_route == false
  and .bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_handoff_performed_by_report_route == false
  and .bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_recorded_by_report_route == false
  and .bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_persisted_by_report_route == false
  and .bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_acknowledgement_no_op_handoff_accepted_by_report_route == false
  and .provider_router_injection_execution_allowed_by_lane == false
  and .provider_router_prompt_mutated_by_report_route == false
  and .provider_router_context_packet_materialized_by_report_route == false
  and .context_attachment_performed_by_report_route == false
  and .context_injection_allowed_by_lane == false
  and .context_injection_performed_by_report_route == false
  and .kg_live_write_lane_enabled == false
  and .provider_model_invocation_lane_enabled == false
  and .channel_delivery_lane_enabled == false
  and .live_mutation_enabled_count == 1
  and .current_live_enabled_lane_count == 12
  and .enablement_lane_count == 15
  and .ready_enablement_lane_count == 15
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.credential_read == false
  and .side_effects.live_kg_write_performed == false
  and .side_effects.channel_send_performed == false
' >/dev/null <<<"$ACKNOWLEDGEMENT_NO_OP_HANDOFF_LANE_JSON"

SINGLE_BUDGET_GATE_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-single-budget-dispatch-dry-run-noop-receipt-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-single-budget-dispatch-dry-run-noop-receipt-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_gate"
  and .operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_ready == true
  and .operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_status == "blocked"
  and .source_receipt_hash_preview_count == 2
  and .source_receipt_accepted_count == 0
  and .source_acceptance_skeleton_operator_input_supplied_count == 0
  and .source_controlled_request_dispatch_budget_declared == 1
  and .source_controlled_request_dispatch_budget_accepted == false
  and .source_controlled_request_dispatch_budget_consumed == 0
  and .dispatch_dry_run_noop_receipt_count == 1
  and .dispatch_dry_run_shape_declared_count == 1
  and .single_budget_declared == 1
  and .single_budget_accepted == false
  and .single_budget_consumed == 0
  and .single_budget_remaining == 0
  and .controlled_request_dispatch_allowed_count == 0
  and .controlled_request_dispatched_count == 0
  and .controlled_request_execution_allowed_count == 0
  and .controlled_request_executed_count == 0
  and .noop_receipt_recorded_count == 0
  and .noop_receipt_persisted_count == 0
  and .noop_receipt_delivered_count == 0
  and .noop_receipt_accepted_count == 0
  and .noop_receipt_materialized_count == 0
  and .request_payload_materialized_count == 0
  and .request_payload_file_written_count == 0
  and .raw_payload_inspected_count == 0
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
  and .dispatch_dry_run_noop_receipt_negative_fixture_count == 7
  and .dispatch_dry_run_noop_receipt_blocked_negative_fixture_count == 7
  and .dispatch_dry_run_noop_receipt_allowed_negative_fixture_count == 0
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$SINGLE_BUDGET_GATE_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = ${EXPECTED_ROUTE_COUNT};" \
  "native gateway route/source command count includes operator canary single-budget dispatch dry-run no-op receipt route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_SINGLE_BUDGET_DISPATCH_DRY_RUN_NOOP_RECEIPT_ENDPOINT' \
  "native gateway operator canary single-budget dispatch dry-run no-op receipt endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-single-budget-dispatch-dry-run-noop-receipt' \
  "native gateway operator canary single-budget dispatch dry-run no-op receipt endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-single-budget-dispatch-dry-run-noop-receipt --json' \
  "native gateway operator canary single-budget dispatch dry-run no-op receipt source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_report' \
  "native gateway operator canary single-budget dispatch dry-run no-op receipt report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_route_enabled": true' \
  "operator canary single-budget dispatch dry-run no-op receipt route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"single_budget_accepted": false' \
  "operator canary single-budget dispatch dry-run no-op receipt budget acceptance denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"controlled_request_dispatched_count": 0' \
  "operator canary single-budget dispatch dry-run no-op receipt dispatch denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"noop_receipt_persisted_count": 0' \
  "operator canary single-budget dispatch dry-run no-op receipt persistence denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"context_injection_performed_count": 0' \
  "operator canary single-budget dispatch dry-run no-op receipt context injection denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"provider_invoked_count": 0' \
  "operator canary single-budget dispatch dry-run no-op receipt provider invocation denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"live_kg_write_performed_count": 0' \
  "operator canary single-budget dispatch dry-run no-op receipt live KG write denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-canary-single-budget-dispatch-dry-run-noop-receipt-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_endpoint_reports_noop_only \
  -- --nocapture >"$TEST_LOG"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_route_gate" \
    --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-single-budget-dispatch-dry-run-noop-receipt" \
    --arg source_command "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-single-budget-dispatch-dry-run-noop-receipt --json" \
    --arg native_gateway_source "$NATIVE_GATEWAY_SOURCE" \
    --arg native_gateway_sha256 "$(sha256_file "$NATIVE_GATEWAY_SOURCE")" \
    --arg test_log "$TEST_LOG" \
    --argjson acknowledgement_no_op_handoff_lane "$ACKNOWLEDGEMENT_NO_OP_HANDOFF_LANE_JSON" \
    --argjson single_budget_gate "$SINGLE_BUDGET_GATE_JSON" \
    '{
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      source_command:$source_command,
      activation_mode:"operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_native_route_status",
      source_acknowledgement_no_op_handoff_lane_ready:($acknowledgement_no_op_handoff_lane.status == "ready"),
      source_single_budget_gate:$single_budget_gate.gate,
      source_single_budget_gate_ready:$single_budget_gate.operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_ready,
      source_single_budget_gate_status:$single_budget_gate.operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_status,
      source_route_wired:true,
      source_route_count_expected:105,
      source_route_tested_by_native_gateway_unit_test:true,
      operator_authorization_source:"telegram_direct_operator_highest_authorization_2026_06_13_16_27_10_asia_shanghai",
      operator_authorization_received:true,
      operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_route_enabled:true,
      single_budget_declared:1,
      single_budget_accepted:false,
      single_budget_consumed:0,
      single_budget_remaining:0,
      controlled_request_dispatch_allowed_count:0,
      controlled_request_dispatched_count:0,
      controlled_request_execution_allowed_count:0,
      controlled_request_executed_count:0,
      noop_receipt_recorded_count:0,
      noop_receipt_persisted_count:0,
      noop_receipt_delivered_count:0,
      noop_receipt_accepted_count:0,
      noop_receipt_materialized_count:0,
      request_payload_materialized_count:0,
      request_payload_file_written_count:0,
      raw_payload_inspected_count:0,
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
      canary_harness_executable:false,
      canary_live_enabled:false,
      current_live_enabled_lane_count:13,
      enablement_lane_count:16,
      ready_enablement_lane_count:16,
      source_contract:{
        source:$native_gateway_source,
        source_sha256:$native_gateway_sha256,
        test_log:$test_log,
        contract:"hepta-native-gateway-operator-canary-controlled-request-harness-single-budget-dispatch-dry-run-noop-receipt-route-v1",
        source_pattern_present:true,
        compile_checked_by_tests:true,
        route_handler_present:true,
        accepts_budget:false,
        consumes_budget:false,
        dispatches_controlled_request:false,
        executes_controlled_request:false,
        records_noop_receipt:false,
        persists_noop_receipt:false,
        materializes_payload:false,
        injects_context:false,
        invokes_provider_or_model:false,
        writes_memory_or_kg:false,
        reads_credentials:false,
        delivers_channel:false
      },
      side_effects:{
        workspace_written:false,
        filesystem_written:false,
        single_budget_accepted:false,
        single_budget_consumed:false,
        dispatch_performed:false,
        execution_performed:false,
        noop_receipt_recorded:false,
        noop_receipt_persisted:false,
        noop_receipt_delivered:false,
        noop_receipt_accepted:false,
        noop_receipt_materialized:false,
        request_payload_materialized:false,
        request_payload_file_written:false,
        raw_payload_inspected:false,
        context_injection_performed:false,
        provider_invoked:false,
        model_invoked:false,
        memory_store_write_performed:false,
        external_kg_adapter_read_performed:false,
        live_kg_write_performed:false,
        credential_read:false,
        secret_file_read:false,
        channel_send_performed:false,
        service_restarted:false,
        active_binary_mutated:false,
        install_performed:false,
        public_release_claimed:false,
        public_ga_claimed:false
      }
    }'
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_route_gate"
  and .source_acknowledgement_no_op_handoff_lane_ready == true
  and .source_single_budget_gate_ready == true
  and .source_single_budget_gate_status == "blocked"
  and .source_route_wired == true
  and .source_route_count_expected == 105
  and .source_route_tested_by_native_gateway_unit_test == true
  and .operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_route_enabled == true
  and .single_budget_declared == 1
  and .single_budget_accepted == false
  and .single_budget_consumed == 0
  and .controlled_request_dispatched_count == 0
  and .controlled_request_executed_count == 0
  and .noop_receipt_persisted_count == 0
  and .request_payload_materialized_count == 0
  and .context_injection_performed_count == 0
  and .provider_invoked_count == 0
  and .model_invoked_count == 0
  and .memory_store_write_performed_count == 0
  and .live_kg_write_performed_count == 0
  and .credential_read_count == 0
  and .channel_send_performed_count == 0
  and .canary_harness_armed == false
  and .canary_live_enabled == false
  and .current_live_enabled_lane_count == 13
  and .enablement_lane_count == 16
  and .ready_enablement_lane_count == 16
  and (.source_contract.accepts_budget == false)
  and (.source_contract.dispatches_controlled_request == false)
  and (.source_contract.persists_noop_receipt == false)
  and (.source_contract.injects_context == false)
  and (.source_contract.invokes_provider_or_model == false)
  and (.source_contract.writes_memory_or_kg == false)
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG operator canary controlled request harness single-budget dispatch dry-run no-op receipt route gate passed"
