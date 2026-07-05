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
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-boundary-gate.sh
)"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_ready == true
    and $source.scoped_production_durable_memory_write_dry_run_execution_result_receipt_accepted == true
    and $source.accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_fixture_count == 1
    and $source.blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_fixture_count == 9
    and $source.scoped_production_durable_memory_write_dry_run_execution_result_receipt_result_accepted_count == 1
    and $source.dry_run_execution_result_receipt_persisted == false
    and $source.dry_run_execution_executed == false
    and $source.dry_run_execution_envelope_persisted == false
    and $source.dry_run_execution_result_persisted == false
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
    and $source.raw_payload_plaintext_recorded == false
    and $source.raw_payload_plaintext_persisted == false
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
    and $source.side_effects.scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_performed == true
    and $source.side_effects.scoped_production_durable_memory_write_dry_run_execution_result_receipt_result_accepted == true
    and $source.side_effects.dry_run_execution_result_receipt_persisted == false
    and $source.side_effects.dry_run_execution_executed == false
    and $source.side_effects.production_durable_memory_store_write_performed == false
    and $source.side_effects.memory_store_write_performed == false
    and $source.side_effects.wal_write_performed == false
    and $source.side_effects.receipt_persisted == false
    and $source.side_effects.external_send_performed == false
    and $source.allowed_next_actions[1].action == "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary"
    and $source.allowed_next_actions[1].requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary == true
    and $source.allowed_next_actions[1].executes_dry_run == false
    and $source.allowed_next_actions[1].writes_production_durable_memory == false
    and $source.allowed_next_actions[1].persists_dry_run_result_receipt == false
  ' >/dev/null

