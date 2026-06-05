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

DISPATCH_ENVELOPE_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-dispatch-envelope-preview-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-dispatch-envelope-preview-gate.sh
)"

dispatch_envelope_report_sha256="$(sha256_text "$DISPATCH_ENVELOPE_JSON")"
payload_preview_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-controlled-request-payload-preview-no-write-sink:v1:source-dispatch-envelope:report-only:no-payload-file:no-dispatch:no-execute:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_canary_controlled_request_payload_preview_no_write_sink_side_effects=false;payload_file=false;sink_write=false;dispatch=false;execute=false;context=false;provider=false;model=false;memory=false;kg=false;secret=false;restart=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$DISPATCH_ENVELOPE_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_dispatch_envelope_preview_gate"
    and $source.operator_canary_dispatch_envelope_preview_ready == true
    and $source.operator_canary_dispatch_envelope_preview_status == "blocked"
    and $source.operator_canary_dispatch_envelope_count == 5
    and $source.operator_canary_dispatch_envelope_shape_declared_count == 5
    and $source.operator_canary_dispatch_envelope_preview_shape_ready_count == 5
    and $source.operator_canary_dispatch_envelope_preview_accepted_count == 0
    and $source.operator_canary_dispatch_preconditions_satisfied_count == 0
    and $source.operator_canary_controlled_request_budget_total == 5
    and $source.operator_canary_controlled_request_budget_accepted_count == 0
    and $source.operator_canary_controlled_request_dispatch_ready_count == 0
    and $source.operator_canary_controlled_request_dispatch_allowed_count == 0
    and $source.operator_canary_controlled_request_dispatched_count == 0
    and $source.operator_canary_controlled_request_execution_allowed_count == 0
    and $source.operator_canary_controlled_request_executed_count == 0
    and $source.operator_canary_request_payload_materialized_count == 0
    and $source.operator_canary_request_payload_persisted_count == 0
    and $source.operator_canary_context_attachment_allowed_count == 0
    and $source.operator_canary_provider_model_invocation_allowed_count == 0
    and $source.operator_canary_memory_write_allowed_count == 0
    and $source.operator_canary_external_kg_read_allowed_count == 0
    and $source.operator_canary_live_kg_write_allowed_count == 0
    and $source.operator_canary_dispatch_envelope_accepted == false
    and $source.operator_canary_dispatch_envelope_authorizes_dispatch == false
    and $source.operator_canary_dispatch_envelope_authorizes_live_execution == false
    and $source.controlled_request_dispatched == false
    and $source.controlled_request_executed == false
    and $source.request_payload_materialized == false
    and $source.request_payload_persisted == false
    and $source.context_injection_performed == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.memory_store_write_performed == false
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
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_preview_no_write_sink_gate" \
    --arg dispatch_envelope_report_sha256 "$dispatch_envelope_report_sha256" \
    --arg payload_preview_policy_hash_sha256 "$payload_preview_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --arg preview_id "hepta-memory-intelligence-kg-operator-canary-controlled-request-payload-preview-no-write-sink-report-only-v1" \
    --arg preview_schema_id "hepta.memory_intelligence_kg.canary.controlled_request.payload_preview_no_write_sink.v1" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$DISPATCH_ENVELOPE_JSON" \
    '
      [
        $source.operator_canary_dispatch_envelopes[]
        | {
            payload_preview_order: .dispatch_envelope_order,
            payload_preview_id: ("hepta-canary-controlled-request-payload-preview-" + .stage_id),
            payload_preview_schema_id: "hepta.memory_intelligence_kg.canary.controlled_request.payload_preview.v1",
            no_write_sink_contract_id: ("hepta-canary-controlled-request-no-write-sink-" + .stage_id),
            no_write_sink_contract_schema_id: "hepta.memory_intelligence_kg.canary.no_write_sink_contract.v1",
            stage_id: .stage_id,
            source_phase_id: .source_phase_id,
            source_dispatch_envelope_id: .dispatch_envelope_id,
            route_id: .route_id,
            namespace_id: .namespace_id,
            source_fixture_bound: .source_fixture_bound,
            source_dispatch_envelope_shape_declared: .dispatch_envelope_shape_declared,
            source_dispatch_envelope_preview_shape_ready: .dispatch_envelope_preview_shape_ready,
            source_dispatch_envelope_preview_accepted: .dispatch_envelope_preview_accepted,
            source_dispatch_preconditions_satisfied: .controlled_request_dispatch_preconditions_satisfied,
            source_dispatch_ready: .controlled_request_dispatch_ready,
            source_dispatch_allowed: .controlled_request_dispatch_allowed,
            source_payload_materialized: .request_payload_materialized,
            source_payload_persisted: .request_payload_persisted,
            controlled_request_method: .request_method,
            controlled_request_budget: .controlled_request_budget,
            controlled_request_route_bound: true,
            controlled_request_namespace_bound: true,
            payload_preview_shape_declared: true,
            payload_preview_materialized_in_report: true,
            payload_preview_contains_redacted_fields_only: true,
            payload_preview_contains_secret: false,
            payload_preview_contains_credential: false,
            payload_preview_contains_live_context: false,
            payload_preview_contains_provider_input: false,
            payload_preview_contains_memory_mutation: false,
            payload_preview_contains_kg_mutation: false,
            payload_preview_hash_shape_declared: true,
            payload_preview_hash_accepted: false,
            payload_preview_accepted: false,
            payload_preview_recorded: false,
            payload_preview_persisted: false,
            payload_preview_delivered: false,
            request_payload_materialized: false,
            request_payload_persisted: false,
            no_write_sink_contract_shape_declared: true,
            no_write_sink_preview_only: true,
            no_write_sink_filesystem_write_allowed: false,
            no_write_sink_filesystem_write_performed: false,
            no_write_sink_workspace_write_allowed: false,
            no_write_sink_workspace_write_performed: false,
            no_write_sink_external_write_allowed: false,
            no_write_sink_external_write_performed: false,
            no_write_sink_dispatch_allowed: false,
            no_write_sink_dispatch_performed: false,
            redaction_proof_shape_declared: true,
            redaction_proof_accepted: false,
            audit_entry_shape_declared: true,
            audit_entry_recorded: false,
            readback_receipt_shape_declared: true,
            readback_receipt_recorded: false,
            idempotency_nonce_shape_declared: true,
            idempotency_nonce_accepted: false,
            rollback_kill_switch_shape_declared: true,
            rollback_kill_switch_armed: false,
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
            status: "blocked_payload_preview_no_write_sink_only"
          }
      ] as $payload_previews
      | {
          product: $product,
          runtime: $runtime,
          status: "ready",
          base_url: $base_url,
          gate: $gate,
          operator_canary_controlled_request_payload_preview_no_write_sink_schema_version: "memory_intelligence_kg_operator_canary_controlled_request_payload_preview_no_write_sink_v1",
          operator_canary_controlled_request_payload_preview_no_write_sink_ready: true,
          operator_canary_controlled_request_payload_preview_no_write_sink_status: "blocked",
          operator_canary_controlled_request_payload_preview_no_write_sink_mode: "report_only_payload_preview_no_sink_write_no_dispatch_no_execute_no_live",
          operator_canary_controlled_request_payload_preview_no_write_sink_decision: "controlled_request_payloads_are_previewed_inside_the_report_only_json_shape_but_no_payload_is_written_delivered_dispatched_or_executed",
          operator_canary_controlled_request_payload_preview_no_write_sink_id: $preview_id,
          operator_canary_controlled_request_payload_preview_no_write_sink_schema_id: $preview_schema_id,
          minimum_required_samples: $min_long_soak_samples,
          source_dispatch_envelope_gate: $source.gate,
          source_dispatch_envelope_report_sha256: $dispatch_envelope_report_sha256,
          source_dispatch_envelope_preview_ready: $source.operator_canary_dispatch_envelope_preview_ready,
          source_dispatch_envelope_preview_status: $source.operator_canary_dispatch_envelope_preview_status,
          source_dispatch_envelope_count: $source.operator_canary_dispatch_envelope_count,
          source_dispatch_envelope_preview_shape_ready_count: $source.operator_canary_dispatch_envelope_preview_shape_ready_count,
          source_dispatch_envelope_preview_accepted_count: $source.operator_canary_dispatch_envelope_preview_accepted_count,
          source_dispatch_preconditions_satisfied_count: $source.operator_canary_dispatch_preconditions_satisfied_count,
          source_controlled_request_budget_total: $source.operator_canary_controlled_request_budget_total,
          source_controlled_request_dispatch_allowed_count: $source.operator_canary_controlled_request_dispatch_allowed_count,
          source_controlled_request_dispatched_count: $source.operator_canary_controlled_request_dispatched_count,
          source_controlled_request_executed_count: $source.operator_canary_controlled_request_executed_count,
          source_request_payload_materialized_count: $source.operator_canary_request_payload_materialized_count,
          source_request_payload_persisted_count: $source.operator_canary_request_payload_persisted_count,
          payload_preview_policy_hash_sha256: $payload_preview_policy_hash_sha256,
          side_effect_hash_sha256: $side_effect_hash_sha256,
          operator_canary_controlled_request_payload_previews: $payload_previews,
          operator_canary_controlled_request_payload_preview_count: ($payload_previews | length),
          operator_canary_controlled_request_payload_preview_shape_declared_count: ($payload_previews | map(select(.payload_preview_shape_declared == true)) | length),
          operator_canary_controlled_request_payload_preview_report_materialized_count: ($payload_previews | map(select(.payload_preview_materialized_in_report == true)) | length),
          operator_canary_controlled_request_payload_preview_hash_shape_declared_count: ($payload_previews | map(select(.payload_preview_hash_shape_declared == true)) | length),
          operator_canary_controlled_request_payload_preview_hash_accepted_count: ($payload_previews | map(select(.payload_preview_hash_accepted == true)) | length),
          operator_canary_controlled_request_payload_preview_accepted_count: ($payload_previews | map(select(.payload_preview_accepted == true)) | length),
          operator_canary_controlled_request_payload_preview_recorded_count: ($payload_previews | map(select(.payload_preview_recorded == true)) | length),
          operator_canary_controlled_request_payload_preview_persisted_count: ($payload_previews | map(select(.payload_preview_persisted == true)) | length),
          operator_canary_controlled_request_payload_preview_delivered_count: ($payload_previews | map(select(.payload_preview_delivered == true)) | length),
          operator_canary_request_payload_materialized_count: ($payload_previews | map(select(.request_payload_materialized == true)) | length),
          operator_canary_request_payload_persisted_count: ($payload_previews | map(select(.request_payload_persisted == true)) | length),
          operator_canary_no_write_sink_contract_count: ($payload_previews | map(select(.no_write_sink_contract_shape_declared == true)) | length),
          operator_canary_no_write_sink_filesystem_write_allowed_count: ($payload_previews | map(select(.no_write_sink_filesystem_write_allowed == true)) | length),
          operator_canary_no_write_sink_filesystem_write_performed_count: ($payload_previews | map(select(.no_write_sink_filesystem_write_performed == true)) | length),
          operator_canary_no_write_sink_workspace_write_allowed_count: ($payload_previews | map(select(.no_write_sink_workspace_write_allowed == true)) | length),
          operator_canary_no_write_sink_workspace_write_performed_count: ($payload_previews | map(select(.no_write_sink_workspace_write_performed == true)) | length),
          operator_canary_no_write_sink_external_write_allowed_count: ($payload_previews | map(select(.no_write_sink_external_write_allowed == true)) | length),
          operator_canary_no_write_sink_external_write_performed_count: ($payload_previews | map(select(.no_write_sink_external_write_performed == true)) | length),
          operator_canary_redaction_proof_shape_declared_count: ($payload_previews | map(select(.redaction_proof_shape_declared == true)) | length),
          operator_canary_redaction_proof_accepted_count: ($payload_previews | map(select(.redaction_proof_accepted == true)) | length),
          operator_canary_audit_entry_shape_declared_count: ($payload_previews | map(select(.audit_entry_shape_declared == true)) | length),
          operator_canary_audit_entry_recorded_count: ($payload_previews | map(select(.audit_entry_recorded == true)) | length),
          operator_canary_readback_receipt_shape_declared_count: ($payload_previews | map(select(.readback_receipt_shape_declared == true)) | length),
          operator_canary_readback_receipt_recorded_count: ($payload_previews | map(select(.readback_receipt_recorded == true)) | length),
          operator_canary_idempotency_nonce_shape_declared_count: ($payload_previews | map(select(.idempotency_nonce_shape_declared == true)) | length),
          operator_canary_idempotency_nonce_accepted_count: ($payload_previews | map(select(.idempotency_nonce_accepted == true)) | length),
          operator_canary_controlled_request_budget_total: ($payload_previews | map(.controlled_request_budget) | add),
          operator_canary_controlled_request_dispatch_preconditions_satisfied_count: ($payload_previews | map(select(.controlled_request_dispatch_preconditions_satisfied == true)) | length),
          operator_canary_controlled_request_dispatch_ready_count: ($payload_previews | map(select(.controlled_request_dispatch_ready == true)) | length),
          operator_canary_controlled_request_dispatch_allowed_count: ($payload_previews | map(select(.controlled_request_dispatch_allowed == true)) | length),
          operator_canary_controlled_request_dispatched_count: ($payload_previews | map(.controlled_request_dispatched_count) | add),
          operator_canary_controlled_request_execution_allowed_count: ($payload_previews | map(select(.controlled_request_execution_allowed == true)) | length),
          operator_canary_controlled_request_executed_count: ($payload_previews | map(.controlled_request_executed_count) | add),
          operator_canary_context_attachment_allowed_count: ($payload_previews | map(select(.context_attachment_allowed == true)) | length),
          operator_canary_provider_model_invocation_allowed_count: ($payload_previews | map(select(.provider_model_invocation_allowed == true)) | length),
          operator_canary_memory_write_allowed_count: ($payload_previews | map(select(.memory_write_allowed == true)) | length),
          operator_canary_external_kg_read_allowed_count: ($payload_previews | map(select(.external_kg_read_allowed == true)) | length),
          operator_canary_live_kg_write_allowed_count: ($payload_previews | map(select(.live_kg_write_allowed == true)) | length),
          operator_canary_dispatch_envelope_accepted: false,
          operator_canary_dispatch_envelope_authorizes_dispatch: false,
          operator_canary_dispatch_envelope_authorizes_live_execution: false,
          operator_canary_payload_preview_accepted: false,
          operator_canary_payload_preview_authorizes_dispatch: false,
          operator_canary_payload_preview_authorizes_live_execution: false,
          canary_harness_shape_ready: true,
          canary_harness_activation_ready: false,
          canary_harness_armed: false,
          canary_harness_executable: false,
          canary_live_enabled: false,
          canary_execution_performed: false,
          controlled_request_dispatched: false,
          controlled_request_executed: false,
          payload_preview_materialized_in_report: true,
          request_payload_materialized: false,
          request_payload_persisted: false,
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
          denied_by_operator_canary_payload_preview_no_write_sink: [
            "dispatch_envelope_acceptance_missing",
            "dispatch_preconditions_not_satisfied",
            "payload_preview_hash_not_accepted",
            "payload_preview_not_recorded_or_persisted",
            "no_write_sink_allows_no_filesystem_workspace_or_external_write",
            "audit_entry_not_recorded",
            "readback_receipt_not_recorded",
            "idempotency_nonce_not_accepted",
            "redaction_proof_not_accepted",
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
            payload_preview_delivered: false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_preview_no_write_sink_gate"
  and .operator_canary_controlled_request_payload_preview_no_write_sink_ready == true
  and .operator_canary_controlled_request_payload_preview_no_write_sink_status == "blocked"
  and .source_dispatch_envelope_gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_dispatch_envelope_preview_gate"
  and .source_dispatch_envelope_preview_ready == true
  and .source_dispatch_envelope_preview_status == "blocked"
  and .source_dispatch_envelope_count == 5
  and .source_dispatch_envelope_preview_shape_ready_count == 5
  and .source_dispatch_envelope_preview_accepted_count == 0
  and .source_dispatch_preconditions_satisfied_count == 0
  and .source_controlled_request_budget_total == 5
  and .source_controlled_request_dispatch_allowed_count == 0
  and .source_controlled_request_dispatched_count == 0
  and .source_controlled_request_executed_count == 0
  and .source_request_payload_materialized_count == 0
  and .source_request_payload_persisted_count == 0
  and .operator_canary_controlled_request_payload_preview_count == 5
  and .operator_canary_controlled_request_payload_preview_shape_declared_count == 5
  and .operator_canary_controlled_request_payload_preview_report_materialized_count == 5
  and .operator_canary_controlled_request_payload_preview_hash_shape_declared_count == 5
  and .operator_canary_controlled_request_payload_preview_hash_accepted_count == 0
  and .operator_canary_controlled_request_payload_preview_accepted_count == 0
  and .operator_canary_controlled_request_payload_preview_recorded_count == 0
  and .operator_canary_controlled_request_payload_preview_persisted_count == 0
  and .operator_canary_controlled_request_payload_preview_delivered_count == 0
  and .operator_canary_request_payload_materialized_count == 0
  and .operator_canary_request_payload_persisted_count == 0
  and .operator_canary_no_write_sink_contract_count == 5
  and .operator_canary_no_write_sink_filesystem_write_allowed_count == 0
  and .operator_canary_no_write_sink_filesystem_write_performed_count == 0
  and .operator_canary_no_write_sink_workspace_write_allowed_count == 0
  and .operator_canary_no_write_sink_workspace_write_performed_count == 0
  and .operator_canary_no_write_sink_external_write_allowed_count == 0
  and .operator_canary_no_write_sink_external_write_performed_count == 0
  and .operator_canary_redaction_proof_shape_declared_count == 5
  and .operator_canary_redaction_proof_accepted_count == 0
  and .operator_canary_audit_entry_shape_declared_count == 5
  and .operator_canary_audit_entry_recorded_count == 0
  and .operator_canary_readback_receipt_shape_declared_count == 5
  and .operator_canary_readback_receipt_recorded_count == 0
  and .operator_canary_idempotency_nonce_shape_declared_count == 5
  and .operator_canary_idempotency_nonce_accepted_count == 0
  and .operator_canary_controlled_request_budget_total == 5
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
  and (.operator_canary_controlled_request_payload_previews | all(
    .source_fixture_bound == true
    and .source_dispatch_envelope_shape_declared == true
    and .source_dispatch_envelope_preview_shape_ready == true
    and .source_dispatch_envelope_preview_accepted == false
    and .source_dispatch_preconditions_satisfied == false
    and .source_dispatch_ready == false
    and .source_dispatch_allowed == false
    and .source_payload_materialized == false
    and .source_payload_persisted == false
    and .controlled_request_method == "POST"
    and .controlled_request_budget == 1
    and .controlled_request_route_bound == true
    and .controlled_request_namespace_bound == true
    and .payload_preview_shape_declared == true
    and .payload_preview_materialized_in_report == true
    and .payload_preview_contains_redacted_fields_only == true
    and .payload_preview_contains_secret == false
    and .payload_preview_contains_credential == false
    and .payload_preview_contains_live_context == false
    and .payload_preview_contains_provider_input == false
    and .payload_preview_contains_memory_mutation == false
    and .payload_preview_contains_kg_mutation == false
    and .payload_preview_hash_shape_declared == true
    and .payload_preview_hash_accepted == false
    and .payload_preview_accepted == false
    and .payload_preview_recorded == false
    and .payload_preview_persisted == false
    and .payload_preview_delivered == false
    and .request_payload_materialized == false
    and .request_payload_persisted == false
    and .no_write_sink_contract_shape_declared == true
    and .no_write_sink_preview_only == true
    and .no_write_sink_filesystem_write_allowed == false
    and .no_write_sink_filesystem_write_performed == false
    and .no_write_sink_workspace_write_allowed == false
    and .no_write_sink_workspace_write_performed == false
    and .no_write_sink_external_write_allowed == false
    and .no_write_sink_external_write_performed == false
    and .no_write_sink_dispatch_allowed == false
    and .no_write_sink_dispatch_performed == false
    and .redaction_proof_shape_declared == true
    and .redaction_proof_accepted == false
    and .audit_entry_shape_declared == true
    and .audit_entry_recorded == false
    and .readback_receipt_shape_declared == true
    and .readback_receipt_recorded == false
    and .idempotency_nonce_shape_declared == true
    and .idempotency_nonce_accepted == false
    and .rollback_kill_switch_shape_declared == true
    and .rollback_kill_switch_armed == false
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
    and .status == "blocked_payload_preview_no_write_sink_only"
  ))
  and .operator_canary_dispatch_envelope_accepted == false
  and .operator_canary_dispatch_envelope_authorizes_dispatch == false
  and .operator_canary_dispatch_envelope_authorizes_live_execution == false
  and .operator_canary_payload_preview_accepted == false
  and .operator_canary_payload_preview_authorizes_dispatch == false
  and .operator_canary_payload_preview_authorizes_live_execution == false
  and .canary_harness_shape_ready == true
  and .canary_harness_activation_ready == false
  and .canary_harness_armed == false
  and .canary_harness_executable == false
  and .canary_live_enabled == false
  and .canary_execution_performed == false
  and .controlled_request_dispatched == false
  and .controlled_request_executed == false
  and .payload_preview_materialized_in_report == true
  and .request_payload_materialized == false
  and .request_payload_persisted == false
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
echo "Hepta Memory/Intelligence/KG controlled request payload preview no-write sink gate passed"
