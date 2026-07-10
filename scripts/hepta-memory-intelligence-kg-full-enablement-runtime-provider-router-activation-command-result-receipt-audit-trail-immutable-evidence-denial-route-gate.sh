#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"
EXPECTED_ROUTE_COUNT="${HEPTA_EXPECTED_ROUTE_COUNT:-$(bash "$REPO_ROOT/scripts/lib/hepta-native-route-count.sh")}"

cd "$REPO_ROOT"

source scripts/lib/hepta-json-report-capture.sh

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
    echo "missing runtime provider-router activation command result receipt audit-trail/immutable-evidence route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report \
      "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-audit-trail-immutable-evidence-denial-gate" \
      scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-audit-trail-immutable-evidence-denial-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_gate"
  and .runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status == "blocked"
  and .source_activation_command_result_receipt_cancellation_supersession_ready == true
  and .source_activation_command_result_receipt_cancellation_supersession_status == "blocked"
  and .runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_no_persistence_ready == true
  and .minimum_required_samples >= 24
  and .audit_trail_immutable_evidence_surface_count == 12
  and .audit_trail_immutable_evidence_surface_ready_count == 12
  and .audit_trail_immutable_evidence_side_effect_free_surface_count == 12
  and .audit_trail_immutable_evidence_fixture_count == 10
  and .blocked_audit_trail_immutable_evidence_fixture_count == 10
  and .noop_audit_trail_immutable_evidence_fixture_count == 10
  and .allowed_audit_trail_immutable_evidence_fixture_count == 0
  and .accepted_audit_trail_immutable_evidence_fixture_count == 0
  and .audit_trail_denied_count == 10
  and .immutable_evidence_denied_count == 6
  and .audit_trail_performed_count == 0
  and .immutable_evidence_performed_count == 0
  and .hash_chain_recorded_count == 0
  and .merkle_root_recorded_count == 0
  and .attestation_recorded_count == 0
  and .witness_recorded_count == 0
  and .notary_recorded_count == 0
  and .activation_command_result_receipt_audit_trail_allowed == false
  and .activation_command_result_receipt_audit_trail_recorded == false
  and .activation_command_result_receipt_audit_trail_persisted == false
  and .activation_command_result_receipt_audit_trail_materialized == false
  and .activation_command_result_receipt_audit_trail_filesystem_written == false
  and .activation_command_result_receipt_immutable_evidence_allowed == false
  and .activation_command_result_receipt_immutable_evidence_recorded == false
  and .activation_command_result_receipt_immutable_evidence_persisted == false
  and .activation_command_result_receipt_immutable_evidence_materialized == false
  and .activation_command_result_receipt_immutable_evidence_filesystem_written == false
  and .activation_command_result_receipt_hash_chain_recorded == false
  and .activation_command_result_receipt_merkle_root_recorded == false
  and .activation_command_result_receipt_attestation_recorded == false
  and .activation_command_result_receipt_witness_recorded == false
  and .activation_command_result_receipt_notary_recorded == false
  and .activation_command_result_receipt_cancellation_allowed == false
  and .activation_command_result_receipt_supersession_allowed == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_from_audit_trail_allowed == false
  and .activation_from_immutable_evidence_allowed == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_request_accepted == false
  and .activation_request_executed == false
  and .activation_activated == false
  and .runtime_router_mutated == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .secret_file_read == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .live_kg_write_performed == false
  and .rollback_executed == false
  and .external_send_performed == false
  and .install_executed == false
  and .service_restart_performed == false
  and .active_binary_mutated == false
  and (.audit_trail_immutable_evidence_surfaces | length) == 12
  and (.audit_trail_immutable_evidence_fixtures | length) == 10
  and (.audit_trail_immutable_evidence_fixtures | all(
    (.audit_evidence_status | startswith("blocked"))
    and .audit_trail_allowed == false
    and .audit_trail_recorded == false
    and .audit_trail_persisted == false
    and .immutable_evidence_allowed == false
    and .immutable_evidence_recorded == false
    and .immutable_evidence_persisted == false
    and .hash_chain_recorded == false
    and .merkle_root_recorded == false
    and .attestation_recorded == false
    and .witness_recorded == false
    and .notary_recorded == false
    and .activation_command_result_receipt_recorded == false
    and .activation_command_result_receipt_persisted == false
    and .activation_command_result_receipt_accepted == false
    and .activation_activated == false
    and .runtime_router_mutated == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .secret_file_read == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .rollback_executed == false
    and .external_send_performed == false
    and .install_executed == false
    and .service_restart_performed == false
    and .active_binary_mutated == false
    and .receipt_noop_confirmed == true
  ))
  and ([.audit_trail_immutable_evidence_fixtures[] | select(.source_cancellation_supersession_present == false)] | length) == 1
  and ([.audit_trail_immutable_evidence_fixtures[] | select(.immutable_evidence_requested == true)] | length) == 6
  and ([.audit_trail_immutable_evidence_fixtures[] | select(.hash_chain_requested == true and .merkle_root_requested == true)] | length) == 1
  and ([.audit_trail_immutable_evidence_fixtures[] | select(.attestation_requested == true and .witness_requested == true and .notary_requested == true)] | length) == 1
  and ([.audit_trail_immutable_evidence_fixtures[] | select(.audit_trail_materialization_requested == true and .audit_trail_filesystem_write_requested == true)] | length) == 1
  and ([.audit_trail_immutable_evidence_fixtures[] | select(.ledger_evidence_requested == true and .index_evidence_requested == true and .delivery_evidence_requested == true)] | length) == 1
  and ([.audit_trail_immutable_evidence_fixtures[] | select(.activation_from_audit_evidence_requested == true)] | length) == 1
  and ([.audit_trail_immutable_evidence_fixtures[] | select(.memory_store_evidence_requested == true and .live_kg_evidence_requested == true and .provider_prompt_evidence_requested == true)] | length) == 1
  and ([.audit_trail_immutable_evidence_fixtures[] | select(.external_send_evidence_requested == true and .install_evidence_requested == true and .active_binary_mutation_evidence_requested == true)] | length) == 1
  and (.denied_by_audit_trail_immutable_evidence | length) == 24
  and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial" and .status == "allowed_report_only_next_slice" and .performs_retention == false and .performs_gc == false and .mutates_runtime == false and .invokes_model == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$SOURCE_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/hepta-native-gateway/src/native_gateway.rs"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native source command count"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT' \
  "runtime provider-router activation command result receipt audit-trail/immutable-evidence endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-audit-trail-immutable-evidence-denial' \
  "runtime provider-router activation command result receipt audit-trail/immutable-evidence endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-audit-trail-immutable-evidence-denial --json' \
  "runtime provider-router activation command result receipt audit-trail/immutable-evidence source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_report' \
  "runtime provider-router activation command result receipt audit-trail/immutable-evidence report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_route_enabled": true' \
  "runtime provider-router activation command result receipt audit-trail/immutable-evidence route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_endpoint_blocks_evidence' \
  "runtime provider-router activation command result receipt audit-trail/immutable-evidence focused test"

