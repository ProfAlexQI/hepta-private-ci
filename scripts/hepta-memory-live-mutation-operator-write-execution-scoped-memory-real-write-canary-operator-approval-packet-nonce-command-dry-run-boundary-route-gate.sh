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
    echo "missing scoped Memory real-write canary approval/nonce/command dry-run source text: $label" >&2
    exit 1
  fi
}

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"
ENDPOINT="/api/hepta-memory-live-mutation-operator-write-execution-scoped-memory-real-write-canary-operator-approval-packet-nonce-command-dry-run-boundary"
SOURCE_COMMAND="/hepta-memory-live-mutation-operator-write-execution-scoped-memory-real-write-canary-operator-approval-packet-nonce-command-dry-run-boundary --json"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes scoped Memory real-write canary dry-run boundary"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_MEMORY_REAL_WRITE_CANARY_OPERATOR_APPROVAL_PACKET_NONCE_COMMAND_DRY_RUN_BOUNDARY_ENDPOINT" \
  "scoped Memory real-write canary endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" "$ENDPOINT" \
  "scoped Memory real-write canary endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" "$SOURCE_COMMAND" \
  "scoped Memory real-write canary source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_live_mutation_operator_write_execution_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_boundary_report" \
  "scoped Memory real-write canary report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_ready\"" \
  "scoped Memory real-write canary ready field"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"denied_by_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_count\"" \
  "scoped Memory real-write canary denial count"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_write_execution_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_boundary_blocks_real_write" \
  "focused scoped Memory real-write canary unit test"

