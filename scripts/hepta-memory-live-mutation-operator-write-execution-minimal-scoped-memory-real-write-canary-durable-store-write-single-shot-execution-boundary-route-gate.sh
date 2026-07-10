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
    echo "missing minimal scoped Memory real-write canary durable store write single-shot execution source text: $label" >&2
    exit 1
  fi
}

NATIVE_GATEWAY_SOURCE="codex-rs/hepta-native-gateway/src/native_gateway.rs"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"
ENDPOINT="/api/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution-boundary"
SOURCE_COMMAND="/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution-boundary --json"
FOCUSED_TEST="hepta_memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_executes_canary_store_with_zero_residue_without_production_or_external_side_effects"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes minimal scoped Memory canary durable store write single-shot execution boundary"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_STORE_WRITE_SINGLE_SHOT_EXECUTION_BOUNDARY_ENDPOINT" \
  "minimal scoped Memory canary durable store write single-shot endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" "$ENDPOINT" \
  "minimal scoped Memory canary durable store write single-shot endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$SOURCE_COMMAND" \
  "minimal scoped Memory canary durable store write single-shot source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_report" \
  "minimal scoped Memory canary durable store write single-shot report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"production_durable_memory_store_write_performed\"" \
  "production durable Memory backend write remains separately reported and false"
require_source_text "$NATIVE_GATEWAY_SOURCE" "$FOCUSED_TEST" \
  "focused minimal scoped Memory canary durable store write single-shot unit test"

TEST_LOG="$(mktemp /tmp/hepta-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  "$FOCUSED_TEST" \
  -- --nocapture >"$TEST_LOG"

SCRIPT_GATE_JSON="$(
  capture_json_report \
    "hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution-boundary-gate.sh
)"