TEST_LOG="$(mktemp /tmp/hepta-runtime-provider-router-activation-command-result-receipt-audit-trail-immutable-evidence-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_endpoint_blocks_evidence \
  -- --nocapture >"$TEST_LOG"

LIVE_JSON='{}'
live_checked=false
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_JSON="$(
    curl -fsS \
      "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-audit-trail-immutable-evidence-denial"
  )"
  jq -e --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" '
    .runtime == "hepta"
    and .status == "ready"
    and .route_count == $expected_route_count
    and .implemented_route_count == $expected_route_count
    and .missing_route_count == 0
    and .native_gateway_source_command_count == $expected_route_count
    and .runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_route_enabled == true
    and .runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready == true
    and .runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status == "blocked"
    and .source_activation_command_result_receipt_cancellation_supersession_ready == true
    and .audit_trail_immutable_evidence_surface_count == 12
    and .audit_trail_immutable_evidence_fixture_count == 10
    and .blocked_audit_trail_immutable_evidence_fixture_count == 10
    and .accepted_audit_trail_immutable_evidence_fixture_count == 0
    and .audit_trail_performed_count == 0
    and .immutable_evidence_performed_count == 0
    and .hash_chain_recorded_count == 0
    and .merkle_root_recorded_count == 0
    and .attestation_recorded_count == 0
    and .activation_command_result_receipt_audit_trail_recorded == false
    and .activation_command_result_receipt_immutable_evidence_recorded == false
    and .activation_command_result_receipt_hash_chain_recorded == false
    and .activation_from_audit_trail_allowed == false
    and .activation_from_immutable_evidence_allowed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .secret_file_read == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .external_send_performed == false
    and .service_restart_performed == false
    and .active_binary_mutated == false
    and (.denied_by_audit_trail_immutable_evidence | length) == 24
    and (.side_effects | to_entries | all(.value == false))
  ' >/dev/null <<<"$LIVE_JSON"
  live_checked=true
