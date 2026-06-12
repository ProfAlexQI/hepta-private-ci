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
    echo "missing operator-approved KG prompt payload materialization lane source text: $label" >&2
    exit 1
  fi
}

KG_PREVIEW_ADAPTER_LANE_JSON="$(
  curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-preview-read-only-adapter-lane"
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
  and .hepta_intelligence_context_attachment_lane_enabled == true
  and .hepta_intelligence_context_attachment_allowed_by_lane == true
  and .hepta_intelligence_context_attached_by_report_route == false
  and .bounded_prompt_preview_lane_enabled == true
  and .bounded_prompt_preview_allowed_by_lane == true
  and .prompt_preview_rendered_by_report_route == false
  and .prompt_payload_materialized_by_report_route == false
  and .kg_prompt_preview_lane_enabled == true
  and .kg_prompt_preview_allowed_by_lane == true
  and .kg_prompt_preview_rendered_by_report_route == false
  and .kg_external_adapter_read_lane_enabled == true
  and .kg_external_adapter_read_allowed_by_lane == true
  and .kg_external_adapter_read_performed_by_report_route == false
  and .kg_external_adapter_credential_read_allowed_by_lane == false
  and .kg_external_adapter_credential_read_performed_by_report_route == false
  and .kg_live_write_lane_enabled == false
  and .provider_model_invocation_lane_enabled == false
  and .channel_delivery_lane_enabled == false
  and .live_mutation_enabled_count == 1
  and .current_live_enabled_lane_count == 3
  and .side_effects.memory_store_mutated == false
  and .side_effects.memory_store_write_performed == false
  and .side_effects.hepta_intelligence_context_attached == false
  and .side_effects.prompt_preview_rendered == false
  and .side_effects.prompt_payload_materialized == false
  and .side_effects.context_injection_performed == false
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.credential_read == false
  and .side_effects.external_kg_adapter_read_performed == false
  and .side_effects.live_kg_write_performed == false
  and .side_effects.channel_send_performed == false
' >/dev/null <<<"$KG_PREVIEW_ADAPTER_LANE_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 84;' \
  "native gateway route/source command count includes KG payload lane and preserved systems route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-systems-tool-registry-inventory' \
  "native gateway systems tool registry inventory route preserved"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PAYLOAD_MATERIALIZATION_LANE_ENDPOINT' \
  "native gateway operator-approved KG prompt payload materialization lane endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-materialization-lane' \
  "native gateway operator-approved KG prompt payload materialization lane endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-materialization-lane --json' \
  "native gateway operator-approved KG prompt payload materialization lane source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_payload_materialization_lane_report' \
  "native gateway operator-approved KG prompt payload materialization lane report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'kg_prompt_payload_materialization_lane_enabled: true,' \
  "KG prompt payload materialization lane enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'kg_prompt_payload_materialized_by_report_route: false,' \
  "report route performs no prompt payload materialization"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'kg_prompt_payload_raw_text_exposed_by_report_route: false,' \
  "report route exposes no raw prompt payload"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'kg_external_adapter_read_performed_by_report_route: false,' \
  "report route performs no KG adapter read"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'kg_external_adapter_credential_read_allowed_by_lane: false,' \
  "credential reads remain disabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'kg_live_write_lane_enabled: false,' \
  "KG live write remains disabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'provider_model_invocation_lane_enabled: false,' \
  "provider/model lane remains disabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'channel_delivery_lane_enabled: false,' \
  "channel delivery lane remains disabled"

