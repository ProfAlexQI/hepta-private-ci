#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"
EXPECTED_ROUTE_COUNT="${HEPTA_EXPECTED_ROUTE_COUNT:-$(bash "$REPO_ROOT/scripts/lib/hepta-native-route-count.sh")}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

require_source_text() {
  local source_file="$1"
  local source_text="$2"
  local label="$3"

  if ! rg -Fq "$source_text" "$source_file"; then
    echo "missing scoped production durable Memory write dry-run result receipt ordering/monotonicity denial source text: $label" >&2
    exit 1
  fi
}

source "$REPO_ROOT/scripts/lib/hepta-source-set.sh"
NATIVE_GATEWAY_SOURCE="hepta-native-gateway-source-set-v1"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"
ENDPOINT="/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-ordering-monotonicity-denial-boundary"
SOURCE_COMMAND="/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-ordering-monotonicity-denial-boundary --json"
FOCUSED_TEST="hepta_memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_blocks_ordering_without_cursor_sequence_execution_or_production_side_effects"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes scoped production durable Memory write dry-run result receipt ordering/monotonicity denial boundary"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  "HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_BOUNDARY_ENDPOINT" \
  "scoped production durable Memory write dry-run result receipt ordering/monotonicity denial endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$ENDPOINT" \
  "scoped production durable Memory write dry-run result receipt ordering/monotonicity denial endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$SOURCE_COMMAND" \
  "scoped production durable Memory write dry-run result receipt ordering/monotonicity denial source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_report" \
  "scoped production durable Memory write dry-run result receipt ordering/monotonicity denial report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_ready\"" \
  "scoped production durable Memory write dry-run result receipt ordering/monotonicity denial ready field"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"dry_run_execution_result_receipt_ordering_cursor_persisted\"" \
  "scoped production durable Memory write dry-run result receipt ordering cursor false field"
require_source_text "$NATIVE_GATEWAY_SOURCE" "$FOCUSED_TEST" \
  "focused scoped production durable Memory write dry-run result receipt ordering/monotonicity denial unit test"

TEST_LOG="$(mktemp /tmp/hepta-scoped-production-durable-memory-write-dry-run-result-receipt-ordering-monotonicity-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  "$FOCUSED_TEST" \
  -- --nocapture >"$TEST_LOG"

SCRIPT_GATE_JSON="$(
  capture_json_report \
    "hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-ordering-monotonicity-denial-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-ordering-monotonicity-denial-boundary-gate.sh
)"

