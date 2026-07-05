#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-ordering-monotonicity-denial-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-ordering-monotonicity-denial-boundary-gate.sh
)"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_ready == true
    and $source.scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_accepted == true
    and $source.accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count == 1
    and $source.blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count == 9
    and $source.denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_count == 55
    and $source.dry_run_execution_result_receipt_ordering_cursor_persisted == false
    and $source.dry_run_execution_result_receipt_monotonic_sequence_recorded == false
    and $source.dry_run_execution_executed == false
    and $source.dry_run_execution_result_receipt_persisted == false
    and $source.production_durable_memory_write_executed == false
    and $source.production_durable_memory_store_write_performed == false
    and $source.actual_production_durable_memory_write_performed == false
    and $source.durable_memory_store_write_performed == false
    and $source.durable_memory_store_read_performed == false
    and $source.durable_memory_store_rollback_performed == false
    and $source.memory_store_write_performed == false
    and $source.wal_write_performed == false
    and $source.receipt_persisted == false
    and $source.post_write_readback_performed == false
    and $source.rollback_executed == false
    and $source.tombstone_cleanup_executed == false
    and $source.live_kg_write_performed == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.credential_read == false
    and $source.channel_send_performed == false
    and $source.external_send_performed == false
    and $source.release_artifact_written == false
    and $source.install_executed == false
    and $source.service_restarted == false
    and $source.active_binary_mutated == false
    and $source.allowed_next_actions[1].action == "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary"
    and $source.allowed_next_actions[1].requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary == true
    and $source.allowed_next_actions[1].executes_dry_run == false
    and $source.allowed_next_actions[1].writes_production_durable_memory == false
  ' >/dev/null

approved_production_namespace="$(jq -r '.approved_production_namespace // ""' <<<"$SOURCE_JSON")"
approved_production_store="$(jq -r '.approved_production_store // ""' <<<"$SOURCE_JSON")"
approved_production_scope="$(jq -r '.approved_production_scope // ""' <<<"$SOURCE_JSON")"
production_durable_memory_target_id="$(jq -r '.production_durable_memory_target_id // ""' <<<"$SOURCE_JSON")"
production_durable_memory_payload_class="$(jq -r '.production_durable_memory_payload_class // ""' <<<"$SOURCE_JSON")"
operator_packet_scope="$(jq -r '.operator_packet_scope // ""' <<<"$SOURCE_JSON")"
source_report_sha256="$(sha256_text "$SOURCE_JSON")"
source_ordering_boundary_hash_sha256="$(jq -r '.scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_ordering_policy_hash_sha256="$(jq -r '.scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_policy_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_ordering_matrix_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_ordering_monotonicity_denial_matrix_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_ordering_sequence_policy_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_ordering_sequence_policy_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_late_receipt_denial_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_late_receipt_denial_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_future_receipt_denial_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_future_receipt_denial_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_rollback_sequence_denial_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_rollback_sequence_denial_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_same_sequence_replacement_denial_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_same_sequence_replacement_denial_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_latest_wins_promotion_denial_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_latest_wins_promotion_denial_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_sequence_gap_denial_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_sequence_gap_denial_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_ordering_handoff_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_ordering_handoff_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_ordering_result_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_ordering_result_hash_sha256 // ""' <<<"$SOURCE_JSON")"

