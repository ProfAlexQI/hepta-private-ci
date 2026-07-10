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

require_source_text() {
  local source_file="$1"
  local source_text="$2"
  local label="$3"

  if ! rg -Fq "$source_text" "$source_file"; then
    echo "missing memory write execution activation command result receipt replay/idempotency boundary source text: $label" >&2
    exit 1
  fi
}

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"
ENDPOINT="/api/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-replay-idempotency-denial-boundary"
SOURCE_COMMAND="/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-replay-idempotency-denial-boundary --json"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes memory write execution activation command result receipt replay/idempotency denial boundary"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_BOUNDARY_ENDPOINT" \
  "memory write execution activation command result receipt replay/idempotency denial boundary endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" "$ENDPOINT" \
  "memory write execution activation command result receipt replay/idempotency denial boundary endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" "$SOURCE_COMMAND" \
  "memory write execution activation command result receipt replay/idempotency denial boundary source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_report" \
  "memory write execution activation command result receipt replay/idempotency denial boundary report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_ready\"" \
  "memory write execution activation command result receipt replay/idempotency denial boundary ready field"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"denied_by_activation_command_result_receipt_replay_idempotency_count\"" \
  "memory write execution activation command result receipt replay/idempotency denied count"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_endpoint_blocks_replay_and_idempotency" \
  "focused memory write execution activation command result receipt replay/idempotency boundary unit test"

TEST_LOG="$(mktemp /tmp/hepta-memory-write-execution-activation-command-result-receipt-replay-idempotency-boundary-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_endpoint_blocks_replay_and_idempotency \
  -- --nocapture >"$TEST_LOG"

