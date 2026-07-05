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
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-replay-idempotency-denial-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-replay-idempotency-denial-boundary-gate.sh
)"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_ready == true
    and $source.scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_accepted == true
    and $source.accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixture_count == 1
    and $source.blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixture_count == 9
    and $source.scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_result_accepted_count == 1
    and $source.dry_run_execution_result_receipt_replay_state_persisted == false
    and $source.dry_run_execution_result_receipt_idempotency_ledger_written == false
    and $source.dry_run_execution_result_receipt_duplicate_receipt_accepted == false
    and $source.dry_run_execution_result_receipt_stale_receipt_accepted == false
    and $source.dry_run_execution_result_receipt_cross_session_replay_accepted == false
    and $source.dry_run_execution_result_receipt_hash_chain_mismatch_accepted == false
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
    and $source.allowed_next_actions[1].action == "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary"
    and $source.allowed_next_actions[1].requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary == true
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
source_replay_boundary_hash_sha256="$(jq -r '.scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_replay_policy_hash_sha256="$(jq -r '.scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_policy_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_replay_matrix_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_replay_idempotency_denial_matrix_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_replay_identity_session_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_replay_idempotency_identity_session_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_replay_nonce_scope_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_replay_idempotency_nonce_scope_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_duplicate_receipt_denial_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_replay_idempotency_duplicate_receipt_denial_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_stale_receipt_denial_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_replay_idempotency_stale_receipt_denial_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_hash_chain_mismatch_denial_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_replay_idempotency_hash_chain_mismatch_denial_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_cross_session_denial_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_replay_idempotency_cross_session_denial_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_replay_handoff_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_replay_idempotency_handoff_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_replay_result_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_replay_idempotency_result_hash_sha256 // ""' <<<"$SOURCE_JSON")"

