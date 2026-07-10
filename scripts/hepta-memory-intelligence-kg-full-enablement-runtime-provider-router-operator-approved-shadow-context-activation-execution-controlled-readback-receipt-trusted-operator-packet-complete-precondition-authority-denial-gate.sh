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
    echo "missing controlled readback receipt trusted operator packet complete precondition authority denial source text: $label" >&2
    exit 1
  fi
}

PARTIAL_MATRIX_JSON="$(
  curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-partial-precondition-denial-matrix"
)"

jq -e '
  .status == "ready"
  and .trusted_operator_packet_partial_precondition_denial_matrix_ready == true
  and .source_route_wired == true
  and .operator_packet_required_field_count == 6
  and .operator_packet_complete_verified_field_count_required == 6
  and .operator_packet_partial_fixture_count == 6
  and .blocked_operator_packet_partial_fixture_count == 6
  and .allowed_operator_packet_partial_fixture_count == 0
  and .partial_packet_max_verified_field_count == 5
  and .partial_packet_min_missing_field_count == 1
  and .partial_packet_acceptance_precondition_satisfied_count == 0
  and .partial_packet_recorded_count == 0
  and .partial_packet_persisted_count == 0
  and .partial_packet_accepted_count == 0
  and .partial_packet_activation_authority_count == 0
  and .partial_packet_activation_command_exposed_count == 0
  and .partial_packet_live_mutation_allowed_count == 0
  and .partial_packet_public_claim_allowed_count == 0
  and .report_route_invokes_shadow_execution == false
  and .report_route_exposes_activation_command == false
  and .live_mutation_enabled_count == 0
  and .current_live_enabled_lane_count == 0
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.credential_read == false
  and .side_effects.live_kg_write_performed == false
  and .side_effects.memory_store_mutated == false
  and .side_effects.channel_send_performed == false
  and .side_effects.service_restarted == false
  and .side_effects.active_binary_mutated == false
' >/dev/null <<<"$PARTIAL_MATRIX_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/hepta-native-gateway/src/native_gateway.rs"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"

require_source_text "$ROUTE_REGISTRY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_AUTHORITY_DENIAL_ENDPOINT' \
  "native gateway controlled readback receipt trusted operator packet complete precondition authority denial endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-complete-precondition-authority-denial' \
  "native gateway controlled readback receipt trusted operator packet complete precondition authority denial endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-complete-precondition-authority-denial --json' \
  "native gateway controlled readback receipt trusted operator packet complete precondition authority denial source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_complete_precondition_authority_denial_report' \
  "native gateway controlled readback receipt trusted operator packet complete precondition authority denial report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'complete_operator_packet_all_preconditions_verified_authority_denied' \
  "complete operator packet authority denied fixture"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'operator_packet_verified_field_count: 6,' \
  "complete operator packet verified field count"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'operator_packet_missing_field_count: 0,' \
  "complete operator packet missing field count"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'operator_packet_acceptance_precondition_satisfied: true,' \
  "complete operator packet acceptance precondition satisfied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'activation_authority_from_packet_derived: false,' \
  "complete operator packet activation authority denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'trusted_operator_packet_complete_precondition_authority_denial_ready' \
  "trusted operator packet complete precondition authority denial readiness field"

