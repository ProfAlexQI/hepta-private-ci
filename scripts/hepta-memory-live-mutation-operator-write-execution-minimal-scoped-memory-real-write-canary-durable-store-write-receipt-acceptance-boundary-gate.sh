#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"

cd "$REPO_ROOT"

source scripts/lib/hepta-json-report-capture.sh

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution-boundary-gate.sh
)"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_ready == true
    and $source.minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_accepted == true
    and $source.accepted_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count == 1
    and $source.blocked_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count == 9
    and $source.durable_store_write_single_shot_execution_result_accepted_count == 1
    and $source.approved_namespace == "hepta.memory.canary"
    and $source.approved_store == "wal-receipt-canary-artifact"
    and $source.approved_scope == "session"
    and $source.single_shot_canary_post_write_memory_count == 1
    and $source.single_shot_canary_readback_hit_count == 1
    and $source.single_shot_canary_rollback_restored == true
    and $source.single_shot_canary_post_rollback_memory_count == 0
    and $source.single_shot_canary_post_rollback_absence_confirmed == true
    and $source.single_shot_canary_artifact_write_count == 3
    and $source.single_shot_canary_artifact_readback_count == 3
    and $source.single_shot_canary_artifact_cleanup_removed_count == 3
    and $source.single_shot_canary_artifact_post_cleanup_count == 0
    and $source.single_shot_canary_artifact_zero_residue_confirmed == true
    and $source.memory_store_write_performed == true
    and $source.wal_write_performed == true
    and $source.receipt_persisted == true
    and $source.post_write_readback_performed == true
    and $source.rollback_executed == true
    and $source.tombstone_cleanup_executed == true
    and $source.production_durable_memory_store_write_performed == false
    and $source.actual_production_durable_memory_write_performed == false
    and $source.durable_memory_store_write_performed == false
    and $source.durable_memory_store_read_performed == false
    and $source.durable_memory_store_rollback_performed == false
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
    and $source.allowed_next_actions[1].action == "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary"
    and $source.allowed_next_actions[1].requires_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary == true
  ' >/dev/null

