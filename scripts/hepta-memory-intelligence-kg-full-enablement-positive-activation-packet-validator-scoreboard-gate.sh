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

SCAFFOLD_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-positive-activation-packet-dry-run-scaffold-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-positive-activation-packet-dry-run-scaffold-gate.sh
)"

scaffold_report_sha256="$(sha256_text "$SCAFFOLD_JSON")"
validator_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-positive-activation-packet-validator-scoreboard:v1:report-only:no-approval:no-live-enable:no-persistence:no-provider:no-memory-write:no-kg-read-write:no-secret"
)"
side_effect_hash_sha256="$(
  sha256_text "validator_scoreboard_side_effects=false;operator_approval=false;activation_packet=false;canary_live=false;memory_write=false;kg_read_write=false;provider_model=false;secret=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson scaffold "$SCAFFOLD_JSON" \
  '
    $scaffold.runtime == "hepta"
    and $scaffold.status == "ready"
    and $scaffold.gate == "hepta_memory_intelligence_kg_full_enablement_positive_activation_packet_dry_run_scaffold_gate"
    and $scaffold.positive_activation_packet_dry_run_scaffold_ready == true
    and $scaffold.positive_activation_packet_dry_run_scaffold_status == "blocked"
    and $scaffold.positive_activation_packet_dry_run_scaffold_mode == "stdout_only_report_only_positive_activation_packet_shape_no_approval_no_persistence_no_live_enablement"
    and $scaffold.source_report_count == 5
    and $scaffold.source_memory_surface_count == 14
    and $scaffold.source_absorbed_or_represented_memory_surface_count == 14
    and $scaffold.source_current_live_enabled_lane_count == 0
    and $scaffold.source_kg_source_gate_count == 5
    and $scaffold.source_kg_ready_source_gate_count == 5
    and $scaffold.source_kg_blocked_source_gate_count == 5
    and $scaffold.operator_approval_checklist_item_count == 7
    and $scaffold.missing_operator_approval_checklist_item_count == 7
    and $scaffold.bounded_activation_packet_item_count == 9
    and $scaffold.bounded_activation_packet_accepted_item_count == 0
    and $scaffold.runtime_attachment_packet_item_count == 12
    and $scaffold.runtime_attachment_packet_accepted_item_count == 0
    and $scaffold.memory_write_approval_packet_required_field_count == 21
    and $scaffold.memory_write_approval_packet_recorded_field_count == 0
    and $scaffold.phase_count == 5
    and $scaffold.ready_phase_count == 5
    and $scaffold.blocked_phase_count == 5
    and $scaffold.accepted_phase_count == 0
    and $scaffold.live_enabled_phase_count == 0
    and ($scaffold.activation_phases | length) == 5
    and ($scaffold.activation_phases | all(
      .packet_shape_declared == true
      and .status == "blocked"
      and .operator_approval_recorded == false
      and .operator_approval_accepted == false
      and .activation_packet_recorded == false
      and .activation_packet_persisted == false
      and .activation_packet_accepted == false
      and .activation_packet_delivered == false
      and .live_enabled == false
    ))
    and $scaffold.operator_approval_recorded == false
    and $scaffold.operator_approval_accepted == false
    and $scaffold.activation_packet_recorded == false
    and $scaffold.activation_packet_persisted == false
    and $scaffold.activation_packet_accepted == false
    and $scaffold.activation_packet_delivered == false
    and $scaffold.runtime_router_mutated == false
    and $scaffold.router_handoff_recorded == false
    and $scaffold.hepta_intelligence_context_attached == false
    and $scaffold.live_context_attached_to_prompt == false
    and $scaffold.context_injection_performed == false
    and $scaffold.provider_invoked == false
    and $scaffold.model_invoked == false
    and $scaffold.memory_write_operation_allowed == false
    and $scaffold.memory_store_write_performed == false
    and $scaffold.memory_store_mutated == false
    and $scaffold.external_kg_adapter_read_performed == false
    and $scaffold.network_call_performed == false
    and $scaffold.external_db_write_performed == false
    and $scaffold.live_kg_write_performed == false
    and $scaffold.credential_read == false
    and $scaffold.auth_secret_read == false
    and $scaffold.secret_file_read == false
    and $scaffold.service_restarted == false
    and $scaffold.active_binary_mutated == false
    and ($scaffold.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

authority_families_json="$(
  jq -n '
    [
      {
        family_id: "operator_approval_record",
        family_label: "operator approval record",
        missing_reason: "explicit operator approval record is missing"
      },
      {
        family_id: "single_route_namespace_scope",
        family_label: "single route and namespace scope",
        missing_reason: "single route and namespace scope binding is missing"
      },
      {
        family_id: "rollback_kill_switch",
        family_label: "rollback and kill switch",
        missing_reason: "accepted rollback and kill-switch controls are missing"
      },
      {
        family_id: "redaction_context_bounds",
        family_label: "redaction and context bounds",
        missing_reason: "accepted redaction and bounded context policy is missing"
      },
      {
        family_id: "audit_trail",
        family_label: "audit trail",
        missing_reason: "immutable audit trail target is missing"
      },
      {
        family_id: "readback_receipt",
        family_label: "readback receipt",
        missing_reason: "readback receipt and verification path are missing"
      },
      {
        family_id: "idempotency_nonce",
        family_label: "idempotency nonce",
        missing_reason: "accepted idempotency key and nonce binding are missing"
      },
      {
        family_id: "retention_export_observability",
        family_label: "retention export and observability",
        missing_reason: "retention, export/query, and observability acceptance is missing"
      }
    ]
  '
)"

