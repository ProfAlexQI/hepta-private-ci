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
    echo "missing scoped production durable Memory write dry-run execution envelope source text: $label" >&2
    exit 1
  fi
}

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"
ENDPOINT="/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-envelope-boundary"
SOURCE_COMMAND="/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-envelope-boundary --json"
FOCUSED_TEST="hepta_memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_envelope_binds_envelope_without_execution_persistence_or_production_side_effects"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes scoped production durable Memory write dry-run execution envelope boundary"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_ENVELOPE_BOUNDARY_ENDPOINT" \
  "scoped production durable Memory write dry-run execution envelope endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" "$ENDPOINT" \
  "scoped production durable Memory write dry-run execution envelope endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" "$SOURCE_COMMAND" \
  "scoped production durable Memory write dry-run execution envelope source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_report" \
  "scoped production durable Memory write dry-run execution envelope report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"scoped_production_durable_memory_write_dry_run_execution_envelope_ready\"" \
  "scoped production durable Memory write dry-run execution envelope ready field"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"scoped_production_durable_memory_write_dry_run_execution_envelope_result_accepted_count\"" \
  "scoped production durable Memory write dry-run execution envelope result accepted count field"
require_source_text "$NATIVE_GATEWAY_SOURCE" "$FOCUSED_TEST" \
  "focused scoped production durable Memory write dry-run execution envelope unit test"

TEST_LOG="$(mktemp /tmp/hepta-scoped-production-durable-memory-write-dry-run-execution-envelope-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  "$FOCUSED_TEST" \
  -- --nocapture >"$TEST_LOG"

SCRIPT_GATE_JSON="$(
  capture_json_report \
    "hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-envelope-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-envelope-boundary-gate.sh
)"

