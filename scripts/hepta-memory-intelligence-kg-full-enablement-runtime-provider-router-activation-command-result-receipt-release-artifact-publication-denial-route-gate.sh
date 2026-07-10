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
    echo "missing runtime provider-router activation command result receipt release artifact publication route source text: $label" >&2
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
      "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-release-artifact-publication-denial-gate" \
      scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-release-artifact-publication-denial-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_gate"
  and .activation_command_result_receipt_release_artifact_publication_mode == "runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial"
  and .runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
  and .minimum_required_samples >= 24
  and .required_activation_command_result_receipt_release_artifact_publication_surface_count == 12
  and .ready_activation_command_result_receipt_release_artifact_publication_surface_count == 12
  and .side_effect_free_activation_command_result_receipt_release_artifact_publication_surface_count == 12
  and .required_activation_command_result_receipt_release_artifact_publication_fixture_count == 10
  and .activation_command_result_receipt_release_artifact_publication_fixture_count == 10
  and .blocked_activation_command_result_receipt_release_artifact_publication_fixture_count == 10
  and .noop_activation_command_result_receipt_release_artifact_publication_fixture_count == 10
  and .allowed_activation_command_result_receipt_release_artifact_publication_fixture_count == 0
  and .accepted_activation_command_result_receipt_release_artifact_publication_fixture_count == 0
  and .activation_command_result_receipt_release_artifact_publication_performed_count == 0
  and .release_artifact_publication_allowed == false
  and .release_artifact_publication_requested == false
  and .release_artifact_publication_accepted == false
  and .release_artifact_publication_recorded == false
  and .release_artifact_publication_persisted == false
  and .release_artifact_publication_materialized == false
  and .release_artifact_filesystem_written == false
  and .release_artifact_written == false
  and .public_artifact_written == false
  and .artifact_signature_accepted == false
  and .artifact_notarization_accepted == false
  and .publication_queue_enqueued == false
  and .publication_manifest_written == false
  and .public_distribution_performed == false
  and .public_release_published == false
  and .public_ga_claimed == false
  and .public_claim_promoted == false
  and .public_version_tag_created == false
  and .release_notes_materialized == false
  and .changelog_materialized == false
  and .terminal_operator_decision_promoted_to_release_approval == false
  and .telegram_send_performed == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_completion_ack_recorded == false
  and .activation_allowed_by_release_artifact_publication == false
  and .activation_allowed == false
  and .activation_performed == false
  and .live_mutation_execution_performed == false
  and .memory_write_execution_performed == false
  and .memory_store_write_performed == false
  and .memory_store_write_performed_count == 0
  and .memory_store_mutated == false
  and .rollback_executed == false
  and .secret_material_read == false
  and .provider_invoked == false
  and .model_invoked == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and (.activation_command_result_receipt_release_artifact_publication_fixtures | length) == 10
  and ([.activation_command_result_receipt_release_artifact_publication_fixtures[] | select(.source_terminal_operator_decision_present == false)] | length) == 1
  and ([.activation_command_result_receipt_release_artifact_publication_fixtures[] | select(.release_artifact_write_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_release_artifact_publication_fixtures[] | select(.public_artifact_write_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_release_artifact_publication_fixtures[] | select(.artifact_signature_requested == true and .artifact_notarization_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_release_artifact_publication_fixtures[] | select(.service_restart_publication_requested == true and .active_binary_publication_requested == true)] | length) == 1
  and (.activation_command_result_receipt_release_artifact_publication_fixtures | all((.release_artifact_publication_status == "blocked_noop" or .release_artifact_publication_status == "blocked_artifact_noop" or .release_artifact_publication_status == "blocked_publication_noop" or .release_artifact_publication_status == "blocked_distribution_noop" or .release_artifact_publication_status == "blocked_release_noop" or .release_artifact_publication_status == "blocked_promotion_noop") and .release_artifact_publication_allowed == false and .release_artifact_publication_accepted == false and .release_artifact_publication_recorded == false and .release_artifact_publication_persisted == false and .release_artifact_filesystem_written == false and .release_artifact_written == false and .public_artifact_written == false and .publication_queue_enqueued == false and .public_distribution_performed == false and .public_release_published == false and .public_ga_claimed == false and .public_claim_promoted == false and .telegram_send_performed == false and .external_send_performed == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false and .release_artifact_publication_noop_confirmed == true))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$SOURCE_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/hepta-native-gateway/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native source command count"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_DENIAL_ENDPOINT' \
  "runtime provider-router activation command result receipt release artifact publication endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-release-artifact-publication-denial' \
  "runtime provider-router activation command result receipt release artifact publication endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-release-artifact-publication-denial --json' \
  "runtime provider-router activation command result receipt release artifact publication source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_report' \
  "runtime provider-router activation command result receipt release artifact publication report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_route_enabled": true' \
  "runtime provider-router activation command result receipt release artifact publication route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_release_artifact_publication_endpoint_blocks_publication_and_authority' \
  "runtime provider-router activation command result receipt release artifact publication focused test"

TEST_LOG="$(mktemp /tmp/hepta-runtime-provider-router-activation-command-result-receipt-release-artifact-publication-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_release_artifact_publication_endpoint_blocks_publication_and_authority \
  -- --nocapture >"$TEST_LOG"

LIVE_JSON='{}'
live_checked=false
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_JSON="$(
    curl -fsS \
      "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-release-artifact-publication-denial"
  )"
  jq -e --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" '
    .runtime == "hepta"
    and .status == "ready"
    and .route_count == $expected_route_count
    and .implemented_route_count == $expected_route_count
    and .missing_route_count == 0
    and .native_gateway_source_command_count == $expected_route_count
    and .runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_route_enabled == true
    and .runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_ready == true
    and .runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_status == "blocked"
    and .source_activation_command_result_receipt_terminal_operator_decision_public_claim_ready == true
    and .source_activation_command_result_receipt_terminal_operator_decision_public_claim_status == "blocked"
    and .required_activation_command_result_receipt_release_artifact_publication_surface_count == 12
    and .activation_command_result_receipt_release_artifact_publication_fixture_count == 10
    and .accepted_activation_command_result_receipt_release_artifact_publication_fixture_count == 0
    and .activation_command_result_receipt_release_artifact_publication_performed_count == 0
    and .release_artifact_publication_recorded == false
    and .release_artifact_publication_persisted == false
    and .release_artifact_filesystem_written == false
    and .release_artifact_written == false
    and .public_artifact_written == false
    and .artifact_signature_accepted == false
    and .artifact_notarization_accepted == false
    and .publication_queue_enqueued == false
    and .publication_manifest_written == false
    and .public_distribution_performed == false
    and .public_release_published == false
    and .public_ga_claimed == false
    and .public_claim_promoted == false
    and .terminal_operator_decision_promoted_to_release_approval == false
    and .activation_allowed_by_release_artifact_publication == false
    and .activation_allowed == false
    and .activation_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .secret_file_read == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .telegram_send_performed == false
    and .external_send_performed == false
    and .install_executed == false
    and .service_restart_performed == false
    and .active_binary_mutated == false
    and (.denied_by_activation_command_result_receipt_release_artifact_publication | length) == 70
    and .source_terminal_operator_decision_public_claim_denial_count == 56
    and (.side_effects | to_entries | all(.value == false))
  ' >/dev/null <<<"$LIVE_JSON"
  live_checked=true
fi

jq -n \
  --arg status "ready" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_route_gate" \
  --arg source_gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_gate" \
  --arg focused_test_log "$TEST_LOG" \
  --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
  --argjson live_checked "$live_checked" \
  --argjson live_route_count "$(jq -r '.route_count // 0' <<<"$LIVE_JSON")" \
  --argjson live_missing_route_count "$(jq -r '.missing_route_count // 0' <<<"$LIVE_JSON")" \
  --argjson accepted_release_artifact_publication_fixture_count "$(jq -r '.accepted_activation_command_result_receipt_release_artifact_publication_fixture_count // 0' <<<"$SOURCE_JSON")" \
  --argjson release_artifact_publication_performed_count "$(jq -r '.activation_command_result_receipt_release_artifact_publication_performed_count // 0' <<<"$SOURCE_JSON")" \
  --argjson source_release_artifact_publication_denial_count "$(jq -r '(.denied_by_activation_command_result_receipt_release_artifact_publication // []) | length' <<<"$SOURCE_JSON")" \
  '{
    status:$status,
    gate:$gate,
    source_gate:$source_gate,
    expected_route_count:$expected_route_count,
    focused_test_log:$focused_test_log,
    live_endpoint_checked:$live_checked,
    live_route_count:$live_route_count,
    live_missing_route_count:$live_missing_route_count,
    accepted_release_artifact_publication_fixture_count:$accepted_release_artifact_publication_fixture_count,
    release_artifact_publication_performed_count:$release_artifact_publication_performed_count,
    source_release_artifact_publication_denial_count:$source_release_artifact_publication_denial_count
  }'

echo "Hepta memory/intelligence/KG full enablement runtime provider-router activation command result receipt release artifact publication denial route gate passed"
