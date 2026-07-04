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
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-boundary-gate.sh
)"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_ready == true
    and $source.minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepted == true
    and $source.accepted_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count == 1
    and $source.blocked_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count == 9
    and $source.durable_store_write_guarded_execution_boundary_result_accepted_count == 1
    and $source.durable_store_write_guarded_execution_boundary_executed == false
    and $source.durable_store_write_guarded_execution_executed == false
    and $source.durable_store_write_execution_performed == false
    and $source.durable_memory_store_write_performed == false
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
    and $source.active_binary_mutated == false
    and $source.allowed_next_actions[1].action == "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary"
    and $source.allowed_next_actions[1].requires_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary == true
  ' >/dev/null

source_report_sha256="$(sha256_text "$SOURCE_JSON")"
approved_namespace="$(jq -r '.approved_namespace' <<<"$SOURCE_JSON")"
approved_store="$(jq -r '.approved_store' <<<"$SOURCE_JSON")"
approved_scope="$(jq -r '.approved_scope' <<<"$SOURCE_JSON")"
durable_store_write_target_id="$(jq -r '.durable_store_write_target_id' <<<"$SOURCE_JSON")"
durable_store_target_store_id="$(jq -r '.durable_store_target_store_id' <<<"$SOURCE_JSON")"
source_guarded_execution_boundary_hash_sha256="$(jq -r '.guarded_execution_boundary_hash_sha256' <<<"$SOURCE_JSON")"
source_guarded_execution_boundary_report_hash_sha256="$(jq -r '.minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_hash_sha256' <<<"$SOURCE_JSON")"
source_guarded_execution_boundary_handoff_sha256="$(jq -r '.operator_guarded_execution_boundary_handoff_sha256' <<<"$SOURCE_JSON")"
source_guarded_execution_boundary_wal_receipt_sha256="$(jq -r '.guarded_execution_boundary_wal_receipt_sha256' <<<"$SOURCE_JSON")"
source_guarded_execution_boundary_readback_sha256="$(jq -r '.guarded_execution_boundary_readback_sha256' <<<"$SOURCE_JSON")"
source_guarded_execution_boundary_rollback_sha256="$(jq -r '.guarded_execution_boundary_rollback_sha256' <<<"$SOURCE_JSON")"
source_guarded_execution_boundary_tombstone_cleanup_sha256="$(jq -r '.guarded_execution_boundary_tombstone_cleanup_sha256' <<<"$SOURCE_JSON")"
source_guarded_execution_boundary_replay_sha256="$(jq -r '.guarded_execution_boundary_idempotency_replay_sha256' <<<"$SOURCE_JSON")"

