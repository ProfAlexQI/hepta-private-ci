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
    echo "missing minimal scoped Memory real-write canary rollback receipt acceptance source text: $label" >&2
    exit 1
  fi
}

source "$REPO_ROOT/scripts/lib/hepta-source-set.sh"
NATIVE_GATEWAY_SOURCE="hepta-native-gateway-source-set-v1"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"
ENDPOINT="/api/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-rollback-receipt-acceptance-boundary"
SOURCE_COMMAND="/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-rollback-receipt-acceptance-boundary --json"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes minimal scoped Memory canary rollback receipt acceptance boundary"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  "HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_ROLLBACK_RECEIPT_ACCEPTANCE_BOUNDARY_ENDPOINT" \
  "minimal scoped Memory canary rollback receipt acceptance endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$ENDPOINT" \
  "minimal scoped Memory canary rollback receipt acceptance endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$SOURCE_COMMAND" \
  "minimal scoped Memory canary rollback receipt acceptance source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_report" \
  "minimal scoped Memory canary rollback receipt acceptance report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_ready\"" \
  "minimal scoped Memory canary rollback receipt acceptance ready field"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"rollback_receipt_acceptance_result_accepted_count\"" \
  "minimal scoped Memory canary rollback receipt acceptance result count"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_write_execution_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_accepts_receipt_without_rollback_or_external_side_effects" \
  "focused minimal scoped Memory canary rollback receipt acceptance unit test"

TEST_LOG="$(mktemp /tmp/hepta-minimal-scoped-memory-real-write-canary-rollback-receipt-acceptance-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_memory_write_execution_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_accepts_receipt_without_rollback_or_external_side_effects \
  -- --nocapture >"$TEST_LOG"

SCRIPT_GATE_JSON="$(
  capture_json_report \
    "hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-rollback-receipt-acceptance-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-rollback-receipt-acceptance-boundary-gate.sh
)"

