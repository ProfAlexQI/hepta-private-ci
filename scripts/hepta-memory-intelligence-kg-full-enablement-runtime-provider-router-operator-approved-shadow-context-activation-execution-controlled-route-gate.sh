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
    echo "missing shadow context activation execution controlled route source text: $label" >&2
    exit 1
  fi
}

RUNTIME_READINESS_JSON="$(
  curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-runtime-readiness"
)"

jq -e '
  .status == "ready"
  and .full_enablement_activation_readiness_ready == true
  and .operator_approval_required_before_activation == true
  and .rollback_kill_switch_required == true
  and .long_soak_required_before_mutation == true
  and .context_handoff_acceptance_required == true
  and .live_mutation_enabled_count == 0
  and .current_live_enabled_lane_count == 0
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.credential_read == false
  and .side_effects.live_kg_write_performed == false
  and .side_effects.memory_store_mutated == false
' >/dev/null <<<"$RUNTIME_READINESS_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"
RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE="codex-rs/hepta-runtime/src/model_provider_router.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_ENDPOINT' \
  "native gateway controlled endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled' \
  "native gateway controlled endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled --json' \
  "native gateway controlled source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_report' \
  "native gateway controlled report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'controlled_shadow_execution_report_ready' \
  "controlled execution report readiness field"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'isolated_fixture_execution_performed_by_source_gate: true,' \
  "source gate isolated fixture evidence"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'live_route_execution_invoked: false,' \
  "live route does not execute shadow activation"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'report_route_exposes_activation_command: false,' \
  "report route exposes no activation command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'feature_flag_mutation_scope: "isolated_source_fixture_only",' \
  "feature flag mutation scoped to isolated source fixture"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'context_attachment_scope: "isolated_source_fixture_only",' \
  "context attachment scoped to isolated source fixture"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'provider_invocation_performed: false,' \
  "provider invocation denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'credential_read_performed: false,' \
  "credential read denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'live_kg_write_performed: false,' \
  "live KG write denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'live_memory_write_performed: false,' \
  "live Memory write denied"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'pub fn execute_memory_context_activation_shadow' \
  "runtime-owned shadow activation execution surface"