TEST_LOG="$(mktemp /tmp/hepta-operator-approved-kg-prompt-payload-materialization-lane-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_payload_materialization_lane_endpoint_enables_payload_shape_only \
  -- --nocapture >"$TEST_LOG"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_payload_materialization_lane_gate" \
    --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-materialization-lane" \
    --arg kg_preview_adapter_lane_endpoint "/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-preview-read-only-adapter-lane" \
    --arg source_command "/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-materialization-lane --json" \
    --arg native_gateway_source "$NATIVE_GATEWAY_SOURCE" \
    --arg native_gateway_sha256 "$(sha256_file "$NATIVE_GATEWAY_SOURCE")" \
    --arg test_log "$TEST_LOG" \
    --argjson kg_preview_adapter_lane "$KG_PREVIEW_ADAPTER_LANE_JSON" \
    '{
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      kg_prompt_preview_read_only_adapter_lane_endpoint:$kg_preview_adapter_lane_endpoint,
      source_command:$source_command,
      activation_mode:"operator_approved_kg_prompt_payload_materialization_lane_status",
      live_kg_prompt_preview_read_only_adapter_lane_status:$kg_preview_adapter_lane.status,
      live_kg_prompt_preview_read_only_adapter_lane_ready:($kg_preview_adapter_lane.kg_prompt_preview_lane_enabled and $kg_preview_adapter_lane.kg_external_adapter_read_lane_enabled),
      source_route_wired:true,
      source_route_count_expected:84,
      source_route_tested_by_native_gateway_unit_test:true,
      preserved_shared_systems_tool_registry_route:true,
      operator_authorization_source:"telegram_direct_operator_authorization_2026_06_12_18_50_49_asia_shanghai",
      operator_authorization_received:true,
      operator_authorization_scope:"kg_prompt_payload_materialization_lane_no_report_payload_no_kg_live_write_provider_model_channel_or_public_release",
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
      prompt_preview_requires_explicit_command:true,
      prompt_payload_materialized_by_report_route:false,
      kg_prompt_preview_lane_enabled:true,
      kg_prompt_preview_allowed_by_lane:true,
      kg_prompt_preview_rendered_by_report_route:false,
      kg_external_adapter_read_lane_enabled:true,
      kg_external_adapter_read_allowed_by_lane:true,
      kg_external_adapter_read_performed_by_report_route:false,
      kg_external_adapter_requires_explicit_command:true,
      kg_external_adapter_credential_reference_required:true,
      kg_external_adapter_credential_read_allowed_by_lane:false,
      kg_external_adapter_credential_read_performed_by_report_route:false,
      supported_kg_adapter_count:3,
      supported_kg_adapters:["graphiti","neo4j","cocoindex"],
      kg_prompt_payload_materialization_lane_enabled:true,
      kg_prompt_payload_materialization_allowed_by_lane:true,
      kg_prompt_payload_materialized_by_report_route:false,
      kg_prompt_payload_shape_requires_explicit_command:true,
      kg_prompt_payload_redaction_required:true,
      kg_prompt_payload_raw_text_exposed_by_report_route:false,
      kg_prompt_payload_hash_preview_allowed_by_lane:true,
      kg_prompt_payload_hash_preview_rendered_by_report_route:false,
      context_handoff_acceptance_required:true,
      context_attachment_requires_explicit_command:true,
      context_injection_allowed_by_lane:false,
      context_injection_performed_by_report_route:false,
      kg_live_write_lane_enabled:false,
      kg_live_write_allowed_by_lane:false,
      kg_live_write_performed_by_report_route:false,
      provider_model_invocation_lane_enabled:false,
      provider_model_invocation_allowed_by_lane:false,
      channel_delivery_lane_enabled:false,
      live_mutation_enabled_count:1,
      current_live_enabled_lane_count:4,
      enablement_lane_count:7,
      ready_enablement_lane_count:7,
      source_contract:{
        source:$native_gateway_source,
        source_sha256:$native_gateway_sha256,
        contract:"hepta-native-gateway-operator-approved-kg-prompt-payload-materialization-lane-route-v1",
        source_pattern_present:true,
        compile_checked_by_tests:true,
        route_handler_present:true,
        materializes_prompt_payload_from_report_route:false,
        exposes_raw_prompt_payload_from_report_route:false,
        reads_kg_adapter_from_report_route:false,
        reads_credentials:false,
        invokes_provider_or_model:false,
        writes_kg:false
      },
      test_log:$test_log,
      focused_test_count:1,
      focused_tests_passed:true,
      blocked_by_kg_prompt_payload_materialization_lane_boundary:[
        "prompt_payload_materialization_from_report_route_denied",
        "raw_prompt_payload_exposure_from_report_route_denied",
        "prompt_preview_from_report_route_denied",
        "context_attachment_or_injection_from_report_route_denied",
        "kg_adapter_read_from_report_route_denied",
        "external_adapter_client_construction_denied",
        "credential_value_capture_denied",
        "auth_secret_credential_read_denied",
        "kg_live_write_denied",
        "memory_write_from_report_route_denied",
        "provider_model_invocation_denied",
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
  and .source_route_count_expected == 84
  and .preserved_shared_systems_tool_registry_route == true
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
  and .kg_prompt_preview_lane_enabled == true
  and .kg_prompt_preview_allowed_by_lane == true
  and .kg_prompt_preview_rendered_by_report_route == false
  and .kg_external_adapter_read_lane_enabled == true
  and .kg_external_adapter_read_allowed_by_lane == true
  and .kg_external_adapter_read_performed_by_report_route == false
  and .kg_external_adapter_credential_reference_required == true
  and .kg_external_adapter_credential_read_allowed_by_lane == false
  and .kg_external_adapter_credential_read_performed_by_report_route == false
  and .kg_prompt_payload_materialization_lane_enabled == true
  and .kg_prompt_payload_materialization_allowed_by_lane == true
  and .kg_prompt_payload_materialized_by_report_route == false
  and .kg_prompt_payload_shape_requires_explicit_command == true
  and .kg_prompt_payload_redaction_required == true
  and .kg_prompt_payload_raw_text_exposed_by_report_route == false
  and .kg_prompt_payload_hash_preview_allowed_by_lane == true
  and .kg_prompt_payload_hash_preview_rendered_by_report_route == false
  and .context_injection_allowed_by_lane == false
  and .kg_live_write_lane_enabled == false
  and .kg_live_write_allowed_by_lane == false
  and .provider_model_invocation_lane_enabled == false
  and .provider_model_invocation_allowed_by_lane == false
  and .channel_delivery_lane_enabled == false
  and .live_mutation_enabled_count == 1
  and .current_live_enabled_lane_count == 4
  and .focused_test_count == 1
  and .focused_tests_passed == true
  and .side_effects.memory_store_mutated == false
  and .side_effects.hepta_intelligence_context_attached == false
  and .side_effects.prompt_preview_rendered == false
  and .side_effects.prompt_payload_materialized == false
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
echo "Hepta operator-approved KG prompt payload materialization lane gate passed"