jq -e '
  .status == "ready"
  and .memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_ready == true
  and .scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_ready == true
  and .scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_accepted == true
  and .scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_mode == "dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_no_ordering_cursor_no_monotonic_sequence_no_execution_no_production_durable_memory_mutation"
  and .source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_ready == true
  and .source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_accepted_count == 1
  and .source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixture_count == 1
  and .source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixture_count == 9
  and .approved_production_namespace == "hepta.memory.production.scoped"
  and .approved_production_store == "hepta-memory-durable-store-production-preflight-only"
  and .approved_production_scope == "operator-approved-session"
  and .production_durable_memory_target_id == "hepta-scoped-production-durable-memory-write-target-v1"
  and (.dry_run_execution_result_receipt_ordering_monotonicity_denial_matrix_hash_sha256 | type == "string" and length > 0)
  and (.dry_run_execution_result_receipt_ordering_sequence_policy_hash_sha256 | type == "string" and length > 0)
  and (.dry_run_execution_result_receipt_late_receipt_denial_hash_sha256 | type == "string" and length > 0)
  and (.dry_run_execution_result_receipt_future_receipt_denial_hash_sha256 | type == "string" and length > 0)
  and (.dry_run_execution_result_receipt_rollback_sequence_denial_hash_sha256 | type == "string" and length > 0)
  and (.dry_run_execution_result_receipt_same_sequence_replacement_denial_hash_sha256 | type == "string" and length > 0)
  and (.dry_run_execution_result_receipt_latest_wins_promotion_denial_hash_sha256 | type == "string" and length > 0)
  and (.dry_run_execution_result_receipt_sequence_gap_denial_hash_sha256 | type == "string" and length > 0)
  and .required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_surface_count == 16
  and .ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_surface_count == 16
  and .scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count == 10
  and .accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count == 1
  and .blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count == 9
  and .denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_count == 55
  and ([.scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixtures[] | select(.scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_accepted == true)] | length) == 1
  and .dry_run_execution_result_receipt_ordering_monotonicity_denial_matrix_bound_count == 1
  and .dry_run_execution_result_receipt_late_receipt_denied_count == 1
  and .dry_run_execution_result_receipt_future_receipt_denied_count == 1
  and .dry_run_execution_result_receipt_rollback_sequence_denied_count == 1
  and .dry_run_execution_result_receipt_same_sequence_replacement_denied_count == 1
  and .dry_run_execution_result_receipt_latest_wins_promotion_denied_count == 1
  and .dry_run_execution_result_receipt_sequence_gap_denied_count == 1
  and .dry_run_execution_result_receipt_ordering_cursor_persisted_count == 0
  and .dry_run_execution_result_receipt_ordering_ledger_written_count == 0
  and .dry_run_execution_result_receipt_monotonic_sequence_recorded_count == 0
  and .dry_run_execution_result_receipt_late_receipt_accepted_count == 0
  and .dry_run_execution_result_receipt_future_receipt_accepted_count == 0
  and .dry_run_execution_result_receipt_rollback_sequence_accepted_count == 0
  and .dry_run_execution_result_receipt_same_sequence_replacement_accepted_count == 0
  and .dry_run_execution_result_receipt_latest_wins_promoted_count == 0
  and .dry_run_execution_result_receipt_sequence_gap_accepted_count == 0
  and .dry_run_execution_result_receipt_ordering_cursor_persisted == false
  and .dry_run_execution_result_receipt_ordering_ledger_written == false
  and .dry_run_execution_result_receipt_monotonic_sequence_recorded == false
  and .dry_run_execution_result_receipt_late_receipt_accepted == false
  and .dry_run_execution_result_receipt_future_receipt_accepted == false
  and .dry_run_execution_result_receipt_rollback_sequence_accepted == false
  and .dry_run_execution_result_receipt_same_sequence_replacement_accepted == false
  and .dry_run_execution_result_receipt_latest_wins_promoted == false
  and .dry_run_execution_result_receipt_sequence_gap_accepted == false
  and .dry_run_execution_result_receipt_persisted == false
  and .dry_run_execution_executed == false
  and .production_durable_memory_write_executed == false
  and .production_durable_memory_store_write_performed == false
  and .actual_production_durable_memory_write_performed == false
  and .durable_memory_store_write_performed == false
  and .durable_memory_store_read_performed == false
  and .durable_memory_store_rollback_performed == false
  and .memory_store_write_performed == false
  and .wal_write_performed == false
  and .receipt_persisted == false
  and .post_write_readback_performed == false
  and .rollback_executed == false
  and .rollback_performed == false
  and .tombstone_cleanup_executed == false
  and .live_kg_write_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .release_artifact_written == false
  and .install_executed == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and .side_effects.scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_performed == true
  and .side_effects.scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_result_accepted == true
  and .side_effects.dry_run_execution_result_receipt_ordering_cursor_persisted == false
  and .side_effects.dry_run_execution_result_receipt_monotonic_sequence_recorded == false
  and .side_effects.dry_run_execution_executed == false
  and .side_effects.production_durable_memory_store_write_performed == false
  and .side_effects.memory_store_write_performed == false
  and .side_effects.wal_write_performed == false
  and .side_effects.receipt_persisted == false
  and .side_effects.external_send_performed == false
  and .allowed_next_actions[0].action == "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_require_live_gate"
  and .allowed_next_actions[0].persists_ordering_cursor == false
  and .allowed_next_actions[0].records_monotonic_sequence == false
  and .allowed_next_actions[0].executes_dry_run == false
  and .allowed_next_actions[0].writes_production_durable_memory == false
  and .allowed_next_actions[1].action == "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary"
  and .allowed_next_actions[1].requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary == true
' >/dev/null <<<"$SCRIPT_GATE_JSON"

