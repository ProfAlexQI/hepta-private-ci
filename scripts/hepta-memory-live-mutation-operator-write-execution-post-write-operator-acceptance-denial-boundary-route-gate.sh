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
    echo "missing memory write execution post-write operator acceptance denial boundary source text: $label" >&2
    exit 1
  fi
}

NATIVE_GATEWAY_SOURCE="codex-rs/hepta-native-gateway/src/native_gateway.rs"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"
ENDPOINT="/api/hepta-memory-live-mutation-operator-write-execution-post-write-operator-acceptance-denial-boundary"
SOURCE_COMMAND="/hepta-memory-live-mutation-operator-write-execution-post-write-operator-acceptance-denial-boundary --json"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes memory write execution post-write operator acceptance denial boundary"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  "HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_POST_WRITE_OPERATOR_ACCEPTANCE_DENIAL_BOUNDARY_ENDPOINT" \
  "memory write execution post-write operator acceptance denial boundary endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$ENDPOINT" \
  "memory write execution post-write operator acceptance denial boundary endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$SOURCE_COMMAND" \
  "memory write execution post-write operator acceptance denial boundary source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_live_mutation_operator_write_execution_post_write_operator_acceptance_denial_boundary_report" \
  "memory write execution post-write operator acceptance denial boundary report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"memory_write_execution_post_write_operator_acceptance_denial_boundary_ready\"" \
  "memory write execution post-write operator acceptance denial boundary ready field"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"denied_by_operator_acceptance_count\", 21" \
  "memory write execution post-write operator acceptance denied count"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_write_execution_post_write_operator_acceptance_denial_boundary_endpoint_blocks_operator_acceptance_without_activation" \
  "focused memory write execution post-write operator acceptance denial boundary unit test"

TEST_LOG="$(mktemp /tmp/hepta-memory-write-execution-post-write-operator-acceptance-denial-boundary-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_memory_write_execution_post_write_operator_acceptance_denial_boundary_endpoint_blocks_operator_acceptance_without_activation \
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
    and .memory_write_execution_post_write_operator_acceptance_denial_boundary_ready == true
    and .operator_acceptance_denial_mode == "memory_write_execution_post_write_operator_acceptance_denial_non_activation"
    and .source_memory_write_execution_post_write_validation_dry_run_boundary_ready == true
    and .source_memory_write_execution_post_write_validation_dry_run_ready == true
    and .source_memory_write_execution_post_write_validation_dry_run_boundary_report_sha256 != ""
    and .source_memory_write_execution_write_enable_fixture_boundary_report_sha256 != ""
    and .source_memory_write_execution_no_write_sink_contract_boundary_report_sha256 != ""
    and .source_memory_write_execution_denial_matrix_boundary_report_sha256 != ""
    and .minimum_required_samples >= 24
    and .memory_write_execution_post_write_operator_acceptance_denial_ready == true
    and .memory_write_execution_post_write_validation_dry_run_ready == true
    and .memory_write_execution_write_enable_fixture_ready == true
    and .memory_write_execution_no_write_sink_contract_ready == true
    and .required_post_write_validation_surface_count == 9
    and .ready_post_write_validation_surface_count == 9
    and .required_operator_acceptance_surface_count == 11
    and .ready_operator_acceptance_surface_count == 11
    and .side_effect_free_operator_acceptance_surface_count == 11
    and .required_operator_acceptance_fixture_count == 9
    and .operator_acceptance_fixture_count == 9
    and .blocked_operator_acceptance_fixture_count == 9
    and .allowed_operator_acceptance_fixture_count == 0
    and .accepted_operator_acceptance_fixture_count == 0
    and .operator_acceptance_denied_count == 9
    and .operator_acceptance_performed_count == 0
    and .operator_post_write_acceptance_recorded == false
    and .operator_post_write_acceptance_persisted == false
    and .operator_post_write_acceptance_accepted == false
    and .operator_post_write_acceptance_performed == false
    and .operator_post_write_acceptance_materialized == false
    and .operator_post_write_acceptance_filesystem_written == false
    and .accepted_post_write_validation_report_recorded == false
    and .accepted_post_write_validation_report_accepted == false
    and .accepted_post_write_validation_report_hash_bound == false
    and .write_result_receipt_hash_bound == false
    and .pre_write_memory_store_hash_bound == false
    and .post_write_memory_store_hash_bound == false
    and .post_write_diff_scope_accepted == false
    and .post_write_watchdog_soak_evidence_accepted == false
    and .post_write_route_regression_check_accepted == false
    and .post_write_dependency_isolation_check_accepted == false
    and .rollback_validation_accepted == false
    and .rollback_execution_allowed == false
    and .rollback_executed == false
    and .audit_redaction_validation_accepted == false
    and .secret_material_read == false
    and .activation_closure_packet_recorded == false
    and .activation_closure_packet_accepted == false
    and .activation_allowed_by_operator_acceptance == false
    and .activation_allowed == false
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
    and .service_restarted == false
    and .active_binary_mutated == false
    and (.operator_acceptance_surfaces | length) == 11
    and (.operator_acceptance_fixtures | length) == 9
    and (.operator_acceptance_fixtures | all(.operator_acceptance_requested == true and .acceptance_status == "blocked" and .acceptance_allowed == false and .acceptance_performed == false and .acceptance_accepted == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false))
    and ([.operator_acceptance_fixtures[] | select(.route_readiness_regression_detected == true)] | length) == 1
    and ([.operator_acceptance_fixtures[] | select(.direct_live_mutation_execution_requested == true)] | length) == 1
    and ([.operator_acceptance_fixtures[] | select(.raw_payload_plaintext_recorded == true and .secret_material_read == true)] | length) == 1
    and ([.operator_acceptance_fixtures[] | select(.external_send_requested == true and .release_artifact_write_requested == true)] | length) == 1
    and .denied_by_operator_acceptance_count == 21
    and (.denied_by_operator_acceptance | length) == 21
    and .allowed_next_actions[0].action == "run_memory_write_execution_post_write_operator_acceptance_denial_boundary_require_live_gate"
    and .allowed_next_actions[1].action == "prepare_memory_write_execution_activation_closure_denial_boundary"
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
    gate:"hepta_memory_live_mutation_operator_write_execution_post_write_operator_acceptance_denial_boundary_route_gate",
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
    memory_write_execution_post_write_operator_acceptance_denial_boundary_ready:true,
    operator_acceptance_fixture_count:9,
    blocked_operator_acceptance_fixture_count:9,
    allowed_operator_acceptance_fixture_count:0,
    accepted_operator_acceptance_fixture_count:0,
    operator_acceptance_performed_count:0,
    memory_write_execution_performed:false,
    memory_store_write_performed_count:0,
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
      memory_store_write_performed:false,
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
