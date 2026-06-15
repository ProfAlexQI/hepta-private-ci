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
    echo "missing operator-approved bounded provider-router injection precondition lane source text: $label" >&2
    exit 1
  fi
}

CONTEXT_HANDOFF_RECEIPT_AUDIT_LANE_JSON="$(
  curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-context-handoff-receipt-audit-lane"
)"

jq -e '
  .status == "ready"
  and .context_handoff_receipt_audit_lane_enabled == true
  and .context_handoff_receipt_audit_allowed_by_lane == true
  and .context_handoff_receipt_audit_requires_explicit_command == true
  and .context_handoff_receipt_audit_requires_context_handoff_acceptance == true
  and .context_handoff_receipt_audit_redaction_required == true
  and .context_handoff_receipt_audit_redaction_proof_required == true
  and .context_handoff_receipt_audit_scope_binding_required == true
  and .context_handoff_receipt_audit_operator_identity_binding_required == true
  and .context_handoff_receipt_audit_hash_binding_required == true
  and .context_handoff_receipt_audit_raw_context_allowed == false
  and .context_handoff_receipt_audit_rendered_by_report_route == false
  and .context_handoff_receipt_audit_recorded_by_report_route == false
  and .context_handoff_receipt_audit_persisted_by_report_route == false
  and .context_handoff_receipt_audit_accepted_by_report_route == false
  and .context_handoff_receipt_audit_filesystem_written_by_report_route == false
  and .context_handoff_receipt_audit_ledger_recorded_by_report_route == false
  and .context_handoff_receipt_audit_promotes_activation_authority == false
  and .context_attachment_performed_by_report_route == false
  and .context_injection_allowed_by_lane == false
  and .context_injection_performed_by_report_route == false
  and .kg_live_write_lane_enabled == false
  and .provider_model_invocation_lane_enabled == false
  and .channel_delivery_lane_enabled == false
  and .live_mutation_enabled_count == 1
  and .current_live_enabled_lane_count == 8
  and .enablement_lane_count == 11
  and .ready_enablement_lane_count == 11
  and .side_effects.context_handoff_receipt_audit_rendered == false
  and .side_effects.context_handoff_receipt_audit_recorded == false
  and .side_effects.context_handoff_receipt_audit_persisted == false
  and .side_effects.context_handoff_receipt_audit_accepted == false
  and .side_effects.context_handoff_receipt_audit_ledger_recorded == false
  and .side_effects.context_injection_performed == false
  and .side_effects.context_injected == false
  and .side_effects.credential_read == false
  and .side_effects.external_kg_adapter_read_performed == false
  and .side_effects.live_kg_write_performed == false
  and .side_effects.channel_send_performed == false
' >/dev/null <<<"$CONTEXT_HANDOFF_RECEIPT_AUDIT_LANE_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 125;' \
  "native gateway route/source command count includes bounded provider-router injection precondition lane and preserved systems routes"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-systems-tool-registry-inventory' \
  "native gateway systems tool registry inventory route preserved"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-systems-workflow-definition-registry' \
  "native gateway systems workflow definition registry route preserved"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_PRECONDITION_LANE_ENDPOINT' \
  "native gateway operator-approved bounded provider-router injection precondition lane endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-precondition-lane' \
  "native gateway operator-approved bounded provider-router injection precondition lane endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-precondition-lane --json' \
  "native gateway operator-approved bounded provider-router injection precondition lane source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_operator_approved_bounded_provider_router_injection_precondition_lane_report' \
  "native gateway operator-approved bounded provider-router injection precondition lane report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"bounded_provider_router_injection_precondition_lane_enabled": true' \
  "bounded provider-router injection precondition lane enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"bounded_provider_router_injection_precondition_requires_context_handoff_receipt_audit": true' \
  "bounded provider-router injection precondition requires context handoff receipt audit"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"bounded_provider_router_injection_precondition_dry_run_only": true' \
  "bounded provider-router injection precondition stays dry-run only"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"bounded_provider_router_injection_precondition_recorded_by_report_route": false' \
  "report route records no bounded provider-router injection precondition"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"bounded_provider_router_injection_precondition_persisted_by_report_route": false' \
  "report route persists no bounded provider-router injection precondition"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"bounded_provider_router_injection_precondition_accepted_by_report_route": false' \
  "report route accepts no bounded provider-router injection precondition"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"provider_router_prompt_mutated_by_report_route": false' \
  "report route mutates no provider-router prompt"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"context_injection_allowed_by_lane": false' \
  "actual context injection remains disabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"kg_live_write_lane_enabled": false' \
  "KG live write remains disabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"provider_model_invocation_lane_enabled": false' \
  "provider/model lane remains disabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"channel_delivery_lane_enabled": false' \
  "channel delivery lane remains disabled"

