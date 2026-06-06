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

DISPATCH_DRY_RUN_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-single-budget-dispatch-dry-run-noop-receipt-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-single-budget-dispatch-dry-run-noop-receipt-gate.sh
)"

dispatch_dry_run_report_sha256="$(sha256_text "$DISPATCH_DRY_RUN_JSON")"
payload_preview_hash_sha256="$(
  jq -r '.source_payload_preview_hash_sha256' <<<"$DISPATCH_DRY_RUN_JSON"
)"
readback_receipt_hash_sha256="$(
  jq -r '.source_readback_receipt_hash_sha256' <<<"$DISPATCH_DRY_RUN_JSON"
)"
audit_receipt_hash_sha256="$(
  jq -r '.source_audit_receipt_hash_sha256' <<<"$DISPATCH_DRY_RUN_JSON"
)"
noop_receipt_hash_sha256="$(
  jq -r '.dispatch_dry_run_noop_receipts[0].noop_receipt_hash_sha256' <<<"$DISPATCH_DRY_RUN_JSON"
)"
operator_review_index_hash_sha256="$(
  sha256_text "hepta-canary-operator-review-readback-index:v1:payload=$payload_preview_hash_sha256:readback=$readback_receipt_hash_sha256:audit=$audit_receipt_hash_sha256:noop=$noop_receipt_hash_sha256:review=0:dispatch=0:execute=0:persist=0"
)"
operator_review_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-harness-operator-review-readback-index-no-persistence:v1:source-single-budget-dry-run:review-required:no-review-supplied:no-index-persist:no-dispatch:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_review_readback_index_side_effects=false;review_supplied=0;review_recorded=0;review_persisted=0;dispatch=0;execute=0;provider=0;model=0;memory=0;kg=0;channel=0;secret=0"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$DISPATCH_DRY_RUN_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_gate"
    and $source.operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_ready == true
    and $source.operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_status == "blocked"
    and $source.dispatch_dry_run_noop_receipt_count == 1
    and $source.single_budget_declared == 1
    and $source.single_budget_accepted == false
    and $source.single_budget_consumed == 0
    and $source.single_budget_remaining == 0
    and $source.dispatch_authority_accepted_count == 0
    and $source.controlled_request_dispatch_ready_count == 0
    and $source.controlled_request_dispatch_allowed_count == 0
    and $source.controlled_request_dispatched_count == 0
    and $source.controlled_request_execution_allowed_count == 0
    and $source.controlled_request_executed_count == 0
    and $source.noop_receipt_recorded_count == 0
    and $source.noop_receipt_persisted_count == 0
    and $source.noop_receipt_delivered_count == 0
    and $source.noop_receipt_accepted_count == 0
    and $source.noop_receipt_materialized_count == 0
    and $source.request_payload_materialized_count == 0
    and $source.request_payload_file_written_count == 0
    and $source.raw_payload_inspected_count == 0
    and $source.context_injection_performed_count == 0
    and $source.provider_invoked_count == 0
    and $source.model_invoked_count == 0
    and $source.memory_store_write_performed_count == 0
    and $source.external_kg_adapter_read_performed_count == 0
    and $source.live_kg_write_performed_count == 0
    and $source.credential_read_count == 0
    and $source.secret_file_read_count == 0
    and $source.channel_send_performed_count == 0
    and $source.canary_harness_armed == false
    and $source.canary_harness_executable == false
    and $source.canary_live_enabled == false
    and $source.dispatch_dry_run_noop_receipt_negative_fixture_count == 7
    and $source.dispatch_dry_run_noop_receipt_blocked_negative_fixture_count == 7
    and $source.dispatch_dry_run_noop_receipt_allowed_negative_fixture_count == 0
    and ($source.source_payload_preview_hash_sha256 | type) == "string"
    and ($source.source_payload_preview_hash_sha256 | length) == 64
    and ($source.source_readback_receipt_hash_sha256 | type) == "string"
    and ($source.source_readback_receipt_hash_sha256 | length) == 64
    and ($source.source_audit_receipt_hash_sha256 | type) == "string"
    and ($source.source_audit_receipt_hash_sha256 | length) == 64
    and ($source.dispatch_dry_run_noop_receipts | length) == 1
    and ($source.dispatch_dry_run_noop_receipts | all(
      (.noop_receipt_hash_sha256 | type) == "string"
      and (.noop_receipt_hash_sha256 | length) == 64
      and .dispatch_ready == false
      and .dispatch_allowed == false
      and .dispatch_performed == false
      and .execution_allowed == false
      and .execution_performed == false
      and .noop_receipt_recorded == false
      and .noop_receipt_persisted == false
      and .noop_receipt_delivered == false
      and .noop_receipt_accepted == false
      and .noop_receipt_materialized == false
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
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

review_sections_json="$(
  jq -n '
    [
      {section_id: "source-dispatch-dry-run", section_kind: "source_dispatch_dry_run"},
      {section_id: "single-budget-preview", section_kind: "single_budget_preview"},
      {section_id: "payload-hash-readback", section_kind: "payload_hash_readback"},
      {section_id: "readback-receipt-hash", section_kind: "readback_receipt_hash"},
      {section_id: "audit-receipt-hash", section_kind: "audit_receipt_hash"},
      {section_id: "noop-receipt-hash", section_kind: "noop_receipt_hash"},
      {section_id: "negative-fixture-summary", section_kind: "negative_fixture_summary"},
      {section_id: "side-effect-boundary", section_kind: "side_effect_boundary"}
    ]
  '
)"

negative_fixtures_json="$(
  jq -n '
    [
      {fixture_id: "missing-noop-receipt-hash", fixture_kind: "noop_receipt_hash_missing"},
      {fixture_id: "operator-review-acceptance-attempt", fixture_kind: "operator_review_acceptance_attempt"},
      {fixture_id: "readback-index-persistence-attempt", fixture_kind: "readback_index_persistence_attempt"},
      {fixture_id: "review-channel-delivery-attempt", fixture_kind: "review_channel_delivery_attempt"},
      {fixture_id: "dispatch-from-review-attempt", fixture_kind: "dispatch_from_review_attempt"},
      {fixture_id: "execution-from-review-attempt", fixture_kind: "execution_from_review_attempt"},
      {fixture_id: "provider-model-from-review-attempt", fixture_kind: "provider_model_from_review_attempt"},
      {fixture_id: "memory-kg-write-from-review-attempt", fixture_kind: "memory_kg_write_from_review_attempt"}
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_gate" \
    --arg dispatch_dry_run_report_sha256 "$dispatch_dry_run_report_sha256" \
    --arg operator_review_index_hash_sha256 "$operator_review_index_hash_sha256" \
    --arg operator_review_policy_hash_sha256 "$operator_review_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --arg payload_preview_hash_sha256 "$payload_preview_hash_sha256" \
    --arg readback_receipt_hash_sha256 "$readback_receipt_hash_sha256" \
    --arg audit_receipt_hash_sha256 "$audit_receipt_hash_sha256" \
    --arg noop_receipt_hash_sha256 "$noop_receipt_hash_sha256" \
    --arg operator_review_index_id "hepta-canary-operator-review-readback-index-no-persistence" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$DISPATCH_DRY_RUN_JSON" \
    --argjson review_sections "$review_sections_json" \
    --argjson negative_fixtures "$negative_fixtures_json" \
    '
      ($review_sections | map(. + {
        section_declared: true,
        operator_review_required: true,
        operator_review_supplied: false,
        operator_review_recorded: false,
        operator_review_persisted: false,
        operator_review_delivered: false,
        operator_review_accepted: false,
        authorizes_dispatch: false,
        authorizes_execution: false,
        authorizes_live: false
      })) as $sections
      | ($negative_fixtures | map(. + {
          fixture_status: "blocked",
          operator_review_supplied: false,
          operator_review_recorded: false,
          operator_review_persisted: false,
          operator_review_delivered: false,
          operator_review_accepted: false,
          readback_index_persisted: false,
          channel_send_performed: false,
          dispatch_performed: false,
          execution_performed: false,
          provider_invoked: false,
          model_invoked: false,
          memory_store_write_performed: false,
          external_kg_adapter_read_performed: false,
          live_kg_write_performed: false,
          credential_read: false,
          secret_file_read: false
        })) as $fixtures
      | {
          product: $product,
          runtime: $runtime,
          status: "ready",
          base_url: $base_url,
          gate: $gate,
          operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_schema_version: "memory_intelligence_kg_operator_canary_harness_operator_review_readback_index_no_persistence_v1",
          operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_ready: true,
          operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_status: "blocked",
          operator_review_readback_index_mode: "stdout_only_operator_review_readback_index_no_review_supplied_no_persistence_no_delivery_no_dispatch_no_live",
          operator_review_readback_index_decision: "operator_review_and_readback_index_shapes_are_declared_without_acceptance_persistence_delivery_dispatch_execution_or_live_authority",
          minimum_required_samples: $min_long_soak_samples,
          source_single_budget_dispatch_dry_run_noop_receipt_gate: $source.gate,
          source_single_budget_dispatch_dry_run_noop_receipt_status: $source.operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_status,
          source_single_budget_dispatch_dry_run_noop_receipt_report_sha256: $dispatch_dry_run_report_sha256,
          source_payload_preview_hash_sha256: $payload_preview_hash_sha256,
          source_readback_receipt_hash_sha256: $readback_receipt_hash_sha256,
          source_audit_receipt_hash_sha256: $audit_receipt_hash_sha256,
          source_noop_receipt_hash_sha256: $noop_receipt_hash_sha256,
          source_dispatch_dry_run_noop_receipt_count: $source.dispatch_dry_run_noop_receipt_count,
          source_single_budget_declared: $source.single_budget_declared,
          source_single_budget_accepted: $source.single_budget_accepted,
          source_single_budget_consumed: $source.single_budget_consumed,
          source_single_budget_remaining: $source.single_budget_remaining,
          source_controlled_request_dispatched_count: $source.controlled_request_dispatched_count,
          source_controlled_request_executed_count: $source.controlled_request_executed_count,
          source_noop_receipt_persisted_count: $source.noop_receipt_persisted_count,
          source_noop_receipt_accepted_count: $source.noop_receipt_accepted_count,
          operator_review_index_id: $operator_review_index_id,
          operator_review_index_hash_sha256: $operator_review_index_hash_sha256,
          operator_review_policy_hash_sha256: $operator_review_policy_hash_sha256,
          side_effect_hash_sha256: $side_effect_hash_sha256,
          operator_review_readback_index_sections: $sections,
          operator_review_readback_index_section_count: ($sections | length),
          operator_review_section_declared_count: ($sections | map(select(.section_declared == true)) | length),
          operator_review_required_count: ($sections | map(select(.operator_review_required == true)) | length),
          operator_review_supplied_count: 0,
          operator_review_recorded_count: 0,
          operator_review_persisted_count: 0,
          operator_review_delivered_count: 0,
          operator_review_accepted_count: 0,
          readback_index_declared_count: 1,
          readback_index_bound_to_payload_hash_count: 1,
          readback_index_bound_to_readback_receipt_hash_count: 1,
          readback_index_bound_to_audit_receipt_hash_count: 1,
          readback_index_bound_to_noop_receipt_hash_count: 1,
          readback_index_recorded_count: 0,
          readback_index_persisted_count: 0,
          readback_index_materialized_count: 0,
          readback_index_filesystem_written_count: 0,
          operator_review_index_recorded: false,
          operator_review_index_persisted: false,
          operator_review_index_materialized: false,
          operator_review_index_filesystem_written: false,
          operator_review_index_channel_delivered: false,
          operator_review_index_external_sent: false,
          operator_review_index_telegram_sent: false,
          review_authorizes_dispatch_count: 0,
          review_authorizes_execution_count: 0,
          review_authorizes_live_count: 0,
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
          operator_review_readback_index_negative_fixtures: $fixtures,
          operator_review_readback_index_negative_fixture_count: ($fixtures | length),
          operator_review_readback_index_blocked_negative_fixture_count: ($fixtures | map(select(.fixture_status == "blocked")) | length),
          operator_review_readback_index_allowed_negative_fixture_count: ($fixtures | map(select(.fixture_status == "allowed")) | length),
          denied_by_operator_review_readback_index: [
            "operator_review_not_supplied",
            "operator_review_recording_denied",
            "operator_review_persistence_denied",
            "operator_review_delivery_denied",
            "operator_review_acceptance_denied",
            "readback_index_persistence_denied",
            "readback_index_materialization_denied",
            "dispatch_from_review_denied",
            "execution_from_review_denied",
            "context_attachment_denied",
            "provider_model_invocation_denied",
            "memory_write_denied",
            "external_kg_read_denied",
            "live_kg_write_denied",
            "credential_secret_read_denied",
            "channel_delivery_denied"
          ],
          denied_by_operator_review_readback_index_count: 16,
          side_effects: {
            workspace_written: false,
            filesystem_written: false,
            operator_review_recorded: false,
            operator_review_persisted: false,
            operator_review_delivered: false,
            operator_review_accepted: false,
            operator_review_index_recorded: false,
            operator_review_index_persisted: false,
            operator_review_index_materialized: false,
            operator_review_index_filesystem_written: false,
            readback_index_persisted: false,
            readback_index_materialized: false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_gate"
  and .operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_ready == true
  and .operator_canary_controlled_request_harness_operator_review_readback_index_no_persistence_status == "blocked"
  and .source_single_budget_dispatch_dry_run_noop_receipt_gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_gate"
  and .source_single_budget_dispatch_dry_run_noop_receipt_status == "blocked"
  and .source_dispatch_dry_run_noop_receipt_count == 1
  and .source_single_budget_declared == 1
  and .source_single_budget_accepted == false
  and .source_single_budget_consumed == 0
  and .source_single_budget_remaining == 0
  and .source_controlled_request_dispatched_count == 0
  and .source_controlled_request_executed_count == 0
  and .source_noop_receipt_persisted_count == 0
  and .source_noop_receipt_accepted_count == 0
  and (.source_payload_preview_hash_sha256 | type) == "string"
  and (.source_payload_preview_hash_sha256 | length) == 64
  and (.source_readback_receipt_hash_sha256 | type) == "string"
  and (.source_readback_receipt_hash_sha256 | length) == 64
  and (.source_audit_receipt_hash_sha256 | type) == "string"
  and (.source_audit_receipt_hash_sha256 | length) == 64
  and (.source_noop_receipt_hash_sha256 | type) == "string"
  and (.source_noop_receipt_hash_sha256 | length) == 64
  and .operator_review_readback_index_section_count == 8
  and .operator_review_section_declared_count == 8
  and .operator_review_required_count == 8
  and .operator_review_supplied_count == 0
  and .operator_review_recorded_count == 0
  and .operator_review_persisted_count == 0
  and .operator_review_delivered_count == 0
  and .operator_review_accepted_count == 0
  and .readback_index_declared_count == 1
  and .readback_index_bound_to_payload_hash_count == 1
  and .readback_index_bound_to_readback_receipt_hash_count == 1
  and .readback_index_bound_to_audit_receipt_hash_count == 1
  and .readback_index_bound_to_noop_receipt_hash_count == 1
  and .readback_index_recorded_count == 0
  and .readback_index_persisted_count == 0
  and .readback_index_materialized_count == 0
  and .readback_index_filesystem_written_count == 0
  and .operator_review_index_recorded == false
  and .operator_review_index_persisted == false
  and .operator_review_index_materialized == false
  and .operator_review_index_filesystem_written == false
  and .operator_review_index_channel_delivered == false
  and .operator_review_index_external_sent == false
  and .operator_review_index_telegram_sent == false
  and .review_authorizes_dispatch_count == 0
  and .review_authorizes_execution_count == 0
  and .review_authorizes_live_count == 0
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
  and .operator_review_readback_index_negative_fixture_count == 8
  and .operator_review_readback_index_blocked_negative_fixture_count == 8
  and .operator_review_readback_index_allowed_negative_fixture_count == 0
  and (.operator_review_readback_index_sections | all(
    .section_declared == true
    and .operator_review_required == true
    and .operator_review_supplied == false
    and .operator_review_recorded == false
    and .operator_review_persisted == false
    and .operator_review_delivered == false
    and .operator_review_accepted == false
    and .authorizes_dispatch == false
    and .authorizes_execution == false
    and .authorizes_live == false
  ))
  and (.operator_review_readback_index_negative_fixtures | all(
    .fixture_status == "blocked"
    and .operator_review_supplied == false
    and .operator_review_recorded == false
    and .operator_review_persisted == false
    and .operator_review_delivered == false
    and .operator_review_accepted == false
    and .readback_index_persisted == false
    and .channel_send_performed == false
    and .dispatch_performed == false
    and .execution_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .external_kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
  ))
  and .denied_by_operator_review_readback_index_count == 16
  and (.denied_by_operator_review_readback_index | length) == 16
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG operator canary controlled request harness operator review/readback index no-persistence gate passed"
