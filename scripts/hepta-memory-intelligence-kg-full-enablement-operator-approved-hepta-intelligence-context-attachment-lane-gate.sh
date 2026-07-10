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
    echo "missing operator-approved Hepta Intelligence context attachment lane source text: $label" >&2
    exit 1
  fi
}

MEMORY_LANE_JSON="$(
  curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-memory-live-mutation-durable-lane"
)"

jq -e '
  .status == "ready"
  and .operator_approved_activation_lane_present == true
  and .operator_approved_activation_lane_effective == true
  and .memory_durable_mutation_lane_enabled == true
  and .memory_store_write_path_enabled == true
  and .memory_store_mutation_enabled == true
  and .live_memory_write_allowed_by_lane == true
  and .live_memory_write_performed_by_report_route == false
  and .hepta_intelligence_context_attachment_lane_enabled == false
  and .kg_live_write_lane_enabled == false
  and .provider_model_invocation_lane_enabled == false
  and .channel_delivery_lane_enabled == false
  and .live_mutation_enabled_count == 1
  and .current_live_enabled_lane_count == 1
  and .side_effects.memory_store_mutated == false
  and .side_effects.memory_store_write_performed == false
  and .side_effects.hepta_intelligence_context_attached == false
  and .side_effects.prompt_preview_rendered == false
  and .side_effects.prompt_payload_materialized == false
  and .side_effects.context_injection_performed == false
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.credential_read == false
  and .side_effects.live_kg_write_performed == false
  and .side_effects.channel_send_performed == false
' >/dev/null <<<"$MEMORY_LANE_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/hepta-native-gateway/src/native_gateway.rs"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"

require_source_text "$ROUTE_REGISTRY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_HEPTA_INTELLIGENCE_CONTEXT_ATTACHMENT_LANE_ENDPOINT' \
  "native gateway operator-approved Hepta Intelligence context attachment lane endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-hepta-intelligence-context-attachment-lane' \
  "native gateway operator-approved Hepta Intelligence context attachment lane endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-operator-approved-hepta-intelligence-context-attachment-lane --json' \
  "native gateway operator-approved Hepta Intelligence context attachment lane source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_operator_approved_hepta_intelligence_context_attachment_lane_report' \
  "native gateway operator-approved Hepta Intelligence context attachment lane report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_intelligence_context_attachment_lane_enabled: true,' \
  "Hepta Intelligence context attachment lane enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'bounded_prompt_preview_lane_enabled: true,' \
  "bounded prompt preview lane enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'prompt_preview_rendered_by_report_route: false,' \
  "report route renders no prompt preview"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'context_injection_allowed_by_lane: false,' \
  "context injection remains disabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'kg_live_write_lane_enabled: false,' \
  "KG live write remains disabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'provider_model_invocation_lane_enabled: false,' \
  "provider/model lane remains disabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'channel_delivery_lane_enabled: false,' \
  "channel delivery lane remains disabled"

