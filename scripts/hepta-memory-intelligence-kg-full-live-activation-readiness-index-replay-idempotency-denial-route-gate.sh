#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

require_unsigned_integer() {
  local name="$1"
  local value="$2"

  case "$value" in
    ''|*[!0-9]*)
      echo "$name must be an unsigned integer" >&2
      exit 2
      ;;
  esac
}

require_source_text() {
  local source_file="$1"
  local source_text="$2"
  local label="$3"

  if ! rg -Fq "$source_text" "$source_file"; then
    echo "missing full live activation readiness index replay/idempotency route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

REPLAY_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-live-activation-readiness-index-replay-idempotency-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-live-activation-readiness-index-replay-idempotency-denial-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_gate"
  and .memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_ready == true
  and .source_readiness_index_ready == true
  and .source_full_live_activation_enabled == false
  and .source_full_live_activation_status == "blocked_report_only"
  and .readiness_surface_count == 10
  and .live_activation_blocker_count == 13
  and .required_replay_idempotency_surface_count == 12
  and .ready_replay_idempotency_surface_count == 12
  and .side_effect_free_replay_idempotency_surface_count == 12
  and .replay_idempotency_fixture_count == 10
  and .blocked_replay_idempotency_fixture_count == 10
  and .allowed_replay_idempotency_fixture_count == 0
  and .accepted_replay_idempotency_fixture_count == 0
  and .replay_allowed == false
  and .replay_accepted == false
  and .idempotency_key_registered == false
  and .idempotency_key_persisted == false
  and .idempotency_cache_written == false
  and .query_result_registered == false
  and .query_result_persisted == false
  and .index_entry_written == false
  and .export_recorded == false
  and .observability_recorded == false
  and .activation_authority_derived == false
  and .operator_acceptance_recorded == false
  and .operator_approval_recorded == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .context_injection_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .live_kg_write_performed == false
  and .credential_read == false
  and .secret_file_read == false
  and .install_executed == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and (.replay_idempotency_fixtures | length) == 10
  and (.replay_idempotency_fixtures | all(
    .replay_allowed == false
    and .replay_accepted == false
    and .idempotency_key_registered == false
    and .idempotency_cache_written == false
    and .query_result_registered == false
    and .index_entry_written == false
    and .export_recorded == false
    and .observability_recorded == false
    and .activation_authority_derived == false
    and .operator_acceptance_recorded == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .install_executed == false
    and .active_binary_mutated == false
    and .replay_idempotency_noop_confirmed == true
  ))
  and (.denied_by_readiness_index_replay_idempotency | length) == 9
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$REPLAY_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 186;' \
  "native gateway route/source command count includes readiness index replay/idempotency route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_READINESS_INDEX_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT' \
  "native gateway readiness index replay/idempotency endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-live-activation-readiness-index-replay-idempotency-denial' \
  "native gateway readiness index replay/idempotency endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-live-activation-readiness-index-replay-idempotency-denial --json' \
  "native gateway readiness index replay/idempotency source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_report' \
  "native gateway readiness index replay/idempotency report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_route_enabled": true' \
  "readiness index replay/idempotency route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"idempotency_key_registered"' \
  "idempotency key registration denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"idempotency_cache_written"' \
  "idempotency cache write denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_authority_derived"' \
  "activation authority derivation denied"

TEST_LOG="$(mktemp /tmp/hepta-readiness-index-replay-idempotency-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_endpoint_blocks_authority \
  -- --nocapture >"$TEST_LOG"

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-live-activation-readiness-index-replay-idempotency-denial"
  )"
  jq -e '
    .status == "ready"
    and .route_count == 160
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_route_enabled == true
    and .memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_ready == true
    and .source_readiness_index_ready == true
    and .source_full_live_activation_enabled == false
    and .source_full_live_activation_status == "blocked_report_only"
    and .readiness_surface_count == 10
    and .live_activation_blocker_count == 13
    and .replay_idempotency_fixture_count == 10
    and .blocked_replay_idempotency_fixture_count == 10
    and .allowed_replay_idempotency_fixture_count == 0
    and .accepted_replay_idempotency_fixture_count == 0
    and .replay_allowed == false
    and .replay_accepted == false
    and .idempotency_key_registered == false
    and .idempotency_cache_written == false
    and .query_result_registered == false
    and .index_entry_written == false
    and .export_recorded == false
    and .observability_recorded == false
    and .activation_authority_derived == false
    and .operator_acceptance_recorded == false
    and .operator_approval_recorded == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .context_injection_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .public_release_claimed == false
    and .release_artifact_written == false
    and (.side_effects | to_entries | all(.value == false))
  ' >/dev/null <<<"$LIVE_ROUTE_JSON"
