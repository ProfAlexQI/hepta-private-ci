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
    echo "missing minimal scoped Memory real-write canary WAL/receipt source text: $label" >&2
    exit 1
  fi
}

NATIVE_GATEWAY_SOURCE="codex-rs/hepta-native-gateway/src/native_gateway.rs"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"
ENDPOINT="/api/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-wal-receipt-binding-boundary"
SOURCE_COMMAND="/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-wal-receipt-binding-boundary --json"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes minimal scoped Memory canary WAL/receipt binding boundary"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  "HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_WAL_RECEIPT_BINDING_BOUNDARY_ENDPOINT" \
  "minimal scoped Memory canary WAL/receipt binding endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$ENDPOINT" \
  "minimal scoped Memory canary WAL/receipt binding endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$SOURCE_COMMAND" \
  "minimal scoped Memory canary WAL/receipt binding source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_report" \
  "minimal scoped Memory canary WAL/receipt binding report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"minimal_scoped_memory_real_write_canary_wal_receipt_binding_ready\"" \
  "minimal scoped Memory canary WAL/receipt binding ready field"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"denied_by_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_count\"" \
  "minimal scoped Memory canary WAL/receipt binding denial count"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_write_execution_minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepts_bindings_without_writes" \
  "focused minimal scoped Memory canary WAL/receipt binding unit test"

TEST_LOG="$(mktemp /tmp/hepta-minimal-scoped-memory-real-write-canary-wal-receipt-binding-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_memory_write_execution_minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepts_bindings_without_writes \
  -- --nocapture >"$TEST_LOG"

SCRIPT_GATE_JSON="$(
  capture_json_report \
    "hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-wal-receipt-binding-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-wal-receipt-binding-boundary-gate.sh
)"

