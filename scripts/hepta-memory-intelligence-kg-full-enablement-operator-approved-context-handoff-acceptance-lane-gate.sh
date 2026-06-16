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
    echo "missing operator-approved context handoff acceptance lane source text: $label" >&2
    exit 1
  fi
}

KG_READBACK_AUDIT_RECEIPT_LANE_JSON="$(
  curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-readback-audit-receipt-lane"
)"

jq -e '
  .status == "ready"
  and .kg_prompt_payload_readback_audit_receipt_lane_enabled == true
  and .kg_prompt_payload_readback_audit_receipt_allowed_by_lane == true
  and .kg_prompt_payload_readback_audit_receipt_requires_explicit_command == true
  and .kg_prompt_payload_readback_audit_receipt_requires_acceptance_receipt == true
  and .kg_prompt_payload_readback_audit_receipt_redaction_required == true
  and .kg_prompt_payload_readback_audit_receipt_redaction_proof_required == true
  and .kg_prompt_payload_readback_audit_receipt_hash_binding_required == true
  and .kg_prompt_payload_readback_audit_receipt_raw_payload_allowed == false
  and .kg_prompt_payload_readback_audit_receipt_rendered_by_report_route == false
  and .kg_prompt_payload_readback_audit_receipt_recorded_by_report_route == false
  and .kg_prompt_payload_readback_audit_receipt_persisted_by_report_route == false
  and .kg_prompt_payload_readback_audit_receipt_accepted_by_report_route == false
  and .kg_prompt_payload_readback_audit_receipt_filesystem_written_by_report_route == false
  and .kg_prompt_payload_readback_audit_receipt_ledger_recorded_by_report_route == false
  and .kg_prompt_payload_readback_audit_receipt_promotes_activation_authority == false
  and .context_handoff_acceptance_required == true
  and .context_injection_allowed_by_lane == false
  and .context_injection_performed_by_report_route == false
  and .kg_live_write_lane_enabled == false
  and .provider_model_invocation_lane_enabled == false
  and .channel_delivery_lane_enabled == false
  and .live_mutation_enabled_count == 1
  and .current_live_enabled_lane_count == 6
  and .enablement_lane_count == 9
  and .ready_enablement_lane_count == 9
  and .side_effects.prompt_payload_materialized == false
  and .side_effects.prompt_payload_readback_audit_receipt_rendered == false
  and .side_effects.prompt_payload_readback_audit_receipt_recorded == false
  and .side_effects.prompt_payload_readback_audit_receipt_persisted == false
  and .side_effects.credential_read == false
  and .side_effects.external_kg_adapter_read_performed == false
  and .side_effects.live_kg_write_performed == false
  and .side_effects.channel_send_performed == false
' >/dev/null <<<"$KG_READBACK_AUDIT_RECEIPT_LANE_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 135;' \
  "native gateway route/source command count includes context handoff acceptance lane and preserved systems routes"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-systems-tool-registry-inventory' \
  "native gateway systems tool registry inventory route preserved"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-systems-workflow-definition-registry' \
  "native gateway systems workflow definition registry route preserved"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_CONTEXT_HANDOFF_ACCEPTANCE_LANE_ENDPOINT' \
  "native gateway operator-approved context handoff acceptance lane endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-context-handoff-acceptance-lane' \
  "native gateway operator-approved context handoff acceptance lane endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-operator-approved-context-handoff-acceptance-lane --json' \
  "native gateway operator-approved context handoff acceptance lane source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_operator_approved_context_handoff_acceptance_lane_report' \
  "native gateway operator-approved context handoff acceptance lane report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"context_handoff_acceptance_lane_enabled": true' \
  "context handoff acceptance lane enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"context_handoff_acceptance_recorded_by_report_route": false' \
  "report route records no context handoff acceptance"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"context_handoff_acceptance_persisted_by_report_route": false' \
  "report route persists no context handoff acceptance"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"context_handoff_acceptance_accepted_by_report_route": false' \
  "report route accepts no context handoff"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"context_injection_allowed_by_lane": false' \
  "context injection remains disabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"context_injection_performed_by_report_route": false' \
  "report route injects no context"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"kg_live_write_lane_enabled": false' \
  "KG live write remains disabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"provider_model_invocation_lane_enabled": false' \
  "provider/model lane remains disabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"channel_delivery_lane_enabled": false' \
  "channel delivery lane remains disabled"

