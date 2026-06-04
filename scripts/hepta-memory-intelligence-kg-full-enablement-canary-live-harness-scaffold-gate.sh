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

VALIDATOR_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-positive-activation-packet-validator-scoreboard-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-positive-activation-packet-validator-scoreboard-gate.sh
)"

validator_report_sha256="$(sha256_text "$VALIDATOR_JSON")"
canary_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-canary-live-harness-scaffold:v1:report-only:not-armed:no-execution:no-provider:no-memory-write:no-kg-read-write:no-secret:no-restart"
)"
side_effect_hash_sha256="$(
  sha256_text "canary_live_harness_scaffold_side_effects=false;armed=false;executed=false;provider=false;model=false;memory=false;kg=false;secret=false;restart=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson validator "$VALIDATOR_JSON" \
  '
    $validator.runtime == "hepta"
    and $validator.status == "ready"
    and $validator.gate == "hepta_memory_intelligence_kg_full_enablement_positive_activation_packet_validator_scoreboard_gate"
    and $validator.positive_activation_packet_validator_scoreboard_ready == true
    and $validator.positive_activation_packet_validator_scoreboard_status == "blocked"
    and $validator.canary_harness_shape_ready == true
    and $validator.canary_harness_activation_ready == false
    and $validator.canary_harness_next_slice_allowed == true
    and $validator.canary_harness_next_slice_performs_live_activation == false
    and $validator.phase_count == 5
    and $validator.validated_phase_shape_count == 5
    and $validator.accepted_phase_count == 0
    and $validator.live_enabled_phase_count == 0
    and $validator.required_authority_family_count == 8
    and $validator.required_scoreboard_item_count == 40
    and $validator.authority_satisfied_scoreboard_item_count == 0
    and $validator.missing_authority_scoreboard_item_count == 40
    and $validator.canary_harness_required_precondition_count == 12
    and $validator.canary_harness_satisfied_precondition_count == 0
    and $validator.canary_harness_missing_precondition_count == 12
    and ($validator.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_canary_live_harness_scaffold_gate" \
  --arg validator_report_sha256 "$validator_report_sha256" \
  --arg canary_policy_hash_sha256 "$canary_policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --arg route_id "hepta-memory-intelligence-kg-canary-single-route-report-only" \
  --arg namespace_id "hepta-memory-intelligence-kg-canary-single-namespace-report-only" \
  --arg rollback_id "hepta-memory-intelligence-kg-canary-kill-switch-report-only" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson validator "$VALIDATOR_JSON" \
  '
    ($validator.phase_authority_scoreboard | unique_by(.phase_id) | sort_by(.phase_order)) as $phases
    | [
        $phases[]
        | {
            stage_order: .phase_order,
            stage_id: ("canary_" + (.phase_id | sub("_shape$"; ""))),
            source_phase_id: .phase_id,
            source_phase_label: .phase_label,
            source_gate: .source_gate,
            source_report_sha256: .source_report_sha256,
            route_id: $route_id,
            namespace_id: $namespace_id,
            scope_shape_declared: true,
            single_route_binding_shape_declared: true,
            single_route_binding_accepted: false,
            single_namespace_binding_shape_declared: true,
            single_namespace_binding_accepted: false,
            controlled_request_budget_shape_declared: true,
            controlled_request_budget_accepted: false,
            max_controlled_request_count: 1,
            controlled_request_dispatched_count: 0,
            controlled_request_executed_count: 0,
            rollback_kill_switch_shape_declared: true,
            rollback_kill_switch_id: $rollback_id,
            rollback_kill_switch_accepted: false,
            rollback_kill_switch_armed: false,
            redaction_policy_shape_declared: true,
            redaction_policy_accepted: false,
            readback_receipt_shape_declared: true,
            readback_receipt_recorded: false,
            readback_receipt_accepted: false,
            audit_trail_shape_declared: true,
            audit_trail_recorded: false,
            idempotency_nonce_shape_declared: true,
            idempotency_nonce_recorded: false,
            idempotency_nonce_accepted: false,
            retention_export_observability_shape_declared: true,
            retention_export_observability_accepted: false,
            provider_model_secret_use_policy_shape_declared: true,
            provider_model_secret_use_policy_accepted: false,
            phase_specific_memory_kg_write_policy_shape_declared: true,
            phase_specific_memory_kg_write_policy_accepted: false,
            stage_shape_declared: true,
            stage_armed: false,
            stage_executable: false,
            stage_executed: false,
            stage_live_enabled: false,
            status: "blocked_not_armed"
          }
      ] as $stages
    | [
        $validator.canary_harness_preconditions[]
        | {
            guard_order: .precondition_order,
            guard_id: .precondition_id,
            required: true,
            shape_declared: true,
            accepted: false,
            satisfied: false,
            armed: false,
            missing: true,
            live_enabling: false,
            status: "missing"
          }
      ] as $guards
    | {
        product: $product,
        runtime: $runtime,
        status: "ready",
        base_url: $base_url,
        gate: $gate,
        canary_live_harness_scaffold_schema_version: "memory_intelligence_kg_full_enablement_canary_live_harness_scaffold_v1",
        canary_live_harness_scaffold_ready: true,
        canary_live_harness_scaffold_status: "blocked",
        canary_live_harness_scaffold_mode: "stdout_only_report_only_canary_harness_shape_not_armed_no_live_execution",
        canary_live_harness_scaffold_decision: "single_route_single_namespace_canary_harness_shape_is_declared_but_not_armed_until_positive_authority_preconditions_are_accepted",
        minimum_required_samples: $min_long_soak_samples,
        source_validator_gate: $validator.gate,
        source_validator_report_sha256: $validator_report_sha256,
        source_validator_ready: $validator.positive_activation_packet_validator_scoreboard_ready,
        source_validator_status: $validator.positive_activation_packet_validator_scoreboard_status,
        source_phase_count: $validator.phase_count,
        source_validated_phase_shape_count: $validator.validated_phase_shape_count,
        source_required_authority_family_count: $validator.required_authority_family_count,
        source_required_scoreboard_item_count: $validator.required_scoreboard_item_count,
        source_authority_satisfied_scoreboard_item_count: $validator.authority_satisfied_scoreboard_item_count,
        source_missing_authority_scoreboard_item_count: $validator.missing_authority_scoreboard_item_count,
        source_canary_harness_required_precondition_count: $validator.canary_harness_required_precondition_count,
        source_canary_harness_satisfied_precondition_count: $validator.canary_harness_satisfied_precondition_count,
        source_canary_harness_missing_precondition_count: $validator.canary_harness_missing_precondition_count,
        source_canary_harness_next_slice_allowed: $validator.canary_harness_next_slice_allowed,
        source_canary_harness_next_slice_performs_live_activation: $validator.canary_harness_next_slice_performs_live_activation,
        canary_policy_hash_sha256: $canary_policy_hash_sha256,
        side_effect_hash_sha256: $side_effect_hash_sha256,
        canary_route_id: $route_id,
        canary_namespace_id: $namespace_id,
        canary_rollback_kill_switch_id: $rollback_id,
        canary_single_route_shape_declared: true,
        canary_single_route_binding_accepted: false,
        canary_single_namespace_shape_declared: true,
        canary_single_namespace_binding_accepted: false,
        canary_zero_or_one_request_budget_shape_declared: true,
        canary_max_controlled_request_count: 1,
        canary_controlled_request_budget_accepted: false,
        canary_controlled_request_dispatched_count: 0,
        canary_controlled_request_executed_count: 0,
        canary_harness_stage_count: ($stages | length),
        canary_harness_stage_shape_declared_count: ($stages | map(select(.stage_shape_declared == true)) | length),
        canary_harness_stage_blocked_count: ($stages | map(select(.status == "blocked_not_armed")) | length),
        canary_harness_stage_armed_count: ($stages | map(select(.stage_armed == true)) | length),
        canary_harness_stage_executable_count: ($stages | map(select(.stage_executable == true)) | length),
        canary_harness_stage_executed_count: ($stages | map(select(.stage_executed == true)) | length),
        canary_harness_stage_live_enabled_count: ($stages | map(select(.stage_live_enabled == true)) | length),
        canary_harness_stages: $stages,
        canary_guard_count: ($guards | length),
        canary_guard_shape_declared_count: ($guards | map(select(.shape_declared == true)) | length),
        canary_guard_accepted_count: ($guards | map(select(.accepted == true)) | length),
        canary_guard_satisfied_count: ($guards | map(select(.satisfied == true)) | length),
        canary_guard_armed_count: ($guards | map(select(.armed == true)) | length),
        canary_guard_missing_count: ($guards | map(select(.missing == true)) | length),
        canary_guards: $guards,
        canary_harness_shape_ready: true,
        canary_harness_activation_ready: false,
        canary_harness_armed: false,
        canary_harness_executable: false,
        canary_live_enabled: false,
        canary_execution_performed: false,
        operator_approval_recorded: false,
        operator_approval_accepted: false,
        activation_packet_recorded: false,
        activation_packet_persisted: false,
        activation_packet_accepted: false,
        activation_packet_delivered: false,
        readiness_index_persisted: false,
        readiness_index_delivered: false,
        runtime_router_mutated: false,
        router_handoff_recorded: false,
        hepta_intelligence_context_attached: false,
        live_context_attached_to_prompt: false,
        prompt_preview_rendered: false,
        prompt_payload_materialized: false,
        context_injection_performed: false,
        provider_invoked: false,
        model_invoked: false,
        usage_recorded: false,
        memory_write_operation_allowed: false,
        memory_store_write_performed: false,
        memory_store_mutated: false,
        external_kg_adapter_read_performed: false,
        external_adapter_client_constructed: false,
        network_call_performed: false,
        external_db_write_performed: false,
        live_kg_write_performed: false,
        audit_trail_recorded: false,
        readback_receipt_recorded: false,
        rollback_executed: false,
        credential_read: false,
        auth_secret_read: false,
        secret_file_read: false,
        channel_send_performed: false,
        telegram_send_performed: false,
        external_send_performed: false,
        filesystem_written: false,
        release_artifact_written: false,
        public_release_claimed: false,
        public_ga_claimed: false,
        install_performed: false,
        service_restarted: false,
        active_binary_mutated: false,
        upstream_fetch_performed: false,
        upstream_merge_performed: false,
        next_required_step: "record_explicit_operator_approved_canary_packet_before_any_live_execution",
        side_effects: {
          operator_approval_recorded: false,
          operator_approval_accepted: false,
          activation_packet_recorded: false,
          activation_packet_persisted: false,
          activation_packet_accepted: false,
          activation_packet_delivered: false,
          readiness_index_persisted: false,
          readiness_index_delivered: false,
          runtime_router_mutated: false,
          router_handoff_recorded: false,
          hepta_intelligence_context_attached: false,
          live_context_attached_to_prompt: false,
          prompt_preview_rendered: false,
          prompt_payload_materialized: false,
          context_injection_performed: false,
          provider_invoked: false,
          model_invoked: false,
          usage_recorded: false,
          memory_store_write_performed: false,
          memory_store_mutated: false,
          external_kg_adapter_read_performed: false,
          external_adapter_client_constructed: false,
          network_call_performed: false,
          external_db_write_performed: false,
          live_kg_write_performed: false,
          audit_trail_recorded: false,
          readback_receipt_recorded: false,
          rollback_executed: false,
          credential_read: false,
          auth_secret_read: false,
          secret_file_read: false,
          channel_send_performed: false,
          telegram_send_performed: false,
          external_send_performed: false,
          filesystem_written: false,
          release_artifact_written: false,
          public_release_claimed: false,
          public_ga_claimed: false,
          install_performed: false,
          service_restarted: false,
          active_binary_mutated: false,
          upstream_fetch_performed: false,
          upstream_merge_performed: false,
          canary_execution_performed: false
        }
      }
  ')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_canary_live_harness_scaffold_gate"
  and .canary_live_harness_scaffold_ready == true
  and .canary_live_harness_scaffold_status == "blocked"
  and .canary_live_harness_scaffold_mode == "stdout_only_report_only_canary_harness_shape_not_armed_no_live_execution"
  and .source_validator_gate == "hepta_memory_intelligence_kg_full_enablement_positive_activation_packet_validator_scoreboard_gate"
  and .source_validator_ready == true
  and .source_validator_status == "blocked"
  and .source_phase_count == 5
  and .source_validated_phase_shape_count == 5
  and .source_required_authority_family_count == 8
  and .source_required_scoreboard_item_count == 40
  and .source_authority_satisfied_scoreboard_item_count == 0
  and .source_missing_authority_scoreboard_item_count == 40
  and .source_canary_harness_required_precondition_count == 12
  and .source_canary_harness_satisfied_precondition_count == 0
  and .source_canary_harness_missing_precondition_count == 12
  and .source_canary_harness_next_slice_allowed == true
  and .source_canary_harness_next_slice_performs_live_activation == false
  and .canary_single_route_shape_declared == true
  and .canary_single_route_binding_accepted == false
  and .canary_single_namespace_shape_declared == true
  and .canary_single_namespace_binding_accepted == false
  and .canary_zero_or_one_request_budget_shape_declared == true
  and .canary_max_controlled_request_count == 1
  and .canary_controlled_request_budget_accepted == false
  and .canary_controlled_request_dispatched_count == 0
  and .canary_controlled_request_executed_count == 0
  and .canary_harness_stage_count == 5
  and .canary_harness_stage_shape_declared_count == 5
  and .canary_harness_stage_blocked_count == 5
  and .canary_harness_stage_armed_count == 0
  and .canary_harness_stage_executable_count == 0
  and .canary_harness_stage_executed_count == 0
  and .canary_harness_stage_live_enabled_count == 0
  and (.canary_harness_stages | length) == 5
  and (.canary_harness_stages | all(
    .scope_shape_declared == true
    and .single_route_binding_shape_declared == true
    and .single_route_binding_accepted == false
    and .single_namespace_binding_shape_declared == true
    and .single_namespace_binding_accepted == false
    and .controlled_request_budget_shape_declared == true
    and .controlled_request_budget_accepted == false
    and .controlled_request_dispatched_count == 0
    and .controlled_request_executed_count == 0
    and .rollback_kill_switch_shape_declared == true
    and .rollback_kill_switch_accepted == false
    and .rollback_kill_switch_armed == false
    and .redaction_policy_shape_declared == true
    and .redaction_policy_accepted == false
    and .readback_receipt_shape_declared == true
    and .readback_receipt_recorded == false
    and .readback_receipt_accepted == false
    and .audit_trail_shape_declared == true
    and .audit_trail_recorded == false
    and .idempotency_nonce_shape_declared == true
    and .idempotency_nonce_recorded == false
    and .idempotency_nonce_accepted == false
    and .stage_shape_declared == true
    and .stage_armed == false
    and .stage_executable == false
    and .stage_executed == false
    and .stage_live_enabled == false
    and .status == "blocked_not_armed"
  ))
  and .canary_guard_count == 12
  and .canary_guard_shape_declared_count == 12
  and .canary_guard_accepted_count == 0
  and .canary_guard_satisfied_count == 0
  and .canary_guard_armed_count == 0
  and .canary_guard_missing_count == 12
  and (.canary_guards | length) == 12
  and (.canary_guards | all(
    .required == true
    and .shape_declared == true
    and .accepted == false
    and .satisfied == false
    and .armed == false
    and .missing == true
    and .live_enabling == false
    and .status == "missing"
  ))
  and .canary_harness_shape_ready == true
  and .canary_harness_activation_ready == false
  and .canary_harness_armed == false
  and .canary_harness_executable == false
  and .canary_live_enabled == false
  and .canary_execution_performed == false
  and .operator_approval_recorded == false
  and .operator_approval_accepted == false
  and .activation_packet_recorded == false
  and .activation_packet_persisted == false
  and .activation_packet_accepted == false
  and .activation_packet_delivered == false
  and .runtime_router_mutated == false
  and .router_handoff_recorded == false
  and .hepta_intelligence_context_attached == false
  and .live_context_attached_to_prompt == false
  and .prompt_preview_rendered == false
  and .prompt_payload_materialized == false
  and .context_injection_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .external_kg_adapter_read_performed == false
  and .network_call_performed == false
  and .external_db_write_performed == false
  and .live_kg_write_performed == false
  and .audit_trail_recorded == false
  and .readback_receipt_recorded == false
  and .rollback_executed == false
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
echo "Hepta Memory/Intelligence/KG canary live harness scaffold gate passed"
