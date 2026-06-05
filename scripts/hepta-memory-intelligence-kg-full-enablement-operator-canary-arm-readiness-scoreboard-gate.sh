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

ARM_PLAN_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-arm-plan-dry-run-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-arm-plan-dry-run-gate.sh
)"

arm_plan_report_sha256="$(sha256_text "$ARM_PLAN_JSON")"
readiness_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-arm-readiness-scoreboard:v1:source-arm-plan:report-only:no-accept:no-arm:no-execute:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_canary_arm_readiness_scoreboard_side_effects=false;accepted=false;armed=false;executed=false;provider=false;model=false;memory=false;kg=false;secret=false;restart=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$ARM_PLAN_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_arm_plan_dry_run_gate"
    and $source.operator_canary_arm_plan_dry_run_ready == true
    and $source.operator_canary_arm_plan_dry_run_status == "blocked"
    and $source.operator_canary_arm_plan_shape_declared == true
    and $source.operator_canary_arm_plan_guard_count == 16
    and $source.operator_canary_arm_plan_guard_shape_declared_count == 16
    and $source.operator_canary_arm_plan_guard_satisfied_count == 0
    and $source.operator_canary_arm_plan_guard_missing_count == 16
    and $source.operator_canary_arm_plan_guard_accepted_count == 0
    and $source.operator_canary_arm_plan_stage_transition_count == 5
    and $source.operator_canary_arm_plan_stage_transition_shape_declared_count == 5
    and $source.operator_canary_arm_plan_stage_transition_accepted_count == 0
    and $source.operator_canary_arm_plan_stage_armable_count == 0
    and $source.operator_canary_arm_plan_stage_dry_run_ready_count == 0
    and $source.operator_canary_arm_plan_stage_live_execution_allowed_count == 0
    and $source.operator_canary_arm_plan_controlled_request_dispatched_count == 0
    and $source.operator_canary_arm_plan_controlled_request_executed_count == 0
    and $source.operator_canary_arm_plan_recorded == false
    and $source.operator_canary_arm_plan_persisted == false
    and $source.operator_canary_arm_plan_accepted == false
    and $source.operator_canary_packet_accepted == false
    and $source.operator_canary_packet_authorizes_canary_arm == false
    and $source.operator_canary_packet_authorizes_live_execution == false
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
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_arm_readiness_scoreboard_gate" \
    --arg arm_plan_report_sha256 "$arm_plan_report_sha256" \
    --arg readiness_policy_hash_sha256 "$readiness_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --arg scoreboard_id "hepta-memory-intelligence-kg-operator-canary-arm-readiness-scoreboard-report-only-v1" \
    --arg scoreboard_schema_id "hepta.memory_intelligence_kg.canary.operator_arm_readiness_scoreboard.v1" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$ARM_PLAN_JSON" \
    '
      [
        $source.operator_canary_arm_plan_guards[]
        | {
            readiness_order: .guard_order,
            readiness_id: .guard_id,
            readiness_family: .guard_family,
            required: .required,
            source_precondition_shape_covered: .source_precondition_shape_covered,
            source_arm_plan_guard_shape_declared: .arm_plan_guard_shape_declared,
            source_arm_plan_guard_satisfied: .arm_plan_guard_satisfied,
            source_arm_plan_guard_missing: .arm_plan_guard_missing,
            source_arm_plan_guard_accepted: .arm_plan_guard_accepted,
            arm_readiness_item_shape_declared: true,
            arm_readiness_item_satisfied: false,
            arm_readiness_item_missing: true,
            arm_readiness_item_accepted: false,
            arm_readiness_item_blocks_arm: true,
            arm_readiness_item_live_enabling: false,
            status: "blocked_missing_trusted_acceptance"
          }
      ] as $guard_readiness
      | [
          $source.operator_canary_arm_plan_stage_transitions[]
          | {
              stage_order: .stage_order,
              stage_id: .stage_id,
              source_phase_id: .source_phase_id,
              route_id: .route_id,
              namespace_id: .namespace_id,
              source_fixture_bound: .source_fixture_bound,
              source_packet_binding_accepted: .source_packet_binding_accepted,
              source_stage_armed: .source_stage_armed,
              source_stage_executable: .source_stage_executable,
              source_stage_executed: .source_stage_executed,
              source_stage_live_enabled: .source_stage_live_enabled,
              source_arm_transition_shape_declared: .arm_transition_shape_declared,
              source_arm_transition_accepted: .arm_transition_accepted,
              source_stage_armable_by_plan: .stage_armable_by_plan,
              source_stage_dry_run_ready_by_plan: .stage_dry_run_ready_by_plan,
              source_stage_live_execution_allowed_by_plan: .stage_live_execution_allowed_by_plan,
              stage_readiness_shape_declared: true,
              stage_readiness_evaluated: true,
              stage_preconditions_satisfied: false,
              stage_arm_ready: false,
              stage_dry_run_ready: false,
              stage_live_execution_ready: false,
              controlled_request_budget: .controlled_request_budget,
              controlled_request_dispatch_ready: false,
              controlled_request_dispatch_allowed: false,
              controlled_request_dispatched_count: 0,
              controlled_request_executed_count: 0,
              status: "blocked_not_arm_ready"
            }
        ] as $stage_readiness
      | {
          product: $product,
          runtime: $runtime,
          status: "ready",
          base_url: $base_url,
          gate: $gate,
          operator_canary_arm_readiness_scoreboard_schema_version: "memory_intelligence_kg_operator_canary_arm_readiness_scoreboard_v1",
          operator_canary_arm_readiness_scoreboard_ready: true,
          operator_canary_arm_readiness_scoreboard_status: "blocked",
          operator_canary_arm_readiness_scoreboard_mode: "report_only_arm_readiness_scoreboard_no_record_no_accept_no_arm_no_live",
          operator_canary_arm_readiness_scoreboard_decision: "canary_arm_readiness_is_scored_but_cannot_be_accepted_until_a_trusted_operator_canary_packet_and_all_arm_plan_guards_are_accepted",
          operator_canary_arm_readiness_scoreboard_id: $scoreboard_id,
          operator_canary_arm_readiness_scoreboard_schema_id: $scoreboard_schema_id,
          minimum_required_samples: $min_long_soak_samples,
          source_arm_plan_gate: $source.gate,
          source_arm_plan_report_sha256: $arm_plan_report_sha256,
          source_arm_plan_ready: $source.operator_canary_arm_plan_dry_run_ready,
          source_arm_plan_status: $source.operator_canary_arm_plan_dry_run_status,
          source_arm_plan_guard_count: $source.operator_canary_arm_plan_guard_count,
          source_arm_plan_guard_missing_count: $source.operator_canary_arm_plan_guard_missing_count,
          source_arm_plan_guard_accepted_count: $source.operator_canary_arm_plan_guard_accepted_count,
          source_arm_plan_stage_transition_count: $source.operator_canary_arm_plan_stage_transition_count,
          source_arm_plan_stage_armable_count: $source.operator_canary_arm_plan_stage_armable_count,
          source_arm_plan_stage_live_execution_allowed_count: $source.operator_canary_arm_plan_stage_live_execution_allowed_count,
          source_controlled_request_dispatched_count: $source.operator_canary_arm_plan_controlled_request_dispatched_count,
          source_controlled_request_executed_count: $source.operator_canary_arm_plan_controlled_request_executed_count,
          readiness_policy_hash_sha256: $readiness_policy_hash_sha256,
          side_effect_hash_sha256: $side_effect_hash_sha256,
          operator_canary_arm_readiness_items: $guard_readiness,
          operator_canary_arm_readiness_item_count: ($guard_readiness | length),
          operator_canary_arm_readiness_item_shape_declared_count: ($guard_readiness | map(select(.arm_readiness_item_shape_declared == true)) | length),
          operator_canary_arm_readiness_item_satisfied_count: ($guard_readiness | map(select(.arm_readiness_item_satisfied == true)) | length),
          operator_canary_arm_readiness_item_missing_count: ($guard_readiness | map(select(.arm_readiness_item_missing == true)) | length),
          operator_canary_arm_readiness_item_accepted_count: ($guard_readiness | map(select(.arm_readiness_item_accepted == true)) | length),
          operator_canary_arm_readiness_item_blocks_arm_count: ($guard_readiness | map(select(.arm_readiness_item_blocks_arm == true)) | length),
          operator_canary_stage_readiness: $stage_readiness,
          operator_canary_stage_readiness_count: ($stage_readiness | length),
          operator_canary_stage_readiness_shape_declared_count: ($stage_readiness | map(select(.stage_readiness_shape_declared == true)) | length),
          operator_canary_stage_preconditions_satisfied_count: ($stage_readiness | map(select(.stage_preconditions_satisfied == true)) | length),
          operator_canary_stage_arm_ready_count: ($stage_readiness | map(select(.stage_arm_ready == true)) | length),
          operator_canary_stage_dry_run_ready_count: ($stage_readiness | map(select(.stage_dry_run_ready == true)) | length),
          operator_canary_stage_live_execution_ready_count: ($stage_readiness | map(select(.stage_live_execution_ready == true)) | length),
          operator_canary_controlled_request_budget_total: ($stage_readiness | map(.controlled_request_budget) | add),
          operator_canary_controlled_request_dispatch_ready_count: ($stage_readiness | map(select(.controlled_request_dispatch_ready == true)) | length),
          operator_canary_controlled_request_dispatch_allowed_count: ($stage_readiness | map(select(.controlled_request_dispatch_allowed == true)) | length),
          operator_canary_controlled_request_dispatched_count: ($stage_readiness | map(.controlled_request_dispatched_count) | add),
          operator_canary_controlled_request_executed_count: ($stage_readiness | map(.controlled_request_executed_count) | add),
          operator_canary_arm_readiness_acceptance_ready: false,
          operator_canary_arm_readiness_accepted: false,
          operator_canary_arm_readiness_recorded: false,
          operator_canary_arm_readiness_persisted: false,
          operator_canary_arm_readiness_delivered: false,
          operator_canary_arm_readiness_authorizes_canary_arm: false,
          operator_canary_arm_readiness_authorizes_live_execution: false,
          operator_canary_arm_plan_recorded: false,
          operator_canary_arm_plan_persisted: false,
          operator_canary_arm_plan_accepted: false,
          operator_canary_packet_recorded: false,
          operator_canary_packet_persisted: false,
          operator_canary_packet_accepted: false,
          operator_canary_packet_authorizes_canary_arm: false,
          operator_canary_packet_authorizes_live_execution: false,
          canary_route_id: $source.canary_route_id,
          canary_namespace_id: $source.canary_namespace_id,
          canary_rollback_kill_switch_id: $source.canary_rollback_kill_switch_id,
          canary_route_binding_accepted: false,
          canary_namespace_binding_accepted: false,
          canary_controlled_request_budget_accepted: false,
          canary_controlled_request_dispatched_count: 0,
          canary_controlled_request_executed_count: 0,
          canary_rollback_kill_switch_accepted: false,
          canary_rollback_kill_switch_armed: false,
          canary_harness_shape_ready: true,
          canary_harness_activation_ready: false,
          canary_harness_armed: false,
          canary_harness_executable: false,
          canary_live_enabled: false,
          canary_execution_performed: false,
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
          next_required_step: "accept_trusted_operator_canary_packet_and_all_arm_plan_guards_before_canary_arm_readiness_can_be_accepted",
          side_effects: {
            operator_canary_arm_readiness_recorded: false,
            operator_canary_arm_readiness_persisted: false,
            operator_canary_arm_readiness_delivered: false,
            operator_canary_arm_readiness_accepted: false,
            operator_canary_arm_readiness_authorizes_canary_arm: false,
            operator_canary_arm_readiness_authorizes_live_execution: false,
            operator_canary_arm_plan_recorded: false,
            operator_canary_arm_plan_persisted: false,
            operator_canary_arm_plan_accepted: false,
            operator_canary_packet_recorded: false,
            operator_canary_packet_persisted: false,
            operator_canary_packet_accepted: false,
            operator_canary_packet_authorizes_canary_arm: false,
            operator_canary_packet_authorizes_live_execution: false,
            canary_route_binding_accepted: false,
            canary_namespace_binding_accepted: false,
            canary_controlled_request_budget_accepted: false,
            canary_controlled_request_dispatched: false,
            canary_controlled_request_executed: false,
            canary_rollback_kill_switch_accepted: false,
            canary_rollback_kill_switch_armed: false,
            canary_harness_armed: false,
            canary_harness_executable: false,
            canary_live_enabled: false,
            canary_execution_performed: false,
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
            upstream_merge_performed: false
          }
        }
    ')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_arm_readiness_scoreboard_gate"
  and .operator_canary_arm_readiness_scoreboard_ready == true
  and .operator_canary_arm_readiness_scoreboard_status == "blocked"
  and .source_arm_plan_gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_arm_plan_dry_run_gate"
  and .source_arm_plan_ready == true
  and .source_arm_plan_status == "blocked"
  and .source_arm_plan_guard_count == 16
  and .source_arm_plan_guard_missing_count == 16
  and .source_arm_plan_guard_accepted_count == 0
  and .source_arm_plan_stage_transition_count == 5
  and .source_arm_plan_stage_armable_count == 0
  and .source_arm_plan_stage_live_execution_allowed_count == 0
  and .source_controlled_request_dispatched_count == 0
  and .source_controlled_request_executed_count == 0
  and .operator_canary_arm_readiness_item_count == 16
  and .operator_canary_arm_readiness_item_shape_declared_count == 16
  and .operator_canary_arm_readiness_item_satisfied_count == 0
  and .operator_canary_arm_readiness_item_missing_count == 16
  and .operator_canary_arm_readiness_item_accepted_count == 0
  and .operator_canary_arm_readiness_item_blocks_arm_count == 16
  and (.operator_canary_arm_readiness_items | all(
    .required == true
    and .source_precondition_shape_covered == true
    and .source_arm_plan_guard_shape_declared == true
    and .source_arm_plan_guard_satisfied == false
    and .source_arm_plan_guard_missing == true
    and .source_arm_plan_guard_accepted == false
    and .arm_readiness_item_shape_declared == true
    and .arm_readiness_item_satisfied == false
    and .arm_readiness_item_missing == true
    and .arm_readiness_item_accepted == false
    and .arm_readiness_item_blocks_arm == true
    and .arm_readiness_item_live_enabling == false
    and .status == "blocked_missing_trusted_acceptance"
  ))
  and .operator_canary_stage_readiness_count == 5
  and .operator_canary_stage_readiness_shape_declared_count == 5
  and .operator_canary_stage_preconditions_satisfied_count == 0
  and .operator_canary_stage_arm_ready_count == 0
  and .operator_canary_stage_dry_run_ready_count == 0
  and .operator_canary_stage_live_execution_ready_count == 0
  and .operator_canary_controlled_request_budget_total == 5
  and .operator_canary_controlled_request_dispatch_ready_count == 0
  and .operator_canary_controlled_request_dispatch_allowed_count == 0
  and .operator_canary_controlled_request_dispatched_count == 0
  and .operator_canary_controlled_request_executed_count == 0
  and (.operator_canary_stage_readiness | all(
    .source_fixture_bound == true
    and .source_packet_binding_accepted == false
    and .source_stage_armed == false
    and .source_stage_executable == false
    and .source_stage_executed == false
    and .source_stage_live_enabled == false
    and .source_arm_transition_shape_declared == true
    and .source_arm_transition_accepted == false
    and .source_stage_armable_by_plan == false
    and .source_stage_dry_run_ready_by_plan == false
    and .source_stage_live_execution_allowed_by_plan == false
    and .stage_readiness_shape_declared == true
    and .stage_readiness_evaluated == true
    and .stage_preconditions_satisfied == false
    and .stage_arm_ready == false
    and .stage_dry_run_ready == false
    and .stage_live_execution_ready == false
    and .controlled_request_budget == 1
    and .controlled_request_dispatch_ready == false
    and .controlled_request_dispatch_allowed == false
    and .controlled_request_dispatched_count == 0
    and .controlled_request_executed_count == 0
    and .status == "blocked_not_arm_ready"
  ))
  and .operator_canary_arm_readiness_acceptance_ready == false
  and .operator_canary_arm_readiness_accepted == false
  and .operator_canary_arm_readiness_recorded == false
  and .operator_canary_arm_readiness_persisted == false
  and .operator_canary_arm_readiness_authorizes_canary_arm == false
  and .operator_canary_arm_readiness_authorizes_live_execution == false
  and .operator_canary_arm_plan_recorded == false
  and .operator_canary_arm_plan_persisted == false
  and .operator_canary_arm_plan_accepted == false
  and .operator_canary_packet_recorded == false
  and .operator_canary_packet_persisted == false
  and .operator_canary_packet_accepted == false
  and .operator_canary_packet_authorizes_canary_arm == false
  and .operator_canary_packet_authorizes_live_execution == false
  and .canary_route_binding_accepted == false
  and .canary_namespace_binding_accepted == false
  and .canary_controlled_request_budget_accepted == false
  and .canary_controlled_request_dispatched_count == 0
  and .canary_controlled_request_executed_count == 0
  and .canary_rollback_kill_switch_accepted == false
  and .canary_rollback_kill_switch_armed == false
  and .canary_harness_shape_ready == true
  and .canary_harness_activation_ready == false
  and .canary_harness_armed == false
  and .canary_harness_executable == false
  and .canary_live_enabled == false
  and .canary_execution_performed == false
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
  and .network_call_performed == false
  and .external_db_write_performed == false
  and .live_kg_write_performed == false
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
echo "Hepta Memory/Intelligence/KG operator canary arm readiness scoreboard gate passed"