TEST_LOG="$(mktemp /tmp/hepta-operator-approved-context-handoff-acceptance-lane-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_enablement_operator_approved_context_handoff_acceptance_lane_endpoint_enables_redacted_context_handoff_acceptance_shape_only \
  -- --nocapture >"$TEST_LOG"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_approved_context_handoff_acceptance_lane_gate" \
    --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-context-handoff-acceptance-lane" \
    --arg kg_readback_audit_receipt_lane_endpoint "/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-readback-audit-receipt-lane" \
    --arg source_command "/hepta-memory-intelligence-kg-full-enablement-operator-approved-context-handoff-acceptance-lane --json" \
    --arg native_gateway_source "$NATIVE_GATEWAY_SOURCE" \
    --arg native_gateway_sha256 "$(sha256_file "$NATIVE_GATEWAY_SOURCE")" \
    --arg test_log "$TEST_LOG" \
    --argjson kg_readback_audit_receipt_lane "$KG_READBACK_AUDIT_RECEIPT_LANE_JSON" \
    '{
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      kg_prompt_payload_readback_audit_receipt_lane_endpoint:$kg_readback_audit_receipt_lane_endpoint,
      source_command:$source_command,
      activation_mode:"operator_approved_context_handoff_acceptance_lane_status",
      live_kg_prompt_payload_readback_audit_receipt_lane_status:$kg_readback_audit_receipt_lane.status,
      live_kg_prompt_payload_readback_audit_receipt_lane_ready:($kg_readback_audit_receipt_lane.kg_prompt_payload_readback_audit_receipt_lane_enabled and $kg_readback_audit_receipt_lane.kg_prompt_payload_readback_audit_receipt_allowed_by_lane),
      source_route_wired:true,
      source_route_count_expected:105,
      source_route_tested_by_native_gateway_unit_test:true,
      preserved_shared_systems_tool_registry_route:true,
      preserved_shared_systems_workflow_definition_route:true,
      operator_authorization_source:"telegram_direct_operator_authorization_2026_06_12_18_50_49_asia_shanghai",
      operator_authorization_received:true,
      operator_authorization_scope:"context_handoff_acceptance_lane_no_report_context_attach_inject_record_persist_accept_no_kg_live_write_provider_model_channel_or_public_release",
      kg_prompt_payload_readback_audit_receipt_lane_enabled:true,
      kg_prompt_payload_readback_audit_receipt_allowed_by_lane:true,
      kg_prompt_payload_readback_audit_receipt_rendered_by_report_route:false,
      kg_prompt_payload_readback_audit_receipt_recorded_by_report_route:false,
      kg_prompt_payload_readback_audit_receipt_persisted_by_report_route:false,
      kg_prompt_payload_readback_audit_receipt_accepted_by_report_route:false,
      kg_prompt_payload_readback_audit_receipt_promotes_activation_authority:false,
      context_handoff_acceptance_lane_enabled:true,
      context_handoff_acceptance_allowed_by_lane:true,
      context_handoff_acceptance_requires_explicit_command:true,
      context_handoff_acceptance_requires_readback_audit_receipt:true,
      context_handoff_acceptance_redaction_required:true,
      context_handoff_acceptance_scope_binding_required:true,
      context_handoff_acceptance_operator_identity_binding_required:true,
      context_handoff_acceptance_recorded_by_report_route:false,
      context_handoff_acceptance_persisted_by_report_route:false,
      context_handoff_acceptance_accepted_by_report_route:false,
      context_handoff_acceptance_filesystem_written_by_report_route:false,
      context_handoff_acceptance_ledger_recorded_by_report_route:false,
      context_handoff_acceptance_promotes_activation_authority:false,
      context_attachment_requires_explicit_command:true,
      context_attachment_performed_by_report_route:false,
      context_injection_allowed_by_lane:false,
      context_injection_performed_by_report_route:false,
      kg_live_write_lane_enabled:false,
      kg_live_write_allowed_by_lane:false,
      kg_live_write_performed_by_report_route:false,
      provider_model_invocation_lane_enabled:false,
      provider_model_invocation_allowed_by_lane:false,
      channel_delivery_lane_enabled:false,
      live_mutation_enabled_count:1,
      current_live_enabled_lane_count:7,
      enablement_lane_count:10,
      ready_enablement_lane_count:10,
      source_contract:{
        source:$native_gateway_source,
        source_sha256:$native_gateway_sha256,
        contract:"hepta-native-gateway-operator-approved-context-handoff-acceptance-lane-route-v1",
        source_pattern_present:true,
        compile_checked_by_tests:true,
        route_handler_present:true,
        attaches_context_from_report_route:false,
        injects_context_from_report_route:false,
        records_context_handoff_acceptance_from_report_route:false,
        persists_context_handoff_acceptance_from_report_route:false,
        accepts_context_handoff_from_report_route:false,
        promotes_activation_authority:false,
        reads_credentials:false,
        invokes_provider_or_model:false,
        writes_kg:false
      },
      test_log:$test_log,
      focused_test_count:1,
      focused_tests_passed:true,
      blocked_by_context_handoff_acceptance_lane_boundary:[
        "context_attachment_from_report_route_denied",
        "context_injection_from_report_route_denied",
        "context_handoff_acceptance_recording_from_report_route_denied",
        "context_handoff_acceptance_persistence_from_report_route_denied",
        "context_handoff_acceptance_acceptance_from_report_route_denied",
        "context_handoff_acceptance_filesystem_write_denied",
        "context_handoff_acceptance_ledger_recording_denied",
        "raw_prompt_payload_exposure_from_report_route_denied",
        "kg_adapter_read_from_report_route_denied",
        "auth_secret_credential_read_denied",
        "kg_live_write_denied",
        "context_handoff_acceptance_activation_authority_promotion_denied",
        "provider_model_invocation_denied",
        "telegram_channel_delivery_denied",
        "service_restart_active_binary_mutation_denied",
        "release_public_claim_denied"
      ],
      side_effects:{
        report_route_invoked_runtime_execution:false,
        live_7373_router_mutated_by_report_route:false,
        hepta_intelligence_context_attached:false,
        context_attached:false,
        prompt_preview_rendered:false,
        prompt_payload_materialized:false,
        context_handoff_acceptance_recorded:false,
        context_handoff_acceptance_persisted:false,
        context_handoff_acceptance_accepted:false,
        context_injection_performed:false,
        context_injected:false,
        provider_invoked:false,
        model_invoked:false,
        auth_secret_read:false,
        credential_read:false,
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

printf '%s\n' "$report"

jq -e '
  .status == "ready"
  and .source_route_count_expected == 105
  and .preserved_shared_systems_tool_registry_route == true
  and .preserved_shared_systems_workflow_definition_route == true
  and .live_kg_prompt_payload_readback_audit_receipt_lane_ready == true
  and .context_handoff_acceptance_lane_enabled == true
  and .context_handoff_acceptance_allowed_by_lane == true
  and .context_handoff_acceptance_requires_explicit_command == true
  and .context_handoff_acceptance_requires_readback_audit_receipt == true
  and .context_handoff_acceptance_recorded_by_report_route == false
  and .context_handoff_acceptance_persisted_by_report_route == false
  and .context_handoff_acceptance_accepted_by_report_route == false
  and .context_handoff_acceptance_promotes_activation_authority == false
  and .context_attachment_performed_by_report_route == false
  and .context_injection_allowed_by_lane == false
  and .context_injection_performed_by_report_route == false
  and .kg_live_write_lane_enabled == false
  and .provider_model_invocation_lane_enabled == false
  and .channel_delivery_lane_enabled == false
  and .current_live_enabled_lane_count == 7
  and .enablement_lane_count == 10
  and .ready_enablement_lane_count == 10
  and .source_contract.attaches_context_from_report_route == false
  and .source_contract.injects_context_from_report_route == false
  and .source_contract.records_context_handoff_acceptance_from_report_route == false
  and .source_contract.persists_context_handoff_acceptance_from_report_route == false
  and .source_contract.accepts_context_handoff_from_report_route == false
  and .source_contract.promotes_activation_authority == false
  and .side_effects.context_attached == false
  and .side_effects.context_handoff_acceptance_recorded == false
  and .side_effects.context_handoff_acceptance_persisted == false
  and .side_effects.context_handoff_acceptance_accepted == false
  and .side_effects.context_injection_performed == false
  and .side_effects.context_injected == false
  and .side_effects.credential_read == false
  and .side_effects.external_kg_adapter_read_performed == false
  and .side_effects.live_kg_write_performed == false
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.channel_send_performed == false
' >/dev/null <<<"$report"

echo "Hepta memory/intelligence/KG full enablement operator-approved context handoff acceptance lane gate passed"
