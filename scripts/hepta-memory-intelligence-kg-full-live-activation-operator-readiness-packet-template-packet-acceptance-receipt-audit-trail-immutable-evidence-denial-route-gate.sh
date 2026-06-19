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
    echo "missing operator readiness packet template packet-acceptance receipt audit-trail/immutable-evidence route source text: $label" >&2
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
    "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-audit-trail-immutable-evidence-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-audit-trail-immutable-evidence-denial-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_ready == true
  and .source_packet_acceptance_receipt_cancellation_supersession_ready == true
  and .source_cancellation_supersession_surface_count == 14
  and .source_cancellation_supersession_attempt_count == 14
  and .source_cancellation_accepted_count == 0
  and .source_supersession_accepted_count == 0
  and .source_replacement_receipt_accepted_count == 0
  and .source_tombstone_recorded_count == 0
  and .source_cancellation_supersession_acceptance_recorded_count == 0
  and .source_cancellation_supersession_activation_authority_derived_count == 0
  and .audit_evidence_surface_count == 16
  and .audit_evidence_attempt_count == 16
  and .audit_trail_accepted_count == 0
  and .audit_trail_recorded_count == 0
  and .audit_trail_persisted_count == 0
  and .audit_trail_materialized_count == 0
  and .immutable_evidence_accepted_count == 0
  and .immutable_evidence_recorded_count == 0
  and .immutable_evidence_persisted_count == 0
  and .immutable_evidence_materialized_count == 0
  and .hash_chain_recorded_count == 0
  and .hash_chain_persisted_count == 0
  and .merkle_root_recorded_count == 0
  and .merkle_root_persisted_count == 0
  and .attestation_recorded_count == 0
  and .attestation_persisted_count == 0
  and .witness_recorded_count == 0
  and .witness_persisted_count == 0
  and .notary_recorded_count == 0
  and .notary_persisted_count == 0
  and .ledger_evidence_recorded_count == 0
  and .ledger_evidence_persisted_count == 0
  and .index_evidence_recorded_count == 0
  and .index_evidence_persisted_count == 0
  and .delivery_evidence_recorded_count == 0
  and .delivery_evidence_persisted_count == 0
  and .export_evidence_recorded_count == 0
  and .query_evidence_registered_count == 0
  and .observability_evidence_recorded_count == 0
  and .readback_evidence_recorded_count == 0
  and .audit_evidence_acceptance_recorded_count == 0
  and .audit_evidence_operator_approval_derived_count == 0
  and .audit_evidence_activation_authority_derived_count == 0
  and .audit_evidence_activation_command_derived_count == 0
  and .audit_evidence_live_execution_allowed_count == 0
  and (.audit_surfaces | all(
    .audit_or_evidence_attempted == true
    and .audit_trail_accepted == false
    and .audit_trail_recorded == false
    and .audit_trail_persisted == false
    and .immutable_evidence_accepted == false
    and .immutable_evidence_recorded == false
    and .immutable_evidence_persisted == false
    and .hash_chain_recorded == false
    and .merkle_root_recorded == false
    and .attestation_recorded == false
    and .witness_recorded == false
    and .notary_recorded == false
    and .ledger_evidence_recorded == false
    and .index_evidence_recorded == false
    and .delivery_evidence_recorded == false
    and .export_evidence_recorded == false
    and .query_evidence_registered == false
    and .observability_evidence_recorded == false
    and .readback_evidence_recorded == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .live_execution_allowed == false
    and .audit_evidence_status == "audit_trail_immutable_evidence_denied"
  ))
  and (.denied_by_packet_receipt_audit_trail_immutable_evidence | length) == 21
  and .packet_acceptance_receipt_audit_trail_accepted == false
  and .packet_acceptance_receipt_audit_trail_recorded == false
  and .packet_acceptance_receipt_audit_trail_persisted == false
  and .packet_acceptance_receipt_immutable_evidence_accepted == false
  and .packet_acceptance_receipt_immutable_evidence_recorded == false
  and .packet_acceptance_receipt_immutable_evidence_persisted == false
  and .packet_acceptance_receipt_hash_chain_recorded == false
  and .packet_acceptance_receipt_merkle_root_recorded == false
  and .packet_acceptance_receipt_attestation_recorded == false
  and .packet_acceptance_receipt_ledger_evidence_recorded == false
  and .packet_acceptance_receipt_readback_evidence_recorded == false
  and .operator_acceptance_recorded == false
  and .operator_approval_recorded == false
  and .activation_authority_derived == false
  and .activation_command_derived == false
  and .activation_allowed == false
  and .activation_performed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$AUDIT_EVIDENCE_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 150;' \
  "native gateway route/source command count includes operator readiness packet acceptance receipt audit-trail/immutable-evidence route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT' \
  "native gateway operator readiness packet acceptance receipt audit-trail/immutable-evidence endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-audit-trail-immutable-evidence-denial' \
  "native gateway operator readiness packet acceptance receipt audit-trail/immutable-evidence endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-audit-trail-immutable-evidence-denial --json' \
  "native gateway operator readiness packet acceptance receipt audit-trail/immutable-evidence source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_report' \
  "native gateway operator readiness packet acceptance receipt audit-trail/immutable-evidence report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_route_enabled": true' \
  "operator readiness packet acceptance receipt audit-trail/immutable-evidence route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"audit_evidence_surface_count": audit_evidence_surface_count' \
  "packet acceptance receipt audit-trail/immutable-evidence surface count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"packet_acceptance_receipt_audit_trail_recorded": false' \
  "packet acceptance receipt audit-trail recording denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"packet_acceptance_receipt_immutable_evidence_recorded": false' \
  "packet acceptance receipt immutable-evidence recording denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"packet_acceptance_receipt_hash_chain_recorded": false' \
  "packet acceptance receipt hash-chain recording denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-readiness-packet-template-packet-acceptance-receipt-audit-trail-immutable-evidence-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_endpoint_blocks_evidence \
  -- --nocapture >"$TEST_LOG"

