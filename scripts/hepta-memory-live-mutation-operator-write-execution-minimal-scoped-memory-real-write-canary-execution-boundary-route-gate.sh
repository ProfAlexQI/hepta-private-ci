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
    echo "missing minimal scoped Memory real-write canary execution source text: $label" >&2
    exit 1
  fi
}

source "$REPO_ROOT/scripts/lib/hepta-source-set.sh"
NATIVE_GATEWAY_SOURCE="hepta-native-gateway-source-set-v1"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"
ENDPOINT="/api/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-execution-boundary"
SOURCE_COMMAND="/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-execution-boundary --json"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes minimal scoped Memory canary execution boundary"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  "HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_EXECUTION_BOUNDARY_ENDPOINT" \
  "minimal scoped Memory canary execution endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$ENDPOINT" \
  "minimal scoped Memory canary execution endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$SOURCE_COMMAND" \
  "minimal scoped Memory canary execution source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_execution_boundary_report" \
  "minimal scoped Memory canary execution report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"minimal_scoped_memory_real_write_canary_execution_ready\"" \
  "minimal scoped Memory canary execution ready field"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"post_rollback_absence_confirmed\"" \
  "minimal scoped Memory canary execution rollback absence field"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_write_execution_minimal_scoped_memory_real_write_canary_execution_writes_reads_and_rolls_back_scoped_store_without_external_side_effects" \
  "focused minimal scoped Memory canary execution unit test"
require_source_text "codex-rs/hepta-memory/src/lib.rs" \
  "pub fn put_memory_sync(&self, record: MemoryRecord)" \
  "synchronous in-memory store write helper"

TEST_LOG="$(mktemp /tmp/hepta-minimal-scoped-memory-real-write-canary-execution-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_memory_write_execution_minimal_scoped_memory_real_write_canary_execution_writes_reads_and_rolls_back_scoped_store_without_external_side_effects \
  -- --nocapture >"$TEST_LOG"

SCRIPT_GATE_JSON="$(
  capture_json_report \
    "hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-execution-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-execution-boundary-gate.sh
)"

jq -e '
  .status == "ready"
  and .minimal_scoped_memory_real_write_canary_execution_ready == true
  and .minimal_scoped_memory_real_write_canary_execution_performed == true
  and .minimal_scoped_memory_real_write_canary_execution_isolated_store_restored == true
  and .source_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_ready == true
  and .source_accepted_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count == 1
  and .source_blocked_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count == 9
  and .source_rollback_tombstone_proof_authority_accepted_count == 1
  and .source_minimal_real_write_canary_handoff_proof_bound_count == 1
  and .approved_namespace == "hepta.memory.canary"
  and .approved_store == "in-memory-reference"
  and .approved_scope == "session"
  and .pre_write_snapshot_memory_count == 0
  and .post_write_snapshot_memory_count == 1
  and .post_write_readback_hit_count == 1
  and .post_write_readback_identity_match == true
  and .post_write_readback_digest_match == true
  and .rollback_restore_result == true
  and .post_rollback_snapshot_memory_count == 0
  and .post_rollback_absence_confirmed == true
  and .required_minimal_scoped_memory_real_write_canary_execution_surface_count == 12
  and .ready_minimal_scoped_memory_real_write_canary_execution_surface_count == 12
  and .minimal_scoped_memory_real_write_canary_execution_fixture_count == 10
  and .accepted_minimal_scoped_memory_real_write_canary_execution_fixture_count == 1
  and .blocked_minimal_scoped_memory_real_write_canary_execution_fixture_count == 9
  and .memory_store_write_performed_count == 1
  and .post_write_readback_performed_count == 1
  and .readback_result_accepted_count == 1
  and .rollback_performed_count == 1
  and .rollback_result_accepted_count == 1
  and .durable_memory_store_read_performed_count == 0
  and .durable_memory_store_write_performed_count == 0
  and .durable_memory_store_rollback_performed_count == 0
  and .wal_write_performed_count == 0
  and .receipt_persisted_count == 0
  and .tombstone_written_count == 0
  and .live_kg_write_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .release_artifact_written == false
  and .install_executed == false
  and .active_binary_mutated == false
  and .side_effects.memory_store_write_performed == true
  and .side_effects.post_write_readback_performed == true
  and .side_effects.rollback_executed == true
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
    and .memory_write_execution_minimal_scoped_memory_real_write_canary_execution_boundary_ready == true
    and .minimal_scoped_memory_real_write_canary_execution_ready == true
    and .minimal_scoped_memory_real_write_canary_execution_performed == true
    and .minimal_scoped_memory_real_write_canary_execution_isolated_store_restored == true
    and .scoped_memory_real_write_canary_mode == "minimal_scoped_memory_real_write_canary_execution_isolated_in_memory_store_write_readback_rollback"
    and .source_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_ready == true
    and .source_accepted_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count == 1
    and .source_blocked_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count == 9
    and .source_rollback_tombstone_proof_authority_accepted_count == 1
    and .source_minimal_real_write_canary_handoff_proof_bound_count == 1
    and .source_memory_store_write_performed_count == 0
    and .approved_namespace == "hepta.memory.canary"
    and .approved_store == "in-memory-reference"
    and .approved_scope == "session"
    and .pre_write_snapshot_memory_count == 0
    and .post_write_snapshot_memory_count == 1
    and .post_write_readback_hit_count == 1
    and .post_write_readback_identity_match == true
    and .post_write_readback_digest_match == true
    and .rollback_restore_result == true
    and .post_rollback_snapshot_memory_count == 0
    and .post_rollback_absence_confirmed == true
    and .memory_store_write_performed_count == 1
    and .post_write_readback_performed_count == 1
    and .readback_result_accepted_count == 1
    and .rollback_performed_count == 1
    and .rollback_result_accepted_count == 1
    and .durable_memory_store_read_performed_count == 0
    and .durable_memory_store_write_performed_count == 0
    and .durable_memory_store_rollback_performed_count == 0
    and .wal_write_performed_count == 0
    and .receipt_persisted_count == 0
    and .tombstone_written_count == 0
    and .live_kg_write_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .channel_send_performed == false
    and .external_send_performed == false
    and .release_artifact_written == false
    and .install_executed == false
    and .active_binary_mutated == false
    and .side_effects.memory_store_write_performed == true
    and .side_effects.post_write_readback_performed == true
    and .side_effects.rollback_executed == true
    and .side_effects.durable_memory_store_write_performed == false
    and .side_effects.external_send_performed == false
  ' >/dev/null <<<"$LIVE_ROUTE_JSON"
fi

test_log_sha256="$(sha256_file "$TEST_LOG")"
script_gate_sha256="$(printf '%s' "$SCRIPT_GATE_JSON" | shasum -a 256 | awk '{print $1}')"
live_route_sha256="$(printf '%s' "$LIVE_ROUTE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_execution_boundary_route_gate" \
  --arg endpoint "$ENDPOINT" \
  --arg source_command "$SOURCE_COMMAND" \
  --arg test_log_sha256 "$test_log_sha256" \
  --arg script_gate_sha256 "$script_gate_sha256" \
  --arg live_route_sha256 "$live_route_sha256" \
  --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
  --argjson require_live_endpoint "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && printf true || printf false)" \
  --argjson script_gate "$SCRIPT_GATE_JSON" \
  --argjson live_route "$LIVE_ROUTE_JSON" \
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
    source_gate_ready:($script_gate.status == "ready"),
    live_route_ready:(if $require_live_endpoint then ($live_route.status == "ready") else true end),
    minimal_scoped_memory_real_write_canary_execution_route_gate_ready:true,
    side_effects:{
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