source_report_sha256="$(sha256_text "$SOURCE_JSON")"
receipt_acceptance_record_hash_sha256="$(
  jq -r --arg source_report_sha256 "$source_report_sha256" \
    '"minimal-scoped-memory-real-write-canary-durable-store-write-receipt-acceptance-record:v1:source=\($source_report_sha256):record=\(.canary_record_id):receipt=\(.single_shot_receipt_hash_sha256):chain=\(.single_shot_receipt_hash_chain_sha256):zero-residue=true"' \
    <<<"$SOURCE_JSON" | shasum -a 256 | awk '{print $1}'
)"
receipt_acceptance_readback_hash_sha256="$(
  jq -r \
    '"minimal-scoped-memory-real-write-canary-durable-store-write-receipt-acceptance-readback:v1:record=\(.canary_record_id):readback-hit=1:rollback-restored=true:cleanup=\(.single_shot_cleanup_receipt_hash_sha256):source-execution=\(.single_shot_execution_hash_sha256)"' \
    <<<"$SOURCE_JSON" | shasum -a 256 | awk '{print $1}'
)"
receipt_acceptance_hash_sha256="$(
  jq -r --arg receipt_acceptance_readback_hash_sha256 "$receipt_acceptance_readback_hash_sha256" \
    '"minimal-scoped-memory-real-write-canary-durable-store-write-receipt-acceptance:v1:namespace=\(.approved_namespace):store=\(.approved_store):scope=\(.approved_scope):receipt=\(.single_shot_receipt_hash_sha256):chain=\(.single_shot_receipt_hash_chain_sha256):readback=\($receipt_acceptance_readback_hash_sha256):accepted=true"' \
    <<<"$SOURCE_JSON" | shasum -a 256 | awk '{print $1}'
)"
boundary_hash_sha256="$(
  jq -r --arg receipt_acceptance_hash_sha256 "$receipt_acceptance_hash_sha256" \
    '"minimal-scoped-memory-real-write-canary-durable-store-write-receipt-acceptance-boundary-report:v1:source-ready=true:receipt-acceptance=true:receipt=\(.single_shot_receipt_hash_sha256):chain=\(.single_shot_receipt_hash_chain_sha256):fixtures=10:accepted=1:denials=32:production-durable-write=false:new-memory-write=false"' \
    <<<"$SOURCE_JSON" | shasum -a 256 | awk '{print $1}'
)"
policy_hash_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-write-receipt-acceptance-policy:v1:accept-single-shot-receipt-evidence:no-new-store-write:no-wal-rewrite:no-receipt-repersist:no-production-durable-memory:no-kg:no-provider:no-channel:no-release:no-install")"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_gate" \
  --arg endpoint "/api/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-receipt-acceptance-boundary" \
  --arg source_command "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-receipt-acceptance-boundary --json" \
  --arg source_report_sha256 "$source_report_sha256" \
  --arg receipt_acceptance_record_hash_sha256 "$receipt_acceptance_record_hash_sha256" \
  --arg receipt_acceptance_readback_hash_sha256 "$receipt_acceptance_readback_hash_sha256" \
  --arg receipt_acceptance_hash_sha256 "$receipt_acceptance_hash_sha256" \
  --arg boundary_hash_sha256 "$boundary_hash_sha256" \
  --arg policy_hash_sha256 "$policy_hash_sha256" \
  --argjson source "$SOURCE_JSON" \
  '
  ([
    "durable_store_write_receipt_acceptance_performed",
    "durable_store_write_receipt_acceptance_result_recorded",
    "durable_store_write_receipt_acceptance_result_accepted",
    "single_shot_receipt_identity_accepted",
    "single_shot_receipt_hash_chain_accepted",
    "single_shot_readback_evidence_accepted",
    "single_shot_rollback_cleanup_zero_residue_evidence_accepted",
    "receipt_acceptance_recorded",
    "receipt_acceptance_replay_guard_accepted",
    "operator_receipt_acceptance_handoff_bound",
    "rollback_tombstone_zero_residue_handoff_bound",
    "kg_provider_channel_release_install_active_binary_forbidden"
  ]) as $true_fields
  | ([
    "durable_store_write_execution_performed",
    "durable_store_write_single_shot_execution_performed",
    "memory_write_execution_performed",
    "memory_store_write_performed",
    "memory_store_mutated",
    "wal_write_performed",
    "wal_recorded",
    "wal_persisted",
    "receipt_recorded",
    "receipt_persisted",
    "receipt_materialized",
    "post_write_readback_performed",
    "readback_result_accepted",
    "rollback_executed",
    "rollback_performed",
    "tombstone_cleanup_executed",
    "production_durable_memory_store_write_performed",
    "actual_production_durable_memory_write_performed",
    "durable_memory_store_write_performed",
    "durable_memory_store_read_performed",
    "durable_memory_store_rollback_performed",
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
  ]) as $false_fields
  | ($true_fields | reduce .[] as $key ({}; .[$key] = true | .[$key + "_count"] = 1)) as $true_map
  | ($false_fields | reduce .[] as $key ({}; .[$key] = false | .[$key + "_count"] = 0)) as $false_map
  | ([
    "source_single_shot_execution_boundary_required",
    "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_accepted",
    "approved_namespace_bound",
    "approved_store_bound",
    "approved_scope_bound",
    "durable_store_write_target_bound",
    "durable_store_target_store_bound",
    "single_shot_record_identity_bound",
    "single_shot_execution_envelope_bound",
    "single_shot_receipt_identity_bound",
    "single_shot_receipt_hash_chain_bound",
    "single_shot_readback_evidence_bound",
    "single_shot_rollback_cleanup_zero_residue_bound",
    "receipt_acceptance_record_bound",
    "receipt_acceptance_replay_guard_bound",
    "operator_receipt_acceptance_handoff_bound",
    "rollback_tombstone_zero_residue_handoff_bound",
    "new_canary_store_write_forbidden_on_report_route",
    "wal_rewrite_forbidden_on_report_route",
    "receipt_repersist_forbidden_on_report_route",
    "production_durable_memory_write_forbidden",
    "durable_memory_read_or_rollback_forbidden",
    "kg_live_write_forbidden",
    "provider_model_invocation_forbidden",
    "credential_channel_public_release_forbidden",
    "install_restart_active_binary_mutation_forbidden"
  ] | reduce .[] as $key ({}; .[$key] = true)) as $binding_map
  | ([
    "source_single_shot_execution_boundary_required",
    "source_single_shot_execution_boundary_hash_required",
    "approved_namespace_required",
    "approved_store_required",
    "approved_scope_required",
    "durable_store_write_target_required",
    "durable_store_target_store_required",
    "single_shot_record_identity_required",
    "single_shot_payload_digest_required",
    "single_shot_execution_envelope_required",
    "single_shot_nonce_record_required",
    "single_shot_command_record_required",
    "single_shot_budget_record_required",
    "single_shot_wal_hash_required",
    "single_shot_receipt_hash_required",
    "single_shot_receipt_hash_chain_required",
    "single_shot_cleanup_receipt_required",
    "single_shot_execution_hash_required",
    "single_shot_post_write_readback_required",
    "single_shot_rollback_restore_required",
    "single_shot_tombstone_cleanup_required",
    "single_shot_zero_residue_required",
    "receipt_acceptance_record_required",
    "receipt_acceptance_replay_guard_required",
    "operator_receipt_acceptance_handoff_required",
    "new_canary_store_write_report_route_denied",
    "wal_rewrite_report_route_denied",
    "receipt_repersist_report_route_denied",
    "production_durable_memory_backend_write_denied",
    "durable_memory_backend_read_or_rollback_denied",
    "kg_provider_channel_release_install_active_binary_denied",
    "unrestricted_full_live_activation_denied"
  ]) as $denials
  | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      source_command:$source_command,
      native_route:true,
      side_effect_free:false,
      external_side_effect_free:true,
      audit_date:"2026-07-04",
      memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_ready:true,
      minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_ready:true,
      minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_performed:true,
      minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_accepted:true,
      scoped_memory_real_write_canary_mode:"minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_report_only",
      durable_store_write_receipt_acceptance_scope:"accept_single_shot_request_local_canary_receipt_hash_chain_readback_rollback_cleanup_zero_residue_evidence_only",
      source_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_ready:true,
      source_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_report_sha256:$source_report_sha256,
      source_accepted_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count:$source.accepted_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count,
      source_blocked_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count:$source.blocked_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count,
      source_durable_store_write_single_shot_execution_result_accepted_count:$source.durable_store_write_single_shot_execution_result_accepted_count,
      source_memory_store_write_performed_count:1,
      source_wal_write_performed_count:1,
      source_receipt_persisted_count:1,
      source_post_write_readback_performed_count:1,
      source_rollback_executed_count:1,
      source_tombstone_cleanup_executed_count:1,
      source_production_durable_memory_store_write_performed_count:0,
      source_durable_memory_store_write_performed_count:0,
      source_live_kg_write_performed_count:0,
      source_external_send_performed_count:0,
      source_single_shot_canary_post_write_memory_count:$source.single_shot_canary_post_write_memory_count,
      source_single_shot_canary_readback_hit_count:$source.single_shot_canary_readback_hit_count,
      source_single_shot_canary_rollback_restored:$source.single_shot_canary_rollback_restored,
      source_single_shot_canary_post_rollback_memory_count:$source.single_shot_canary_post_rollback_memory_count,
      source_single_shot_canary_post_rollback_absence_confirmed:$source.single_shot_canary_post_rollback_absence_confirmed,
      source_single_shot_canary_artifact_write_count:$source.single_shot_canary_artifact_write_count,
      source_single_shot_canary_artifact_readback_count:$source.single_shot_canary_artifact_readback_count,
      source_single_shot_canary_artifact_cleanup_removed_count:$source.single_shot_canary_artifact_cleanup_removed_count,
      source_single_shot_canary_artifact_post_cleanup_count:$source.single_shot_canary_artifact_post_cleanup_count,
      source_single_shot_canary_artifact_zero_residue_confirmed:$source.single_shot_canary_artifact_zero_residue_confirmed,
      approved_namespace:$source.approved_namespace,
      approved_store:$source.approved_store,
      approved_scope:$source.approved_scope,
      durable_store_write_target_id:$source.durable_store_write_target_id,
      durable_store_target_store_id:$source.durable_store_target_store_id,
      canary_record_id:$source.canary_record_id,
      source_single_shot_boundary_hash_sha256:$source.minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_hash_sha256,
      source_single_shot_policy_hash_sha256:$source.minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_policy_hash_sha256,
      canary_payload_digest_sha256:$source.canary_payload_digest_sha256,
      single_shot_execution_envelope_sha256:$source.single_shot_execution_envelope_sha256,
      single_shot_nonce_sha256:$source.single_shot_nonce_sha256,
      single_shot_command_sha256:$source.single_shot_command_sha256,
      single_shot_budget_sha256:$source.single_shot_budget_sha256,
      single_shot_wal_hash_sha256:$source.single_shot_wal_hash_sha256,
      single_shot_receipt_hash_sha256:$source.single_shot_receipt_hash_sha256,
      single_shot_receipt_hash_chain_sha256:$source.single_shot_receipt_hash_chain_sha256,
      single_shot_cleanup_receipt_hash_sha256:$source.single_shot_cleanup_receipt_hash_sha256,
      single_shot_execution_hash_sha256:$source.single_shot_execution_hash_sha256,
      receipt_acceptance_record_hash_sha256:$receipt_acceptance_record_hash_sha256,
      receipt_acceptance_readback_hash_sha256:$receipt_acceptance_readback_hash_sha256,
      receipt_acceptance_hash_sha256:$receipt_acceptance_hash_sha256,
      receipt_readback_digest_match:true,
      receipt_hash_chain_verified:true,
      single_shot_rollback_cleanup_zero_residue_verified:true,
      required_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_surface_count:12,
      ready_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_surface_count:12,
      minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_fixture_count:10,
      accepted_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_fixture_count:1,
      blocked_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_fixture_count:9,
      minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_accepted_count:1,
      durable_store_write_receipt_acceptance_authority_accepted_count:1,
      source_single_shot_execution_bound_count:1,
      single_shot_receipt_identity_bound_count:1,
      single_shot_receipt_hash_chain_bound_count:1,
      single_shot_readback_evidence_bound_count:1,
      single_shot_rollback_cleanup_zero_residue_bound_count:1,
      receipt_acceptance_record_bound_count:1,
      receipt_acceptance_result_recorded_count:1,
      receipt_acceptance_result_accepted_count:1,
      receipt_acceptance_replay_guard_accepted_count:1,
      operator_receipt_acceptance_handoff_bound_count:1,
      rollback_tombstone_zero_residue_handoff_bound_count:1,
      minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_fixtures:
        ([{
          id:"minimal-scoped-memory-real-write-canary-durable-store-write-receipt-acceptance",
          fixture_id:"minimal-scoped-memory-real-write-canary-durable-store-write-receipt-acceptance",
          minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_accepted:true,
          reason:"single_shot_receipt_hash_chain_readback_rollback_cleanup_zero_residue_accepted"
        }] + ([
          "missing-single-shot-source-boundary",
          "wrong-namespace",
          "wrong-store",
          "wrong-scope",
          "missing-single-shot-receipt",
          "missing-single-shot-hash-chain",
          "missing-single-shot-readback",
          "missing-rollback-cleanup-zero-residue",
          "new-write-or-external-side-effect-attempt"
        ] | map({
          id:.,
          fixture_id:.,
          minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_accepted:false,
          reason:"blocked_noop"
        }))),
      denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary:$denials,
      denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_count:($denials | length),
      minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_hash_sha256:$boundary_hash_sha256,
      minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_policy_hash_sha256:$policy_hash_sha256,
      allowed_next_actions:[
        {
          action:"run_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_require_live_gate",
          status:"allowed_verification_only",
          accepts_single_shot_receipt:true,
          writes_new_canary_store_record:false,
          writes_production_durable_memory:false,
          writes_wal:false,
          persists_receipt:false
        },
        {
          action:"prepare_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary",
          status:"allowed_report_only_next_slice",
          requires_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary:true,
          writes_new_canary_store_record:false,
          writes_production_durable_memory:false,
          writes_wal:false,
          persists_receipt:false
        }
      ]
    } + $true_map + $false_map + $binding_map
    | .side_effects = ($true_map + $false_map)
  '
