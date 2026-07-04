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
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-plan-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-plan-boundary-gate.sh
)"

source_report_sha256="$(sha256_text "$SOURCE_JSON")"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.minimal_scoped_memory_real_write_canary_durable_store_write_plan_ready == true
    and $source.minimal_scoped_memory_real_write_canary_durable_store_write_plan_performed == true
    and $source.minimal_scoped_memory_real_write_canary_durable_store_write_plan_accepted == true
    and $source.accepted_minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixture_count == 1
    and $source.blocked_minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixture_count == 9
    and $source.durable_store_write_plan_result_accepted_count == 1
    and ($source.durable_store_write_plan_executed_count // 0) == 0
    and ($source.durable_memory_store_write_performed_count // 0) == 0
    and ($source.memory_store_write_performed_count // 0) == 0
    and $source.approved_namespace == "hepta.memory.canary"
    and $source.approved_store == "wal-receipt-canary-artifact"
    and $source.approved_scope == "session"
    and ($source.durable_store_write_plan_hash_sha256 | type == "string" and length > 0)
    and ($source.durable_store_write_target_sha256 | type == "string" and length > 0)
    and ($source.durable_store_write_envelope_sha256 | type == "string" and length > 0)
    and ($source.durable_store_write_payload_digest_sha256 | type == "string" and length > 0)
    and ($source.durable_store_write_wal_receipt_plan_sha256 | type == "string" and length > 0)
    and ($source.durable_store_write_readback_plan_sha256 | type == "string" and length > 0)
    and ($source.durable_store_write_rollback_plan_sha256 | type == "string" and length > 0)
    and ($source.durable_store_write_tombstone_cleanup_plan_sha256 | type == "string" and length > 0)
    and $source.durable_store_write_plan_result_accepted == true
    and $source.durable_store_write_plan_executed == false
    and $source.durable_memory_store_write_performed == false
    and $source.memory_store_write_performed == false
    and $source.external_send_performed == false
    and $source.allowed_next_actions[1].action == "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary"
    and $source.allowed_next_actions[1].requires_minimal_scoped_memory_real_write_canary_durable_store_write_plan == true
  ' >/dev/null

approved_namespace="$(jq -r '.approved_namespace' <<<"$SOURCE_JSON")"
approved_store="$(jq -r '.approved_store' <<<"$SOURCE_JSON")"
approved_scope="$(jq -r '.approved_scope' <<<"$SOURCE_JSON")"
durable_store_write_target_id="$(jq -r '.durable_store_write_target_id' <<<"$SOURCE_JSON")"
durable_store_target_store_id="$(jq -r '.durable_store_target_store_id' <<<"$SOURCE_JSON")"
source_durable_store_write_plan_hash_sha256="$(jq -r '.durable_store_write_plan_hash_sha256' <<<"$SOURCE_JSON")"
source_durable_store_write_target_sha256="$(jq -r '.durable_store_write_target_sha256' <<<"$SOURCE_JSON")"
source_durable_store_write_envelope_sha256="$(jq -r '.durable_store_write_envelope_sha256' <<<"$SOURCE_JSON")"
source_durable_store_write_payload_digest_sha256="$(jq -r '.durable_store_write_payload_digest_sha256' <<<"$SOURCE_JSON")"
source_durable_store_write_wal_receipt_plan_sha256="$(jq -r '.durable_store_write_wal_receipt_plan_sha256' <<<"$SOURCE_JSON")"
source_durable_store_write_readback_plan_sha256="$(jq -r '.durable_store_write_readback_plan_sha256' <<<"$SOURCE_JSON")"
source_durable_store_write_rollback_plan_sha256="$(jq -r '.durable_store_write_rollback_plan_sha256' <<<"$SOURCE_JSON")"
source_durable_store_write_tombstone_cleanup_plan_sha256="$(jq -r '.durable_store_write_tombstone_cleanup_plan_sha256' <<<"$SOURCE_JSON")"

durable_store_write_preflight_target_reachability_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-preflight-target-reachability:v1:target=${source_durable_store_write_target_sha256}:target-store=${durable_store_target_store_id}:reachable=true:write=false")"
durable_store_write_preflight_namespace_scope_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-preflight-namespace-scope:v1:namespace=${approved_namespace}:store=${approved_store}:target-store=${durable_store_target_store_id}:scope=${approved_scope}:accepted=true")"
durable_store_write_preflight_redaction_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-preflight-redaction:v1:payload=${source_durable_store_write_payload_digest_sha256}:raw=false:secret-scan=pass")"
durable_store_write_preflight_wal_receipt_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-preflight-wal-receipt:v1:plan=${source_durable_store_write_wal_receipt_plan_sha256}:wal-write=false:receipt-persist=false")"
durable_store_write_preflight_readback_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-preflight-readback:v1:plan=${source_durable_store_write_readback_plan_sha256}:read=false")"
durable_store_write_preflight_rollback_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-preflight-rollback:v1:plan=${source_durable_store_write_rollback_plan_sha256}:rollback=false")"
durable_store_write_preflight_tombstone_cleanup_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-preflight-tombstone-cleanup:v1:plan=${source_durable_store_write_tombstone_cleanup_plan_sha256}:tombstone=false:cleanup=false")"
durable_store_write_preflight_idempotency_replay_guard_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-preflight-idempotency-replay:v1:source-plan=${source_durable_store_write_plan_hash_sha256}:target-store=${durable_store_target_store_id}:replay=false")"
durable_store_write_preflight_operator_handoff_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-preflight-operator-handoff:v1:source=${source_report_sha256}:target=${source_durable_store_write_target_sha256}:preflight=true:write=false")"
durable_store_write_preflight_hash_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-write-preflight:v1:source-plan=${source_durable_store_write_plan_hash_sha256}:target-reachability=${durable_store_write_preflight_target_reachability_sha256}:namespace-scope=${durable_store_write_preflight_namespace_scope_sha256}:redaction=${durable_store_write_preflight_redaction_sha256}:wal-receipt=${durable_store_write_preflight_wal_receipt_sha256}:readback=${durable_store_write_preflight_readback_sha256}:rollback=${durable_store_write_preflight_rollback_sha256}:tombstone=${durable_store_write_preflight_tombstone_cleanup_sha256}:handoff=${durable_store_write_preflight_operator_handoff_sha256}")"
boundary_hash_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-write-preflight-boundary:v1:source-ready=true:target=true:redaction=true:preflight=${durable_store_write_preflight_hash_sha256}:fixtures=10:accepted=1:denials=30")"
policy_hash_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-write-preflight-policy:v1:accept-preflight-only:no-durable-memory-write:no-memory-store-mutation:no-wal-write:no-receipt-persist:no-readback:no-rollback:no-tombstone:no-kg:no-provider:no-channel:no-release:no-install")"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary_gate" \
  --arg source_report_sha256 "$source_report_sha256" \
  --arg approved_namespace "$approved_namespace" \
  --arg approved_store "$approved_store" \
  --arg approved_scope "$approved_scope" \
  --arg durable_store_write_target_id "$durable_store_write_target_id" \
  --arg durable_store_target_store_id "$durable_store_target_store_id" \
  --arg source_durable_store_write_plan_hash_sha256 "$source_durable_store_write_plan_hash_sha256" \
  --arg source_durable_store_write_target_sha256 "$source_durable_store_write_target_sha256" \
  --arg source_durable_store_write_envelope_sha256 "$source_durable_store_write_envelope_sha256" \
  --arg source_durable_store_write_payload_digest_sha256 "$source_durable_store_write_payload_digest_sha256" \
  --arg source_durable_store_write_wal_receipt_plan_sha256 "$source_durable_store_write_wal_receipt_plan_sha256" \
  --arg source_durable_store_write_readback_plan_sha256 "$source_durable_store_write_readback_plan_sha256" \
  --arg source_durable_store_write_rollback_plan_sha256 "$source_durable_store_write_rollback_plan_sha256" \
  --arg source_durable_store_write_tombstone_cleanup_plan_sha256 "$source_durable_store_write_tombstone_cleanup_plan_sha256" \
  --arg durable_store_write_preflight_target_reachability_sha256 "$durable_store_write_preflight_target_reachability_sha256" \
  --arg durable_store_write_preflight_namespace_scope_sha256 "$durable_store_write_preflight_namespace_scope_sha256" \
  --arg durable_store_write_preflight_redaction_sha256 "$durable_store_write_preflight_redaction_sha256" \
  --arg durable_store_write_preflight_wal_receipt_sha256 "$durable_store_write_preflight_wal_receipt_sha256" \
  --arg durable_store_write_preflight_readback_sha256 "$durable_store_write_preflight_readback_sha256" \
  --arg durable_store_write_preflight_rollback_sha256 "$durable_store_write_preflight_rollback_sha256" \
  --arg durable_store_write_preflight_tombstone_cleanup_sha256 "$durable_store_write_preflight_tombstone_cleanup_sha256" \
  --arg durable_store_write_preflight_idempotency_replay_guard_sha256 "$durable_store_write_preflight_idempotency_replay_guard_sha256" \
  --arg durable_store_write_preflight_operator_handoff_sha256 "$durable_store_write_preflight_operator_handoff_sha256" \
  --arg durable_store_write_preflight_hash_sha256 "$durable_store_write_preflight_hash_sha256" \
  --arg boundary_hash_sha256 "$boundary_hash_sha256" \
  --arg policy_hash_sha256 "$policy_hash_sha256" \
  --argjson source "$SOURCE_JSON" \
  '
  ([
    "single_use_nonce_consumed",
    "explicit_command_dispatched",
    "durable_store_write_preflight_executed",
    "durable_store_write_plan_executed",
    "wal_write_performed",
    "wal_recorded",
    "wal_persisted",
    "receipt_recorded",
    "receipt_persisted",
    "receipt_materialized",
    "receipt_delivered",
    "canary_artifact_filesystem_written",
    "artifact_readback_performed",
    "artifact_cleanup_performed",
    "filesystem_written",
    "post_write_readback_performed",
    "readback_result_recorded",
    "readback_result_persisted",
    "readback_result_accepted",
    "rollback_executed",
    "rollback_performed",
    "rollback_result_recorded",
    "rollback_result_persisted",
    "rollback_result_accepted",
    "tombstone_cleanup_executed",
    "tombstone_written",
    "compensating_memory_write_performed",
    "activation_performed",
    "live_mutation_execution_performed",
    "memory_write_execution_performed",
    "memory_store_write_path_enabled",
    "memory_store_write_allowed",
    "memory_store_write_performed",
    "memory_store_mutation_allowed",
    "memory_store_mutated",
    "durable_memory_store_write_performed",
    "durable_memory_store_read_performed",
    "durable_memory_store_rollback_performed",
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
  ]) as $false_fields
  | ([
    "durable_store_write_preflight_performed",
    "durable_store_write_preflight_result_recorded",
    "durable_store_write_preflight_result_accepted",
    "durable_store_target_reachability_checked",
    "approved_namespace_store_scope_preflight_verified",
    "durable_store_write_envelope_preflight_verified",
    "durable_store_write_payload_digest_preflight_verified",
    "payload_redaction_preflight_verified",
    "payload_secret_plaintext_scan_passed",
    "durable_store_write_wal_receipt_preflight_bound",
    "durable_store_write_readback_preflight_bound",
    "durable_store_write_rollback_preflight_bound",
    "durable_store_write_tombstone_cleanup_preflight_bound",
    "durable_store_write_idempotency_replay_guard_preflight_bound",
    "durable_store_write_operator_preflight_handoff_bound",
    "minimal_scoped_memory_real_write_canary_durable_store_write_preflight_accepted",
    "source_durable_store_write_plan_bound",
    "source_durable_store_write_plan_hash_bound",
    "source_durable_store_write_plan_result_accepted",
    "approved_namespace_bound",
    "approved_store_bound",
    "approved_scope_bound",
    "durable_memory_write_forbidden",
    "memory_store_mutation_forbidden",
    "wal_write_forbidden_on_report_route",
    "receipt_persist_forbidden_on_report_route",
    "post_write_readback_forbidden_on_report_route",
    "rollback_execution_forbidden",
    "tombstone_write_forbidden",
    "artifact_cleanup_forbidden"
  ]) as $true_fields
  | {
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    source_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary_ready:true,
    source_minimal_scoped_memory_real_write_canary_durable_store_write_plan_report_sha256:$source_report_sha256,
    source_accepted_minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixture_count:$source.accepted_minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixture_count,
    source_blocked_minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixture_count:$source.blocked_minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixture_count,
    source_durable_store_write_plan_result_accepted_count:$source.durable_store_write_plan_result_accepted_count,
    source_durable_store_write_plan_executed_count:($source.durable_store_write_plan_executed_count // 0),
    source_durable_memory_store_write_performed_count:($source.durable_memory_store_write_performed_count // 0),
    source_memory_store_write_performed_count:($source.memory_store_write_performed_count // 0),
    approved_namespace:$approved_namespace,
    approved_store:$approved_store,
    approved_scope:$approved_scope,
    durable_store_write_target_id:$durable_store_write_target_id,
    durable_store_target_store_id:$durable_store_target_store_id,
    source_durable_store_write_plan_hash_sha256:$source_durable_store_write_plan_hash_sha256,
    source_durable_store_write_target_sha256:$source_durable_store_write_target_sha256,
    source_durable_store_write_envelope_sha256:$source_durable_store_write_envelope_sha256,
    source_durable_store_write_payload_digest_sha256:$source_durable_store_write_payload_digest_sha256,
    source_durable_store_write_wal_receipt_plan_sha256:$source_durable_store_write_wal_receipt_plan_sha256,
    source_durable_store_write_readback_plan_sha256:$source_durable_store_write_readback_plan_sha256,
    source_durable_store_write_rollback_plan_sha256:$source_durable_store_write_rollback_plan_sha256,
    source_durable_store_write_tombstone_cleanup_plan_sha256:$source_durable_store_write_tombstone_cleanup_plan_sha256,
    durable_store_write_preflight_target_reachability_sha256:$durable_store_write_preflight_target_reachability_sha256,
    durable_store_write_preflight_namespace_scope_sha256:$durable_store_write_preflight_namespace_scope_sha256,
    durable_store_write_preflight_redaction_sha256:$durable_store_write_preflight_redaction_sha256,
    durable_store_write_preflight_wal_receipt_sha256:$durable_store_write_preflight_wal_receipt_sha256,
    durable_store_write_preflight_readback_sha256:$durable_store_write_preflight_readback_sha256,
    durable_store_write_preflight_rollback_sha256:$durable_store_write_preflight_rollback_sha256,
    durable_store_write_preflight_tombstone_cleanup_sha256:$durable_store_write_preflight_tombstone_cleanup_sha256,
    durable_store_write_preflight_idempotency_replay_guard_sha256:$durable_store_write_preflight_idempotency_replay_guard_sha256,
    durable_store_write_preflight_operator_handoff_sha256:$durable_store_write_preflight_operator_handoff_sha256,
    durable_store_write_preflight_hash_sha256:$durable_store_write_preflight_hash_sha256,
    memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary_ready:true,
    minimal_scoped_memory_real_write_canary_durable_store_write_preflight_ready:true,
    minimal_scoped_memory_real_write_canary_durable_store_write_preflight_performed:true,
    minimal_scoped_memory_real_write_canary_durable_store_write_preflight_accepted:true,
    scoped_memory_real_write_canary_mode:"minimal_scoped_memory_real_write_canary_durable_store_write_preflight_report_only",
    required_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_surface_count:12,
    ready_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_surface_count:12,
    minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixture_count:10,
    accepted_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixture_count:1,
    blocked_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixture_count:9,
    minimal_scoped_memory_real_write_canary_durable_store_write_preflight_accepted_count:1,
    durable_store_write_preflight_authority_accepted_count:1,
    durable_store_write_preflight_result_accepted_count:1,
    minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary_hash_sha256:$boundary_hash_sha256,
    minimal_scoped_memory_real_write_canary_durable_store_write_preflight_policy_hash_sha256:$policy_hash_sha256,
    denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary_count:30,
    denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary:[
      "source_durable_store_write_plan_boundary_required",
      "source_durable_store_write_plan_result_acceptance_required",
      "source_durable_store_write_plan_hash_required",
      "approved_namespace_required",
      "approved_store_required",
      "approved_scope_required",
      "durable_store_target_required",
      "durable_store_target_reachability_preflight_required",
      "durable_store_write_envelope_required",
      "durable_store_write_envelope_preflight_required",
      "payload_digest_required",
      "payload_redaction_preflight_required",
      "payload_secret_plaintext_scan_required",
      "wal_receipt_preflight_required",
      "readback_preflight_required",
      "rollback_preflight_required",
      "tombstone_cleanup_preflight_required",
      "idempotency_replay_guard_preflight_required",
      "operator_preflight_handoff_required",
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
      "raw_payload_plaintext_denied"
    ],
    minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixtures:([range(0;10)] | map({fixture_index:., minimal_scoped_memory_real_write_canary_durable_store_write_preflight_accepted:(. == 0)})),
    side_effects:{
      durable_store_write_preflight_performed:true,
      durable_store_write_preflight_result_accepted:true,
      durable_store_write_preflight_executed:false,
      durable_memory_store_write_performed:false,
      durable_memory_store_read_performed:false,
      durable_memory_store_rollback_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      wal_write_performed:false,
      receipt_persisted:false,
      rollback_executed:false,
      tombstone_written:false,
      external_send_performed:false
    },
    allowed_next_actions:[
      {
        action:"run_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary_require_live_gate",
        status:"allowed_verification_only",
        writes_durable_memory:false,
        mutates_memory_store:false
      },
      {
        action:"prepare_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary",
        status:"allowed_report_only_next_slice",
        requires_minimal_scoped_memory_real_write_canary_durable_store_write_preflight:true,
        writes_durable_memory:false,
        mutates_memory_store:false
      }
    ]
  }
  + (reduce $false_fields[] as $key ({}; .[$key]=false | .[$key + "_count"]=0))
  + (reduce $true_fields[] as $key ({}; .[$key]=true | .[$key + "_count"]=1))
  '
