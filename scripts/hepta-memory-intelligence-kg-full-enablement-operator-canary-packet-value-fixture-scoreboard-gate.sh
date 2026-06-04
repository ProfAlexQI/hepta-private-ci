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

PACKET_SCAFFOLD_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-explicit-operator-approved-canary-packet-record-scaffold-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-explicit-operator-approved-canary-packet-record-scaffold-gate.sh
)"

packet_scaffold_report_sha256="$(sha256_text "$PACKET_SCAFFOLD_JSON")"
value_fixture_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-packet-value-fixture-scoreboard:v1:synthetic-values-only:no-record:no-accept:no-arm:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_canary_packet_value_fixture_scoreboard_side_effects=false;synthetic_values=true;accepted=false;armed=false;executed=false;provider=false;model=false;memory=false;kg=false;secret=false;restart=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson scaffold "$PACKET_SCAFFOLD_JSON" \
  '
    $scaffold.runtime == "hepta"
    and $scaffold.status == "ready"
    and $scaffold.gate == "hepta_memory_intelligence_kg_full_enablement_explicit_operator_approved_canary_packet_record_scaffold_gate"
    and $scaffold.explicit_operator_approved_canary_packet_record_scaffold_ready == true
    and $scaffold.explicit_operator_approved_canary_packet_record_scaffold_status == "blocked"
    and $scaffold.operator_canary_packet_shape_declared == true
    and $scaffold.operator_canary_packet_field_count == 12
    and $scaffold.operator_canary_packet_field_missing_count == 12
    and $scaffold.operator_canary_packet_stage_binding_count == 5
    and $scaffold.operator_canary_packet_accepted == false
    and $scaffold.operator_canary_packet_authorizes_canary_arm == false
    and $scaffold.operator_canary_packet_authorizes_live_execution == false
    and $scaffold.canary_harness_armed == false
    and $scaffold.canary_harness_executable == false
    and $scaffold.canary_live_enabled == false
    and $scaffold.canary_execution_performed == false
    and ($scaffold.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_packet_value_fixture_scoreboard_gate" \
    --arg packet_scaffold_report_sha256 "$packet_scaffold_report_sha256" \
    --arg value_fixture_policy_hash_sha256 "$value_fixture_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --arg synthetic_packet_id "hepta-memory-intelligence-kg-synthetic-operator-canary-packet-values-report-only-v1" \
    --arg synthetic_packet_schema_id "hepta.memory_intelligence_kg.canary.synthetic_operator_packet_values.v1" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson scaffold "$PACKET_SCAFFOLD_JSON" \
    '
      [
        $scaffold.operator_canary_packet_fields[]
        | {
            field_order: .field_order,
            field_id: .field_id,
            source_guard_id: .source_guard_id,
            required: true,
            source_packet_field_shape_declared: .packet_field_shape_declared,
            synthetic_value_fixture_shape_declared: true,
            synthetic_value_present: true,
            synthetic_value_kind: (
              if .field_id == "explicit_operator_approval" then "operator_approval_reference_placeholder"
              elif .field_id == "accepted_activation_packet_digest" then "activation_packet_digest_placeholder"
              elif .field_id == "single_route_binding" then "single_route_binding_placeholder"
              elif .field_id == "single_namespace_binding" then "single_namespace_binding_placeholder"
              elif .field_id == "zero_or_one_controlled_request_budget" then "controlled_request_budget_placeholder"
              elif .field_id == "rollback_kill_switch_acceptance" then "rollback_kill_switch_acceptance_placeholder"
              elif .field_id == "redaction_policy_acceptance" then "redaction_policy_acceptance_placeholder"
              elif .field_id == "readback_receipt_acceptance" then "readback_receipt_acceptance_placeholder"
              elif .field_id == "idempotency_nonce_acceptance" then "idempotency_nonce_acceptance_placeholder"
              elif .field_id == "audit_retention_acceptance" then "audit_retention_acceptance_placeholder"
              elif .field_id == "provider_model_secret_use_policy_acceptance" then "provider_model_secret_use_policy_acceptance_placeholder"
              elif .field_id == "phase_specific_memory_kg_write_policy_acceptance" then "phase_specific_memory_kg_write_policy_acceptance_placeholder"
              else "unknown_placeholder"
              end
            ),
            synthetic_value_digest_placeholder: ("sha256:synthetic-report-only-" + .field_id),
            synthetic_value_is_operator_authority: false,
            operator_value_recorded: false,
            operator_value_persisted: false,
            operator_value_hash_recorded: false,
            operator_value_trusted: false,
            operator_value_accepted: false,
            packet_field_accepted: false,
            packet_field_satisfied: false,
            packet_field_live_enabling: false,
            status: "synthetic_value_present_but_untrusted"
          }
      ] as $values
      | [
          {
            precondition_order: 1,
            precondition_id: "trusted_operator_identity_accepted",
            family: "operator_identity",
            required: true,
            synthetic_fixture_covers_shape: true,
            satisfied: false,
            missing: true
          },
          {
            precondition_order: 2,
            precondition_id: "trusted_operator_signature_verified",
            family: "operator_signature",
            required: true,
            synthetic_fixture_covers_shape: true,
            satisfied: false,
            missing: true
          },
          {
            precondition_order: 3,
            precondition_id: "operator_timestamp_freshness_accepted",
            family: "operator_timestamp",
            required: true,
            synthetic_fixture_covers_shape: true,
            satisfied: false,
            missing: true
          },
          {
            precondition_order: 4,
            precondition_id: "packet_digest_bound_to_all_field_values",
            family: "packet_integrity",
            required: true,
            synthetic_fixture_covers_shape: true,
            satisfied: false,
            missing: true
          },
          {
            precondition_order: 5,
            precondition_id: "source_canary_scaffold_hash_pinned",
            family: "source_binding",
            required: true,
            synthetic_fixture_covers_shape: true,
            satisfied: false,
            missing: true
          },
          {
            precondition_order: 6,
            precondition_id: "single_route_binding_accepted",
            family: "scope",
            required: true,
            synthetic_fixture_covers_shape: true,
            satisfied: false,
            missing: true
          },
          {
            precondition_order: 7,
            precondition_id: "single_namespace_binding_accepted",
            family: "scope",
            required: true,
            synthetic_fixture_covers_shape: true,
            satisfied: false,
            missing: true
          },
          {
            precondition_order: 8,
            precondition_id: "controlled_request_budget_accepted",
            family: "blast_radius",
            required: true,
            synthetic_fixture_covers_shape: true,
            satisfied: false,
            missing: true
          },
          {
            precondition_order: 9,
            precondition_id: "rollback_kill_switch_accepted_and_armable",
            family: "rollback",
            required: true,
            synthetic_fixture_covers_shape: true,
            satisfied: false,
            missing: true
          },
          {
            precondition_order: 10,
            precondition_id: "redaction_policy_accepted",
            family: "privacy",
            required: true,
            synthetic_fixture_covers_shape: true,
            satisfied: false,
            missing: true
          },
          {
            precondition_order: 11,
            precondition_id: "readback_receipt_acceptance_policy_accepted",
            family: "readback",
            required: true,
            synthetic_fixture_covers_shape: true,
            satisfied: false,
            missing: true
          },
          {
            precondition_order: 12,
            precondition_id: "audit_retention_export_observability_policy_accepted",
            family: "audit_retention",
            required: true,
            synthetic_fixture_covers_shape: true,
            satisfied: false,
            missing: true
          },
          {
            precondition_order: 13,
            precondition_id: "provider_model_secret_use_policy_accepted",
            family: "provider_model_secret_boundary",
            required: true,
            synthetic_fixture_covers_shape: true,
            satisfied: false,
            missing: true
          },
          {
            precondition_order: 14,
            precondition_id: "phase_specific_memory_kg_write_policy_accepted",
            family: "memory_kg_write_boundary",
            required: true,
            synthetic_fixture_covers_shape: true,
            satisfied: false,
            missing: true
          },
          {
            precondition_order: 15,
            precondition_id: "packet_record_persistence_approved",
            family: "record_persistence",
            required: true,
            synthetic_fixture_covers_shape: true,
            satisfied: false,
            missing: true
          },
          {
            precondition_order: 16,
            precondition_id: "packet_acceptance_completion_ack_accepted",
            family: "completion_ack",
            required: true,
            synthetic_fixture_covers_shape: true,
            satisfied: false,
            missing: true
          }
        ] as $preconditions
      | [
          $scaffold.operator_canary_packet_stage_bindings[]
          | {
              stage_order: .stage_order,
              stage_id: .stage_id,
              source_phase_id: .source_phase_id,
              route_id: .route_id,
              namespace_id: .namespace_id,
              synthetic_packet_value_fixture_bound: true,
              acceptance_preconditions_satisfied: false,
              packet_binding_accepted: false,
              stage_armed_by_packet: false,
              stage_executable_by_packet: false,
              stage_executed_by_packet: false,
              stage_live_enabled_by_packet: false,
              status: "blocked_synthetic_fixture_not_accepted"
            }
        ] as $stage_bindings
      | {
          product: $product,
          runtime: $runtime,
          status: "ready",
          base_url: $base_url,
          gate: $gate,
          operator_canary_packet_value_fixture_scoreboard_schema_version: "memory_intelligence_kg_operator_canary_packet_value_fixture_scoreboard_v1",
          operator_canary_packet_value_fixture_scoreboard_ready: true,
          operator_canary_packet_value_fixture_scoreboard_status: "blocked",
          operator_canary_packet_value_fixture_scoreboard_mode: "synthetic_values_report_only_no_record_no_accept_no_arm_no_live",
          operator_canary_packet_value_fixture_scoreboard_decision: "synthetic_operator_canary_packet_values_are_complete_as_fixture_data_but_no_trusted_operator_record_or_acceptance_preconditions_are_satisfied",
          minimum_required_samples: $min_long_soak_samples,
          source_packet_scaffold_gate: $scaffold.gate,
          source_packet_scaffold_report_sha256: $packet_scaffold_report_sha256,
          source_packet_scaffold_ready: $scaffold.explicit_operator_approved_canary_packet_record_scaffold_ready,
          source_packet_scaffold_status: $scaffold.explicit_operator_approved_canary_packet_record_scaffold_status,
          source_packet_field_count: $scaffold.operator_canary_packet_field_count,
          source_packet_field_missing_count: $scaffold.operator_canary_packet_field_missing_count,
          source_stage_binding_count: $scaffold.operator_canary_packet_stage_binding_count,
          value_fixture_policy_hash_sha256: $value_fixture_policy_hash_sha256,
          side_effect_hash_sha256: $side_effect_hash_sha256,
          synthetic_operator_canary_packet_id: $synthetic_packet_id,
          synthetic_operator_canary_packet_schema_id: $synthetic_packet_schema_id,
          synthetic_operator_canary_packet_value_fixture_declared: true,
          synthetic_operator_canary_packet_value_fixture_complete: true,
          synthetic_operator_canary_packet_values: $values,
          synthetic_operator_canary_packet_value_field_count: ($values | length),
          synthetic_operator_canary_packet_value_present_count: ($values | map(select(.synthetic_value_present == true)) | length),
          synthetic_operator_canary_packet_value_trusted_count: ($values | map(select(.operator_value_trusted == true)) | length),
          synthetic_operator_canary_packet_value_accepted_count: ($values | map(select(.operator_value_accepted == true)) | length),
          synthetic_operator_canary_packet_field_accepted_count: ($values | map(select(.packet_field_accepted == true)) | length),
          operator_value_recorded_count: ($values | map(select(.operator_value_recorded == true)) | length),
          operator_value_persisted_count: ($values | map(select(.operator_value_persisted == true)) | length),
          operator_value_hash_recorded_count: ($values | map(select(.operator_value_hash_recorded == true)) | length),
          packet_acceptance_preconditions: $preconditions,
          packet_acceptance_precondition_count: ($preconditions | length),
          packet_acceptance_precondition_shape_covered_count: ($preconditions | map(select(.synthetic_fixture_covers_shape == true)) | length),
          packet_acceptance_precondition_satisfied_count: ($preconditions | map(select(.satisfied == true)) | length),
          packet_acceptance_precondition_missing_count: ($preconditions | map(select(.missing == true)) | length),
          packet_acceptance_ready: false,
          packet_acceptance_blocked_reason: "synthetic_fixture_is_not_a_trusted_operator_record",
          operator_canary_packet_stage_bindings: $stage_bindings,
          operator_canary_packet_stage_binding_count: ($stage_bindings | length),
          operator_canary_packet_stage_binding_fixture_bound_count: ($stage_bindings | map(select(.synthetic_packet_value_fixture_bound == true)) | length),
          operator_canary_packet_stage_binding_accepted_count: ($stage_bindings | map(select(.packet_binding_accepted == true)) | length),
          operator_canary_packet_stage_armed_count: ($stage_bindings | map(select(.stage_armed_by_packet == true)) | length),
          operator_canary_packet_stage_executable_count: ($stage_bindings | map(select(.stage_executable_by_packet == true)) | length),
          operator_canary_packet_stage_executed_count: ($stage_bindings | map(select(.stage_executed_by_packet == true)) | length),
          operator_canary_packet_stage_live_enabled_count: ($stage_bindings | map(select(.stage_live_enabled_by_packet == true)) | length),
          operator_canary_packet_recorded: false,
          operator_canary_packet_persisted: false,
          operator_canary_packet_materialized: false,
          operator_canary_packet_filesystem_written: false,
          operator_canary_packet_delivered: false,
          operator_canary_packet_accepted: false,
          operator_canary_packet_authorizes_canary_arm: false,
          operator_canary_packet_authorizes_live_execution: false,
          operator_identity_recorded: false,
          operator_identity_accepted: false,
          operator_signature_recorded: false,
          operator_signature_accepted: false,
          operator_timestamp_recorded: false,
          operator_timestamp_accepted: false,
          canary_route_id: $scaffold.canary_route_id,
          canary_namespace_id: $scaffold.canary_namespace_id,
          canary_rollback_kill_switch_id: $scaffold.canary_rollback_kill_switch_id,
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
          next_required_step: "replace_synthetic_values_with_trusted_operator_record_and_accept_all_preconditions_before_canary_arm",
          side_effects: {
            synthetic_fixture_workspace_written: false,
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
    '
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_packet_value_fixture_scoreboard_gate"
  and .operator_canary_packet_value_fixture_scoreboard_ready == true
  and .operator_canary_packet_value_fixture_scoreboard_status == "blocked"
  and .source_packet_scaffold_gate == "hepta_memory_intelligence_kg_full_enablement_explicit_operator_approved_canary_packet_record_scaffold_gate"
  and .source_packet_scaffold_ready == true
  and .source_packet_scaffold_status == "blocked"
  and .source_packet_field_count == 12
  and .source_packet_field_missing_count == 12
  and .source_stage_binding_count == 5
  and .synthetic_operator_canary_packet_value_fixture_declared == true
  and .synthetic_operator_canary_packet_value_fixture_complete == true
  and .synthetic_operator_canary_packet_value_field_count == 12
  and .synthetic_operator_canary_packet_value_present_count == 12
  and .synthetic_operator_canary_packet_value_trusted_count == 0
  and .synthetic_operator_canary_packet_value_accepted_count == 0
  and .synthetic_operator_canary_packet_field_accepted_count == 0
  and .operator_value_recorded_count == 0
  and .operator_value_persisted_count == 0
  and .operator_value_hash_recorded_count == 0
  and (.synthetic_operator_canary_packet_values | length) == 12
  and (.synthetic_operator_canary_packet_values | all(
    .required == true
    and .source_packet_field_shape_declared == true
    and .synthetic_value_fixture_shape_declared == true
    and .synthetic_value_present == true
    and .synthetic_value_is_operator_authority == false
    and .operator_value_recorded == false
    and .operator_value_persisted == false
    and .operator_value_hash_recorded == false
    and .operator_value_trusted == false
    and .operator_value_accepted == false
    and .packet_field_accepted == false
    and .packet_field_satisfied == false
    and .packet_field_live_enabling == false
    and .status == "synthetic_value_present_but_untrusted"
  ))
  and .packet_acceptance_precondition_count == 16
  and .packet_acceptance_precondition_shape_covered_count == 16
  and .packet_acceptance_precondition_satisfied_count == 0
  and .packet_acceptance_precondition_missing_count == 16
  and (.packet_acceptance_preconditions | all(
    .required == true
    and .synthetic_fixture_covers_shape == true
    and .satisfied == false
    and .missing == true
  ))
  and .packet_acceptance_ready == false
  and .packet_acceptance_blocked_reason == "synthetic_fixture_is_not_a_trusted_operator_record"
  and .operator_canary_packet_stage_binding_count == 5
  and .operator_canary_packet_stage_binding_fixture_bound_count == 5
  and .operator_canary_packet_stage_binding_accepted_count == 0
  and .operator_canary_packet_stage_armed_count == 0
  and .operator_canary_packet_stage_executable_count == 0
  and .operator_canary_packet_stage_executed_count == 0
  and .operator_canary_packet_stage_live_enabled_count == 0
  and (.operator_canary_packet_stage_bindings | all(
    .synthetic_packet_value_fixture_bound == true
    and .acceptance_preconditions_satisfied == false
    and .packet_binding_accepted == false
    and .stage_armed_by_packet == false
    and .stage_executable_by_packet == false
    and .stage_executed_by_packet == false
    and .stage_live_enabled_by_packet == false
    and .status == "blocked_synthetic_fixture_not_accepted"
  ))
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
echo "Hepta Memory/Intelligence/KG operator canary packet value fixture scoreboard gate passed"