LIVE_ROUTE_JSON='{}'
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(curl -fsS "$BASE_URL$ENDPOINT")"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .implemented_route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_ready == true
    and .activation_command_result_receipt_replay_idempotency_mode == "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial"
    and .source_activation_command_result_receipt_no_persistence_boundary_ready == true
    and .source_activation_command_result_receipt_no_persistence_ready == true
    and .source_activation_command_result_receipt_no_persistence_boundary_report_sha256 != ""
    and .source_activation_command_noop_handoff_boundary_report_sha256 != ""
    and .source_memory_write_execution_activation_closure_denial_boundary_report_sha256 != ""
    and .source_memory_write_execution_post_write_operator_acceptance_denial_boundary_report_sha256 != ""
    and .source_memory_write_execution_post_write_validation_dry_run_boundary_report_sha256 != ""
    and .source_memory_write_execution_write_enable_fixture_boundary_report_sha256 != ""
    and .source_memory_write_execution_no_write_sink_contract_boundary_report_sha256 != ""
    and .source_memory_write_execution_denial_matrix_boundary_report_sha256 != ""
    and .minimum_required_samples >= 24
    and .memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready == true
    and .memory_write_execution_activation_command_result_receipt_no_persistence_ready == true
    and .memory_write_execution_activation_command_noop_handoff_ready == true
    and .memory_write_execution_activation_closure_denial_ready == true
    and .memory_write_execution_post_write_operator_acceptance_denial_ready == true
    and .memory_write_execution_post_write_validation_dry_run_ready == true
    and .memory_write_execution_write_enable_fixture_ready == true
    and .memory_write_execution_no_write_sink_contract_ready == true
    and .required_activation_command_result_receipt_replay_idempotency_surface_count == 12
    and .ready_activation_command_result_receipt_replay_idempotency_surface_count == 12
    and .side_effect_free_activation_command_result_receipt_replay_idempotency_surface_count == 12
    and .required_activation_command_result_receipt_replay_idempotency_fixture_count == 10
    and .activation_command_result_receipt_replay_idempotency_fixture_count == 10
    and .blocked_activation_command_result_receipt_replay_idempotency_fixture_count == 10
    and .noop_activation_command_result_receipt_replay_idempotency_fixture_count == 10
    and .allowed_activation_command_result_receipt_replay_idempotency_fixture_count == 0
    and .accepted_activation_command_result_receipt_replay_idempotency_fixture_count == 0
    and .duplicate_activation_command_result_receipt_fixture_count == 2
    and .cross_scope_activation_command_result_receipt_fixture_count == 1
    and .status_upgrade_activation_command_result_receipt_fixture_count == 1
    and .activation_command_result_receipt_replay_denied_count == 10
    and .activation_command_result_receipt_duplicate_denied_count == 10
    and .activation_command_result_receipt_idempotency_denied_count == 10
    and .activation_command_result_receipt_replay_performed_count == 0
    and .activation_command_result_receipt_duplicate_accepted_count == 0
    and .activation_command_result_receipt_idempotency_state_recorded_count == 0
    and .activation_command_result_receipt_replay_allowed == false
    and .activation_command_result_receipt_replay_recorded == false
    and .activation_command_result_receipt_replay_persisted == false
    and .activation_command_result_receipt_duplicate_accepted == false
    and .activation_command_result_receipt_duplicate_recorded == false
    and .activation_command_result_receipt_duplicate_persisted == false
    and .activation_command_result_receipt_idempotency_key_accepted == false
    and .activation_command_result_receipt_idempotency_state_recorded == false
    and .activation_command_result_receipt_idempotency_state_persisted == false
    and .activation_command_result_receipt_replay_nonce_accepted == false
    and .activation_command_result_receipt_replay_nonce_recorded == false
    and .activation_command_result_receipt_cross_scope_reuse_accepted == false
    and .activation_command_result_receipt_status_upgrade_accepted == false
    and .activation_command_result_receipt_completed_status_accepted == false
    and .activation_command_result_receipt_ack_replay_accepted == false
    and .activation_command_result_receipt_ledger_replay_accepted == false
    and .activation_command_result_receipt_delivery_replay_accepted == false
    and .activation_command_result_receipt_write_replay_accepted == false
    and .activation_command_result_receipt_rollback_replay_accepted == false
    and .activation_command_result_receipt_secret_provider_replay_accepted == false
    and .activation_command_result_receipt_external_public_install_replay_accepted == false
    and .activation_command_result_receipt_recorded == false
    and .activation_command_result_receipt_persisted == false
    and .activation_command_result_receipt_accepted == false
    and .activation_command_result_receipt_materialized == false
    and .activation_command_result_receipt_filesystem_written == false
    and .activation_command_result_receipt_ledger_written == false
    and .activation_command_result_receipt_indexed == false
    and .activation_command_result_receipt_enqueued == false
    and .activation_command_result_receipt_delivered == false
    and .activation_command_completion_ack_recorded == false
    and .activation_command_completion_ack_persisted == false
    and .activation_command_completion_ack_accepted == false
    and .activation_command_completion_ack_delivered == false
    and .activation_command_enabled == false
    and .activation_command_invoked == false
    and .activation_command_dispatched == false
    and .activation_allowed_by_result_receipt_replay == false
    and .activation_allowed_by_result_receipt == false
    and .activation_allowed == false
    and .activation_performed == false
    and .live_mutation_execution_ready == false
    and .live_mutation_execution_allowed == false
    and .live_mutation_execution_performed == false
    and .memory_write_execution_allowed == false
    and .memory_write_execution_ready == false
    and .memory_write_execution_performed == false
    and .memory_store_write_path_enabled == false
    and .memory_store_write_allowed == false
    and .memory_store_write_performed == false
    and .memory_store_write_performed_count == 0
    and .memory_store_mutation_allowed == false
    and .memory_store_mutated == false
    and .rollback_execution_allowed == false
    and .rollback_executed == false
    and .secret_material_read == false
    and .provider_prompt_replay_enabled == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .secret_file_read == false
    and .kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .channel_send_performed == false
    and .telegram_send_performed == false
    and .external_send_enabled == false
    and .external_send_performed == false
    and .public_claim_or_release_artifact_write_enabled == false
    and .public_release_published == false
    and .public_ga_claimed == false
    and .public_release_claimed == false
    and .release_artifact_written == false
    and .public_artifact_written == false
    and .install_executed == false
    and .launchd_mutated == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and (.activation_command_result_receipt_replay_idempotency_surfaces | length) == 12
    and (.activation_command_result_receipt_replay_idempotency_fixtures | length) == 10
    and (.activation_command_result_receipt_replay_idempotency_fixtures | all((.replay_status == "blocked_noop" or .replay_status == "blocked_duplicate_noop") and .replay_requested == true and .replay_allowed == false and .replay_recorded == false and .replay_persisted == false and .duplicate_accepted == false and .idempotency_key_accepted == false and .idempotency_state_recorded == false and .idempotency_state_persisted == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .completion_ack_recorded == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false and .receipt_noop_confirmed == true))
    and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.duplicate_receipt_id_requested == true)] | length) == 1
    and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.stale_idempotency_key_requested == true)] | length) == 1
    and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.cross_scope_reuse_requested == true)] | length) == 1
    and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.receipt_status_requested == "completed")] | length) == 1
    and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.completion_ack_replay_requested == true)] | length) == 1
    and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.ledger_replay_requested == true and .index_replay_requested == true and .delivery_replay_requested == true)] | length) == 1
    and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.memory_write_replay_requested == true and .live_mutation_replay_requested == true)] | length) == 1
    and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.rollback_replay_requested == true and .secret_material_replay_requested == true and .provider_prompt_replay_requested == true)] | length) == 1
    and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.external_send_replay_requested == true and .install_replay_requested == true and .active_binary_mutation_replay_requested == true)] | length) == 1
    and .denied_by_activation_command_result_receipt_replay_idempotency_count == 24
    and (.denied_by_activation_command_result_receipt_replay_idempotency | length) == 24
    and .allowed_next_actions[0].action == "run_memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_require_live_gate"
    and .allowed_next_actions[1].action == "prepare_memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary"
    and (.side_effects | to_entries | all(.value == false))
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

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"
terminal_required_marker_count="$(jq -r '.required_marker_count // 0' <<<"$TERMINAL_COVERAGE_JSON")"
terminal_present_required_marker_count="$(jq -r '.present_required_marker_count // 0' <<<"$TERMINAL_COVERAGE_JSON")"
terminal_missing_required_marker_count="$(jq -r '.missing_required_marker_count // 0' <<<"$TERMINAL_COVERAGE_JSON")"
live_route_status="$(jq -r '.status // "skipped"' <<<"$LIVE_ROUTE_JSON")"
live_route_count="$(jq -r '.route_count // 0' <<<"$LIVE_ROUTE_JSON")"
live_missing_route_count="$(jq -r '.missing_route_count // 0' <<<"$LIVE_ROUTE_JSON")"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg endpoint "$ENDPOINT" \
  --arg source_command "$SOURCE_COMMAND" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --arg test_log "$TEST_LOG" \
  --arg live_route_status "$live_route_status" \
  --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
  --argjson live_endpoint_checked "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && echo true || echo false)" \
  --argjson live_route_count "$live_route_count" \
  --argjson live_missing_route_count "$live_missing_route_count" \
  --argjson terminal_required_marker_count "$terminal_required_marker_count" \
  --argjson terminal_present_required_marker_count "$terminal_present_required_marker_count" \
  --argjson terminal_missing_required_marker_count "$terminal_missing_required_marker_count" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:"hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_route_gate",
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
    memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_ready:true,
    activation_command_result_receipt_replay_idempotency_fixture_count:10,
    blocked_activation_command_result_receipt_replay_idempotency_fixture_count:10,
    noop_activation_command_result_receipt_replay_idempotency_fixture_count:10,
    allowed_activation_command_result_receipt_replay_idempotency_fixture_count:0,
    accepted_activation_command_result_receipt_replay_idempotency_fixture_count:0,
    denied_by_activation_command_result_receipt_replay_idempotency_count:24,
    activation_command_result_receipt_replay_performed_count:0,
    memory_store_write_performed_count:0,
    side_effects_all_false:true,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    terminal_required_marker_count:$terminal_required_marker_count,
    terminal_present_required_marker_count:$terminal_present_required_marker_count,
    terminal_missing_required_marker_count:$terminal_missing_required_marker_count
  }'