LIVE_ROUTE_JSON='{}'
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-audit-trail-immutable-evidence-denial"
  )"
  jq -e '
    .status == "ready"
    and .route_count == 150
    and .implemented_route_count == 150
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_route_enabled == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_ready == true
    and .source_packet_acceptance_receipt_cancellation_supersession_ready == true
    and .source_cancellation_supersession_surface_count == 14
    and .source_cancellation_supersession_attempt_count == 14
    and .source_cancellation_accepted_count == 0
    and .source_supersession_accepted_count == 0
    and .source_replacement_receipt_accepted_count == 0
    and .source_tombstone_recorded_count == 0
    and .audit_evidence_surface_count == 16
    and .audit_evidence_attempt_count == 16
    and .audit_trail_accepted_count == 0
    and .audit_trail_recorded_count == 0
    and .audit_trail_persisted_count == 0
    and .immutable_evidence_accepted_count == 0
    and .immutable_evidence_recorded_count == 0
    and .immutable_evidence_persisted_count == 0
    and .hash_chain_recorded_count == 0
    and .merkle_root_recorded_count == 0
    and .attestation_recorded_count == 0
    and .ledger_evidence_recorded_count == 0
    and .readback_evidence_recorded_count == 0
    and .audit_evidence_activation_authority_derived_count == 0
    and .audit_evidence_activation_command_derived_count == 0
    and .audit_evidence_live_execution_allowed_count == 0
    and (.audit_surfaces | all(
      .audit_or_evidence_attempted == true
      and .audit_trail_recorded == false
      and .immutable_evidence_recorded == false
      and .hash_chain_recorded == false
      and .activation_authority_derived == false
      and .live_execution_allowed == false
      and .audit_evidence_status == "audit_trail_immutable_evidence_denied"
    ))
    and (.denied_by_packet_receipt_audit_trail_immutable_evidence | length) == 21
    and .packet_acceptance_receipt_audit_trail_recorded == false
    and .packet_acceptance_receipt_immutable_evidence_recorded == false
    and .packet_acceptance_receipt_hash_chain_recorded == false
    and .operator_acceptance_recorded == false
    and .operator_approval_recorded == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .activation_allowed == false
    and .activation_performed == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .install_executed == false
    and .service_restarted == false
    and .external_send_performed == false
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
  and .required_marker_count == 290
  and .present_required_marker_count == 290
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
source_audit_evidence_gate_sha256="$(printf '%s' "$AUDIT_EVIDENCE_JSON" | shasum -a 256 | awk '{print $1}')"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-audit-trail-immutable-evidence-denial" \
  --arg source_command "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-audit-trail-immutable-evidence-denial --json" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg source_audit_evidence_gate_sha256 "$source_audit_evidence_gate_sha256" \
  --arg test_log "$TEST_LOG" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --argjson source "$AUDIT_EVIDENCE_JSON" \
  --argjson terminal "$TERMINAL_COVERAGE_JSON" \
  --argjson live "$LIVE_ROUTE_JSON" \
  --argjson live_checked "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && echo true || echo false)" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:"hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_route_gate",
    endpoint:$endpoint,
    source_command:$source_command,
    activation_mode:"full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_native_route_status",
    side_effect_free:true,
    native_route:true,
    route_enabled:true,
    source_packet_acceptance_receipt_audit_trail_immutable_evidence_gate_ready:true,
    source_audit_evidence_gate_sha256:$source_audit_evidence_gate_sha256,
    native_gateway_sha256:$native_gateway_sha256,
    focused_test_log:$test_log,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    live_endpoint_checked:$live_checked,
    source_route_count_expected:150,
    terminal_required_marker_count_expected:290,
    source_packet_acceptance_receipt_cancellation_supersession_ready:$source.source_packet_acceptance_receipt_cancellation_supersession_ready,
    audit_evidence_surface_count:$source.audit_evidence_surface_count,
    audit_evidence_attempt_count:$source.audit_evidence_attempt_count,
    audit_trail_recorded_count:$source.audit_trail_recorded_count,
    immutable_evidence_recorded_count:$source.immutable_evidence_recorded_count,
    hash_chain_recorded_count:$source.hash_chain_recorded_count,
    merkle_root_recorded_count:$source.merkle_root_recorded_count,
    attestation_recorded_count:$source.attestation_recorded_count,
    ledger_evidence_recorded_count:$source.ledger_evidence_recorded_count,
    readback_evidence_recorded_count:$source.readback_evidence_recorded_count,
    audit_evidence_activation_authority_derived_count:$source.audit_evidence_activation_authority_derived_count,
    audit_evidence_live_execution_allowed_count:$source.audit_evidence_live_execution_allowed_count,
    route_source_texts_ready:true,
    terminal_coverage_ready:true,
    terminal_required_marker_count:$terminal.required_marker_count,
    terminal_present_required_marker_count:$terminal.present_required_marker_count,
    terminal_missing_required_marker_count:$terminal.missing_required_marker_count,
    live_route_status:($live.status // null),
    live_route_count:($live.route_count // null),
    live_missing_route_count:($live.missing_route_count // null),
    side_effects:{
      route_gate_filesystem_written:false,
      route_gate_runtime_mutated:false,
      route_gate_service_restarted:false,
      route_gate_external_send_performed:false,
      source_gate_side_effects:$source.side_effects,
      live_route_side_effects:($live.side_effects // {})
    }
  }'

echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt audit-trail/immutable-evidence denial route gate passed"