jq -e '
  .status == "ready"
  and .minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_ready == true
  and .minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_performed == true
  and .source_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary_ready == true
  and .source_accepted_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixture_count == 1
  and .source_blocked_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixture_count == 9
  and .source_receipt_readback_performed_count == 1
  and .source_receipt_readback_result_accepted_count == 1
  and .source_receipt_acceptance_recorded_count == 1
  and .source_rollback_receipt_acceptance_handoff_bound_count == 1
  and .source_wal_write_performed_count == 0
  and .source_receipt_persisted_count == 0
  and .source_durable_memory_store_write_performed_count == 0
  and .source_memory_store_write_performed_count == 0
  and .approved_namespace == "hepta.memory.canary"
  and .approved_store == "wal-receipt-canary-artifact"
  and .approved_scope == "session"
  and (.source_receipt_acceptance_hash_sha256 | type == "string" and length > 0)
  and (.source_receipt_readback_report_sha256 | type == "string" and length > 0)
  and (.source_receipt_hash_chain_sha256 | type == "string" and length > 0)
  and .rollback_receipt_id == "hepta-minimal-scoped-memory-real-write-canary-rollback-receipt-v1"
  and (.rollback_receipt_sha256 | type == "string" and length > 0)
  and (.rollback_receipt_hash_chain_sha256 | type == "string" and length > 0)
  and (.rollback_receipt_acceptance_hash_sha256 | type == "string" and length > 0)
  and .rollback_receipt_digest_match == true
  and .rollback_receipt_hash_chain_verified == true
  and .required_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_surface_count == 12
  and .ready_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_surface_count == 12
  and .minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixture_count == 10
  and .accepted_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixture_count == 1
  and .blocked_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixture_count == 9
  and .minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_accepted_count == 1
  and .rollback_receipt_acceptance_authority_accepted_count == 1
  and .source_durable_readback_receipt_acceptance_bound_count == 1
  and .receipt_acceptance_hash_bound_count == 1
  and .rollback_receipt_identity_bound_count == 1
  and .rollback_receipt_digest_bound_count == 1
  and .rollback_receipt_hash_chain_bound_count == 1
  and .rollback_receipt_source_readback_linkage_bound_count == 1
  and .rollback_receipt_acceptance_record_bound_count == 1
  and .rollback_receipt_replay_guard_accepted_count == 1
  and .tombstone_cleanup_handoff_bound_count == 1
  and .rollback_receipt_acceptance_result_recorded_count == 1
  and .rollback_receipt_acceptance_result_accepted_count == 1
  and .single_use_nonce_consumed_count == 0
  and .explicit_command_dispatched_count == 0
  and .wal_write_performed_count == 0
  and .wal_recorded_count == 0
  and .wal_persisted_count == 0
  and .receipt_recorded_count == 0
  and .receipt_persisted_count == 0
  and .receipt_materialized_count == 0
  and .canary_artifact_filesystem_written_count == 0
  and .artifact_readback_performed_count == 0
  and .artifact_cleanup_performed_count == 0
  and .post_write_readback_performed_count == 0
  and .readback_result_accepted_count == 0
  and .rollback_performed_count == 0
  and .rollback_result_accepted_count == 0
  and .tombstone_written_count == 0
  and .compensating_memory_write_performed_count == 0
  and .durable_memory_store_read_performed_count == 0
  and .durable_memory_store_write_performed_count == 0
  and .durable_memory_store_rollback_performed_count == 0
  and .memory_store_write_performed_count == 0
  and .kg_live_write_performed_count == 0
  and .provider_invoked_count == 0
  and .model_invoked_count == 0
  and .credential_read_count == 0
  and .channel_send_performed_count == 0
  and .external_send_performed_count == 0
  and .release_artifact_written_count == 0
  and .install_executed_count == 0
  and .service_restarted_count == 0
  and .active_binary_mutated_count == 0
  and (.minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixtures | length) == 10
  and ([.minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixtures[] | select(.minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_accepted == true)] | length) == 1
  and .denied_by_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_count == 28
  and (.denied_by_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary | length) == 28
  and .rollback_receipt_acceptance_performed == true
  and .rollback_receipt_acceptance_result_recorded == true
  and .rollback_receipt_acceptance_result_accepted == true
  and .rollback_receipt_identity_accepted == true
  and .rollback_receipt_digest_accepted == true
  and .rollback_receipt_hash_chain_accepted == true
  and .minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_accepted == true
  and .single_use_nonce_consumed == false
  and .explicit_command_dispatched == false
  and .wal_write_performed == false
  and .receipt_persisted == false
  and .rollback_executed == false
  and .rollback_performed == false
  and .tombstone_written == false
  and .compensating_memory_write_performed == false
  and .durable_memory_store_read_performed == false
  and .durable_memory_store_write_performed == false
  and .durable_memory_store_rollback_performed == false
  and .memory_store_write_performed == false
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
  and .allowed_next_actions[0].action == "run_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_require_live_gate"
  and .allowed_next_actions[0].accepts_rollback_receipt == true
  and .allowed_next_actions[0].executes_rollback == false
  and .allowed_next_actions[0].writes_tombstone == false
  and .allowed_next_actions[1].action == "prepare_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary"
  and .allowed_next_actions[1].requires_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance == true
  and .side_effects.rollback_receipt_acceptance_performed == true
  and .side_effects.rollback_receipt_acceptance_result_accepted == true
  and .side_effects.rollback_executed == false
  and .side_effects.tombstone_written == false
  and .side_effects.durable_memory_store_write_performed == false
  and .side_effects.external_send_performed == false
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
    and .memory_write_execution_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_ready == true
    and .minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_ready == true
    and .minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_performed == true
    and .source_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary_ready == true
    and .source_accepted_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixture_count == 1
    and .source_blocked_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixture_count == 9
    and .source_receipt_readback_result_accepted_count == 1
    and .source_rollback_receipt_acceptance_handoff_bound_count == 1
    and .approved_namespace == "hepta.memory.canary"
    and .approved_store == "wal-receipt-canary-artifact"
    and .approved_scope == "session"
    and .rollback_receipt_id == "hepta-minimal-scoped-memory-real-write-canary-rollback-receipt-v1"
    and .rollback_receipt_digest_match == true
    and .rollback_receipt_hash_chain_verified == true
    and .rollback_receipt_acceptance_result_accepted_count == 1
    and .rollback_receipt_acceptance_result_accepted == true
    and .wal_write_performed_count == 0
    and .receipt_persisted_count == 0
    and .rollback_performed_count == 0
    and .tombstone_written_count == 0
    and .compensating_memory_write_performed_count == 0
    and .durable_memory_store_read_performed_count == 0
    and .durable_memory_store_write_performed_count == 0
    and .durable_memory_store_rollback_performed_count == 0
    and .memory_store_write_performed_count == 0
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
    and .rollback_receipt_acceptance_performed == true
    and .rollback_receipt_acceptance_result_accepted == true
    and .rollback_executed == false
    and .rollback_performed == false
    and .tombstone_written == false
    and .compensating_memory_write_performed == false
    and .durable_memory_store_write_performed == false
    and .side_effects.rollback_receipt_acceptance_performed == true
    and .side_effects.rollback_receipt_acceptance_result_accepted == true
    and .side_effects.rollback_executed == false
    and .side_effects.tombstone_written == false
    and .side_effects.durable_memory_store_write_performed == false
    and .side_effects.external_send_performed == false
  ' >/dev/null <<<"$LIVE_ROUTE_JSON"
