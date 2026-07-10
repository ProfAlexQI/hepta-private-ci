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
    echo "missing complete precondition operator approval lane separation source text: $label" >&2
    exit 1
  fi
}

COMPLETE_PRECONDITION_JSON="$(
  curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-complete-precondition-authority-denial"
)"

jq -e '
  .status == "ready"
  and .trusted_operator_packet_complete_precondition_authority_denial_ready == true
  and .source_route_wired == true
  and .operator_packet_required_field_count == 6
  and .operator_packet_verified_field_count == 6
  and .operator_packet_missing_field_count == 0
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
' >/dev/null <<<"$COMPLETE_PRECONDITION_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/hepta-native-gateway/src/native_gateway.rs"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"

require_source_text "$ROUTE_REGISTRY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_OPERATOR_APPROVAL_LANE_SEPARATION_ENDPOINT' \
  "native gateway complete precondition operator approval lane separation endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-complete-precondition-operator-approval-lane-separation' \
  "native gateway complete precondition operator approval lane separation endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-complete-precondition-operator-approval-lane-separation --json' \
  "native gateway complete precondition operator approval lane separation source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_complete_precondition_operator_approval_lane_separation_report' \
  "native gateway complete precondition operator approval lane separation report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'complete_precondition_can_substitute_operator_approval: false,' \
  "complete precondition cannot substitute operator approval"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'operator_approved_activation_lane_required: true,' \
  "separate operator-approved activation lane required"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'operator_approved_activation_lane_present: false,' \
  "operator-approved activation lane absent from report route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'activation_lane_acceptance_allowed: false,' \
  "activation lane acceptance denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'trusted_operator_packet_complete_precondition_operator_approval_lane_separation_ready' \
  "complete precondition operator approval lane separation readiness field"

TEST_LOG="$(mktemp /tmp/hepta-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-complete-precondition-operator-approval-lane-separation-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_complete_precondition_operator_approval_lane_separation_endpoint_is_report_only \
  -- --nocapture >"$TEST_LOG"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_operator_approved_shadow_context_activation_execution_controlled_readback_receipt_trusted_operator_packet_complete_precondition_operator_approval_lane_separation_gate" \
    --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-complete-precondition-operator-approval-lane-separation" \
    --arg complete_precondition_authority_denial_endpoint "/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-complete-precondition-authority-denial" \
    --arg source_command "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-complete-precondition-operator-approval-lane-separation --json" \
    --arg native_gateway_source "$NATIVE_GATEWAY_SOURCE" \
    --arg native_gateway_sha256 "$(sha256_file "$NATIVE_GATEWAY_SOURCE")" \
    --arg test_log "$TEST_LOG" \
    --argjson complete_precondition "$COMPLETE_PRECONDITION_JSON" \
    '{
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      complete_precondition_authority_denial_endpoint:$complete_precondition_authority_denial_endpoint,
      source_command:$source_command,
      activation_mode:"operator_approved_shadow_context_activation_execution_controlled_readback_receipt_trusted_operator_packet_complete_precondition_operator_approval_lane_separation_report",
      live_complete_precondition_authority_denial_route_status:$complete_precondition.status,
      live_complete_precondition_authority_denial_route_ready:$complete_precondition.trusted_operator_packet_complete_precondition_authority_denial_ready,
      source_route_wired:true,
      source_route_count_expected:81,
      source_route_tested_by_native_gateway_unit_test:true,
      trusted_operator_packet_complete_precondition_authority_denial_ready:true,
      trusted_operator_packet_complete_precondition_operator_approval_lane_separation_ready:true,
      operator_packet_required_field_count:6,
      operator_packet_verified_field_count:6,
      operator_packet_missing_field_count:0,
      operator_packet_acceptance_precondition_satisfied:true,
      operator_packet_accepted:false,
      operator_approval_from_packet_accepted:false,
      complete_precondition_can_substitute_operator_approval:false,
      complete_precondition_can_create_activation_lane:false,
      operator_approved_activation_lane_required:true,
      operator_approved_activation_lane_present:false,
      activation_lane_acceptance_allowed:false,
      activation_lane_recorded:false,
      activation_lane_persisted:false,
      activation_lane_enqueued:false,
      activation_lane_effective:false,
      activation_authority_from_packet_derived:false,
      activation_request_from_packet_allowed:false,
      activation_command_from_packet_exposed:false,
      live_mutation_from_packet_allowed:false,
      public_claim_from_packet_allowed:false,
      source_contract:{
        source:$native_gateway_source,
        source_sha256:$native_gateway_sha256,
        contract:"hepta-native-gateway-controlled-shadow-readback-receipt-trusted-operator-packet-complete-precondition-operator-approval-lane-separation-route-v1",
        source_pattern_present:true,
        compile_checked_by_tests:true,
        route_handler_present:true,
        invokes_shadow_execution_from_report_route:false
      },
      test_log:$test_log,
      focused_test_count:1,
      focused_tests_passed:true,
      denied_by_operator_approval_lane_separation_gate:[
        "complete_packet_precondition_substitution_for_operator_approval_denied",
        "operator_approval_lane_creation_from_complete_packet_denied",
        "operator_approval_lane_recording_denied",
        "operator_approval_lane_persistence_denied",
        "activation_lane_enqueue_denied",
        "activation_authority_derivation_denied",
        "activation_command_exposure_denied",
        "live_mutation_denied",
        "public_claim_denied",
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
        complete_operator_packet_accepted:false,
        complete_operator_packet_operator_approval_recorded:false,
        operator_approval_lane_recorded:false,
        operator_approval_lane_persisted:false,
        operator_approval_lane_materialized:false,
        operator_approval_lane_enqueued:false,
        operator_approval_lane_effective:false,
        activation_authority_recorded:false,
        activation_request_enqueued:false,
        activation_command_exposed:false,
        live_mutation_performed:false,
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
  and .trusted_operator_packet_complete_precondition_authority_denial_ready == true
  and .trusted_operator_packet_complete_precondition_operator_approval_lane_separation_ready == true
  and .operator_packet_required_field_count == 6
  and .operator_packet_verified_field_count == 6
  and .operator_packet_missing_field_count == 0
  and .operator_packet_acceptance_precondition_satisfied == true
  and .operator_packet_accepted == false
  and .operator_approval_from_packet_accepted == false
  and .complete_precondition_can_substitute_operator_approval == false
  and .complete_precondition_can_create_activation_lane == false
  and .operator_approved_activation_lane_required == true
  and .operator_approved_activation_lane_present == false
  and .activation_lane_acceptance_allowed == false
  and .activation_lane_recorded == false
  and .activation_lane_persisted == false
  and .activation_lane_enqueued == false
  and .activation_lane_effective == false
  and .activation_authority_from_packet_derived == false
  and .activation_command_from_packet_exposed == false
  and .live_mutation_from_packet_allowed == false
  and .public_claim_from_packet_allowed == false
  and .focused_test_count == 1
  and .focused_tests_passed == true
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
echo "Hepta controlled shadow readback receipt trusted operator packet complete precondition operator approval lane separation gate passed"
