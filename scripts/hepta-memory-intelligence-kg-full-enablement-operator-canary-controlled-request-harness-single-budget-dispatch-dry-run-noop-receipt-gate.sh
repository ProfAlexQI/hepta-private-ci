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

RECEIPT_HASH_PREVIEW_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-readback-audit-receipt-hash-preview-acceptance-skeleton-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-readback-audit-receipt-hash-preview-acceptance-skeleton-gate.sh
)"

receipt_hash_preview_report_sha256="$(sha256_text "$RECEIPT_HASH_PREVIEW_JSON")"
payload_preview_hash_sha256="$(
  jq -r '.source_payload_preview_hash_sha256' <<<"$RECEIPT_HASH_PREVIEW_JSON"
)"
readback_receipt_hash_sha256="$(
  jq -r '.receipt_hash_previews[] | select(.receipt_kind == "readback") | .receipt_hash_sha256' <<<"$RECEIPT_HASH_PREVIEW_JSON"
)"
audit_receipt_hash_sha256="$(
  jq -r '.receipt_hash_previews[] | select(.receipt_kind == "audit") | .receipt_hash_sha256' <<<"$RECEIPT_HASH_PREVIEW_JSON"
)"
noop_receipt_hash_sha256="$(
  sha256_text "hepta-canary-single-budget-dispatch-dry-run-noop-receipt:v1:payload=$payload_preview_hash_sha256:readback=$readback_receipt_hash_sha256:audit=$audit_receipt_hash_sha256:dispatch=0:execute=0:persist=0"
)"
dispatch_dry_run_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-harness-single-budget-dispatch-dry-run-noop-receipt:v1:source-receipt-hash-preview:single-budget:no-accept:no-consume:no-dispatch:no-execute:no-persist:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "single_budget_dispatch_dry_run_noop_receipt_side_effects=false;budget_accepted=0;budget_consumed=0;dispatch=0;execute=0;noop_receipt_persisted=0;context=0;provider=0;model=0;memory=0;kg=0;secret=0"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$RECEIPT_HASH_PREVIEW_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_readback_audit_receipt_hash_preview_acceptance_skeleton_gate"
    and $source.operator_canary_controlled_request_harness_readback_audit_receipt_hash_preview_acceptance_skeleton_ready == true
    and $source.operator_canary_controlled_request_harness_readback_audit_receipt_hash_preview_acceptance_skeleton_status == "blocked"
    and $source.receipt_hash_preview_count == 2
    and $source.receipt_hash_preview_shape_declared_count == 2
    and $source.receipt_hash_shape_declared_count == 2
    and $source.receipt_hash_bound_to_payload_preview_count == 2
    and $source.readback_receipt_hash_preview_declared_count == 1
    and $source.audit_receipt_hash_preview_declared_count == 1
    and $source.receipt_hash_accepted_count == 0
    and $source.receipt_recorded_count == 0
    and $source.receipt_persisted_count == 0
    and $source.receipt_delivered_count == 0
    and $source.receipt_accepted_count == 0
    and $source.receipt_materialized_count == 0
    and $source.receipt_report_only_count == 2
    and $source.acceptance_skeleton_declared_count == 2
    and $source.acceptance_skeleton_operator_input_required_count == 2
    and $source.acceptance_skeleton_operator_input_supplied_count == 0
    and $source.acceptance_skeleton_recorded_count == 0
    and $source.acceptance_skeleton_persisted_count == 0
    and $source.acceptance_skeleton_accepted_count == 0
    and $source.payload_preview_hash_accepted == false
    and $source.payload_preview_accepted == false
    and $source.request_payload_materialized == false
    and $source.request_payload_file_written == false
    and $source.raw_payload_inspected == false
    and $source.controlled_request_dispatch_budget_declared == 1
    and $source.controlled_request_dispatch_budget_accepted == false
    and $source.controlled_request_dispatch_budget_consumed == 0
    and $source.controlled_request_dispatch_budget_remaining == 0
    and $source.controlled_request_dispatch_allowed == false
    and $source.controlled_request_dispatched == false
    and $source.controlled_request_execution_allowed == false
    and $source.controlled_request_executed == false
    and $source.readback_receipt_persisted == false
    and $source.audit_receipt_persisted == false
    and $source.context_injection_performed == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.memory_store_write_performed == false
    and $source.external_kg_adapter_read_performed == false
    and $source.live_kg_write_performed == false
    and $source.credential_read == false
    and $source.secret_file_read == false
    and $source.channel_send_performed == false
    and $source.canary_harness_armed == false
    and $source.canary_harness_executable == false
    and $source.canary_live_enabled == false
    and $source.receipt_hash_preview_negative_fixture_count == 6
    and $source.receipt_hash_preview_blocked_negative_fixture_count == 6
    and $source.receipt_hash_preview_allowed_negative_fixture_count == 0
    and ($source.source_payload_preview_hash_sha256 | type) == "string"
    and ($source.source_payload_preview_hash_sha256 | length) == 64
    and ($source.receipt_hash_previews | all(
      .receipt_hash_bound_to_payload_preview == true
      and .receipt_hash_accepted == false
      and .receipt_recorded == false
      and .receipt_persisted == false
      and .receipt_delivered == false
      and .receipt_accepted == false
      and .receipt_materialized == false
      and .receipt_report_only == true
      and .acceptance_skeleton_declared == true
      and .acceptance_skeleton_operator_input_required == true
      and .acceptance_skeleton_operator_input_supplied == false
      and .acceptance_skeleton_recorded == false
      and .acceptance_skeleton_persisted == false
      and .acceptance_skeleton_accepted == false
      and .authorizes_dispatch == false
      and .authorizes_execution == false
    ))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