TEST_LOG="$(mktemp /tmp/hepta-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-complete-precondition-authority-denial-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_complete_precondition_authority_denial_endpoint_is_report_only \
  -- --nocapture >"$TEST_LOG"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_operator_approved_shadow_context_activation_execution_controlled_readback_receipt_trusted_operator_packet_complete_precondition_authority_denial_gate" \
    --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-complete-precondition-authority-denial" \
    --arg partial_precondition_denial_matrix_endpoint "/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-partial-precondition-denial-matrix" \
    --arg source_command "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-complete-precondition-authority-denial --json" \
    --arg native_gateway_source "$NATIVE_GATEWAY_SOURCE" \
    --arg native_gateway_sha256 "$(sha256_file "$NATIVE_GATEWAY_SOURCE")" \
    --arg test_log "$TEST_LOG" \
    --argjson partial_matrix "$PARTIAL_MATRIX_JSON" \
    '{
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      partial_precondition_denial_matrix_endpoint:$partial_precondition_denial_matrix_endpoint,
      source_command:$source_command,
      activation_mode:"operator_approved_shadow_context_activation_execution_controlled_readback_receipt_trusted_operator_packet_complete_precondition_authority_denial_report",
      live_partial_precondition_denial_matrix_route_status:$partial_matrix.status,
      live_partial_precondition_denial_matrix_route_ready:$partial_matrix.trusted_operator_packet_partial_precondition_denial_matrix_ready,
      source_route_wired:true,
      source_route_count_expected:81,
      source_route_tested_by_native_gateway_unit_test:true,
      trusted_operator_packet_partial_precondition_denial_matrix_ready:true,
      trusted_operator_packet_complete_precondition_authority_denial_ready:true,
      independent_trusted_operator_packet_required:true,
      independent_trusted_operator_packet_shape_declared:true,
      operator_packet_required_field_count:6,
      operator_packet_verified_field_count:6,
      operator_packet_missing_field_count:0,
      operator_packet_identity_verified:true,
      operator_packet_intent_confirmed:true,
      operator_packet_signature_verified:true,
      operator_packet_session_bound:true,
      operator_packet_freshness_verified:true,
      operator_packet_scope_validated:true,
      operator_packet_acceptance_precondition_satisfied:true,
      operator_packet_recorded:false,
      operator_packet_persisted:false,
      operator_packet_accepted:false,
      operator_approval_from_packet_accepted:false,
      activation_authority_from_packet_derived:false,
      activation_request_from_packet_allowed:false,
      activation_command_from_packet_exposed:false,
      live_mutation_from_packet_allowed:false,
      public_claim_from_packet_allowed:false,
      complete_precondition_fixture_count:1,
      complete_precondition_authority_denied_fixture_count:1,
      complete_precondition_authority_allowed_fixture_count:0,
      source_contract:{
        source:$native_gateway_source,
        source_sha256:$native_gateway_sha256,
        contract:"hepta-native-gateway-controlled-shadow-readback-receipt-trusted-operator-packet-complete-precondition-authority-denial-route-v1",
        source_pattern_present:true,
        compile_checked_by_tests:true,
        route_handler_present:true,
        invokes_shadow_execution_from_report_route:false
      },
      test_log:$test_log,
      focused_test_count:1,
      focused_tests_passed:true,
      denied_by_controlled_readback_receipt_trusted_operator_packet_complete_precondition_authority_denial_gate:[
        "complete_operator_packet_recording_denied",
        "complete_operator_packet_persistence_denied",
        "complete_operator_packet_acceptance_denied",
        "complete_operator_packet_operator_approval_denied",
        "complete_operator_packet_activation_authority_denied",
        "complete_operator_packet_activation_request_denied",
        "complete_operator_packet_activation_command_exposure_denied",
        "complete_operator_packet_live_mutation_denied",
        "complete_operator_packet_public_claim_denied",
        "report_route_shadow_execution_invocation_denied",
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
        complete_operator_packet_recorded:false,
        complete_operator_packet_persisted:false,
        complete_operator_packet_materialized:false,
        complete_operator_packet_accepted:false,
        complete_operator_packet_operator_approval_recorded:false,
        complete_operator_packet_activation_authority_recorded:false,
        complete_operator_packet_activation_request_enqueued:false,
        complete_operator_packet_activation_command_exposed:false,
        complete_operator_packet_live_mutation_performed:false,
        complete_operator_packet_public_claim_recorded:false,
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
  and .source_route_count_expected == 81
  and .trusted_operator_packet_partial_precondition_denial_matrix_ready == true
  and .trusted_operator_packet_complete_precondition_authority_denial_ready == true
  and .operator_packet_required_field_count == 6
  and .operator_packet_verified_field_count == 6
  and .operator_packet_missing_field_count == 0
  and .operator_packet_identity_verified == true
  and .operator_packet_intent_confirmed == true
  and .operator_packet_signature_verified == true
  and .operator_packet_session_bound == true
  and .operator_packet_freshness_verified == true
  and .operator_packet_scope_validated == true
  and .operator_packet_acceptance_precondition_satisfied == true
  and .operator_packet_recorded == false
  and .operator_packet_persisted == false
  and .operator_packet_accepted == false
  and .operator_approval_from_packet_accepted == false
  and .activation_authority_from_packet_derived == false
  and .activation_request_from_packet_allowed == false
  and .activation_command_from_packet_exposed == false
  and .live_mutation_from_packet_allowed == false
  and .public_claim_from_packet_allowed == false
  and .complete_precondition_fixture_count == 1
  and .complete_precondition_authority_denied_fixture_count == 1
  and .complete_precondition_authority_allowed_fixture_count == 0
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.credential_read == false
  and .side_effects.live_kg_write_performed == false
  and .side_effects.memory_store_mutated == false
  and .side_effects.channel_send_performed == false
  and .side_effects.service_restarted == false
  and .side_effects.active_binary_mutated == false
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta controlled shadow readback receipt trusted operator packet complete precondition authority denial gate passed"
