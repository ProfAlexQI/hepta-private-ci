#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"
EXPECTED_ROUTE_COUNT=192

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
    echo "missing operator readiness packet template packet-acceptance receipt redaction/privacy route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

REDACTION_PRIVACY_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-redaction-privacy-payload-exposure-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-redaction-privacy-payload-exposure-denial-gate.sh
)"

jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_ready == true
  and .source_packet_acceptance_receipt_export_query_observability_ready == true
  and .source_export_query_observability_surface_count == 16
  and .source_query_registered_count == 0
  and .source_export_snapshot_recorded_count == 0
  and .source_observability_metric_recorded_count == 0
  and .source_operator_summary_recorded_count == 0
  and .source_readback_surface_recorded_count == 0
  and .source_export_query_observability_activation_authority_derived_count == 0
  and .redaction_privacy_surface_count == 16
  and .redaction_privacy_attempt_count == 16
  and .redacted_payload_preview_recorded_count == 0
  and .payload_hash_preview_recorded_count == 0
  and .payload_diff_recorded_count == 0
  and .readback_text_recorded_count == 0
  and .operator_summary_text_recorded_count == 0
  and .privacy_review_recorded_count == 0
  and .privacy_review_persisted_count == 0
  and .secret_scan_performed_count == 0
  and .pii_scan_performed_count == 0
  and .raw_payload_inspected_count == 0
  and .plaintext_materialized_count == 0
  and .redaction_bypass_allowed_count == 0
  and .hash_to_payload_link_recorded_count == 0
  and .external_redaction_review_performed_count == 0
  and .privacy_acceptance_recorded_count == 0
  and .redaction_privacy_acceptance_recorded_count == 0
  and .redaction_privacy_operator_approval_derived_count == 0
  and .redaction_privacy_activation_authority_derived_count == 0
  and .redaction_privacy_activation_command_derived_count == 0
  and .redaction_privacy_live_execution_allowed_count == 0
  and (.redaction_privacy_surfaces | all(
    .redaction_privacy_or_payload_exposure_attempted == true
    and .redacted_payload_preview_recorded == false
    and .payload_hash_preview_recorded == false
    and .payload_diff_recorded == false
    and .readback_text_recorded == false
    and .operator_summary_text_recorded == false
    and .privacy_review_recorded == false
    and .secret_scan_performed == false
    and .pii_scan_performed == false
    and .raw_payload_inspected == false
    and .plaintext_materialized == false
    and .redaction_bypass_allowed == false
    and .hash_to_payload_link_recorded == false
    and .external_redaction_review_performed == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .live_execution_allowed == false
    and .redaction_privacy_status == "redaction_privacy_payload_exposure_denied"
  ))
  and (.denied_by_packet_receipt_redaction_privacy | length) == 17
  and .packet_acceptance_receipt_redacted_payload_preview_recorded == false
  and .packet_acceptance_receipt_payload_hash_preview_recorded == false
  and .packet_acceptance_receipt_readback_text_recorded == false
  and .packet_acceptance_receipt_operator_summary_text_recorded == false
  and .packet_acceptance_receipt_privacy_review_recorded == false
  and .packet_acceptance_receipt_secret_scan_performed == false
  and .packet_acceptance_receipt_raw_payload_inspected == false
  and .packet_acceptance_receipt_plaintext_materialized == false
  and .packet_acceptance_receipt_hash_to_payload_link_recorded == false
  and .operator_acceptance_recorded == false
  and .operator_approval_recorded == false
  and .activation_authority_derived == false
  and .activation_command_derived == false
  and .activation_allowed == false
  and .activation_performed == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .hepta_intelligence_context_attached == false
  and .context_injection_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .external_kg_adapter_read_performed == false
  and .network_call_performed == false
  and .external_db_write_performed == false
  and .live_kg_write_performed == false
  and .credential_read == false
  and .secret_file_read == false
  and .install_executed == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and .external_send_performed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$REDACTION_PRIVACY_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 192;' \
  "native gateway route/source command count includes operator readiness packet acceptance receipt redaction/privacy route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_REDACTION_PRIVACY_PAYLOAD_EXPOSURE_DENIAL_ENDPOINT' \
  "native gateway operator readiness packet acceptance receipt redaction/privacy endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-redaction-privacy-payload-exposure-denial' \
  "native gateway operator readiness packet acceptance receipt redaction/privacy endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-redaction-privacy-payload-exposure-denial --json' \
  "native gateway operator readiness packet acceptance receipt redaction/privacy source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_report' \
  "native gateway operator readiness packet acceptance receipt redaction/privacy report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_route_enabled": true' \
  "operator readiness packet acceptance receipt redaction/privacy route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"redaction_privacy_surface_count": redaction_privacy_surface_count' \
  "packet acceptance receipt redaction/privacy surface count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"packet_acceptance_receipt_redacted_payload_preview_recorded": false' \
  "packet acceptance receipt redacted payload preview denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"packet_acceptance_receipt_plaintext_materialized": false' \
  "packet acceptance receipt plaintext materialization denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"packet_acceptance_receipt_hash_to_payload_link_recorded": false' \
  "packet acceptance receipt hash-to-payload link denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-readiness-packet-template-packet-acceptance-receipt-redaction-privacy-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_endpoint_blocks_payload_exposure \
  -- --nocapture >"$TEST_LOG"

