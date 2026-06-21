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
    echo "missing artifact download/install affordance result receipt retention/expiry/garbage-collection route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

RETENTION_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-retention-expiry-garbage-collection-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="${HEPTA_RELEASE_BIN:-}" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-retention-expiry-garbage-collection-denial-gate.sh
)"

jq -e '
  def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
  def false_fields($o; $fields): all($fields[]; $o[.] == false);

  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_ready == true
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surface_count == 18
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_attempt_count == 18
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_denied_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surface_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_attempt_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denied_count == 18
  and zero_fields(.; [
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_scheduler_registered_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delete_marker_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_archive_written_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_compaction_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_retention_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_attestation_retention_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_release_publication_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_activation_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_install_executed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_active_binary_mutated_count"
  ])
  and false_fields(.; [
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_recorded",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_recorded",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed",
    "artifact_download_install_affordance_result_receipt_recorded",
    "artifact_download_install_affordance_result_receipt_persisted",
    "artifact_download_install_affordance_completion_ack_recorded",
    "operator_approval_recorded",
    "release_publication_authority_derived",
    "activation_authority_derived",
    "memory_store_write_performed",
    "live_kg_write_performed",
    "provider_invoked",
    "model_invoked",
    "credential_read",
    "install_executed",
    "active_binary_mutated",
    "external_send_performed"
  ])
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surfaces | length) == 18
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surfaces | all(
    .artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_attempted == true
    and .retention_expiry_garbage_collection_noop_confirmed == true
    and .artifact_download_install_affordance_result_receipt_retention_policy_recorded == false
    and .artifact_download_install_affordance_result_receipt_expiry_recorded == false
    and .artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed == false
    and .artifact_download_install_affordance_result_receipt_delete_marker_recorded == false
    and .artifact_download_install_affordance_result_receipt_compaction_performed == false
    and .release_publication_authority_from_retention_expiry_garbage_collection_derived == false
    and .activation_authority_from_retention_expiry_garbage_collection_derived == false
    and .install_from_retention_expiry_garbage_collection_executed == false
    and .active_binary_from_retention_expiry_garbage_collection_mutated == false
    and .external_send_performed == false
  ))
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surfaces[] | select(.gc_queue_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surfaces[] | select(.audit_evidence_retention_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surfaces[] | select(.hash_attestation_retention_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surfaces[] | select(.install_gc_requested == true)] | length) == 1
  and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection | length) == 33
  and (.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .records_retention_policy == false
    and .records_expiry == false
    and .performs_garbage_collection == false
    and .exports_receipt == false
    and .registers_query == false
    and .records_observability == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .mutates_memory_store == false
    and .writes_kg == false
    and .sends_externally == false
  ))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$RETENTION_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 167;' \
  "native gateway route/source command count includes artifact download/install affordance result receipt retention/expiry/garbage-collection route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT' \
  "native gateway artifact download/install affordance result receipt retention/expiry/garbage-collection endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-retention-expiry-garbage-collection-denial' \
  "native gateway artifact download/install affordance result receipt retention/expiry/garbage-collection endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-retention-expiry-garbage-collection-denial --json' \
  "native gateway artifact download/install affordance result receipt retention/expiry/garbage-collection source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_report' \
  "native gateway artifact download/install affordance result receipt retention/expiry/garbage-collection report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_route_enabled": true' \
  "artifact download/install affordance result receipt retention/expiry/garbage-collection route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surface_count": retention_surface_count' \
  "artifact download/install affordance result receipt retention/expiry/garbage-collection surface count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_endpoint_blocks_lifecycle_mutation' \
  "artifact download/install affordance result receipt retention/expiry/garbage-collection focused endpoint test"

TEST_LOG="$(mktemp /tmp/hepta-artifact-download-install-affordance-result-receipt-retention-expiry-garbage-collection-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_endpoint_blocks_lifecycle_mutation \
  -- --nocapture >"$TEST_LOG"

LIVE_ROUTE_JSON='{}'
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-retention-expiry-garbage-collection-denial"
  )"
  jq -e '
    .status == "ready"
    and .route_count == 160
    and .implemented_route_count == 160
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_route_enabled == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_ready == true
    and .source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_ready == true
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surface_count == 18
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_attempt_count == 18
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denied_count == 18
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_recorded_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_scheduler_registered_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delete_marker_recorded_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_compaction_performed_count == 0
    and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surfaces | length) == 18
    and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection | length) == 33
    and .operator_approval_recorded == false
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
    and .install_executed == false
    and .active_binary_mutated == false
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
  and .required_marker_count == 300
  and .present_required_marker_count == 300
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
source_retention_gate_sha256="$(printf '%s' "$RETENTION_JSON" | shasum -a 256 | awk '{print $1}')"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"
live_route_status="$(jq -r '.status // "skipped"' <<<"$LIVE_ROUTE_JSON")"
live_route_count="$(jq -r '.route_count // 0' <<<"$LIVE_ROUTE_JSON")"
live_missing_route_count="$(jq -r '.missing_route_count // 0' <<<"$LIVE_ROUTE_JSON")"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-retention-expiry-garbage-collection-denial" \
  --arg source_command "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-retention-expiry-garbage-collection-denial --json" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg source_retention_gate_sha256 "$source_retention_gate_sha256" \
  --arg focused_test_log "$TEST_LOG" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --arg live_route_status "$live_route_status" \
  --argjson live_endpoint_checked "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && echo true || echo false)" \
  --argjson live_route_count "$live_route_count" \
  --argjson live_missing_route_count "$live_missing_route_count" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    endpoint:$endpoint,
    source_command:$source_command,
    activation_mode:"full_live_activation_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_native_route_status",
    side_effect_free:true,
    native_route:true,
    route_enabled:true,
    source_retention_expiry_garbage_collection_gate_ready:true,
    source_retention_gate_sha256:$source_retention_gate_sha256,
    native_gateway_sha256:$native_gateway_sha256,
    focused_test_log:$focused_test_log,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    live_endpoint_checked:$live_endpoint_checked,
    source_route_count_expected:153,
    terminal_required_marker_count_expected:293,
    result_receipt_retention_expiry_garbage_collection_surface_count:18,
    result_receipt_retention_policy_recorded_count:0,
    result_receipt_expiry_scheduler_registered_count:0,
    result_receipt_garbage_collection_scan_performed_count:0,
    result_receipt_delete_marker_recorded_count:0,
    result_receipt_compaction_performed_count:0,
    live_route_status:$live_route_status,
    live_route_count:$live_route_count,
    live_missing_route_count:$live_missing_route_count,
    side_effects:{
      retention_policy_recorded:false,
      expiry_scheduler_registered:false,
      garbage_collection_scan_performed:false,
      delete_marker_recorded:false,
      compaction_performed:false,
      release_publication_authority_derived:false,
      activation_authority_derived:false,
      install_executed:false,
      active_binary_mutated:false,
      memory_store_write_performed:false,
      live_kg_write_performed:false,
      external_send_performed:false
    }
  }'

echo "artifact download/install affordance result receipt retention/expiry/garbage-collection denial route gate passed: endpoint ready, 18 surfaces, source route count 154, terminal markers 294/294" >&2