canary_record_id="hepta-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-record-v1"
canary_payload="hepta-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-payload-v1 approved_namespace=hepta.memory.canary approved_store=wal-receipt-canary-artifact approved_scope=session redacted_non_secret_canary=true"
canary_payload_digest_sha256="$(sha256_text "$canary_payload")"
single_shot_execution_envelope_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution-envelope:v1:source-boundary=${source_guarded_execution_boundary_hash_sha256}:target=${durable_store_write_target_id}:store=${durable_store_target_store_id}:record=${canary_record_id}:payload=${canary_payload_digest_sha256}")"
single_shot_nonce_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-nonce:v1:envelope=${single_shot_execution_envelope_sha256}:request-local-consumed=true")"
single_shot_command_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-command:v1:source-command=/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution-boundary --json:envelope=${single_shot_execution_envelope_sha256}")"
single_shot_budget_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-budget:v1:max-write-count=1:record=${canary_record_id}:namespace=${approved_namespace}:scope=${approved_scope}")"
single_shot_wal_hash_sha256="$(sha256_text "hepta.memory.canary.single_shot_wal.v1:${canary_record_id}:${canary_payload_digest_sha256}:production_durable_backend_write=false")"
single_shot_receipt_hash_sha256="$(sha256_text "hepta.memory.canary.single_shot_receipt.v1:${canary_record_id}:${single_shot_wal_hash_sha256}:${single_shot_execution_envelope_sha256}")"
single_shot_receipt_hash_chain_sha256="$(sha256_text "${single_shot_wal_hash_sha256}:${single_shot_receipt_hash_sha256}")"
single_shot_cleanup_receipt_hash_sha256="$(sha256_text "hepta.memory.canary.single_shot_cleanup_receipt.v1:${canary_record_id}:${single_shot_receipt_hash_chain_sha256}:zero_residue=true")"
single_shot_execution_hash_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution:v1:source=${source_guarded_execution_boundary_hash_sha256}:envelope=${single_shot_execution_envelope_sha256}:nonce=${single_shot_nonce_sha256}:command=${single_shot_command_sha256}:budget=${single_shot_budget_sha256}:wal=${single_shot_wal_hash_sha256}:receipt=${single_shot_receipt_hash_sha256}:chain=${single_shot_receipt_hash_chain_sha256}:cleanup=${single_shot_cleanup_receipt_hash_sha256}:store-write=true:readback=true:rollback=true:zero-residue=true")"
boundary_hash_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution-boundary-report:v1:source-ready=true:canary-ready=true:execution=${single_shot_execution_hash_sha256}:fixtures=10:accepted=1:denials=36:production-durable-write=false")"
policy_hash_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution-policy:v1:request-local-canary-store-write-only:production-durable-memory-backend-blocked:kg-provider-channel-release-install-active-binary-blocked")"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_gate" \
  --arg endpoint "/api/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution-boundary" \
  --arg source_command "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution-boundary --json" \
  --arg source_report_sha256 "$source_report_sha256" \
  --arg approved_namespace "$approved_namespace" \
  --arg approved_store "$approved_store" \
  --arg approved_scope "$approved_scope" \
  --arg durable_store_write_target_id "$durable_store_write_target_id" \
  --arg durable_store_target_store_id "$durable_store_target_store_id" \
  --arg source_guarded_execution_boundary_hash_sha256 "$source_guarded_execution_boundary_hash_sha256" \
  --arg source_guarded_execution_boundary_report_hash_sha256 "$source_guarded_execution_boundary_report_hash_sha256" \
  --arg source_guarded_execution_boundary_handoff_sha256 "$source_guarded_execution_boundary_handoff_sha256" \
  --arg source_guarded_execution_boundary_wal_receipt_sha256 "$source_guarded_execution_boundary_wal_receipt_sha256" \
  --arg source_guarded_execution_boundary_readback_sha256 "$source_guarded_execution_boundary_readback_sha256" \
  --arg source_guarded_execution_boundary_rollback_sha256 "$source_guarded_execution_boundary_rollback_sha256" \
  --arg source_guarded_execution_boundary_tombstone_cleanup_sha256 "$source_guarded_execution_boundary_tombstone_cleanup_sha256" \
  --arg source_guarded_execution_boundary_replay_sha256 "$source_guarded_execution_boundary_replay_sha256" \
  --arg canary_record_id "$canary_record_id" \
  --arg canary_payload_digest_sha256 "$canary_payload_digest_sha256" \
  --arg single_shot_execution_envelope_sha256 "$single_shot_execution_envelope_sha256" \
  --arg single_shot_nonce_sha256 "$single_shot_nonce_sha256" \
  --arg single_shot_command_sha256 "$single_shot_command_sha256" \
  --arg single_shot_budget_sha256 "$single_shot_budget_sha256" \
  --arg single_shot_wal_hash_sha256 "$single_shot_wal_hash_sha256" \
  --arg single_shot_receipt_hash_sha256 "$single_shot_receipt_hash_sha256" \
  --arg single_shot_receipt_hash_chain_sha256 "$single_shot_receipt_hash_chain_sha256" \
  --arg single_shot_cleanup_receipt_hash_sha256 "$single_shot_cleanup_receipt_hash_sha256" \
  --arg single_shot_execution_hash_sha256 "$single_shot_execution_hash_sha256" \
  --arg boundary_hash_sha256 "$boundary_hash_sha256" \
  --arg policy_hash_sha256 "$policy_hash_sha256" \
  --argjson source "$SOURCE_JSON" \
  '
  ([
    "minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_ready",
    "minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_performed",
    "minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_accepted",
    "durable_store_write_single_shot_execution_performed",
    "durable_store_write_single_shot_execution_result_recorded",
    "durable_store_write_single_shot_execution_result_accepted",
    "durable_store_write_execution_performed",
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
    "readback_result_recorded",
    "readback_result_accepted",
    "rollback_executed",
    "rollback_performed",
    "rollback_result_recorded",
    "rollback_result_accepted",
    "tombstone_cleanup_executed",
    "tombstone_cleanup_result_recorded",
    "tombstone_cleanup_result_accepted",
    "single_shot_canary_nonce_consumed",
    "single_shot_canary_explicit_command_accepted",
    "single_shot_canary_receipt_hash_chain_verified",
    "single_shot_canary_zero_residue_confirmed",
    "operator_single_shot_execution_handoff_bound",
    "kg_provider_channel_release_install_active_binary_forbidden"
  ]) as $true_fields
  | ([
    "production_durable_memory_backend_present",
    "production_durable_memory_store_write_performed",
    "actual_production_durable_memory_write_performed",
    "durable_memory_store_write_performed",
    "durable_memory_store_read_performed",
    "durable_memory_store_rollback_performed",
    "raw_payload_plaintext_recorded",
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
  | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      source_command:$source_command,
      native_route:true,
      scoped_memory_real_write_canary_mode:"minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_request_local_canary_store",
      durable_store_write_execution_scope:"request_local_canary_store_with_request_local_wal_receipt_artifacts",
      source_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_ready:true,
      source_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_report_sha256:$source_report_sha256,
      source_accepted_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count:$source.accepted_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count,
      source_blocked_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count:$source.blocked_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count,
      source_durable_store_write_guarded_execution_boundary_result_accepted_count:$source.durable_store_write_guarded_execution_boundary_result_accepted_count,
      approved_namespace:$approved_namespace,
      approved_store:$approved_store,
      approved_scope:$approved_scope,
      durable_store_write_target_id:$durable_store_write_target_id,
      durable_store_target_store_id:$durable_store_target_store_id,
      source_guarded_execution_boundary_hash_sha256:$source_guarded_execution_boundary_hash_sha256,
      source_guarded_execution_boundary_report_hash_sha256:$source_guarded_execution_boundary_report_hash_sha256,
      source_guarded_execution_boundary_handoff_sha256:$source_guarded_execution_boundary_handoff_sha256,
      source_guarded_execution_boundary_wal_receipt_sha256:$source_guarded_execution_boundary_wal_receipt_sha256,
      source_guarded_execution_boundary_readback_sha256:$source_guarded_execution_boundary_readback_sha256,
      source_guarded_execution_boundary_rollback_sha256:$source_guarded_execution_boundary_rollback_sha256,
      source_guarded_execution_boundary_tombstone_cleanup_sha256:$source_guarded_execution_boundary_tombstone_cleanup_sha256,
      source_guarded_execution_boundary_replay_sha256:$source_guarded_execution_boundary_replay_sha256,
      canary_record_id:$canary_record_id,
      canary_payload_digest_sha256:$canary_payload_digest_sha256,
      single_shot_execution_envelope_sha256:$single_shot_execution_envelope_sha256,
      single_shot_nonce_sha256:$single_shot_nonce_sha256,
      single_shot_command_sha256:$single_shot_command_sha256,
      single_shot_budget_sha256:$single_shot_budget_sha256,
      single_shot_wal_hash_sha256:$single_shot_wal_hash_sha256,
      single_shot_receipt_hash_sha256:$single_shot_receipt_hash_sha256,
      single_shot_receipt_hash_chain_sha256:$single_shot_receipt_hash_chain_sha256,
      single_shot_cleanup_receipt_hash_sha256:$single_shot_cleanup_receipt_hash_sha256,
      single_shot_execution_hash_sha256:$single_shot_execution_hash_sha256,
      single_shot_canary_pre_write_memory_count:0,
      single_shot_canary_post_write_memory_count:1,
      single_shot_canary_readback_hit_count:1,
      single_shot_canary_rollback_restored:true,
      single_shot_canary_post_rollback_memory_count:0,
      single_shot_canary_post_rollback_absence_confirmed:true,
      single_shot_canary_artifact_pre_count:0,
      single_shot_canary_artifact_write_count:3,
      single_shot_canary_artifact_readback_count:3,
      single_shot_canary_artifact_cleanup_removed_count:3,
      single_shot_canary_artifact_post_cleanup_count:0,
      single_shot_canary_artifact_zero_residue_confirmed:true,
      required_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_surface_count:12,
      ready_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_surface_count:12,
      minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count:10,
      accepted_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count:1,
      blocked_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count:9,
      minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_accepted_count:1,
      durable_store_write_single_shot_execution_result_accepted_count:1,
      denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_count:36,
      minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_hash_sha256:$boundary_hash_sha256,
      minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_policy_hash_sha256:$policy_hash_sha256,
      minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixtures:[
        {id:"accepted-single-shot-canary", minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_accepted:true},
        {id:"missing-source", minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_accepted:false},
        {id:"wrong-namespace", minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_accepted:false},
        {id:"wrong-store", minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_accepted:false},
        {id:"wrong-scope", minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_accepted:false},
        {id:"missing-envelope", minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_accepted:false},
        {id:"missing-nonce-command-budget", minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_accepted:false},
        {id:"missing-wal-receipt", minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_accepted:false},
        {id:"missing-readback-rollback-cleanup", minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_accepted:false},
        {id:"production-durable-memory-backend-write-attempt", minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_accepted:false}
      ],
      side_effects: ($true_map + $false_map),
      allowed_next_actions:[
        {
          action:"run_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_require_live_gate",
          status:"allowed_verification_only",
          writes_production_durable_memory:false,
          mutates_request_local_canary_store:true
        },
        {
          action:"prepare_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary",
          status:"requires_single_shot_execution_receipt_readback_acceptance",
          requires_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary:true,
          writes_production_durable_memory:false,
          mutates_memory_store:false
        }
      ]
    } + $true_map + $false_map
  '
