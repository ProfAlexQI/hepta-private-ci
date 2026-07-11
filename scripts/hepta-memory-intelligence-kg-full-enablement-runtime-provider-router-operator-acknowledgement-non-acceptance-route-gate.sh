#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"
EXPECTED_ROUTE_COUNT="${HEPTA_EXPECTED_ROUTE_COUNT:-$(bash "$REPO_ROOT/scripts/lib/hepta-native-route-count.sh")}"

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
    echo "missing runtime provider-router operator acknowledgement non-acceptance route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

SOURCE_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-acknowledgement-non-acceptance-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-acknowledgement-non-acceptance-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_operator_acknowledgement_non_acceptance_gate"
  and .runtime_provider_router_operator_acknowledgement_non_acceptance_ready == true
  and .runtime_provider_router_operator_acknowledgement_non_acceptance_status == "blocked"
  and .operator_facing_summary_non_persistence_ready == true
  and .receipt_observability_denial_ready == true
  and .operator_acknowledgement_surface_count == 12
  and .operator_acknowledgement_surface_ready_count == 12
  and .operator_acknowledgement_fixture_count == 10
  and .blocked_operator_acknowledgement_fixture_count == 10
  and .noop_operator_acknowledgement_fixture_count == 10
  and .allowed_operator_acknowledgement_fixture_count == 0
  and .accepted_operator_acknowledgement_fixture_count == 0
  and .operator_acknowledgement_denied_count == 10
  and .operator_acknowledgement_performed_count == 0
  and .operator_acknowledgement_allowed == false
  and .operator_acknowledgement_request_accepted == false
  and .operator_acknowledgement_recorded == false
  and .operator_acknowledgement_persisted == false
  and .operator_acknowledgement_accepted == false
  and .operator_identity_accepted == false
  and .operator_scope_accepted == false
  and .operator_activation_plan_accepted == false
  and .receipt_acknowledgement_accepted == false
  and .runtime_attachment_acknowledged == false
  and .live_context_acknowledged == false
  and .memory_kg_acknowledged == false
  and .provider_secret_acknowledged == false
  and .receipt_exported == false
  and .receipt_query_registered == false
  and .receipt_observability_recorded == false
  and .receipt_recorded == false
  and .receipt_persisted == false
  and .receipt_accepted == false
  and .readback_evidence_recorded == false
  and .readback_evidence_persisted == false
  and .router_handoff_recorded == false
  and .router_handoff_persisted == false
  and .runtime_router_mutated == false
  and .live_context_attached == false
  and .context_injection_performed == false
  and .adapter_invoked == false
  and .provider_invoked == false
  and .model_invoked == false
  and .auth_secret_read == false
  and .credential_read == false
  and .secret_file_read == false
  and .usage_recorded == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .live_kg_write_performed == false
  and .rollback_executed == false
  and .service_restart_performed == false
  and .active_binary_mutated == false
  and (.operator_acknowledgement_fixtures | length) == 10
  and (.operator_acknowledgement_fixtures | all(
    (.operator_acknowledgement_status | startswith("blocked_"))
    and .operator_acknowledgement_recorded == false
    and .operator_acknowledgement_persisted == false
    and .operator_acknowledgement_accepted == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .acknowledgement_noop_confirmed == true
  ))
  and (.denied_by_operator_acknowledgement_non_acceptance | length) == 27
  and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_request_denial_matrix" and .status == "allowed_report_only_next_slice" and .accepts_activation_request == false and .invokes_model == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$SOURCE_JSON"

source "$REPO_ROOT/scripts/lib/hepta-source-set.sh"
NATIVE_GATEWAY_SOURCE="hepta-native-gateway-source-set-v1"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes runtime provider-router operator acknowledgement route"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_ENDPOINT' \
  "native gateway runtime provider-router operator acknowledgement endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-acknowledgement-non-acceptance' \
  "native gateway runtime provider-router operator acknowledgement endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-acknowledgement-non-acceptance --json' \
  "native gateway runtime provider-router operator acknowledgement source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_operator_acknowledgement_non_acceptance_report' \
  "native gateway runtime provider-router operator acknowledgement report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"runtime_provider_router_operator_acknowledgement_non_acceptance_route_enabled": true' \
  "runtime provider-router operator acknowledgement route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"operator_acknowledgement_recorded": false' \
  "operator acknowledgement recording denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"operator_acknowledgement_accepted": false' \
  "operator acknowledgement acceptance denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"stage_runtime_provider_router_activation_request_denial_matrix"' \
  "next action remains activation request denial matrix"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_operator_acknowledgement_non_acceptance_endpoint_blocks_acknowledgement_side_effects' \
  "focused runtime provider-router operator acknowledgement unit test"