TEST_LOG="$(mktemp /tmp/hepta-scoped-memory-real-write-canary-approval-nonce-command-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_write_execution_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_boundary_blocks_real_write \
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
    and .memory_write_execution_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_boundary_ready == true
    and .scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_ready == true
    and .scoped_memory_real_write_canary_mode == "scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_no_write"
    and .source_activation_command_result_receipt_release_artifact_publication_boundary_ready == true
    and .source_activation_command_result_receipt_release_artifact_publication_ready == true
    and .source_activation_command_result_receipt_release_artifact_publication_report_sha256 != ""
    and .source_release_artifact_publication_fixture_count == 10
    and .source_accepted_release_artifact_publication_fixture_count == 0
    and .source_release_artifact_publication_performed_count == 0
    and .source_release_artifact_publication_denial_count == 69
    and .minimum_required_samples >= 24
    and .required_scoped_memory_real_write_canary_operator_approval_surface_count == 12
    and .ready_scoped_memory_real_write_canary_operator_approval_surface_count == 12
    and .side_effect_free_scoped_memory_real_write_canary_operator_approval_surface_count == 12
    and .required_scoped_memory_real_write_canary_operator_approval_fixture_count == 10
    and .scoped_memory_real_write_canary_operator_approval_fixture_count == 10
    and .blocked_scoped_memory_real_write_canary_operator_approval_fixture_count == 10
    and .noop_scoped_memory_real_write_canary_operator_approval_fixture_count == 10
    and .allowed_scoped_memory_real_write_canary_operator_approval_fixture_count == 0
    and .accepted_scoped_memory_real_write_canary_operator_approval_fixture_count == 0
    and .scoped_memory_real_write_canary_approval_packet_accepted_count == 0
    and .single_use_nonce_consumed_count == 0
    and .explicit_command_dispatched_count == 0
    and .wal_write_performed_count == 0
    and .receipt_persisted_count == 0
    and .post_write_readback_performed_count == 0
    and .rollback_tombstone_performed_count == 0
    and .memory_store_write_performed_count == 0
    and .required_before_scoped_memory_real_write_canary_acceptance_count == 18
    and (.required_scoped_memory_real_write_canary_operator_approval_fields | length) == 18
    and .fresh_operator_approval_packet_recorded == false
    and .fresh_operator_approval_packet_persisted == false
    and .fresh_operator_approval_packet_accepted == false
    and .operator_identity_bound == false
    and .operator_session_bound == false
    and .single_use_nonce_issued == false
    and .single_use_nonce_consumed == false
    and .explicit_command_accepted == false
    and .explicit_command_dispatched == false
    and .canary_scope_bound == false
    and .canary_namespace_bound == false
    and .canary_store_bound == false
    and .payload_digest_bound == false
    and .active_binary_sha_bound == false
    and .route_count_bound == false
    and .wal_receipt_plan_accepted == false
    and .receipt_persisted == false
    and .post_write_readback_performed == false
    and .rollback_tombstone_plan_accepted == false
    and .rollback_executed == false
    and .activation_allowed == false
    and .live_mutation_execution_performed == false
    and .memory_write_execution_performed == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .durable_memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .secret_file_read == false
    and .telegram_send_performed == false
    and .channel_send_performed == false
    and .external_send_performed == false
    and .public_claim_promoted == false
    and .public_release_published == false
    and .public_ga_claimed == false
    and .release_artifact_written == false
    and .public_artifact_written == false
    and .install_executed == false
    and .launchd_mutated == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and (.scoped_memory_real_write_canary_operator_approval_surfaces | length) == 12
    and (.scoped_memory_real_write_canary_operator_approval_fixtures | length) == 10
    and (.scoped_memory_real_write_canary_operator_approval_fixtures | all((.scoped_memory_real_write_canary_status | startswith("blocked")) and .approval_packet_accepted == false and .single_use_nonce_consumed == false and .explicit_command_dispatched == false and .receipt_persisted == false and .post_write_readback_performed == false and .rollback_executed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .durable_memory_store_write_performed == false and .live_kg_write_performed == false and .provider_invoked == false and .model_invoked == false and .credential_read == false and .channel_send_performed == false and .external_send_performed == false and .release_artifact_written == false and .install_executed == false and .active_binary_mutated == false and .scoped_canary_dry_run_noop_confirmed == true))
    and ([.scoped_memory_real_write_canary_operator_approval_fixtures[] | select(.source_release_artifact_publication_denial_present == false)] | length) == 1
    and ([.scoped_memory_real_write_canary_operator_approval_fixtures[] | select(.nonce_consume_requested == true)] | length) == 1
    and ([.scoped_memory_real_write_canary_operator_approval_fixtures[] | select(.explicit_command_requested == true)] | length) == 1
    and ([.scoped_memory_real_write_canary_operator_approval_fixtures[] | select(.durable_memory_write_requested == true)] | length) == 1
    and .denied_by_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_count == 32
    and (.denied_by_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run | length) == 32
    and .fresh_operator_approval_packet_required == true
    and .single_use_nonce_required == true
    and .explicit_command_required == true
    and .durable_memory_write_forbidden == true
    and .kg_live_write_forbidden == true
    and .provider_model_invocation_forbidden == true
    and .credential_read_forbidden == true
    and .channel_external_send_forbidden == true
    and .public_claim_release_artifact_forbidden == true
    and .install_restart_active_binary_mutation_forbidden == true
    and .allowed_next_actions[0].action == "run_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_boundary_require_live_gate"
    and .allowed_next_actions[0].accepts_operator_approval_packet == false
    and .allowed_next_actions[0].consumes_nonce == false
    and .allowed_next_actions[0].dispatches_command == false
    and .allowed_next_actions[1].action == "prepare_scoped_memory_real_write_canary_readback_rollback_tombstone_dry_run_boundary"
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
  --argjson live_route_count "$live_route_count" \
  --argjson live_missing_route_count "$live_missing_route_count" \
  --argjson terminal_required_marker_count "$terminal_required_marker_count" \
  --argjson terminal_present_required_marker_count "$terminal_present_required_marker_count" \
  --argjson terminal_missing_required_marker_count "$terminal_missing_required_marker_count" \
  '{
    product: $product,
    runtime: $runtime,
    status: "ready",
    gate: "hepta_memory_live_mutation_operator_write_execution_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_boundary_route_gate",
    endpoint: $endpoint,
    source_command: $source_command,
    base_url: $base_url,
    native_gateway_sha256: $native_gateway_sha256,
    expected_route_count: $expected_route_count,
    focused_test_passed: true,
    focused_test_log: $test_log,
    require_live_endpoint: (env.HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT // "0"),
    live_route_status: $live_route_status,
    live_route_count: $live_route_count,
    live_missing_route_count: $live_missing_route_count,
    terminal_coverage_sha256: $terminal_coverage_sha256,
    terminal_required_marker_count: $terminal_required_marker_count,
    terminal_present_required_marker_count: $terminal_present_required_marker_count,
    terminal_missing_required_marker_count: $terminal_missing_required_marker_count,
    route_gate_ready: true,
    scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_ready: true,
    fresh_operator_approval_packet_accepted: false,
    single_use_nonce_consumed: false,
    explicit_command_dispatched: false,
    memory_store_write_performed: false,
    memory_store_mutated: false,
    durable_memory_store_write_performed: false,
    live_kg_write_performed: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    channel_send_performed: false,
    external_send_performed: false,
    release_artifact_written: false,
    install_executed: false,
    active_binary_mutated: false,
    side_effects: {
      fresh_operator_approval_packet_accepted: false,
      single_use_nonce_consumed: false,
      explicit_command_dispatched: false,
      durable_memory_store_write_performed: false,
      memory_store_mutated: false,
      live_kg_write_performed: false,
      provider_invoked: false,
      model_invoked: false,
      credential_read: false,
      channel_send_performed: false,
      external_send_performed: false,
      release_artifact_written: false,
      install_executed: false,
      service_restarted: false,
      active_binary_mutated: false,
      filesystem_written: false
    }
  }'

echo "Hepta scoped Memory real-write canary operator approval packet nonce command dry-run boundary route gate passed"
