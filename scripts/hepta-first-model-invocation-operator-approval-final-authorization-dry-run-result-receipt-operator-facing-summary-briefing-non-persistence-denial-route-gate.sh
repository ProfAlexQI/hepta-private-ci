#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"
EXPECTED_ROUTE_COUNT="${HEPTA_EXPECTED_ROUTE_COUNT:-$(bash "$REPO_ROOT/scripts/lib/hepta-native-route-count.sh")}"

cd "$REPO_ROOT"

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

require_source_text() {
  local source_file="$1"
  local source_text="$2"
  local label="$3"

  if ! rg -Fq "$source_text" "$source_file"; then
    echo "missing first model invocation final authorization dry-run result receipt operator-facing summary/briefing source text: $label" >&2
    exit 1
  fi
}

source "$REPO_ROOT/scripts/lib/hepta-source-set.sh"
NATIVE_GATEWAY_SOURCE="hepta-native-gateway-source-set-v1"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes operator-facing summary/briefing route"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  'HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT' \
  "operator-facing summary/briefing endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  '/api/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-operator-facing-summary-briefing-non-persistence-denial' \
  "operator-facing summary/briefing endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  '/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-operator-facing-summary-briefing-non-persistence-denial --json' \
  "operator-facing summary/briefing source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report' \
  "operator-facing summary/briefing report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_no_delivery_no_provider_model_invocation' \
  "operator-facing summary/briefing execution mode"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'first_model_invocation_denial_routes_are_data_driven_and_side_effect_free' \
  "focused operator-facing summary/briefing unit test"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"action": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial"' \
  "next action remains final operator acknowledgement non-acceptance denial"

TEST_LOG="$(mktemp /tmp/hepta-first-model-invocation-final-authorization-dry-run-result-receipt-operator-summary-briefing-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  first_model_invocation_denial_routes_are_data_driven_and_side_effect_free \
  -- --nocapture >"$TEST_LOG"

live_route_status="skipped"
live_route_count=0
live_missing_route_count=0

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-operator-facing-summary-briefing-non-persistence-denial"
  )"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .implemented_route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial_ready == true
    and .first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
    and .canary_execution_mode == "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_no_delivery_no_provider_model_invocation"
    and .result_receipt_operator_facing_summary_briefing_state == "final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denied"
    and .operator_facing_summary_briefing_fixture_count == 8
    and .blocked_operator_facing_summary_briefing_fixture_count == 8
    and .noop_operator_facing_summary_briefing_fixture_count == 8
    and .allowed_operator_facing_summary_briefing_fixture_count == 0
    and .accepted_operator_facing_summary_briefing_fixture_count == 0
    and .operator_facing_summary_briefing_performed_count == 0
    and .operator_summary_recorded_count == 0
    and .operator_summary_persisted_count == 0
    and .operator_briefing_recorded_count == 0
    and .operator_briefing_persisted_count == 0
    and .operator_briefing_materialized_count == 0
    and .operator_summary_dashboard_published_count == 0
    and .operator_readback_recorded_count == 0
    and .operator_final_note_recorded_count == 0
    and .operator_final_note_delivered_count == 0
    and (.operator_facing_summary_briefing_fixtures | length) == 8
    and (.operator_facing_summary_briefing_fixtures | all(
      (.operator_facing_summary_briefing_status | startswith("blocked_"))
      and .receipt_noop_confirmed == true
    ))
    and .final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_readback_hash_matched == true
    and .operator_summary_recorded == false
    and .operator_summary_persisted == false
    and .operator_briefing_recorded == false
    and .operator_briefing_persisted == false
    and .operator_briefing_materialized == false
    and .operator_summary_dashboard_published == false
    and .operator_readback_recorded == false
    and .operator_final_note_recorded == false
    and .operator_final_note_delivered == false
    and .operator_acknowledgement_from_summary_accepted == false
    and .activation_from_operator_briefing_allowed == false
    and .activation_authority_from_operator_briefing_derived == false
    and .provider_invocation_authorized == false
    and .model_invocation_authorized == false
    and .provider_invocation_authorized_from_operator_briefing == false
    and .model_invocation_authorized_from_operator_briefing == false
    and .provider_invocation_budget == 0
    and .model_invocation_budget == 0
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_value_read == false
    and .credential_read == false
    and .secret_file_read == false
    and .provider_router_live_envelope_executed == false
    and .provider_prompt_injection_performed == false
    and .context_injection_performed == false
    and .kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .memory_store_write_performed == false
    and .channel_send_performed == false
    and .telegram_send_performed == false
    and .external_send_performed == false
    and (.audit_steps | length) == 6
    and .allowed_next_actions[0].action == "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial"
    and .allowed_next_actions[0].records_summary == false
    and .allowed_next_actions[0].persists_briefing == false
    and .allowed_next_actions[0].delivers_briefing == false
    and .allowed_next_actions[0].accepts_acknowledgement == false
    and .allowed_next_actions[0].invokes_provider == false
    and .allowed_next_actions[0].invokes_model == false
    and (.side_effects | to_entries | all(.value == false))
  ' >/dev/null <<<"$LIVE_JSON"
  live_route_status="$(jq -r '.status' <<<"$LIVE_JSON")"
  live_route_count="$(jq -r '.route_count' <<<"$LIVE_JSON")"
  live_missing_route_count="$(jq -r '.missing_route_count' <<<"$LIVE_JSON")"
fi

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_route_gate" \
    --arg endpoint "/api/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-operator-facing-summary-briefing-non-persistence-denial" \
    --arg source_command "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-operator-facing-summary-briefing-non-persistence-denial --json" \
    --arg native_gateway_sha256 "$(sha256_file "$NATIVE_GATEWAY_SOURCE")" \
    --arg test_log "$TEST_LOG" \
    --arg live_route_status "$live_route_status" \
    --argjson live_route_count "$live_route_count" \
    --argjson live_missing_route_count "$live_missing_route_count" \
    --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
    --argjson live_endpoint_checked "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && echo true || echo false)" \
    '{
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      source_command:$source_command,
      native_gateway_sha256:$native_gateway_sha256,
      focused_test_log:$test_log,
      live_endpoint_checked:$live_endpoint_checked,
      live_route_status:$live_route_status,
      live_route_count:$live_route_count,
      live_missing_route_count:$live_missing_route_count,
      expected_route_count:$expected_route_count,
      route_gate_ready:true,
      first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready:true,
      canary_execution_mode:"first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_no_delivery_no_provider_model_invocation",
      result_receipt_operator_facing_summary_briefing_state:"final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denied",
      operator_facing_summary_briefing_fixture_count:8,
      blocked_operator_facing_summary_briefing_fixture_count:8,
      allowed_operator_facing_summary_briefing_fixture_count:0,
      accepted_operator_facing_summary_briefing_fixture_count:0,
      operator_facing_summary_briefing_performed_count:0,
      operator_summary_recorded_count:0,
      operator_briefing_persisted_count:0,
      operator_final_note_delivered_count:0,
      provider_invocation_authorized:false,
      model_invocation_authorized:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      live_kg_write_performed:false,
      memory_store_write_performed:false,
      channel_send_performed:false,
      telegram_send_performed:false,
      external_send_performed:false,
      next_slice:"first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial"
    }'
)"

printf '%s\n' "$report"
echo "Hepta first model invocation operator approval final authorization dry-run result receipt operator-facing summary/briefing non-persistence denial route gate passed"
