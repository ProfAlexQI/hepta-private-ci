#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

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
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-preview-no-write-sink-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-preview-no-write-sink-gate.sh
)"

payload_preview_report_sha256="$(sha256_text "$PAYLOAD_PREVIEW_JSON")"
readback_audit_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-controlled-request-payload-readback-audit-receipt-preview:v1:source-payload-preview-no-write-sink:report-only:no-record:no-persist:no-dispatch:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_canary_payload_readback_audit_receipt_preview_side_effects=false;readback_record=false;audit_record=false;receipt_record=false;persistence=false;dispatch=false;execute=false;context=false;provider=false;model=false;memory=false;kg=false;secret=false;restart=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$PAYLOAD_PREVIEW_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_preview_no_write_sink_gate"
    and $source.operator_canary_controlled_request_payload_preview_no_write_sink_ready == true
    and $source.operator_canary_controlled_request_payload_preview_no_write_sink_status == "blocked"
    and $source.source_dispatch_envelope_preview_ready == true
    and $source.source_dispatch_envelope_preview_status == "blocked"
    and $source.operator_canary_controlled_request_payload_preview_count == 5
    and $source.operator_canary_controlled_request_payload_preview_shape_declared_count == 5
    and $source.operator_canary_controlled_request_payload_preview_report_materialized_count == 5
    and $source.operator_canary_controlled_request_payload_preview_hash_shape_declared_count == 5
    and $source.operator_canary_controlled_request_payload_preview_hash_accepted_count == 0
    and $source.operator_canary_controlled_request_payload_preview_accepted_count == 0
    and $source.operator_canary_controlled_request_payload_preview_recorded_count == 0
    and $source.operator_canary_controlled_request_payload_preview_persisted_count == 0
    and $source.operator_canary_controlled_request_payload_preview_delivered_count == 0
    and $source.operator_canary_request_payload_materialized_count == 0
    and $source.operator_canary_request_payload_persisted_count == 0
    and $source.operator_canary_no_write_sink_contract_count == 5
    and $source.operator_canary_no_write_sink_filesystem_write_allowed_count == 0
    and $source.operator_canary_no_write_sink_filesystem_write_performed_count == 0
    and $source.operator_canary_no_write_sink_workspace_write_allowed_count == 0
    and $source.operator_canary_no_write_sink_workspace_write_performed_count == 0
    and $source.operator_canary_no_write_sink_external_write_allowed_count == 0
    and $source.operator_canary_no_write_sink_external_write_performed_count == 0
    and $source.operator_canary_audit_entry_shape_declared_count == 5
    and $source.operator_canary_audit_entry_recorded_count == 0
    and $source.operator_canary_readback_receipt_shape_declared_count == 5
    and $source.operator_canary_readback_receipt_recorded_count == 0
    and $source.operator_canary_controlled_request_dispatch_preconditions_satisfied_count == 0
    and $source.operator_canary_controlled_request_dispatch_allowed_count == 0
    and $source.operator_canary_controlled_request_dispatched_count == 0
    and $source.operator_canary_controlled_request_execution_allowed_count == 0
    and $source.operator_canary_controlled_request_executed_count == 0
    and $source.operator_canary_context_attachment_allowed_count == 0
    and $source.operator_canary_provider_model_invocation_allowed_count == 0
    and $source.operator_canary_memory_write_allowed_count == 0
    and $source.operator_canary_external_kg_read_allowed_count == 0
    and $source.operator_canary_live_kg_write_allowed_count == 0
    and $source.operator_canary_payload_preview_accepted == false
    and $source.operator_canary_payload_preview_authorizes_dispatch == false
    and $source.operator_canary_payload_preview_authorizes_live_execution == false
    and $source.controlled_request_dispatched == false
    and $source.controlled_request_executed == false
    and $source.request_payload_materialized == false
    and $source.request_payload_persisted == false
    and $source.context_injection_performed == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.memory_store_write_performed == false
    and $source.memory_store_mutated == false
    and $source.external_kg_adapter_read_performed == false
    and $source.live_kg_write_performed == false
    and $source.network_call_performed == false
    and $source.credential_read == false
    and $source.auth_secret_read == false
    and $source.secret_file_read == false
    and $source.channel_send_performed == false
    and $source.service_restarted == false
    and $source.active_binary_mutated == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_preview_gate" \
    --arg payload_preview_report_sha256 "$payload_preview_report_sha256" \
    --arg readback_audit_policy_hash_sha256 "$readback_audit_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --arg preview_id "hepta-memory-intelligence-kg-operator-canary-controlled-request-payload-readback-audit-receipt-preview-report-only-v1" \
    --arg preview_schema_id "hepta.memory_intelligence_kg.canary.controlled_request.payload_readback_audit_receipt_preview.v1" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$PAYLOAD_PREVIEW_JSON" \
    '
      [
        $source.operator_canary_controlled_request_payload_previews[]
        | {
            readback_audit_preview_order: .payload_preview_order,
            readback_audit_preview_id: ("hepta-canary-controlled-request-payload-readback-audit-receipt-preview-" + .stage_id),
            readback_audit_preview_schema_id: "hepta.memory_intelligence_kg.canary.controlled_request.payload_readback_audit_receipt_preview.v1",
            stage_id: .stage_id,
            source_phase_id: .source_phase_id,
            source_payload_preview_id: .payload_preview_id,
            source_dispatch_envelope_id: .source_dispatch_envelope_id,
            route_id: .route_id,
            namespace_id: .namespace_id,
            source_payload_preview_shape_declared: .payload_preview_shape_declared,
            source_payload_preview_report_materialized: .payload_preview_materialized_in_report,
            source_payload_preview_hash_shape_declared: .payload_preview_hash_shape_declared,
            source_payload_preview_hash_accepted: .payload_preview_hash_accepted,
            source_payload_preview_accepted: .payload_preview_accepted,
            source_payload_preview_recorded: .payload_preview_recorded,
            source_payload_preview_persisted: .payload_preview_persisted,
            source_payload_preview_delivered: .payload_preview_delivered,
            source_request_payload_materialized: .request_payload_materialized,
            source_request_payload_persisted: .request_payload_persisted,
            source_no_write_sink_contract_shape_declared: .no_write_sink_contract_shape_declared,
            source_no_write_sink_preview_only: .no_write_sink_preview_only,
            source_no_write_sink_filesystem_write_performed: .no_write_sink_filesystem_write_performed,
            source_no_write_sink_workspace_write_performed: .no_write_sink_workspace_write_performed,
            source_no_write_sink_external_write_performed: .no_write_sink_external_write_performed,
            source_audit_entry_shape_declared: .audit_entry_shape_declared,
            source_audit_entry_recorded: .audit_entry_recorded,
            source_readback_receipt_shape_declared: .readback_receipt_shape_declared,
            source_readback_receipt_recorded: .readback_receipt_recorded,
            payload_preview_report_hash_bound: true,
            payload_preview_report_sha256: $payload_preview_report_sha256,
            payload_readback_shape_declared: true,
            payload_readback_preview_materialized_in_report: true,
            payload_readback_hash_shape_declared: true,
            payload_readback_hash_matches_source_report: true,
            payload_readback_hash_accepted: false,
            payload_readback_proof_shape_declared: true,
            payload_readback_proof_accepted: false,
            audit_entry_preview_shape_declared: true,
            audit_entry_preview_materialized_in_report: true,
            audit_entry_recorded: false,
            audit_entry_persisted: false,
            audit_entry_delivered: false,
            readback_receipt_preview_shape_declared: true,
            readback_receipt_preview_materialized_in_report: true,
            readback_receipt_recorded: false,
            readback_receipt_persisted: false,
            readback_receipt_delivered: false,
            readback_receipt_accepted: false,
            payload_preview_accepted: false,
            payload_preview_recorded: false,
            payload_preview_persisted: false,
            request_payload_materialized: false,
            request_payload_persisted: false,
            no_write_sink_filesystem_write_performed: false,
            no_write_sink_workspace_write_performed: false,
            no_write_sink_external_write_performed: false,
            controlled_request_dispatch_preconditions_satisfied: false,
            controlled_request_dispatch_ready: false,
            controlled_request_dispatch_allowed: false,
            controlled_request_dispatched_count: 0,
            controlled_request_execution_allowed: false,
            controlled_request_executed_count: 0,
            context_attachment_allowed: false,
            context_attachment_performed: false,
            provider_model_invocation_allowed: false,
            provider_invoked: false,
            model_invoked: false,
            memory_write_allowed: false,
            memory_store_write_performed: false,
            external_kg_read_allowed: false,
            external_kg_adapter_read_performed: false,
            live_kg_write_allowed: false,
            live_kg_write_performed: false,
            network_call_allowed: false,
            network_call_performed: false,
            credential_read_allowed: false,
            credential_read: false,
            channel_delivery_allowed: false,
            channel_send_performed: false,
            status: "blocked_payload_readback_audit_receipt_preview_only"
          }
      ] as $readback_previews
      | {
          product: $product,
          runtime: $runtime,
          status: "ready",
          base_url: $base_url,
          gate: $gate,
          operator_canary_controlled_request_payload_readback_audit_receipt_preview_schema_version: "memory_intelligence_kg_operator_canary_controlled_request_payload_readback_audit_receipt_preview_v1",
          operator_canary_controlled_request_payload_readback_audit_receipt_preview_ready: true,
          operator_canary_controlled_request_payload_readback_audit_receipt_preview_status: "blocked",
          operator_canary_controlled_request_payload_readback_audit_receipt_preview_mode: "report_only_readback_audit_receipt_preview_no_record_no_persist_no_dispatch_no_execute_no_live",
          operator_canary_controlled_request_payload_readback_audit_receipt_preview_decision: "payload_preview_readback_audit_and_receipt_shapes_are_hash_bound_to_the_source_report_but_not_accepted_recorded_persisted_delivered_or_authorizing_dispatch",
          operator_canary_controlled_request_payload_readback_audit_receipt_preview_id: $preview_id,
          operator_canary_controlled_request_payload_readback_audit_receipt_preview_schema_id: $preview_schema_id,
          minimum_required_samples: $min_long_soak_samples,
          source_payload_preview_gate: $source.gate,
          source_payload_preview_report_sha256: $payload_preview_report_sha256,
          source_payload_preview_ready: $source.operator_canary_controlled_request_payload_preview_no_write_sink_ready,
          source_payload_preview_status: $source.operator_canary_controlled_request_payload_preview_no_write_sink_status,
          source_payload_preview_count: $source.operator_canary_controlled_request_payload_preview_count,
          source_payload_preview_report_materialized_count: $source.operator_canary_controlled_request_payload_preview_report_materialized_count,
          source_payload_preview_hash_shape_declared_count: $source.operator_canary_controlled_request_payload_preview_hash_shape_declared_count,
          source_payload_preview_hash_accepted_count: $source.operator_canary_controlled_request_payload_preview_hash_accepted_count,
          source_payload_preview_accepted_count: $source.operator_canary_controlled_request_payload_preview_accepted_count,
          source_payload_preview_recorded_count: $source.operator_canary_controlled_request_payload_preview_recorded_count,
          source_payload_preview_persisted_count: $source.operator_canary_controlled_request_payload_preview_persisted_count,
          source_request_payload_materialized_count: $source.operator_canary_request_payload_materialized_count,
          source_request_payload_persisted_count: $source.operator_canary_request_payload_persisted_count,
          source_no_write_sink_contract_count: $source.operator_canary_no_write_sink_contract_count,
          source_no_write_sink_filesystem_write_performed_count: $source.operator_canary_no_write_sink_filesystem_write_performed_count,
          source_no_write_sink_workspace_write_performed_count: $source.operator_canary_no_write_sink_workspace_write_performed_count,
          source_no_write_sink_external_write_performed_count: $source.operator_canary_no_write_sink_external_write_performed_count,
          source_audit_entry_shape_declared_count: $source.operator_canary_audit_entry_shape_declared_count,
          source_audit_entry_recorded_count: $source.operator_canary_audit_entry_recorded_count,
          source_readback_receipt_shape_declared_count: $source.operator_canary_readback_receipt_shape_declared_count,
          source_readback_receipt_recorded_count: $source.operator_canary_readback_receipt_recorded_count,
          readback_audit_policy_hash_sha256: $readback_audit_policy_hash_sha256,
          side_effect_hash_sha256: $side_effect_hash_sha256,
          operator_canary_controlled_request_payload_readback_audit_receipt_previews: $readback_previews,
          operator_canary_payload_readback_audit_receipt_preview_count: ($readback_previews | length),
          operator_canary_payload_preview_report_hash_bound_count: ($readback_previews | map(select(.payload_preview_report_hash_bound == true)) | length),
          operator_canary_payload_readback_shape_declared_count: ($readback_previews | map(select(.payload_readback_shape_declared == true)) | length),
          operator_canary_payload_readback_preview_report_materialized_count: ($readback_previews | map(select(.payload_readback_preview_materialized_in_report == true)) | length),
          operator_canary_payload_readback_hash_shape_declared_count: ($readback_previews | map(select(.payload_readback_hash_shape_declared == true)) | length),
          operator_canary_payload_readback_hash_matches_source_report_count: ($readback_previews | map(select(.payload_readback_hash_matches_source_report == true)) | length),
          operator_canary_payload_readback_hash_accepted_count: ($readback_previews | map(select(.payload_readback_hash_accepted == true)) | length),
          operator_canary_payload_readback_proof_shape_declared_count: ($readback_previews | map(select(.payload_readback_proof_shape_declared == true)) | length),
          operator_canary_payload_readback_proof_accepted_count: ($readback_previews | map(select(.payload_readback_proof_accepted == true)) | length),
          operator_canary_audit_entry_preview_shape_declared_count: ($readback_previews | map(select(.audit_entry_preview_shape_declared == true)) | length),
          operator_canary_audit_entry_preview_report_materialized_count: ($readback_previews | map(select(.audit_entry_preview_materialized_in_report == true)) | length),
          operator_canary_audit_entry_recorded_count: ($readback_previews | map(select(.audit_entry_recorded == true)) | length),
          operator_canary_audit_entry_persisted_count: ($readback_previews | map(select(.audit_entry_persisted == true)) | length),
          operator_canary_audit_entry_delivered_count: ($readback_previews | map(select(.audit_entry_delivered == true)) | length),
          operator_canary_readback_receipt_preview_shape_declared_count: ($readback_previews | map(select(.readback_receipt_preview_shape_declared == true)) | length),
          operator_canary_readback_receipt_preview_report_materialized_count: ($readback_previews | map(select(.readback_receipt_preview_materialized_in_report == true)) | length),
          operator_canary_readback_receipt_recorded_count: ($readback_previews | map(select(.readback_receipt_recorded == true)) | length),
          operator_canary_readback_receipt_persisted_count: ($readback_previews | map(select(.readback_receipt_persisted == true)) | length),
          operator_canary_readback_receipt_delivered_count: ($readback_previews | map(select(.readback_receipt_delivered == true)) | length),
          operator_canary_readback_receipt_accepted_count: ($readback_previews | map(select(.readback_receipt_accepted == true)) | length),
          operator_canary_payload_preview_accepted_count: ($readback_previews | map(select(.payload_preview_accepted == true)) | length),
          operator_canary_payload_preview_recorded_count: ($readback_previews | map(select(.payload_preview_recorded == true)) | length),
          operator_canary_payload_preview_persisted_count: ($readback_previews | map(select(.payload_preview_persisted == true)) | length),
          operator_canary_request_payload_materialized_count: ($readback_previews | map(select(.request_payload_materialized == true)) | length),
          operator_canary_request_payload_persisted_count: ($readback_previews | map(select(.request_payload_persisted == true)) | length),
          operator_canary_no_write_sink_filesystem_write_performed_count: ($readback_previews | map(select(.no_write_sink_filesystem_write_performed == true)) | length),
          operator_canary_no_write_sink_workspace_write_performed_count: ($readback_previews | map(select(.no_write_sink_workspace_write_performed == true)) | length),
          operator_canary_no_write_sink_external_write_performed_count: ($readback_previews | map(select(.no_write_sink_external_write_performed == true)) | length),
          operator_canary_controlled_request_dispatch_preconditions_satisfied_count: ($readback_previews | map(select(.controlled_request_dispatch_preconditions_satisfied == true)) | length),
          operator_canary_controlled_request_dispatch_ready_count: ($readback_previews | map(select(.controlled_request_dispatch_ready == true)) | length),
          operator_canary_controlled_request_dispatch_allowed_count: ($readback_previews | map(select(.controlled_request_dispatch_allowed == true)) | length),
          operator_canary_controlled_request_dispatched_count: ($readback_previews | map(.controlled_request_dispatched_count) | add),
          operator_canary_controlled_request_execution_allowed_count: ($readback_previews | map(select(.controlled_request_execution_allowed == true)) | length),
          operator_canary_controlled_request_executed_count: ($readback_previews | map(.controlled_request_executed_count) | add),
          operator_canary_context_attachment_allowed_count: ($readback_previews | map(select(.context_attachment_allowed == true)) | length),
          operator_canary_provider_model_invocation_allowed_count: ($readback_previews | map(select(.provider_model_invocation_allowed == true)) | length),
          operator_canary_memory_write_allowed_count: ($readback_previews | map(select(.memory_write_allowed == true)) | length),
          operator_canary_external_kg_read_allowed_count: ($readback_previews | map(select(.external_kg_read_allowed == true)) | length),
          operator_canary_live_kg_write_allowed_count: ($readback_previews | map(select(.live_kg_write_allowed == true)) | length),
          operator_canary_payload_readback_audit_receipt_preview_accepted: false,
          operator_canary_payload_readback_audit_receipt_preview_authorizes_dispatch: false,
          operator_canary_payload_readback_audit_receipt_preview_authorizes_live_execution: false,
          canary_harness_shape_ready: true,
          canary_harness_activation_ready: false,
          canary_harness_armed: false,
          canary_harness_executable: false,
          canary_live_enabled: false,
          canary_execution_performed: false,
          controlled_request_dispatched: false,
          controlled_request_executed: false,
          payload_preview_materialized_in_report: true,
          payload_readback_preview_materialized_in_report: true,
          request_payload_materialized: false,
          request_payload_persisted: false,
          audit_entry_recorded: false,
          audit_entry_persisted: false,
          readback_receipt_recorded: false,
          readback_receipt_persisted: false,
          readback_receipt_accepted: false,
          no_write_sink_filesystem_write_performed: false,
          no_write_sink_workspace_write_performed: false,
          no_write_sink_external_write_performed: false,
          runtime_router_mutated: false,
          router_handoff_recorded: false,
          hepta_intelligence_context_attached: false,
          live_context_attached_to_prompt: false,
          context_injection_performed: false,
          provider_invoked: false,
          model_invoked: false,
          memory_store_write_performed: false,
          memory_store_mutated: false,
          external_kg_adapter_read_performed: false,
          live_kg_write_performed: false,
          network_call_performed: false,
          external_db_write_performed: false,
          credential_read: false,
          auth_secret_read: false,
          secret_file_read: false,
          channel_send_performed: false,
          telegram_send_performed: false,
          external_send_performed: false,
          install_performed: false,
          service_restarted: false,
          active_binary_mutated: false,
          upstream_fetch_performed: false,
          upstream_merge_performed: false,
          denied_by_operator_canary_payload_readback_audit_receipt_preview: [
            "payload_preview_hash_not_accepted",
            "payload_readback_hash_not_accepted",
            "payload_readback_proof_not_accepted",
            "audit_entry_not_recorded_or_persisted",
            "readback_receipt_not_recorded_persisted_or_accepted",
            "payload_preview_not_recorded_or_persisted",
            "request_payload_not_materialized_or_persisted",
            "no_write_sink_allows_no_filesystem_workspace_or_external_write",
            "controlled_request_dispatch_denied",
            "controlled_request_execution_denied",
            "context_attachment_denied",
            "provider_model_invocation_denied",
            "memory_kg_live_mutation_denied",
            "credential_secret_read_denied",
            "install_restart_active_binary_mutation_denied"
          ],
          side_effects: {
            workspace_written: false,
            filesystem_written: false,
            payload_preview_recorded: false,
            payload_preview_persisted: false,
            payload_readback_recorded: false,
            payload_readback_persisted: false,
            audit_entry_recorded: false,
            audit_entry_persisted: false,
            readback_receipt_recorded: false,
            readback_receipt_persisted: false,
            readback_receipt_accepted: false,
            request_payload_materialized: false,
            request_payload_persisted: false,
            no_write_sink_filesystem_write_performed: false,
            no_write_sink_workspace_write_performed: false,
            no_write_sink_external_write_performed: false,
            controlled_request_dispatched: false,
            controlled_request_executed: false,
            runtime_router_mutated: false,
            router_handoff_recorded: false,
            hepta_intelligence_context_attached: false,
            live_context_attached_to_prompt: false,
            context_injection_performed: false,
            provider_invoked: false,
            model_invoked: false,
            memory_store_write_performed: false,
            memory_store_mutated: false,
            external_kg_adapter_read_performed: false,
            live_kg_write_performed: false,
            network_call_performed: false,
            external_db_write_performed: false,
            credential_read: false,
            auth_secret_read: false,
            secret_file_read: false,
            channel_send_performed: false,
            telegram_send_performed: false,
            external_send_performed: false,
            install_performed: false,
            service_restarted: false,
            active_binary_mutated: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false
          }
        }
    ')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_preview_gate"
  and .operator_canary_controlled_request_payload_readback_audit_receipt_preview_ready == true
  and .operator_canary_controlled_request_payload_readback_audit_receipt_preview_status == "blocked"
  and .source_payload_preview_gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_preview_no_write_sink_gate"
  and .source_payload_preview_ready == true
  and .source_payload_preview_status == "blocked"
  and .source_payload_preview_count == 5
  and .source_payload_preview_report_materialized_count == 5
  and .source_payload_preview_hash_shape_declared_count == 5
  and .source_payload_preview_hash_accepted_count == 0
  and .source_payload_preview_accepted_count == 0
  and .source_payload_preview_recorded_count == 0
  and .source_payload_preview_persisted_count == 0
  and .source_request_payload_materialized_count == 0
  and .source_request_payload_persisted_count == 0
  and .source_no_write_sink_contract_count == 5
  and .source_no_write_sink_filesystem_write_performed_count == 0
  and .source_no_write_sink_workspace_write_performed_count == 0
  and .source_no_write_sink_external_write_performed_count == 0
  and .source_audit_entry_shape_declared_count == 5
  and .source_audit_entry_recorded_count == 0
  and .source_readback_receipt_shape_declared_count == 5
  and .source_readback_receipt_recorded_count == 0
  and .operator_canary_payload_readback_audit_receipt_preview_count == 5
  and .operator_canary_payload_preview_report_hash_bound_count == 5
  and .operator_canary_payload_readback_shape_declared_count == 5
  and .operator_canary_payload_readback_preview_report_materialized_count == 5
  and .operator_canary_payload_readback_hash_shape_declared_count == 5
  and .operator_canary_payload_readback_hash_matches_source_report_count == 5
  and .operator_canary_payload_readback_hash_accepted_count == 0
  and .operator_canary_payload_readback_proof_shape_declared_count == 5
  and .operator_canary_payload_readback_proof_accepted_count == 0
  and .operator_canary_audit_entry_preview_shape_declared_count == 5
  and .operator_canary_audit_entry_preview_report_materialized_count == 5
  and .operator_canary_audit_entry_recorded_count == 0
  and .operator_canary_audit_entry_persisted_count == 0
  and .operator_canary_audit_entry_delivered_count == 0
  and .operator_canary_readback_receipt_preview_shape_declared_count == 5
  and .operator_canary_readback_receipt_preview_report_materialized_count == 5
  and .operator_canary_readback_receipt_recorded_count == 0
  and .operator_canary_readback_receipt_persisted_count == 0
  and .operator_canary_readback_receipt_delivered_count == 0
  and .operator_canary_readback_receipt_accepted_count == 0
  and .operator_canary_payload_preview_accepted_count == 0
  and .operator_canary_payload_preview_recorded_count == 0
  and .operator_canary_payload_preview_persisted_count == 0
  and .operator_canary_request_payload_materialized_count == 0
  and .operator_canary_request_payload_persisted_count == 0
  and .operator_canary_no_write_sink_filesystem_write_performed_count == 0
  and .operator_canary_no_write_sink_workspace_write_performed_count == 0
  and .operator_canary_no_write_sink_external_write_performed_count == 0
  and .operator_canary_controlled_request_dispatch_preconditions_satisfied_count == 0
  and .operator_canary_controlled_request_dispatch_ready_count == 0
  and .operator_canary_controlled_request_dispatch_allowed_count == 0
  and .operator_canary_controlled_request_dispatched_count == 0
  and .operator_canary_controlled_request_execution_allowed_count == 0
  and .operator_canary_controlled_request_executed_count == 0
  and .operator_canary_context_attachment_allowed_count == 0
  and .operator_canary_provider_model_invocation_allowed_count == 0
  and .operator_canary_memory_write_allowed_count == 0
  and .operator_canary_external_kg_read_allowed_count == 0
  and .operator_canary_live_kg_write_allowed_count == 0
  and (.operator_canary_controlled_request_payload_readback_audit_receipt_previews | all(
    .source_payload_preview_shape_declared == true
    and .source_payload_preview_report_materialized == true
    and .source_payload_preview_hash_shape_declared == true
    and .source_payload_preview_hash_accepted == false
    and .source_payload_preview_accepted == false
    and .source_payload_preview_recorded == false
    and .source_payload_preview_persisted == false
    and .source_payload_preview_delivered == false
    and .source_request_payload_materialized == false
    and .source_request_payload_persisted == false
    and .source_no_write_sink_contract_shape_declared == true
    and .source_no_write_sink_preview_only == true
    and .source_no_write_sink_filesystem_write_performed == false
    and .source_no_write_sink_workspace_write_performed == false
    and .source_no_write_sink_external_write_performed == false
    and .source_audit_entry_shape_declared == true
    and .source_audit_entry_recorded == false
    and .source_readback_receipt_shape_declared == true
    and .source_readback_receipt_recorded == false
    and .payload_preview_report_hash_bound == true
    and .payload_readback_shape_declared == true
    and .payload_readback_preview_materialized_in_report == true
    and .payload_readback_hash_shape_declared == true
    and .payload_readback_hash_matches_source_report == true
    and .payload_readback_hash_accepted == false
    and .payload_readback_proof_shape_declared == true
    and .payload_readback_proof_accepted == false
    and .audit_entry_preview_shape_declared == true
    and .audit_entry_preview_materialized_in_report == true
    and .audit_entry_recorded == false
    and .audit_entry_persisted == false
    and .audit_entry_delivered == false
    and .readback_receipt_preview_shape_declared == true
    and .readback_receipt_preview_materialized_in_report == true
    and .readback_receipt_recorded == false
    and .readback_receipt_persisted == false
    and .readback_receipt_delivered == false
    and .readback_receipt_accepted == false
    and .payload_preview_accepted == false
    and .payload_preview_recorded == false
    and .payload_preview_persisted == false
    and .request_payload_materialized == false
    and .request_payload_persisted == false
    and .no_write_sink_filesystem_write_performed == false
    and .no_write_sink_workspace_write_performed == false
    and .no_write_sink_external_write_performed == false
    and .controlled_request_dispatch_preconditions_satisfied == false
    and .controlled_request_dispatch_ready == false
    and .controlled_request_dispatch_allowed == false
    and .controlled_request_dispatched_count == 0
    and .controlled_request_execution_allowed == false
    and .controlled_request_executed_count == 0
    and .context_attachment_allowed == false
    and .context_attachment_performed == false
    and .provider_model_invocation_allowed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_write_allowed == false
    and .memory_store_write_performed == false
    and .external_kg_read_allowed == false
    and .external_kg_adapter_read_performed == false
    and .live_kg_write_allowed == false
    and .live_kg_write_performed == false
    and .network_call_allowed == false
    and .network_call_performed == false
    and .credential_read_allowed == false
    and .credential_read == false
    and .channel_delivery_allowed == false
    and .channel_send_performed == false
    and .status == "blocked_payload_readback_audit_receipt_preview_only"
  ))
  and .operator_canary_payload_readback_audit_receipt_preview_accepted == false
  and .operator_canary_payload_readback_audit_receipt_preview_authorizes_dispatch == false
  and .operator_canary_payload_readback_audit_receipt_preview_authorizes_live_execution == false
  and .canary_harness_shape_ready == true
  and .canary_harness_activation_ready == false
  and .canary_harness_armed == false
  and .canary_harness_executable == false
  and .canary_live_enabled == false
  and .canary_execution_performed == false
  and .controlled_request_dispatched == false
  and .controlled_request_executed == false
  and .payload_preview_materialized_in_report == true
  and .payload_readback_preview_materialized_in_report == true
  and .request_payload_materialized == false
  and .request_payload_persisted == false
  and .audit_entry_recorded == false
  and .audit_entry_persisted == false
  and .readback_receipt_recorded == false
  and .readback_receipt_persisted == false
  and .readback_receipt_accepted == false
  and .no_write_sink_filesystem_write_performed == false
  and .no_write_sink_workspace_write_performed == false
  and .no_write_sink_external_write_performed == false
  and .runtime_router_mutated == false
  and .router_handoff_recorded == false
  and .hepta_intelligence_context_attached == false
  and .live_context_attached_to_prompt == false
  and .context_injection_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .external_kg_adapter_read_performed == false
  and .live_kg_write_performed == false
  and .network_call_performed == false
  and .external_db_write_performed == false
  and .credential_read == false
  and .auth_secret_read == false
  and .secret_file_read == false
  and .channel_send_performed == false
  and .telegram_send_performed == false
  and .external_send_performed == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG controlled request payload readback audit receipt preview gate passed"