LIVE_ROUTE_JSON='{}'
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-redaction-privacy-payload-exposure-denial"
  )"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .implemented_route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_route_enabled == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_ready == true
    and .source_packet_acceptance_receipt_export_query_observability_ready == true
    and .source_export_query_observability_surface_count == 16
    and .source_query_registered_count == 0
    and .source_export_snapshot_recorded_count == 0
    and .source_observability_metric_recorded_count == 0
    and .redaction_privacy_surface_count == 16
    and .redaction_privacy_attempt_count == 16
    and .redacted_payload_preview_recorded_count == 0
    and .payload_hash_preview_recorded_count == 0
    and .privacy_review_recorded_count == 0
    and .secret_scan_performed_count == 0
    and .raw_payload_inspected_count == 0
    and .plaintext_materialized_count == 0
    and .hash_to_payload_link_recorded_count == 0
    and .redaction_privacy_activation_authority_derived_count == 0
    and .redaction_privacy_live_execution_allowed_count == 0
    and (.redaction_privacy_surfaces | all(
      .redaction_privacy_or_payload_exposure_attempted == true
      and .redacted_payload_preview_recorded == false
      and .privacy_review_recorded == false
      and .secret_scan_performed == false
      and .raw_payload_inspected == false
      and .plaintext_materialized == false
      and .hash_to_payload_link_recorded == false
      and .activation_authority_derived == false
      and .live_execution_allowed == false
      and .redaction_privacy_status == "redaction_privacy_payload_exposure_denied"
    ))
    and (.denied_by_packet_receipt_redaction_privacy | length) == 17
    and .packet_acceptance_receipt_redacted_payload_preview_recorded == false
    and .packet_acceptance_receipt_privacy_review_recorded == false
    and .packet_acceptance_receipt_secret_scan_performed == false
    and .packet_acceptance_receipt_plaintext_materialized == false
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

jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
  .status == "ready"
  and .preflight_terminal_coverage_inventory_ready == true
  and .required_marker_count == 300
  and .present_required_marker_count == 300
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
source_redaction_privacy_gate_sha256="$(printf '%s' "$REDACTION_PRIVACY_JSON" | shasum -a 256 | awk '{print $1}')"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-redaction-privacy-payload-exposure-denial" \
  --arg source_command "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-redaction-privacy-payload-exposure-denial --json" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg source_redaction_privacy_gate_sha256 "$source_redaction_privacy_gate_sha256" \
  --arg test_log "$TEST_LOG" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --argjson source "$REDACTION_PRIVACY_JSON" \
  --argjson terminal "$TERMINAL_COVERAGE_JSON" \
  --argjson live "$LIVE_ROUTE_JSON" \
  --argjson live_checked "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && echo true || echo false)" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:"hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_route_gate",
    endpoint:$endpoint,
    source_command:$source_command,
    activation_mode:"full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_native_route_status",
    side_effect_free:true,
    native_route:true,
    route_enabled:true,
    source_packet_acceptance_receipt_redaction_privacy_gate_ready:true,
    source_redaction_privacy_gate_sha256:$source_redaction_privacy_gate_sha256,
    native_gateway_sha256:$native_gateway_sha256,
    focused_test_log:$test_log,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    live_endpoint_checked:$live_checked,
    source_route_count_expected:153,
    terminal_required_marker_count_expected:293,
    source_packet_acceptance_receipt_export_query_observability_ready:$source.source_packet_acceptance_receipt_export_query_observability_ready,
    source_export_query_observability_surface_count:$source.source_export_query_observability_surface_count,
    redaction_privacy_surface_count:$source.redaction_privacy_surface_count,
    redaction_privacy_attempt_count:$source.redaction_privacy_attempt_count,
    redacted_payload_preview_recorded_count:$source.redacted_payload_preview_recorded_count,
    privacy_review_recorded_count:$source.privacy_review_recorded_count,
    secret_scan_performed_count:$source.secret_scan_performed_count,
    raw_payload_inspected_count:$source.raw_payload_inspected_count,
    plaintext_materialized_count:$source.plaintext_materialized_count,
    hash_to_payload_link_recorded_count:$source.hash_to_payload_link_recorded_count,
    redaction_privacy_activation_authority_derived_count:$source.redaction_privacy_activation_authority_derived_count,
    redaction_privacy_live_execution_allowed_count:$source.redaction_privacy_live_execution_allowed_count,
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

echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt redaction/privacy/payload-exposure denial route gate passed"