LIVE_ROUTE_JSON='{}'
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(curl -fsS "$BASE_URL$ENDPOINT")"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .implemented_route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_accepted == true
    and .source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_ready == true
    and .accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count == 1
    and .blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count == 9
    and .denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_count == 55
    and .dry_run_execution_result_receipt_ordering_cursor_persisted == false
    and .dry_run_execution_result_receipt_monotonic_sequence_recorded == false
    and .dry_run_execution_result_receipt_late_receipt_accepted == false
    and .dry_run_execution_result_receipt_future_receipt_accepted == false
    and .dry_run_execution_result_receipt_rollback_sequence_accepted == false
    and .dry_run_execution_result_receipt_same_sequence_replacement_accepted == false
    and .dry_run_execution_result_receipt_latest_wins_promoted == false
    and .dry_run_execution_result_receipt_sequence_gap_accepted == false
    and .dry_run_execution_result_receipt_persisted == false
    and .dry_run_execution_executed == false
    and .production_durable_memory_write_executed == false
    and .production_durable_memory_store_write_performed == false
    and .actual_production_durable_memory_write_performed == false
    and .durable_memory_store_write_performed == false
    and .durable_memory_store_read_performed == false
    and .durable_memory_store_rollback_performed == false
    and .memory_store_write_performed == false
    and .wal_write_performed == false
    and .receipt_persisted == false
    and .post_write_readback_performed == false
    and .rollback_executed == false
    and .rollback_performed == false
    and .tombstone_cleanup_executed == false
    and .raw_payload_plaintext_recorded == false
    and .raw_payload_plaintext_persisted == false
    and .live_kg_write_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .channel_send_performed == false
    and .external_send_performed == false
    and .release_artifact_written == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
  ' >/dev/null <<<"$LIVE_ROUTE_JSON"
fi

TERMINAL_COVERAGE_JSON="$(
  capture_json_report \
    "hepta-preflight-terminal-coverage-inventory-gate" \
    scripts/hepta-preflight-terminal-coverage-inventory-gate.sh
)"

jq -e '
  .status == "ready"
  and .missing_required_marker_count == 0
  and .present_required_marker_count == .required_marker_count
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
script_gate_sha256="$(sha256_text "$SCRIPT_GATE_JSON")"
terminal_coverage_sha256="$(sha256_text "$TERMINAL_COVERAGE_JSON")"
test_log_sha256="$(sha256_file "$TEST_LOG")"
live_endpoint_verified=false
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  live_endpoint_verified=true
fi

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg status "ready" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_route_gate" \
  --arg endpoint "$ENDPOINT" \
  --arg source_command "$SOURCE_COMMAND" \
  --arg focused_test "$FOCUSED_TEST" \
  --arg focused_test_log "$TEST_LOG" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg script_gate_sha256 "$script_gate_sha256" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --arg test_log_sha256 "$test_log_sha256" \
  --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
  --argjson live_endpoint_verified "$live_endpoint_verified" \
  --argjson script_gate "$SCRIPT_GATE_JSON" \
  --argjson live_route "$LIVE_ROUTE_JSON" \
  --argjson terminal_coverage "$TERMINAL_COVERAGE_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:$status,
    gate:$gate,
    endpoint:$endpoint,
    source_command:$source_command,
    expected_route_count:$expected_route_count,
    route_gate_ready:true,
    native_gateway_sha256:$native_gateway_sha256,
    focused_test:$focused_test,
    focused_test_log:$focused_test_log,
    focused_test_log_sha256:$test_log_sha256,
    script_gate_sha256:$script_gate_sha256,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    live_endpoint_required:($live_endpoint_verified == true),
    live_endpoint_verified:$live_endpoint_verified,
    live_route_count:($live_route.route_count // null),
    live_missing_route_count:($live_route.missing_route_count // null),
    script_gate_status:$script_gate.status,
    script_gate_denial_count:$script_gate.denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_count,
    script_gate_ordering_cursor_persisted:$script_gate.dry_run_execution_result_receipt_ordering_cursor_persisted,
    script_gate_monotonic_sequence_recorded:$script_gate.dry_run_execution_result_receipt_monotonic_sequence_recorded,
    script_gate_dry_run_executed:$script_gate.dry_run_execution_executed,
    script_gate_production_write_executed:$script_gate.production_durable_memory_write_executed,
    terminal_present_required_marker_count:$terminal_coverage.present_required_marker_count,
    terminal_required_marker_count:$terminal_coverage.required_marker_count,
    terminal_missing_required_marker_count:$terminal_coverage.missing_required_marker_count,
    side_effects:{
      route_gate_workspace_written:false,
      dry_run_execution_executed:false,
      production_durable_memory_write_executed:false,
      ordering_cursor_persisted:false,
      monotonic_sequence_recorded:false,
      idempotency_ledger_written:false,
      replay_state_persisted:false,
      memory_store_mutated:false,
      wal_write_performed:false,
      receipt_persisted:false,
      live_kg_write_performed:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      channel_send_performed:false,
      external_send_performed:false,
      release_artifact_written:false,
      install_executed:false,
      service_restarted:false,
      active_binary_mutated:false
    }
  }'

rm -f "$TEST_LOG"