jq -e '
  .status == "ready"
  and .memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_ready == true
  and .scoped_production_durable_memory_write_dry_run_execution_envelope_ready == true
  and .scoped_production_durable_memory_write_dry_run_execution_envelope_performed == true
  and .scoped_production_durable_memory_write_dry_run_execution_envelope_accepted == true
  and .scoped_production_durable_memory_write_dry_run_execution_envelope_mode == "dry_run_execution_envelope_boundary_no_execution_no_persistence_no_production_durable_memory_mutation"
  and .source_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_ready == true
  and .source_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_accepted_count == 1
  and .source_accepted_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_fixture_count == 1
  and .source_blocked_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_fixture_count == 9
  and .approved_production_namespace == "hepta.memory.production.scoped"
  and .approved_production_store == "hepta-memory-durable-store-production-preflight-only"
  and .approved_production_scope == "operator-approved-session"
  and .production_durable_memory_target_id == "hepta-scoped-production-durable-memory-write-target-v1"
  and (.source_acceptance_receipt_result_hash_sha256 | type == "string" and length > 0)
  and (.source_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_hash_sha256 | type == "string" and length > 0)
  and (.source_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_policy_hash_sha256 | type == "string" and length > 0)
  and (.dry_run_execution_envelope_hash_sha256 | type == "string" and length > 0)
  and (.dry_run_execution_target_snapshot_hash_sha256 | type == "string" and length > 0)
  and (.dry_run_execution_write_plan_hash_sha256 | type == "string" and length > 0)
  and (.dry_run_execution_wal_receipt_preview_hash_sha256 | type == "string" and length > 0)
  and (.dry_run_execution_readback_preview_hash_sha256 | type == "string" and length > 0)
  and (.dry_run_execution_rollback_tombstone_preview_hash_sha256 | type == "string" and length > 0)
  and (.dry_run_execution_replay_guard_hash_sha256 | type == "string" and length > 0)
  and (.dry_run_execution_result_hash_sha256 | type == "string" and length > 0)
  and (.scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_hash_sha256 | type == "string" and length > 0)
  and (.scoped_production_durable_memory_write_dry_run_execution_envelope_policy_hash_sha256 | type == "string" and length > 0)
  and .required_scoped_production_durable_memory_write_dry_run_execution_envelope_surface_count == 15
  and .ready_scoped_production_durable_memory_write_dry_run_execution_envelope_surface_count == 15
  and .scoped_production_durable_memory_write_dry_run_execution_envelope_fixture_count == 10
  and .accepted_scoped_production_durable_memory_write_dry_run_execution_envelope_fixture_count == 1
  and .blocked_scoped_production_durable_memory_write_dry_run_execution_envelope_fixture_count == 9
  and .denied_by_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_count == 44
  and ([.scoped_production_durable_memory_write_dry_run_execution_envelope_fixtures[] | select(.scoped_production_durable_memory_write_dry_run_execution_envelope_accepted == true)] | length) == 1
  and .scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_performed_count == 1
  and .scoped_production_durable_memory_write_dry_run_execution_envelope_result_recorded_count == 1
  and .scoped_production_durable_memory_write_dry_run_execution_envelope_result_accepted_count == 1
  and .dry_run_execution_envelope_bound_count == 1
  and .dry_run_execution_write_plan_bound_count == 1
  and .dry_run_execution_wal_receipt_preview_bound_count == 1
  and .dry_run_execution_readback_preview_bound_count == 1
  and .dry_run_execution_rollback_tombstone_preview_bound_count == 1
  and .dry_run_execution_replay_guard_bound_count == 1
  and .dry_run_execution_envelope_persisted_count == 0
  and .dry_run_execution_executed_count == 0
  and .dry_run_execution_result_persisted_count == 0
  and .acceptance_receipt_persisted_count == 0
  and .operator_packet_persisted_count == 0
  and .production_durable_memory_write_executed_count == 0
  and .production_durable_memory_store_write_performed_count == 0
  and .actual_production_durable_memory_write_performed_count == 0
  and .durable_memory_store_write_performed_count == 0
  and .durable_memory_store_read_performed_count == 0
  and .durable_memory_store_rollback_performed_count == 0
  and .memory_store_write_performed_count == 0
  and .wal_write_performed_count == 0
  and .receipt_persisted_count == 0
  and .post_write_readback_performed_count == 0
  and .rollback_performed_count == 0
  and .tombstone_cleanup_executed_count == 0
  and .live_kg_write_performed_count == 0
  and .provider_invoked_count == 0
  and .model_invoked_count == 0
  and .credential_read_count == 0
  and .channel_send_performed_count == 0
  and .external_send_performed_count == 0
  and .release_artifact_written_count == 0
  and .install_executed_count == 0
  and .service_restarted_count == 0
  and .active_binary_mutated_count == 0
  and .source_operator_packet_acceptance_receipt_boundary_bound == true
  and .dry_run_execution_envelope_bound == true
  and .dry_run_execution_write_plan_bound == true
  and .dry_run_execution_readback_preview_bound == true
  and .dry_run_execution_rollback_tombstone_preview_bound == true
  and .dry_run_execution_persistence_forbidden_on_report_route == true
  and .dry_run_execution_execution_forbidden_on_report_route == true
  and .production_write_execution_forbidden_on_dry_run_envelope_route == true
  and .production_durable_memory_write_forbidden == true
  and .memory_store_mutation_forbidden == true
  and .kg_live_write_forbidden == true
  and .provider_model_invocation_forbidden == true
  and .credential_channel_public_release_forbidden == true
  and .install_restart_active_binary_mutation_forbidden == true
  and .dry_run_execution_envelope_persisted == false
  and .dry_run_execution_executed == false
  and .dry_run_execution_result_persisted == false
  and .acceptance_receipt_persisted == false
  and .operator_packet_persisted == false
  and .operator_packet_acceptance_receipt_persisted == false
  and .production_durable_memory_write_executed == false
  and .production_durable_memory_store_write_performed == false
  and .actual_production_durable_memory_write_performed == false
  and .durable_memory_store_write_performed == false
  and .durable_memory_store_read_performed == false
  and .durable_memory_store_rollback_performed == false
  and .memory_write_execution_performed == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .wal_write_performed == false
  and .receipt_persisted == false
  and .post_write_readback_performed == false
  and .rollback_executed == false
  and .rollback_performed == false
  and .tombstone_write_performed == false
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
  and .side_effects.scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_performed == true
  and .side_effects.scoped_production_durable_memory_write_dry_run_execution_envelope_result_accepted == true
  and .side_effects.dry_run_execution_executed == false
  and .side_effects.production_durable_memory_store_write_performed == false
  and .side_effects.memory_store_write_performed == false
  and .side_effects.wal_write_performed == false
  and .side_effects.receipt_persisted == false
  and .side_effects.external_send_performed == false
  and .allowed_next_actions[0].action == "run_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_require_live_gate"
  and .allowed_next_actions[0].executes_dry_run == false
  and .allowed_next_actions[0].writes_production_durable_memory == false
  and .allowed_next_actions[1].action == "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary"
  and .allowed_next_actions[1].requires_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary == true
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
    and .scoped_production_durable_memory_write_dry_run_execution_envelope_accepted == true
    and .source_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_ready == true
    and .source_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_accepted_count == 1
    and .accepted_scoped_production_durable_memory_write_dry_run_execution_envelope_fixture_count == 1
    and .blocked_scoped_production_durable_memory_write_dry_run_execution_envelope_fixture_count == 9
    and .denied_by_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_count == 44
    and .scoped_production_durable_memory_write_dry_run_execution_envelope_result_accepted == true
    and .dry_run_execution_envelope_persisted == false
    and .dry_run_execution_executed == false
    and .dry_run_execution_result_persisted == false
    and .acceptance_receipt_persisted == false
    and .operator_packet_persisted == false
    and .operator_packet_acceptance_receipt_persisted == false
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
  --arg gate "hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_route_gate" \
  --arg endpoint "$ENDPOINT" \
  --arg source_command "$SOURCE_COMMAND" \
  --arg test_log_sha256 "$test_log_sha256" \
  --arg script_gate_sha256 "$script_gate_sha256" \
  --arg live_route_sha256 "$live_route_sha256" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --argjson require_live_endpoint "$REQUIRE_LIVE_ENDPOINT" \
  --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
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
    expected_route_count:$expected_route_count,
    require_live_endpoint:$require_live_endpoint,
    scoped_production_durable_memory_write_dry_run_execution_envelope_route_gate_ready:true,
    source_text_verified:true,
    focused_rust_test_verified:true,
    script_gate_verified:true,
    live_endpoint_verified:($require_live_endpoint == 1),
    terminal_coverage_verified:true,
    test_log_sha256:$test_log_sha256,
    script_gate_sha256:$script_gate_sha256,
    live_route_sha256:$live_route_sha256,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    dry_run_execution_envelope_accepted:$script_gate.scoped_production_durable_memory_write_dry_run_execution_envelope_accepted,
    source_operator_packet_acceptance_receipt_boundary_ready:$script_gate.source_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_ready,
    dry_run_execution_executed:$script_gate.dry_run_execution_executed,
    dry_run_execution_envelope_persisted:$script_gate.dry_run_execution_envelope_persisted,
    production_durable_write:$script_gate.production_durable_memory_store_write_performed,
    memory_store_write:$script_gate.memory_store_write_performed,
    wal_write:$script_gate.wal_write_performed,
    receipt_persisted:$script_gate.receipt_persisted,
    rollback_executed:$script_gate.rollback_executed,
    external_send:$script_gate.external_send_performed,
    script_gate_summary:{
      status:$script_gate.status,
      accepted:$script_gate.scoped_production_durable_memory_write_dry_run_execution_envelope_accepted,
      fixture_count:$script_gate.scoped_production_durable_memory_write_dry_run_execution_envelope_fixture_count,
      accepted_fixture_count:$script_gate.accepted_scoped_production_durable_memory_write_dry_run_execution_envelope_fixture_count,
      blocked_fixture_count:$script_gate.blocked_scoped_production_durable_memory_write_dry_run_execution_envelope_fixture_count,
      denied_count:$script_gate.denied_by_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_count
    },
    live_route_summary:if $require_live_endpoint == 1 then {
      status:$live_route.status,
      route_count:$live_route.route_count,
      implemented_route_count:$live_route.implemented_route_count,
      accepted:$live_route.scoped_production_durable_memory_write_dry_run_execution_envelope_accepted,
      dry_run_execution_executed:$live_route.dry_run_execution_executed,
      dry_run_execution_envelope_persisted:$live_route.dry_run_execution_envelope_persisted,
      production_durable_write:$live_route.production_durable_memory_store_write_performed
    } else null end,
    terminal_coverage_summary:{
      required_marker_count:$terminal_coverage.required_marker_count,
      present_required_marker_count:$terminal_coverage.present_required_marker_count,
      missing_required_marker_count:$terminal_coverage.missing_required_marker_count
    }
  }'
