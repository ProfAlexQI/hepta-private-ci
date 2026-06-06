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

PAYLOAD_PREVIEW_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-redacted-payload-preview-no-materialization-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-redacted-payload-preview-no-materialization-gate.sh
)"

payload_preview_report_sha256="$(sha256_text "$PAYLOAD_PREVIEW_JSON")"
source_payload_preview_hash_sha256="$(
  jq -r '.payload_previews[0].payload_preview_hash_sha256' <<<"$PAYLOAD_PREVIEW_JSON"
)"
source_readback_preview_hash_sha256="$(
  jq -r '.payload_previews[0].readback_preview_hash_sha256' <<<"$PAYLOAD_PREVIEW_JSON"
)"
source_audit_preview_hash_sha256="$(
  jq -r '.payload_previews[0].audit_preview_hash_sha256' <<<"$PAYLOAD_PREVIEW_JSON"
)"

readback_receipt_hash_sha256="$(
  sha256_text "hepta-canary-readback-receipt-hash-preview-acceptance-skeleton:v1:payload-preview=$source_payload_preview_hash_sha256:source-readback=$source_readback_preview_hash_sha256:no-record:no-persist:no-dispatch"
)"
audit_receipt_hash_sha256="$(
  sha256_text "hepta-canary-audit-receipt-hash-preview-acceptance-skeleton:v1:payload-preview=$source_payload_preview_hash_sha256:source-audit=$source_audit_preview_hash_sha256:no-record:no-persist:no-dispatch"
)"
acceptance_skeleton_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-harness-readback-audit-receipt-hash-preview-acceptance-skeleton:v1:source-redacted-payload-preview:readback:audit:hash-only:no-record:no-persist:no-deliver:no-accept:no-dispatch:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "readback_audit_receipt_hash_preview_acceptance_skeleton_side_effects=false;receipt_previews=2;accepted=0;recorded=0;persisted=0;delivered=0;dispatch=0;execute=0;context=0;provider=0;model=0;memory=0;kg=0;secret=0"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$PAYLOAD_PREVIEW_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_redacted_payload_preview_no_materialization_gate"
    and $source.operator_canary_controlled_request_harness_redacted_payload_preview_no_materialization_ready == true
    and $source.operator_canary_controlled_request_harness_redacted_payload_preview_no_materialization_status == "blocked"
    and $source.payload_preview_count == 1
    and $source.payload_preview_shape_declared_count == 1
    and $source.payload_preview_report_only_count == 1
    and $source.payload_preview_hash_shape_declared_count == 1
    and $source.payload_preview_hash_accepted_count == 0
    and $source.payload_preview_accepted_count == 0
    and $source.payload_preview_recorded_count == 0
    and $source.payload_preview_persisted_count == 0
    and $source.payload_preview_delivered_count == 0
    and $source.request_payload_materialization_allowed_count == 0
    and $source.request_payload_materialized_count == 0
    and $source.request_payload_persisted_count == 0
    and $source.request_payload_file_written_count == 0
    and $source.raw_payload_inspected_count == 0
    and $source.redacted_payload_preview_contains_secret_count == 0
    and $source.redacted_payload_preview_contains_credential_count == 0
    and $source.redacted_payload_preview_contains_live_context_count == 0
    and $source.redacted_payload_preview_contains_provider_input_count == 0
    and $source.redacted_payload_preview_contains_memory_mutation_count == 0
    and $source.redacted_payload_preview_contains_kg_mutation_count == 0
    and $source.readback_receipt_preview_declared_count == 1
    and $source.readback_receipt_persisted_count == 0
    and $source.audit_receipt_preview_declared_count == 1
    and $source.audit_receipt_persisted_count == 0
    and $source.controlled_request_dispatch_budget_declared == 1
    and $source.controlled_request_dispatch_budget_accepted == false
    and $source.controlled_request_dispatch_budget_consumed == 0
    and $source.controlled_request_dispatch_budget_remaining == 0
    and $source.controlled_request_dispatched_count == 0
    and $source.controlled_request_executed_count == 0
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
    and ($source.payload_previews | length) == 1
    and ($source.payload_previews[0].payload_preview_hash_sha256 | type) == "string"
    and ($source.payload_previews[0].payload_preview_hash_sha256 | length) == 64
    and ($source.payload_previews[0].readback_preview_hash_sha256 | type) == "string"
    and ($source.payload_previews[0].readback_preview_hash_sha256 | length) == 64
    and ($source.payload_previews[0].audit_preview_hash_sha256 | type) == "string"
    and ($source.payload_previews[0].audit_preview_hash_sha256 | length) == 64
    and ($source.payload_previews | all(
      .payload_preview_status == "blocked_preview_only"
      and .payload_preview_report_only == true
      and .payload_preview_accepted == false
      and .payload_preview_recorded == false
      and .payload_preview_persisted == false
      and .payload_preview_delivered == false
      and .request_payload_materialization_allowed == false
      and .request_payload_materialized == false
      and .request_payload_file_written == false
      and .raw_payload_inspected == false
      and .readback_receipt_persistence_allowed == false
      and .readback_receipt_persisted == false
      and .audit_receipt_persistence_allowed == false
      and .audit_receipt_persisted == false
      and .controlled_request_dispatched == false
      and .controlled_request_executed == false
    ))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