approved_production_namespace="$(jq -r '.approved_production_namespace // ""' <<<"$SOURCE_JSON")"
approved_production_store="$(jq -r '.approved_production_store // ""' <<<"$SOURCE_JSON")"
approved_production_scope="$(jq -r '.approved_production_scope // ""' <<<"$SOURCE_JSON")"
production_durable_memory_target_id="$(jq -r '.production_durable_memory_target_id // ""' <<<"$SOURCE_JSON")"
production_durable_memory_payload_class="$(jq -r '.production_durable_memory_payload_class // ""' <<<"$SOURCE_JSON")"
operator_packet_scope="$(jq -r '.operator_packet_scope // ""' <<<"$SOURCE_JSON")"
source_report_sha256="$(sha256_text "$SOURCE_JSON")"
source_dry_run_execution_result_hash_sha256="$(jq -r '.source_dry_run_execution_result_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_result_receipt_boundary_hash_sha256="$(jq -r '.scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_result_receipt_policy_hash_sha256="$(jq -r '.scoped_production_durable_memory_write_dry_run_execution_result_receipt_policy_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_result_receipt_envelope_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_envelope_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_result_receipt_identity_session_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_identity_session_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_result_receipt_digest_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_digest_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_result_receipt_hash_chain_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_hash_chain_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_result_receipt_readback_plan_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_readback_plan_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_result_receipt_replay_guard_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_replay_guard_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_result_receipt_handoff_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_handoff_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_result_receipt_result_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_result_hash_sha256 // ""' <<<"$SOURCE_JSON")"

dry_run_execution_result_receipt_replay_idempotency_denial_matrix_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-replay-idempotency-denial-matrix:v1:source-result-receipt=${source_result_receipt_result_hash_sha256}:source-replay=${source_result_receipt_replay_guard_hash_sha256}:duplicate=deny:stale=deny:cross-session=deny:hash-chain-mismatch=deny:persist-state=false"
)"
dry_run_execution_result_receipt_replay_idempotency_identity_session_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-replay-idempotency-identity-session:v1:matrix=${dry_run_execution_result_receipt_replay_idempotency_denial_matrix_hash_sha256}:source-identity=${source_result_receipt_identity_session_hash_sha256}:cross-session=false"
)"
dry_run_execution_result_receipt_replay_idempotency_nonce_scope_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-replay-idempotency-nonce-scope:v1:matrix=${dry_run_execution_result_receipt_replay_idempotency_denial_matrix_hash_sha256}:source-replay=${source_result_receipt_replay_guard_hash_sha256}:single-use=true:reuse=deny"
)"
dry_run_execution_result_receipt_replay_idempotency_duplicate_receipt_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-duplicate-denial:v1:nonce=${dry_run_execution_result_receipt_replay_idempotency_nonce_scope_hash_sha256}:source-receipt=${source_result_receipt_envelope_hash_sha256}:accepted=false:persist-state=false"
)"
dry_run_execution_result_receipt_replay_idempotency_stale_receipt_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-stale-denial:v1:source-receipt=${source_result_receipt_envelope_hash_sha256}:source-boundary=${source_result_receipt_boundary_hash_sha256}:accepted=false"
)"
dry_run_execution_result_receipt_replay_idempotency_hash_chain_mismatch_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-hash-chain-mismatch-denial:v1:source-hash-chain=${source_result_receipt_hash_chain_hash_sha256}:source-policy=${source_result_receipt_policy_hash_sha256}:accepted=false"
)"
dry_run_execution_result_receipt_replay_idempotency_cross_session_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-cross-session-denial:v1:identity=${dry_run_execution_result_receipt_replay_idempotency_identity_session_hash_sha256}:accepted=false"
)"
dry_run_execution_result_receipt_replay_idempotency_handoff_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-replay-idempotency-handoff:v1:duplicate=${dry_run_execution_result_receipt_replay_idempotency_duplicate_receipt_denial_hash_sha256}:stale=${dry_run_execution_result_receipt_replay_idempotency_stale_receipt_denial_hash_sha256}:hash-chain=${dry_run_execution_result_receipt_replay_idempotency_hash_chain_mismatch_denial_hash_sha256}:cross-session=${dry_run_execution_result_receipt_replay_idempotency_cross_session_denial_hash_sha256}:next=ordering-monotonicity-denial-boundary"
)"
dry_run_execution_result_receipt_replay_idempotency_result_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-replay-idempotency-result:v1:matrix=${dry_run_execution_result_receipt_replay_idempotency_denial_matrix_hash_sha256}:handoff=${dry_run_execution_result_receipt_replay_idempotency_handoff_hash_sha256}:accepted=true:replay-state-persisted=false:idempotency-ledger-written=false:executed=false:production-write=false"
)"
scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-replay-idempotency-denial-boundary:v1:source=${source_report_sha256}:result=${dry_run_execution_result_receipt_replay_idempotency_result_hash_sha256}:fixtures=10:accepted=1:denials=54:replay-state-persisted=false:idempotency-ledger=false:dry-run-executed=false:production-write=false"
)"
scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_policy_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-replay-idempotency-denial-policy:v1:bind-source-result-receipt-matrix-identity-session-nonce-duplicate-stale-hash-chain-cross-session-handoff:no-replay-state-persistence:no-idempotency-ledger:no-execution:no-production-write:no-kg:no-provider:no-channel:no-release:no-install"
)"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_gate" \
  --arg endpoint "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-replay-idempotency-denial-boundary" \
  --arg source_command "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-replay-idempotency-denial-boundary --json" \
  --arg approved_production_namespace "$approved_production_namespace" \
  --arg approved_production_store "$approved_production_store" \
  --arg approved_production_scope "$approved_production_scope" \
  --arg production_durable_memory_target_id "$production_durable_memory_target_id" \
  --arg production_durable_memory_payload_class "$production_durable_memory_payload_class" \
  --arg operator_packet_scope "$operator_packet_scope" \
  --arg source_report_sha256 "$source_report_sha256" \
  --arg source_dry_run_execution_result_hash_sha256 "$source_dry_run_execution_result_hash_sha256" \
  --arg source_result_receipt_boundary_hash_sha256 "$source_result_receipt_boundary_hash_sha256" \
  --arg source_result_receipt_policy_hash_sha256 "$source_result_receipt_policy_hash_sha256" \
  --arg source_result_receipt_envelope_hash_sha256 "$source_result_receipt_envelope_hash_sha256" \
  --arg source_result_receipt_identity_session_hash_sha256 "$source_result_receipt_identity_session_hash_sha256" \
  --arg source_result_receipt_digest_hash_sha256 "$source_result_receipt_digest_hash_sha256" \
  --arg source_result_receipt_hash_chain_hash_sha256 "$source_result_receipt_hash_chain_hash_sha256" \
  --arg source_result_receipt_readback_plan_hash_sha256 "$source_result_receipt_readback_plan_hash_sha256" \
  --arg source_result_receipt_replay_guard_hash_sha256 "$source_result_receipt_replay_guard_hash_sha256" \
  --arg source_result_receipt_handoff_hash_sha256 "$source_result_receipt_handoff_hash_sha256" \
  --arg source_result_receipt_result_hash_sha256 "$source_result_receipt_result_hash_sha256" \
  --arg dry_run_execution_result_receipt_replay_idempotency_denial_matrix_hash_sha256 "$dry_run_execution_result_receipt_replay_idempotency_denial_matrix_hash_sha256" \
  --arg dry_run_execution_result_receipt_replay_idempotency_identity_session_hash_sha256 "$dry_run_execution_result_receipt_replay_idempotency_identity_session_hash_sha256" \
  --arg dry_run_execution_result_receipt_replay_idempotency_nonce_scope_hash_sha256 "$dry_run_execution_result_receipt_replay_idempotency_nonce_scope_hash_sha256" \
  --arg dry_run_execution_result_receipt_replay_idempotency_duplicate_receipt_denial_hash_sha256 "$dry_run_execution_result_receipt_replay_idempotency_duplicate_receipt_denial_hash_sha256" \
  --arg dry_run_execution_result_receipt_replay_idempotency_stale_receipt_denial_hash_sha256 "$dry_run_execution_result_receipt_replay_idempotency_stale_receipt_denial_hash_sha256" \
  --arg dry_run_execution_result_receipt_replay_idempotency_hash_chain_mismatch_denial_hash_sha256 "$dry_run_execution_result_receipt_replay_idempotency_hash_chain_mismatch_denial_hash_sha256" \
  --arg dry_run_execution_result_receipt_replay_idempotency_cross_session_denial_hash_sha256 "$dry_run_execution_result_receipt_replay_idempotency_cross_session_denial_hash_sha256" \
  --arg dry_run_execution_result_receipt_replay_idempotency_handoff_hash_sha256 "$dry_run_execution_result_receipt_replay_idempotency_handoff_hash_sha256" \
  --arg dry_run_execution_result_receipt_replay_idempotency_result_hash_sha256 "$dry_run_execution_result_receipt_replay_idempotency_result_hash_sha256" \
  --arg scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_hash_sha256 "$scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_hash_sha256" \
  --arg scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_policy_hash_sha256 "$scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_policy_hash_sha256" \
  --argjson source "$SOURCE_JSON" \
  '
  def zero_fields($keys): reduce $keys[] as $k ({}; .[$k]=false | .[$k + "_count"]=0);
  def true_count_fields($keys): reduce $keys[] as $k ({}; .[$k]=true | .[$k + "_count"]=1);
  ([
    "source_dry_run_execution_result_receipt_boundary_required",
    "source_dry_run_execution_result_receipt_result_required",
    "dry_run_execution_result_receipt_replay_idempotency_denial_matrix_required",
    "dry_run_execution_result_receipt_replay_idempotency_identity_session_required",
    "dry_run_execution_result_receipt_replay_idempotency_nonce_scope_required",
    "dry_run_execution_result_receipt_replay_idempotency_duplicate_receipt_denial_required",
    "dry_run_execution_result_receipt_replay_idempotency_stale_receipt_denial_required",
    "dry_run_execution_result_receipt_replay_idempotency_hash_chain_mismatch_denial_required",
    "dry_run_execution_result_receipt_replay_idempotency_cross_session_denial_required",
    "dry_run_execution_result_receipt_replay_idempotency_handoff_required",
    "dry_run_execution_result_receipt_replay_state_persistence_forbidden",
    "dry_run_execution_result_receipt_idempotency_ledger_write_forbidden",
    "dry_run_execution_execution_forbidden_on_replay_idempotency_route",
    "dry_run_execution_result_receipt_persistence_forbidden_on_replay_idempotency_route",
    "production_write_execution_forbidden_on_replay_idempotency_route",
    "kg_provider_channel_release_install_active_binary_forbidden"
  ]) as $surfaces
  | ([
    "source_dry_run_execution_result_receipt_boundary_required",
    "source_dry_run_execution_result_receipt_result_hash_required",
    "source_dry_run_execution_result_receipt_policy_hash_required",
    "approved_production_namespace_required",
    "approved_production_store_required",
    "approved_production_scope_required",
    "production_durable_memory_target_required",
    "source_dry_run_execution_result_receipt_envelope_required",
    "source_dry_run_execution_result_receipt_digest_required",
    "source_dry_run_execution_result_receipt_hash_chain_required",
    "source_dry_run_execution_result_receipt_readback_plan_required",
    "source_dry_run_execution_result_receipt_replay_guard_required",
    "source_dry_run_execution_result_receipt_handoff_required",
    "replay_idempotency_denial_matrix_required",
    "replay_idempotency_identity_session_required",
    "replay_idempotency_nonce_scope_required",
    "replay_idempotency_duplicate_receipt_denial_required",
    "replay_idempotency_stale_receipt_denial_required",
    "replay_idempotency_hash_chain_mismatch_denial_required",
    "replay_idempotency_cross_session_denial_required",
    "replay_idempotency_handoff_required",
    "replay_state_persistence_denied",
    "idempotency_ledger_write_denied",
    "replay_guard_state_recording_denied",
    "duplicate_receipt_acceptance_denied",
    "stale_receipt_acceptance_denied",
    "cross_session_replay_acceptance_denied",
    "hash_chain_mismatch_acceptance_denied",
    "result_receipt_replay_attempt_denied",
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
    "dry_run_execution_result_receipt_replay_state_persisted",
    "dry_run_execution_result_receipt_idempotency_ledger_written",
    "dry_run_execution_result_receipt_replay_guard_state_recorded",
    "dry_run_execution_result_receipt_duplicate_receipt_accepted",
    "dry_run_execution_result_receipt_stale_receipt_accepted",
    "dry_run_execution_result_receipt_cross_session_replay_accepted",
    "dry_run_execution_result_receipt_hash_chain_mismatch_accepted",
    "dry_run_execution_result_receipt_replay_attempt_accepted",
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
    "raw_payload_plaintext_recorded",
    "raw_payload_plaintext_persisted",
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
    "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_performed",
    "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_result_recorded",
    "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_result_accepted",
    "source_dry_run_execution_result_receipt_boundary_accepted",
    "dry_run_execution_result_receipt_replay_idempotency_denial_matrix_bound",
    "dry_run_execution_result_receipt_replay_idempotency_identity_session_bound",
    "dry_run_execution_result_receipt_replay_idempotency_nonce_scope_bound",
    "dry_run_execution_result_receipt_replay_idempotency_duplicate_receipt_denied",
    "dry_run_execution_result_receipt_replay_idempotency_stale_receipt_denied",
    "dry_run_execution_result_receipt_replay_idempotency_hash_chain_mismatch_denied",
    "dry_run_execution_result_receipt_replay_idempotency_cross_session_denied",
    "dry_run_execution_result_receipt_replay_idempotency_handoff_bound"
  ]) as $true_keys
  | ({
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      source_command:$source_command,
      memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_ready:true,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_ready:true,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_performed:true,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_accepted:true,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_mode:"dry_run_execution_result_receipt_replay_idempotency_denial_boundary_no_replay_state_persistence_no_execution_no_production_durable_memory_mutation",
      source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_ready:true,
      source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_report_sha256:$source_report_sha256,
      source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_accepted_count:1,
      source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_fixture_count:1,
      source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_fixture_count:9,
      approved_production_namespace:$approved_production_namespace,
      approved_production_store:$approved_production_store,
      approved_production_scope:$approved_production_scope,
      production_durable_memory_target_id:$production_durable_memory_target_id,
      production_durable_memory_payload_class:$production_durable_memory_payload_class,
      operator_packet_scope:$operator_packet_scope,
      source_dry_run_execution_result_hash_sha256:$source_dry_run_execution_result_hash_sha256,
      source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_hash_sha256:$source_result_receipt_boundary_hash_sha256,
      source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_policy_hash_sha256:$source_result_receipt_policy_hash_sha256,
      source_dry_run_execution_result_receipt_envelope_hash_sha256:$source_result_receipt_envelope_hash_sha256,
      source_dry_run_execution_result_receipt_identity_session_hash_sha256:$source_result_receipt_identity_session_hash_sha256,
      source_dry_run_execution_result_receipt_digest_hash_sha256:$source_result_receipt_digest_hash_sha256,
      source_dry_run_execution_result_receipt_hash_chain_hash_sha256:$source_result_receipt_hash_chain_hash_sha256,
      source_dry_run_execution_result_receipt_readback_plan_hash_sha256:$source_result_receipt_readback_plan_hash_sha256,
      source_dry_run_execution_result_receipt_replay_guard_hash_sha256:$source_result_receipt_replay_guard_hash_sha256,
      source_dry_run_execution_result_receipt_handoff_hash_sha256:$source_result_receipt_handoff_hash_sha256,
      source_dry_run_execution_result_receipt_result_hash_sha256:$source_result_receipt_result_hash_sha256,
      dry_run_execution_result_receipt_replay_idempotency_denial_matrix_hash_sha256:$dry_run_execution_result_receipt_replay_idempotency_denial_matrix_hash_sha256,
      dry_run_execution_result_receipt_replay_idempotency_identity_session_hash_sha256:$dry_run_execution_result_receipt_replay_idempotency_identity_session_hash_sha256,
      dry_run_execution_result_receipt_replay_idempotency_nonce_scope_hash_sha256:$dry_run_execution_result_receipt_replay_idempotency_nonce_scope_hash_sha256,
      dry_run_execution_result_receipt_replay_idempotency_duplicate_receipt_denial_hash_sha256:$dry_run_execution_result_receipt_replay_idempotency_duplicate_receipt_denial_hash_sha256,
      dry_run_execution_result_receipt_replay_idempotency_stale_receipt_denial_hash_sha256:$dry_run_execution_result_receipt_replay_idempotency_stale_receipt_denial_hash_sha256,
      dry_run_execution_result_receipt_replay_idempotency_hash_chain_mismatch_denial_hash_sha256:$dry_run_execution_result_receipt_replay_idempotency_hash_chain_mismatch_denial_hash_sha256,
      dry_run_execution_result_receipt_replay_idempotency_cross_session_denial_hash_sha256:$dry_run_execution_result_receipt_replay_idempotency_cross_session_denial_hash_sha256,
      dry_run_execution_result_receipt_replay_idempotency_handoff_hash_sha256:$dry_run_execution_result_receipt_replay_idempotency_handoff_hash_sha256,
      dry_run_execution_result_receipt_replay_idempotency_result_hash_sha256:$dry_run_execution_result_receipt_replay_idempotency_result_hash_sha256,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_hash_sha256:$scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_hash_sha256,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_policy_hash_sha256:$scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_policy_hash_sha256,
      required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_surface_count:($surfaces | length),
      ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_surface_count:($surfaces | length),
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_surfaces:$surfaces,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixture_count:10,
      accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixture_count:1,
      blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixture_count:9,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixtures:[
        {
          id:"scoped-production-durable-memory-write-dry-run-execution-result-receipt-replay-idempotency-denial",
          fixture_id:"scoped-production-durable-memory-write-dry-run-execution-result-receipt-replay-idempotency-denial",
          scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_accepted:true
        },
        {id:"missing-result-receipt-source", fixture_id:"missing-result-receipt-source", scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_accepted:false},
        {id:"missing-result-receipt-result-hash", fixture_id:"missing-result-receipt-result-hash", scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_accepted:false},
        {id:"missing-replay-idempotency-denial-matrix", fixture_id:"missing-replay-idempotency-denial-matrix", scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_accepted:false},
        {id:"duplicate-result-receipt-attempt", fixture_id:"duplicate-result-receipt-attempt", scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_accepted:false},
        {id:"stale-result-receipt-attempt", fixture_id:"stale-result-receipt-attempt", scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_accepted:false},
        {id:"cross-session-result-receipt-replay-attempt", fixture_id:"cross-session-result-receipt-replay-attempt", scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_accepted:false},
        {id:"hash-chain-mismatch-result-receipt-attempt", fixture_id:"hash-chain-mismatch-result-receipt-attempt", scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_accepted:false},
        {id:"replay-state-persistence-or-idempotency-ledger-write-attempt", fixture_id:"replay-state-persistence-or-idempotency-ledger-write-attempt", scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_accepted:false},
        {id:"dry-run-execution-or-production-write-attempt", fixture_id:"dry-run-execution-or-production-write-attempt", scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_accepted:false}
      ],
      denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary:$denials,
      denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_count:($denials | length),
      allowed_next_actions:[
        {
          action:"run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_require_live_gate",
          status:"allowed_verification_only",
          accepts_replay_idempotency_denial_matrix:true,
          persists_replay_state:false,
          writes_idempotency_ledger:false,
          executes_dry_run:false,
          persists_dry_run_result_receipt:false,
          writes_production_durable_memory:false,
          writes_memory_store:false,
          writes_wal:false,
          persists_receipt:false
        },
        {
          action:"prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary",
          status:"requires_separate_result_receipt_ordering_monotonicity_denial_gate",
          requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary:true,
          persists_replay_state:false,
          writes_idempotency_ledger:false,
          executes_dry_run:false,
          writes_production_durable_memory:false,
          persists_dry_run_result_receipt:false
        }
      ],
      source_dry_run_execution_result_receipt_boundary_bound:true,
      approved_production_namespace_bound:true,
      approved_production_store_bound:true,
      approved_production_scope_bound:true,
      production_durable_memory_target_bound:true,
      dry_run_execution_result_receipt_envelope_bound:true,
      dry_run_execution_result_receipt_digest_bound:true,
      dry_run_execution_result_receipt_hash_chain_bound:true,
      dry_run_execution_result_receipt_readback_plan_bound:true,
      dry_run_execution_result_receipt_replay_guard_bound:true,
      dry_run_execution_result_receipt_handoff_bound:true,
      dry_run_execution_result_receipt_result_bound:true,
      dry_run_execution_result_receipt_replay_state_persistence_forbidden:true,
      dry_run_execution_result_receipt_idempotency_ledger_write_forbidden:true,
      dry_run_execution_execution_forbidden_on_replay_idempotency_route:true,
      dry_run_execution_result_receipt_persistence_forbidden_on_replay_idempotency_route:true,
      production_write_execution_forbidden_on_replay_idempotency_route:true,
      production_durable_memory_write_forbidden:true,
      memory_store_mutation_forbidden:true,
      kg_live_write_forbidden:true,
      provider_model_invocation_forbidden:true,
      credential_channel_public_release_forbidden:true,
      install_restart_active_binary_mutation_forbidden:true
    }
    + zero_fields($false_keys)
    + true_count_fields($true_keys)
    + {
      side_effects:(zero_fields($false_keys) + true_count_fields($true_keys))
    })
  '
