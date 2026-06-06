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

VALUE_SCOREBOARD_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-acceptance-packet-value-scoreboard-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-acceptance-packet-value-scoreboard-gate.sh
)"

value_scoreboard_report_sha256="$(sha256_text "$VALUE_SCOREBOARD_JSON")"
trusted_record_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-trusted-operator-acceptance-record-scaffold:v1:source-value-scoreboard:report-only:no-trust:no-record:no-persist:no-dispatch:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_canary_trusted_operator_acceptance_record_scaffold_side_effects=false;trusted=false;recorded=false;persisted=false;accepted=false;dispatch=false;execute=false;context=false;provider=false;model=false;memory=false;kg=false;secret=false;restart=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$VALUE_SCOREBOARD_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_acceptance_packet_value_scoreboard_gate"
    and $source.operator_canary_readback_acceptance_packet_value_scoreboard_ready == true
    and $source.operator_canary_readback_acceptance_packet_value_scoreboard_status == "blocked"
    and $source.source_acceptance_packet_count == 5
    and $source.source_required_authority_item_count == 80
    and $source.source_satisfied_authority_item_count == 0
    and $source.source_missing_authority_item_count == 80
    and $source.operator_canary_readback_acceptance_packet_value_scoreboard_item_count == 80
    and $source.operator_canary_readback_acceptance_packet_value_scoreboard_item_shape_declared_count == 80
    and $source.operator_canary_readback_acceptance_packet_value_scoreboard_item_materialized_count == 80
    and $source.operator_canary_readback_acceptance_packet_value_scoreboard_item_hash_bound_count == 80
    and $source.operator_canary_readback_acceptance_packet_value_scoreboard_trusted_value_count == 0
    and $source.operator_canary_readback_acceptance_packet_value_scoreboard_recorded_value_count == 0
    and $source.operator_canary_readback_acceptance_packet_value_scoreboard_persisted_value_count == 0
    and $source.operator_canary_readback_acceptance_packet_value_scoreboard_accepted_value_count == 0
    and $source.operator_canary_readback_acceptance_packet_value_scoreboard_satisfied_authority_count == 0
    and $source.operator_canary_readback_acceptance_packet_value_scoreboard_missing_authority_count == 80
    and $source.operator_canary_readback_acceptance_packet_value_scoreboard_blocking_item_count == 80
    and $source.operator_canary_readback_acceptance_packet_value_score_count == 5
    and $source.operator_canary_readback_acceptance_packet_value_score_shape_declared_count == 5
    and $source.operator_canary_readback_acceptance_packet_value_score_complete_count == 0
    and $source.operator_canary_readback_acceptance_packet_value_acceptance_ready_count == 0
    and $source.operator_canary_readback_acceptance_packet_value_accepted_count == 0
    and $source.operator_canary_readback_acceptance_packet_value_authorizes_dispatch_count == 0
    and $source.operator_canary_readback_acceptance_packet_value_authorizes_live_execution_count == 0
    and $source.operator_canary_readback_acceptance_packet_value_family_count == 9
    and $source.operator_canary_readback_acceptance_packet_value_family_shape_declared_count == 9
    and $source.operator_canary_readback_acceptance_packet_value_family_trusted_count == 0
    and $source.operator_canary_readback_acceptance_packet_value_family_accepted_count == 0
    and $source.operator_canary_readback_acceptance_packet_value_family_satisfied_count == 0
    and $source.operator_canary_readback_acceptance_packet_value_family_missing_count == 9
    and ($source.operator_canary_readback_acceptance_packet_value_scores | all(
      .source_required_authority_item_count == 16
      and .source_satisfied_authority_item_count == 0
      and .source_missing_authority_item_count == 16
      and .value_scoreboard_item_count == 16
      and .value_scoreboard_trusted_value_count == 0
      and .value_scoreboard_accepted_value_count == 0
      and .value_scoreboard_missing_authority_count == 16
      and .acceptance_packet_value_score == 0
      and .acceptance_packet_value_score_complete == false
      and .acceptance_packet_value_acceptance_ready == false
      and .acceptance_packet_value_accepted == false
      and .acceptance_packet_value_authorizes_dispatch == false
      and .acceptance_packet_value_authorizes_live_execution == false
    ))
    and $source.operator_canary_readback_acceptance_packet_value_scoreboard_accepted == false
    and $source.operator_canary_readback_acceptance_packet_value_scoreboard_recorded == false
    and $source.operator_canary_readback_acceptance_packet_value_scoreboard_persisted == false
    and $source.operator_canary_readback_acceptance_packet_value_scoreboard_delivered == false
    and $source.operator_canary_readback_acceptance_packet_value_scoreboard_authorizes_dispatch == false
    and $source.operator_canary_readback_acceptance_packet_value_scoreboard_authorizes_context_attachment == false
    and $source.operator_canary_readback_acceptance_packet_value_scoreboard_authorizes_provider_model_invocation == false
    and $source.operator_canary_readback_acceptance_packet_value_scoreboard_authorizes_memory_write == false
    and $source.operator_canary_readback_acceptance_packet_value_scoreboard_authorizes_external_kg_read == false
    and $source.operator_canary_readback_acceptance_packet_value_scoreboard_authorizes_live_kg_write == false
    and $source.operator_canary_readback_acceptance_packet_value_scoreboard_authorizes_live_execution == false
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
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_scaffold_gate" \
    --arg value_scoreboard_report_sha256 "$value_scoreboard_report_sha256" \
    --arg trusted_record_policy_hash_sha256 "$trusted_record_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --arg record_schema_id "hepta.memory_intelligence_kg.canary.trusted_operator_acceptance_record.v1" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$VALUE_SCOREBOARD_JSON" \
    '
      [
        $source.operator_canary_readback_acceptance_packet_value_scores[]
        | {
            acceptance_record_order: .acceptance_packet_order,
            acceptance_record_id: ("trusted-operator-acceptance-record-" + .acceptance_packet_id),
            acceptance_record_schema_id: $record_schema_id,
            acceptance_packet_id: .acceptance_packet_id,
            stage_id: .stage_id,
            source_phase_id: .source_phase_id,
            route_id: .route_id,
            namespace_id: .namespace_id,
            source_value_scoreboard_report_hash_bound: true,
            source_value_scoreboard_report_sha256: $value_scoreboard_report_sha256,
            value_scoreboard_item_count: .value_scoreboard_item_count,
            value_scoreboard_item_shape_declared_count: .value_scoreboard_item_shape_declared_count,
            value_scoreboard_missing_authority_count: .value_scoreboard_missing_authority_count,
            value_scoreboard_trusted_value_count: .value_scoreboard_trusted_value_count,
            value_scoreboard_accepted_value_count: .value_scoreboard_accepted_value_count,
            trusted_operator_acceptance_record_shape_declared: true,
            trusted_operator_acceptance_record_materialized_in_report: true,
            trusted_operator_acceptance_record_hash_bound_to_value_scoreboard: true,
            trusted_operator_identity_accepted: false,
            trusted_operator_signature_hash_accepted: false,
            trusted_operator_timestamp_accepted: false,
            trusted_operator_scope_accepted: false,
            trusted_operator_redaction_scope_accepted: false,
            trusted_operator_readback_receipt_accepted: false,
            trusted_operator_audit_receipt_accepted: false,
            trusted_operator_idempotency_nonce_accepted: false,
            trusted_operator_rollback_kill_switch_accepted: false,
            trusted_operator_dispatch_budget_accepted: false,
            trusted_operator_no_write_boundary_accepted: false,
            trusted_operator_acceptance_record_recorded: false,
            trusted_operator_acceptance_record_persisted: false,
            trusted_operator_acceptance_record_delivered: false,
            trusted_operator_acceptance_record_accepted: false,
            trusted_operator_acceptance_record_authorizes_dispatch: false,
            trusted_operator_acceptance_record_authorizes_context_attachment: false,
            trusted_operator_acceptance_record_authorizes_provider_model_invocation: false,
            trusted_operator_acceptance_record_authorizes_memory_write: false,
            trusted_operator_acceptance_record_authorizes_external_kg_read: false,
            trusted_operator_acceptance_record_authorizes_live_kg_write: false,
            trusted_operator_acceptance_record_authorizes_live_execution: false,
            status: "blocked_trusted_operator_acceptance_record_missing_authority"
          }
      ] as $acceptance_records
      | [
          $source.operator_canary_readback_acceptance_packet_value_family_scores[]
          | {
              authority_family: .authority_family,
              source_family_item_count: .family_item_count,
              source_family_missing_authority_count: .family_missing_authority_count,
              trusted_operator_acceptance_record_family_shape_declared: true,
              trusted_operator_acceptance_record_family_accepted: false,
              trusted_operator_acceptance_record_family_authorizes_dispatch: false,
              trusted_operator_acceptance_record_family_authorizes_live_execution: false,
              status: "blocked_family_missing_trusted_operator_acceptance_record"
            }
        ] as $family_records
      | {
          product: $product,
          runtime: $runtime,
          status: "ready",
          base_url: $base_url,
          gate: $gate,
          operator_canary_trusted_operator_acceptance_record_scaffold_schema_version: "memory_intelligence_kg_operator_canary_trusted_operator_acceptance_record_scaffold_v1",
          operator_canary_trusted_operator_acceptance_record_scaffold_ready: true,
          operator_canary_trusted_operator_acceptance_record_scaffold_status: "blocked",
          operator_canary_trusted_operator_acceptance_record_scaffold_mode: "report_only_record_shape_no_trust_no_record_no_persist_no_accept_no_dispatch_no_execute_no_live",
          operator_canary_trusted_operator_acceptance_record_scaffold_decision: "trusted_operator_acceptance_records_are_shaped_and_hash_bound_to_the_value_scoreboard_but_remain_untrusted_unrecorded_unaccepted_and_non_authorizing",
          minimum_required_samples: $min_long_soak_samples,
          source_value_scoreboard_gate: $source.gate,
          source_value_scoreboard_report_sha256: $value_scoreboard_report_sha256,
          source_value_scoreboard_ready: $source.operator_canary_readback_acceptance_packet_value_scoreboard_ready,
          source_value_scoreboard_status: $source.operator_canary_readback_acceptance_packet_value_scoreboard_status,
          source_acceptance_packet_count: $source.source_acceptance_packet_count,
          source_value_scoreboard_item_count: $source.operator_canary_readback_acceptance_packet_value_scoreboard_item_count,
          source_value_scoreboard_trusted_value_count: $source.operator_canary_readback_acceptance_packet_value_scoreboard_trusted_value_count,
          source_value_scoreboard_accepted_value_count: $source.operator_canary_readback_acceptance_packet_value_scoreboard_accepted_value_count,
          source_value_scoreboard_missing_authority_count: $source.operator_canary_readback_acceptance_packet_value_scoreboard_missing_authority_count,
          source_value_scoreboard_blocking_item_count: $source.operator_canary_readback_acceptance_packet_value_scoreboard_blocking_item_count,
          source_value_packet_score_count: $source.operator_canary_readback_acceptance_packet_value_score_count,
          source_value_packet_acceptance_ready_count: $source.operator_canary_readback_acceptance_packet_value_acceptance_ready_count,
          source_value_packet_accepted_count: $source.operator_canary_readback_acceptance_packet_value_accepted_count,
          source_value_packet_authorizes_dispatch_count: $source.operator_canary_readback_acceptance_packet_value_authorizes_dispatch_count,
          source_value_packet_authorizes_live_execution_count: $source.operator_canary_readback_acceptance_packet_value_authorizes_live_execution_count,
          trusted_record_policy_hash_sha256: $trusted_record_policy_hash_sha256,
          side_effect_hash_sha256: $side_effect_hash_sha256,
          operator_canary_trusted_operator_acceptance_records: $acceptance_records,
          operator_canary_trusted_operator_acceptance_record_count: ($acceptance_records | length),
          operator_canary_trusted_operator_acceptance_record_shape_declared_count: ($acceptance_records | map(select(.trusted_operator_acceptance_record_shape_declared == true)) | length),
          operator_canary_trusted_operator_acceptance_record_materialized_count: ($acceptance_records | map(select(.trusted_operator_acceptance_record_materialized_in_report == true)) | length),
          operator_canary_trusted_operator_acceptance_record_hash_bound_count: ($acceptance_records | map(select(.trusted_operator_acceptance_record_hash_bound_to_value_scoreboard == true)) | length),
          operator_canary_trusted_operator_identity_accepted_count: ($acceptance_records | map(select(.trusted_operator_identity_accepted == true)) | length),
          operator_canary_trusted_operator_signature_hash_accepted_count: ($acceptance_records | map(select(.trusted_operator_signature_hash_accepted == true)) | length),
          operator_canary_trusted_operator_timestamp_accepted_count: ($acceptance_records | map(select(.trusted_operator_timestamp_accepted == true)) | length),
          operator_canary_trusted_operator_acceptance_record_recorded_count: ($acceptance_records | map(select(.trusted_operator_acceptance_record_recorded == true)) | length),
          operator_canary_trusted_operator_acceptance_record_persisted_count: ($acceptance_records | map(select(.trusted_operator_acceptance_record_persisted == true)) | length),
          operator_canary_trusted_operator_acceptance_record_delivered_count: ($acceptance_records | map(select(.trusted_operator_acceptance_record_delivered == true)) | length),
          operator_canary_trusted_operator_acceptance_record_accepted_count: ($acceptance_records | map(select(.trusted_operator_acceptance_record_accepted == true)) | length),
          operator_canary_trusted_operator_acceptance_record_authorizes_dispatch_count: ($acceptance_records | map(select(.trusted_operator_acceptance_record_authorizes_dispatch == true)) | length),
          operator_canary_trusted_operator_acceptance_record_authorizes_context_attachment_count: ($acceptance_records | map(select(.trusted_operator_acceptance_record_authorizes_context_attachment == true)) | length),
          operator_canary_trusted_operator_acceptance_record_authorizes_provider_model_invocation_count: ($acceptance_records | map(select(.trusted_operator_acceptance_record_authorizes_provider_model_invocation == true)) | length),
          operator_canary_trusted_operator_acceptance_record_authorizes_memory_write_count: ($acceptance_records | map(select(.trusted_operator_acceptance_record_authorizes_memory_write == true)) | length),
          operator_canary_trusted_operator_acceptance_record_authorizes_external_kg_read_count: ($acceptance_records | map(select(.trusted_operator_acceptance_record_authorizes_external_kg_read == true)) | length),
          operator_canary_trusted_operator_acceptance_record_authorizes_live_kg_write_count: ($acceptance_records | map(select(.trusted_operator_acceptance_record_authorizes_live_kg_write == true)) | length),
          operator_canary_trusted_operator_acceptance_record_authorizes_live_execution_count: ($acceptance_records | map(select(.trusted_operator_acceptance_record_authorizes_live_execution == true)) | length),
          operator_canary_trusted_operator_acceptance_record_required_value_count: ($acceptance_records | map(.value_scoreboard_item_count) | add),
          operator_canary_trusted_operator_acceptance_record_trusted_value_count: ($acceptance_records | map(.value_scoreboard_trusted_value_count) | add),
          operator_canary_trusted_operator_acceptance_record_missing_value_count: ($acceptance_records | map(.value_scoreboard_missing_authority_count) | add),
          operator_canary_trusted_operator_acceptance_record_family_records: $family_records,
          operator_canary_trusted_operator_acceptance_record_family_count: ($family_records | length),
          operator_canary_trusted_operator_acceptance_record_family_shape_declared_count: ($family_records | map(select(.trusted_operator_acceptance_record_family_shape_declared == true)) | length),
          operator_canary_trusted_operator_acceptance_record_family_accepted_count: ($family_records | map(select(.trusted_operator_acceptance_record_family_accepted == true)) | length),
          operator_canary_trusted_operator_acceptance_record_family_authorizes_dispatch_count: ($family_records | map(select(.trusted_operator_acceptance_record_family_authorizes_dispatch == true)) | length),
          operator_canary_trusted_operator_acceptance_record_family_authorizes_live_execution_count: ($family_records | map(select(.trusted_operator_acceptance_record_family_authorizes_live_execution == true)) | length),
          operator_canary_trusted_operator_acceptance_record_accepted: false,
          operator_canary_trusted_operator_acceptance_record_recorded: false,
          operator_canary_trusted_operator_acceptance_record_persisted: false,
          operator_canary_trusted_operator_acceptance_record_delivered: false,
          operator_canary_trusted_operator_acceptance_record_authorizes_dispatch: false,
          operator_canary_trusted_operator_acceptance_record_authorizes_context_attachment: false,
          operator_canary_trusted_operator_acceptance_record_authorizes_provider_model_invocation: false,
          operator_canary_trusted_operator_acceptance_record_authorizes_memory_write: false,
          operator_canary_trusted_operator_acceptance_record_authorizes_external_kg_read: false,
          operator_canary_trusted_operator_acceptance_record_authorizes_live_kg_write: false,
          operator_canary_trusted_operator_acceptance_record_authorizes_live_execution: false,
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
          denied_by_trusted_operator_acceptance_record_scaffold: [
            "trusted_operator_identity_signature_timestamp_missing",
            "trusted_operator_acceptance_record_unrecorded",
            "trusted_operator_acceptance_record_unpersisted",
            "trusted_operator_acceptance_record_unaccepted",
            "source_value_scoreboard_has_zero_trusted_values",
            "source_value_scoreboard_has_eighty_missing_authority_values",
            "all_acceptance_record_authority_families_block_dispatch",
            "controlled_request_dispatch_denied",
            "context_provider_model_memory_kg_live_execution_denied",
            "credential_secret_read_denied",
            "install_restart_active_binary_mutation_denied"
          ],
          next_required_step: "record_real_trusted_operator_acceptance_record_with_identity_signature_timestamp_scope_and_kill_switch_before_canary_dispatch_or_live_execution",
          side_effects: {
            workspace_written: false,
            filesystem_written: false,
            trusted_operator_acceptance_record_recorded: false,
            trusted_operator_acceptance_record_persisted: false,
            trusted_operator_acceptance_record_delivered: false,
            trusted_operator_acceptance_record_accepted: false,
            value_scoreboard_recorded: false,
            value_scoreboard_persisted: false,
            value_scoreboard_accepted: false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_trusted_operator_acceptance_record_scaffold_gate"
  and .operator_canary_trusted_operator_acceptance_record_scaffold_ready == true
  and .operator_canary_trusted_operator_acceptance_record_scaffold_status == "blocked"
  and .source_value_scoreboard_gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_acceptance_packet_value_scoreboard_gate"
  and .source_value_scoreboard_ready == true
  and .source_value_scoreboard_status == "blocked"
  and .source_acceptance_packet_count == 5
  and .source_value_scoreboard_item_count == 80
  and .source_value_scoreboard_trusted_value_count == 0
  and .source_value_scoreboard_accepted_value_count == 0
  and .source_value_scoreboard_missing_authority_count == 80
  and .source_value_scoreboard_blocking_item_count == 80
  and .source_value_packet_score_count == 5
  and .source_value_packet_acceptance_ready_count == 0
  and .source_value_packet_accepted_count == 0
  and .source_value_packet_authorizes_dispatch_count == 0
  and .source_value_packet_authorizes_live_execution_count == 0
  and .operator_canary_trusted_operator_acceptance_record_count == 5
  and .operator_canary_trusted_operator_acceptance_record_shape_declared_count == 5
  and .operator_canary_trusted_operator_acceptance_record_materialized_count == 5
  and .operator_canary_trusted_operator_acceptance_record_hash_bound_count == 5
  and .operator_canary_trusted_operator_identity_accepted_count == 0
  and .operator_canary_trusted_operator_signature_hash_accepted_count == 0
  and .operator_canary_trusted_operator_timestamp_accepted_count == 0
  and .operator_canary_trusted_operator_acceptance_record_recorded_count == 0
  and .operator_canary_trusted_operator_acceptance_record_persisted_count == 0
  and .operator_canary_trusted_operator_acceptance_record_delivered_count == 0
  and .operator_canary_trusted_operator_acceptance_record_accepted_count == 0
  and .operator_canary_trusted_operator_acceptance_record_authorizes_dispatch_count == 0
  and .operator_canary_trusted_operator_acceptance_record_authorizes_context_attachment_count == 0
  and .operator_canary_trusted_operator_acceptance_record_authorizes_provider_model_invocation_count == 0
  and .operator_canary_trusted_operator_acceptance_record_authorizes_memory_write_count == 0
  and .operator_canary_trusted_operator_acceptance_record_authorizes_external_kg_read_count == 0
  and .operator_canary_trusted_operator_acceptance_record_authorizes_live_kg_write_count == 0
  and .operator_canary_trusted_operator_acceptance_record_authorizes_live_execution_count == 0
  and .operator_canary_trusted_operator_acceptance_record_required_value_count == 80
  and .operator_canary_trusted_operator_acceptance_record_trusted_value_count == 0
  and .operator_canary_trusted_operator_acceptance_record_missing_value_count == 80
  and (.operator_canary_trusted_operator_acceptance_records | all(
    .source_value_scoreboard_report_hash_bound == true
    and .value_scoreboard_item_count == 16
    and .value_scoreboard_item_shape_declared_count == 16
    and .value_scoreboard_missing_authority_count == 16
    and .value_scoreboard_trusted_value_count == 0
    and .value_scoreboard_accepted_value_count == 0
    and .trusted_operator_acceptance_record_shape_declared == true
    and .trusted_operator_acceptance_record_materialized_in_report == true
    and .trusted_operator_acceptance_record_hash_bound_to_value_scoreboard == true
    and .trusted_operator_identity_accepted == false
    and .trusted_operator_signature_hash_accepted == false
    and .trusted_operator_timestamp_accepted == false
    and .trusted_operator_scope_accepted == false
    and .trusted_operator_redaction_scope_accepted == false
    and .trusted_operator_readback_receipt_accepted == false
    and .trusted_operator_audit_receipt_accepted == false
    and .trusted_operator_idempotency_nonce_accepted == false
    and .trusted_operator_rollback_kill_switch_accepted == false
    and .trusted_operator_dispatch_budget_accepted == false
    and .trusted_operator_no_write_boundary_accepted == false
    and .trusted_operator_acceptance_record_recorded == false
    and .trusted_operator_acceptance_record_persisted == false
    and .trusted_operator_acceptance_record_delivered == false
    and .trusted_operator_acceptance_record_accepted == false
    and .trusted_operator_acceptance_record_authorizes_dispatch == false
    and .trusted_operator_acceptance_record_authorizes_context_attachment == false
    and .trusted_operator_acceptance_record_authorizes_provider_model_invocation == false
    and .trusted_operator_acceptance_record_authorizes_memory_write == false
    and .trusted_operator_acceptance_record_authorizes_external_kg_read == false
    and .trusted_operator_acceptance_record_authorizes_live_kg_write == false
    and .trusted_operator_acceptance_record_authorizes_live_execution == false
    and .status == "blocked_trusted_operator_acceptance_record_missing_authority"
  ))
  and .operator_canary_trusted_operator_acceptance_record_family_count == 9
  and .operator_canary_trusted_operator_acceptance_record_family_shape_declared_count == 9
  and .operator_canary_trusted_operator_acceptance_record_family_accepted_count == 0
  and .operator_canary_trusted_operator_acceptance_record_family_authorizes_dispatch_count == 0
  and .operator_canary_trusted_operator_acceptance_record_family_authorizes_live_execution_count == 0
  and (.operator_canary_trusted_operator_acceptance_record_family_records | all(
    .source_family_item_count > 0
    and .source_family_missing_authority_count == .source_family_item_count
    and .trusted_operator_acceptance_record_family_shape_declared == true
    and .trusted_operator_acceptance_record_family_accepted == false
    and .trusted_operator_acceptance_record_family_authorizes_dispatch == false
    and .trusted_operator_acceptance_record_family_authorizes_live_execution == false
    and .status == "blocked_family_missing_trusted_operator_acceptance_record"
  ))
  and .operator_canary_trusted_operator_acceptance_record_accepted == false
  and .operator_canary_trusted_operator_acceptance_record_recorded == false
  and .operator_canary_trusted_operator_acceptance_record_persisted == false
  and .operator_canary_trusted_operator_acceptance_record_delivered == false
  and .operator_canary_trusted_operator_acceptance_record_authorizes_dispatch == false
  and .operator_canary_trusted_operator_acceptance_record_authorizes_context_attachment == false
  and .operator_canary_trusted_operator_acceptance_record_authorizes_provider_model_invocation == false
  and .operator_canary_trusted_operator_acceptance_record_authorizes_memory_write == false
  and .operator_canary_trusted_operator_acceptance_record_authorizes_external_kg_read == false
  and .operator_canary_trusted_operator_acceptance_record_authorizes_live_kg_write == false
  and .operator_canary_trusted_operator_acceptance_record_authorizes_live_execution == false
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
echo "Hepta Memory/Intelligence/KG controlled request payload readback audit receipt trusted operator acceptance record scaffold gate passed"
