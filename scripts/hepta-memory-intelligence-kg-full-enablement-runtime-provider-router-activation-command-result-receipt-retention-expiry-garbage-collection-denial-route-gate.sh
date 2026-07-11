#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"
EXPECTED_ROUTE_COUNT="${HEPTA_EXPECTED_ROUTE_COUNT:-$(bash "$REPO_ROOT/scripts/lib/hepta-native-route-count.sh")}"

cd "$REPO_ROOT"

source scripts/lib/hepta-json-report-capture.sh

require_unsigned_integer() {
  local name="$1"
  local value="$2"

  case "$value" in
    ''|*[!0-9]*)
      echo "$name must be an unsigned integer" >&2
      exit 2
      ;;
  esac
}

require_source_text() {
  local source_file="$1"
  local source_text="$2"
  local label="$3"

  if ! rg -Fq "$source_text" "$source_file"; then
    echo "missing runtime provider-router activation command result receipt retention/expiry/garbage-collection route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report \
      "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-retention-expiry-garbage-collection-denial-gate" \
      scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-retention-expiry-garbage-collection-denial-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_gate"
  and .runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_status == "blocked"
  and .source_activation_command_result_receipt_audit_trail_immutable_evidence_ready == true
  and .source_activation_command_result_receipt_audit_trail_immutable_evidence_status == "blocked"
  and .runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_no_persistence_ready == true
  and .minimum_required_samples >= 24
  and .retention_expiry_garbage_collection_surface_count == 12
  and .retention_expiry_garbage_collection_surface_ready_count == 12
  and .retention_expiry_garbage_collection_side_effect_free_surface_count == 12
  and .retention_expiry_garbage_collection_fixture_count == 10
  and .blocked_retention_expiry_garbage_collection_fixture_count == 10
  and .noop_retention_expiry_garbage_collection_fixture_count == 10
  and .allowed_retention_expiry_garbage_collection_fixture_count == 0
  and .accepted_retention_expiry_garbage_collection_fixture_count == 0
  and .retention_denied_count == 10
  and .expiry_denied_count == 10
  and .garbage_collection_denied_count == 10
  and .retention_performed_count == 0
  and .expiry_performed_count == 0
  and .garbage_collection_performed_count == 0
  and .activation_command_result_receipt_retention_policy_allowed == false
  and .activation_command_result_receipt_retention_policy_recorded == false
  and .activation_command_result_receipt_retention_policy_persisted == false
  and .activation_command_result_receipt_expiry_allowed == false
  and .activation_command_result_receipt_expiry_scheduler_registered == false
  and .activation_command_result_receipt_garbage_collection_allowed == false
  and .activation_command_result_receipt_garbage_collection_scan_performed == false
  and .activation_command_result_receipt_delete_performed == false
  and .activation_command_result_receipt_tombstone_recorded == false
  and .activation_command_result_receipt_sweep_performed == false
  and .activation_command_result_receipt_archive_written == false
  and .activation_command_result_receipt_compaction_performed == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_allowed_by_result_receipt_retention == false
  and .activation_allowed_by_result_receipt_expiry == false
  and .activation_allowed_by_result_receipt_garbage_collection == false
  and .activation_allowed_by_result_receipt == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_activated == false
  and .runtime_router_mutated == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .secret_file_read == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .live_kg_write_performed == false
  and .rollback_executed == false
  and .external_send_performed == false
  and .install_executed == false
  and .service_restart_performed == false
  and .active_binary_mutated == false
  and (.retention_expiry_garbage_collection_surfaces | length) == 12
  and (.retention_expiry_garbage_collection_fixtures | length) == 10
  and (.retention_expiry_garbage_collection_fixtures | all(
    (.retention_gc_status == "blocked_noop" or .retention_gc_status == "blocked_expiry_noop" or .retention_gc_status == "blocked_gc_noop")
    and .retention_policy_allowed == false
    and .retention_policy_recorded == false
    and .retention_policy_persisted == false
    and .expiry_allowed == false
    and .expiry_scheduler_registered == false
    and .garbage_collection_allowed == false
    and .garbage_collection_scan_performed == false
    and .delete_performed == false
    and .tombstone_recorded == false
    and .sweep_performed == false
    and .archive_written == false
    and .compaction_performed == false
    and .activation_command_result_receipt_accepted == false
    and .activation_activated == false
    and .runtime_router_mutated == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .secret_file_read == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .external_send_performed == false
    and .install_executed == false
    and .service_restart_performed == false
    and .active_binary_mutated == false
    and .receipt_noop_confirmed == true
  ))
  and ([.retention_expiry_garbage_collection_fixtures[] | select(.source_audit_evidence_present == false)] | length) == 1
  and ([.retention_expiry_garbage_collection_fixtures[] | select(.retention_policy_request_shape == "record_blocked_noop_receipt_retention_policy")] | length) == 1
  and ([.retention_expiry_garbage_collection_fixtures[] | select(.retention_index_requested == true)] | length) == 1
  and ([.retention_expiry_garbage_collection_fixtures[] | select(.expiry_schedule_requested == true and .expiry_timer_requested == true)] | length) == 1
  and ([.retention_expiry_garbage_collection_fixtures[] | select(.ttl_update_requested == true and .ttl_extension_requested == true)] | length) == 1
  and ([.retention_expiry_garbage_collection_fixtures[] | select(.garbage_collection_scan_requested == true)] | length) == 1
  and ([.retention_expiry_garbage_collection_fixtures[] | select(.delete_requested == true and .tombstone_requested == true and .sweep_requested == true)] | length) == 1
  and ([.retention_expiry_garbage_collection_fixtures[] | select(.archive_requested == true and .compaction_requested == true)] | length) == 1
  and ([.retention_expiry_garbage_collection_fixtures[] | select(.activation_from_retention_gc_requested == true and .memory_store_gc_evidence_requested == true and .live_kg_gc_evidence_requested == true and .provider_prompt_gc_evidence_requested == true)] | length) == 1
  and ([.retention_expiry_garbage_collection_fixtures[] | select(.external_send_gc_evidence_requested == true and .install_gc_evidence_requested == true and .active_binary_gc_evidence_requested == true)] | length) == 1
  and (.denied_by_retention_expiry_garbage_collection | length) == 29
  and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial" and .status == "allowed_report_only_next_slice" and .exports_receipt == false and .registers_query == false and .records_observability == false and .mutates_runtime == false and .invokes_model == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$SOURCE_JSON"

source "$REPO_ROOT/scripts/lib/hepta-source-set.sh"
NATIVE_GATEWAY_SOURCE="hepta-native-gateway-source-set-v1"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native source command count"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT' \
  "runtime provider-router activation command result receipt retention/expiry/garbage-collection endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-retention-expiry-garbage-collection-denial' \
  "runtime provider-router activation command result receipt retention/expiry/garbage-collection endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-retention-expiry-garbage-collection-denial --json' \
  "runtime provider-router activation command result receipt retention/expiry/garbage-collection source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_report' \
  "runtime provider-router activation command result receipt retention/expiry/garbage-collection report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_route_enabled": true' \
  "runtime provider-router activation command result receipt retention/expiry/garbage-collection route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_endpoint_blocks_lifecycle' \
  "runtime provider-router activation command result receipt retention/expiry/garbage-collection focused test"

TEST_LOG="$(mktemp /tmp/hepta-runtime-provider-router-activation-command-result-receipt-retention-expiry-garbage-collection-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_endpoint_blocks_lifecycle \
  -- --nocapture >"$TEST_LOG"

LIVE_JSON='{}'
live_checked=false
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_JSON="$(
    curl -fsS \
      "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-retention-expiry-garbage-collection-denial"
  )"
  jq -e --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" '
    .runtime == "hepta"
    and .status == "ready"
    and .route_count == $expected_route_count
    and .implemented_route_count == $expected_route_count
    and .missing_route_count == 0
    and .native_gateway_source_command_count == $expected_route_count
    and .runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_route_enabled == true
    and .runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready == true
    and .runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_status == "blocked"
    and .source_activation_command_result_receipt_audit_trail_immutable_evidence_ready == true
    and .retention_expiry_garbage_collection_surface_count == 12
    and .retention_expiry_garbage_collection_fixture_count == 10
    and .blocked_retention_expiry_garbage_collection_fixture_count == 10
    and .accepted_retention_expiry_garbage_collection_fixture_count == 0
    and .retention_performed_count == 0
    and .expiry_performed_count == 0
    and .garbage_collection_performed_count == 0
    and .delete_performed_count == 0
    and .archive_written_count == 0
    and .compaction_performed_count == 0
    and .activation_command_result_receipt_retention_policy_recorded == false
    and .activation_command_result_receipt_expiry_scheduler_registered == false
    and .activation_command_result_receipt_garbage_collection_scan_performed == false
    and .activation_command_result_receipt_delete_performed == false
    and .activation_command_result_receipt_tombstone_recorded == false
    and .activation_allowed_by_result_receipt_retention == false
    and .activation_allowed_by_result_receipt_expiry == false
    and .activation_allowed_by_result_receipt_garbage_collection == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .secret_file_read == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .external_send_performed == false
    and .service_restart_performed == false
    and .active_binary_mutated == false
    and (.denied_by_retention_expiry_garbage_collection | length) == 29
    and (.side_effects | to_entries | all(.value == false))
  ' >/dev/null <<<"$LIVE_JSON"
  live_checked=true