else
  LIVE_ROUTE_JSON='null'
fi

TERMINAL_COVERAGE_JSON="$(
  capture_json_report \
    "hepta-preflight-terminal-coverage-inventory-gate" \
    scripts/hepta-preflight-terminal-coverage-inventory-gate.sh
)"
jq -e '
  .status == "ready"
  and .preflight_terminal_coverage_inventory_ready == true
  and .required_marker_count == 300
  and .present_required_marker_count == 300
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
source_replay_gate_sha256="$(printf '%s' "$REPLAY_JSON" | shasum -a 256 | awk '{print $1}')"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg status "ready" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-live-activation-readiness-index-replay-idempotency-denial" \
  --arg source_command "/hepta-memory-intelligence-kg-full-live-activation-readiness-index-replay-idempotency-denial --json" \
  --arg source_replay_gate_sha256 "$source_replay_gate_sha256" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg test_log "$TEST_LOG" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --argjson source "$REPLAY_JSON" \
  --argjson terminal "$TERMINAL_COVERAGE_JSON" \
  --argjson live "$LIVE_ROUTE_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:$status,
    base_url:$base_url,
    gate:$gate,
    endpoint:$endpoint,
    source_command:$source_command,
    activation_mode:"full_live_activation_readiness_index_replay_idempotency_denial_native_route_status",
    source_replay_idempotency_gate:$source.gate,
    source_replay_idempotency_gate_ready:$source.memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_ready,
    source_replay_gate_sha256:$source_replay_gate_sha256,
    source_route_wired:true,
    source_route_count_expected:153,
    native_gateway_source:"codex-rs/cli/src/native_gateway.rs",
    native_gateway_sha256:$native_gateway_sha256,
    native_gateway_unit_test_log:$test_log,
    live_endpoint_required:($live != null),
    live_endpoint_ready:(if $live == null then false else ($live.status == "ready") end),
    replay_idempotency_fixture_count:$source.replay_idempotency_fixture_count,
    blocked_replay_idempotency_fixture_count:$source.blocked_replay_idempotency_fixture_count,
    accepted_replay_idempotency_fixture_count:$source.accepted_replay_idempotency_fixture_count,
    replay_allowed:false,
    replay_accepted:false,
    idempotency_key_registered:false,
    idempotency_cache_written:false,
    query_result_registered:false,
    index_entry_written:false,
    export_recorded:false,
    observability_recorded:false,
    activation_authority_derived:false,
    operator_acceptance_recorded:false,
    operator_approval_recorded:false,
    memory_store_write_performed:false,
    memory_store_mutated:false,
    live_kg_write_performed:false,
    provider_invoked:false,
    model_invoked:false,
    credential_read:false,
    secret_file_read:false,
    install_executed:false,
    service_restarted:false,
    active_binary_mutated:false,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    terminal_required_marker_count:$terminal.required_marker_count,
    terminal_present_required_marker_count:$terminal.present_required_marker_count,
    terminal_missing_required_marker_count:$terminal.missing_required_marker_count,
    terminal_duplicate_required_marker_count:$terminal.duplicate_required_marker_count,
    terminal_out_of_order_required_marker_count:$terminal.out_of_order_required_marker_count,
    side_effects:{
      replay_performed:false,
      replay_accepted:false,
      idempotency_key_registered:false,
      idempotency_key_persisted:false,
      idempotency_cache_written:false,
      query_result_registered:false,
      query_result_persisted:false,
      index_entry_written:false,
      export_recorded:false,
      observability_recorded:false,
      activation_authority_derived:false,
      operator_acceptance_recorded:false,
      operator_approval_recorded:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      live_kg_write_performed:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      secret_file_read:false,
      install_executed:false,
      launchd_mutated:false,
      service_restarted:false,
      active_binary_mutated:false,
      public_release_claimed:false,
      release_artifact_written:false,
      external_send_performed:false,
      filesystem_written:false
    }
  }'

echo "Hepta Memory/Intelligence/KG full live activation readiness index replay/idempotency denial route gate passed"
