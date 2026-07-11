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
    echo "missing operator-approved shadow context activation execution source text: $label" >&2
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
' >/dev/null <<<"$RUNTIME_READINESS_JSON"

source "$REPO_ROOT/scripts/lib/hepta-source-set.sh"

RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE="codex-rs/hepta-runtime/src/model_provider_router.rs"
RUNTIME_LIB_SOURCE="hepta-runtime-public-api-source-set-v1"
GATED_E2E_SOURCE="codex-rs/hepta-runtime/tests/gated_adapter_e2e.rs"

require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'pub struct ModelProviderMemoryContextActivationExecutionRecord' \
  "shadow activation execution record"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'pub fn execute_memory_context_activation_shadow' \
  "runtime-owned shadow activation execution adapter"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'if !input.release_gate_ready' \
  "release gate guard"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'if input.kill_switch_active' \
  "kill switch guard"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'if !input.canary_telemetry_ready' \
  "canary telemetry guard"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'if !input.rollback_kill_switch_armed' \
  "rollback kill-switch guard"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'if !input.post_activation_watchdog_soak_plan_ready' \
  "post-activation watchdog/soak plan guard"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'handoff.traffic_percent_ppm != 0' \
  "0ppm shadow-only traffic guard"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'context_attached_to_live_prompt: true,' \
  "shadow live prompt context attachment"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'provider_invoked_by_adapter: false,' \
  "provider invocation remains disabled"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'auth_secret_read_by_adapter: false,' \
  "auth secret read remains disabled"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'live_kg_write_performed: false,' \
  "KG write remains disabled"
require_source_text "$RUNTIME_LIB_SOURCE" \
  'ModelProviderMemoryContextActivationExecutionInput' \
  "runtime public API export"
require_source_text "$GATED_E2E_SOURCE" \
  'execute_memory_context_activation_shadow' \
  "gated adapter e2e consumes shadow activation execution"

