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

CANARY_SCAFFOLD_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-canary-live-harness-scaffold-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-canary-live-harness-scaffold-gate.sh
)"

canary_scaffold_report_sha256="$(sha256_text "$CANARY_SCAFFOLD_JSON")"
operator_canary_packet_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-explicit-operator-approved-canary-packet-record-scaffold:v1:stdout-only:no-record:no-persist:no-accept:no-arm:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "explicit_operator_canary_packet_record_scaffold_side_effects=false;packet_recorded=false;packet_accepted=false;armed=false;executed=false;provider=false;model=false;memory=false;kg=false;secret=false;restart=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson canary "$CANARY_SCAFFOLD_JSON" \
  '
    $canary.runtime == "hepta"
    and $canary.status == "ready"
    and $canary.gate == "hepta_memory_intelligence_kg_full_enablement_canary_live_harness_scaffold_gate"
    and $canary.canary_live_harness_scaffold_ready == true
    and $canary.canary_live_harness_scaffold_status == "blocked"
    and $canary.canary_harness_shape_ready == true
    and $canary.canary_harness_activation_ready == false
    and $canary.canary_harness_armed == false
    and $canary.canary_harness_executable == false
    and $canary.canary_live_enabled == false
    and $canary.canary_execution_performed == false
    and $canary.canary_harness_stage_count == 5
    and $canary.canary_harness_stage_armed_count == 0
    and $canary.canary_harness_stage_executable_count == 0
    and $canary.canary_harness_stage_executed_count == 0
    and $canary.canary_harness_stage_live_enabled_count == 0
    and $canary.canary_guard_count == 12
    and $canary.canary_guard_accepted_count == 0
    and $canary.canary_guard_satisfied_count == 0
    and $canary.canary_guard_armed_count == 0
    and $canary.canary_guard_missing_count == 12
    and $canary.canary_controlled_request_dispatched_count == 0
    and $canary.canary_controlled_request_executed_count == 0
    and ($canary.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_explicit_operator_approved_canary_packet_record_scaffold_gate" \
  --arg canary_scaffold_report_sha256 "$canary_scaffold_report_sha256" \
  --arg operator_canary_packet_policy_hash_sha256 "$operator_canary_packet_policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --arg packet_id "hepta-memory-intelligence-kg-explicit-operator-approved-canary-packet-report-only-v1" \
  --arg packet_schema_id "hepta.memory_intelligence_kg.canary.operator_approved_packet.v1" \
  --arg packet_digest_placeholder "sha256:operator-canary-packet-values-not-recorded" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson canary "$CANARY_SCAFFOLD_JSON" \
  '
    [
      $canary.canary_guards[]
      | {
          field_order: .guard_order,
          field_id: .guard_id,
          source_guard_id: .guard_id,
          required: true,
          packet_field_shape_declared: true,
          operator_value_shape_declared: true,
          operator_value_present: false,
          operator_value_hash_recorded: false,
          packet_field_accepted: false,
          packet_field_satisfied: false,
          packet_field_live_enabling: false,
          packet_field_missing: true,
          status: "missing_operator_value"
        }
    ] as $fields
    | [
        $canary.canary_harness_stages[]
        | {
            stage_order: .stage_order,
            stage_id: .stage_id,
            source_phase_id: .source_phase_id,
            route_id: .route_id,
            namespace_id: .namespace_id,
            packet_binding_shape_declared: true,
            packet_binding_accepted: false,
            packet_binding_satisfied: false,
            stage_armed_by_packet: false,
            stage_executable_by_packet: false,
            stage_executed_by_packet: false,
            stage_live_enabled_by_packet: false,
            status: "blocked_packet_not_accepted"
          }
      ] as $stage_bindings
    | {
        product: $product,
        runtime: $runtime,
        status: "ready",
        base_url: $base_url,
        gate: $gate,
        explicit_operator_approved_canary_packet_record_scaffold_schema_version: "memory_intelligence_kg_full_enablement_explicit_operator_approved_canary_packet_record_scaffold_v1",
        explicit_operator_approved_canary_packet_record_scaffold_ready: true,
        explicit_operator_approved_canary_packet_record_scaffold_status: "blocked",
        explicit_operator_approved_canary_packet_record_scaffold_mode: "stdout_only_future_packet_shape_no_record_no_persist_no_accept_no_arm_no_live",
        explicit_operator_approved_canary_packet_record_scaffold_decision: "operator_canary_packet_record_shape_is_declared_but_no_operator_values_are_recorded_or_accepted_and_the_canary_remains_unarmed",
        minimum_required_samples: $min_long_soak_samples,
        source_canary_scaffold_gate: $canary.gate,
        source_canary_scaffold_report_sha256: $canary_scaffold_report_sha256,
        source_canary_scaffold_ready: $canary.canary_live_harness_scaffold_ready,
        source_canary_scaffold_status: $canary.canary_live_harness_scaffold_status,
        source_canary_harness_shape_ready: $canary.canary_harness_shape_ready,
        source_canary_harness_activation_ready: $canary.canary_harness_activation_ready,
        source_canary_guard_count: $canary.canary_guard_count,
        source_canary_guard_missing_count: $canary.canary_guard_missing_count,
        source_canary_stage_count: $canary.canary_harness_stage_count,
        source_canary_stage_blocked_count: $canary.canary_harness_stage_blocked_count,
        source_canary_max_controlled_request_count: $canary.canary_max_controlled_request_count,
        source_canary_controlled_request_dispatched_count: $canary.canary_controlled_request_dispatched_count,
        source_canary_controlled_request_executed_count: $canary.canary_controlled_request_executed_count,
        source_next_required_step: $canary.next_required_step,
        operator_canary_packet_policy_hash_sha256: $operator_canary_packet_policy_hash_sha256,
        side_effect_hash_sha256: $side_effect_hash_sha256,
        operator_canary_packet_id: $packet_id,
        operator_canary_packet_schema_id: $packet_schema_id,
        operator_canary_packet_digest_placeholder: $packet_digest_placeholder,
        operator_canary_packet_shape_declared: true,
        operator_canary_packet_fields: $fields,
        operator_canary_packet_field_count: ($fields | length),
        operator_canary_packet_required_field_count: ($fields | map(select(.required == true)) | length),
        operator_canary_packet_field_shape_declared_count: ($fields | map(select(.packet_field_shape_declared == true)) | length),
        operator_canary_packet_operator_value_present_count: ($fields | map(select(.operator_value_present == true)) | length),
        operator_canary_packet_operator_value_hash_recorded_count: ($fields | map(select(.operator_value_hash_recorded == true)) | length),
        operator_canary_packet_field_accepted_count: ($fields | map(select(.packet_field_accepted == true)) | length),
        operator_canary_packet_field_satisfied_count: ($fields | map(select(.packet_field_satisfied == true)) | length),
        operator_canary_packet_field_missing_count: ($fields | map(select(.packet_field_missing == true)) | length),
        operator_canary_packet_stage_bindings: $stage_bindings,
        operator_canary_packet_stage_binding_count: ($stage_bindings | length),
        operator_canary_packet_stage_binding_accepted_count: ($stage_bindings | map(select(.packet_binding_accepted == true)) | length),
        operator_canary_packet_stage_binding_satisfied_count: ($stage_bindings | map(select(.packet_binding_satisfied == true)) | length),
        operator_canary_packet_stage_armed_count: ($stage_bindings | map(select(.stage_armed_by_packet == true)) | length),
        operator_canary_packet_stage_executable_count: ($stage_bindings | map(select(.stage_executable_by_packet == true)) | length),
        operator_canary_packet_stage_executed_count: ($stage_bindings | map(select(.stage_executed_by_packet == true)) | length),
        operator_canary_packet_stage_live_enabled_count: ($stage_bindings | map(select(.stage_live_enabled_by_packet == true)) | length),
        operator_canary_packet_record_shape_declared: true,
        operator_canary_packet_recorded: false,
        operator_canary_packet_persisted: false,
        operator_canary_packet_materialized: false,
        operator_canary_packet_filesystem_written: false,
        operator_canary_packet_delivered: false,
        operator_canary_packet_accepted: false,
        operator_canary_packet_authorizes_canary_arm: false,
        operator_canary_packet_authorizes_live_execution: false,
        operator_identity_shape_declared: true,
        operator_identity_recorded: false,
        operator_identity_accepted: false,
        operator_signature_shape_declared: true,
        operator_signature_recorded: false,
        operator_signature_accepted: false,
        operator_timestamp_shape_declared: true,
        operator_timestamp_recorded: false,
        operator_timestamp_accepted: false,
        canary_route_id: $canary.canary_route_id,
        canary_namespace_id: $canary.canary_namespace_id,
        canary_rollback_kill_switch_id: $canary.canary_rollback_kill_switch_id,
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
        next_required_step: "populate_operator_canary_packet_values_and_accept_packet_before_any_canary_arm_or_live_execution",
        side_effects: {
          operator_canary_packet_recorded: false,
          operator_canary_packet_persisted: false,
          operator_canary_packet_materialized: false,
          operator_canary_packet_filesystem_written: false,
          operator_canary_packet_delivered: false,
          operator_canary_packet_accepted: false,
          operator_identity_recorded: false,
          operator_identity_accepted: false,
          operator_signature_recorded: false,
          operator_signature_accepted: false,
          operator_timestamp_recorded: false,
          operator_timestamp_accepted: false,
          canary_route_binding_accepted: false,
          canary_namespace_binding_accepted: false,
          canary_controlled_request_budget_accepted: false,
          canary_controlled_request_dispatched: false,
          canary_controlled_request_executed: false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_explicit_operator_approved_canary_packet_record_scaffold_gate"
  and .explicit_operator_approved_canary_packet_record_scaffold_ready == true
  and .explicit_operator_approved_canary_packet_record_scaffold_status == "blocked"
  and .source_canary_scaffold_gate == "hepta_memory_intelligence_kg_full_enablement_canary_live_harness_scaffold_gate"
  and .source_canary_scaffold_ready == true
  and .source_canary_scaffold_status == "blocked"
  and .source_canary_harness_shape_ready == true
  and .source_canary_harness_activation_ready == false
  and .source_canary_guard_count == 12
  and .source_canary_guard_missing_count == 12
  and .source_canary_stage_count == 5
  and .source_canary_stage_blocked_count == 5
  and .source_canary_max_controlled_request_count == 1
  and .source_canary_controlled_request_dispatched_count == 0
  and .source_canary_controlled_request_executed_count == 0
  and .operator_canary_packet_shape_declared == true
  and .operator_canary_packet_field_count == 12
  and .operator_canary_packet_required_field_count == 12
  and .operator_canary_packet_field_shape_declared_count == 12
  and .operator_canary_packet_operator_value_present_count == 0
  and .operator_canary_packet_operator_value_hash_recorded_count == 0
  and .operator_canary_packet_field_accepted_count == 0
  and .operator_canary_packet_field_satisfied_count == 0
  and .operator_canary_packet_field_missing_count == 12
  and (.operator_canary_packet_fields | length) == 12
  and (.operator_canary_packet_fields | all(
    .required == true
    and .packet_field_shape_declared == true
    and .operator_value_shape_declared == true
    and .operator_value_present == false
    and .operator_value_hash_recorded == false
    and .packet_field_accepted == false
    and .packet_field_satisfied == false
    and .packet_field_live_enabling == false
    and .packet_field_missing == true
    and .status == "missing_operator_value"
  ))
  and .operator_canary_packet_stage_binding_count == 5
  and .operator_canary_packet_stage_binding_accepted_count == 0
  and .operator_canary_packet_stage_binding_satisfied_count == 0
  and .operator_canary_packet_stage_armed_count == 0
  and .operator_canary_packet_stage_executable_count == 0
  and .operator_canary_packet_stage_executed_count == 0
  and .operator_canary_packet_stage_live_enabled_count == 0
  and (.operator_canary_packet_stage_bindings | length) == 5
  and (.operator_canary_packet_stage_bindings | all(
    .packet_binding_shape_declared == true
    and .packet_binding_accepted == false
    and .packet_binding_satisfied == false
    and .stage_armed_by_packet == false
    and .stage_executable_by_packet == false
    and .stage_executed_by_packet == false
    and .stage_live_enabled_by_packet == false
    and .status == "blocked_packet_not_accepted"
  ))
  and .operator_canary_packet_record_shape_declared == true
  and .operator_canary_packet_recorded == false
  and .operator_canary_packet_persisted == false
  and .operator_canary_packet_materialized == false
  and .operator_canary_packet_filesystem_written == false
  and .operator_canary_packet_delivered == false
  and .operator_canary_packet_accepted == false
  and .operator_canary_packet_authorizes_canary_arm == false
  and .operator_canary_packet_authorizes_live_execution == false
  and .operator_identity_recorded == false
  and .operator_identity_accepted == false
  and .operator_signature_recorded == false
  and .operator_signature_accepted == false
  and .operator_timestamp_recorded == false
  and .operator_timestamp_accepted == false
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
echo "Hepta Memory/Intelligence/KG explicit operator-approved canary packet record scaffold gate passed"