negative_fixtures_json="$(
  jq -n '
    [
      {
        fixture_id: "missing-receipt-hash-preview",
        fixture_kind: "receipt_hash_preview_missing",
        denial_reason: "dispatch dry-run no-op receipt requires both readback and audit receipt hash previews"
      },
      {
        fixture_id: "dispatch-budget-acceptance-attempt",
        fixture_kind: "dispatch_budget_acceptance_attempt",
        denial_reason: "dispatch budget preview cannot be accepted without operator authority"
      },
      {
        fixture_id: "dispatch-budget-consumption-attempt",
        fixture_kind: "dispatch_budget_consumption_attempt",
        denial_reason: "dispatch dry-run cannot consume the single budget slot"
      },
      {
        fixture_id: "noop-receipt-persistence-attempt",
        fixture_kind: "noop_receipt_persistence_attempt",
        denial_reason: "no-op receipt hash preview cannot persist or become accepted evidence"
      },
      {
        fixture_id: "provider-model-invocation-attempt",
        fixture_kind: "provider_model_invocation_attempt",
        denial_reason: "dispatch dry-run cannot call provider or model"
      },
      {
        fixture_id: "memory-kg-write-attempt",
        fixture_kind: "memory_kg_write_attempt",
        denial_reason: "dispatch dry-run cannot write Memory or KG state"
      },
      {
        fixture_id: "channel-delivery-attempt",
        fixture_kind: "channel_delivery_attempt",
        denial_reason: "dispatch dry-run cannot send or deliver a channel message"
      }
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_gate" \
    --arg receipt_hash_preview_report_sha256 "$receipt_hash_preview_report_sha256" \
    --arg dispatch_dry_run_policy_hash_sha256 "$dispatch_dry_run_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --arg payload_preview_hash_sha256 "$payload_preview_hash_sha256" \
    --arg readback_receipt_hash_sha256 "$readback_receipt_hash_sha256" \
    --arg audit_receipt_hash_sha256 "$audit_receipt_hash_sha256" \
    --arg noop_receipt_hash_sha256 "$noop_receipt_hash_sha256" \
    --arg dispatch_dry_run_id "hepta-canary-single-budget-dispatch-dry-run-noop-receipt" \
    --arg payload_preview_id "hepta-canary-controlled-request-single-route-single-namespace-redacted-payload-preview" \
    --arg route_id "hepta.memory_intelligence_kg.canary.single_route.preview" \
    --arg namespace_id "hepta-memory-intelligence-kg-canary-single-namespace" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$RECEIPT_HASH_PREVIEW_JSON" \
    --argjson negative_fixtures "$negative_fixtures_json" \
    '
      {
        dispatch_dry_run_id: $dispatch_dry_run_id,
        payload_preview_id: $payload_preview_id,
        route_id: $route_id,
        namespace_id: $namespace_id,
        payload_preview_hash_sha256: $payload_preview_hash_sha256,
        readback_receipt_hash_sha256: $readback_receipt_hash_sha256,
        audit_receipt_hash_sha256: $audit_receipt_hash_sha256,
        noop_receipt_hash_sha256: $noop_receipt_hash_sha256,
        dispatch_dry_run_shape_declared: true,
        dispatch_intent_shape_declared: true,
        single_budget_shape_declared: true,
        single_budget_declared: 1,
        single_budget_accepted: false,
        single_budget_consumed: 0,
        single_budget_remaining: 0,
        dispatch_authority_accepted: false,
        dispatch_preconditions_satisfied: false,
        dispatch_ready: false,
        dispatch_allowed: false,
        dispatch_performed: false,
        execution_allowed: false,
        execution_performed: false,
        noop_receipt_shape_declared: true,
        noop_receipt_hash_shape_declared: true,
        noop_receipt_hash_bound_to_payload_preview: true,
        noop_receipt_hash_bound_to_readback_receipt_hash: true,
        noop_receipt_hash_bound_to_audit_receipt_hash: true,
        noop_receipt_recorded: false,
        noop_receipt_persisted: false,
        noop_receipt_delivered: false,
        noop_receipt_accepted: false,
        noop_receipt_materialized: false,
        readback_receipt_hash_preview_accepted: false,
        audit_receipt_hash_preview_accepted: false,
        acceptance_skeleton_accepted: false,
        request_payload_materialization_allowed: false,
        request_payload_materialized: false,
        request_payload_file_written: false,
        raw_payload_inspected: false,
        context_injection_allowed: false,
        context_injection_performed: false,
        provider_model_invocation_allowed: false,
        provider_invoked: false,
        model_invoked: false,
        memory_write_allowed: false,
        memory_store_write_performed: false,
        external_kg_read_allowed: false,
        external_kg_adapter_read_performed: false,
        live_kg_write_allowed: false,
        live_kg_write_performed: false,
        credential_read_allowed: false,
        credential_read: false,
        secret_file_read: false,
        channel_delivery_allowed: false,
        channel_send_performed: false,
        status: "blocked_single_budget_dispatch_dry_run_noop_receipt_only"
      } as $noop_receipt
      | ($negative_fixtures | map(. + {
          fixture_status: "blocked",
          single_budget_accepted: false,
          single_budget_consumed: 0,
          dispatch_performed: false,
          execution_performed: false,
          noop_receipt_recorded: false,
          noop_receipt_persisted: false,
          noop_receipt_accepted: false,
          noop_receipt_materialized: false,
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
          operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_schema_version: "memory_intelligence_kg_operator_canary_harness_single_budget_dispatch_dry_run_noop_receipt_v1",
          operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_ready: true,
          operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_status: "blocked",
          dispatch_dry_run_noop_receipt_mode: "stdout_only_single_budget_dispatch_dry_run_noop_receipt_no_accept_no_consume_no_dispatch_no_execute_no_persist_no_live",
          dispatch_dry_run_noop_receipt_decision: "single_budget_dispatch_dry_run_and_noop_receipt_shapes_are_declared_without_accepting_consuming_dispatching_executing_or_persisting",
          minimum_required_samples: $min_long_soak_samples,
          source_receipt_hash_preview_acceptance_skeleton_gate: $source.gate,
          source_receipt_hash_preview_acceptance_skeleton_status: $source.operator_canary_controlled_request_harness_readback_audit_receipt_hash_preview_acceptance_skeleton_status,
          source_receipt_hash_preview_acceptance_skeleton_report_sha256: $receipt_hash_preview_report_sha256,
          source_payload_preview_hash_sha256: $payload_preview_hash_sha256,
          source_readback_receipt_hash_sha256: $readback_receipt_hash_sha256,
          source_audit_receipt_hash_sha256: $audit_receipt_hash_sha256,
          source_receipt_hash_preview_count: $source.receipt_hash_preview_count,
          source_receipt_hash_accepted_count: $source.receipt_hash_accepted_count,
          source_receipt_recorded_count: $source.receipt_recorded_count,
          source_receipt_persisted_count: $source.receipt_persisted_count,
          source_receipt_delivered_count: $source.receipt_delivered_count,
          source_receipt_accepted_count: $source.receipt_accepted_count,
          source_receipt_materialized_count: $source.receipt_materialized_count,
          source_acceptance_skeleton_declared_count: $source.acceptance_skeleton_declared_count,
          source_acceptance_skeleton_operator_input_required_count: $source.acceptance_skeleton_operator_input_required_count,
          source_acceptance_skeleton_operator_input_supplied_count: $source.acceptance_skeleton_operator_input_supplied_count,
          source_acceptance_skeleton_accepted_count: $source.acceptance_skeleton_accepted_count,
          source_controlled_request_dispatch_budget_declared: $source.controlled_request_dispatch_budget_declared,
          source_controlled_request_dispatch_budget_accepted: $source.controlled_request_dispatch_budget_accepted,
          source_controlled_request_dispatch_budget_consumed: $source.controlled_request_dispatch_budget_consumed,
          source_controlled_request_dispatch_budget_remaining: $source.controlled_request_dispatch_budget_remaining,
          dispatch_dry_run_policy_hash_sha256: $dispatch_dry_run_policy_hash_sha256,
          side_effect_hash_sha256: $side_effect_hash_sha256,
          dispatch_dry_run_noop_receipts: [$noop_receipt],
          dispatch_dry_run_noop_receipt_count: 1,
          dispatch_dry_run_shape_declared_count: 1,
          dispatch_intent_shape_declared_count: 1,
          single_budget_shape_declared_count: 1,
          single_budget_declared: 1,
          single_budget_accepted: false,
          single_budget_consumed: 0,
          single_budget_remaining: 0,
          dispatch_authority_accepted_count: 0,
          dispatch_preconditions_satisfied_count: 0,
          controlled_request_dispatch_ready_count: 0,
          controlled_request_dispatch_allowed_count: 0,
          controlled_request_dispatched_count: 0,
          controlled_request_execution_allowed_count: 0,
          controlled_request_executed_count: 0,
          noop_receipt_shape_declared_count: 1,
          noop_receipt_hash_shape_declared_count: 1,
          noop_receipt_hash_bound_to_payload_preview_count: 1,
          noop_receipt_hash_bound_to_receipt_hash_preview_count: 1,
          noop_receipt_recorded_count: 0,
          noop_receipt_persisted_count: 0,
          noop_receipt_delivered_count: 0,
          noop_receipt_accepted_count: 0,
          noop_receipt_materialized_count: 0,
          readback_receipt_hash_preview_accepted_count: 0,
          audit_receipt_hash_preview_accepted_count: 0,
          acceptance_skeleton_accepted_count: 0,
          request_payload_materialized_count: 0,
          request_payload_file_written_count: 0,
          raw_payload_inspected_count: 0,
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
          dispatch_dry_run_noop_receipt_negative_fixtures: $fixtures,
          dispatch_dry_run_noop_receipt_negative_fixture_count: ($fixtures | length),
          dispatch_dry_run_noop_receipt_blocked_negative_fixture_count: ($fixtures | map(select(.fixture_status == "blocked")) | length),
          dispatch_dry_run_noop_receipt_allowed_negative_fixture_count: ($fixtures | map(select(.fixture_status == "allowed")) | length),
          denied_by_dispatch_dry_run_noop_receipt: [
            "single_budget_dispatch_dry_run_not_operator_approval",
            "dispatch_budget_acceptance_denied",
            "dispatch_budget_consumption_denied",
            "dispatch_execution_denied",
            "noop_receipt_recording_denied",
            "noop_receipt_persistence_denied",
            "noop_receipt_delivery_denied",
            "noop_receipt_acceptance_denied",
            "request_payload_materialization_denied",
            "context_attachment_denied",
            "provider_model_invocation_denied",
            "memory_write_denied",
            "external_kg_read_denied",
            "live_kg_write_denied",
            "credential_secret_read_denied",
            "channel_delivery_denied"
          ],
          denied_by_dispatch_dry_run_noop_receipt_count: 16,
          side_effects: {
            workspace_written: false,
            filesystem_written: false,
            single_budget_accepted: false,
            single_budget_consumed: false,
            dispatch_performed: false,
            execution_performed: false,
            noop_receipt_recorded: false,
            noop_receipt_persisted: false,
            noop_receipt_delivered: false,
            noop_receipt_accepted: false,
            noop_receipt_materialized: false,
            request_payload_materialized: false,
            request_payload_file_written: false,
            raw_payload_inspected: false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_gate"
  and .operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_ready == true
  and .operator_canary_controlled_request_harness_single_budget_dispatch_dry_run_noop_receipt_status == "blocked"
  and .source_receipt_hash_preview_acceptance_skeleton_gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_readback_audit_receipt_hash_preview_acceptance_skeleton_gate"
  and .source_receipt_hash_preview_acceptance_skeleton_status == "blocked"
  and .source_receipt_hash_preview_count == 2
  and .source_receipt_hash_accepted_count == 0
  and .source_receipt_recorded_count == 0
  and .source_receipt_persisted_count == 0
  and .source_receipt_delivered_count == 0
  and .source_receipt_accepted_count == 0
  and .source_receipt_materialized_count == 0
  and .source_acceptance_skeleton_declared_count == 2
  and .source_acceptance_skeleton_operator_input_required_count == 2
  and .source_acceptance_skeleton_operator_input_supplied_count == 0
  and .source_acceptance_skeleton_accepted_count == 0
  and .source_controlled_request_dispatch_budget_declared == 1
  and .source_controlled_request_dispatch_budget_accepted == false
  and .source_controlled_request_dispatch_budget_consumed == 0
  and .source_controlled_request_dispatch_budget_remaining == 0
  and (.source_payload_preview_hash_sha256 | type) == "string"
  and (.source_payload_preview_hash_sha256 | length) == 64
  and (.source_readback_receipt_hash_sha256 | type) == "string"
  and (.source_readback_receipt_hash_sha256 | length) == 64
  and (.source_audit_receipt_hash_sha256 | type) == "string"
  and (.source_audit_receipt_hash_sha256 | length) == 64
  and .dispatch_dry_run_noop_receipt_count == 1
  and .dispatch_dry_run_shape_declared_count == 1
  and .dispatch_intent_shape_declared_count == 1
  and .single_budget_shape_declared_count == 1
  and .single_budget_declared == 1
  and .single_budget_accepted == false
  and .single_budget_consumed == 0
  and .single_budget_remaining == 0
  and .dispatch_authority_accepted_count == 0
  and .dispatch_preconditions_satisfied_count == 0
  and .controlled_request_dispatch_ready_count == 0
  and .controlled_request_dispatch_allowed_count == 0
  and .controlled_request_dispatched_count == 0
  and .controlled_request_execution_allowed_count == 0
  and .controlled_request_executed_count == 0
  and .noop_receipt_shape_declared_count == 1
  and .noop_receipt_hash_shape_declared_count == 1
  and .noop_receipt_hash_bound_to_payload_preview_count == 1
  and .noop_receipt_hash_bound_to_receipt_hash_preview_count == 1
  and .noop_receipt_recorded_count == 0
  and .noop_receipt_persisted_count == 0
  and .noop_receipt_delivered_count == 0
  and .noop_receipt_accepted_count == 0
  and .noop_receipt_materialized_count == 0
  and .readback_receipt_hash_preview_accepted_count == 0
  and .audit_receipt_hash_preview_accepted_count == 0
  and .acceptance_skeleton_accepted_count == 0
  and .request_payload_materialized_count == 0
  and .request_payload_file_written_count == 0
  and .raw_payload_inspected_count == 0
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
  and .dispatch_dry_run_noop_receipt_negative_fixture_count == 7
  and .dispatch_dry_run_noop_receipt_blocked_negative_fixture_count == 7
  and .dispatch_dry_run_noop_receipt_allowed_negative_fixture_count == 0
  and (.dispatch_dry_run_noop_receipts | length) == 1
  and (.dispatch_dry_run_noop_receipts | all(
    .dispatch_dry_run_shape_declared == true
    and .dispatch_intent_shape_declared == true
    and .single_budget_shape_declared == true
    and .single_budget_declared == 1
    and .single_budget_accepted == false
    and .single_budget_consumed == 0
    and .single_budget_remaining == 0
    and .dispatch_authority_accepted == false
    and .dispatch_preconditions_satisfied == false
    and .dispatch_ready == false
    and .dispatch_allowed == false
    and .dispatch_performed == false
    and .execution_allowed == false
    and .execution_performed == false
    and .noop_receipt_shape_declared == true
    and .noop_receipt_hash_shape_declared == true
    and .noop_receipt_hash_bound_to_payload_preview == true
    and .noop_receipt_hash_bound_to_readback_receipt_hash == true
    and .noop_receipt_hash_bound_to_audit_receipt_hash == true
    and .noop_receipt_recorded == false
    and .noop_receipt_persisted == false
    and .noop_receipt_delivered == false
    and .noop_receipt_accepted == false
    and .noop_receipt_materialized == false
    and .readback_receipt_hash_preview_accepted == false
    and .audit_receipt_hash_preview_accepted == false
    and .acceptance_skeleton_accepted == false
    and .request_payload_materialized == false
    and .request_payload_file_written == false
    and .raw_payload_inspected == false
    and .context_injection_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .external_kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .channel_send_performed == false
    and .status == "blocked_single_budget_dispatch_dry_run_noop_receipt_only"
  ))
  and (.dispatch_dry_run_noop_receipt_negative_fixtures | all(
    .fixture_status == "blocked"
    and .single_budget_accepted == false
    and .single_budget_consumed == 0
    and .dispatch_performed == false
    and .execution_performed == false
    and .noop_receipt_recorded == false
    and .noop_receipt_persisted == false
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
  and .denied_by_dispatch_dry_run_noop_receipt_count == 16
  and (.denied_by_dispatch_dry_run_noop_receipt | length) == 16
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG operator canary controlled request harness single-budget dispatch dry-run no-op receipt gate passed"
