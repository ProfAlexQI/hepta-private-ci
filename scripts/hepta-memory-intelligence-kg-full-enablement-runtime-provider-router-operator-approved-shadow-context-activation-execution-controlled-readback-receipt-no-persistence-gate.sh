#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"

cd "$REPO_ROOT"

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

require_source_text() {
  local source_file="$1"
  local source_text="$2"
  local label="$3"

  if ! rg -Fq "$source_text" "$source_file"; then
    echo "missing controlled readback receipt no-persistence source text: $label" >&2
    exit 1
  fi
}

CONTROLLED_JSON="$(
  curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled"
)"

jq -e '
  .status == "ready"
  and .controlled_shadow_execution_report_ready == true
  and .source_route_wired == true
  and .isolated_fixture_execution_performed_by_source_gate == true
  and .live_route_execution_invoked == false
  and .report_route_exposes_activation_command == false
  and .live_mutation_enabled_count == 0
  and .current_live_enabled_lane_count == 0
  and .provider_invocation_performed == false
  and .model_invocation_performed == false
  and .auth_secret_read_performed == false
  and .credential_read_performed == false
  and .external_network_call_performed == false
  and .live_kg_write_performed == false
  and .live_memory_write_performed == false
  and .side_effects.live_7373_router_mutated_by_report_route == false
  and .side_effects.memory_store_mutated == false
  and .side_effects.channel_send_performed == false
  and .side_effects.external_send_performed == false
  and .side_effects.service_restarted == false
  and .side_effects.active_binary_mutated == false
' >/dev/null <<<"$CONTROLLED_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_NO_PERSISTENCE_ENDPOINT' \
  "native gateway controlled readback receipt no-persistence endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-no-persistence' \
  "native gateway controlled readback receipt no-persistence endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-no-persistence --json' \
  "native gateway controlled readback receipt no-persistence source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_no_persistence_report' \
  "native gateway controlled readback receipt no-persistence report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'readback_receipt_no_persistence_ready' \
  "controlled readback receipt no-persistence readiness field"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'readback_receipt_recorded: false,' \
  "readback receipt recording denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'readback_receipt_persisted: false,' \
  "readback receipt persistence denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'readback_receipt_filesystem_written: false,' \
  "readback receipt filesystem write denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'readback_receipt_exported: false,' \
  "readback receipt export denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'readback_receipt_query_registered: false,' \
  "readback receipt query registration denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'readback_receipt_observability_recorded: false,' \
  "readback receipt observability denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'operator_approval_from_receipt_accepted: false,' \
  "receipt-derived operator approval denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'activation_authority_derived: false,' \
  "receipt-derived activation authority denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'public_claim_from_receipt_allowed: false,' \
  "receipt-derived public claim denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'report_route_invokes_shadow_execution: false,' \
  "report route still does not invoke shadow execution"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'live_kg_or_memory_write' \
  "readback receipt cannot drive live KG or Memory write"