jq -e '
  .status == "ready"
  and .minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_ready == true
  and .minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_performed == true
  and .minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_accepted == true
  and .source_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_ready == true
  and .source_accepted_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count == 1
  and .source_blocked_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count == 9
  and .source_durable_store_write_guarded_execution_boundary_result_accepted_count == 1
  and .approved_namespace == "hepta.memory.canary"
  and .approved_store == "wal-receipt-canary-artifact"
  and .approved_scope == "session"
  and .durable_store_write_target_id == "hepta-minimal-scoped-memory-real-write-canary-durable-store-write-target-v1"
  and .durable_store_target_store_id == "hepta-memory-durable-store-canary-plan-only"
  and (.source_guarded_execution_boundary_hash_sha256 | type == "string" and length > 0)
  and (.source_guarded_execution_boundary_report_hash_sha256 | type == "string" and length > 0)
  and (.source_guarded_execution_boundary_handoff_sha256 | type == "string" and length > 0)
  and (.single_shot_execution_envelope_sha256 | type == "string" and length > 0)
  and (.single_shot_nonce_sha256 | type == "string" and length > 0)
  and (.single_shot_command_sha256 | type == "string" and length > 0)
  and (.single_shot_budget_sha256 | type == "string" and length > 0)
  and (.single_shot_wal_hash_sha256 | type == "string" and length > 0)
  and (.single_shot_receipt_hash_sha256 | type == "string" and length > 0)
  and (.single_shot_receipt_hash_chain_sha256 | type == "string" and length > 0)
  and (.single_shot_cleanup_receipt_hash_sha256 | type == "string" and length > 0)
  and (.single_shot_execution_hash_sha256 | type == "string" and length > 0)
  and (.minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_hash_sha256 | type == "string" and length > 0)
  and .single_shot_canary_pre_write_memory_count == 0
  and .single_shot_canary_post_write_memory_count == 1
  and .single_shot_canary_readback_hit_count == 1
  and .single_shot_canary_rollback_restored == true
  and .single_shot_canary_post_rollback_memory_count == 0
  and .single_shot_canary_post_rollback_absence_confirmed == true
  and .single_shot_canary_artifact_pre_count == 0
  and .single_shot_canary_artifact_write_count == 3
  and .single_shot_canary_artifact_readback_count == 3
  and .single_shot_canary_artifact_cleanup_removed_count == 3
  and .single_shot_canary_artifact_post_cleanup_count == 0
  and .single_shot_canary_artifact_zero_residue_confirmed == true
  and .required_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_surface_count == 12
  and .ready_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_surface_count == 12
  and .minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count == 10
  and .accepted_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count == 1
  and .blocked_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count == 9
  and .durable_store_write_single_shot_execution_result_accepted_count == 1
  and .denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_count == 36
  and (.minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixtures | length) == 10
  and ([.minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixtures[] | select(.minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_accepted == true)] | length) == 1
  and .durable_store_write_execution_performed == true
  and .memory_write_execution_performed == true
  and .memory_store_write_performed == true
  and .memory_store_mutated == true
  and .wal_write_performed == true
  and .receipt_persisted == true
  and .post_write_readback_performed == true
  and .rollback_executed == true
  and .tombstone_cleanup_executed == true
  and .single_shot_canary_nonce_consumed == true
  and .single_shot_canary_explicit_command_accepted == true
  and .single_shot_canary_receipt_hash_chain_verified == true
  and .single_shot_canary_zero_residue_confirmed == true
  and .production_durable_memory_backend_present == false
  and .production_durable_memory_store_write_performed == false
  and .actual_production_durable_memory_write_performed == false
  and .durable_memory_store_read_performed == false
  and .durable_memory_store_write_performed == false
  and .durable_memory_store_rollback_performed == false
  and .raw_payload_plaintext_recorded == false
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
  and .side_effects.memory_store_write_performed == true
  and .side_effects.durable_memory_store_write_performed == false
  and .side_effects.external_send_performed == false
  and .allowed_next_actions[0].action == "run_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_require_live_gate"
  and .allowed_next_actions[0].writes_production_durable_memory == false
  and .allowed_next_actions[0].mutates_request_local_canary_store == true
  and .allowed_next_actions[1].action == "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary"
  and .allowed_next_actions[1].requires_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary == true
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
    and .memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_ready == true
    and .minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_ready == true
    and .minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_performed == true
    and .minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_accepted == true
    and .source_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_ready == true
    and .approved_namespace == "hepta.memory.canary"
    and .approved_store == "wal-receipt-canary-artifact"
    and .approved_scope == "session"
    and .single_shot_canary_pre_write_memory_count == 0
    and .single_shot_canary_post_write_memory_count == 1
    and .single_shot_canary_readback_hit_count == 1
    and .single_shot_canary_rollback_restored == true
    and .single_shot_canary_post_rollback_memory_count == 0
    and .single_shot_canary_post_rollback_absence_confirmed == true
    and .single_shot_canary_artifact_write_count == 3
    and .single_shot_canary_artifact_readback_count == 3
    and .single_shot_canary_artifact_cleanup_removed_count == 3
    and .single_shot_canary_artifact_post_cleanup_count == 0
    and .single_shot_canary_artifact_zero_residue_confirmed == true
    and .durable_store_write_execution_performed == true
    and .memory_store_write_performed == true
    and .wal_write_performed == true
    and .receipt_persisted == true
    and .post_write_readback_performed == true
    and .rollback_executed == true
    and .tombstone_cleanup_executed == true
    and .production_durable_memory_backend_present == false
    and .production_durable_memory_store_write_performed == false
    and .actual_production_durable_memory_write_performed == false
    and .durable_memory_store_read_performed == false
    and .durable_memory_store_write_performed == false
    and .durable_memory_store_rollback_performed == false
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
    and .side_effects.memory_store_write_performed == true
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
  --arg gate "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_route_gate" \
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
    minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_route_gate_ready:true,
    source_text_verified:true,
    focused_rust_test_verified:true,
    script_gate_verified:true,
    live_endpoint_verified:($require_live_endpoint == 1),
    terminal_coverage_verified:true,
    test_log_sha256:$test_log_sha256,
    script_gate_sha256:$script_gate_sha256,
    live_route_sha256:$live_route_sha256,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    script_gate_summary:{
      accepted_fixture_count:$script_gate.accepted_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count,
      blocked_fixture_count:$script_gate.blocked_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count,
      canary_write:$script_gate.memory_store_write_performed,
      production_durable_write:$script_gate.production_durable_memory_store_write_performed,
      zero_residue:$script_gate.single_shot_canary_artifact_zero_residue_confirmed
    },
    live_route_summary:(if $require_live_endpoint == 1 then {
      route_count:$live_route.route_count,
      accepted_fixture_count:$live_route.accepted_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count,
      canary_write:$live_route.memory_store_write_performed,
      production_durable_write:$live_route.production_durable_memory_store_write_performed,
      zero_residue:$live_route.single_shot_canary_artifact_zero_residue_confirmed
    } else {} end),
    denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_route_gate:[
      "production_durable_memory_backend_write_denied",
      "kg_live_write_denied",
      "provider_model_invocation_denied",
      "credential_read_denied",
      "channel_external_send_denied",
      "public_release_artifact_denied",
      "install_restart_active_binary_mutation_denied"
    ],
    side_effects:{
      cargo_focused_test_executed:true,
      child_gate_execution_performed:true,
      live_endpoint_read_performed:($require_live_endpoint == 1),
      production_durable_memory_store_write_performed:false,
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
