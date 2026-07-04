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
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-preflight-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-preflight-boundary-gate.sh
)"

source_report_sha256="$(sha256_text "$SOURCE_JSON")"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.minimal_scoped_memory_real_write_canary_durable_store_write_preflight_ready == true
    and $source.minimal_scoped_memory_real_write_canary_durable_store_write_preflight_performed == true
    and $source.minimal_scoped_memory_real_write_canary_durable_store_write_preflight_accepted == true
    and $source.accepted_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixture_count == 1
    and $source.blocked_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixture_count == 9
    and $source.durable_store_write_preflight_result_accepted_count == 1
    and ($source.durable_store_write_preflight_executed_count // 0) == 0
    and ($source.durable_memory_store_write_performed_count // 0) == 0
    and ($source.memory_store_write_performed_count // 0) == 0
    and $source.approved_namespace == "hepta.memory.canary"
    and $source.approved_store == "wal-receipt-canary-artifact"
    and $source.approved_scope == "session"
    and $source.durable_store_write_target_id == "hepta-minimal-scoped-memory-real-write-canary-durable-store-write-target-v1"
    and $source.durable_store_target_store_id == "hepta-memory-durable-store-canary-plan-only"
    and ($source.durable_store_write_preflight_hash_sha256 | type == "string" and length > 0)
    and ($source.durable_store_write_preflight_operator_handoff_sha256 | type == "string" and length > 0)
    and ($source.source_durable_store_write_target_sha256 | type == "string" and length > 0)
    and ($source.source_durable_store_write_payload_digest_sha256 | type == "string" and length > 0)
    and ($source.source_durable_store_write_wal_receipt_plan_sha256 | type == "string" and length > 0)
    and ($source.source_durable_store_write_readback_plan_sha256 | type == "string" and length > 0)
    and ($source.source_durable_store_write_rollback_plan_sha256 | type == "string" and length > 0)
    and ($source.source_durable_store_write_tombstone_cleanup_plan_sha256 | type == "string" and length > 0)
    and $source.durable_store_write_preflight_result_accepted == true
    and $source.durable_store_write_preflight_executed == false
    and $source.durable_store_write_plan_executed == false
    and $source.durable_memory_store_write_performed == false
    and $source.memory_store_write_performed == false
    and $source.wal_write_performed == false
    and $source.receipt_persisted == false
    and $source.post_write_readback_performed == false
    and $source.rollback_executed == false
    and $source.tombstone_cleanup_executed == false
    and $source.raw_payload_plaintext_recorded == false
    and $source.live_kg_write_performed == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.credential_read == false
    and $source.channel_send_performed == false
    and $source.external_send_performed == false
    and $source.release_artifact_written == false
    and $source.install_executed == false
    and $source.active_binary_mutated == false
    and $source.allowed_next_actions[1].action == "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary"
    and $source.allowed_next_actions[1].requires_minimal_scoped_memory_real_write_canary_durable_store_write_preflight == true
  ' >/dev/null

approved_namespace="$(jq -r '.approved_namespace' <<<"$SOURCE_JSON")"
approved_store="$(jq -r '.approved_store' <<<"$SOURCE_JSON")"
approved_scope="$(jq -r '.approved_scope' <<<"$SOURCE_JSON")"
durable_store_write_target_id="$(jq -r '.durable_store_write_target_id' <<<"$SOURCE_JSON")"
durable_store_target_store_id="$(jq -r '.durable_store_target_store_id' <<<"$SOURCE_JSON")"
source_durable_store_write_preflight_hash_sha256="$(jq -r '.durable_store_write_preflight_hash_sha256' <<<"$SOURCE_JSON")"
source_durable_store_write_preflight_operator_handoff_sha256="$(jq -r '.durable_store_write_preflight_operator_handoff_sha256' <<<"$SOURCE_JSON")"
source_durable_store_write_target_sha256="$(jq -r '.source_durable_store_write_target_sha256' <<<"$SOURCE_JSON")"
source_durable_store_write_payload_digest_sha256="$(jq -r '.source_durable_store_write_payload_digest_sha256' <<<"$SOURCE_JSON")"
source_durable_store_write_wal_receipt_plan_sha256="$(jq -r '.source_durable_store_write_wal_receipt_plan_sha256' <<<"$SOURCE_JSON")"
source_durable_store_write_readback_plan_sha256="$(jq -r '.source_durable_store_write_readback_plan_sha256' <<<"$SOURCE_JSON")"
source_durable_store_write_rollback_plan_sha256="$(jq -r '.source_durable_store_write_rollback_plan_sha256' <<<"$SOURCE_JSON")"
source_durable_store_write_tombstone_cleanup_plan_sha256="$(jq -r '.source_durable_store_write_tombstone_cleanup_plan_sha256' <<<"$SOURCE_JSON")"

guarded_execution_envelope_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-envelope:v1:source-preflight=${source_durable_store_write_preflight_hash_sha256}:target-store=${durable_store_target_store_id}:namespace=${approved_namespace}:scope=${approved_scope}:execute=false")"
single_use_nonce_guard_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-nonce:v1:source-preflight=${source_durable_store_write_preflight_hash_sha256}:nonce-consumed=false:execute=false")"
explicit_command_guard_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-command:v1:source-preflight=${source_durable_store_write_preflight_hash_sha256}:command-dispatched=false:operator-explicit-required=true")"
single_write_budget_guard_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-budget:v1:target=${durable_store_write_target_id}:max-write=1:max-readback=1:max-rollback=1:execute=false")"
wal_receipt_guard_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-wal-receipt:v1:plan=${source_durable_store_write_wal_receipt_plan_sha256}:wal-write=false:receipt-persist=false")"
readback_guard_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-readback:v1:plan=${source_durable_store_write_readback_plan_sha256}:read=false")"
rollback_guard_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-rollback:v1:plan=${source_durable_store_write_rollback_plan_sha256}:rollback=false")"
tombstone_cleanup_guard_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-tombstone-cleanup:v1:plan=${source_durable_store_write_tombstone_cleanup_plan_sha256}:tombstone=false:cleanup=false")"
idempotency_replay_guard_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-idempotency-replay:v1:source-preflight=${source_durable_store_write_preflight_hash_sha256}:target-store=${durable_store_target_store_id}:replay=false")"
operator_guarded_execution_handoff_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-operator-handoff:v1:source=${source_report_sha256}:preflight-handoff=${source_durable_store_write_preflight_operator_handoff_sha256}:readiness=true:execute=false")"
guarded_execution_readiness_hash_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-readiness:v1:source-preflight=${source_durable_store_write_preflight_hash_sha256}:envelope=${guarded_execution_envelope_sha256}:nonce=${single_use_nonce_guard_sha256}:command=${explicit_command_guard_sha256}:budget=${single_write_budget_guard_sha256}:wal=${wal_receipt_guard_sha256}:readback=${readback_guard_sha256}:rollback=${rollback_guard_sha256}:tombstone=${tombstone_cleanup_guard_sha256}:handoff=${operator_guarded_execution_handoff_sha256}")"
boundary_hash_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-readiness-boundary:v1:source-ready=true:target=true:guards=${guarded_execution_readiness_hash_sha256}:fixtures=10:accepted=1:denials=32")"
policy_hash_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-readiness-policy:v1:accept-readiness-only:no-durable-memory-write:no-memory-store-mutation:no-wal-write:no-receipt-persist:no-readback:no-rollback:no-tombstone:no-kg:no-provider:no-channel:no-release:no-install")"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary_gate" \
  --arg source_report_sha256 "$source_report_sha256" \
  --arg approved_namespace "$approved_namespace" \
  --arg approved_store "$approved_store" \
  --arg approved_scope "$approved_scope" \
  --arg durable_store_write_target_id "$durable_store_write_target_id" \
  --arg durable_store_target_store_id "$durable_store_target_store_id" \
  --arg source_durable_store_write_preflight_hash_sha256 "$source_durable_store_write_preflight_hash_sha256" \
  --arg source_durable_store_write_preflight_operator_handoff_sha256 "$source_durable_store_write_preflight_operator_handoff_sha256" \
  --arg source_durable_store_write_target_sha256 "$source_durable_store_write_target_sha256" \
  --arg source_durable_store_write_payload_digest_sha256 "$source_durable_store_write_payload_digest_sha256" \
  --arg source_durable_store_write_wal_receipt_plan_sha256 "$source_durable_store_write_wal_receipt_plan_sha256" \
  --arg source_durable_store_write_readback_plan_sha256 "$source_durable_store_write_readback_plan_sha256" \
  --arg source_durable_store_write_rollback_plan_sha256 "$source_durable_store_write_rollback_plan_sha256" \
  --arg source_durable_store_write_tombstone_cleanup_plan_sha256 "$source_durable_store_write_tombstone_cleanup_plan_sha256" \
  --arg guarded_execution_envelope_sha256 "$guarded_execution_envelope_sha256" \
  --arg single_use_nonce_guard_sha256 "$single_use_nonce_guard_sha256" \
  --arg explicit_command_guard_sha256 "$explicit_command_guard_sha256" \
  --arg single_write_budget_guard_sha256 "$single_write_budget_guard_sha256" \
  --arg wal_receipt_guard_sha256 "$wal_receipt_guard_sha256" \
  --arg readback_guard_sha256 "$readback_guard_sha256" \
  --arg rollback_guard_sha256 "$rollback_guard_sha256" \
  --arg tombstone_cleanup_guard_sha256 "$tombstone_cleanup_guard_sha256" \
  --arg idempotency_replay_guard_sha256 "$idempotency_replay_guard_sha256" \
  --arg operator_guarded_execution_handoff_sha256 "$operator_guarded_execution_handoff_sha256" \
  --arg guarded_execution_readiness_hash_sha256 "$guarded_execution_readiness_hash_sha256" \
  --arg boundary_hash_sha256 "$boundary_hash_sha256" \
  --arg policy_hash_sha256 "$policy_hash_sha256" \
  --argjson source "$SOURCE_JSON" \
  '
  ([
    "single_use_nonce_consumed",
    "explicit_command_dispatched",
    "guarded_execution_command_dispatched",
    "durable_store_write_preflight_executed",
    "durable_store_write_guarded_execution_readiness_executed",
    "durable_store_write_guarded_execution_executed",
    "durable_store_write_execution_performed",
    "durable_store_write_plan_executed",
    "wal_write_performed",
    "receipt_persisted",
    "post_write_readback_performed",
    "rollback_executed",
    "rollback_performed",
    "tombstone_cleanup_executed",
    "tombstone_written",
    "durable_memory_store_write_performed",
    "durable_memory_store_read_performed",
    "durable_memory_store_rollback_performed",
    "memory_store_write_performed",
    "memory_store_mutated",
    "raw_payload_plaintext_recorded",
    "live_kg_write_performed",
    "provider_invoked",
    "model_invoked",
    "credential_read",
    "channel_send_performed",
    "external_send_performed",
    "release_artifact_written",
    "public_artifact_written",
    "install_executed",
    "service_restarted",
    "active_binary_mutated"
  ]) as $false_fields
  | ([
    "durable_store_write_guarded_execution_readiness_performed",
    "durable_store_write_guarded_execution_readiness_result_recorded",
    "durable_store_write_guarded_execution_readiness_result_accepted",
    "source_durable_store_write_preflight_bound",
    "source_durable_store_write_preflight_hash_bound",
    "source_durable_store_write_preflight_result_accepted",
    "approved_namespace_store_scope_guard_verified",
    "durable_store_target_guard_verified",
    "guarded_execution_envelope_bound",
    "single_use_nonce_guard_bound",
    "explicit_command_guard_bound",
    "single_write_budget_guard_bound",
    "wal_receipt_guard_bound",
    "post_write_readback_guard_bound",
    "rollback_guard_bound",
    "tombstone_cleanup_guard_bound",
    "idempotency_replay_guard_bound",
    "operator_guarded_execution_handoff_bound",
    "durable_memory_write_forbidden_until_guarded_execution_boundary",
    "memory_store_mutation_forbidden_until_guarded_execution_boundary",
    "kg_provider_channel_release_install_active_binary_forbidden",
    "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_accepted"
  ]) as $true_fields
  | ($false_fields | reduce .[] as $key ({}; .[$key] = false | .[$key + "_count"] = 0)) as $false_map
  | ($true_fields | reduce .[] as $key ({}; .[$key] = true)) as $true_map
  | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      source_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary_ready:true,
      source_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_report_sha256:$source_report_sha256,
      source_accepted_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixture_count:$source.accepted_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixture_count,
      source_blocked_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixture_count:$source.blocked_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixture_count,
      source_durable_store_write_preflight_result_accepted_count:$source.durable_store_write_preflight_result_accepted_count,
      source_durable_store_write_preflight_executed_count:($source.durable_store_write_preflight_executed_count // 0),
      source_durable_memory_store_write_performed_count:($source.durable_memory_store_write_performed_count // 0),
      source_memory_store_write_performed_count:($source.memory_store_write_performed_count // 0),
      approved_namespace:$approved_namespace,
      approved_store:$approved_store,
      approved_scope:$approved_scope,
      durable_store_write_target_id:$durable_store_write_target_id,
      durable_store_target_store_id:$durable_store_target_store_id,
      source_durable_store_write_preflight_hash_sha256:$source_durable_store_write_preflight_hash_sha256,
      source_durable_store_write_preflight_operator_handoff_sha256:$source_durable_store_write_preflight_operator_handoff_sha256,
      source_durable_store_write_target_sha256:$source_durable_store_write_target_sha256,
      source_durable_store_write_payload_digest_sha256:$source_durable_store_write_payload_digest_sha256,
      source_durable_store_write_wal_receipt_plan_sha256:$source_durable_store_write_wal_receipt_plan_sha256,
      source_durable_store_write_readback_plan_sha256:$source_durable_store_write_readback_plan_sha256,
      source_durable_store_write_rollback_plan_sha256:$source_durable_store_write_rollback_plan_sha256,
      source_durable_store_write_tombstone_cleanup_plan_sha256:$source_durable_store_write_tombstone_cleanup_plan_sha256,
      guarded_execution_envelope_sha256:$guarded_execution_envelope_sha256,
      single_use_nonce_guard_sha256:$single_use_nonce_guard_sha256,
      explicit_command_guard_sha256:$explicit_command_guard_sha256,
      single_write_budget_guard_sha256:$single_write_budget_guard_sha256,
      wal_receipt_guard_sha256:$wal_receipt_guard_sha256,
      readback_guard_sha256:$readback_guard_sha256,
      rollback_guard_sha256:$rollback_guard_sha256,
      tombstone_cleanup_guard_sha256:$tombstone_cleanup_guard_sha256,
      idempotency_replay_guard_sha256:$idempotency_replay_guard_sha256,
      operator_guarded_execution_handoff_sha256:$operator_guarded_execution_handoff_sha256,
      guarded_execution_readiness_hash_sha256:$guarded_execution_readiness_hash_sha256,
      memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary_ready:true,
      minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_ready:true,
      minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_performed:true,
      minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_accepted:true,
      scoped_memory_real_write_canary_mode:"minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_report_only",
      required_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_surface_count:12,
      ready_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_surface_count:12,
      minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixture_count:10,
      accepted_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixture_count:1,
      blocked_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixture_count:9,
      minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_accepted_count:1,
      durable_store_write_guarded_execution_readiness_authority_accepted_count:1,
      durable_store_write_guarded_execution_readiness_result_recorded_count:1,
      durable_store_write_guarded_execution_readiness_result_accepted_count:1,
      denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary_count:32,
      denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary:[
        "source_durable_store_write_preflight_boundary_required",
        "source_durable_store_write_preflight_result_acceptance_required",
        "source_durable_store_write_preflight_hash_required",
        "approved_namespace_required",
        "approved_store_required",
        "approved_scope_required",
        "durable_store_target_required",
        "durable_store_target_reachability_preflight_required",
        "guarded_execution_envelope_required",
        "single_use_nonce_guard_required",
        "explicit_command_guard_required",
        "single_write_budget_guard_required",
        "wal_receipt_guard_required",
        "readback_guard_required",
        "rollback_guard_required",
        "tombstone_cleanup_guard_required",
        "idempotency_replay_guard_required",
        "operator_guarded_execution_handoff_required",
        "durable_memory_store_read_denied",
        "durable_memory_store_write_denied",
        "durable_memory_store_rollback_denied",
        "memory_store_mutation_denied",
        "wal_write_denied",
        "receipt_record_persist_materialize_denied",
        "artifact_filesystem_write_denied",
        "post_write_readback_denied",
        "rollback_tombstone_execution_denied",
        "kg_provider_credential_channel_release_install_denied",
        "raw_payload_plaintext_denied",
        "guard_bypass_denied",
        "stale_preflight_denied",
        "direct_execution_authority_denied"
      ],
      minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary_hash_sha256:$boundary_hash_sha256,
      minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_policy_hash_sha256:$policy_hash_sha256,
      minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixtures:[
        {id:"accepted-guarded-execution-readiness", minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_accepted:true},
        {id:"missing-source-preflight", minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_accepted:false},
        {id:"wrong-namespace", minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_accepted:false},
        {id:"wrong-store", minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_accepted:false},
        {id:"wrong-scope", minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_accepted:false},
        {id:"execution-envelope-missing", minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_accepted:false},
        {id:"nonce-command-guard-missing", minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_accepted:false},
        {id:"budget-guard-missing", minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_accepted:false},
        {id:"readback-rollback-tombstone-guard-missing", minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_accepted:false},
        {id:"direct-execution-attempt", minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_accepted:false}
      ],
      side_effects: ($false_map + {
        durable_store_write_guarded_execution_readiness_performed:true,
        durable_store_write_guarded_execution_readiness_result_accepted:true,
        durable_memory_store_write_performed:false,
        memory_store_write_performed:false,
        external_send_performed:false
      }),
      allowed_next_actions:[
        {
          action:"run_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary_require_live_gate",
          status:"allowed_verification_only",
          writes_durable_memory:false,
          mutates_memory_store:false
        },
        {
          action:"prepare_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary",
          status:"allowed_report_only_next_slice",
          requires_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness:true,
          writes_durable_memory:false,
          mutates_memory_store:false
        }
      ]
    } + $false_map + $true_map
  '