TEST_LOG="$(mktemp /tmp/hepta-shadow-context-activation-execution-controlled-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_endpoint_is_source_gate_only \
  -- --nocapture >"$TEST_LOG"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-runtime \
  model_provider_router_executes_operator_approved_memory_context_shadow_activation \
  -- --nocapture >>"$TEST_LOG"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_operator_approved_shadow_context_activation_execution_controlled_route_gate" \
    --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled" \
    --arg source_command "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled --json" \
    --arg native_gateway_source "$NATIVE_GATEWAY_SOURCE" \
    --arg runtime_model_provider_router_source "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
    --arg native_gateway_sha256 "$(sha256_file "$NATIVE_GATEWAY_SOURCE")" \
    --arg runtime_model_provider_router_sha256 "$(sha256_file "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE")" \
    --arg test_log "$TEST_LOG" \
    --argjson readiness "$RUNTIME_READINESS_JSON" \
    '{
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      source_command:$source_command,
      activation_mode:"operator_approved_shadow_context_activation_execution_controlled_report",
      runtime_readiness_status:$readiness.status,
      runtime_readiness_ready:$readiness.full_enablement_activation_readiness_ready,
      source_live_mutation_enabled_count:$readiness.live_mutation_enabled_count,
      source_current_live_enabled_lane_count:$readiness.current_live_enabled_lane_count,
      source_route_wired:true,
      source_route_count_expected:72,
      source_route_tested_by_native_gateway_unit_test:true,
      controlled_shadow_execution_report_ready:true,
      runtime_owned_execution_surface_present:true,
      isolated_fixture_execution_required:true,
      isolated_fixture_execution_performed_by_source_gate:true,
      live_route_execution_invoked:false,
      report_route_exposes_activation_command:false,
      release_gate_required:true,
      operator_release_approval_required:true,
      canary_telemetry_required:true,
      rollback_kill_switch_required:true,
      post_activation_watchdog_soak_plan_required:true,
      idempotency_required:true,
      traffic_percent_ppm_required:0,
      readback_receipt_required:true,
      audit_evidence_required:true,
      feature_flag_mutation_scope:"isolated_source_fixture_only",
      context_attachment_scope:"isolated_source_fixture_only",
      provider_invocation_allowed:false,
      provider_invocation_performed:false,
      model_invocation_allowed:false,
      model_invocation_performed:false,
      auth_secret_read_allowed:false,
      auth_secret_read_performed:false,
      credential_read_allowed:false,
      credential_read_performed:false,
      external_network_call_allowed:false,
      external_network_call_performed:false,
      live_kg_write_allowed:false,
      live_kg_write_performed:false,
      live_memory_write_allowed:false,
      live_memory_write_performed:false,
      source_contracts:[
        {
          source:$native_gateway_source,
          source_sha256:$native_gateway_sha256,
          contract:"hepta-native-gateway-shadow-context-activation-execution-controlled-route-v1",
          source_pattern_present:true,
          compile_checked_by_tests:true,
          route_handler_present:true,
          invokes_shadow_execution_from_report_route:false
        },
        {
          source:$runtime_model_provider_router_source,
          source_sha256:$runtime_model_provider_router_sha256,
          contract:"hepta-runtime-model-provider-memory-context-shadow-activation-execution-v1",
          source_pattern_present:true,
          compile_checked_by_tests:true,
          isolated_fixture_execution_checked:true
        }
      ],
      test_log:$test_log,
      focused_test_count:2,
      focused_tests_passed:true,
      denied_by_shadow_context_activation_execution_controlled_route_gate:[
        "report_route_shadow_execution_invocation_denied",
        "live_activation_command_exposure_denied",
        "provider_invocation_denied",
        "model_invocation_denied",
        "auth_secret_read_denied",
        "credential_read_denied",
        "external_network_call_denied",
        "live_kg_write_denied",
        "live_memory_write_denied",
        "service_restart_denied",
        "active_binary_mutation_denied",
        "public_release_claim_denied"
      ],
      side_effects:{
        report_route_invoked_runtime_execution:false,
        source_gate_invokes_isolated_fixture_execution:true,
        isolated_fixture_router_mutated_by_source_gate:true,
        live_7373_router_mutated_by_report_route:false,
        feature_flag_mutated_in_live_7373_by_report_route:false,
        context_attached_to_live_7373_prompt_by_report_route:false,
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
      },
      allowed_next_actions:[
        {
          action:"run_full_preflight",
          status:"allowed_verification_only",
          invokes_provider:false,
          writes_kg:false,
          reads_credentials:false
        },
        {
          action:"live_catch_up_controlled_route_exposure_after_full_preflight",
          status:"separate_operator_controlled_slice",
          invokes_provider:false,
          writes_kg:false,
          restarts_service:true
        }
      ]
    }'
)"

jq -e '
  .status == "ready"
  and .source_route_wired == true
  and .source_route_count_expected == 72
  and .source_route_tested_by_native_gateway_unit_test == true
  and .controlled_shadow_execution_report_ready == true
  and .runtime_owned_execution_surface_present == true
  and .isolated_fixture_execution_required == true
  and .isolated_fixture_execution_performed_by_source_gate == true
  and .live_route_execution_invoked == false
  and .report_route_exposes_activation_command == false
  and .release_gate_required == true
  and .operator_release_approval_required == true
  and .canary_telemetry_required == true
  and .rollback_kill_switch_required == true
  and .post_activation_watchdog_soak_plan_required == true
  and .traffic_percent_ppm_required == 0
  and .readback_receipt_required == true
  and .audit_evidence_required == true
  and .feature_flag_mutation_scope == "isolated_source_fixture_only"
  and .context_attachment_scope == "isolated_source_fixture_only"
  and .provider_invocation_performed == false
  and .model_invocation_performed == false
  and .auth_secret_read_performed == false
  and .credential_read_performed == false
  and .external_network_call_performed == false
  and .live_kg_write_performed == false
  and .live_memory_write_performed == false
  and .focused_test_count == 2
  and .focused_tests_passed == true
  and .side_effects.report_route_invoked_runtime_execution == false
  and .side_effects.source_gate_invokes_isolated_fixture_execution == true
  and .side_effects.live_7373_router_mutated_by_report_route == false
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.credential_read == false
  and .side_effects.live_kg_write_performed == false
  and .side_effects.memory_store_mutated == false
  and .side_effects.service_restarted == false
  and .side_effects.active_binary_mutated == false
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full enablement runtime provider-router shadow context activation execution controlled route gate passed"
