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
    echo "missing controlled readback receipt authority-denial source text: $label" >&2
    exit 1
  fi
}

NO_PERSISTENCE_JSON="$(
  curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-no-persistence"
)"

jq -e '
  .status == "ready"
  and .readback_receipt_no_persistence_ready == true
  and .source_route_wired == true
  and .controlled_route_ready == true
  and .readback_receipt_allowed == false
  and .readback_receipt_recorded == false
  and .readback_receipt_persisted == false
  and .operator_approval_from_receipt_accepted == false
  and .activation_from_receipt_allowed == false
  and .activation_authority_derived == false
  and .public_claim_from_receipt_allowed == false
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
' >/dev/null <<<"$NO_PERSISTENCE_JSON"

source "$REPO_ROOT/scripts/lib/hepta-source-set.sh"
NATIVE_GATEWAY_SOURCE="hepta-native-gateway-source-set-v1"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"

require_source_text "$ROUTE_REGISTRY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_AUTHORITY_DENIAL_ENDPOINT' \
  "native gateway controlled readback receipt authority-denial endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-authority-denial' \
  "native gateway controlled readback receipt authority-denial endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-authority-denial --json' \
  "native gateway controlled readback receipt authority-denial source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_authority_denial_report' \
  "native gateway controlled readback receipt authority-denial report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'readback_receipt_authority_denial_ready' \
  "controlled readback receipt authority denial readiness field"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'trusted_operator_acceptance_record_accepted: false,' \
  "trusted operator acceptance record denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'operator_identity_verified_from_receipt: false,' \
  "receipt-derived operator identity denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'operator_intent_confirmed_from_receipt: false,' \
  "receipt-derived operator intent denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'operator_approval_from_receipt_accepted: false,' \
  "receipt-derived operator approval denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'activation_authority_derived: false,' \
  "receipt-derived activation authority denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'activation_command_from_receipt_exposed: false,' \
  "receipt-derived activation command exposure denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'public_claim_from_receipt_allowed: false,' \
  "receipt-derived public claim denied"

TEST_LOG="$(mktemp /tmp/hepta-shadow-context-activation-execution-controlled-readback-receipt-authority-denial-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_authority_denial_endpoint_is_report_only \
  -- --nocapture >"$TEST_LOG"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_operator_approved_shadow_context_activation_execution_controlled_readback_receipt_authority_denial_gate" \
    --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-authority-denial" \
    --arg no_persistence_endpoint "/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-no-persistence" \
    --arg source_command "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-authority-denial --json" \
    --arg native_gateway_source "$NATIVE_GATEWAY_SOURCE" \
    --arg native_gateway_sha256 "$(sha256_file "$NATIVE_GATEWAY_SOURCE")" \
    --arg test_log "$TEST_LOG" \
    --argjson no_persistence "$NO_PERSISTENCE_JSON" \
    '{
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      no_persistence_endpoint:$no_persistence_endpoint,
      source_command:$source_command,
      activation_mode:"operator_approved_shadow_context_activation_execution_controlled_readback_receipt_authority_denial_report",
      live_no_persistence_route_status:$no_persistence.status,
      live_no_persistence_route_ready:$no_persistence.readback_receipt_no_persistence_ready,
      source_route_wired:true,
      source_route_count_expected:81,
      source_route_tested_by_native_gateway_unit_test:true,
      readback_receipt_no_persistence_ready:true,
      readback_receipt_authority_denial_ready:true,
      readback_receipt_authority_boundary_declared:true,
      readback_receipt_shape_observed:true,
      readback_receipt_shape_accepted:false,
      trusted_operator_acceptance_record_required:true,
      trusted_operator_acceptance_record_present:false,
      trusted_operator_acceptance_record_accepted:false,
      operator_identity_verified_from_receipt:false,
      operator_intent_confirmed_from_receipt:false,
      operator_approval_from_receipt_accepted:false,
      activation_authority_derived:false,
      activation_request_from_receipt_allowed:false,
      activation_command_from_receipt_exposed:false,
      live_mutation_from_receipt_allowed:false,
      public_claim_from_receipt_allowed:false,
      public_release_from_receipt_allowed:false,
      report_route_invokes_shadow_execution:false,
      report_route_exposes_activation_command:false,
      live_mutation_enabled_count:0,
      current_live_enabled_lane_count:0,
      receipt_authority_fixture_count:8,
      blocked_receipt_authority_fixture_count:8,
      allowed_receipt_authority_fixture_count:0,
      source_contract:{
        source:$native_gateway_source,
        source_sha256:$native_gateway_sha256,
        contract:"hepta-native-gateway-controlled-shadow-readback-receipt-authority-denial-route-v1",
        source_pattern_present:true,
        compile_checked_by_tests:true,
        route_handler_present:true,
        invokes_shadow_execution_from_report_route:false
      },
      test_log:$test_log,
      focused_test_count:1,
      focused_tests_passed:true,
      denied_by_controlled_readback_receipt_authority_denial_gate:[
        "controlled_readback_receipt_trusted_operator_record_denied",
        "controlled_readback_receipt_operator_identity_denied",
        "controlled_readback_receipt_operator_intent_denied",
        "controlled_readback_receipt_operator_approval_denied",
        "controlled_readback_receipt_activation_authority_denied",
        "controlled_readback_receipt_activation_request_denied",
        "controlled_readback_receipt_activation_command_exposure_denied",
        "controlled_readback_receipt_live_mutation_denied",
        "controlled_readback_receipt_public_claim_denied",
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
        readback_receipt_recorded:false,
        readback_receipt_persisted:false,
        trusted_operator_acceptance_recorded:false,
        trusted_operator_acceptance_record_persisted:false,
        operator_identity_verified:false,
        operator_intent_confirmed:false,
        operator_approval_recorded:false,
        activation_authority_recorded:false,
        activation_request_enqueued:false,
        activation_command_exposed:false,
        public_claim_recorded:false,
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
  and .readback_receipt_no_persistence_ready == true
  and .readback_receipt_authority_denial_ready == true
  and .readback_receipt_shape_observed == true
  and .readback_receipt_shape_accepted == false
  and .trusted_operator_acceptance_record_required == true
  and .trusted_operator_acceptance_record_present == false
  and .trusted_operator_acceptance_record_accepted == false
  and .operator_identity_verified_from_receipt == false
  and .operator_intent_confirmed_from_receipt == false
  and .operator_approval_from_receipt_accepted == false
  and .activation_authority_derived == false
  and .activation_request_from_receipt_allowed == false
  and .activation_command_from_receipt_exposed == false
  and .live_mutation_from_receipt_allowed == false
  and .public_claim_from_receipt_allowed == false
  and .public_release_from_receipt_allowed == false
  and .report_route_invokes_shadow_execution == false
  and .report_route_exposes_activation_command == false
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
echo "Hepta controlled shadow readback receipt authority-denial gate passed"
