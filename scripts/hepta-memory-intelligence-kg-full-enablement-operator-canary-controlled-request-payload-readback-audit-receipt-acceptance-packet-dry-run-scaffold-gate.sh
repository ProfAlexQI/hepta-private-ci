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

READBACK_PREVIEW_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-preview-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-preview-gate.sh
)"

readback_preview_report_sha256="$(sha256_text "$READBACK_PREVIEW_JSON")"
acceptance_packet_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-controlled-request-payload-readback-audit-receipt-acceptance-packet-dry-run-scaffold:v1:source-readback-preview:report-only:no-accept:no-record:no-persist:no-dispatch:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_canary_payload_readback_acceptance_packet_dry_run_side_effects=false;acceptance=false;record=false;persistence=false;dispatch=false;execute=false;context=false;provider=false;model=false;memory=false;kg=false;secret=false;restart=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$READBACK_PREVIEW_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_preview_gate"
    and $source.operator_canary_controlled_request_payload_readback_audit_receipt_preview_ready == true
    and $source.operator_canary_controlled_request_payload_readback_audit_receipt_preview_status == "blocked"
    and $source.source_payload_preview_ready == true
    and $source.source_payload_preview_status == "blocked"
    and $source.operator_canary_payload_readback_audit_receipt_preview_count == 5
    and $source.operator_canary_payload_preview_report_hash_bound_count == 5
    and $source.operator_canary_payload_readback_shape_declared_count == 5
    and $source.operator_canary_payload_readback_preview_report_materialized_count == 5
    and $source.operator_canary_payload_readback_hash_shape_declared_count == 5
    and $source.operator_canary_payload_readback_hash_matches_source_report_count == 5
    and $source.operator_canary_payload_readback_hash_accepted_count == 0
    and $source.operator_canary_payload_readback_proof_shape_declared_count == 5
    and $source.operator_canary_payload_readback_proof_accepted_count == 0
    and $source.operator_canary_audit_entry_preview_shape_declared_count == 5
    and $source.operator_canary_audit_entry_preview_report_materialized_count == 5
    and $source.operator_canary_audit_entry_recorded_count == 0
    and $source.operator_canary_audit_entry_persisted_count == 0
    and $source.operator_canary_readback_receipt_preview_shape_declared_count == 5
    and $source.operator_canary_readback_receipt_preview_report_materialized_count == 5
    and $source.operator_canary_readback_receipt_recorded_count == 0
    and $source.operator_canary_readback_receipt_persisted_count == 0
    and $source.operator_canary_readback_receipt_accepted_count == 0
    and $source.operator_canary_controlled_request_dispatch_allowed_count == 0
    and $source.operator_canary_controlled_request_dispatched_count == 0
    and $source.operator_canary_controlled_request_executed_count == 0
    and $source.operator_canary_context_attachment_allowed_count == 0
    and $source.operator_canary_provider_model_invocation_allowed_count == 0
    and $source.operator_canary_memory_write_allowed_count == 0
    and $source.operator_canary_external_kg_read_allowed_count == 0
    and $source.operator_canary_live_kg_write_allowed_count == 0
    and $source.operator_canary_payload_readback_audit_receipt_preview_accepted == false
    and $source.operator_canary_payload_readback_audit_receipt_preview_authorizes_dispatch == false
    and $source.operator_canary_payload_readback_audit_receipt_preview_authorizes_live_execution == false
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
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_acceptance_packet_dry_run_scaffold_gate" \
    --arg readback_preview_report_sha256 "$readback_preview_report_sha256" \
    --arg acceptance_packet_policy_hash_sha256 "$acceptance_packet_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --arg packet_schema_id "hepta.memory_intelligence_kg.canary.controlled_request.payload_readback_acceptance_packet.v1" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$READBACK_PREVIEW_JSON" \
    '
      [
        $source.operator_canary_controlled_request_payload_readback_audit_receipt_previews[]
        | {
            acceptance_packet_order: .readback_audit_preview_order,
            acceptance_packet_id: ("hepta-canary-controlled-request-payload-readback-acceptance-packet-" + .stage_id),
            acceptance_packet_schema_id: $packet_schema_id,
            stage_id: .stage_id,
            source_phase_id: .source_phase_id,
            source_payload_preview_id: .source_payload_preview_id,
            source_dispatch_envelope_id: .source_dispatch_envelope_id,
            source_readback_audit_preview_id: .readback_audit_preview_id,
            route_id: .route_id,
            namespace_id: .namespace_id,
            source_readback_preview_report_hash_bound: .payload_preview_report_hash_bound,
            source_readback_preview_report_sha256: $readback_preview_report_sha256,
            source_payload_readback_shape_declared: .payload_readback_shape_declared,
            source_payload_readback_preview_materialized: .payload_readback_preview_materialized_in_report,
            source_payload_readback_hash_shape_declared: .payload_readback_hash_shape_declared,
            source_payload_readback_hash_matches_source_report: .payload_readback_hash_matches_source_report,
            source_payload_readback_hash_accepted: .payload_readback_hash_accepted,
            source_payload_readback_proof_shape_declared: .payload_readback_proof_shape_declared,
            source_payload_readback_proof_accepted: .payload_readback_proof_accepted,
            source_audit_entry_preview_shape_declared: .audit_entry_preview_shape_declared,
            source_audit_entry_recorded: .audit_entry_recorded,
            source_audit_entry_persisted: .audit_entry_persisted,
            source_readback_receipt_preview_shape_declared: .readback_receipt_preview_shape_declared,
            source_readback_receipt_recorded: .readback_receipt_recorded,
            source_readback_receipt_persisted: .readback_receipt_persisted,
            source_readback_receipt_accepted: .readback_receipt_accepted,
            acceptance_packet_shape_declared: true,
            acceptance_packet_materialized_in_report: true,
            acceptance_packet_source_report_hash_bound: true,
            acceptance_packet_hash_shape_declared: true,
            acceptance_packet_hash_accepted: false,
            acceptance_packet_accepted: false,
            acceptance_packet_recorded: false,
            acceptance_packet_persisted: false,
            acceptance_packet_delivered: false,
            acceptance_packet_authorizes_dispatch: false,
            acceptance_packet_authorizes_context_attachment: false,
            acceptance_packet_authorizes_provider_model_invocation: false,
            acceptance_packet_authorizes_memory_write: false,
            acceptance_packet_authorizes_external_kg_read: false,
            acceptance_packet_authorizes_live_kg_write: false,
            acceptance_packet_authorizes_live_execution: false,
            required_authority_items: [
              { id: "operator_approval_record", shape_declared: true, satisfied: false, missing: true },
              { id: "operator_identity_signature_timestamp", shape_declared: true, satisfied: false, missing: true },
              { id: "source_preview_report_hash_acceptance", shape_declared: true, satisfied: false, missing: true },
              { id: "payload_preview_hash_acceptance", shape_declared: true, satisfied: false, missing: true },
              { id: "payload_readback_hash_acceptance", shape_declared: true, satisfied: false, missing: true },
              { id: "payload_readback_proof_acceptance", shape_declared: true, satisfied: false, missing: true },
              { id: "audit_entry_record_persistence", shape_declared: true, satisfied: false, missing: true },
              { id: "readback_receipt_record_persistence_acceptance", shape_declared: true, satisfied: false, missing: true },
              { id: "redaction_proof_acceptance", shape_declared: true, satisfied: false, missing: true },
              { id: "route_scope_acceptance", shape_declared: true, satisfied: false, missing: true },
              { id: "namespace_scope_acceptance", shape_declared: true, satisfied: false, missing: true },
              { id: "idempotency_nonce_acceptance", shape_declared: true, satisfied: false, missing: true },
              { id: "rollback_kill_switch_armed", shape_declared: true, satisfied: false, missing: true },
              { id: "dispatch_budget_acceptance", shape_declared: true, satisfied: false, missing: true },
              { id: "no_write_sink_contract_acceptance", shape_declared: true, satisfied: false, missing: true },
              { id: "single_controlled_request_window_acceptance", shape_declared: true, satisfied: false, missing: true }
            ],
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
            status: "blocked_acceptance_packet_dry_run_scaffold_only"
          }
        | . + {
            required_authority_item_count: (.required_authority_items | length),
            satisfied_authority_item_count: (.required_authority_items | map(select(.satisfied == true)) | length),
            missing_authority_item_count: (.required_authority_items | map(select(.missing == true)) | length)
          }
      ] as $acceptance_packets
      | {
          product: $product,
          runtime: $runtime,
          status: "ready",
          base_url: $base_url,
          gate: $gate,
          operator_canary_payload_readback_acceptance_packet_dry_run_scaffold_schema_version: "memory_intelligence_kg_operator_canary_payload_readback_acceptance_packet_dry_run_scaffold_v1",
          operator_canary_payload_readback_acceptance_packet_dry_run_scaffold_ready: true,
          operator_canary_payload_readback_acceptance_packet_dry_run_scaffold_status: "blocked",
          operator_canary_payload_readback_acceptance_packet_dry_run_scaffold_mode: "report_only_acceptance_packet_shape_no_accept_no_record_no_persist_no_dispatch_no_execute_no_live",
          operator_canary_payload_readback_acceptance_packet_dry_run_scaffold_decision: "acceptance_packet_shapes_are_declared_for_operator_review_but_all_authority_items_remain_missing_and_no_dispatch_or_live_execution_is_authorized",
          minimum_required_samples: $min_long_soak_samples,
          source_readback_preview_gate: $source.gate,
          source_readback_preview_report_sha256: $readback_preview_report_sha256,
          source_readback_preview_ready: $source.operator_canary_controlled_request_payload_readback_audit_receipt_preview_ready,
          source_readback_preview_status: $source.operator_canary_controlled_request_payload_readback_audit_receipt_preview_status,
          source_readback_preview_count: $source.operator_canary_payload_readback_audit_receipt_preview_count,
          source_readback_preview_hash_bound_count: $source.operator_canary_payload_preview_report_hash_bound_count,
          source_payload_readback_shape_declared_count: $source.operator_canary_payload_readback_shape_declared_count,
          source_payload_readback_hash_matches_source_report_count: $source.operator_canary_payload_readback_hash_matches_source_report_count,
          source_payload_readback_hash_accepted_count: $source.operator_canary_payload_readback_hash_accepted_count,
          source_payload_readback_proof_accepted_count: $source.operator_canary_payload_readback_proof_accepted_count,
          source_audit_entry_recorded_count: $source.operator_canary_audit_entry_recorded_count,
          source_audit_entry_persisted_count: $source.operator_canary_audit_entry_persisted_count,
          source_readback_receipt_recorded_count: $source.operator_canary_readback_receipt_recorded_count,
          source_readback_receipt_persisted_count: $source.operator_canary_readback_receipt_persisted_count,
          source_readback_receipt_accepted_count: $source.operator_canary_readback_receipt_accepted_count,
          acceptance_packet_policy_hash_sha256: $acceptance_packet_policy_hash_sha256,
          side_effect_hash_sha256: $side_effect_hash_sha256,
          operator_canary_readback_acceptance_packets: $acceptance_packets,
          operator_canary_readback_acceptance_packet_count: ($acceptance_packets | length),
          operator_canary_readback_acceptance_packet_shape_declared_count: ($acceptance_packets | map(select(.acceptance_packet_shape_declared == true)) | length),
          operator_canary_readback_acceptance_packet_materialized_count: ($acceptance_packets | map(select(.acceptance_packet_materialized_in_report == true)) | length),
          operator_canary_readback_acceptance_packet_source_hash_bound_count: ($acceptance_packets | map(select(.acceptance_packet_source_report_hash_bound == true)) | length),
          operator_canary_readback_acceptance_packet_hash_shape_declared_count: ($acceptance_packets | map(select(.acceptance_packet_hash_shape_declared == true)) | length),
          operator_canary_readback_acceptance_packet_hash_accepted_count: ($acceptance_packets | map(select(.acceptance_packet_hash_accepted == true)) | length),
          operator_canary_readback_acceptance_packet_accepted_count: ($acceptance_packets | map(select(.acceptance_packet_accepted == true)) | length),
          operator_canary_readback_acceptance_packet_recorded_count: ($acceptance_packets | map(select(.acceptance_packet_recorded == true)) | length),
          operator_canary_readback_acceptance_packet_persisted_count: ($acceptance_packets | map(select(.acceptance_packet_persisted == true)) | length),
          operator_canary_readback_acceptance_packet_delivered_count: ($acceptance_packets | map(select(.acceptance_packet_delivered == true)) | length),
          operator_canary_readback_acceptance_packet_required_item_count: ($acceptance_packets | map(.required_authority_item_count) | add),
          operator_canary_readback_acceptance_packet_satisfied_item_count: ($acceptance_packets | map(.satisfied_authority_item_count) | add),
          operator_canary_readback_acceptance_packet_missing_item_count: ($acceptance_packets | map(.missing_authority_item_count) | add),
          operator_canary_readback_acceptance_packet_authorizes_dispatch_count: ($acceptance_packets | map(select(.acceptance_packet_authorizes_dispatch == true)) | length),
          operator_canary_readback_acceptance_packet_authorizes_context_attachment_count: ($acceptance_packets | map(select(.acceptance_packet_authorizes_context_attachment == true)) | length),
          operator_canary_readback_acceptance_packet_authorizes_provider_model_invocation_count: ($acceptance_packets | map(select(.acceptance_packet_authorizes_provider_model_invocation == true)) | length),
          operator_canary_readback_acceptance_packet_authorizes_memory_write_count: ($acceptance_packets | map(select(.acceptance_packet_authorizes_memory_write == true)) | length),
          operator_canary_readback_acceptance_packet_authorizes_external_kg_read_count: ($acceptance_packets | map(select(.acceptance_packet_authorizes_external_kg_read == true)) | length),
          operator_canary_readback_acceptance_packet_authorizes_live_kg_write_count: ($acceptance_packets | map(select(.acceptance_packet_authorizes_live_kg_write == true)) | length),
          operator_canary_readback_acceptance_packet_authorizes_live_execution_count: ($acceptance_packets | map(select(.acceptance_packet_authorizes_live_execution == true)) | length),
          operator_canary_controlled_request_dispatch_allowed_count: ($acceptance_packets | map(select(.controlled_request_dispatch_allowed == true)) | length),
          operator_canary_controlled_request_dispatched_count: ($acceptance_packets | map(.controlled_request_dispatched_count) | add),
          operator_canary_controlled_request_execution_allowed_count: ($acceptance_packets | map(select(.controlled_request_execution_allowed == true)) | length),
          operator_canary_controlled_request_executed_count: ($acceptance_packets | map(.controlled_request_executed_count) | add),
          operator_canary_context_attachment_allowed_count: ($acceptance_packets | map(select(.context_attachment_allowed == true)) | length),
          operator_canary_provider_model_invocation_allowed_count: ($acceptance_packets | map(select(.provider_model_invocation_allowed == true)) | length),
          operator_canary_memory_write_allowed_count: ($acceptance_packets | map(select(.memory_write_allowed == true)) | length),
          operator_canary_external_kg_read_allowed_count: ($acceptance_packets | map(select(.external_kg_read_allowed == true)) | length),
          operator_canary_live_kg_write_allowed_count: ($acceptance_packets | map(select(.live_kg_write_allowed == true)) | length),
          operator_canary_readback_acceptance_packet_accepted: false,
          operator_canary_readback_acceptance_packet_authorizes_dispatch: false,
          operator_canary_readback_acceptance_packet_authorizes_live_execution: false,
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
          denied_by_operator_canary_readback_acceptance_packet_dry_run_scaffold: [
            "operator_approval_record_missing",
            "operator_identity_signature_timestamp_missing",
            "source_preview_report_hash_not_accepted",
            "payload_preview_hash_not_accepted",
            "payload_readback_hash_not_accepted",
            "payload_readback_proof_not_accepted",
            "audit_entry_not_recorded_or_persisted",
            "readback_receipt_not_recorded_persisted_or_accepted",
            "redaction_proof_not_accepted",
            "route_namespace_scope_not_accepted",
            "idempotency_nonce_not_accepted",
            "rollback_kill_switch_not_armed",
            "dispatch_budget_not_accepted",
            "controlled_request_dispatch_denied",
            "controlled_request_execution_denied",
            "context_provider_model_memory_kg_live_execution_denied",
            "credential_secret_read_denied",
            "install_restart_active_binary_mutation_denied"
          ],
          side_effects: {
            workspace_written: false,
            filesystem_written: false,
            acceptance_packet_recorded: false,
            acceptance_packet_persisted: false,
            acceptance_packet_accepted: false,
            acceptance_packet_delivered: false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_acceptance_packet_dry_run_scaffold_gate"
  and .operator_canary_payload_readback_acceptance_packet_dry_run_scaffold_ready == true
  and .operator_canary_payload_readback_acceptance_packet_dry_run_scaffold_status == "blocked"
  and .source_readback_preview_gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_preview_gate"
  and .source_readback_preview_ready == true
  and .source_readback_preview_status == "blocked"
  and .source_readback_preview_count == 5
  and .source_readback_preview_hash_bound_count == 5
  and .source_payload_readback_shape_declared_count == 5
  and .source_payload_readback_hash_matches_source_report_count == 5
  and .source_payload_readback_hash_accepted_count == 0
  and .source_payload_readback_proof_accepted_count == 0
  and .source_audit_entry_recorded_count == 0
  and .source_audit_entry_persisted_count == 0
  and .source_readback_receipt_recorded_count == 0
  and .source_readback_receipt_persisted_count == 0
  and .source_readback_receipt_accepted_count == 0
  and .operator_canary_readback_acceptance_packet_count == 5
  and .operator_canary_readback_acceptance_packet_shape_declared_count == 5
  and .operator_canary_readback_acceptance_packet_materialized_count == 5
  and .operator_canary_readback_acceptance_packet_source_hash_bound_count == 5
  and .operator_canary_readback_acceptance_packet_hash_shape_declared_count == 5
  and .operator_canary_readback_acceptance_packet_hash_accepted_count == 0
  and .operator_canary_readback_acceptance_packet_accepted_count == 0
  and .operator_canary_readback_acceptance_packet_recorded_count == 0
  and .operator_canary_readback_acceptance_packet_persisted_count == 0
  and .operator_canary_readback_acceptance_packet_delivered_count == 0
  and .operator_canary_readback_acceptance_packet_required_item_count == 80
  and .operator_canary_readback_acceptance_packet_satisfied_item_count == 0
  and .operator_canary_readback_acceptance_packet_missing_item_count == 80
  and .operator_canary_readback_acceptance_packet_authorizes_dispatch_count == 0
  and .operator_canary_readback_acceptance_packet_authorizes_context_attachment_count == 0
  and .operator_canary_readback_acceptance_packet_authorizes_provider_model_invocation_count == 0
  and .operator_canary_readback_acceptance_packet_authorizes_memory_write_count == 0
  and .operator_canary_readback_acceptance_packet_authorizes_external_kg_read_count == 0
  and .operator_canary_readback_acceptance_packet_authorizes_live_kg_write_count == 0
  and .operator_canary_readback_acceptance_packet_authorizes_live_execution_count == 0
  and .operator_canary_controlled_request_dispatch_allowed_count == 0
  and .operator_canary_controlled_request_dispatched_count == 0
  and .operator_canary_controlled_request_execution_allowed_count == 0
  and .operator_canary_controlled_request_executed_count == 0
  and .operator_canary_context_attachment_allowed_count == 0
  and .operator_canary_provider_model_invocation_allowed_count == 0
  and .operator_canary_memory_write_allowed_count == 0
  and .operator_canary_external_kg_read_allowed_count == 0
  and .operator_canary_live_kg_write_allowed_count == 0
  and (.operator_canary_readback_acceptance_packets | all(
    .acceptance_packet_shape_declared == true
    and .acceptance_packet_materialized_in_report == true
    and .acceptance_packet_source_report_hash_bound == true
    and .acceptance_packet_hash_shape_declared == true
    and .acceptance_packet_hash_accepted == false
    and .acceptance_packet_accepted == false
    and .acceptance_packet_recorded == false
    and .acceptance_packet_persisted == false
    and .acceptance_packet_delivered == false
    and .acceptance_packet_authorizes_dispatch == false
    and .acceptance_packet_authorizes_context_attachment == false
    and .acceptance_packet_authorizes_provider_model_invocation == false
    and .acceptance_packet_authorizes_memory_write == false
    and .acceptance_packet_authorizes_external_kg_read == false
    and .acceptance_packet_authorizes_live_kg_write == false
    and .acceptance_packet_authorizes_live_execution == false
    and .required_authority_item_count == 16
    and .satisfied_authority_item_count == 0
    and .missing_authority_item_count == 16
    and (.required_authority_items | all(.shape_declared == true and .satisfied == false and .missing == true))
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
    and .status == "blocked_acceptance_packet_dry_run_scaffold_only"
  ))
  and .operator_canary_readback_acceptance_packet_accepted == false
  and .operator_canary_readback_acceptance_packet_authorizes_dispatch == false
  and .operator_canary_readback_acceptance_packet_authorizes_live_execution == false
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
echo "Hepta Memory/Intelligence/KG controlled request payload readback audit receipt acceptance packet dry-run scaffold gate passed"
