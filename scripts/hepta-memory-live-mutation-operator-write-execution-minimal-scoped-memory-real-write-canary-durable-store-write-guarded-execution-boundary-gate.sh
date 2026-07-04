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
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-readiness-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-readiness-boundary-gate.sh
)"

source_report_sha256="$(sha256_text "$SOURCE_JSON")"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_ready == true
    and $source.minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_performed == true
    and $source.minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_accepted == true
    and $source.accepted_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixture_count == 1
    and $source.blocked_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixture_count == 9
    and $source.durable_store_write_guarded_execution_readiness_result_accepted_count == 1
    and ($source.durable_store_write_guarded_execution_readiness_executed_count // 0) == 0
    and ($source.durable_store_write_guarded_execution_executed_count // 0) == 0
    and ($source.durable_store_write_execution_performed_count // 0) == 0
    and ($source.durable_memory_store_write_performed_count // 0) == 0
    and ($source.memory_store_write_performed_count // 0) == 0
    and $source.approved_namespace == "hepta.memory.canary"
    and $source.approved_store == "wal-receipt-canary-artifact"
    and $source.approved_scope == "session"
    and $source.durable_store_write_target_id == "hepta-minimal-scoped-memory-real-write-canary-durable-store-write-target-v1"
    and $source.durable_store_target_store_id == "hepta-memory-durable-store-canary-plan-only"
    and ($source.guarded_execution_readiness_hash_sha256 | type == "string" and length > 0)
    and ($source.guarded_execution_envelope_sha256 | type == "string" and length > 0)
    and ($source.single_use_nonce_guard_sha256 | type == "string" and length > 0)
    and ($source.explicit_command_guard_sha256 | type == "string" and length > 0)
    and ($source.single_write_budget_guard_sha256 | type == "string" and length > 0)
    and ($source.wal_receipt_guard_sha256 | type == "string" and length > 0)
    and ($source.readback_guard_sha256 | type == "string" and length > 0)
    and ($source.rollback_guard_sha256 | type == "string" and length > 0)
    and ($source.tombstone_cleanup_guard_sha256 | type == "string" and length > 0)
    and ($source.idempotency_replay_guard_sha256 | type == "string" and length > 0)
    and ($source.operator_guarded_execution_handoff_sha256 | type == "string" and length > 0)
    and $source.durable_store_write_guarded_execution_readiness_result_accepted == true
    and $source.durable_store_write_guarded_execution_readiness_executed == false
    and $source.durable_store_write_guarded_execution_executed == false
    and $source.durable_store_write_execution_performed == false
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
    and $source.allowed_next_actions[1].action == "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary"
    and $source.allowed_next_actions[1].requires_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness == true
  ' >/dev/null

approved_namespace="$(jq -r '.approved_namespace' <<<"$SOURCE_JSON")"
approved_store="$(jq -r '.approved_store' <<<"$SOURCE_JSON")"
approved_scope="$(jq -r '.approved_scope' <<<"$SOURCE_JSON")"
durable_store_write_target_id="$(jq -r '.durable_store_write_target_id' <<<"$SOURCE_JSON")"
durable_store_target_store_id="$(jq -r '.durable_store_target_store_id' <<<"$SOURCE_JSON")"
source_guarded_execution_readiness_hash_sha256="$(jq -r '.guarded_execution_readiness_hash_sha256' <<<"$SOURCE_JSON")"
source_guarded_execution_envelope_sha256="$(jq -r '.guarded_execution_envelope_sha256' <<<"$SOURCE_JSON")"
source_single_use_nonce_guard_sha256="$(jq -r '.single_use_nonce_guard_sha256' <<<"$SOURCE_JSON")"
source_explicit_command_guard_sha256="$(jq -r '.explicit_command_guard_sha256' <<<"$SOURCE_JSON")"
source_single_write_budget_guard_sha256="$(jq -r '.single_write_budget_guard_sha256' <<<"$SOURCE_JSON")"
source_wal_receipt_guard_sha256="$(jq -r '.wal_receipt_guard_sha256' <<<"$SOURCE_JSON")"
source_readback_guard_sha256="$(jq -r '.readback_guard_sha256' <<<"$SOURCE_JSON")"
source_rollback_guard_sha256="$(jq -r '.rollback_guard_sha256' <<<"$SOURCE_JSON")"
source_tombstone_cleanup_guard_sha256="$(jq -r '.tombstone_cleanup_guard_sha256' <<<"$SOURCE_JSON")"
source_idempotency_replay_guard_sha256="$(jq -r '.idempotency_replay_guard_sha256' <<<"$SOURCE_JSON")"
source_operator_guarded_execution_handoff_sha256="$(jq -r '.operator_guarded_execution_handoff_sha256' <<<"$SOURCE_JSON")"

guarded_execution_boundary_envelope_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-boundary-envelope:v1:source-readiness=${source_guarded_execution_readiness_hash_sha256}:target-store=${durable_store_target_store_id}:namespace=${approved_namespace}:scope=${approved_scope}:execute=false")"
guarded_execution_boundary_nonce_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-boundary-nonce:v1:source-readiness=${source_guarded_execution_readiness_hash_sha256}:source-nonce=${source_single_use_nonce_guard_sha256}:nonce-consumed=false")"
guarded_execution_boundary_command_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-boundary-command:v1:source-readiness=${source_guarded_execution_readiness_hash_sha256}:source-command=${source_explicit_command_guard_sha256}:command-dispatched=false")"
guarded_execution_boundary_budget_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-boundary-budget:v1:source-budget=${source_single_write_budget_guard_sha256}:target=${durable_store_write_target_id}:execute=false")"
guarded_execution_boundary_wal_receipt_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-boundary-wal-receipt:v1:source-wal=${source_wal_receipt_guard_sha256}:wal-write=false:receipt-persist=false")"
guarded_execution_boundary_readback_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-boundary-readback:v1:source-readback=${source_readback_guard_sha256}:readback=false")"
guarded_execution_boundary_rollback_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-boundary-rollback:v1:source-rollback=${source_rollback_guard_sha256}:rollback=false")"
guarded_execution_boundary_tombstone_cleanup_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-boundary-tombstone-cleanup:v1:source-tombstone=${source_tombstone_cleanup_guard_sha256}:tombstone=false:cleanup=false")"
guarded_execution_boundary_idempotency_replay_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-boundary-idempotency-replay:v1:source-replay=${source_idempotency_replay_guard_sha256}:replay=false")"
operator_guarded_execution_boundary_handoff_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-boundary-operator-handoff:v1:source=${source_report_sha256}:readiness-handoff=${source_operator_guarded_execution_handoff_sha256}:boundary=true:execute=false")"
guarded_execution_boundary_hash_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-boundary:v1:source-readiness=${source_guarded_execution_readiness_hash_sha256}:source-envelope=${source_guarded_execution_envelope_sha256}:envelope=${guarded_execution_boundary_envelope_sha256}:nonce=${guarded_execution_boundary_nonce_sha256}:command=${guarded_execution_boundary_command_sha256}:budget=${guarded_execution_boundary_budget_sha256}:wal=${guarded_execution_boundary_wal_receipt_sha256}:readback=${guarded_execution_boundary_readback_sha256}:rollback=${guarded_execution_boundary_rollback_sha256}:tombstone=${guarded_execution_boundary_tombstone_cleanup_sha256}:handoff=${operator_guarded_execution_boundary_handoff_sha256}")"
boundary_hash_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-boundary-report:v1:source-ready=true:target=true:boundary=${guarded_execution_boundary_hash_sha256}:fixtures=10:accepted=1:denials=34:execute=false")"
policy_hash_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-boundary-policy:v1:accept-boundary-only:no-durable-memory-write:no-memory-store-mutation:no-wal-write:no-receipt-persist:no-readback:no-rollback:no-tombstone:no-kg:no-provider:no-channel:no-release:no-install")"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_gate" \
  --arg endpoint "/api/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-boundary" \
  --arg source_command "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-boundary --json" \
  --arg source_report_sha256 "$source_report_sha256" \
  --arg approved_namespace "$approved_namespace" \
  --arg approved_store "$approved_store" \
  --arg approved_scope "$approved_scope" \
  --arg durable_store_write_target_id "$durable_store_write_target_id" \
  --arg durable_store_target_store_id "$durable_store_target_store_id" \
  --arg source_guarded_execution_readiness_hash_sha256 "$source_guarded_execution_readiness_hash_sha256" \
  --arg source_guarded_execution_envelope_sha256 "$source_guarded_execution_envelope_sha256" \
  --arg source_single_use_nonce_guard_sha256 "$source_single_use_nonce_guard_sha256" \
  --arg source_explicit_command_guard_sha256 "$source_explicit_command_guard_sha256" \
  --arg source_single_write_budget_guard_sha256 "$source_single_write_budget_guard_sha256" \
  --arg source_wal_receipt_guard_sha256 "$source_wal_receipt_guard_sha256" \
  --arg source_readback_guard_sha256 "$source_readback_guard_sha256" \
  --arg source_rollback_guard_sha256 "$source_rollback_guard_sha256" \
  --arg source_tombstone_cleanup_guard_sha256 "$source_tombstone_cleanup_guard_sha256" \
  --arg source_idempotency_replay_guard_sha256 "$source_idempotency_replay_guard_sha256" \
  --arg source_operator_guarded_execution_handoff_sha256 "$source_operator_guarded_execution_handoff_sha256" \
  --arg guarded_execution_boundary_envelope_sha256 "$guarded_execution_boundary_envelope_sha256" \
  --arg guarded_execution_boundary_nonce_sha256 "$guarded_execution_boundary_nonce_sha256" \
  --arg guarded_execution_boundary_command_sha256 "$guarded_execution_boundary_command_sha256" \
  --arg guarded_execution_boundary_budget_sha256 "$guarded_execution_boundary_budget_sha256" \
  --arg guarded_execution_boundary_wal_receipt_sha256 "$guarded_execution_boundary_wal_receipt_sha256" \
  --arg guarded_execution_boundary_readback_sha256 "$guarded_execution_boundary_readback_sha256" \
  --arg guarded_execution_boundary_rollback_sha256 "$guarded_execution_boundary_rollback_sha256" \
  --arg guarded_execution_boundary_tombstone_cleanup_sha256 "$guarded_execution_boundary_tombstone_cleanup_sha256" \
  --arg guarded_execution_boundary_idempotency_replay_sha256 "$guarded_execution_boundary_idempotency_replay_sha256" \
  --arg operator_guarded_execution_boundary_handoff_sha256 "$operator_guarded_execution_boundary_handoff_sha256" \
  --arg guarded_execution_boundary_hash_sha256 "$guarded_execution_boundary_hash_sha256" \
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
    "durable_store_write_guarded_execution_boundary_executed",
    "durable_store_write_guarded_execution_executed",
    "durable_store_write_execution_performed",
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
    "durable_store_write_guarded_execution_boundary_performed",
    "durable_store_write_guarded_execution_boundary_result_recorded",
    "durable_store_write_guarded_execution_boundary_result_accepted",
    "source_durable_store_write_guarded_execution_readiness_bound",
    "source_durable_store_write_guarded_execution_readiness_hash_bound",
    "source_durable_store_write_guarded_execution_readiness_result_accepted",
    "approved_namespace_store_scope_execution_guard_verified",
    "durable_store_target_execution_guard_verified",
    "guarded_execution_boundary_envelope_bound",
    "single_use_nonce_execution_guard_verified",
    "explicit_command_execution_guard_verified",
    "single_write_budget_execution_guard_verified",
    "wal_receipt_execution_guard_verified",
    "post_write_readback_execution_guard_verified",
    "rollback_execution_guard_verified",
    "tombstone_cleanup_execution_guard_verified",
    "idempotency_replay_execution_guard_verified",
    "operator_guarded_execution_boundary_handoff_bound",
    "durable_memory_write_forbidden_until_single_shot_execution",
    "memory_store_mutation_forbidden_until_single_shot_execution",
    "kg_provider_channel_release_install_active_binary_forbidden",
    "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepted"
  ]) as $true_fields
  | ($false_fields | reduce .[] as $key ({}; .[$key] = false | .[$key + "_count"] = 0)) as $false_map
  | ($true_fields | reduce .[] as $key ({}; .[$key] = true)) as $true_map
  | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      source_command:$source_command,
      native_route:true,
      minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_ready:true,
      minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_performed:true,
      minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepted:true,
      scoped_memory_real_write_canary_mode:"minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_report_only",
      source_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary_ready:true,
      source_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_report_sha256:$source_report_sha256,
      source_accepted_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixture_count:$source.accepted_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixture_count,
      source_blocked_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixture_count:$source.blocked_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixture_count,
      source_durable_store_write_guarded_execution_readiness_result_accepted_count:$source.durable_store_write_guarded_execution_readiness_result_accepted_count,
      source_durable_store_write_guarded_execution_readiness_executed_count:($source.durable_store_write_guarded_execution_readiness_executed_count // 0),
      source_durable_store_write_guarded_execution_executed_count:($source.durable_store_write_guarded_execution_executed_count // 0),
      source_durable_memory_store_write_performed_count:($source.durable_memory_store_write_performed_count // 0),
      source_memory_store_write_performed_count:($source.memory_store_write_performed_count // 0),
      approved_namespace:$approved_namespace,
      approved_store:$approved_store,
      approved_scope:$approved_scope,
      durable_store_write_target_id:$durable_store_write_target_id,
      durable_store_target_store_id:$durable_store_target_store_id,
      source_guarded_execution_readiness_hash_sha256:$source_guarded_execution_readiness_hash_sha256,
      source_guarded_execution_envelope_sha256:$source_guarded_execution_envelope_sha256,
      source_single_use_nonce_guard_sha256:$source_single_use_nonce_guard_sha256,
      source_explicit_command_guard_sha256:$source_explicit_command_guard_sha256,
      source_single_write_budget_guard_sha256:$source_single_write_budget_guard_sha256,
      source_wal_receipt_guard_sha256:$source_wal_receipt_guard_sha256,
      source_readback_guard_sha256:$source_readback_guard_sha256,
      source_rollback_guard_sha256:$source_rollback_guard_sha256,
      source_tombstone_cleanup_guard_sha256:$source_tombstone_cleanup_guard_sha256,
      source_idempotency_replay_guard_sha256:$source_idempotency_replay_guard_sha256,
      source_operator_guarded_execution_handoff_sha256:$source_operator_guarded_execution_handoff_sha256,
      guarded_execution_boundary_envelope_sha256:$guarded_execution_boundary_envelope_sha256,
      guarded_execution_boundary_nonce_sha256:$guarded_execution_boundary_nonce_sha256,
      guarded_execution_boundary_command_sha256:$guarded_execution_boundary_command_sha256,
      guarded_execution_boundary_budget_sha256:$guarded_execution_boundary_budget_sha256,
      guarded_execution_boundary_wal_receipt_sha256:$guarded_execution_boundary_wal_receipt_sha256,
      guarded_execution_boundary_readback_sha256:$guarded_execution_boundary_readback_sha256,
      guarded_execution_boundary_rollback_sha256:$guarded_execution_boundary_rollback_sha256,
      guarded_execution_boundary_tombstone_cleanup_sha256:$guarded_execution_boundary_tombstone_cleanup_sha256,
      guarded_execution_boundary_idempotency_replay_sha256:$guarded_execution_boundary_idempotency_replay_sha256,
      operator_guarded_execution_boundary_handoff_sha256:$operator_guarded_execution_boundary_handoff_sha256,
      guarded_execution_boundary_hash_sha256:$guarded_execution_boundary_hash_sha256,
      required_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_surface_count:12,
      ready_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_surface_count:12,
      minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count:10,
      accepted_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count:1,
      blocked_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count:9,
      minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepted_count:1,
      durable_store_write_guarded_execution_boundary_authority_accepted_count:1,
      durable_store_write_guarded_execution_boundary_result_accepted_count:1,
      denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_count:34,
      minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_hash_sha256:$boundary_hash_sha256,
      minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_policy_hash_sha256:$policy_hash_sha256,
      minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixtures:[
        {fixture_id:"accepted-guarded-execution-boundary", minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepted:true},
        {fixture_id:"missing-source-readiness", minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepted:false},
        {fixture_id:"wrong-namespace", minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepted:false},
        {fixture_id:"wrong-store", minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepted:false},
        {fixture_id:"wrong-scope", minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepted:false},
        {fixture_id:"boundary-envelope-missing", minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepted:false},
        {fixture_id:"nonce-command-guard-missing", minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepted:false},
        {fixture_id:"budget-or-wal-guard-missing", minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepted:false},
        {fixture_id:"readback-rollback-tombstone-guard-missing", minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepted:false},
        {fixture_id:"direct-durable-write-attempt", minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepted:false}
      ],
      denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary:[
        "source_guarded_execution_readiness_boundary_required",
        "source_guarded_execution_readiness_result_acceptance_required",
        "source_guarded_execution_readiness_hash_required",
        "approved_namespace_required",
        "approved_store_required",
        "approved_scope_required",
        "durable_store_target_required",
        "guarded_execution_boundary_envelope_required",
        "single_use_nonce_guard_required",
        "explicit_command_guard_required",
        "single_write_budget_guard_required",
        "wal_receipt_guard_required",
        "readback_guard_required",
        "rollback_guard_required",
        "tombstone_cleanup_guard_required",
        "idempotency_replay_guard_required",
        "operator_guarded_execution_boundary_handoff_required",
        "direct_durable_store_write_execution_denied",
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
        "replay_execution_denied",
        "guard_bypass_denied",
        "stale_readiness_denied",
        "single_shot_execution_command_required",
        "actual_durable_write_must_use_next_boundary"
      ],
      allowed_next_actions:[
        {action:"run_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_require_live_gate", status:"allowed_verification_only", writes_durable_memory:false, mutates_memory_store:false},
        {action:"prepare_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary", status:"requires_separate_guarded_execution_next_slice", requires_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary:true, writes_durable_memory:false, mutates_memory_store:false, actual_write_requires_separate_explicit_command:true}
      ],
      side_effects:({
        durable_store_write_guarded_execution_boundary_performed:true,
        durable_store_write_guarded_execution_boundary_result_accepted:true
      } + $false_map)
    } + $false_map + $true_map
  '
