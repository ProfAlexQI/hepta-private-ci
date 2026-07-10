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
    echo "missing first model invocation final authorization dry-run result receipt retention/expiry/GC source text: $label" >&2
    exit 1
  fi
}

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes retention/expiry/GC route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT' \
  "retention/expiry/GC endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-retention-expiry-garbage-collection-denial' \
  "retention/expiry/GC endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-retention-expiry-garbage-collection-denial --json' \
  "retention/expiry/GC source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_report' \
  "retention/expiry/GC report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_no_provider_model_invocation' \
  "retention/expiry/GC execution mode"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_endpoint_blocks_retention_gc_and_invocation_side_effects' \
  "focused retention/expiry/GC unit test"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"action": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial"' \
  "next action remains export/query/observability denial before invocation"

TEST_LOG="$(mktemp /tmp/hepta-first-model-invocation-final-authorization-dry-run-result-receipt-retention-expiry-gc-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_endpoint_blocks_retention_gc_and_invocation_side_effects \
  -- --nocapture >"$TEST_LOG"

live_route_status="skipped"
live_route_count=0
live_missing_route_count=0

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-retention-expiry-garbage-collection-denial"
  )"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .implemented_route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial_ready == true
    and .first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_ready == true
    and .canary_execution_mode == "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_no_provider_model_invocation"
    and .result_receipt_retention_expiry_garbage_collection_state == "final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denied"
    and .retention_expiry_garbage_collection_fixture_count == 8
    and .blocked_retention_expiry_garbage_collection_fixture_count == 8
    and .noop_retention_expiry_garbage_collection_fixture_count == 8
    and .allowed_retention_expiry_garbage_collection_fixture_count == 0
    and .accepted_retention_expiry_garbage_collection_fixture_count == 0
    and .retention_expiry_garbage_collection_performed_count == 0
    and .retention_recorded_count == 0
    and .retention_policy_persisted_count == 0
    and .ttl_scheduled_count == 0
    and .expiry_applied_count == 0
    and .garbage_collection_scan_performed_count == 0
    and .garbage_collection_candidate_recorded_count == 0
    and .delete_marker_recorded_count == 0
    and .archive_recorded_count == 0
    and .compaction_performed_count == 0
    and .retention_export_recorded_count == 0
    and .external_retention_notification_sent_count == 0
    and (.retention_expiry_garbage_collection_fixtures | length) == 8
    and (.retention_expiry_garbage_collection_fixtures | all(
      (.retention_expiry_garbage_collection_status | startswith("blocked_"))
      and .receipt_noop_confirmed == true
    ))
    and .final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_readback_hash_matched == true
    and .final_authorization_dry_run_result_receipt_retention_recorded == false
    and .final_authorization_dry_run_result_receipt_retention_policy_persisted == false
    and .final_authorization_dry_run_result_receipt_ttl_scheduled == false
    and .final_authorization_dry_run_result_receipt_expiry_applied == false
    and .final_authorization_dry_run_result_receipt_garbage_collection_scan_performed == false
    and .final_authorization_dry_run_result_receipt_delete_marker_recorded == false
    and .final_authorization_dry_run_result_receipt_archive_recorded == false
    and .final_authorization_dry_run_result_receipt_compaction_performed == false
    and .final_authorization_dry_run_result_receipt_retention_export_recorded == false
    and .final_authorization_dry_run_result_receipt_external_retention_notification_sent == false
    and .result_receipt_retention_query_registered == false
    and .final_authorization_from_retention_expiry_garbage_collection_allowed == false
    and .operator_approval_from_retention_expiry_garbage_collection_accepted == false
    and .activation_from_retention_expiry_garbage_collection_allowed == false
    and .provider_invocation_authorized == false
    and .model_invocation_authorized == false
    and .provider_invocation_authorized_from_retention_expiry_garbage_collection == false
    and .model_invocation_authorized_from_retention_expiry_garbage_collection == false
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
    and .allowed_next_actions[0].action == "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial"
    and .allowed_next_actions[0].records_retention == false
    and .allowed_next_actions[0].records_expiry == false
    and .allowed_next_actions[0].records_garbage_collection == false
    and .allowed_next_actions[0].exports_receipt == false
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
    --arg gate "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_route_gate" \
    --arg endpoint "/api/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-retention-expiry-garbage-collection-denial" \
    --arg source_command "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-retention-expiry-garbage-collection-denial --json" \
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
      first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_ready:true,
      canary_execution_mode:"first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_no_provider_model_invocation",
      result_receipt_retention_expiry_garbage_collection_state:"final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denied",
      retention_expiry_garbage_collection_fixture_count:8,
      blocked_retention_expiry_garbage_collection_fixture_count:8,
      allowed_retention_expiry_garbage_collection_fixture_count:0,
      accepted_retention_expiry_garbage_collection_fixture_count:0,
      retention_expiry_garbage_collection_performed_count:0,
      retention_recorded_count:0,
      retention_policy_persisted_count:0,
      ttl_scheduled_count:0,
      expiry_applied_count:0,
      garbage_collection_scan_performed_count:0,
      delete_marker_recorded_count:0,
      archive_recorded_count:0,
      compaction_performed_count:0,
      external_retention_notification_sent_count:0,
      provider_invocation_authorized:false,
      model_invocation_authorized:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      live_kg_write_performed:false,
      memory_store_write_performed:false,
      channel_send_performed:false,
      external_send_performed:false,
      next_slice:"first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial"
    }'
)"

printf '%s\n' "$report"
echo "Hepta first model invocation operator approval final authorization dry-run result receipt retention/expiry/GC denial route gate passed"
