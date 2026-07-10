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
    echo "missing memory write execution write-enable fixture boundary source text: $label" >&2
    exit 1
  fi
}

NATIVE_GATEWAY_SOURCE="codex-rs/hepta-native-gateway/src/native_gateway.rs"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"
ENDPOINT="/api/hepta-memory-live-mutation-operator-write-execution-write-enable-fixture-boundary"
SOURCE_COMMAND="/hepta-memory-live-mutation-operator-write-execution-write-enable-fixture-boundary --json"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes memory write execution write-enable fixture boundary"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_WRITE_ENABLE_FIXTURE_BOUNDARY_ENDPOINT" \
  "memory write execution write-enable fixture boundary endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" "$ENDPOINT" \
  "memory write execution write-enable fixture boundary endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$SOURCE_COMMAND" \
  "memory write execution write-enable fixture boundary source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_live_mutation_operator_write_execution_write_enable_fixture_boundary_report" \
  "memory write execution write-enable fixture boundary report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"memory_write_execution_write_enable_fixture_boundary_ready\"" \
  "memory write execution write-enable fixture boundary ready field"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"denied_by_write_enable_fixture_count\", 13" \
  "memory write execution write-enable fixture denied count"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_write_execution_write_enable_fixture_boundary_endpoint_blocks_write_enablement_without_store_write" \
  "focused memory write execution write-enable fixture boundary unit test"

TEST_LOG="$(mktemp /tmp/hepta-memory-write-execution-write-enable-fixture-boundary-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_memory_write_execution_write_enable_fixture_boundary_endpoint_blocks_write_enablement_without_store_write \
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
    and .memory_write_execution_write_enable_fixture_boundary_ready == true
    and .write_enable_fixture_mode == "memory_write_execution_write_enable_fixture_non_activation"
    and .source_memory_write_execution_no_write_sink_contract_boundary_ready == true
    and .minimum_required_samples >= 24
    and .memory_write_execution_write_enable_fixture_ready == true
    and .memory_write_execution_no_write_sink_contract_ready == true
    and .memory_write_execution_denial_matrix_ready == true
    and .required_pre_execution_validation_check_count == 17
    and .accepted_pre_execution_validation_check_count == 0
    and .required_write_enable_surface_count == 10
    and .ready_write_enable_surface_count == 10
    and .side_effect_free_write_enable_surface_count == 10
    and .required_write_enable_fixture_count == 7
    and .write_enable_fixture_count == 7
    and .blocked_write_enable_fixture_count == 7
    and .allowed_write_enable_fixture_count == 0
    and .explicit_write_enable_requested_fixture_count == 7
    and .write_enable_denied_missing_approval_preflight_count == 1
    and .write_enable_denied_missing_operator_scope_count == 1
    and .write_enable_denied_allowlist_mismatch_count == 1
    and .write_enable_denied_payload_binding_count == 1
    and .write_enable_denied_stale_soak_rollback_validation_count == 1
    and .write_enable_denied_public_artifact_count == 1
    and .write_enable_denied_store_or_rollback_execution_count == 1
    and .memory_write_execution_denied_count == 7
    and .memory_write_execution_allowed_count == 0
    and .memory_write_execution_performed_count == 0
    and .memory_store_write_requested_fixture_count == 7
    and .memory_store_write_allowed_count == 0
    and .memory_store_write_performed_count == 0
    and .memory_store_mutation_allowed == false
    and .memory_store_mutated == false
    and .explicit_write_enablement_recorded == false
    and .explicit_write_enablement_persisted == false
    and .explicit_write_enablement_accepted == false
    and .write_enable_fixture_recorded == false
    and .write_enable_fixture_persisted == false
    and .write_enable_fixture_materialized == false
    and .write_enable_fixture_filesystem_written == false
    and .memory_write_approval_packet_accepted == false
    and .memory_write_request_accepted == false
    and .operator_approval_recorded == false
    and .accepted_redaction_proof_count == 0
    and .source_report_hash_bindings_recorded == false
    and .raw_payload_sha256_bound == false
    and .redacted_payload_summary_sha256_bound == false
    and .raw_payload_plaintext_recorded == false
    and .raw_payload_plaintext_persisted == false
    and .memory_write_execution_allowed == false
    and .memory_write_execution_ready == false
    and .memory_write_execution_performed == false
    and .memory_store_write_path_enabled == false
    and .no_write_sink_write_path_enabled_by_default == false
    and .live_mutation_execution_ready == false
    and .rollback_execution_allowed == false
    and .rollback_executed == false
    and .external_send_enabled == false
    and .external_send_performed == false
    and .public_claim_or_release_artifact_write_enabled == false
    and .public_release_published == false
    and .release_artifact_written == false
    and (.write_enable_surfaces | length) == 10
    and (.write_enable_fixtures | length) == 7
    and (.write_enable_fixtures | all(.explicit_write_enable_requested == true and .write_enable_status == "blocked" and .execution_allowed == false and .execution_performed == false and .memory_store_write_allowed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .activation_allowed == false))
    and ([.write_enable_fixtures[] | select(.raw_payload_plaintext_recorded == true)] | length) == 1
    and ([.write_enable_fixtures[] | select(.public_claim_requested == true and .release_artifact_write_requested == true)] | length) == 1
    and ([.write_enable_fixtures[] | select(.rollback_execution_requested == true)] | length) == 1
    and .denied_by_write_enable_fixture_count == 13
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .secret_file_read == false
    and .kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .channel_send_performed == false
    and .telegram_send_performed == false
    and .external_send_performed == false
    and .public_artifact_written == false
    and .public_release_claimed == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .allowed_next_actions[0].action == "run_memory_write_execution_write_enable_fixture_boundary_require_live_gate"
    and .allowed_next_actions[1].action == "prepare_memory_write_execution_post_write_validation_dry_run_boundary"
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
    gate:"hepta_memory_live_mutation_operator_write_execution_write_enable_fixture_boundary_route_gate",
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
    memory_write_execution_write_enable_fixture_boundary_ready:true,
    write_enable_fixture_recorded:false,
    write_enable_fixture_persisted:false,
    write_enable_fixture_materialized:false,
    write_enable_fixture_filesystem_written:false,
    write_enable_fixture_count:7,
    blocked_write_enable_fixture_count:7,
    allowed_write_enable_fixture_count:0,
    memory_write_execution_performed_count:0,
    memory_store_write_performed_count:0,
    durable_memory_store_write_performed:false,
    memory_store_mutated:false,
    rollback_executed:false,
    live_kg_write_performed:false,
    provider_invoked:false,
    model_invoked:false,
    credential_read:false,
    external_send_performed:false,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    terminal_required_marker_count:$terminal_required_marker_count,
    terminal_present_required_marker_count:$terminal_present_required_marker_count,
    terminal_missing_required_marker_count:$terminal_missing_required_marker_count,
    side_effects:{
      durable_memory_store_write_performed:false,
      memory_store_mutated:false,
      rollback_executed:false,
      live_kg_write_performed:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      secret_file_read:false,
      channel_send_performed:false,
      telegram_send_performed:false,
      external_send_performed:false,
      release_artifact_written:false,
      public_artifact_written:false,
      install_executed:false,
      service_restarted:false,
      active_binary_mutated:false,
      filesystem_written:false
    }
  }'
