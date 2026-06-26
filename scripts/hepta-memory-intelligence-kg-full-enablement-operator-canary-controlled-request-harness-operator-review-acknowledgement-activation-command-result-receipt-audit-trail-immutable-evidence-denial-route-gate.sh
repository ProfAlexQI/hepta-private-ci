#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
RELEASE_BIN="${HEPTA_RELEASE_BIN:-${HEPTA_CODEX_RELEASE_BIN:-$HOME/.local/opt/hepta/bin/hepta}}"
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
    echo "missing operator canary activation command result receipt audit-trail/immutable-evidence route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

AUDIT_EVIDENCE_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial-gate.sh
)"

jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_gate"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status == "blocked"
  and .source_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_gate"
  and .source_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_status == "blocked"
  and .source_cancellation_supersession_fixture_count == 10
  and .source_accepted_cancellation_supersession_fixture_count == 0
  and .source_cancellation_performed_count == 0
  and .source_supersession_performed_count == 0
  and .source_replacement_receipt_recorded_count == 0
  and .audit_trail_immutable_evidence_fixture_count == 10
  and .blocked_audit_trail_immutable_evidence_fixture_count == 10
  and .noop_audit_trail_immutable_evidence_fixture_count == 10
  and .allowed_audit_trail_immutable_evidence_fixture_count == 0
  and .accepted_audit_trail_immutable_evidence_fixture_count == 0
  and .audit_trail_performed_count == 0
  and .immutable_evidence_performed_count == 0
  and .hash_chain_recorded_count == 0
  and .merkle_root_recorded_count == 0
  and .attestation_recorded_count == 0
  and .witness_recorded_count == 0
  and .notary_recorded_count == 0
  and .ledger_evidence_recorded_count == 0
  and .index_evidence_recorded_count == 0
  and .delivery_evidence_recorded_count == 0
  and .activation_command_result_receipt_audit_trail_allowed == false
  and .activation_command_result_receipt_audit_trail_recorded == false
  and .activation_command_result_receipt_audit_trail_persisted == false
  and .activation_command_result_receipt_immutable_evidence_allowed == false
  and .activation_command_result_receipt_immutable_evidence_recorded == false
  and .activation_command_result_receipt_immutable_evidence_persisted == false
  and .activation_command_result_receipt_hash_chain_recorded == false
  and .activation_command_result_receipt_attestation_recorded == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .operator_approval_from_audit_trail_accepted == false
  and .operator_approval_from_immutable_evidence_accepted == false
  and .activation_from_audit_trail_allowed == false
  and .activation_from_immutable_evidence_allowed == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_request_accepted == false
  and .activation_request_executed == false
  and .dispatch_performed_count == 0
  and .execution_performed_count == 0
  and .context_injection_performed_count == 0
  and .provider_invoked_count == 0
  and .model_invoked_count == 0
  and .memory_store_write_performed_count == 0
  and .external_kg_adapter_read_performed_count == 0
  and .live_kg_write_performed_count == 0
  and .credential_read_count == 0
  and .secret_file_read_count == 0
  and .channel_send_performed_count == 0
  and .install_performed_count == 0
  and .service_restarted_count == 0
  and .active_binary_mutated_count == 0
  and .upstream_fetch_performed_count == 0
  and .upstream_merge_performed_count == 0
  and .canary_harness_armed == false
  and .canary_harness_executable == false
  and .canary_live_enabled == false
  and (.audit_trail_immutable_evidence_fixtures | length) == 10
  and (.audit_trail_immutable_evidence_fixtures | all(
    (.audit_evidence_status | startswith("blocked"))
    and .audit_trail_recorded == false
    and .audit_trail_persisted == false
    and .immutable_evidence_recorded == false
    and .immutable_evidence_persisted == false
    and .hash_chain_recorded == false
    and .attestation_recorded == false
    and .activation_command_result_receipt_accepted == false
    and .operator_approval_from_audit_trail_accepted == false
    and .operator_approval_from_immutable_evidence_accepted == false
    and .activation_from_audit_trail_allowed == false
    and .activation_from_immutable_evidence_allowed == false
    and .dispatch_performed == false
    and .execution_performed == false
    and .context_injection_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .external_kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .channel_send_performed == false
    and .install_performed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .upstream_fetch_performed == false
    and .upstream_merge_performed == false
    and .receipt_noop_confirmed == true
  ))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$AUDIT_EVIDENCE_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = ${EXPECTED_ROUTE_COUNT};" \
  "native gateway route/source command count includes activation command result receipt audit-trail/immutable-evidence denial route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT' \
  "native gateway activation command result receipt audit-trail/immutable-evidence endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial' \
  "native gateway activation command result receipt audit-trail/immutable-evidence endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial --json' \
  "native gateway activation command result receipt audit-trail/immutable-evidence source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_report' \
  "native gateway activation command result receipt audit-trail/immutable-evidence report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_route_enabled": true' \
  "activation command result receipt audit-trail/immutable-evidence route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_command_result_receipt_audit_trail_recorded"' \
  "activation command result receipt audit trail recording denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_command_result_receipt_immutable_evidence_recorded"' \
  "activation command result receipt immutable evidence recording denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_command_result_receipt_hash_chain_recorded"' \
  "activation command result receipt hash-chain recording denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_from_audit_trail_allowed"' \
  "activation from result receipt audit trail denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-canary-activation-command-result-receipt-audit-trail-immutable-evidence-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_endpoint_blocks_audit_evidence \
  -- --nocapture >"$TEST_LOG"

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial"
  )"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .source_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_route_ready == true
    and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_route_enabled == true
    and .audit_trail_immutable_evidence_fixture_count == 10
    and .blocked_audit_trail_immutable_evidence_fixture_count == 10
    and .allowed_audit_trail_immutable_evidence_fixture_count == 0
    and .audit_trail_performed_count == 0
    and .immutable_evidence_performed_count == 0
    and .hash_chain_recorded_count == 0
    and .activation_command_result_receipt_audit_trail_recorded == false
    and .activation_command_result_receipt_immutable_evidence_recorded == false
    and .activation_command_result_receipt_hash_chain_recorded == false
    and .activation_command_result_receipt_attestation_recorded == false
    and .operator_approval_from_audit_trail_accepted == false
    and .operator_approval_from_immutable_evidence_accepted == false
    and .activation_from_audit_trail_allowed == false
    and .activation_from_immutable_evidence_allowed == false
    and .provider_invoked_count == 0
    and .model_invoked_count == 0
    and .memory_store_write_performed_count == 0
    and .live_kg_write_performed_count == 0
    and .credential_read_count == 0
    and .secret_file_read_count == 0
    and .channel_send_performed_count == 0
    and .install_performed_count == 0
    and .service_restarted_count == 0
    and .active_binary_mutated_count == 0
    and (.side_effects | to_entries | all(.value == false))
  ' >/dev/null <<<"$LIVE_ROUTE_JSON"
