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

ACCEPTANCE_PACKET_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-acceptance-packet-dry-run-scaffold-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-acceptance-packet-dry-run-scaffold-gate.sh
)"

acceptance_packet_report_sha256="$(sha256_text "$ACCEPTANCE_PACKET_JSON")"
value_scoreboard_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-controlled-request-payload-readback-acceptance-packet-value-scoreboard:v1:source-acceptance-packet:report-only:no-trust:no-accept:no-record:no-dispatch:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_canary_readback_acceptance_packet_value_scoreboard_side_effects=false;trusted=false;accepted=false;recorded=false;persisted=false;dispatch=false;execute=false;context=false;provider=false;model=false;memory=false;kg=false;secret=false;restart=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$ACCEPTANCE_PACKET_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_acceptance_packet_dry_run_scaffold_gate"
    and $source.operator_canary_payload_readback_acceptance_packet_dry_run_scaffold_ready == true
    and $source.operator_canary_payload_readback_acceptance_packet_dry_run_scaffold_status == "blocked"
    and $source.source_readback_preview_ready == true
    and $source.source_readback_preview_status == "blocked"
    and $source.source_readback_preview_count == 5
    and $source.operator_canary_readback_acceptance_packet_count == 5
    and $source.operator_canary_readback_acceptance_packet_shape_declared_count == 5
    and $source.operator_canary_readback_acceptance_packet_materialized_count == 5
    and $source.operator_canary_readback_acceptance_packet_source_hash_bound_count == 5
    and $source.operator_canary_readback_acceptance_packet_hash_shape_declared_count == 5
    and $source.operator_canary_readback_acceptance_packet_hash_accepted_count == 0
    and $source.operator_canary_readback_acceptance_packet_accepted_count == 0
    and $source.operator_canary_readback_acceptance_packet_recorded_count == 0
    and $source.operator_canary_readback_acceptance_packet_persisted_count == 0
    and $source.operator_canary_readback_acceptance_packet_delivered_count == 0
    and $source.operator_canary_readback_acceptance_packet_required_item_count == 80
    and $source.operator_canary_readback_acceptance_packet_satisfied_item_count == 0
    and $source.operator_canary_readback_acceptance_packet_missing_item_count == 80
    and $source.operator_canary_readback_acceptance_packet_authorizes_dispatch_count == 0
    and $source.operator_canary_readback_acceptance_packet_authorizes_context_attachment_count == 0
    and $source.operator_canary_readback_acceptance_packet_authorizes_provider_model_invocation_count == 0
    and $source.operator_canary_readback_acceptance_packet_authorizes_memory_write_count == 0
    and $source.operator_canary_readback_acceptance_packet_authorizes_external_kg_read_count == 0
    and $source.operator_canary_readback_acceptance_packet_authorizes_live_kg_write_count == 0
    and $source.operator_canary_readback_acceptance_packet_authorizes_live_execution_count == 0
    and $source.operator_canary_controlled_request_dispatch_allowed_count == 0
    and $source.operator_canary_controlled_request_dispatched_count == 0
    and $source.operator_canary_controlled_request_execution_allowed_count == 0
    and $source.operator_canary_controlled_request_executed_count == 0
    and $source.operator_canary_context_attachment_allowed_count == 0
    and $source.operator_canary_provider_model_invocation_allowed_count == 0
    and $source.operator_canary_memory_write_allowed_count == 0
    and $source.operator_canary_external_kg_read_allowed_count == 0
    and $source.operator_canary_live_kg_write_allowed_count == 0
    and ($source.operator_canary_readback_acceptance_packets | all(
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
    ))
    and $source.operator_canary_readback_acceptance_packet_accepted == false
    and $source.operator_canary_readback_acceptance_packet_authorizes_dispatch == false
    and $source.operator_canary_readback_acceptance_packet_authorizes_live_execution == false
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
    and $source.telegram_send_performed == false
    and $source.external_send_performed == false
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
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_acceptance_packet_value_scoreboard_gate" \
    --arg acceptance_packet_report_sha256 "$acceptance_packet_report_sha256" \
    --arg value_scoreboard_policy_hash_sha256 "$value_scoreboard_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --arg scoreboard_id "hepta-memory-intelligence-kg-operator-canary-readback-acceptance-packet-value-scoreboard-report-only-v1" \
    --arg scoreboard_schema_id "hepta.memory_intelligence_kg.canary.readback_acceptance_packet_value_scoreboard.v1" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$ACCEPTANCE_PACKET_JSON" \
    '
      def authority_family($id):
        if ($id | startswith("operator_")) then "operator-approval"
        elif ($id == "source_preview_report_hash_acceptance" or $id == "payload_preview_hash_acceptance") then "source-binding"
        elif ($id == "payload_readback_hash_acceptance" or $id == "payload_readback_proof_acceptance") then "readback-proof"
        elif ($id == "audit_entry_record_persistence" or $id == "readback_receipt_record_persistence_acceptance") then "audit-receipt"
        elif ($id == "redaction_proof_acceptance" or $id == "route_scope_acceptance" or $id == "namespace_scope_acceptance") then "privacy-scope"
        elif ($id == "idempotency_nonce_acceptance") then "idempotency"
        elif ($id == "rollback_kill_switch_armed") then "rollback"
        elif ($id == "dispatch_budget_acceptance" or $id == "single_controlled_request_window_acceptance") then "dispatch-budget"
        elif ($id == "no_write_sink_contract_acceptance") then "no-write-live-boundary"
        else "unknown"
        end;

      [
        $source.operator_canary_readback_acceptance_packets[]
        | . as $packet
        | $packet.required_authority_items
        | to_entries[]
        | . as $entry
        | $entry.value as $item
        | {
            scoreboard_order: (($packet.acceptance_packet_order * 100) + ($entry.key + 1)),
            acceptance_packet_order: $packet.acceptance_packet_order,
            acceptance_packet_id: $packet.acceptance_packet_id,
            stage_id: $packet.stage_id,
            source_phase_id: $packet.source_phase_id,
            route_id: $packet.route_id,
            namespace_id: $packet.namespace_id,
            authority_item_order: ($entry.key + 1),
            authority_item_id: $item.id,
            authority_family: authority_family($item.id),
            source_authority_item_shape_declared: $item.shape_declared,
            source_authority_item_satisfied: $item.satisfied,
            source_authority_item_missing: $item.missing,
            acceptance_packet_shape_declared: $packet.acceptance_packet_shape_declared,
            acceptance_packet_materialized_in_report: $packet.acceptance_packet_materialized_in_report,
            acceptance_packet_source_report_hash_bound: $packet.acceptance_packet_source_report_hash_bound,
            value_scoreboard_item_shape_declared: true,
            value_scoreboard_item_materialized_in_report: true,
            value_scoreboard_item_hash_bound_to_source_packet: true,
            value_scoreboard_item_has_trusted_operator_value: false,
            value_scoreboard_item_value_recorded: false,
            value_scoreboard_item_value_persisted: false,
            value_scoreboard_item_value_accepted: false,
            value_scoreboard_item_authority_satisfied: false,
            value_scoreboard_item_missing_authority: true,
            value_scoreboard_item_blocks_acceptance: true,
            value_scoreboard_item_live_enabling: false,
            status: "blocked_missing_trusted_authority_value"
          }
      ] as $scoreboard_items
      | [
          $source.operator_canary_readback_acceptance_packets[]
          | . as $packet
          | {
              acceptance_packet_order: $packet.acceptance_packet_order,
              acceptance_packet_id: $packet.acceptance_packet_id,
              stage_id: $packet.stage_id,
              source_phase_id: $packet.source_phase_id,
              route_id: $packet.route_id,
              namespace_id: $packet.namespace_id,
              source_required_authority_item_count: $packet.required_authority_item_count,
              source_satisfied_authority_item_count: $packet.satisfied_authority_item_count,
              source_missing_authority_item_count: $packet.missing_authority_item_count,
              value_scoreboard_item_count: ($scoreboard_items | map(select(.acceptance_packet_id == $packet.acceptance_packet_id)) | length),
              value_scoreboard_item_shape_declared_count: ($scoreboard_items | map(select(.acceptance_packet_id == $packet.acceptance_packet_id and .value_scoreboard_item_shape_declared == true)) | length),
              value_scoreboard_trusted_value_count: ($scoreboard_items | map(select(.acceptance_packet_id == $packet.acceptance_packet_id and .value_scoreboard_item_has_trusted_operator_value == true)) | length),
              value_scoreboard_accepted_value_count: ($scoreboard_items | map(select(.acceptance_packet_id == $packet.acceptance_packet_id and .value_scoreboard_item_value_accepted == true)) | length),
              value_scoreboard_satisfied_authority_count: ($scoreboard_items | map(select(.acceptance_packet_id == $packet.acceptance_packet_id and .value_scoreboard_item_authority_satisfied == true)) | length),
              value_scoreboard_missing_authority_count: ($scoreboard_items | map(select(.acceptance_packet_id == $packet.acceptance_packet_id and .value_scoreboard_item_missing_authority == true)) | length),
              value_scoreboard_blocking_item_count: ($scoreboard_items | map(select(.acceptance_packet_id == $packet.acceptance_packet_id and .value_scoreboard_item_blocks_acceptance == true)) | length),
              acceptance_packet_value_score: 0,
              acceptance_packet_value_score_max: $packet.required_authority_item_count,
              acceptance_packet_value_score_complete: false,
              acceptance_packet_value_acceptance_ready: false,
              acceptance_packet_value_accepted: false,
              acceptance_packet_value_authorizes_dispatch: false,
              acceptance_packet_value_authorizes_live_execution: false,
              status: "blocked_acceptance_packet_values_untrusted"
            }
        ] as $packet_scores
      | [
          "operator-approval",
          "source-binding",
          "readback-proof",
          "audit-receipt",
          "privacy-scope",
          "idempotency",
          "rollback",
          "dispatch-budget",
          "no-write-live-boundary"
        ] as $families
      | [
          $families[]
          | . as $family
          | {
              authority_family: $family,
              family_item_count: ($scoreboard_items | map(select(.authority_family == $family)) | length),
              family_shape_declared_count: ($scoreboard_items | map(select(.authority_family == $family and .value_scoreboard_item_shape_declared == true)) | length),
              family_trusted_value_count: ($scoreboard_items | map(select(.authority_family == $family and .value_scoreboard_item_has_trusted_operator_value == true)) | length),
              family_accepted_value_count: ($scoreboard_items | map(select(.authority_family == $family and .value_scoreboard_item_value_accepted == true)) | length),
              family_satisfied_authority_count: ($scoreboard_items | map(select(.authority_family == $family and .value_scoreboard_item_authority_satisfied == true)) | length),
              family_missing_authority_count: ($scoreboard_items | map(select(.authority_family == $family and .value_scoreboard_item_missing_authority == true)) | length),
              family_blocks_acceptance: true,
              family_live_enabling: false,
              status: "blocked_family_missing_trusted_values"
            }
        ] as $family_scores
      | {
          product: $product,
          runtime: $runtime,
          status: "ready",
          base_url: $base_url,
          gate: $gate,
          operator_canary_readback_acceptance_packet_value_scoreboard_schema_version: "memory_intelligence_kg_operator_canary_readback_acceptance_packet_value_scoreboard_v1",
          operator_canary_readback_acceptance_packet_value_scoreboard_ready: true,
          operator_canary_readback_acceptance_packet_value_scoreboard_status: "blocked",
          operator_canary_readback_acceptance_packet_value_scoreboard_mode: "report_only_value_scoreboard_no_trust_no_accept_no_record_no_dispatch_no_execute_no_live",
          operator_canary_readback_acceptance_packet_value_scoreboard_decision: "acceptance_packet_authority_values_are_scored_by_packet_and_family_but_all_items_remain_untrusted_missing_and_non_authorizing",
          operator_canary_readback_acceptance_packet_value_scoreboard_id: $scoreboard_id,
          operator_canary_readback_acceptance_packet_value_scoreboard_schema_id: $scoreboard_schema_id,
          minimum_required_samples: $min_long_soak_samples,
          source_acceptance_packet_gate: $source.gate,
          source_acceptance_packet_report_sha256: $acceptance_packet_report_sha256,
          source_acceptance_packet_ready: $source.operator_canary_payload_readback_acceptance_packet_dry_run_scaffold_ready,
          source_acceptance_packet_status: $source.operator_canary_payload_readback_acceptance_packet_dry_run_scaffold_status,
          source_acceptance_packet_count: $source.operator_canary_readback_acceptance_packet_count,
          source_acceptance_packet_shape_declared_count: $source.operator_canary_readback_acceptance_packet_shape_declared_count,
          source_acceptance_packet_materialized_count: $source.operator_canary_readback_acceptance_packet_materialized_count,
          source_acceptance_packet_hash_bound_count: $source.operator_canary_readback_acceptance_packet_source_hash_bound_count,
          source_acceptance_packet_hash_accepted_count: $source.operator_canary_readback_acceptance_packet_hash_accepted_count,
          source_acceptance_packet_accepted_count: $source.operator_canary_readback_acceptance_packet_accepted_count,
          source_acceptance_packet_recorded_count: $source.operator_canary_readback_acceptance_packet_recorded_count,
          source_acceptance_packet_persisted_count: $source.operator_canary_readback_acceptance_packet_persisted_count,
          source_acceptance_packet_delivered_count: $source.operator_canary_readback_acceptance_packet_delivered_count,
          source_required_authority_item_count: $source.operator_canary_readback_acceptance_packet_required_item_count,
          source_satisfied_authority_item_count: $source.operator_canary_readback_acceptance_packet_satisfied_item_count,
          source_missing_authority_item_count: $source.operator_canary_readback_acceptance_packet_missing_item_count,
          value_scoreboard_policy_hash_sha256: $value_scoreboard_policy_hash_sha256,
          side_effect_hash_sha256: $side_effect_hash_sha256,
          operator_canary_readback_acceptance_packet_value_scoreboard_items: $scoreboard_items,
          operator_canary_readback_acceptance_packet_value_scoreboard_item_count: ($scoreboard_items | length),
          operator_canary_readback_acceptance_packet_value_scoreboard_item_shape_declared_count: ($scoreboard_items | map(select(.value_scoreboard_item_shape_declared == true)) | length),
          operator_canary_readback_acceptance_packet_value_scoreboard_item_materialized_count: ($scoreboard_items | map(select(.value_scoreboard_item_materialized_in_report == true)) | length),
          operator_canary_readback_acceptance_packet_value_scoreboard_item_hash_bound_count: ($scoreboard_items | map(select(.value_scoreboard_item_hash_bound_to_source_packet == true)) | length),
          operator_canary_readback_acceptance_packet_value_scoreboard_trusted_value_count: ($scoreboard_items | map(select(.value_scoreboard_item_has_trusted_operator_value == true)) | length),
          operator_canary_readback_acceptance_packet_value_scoreboard_recorded_value_count: ($scoreboard_items | map(select(.value_scoreboard_item_value_recorded == true)) | length),
          operator_canary_readback_acceptance_packet_value_scoreboard_persisted_value_count: ($scoreboard_items | map(select(.value_scoreboard_item_value_persisted == true)) | length),
          operator_canary_readback_acceptance_packet_value_scoreboard_accepted_value_count: ($scoreboard_items | map(select(.value_scoreboard_item_value_accepted == true)) | length),
          operator_canary_readback_acceptance_packet_value_scoreboard_satisfied_authority_count: ($scoreboard_items | map(select(.value_scoreboard_item_authority_satisfied == true)) | length),
          operator_canary_readback_acceptance_packet_value_scoreboard_missing_authority_count: ($scoreboard_items | map(select(.value_scoreboard_item_missing_authority == true)) | length),
          operator_canary_readback_acceptance_packet_value_scoreboard_blocking_item_count: ($scoreboard_items | map(select(.value_scoreboard_item_blocks_acceptance == true)) | length),
          operator_canary_readback_acceptance_packet_value_scores: $packet_scores,
          operator_canary_readback_acceptance_packet_value_score_count: ($packet_scores | length),
          operator_canary_readback_acceptance_packet_value_score_shape_declared_count: ($packet_scores | map(select(.value_scoreboard_item_shape_declared_count == .source_required_authority_item_count)) | length),
          operator_canary_readback_acceptance_packet_value_score_complete_count: ($packet_scores | map(select(.acceptance_packet_value_score_complete == true)) | length),
          operator_canary_readback_acceptance_packet_value_acceptance_ready_count: ($packet_scores | map(select(.acceptance_packet_value_acceptance_ready == true)) | length),
          operator_canary_readback_acceptance_packet_value_accepted_count: ($packet_scores | map(select(.acceptance_packet_value_accepted == true)) | length),
          operator_canary_readback_acceptance_packet_value_authorizes_dispatch_count: ($packet_scores | map(select(.acceptance_packet_value_authorizes_dispatch == true)) | length),
          operator_canary_readback_acceptance_packet_value_authorizes_live_execution_count: ($packet_scores | map(select(.acceptance_packet_value_authorizes_live_execution == true)) | length),
          operator_canary_readback_acceptance_packet_value_family_scores: $family_scores,
          operator_canary_readback_acceptance_packet_value_family_count: ($family_scores | length),
          operator_canary_readback_acceptance_packet_value_family_shape_declared_count: ($family_scores | map(select(.family_shape_declared_count == .family_item_count)) | length),
          operator_canary_readback_acceptance_packet_value_family_trusted_count: ($family_scores | map(select(.family_trusted_value_count > 0)) | length),
          operator_canary_readback_acceptance_packet_value_family_accepted_count: ($family_scores | map(select(.family_accepted_value_count == .family_item_count and .family_item_count > 0)) | length),
          operator_canary_readback_acceptance_packet_value_family_satisfied_count: ($family_scores | map(select(.family_satisfied_authority_count == .family_item_count and .family_item_count > 0)) | length),
          operator_canary_readback_acceptance_packet_value_family_missing_count: ($family_scores | map(select(.family_missing_authority_count > 0)) | length),
          operator_canary_readback_acceptance_packet_value_scoreboard_accepted: false,
          operator_canary_readback_acceptance_packet_value_scoreboard_recorded: false,
          operator_canary_readback_acceptance_packet_value_scoreboard_persisted: false,
          operator_canary_readback_acceptance_packet_value_scoreboard_delivered: false,
          operator_canary_readback_acceptance_packet_value_scoreboard_authorizes_dispatch: false,
          operator_canary_readback_acceptance_packet_value_scoreboard_authorizes_context_attachment: false,
          operator_canary_readback_acceptance_packet_value_scoreboard_authorizes_provider_model_invocation: false,
          operator_canary_readback_acceptance_packet_value_scoreboard_authorizes_memory_write: false,
          operator_canary_readback_acceptance_packet_value_scoreboard_authorizes_external_kg_read: false,
          operator_canary_readback_acceptance_packet_value_scoreboard_authorizes_live_kg_write: false,
          operator_canary_readback_acceptance_packet_value_scoreboard_authorizes_live_execution: false,
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
          denied_by_operator_canary_readback_acceptance_packet_value_scoreboard: [
            "all_authority_values_untrusted",
            "all_authority_items_missing",
            "no_value_score_reaches_acceptance_threshold",
            "operator_approval_identity_signature_timestamp_not_accepted",
            "source_payload_readback_hashes_not_accepted",
            "readback_proof_not_accepted",
            "audit_receipt_not_recorded_persisted_or_accepted",
            "redaction_scope_idempotency_rollback_dispatch_budget_not_accepted",
            "no_write_live_boundary_not_accepted",
            "controlled_request_dispatch_denied",
            "context_provider_model_memory_kg_live_execution_denied",
            "credential_secret_read_denied",
            "install_restart_active_binary_mutation_denied"
          ],
          next_required_step: "replace_report_only_value_scoreboard_with_trusted_operator_acceptance_record_before_dispatch_or_live_canary",
          side_effects: {
            workspace_written: false,
            filesystem_written: false,
            value_scoreboard_recorded: false,
            value_scoreboard_persisted: false,
            value_scoreboard_delivered: false,
            value_scoreboard_accepted: false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_acceptance_packet_value_scoreboard_gate"
  and .operator_canary_readback_acceptance_packet_value_scoreboard_ready == true
  and .operator_canary_readback_acceptance_packet_value_scoreboard_status == "blocked"
  and .source_acceptance_packet_gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_payload_readback_audit_receipt_acceptance_packet_dry_run_scaffold_gate"
  and .source_acceptance_packet_ready == true
  and .source_acceptance_packet_status == "blocked"
  and .source_acceptance_packet_count == 5
  and .source_acceptance_packet_shape_declared_count == 5
  and .source_acceptance_packet_materialized_count == 5
  and .source_acceptance_packet_hash_bound_count == 5
  and .source_acceptance_packet_hash_accepted_count == 0
  and .source_acceptance_packet_accepted_count == 0
  and .source_acceptance_packet_recorded_count == 0
  and .source_acceptance_packet_persisted_count == 0
  and .source_acceptance_packet_delivered_count == 0
  and .source_required_authority_item_count == 80
  and .source_satisfied_authority_item_count == 0
  and .source_missing_authority_item_count == 80
  and .operator_canary_readback_acceptance_packet_value_scoreboard_item_count == 80
  and .operator_canary_readback_acceptance_packet_value_scoreboard_item_shape_declared_count == 80
  and .operator_canary_readback_acceptance_packet_value_scoreboard_item_materialized_count == 80
  and .operator_canary_readback_acceptance_packet_value_scoreboard_item_hash_bound_count == 80
  and .operator_canary_readback_acceptance_packet_value_scoreboard_trusted_value_count == 0
  and .operator_canary_readback_acceptance_packet_value_scoreboard_recorded_value_count == 0
  and .operator_canary_readback_acceptance_packet_value_scoreboard_persisted_value_count == 0
  and .operator_canary_readback_acceptance_packet_value_scoreboard_accepted_value_count == 0
  and .operator_canary_readback_acceptance_packet_value_scoreboard_satisfied_authority_count == 0
  and .operator_canary_readback_acceptance_packet_value_scoreboard_missing_authority_count == 80
  and .operator_canary_readback_acceptance_packet_value_scoreboard_blocking_item_count == 80
  and (.operator_canary_readback_acceptance_packet_value_scoreboard_items | length) == 80
  and (.operator_canary_readback_acceptance_packet_value_scoreboard_items | all(
    .source_authority_item_shape_declared == true
    and .source_authority_item_satisfied == false
    and .source_authority_item_missing == true
    and .acceptance_packet_shape_declared == true
    and .acceptance_packet_materialized_in_report == true
    and .acceptance_packet_source_report_hash_bound == true
    and .value_scoreboard_item_shape_declared == true
    and .value_scoreboard_item_materialized_in_report == true
    and .value_scoreboard_item_hash_bound_to_source_packet == true
    and .value_scoreboard_item_has_trusted_operator_value == false
    and .value_scoreboard_item_value_recorded == false
    and .value_scoreboard_item_value_persisted == false
    and .value_scoreboard_item_value_accepted == false
    and .value_scoreboard_item_authority_satisfied == false
    and .value_scoreboard_item_missing_authority == true
    and .value_scoreboard_item_blocks_acceptance == true
    and .value_scoreboard_item_live_enabling == false
    and .status == "blocked_missing_trusted_authority_value"
  ))
  and .operator_canary_readback_acceptance_packet_value_score_count == 5
  and .operator_canary_readback_acceptance_packet_value_score_shape_declared_count == 5
  and .operator_canary_readback_acceptance_packet_value_score_complete_count == 0
  and .operator_canary_readback_acceptance_packet_value_acceptance_ready_count == 0
  and .operator_canary_readback_acceptance_packet_value_accepted_count == 0
  and .operator_canary_readback_acceptance_packet_value_authorizes_dispatch_count == 0
  and .operator_canary_readback_acceptance_packet_value_authorizes_live_execution_count == 0
  and (.operator_canary_readback_acceptance_packet_value_scores | all(
    .source_required_authority_item_count == 16
    and .source_satisfied_authority_item_count == 0
    and .source_missing_authority_item_count == 16
    and .value_scoreboard_item_count == 16
    and .value_scoreboard_item_shape_declared_count == 16
    and .value_scoreboard_trusted_value_count == 0
    and .value_scoreboard_accepted_value_count == 0
    and .value_scoreboard_satisfied_authority_count == 0
    and .value_scoreboard_missing_authority_count == 16
    and .value_scoreboard_blocking_item_count == 16
    and .acceptance_packet_value_score == 0
    and .acceptance_packet_value_score_max == 16
    and .acceptance_packet_value_score_complete == false
    and .acceptance_packet_value_acceptance_ready == false
    and .acceptance_packet_value_accepted == false
    and .acceptance_packet_value_authorizes_dispatch == false
    and .acceptance_packet_value_authorizes_live_execution == false
  ))
  and .operator_canary_readback_acceptance_packet_value_family_count == 9
  and .operator_canary_readback_acceptance_packet_value_family_shape_declared_count == 9
  and .operator_canary_readback_acceptance_packet_value_family_trusted_count == 0
  and .operator_canary_readback_acceptance_packet_value_family_accepted_count == 0
  and .operator_canary_readback_acceptance_packet_value_family_satisfied_count == 0
  and .operator_canary_readback_acceptance_packet_value_family_missing_count == 9
  and (.operator_canary_readback_acceptance_packet_value_family_scores | all(
    .family_item_count > 0
    and .family_shape_declared_count == .family_item_count
    and .family_trusted_value_count == 0
    and .family_accepted_value_count == 0
    and .family_satisfied_authority_count == 0
    and .family_missing_authority_count == .family_item_count
    and .family_blocks_acceptance == true
    and .family_live_enabling == false
    and .status == "blocked_family_missing_trusted_values"
  ))
  and .operator_canary_readback_acceptance_packet_value_scoreboard_accepted == false
  and .operator_canary_readback_acceptance_packet_value_scoreboard_recorded == false
  and .operator_canary_readback_acceptance_packet_value_scoreboard_persisted == false
  and .operator_canary_readback_acceptance_packet_value_scoreboard_delivered == false
  and .operator_canary_readback_acceptance_packet_value_scoreboard_authorizes_dispatch == false
  and .operator_canary_readback_acceptance_packet_value_scoreboard_authorizes_context_attachment == false
  and .operator_canary_readback_acceptance_packet_value_scoreboard_authorizes_provider_model_invocation == false
  and .operator_canary_readback_acceptance_packet_value_scoreboard_authorizes_memory_write == false
  and .operator_canary_readback_acceptance_packet_value_scoreboard_authorizes_external_kg_read == false
  and .operator_canary_readback_acceptance_packet_value_scoreboard_authorizes_live_kg_write == false
  and .operator_canary_readback_acceptance_packet_value_scoreboard_authorizes_live_execution == false
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
  and .filesystem_written == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG controlled request payload readback audit receipt acceptance packet value scoreboard gate passed"