cancellation_matrix_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-cancellation-supersession-denial-matrix:v1:source-ordering=${source_ordering_result_hash_sha256}:late=${source_late_receipt_denial_hash_sha256}:future=${source_future_receipt_denial_hash_sha256}:rollback=${source_rollback_sequence_denial_hash_sha256}:same=${source_same_sequence_replacement_denial_hash_sha256}:latest=${source_latest_wins_promotion_denial_hash_sha256}:gap=${source_sequence_gap_denial_hash_sha256}:cancel=deny:supersede=deny:replacement=deny:tombstone=deny:persist=false"
)"
cancellation_policy_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-cancellation-policy:v1:matrix=${cancellation_matrix_hash_sha256}:source-policy=${source_ordering_policy_hash_sha256}:record=false:persist=false:ledger=false"
)"
supersession_policy_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-supersession-policy:v1:matrix=${cancellation_matrix_hash_sha256}:source-sequence-policy=${source_ordering_sequence_policy_hash_sha256}:supersede=false:replace=false:tombstone=false"
)"
replacement_receipt_denial_hash_sha256="$(sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-replacement-denial:v1:policy=${supersession_policy_hash_sha256}:accepted=false")"
tombstone_delete_marker_denial_hash_sha256="$(sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-tombstone-delete-marker-denial:v1:policy=${supersession_policy_hash_sha256}:accepted=false")"
latest_replacement_denial_hash_sha256="$(sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-latest-replacement-denial:v1:policy=${supersession_policy_hash_sha256}:accepted=false")"
completion_ack_replacement_denial_hash_sha256="$(sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-completion-ack-replacement-denial:v1:policy=${supersession_policy_hash_sha256}:accepted=false")"
export_query_replacement_denial_hash_sha256="$(sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-export-query-replacement-denial:v1:policy=${supersession_policy_hash_sha256}:accepted=false")"
cancellation_handoff_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-cancellation-supersession-handoff:v1:replacement=${replacement_receipt_denial_hash_sha256}:tombstone=${tombstone_delete_marker_denial_hash_sha256}:latest=${latest_replacement_denial_hash_sha256}:completion=${completion_ack_replacement_denial_hash_sha256}:export=${export_query_replacement_denial_hash_sha256}:next=audit-trail-immutable-evidence-denial-boundary"
)"
cancellation_result_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-cancellation-supersession-result:v1:matrix=${cancellation_matrix_hash_sha256}:handoff=${cancellation_handoff_hash_sha256}:accepted=true:cancel=false:supersede=false:replace=false:tombstone=false:executed=false:production-write=false"
)"
cancellation_boundary_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-cancellation-supersession-denial-boundary:v1:source=${source_report_sha256}:result=${cancellation_result_hash_sha256}:fixtures=10:accepted=1:denials=65:cancel=false:supersede=false:replacement=false:tombstone=false:dry-run-executed=false:production-write=false"
)"
cancellation_boundary_policy_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-cancellation-supersession-denial-policy:v1:bind-source-ordering-matrix-cancel-supersede-replacement-tombstone-latest-completion-export-handoff:no-cancel:no-supersede:no-replacement:no-tombstone:no-execution:no-production-write:no-kg:no-provider:no-channel:no-release:no-install"
)"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_gate" \
  --arg endpoint "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-cancellation-supersession-denial-boundary" \
  --arg source_command "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-cancellation-supersession-denial-boundary --json" \
  --arg approved_production_namespace "$approved_production_namespace" \
  --arg approved_production_store "$approved_production_store" \
  --arg approved_production_scope "$approved_production_scope" \
  --arg production_durable_memory_target_id "$production_durable_memory_target_id" \
  --arg production_durable_memory_payload_class "$production_durable_memory_payload_class" \
  --arg operator_packet_scope "$operator_packet_scope" \
  --arg source_report_sha256 "$source_report_sha256" \
  --arg source_ordering_boundary_hash_sha256 "$source_ordering_boundary_hash_sha256" \
  --arg source_ordering_policy_hash_sha256 "$source_ordering_policy_hash_sha256" \
  --arg source_ordering_matrix_hash_sha256 "$source_ordering_matrix_hash_sha256" \
  --arg source_ordering_sequence_policy_hash_sha256 "$source_ordering_sequence_policy_hash_sha256" \
  --arg source_late_receipt_denial_hash_sha256 "$source_late_receipt_denial_hash_sha256" \
  --arg source_future_receipt_denial_hash_sha256 "$source_future_receipt_denial_hash_sha256" \
  --arg source_rollback_sequence_denial_hash_sha256 "$source_rollback_sequence_denial_hash_sha256" \
  --arg source_same_sequence_replacement_denial_hash_sha256 "$source_same_sequence_replacement_denial_hash_sha256" \
  --arg source_latest_wins_promotion_denial_hash_sha256 "$source_latest_wins_promotion_denial_hash_sha256" \
  --arg source_sequence_gap_denial_hash_sha256 "$source_sequence_gap_denial_hash_sha256" \
  --arg source_ordering_handoff_hash_sha256 "$source_ordering_handoff_hash_sha256" \
  --arg source_ordering_result_hash_sha256 "$source_ordering_result_hash_sha256" \
  --arg cancellation_matrix_hash_sha256 "$cancellation_matrix_hash_sha256" \
  --arg cancellation_policy_hash_sha256 "$cancellation_policy_hash_sha256" \
  --arg supersession_policy_hash_sha256 "$supersession_policy_hash_sha256" \
  --arg replacement_receipt_denial_hash_sha256 "$replacement_receipt_denial_hash_sha256" \
  --arg tombstone_delete_marker_denial_hash_sha256 "$tombstone_delete_marker_denial_hash_sha256" \
  --arg latest_replacement_denial_hash_sha256 "$latest_replacement_denial_hash_sha256" \
  --arg completion_ack_replacement_denial_hash_sha256 "$completion_ack_replacement_denial_hash_sha256" \
  --arg export_query_replacement_denial_hash_sha256 "$export_query_replacement_denial_hash_sha256" \
  --arg cancellation_handoff_hash_sha256 "$cancellation_handoff_hash_sha256" \
  --arg cancellation_result_hash_sha256 "$cancellation_result_hash_sha256" \
  --arg cancellation_boundary_hash_sha256 "$cancellation_boundary_hash_sha256" \
  --arg cancellation_boundary_policy_hash_sha256 "$cancellation_boundary_policy_hash_sha256" \
  --argjson source "$SOURCE_JSON" \
  '
  def zero_fields($keys): reduce $keys[] as $k ({}; .[$k]=false | .[$k + "_count"]=0);
  def true_count_fields($keys): reduce $keys[] as $k ({}; .[$k]=true | .[$k + "_count"]=1);
  ([
    "source_ordering_monotonicity_denial_boundary_required",
    "source_ordering_monotonicity_result_required",
    "dry_run_execution_result_receipt_cancellation_supersession_matrix_required",
    "dry_run_execution_result_receipt_cancellation_policy_required",
    "dry_run_execution_result_receipt_supersession_policy_required",
    "dry_run_execution_result_receipt_replacement_receipt_denial_required",
    "dry_run_execution_result_receipt_tombstone_delete_marker_denial_required",
    "dry_run_execution_result_receipt_latest_replacement_denial_required",
    "dry_run_execution_result_receipt_completion_ack_replacement_denial_required",
    "dry_run_execution_result_receipt_export_query_replacement_denial_required",
    "dry_run_execution_result_receipt_cancellation_supersession_handoff_required",
    "dry_run_execution_result_receipt_cancellation_state_persistence_forbidden",
    "dry_run_execution_result_receipt_supersession_state_persistence_forbidden",
    "dry_run_execution_execution_forbidden_on_cancellation_supersession_route",
    "production_write_execution_forbidden_on_cancellation_supersession_route",
    "kg_provider_channel_release_install_active_binary_forbidden"
  ]) as $surfaces
  | ([
    "source_ordering_monotonicity_denial_boundary_required",
    "source_ordering_monotonicity_result_hash_required",
    "source_ordering_policy_hash_required",
    "source_ordering_matrix_required",
    "source_sequence_policy_required",
    "source_late_receipt_denial_required",
    "source_future_receipt_denial_required",
    "source_rollback_sequence_denial_required",
    "source_same_sequence_replacement_denial_required",
    "source_latest_wins_promotion_denial_required",
    "source_sequence_gap_denial_required",
    "approved_production_namespace_required",
    "approved_production_store_required",
    "approved_production_scope_required",
    "production_durable_memory_target_required",
    "cancellation_supersession_matrix_required",
    "cancellation_policy_required",
    "supersession_policy_required",
    "cancellation_request_acceptance_denied",
    "cancellation_recording_denied",
    "cancellation_persistence_denied",
    "cancellation_ledger_write_denied",
    "supersession_request_acceptance_denied",
    "supersession_recording_denied",
    "supersession_persistence_denied",
    "supersession_ledger_write_denied",
    "replacement_receipt_acceptance_denied",
    "replacement_receipt_recording_denied",
    "replacement_receipt_persistence_denied",
    "replacement_receipt_materialization_denied",
    "replacement_receipt_filesystem_write_denied",
    "replacement_receipt_ledger_write_denied",
    "tombstone_delete_marker_acceptance_denied",
    "tombstone_delete_marker_write_denied",
    "latest_replacement_promotion_denied",
    "completion_ack_replacement_denied",
    "export_query_replacement_denied",
    "cancellation_supersession_state_persistence_denied",
    "replacement_authority_derivation_denied",
    "result_receipt_supersession_authority_denied",
    "dry_run_execution_execution_denied",
    "dry_run_execution_envelope_persistence_denied",
    "dry_run_execution_result_persistence_denied",
    "dry_run_execution_result_receipt_persistence_denied",
    "dry_run_execution_result_receipt_filesystem_write_denied",
    "dry_run_execution_result_receipt_ledger_recording_denied",
    "dry_run_execution_result_receipt_delivery_denied",
    "dry_run_execution_result_receipt_materialization_denied",
    "acceptance_receipt_persistence_denied",
    "operator_packet_persistence_denied",
    "production_write_execution_denied",
    "production_durable_memory_backend_write_denied",
    "durable_memory_backend_read_or_rollback_denied",
    "memory_store_mutation_denied",
    "wal_write_denied",
    "receipt_persistence_denied",
    "post_write_readback_denied",
    "rollback_execution_denied",
    "tombstone_write_denied",
    "raw_payload_plaintext_denied",
    "kg_live_write_denied",
    "provider_model_invocation_denied",
    "credential_channel_release_install_denied",
    "active_binary_mutation_denied",
    "unrestricted_full_live_activation_denied"
  ]) as $denials
  | ([
    "dry_run_execution_result_receipt_cancellation_request_accepted",
    "dry_run_execution_result_receipt_cancellation_recorded",
    "dry_run_execution_result_receipt_cancellation_persisted",
    "dry_run_execution_result_receipt_cancellation_ledger_written",
    "dry_run_execution_result_receipt_supersession_request_accepted",
    "dry_run_execution_result_receipt_supersession_recorded",
    "dry_run_execution_result_receipt_supersession_persisted",
    "dry_run_execution_result_receipt_supersession_ledger_written",
    "dry_run_execution_result_receipt_replacement_receipt_accepted",
    "dry_run_execution_result_receipt_replacement_receipt_recorded",
    "dry_run_execution_result_receipt_replacement_receipt_persisted",
    "dry_run_execution_result_receipt_replacement_receipt_materialized",
    "dry_run_execution_result_receipt_replacement_receipt_filesystem_written",
    "dry_run_execution_result_receipt_replacement_receipt_ledger_written",
    "dry_run_execution_result_receipt_tombstone_delete_marker_accepted",
    "dry_run_execution_result_receipt_tombstone_delete_marker_written",
    "dry_run_execution_result_receipt_latest_replacement_promoted",
    "dry_run_execution_result_receipt_completion_ack_replaced",
    "dry_run_execution_result_receipt_export_query_replaced",
    "dry_run_execution_result_receipt_cancellation_supersession_state_persisted",
    "dry_run_execution_result_receipt_cancellation_supersession_ledger_written",
    "dry_run_execution_result_receipt_ordering_cursor_persisted",
    "dry_run_execution_result_receipt_monotonic_sequence_recorded",
    "dry_run_execution_result_receipt_replay_state_persisted",
    "dry_run_execution_result_receipt_idempotency_ledger_written",
    "dry_run_execution_result_receipt_persisted",
    "dry_run_execution_result_receipt_filesystem_written",
    "dry_run_execution_result_receipt_ledger_recorded",
    "dry_run_execution_result_receipt_delivered",
    "dry_run_execution_result_receipt_materialized",
    "dry_run_execution_envelope_persisted",
    "dry_run_execution_executed",
    "dry_run_execution_result_persisted",
    "acceptance_receipt_persisted",
    "operator_packet_persisted",
    "operator_packet_acceptance_receipt_persisted",
    "production_durable_memory_write_executed",
    "production_durable_memory_backend_present",
    "production_durable_memory_store_write_performed",
    "actual_production_durable_memory_write_performed",
    "durable_memory_store_write_performed",
    "durable_memory_store_read_performed",
    "durable_memory_store_rollback_performed",
    "memory_write_execution_performed",
    "memory_store_write_path_enabled",
    "memory_store_write_allowed",
    "memory_store_write_performed",
    "memory_store_mutation_allowed",
    "memory_store_mutated",
    "wal_write_performed",
    "wal_recorded",
    "wal_persisted",
    "receipt_recorded",
    "receipt_persisted",
    "receipt_materialized",
    "receipt_delivered",
    "post_write_readback_performed",
    "readback_result_recorded",
    "readback_result_persisted",
    "readback_result_accepted",
    "rollback_executed",
    "rollback_performed",
    "rollback_result_recorded",
    "rollback_result_persisted",
    "rollback_result_accepted",
    "tombstone_write_performed",
    "tombstone_cleanup_executed",
    "tombstone_cleanup_result_recorded",
    "tombstone_cleanup_result_accepted",
    "raw_payload_plaintext_recorded",
    "raw_payload_plaintext_persisted",
    "secret_material_read",
    "credential_read",
    "secret_file_read",
    "kg_adapter_read_performed",
    "live_kg_write_performed",
    "provider_invoked",
    "model_invoked",
    "telegram_send_performed",
    "channel_send_performed",
    "external_send_performed",
    "public_claim_promoted",
    "public_release_published",
    "public_ga_claimed",
    "release_artifact_written",
    "public_artifact_written",
    "install_executed",
    "launchd_mutated",
    "service_restarted",
    "service_restart_performed",
    "active_binary_mutated"
  ]) as $false_keys
  | ([
    "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_performed",
    "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_result_recorded",
    "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_result_accepted",
    "source_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_accepted",
    "dry_run_execution_result_receipt_cancellation_supersession_denial_matrix_bound",
    "dry_run_execution_result_receipt_cancellation_policy_bound",
    "dry_run_execution_result_receipt_supersession_policy_bound",
    "dry_run_execution_result_receipt_cancellation_request_denied",
    "dry_run_execution_result_receipt_supersession_request_denied",
    "dry_run_execution_result_receipt_replacement_receipt_denied",
    "dry_run_execution_result_receipt_tombstone_delete_marker_denied",
    "dry_run_execution_result_receipt_latest_replacement_denied",
    "dry_run_execution_result_receipt_completion_ack_replacement_denied",
    "dry_run_execution_result_receipt_export_query_replacement_denied",
    "dry_run_execution_result_receipt_cancellation_supersession_handoff_bound",
    "dry_run_execution_result_receipt_cancellation_supersession_state_persistence_forbidden",
    "dry_run_execution_result_receipt_replacement_receipt_persistence_forbidden",
    "dry_run_execution_execution_forbidden_on_cancellation_supersession_route",
    "production_write_execution_forbidden_on_cancellation_supersession_route",
    "kg_provider_channel_release_install_active_binary_forbidden"
  ]) as $true_keys
  | ({
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      source_command:$source_command,
      native_gateway_source_command_count:274,
      route_count:274,
      implemented_route_count:274,
      missing_route_count:0,
      route_count_source_command_accepted:true,
      memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_ready:true,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_ready:true,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_performed:true,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_accepted:true,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_mode:"dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_no_cancel_no_supersede_no_replacement_no_tombstone_no_execution_no_production_durable_memory_mutation",
      source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_ready:true,
      source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_report_sha256:$source_report_sha256,
      source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_accepted_count:1,
      source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count:1,
      source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count:9,
      source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_count:55,
      approved_production_namespace:$approved_production_namespace,
      approved_production_store:$approved_production_store,
      approved_production_scope:$approved_production_scope,
      production_durable_memory_target_id:$production_durable_memory_target_id,
      production_durable_memory_payload_class:$production_durable_memory_payload_class,
      operator_packet_scope:$operator_packet_scope,
      source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_hash_sha256:$source_ordering_boundary_hash_sha256,
      source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_policy_hash_sha256:$source_ordering_policy_hash_sha256,
      source_dry_run_execution_result_receipt_ordering_monotonicity_denial_matrix_hash_sha256:$source_ordering_matrix_hash_sha256,
      source_dry_run_execution_result_receipt_ordering_sequence_policy_hash_sha256:$source_ordering_sequence_policy_hash_sha256,
      source_dry_run_execution_result_receipt_late_receipt_denial_hash_sha256:$source_late_receipt_denial_hash_sha256,
      source_dry_run_execution_result_receipt_future_receipt_denial_hash_sha256:$source_future_receipt_denial_hash_sha256,
      source_dry_run_execution_result_receipt_rollback_sequence_denial_hash_sha256:$source_rollback_sequence_denial_hash_sha256,
      source_dry_run_execution_result_receipt_same_sequence_replacement_denial_hash_sha256:$source_same_sequence_replacement_denial_hash_sha256,
      source_dry_run_execution_result_receipt_latest_wins_promotion_denial_hash_sha256:$source_latest_wins_promotion_denial_hash_sha256,
      source_dry_run_execution_result_receipt_sequence_gap_denial_hash_sha256:$source_sequence_gap_denial_hash_sha256,
      source_dry_run_execution_result_receipt_ordering_handoff_hash_sha256:$source_ordering_handoff_hash_sha256,
      source_dry_run_execution_result_receipt_ordering_result_hash_sha256:$source_ordering_result_hash_sha256,
      dry_run_execution_result_receipt_cancellation_supersession_denial_matrix_hash_sha256:$cancellation_matrix_hash_sha256,
      dry_run_execution_result_receipt_cancellation_policy_hash_sha256:$cancellation_policy_hash_sha256,
      dry_run_execution_result_receipt_supersession_policy_hash_sha256:$supersession_policy_hash_sha256,
      dry_run_execution_result_receipt_replacement_receipt_denial_hash_sha256:$replacement_receipt_denial_hash_sha256,
      dry_run_execution_result_receipt_tombstone_delete_marker_denial_hash_sha256:$tombstone_delete_marker_denial_hash_sha256,
      dry_run_execution_result_receipt_latest_replacement_denial_hash_sha256:$latest_replacement_denial_hash_sha256,
      dry_run_execution_result_receipt_completion_ack_replacement_denial_hash_sha256:$completion_ack_replacement_denial_hash_sha256,
      dry_run_execution_result_receipt_export_query_replacement_denial_hash_sha256:$export_query_replacement_denial_hash_sha256,
      dry_run_execution_result_receipt_cancellation_supersession_handoff_hash_sha256:$cancellation_handoff_hash_sha256,
      dry_run_execution_result_receipt_cancellation_supersession_result_hash_sha256:$cancellation_result_hash_sha256,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_hash_sha256:$cancellation_boundary_hash_sha256,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_policy_hash_sha256:$cancellation_boundary_policy_hash_sha256,
      required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_surface_count:($surfaces | length),
      ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_surface_count:($surfaces | length),
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_surfaces:$surfaces,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_fixture_count:10,
      accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_fixture_count:1,
      blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_fixture_count:9,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_fixtures:([
        {
          id:"scoped-production-durable-memory-write-dry-run-execution-result-receipt-cancellation-supersession-denial",
          fixture_id:"scoped-production-durable-memory-write-dry-run-execution-result-receipt-cancellation-supersession-denial",
          scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_accepted:true,
          reason:"dry_run_execution_result_receipt_cancellation_supersession_denial_bound_without_cancellation_supersession_replacement_tombstone_execution_or_production_write",
          source_ordering_monotonicity_denial_boundary_bound:true,
          dry_run_execution_result_receipt_cancellation_supersession_denial_matrix_bound:true,
          dry_run_execution_result_receipt_cancellation_request_denied:true,
          dry_run_execution_result_receipt_supersession_request_denied:true,
          dry_run_execution_result_receipt_replacement_receipt_denied:true,
          dry_run_execution_result_receipt_tombstone_delete_marker_denied:true,
          dry_run_execution_result_receipt_latest_replacement_denied:true,
          dry_run_execution_result_receipt_completion_ack_replacement_denied:true,
          dry_run_execution_result_receipt_export_query_replacement_denied:true,
          dry_run_execution_result_receipt_cancellation_recorded:false,
          dry_run_execution_result_receipt_supersession_recorded:false,
          dry_run_execution_result_receipt_replacement_receipt_persisted:false,
          dry_run_execution_executed:false,
          production_durable_memory_store_write_performed:false,
          external_send_performed:false
        }
      ] + ([
        "missing-ordering-monotonicity-source",
        "missing-ordering-result-hash",
        "missing-cancellation-supersession-matrix",
        "cancellation-request-attempt",
        "supersession-request-attempt",
        "replacement-receipt-attempt",
        "tombstone-delete-marker-attempt",
        "latest-replacement-promotion-attempt",
        "completion-ack-or-export-query-replacement-attempt"
      ] | map({
        id:.,
        fixture_id:.,
        scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_accepted:false,
        reason:"blocked_noop",
        dry_run_execution_result_receipt_cancellation_recorded:false,
        dry_run_execution_result_receipt_supersession_recorded:false,
        dry_run_execution_result_receipt_replacement_receipt_persisted:false,
        dry_run_execution_executed:false,
        production_durable_memory_store_write_performed:false,
        external_send_performed:false
      }))),
      denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary:$denials,
      denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_count:($denials | length),
      source_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_bound:true,
      approved_production_namespace_bound:true,
      approved_production_store_bound:true,
      approved_production_scope_bound:true,
      production_durable_memory_target_bound:true,
      dry_run_execution_result_receipt_cancellation_supersession_denial_matrix_bound:true,
      dry_run_execution_result_receipt_cancellation_policy_bound:true,
      dry_run_execution_result_receipt_supersession_policy_bound:true,
      dry_run_execution_result_receipt_cancellation_request_denied:true,
      dry_run_execution_result_receipt_supersession_request_denied:true,
      dry_run_execution_result_receipt_replacement_receipt_denied:true,
      dry_run_execution_result_receipt_tombstone_delete_marker_denied:true,
      dry_run_execution_result_receipt_latest_replacement_denied:true,
      dry_run_execution_result_receipt_completion_ack_replacement_denied:true,
      dry_run_execution_result_receipt_export_query_replacement_denied:true,
      dry_run_execution_result_receipt_cancellation_supersession_handoff_bound:true,
      dry_run_execution_result_receipt_cancellation_supersession_state_persistence_forbidden:true,
      dry_run_execution_result_receipt_replacement_receipt_persistence_forbidden:true,
      dry_run_execution_execution_forbidden_on_cancellation_supersession_route:true,
      dry_run_execution_result_receipt_persistence_forbidden_on_cancellation_supersession_route:true,
      production_write_execution_forbidden_on_cancellation_supersession_route:true,
      production_durable_memory_write_forbidden:true,
      memory_store_mutation_forbidden:true,
      wal_write_forbidden_on_cancellation_supersession_route:true,
      receipt_persist_forbidden_on_cancellation_supersession_route:true,
      rollback_execution_forbidden_on_cancellation_supersession_route:true,
      tombstone_write_forbidden_on_cancellation_supersession_route:true,
      kg_live_write_forbidden:true,
      provider_model_invocation_forbidden:true,
      credential_channel_public_release_forbidden:true,
      install_restart_active_binary_mutation_forbidden:true,
      allowed_next_actions:[
        {
          action:"run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_require_live_gate",
          status:"allowed_verification_only",
          accepts_cancellation_supersession_denial_matrix:true,
          records_cancellation:false,
          persists_cancellation:false,
          records_supersession:false,
          persists_supersession:false,
          accepts_replacement_receipt:false,
          writes_tombstone_delete_marker:false,
          executes_dry_run:false,
          persists_dry_run_result_receipt:false,
          writes_production_durable_memory:false,
          writes_memory_store:false,
          writes_wal:false,
          persists_receipt:false
        },
        {
          action:"prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary",
          status:"requires_separate_result_receipt_audit_trail_immutable_evidence_denial_gate",
          requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary:true,
          records_cancellation:false,
          persists_cancellation:false,
          records_supersession:false,
          persists_supersession:false,
          executes_dry_run:false,
          writes_production_durable_memory:false,
          persists_dry_run_result_receipt:false
        }
      ]
    }
    + zero_fields($false_keys)
    + true_count_fields($true_keys)
    + {
      side_effects:(zero_fields($false_keys) + true_count_fields($true_keys))
    })
  '