canary_preconditions_json="$(
  jq -n '
    [
      "explicit_operator_approval",
      "accepted_activation_packet_digest",
      "single_route_binding",
      "single_namespace_binding",
      "zero_or_one_controlled_request_budget",
      "rollback_kill_switch_acceptance",
      "redaction_policy_acceptance",
      "readback_receipt_acceptance",
      "idempotency_nonce_acceptance",
      "audit_retention_acceptance",
      "provider_model_secret_use_policy_acceptance",
      "phase_specific_memory_kg_write_policy_acceptance"
    ]
  '
)"

missing_authority_reasons_json="$(
  jq -n '
    [
      "positive_activation_packet_validator_scoreboard_is_not_live_authority",
      "operator_approval_record_missing",
      "accepted_activation_packet_digest_missing",
      "single_route_namespace_scope_missing",
      "rollback_kill_switch_acceptance_missing",
      "redaction_context_bounds_acceptance_missing",
      "immutable_audit_trail_target_missing",
      "readback_receipt_acceptance_missing",
      "idempotency_nonce_acceptance_missing",
      "retention_export_observability_acceptance_missing",
      "provider_model_secret_use_policy_missing",
      "phase_specific_memory_kg_write_policy_missing"
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_positive_activation_packet_validator_scoreboard_gate" \
  --arg scaffold_report_sha256 "$scaffold_report_sha256" \
  --arg validator_policy_hash_sha256 "$validator_policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson scaffold "$SCAFFOLD_JSON" \
  --argjson authority_families "$authority_families_json" \
  --argjson canary_preconditions "$canary_preconditions_json" \
  --argjson missing_authority_reasons "$missing_authority_reasons_json" \
  '
    $scaffold.activation_phases as $phases
    | [
        $phases[] as $phase
        | $authority_families[] as $family
        | {
            phase_order: $phase.phase_order,
            phase_id: $phase.phase_id,
            phase_label: $phase.phase_label,
            source_gate: $phase.source_gate,
            source_report_sha256: $phase.source_report_sha256,
            authority_family_id: $family.family_id,
            authority_family_label: $family.family_label,
            required: true,
            scoreboard_item_shape_declared: true,
            authority_satisfied: false,
            missing: true,
            missing_reason: $family.missing_reason,
            live_enabled: false,
            status: "missing"
          }
      ] as $scoreboard
    | [
        range(0; ($canary_preconditions | length)) as $idx
        | {
            precondition_order: ($idx + 1),
            precondition_id: $canary_preconditions[$idx],
            required: true,
            shape_declared: true,
            satisfied: false,
            missing: true,
            live_enabling: false,
            status: "missing"
          }
      ] as $canary_scoreboard
    | {
        product: $product,
        runtime: $runtime,
        status: "ready",
        base_url: $base_url,
        gate: $gate,
        positive_activation_packet_validator_scoreboard_schema_version: "memory_intelligence_kg_full_enablement_positive_activation_packet_validator_scoreboard_v1",
        positive_activation_packet_validator_scoreboard_ready: true,
        positive_activation_packet_validator_scoreboard_status: "blocked",
        positive_activation_packet_validator_scoreboard_mode: "stdout_only_report_only_positive_activation_packet_validation_scoreboard_no_approval_no_live_enablement",
        positive_activation_packet_validator_scoreboard_decision: "phase_a_to_e_shapes_are_validated_but_live_canary_remains_blocked_until_required_positive_authority_items_are_satisfied",
        minimum_required_samples: $min_long_soak_samples,
        source_scaffold_gate: $scaffold.gate,
        source_scaffold_report_sha256: $scaffold_report_sha256,
        source_scaffold_status: $scaffold.positive_activation_packet_dry_run_scaffold_status,
        source_scaffold_ready: $scaffold.positive_activation_packet_dry_run_scaffold_ready,
        validator_policy_hash_sha256: $validator_policy_hash_sha256,
        side_effect_hash_sha256: $side_effect_hash_sha256,
        source_report_count: 1,
        source_scaffold_source_report_count: $scaffold.source_report_count,
        source_memory_surface_count: $scaffold.source_memory_surface_count,
        source_absorbed_or_represented_memory_surface_count: $scaffold.source_absorbed_or_represented_memory_surface_count,
        source_current_live_enabled_lane_count: $scaffold.source_current_live_enabled_lane_count,
        source_kg_source_gate_count: $scaffold.source_kg_source_gate_count,
        source_kg_ready_source_gate_count: $scaffold.source_kg_ready_source_gate_count,
        source_kg_blocked_source_gate_count: $scaffold.source_kg_blocked_source_gate_count,
        phase_count: ($phases | length),
        validated_phase_shape_count: ($phases | map(select(.packet_shape_declared == true)) | length),
        blocked_phase_count: ($phases | map(select(.status == "blocked")) | length),
        accepted_phase_count: ($phases | map(select(.activation_packet_accepted == true or .operator_approval_accepted == true)) | length),
        live_enabled_phase_count: ($phases | map(select(.live_enabled == true)) | length),
        authority_satisfied_phase_count: 0,
        required_authority_family_count: ($authority_families | length),
        required_scoreboard_item_count: ($scoreboard | length),
        shape_declared_scoreboard_item_count: ($scoreboard | map(select(.scoreboard_item_shape_declared == true)) | length),
        authority_satisfied_scoreboard_item_count: ($scoreboard | map(select(.authority_satisfied == true)) | length),
        missing_authority_scoreboard_item_count: ($scoreboard | map(select(.missing == true)) | length),
        phase_authority_scoreboard: $scoreboard,
        canary_harness_shape_ready: true,
        canary_harness_activation_ready: false,
        canary_harness_next_slice_allowed: true,
        canary_harness_next_slice_performs_live_activation: false,
        canary_harness_required_precondition_count: ($canary_scoreboard | length),
        canary_harness_satisfied_precondition_count: ($canary_scoreboard | map(select(.satisfied == true)) | length),
        canary_harness_missing_precondition_count: ($canary_scoreboard | map(select(.missing == true)) | length),
        canary_harness_preconditions: $canary_scoreboard,
        missing_authority_reason_count: ($missing_authority_reasons | length),
        missing_authority_reasons: $missing_authority_reasons,
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
        memory_context_activation_handoff_persisted: false,
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
        credential_read: false,
        auth_secret_read: false,
        secret_file_read: false,
        rollback_executed: false,
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
          memory_context_activation_handoff_persisted: false,
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
          credential_read: false,
          auth_secret_read: false,
          secret_file_read: false,
          rollback_executed: false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_positive_activation_packet_validator_scoreboard_gate"
  and .positive_activation_packet_validator_scoreboard_ready == true
  and .positive_activation_packet_validator_scoreboard_status == "blocked"
  and .positive_activation_packet_validator_scoreboard_mode == "stdout_only_report_only_positive_activation_packet_validation_scoreboard_no_approval_no_live_enablement"
  and .source_scaffold_gate == "hepta_memory_intelligence_kg_full_enablement_positive_activation_packet_dry_run_scaffold_gate"
  and .source_scaffold_status == "blocked"
  and .source_scaffold_ready == true
  and .source_report_count == 1
  and .source_scaffold_source_report_count == 5
  and .source_memory_surface_count == 14
  and .source_absorbed_or_represented_memory_surface_count == 14
  and .source_current_live_enabled_lane_count == 0
  and .source_kg_source_gate_count == 5
  and .source_kg_ready_source_gate_count == 5
  and .source_kg_blocked_source_gate_count == 5
  and .phase_count == 5
  and .validated_phase_shape_count == 5
  and .blocked_phase_count == 5
  and .accepted_phase_count == 0
  and .live_enabled_phase_count == 0
  and .authority_satisfied_phase_count == 0
  and .required_authority_family_count == 8
  and .required_scoreboard_item_count == 40
  and .shape_declared_scoreboard_item_count == 40
  and .authority_satisfied_scoreboard_item_count == 0
  and .missing_authority_scoreboard_item_count == 40
  and (.phase_authority_scoreboard | length) == 40
  and (.phase_authority_scoreboard | all(
    .required == true
    and .scoreboard_item_shape_declared == true
    and .authority_satisfied == false
    and .missing == true
    and .live_enabled == false
    and .status == "missing"
  ))
  and .canary_harness_shape_ready == true
  and .canary_harness_activation_ready == false
  and .canary_harness_next_slice_allowed == true
  and .canary_harness_next_slice_performs_live_activation == false
  and .canary_harness_required_precondition_count == 12
  and .canary_harness_satisfied_precondition_count == 0
  and .canary_harness_missing_precondition_count == 12
  and (.canary_harness_preconditions | length) == 12
  and (.canary_harness_preconditions | all(
    .required == true
    and .shape_declared == true
    and .satisfied == false
    and .missing == true
    and .live_enabling == false
    and .status == "missing"
  ))
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
  and .usage_recorded == false
  and .memory_write_operation_allowed == false
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
echo "Hepta Memory/Intelligence/KG positive activation packet validator scoreboard gate passed"