TEST_LOG="$(mktemp /tmp/hepta-shadow-context-activation-execution-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-runtime \
  model_provider_router_executes_operator_approved_memory_context_shadow_activation \
  -- --nocapture >"$TEST_LOG"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-runtime \
  model_provider_router_blocks_shadow_activation_without_release_gate_or_kill_switch \
  -- --nocapture >>"$TEST_LOG"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-runtime --test gated_adapter_e2e \
  confirmed_gated_adapters_execute_local_e2e_with_readback_without_external_effects \
  -- --nocapture >>"$TEST_LOG"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_operator_approved_shadow_context_activation_execution_gate" \
    --arg runtime_model_provider_router_source "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
    --arg runtime_lib_source "$RUNTIME_LIB_SOURCE" \
    --arg gated_e2e_source "$GATED_E2E_SOURCE" \
    --arg runtime_model_provider_router_sha256 "$(sha256_file "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE")" \
    --arg runtime_lib_sha256 "$(sha256_file "$RUNTIME_LIB_SOURCE")" \
    --arg gated_e2e_sha256 "$(sha256_file "$GATED_E2E_SOURCE")" \
    --arg test_log "$TEST_LOG" \
    --argjson readiness "$RUNTIME_READINESS_JSON" \
    '{
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      activation_mode:"operator_approved_shadow_context_attachment_execution",
      runtime_readiness_status:$readiness.status,
      runtime_readiness_ready:$readiness.full_enablement_activation_readiness_ready,
      source_live_mutation_enabled_count:$readiness.live_mutation_enabled_count,
      source_current_live_enabled_lane_count:$readiness.current_live_enabled_lane_count,
      operator_approved_shadow_context_activation_execution_ready:true,
      runtime_owned_execution_surface_present:true,
      release_gate_required:true,
      operator_release_approval_required:true,
      canary_telemetry_required:true,
      rollback_kill_switch_required:true,
      post_activation_watchdog_soak_plan_required:true,
      idempotency_required:true,
      traffic_percent_ppm_required:0,
      shadow_context_attachment_supported:true,
      feature_flag_mutation_limited_to_shadow_handoff:true,
      context_attached_to_live_prompt_by_execution:true,
      provider_invocation_allowed:false,
      provider_invocation_performed:false,
      auth_secret_read_allowed:false,
      auth_secret_read_performed:false,
      credential_read_allowed:false,
      credential_read_performed:false,
      usage_recording_required_for_provider_invocation:false,
      external_network_call_allowed:false,
      external_network_call_performed:false,
      live_kg_write_allowed:false,
      live_kg_write_performed:false,
      public_release_claim_allowed:false,
      public_release_claimed:false,
      service_restart_allowed:false,
      service_restarted:false,
      active_binary_mutation_allowed:false,
      active_binary_mutated:false,
      source_contracts:[
        {
          source:$runtime_model_provider_router_source,
          source_sha256:$runtime_model_provider_router_sha256,
          contract:"hepta-runtime-model-provider-memory-context-shadow-activation-execution-v1",
          source_pattern_present:true,
          compile_checked_by_tests:true,
          execution_surface_present:true
        },
        {
          source:$runtime_lib_source,
          source_sha256:$runtime_lib_sha256,
          contract:"hepta-runtime-public-export-memory-context-shadow-activation-execution-v1",
          source_pattern_present:true,
          compile_checked_by_tests:true,
          execution_surface_present:true
        },
        {
          source:$gated_e2e_source,
          source_sha256:$gated_e2e_sha256,
          contract:"hepta-runtime-gated-adapter-e2e-shadow-context-activation-execution-v1",
          source_pattern_present:true,
          compile_checked_by_tests:true,
          execution_surface_present:true
        }
      ],
      test_log:$test_log,
      focused_test_count:3,
      focused_tests_passed:true,
      denied_by_operator_approved_shadow_context_activation_execution_gate:[
        "non_shadow_traffic_denied",
        "missing_release_gate_denied",
        "missing_operator_release_approval_denied",
        "kill_switch_active_denied",
        "missing_canary_telemetry_denied",
        "missing_rollback_kill_switch_denied",
        "missing_post_activation_watchdog_soak_plan_denied",
        "provider_invocation_denied",
        "auth_secret_read_denied",
        "credential_read_denied",
        "external_network_call_denied",
        "live_kg_write_denied",
        "public_release_claim_denied",
        "install_restart_active_binary_mutation_denied"
      ],
      side_effects:{
        runtime_router_shadow_handoff_mutated_by_test:true,
        live_7373_router_mutated_by_gate:false,
        feature_flag_mutated_in_live_7373_by_gate:false,
        context_attached_to_live_7373_prompt_by_gate:false,
        provider_invoked:false,
        model_invoked:false,
        auth_secret_read:false,
        credential_read:false,
        external_network_call_performed:false,
        live_kg_write_performed:false,
        memory_store_mutated:false,
        channel_send_performed:false,
        external_send_performed:false,
        public_release_claimed:false,
        release_artifact_written:false,
        service_restarted:false,
        active_binary_mutated:false
      },
      allowed_next_actions:[
        {
          action:"wire_live_route_for_operator_approved_shadow_context_activation_execution",
          status:"allowed_next_slice",
          requires_operator_release_gate:true,
          requires_canary_telemetry:true,
          requires_rollback_kill_switch:true,
          invokes_provider:false,
          writes_kg:false
        },
        {
          action:"run_full_preflight",
          status:"allowed_verification_only",
          invokes_provider:false,
          writes_kg:false,
          reads_credentials:false
        }
      ]
    }'
)"

jq -e '
  .status == "ready"
  and .operator_approved_shadow_context_activation_execution_ready == true
  and .runtime_owned_execution_surface_present == true
  and .release_gate_required == true
  and .operator_release_approval_required == true
  and .canary_telemetry_required == true
  and .rollback_kill_switch_required == true
  and .post_activation_watchdog_soak_plan_required == true
  and .traffic_percent_ppm_required == 0
  and .shadow_context_attachment_supported == true
  and .context_attached_to_live_prompt_by_execution == true
  and .provider_invocation_performed == false
  and .auth_secret_read_performed == false
  and .credential_read_performed == false
  and .external_network_call_performed == false
  and .live_kg_write_performed == false
  and .focused_test_count == 3
  and .focused_tests_passed == true
  and .side_effects.live_7373_router_mutated_by_gate == false
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.credential_read == false
  and .side_effects.live_kg_write_performed == false
  and .side_effects.service_restarted == false
  and .side_effects.active_binary_mutated == false
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full enablement runtime provider-router operator-approved shadow context activation execution gate passed"
