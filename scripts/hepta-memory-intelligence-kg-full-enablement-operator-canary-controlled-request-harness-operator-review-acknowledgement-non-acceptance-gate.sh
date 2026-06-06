#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
RELEASE_BIN="${HEPTA_RELEASE_BIN:-${HEPTA_CODEX_RELEASE_BIN:-$HOME/.local/opt/hepta/bin/hepta}}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
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

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

REVIEW_INDEX_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-readback-index-no-persistence-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-readback-index-no-persistence-gate.sh
)"

review_index_report_sha256="$(sha256_text "$REVIEW_INDEX_JSON")"
payload_preview_hash_sha256="$(jq -r '.source_payload_preview_hash_sha256' <<<"$REVIEW_INDEX_JSON")"
readback_receipt_hash_sha256="$(jq -r '.source_readback_receipt_hash_sha256' <<<"$REVIEW_INDEX_JSON")"
audit_receipt_hash_sha256="$(jq -r '.source_audit_receipt_hash_sha256' <<<"$REVIEW_INDEX_JSON")"
noop_receipt_hash_sha256="$(jq -r '.source_noop_receipt_hash_sha256' <<<"$REVIEW_INDEX_JSON")"
operator_review_index_hash_sha256="$(jq -r '.operator_review_index_hash_sha256' <<<"$REVIEW_INDEX_JSON")"
operator_review_acknowledgement_index_hash_sha256="$(
  sha256_text "hepta-canary-operator-review-acknowledgement-non-acceptance:v1:review=$operator_review_index_hash_sha256:payload=$payload_preview_hash_sha256:readback=$readback_receipt_hash_sha256:audit=$audit_receipt_hash_sha256:noop=$noop_receipt_hash_sha256:ack=0:accept=0:dispatch=0:execute=0:persist=0"
)"
operator_review_acknowledgement_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-harness-operator-review-acknowledgement-non-acceptance:v1:source-review-index:no-ack-accept:no-ack-record:no-ack-persist:no-dispatch:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_review_acknowledgement_side_effects=false;ack_requested=8;ack_performed=0;ack_recorded=0;ack_persisted=0;dispatch=0;execute=0;provider=0;model=0;memory=0;kg=0;channel=0;secret=0"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$REVIEW_INDEX_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_gate"
    and $source.operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_ready == true
    and $source.operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_status == "blocked"
    and $source.operator_review_readback_index_section_count == 8
    and $source.operator_review_required_count == 8
    and $source.operator_review_supplied_count == 0
    and $source.operator_review_recorded_count == 0
    and $source.operator_review_persisted_count == 0
    and $source.operator_review_delivered_count == 0
    and $source.operator_review_accepted_count == 0
    and $source.readback_index_declared_count == 1
    and $source.readback_index_recorded_count == 0
    and $source.readback_index_persisted_count == 0
    and $source.readback_index_materialized_count == 0
    and $source.readback_index_filesystem_written_count == 0
    and $source.operator_review_index_recorded == false
    and $source.operator_review_index_persisted == false
    and $source.operator_review_index_materialized == false
    and $source.operator_review_index_filesystem_written == false
    and $source.operator_review_index_channel_delivered == false
    and $source.operator_review_index_external_sent == false
    and $source.operator_review_index_telegram_sent == false
    and $source.review_authorizes_dispatch_count == 0
    and $source.review_authorizes_execution_count == 0
    and $source.review_authorizes_live_count == 0
    and $source.dispatch_performed_count == 0
    and $source.execution_performed_count == 0
    and $source.context_injection_performed_count == 0
    and $source.provider_invoked_count == 0
    and $source.model_invoked_count == 0
    and $source.memory_store_write_performed_count == 0
    and $source.external_kg_adapter_read_performed_count == 0
    and $source.live_kg_write_performed_count == 0
    and $source.credential_read_count == 0
    and $source.secret_file_read_count == 0
    and $source.channel_send_performed_count == 0
    and $source.operator_review_readback_index_negative_fixture_count == 8
    and $source.operator_review_readback_index_blocked_negative_fixture_count == 8
    and $source.operator_review_readback_index_allowed_negative_fixture_count == 0
    and ($source.source_payload_preview_hash_sha256 | type) == "string"
    and ($source.source_payload_preview_hash_sha256 | length) == 64
    and ($source.source_readback_receipt_hash_sha256 | type) == "string"
    and ($source.source_readback_receipt_hash_sha256 | length) == 64
    and ($source.source_audit_receipt_hash_sha256 | type) == "string"
    and ($source.source_audit_receipt_hash_sha256 | length) == 64
    and ($source.source_noop_receipt_hash_sha256 | type) == "string"
    and ($source.source_noop_receipt_hash_sha256 | length) == 64
    and ($source.operator_review_index_hash_sha256 | type) == "string"
    and ($source.operator_review_index_hash_sha256 | length) == 64
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