fi

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg status "ready" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-retention-expiry-garbage-collection-denial" \
  --arg source_command "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-retention-expiry-garbage-collection-denial --json" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg focused_test_log "$TEST_LOG" \
  --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
  --argjson source "$SOURCE_JSON" \
  --argjson live "$LIVE_JSON" \
  --argjson live_checked "$live_checked" \
  '{
    product:$product,
    runtime:$runtime,
    status:$status,
    base_url:$base_url,
    gate:$gate,
    endpoint:$endpoint,
    source_command:$source_command,
    native_gateway_sha256:$native_gateway_sha256,
    focused_test_log:$focused_test_log,
    live_endpoint_checked:$live_checked,
    live_route_status:(if $live_checked then $live.status else "skipped" end),
    live_route_count:(if $live_checked then $live.route_count else 0 end),
    live_missing_route_count:(if $live_checked then $live.missing_route_count else 0 end),
    expected_route_count:$expected_route_count,
    source_retention_expiry_garbage_collection_gate:$source.gate,
    source_retention_expiry_garbage_collection_ready:$source.runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready,
    source_retention_expiry_garbage_collection_status:$source.runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_status,
    source_audit_trail_immutable_evidence_ready:$source.source_activation_command_result_receipt_audit_trail_immutable_evidence_ready,
    route_gate_ready:true,
    retention_expiry_garbage_collection_surface_count:$source.retention_expiry_garbage_collection_surface_count,
    retention_expiry_garbage_collection_fixture_count:$source.retention_expiry_garbage_collection_fixture_count,
    blocked_retention_expiry_garbage_collection_fixture_count:$source.blocked_retention_expiry_garbage_collection_fixture_count,
    accepted_retention_expiry_garbage_collection_fixture_count:$source.accepted_retention_expiry_garbage_collection_fixture_count,
    retention_performed_count:$source.retention_performed_count,
    expiry_performed_count:$source.expiry_performed_count,
    garbage_collection_performed_count:$source.garbage_collection_performed_count,
    denied_by_retention_expiry_garbage_collection_count:($source.denied_by_retention_expiry_garbage_collection | length),
    next_slice:"runtime_provider_router_activation_command_result_receipt_export_query_observability_denial",
    side_effects:{
      activation_command_result_receipt_retention_policy_recorded:$source.activation_command_result_receipt_retention_policy_recorded,
      activation_command_result_receipt_expiry_scheduler_registered:$source.activation_command_result_receipt_expiry_scheduler_registered,
      activation_command_result_receipt_garbage_collection_scan_performed:$source.activation_command_result_receipt_garbage_collection_scan_performed,
      activation_command_result_receipt_delete_performed:$source.activation_command_result_receipt_delete_performed,
      activation_command_result_receipt_archive_written:$source.activation_command_result_receipt_archive_written,
      activation_command_result_receipt_compaction_performed:$source.activation_command_result_receipt_compaction_performed,
      activation_allowed_by_result_receipt_retention:$source.activation_allowed_by_result_receipt_retention,
      activation_allowed_by_result_receipt_expiry:$source.activation_allowed_by_result_receipt_expiry,
      activation_allowed_by_result_receipt_garbage_collection:$source.activation_allowed_by_result_receipt_garbage_collection,
      provider_invoked:$source.provider_invoked,
      model_invoked:$source.model_invoked,
      credential_read:$source.credential_read,
      secret_file_read:$source.secret_file_read,
      memory_store_write_performed:$source.memory_store_write_performed,
      live_kg_write_performed:$source.live_kg_write_performed,
      external_send_performed:$source.external_send_performed,
      service_restart_performed:$source.service_restart_performed,
      active_binary_mutated:$source.active_binary_mutated
    }
  }'

echo "Hepta memory/intelligence/KG full enablement runtime provider-router activation command result receipt retention/expiry/garbage-collection denial route gate passed"