TEST_LOG="$(mktemp /tmp/hepta-runtime-provider-router-operator-acknowledgement-non-acceptance-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_operator_acknowledgement_non_acceptance_endpoint_blocks_acknowledgement_side_effects \
  -- --nocapture >"$TEST_LOG"

live_route_status="skipped"
live_route_count=0
live_missing_route_count=0

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-acknowledgement-non-acceptance"
  )"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .implemented_route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .runtime_provider_router_operator_acknowledgement_non_acceptance_route_enabled == true
    and .runtime_provider_router_operator_acknowledgement_non_acceptance_ready == true
    and .runtime_provider_router_operator_acknowledgement_non_acceptance_status == "blocked"
    and .source_operator_summary_non_persistence_ready == true
    and .source_receipt_observability_denial_ready == true
    and .operator_acknowledgement_surface_count == 12
    and .operator_acknowledgement_fixture_count == 10
    and .blocked_operator_acknowledgement_fixture_count == 10
    and .allowed_operator_acknowledgement_fixture_count == 0
    and .accepted_operator_acknowledgement_fixture_count == 0
    and .operator_acknowledgement_denied_count == 10
    and .operator_acknowledgement_performed_count == 0
    and .operator_acknowledgement_recorded == false
    and .operator_acknowledgement_persisted == false
    and .operator_acknowledgement_accepted == false
    and .operator_identity_accepted == false
    and .operator_scope_accepted == false
    and .operator_activation_plan_accepted == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .active_binary_mutated == false
    and (.operator_acknowledgement_fixtures | length) == 10
    and (.denied_by_operator_acknowledgement_non_acceptance | length) == 27
    and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_request_denial_matrix" and .status == "allowed_report_only_next_slice"))
    and (.side_effects | to_entries | all(.value == false))
  ' >/dev/null <<<"$LIVE_JSON"
  live_route_status="$(jq -r '.status' <<<"$LIVE_JSON")"
  live_route_count="$(jq -r '.route_count' <<<"$LIVE_JSON")"
  live_missing_route_count="$(jq -r '.missing_route_count' <<<"$LIVE_JSON")"
fi

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_operator_acknowledgement_non_acceptance_route_gate" \
    --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-acknowledgement-non-acceptance" \
    --arg source_command "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-acknowledgement-non-acceptance --json" \
    --arg native_gateway_sha256 "$(sha256_file "$NATIVE_GATEWAY_SOURCE")" \
    --arg test_log "$TEST_LOG" \
    --arg live_route_status "$live_route_status" \
    --argjson live_route_count "$live_route_count" \
    --argjson live_missing_route_count "$live_missing_route_count" \
    --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
    --argjson live_endpoint_checked "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && echo true || echo false)" \
    --argjson source "$SOURCE_JSON" \
    '{
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      source_command:$source_command,
      native_gateway_sha256:$native_gateway_sha256,
      focused_test_log:$test_log,
      live_endpoint_checked:$live_endpoint_checked,
      live_route_status:$live_route_status,
      live_route_count:$live_route_count,
      live_missing_route_count:$live_missing_route_count,
      expected_route_count:$expected_route_count,
      source_operator_acknowledgement_gate:$source.gate,
      source_operator_acknowledgement_ready:$source.runtime_provider_router_operator_acknowledgement_non_acceptance_ready,
      source_operator_acknowledgement_status:$source.runtime_provider_router_operator_acknowledgement_non_acceptance_status,
      route_gate_ready:true,
      runtime_provider_router_operator_acknowledgement_non_acceptance_ready:true,
      operator_acknowledgement_surface_count:12,
      operator_acknowledgement_fixture_count:10,
      accepted_operator_acknowledgement_fixture_count:0,
      denied_by_operator_acknowledgement_non_acceptance_count:27,
      next_slice:"runtime_provider_router_activation_request_denial_matrix",
      side_effects:{
        operator_acknowledgement_recorded:false,
        operator_acknowledgement_persisted:false,
        operator_acknowledgement_accepted:false,
        operator_identity_accepted:false,
        operator_scope_accepted:false,
        operator_activation_plan_accepted:false,
        receipt_acknowledgement_accepted:false,
        runtime_attachment_acknowledged:false,
        live_context_acknowledged:false,
        memory_kg_acknowledged:false,
        provider_secret_acknowledged:false,
        provider_invoked:false,
        model_invoked:false,
        credential_read:false,
        secret_file_read:false,
        memory_store_write_performed:false,
        live_kg_write_performed:false,
        channel_send_performed:false,
        external_send_performed:false,
        install_executed:false,
        service_restarted:false,
        active_binary_mutated:false,
        filesystem_written:false
      }
    }'
)"

printf '%s\n' "$report"
echo "Hepta runtime provider-router operator acknowledgement non-acceptance route gate passed"
