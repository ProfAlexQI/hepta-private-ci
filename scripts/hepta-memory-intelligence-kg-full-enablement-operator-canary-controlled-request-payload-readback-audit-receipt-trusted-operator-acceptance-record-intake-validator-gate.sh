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
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-trusted-operator-acceptance-record-scaffold-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-trusted-operator-acceptance-record-scaffold-gate.sh
)"

scaffold_report_sha256="$(sha256_text "$SCAFFOLD_JSON")"
intake_validator_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-trusted-operator-acceptance-record-intake-validator:v1:source-scaffold:report-only:no-operator-record:no-accept:no-dispatch:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_canary_trusted_operator_acceptance_record_intake_validator_side_effects=false;operator_record_present=false;trusted=false;accepted=false;dispatch=false;execute=false;context=false;provider=false;model=false;memory=false;kg=false;secret=false;restart=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$SCAFFOLD_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_scaffold_gate"
    and $source.operator_canary_trusted_operator_acceptance_record_scaffold_ready == true
    and $source.operator_canary_trusted_operator_acceptance_record_scaffold_status == "blocked"
    and $source.source_acceptance_packet_count == 5
    and $source.source_value_scoreboard_item_count == 80
    and $source.source_value_scoreboard_trusted_value_count == 0
    and $source.source_value_scoreboard_accepted_value_count == 0
    and $source.source_value_scoreboard_missing_authority_count == 80
    and $source.operator_canary_trusted_operator_acceptance_record_count == 5
    and $source.operator_canary_trusted_operator_acceptance_record_shape_declared_count == 5
    and $source.operator_canary_trusted_operator_acceptance_record_materialized_count == 5
    and $source.operator_canary_trusted_operator_acceptance_record_hash_bound_count == 5
    and $source.operator_canary_trusted_operator_identity_accepted_count == 0
    and $source.operator_canary_trusted_operator_signature_hash_accepted_count == 0
    and $source.operator_canary_trusted_operator_timestamp_accepted_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_recorded_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_persisted_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_delivered_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_accepted_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_authorizes_dispatch_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_authorizes_context_attachment_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_authorizes_provider_model_invocation_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_authorizes_memory_write_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_authorizes_external_kg_read_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_authorizes_live_kg_write_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_authorizes_live_execution_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_required_value_count == 80
    and $source.operator_canary_trusted_operator_acceptance_record_trusted_value_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_missing_value_count == 80
    and ($source.operator_canary_trusted_operator_acceptance_records | all(
      .trusted_operator_acceptance_record_shape_declared == true
      and .trusted_operator_acceptance_record_hash_bound_to_value_scoreboard == true
      and .trusted_operator_identity_accepted == false
      and .trusted_operator_signature_hash_accepted == false
      and .trusted_operator_timestamp_accepted == false
      and .trusted_operator_acceptance_record_recorded == false
      and .trusted_operator_acceptance_record_persisted == false
      and .trusted_operator_acceptance_record_delivered == false
      and .trusted_operator_acceptance_record_accepted == false
      and .trusted_operator_acceptance_record_authorizes_dispatch == false
      and .trusted_operator_acceptance_record_authorizes_live_execution == false
    ))
    and $source.operator_canary_trusted_operator_acceptance_record_family_count == 9
    and $source.operator_canary_trusted_operator_acceptance_record_family_accepted_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_family_authorizes_dispatch_count == 0
    and $source.operator_canary_trusted_operator_acceptance_record_family_authorizes_live_execution_count == 0
    and $source.canary_harness_shape_ready == true
    and $source.canary_harness_activation_ready == false
    and $source.canary_harness_armed == false
    and $source.canary_harness_executable == false
    and $source.canary_live_enabled == false
    and $source.canary_execution_performed == false
    and $source.controlled_request_dispatched == false
    and $source.controlled_request_executed == false
    and $source.context_injection_performed == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.memory_store_write_performed == false
    and $source.memory_store_mutated == false
    and $source.external_kg_adapter_read_performed == false
    and $source.live_kg_write_performed == false
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
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_intake_validator_gate" \
    --arg scaffold_report_sha256 "$scaffold_report_sha256" \
    --arg intake_validator_policy_hash_sha256 "$intake_validator_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$SCAFFOLD_JSON" \
    '
      [
        "operator_identity",
        "operator_signature_hash",
        "signed_at_rfc3339",
        "approval_scope_id",
        "route_id",
        "namespace_id",
        "source_value_scoreboard_sha256",
        "source_acceptance_record_scaffold_sha256",
        "redaction_policy_hash",
        "readback_receipt_sha256",
        "audit_receipt_sha256",
        "idempotency_nonce",
        "rollback_plan_id",
        "kill_switch_id",
        "dispatch_budget",
        "live_execution_bounds"
      ] as $required_fields
      | [
          $source.operator_canary_trusted_operator_acceptance_records[]
          | {
              intake_record_order: .acceptance_record_order,
              intake_record_id: ("trusted-operator-intake-validator-" + .acceptance_record_id),
              acceptance_record_id: .acceptance_record_id,
              acceptance_packet_id: .acceptance_packet_id,
              stage_id: .stage_id,
              source_phase_id: .source_phase_id,
              route_id: .route_id,
              namespace_id: .namespace_id,
              source_record_scaffold_hash_bound: true,
              source_record_scaffold_sha256: $scaffold_report_sha256,
              required_fields: $required_fields,
              required_field_count: ($required_fields | length),
              operator_record_supplied: false,
              operator_record_present_field_count: 0,
              operator_record_missing_field_count: ($required_fields | length),
              operator_record_trusted_field_count: 0,
              operator_record_accepted_field_count: 0,
              operator_identity_validated: false,
              operator_signature_hash_validated: false,
              operator_timestamp_validated: false,
              operator_scope_validated: false,
              operator_redaction_validated: false,
              readback_receipt_validated: false,
              audit_receipt_validated: false,
              idempotency_nonce_validated: false,
              rollback_kill_switch_validated: false,
              dispatch_budget_validated: false,
              no_write_boundary_validated: false,
              trusted_operator_acceptance_record_accepted: false,
              trusted_operator_acceptance_record_recorded: false,
              trusted_operator_acceptance_record_persisted: false,
              trusted_operator_acceptance_record_delivered: false,
              authorizes_canary_dispatch: false,
              authorizes_context_attachment: false,
              authorizes_provider_model_invocation: false,
              authorizes_memory_write: false,
              authorizes_external_kg_read: false,
              authorizes_live_kg_write: false,
              authorizes_live_execution: false,
              status: "blocked_operator_record_not_supplied"
            }
        ] as $intake_records
      | [
          $source.operator_canary_trusted_operator_acceptance_record_family_records[]
          | {
              authority_family: .authority_family,
              source_family_item_count: .source_family_item_count,
              source_family_missing_authority_count: .source_family_missing_authority_count,
              intake_validator_family_shape_declared: true,
              intake_validator_family_required: true,
              intake_validator_family_accepted: false,
              intake_validator_family_authorizes_dispatch: false,
              intake_validator_family_authorizes_live_execution: false,
              status: "blocked_operator_record_family_not_accepted"
            }
        ] as $family_records
      | {
          product: $product,
          runtime: $runtime,
          status: "ready",
          base_url: $base_url,
          gate: $gate,
          operator_canary_trusted_operator_acceptance_record_intake_validator_schema_version: "memory_intelligence_kg_operator_canary_trusted_operator_acceptance_record_intake_validator_v1",
          operator_canary_trusted_operator_acceptance_record_intake_validator_ready: true,
          operator_canary_trusted_operator_acceptance_record_intake_validator_status: "blocked",
          operator_canary_trusted_operator_acceptance_record_intake_validator_mode: "report_only_operator_record_intake_shape_no_record_supplied_no_accept_no_dispatch_no_execute_no_live",
          operator_canary_trusted_operator_acceptance_record_intake_validator_decision: "a_real_trusted_operator_acceptance_record_must_supply_identity_signature_timestamp_scope_readback_audit_idempotency_rollback_budget_and_live_bounds_before_canary_dispatch_can_be_armed",
          minimum_required_samples: $min_long_soak_samples,
          source_trusted_operator_acceptance_record_scaffold_gate: $source.gate,
          source_trusted_operator_acceptance_record_scaffold_report_sha256: $scaffold_report_sha256,
          source_trusted_operator_acceptance_record_scaffold_ready: $source.operator_canary_trusted_operator_acceptance_record_scaffold_ready,
          source_trusted_operator_acceptance_record_scaffold_status: $source.operator_canary_trusted_operator_acceptance_record_scaffold_status,
          source_trusted_operator_acceptance_record_count: $source.operator_canary_trusted_operator_acceptance_record_count,
          source_trusted_operator_acceptance_record_accepted_count: $source.operator_canary_trusted_operator_acceptance_record_accepted_count,
          source_trusted_operator_acceptance_record_recorded_count: $source.operator_canary_trusted_operator_acceptance_record_recorded_count,
          source_trusted_operator_acceptance_record_persisted_count: $source.operator_canary_trusted_operator_acceptance_record_persisted_count,
          source_trusted_operator_acceptance_record_missing_value_count: $source.operator_canary_trusted_operator_acceptance_record_missing_value_count,
          intake_validator_policy_hash_sha256: $intake_validator_policy_hash_sha256,
          side_effect_hash_sha256: $side_effect_hash_sha256,
          operator_canary_trusted_operator_acceptance_record_intake_records: $intake_records,
          operator_canary_trusted_operator_acceptance_record_intake_record_count: ($intake_records | length),
          operator_canary_trusted_operator_acceptance_record_intake_record_hash_bound_count: ($intake_records | map(select(.source_record_scaffold_hash_bound == true)) | length),
          operator_canary_trusted_operator_acceptance_record_intake_required_field_count: ($intake_records | map(.required_field_count) | add),
          operator_canary_trusted_operator_acceptance_record_intake_present_field_count: ($intake_records | map(.operator_record_present_field_count) | add),
          operator_canary_trusted_operator_acceptance_record_intake_missing_field_count: ($intake_records | map(.operator_record_missing_field_count) | add),
          operator_canary_trusted_operator_acceptance_record_intake_trusted_field_count: ($intake_records | map(.operator_record_trusted_field_count) | add),
          operator_canary_trusted_operator_acceptance_record_intake_accepted_field_count: ($intake_records | map(.operator_record_accepted_field_count) | add),
          operator_canary_trusted_operator_acceptance_record_intake_identity_validated_count: ($intake_records | map(select(.operator_identity_validated == true)) | length),
          operator_canary_trusted_operator_acceptance_record_intake_signature_validated_count: ($intake_records | map(select(.operator_signature_hash_validated == true)) | length),
          operator_canary_trusted_operator_acceptance_record_intake_timestamp_validated_count: ($intake_records | map(select(.operator_timestamp_validated == true)) | length),
          operator_canary_trusted_operator_acceptance_record_intake_scope_validated_count: ($intake_records | map(select(.operator_scope_validated == true)) | length),
          operator_canary_trusted_operator_acceptance_record_intake_rollback_kill_switch_validated_count: ($intake_records | map(select(.rollback_kill_switch_validated == true)) | length),
          operator_canary_trusted_operator_acceptance_record_intake_record_accepted_count: ($intake_records | map(select(.trusted_operator_acceptance_record_accepted == true)) | length),
          operator_canary_trusted_operator_acceptance_record_intake_record_recorded_count: ($intake_records | map(select(.trusted_operator_acceptance_record_recorded == true)) | length),
          operator_canary_trusted_operator_acceptance_record_intake_record_persisted_count: ($intake_records | map(select(.trusted_operator_acceptance_record_persisted == true)) | length),
          operator_canary_trusted_operator_acceptance_record_intake_record_delivered_count: ($intake_records | map(select(.trusted_operator_acceptance_record_delivered == true)) | length),
          operator_canary_trusted_operator_acceptance_record_intake_authorizes_dispatch_count: ($intake_records | map(select(.authorizes_canary_dispatch == true)) | length),
          operator_canary_trusted_operator_acceptance_record_intake_authorizes_context_attachment_count: ($intake_records | map(select(.authorizes_context_attachment == true)) | length),
          operator_canary_trusted_operator_acceptance_record_intake_authorizes_provider_model_invocation_count: ($intake_records | map(select(.authorizes_provider_model_invocation == true)) | length),
          operator_canary_trusted_operator_acceptance_record_intake_authorizes_memory_write_count: ($intake_records | map(select(.authorizes_memory_write == true)) | length),
          operator_canary_trusted_operator_acceptance_record_intake_authorizes_external_kg_read_count: ($intake_records | map(select(.authorizes_external_kg_read == true)) | length),
          operator_canary_trusted_operator_acceptance_record_intake_authorizes_live_kg_write_count: ($intake_records | map(select(.authorizes_live_kg_write == true)) | length),
          operator_canary_trusted_operator_acceptance_record_intake_authorizes_live_execution_count: ($intake_records | map(select(.authorizes_live_execution == true)) | length),
          operator_canary_trusted_operator_acceptance_record_intake_family_records: $family_records,
          operator_canary_trusted_operator_acceptance_record_intake_family_count: ($family_records | length),
          operator_canary_trusted_operator_acceptance_record_intake_family_accepted_count: ($family_records | map(select(.intake_validator_family_accepted == true)) | length),
          operator_canary_trusted_operator_acceptance_record_intake_family_authorizes_dispatch_count: ($family_records | map(select(.intake_validator_family_authorizes_dispatch == true)) | length),
          operator_canary_trusted_operator_acceptance_record_intake_family_authorizes_live_execution_count: ($family_records | map(select(.intake_validator_family_authorizes_live_execution == true)) | length),
          operator_record_supplied: false,
          operator_record_accepted: false,
          operator_record_recorded: false,
          operator_record_persisted: false,
          operator_record_delivered: false,
          operator_record_authorizes_dispatch: false,
          operator_record_authorizes_context_attachment: false,
          operator_record_authorizes_provider_model_invocation: false,
          operator_record_authorizes_memory_write: false,
          operator_record_authorizes_external_kg_read: false,
          operator_record_authorizes_live_kg_write: false,
          operator_record_authorizes_live_execution: false,
          canary_harness_shape_ready: true,
          canary_harness_activation_ready: false,
          canary_harness_armed: false,
          canary_harness_executable: false,
          canary_live_enabled: false,
          canary_execution_performed: false,
          controlled_request_dispatched: false,
          controlled_request_executed: false,
          context_injection_performed: false,
          provider_invoked: false,
          model_invoked: false,
          usage_recorded: false,
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
          filesystem_written: false,
          release_artifact_written: false,
          public_release_claimed: false,
          public_ga_claimed: false,
          install_performed: false,
          service_restarted: false,
          active_binary_mutated: false,
          upstream_fetch_performed: false,
          upstream_merge_performed: false,
          denied_by_trusted_operator_acceptance_record_intake_validator: [
            "operator_acceptance_record_not_supplied",
            "operator_identity_missing",
            "operator_signature_hash_missing",
            "operator_timestamp_missing",
            "operator_scope_missing",
            "operator_readback_audit_receipts_missing",
            "operator_idempotency_nonce_missing",
            "operator_rollback_kill_switch_missing",
            "operator_dispatch_budget_missing",
            "operator_live_execution_bounds_missing",
            "controlled_request_dispatch_denied",
            "context_provider_model_memory_kg_live_execution_denied",
            "credential_secret_read_denied",
            "install_restart_active_binary_mutation_denied"
          ],
          next_required_step: "supply_real_trusted_operator_acceptance_record_with_identity_signature_timestamp_scope_readback_audit_idempotency_rollback_budget_and_kill_switch_before_canary_dispatch_or_live_execution",
          side_effects: {
            workspace_written: false,
            filesystem_written: false,
            operator_record_recorded: false,
            operator_record_persisted: false,
            operator_record_delivered: false,
            operator_record_accepted: false,
            trusted_operator_acceptance_record_recorded: false,
            trusted_operator_acceptance_record_persisted: false,
            trusted_operator_acceptance_record_delivered: false,
            trusted_operator_acceptance_record_accepted: false,
            controlled_request_dispatched: false,
            controlled_request_executed: false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_intake_validator_gate"
  and .operator_canary_trusted_operator_acceptance_record_intake_validator_ready == true
  and .operator_canary_trusted_operator_acceptance_record_intake_validator_status == "blocked"
  and .source_trusted_operator_acceptance_record_scaffold_ready == true
  and .source_trusted_operator_acceptance_record_scaffold_status == "blocked"
  and .source_trusted_operator_acceptance_record_count == 5
  and .source_trusted_operator_acceptance_record_accepted_count == 0
  and .source_trusted_operator_acceptance_record_recorded_count == 0
  and .source_trusted_operator_acceptance_record_persisted_count == 0
  and .source_trusted_operator_acceptance_record_missing_value_count == 80
  and .operator_canary_trusted_operator_acceptance_record_intake_record_count == 5
  and .operator_canary_trusted_operator_acceptance_record_intake_record_hash_bound_count == 5
  and .operator_canary_trusted_operator_acceptance_record_intake_required_field_count == 80
  and .operator_canary_trusted_operator_acceptance_record_intake_present_field_count == 0
  and .operator_canary_trusted_operator_acceptance_record_intake_missing_field_count == 80
  and .operator_canary_trusted_operator_acceptance_record_intake_trusted_field_count == 0
  and .operator_canary_trusted_operator_acceptance_record_intake_accepted_field_count == 0
  and .operator_canary_trusted_operator_acceptance_record_intake_identity_validated_count == 0
  and .operator_canary_trusted_operator_acceptance_record_intake_signature_validated_count == 0
  and .operator_canary_trusted_operator_acceptance_record_intake_timestamp_validated_count == 0
  and .operator_canary_trusted_operator_acceptance_record_intake_scope_validated_count == 0
  and .operator_canary_trusted_operator_acceptance_record_intake_rollback_kill_switch_validated_count == 0
  and .operator_canary_trusted_operator_acceptance_record_intake_record_accepted_count == 0
  and .operator_canary_trusted_operator_acceptance_record_intake_record_recorded_count == 0
  and .operator_canary_trusted_operator_acceptance_record_intake_record_persisted_count == 0
  and .operator_canary_trusted_operator_acceptance_record_intake_record_delivered_count == 0
  and .operator_canary_trusted_operator_acceptance_record_intake_authorizes_dispatch_count == 0
  and .operator_canary_trusted_operator_acceptance_record_intake_authorizes_context_attachment_count == 0
  and .operator_canary_trusted_operator_acceptance_record_intake_authorizes_provider_model_invocation_count == 0
  and .operator_canary_trusted_operator_acceptance_record_intake_authorizes_memory_write_count == 0
  and .operator_canary_trusted_operator_acceptance_record_intake_authorizes_external_kg_read_count == 0
  and .operator_canary_trusted_operator_acceptance_record_intake_authorizes_live_kg_write_count == 0
  and .operator_canary_trusted_operator_acceptance_record_intake_authorizes_live_execution_count == 0
  and (.operator_canary_trusted_operator_acceptance_record_intake_records | all(
    .source_record_scaffold_hash_bound == true
    and .required_field_count == 16
    and .operator_record_supplied == false
    and .operator_record_present_field_count == 0
    and .operator_record_missing_field_count == 16
    and .operator_record_trusted_field_count == 0
    and .operator_record_accepted_field_count == 0
    and .trusted_operator_acceptance_record_accepted == false
    and .trusted_operator_acceptance_record_recorded == false
    and .trusted_operator_acceptance_record_persisted == false
    and .trusted_operator_acceptance_record_delivered == false
    and .authorizes_canary_dispatch == false
    and .authorizes_context_attachment == false
    and .authorizes_provider_model_invocation == false
    and .authorizes_memory_write == false
    and .authorizes_external_kg_read == false
    and .authorizes_live_kg_write == false
    and .authorizes_live_execution == false
    and .status == "blocked_operator_record_not_supplied"
  ))
  and .operator_canary_trusted_operator_acceptance_record_intake_family_count == 9
  and .operator_canary_trusted_operator_acceptance_record_intake_family_accepted_count == 0
  and .operator_canary_trusted_operator_acceptance_record_intake_family_authorizes_dispatch_count == 0
  and .operator_canary_trusted_operator_acceptance_record_intake_family_authorizes_live_execution_count == 0
  and .operator_record_supplied == false
  and .operator_record_accepted == false
  and .operator_record_recorded == false
  and .operator_record_persisted == false
  and .operator_record_authorizes_dispatch == false
  and .operator_record_authorizes_context_attachment == false
  and .operator_record_authorizes_provider_model_invocation == false
  and .operator_record_authorizes_memory_write == false
  and .operator_record_authorizes_external_kg_read == false
  and .operator_record_authorizes_live_kg_write == false
  and .operator_record_authorizes_live_execution == false
  and .canary_harness_shape_ready == true
  and .canary_harness_activation_ready == false
  and .canary_harness_armed == false
  and .canary_harness_executable == false
  and .canary_live_enabled == false
  and .canary_execution_performed == false
  and .controlled_request_dispatched == false
  and .controlled_request_executed == false
  and .context_injection_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .external_kg_adapter_read_performed == false
  and .live_kg_write_performed == false
  and .credential_read == false
  and .auth_secret_read == false
  and .secret_file_read == false
  and .channel_send_performed == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG trusted operator acceptance record intake validator gate passed"