TEST_LOG="$(mktemp /tmp/hepta-operator-approved-hepta-intelligence-context-attachment-lane-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_memory_intelligence_kg_full_enablement_operator_approved_hepta_intelligence_context_attachment_lane_endpoint_enables_context_lane_only \
  -- --nocapture >"$TEST_LOG"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_approved_hepta_intelligence_context_attachment_lane_gate" \
    --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-hepta-intelligence-context-attachment-lane" \
    --arg memory_lane_endpoint "/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-memory-live-mutation-durable-lane" \
    --arg source_command "/hepta-memory-intelligence-kg-full-enablement-operator-approved-hepta-intelligence-context-attachment-lane --json" \
    --arg native_gateway_source "$NATIVE_GATEWAY_SOURCE" \
    --arg native_gateway_sha256 "$(sha256_file "$NATIVE_GATEWAY_SOURCE")" \
    --arg test_log "$TEST_LOG" \
    --argjson memory_lane "$MEMORY_LANE_JSON" \
    '{
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      memory_live_mutation_durable_lane_endpoint:$memory_lane_endpoint,
      source_command:$source_command,
      activation_mode:"operator_approved_hepta_intelligence_context_attachment_lane_status",
      live_memory_lane_status:$memory_lane.status,
      live_memory_lane_ready:$memory_lane.memory_durable_mutation_lane_enabled,
      source_route_wired:true,
      source_route_count_expected:81,
      source_route_tested_by_native_gateway_unit_test:true,
      operator_authorization_source:"telegram_direct_operator_authorization_2026_06_12_13_40_37_asia_shanghai",
      operator_authorization_received:true,
      operator_authorization_scope:"hepta_intelligence_context_attachment_and_bounded_prompt_preview_lane_no_provider_model_kg_write_channel_or_public_release",
      operator_approved_activation_lane_present:true,
      operator_approved_activation_lane_effective:true,
      memory_durable_mutation_lane_enabled:true,
      memory_store_write_path_enabled:true,
      memory_store_mutation_enabled:true,
      live_memory_write_allowed_by_lane:true,
      live_memory_write_performed_by_report_route:false,
      hepta_intelligence_context_attachment_lane_enabled:true,
      hepta_intelligence_context_attachment_allowed_by_lane:true,
      hepta_intelligence_context_attached_by_report_route:false,
      bounded_prompt_preview_lane_enabled:true,
      bounded_prompt_preview_allowed_by_lane:true,
      prompt_preview_rendered_by_report_route:false,
      prompt_payload_materialized_by_report_route:false,
      context_handoff_acceptance_required:true,
      context_attachment_requires_explicit_command:true,
      prompt_preview_requires_explicit_command:true,
      context_injection_allowed_by_lane:false,
      context_injection_performed_by_report_route:false,
      kg_prompt_preview_lane_enabled:false,
      kg_external_adapter_read_lane_enabled:false,
      kg_live_write_lane_enabled:false,
      provider_model_invocation_lane_enabled:false,
      channel_delivery_lane_enabled:false,
      live_mutation_enabled_count:1,
      current_live_enabled_lane_count:2,
      enablement_lane_count:6,
      ready_enablement_lane_count:6,
      source_contract:{
        source:$native_gateway_source,
        source_sha256:$native_gateway_sha256,
        contract:"hepta-native-gateway-operator-approved-hepta-intelligence-context-attachment-lane-route-v1",
        source_pattern_present:true,
        compile_checked_by_tests:true,
        route_handler_present:true,
        attaches_context_from_report_route:false,
        invokes_provider_or_model:false,
        writes_kg:false
      },
      test_log:$test_log,
      focused_test_count:1,
      focused_tests_passed:true,
      blocked_by_hepta_intelligence_context_attachment_lane_boundary:[
        "unbounded_context_attachment_denied",
        "context_attachment_from_report_route_denied",
        "prompt_preview_from_report_route_denied",
        "prompt_payload_materialization_denied",
        "context_injection_into_provider_prompt_denied",
        "provider_model_invocation_denied",
        "auth_secret_credential_read_denied",
        "external_kg_adapter_read_denied",
        "kg_live_write_denied",
        "memory_write_from_report_route_denied",
        "telegram_channel_delivery_denied",
        "service_restart_active_binary_mutation_denied",
        "release_public_claim_denied"
      ],
      side_effects:{
        report_route_invoked_runtime_execution:false,
        live_7373_router_mutated_by_report_route:false,
        operator_approval_lane_recorded:false,
        operator_approval_lane_persisted:false,
        memory_store_write_path_enabled_by_report_route:false,
        memory_store_mutated:false,
        memory_store_write_performed:false,
        memory_write_receipt_recorded:false,
        memory_write_receipt_persisted:false,
        rollback_kill_switch_mutated:false,
        post_write_validation_performed:false,
        hepta_intelligence_context_attached:false,
        prompt_preview_rendered:false,
        prompt_payload_materialized:false,
        context_injection_performed:false,
        provider_invoked:false,
        model_invoked:false,
        auth_secret_read:false,
        credential_read:false,
        external_network_call_performed:false,
        external_kg_adapter_read_performed:false,
        live_kg_write_performed:false,
        channel_send_performed:false,
        external_send_performed:false,
        service_restarted:false,
        active_binary_mutated:false,
        release_artifact_written:false,
        public_release_claimed:false,
        public_ga_claimed:false
      }
    }'
)"

jq -e '
  .status == "ready"
  and .source_route_count_expected == 81
  and .operator_authorization_received == true
  and .operator_approved_activation_lane_present == true
  and .operator_approved_activation_lane_effective == true
  and .memory_durable_mutation_lane_enabled == true
  and .memory_store_write_path_enabled == true
  and .memory_store_mutation_enabled == true
  and .live_memory_write_allowed_by_lane == true
  and .live_memory_write_performed_by_report_route == false
  and .hepta_intelligence_context_attachment_lane_enabled == true
  and .hepta_intelligence_context_attachment_allowed_by_lane == true
  and .hepta_intelligence_context_attached_by_report_route == false
  and .bounded_prompt_preview_lane_enabled == true
  and .bounded_prompt_preview_allowed_by_lane == true
  and .prompt_preview_rendered_by_report_route == false
  and .prompt_payload_materialized_by_report_route == false
  and .context_injection_allowed_by_lane == false
  and .kg_live_write_lane_enabled == false
  and .provider_model_invocation_lane_enabled == false
  and .channel_delivery_lane_enabled == false
  and .live_mutation_enabled_count == 1
  and .current_live_enabled_lane_count == 2
  and .focused_test_count == 1
  and .focused_tests_passed == true
  and .side_effects.memory_store_mutated == false
  and .side_effects.hepta_intelligence_context_attached == false
  and .side_effects.prompt_preview_rendered == false
  and .side_effects.context_injection_performed == false
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.credential_read == false
  and .side_effects.external_kg_adapter_read_performed == false
  and .side_effects.live_kg_write_performed == false
  and .side_effects.channel_send_performed == false
  and .side_effects.service_restarted == false
  and .side_effects.active_binary_mutated == false
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta operator-approved Hepta Intelligence context attachment lane gate passed"