jq -e '
  .status == "ready"
  and .minimal_scoped_memory_real_write_canary_wal_receipt_binding_ready == true
  and .minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted_no_write == true
  and .source_minimal_scoped_memory_real_write_canary_accepted_gate_boundary_ready == true
  and .accepted_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count == 1
  and .blocked_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count == 9
  and .wal_receipt_binding_authority_accepted_count == 1
  and .wal_record_id_bound_count == 1
  and .receipt_id_bound_count == 1
  and .post_write_readback_handoff_bound_count == 1
  and .single_use_nonce_consumed_count == 0
  and .explicit_command_dispatched_count == 0
  and .wal_write_performed_count == 0
  and .wal_recorded_count == 0
  and .wal_persisted_count == 0
  and .receipt_recorded_count == 0
  and .receipt_persisted_count == 0
  and .receipt_materialized_count == 0
  and .receipt_delivered_count == 0
  and .durable_memory_store_read_performed_count == 0
  and .durable_memory_store_write_performed_count == 0
  and .durable_memory_store_rollback_performed_count == 0
  and .memory_store_write_performed_count == 0
  and .single_use_nonce_consumed == false
  and .explicit_command_dispatched == false
  and .wal_write_performed == false
  and .receipt_recorded == false
  and .receipt_persisted == false
  and .memory_write_execution_performed == false
  and .memory_store_write_performed == false
  and .durable_memory_store_write_performed == false
  and .live_kg_write_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .release_artifact_written == false
  and .install_executed == false
  and .active_binary_mutated == false
  and (.side_effects | to_entries | all(.value == false))
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
    and .memory_write_execution_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_ready == true
    and .minimal_scoped_memory_real_write_canary_wal_receipt_binding_ready == true
    and .minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted_no_write == true
    and .scoped_memory_real_write_canary_mode == "minimal_scoped_memory_real_write_canary_wal_receipt_binding_no_write"
    and .source_minimal_scoped_memory_real_write_canary_accepted_gate_boundary_ready == true
    and .source_minimal_scoped_memory_real_write_canary_accepted_gate_ready == true
    and .source_minimal_scoped_memory_real_write_canary_accepted_gate_report_sha256 != ""
    and .source_accepted_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count == 1
    and .source_blocked_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count == 9
    and .source_minimal_scoped_memory_real_write_canary_authority_accepted_count == 1
    and .source_single_use_nonce_consumed_count == 0
    and .source_explicit_command_dispatched_count == 0
    and .source_wal_write_performed_count == 0
    and .source_receipt_persisted_count == 0
    and .source_durable_memory_store_read_performed_count == 0
    and .source_durable_memory_store_write_performed_count == 0
    and .source_durable_memory_store_rollback_performed_count == 0
    and .source_memory_store_write_performed_count == 0
    and .minimum_required_samples >= 24
    and .required_minimal_scoped_memory_real_write_canary_wal_receipt_binding_surface_count == 12
    and .ready_minimal_scoped_memory_real_write_canary_wal_receipt_binding_surface_count == 12
    and .side_effect_free_minimal_scoped_memory_real_write_canary_wal_receipt_binding_surface_count == 12
    and .required_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count == 10
    and .minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count == 10
    and .accepted_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count == 1
    and .blocked_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count == 9
    and .wal_receipt_binding_authority_accepted_count == 1
    and .wal_namespace_store_scope_bound_count == 1
    and .wal_record_id_bound_count == 1
    and .wal_sequence_guard_bound_count == 1
    and .wal_idempotency_key_bound_count == 1
    and .wal_payload_digest_redaction_bound_count == 1
    and .receipt_id_bound_count == 1
    and .receipt_hash_chain_bound_count == 1
    and .receipt_replay_guard_bound_count == 1
    and .receipt_audit_evidence_bound_count == 1
    and .post_write_readback_handoff_bound_count == 1
    and .minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted_count == 1
    and .single_use_nonce_consumed_count == 0
    and .explicit_command_dispatched_count == 0
    and .wal_write_performed_count == 0
    and .wal_recorded_count == 0
    and .wal_persisted_count == 0
    and .receipt_recorded_count == 0
    and .receipt_persisted_count == 0
    and .receipt_materialized_count == 0
    and .receipt_delivered_count == 0
    and .post_write_readback_performed_count == 0
    and .rollback_performed_count == 0
    and .tombstone_written_count == 0
    and .durable_memory_store_read_performed_count == 0
    and .durable_memory_store_write_performed_count == 0
    and .durable_memory_store_rollback_performed_count == 0
    and .memory_store_write_performed_count == 0
    and .required_before_minimal_scoped_memory_real_write_canary_wal_receipt_binding_count == 17
    and (.required_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fields | length) == 17
    and (.minimal_scoped_memory_real_write_canary_wal_receipt_binding_surfaces | length) == 12
    and (.minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixtures | length) == 10
    and ([.minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixtures[] | select(.minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted == true)] | length) == 1
    and (.minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixtures | all(.single_use_nonce_consumed == false and .explicit_command_dispatched == false and .wal_write_performed == false and .wal_persisted == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_materialized == false and .receipt_delivered == false and .post_write_readback_performed == false and .rollback_executed == false and .tombstone_written == false and .memory_write_execution_performed == false and .memory_store_write_performed == false and .durable_memory_store_read_performed == false and .durable_memory_store_write_performed == false and .durable_memory_store_rollback_performed == false and .live_kg_write_performed == false and .provider_invoked == false and .model_invoked == false and .credential_read == false and .channel_send_performed == false and .external_send_performed == false and .release_artifact_written == false and .install_executed == false and .active_binary_mutated == false and .wal_receipt_binding_noop_confirmed == true))
    and .denied_by_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_count == 28
    and (.denied_by_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary | length) == 28
    and .minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted == true
    and .wal_namespace_bound == true
    and .wal_store_bound == true
    and .wal_scope_bound == true
    and .wal_record_id_bound == true
    and .wal_sequence_guard_bound == true
    and .wal_idempotency_key_bound == true
    and .receipt_id_bound == true
    and .receipt_hash_chain_bound == true
    and .receipt_replay_guard_bound == true
    and .receipt_audit_evidence_bound == true
    and .post_write_readback_handoff_bound == true
    and .nonce_consumption_forbidden_on_report_route == true
    and .explicit_command_dispatch_forbidden_on_report_route == true
    and .wal_write_forbidden == true
    and .wal_persistence_forbidden == true
    and .receipt_recording_forbidden == true
    and .receipt_persistence_forbidden == true
    and .single_use_nonce_consumed == false
    and .explicit_command_dispatched == false
    and .wal_write_performed == false
    and .wal_recorded == false
    and .wal_persisted == false
    and .receipt_recorded == false
    and .receipt_persisted == false
    and .receipt_materialized == false
    and .receipt_delivered == false
    and .post_write_readback_performed == false
    and .rollback_executed == false
    and .tombstone_written == false
    and .memory_write_execution_performed == false
    and .memory_store_write_performed == false
    and .durable_memory_store_write_performed == false
    and .durable_memory_store_read_performed == false
    and .durable_memory_store_rollback_performed == false
    and .live_kg_write_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .channel_send_performed == false
    and .external_send_performed == false
    and .release_artifact_written == false
    and .install_executed == false
    and .active_binary_mutated == false
    and .allowed_next_actions[0].action == "run_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_require_live_gate"
    and .allowed_next_actions[0].writes_memory == false
    and .allowed_next_actions[0].writes_wal == false
    and .allowed_next_actions[1].action == "prepare_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary"
    and .allowed_next_actions[1].requires_minimal_scoped_memory_real_write_canary_wal_receipt_binding == true
    and .allowed_next_actions[1].persists_receipt == false
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
script_gate_sha256="$(printf '%s' "$SCRIPT_GATE_JSON" | shasum -a 256 | awk '{print $1}')"
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
  --arg script_gate_sha256 "$script_gate_sha256" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --arg test_log "$TEST_LOG" \
  --arg live_route_status "$live_route_status" \
  --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
  --argjson live_route_count "$live_route_count" \
  --argjson live_missing_route_count "$live_missing_route_count" \
  --argjson terminal_required_marker_count "$terminal_required_marker_count" \
  --argjson terminal_present_required_marker_count "$terminal_present_required_marker_count" \
  --argjson terminal_missing_required_marker_count "$terminal_missing_required_marker_count" \
  --argjson require_live_endpoint "$(if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then echo true; else echo false; fi)" \
  '{
    product: $product,
    runtime: $runtime,
    status: "ready",
    gate: "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_route_gate",
    base_url: $base_url,
    endpoint: $endpoint,
    source_command: $source_command,
    expected_route_count: $expected_route_count,
    native_gateway_sha256: $native_gateway_sha256,
    script_gate_sha256: $script_gate_sha256,
    terminal_coverage_sha256: $terminal_coverage_sha256,
    focused_rust_test_log: $test_log,
    require_live_endpoint: $require_live_endpoint,
    live_route_status: $live_route_status,
    live_route_count: $live_route_count,
    live_missing_route_count: $live_missing_route_count,
    terminal_required_marker_count: $terminal_required_marker_count,
    terminal_present_required_marker_count: $terminal_present_required_marker_count,
    terminal_missing_required_marker_count: $terminal_missing_required_marker_count,
    minimal_scoped_memory_real_write_canary_wal_receipt_binding_route_gate_ready: true,
    accepts_wal_receipt_binding_only: true,
    consumes_nonce: false,
    dispatches_command: false,
    reads_memory: false,
    writes_memory: false,
    writes_wal: false,
    persists_receipt: false,
    executes_rollback: false,
    writes_tombstone: false,
    writes_kg: false,
    invokes_provider: false,
    reads_credentials: false,
    sends_externally: false,
    publishes_artifacts: false,
    installs_or_restarts: false,
    mutates_active_binary: false
  }'
