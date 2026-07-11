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
    echo "missing artifact signing receipt no-persistence route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

SOURCE_GATE_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-no-persistence-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-no-persistence-denial-gate.sh
)"

jq -e '
  def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
  def false_fields($o; $fields): all($fields[]; $o[.] == false);

  .runtime == "hepta"
  and .status == "ready"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_result_receipt_no_persistence_denial_ready == true
  and .source_artifact_distribution_signing_notarization_surface_ready == true
  and .source_artifact_distribution_signing_notarization_surface_count == 18
  and .source_artifact_signing_executed_count == 0
  and .source_package_signing_executed_count == 0
  and .source_notarization_submitted_count == 0
  and .source_release_publication_authority_from_signing_status_derived_count == 0
  and .source_activation_authority_from_signing_status_derived_count == 0
  and .artifact_distribution_signing_notarization_result_receipt_surface_count == 18
  and .artifact_distribution_signing_notarization_result_receipt_surface_attempt_count == 18
  and .artifact_distribution_signing_notarization_result_receipt_surface_denied_count == 18
  and (.denied_by_artifact_distribution_signing_notarization_result_receipt | length) == 31
  and zero_fields(.; [
    "artifact_distribution_signing_notarization_result_receipt_surface_allowed_count",
    "artifact_distribution_signing_notarization_result_receipt_surface_request_accepted_count",
    "artifact_distribution_signing_notarization_result_receipt_surface_accepted_count",
    "artifact_distribution_signing_notarization_result_receipt_surface_recorded_count",
    "artifact_distribution_signing_notarization_result_receipt_surface_persisted_count",
    "artifact_distribution_signing_notarization_result_receipt_surface_materialized_count",
    "artifact_distribution_signing_notarization_result_receipt_surface_filesystem_written_count",
    "artifact_distribution_signing_notarization_result_receipt_surface_delivered_count",
    "artifact_distribution_signing_notarization_result_receipt_surface_indexed_count",
    "artifact_distribution_signing_notarization_result_receipt_surface_exported_count",
    "artifact_distribution_signing_notarization_result_receipt_surface_query_registered_count",
    "artifact_distribution_signing_notarization_result_receipt_surface_observability_recorded_count",
    "artifact_distribution_signing_notarization_result_receipt_surface_status_exposed_count",
    "artifact_signing_receipt_accepted_count",
    "package_signing_receipt_accepted_count",
    "notarization_submission_receipt_persisted_count",
    "stapling_receipt_filesystem_written_count",
    "installer_signing_receipt_delivered_count",
    "release_publication_authority_from_signing_receipt_derived_count",
    "activation_authority_from_signing_receipt_derived_count",
    "install_from_signing_receipt_executed_count",
    "service_restart_from_signing_receipt_performed_count",
    "active_binary_from_signing_receipt_mutated_count",
    "memory_store_write_performed_count",
    "live_kg_write_performed_count",
    "provider_invoked_count",
    "model_invoked_count",
    "credential_read_count",
    "secret_file_read_count",
    "external_send_performed_count"
  ])
  and false_fields(.; [
    "artifact_distribution_signing_notarization_result_receipt_accepted",
    "artifact_distribution_signing_notarization_result_receipt_recorded",
    "artifact_distribution_signing_notarization_result_receipt_persisted",
    "artifact_distribution_signing_notarization_result_receipt_materialized",
    "artifact_distribution_signing_notarization_result_receipt_delivered",
    "artifact_distribution_signing_notarization_result_receipt_status_exposed",
    "artifact_signing_receipt_accepted",
    "package_signing_receipt_accepted",
    "notarization_submission_receipt_persisted",
    "stapling_receipt_filesystem_written",
    "installer_signing_receipt_delivered",
    "operator_acceptance_recorded",
    "operator_approval_recorded",
    "release_publication_authority_derived",
    "activation_authority_derived",
    "activation_allowed",
    "activation_performed",
    "memory_store_write_performed",
    "memory_store_mutated",
    "live_kg_write_performed",
    "provider_invoked",
    "model_invoked",
    "credential_read",
    "secret_file_read",
    "install_executed",
    "launchd_mutated",
    "service_restarted",
    "active_binary_mutated",
    "external_send_performed"
  ])
  and (.artifact_distribution_signing_notarization_result_receipt_surfaces | length) == 18
  and (.artifact_distribution_signing_notarization_result_receipt_surfaces | all(
    .artifact_distribution_signing_notarization_result_receipt_surface_attempted == true
    and .artifact_distribution_signing_notarization_result_receipt_surface_allowed == false
    and .artifact_distribution_signing_notarization_result_receipt_surface_request_accepted == false
    and .artifact_distribution_signing_notarization_result_receipt_surface_accepted == false
    and .artifact_distribution_signing_notarization_result_receipt_surface_recorded == false
    and .artifact_distribution_signing_notarization_result_receipt_surface_persisted == false
    and .artifact_distribution_signing_notarization_result_receipt_surface_materialized == false
    and .artifact_distribution_signing_notarization_result_receipt_surface_filesystem_written == false
    and .artifact_distribution_signing_notarization_result_receipt_surface_delivered == false
    and .artifact_distribution_signing_notarization_result_receipt_surface_status_exposed == false
    and .artifact_distribution_signing_notarization_result_receipt_surface_noop_confirmed == true
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .secret_file_read == false
    and .external_send_performed == false
  ))
  and ([.artifact_distribution_signing_notarization_result_receipt_surfaces[] | select(.artifact_signing_receipt_acceptance_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_result_receipt_surfaces[] | select(.notarization_submission_receipt_persistence_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_result_receipt_surfaces[] | select(.telegram_signing_receipt_delivery_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_result_receipt_surfaces[] | select(.install_restart_active_binary_from_signing_receipt_requested == true)] | length) == 1
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$SOURCE_GATE_JSON"

source "$REPO_ROOT/scripts/lib/hepta-source-set.sh"
NATIVE_GATEWAY_SOURCE="hepta-native-gateway-source-set-v1"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes artifact signing receipt route"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  "HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_NO_PERSISTENCE_DENIAL_ENDPOINT" \
  "artifact signing receipt endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  "/api/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-no-persistence-denial" \
  "artifact signing receipt endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-no-persistence-denial --json" \
  "artifact signing receipt source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_denial_report" \
  "artifact signing receipt report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_denial_route_enabled\": true" \
  "artifact signing receipt route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"artifact_distribution_signing_notarization_result_receipt_surface_count\": surface_count" \
  "artifact signing receipt surface count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_no_persistence_endpoint_blocks_receipts" \
  "artifact signing receipt focused endpoint test"

TEST_LOG="$(mktemp /tmp/hepta-artifact-signing-receipt-no-persistence-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_no_persistence_endpoint_blocks_receipts \
  -- --nocapture >"$TEST_LOG"

LIVE_ROUTE_JSON='{}'
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-no-persistence-denial"
  )"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .implemented_route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_denial_route_enabled == true
    and .memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_denial_ready == true
    and .source_operator_intent_consent_evidence_persistence_ready == true
    and .source_artifact_distribution_signing_notarization_surface_ready == true
    and .artifact_distribution_signing_notarization_result_receipt_surface_count == 18
    and .artifact_distribution_signing_notarization_result_receipt_surface_denied_count == 18
    and (.denied_by_artifact_distribution_signing_notarization_result_receipt | length) == 31
    and .artifact_distribution_signing_notarization_result_receipt_accepted == false
    and .artifact_distribution_signing_notarization_result_receipt_persisted == false
    and .artifact_distribution_signing_notarization_result_receipt_status_exposed == false
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .secret_file_read == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .external_send_performed == false
    and (.side_effects | to_entries | all(.value == false))
  ' >/dev/null <<<"$LIVE_ROUTE_JSON"
fi

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
source_gate_sha256="$(printf '%s' "$SOURCE_GATE_JSON" | shasum -a 256 | awk '{print $1}')"
live_route_status="$(jq -r '.status // "skipped"' <<<"$LIVE_ROUTE_JSON")"
live_route_count="$(jq -r '.route_count // 0' <<<"$LIVE_ROUTE_JSON")"
live_missing_route_count="$(jq -r '.missing_route_count // 0' <<<"$LIVE_ROUTE_JSON")"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_no_persistence_denial_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-no-persistence-denial" \
  --arg source_command "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-no-persistence-denial --json" \
  --arg source_gate_sha256 "$source_gate_sha256" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg focused_test_log "$TEST_LOG" \
  --arg live_route_status "$live_route_status" \
  --argjson live_endpoint_checked "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && echo true || echo false)" \
  --argjson live_route_count "$live_route_count" \
  --argjson live_missing_route_count "$live_missing_route_count" \
  --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    endpoint:$endpoint,
    source_command:$source_command,
    activation_mode:"artifact_signing_receipt_no_persistence_native_route_status",
    side_effect_free:true,
    native_route:true,
    route_enabled:true,
    source_artifact_signing_receipt_no_persistence_gate_ready:true,
    source_gate_sha256:$source_gate_sha256,
    native_gateway_sha256:$native_gateway_sha256,
    focused_test_log:$focused_test_log,
    live_endpoint_checked:$live_endpoint_checked,
    expected_route_count:$expected_route_count,
    route_source_texts_ready:true,
    artifact_distribution_signing_notarization_result_receipt_surface_count:18,
    artifact_distribution_signing_notarization_result_receipt_surface_denied_count:18,
    artifact_distribution_signing_notarization_result_receipt_recorded_count:0,
    artifact_distribution_signing_notarization_result_receipt_persisted_count:0,
    artifact_distribution_signing_notarization_result_receipt_materialized_count:0,
    artifact_distribution_signing_notarization_result_receipt_delivered_count:0,
    artifact_distribution_signing_notarization_result_receipt_status_exposed_count:0,
    artifact_signing_receipt_accepted_count:0,
    package_signing_receipt_accepted_count:0,
    release_publication_authority_from_signing_receipt_derived_count:0,
    activation_authority_from_signing_receipt_derived_count:0,
    install_from_signing_receipt_executed_count:0,
    service_restart_from_signing_receipt_performed_count:0,
    active_binary_from_signing_receipt_mutated_count:0,
    live_route_status:$live_route_status,
    live_route_count:$live_route_count,
    live_missing_route_count:$live_missing_route_count,
    side_effects:{
      route_gate_filesystem_written:false,
      route_gate_runtime_mutated:false,
      route_gate_service_restarted:false,
      route_gate_external_send_performed:false,
      source_gate_side_effects:{
        artifact_distribution_signing_notarization_result_receipt_recorded:false,
        artifact_distribution_signing_notarization_result_receipt_persisted:false,
        artifact_distribution_signing_notarization_result_receipt_status_exposed:false,
        release_publication_authority_derived:false,
        activation_authority_derived:false,
        install_executed:false,
        service_restarted:false,
        active_binary_mutated:false,
        memory_store_write_performed:false,
        live_kg_write_performed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_read:false,
        secret_file_read:false,
        external_send_performed:false
      }
    }
  }'

echo "Hepta artifact signing receipt no-persistence route gate passed"