fi

native_gateway_sha256="$(shasum -a 256 "$NATIVE_GATEWAY_SOURCE" | awk '{print $1}')"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg status "ready" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-audit-trail-immutable-evidence-denial" \
  --arg source_command "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-audit-trail-immutable-evidence-denial --json" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg focused_test_log "$TEST_LOG" \
  --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
  --argjson source "$SOURCE_JSON" \
  --argjson live "$LIVE_JSON" \
  --argjson live_checked "$live_checked" \
  '{
    product:$product,
    runtime:$runtime,
    status:$status,
    base_url:$base_url,
    gate:$gate,
    endpoint:$endpoint,
    source_command:$source_command,
    native_gateway_sha256:$native_gateway_sha256,
    focused_test_log:$focused_test_log,
    live_endpoint_checked:$live_checked,
    live_route_status:(if $live_checked then $live.status else "skipped" end),
    live_route_count:(if $live_checked then $live.route_count else 0 end),
    live_missing_route_count:(if $live_checked then $live.missing_route_count else 0 end),
    expected_route_count:$expected_route_count,
    source_activation_command_result_receipt_audit_trail_immutable_evidence_denial_gate:$source.gate,
    source_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready:$source.runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready,
    source_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status:$source.runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status,
    source_cancellation_supersession_ready:$source.source_activation_command_result_receipt_cancellation_supersession_ready,
    route_gate_ready:true,
    audit_trail_immutable_evidence_surface_count:$source.audit_trail_immutable_evidence_surface_count,
    audit_trail_immutable_evidence_fixture_count:$source.audit_trail_immutable_evidence_fixture_count,
    blocked_audit_trail_immutable_evidence_fixture_count:$source.blocked_audit_trail_immutable_evidence_fixture_count,
    accepted_audit_trail_immutable_evidence_fixture_count:$source.accepted_audit_trail_immutable_evidence_fixture_count,
    audit_trail_performed_count:$source.audit_trail_performed_count,
    immutable_evidence_performed_count:$source.immutable_evidence_performed_count,
    denied_by_audit_trail_immutable_evidence_count:($source.denied_by_audit_trail_immutable_evidence | length),
    next_slice:"runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial",
    side_effects:{
      activation_command_result_receipt_audit_trail_recorded:false,
      activation_command_result_receipt_immutable_evidence_recorded:false,
      activation_command_result_receipt_hash_chain_recorded:false,
      activation_command_result_receipt_merkle_root_recorded:false,
      activation_from_audit_trail_allowed:false,
      activation_from_immutable_evidence_allowed:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      secret_file_read:false,
      memory_store_write_performed:false,
      live_kg_write_performed:false,
      external_send_performed:false,
      service_restart_performed:false,
      active_binary_mutated:false
    }
  }'

echo "Hepta memory/intelligence/KG full enablement runtime provider-router activation command result receipt audit-trail/immutable-evidence denial route gate passed"
