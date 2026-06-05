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

READINESS_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-arm-readiness-scoreboard-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-arm-readiness-scoreboard-gate.sh
)"

readiness_report_sha256="$(sha256_text "$READINESS_JSON")"
dispatch_envelope_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-dispatch-envelope-preview:v1:source-arm-readiness:report-only:no-accept:no-dispatch:no-execute:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_canary_dispatch_envelope_preview_side_effects=false;accepted=false;dispatched=false;executed=false;context=false;provider=false;model=false;memory=false;kg=false;secret=false;restart=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$READINESS_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_arm_readiness_scoreboard_gate"
    and $source.operator_canary_arm_readiness_scoreboard_ready == true
    and $source.operator_canary_arm_readiness_scoreboard_status == "blocked"
    and $source.operator_canary_arm_readiness_item_count == 16
    and $source.operator_canary_arm_readiness_item_missing_count == 16
    and $source.operator_canary_arm_readiness_item_accepted_count == 0
    and $source.operator_canary_arm_readiness_item_blocks_arm_count == 16
    and $source.operator_canary_stage_readiness_count == 5
    and $source.operator_canary_stage_readiness_shape_declared_count == 5
    and $source.operator_canary_stage_preconditions_satisfied_count == 0
    and $source.operator_canary_stage_arm_ready_count == 0
    and $source.operator_canary_stage_dry_run_ready_count == 0
    and $source.operator_canary_stage_live_execution_ready_count == 0
    and $source.operator_canary_controlled_request_budget_total == 5
    and $source.operator_canary_controlled_request_dispatch_ready_count == 0
    and $source.operator_canary_controlled_request_dispatch_allowed_count == 0
    and $source.operator_canary_controlled_request_dispatched_count == 0
    and $source.operator_canary_controlled_request_executed_count == 0
    and $source.operator_canary_arm_readiness_accepted == false
    and $source.operator_canary_arm_readiness_authorizes_canary_arm == false
    and $source.operator_canary_arm_readiness_authorizes_live_execution == false
    and $source.canary_harness_shape_ready == true
    and $source.canary_harness_activation_ready == false
    and $source.canary_harness_armed == false
    and $source.canary_harness_executable == false
    and $source.canary_live_enabled == false
    and $source.canary_execution_performed == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_dispatch_envelope_preview_gate" \
    --arg readiness_report_sha256 "$readiness_report_sha256" \
    --arg dispatch_envelope_policy_hash_sha256 "$dispatch_envelope_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --arg preview_id "hepta-memory-intelligence-kg-operator-canary-dispatch-envelope-preview-report-only-v1" \
    --arg preview_schema_id "hepta.memory_intelligence_kg.canary.operator_dispatch_envelope_preview.v1" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$READINESS_JSON" \
    '
      [
        $source.operator_canary_stage_readiness[]
        | {
            dispatch_envelope_order: .stage_order,
            dispatch_envelope_id: ("hepta-canary-dispatch-envelope-preview-" + .stage_id),
            dispatch_envelope_schema_id: "hepta.memory_intelligence_kg.canary.controlled_request.dispatch_envelope.v1",
            stage_id: .stage_id,
            source_phase_id: .source_phase_id,
            route_id: .route_id,
            namespace_id: .namespace_id,
            source_fixture_bound: .source_fixture_bound,
            source_stage_readiness_shape_declared: .stage_readiness_shape_declared,
            source_stage_arm_ready: .stage_arm_ready,
            source_stage_dry_run_ready: .stage_dry_run_ready,
            source_stage_live_execution_ready: .stage_live_execution_ready,
            dispatch_envelope_shape_declared: true,
            dispatch_envelope_preview_shape_ready: true,
            dispatch_envelope_preview_accepted: false,
            operator_packet_binding_required: true,
            operator_packet_binding_accepted: false,
            arm_readiness_acceptance_required: true,
            arm_readiness_accepted: false,
            idempotency_nonce_required: true,
            idempotency_nonce_accepted: false,
            route_binding_required: true,
            route_binding_accepted: false,
            namespace_binding_required: true,
            namespace_binding_accepted: false,
            rollback_kill_switch_required: true,
            rollback_kill_switch_armed: false,
            redaction_proof_required: true,
            redaction_proof_accepted: false,
            audit_trail_required: true,
            audit_trail_recorded: false,
            readback_receipt_required: true,
            readback_receipt_recorded: false,
            context_preview_required: true,
            context_preview_accepted: false,
            request_method_shape_declared: true,
            request_method: "POST",
            request_payload_shape_declared: true,
            request_payload_materialized: false,
            request_payload_persisted: false,
            request_payload_hash_shape_declared: true,
            request_payload_hash_accepted: false,
            controlled_request_budget: .controlled_request_budget,
            controlled_request_budget_accepted: false,
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
            status: "blocked_dispatch_envelope_preview_only"
          }
      ] as $dispatch_envelopes
      | {
          product: $product,
          runtime: $runtime,
          status: "ready",
          base_url: $base_url,
          gate: $gate,
          operator_canary_dispatch_envelope_preview_schema_version: "memory_intelligence_kg_operator_canary_dispatch_envelope_preview_v1",
          operator_canary_dispatch_envelope_preview_ready: true,
          operator_canary_dispatch_envelope_preview_status: "blocked",
          operator_canary_dispatch_envelope_preview_mode: "report_only_dispatch_envelope_preview_no_record_no_accept_no_dispatch_no_execute_no_live",
          operator_canary_dispatch_envelope_preview_decision: "controlled_request_dispatch_envelopes_are_shaped_but_cannot_dispatch_until_operator_packet_arm_readiness_and_stage_guards_are_accepted",
          operator_canary_dispatch_envelope_preview_id: $preview_id,
          operator_canary_dispatch_envelope_preview_schema_id: $preview_schema_id,
          minimum_required_samples: $min_long_soak_samples,
          source_arm_readiness_gate: $source.gate,
          source_arm_readiness_report_sha256: $readiness_report_sha256,
          source_arm_readiness_ready: $source.operator_canary_arm_readiness_scoreboard_ready,
          source_arm_readiness_status: $source.operator_canary_arm_readiness_scoreboard_status,
          source_arm_readiness_item_count: $source.operator_canary_arm_readiness_item_count,
          source_arm_readiness_item_missing_count: $source.operator_canary_arm_readiness_item_missing_count,
          source_arm_readiness_item_accepted_count: $source.operator_canary_arm_readiness_item_accepted_count,
          source_arm_readiness_item_blocks_arm_count: $source.operator_canary_arm_readiness_item_blocks_arm_count,
          source_stage_readiness_count: $source.operator_canary_stage_readiness_count,
          source_stage_arm_ready_count: $source.operator_canary_stage_arm_ready_count,
          source_stage_live_execution_ready_count: $source.operator_canary_stage_live_execution_ready_count,
          source_controlled_request_budget_total: $source.operator_canary_controlled_request_budget_total,
          source_controlled_request_dispatch_ready_count: $source.operator_canary_controlled_request_dispatch_ready_count,
          source_controlled_request_dispatch_allowed_count: $source.operator_canary_controlled_request_dispatch_allowed_count,
          source_controlled_request_dispatched_count: $source.operator_canary_controlled_request_dispatched_count,
          source_controlled_request_executed_count: $source.operator_canary_controlled_request_executed_count,
          dispatch_envelope_policy_hash_sha256: $dispatch_envelope_policy_hash_sha256,
          side_effect_hash_sha256: $side_effect_hash_sha256,
          operator_canary_dispatch_envelopes: $dispatch_envelopes,
          operator_canary_dispatch_envelope_count: ($dispatch_envelopes | length),
          operator_canary_dispatch_envelope_shape_declared_count: ($dispatch_envelopes | map(select(.dispatch_envelope_shape_declared == true)) | length),
          operator_canary_dispatch_envelope_preview_shape_ready_count: ($dispatch_envelopes | map(select(.dispatch_envelope_preview_shape_ready == true)) | length),
          operator_canary_dispatch_envelope_preview_accepted_count: ($dispatch_envelopes | map(select(.dispatch_envelope_preview_accepted == true)) | length),
          operator_canary_dispatch_preconditions_satisfied_count: ($dispatch_envelopes | map(select(.controlled_request_dispatch_preconditions_satisfied == true)) | length),
          operator_canary_controlled_request_budget_total: ($dispatch_envelopes | map(.controlled_request_budget) | add),
          operator_canary_controlled_request_budget_accepted_count: ($dispatch_envelopes | map(select(.controlled_request_budget_accepted == true)) | length),
          operator_canary_controlled_request_dispatch_ready_count: ($dispatch_envelopes | map(select(.controlled_request_dispatch_ready == true)) | length),
          operator_canary_controlled_request_dispatch_allowed_count: ($dispatch_envelopes | map(select(.controlled_request_dispatch_allowed == true)) | length),
          operator_canary_controlled_request_dispatched_count: ($dispatch_envelopes | map(.controlled_request_dispatched_count) | add),
          operator_canary_controlled_request_execution_allowed_count: ($dispatch_envelopes | map(select(.controlled_request_execution_allowed == true)) | length),
          operator_canary_controlled_request_executed_count: ($dispatch_envelopes | map(.controlled_request_executed_count) | add),
          operator_canary_request_payload_materialized_count: ($dispatch_envelopes | map(select(.request_payload_materialized == true)) | length),
          operator_canary_request_payload_persisted_count: ($dispatch_envelopes | map(select(.request_payload_persisted == true)) | length),
          operator_canary_context_attachment_allowed_count: ($dispatch_envelopes | map(select(.context_attachment_allowed == true)) | length),
          operator_canary_provider_model_invocation_allowed_count: ($dispatch_envelopes | map(select(.provider_model_invocation_allowed == true)) | length),
          operator_canary_memory_write_allowed_count: ($dispatch_envelopes | map(select(.memory_write_allowed == true)) | length),
          operator_canary_external_kg_read_allowed_count: ($dispatch_envelopes | map(select(.external_kg_read_allowed == true)) | length),
          operator_canary_live_kg_write_allowed_count: ($dispatch_envelopes | map(select(.live_kg_write_allowed == true)) | length),
          operator_canary_dispatch_envelope_recorded: false,
          operator_canary_dispatch_envelope_persisted: false,
          operator_canary_dispatch_envelope_materialized: false,
          operator_canary_dispatch_envelope_delivered: false,
          operator_canary_dispatch_envelope_accepted: false,
          operator_canary_dispatch_envelope_authorizes_dispatch: false,
          operator_canary_dispatch_envelope_authorizes_live_execution: false,
          operator_canary_arm_readiness_accepted: false,
          operator_canary_arm_readiness_authorizes_canary_arm: false,
          operator_canary_packet_accepted: false,
          operator_canary_packet_authorizes_canary_arm: false,
          operator_canary_packet_authorizes_live_execution: false,
          canary_harness_shape_ready: true,
          canary_harness_activation_ready: false,
          canary_harness_armed: false,
          canary_harness_executable: false,
          canary_live_enabled: false,
          canary_execution_performed: false,
          controlled_request_dispatched: false,
          controlled_request_executed: false,
          request_payload_materialized: false,
          request_payload_persisted: false,
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
          denied_by_operator_canary_dispatch_envelope_preview: [
            "operator_canary_packet_acceptance_missing",
            "operator_canary_arm_readiness_acceptance_missing",
            "stage_arm_readiness_missing",
            "controlled_request_budget_acceptance_missing",
            "idempotency_nonce_acceptance_missing",
            "route_namespace_binding_acceptance_missing",
            "rollback_kill_switch_not_armed",
            "redaction_proof_acceptance_missing",
            "audit_trail_recording_missing",
            "readback_receipt_recording_missing",
            "context_preview_acceptance_missing",
            "dispatch_envelope_acceptance_missing",
            "controlled_request_dispatch_denied",
            "controlled_request_execution_denied",
            "provider_model_invocation_denied",
            "memory_kg_live_mutation_denied",
            "credential_secret_read_denied",
            "install_restart_active_binary_mutation_denied"
          ],
          side_effects: {
            workspace_written: false,
            filesystem_written: false,
            operator_canary_dispatch_envelope_recorded: false,
            operator_canary_dispatch_envelope_persisted: false,
            operator_canary_dispatch_envelope_materialized: false,
            operator_canary_dispatch_envelope_delivered: false,
            operator_canary_dispatch_envelope_accepted: false,
            controlled_request_dispatched: false,
            controlled_request_executed: false,
            request_payload_materialized: false,
            request_payload_persisted: false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_dispatch_envelope_preview_gate"
  and .operator_canary_dispatch_envelope_preview_ready == true
  and .operator_canary_dispatch_envelope_preview_status == "blocked"
  and .source_arm_readiness_gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_arm_readiness_scoreboard_gate"
  and .source_arm_readiness_ready == true
  and .source_arm_readiness_status == "blocked"
  and .source_arm_readiness_item_count == 16
  and .source_arm_readiness_item_missing_count == 16
  and .source_arm_readiness_item_accepted_count == 0
  and .source_arm_readiness_item_blocks_arm_count == 16
  and .source_stage_readiness_count == 5
  and .source_stage_arm_ready_count == 0
  and .source_stage_live_execution_ready_count == 0
  and .source_controlled_request_budget_total == 5
  and .source_controlled_request_dispatch_ready_count == 0
  and .source_controlled_request_dispatch_allowed_count == 0
  and .source_controlled_request_dispatched_count == 0
  and .source_controlled_request_executed_count == 0
  and .operator_canary_dispatch_envelope_count == 5
  and .operator_canary_dispatch_envelope_shape_declared_count == 5
  and .operator_canary_dispatch_envelope_preview_shape_ready_count == 5
  and .operator_canary_dispatch_envelope_preview_accepted_count == 0
  and .operator_canary_dispatch_preconditions_satisfied_count == 0
  and .operator_canary_controlled_request_budget_total == 5
  and .operator_canary_controlled_request_budget_accepted_count == 0
  and .operator_canary_controlled_request_dispatch_ready_count == 0
  and .operator_canary_controlled_request_dispatch_allowed_count == 0
  and .operator_canary_controlled_request_dispatched_count == 0
  and .operator_canary_controlled_request_execution_allowed_count == 0
  and .operator_canary_controlled_request_executed_count == 0
  and .operator_canary_request_payload_materialized_count == 0
  and .operator_canary_request_payload_persisted_count == 0
  and .operator_canary_context_attachment_allowed_count == 0
  and .operator_canary_provider_model_invocation_allowed_count == 0
  and .operator_canary_memory_write_allowed_count == 0
  and .operator_canary_external_kg_read_allowed_count == 0
  and .operator_canary_live_kg_write_allowed_count == 0
  and (.operator_canary_dispatch_envelopes | all(
    .source_fixture_bound == true
    and .source_stage_readiness_shape_declared == true
    and .source_stage_arm_ready == false
    and .source_stage_dry_run_ready == false
    and .source_stage_live_execution_ready == false
    and .dispatch_envelope_shape_declared == true
    and .dispatch_envelope_preview_shape_ready == true
    and .dispatch_envelope_preview_accepted == false
    and .operator_packet_binding_required == true
    and .operator_packet_binding_accepted == false
    and .arm_readiness_acceptance_required == true
    and .arm_readiness_accepted == false
    and .idempotency_nonce_required == true
    and .idempotency_nonce_accepted == false
    and .route_binding_required == true
    and .route_binding_accepted == false
    and .namespace_binding_required == true
    and .namespace_binding_accepted == false
    and .rollback_kill_switch_required == true
    and .rollback_kill_switch_armed == false
    and .redaction_proof_required == true
    and .redaction_proof_accepted == false
    and .audit_trail_required == true
    and .audit_trail_recorded == false
    and .readback_receipt_required == true
    and .readback_receipt_recorded == false
    and .context_preview_required == true
    and .context_preview_accepted == false
    and .request_method_shape_declared == true
    and .request_method == "POST"
    and .request_payload_shape_declared == true
    and .request_payload_materialized == false
    and .request_payload_persisted == false
    and .request_payload_hash_shape_declared == true
    and .request_payload_hash_accepted == false
    and .controlled_request_budget == 1
    and .controlled_request_budget_accepted == false
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
    and .status == "blocked_dispatch_envelope_preview_only"
  ))
  and .operator_canary_dispatch_envelope_recorded == false
  and .operator_canary_dispatch_envelope_persisted == false
  and .operator_canary_dispatch_envelope_materialized == false
  and .operator_canary_dispatch_envelope_delivered == false
  and .operator_canary_dispatch_envelope_accepted == false
  and .operator_canary_dispatch_envelope_authorizes_dispatch == false
  and .operator_canary_dispatch_envelope_authorizes_live_execution == false
  and .operator_canary_arm_readiness_accepted == false
  and .operator_canary_arm_readiness_authorizes_canary_arm == false
  and .operator_canary_packet_accepted == false
  and .operator_canary_packet_authorizes_canary_arm == false
  and .operator_canary_packet_authorizes_live_execution == false
  and .canary_harness_shape_ready == true
  and .canary_harness_activation_ready == false
  and .canary_harness_armed == false
  and .canary_harness_executable == false
  and .canary_live_enabled == false
  and .canary_execution_performed == false
  and .controlled_request_dispatched == false
  and .controlled_request_executed == false
  and .request_payload_materialized == false
  and .request_payload_persisted == false
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
echo "Hepta Memory/Intelligence/KG operator canary dispatch envelope preview gate passed"