fi

TERMINAL_COVERAGE_JSON="$(
  capture_json_report \
    "hepta-preflight-terminal-coverage-inventory-gate" \
    scripts/hepta-preflight-terminal-coverage-inventory-gate.sh
)"

jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
  .status == "ready"
  and .preflight_terminal_coverage_inventory_ready == true
  and .required_marker_count >= 300
  and .present_required_marker_count == .required_marker_count
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial" \
  --arg source_command "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial --json" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg test_log "$TEST_LOG" \
  --argjson source "$AUDIT_EVIDENCE_JSON" \
  --argjson live_required "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && echo true || echo false)" \
  --argjson live_ready "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && echo true || echo false)" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:"hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_route_gate",
    endpoint:$endpoint,
    source_command:$source_command,
    activation_mode:"operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_native_route_status",
    source_activation_command_result_receipt_audit_trail_immutable_evidence_denial_gate:$source.gate,
    source_activation_command_result_receipt_audit_trail_immutable_evidence_denial_gate_ready:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready,
    source_activation_command_result_receipt_audit_trail_immutable_evidence_denial_gate_status:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status,
    source_cancellation_supersession_fixture_count:$source.source_cancellation_supersession_fixture_count,
    source_accepted_cancellation_supersession_fixture_count:$source.source_accepted_cancellation_supersession_fixture_count,
    source_route_wired:true,
    source_route_count_expected:105,
    source_route_tested_by_native_gateway_unit_test:true,
    native_gateway_source:"codex-rs/cli/src/native_gateway.rs",
    native_gateway_sha256:$native_gateway_sha256,
    native_gateway_unit_test_log:$test_log,
    live_endpoint_required:$live_required,
    live_endpoint_ready:$live_ready,
    audit_trail_immutable_evidence_fixture_count:$source.audit_trail_immutable_evidence_fixture_count,
    blocked_audit_trail_immutable_evidence_fixture_count:$source.blocked_audit_trail_immutable_evidence_fixture_count,
    noop_audit_trail_immutable_evidence_fixture_count:$source.noop_audit_trail_immutable_evidence_fixture_count,
    accepted_audit_trail_immutable_evidence_fixture_count:$source.accepted_audit_trail_immutable_evidence_fixture_count,
    audit_trail_performed_count:$source.audit_trail_performed_count,
    immutable_evidence_performed_count:$source.immutable_evidence_performed_count,
    hash_chain_recorded_count:$source.hash_chain_recorded_count,
    attestation_recorded_count:$source.attestation_recorded_count,
    activation_command_result_receipt_audit_trail_recorded:$source.activation_command_result_receipt_audit_trail_recorded,
    activation_command_result_receipt_immutable_evidence_recorded:$source.activation_command_result_receipt_immutable_evidence_recorded,
    activation_command_result_receipt_hash_chain_recorded:$source.activation_command_result_receipt_hash_chain_recorded,
    activation_from_audit_trail_allowed:$source.activation_from_audit_trail_allowed,
    activation_from_immutable_evidence_allowed:$source.activation_from_immutable_evidence_allowed,
    activation_command_enabled:$source.activation_command_enabled,
    activation_command_invoked:$source.activation_command_invoked,
    activation_command_dispatched:$source.activation_command_dispatched,
    activation_request_recorded:$source.activation_request_recorded,
    activation_request_executed:$source.activation_request_executed,
    dispatch_performed_count:$source.dispatch_performed_count,
    execution_performed_count:$source.execution_performed_count,
    provider_invoked_count:$source.provider_invoked_count,
    model_invoked_count:$source.model_invoked_count,
    memory_store_write_performed_count:$source.memory_store_write_performed_count,
    external_kg_adapter_read_performed_count:$source.external_kg_adapter_read_performed_count,
    live_kg_write_performed_count:$source.live_kg_write_performed_count,
    credential_read_count:$source.credential_read_count,
    secret_file_read_count:$source.secret_file_read_count,
    channel_send_performed_count:$source.channel_send_performed_count,
    install_performed_count:$source.install_performed_count,
    service_restarted_count:$source.service_restarted_count,
    active_binary_mutated_count:$source.active_binary_mutated_count,
    current_live_enabled_lane_count:$source.current_live_enabled_lane_count,
    enablement_lane_count:$source.enablement_lane_count,
    ready_enablement_lane_count:$source.ready_enablement_lane_count,
    side_effects:$source.side_effects
  }'

echo "Hepta Memory/Intelligence/KG operator canary controlled request harness operator review acknowledgement activation command result receipt audit-trail/immutable-evidence denial route gate passed"