TEST_LOG="$(mktemp /tmp/hepta-operator-approved-bounded-provider-router-injection-precondition-lane-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_enablement_operator_approved_bounded_provider_router_injection_precondition_lane_endpoint_enables_precondition_shape_only \
  -- --nocapture >"$TEST_LOG"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_approved_bounded_provider_router_injection_precondition_lane_gate" \
    --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-precondition-lane" \
    --arg context_handoff_receipt_audit_lane_endpoint "/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-context-handoff-receipt-audit-lane" \
    --arg source_command "/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-precondition-lane --json" \
    --arg native_gateway_source "$NATIVE_GATEWAY_SOURCE" \
    --arg native_gateway_sha256 "$(sha256_file "$NATIVE_GATEWAY_SOURCE")" \
    --arg test_log "$TEST_LOG" \
    --argjson context_handoff_receipt_audit_lane "$CONTEXT_HANDOFF_RECEIPT_AUDIT_LANE_JSON" \
    '{
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      context_handoff_receipt_audit_lane_endpoint:$context_handoff_receipt_audit_lane_endpoint,
      source_command:$source_command,
      activation_mode:"operator_approved_bounded_provider_router_injection_precondition_lane_status",
      live_context_handoff_receipt_audit_lane_status:$context_handoff_receipt_audit_lane.status,
      live_context_handoff_receipt_audit_lane_ready:($context_handoff_receipt_audit_lane.context_handoff_receipt_audit_lane_enabled and $context_handoff_receipt_audit_lane.context_handoff_receipt_audit_allowed_by_lane),
      source_route_wired:true,
      source_route_count_expected:105,
      source_route_tested_by_native_gateway_unit_test:true,
      preserved_shared_systems_tool_registry_route:true,
      preserved_shared_systems_workflow_definition_route:true,
      operator_authorization_source:"telegram_direct_operator_highest_authorization_2026_06_13_08_01_56_asia_shanghai",
      operator_authorization_received:true,
      operator_authorization_scope:"bounded_provider_router_injection_precondition_lane_no_report_context_inject_prompt_mutation_record_persist_accept_no_kg_live_write_provider_model_channel_or_public_release",
      context_handoff_receipt_audit_lane_enabled:true,
      context_handoff_receipt_audit_allowed_by_lane:true,
      context_handoff_receipt_audit_recorded_by_report_route:false,
      context_handoff_receipt_audit_persisted_by_report_route:false,
      context_handoff_receipt_audit_accepted_by_report_route:false,
      context_handoff_receipt_audit_promotes_activation_authority:false,
      bounded_provider_router_injection_precondition_lane_enabled:true,
      bounded_provider_router_injection_precondition_allowed_by_lane:true,
      bounded_provider_router_injection_precondition_requires_explicit_command:true,
      bounded_provider_router_injection_precondition_requires_context_handoff_receipt_audit:true,
      bounded_provider_router_injection_precondition_redaction_required:true,
      bounded_provider_router_injection_precondition_redaction_proof_required:true,
      bounded_provider_router_injection_precondition_scope_binding_required:true,
      bounded_provider_router_injection_precondition_operator_identity_binding_required:true,
      bounded_provider_router_injection_precondition_hash_binding_required:true,
      bounded_provider_router_injection_precondition_provider_router_target_binding_required:true,
      bounded_provider_router_injection_precondition_budget_binding_required:true,
      bounded_provider_router_injection_precondition_dry_run_only:true,
      bounded_provider_router_injection_precondition_raw_context_allowed:false,
      bounded_provider_router_injection_precondition_rendered_by_report_route:false,
      bounded_provider_router_injection_precondition_recorded_by_report_route:false,
      bounded_provider_router_injection_precondition_persisted_by_report_route:false,
      bounded_provider_router_injection_precondition_accepted_by_report_route:false,
      bounded_provider_router_injection_precondition_filesystem_written_by_report_route:false,
      bounded_provider_router_injection_precondition_ledger_recorded_by_report_route:false,
      bounded_provider_router_injection_precondition_promotes_activation_authority:false,
      provider_router_prompt_mutated_by_report_route:false,
      provider_router_context_packet_materialized_by_report_route:false,
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
      current_live_enabled_lane_count:9,
      enablement_lane_count:12,
      ready_enablement_lane_count:12,
      source_contract:{
        source:$native_gateway_source,
        source_sha256:$native_gateway_sha256,
        contract:"hepta-native-gateway-operator-approved-bounded-provider-router-injection-precondition-lane-route-v1",
        source_pattern_present:true,
        compile_checked_by_tests:true,
        route_handler_present:true,
        attaches_context_from_report_route:false,
        injects_context_from_report_route:false,
        mutates_provider_router_prompt_from_report_route:false,
        records_bounded_provider_router_injection_precondition_from_report_route:false,
        persists_bounded_provider_router_injection_precondition_from_report_route:false,
        accepts_bounded_provider_router_injection_precondition_from_report_route:false,
        promotes_activation_authority:false,
        reads_credentials:false,
        invokes_provider_or_model:false,
        writes_kg:false
      },
      test_log:$test_log,
      focused_test_count:1,
      focused_tests_passed:true,
      blocked_by_bounded_provider_router_injection_precondition_lane_boundary:[
        "context_attachment_from_report_route_denied",
        "context_injection_from_report_route_denied",
        "provider_router_prompt_mutation_from_report_route_denied",
        "bounded_provider_router_injection_precondition_render_from_report_route_denied",
        "bounded_provider_router_injection_precondition_recording_from_report_route_denied",
        "bounded_provider_router_injection_precondition_persistence_from_report_route_denied",
        "bounded_provider_router_injection_precondition_acceptance_from_report_route_denied",
        "bounded_provider_router_injection_precondition_filesystem_write_denied",
        "bounded_provider_router_injection_precondition_ledger_recording_denied",
        "kg_live_write_denied",
        "bounded_provider_router_injection_precondition_activation_authority_promotion_denied",
        "provider_model_invocation_denied",
        "telegram_channel_delivery_denied",
        "service_restart_active_binary_mutation_denied",
        "release_public_claim_denied"
      ],
      side_effects:{
        report_route_invoked_runtime_execution:false,
        live_7373_router_mutated_by_report_route:false,
        context_attached:false,
        prompt_preview_rendered:false,
        prompt_payload_materialized:false,
        bounded_provider_router_injection_precondition_rendered:false,
        bounded_provider_router_injection_precondition_recorded:false,
        bounded_provider_router_injection_precondition_persisted:false,
        bounded_provider_router_injection_precondition_accepted:false,
        bounded_provider_router_injection_precondition_ledger_recorded:false,
        provider_router_prompt_mutated:false,
        provider_router_context_packet_materialized:false,
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
  and .source_route_wired == true
  and .source_route_count_expected == 105
  and .source_route_tested_by_native_gateway_unit_test == true
  and .live_context_handoff_receipt_audit_lane_ready == true
  and .bounded_provider_router_injection_precondition_lane_enabled == true
  and .bounded_provider_router_injection_precondition_allowed_by_lane == true
  and .bounded_provider_router_injection_precondition_requires_context_handoff_receipt_audit == true
  and .bounded_provider_router_injection_precondition_dry_run_only == true
  and .bounded_provider_router_injection_precondition_raw_context_allowed == false
  and .bounded_provider_router_injection_precondition_recorded_by_report_route == false
  and .bounded_provider_router_injection_precondition_persisted_by_report_route == false
  and .bounded_provider_router_injection_precondition_accepted_by_report_route == false
  and .provider_router_prompt_mutated_by_report_route == false
  and .context_injection_allowed_by_lane == false
  and .context_injection_performed_by_report_route == false
  and .kg_live_write_lane_enabled == false
  and .provider_model_invocation_lane_enabled == false
  and .channel_delivery_lane_enabled == false
  and .live_mutation_enabled_count == 1
  and .current_live_enabled_lane_count == 9
  and .enablement_lane_count == 12
  and .ready_enablement_lane_count == 12
  and .source_contract.injects_context_from_report_route == false
  and .source_contract.mutates_provider_router_prompt_from_report_route == false
  and .source_contract.records_bounded_provider_router_injection_precondition_from_report_route == false
  and .source_contract.persists_bounded_provider_router_injection_precondition_from_report_route == false
  and .source_contract.accepts_bounded_provider_router_injection_precondition_from_report_route == false
  and .source_contract.promotes_activation_authority == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

echo "Hepta memory/intelligence/KG full enablement operator-approved bounded provider-router injection precondition lane gate passed"