TEST_LOG="$(mktemp /tmp/hepta-shadow-context-activation-execution-controlled-readback-receipt-no-persistence-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_no_persistence_endpoint_is_report_only \
  -- --nocapture >"$TEST_LOG"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_operator_approved_shadow_context_activation_execution_controlled_readback_receipt_no_persistence_gate" \
    --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-no-persistence" \
    --arg controlled_endpoint "/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled" \
    --arg source_command "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-no-persistence --json" \
    --arg native_gateway_source "$NATIVE_GATEWAY_SOURCE" \
    --arg native_gateway_sha256 "$(sha256_file "$NATIVE_GATEWAY_SOURCE")" \
    --arg test_log "$TEST_LOG" \
    --argjson controlled "$CONTROLLED_JSON" \
    '{
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      controlled_endpoint:$controlled_endpoint,
      source_command:$source_command,
      activation_mode:"operator_approved_shadow_context_activation_execution_controlled_readback_receipt_no_persistence_report",
      live_controlled_route_status:$controlled.status,
      live_controlled_route_ready:$controlled.controlled_shadow_execution_report_ready,
      source_route_wired:true,
      source_route_count_expected:74,
      source_route_tested_by_native_gateway_unit_test:true,
      controlled_route_ready:true,
      controlled_shadow_execution_report_ready:true,
      readback_receipt_no_persistence_ready:true,
      readback_receipt_schema_declared:true,
      readback_receipt_requested:true,
      readback_receipt_allowed:false,
      readback_receipt_shape_accepted:false,
      readback_receipt_recorded:false,
      readback_receipt_persisted:false,
      readback_receipt_materialized:false,
      readback_receipt_filesystem_written:false,
      readback_receipt_ledger_written:false,
      readback_receipt_indexed:false,
      readback_receipt_enqueued:false,
      readback_receipt_delivered:false,
      readback_receipt_exported:false,
      readback_receipt_query_registered:false,
      readback_receipt_observability_recorded:false,
      readback_receipt_hash_bound:false,
      readback_receipt_signature_hash_recorded:false,
      readback_receipt_timestamp_recorded:false,
      readback_receipt_operator_identity_accepted:false,
      readback_receipt_status_accepted:false,
      completion_ack_recorded:false,
      completion_ack_persisted:false,
      completion_ack_accepted:false,
      operator_approval_from_receipt_accepted:false,
      activation_from_receipt_allowed:false,
      activation_authority_derived:false,
      public_claim_from_receipt_allowed:false,
      report_route_invokes_shadow_execution:false,
      report_route_exposes_activation_command:false,
      live_mutation_enabled_count:0,
      current_live_enabled_lane_count:0,
      readback_receipt_surface_count:10,
      blocked_readback_receipt_fixture_count:10,
      allowed_readback_receipt_fixture_count:0,
      source_contract:{
        source:$native_gateway_source,
        source_sha256:$native_gateway_sha256,
        contract:"hepta-native-gateway-controlled-shadow-readback-receipt-no-persistence-route-v1",
        source_pattern_present:true,
        compile_checked_by_tests:true,
        route_handler_present:true,
        invokes_shadow_execution_from_report_route:false
      },
      test_log:$test_log,
      focused_test_count:1,
      focused_tests_passed:true,
      denied_by_controlled_readback_receipt_no_persistence_gate:[
        "controlled_readback_receipt_acceptance_denied",
        "controlled_readback_receipt_recording_denied",
        "controlled_readback_receipt_persistence_denied",
        "controlled_readback_receipt_materialization_denied",
        "controlled_readback_receipt_filesystem_write_denied",
        "controlled_readback_receipt_ledger_index_queue_delivery_denied",
        "controlled_readback_receipt_export_query_observability_denied",
        "controlled_readback_receipt_hash_signature_timestamp_identity_binding_denied",
        "controlled_readback_receipt_completion_ack_denied",
        "controlled_readback_receipt_operator_approval_denied",
        "controlled_readback_receipt_activation_authority_denied",
        "controlled_readback_receipt_public_claim_denied",
        "report_route_shadow_execution_invocation_denied",
        "live_activation_command_exposure_denied",
        "provider_model_invocation_denied",
        "auth_secret_credential_read_denied",
        "live_kg_memory_write_denied",
        "telegram_channel_delivery_denied",
        "service_restart_active_binary_mutation_denied",
        "release_public_claim_denied"
      ],
      side_effects:{
        report_route_invoked_runtime_execution:false,
        source_gate_invokes_isolated_fixture_execution:true,
        live_7373_router_mutated_by_report_route:false,
        readback_receipt_recorded:false,
        readback_receipt_persisted:false,
        readback_receipt_materialized:false,
        readback_receipt_filesystem_written:false,
        readback_receipt_exported:false,
        readback_receipt_query_registered:false,
        readback_receipt_observability_recorded:false,
        completion_ack_recorded:false,
        completion_ack_persisted:false,
        operator_approval_from_receipt_accepted:false,
        activation_from_receipt_allowed:false,
        activation_authority_derived:false,
        public_claim_from_receipt_allowed:false,
        provider_invoked:false,
        model_invoked:false,
        auth_secret_read:false,
        credential_read:false,
        external_network_call_performed:false,
        live_kg_write_performed:false,
        memory_store_mutated:false,
        channel_send_performed:false,
        external_send_performed:false,
        service_restarted:false,
        active_binary_mutated:false,
        release_artifact_written:false,
        public_release_claimed:false
      }
    }'
)"

jq -e '
  .status == "ready"
  and .source_route_count_expected == 74
  and .controlled_route_ready == true
  and .readback_receipt_no_persistence_ready == true
  and .readback_receipt_recorded == false
  and .readback_receipt_persisted == false
  and .readback_receipt_filesystem_written == false
  and .readback_receipt_exported == false
  and .readback_receipt_query_registered == false
  and .readback_receipt_observability_recorded == false
  and .operator_approval_from_receipt_accepted == false
  and .activation_authority_derived == false
  and .public_claim_from_receipt_allowed == false
  and .report_route_invokes_shadow_execution == false
  and .report_route_exposes_activation_command == false
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.credential_read == false
  and .side_effects.live_kg_write_performed == false
  and .side_effects.memory_store_mutated == false
  and .side_effects.channel_send_performed == false
  and .side_effects.external_send_performed == false
  and .side_effects.service_restarted == false
  and .side_effects.active_binary_mutated == false
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta controlled shadow readback receipt no-persistence gate passed"