acknowledgement_fixtures_json="$(
  jq -n '
    [
      {fixture_id: "seen-review-index-without-approval", fixture_kind: "seen_review_index_without_approval"},
      {fixture_id: "reviewed-readback-index-attempt", fixture_kind: "reviewed_readback_index_attempt"},
      {fixture_id: "acknowledged-noop-receipt-attempt", fixture_kind: "acknowledged_noop_receipt_attempt"},
      {fixture_id: "acknowledgement-recording-attempt", fixture_kind: "acknowledgement_recording_attempt"},
      {fixture_id: "acknowledgement-delivery-attempt", fixture_kind: "acknowledgement_delivery_attempt"},
      {fixture_id: "dispatch-from-acknowledgement-attempt", fixture_kind: "dispatch_from_acknowledgement_attempt"},
      {fixture_id: "provider-model-from-acknowledgement-attempt", fixture_kind: "provider_model_from_acknowledgement_attempt"},
      {fixture_id: "memory-kg-write-from-acknowledgement-attempt", fixture_kind: "memory_kg_write_from_acknowledgement_attempt"}
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_gate" \
    --arg review_index_report_sha256 "$review_index_report_sha256" \
    --arg payload_preview_hash_sha256 "$payload_preview_hash_sha256" \
    --arg readback_receipt_hash_sha256 "$readback_receipt_hash_sha256" \
    --arg audit_receipt_hash_sha256 "$audit_receipt_hash_sha256" \
    --arg noop_receipt_hash_sha256 "$noop_receipt_hash_sha256" \
    --arg operator_review_index_hash_sha256 "$operator_review_index_hash_sha256" \
    --arg operator_review_acknowledgement_index_hash_sha256 "$operator_review_acknowledgement_index_hash_sha256" \
    --arg operator_review_acknowledgement_policy_hash_sha256 "$operator_review_acknowledgement_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$REVIEW_INDEX_JSON" \
    --argjson acknowledgement_fixtures "$acknowledgement_fixtures_json" \
    '
      ($acknowledgement_fixtures | map(. + {
        acknowledgement_requested: true,
        acknowledgement_status: "blocked_noop",
        acknowledgement_performed: false,
        acknowledgement_accepted: false,
        acknowledgement_recorded: false,
        acknowledgement_persisted: false,
        acknowledgement_materialized: false,
        acknowledgement_filesystem_written: false,
        acknowledgement_delivered: false,
        identity_accepted: false,
        signature_accepted: false,
        operator_approval_recorded: false,
        review_index_persisted: false,
        readback_index_persisted: false,
        dispatch_allowed: false,
        dispatch_performed: false,
        execution_allowed: false,
        execution_performed: false,
        context_injection_performed: false,
        provider_invoked: false,
        model_invoked: false,
        memory_store_write_performed: false,
        external_kg_adapter_read_performed: false,
        live_kg_write_performed: false,
        credential_read: false,
        secret_file_read: false,
        channel_send_performed: false
      })) as $fixtures
      | {
          product: $product,
          runtime: $runtime,
          status: "ready",
          base_url: $base_url,
          gate: $gate,
          operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_schema_version: "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_non_acceptance_v1",
          operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_ready: true,
          operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_status: "blocked",
          operator_review_acknowledgement_mode: "stdout_only_acknowledgement_shapes_no_acceptance_no_recording_no_persistence_no_dispatch_no_live",
          operator_review_acknowledgement_decision: "review_acknowledgement_attempts_remain_blocked_noop_and_do_not_promote_operator_review_or_readback_index_to_authority",
          minimum_required_samples: $min_long_soak_samples,
          source_operator_review_readback_index_gate: $source.gate,
          source_operator_review_readback_index_status: $source.operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_status,
          source_operator_review_readback_index_report_sha256: $review_index_report_sha256,
          source_payload_preview_hash_sha256: $payload_preview_hash_sha256,
          source_readback_receipt_hash_sha256: $readback_receipt_hash_sha256,
          source_audit_receipt_hash_sha256: $audit_receipt_hash_sha256,
          source_noop_receipt_hash_sha256: $noop_receipt_hash_sha256,
          source_operator_review_index_hash_sha256: $operator_review_index_hash_sha256,
          source_operator_review_required_count: $source.operator_review_required_count,
          source_operator_review_accepted_count: $source.operator_review_accepted_count,
          source_readback_index_declared_count: $source.readback_index_declared_count,
          source_readback_index_persisted_count: $source.readback_index_persisted_count,
          source_review_authorizes_dispatch_count: $source.review_authorizes_dispatch_count,
          source_review_authorizes_execution_count: $source.review_authorizes_execution_count,
          source_review_authorizes_live_count: $source.review_authorizes_live_count,
          operator_review_acknowledgement_index_hash_sha256: $operator_review_acknowledgement_index_hash_sha256,
          operator_review_acknowledgement_policy_hash_sha256: $operator_review_acknowledgement_policy_hash_sha256,
          side_effect_hash_sha256: $side_effect_hash_sha256,
          operator_review_acknowledgement_fixtures: $fixtures,
          operator_review_acknowledgement_fixture_count: ($fixtures | length),
          operator_review_acknowledgement_requested_fixture_count: ($fixtures | map(select(.acknowledgement_requested == true)) | length),
          blocked_operator_review_acknowledgement_fixture_count: ($fixtures | map(select(.acknowledgement_status == "blocked_noop")) | length),
          noop_operator_review_acknowledgement_fixture_count: ($fixtures | map(select(.acknowledgement_performed == false)) | length),
          allowed_operator_review_acknowledgement_fixture_count: 0,
          accepted_operator_review_acknowledgement_fixture_count: 0,
          operator_review_acknowledgement_performed_count: 0,
          operator_review_acknowledgement_allowed: false,
          operator_review_acknowledgement_accepted: false,
          operator_review_acknowledgement_recorded: false,
          operator_review_acknowledgement_persisted: false,
          operator_review_acknowledgement_materialized: false,
          operator_review_acknowledgement_filesystem_written: false,
          operator_review_acknowledgement_delivered: false,
          operator_review_acknowledgement_identity_accepted: false,
          operator_review_acknowledgement_signature_accepted: false,
          operator_review_acknowledgement_final_state_promoted: false,
          operator_review_acknowledgement_completion_promoted: false,
          operator_review_acknowledgement_authorizes_dispatch_count: 0,
          operator_review_acknowledgement_authorizes_execution_count: 0,
          operator_review_acknowledgement_authorizes_live_count: 0,
          operator_approval_recorded: false,
          operator_identity_accepted: false,
          readback_index_recorded_count: 0,
          readback_index_persisted_count: 0,
          readback_index_materialized_count: 0,
          readback_index_filesystem_written_count: 0,
          dispatch_allowed_count: 0,
          dispatch_performed_count: 0,
          execution_allowed_count: 0,
          execution_performed_count: 0,
          context_injection_performed_count: 0,
          provider_invoked_count: 0,
          model_invoked_count: 0,
          memory_store_write_performed_count: 0,
          external_kg_adapter_read_performed_count: 0,
          live_kg_write_performed_count: 0,
          credential_read_count: 0,
          secret_file_read_count: 0,
          channel_send_performed_count: 0,
          canary_harness_armed: false,
          canary_harness_executable: false,
          canary_live_enabled: false,
          denied_by_operator_review_acknowledgement_non_acceptance: [
            "operator_review_acknowledgement_acceptance_denied",
            "operator_review_acknowledgement_recording_denied",
            "operator_review_acknowledgement_persistence_denied",
            "operator_review_acknowledgement_materialization_denied",
            "operator_review_acknowledgement_filesystem_write_denied",
            "operator_review_acknowledgement_delivery_denied",
            "operator_review_acknowledgement_identity_acceptance_denied",
            "operator_review_acknowledgement_signature_acceptance_denied",
            "operator_review_acknowledgement_cannot_promote_review_index",
            "operator_review_acknowledgement_cannot_promote_readback_index",
            "operator_review_acknowledgement_cannot_promote_dispatch_authority",
            "operator_review_acknowledgement_cannot_promote_execution_authority",
            "operator_review_acknowledgement_cannot_promote_live_authority",
            "provider_model_invocation_denied",
            "memory_write_denied",
            "external_kg_read_denied",
            "live_kg_write_denied",
            "credential_secret_read_denied",
            "channel_delivery_denied"
          ],
          denied_by_operator_review_acknowledgement_non_acceptance_count: 19,
          side_effects: {
            workspace_written: false,
            filesystem_written: false,
            operator_review_acknowledgement_performed: false,
            operator_review_acknowledgement_recorded: false,
            operator_review_acknowledgement_persisted: false,
            operator_review_acknowledgement_materialized: false,
            operator_review_acknowledgement_filesystem_written: false,
            operator_review_acknowledgement_delivered: false,
            operator_review_acknowledgement_accepted: false,
            operator_approval_recorded: false,
            operator_identity_accepted: false,
            readback_index_recorded: false,
            readback_index_persisted: false,
            readback_index_materialized: false,
            readback_index_filesystem_written: false,
            dispatch_performed: false,
            execution_performed: false,
            context_injection_performed: false,
            provider_invoked: false,
            model_invoked: false,
            memory_store_write_performed: false,
            memory_store_mutated: false,
            external_kg_adapter_read_performed: false,
            live_kg_write_performed: false,
            credential_read: false,
            secret_file_read: false,
            channel_send_performed: false,
            telegram_send_performed: false,
            service_restarted: false,
            active_binary_mutated: false,
            install_performed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false
          }
        }
    '
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_gate"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_status == "blocked"
  and .source_operator_review_readback_index_gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_gate"
  and .source_operator_review_readback_index_status == "blocked"
  and .source_operator_review_required_count == 8
  and .source_operator_review_accepted_count == 0
  and .source_readback_index_declared_count == 1
  and .source_readback_index_persisted_count == 0
  and .source_review_authorizes_dispatch_count == 0
  and .source_review_authorizes_execution_count == 0
  and .source_review_authorizes_live_count == 0
  and (.source_payload_preview_hash_sha256 | type) == "string"
  and (.source_payload_preview_hash_sha256 | length) == 64
  and (.source_readback_receipt_hash_sha256 | type) == "string"
  and (.source_readback_receipt_hash_sha256 | length) == 64
  and (.source_audit_receipt_hash_sha256 | type) == "string"
  and (.source_audit_receipt_hash_sha256 | length) == 64
  and (.source_noop_receipt_hash_sha256 | type) == "string"
  and (.source_noop_receipt_hash_sha256 | length) == 64
  and (.source_operator_review_index_hash_sha256 | type) == "string"
  and (.source_operator_review_index_hash_sha256 | length) == 64
  and (.operator_review_acknowledgement_index_hash_sha256 | type) == "string"
  and (.operator_review_acknowledgement_index_hash_sha256 | length) == 64
  and .operator_review_acknowledgement_fixture_count == 8
  and .operator_review_acknowledgement_requested_fixture_count == 8
  and .blocked_operator_review_acknowledgement_fixture_count == 8
  and .noop_operator_review_acknowledgement_fixture_count == 8
  and .allowed_operator_review_acknowledgement_fixture_count == 0
  and .accepted_operator_review_acknowledgement_fixture_count == 0
  and .operator_review_acknowledgement_performed_count == 0
  and .operator_review_acknowledgement_allowed == false
  and .operator_review_acknowledgement_accepted == false
  and .operator_review_acknowledgement_recorded == false
  and .operator_review_acknowledgement_persisted == false
  and .operator_review_acknowledgement_materialized == false
  and .operator_review_acknowledgement_filesystem_written == false
  and .operator_review_acknowledgement_delivered == false
  and .operator_review_acknowledgement_identity_accepted == false
  and .operator_review_acknowledgement_signature_accepted == false
  and .operator_review_acknowledgement_final_state_promoted == false
  and .operator_review_acknowledgement_completion_promoted == false
  and .operator_review_acknowledgement_authorizes_dispatch_count == 0
  and .operator_review_acknowledgement_authorizes_execution_count == 0
  and .operator_review_acknowledgement_authorizes_live_count == 0
  and .operator_approval_recorded == false
  and .operator_identity_accepted == false
  and .readback_index_recorded_count == 0
  and .readback_index_persisted_count == 0
  and .readback_index_materialized_count == 0
  and .readback_index_filesystem_written_count == 0
  and .dispatch_allowed_count == 0
  and .dispatch_performed_count == 0
  and .execution_allowed_count == 0
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
  and .canary_harness_armed == false
  and .canary_harness_executable == false
  and .canary_live_enabled == false
  and (.operator_review_acknowledgement_fixtures | all(
    .acknowledgement_requested == true
    and .acknowledgement_status == "blocked_noop"
    and .acknowledgement_performed == false
    and .acknowledgement_accepted == false
    and .acknowledgement_recorded == false
    and .acknowledgement_persisted == false
    and .acknowledgement_materialized == false
    and .acknowledgement_filesystem_written == false
    and .acknowledgement_delivered == false
    and .identity_accepted == false
    and .signature_accepted == false
    and .operator_approval_recorded == false
    and .review_index_persisted == false
    and .readback_index_persisted == false
    and .dispatch_allowed == false
    and .dispatch_performed == false
    and .execution_allowed == false
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
  ))
  and .denied_by_operator_review_acknowledgement_non_acceptance_count == 19
  and (.denied_by_operator_review_acknowledgement_non_acceptance | length) == 19
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG operator canary controlled request harness operator review acknowledgement non-acceptance gate passed"