ordering_monotonicity_matrix_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-ordering-monotonicity-denial-matrix:v1:source-replay=${source_replay_result_hash_sha256}:duplicate=${source_duplicate_receipt_denial_hash_sha256}:stale=${source_stale_receipt_denial_hash_sha256}:late=deny:future=deny:rollback=deny:same-sequence=deny:latest-wins=deny:sequence-gap=deny:persist-cursor=false"
)"
ordering_sequence_policy_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-ordering-sequence-policy:v1:matrix=${ordering_monotonicity_matrix_hash_sha256}:source-policy=${source_replay_policy_hash_sha256}:monotonic-record=false:cursor=false"
)"
ordering_identity_session_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-ordering-identity-session:v1:source-identity=${source_replay_identity_session_hash_sha256}:source-cross-session=${source_cross_session_denial_hash_sha256}:cross-session=false"
)"
ordering_latest_sequence_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-ordering-latest-sequence:v1:source-nonce=${source_replay_nonce_scope_hash_sha256}:latest-wins=false:same-sequence=false"
)"
late_receipt_denial_hash_sha256="$(sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-late-denial:v1:policy=${ordering_sequence_policy_hash_sha256}:accepted=false")"
future_receipt_denial_hash_sha256="$(sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-future-denial:v1:policy=${ordering_sequence_policy_hash_sha256}:accepted=false")"
rollback_sequence_denial_hash_sha256="$(sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-rollback-sequence-denial:v1:policy=${ordering_sequence_policy_hash_sha256}:accepted=false")"
same_sequence_replacement_denial_hash_sha256="$(sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-same-sequence-replacement-denial:v1:latest=${ordering_latest_sequence_hash_sha256}:accepted=false")"
latest_wins_promotion_denial_hash_sha256="$(sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-latest-wins-promotion-denial:v1:latest=${ordering_latest_sequence_hash_sha256}:accepted=false")"
sequence_gap_denial_hash_sha256="$(sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-sequence-gap-denial:v1:matrix=${ordering_monotonicity_matrix_hash_sha256}:accepted=false")"
ordering_handoff_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-ordering-handoff:v1:late=${late_receipt_denial_hash_sha256}:future=${future_receipt_denial_hash_sha256}:rollback=${rollback_sequence_denial_hash_sha256}:same=${same_sequence_replacement_denial_hash_sha256}:latest=${latest_wins_promotion_denial_hash_sha256}:gap=${sequence_gap_denial_hash_sha256}:next=cancellation-supersession-denial-boundary"
)"
ordering_result_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-ordering-result:v1:matrix=${ordering_monotonicity_matrix_hash_sha256}:handoff=${ordering_handoff_hash_sha256}:accepted=true:cursor=false:sequence-record=false:ledger=false:executed=false:production-write=false"
)"
ordering_boundary_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-ordering-monotonicity-denial-boundary:v1:source=${source_report_sha256}:result=${ordering_result_hash_sha256}:fixtures=10:accepted=1:denials=55:ordering-cursor=false:monotonic-sequence=false:dry-run-executed=false:production-write=false"
)"
ordering_policy_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-ordering-monotonicity-denial-policy:v1:bind-source-replay-matrix-sequence-late-future-rollback-same-sequence-latest-wins-gap-handoff:no-ordering-cursor:no-monotonic-sequence:no-execution:no-production-write:no-kg:no-provider:no-channel:no-release:no-install"
)"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_gate" \
  --arg endpoint "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-ordering-monotonicity-denial-boundary" \
  --arg source_command "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-ordering-monotonicity-denial-boundary --json" \
  --arg approved_production_namespace "$approved_production_namespace" \
  --arg approved_production_store "$approved_production_store" \
  --arg approved_production_scope "$approved_production_scope" \
  --arg production_durable_memory_target_id "$production_durable_memory_target_id" \
  --arg production_durable_memory_payload_class "$production_durable_memory_payload_class" \
  --arg operator_packet_scope "$operator_packet_scope" \
  --arg source_report_sha256 "$source_report_sha256" \
  --arg source_replay_boundary_hash_sha256 "$source_replay_boundary_hash_sha256" \
  --arg source_replay_policy_hash_sha256 "$source_replay_policy_hash_sha256" \
  --arg source_replay_matrix_hash_sha256 "$source_replay_matrix_hash_sha256" \
  --arg source_replay_identity_session_hash_sha256 "$source_replay_identity_session_hash_sha256" \
  --arg source_replay_nonce_scope_hash_sha256 "$source_replay_nonce_scope_hash_sha256" \
  --arg source_duplicate_receipt_denial_hash_sha256 "$source_duplicate_receipt_denial_hash_sha256" \
  --arg source_stale_receipt_denial_hash_sha256 "$source_stale_receipt_denial_hash_sha256" \
  --arg source_hash_chain_mismatch_denial_hash_sha256 "$source_hash_chain_mismatch_denial_hash_sha256" \
  --arg source_cross_session_denial_hash_sha256 "$source_cross_session_denial_hash_sha256" \
  --arg source_replay_handoff_hash_sha256 "$source_replay_handoff_hash_sha256" \
  --arg source_replay_result_hash_sha256 "$source_replay_result_hash_sha256" \
  --arg ordering_monotonicity_matrix_hash_sha256 "$ordering_monotonicity_matrix_hash_sha256" \
  --arg ordering_sequence_policy_hash_sha256 "$ordering_sequence_policy_hash_sha256" \
  --arg ordering_identity_session_hash_sha256 "$ordering_identity_session_hash_sha256" \
  --arg ordering_latest_sequence_hash_sha256 "$ordering_latest_sequence_hash_sha256" \
  --arg late_receipt_denial_hash_sha256 "$late_receipt_denial_hash_sha256" \
  --arg future_receipt_denial_hash_sha256 "$future_receipt_denial_hash_sha256" \
  --arg rollback_sequence_denial_hash_sha256 "$rollback_sequence_denial_hash_sha256" \
  --arg same_sequence_replacement_denial_hash_sha256 "$same_sequence_replacement_denial_hash_sha256" \
  --arg latest_wins_promotion_denial_hash_sha256 "$latest_wins_promotion_denial_hash_sha256" \
  --arg sequence_gap_denial_hash_sha256 "$sequence_gap_denial_hash_sha256" \
  --arg ordering_handoff_hash_sha256 "$ordering_handoff_hash_sha256" \
  --arg ordering_result_hash_sha256 "$ordering_result_hash_sha256" \
  --arg ordering_boundary_hash_sha256 "$ordering_boundary_hash_sha256" \
  --arg ordering_policy_hash_sha256 "$ordering_policy_hash_sha256" \
  --argjson source "$SOURCE_JSON" \
  '
  def zero_fields($keys): reduce $keys[] as $k ({}; .[$k]=false | .[$k + "_count"]=0);
  def true_count_fields($keys): reduce $keys[] as $k ({}; .[$k]=true | .[$k + "_count"]=1);
  ([
    "source_replay_idempotency_denial_boundary_required",
    "source_replay_idempotency_result_required",
    "dry_run_execution_result_receipt_ordering_monotonicity_matrix_required",
    "dry_run_execution_result_receipt_ordering_sequence_policy_required",
    "dry_run_execution_result_receipt_late_receipt_denial_required",
    "dry_run_execution_result_receipt_future_receipt_denial_required",
    "dry_run_execution_result_receipt_rollback_sequence_denial_required",
    "dry_run_execution_result_receipt_same_sequence_denial_required",
    "dry_run_execution_result_receipt_latest_wins_promotion_denial_required",
    "dry_run_execution_result_receipt_sequence_gap_denial_required",
    "dry_run_execution_result_receipt_ordering_handoff_required",
    "dry_run_execution_result_receipt_ordering_cursor_persistence_forbidden",
    "dry_run_execution_result_receipt_monotonic_sequence_recording_forbidden",
    "dry_run_execution_execution_forbidden_on_ordering_monotonicity_route",
    "production_write_execution_forbidden_on_ordering_monotonicity_route",
    "kg_provider_channel_release_install_active_binary_forbidden"
  ]) as $surfaces
  | ([
    "source_replay_idempotency_denial_boundary_required",
    "source_replay_idempotency_result_hash_required",
    "source_replay_idempotency_policy_hash_required",
    "source_replay_idempotency_matrix_required",
    "source_replay_idempotency_identity_session_required",
    "source_replay_idempotency_nonce_scope_required",
    "source_duplicate_receipt_denial_required",
    "source_stale_receipt_denial_required",
    "source_hash_chain_mismatch_denial_required",
    "source_cross_session_denial_required",
    "approved_production_namespace_required",
    "approved_production_store_required",
    "approved_production_scope_required",
    "production_durable_memory_target_required",
    "ordering_monotonicity_matrix_required",
    "ordering_sequence_policy_required",
    "ordering_identity_session_required",
    "ordering_latest_sequence_required",
    "late_receipt_acceptance_denied",
    "future_receipt_acceptance_denied",
    "rollback_sequence_acceptance_denied",
    "same_sequence_replacement_denied",
    "latest_wins_promotion_denied",
    "sequence_gap_acceptance_denied",
    "ordering_cursor_persistence_denied",
    "monotonic_sequence_recording_denied",
    "ordering_ledger_write_denied",
    "ordering_guard_state_recording_denied",
    "replay_state_persistence_denied",
    "idempotency_ledger_write_denied",
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
    "dry_run_execution_result_receipt_ordering_cursor_persisted",
    "dry_run_execution_result_receipt_ordering_cursor_recorded",
    "dry_run_execution_result_receipt_ordering_ledger_written",
    "dry_run_execution_result_receipt_ordering_guard_state_recorded",
    "dry_run_execution_result_receipt_monotonic_sequence_recorded",
    "dry_run_execution_result_receipt_late_receipt_accepted",
    "dry_run_execution_result_receipt_future_receipt_accepted",
    "dry_run_execution_result_receipt_rollback_sequence_accepted",
    "dry_run_execution_result_receipt_same_sequence_replacement_accepted",
    "dry_run_execution_result_receipt_latest_wins_promoted",
    "dry_run_execution_result_receipt_sequence_gap_accepted",
    "dry_run_execution_result_receipt_ordering_attempt_accepted",
    "dry_run_execution_result_receipt_replay_state_persisted",
    "dry_run_execution_result_receipt_idempotency_ledger_written",
    "dry_run_execution_result_receipt_persisted",
    "dry_run_execution_executed",
    "dry_run_execution_result_persisted",
    "production_durable_memory_write_executed",
    "production_durable_memory_store_write_performed",
    "actual_production_durable_memory_write_performed",
    "durable_memory_store_write_performed",
    "durable_memory_store_read_performed",
    "durable_memory_store_rollback_performed",
    "memory_write_execution_performed",
    "memory_store_write_performed",
    "memory_store_mutated",
    "wal_write_performed",
    "receipt_persisted",
    "post_write_readback_performed",
    "rollback_executed",
    "rollback_performed",
    "tombstone_write_performed",
    "tombstone_cleanup_executed",
    "live_kg_write_performed",
    "provider_invoked",
    "model_invoked",
    "credential_read",
    "channel_send_performed",
    "external_send_performed",
    "release_artifact_written",
    "install_executed",
    "service_restarted",
    "active_binary_mutated"
  ]) as $false_keys
  | ([
    "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_performed",
    "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_result_recorded",
    "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_result_accepted",
    "source_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_accepted",
    "dry_run_execution_result_receipt_ordering_monotonicity_denial_matrix_bound",
    "dry_run_execution_result_receipt_ordering_sequence_policy_bound",
    "dry_run_execution_result_receipt_ordering_identity_session_bound",
    "dry_run_execution_result_receipt_ordering_latest_sequence_bound",
    "dry_run_execution_result_receipt_late_receipt_denied",
    "dry_run_execution_result_receipt_future_receipt_denied",
    "dry_run_execution_result_receipt_rollback_sequence_denied",
    "dry_run_execution_result_receipt_same_sequence_replacement_denied",
    "dry_run_execution_result_receipt_latest_wins_promotion_denied",
    "dry_run_execution_result_receipt_sequence_gap_denied",
    "dry_run_execution_result_receipt_ordering_handoff_bound",
    "dry_run_execution_result_receipt_ordering_cursor_persistence_forbidden",
    "dry_run_execution_result_receipt_monotonic_sequence_recording_forbidden",
    "dry_run_execution_execution_forbidden_on_ordering_monotonicity_route",
    "production_write_execution_forbidden_on_ordering_monotonicity_route",
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
      memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_ready:true,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_ready:true,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_performed:true,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_accepted:true,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_mode:"dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_no_ordering_cursor_no_monotonic_sequence_no_execution_no_production_durable_memory_mutation",
      source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_ready:true,
      source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_report_sha256:$source_report_sha256,
      source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_accepted_count:1,
      source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixture_count:1,
      source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixture_count:9,
      source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_count:54,
      approved_production_namespace:$approved_production_namespace,
      approved_production_store:$approved_production_store,
      approved_production_scope:$approved_production_scope,
      production_durable_memory_target_id:$production_durable_memory_target_id,
      production_durable_memory_payload_class:$production_durable_memory_payload_class,
      operator_packet_scope:$operator_packet_scope,
      source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_hash_sha256:$source_replay_boundary_hash_sha256,
      source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_policy_hash_sha256:$source_replay_policy_hash_sha256,
      source_dry_run_execution_result_receipt_replay_idempotency_denial_matrix_hash_sha256:$source_replay_matrix_hash_sha256,
      source_dry_run_execution_result_receipt_replay_idempotency_identity_session_hash_sha256:$source_replay_identity_session_hash_sha256,
      source_dry_run_execution_result_receipt_replay_idempotency_nonce_scope_hash_sha256:$source_replay_nonce_scope_hash_sha256,
      source_dry_run_execution_result_receipt_replay_idempotency_duplicate_receipt_denial_hash_sha256:$source_duplicate_receipt_denial_hash_sha256,
      source_dry_run_execution_result_receipt_replay_idempotency_stale_receipt_denial_hash_sha256:$source_stale_receipt_denial_hash_sha256,
      source_dry_run_execution_result_receipt_replay_idempotency_hash_chain_mismatch_denial_hash_sha256:$source_hash_chain_mismatch_denial_hash_sha256,
      source_dry_run_execution_result_receipt_replay_idempotency_cross_session_denial_hash_sha256:$source_cross_session_denial_hash_sha256,
      source_dry_run_execution_result_receipt_replay_idempotency_handoff_hash_sha256:$source_replay_handoff_hash_sha256,
      source_dry_run_execution_result_receipt_replay_idempotency_result_hash_sha256:$source_replay_result_hash_sha256,
      dry_run_execution_result_receipt_ordering_monotonicity_denial_matrix_hash_sha256:$ordering_monotonicity_matrix_hash_sha256,
      dry_run_execution_result_receipt_ordering_sequence_policy_hash_sha256:$ordering_sequence_policy_hash_sha256,
      dry_run_execution_result_receipt_ordering_identity_session_hash_sha256:$ordering_identity_session_hash_sha256,
      dry_run_execution_result_receipt_ordering_latest_sequence_hash_sha256:$ordering_latest_sequence_hash_sha256,
      dry_run_execution_result_receipt_late_receipt_denial_hash_sha256:$late_receipt_denial_hash_sha256,
      dry_run_execution_result_receipt_future_receipt_denial_hash_sha256:$future_receipt_denial_hash_sha256,
      dry_run_execution_result_receipt_rollback_sequence_denial_hash_sha256:$rollback_sequence_denial_hash_sha256,
      dry_run_execution_result_receipt_same_sequence_replacement_denial_hash_sha256:$same_sequence_replacement_denial_hash_sha256,
      dry_run_execution_result_receipt_latest_wins_promotion_denial_hash_sha256:$latest_wins_promotion_denial_hash_sha256,
      dry_run_execution_result_receipt_sequence_gap_denial_hash_sha256:$sequence_gap_denial_hash_sha256,
      dry_run_execution_result_receipt_ordering_handoff_hash_sha256:$ordering_handoff_hash_sha256,
      dry_run_execution_result_receipt_ordering_result_hash_sha256:$ordering_result_hash_sha256,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_hash_sha256:$ordering_boundary_hash_sha256,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_policy_hash_sha256:$ordering_policy_hash_sha256,
      required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_surface_count:($surfaces | length),
      ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_surface_count:($surfaces | length),
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_surfaces:$surfaces,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count:10,
      accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count:1,
      blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count:9,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixtures:[
        {id:"scoped-production-durable-memory-write-dry-run-execution-result-receipt-ordering-monotonicity-denial", fixture_id:"scoped-production-durable-memory-write-dry-run-execution-result-receipt-ordering-monotonicity-denial", scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_accepted:true},
        {id:"missing-replay-idempotency-source", fixture_id:"missing-replay-idempotency-source", scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_accepted:false},
        {id:"missing-replay-idempotency-result-hash", fixture_id:"missing-replay-idempotency-result-hash", scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_accepted:false},
        {id:"missing-ordering-monotonicity-matrix", fixture_id:"missing-ordering-monotonicity-matrix", scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_accepted:false},
        {id:"late-result-receipt-attempt", fixture_id:"late-result-receipt-attempt", scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_accepted:false},
        {id:"future-result-receipt-attempt", fixture_id:"future-result-receipt-attempt", scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_accepted:false},
        {id:"rollback-sequence-result-receipt-attempt", fixture_id:"rollback-sequence-result-receipt-attempt", scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_accepted:false},
        {id:"same-sequence-replacement-attempt", fixture_id:"same-sequence-replacement-attempt", scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_accepted:false},
        {id:"latest-wins-promotion-attempt", fixture_id:"latest-wins-promotion-attempt", scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_accepted:false},
        {id:"ordering-cursor-or-monotonic-sequence-persistence-attempt", fixture_id:"ordering-cursor-or-monotonic-sequence-persistence-attempt", scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_accepted:false}
      ],
      denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary:$denials,
      denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_count:($denials | length),
      allowed_next_actions:[
        {action:"run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_require_live_gate", status:"allowed_verification_only", accepts_ordering_monotonicity_denial_matrix:true, persists_ordering_cursor:false, records_monotonic_sequence:false, writes_ordering_ledger:false, executes_dry_run:false, persists_dry_run_result_receipt:false, writes_production_durable_memory:false, writes_memory_store:false, writes_wal:false, persists_receipt:false},
        {action:"prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary", status:"requires_separate_result_receipt_cancellation_supersession_denial_gate", requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary:true, persists_ordering_cursor:false, records_monotonic_sequence:false, executes_dry_run:false, writes_production_durable_memory:false, persists_dry_run_result_receipt:false}
      ],
      source_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_bound:true,
      dry_run_execution_result_receipt_ordering_monotonicity_denial_matrix_bound:true,
      dry_run_execution_result_receipt_ordering_sequence_policy_bound:true,
      dry_run_execution_result_receipt_late_receipt_denied:true,
      dry_run_execution_result_receipt_future_receipt_denied:true,
      dry_run_execution_result_receipt_rollback_sequence_denied:true,
      dry_run_execution_result_receipt_same_sequence_replacement_denied:true,
      dry_run_execution_result_receipt_latest_wins_promotion_denied:true,
      dry_run_execution_result_receipt_sequence_gap_denied:true,
      dry_run_execution_result_receipt_ordering_cursor_persistence_forbidden:true,
      dry_run_execution_result_receipt_monotonic_sequence_recording_forbidden:true,
      dry_run_execution_execution_forbidden_on_ordering_monotonicity_route:true,
      production_write_execution_forbidden_on_ordering_monotonicity_route:true,
      production_durable_memory_write_forbidden:true,
      memory_store_mutation_forbidden:true,
      kg_live_write_forbidden:true,
      provider_model_invocation_forbidden:true,
      credential_channel_public_release_forbidden:true,
      install_restart_active_binary_mutation_forbidden:true
    }
    + zero_fields($false_keys)
    + true_count_fields($true_keys)
    + {side_effects:(zero_fields($false_keys) + true_count_fields($true_keys))})
  '
