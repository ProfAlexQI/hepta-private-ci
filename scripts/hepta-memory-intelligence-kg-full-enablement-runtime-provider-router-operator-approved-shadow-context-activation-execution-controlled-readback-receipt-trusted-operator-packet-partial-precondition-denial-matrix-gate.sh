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
    echo "missing controlled readback receipt trusted operator packet partial precondition denial matrix source text: $label" >&2
    exit 1
  fi
}

INTAKE_PRECONDITION_JSON="$(
  curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-intake-precondition"
)"

jq -e '
  .status == "ready"
  and .trusted_operator_packet_intake_precondition_ready == true
  and .source_route_wired == true
  and .independent_trusted_operator_packet_required == true
  and .independent_trusted_operator_packet_shape_declared == true
  and .operator_packet_required_field_count == 6
  and .operator_packet_verified_field_count == 0
  and .operator_packet_missing_field_count == 6
  and .operator_packet_acceptance_precondition_satisfied == false
  and .operator_packet_recorded == false
  and .operator_packet_persisted == false
  and .operator_packet_accepted == false
  and .operator_approval_from_packet_accepted == false
  and .activation_authority_from_packet_derived == false
  and .activation_request_from_packet_allowed == false
  and .activation_command_from_packet_exposed == false
  and .live_mutation_from_packet_allowed == false
  and .public_claim_from_packet_allowed == false
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
' >/dev/null <<<"$INTAKE_PRECONDITION_JSON"

source "$REPO_ROOT/scripts/lib/hepta-source-set.sh"
NATIVE_GATEWAY_SOURCE="hepta-native-gateway-source-set-v1"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"

require_source_text "$ROUTE_REGISTRY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_PARTIAL_PRECONDITION_DENIAL_MATRIX_ENDPOINT' \
  "native gateway controlled readback receipt trusted operator packet partial precondition denial matrix endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-partial-precondition-denial-matrix' \
  "native gateway controlled readback receipt trusted operator packet partial precondition denial matrix endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-partial-precondition-denial-matrix --json' \
  "native gateway controlled readback receipt trusted operator packet partial precondition denial matrix source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_partial_precondition_denial_matrix_report' \
  "native gateway controlled readback receipt trusted operator packet partial precondition denial matrix report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'partial_operator_packet_missing_identity' \
  "partial operator packet missing identity fixture"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'partial_operator_packet_missing_signature' \
  "partial operator packet missing signature fixture"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'partial_packet_max_verified_field_count' \
  "partial packet max verified field count"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'partial_packet_min_missing_field_count' \
  "partial packet min missing field count"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'trusted_operator_packet_partial_precondition_denial_matrix_ready' \
  "trusted operator packet partial precondition denial matrix readiness field"