fi

TERMINAL_COVERAGE_JSON="$(
  capture_json_report \
    "hepta-preflight-terminal-coverage-inventory-gate" \
    scripts/hepta-preflight-terminal-coverage-inventory-gate.sh
)"

jq -e '
  .status == "ready"
  and .preflight_terminal_coverage_inventory_ready == true
  and .present_required_marker_count == .required_marker_count
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

test_log_sha256="$(sha256_file "$TEST_LOG")"
script_gate_sha256="$(sha256_text "$SCRIPT_GATE_JSON")"
live_route_sha256="$(sha256_text "$LIVE_ROUTE_JSON")"
terminal_coverage_sha256="$(sha256_text "$TERMINAL_COVERAGE_JSON")"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_route_gate" \
  --arg endpoint "$ENDPOINT" \
  --arg source_command "$SOURCE_COMMAND" \
  --arg test_log_sha256 "$test_log_sha256" \
  --arg script_gate_sha256 "$script_gate_sha256" \
  --arg live_route_sha256 "$live_route_sha256" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
  --argjson require_live_endpoint "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && printf true || printf false)" \
  --argjson script_gate "$SCRIPT_GATE_JSON" \
  --argjson live_route "$LIVE_ROUTE_JSON" \
  --argjson terminal_coverage "$TERMINAL_COVERAGE_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    gate:$gate,
    endpoint:$endpoint,
    source_command:$source_command,
    route_count:$expected_route_count,
    implemented_route_count:$expected_route_count,
    missing_route_count:0,
    route_count_source_command_accepted:true,
    require_live_endpoint:$require_live_endpoint,
    focused_unit_test_passed:true,
    test_log_sha256:$test_log_sha256,
    script_gate_sha256:$script_gate_sha256,
    live_route_sha256:$live_route_sha256,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    source_gate_ready:($script_gate.status == "ready"),
    live_route_ready:(if $require_live_endpoint then ($live_route.status == "ready") else true end),
    terminal_required_marker_count:($terminal_coverage.required_marker_count // 0),
    terminal_present_required_marker_count:($terminal_coverage.present_required_marker_count // 0),
    terminal_missing_required_marker_count:($terminal_coverage.missing_required_marker_count // 0),
    minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_route_gate_ready:true,
    rollback_receipt_acceptance_performed:true,
    rollback_receipt_acceptance_result_accepted:true,
    executes_rollback:false,
    writes_tombstone:false,
    wal_write_performed:false,
    receipt_persisted:false,
    durable_memory_store_write_performed:false,
    live_kg_write_performed:false,
    provider_invoked:false,
    model_invoked:false,
    credential_read:false,
    channel_send_performed:false,
    external_send_performed:false,
    release_artifact_written:false,
    install_executed:false,
    active_binary_mutated:false
  }'