negative_fixtures_json="$(
  jq -n '
    [
      {
        fixture_id: "missing-payload-preview-hash",
        fixture_kind: "source_hash_missing",
        denial_reason: "readback/audit receipt hash previews require the source payload preview hash"
      },
      {
        fixture_id: "payload-preview-acceptance-attempt",
        fixture_kind: "payload_preview_acceptance_attempt",
        denial_reason: "payload preview acceptance cannot substitute for operator approval"
      },
      {
        fixture_id: "readback-receipt-persistence-attempt",
        fixture_kind: "readback_receipt_persistence_attempt",
        denial_reason: "readback receipt hash preview cannot persist a receipt"
      },
      {
        fixture_id: "audit-receipt-persistence-attempt",
        fixture_kind: "audit_receipt_persistence_attempt",
        denial_reason: "audit receipt hash preview cannot persist a receipt"
      },
      {
        fixture_id: "dispatch-from-receipt-preview-attempt",
        fixture_kind: "controlled_request_dispatch_attempt",
        denial_reason: "receipt hash preview cannot authorize dispatch or execution"
      },
      {
        fixture_id: "receipt-hash-mismatch",
        fixture_kind: "receipt_hash_binding_mismatch",
        denial_reason: "receipt hash previews must bind to the current payload preview hash"
      }
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_readback_audit_receipt_hash_preview_acceptance_skeleton_gate" \
    --arg payload_preview_report_sha256 "$payload_preview_report_sha256" \
    --arg acceptance_skeleton_policy_hash_sha256 "$acceptance_skeleton_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --arg source_payload_preview_hash_sha256 "$source_payload_preview_hash_sha256" \
    --arg source_readback_preview_hash_sha256 "$source_readback_preview_hash_sha256" \
    --arg source_audit_preview_hash_sha256 "$source_audit_preview_hash_sha256" \
    --arg readback_receipt_hash_sha256 "$readback_receipt_hash_sha256" \
    --arg audit_receipt_hash_sha256 "$audit_receipt_hash_sha256" \
    --arg payload_preview_id "hepta-canary-controlled-request-single-route-single-namespace-redacted-payload-preview" \
    --arg route_id "hepta.memory_intelligence_kg.canary.single_route.preview" \
    --arg namespace_id "hepta-memory-intelligence-kg-canary-single-namespace" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$PAYLOAD_PREVIEW_JSON" \
    --argjson negative_fixtures "$negative_fixtures_json" \
    '
      ([
        {
          receipt_kind: "readback",
          receipt_preview_id: "hepta-canary-readback-receipt-hash-preview-acceptance-skeleton",
          source_preview_hash_sha256: $source_readback_preview_hash_sha256,
          receipt_hash_sha256: $readback_receipt_hash_sha256
        },
        {
          receipt_kind: "audit",
          receipt_preview_id: "hepta-canary-audit-receipt-hash-preview-acceptance-skeleton",
          source_preview_hash_sha256: $source_audit_preview_hash_sha256,
          receipt_hash_sha256: $audit_receipt_hash_sha256
        }
      ]
      | map(. + {
          payload_preview_id: $payload_preview_id,
          route_id: $route_id,
          namespace_id: $namespace_id,
          payload_preview_hash_sha256: $source_payload_preview_hash_sha256,
          receipt_preview_shape_declared: true,
          receipt_hash_shape_declared: true,
          receipt_hash_bound_to_payload_preview: true,
          receipt_hash_accepted: false,
          receipt_recorded: false,
          receipt_persisted: false,
          receipt_delivered: false,
          receipt_accepted: false,
          receipt_materialized: false,
          receipt_report_only: true,
          acceptance_skeleton_declared: true,
          acceptance_skeleton_operator_input_required: true,
          acceptance_skeleton_operator_input_supplied: false,
          acceptance_skeleton_recorded: false,
          acceptance_skeleton_persisted: false,
          acceptance_skeleton_accepted: false,
          authorizes_payload_materialization: false,
          authorizes_dispatch: false,
          authorizes_execution: false,
          authorizes_readback_persistence: false,
          authorizes_audit_persistence: false,
          authorizes_context_attachment: false,
          authorizes_provider_model_invocation: false,
          authorizes_memory_write: false,
          authorizes_external_kg_read: false,
          authorizes_live_kg_write: false,
          status: "blocked_receipt_hash_preview_acceptance_skeleton_only"
        })) as $receipt_previews
      | ($negative_fixtures | map(. + {
          fixture_status: "blocked",
          receipt_hash_accepted: false,
          receipt_recorded: false,
          receipt_persisted: false,
          receipt_delivered: false,
          receipt_accepted: false,
          receipt_materialized: false,
          acceptance_skeleton_accepted: false,
          payload_preview_accepted: false,
          request_payload_materialized: false,
          controlled_request_dispatched: false,
          controlled_request_executed: false,
          context_injection_performed: false,
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
          operator_canary_controlled_request_harness_readback_audit_receipt_hash_preview_acceptance_skeleton_schema_version: "memory_intelligence_kg_operator_canary_harness_readback_audit_receipt_hash_preview_acceptance_skeleton_v1",
          operator_canary_controlled_request_harness_readback_audit_receipt_hash_preview_acceptance_skeleton_ready: true,
          operator_canary_controlled_request_harness_readback_audit_receipt_hash_preview_acceptance_skeleton_status: "blocked",
          receipt_hash_preview_acceptance_skeleton_mode: "stdout_only_hash_bound_readback_audit_receipt_preview_no_record_no_persist_no_deliver_no_dispatch_no_live",
          receipt_hash_preview_acceptance_skeleton_decision: "readback_and_audit_receipt_hash_previews_are_bound_to_the_single_redacted_payload_preview_without_accepting_or_persisting_receipts",
          minimum_required_samples: $min_long_soak_samples,
          source_redacted_payload_preview_gate: $source.gate,
          source_redacted_payload_preview_status: $source.operator_canary_controlled_request_harness_redacted_payload_preview_no_materialization_status,
          source_redacted_payload_preview_report_sha256: $payload_preview_report_sha256,
          source_payload_preview_count: $source.payload_preview_count,
          source_payload_preview_hash_shape_declared_count: $source.payload_preview_hash_shape_declared_count,
          source_payload_preview_hash_accepted_count: $source.payload_preview_hash_accepted_count,
          source_request_payload_materialized_count: $source.request_payload_materialized_count,
          source_raw_payload_inspected_count: $source.raw_payload_inspected_count,
          source_readback_receipt_preview_declared_count: $source.readback_receipt_preview_declared_count,
          source_readback_receipt_persisted_count: $source.readback_receipt_persisted_count,
          source_audit_receipt_preview_declared_count: $source.audit_receipt_preview_declared_count,
          source_audit_receipt_persisted_count: $source.audit_receipt_persisted_count,
          receipt_hash_preview_acceptance_skeleton_policy_hash_sha256: $acceptance_skeleton_policy_hash_sha256,
          side_effect_hash_sha256: $side_effect_hash_sha256,
          payload_preview_id: $payload_preview_id,
          route_id: $route_id,
          namespace_id: $namespace_id,
          source_payload_preview_hash_sha256: $source_payload_preview_hash_sha256,
          source_readback_preview_hash_sha256: $source_readback_preview_hash_sha256,
          source_audit_preview_hash_sha256: $source_audit_preview_hash_sha256,
          receipt_hash_previews: $receipt_previews,
          receipt_hash_preview_count: ($receipt_previews | length),
          receipt_hash_preview_shape_declared_count: ($receipt_previews | map(select(.receipt_preview_shape_declared == true)) | length),
          receipt_hash_shape_declared_count: ($receipt_previews | map(select(.receipt_hash_shape_declared == true)) | length),
          receipt_hash_bound_to_payload_preview_count: ($receipt_previews | map(select(.receipt_hash_bound_to_payload_preview == true)) | length),
          readback_receipt_hash_preview_declared_count: ($receipt_previews | map(select(.receipt_kind == "readback")) | length),
          audit_receipt_hash_preview_declared_count: ($receipt_previews | map(select(.receipt_kind == "audit")) | length),
          receipt_hash_accepted_count: 0,
          receipt_recorded_count: 0,
          receipt_persisted_count: 0,
          receipt_delivered_count: 0,
          receipt_accepted_count: 0,
          receipt_materialized_count: 0,
          receipt_report_only_count: ($receipt_previews | map(select(.receipt_report_only == true)) | length),
          acceptance_skeleton_declared_count: ($receipt_previews | map(select(.acceptance_skeleton_declared == true)) | length),
          acceptance_skeleton_operator_input_required_count: ($receipt_previews | map(select(.acceptance_skeleton_operator_input_required == true)) | length),
          acceptance_skeleton_operator_input_supplied_count: 0,
          acceptance_skeleton_recorded_count: 0,
          acceptance_skeleton_persisted_count: 0,
          acceptance_skeleton_accepted_count: 0,
          payload_preview_hash_accepted: false,
          payload_preview_accepted: false,
          request_payload_materialization_allowed: false,
          request_payload_materialized: false,
          request_payload_file_written: false,
          raw_payload_inspected: false,
          controlled_request_dispatch_budget_declared: 1,
          controlled_request_dispatch_budget_accepted: false,
          controlled_request_dispatch_budget_consumed: 0,
          controlled_request_dispatch_budget_remaining: 0,
          controlled_request_dispatch_allowed: false,
          controlled_request_dispatched: false,
          controlled_request_execution_allowed: false,
          controlled_request_executed: false,
          readback_receipt_persistence_allowed: false,
          readback_receipt_persisted: false,
          audit_receipt_persistence_allowed: false,
          audit_receipt_persisted: false,
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
          canary_harness_arm_allowed: false,
          canary_harness_armed: false,
          canary_harness_executable: false,
          canary_live_enabled: false,
          receipt_hash_preview_negative_fixtures: $fixtures,
          receipt_hash_preview_negative_fixture_count: ($fixtures | length),
          receipt_hash_preview_blocked_negative_fixture_count: ($fixtures | map(select(.fixture_status == "blocked")) | length),
          receipt_hash_preview_allowed_negative_fixture_count: ($fixtures | map(select(.fixture_status == "allowed")) | length),
          denied_by_receipt_hash_preview_acceptance_skeleton: [
            "receipt_hash_preview_not_operator_approval",
            "payload_preview_hash_not_accepted",
            "readback_receipt_hash_not_accepted",
            "audit_receipt_hash_not_accepted",
            "acceptance_skeleton_not_recorded",
            "acceptance_skeleton_not_persisted",
            "acceptance_skeleton_not_accepted",
            "receipt_recording_denied",
            "receipt_persistence_denied",
            "receipt_delivery_denied",
            "receipt_materialization_denied",
            "request_payload_materialization_denied",
            "controlled_request_dispatch_denied",
            "controlled_request_execution_denied",
            "context_attachment_denied",
            "provider_model_invocation_denied",
            "memory_write_denied",
            "external_kg_read_denied",
            "live_kg_write_denied",
            "credential_secret_read_denied"
          ],
          denied_by_receipt_hash_preview_acceptance_skeleton_count: 20,
          side_effects: {
            workspace_written: false,
            filesystem_written: false,
            payload_preview_accepted: false,
            request_payload_materialized: false,
            request_payload_file_written: false,
            raw_payload_inspected: false,
            receipt_recorded: false,
            receipt_persisted: false,
            receipt_delivered: false,
            receipt_accepted: false,
            receipt_materialized: false,
            acceptance_skeleton_recorded: false,
            acceptance_skeleton_persisted: false,
            acceptance_skeleton_accepted: false,
            controlled_request_dispatched: false,
            controlled_request_executed: false,
            readback_receipt_persisted: false,
            audit_receipt_persisted: false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_readback_audit_receipt_hash_preview_acceptance_skeleton_gate"
  and .operator_canary_controlled_request_harness_readback_audit_receipt_hash_preview_acceptance_skeleton_ready == true
  and .operator_canary_controlled_request_harness_readback_audit_receipt_hash_preview_acceptance_skeleton_status == "blocked"
  and .source_redacted_payload_preview_gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_redacted_payload_preview_no_materialization_gate"
  and .source_redacted_payload_preview_status == "blocked"
  and .source_payload_preview_count == 1
  and .source_payload_preview_hash_shape_declared_count == 1
  and .source_payload_preview_hash_accepted_count == 0
  and .source_request_payload_materialized_count == 0
  and .source_raw_payload_inspected_count == 0
  and .source_readback_receipt_preview_declared_count == 1
  and .source_readback_receipt_persisted_count == 0
  and .source_audit_receipt_preview_declared_count == 1
  and .source_audit_receipt_persisted_count == 0
  and (.source_payload_preview_hash_sha256 | type) == "string"
  and (.source_payload_preview_hash_sha256 | length) == 64
  and (.source_readback_preview_hash_sha256 | type) == "string"
  and (.source_readback_preview_hash_sha256 | length) == 64
  and (.source_audit_preview_hash_sha256 | type) == "string"
  and (.source_audit_preview_hash_sha256 | length) == 64
  and .receipt_hash_preview_count == 2
  and .receipt_hash_preview_shape_declared_count == 2
  and .receipt_hash_shape_declared_count == 2
  and .receipt_hash_bound_to_payload_preview_count == 2
  and .readback_receipt_hash_preview_declared_count == 1
  and .audit_receipt_hash_preview_declared_count == 1
  and .receipt_hash_accepted_count == 0
  and .receipt_recorded_count == 0
  and .receipt_persisted_count == 0
  and .receipt_delivered_count == 0
  and .receipt_accepted_count == 0
  and .receipt_materialized_count == 0
  and .receipt_report_only_count == 2
  and .acceptance_skeleton_declared_count == 2
  and .acceptance_skeleton_operator_input_required_count == 2
  and .acceptance_skeleton_operator_input_supplied_count == 0
  and .acceptance_skeleton_recorded_count == 0
  and .acceptance_skeleton_persisted_count == 0
  and .acceptance_skeleton_accepted_count == 0
  and .payload_preview_hash_accepted == false
  and .payload_preview_accepted == false
  and .request_payload_materialization_allowed == false
  and .request_payload_materialized == false
  and .request_payload_file_written == false
  and .raw_payload_inspected == false
  and .controlled_request_dispatch_budget_declared == 1
  and .controlled_request_dispatch_budget_accepted == false
  and .controlled_request_dispatch_budget_consumed == 0
  and .controlled_request_dispatch_budget_remaining == 0
  and .controlled_request_dispatch_allowed == false
  and .controlled_request_dispatched == false
  and .controlled_request_execution_allowed == false
  and .controlled_request_executed == false
  and .readback_receipt_persistence_allowed == false
  and .readback_receipt_persisted == false
  and .audit_receipt_persistence_allowed == false
  and .audit_receipt_persisted == false
  and .context_injection_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .external_kg_adapter_read_performed == false
  and .live_kg_write_performed == false
  and .credential_read == false
  and .secret_file_read == false
  and .channel_send_performed == false
  and .canary_harness_armed == false
  and .canary_harness_executable == false
  and .canary_live_enabled == false
  and .receipt_hash_preview_negative_fixture_count == 6
  and .receipt_hash_preview_blocked_negative_fixture_count == 6
  and .receipt_hash_preview_allowed_negative_fixture_count == 0
  and (.receipt_hash_previews | all(
    .receipt_preview_shape_declared == true
    and .receipt_hash_shape_declared == true
    and .receipt_hash_bound_to_payload_preview == true
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
    and .authorizes_payload_materialization == false
    and .authorizes_dispatch == false
    and .authorizes_execution == false
    and .authorizes_context_attachment == false
    and .authorizes_provider_model_invocation == false
    and .authorizes_memory_write == false
    and .authorizes_external_kg_read == false
    and .authorizes_live_kg_write == false
    and .status == "blocked_receipt_hash_preview_acceptance_skeleton_only"
  ))
  and (.receipt_hash_preview_negative_fixtures | all(
    .fixture_status == "blocked"
    and .receipt_hash_accepted == false
    and .receipt_recorded == false
    and .receipt_persisted == false
    and .receipt_delivered == false
    and .receipt_accepted == false
    and .receipt_materialized == false
    and .acceptance_skeleton_accepted == false
    and .payload_preview_accepted == false
    and .request_payload_materialized == false
    and .controlled_request_dispatched == false
    and .controlled_request_executed == false
    and .context_injection_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .external_kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
  ))
  and .denied_by_receipt_hash_preview_acceptance_skeleton_count == 20
  and (.denied_by_receipt_hash_preview_acceptance_skeleton | length) == 20
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG operator canary controlled request harness readback/audit receipt hash preview acceptance skeleton gate passed"