TEST_LOG="$(mktemp /tmp/hepta-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-partial-precondition-denial-matrix-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_partial_precondition_denial_matrix_endpoint_is_report_only \
  -- --nocapture >"$TEST_LOG"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_operator_approved_shadow_context_activation_execution_controlled_readback_receipt_trusted_operator_packet_partial_precondition_denial_matrix_gate" \
    --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-partial-precondition-denial-matrix" \
    --arg intake_precondition_endpoint "/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-intake-precondition" \
    --arg source_command "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-partial-precondition-denial-matrix --json" \
    --arg native_gateway_source "$NATIVE_GATEWAY_SOURCE" \
    --arg native_gateway_sha256 "$(sha256_file "$NATIVE_GATEWAY_SOURCE")" \
    --arg test_log "$TEST_LOG" \
    --argjson intake_precondition "$INTAKE_PRECONDITION_JSON" \
    '{
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      intake_precondition_endpoint:$intake_precondition_endpoint,
      source_command:$source_command,
      activation_mode:"operator_approved_shadow_context_activation_execution_controlled_readback_receipt_trusted_operator_packet_partial_precondition_denial_matrix_report",
      live_intake_precondition_route_status:$intake_precondition.status,
      live_intake_precondition_route_ready:$intake_precondition.trusted_operator_packet_intake_precondition_ready,
      source_route_wired:true,
      source_route_count_expected:81,
      source_route_tested_by_native_gateway_unit_test:true,
      trusted_operator_packet_intake_precondition_ready:true,
      trusted_operator_packet_partial_precondition_denial_matrix_ready:true,
      independent_trusted_operator_packet_required:true,
      independent_trusted_operator_packet_shape_declared:true,
      operator_packet_required_field_count:6,
      operator_packet_complete_verified_field_count_required:6,
      operator_packet_partial_fixture_count:6,
      blocked_operator_packet_partial_fixture_count:6,
      allowed_operator_packet_partial_fixture_count:0,
      partial_packet_max_verified_field_count:5,
      partial_packet_min_missing_field_count:1,
      partial_packet_acceptance_precondition_satisfied_count:0,
      partial_packet_recorded_count:0,
      partial_packet_persisted_count:0,
      partial_packet_accepted_count:0,
      partial_packet_activation_authority_count:0,
      partial_packet_activation_command_exposed_count:0,
      partial_packet_live_mutation_allowed_count:0,
      partial_packet_public_claim_allowed_count:0,
      missing_identity_fixture_blocked:true,
      missing_intent_fixture_blocked:true,
      missing_signature_fixture_blocked:true,
      missing_session_fixture_blocked:true,
      missing_freshness_fixture_blocked:true,
      missing_scope_fixture_blocked:true,
      source_contract:{
        source:$native_gateway_source,
        source_sha256:$native_gateway_sha256,
        contract:"hepta-native-gateway-controlled-shadow-readback-receipt-trusted-operator-packet-partial-precondition-denial-matrix-route-v1",
        source_pattern_present:true,
        compile_checked_by_tests:true,
        route_handler_present:true,
        invokes_shadow_execution_from_report_route:false
      },
      test_log:$test_log,
      focused_test_count:1,
      focused_tests_passed:true,
      denied_by_controlled_readback_receipt_trusted_operator_packet_partial_precondition_denial_matrix_gate:[
        "partial_operator_packet_missing_identity",
        "partial_operator_packet_missing_intent",
        "partial_operator_packet_missing_signature",
        "partial_operator_packet_missing_session",
        "partial_operator_packet_missing_freshness",
        "partial_operator_packet_missing_scope",
        "partial_operator_packet_recording_denied",
        "partial_operator_packet_persistence_denied",
        "partial_operator_packet_acceptance_denied",
        "partial_operator_packet_activation_authority_denied",
        "partial_operator_packet_activation_request_denied",
        "partial_operator_packet_activation_command_exposure_denied",
        "partial_operator_packet_live_mutation_denied",
        "partial_operator_packet_public_claim_denied",
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
        partial_operator_packet_recorded:false,
        partial_operator_packet_persisted:false,
        partial_operator_packet_materialized:false,
        partial_operator_packet_accepted:false,
        partial_operator_packet_identity_verified:false,
        partial_operator_packet_intent_confirmed:false,
        partial_operator_packet_signature_verified:false,
        partial_operator_packet_session_bound:false,
        partial_operator_packet_freshness_verified:false,
        partial_operator_packet_scope_validated:false,
        partial_operator_packet_activation_authority_recorded:false,
        partial_operator_packet_activation_request_enqueued:false,
        partial_operator_packet_activation_command_exposed:false,
        partial_operator_packet_live_mutation_performed:false,
        partial_operator_packet_public_claim_recorded:false,
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
  and .trusted_operator_packet_intake_precondition_ready == true
  and .trusted_operator_packet_partial_precondition_denial_matrix_ready == true
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
  and .missing_identity_fixture_blocked == true
  and .missing_intent_fixture_blocked == true
  and .missing_signature_fixture_blocked == true
  and .missing_session_fixture_blocked == true
  and .missing_freshness_fixture_blocked == true
  and .missing_scope_fixture_blocked == true
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
echo "Hepta controlled shadow readback receipt trusted operator packet partial precondition denial matrix gate passed"
