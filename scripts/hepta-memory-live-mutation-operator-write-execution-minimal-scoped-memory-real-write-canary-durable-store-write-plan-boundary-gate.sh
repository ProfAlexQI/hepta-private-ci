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
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-tombstone-cleanup-acceptance-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-tombstone-cleanup-acceptance-boundary-gate.sh
)"

source_report_sha256="$(sha256_text "$SOURCE_JSON")"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_ready == true
    and $source.minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_performed == true
    and $source.minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_accepted == true
    and $source.accepted_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_fixture_count == 1
    and $source.blocked_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_fixture_count == 9
    and $source.approved_namespace == "hepta.memory.canary"
    and $source.approved_store == "wal-receipt-canary-artifact"
    and $source.approved_scope == "session"
    and ($source.tombstone_cleanup_acceptance_hash_sha256 | type == "string" and length > 0)
    and ($source.tombstone_cleanup_receipt_linkage_sha256 | type == "string" and length > 0)
    and $source.tombstone_cleanup_receipt_linkage_verified == true
    and $source.tombstone_cleanup_idempotency_guard_verified == true
    and $source.tombstone_cleanup_acceptance_result_accepted_count == 1
    and $source.tombstone_cleanup_plan_bound_count == 1
    and $source.tombstone_cleanup_target_bound_count == 1
    and $source.tombstone_cleanup_receipt_linkage_bound_count == 1
    and $source.tombstone_cleanup_idempotency_guard_accepted_count == 1
    and $source.tombstone_cleanup_executed_count == 0
    and $source.artifact_cleanup_performed_count == 0
    and $source.rollback_performed_count == 0
    and $source.tombstone_written_count == 0
    and $source.durable_memory_store_write_performed_count == 0
    and $source.memory_store_write_performed_count == 0
    and $source.tombstone_cleanup_executed == false
    and $source.tombstone_written == false
    and $source.durable_memory_store_write_performed == false
    and $source.durable_memory_store_read_performed == false
    and $source.durable_memory_store_rollback_performed == false
    and $source.memory_store_write_performed == false
    and $source.live_kg_write_performed == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.credential_read == false
    and $source.channel_send_performed == false
    and $source.external_send_performed == false
    and $source.release_artifact_written == false
    and $source.install_executed == false
    and $source.active_binary_mutated == false
    and $source.allowed_next_actions[1].action == "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary"
    and $source.allowed_next_actions[1].requires_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance == true
  ' >/dev/null

durable_store_write_target_id="hepta-minimal-scoped-memory-real-write-canary-durable-store-write-target-v1"
durable_store_target_store_id="hepta-memory-durable-store-canary-plan-only"
durable_store_write_payload_digest_sha256="$(
  jq -r --arg target_store "$durable_store_target_store_id" \
    '"minimal-scoped-memory-real-write-canary-durable-store-payload:v1:namespace=\(.approved_namespace):target-store=\($target_store):scope=\(.approved_scope):raw=false"' \
    <<<"$SOURCE_JSON" | shasum -a 256 | awk '{print $1}'
)"
durable_store_write_target_sha256="$(
  jq -r --arg target_store "$durable_store_target_store_id" \
    '"minimal-scoped-memory-real-write-canary-durable-store-target:v1:namespace=\(.approved_namespace):approved-store=\(.approved_store):target-store=\($target_store):scope=\(.approved_scope):source-acceptance=\(.tombstone_cleanup_acceptance_hash_sha256)"' \
    <<<"$SOURCE_JSON" | shasum -a 256 | awk '{print $1}'
)"
durable_store_write_envelope_sha256="$(
  printf '%s' "minimal-scoped-memory-real-write-canary-durable-store-write-envelope:v1:source=${source_report_sha256}:target=${durable_store_write_target_sha256}:payload=${durable_store_write_payload_digest_sha256}:write=false" \
    | shasum -a 256 | awk '{print $1}'
)"
durable_store_write_wal_receipt_plan_sha256="$(
  jq -r --arg envelope "$durable_store_write_envelope_sha256" \
    '"minimal-scoped-memory-real-write-canary-durable-store-wal-receipt-plan:v1:envelope=\($envelope):source-linkage=\(.tombstone_cleanup_receipt_linkage_sha256):wal-write=false:receipt-persist=false"' \
    <<<"$SOURCE_JSON" | shasum -a 256 | awk '{print $1}'
)"
durable_store_write_readback_plan_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-readback-plan:v1:wal-receipt=${durable_store_write_wal_receipt_plan_sha256}:read=false")"
durable_store_write_rollback_plan_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-rollback-plan:v1:readback-plan=${durable_store_write_readback_plan_sha256}:rollback=false")"
source_tombstone_cleanup_target_sha256="$(jq -r '.tombstone_cleanup_target_sha256 // ""' <<<"$SOURCE_JSON")"
durable_store_write_tombstone_cleanup_plan_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-tombstone-cleanup-plan:v1:rollback-plan=${durable_store_write_rollback_plan_sha256}:source-target=${source_tombstone_cleanup_target_sha256}:tombstone=false:cleanup=false")"
durable_store_write_operator_handoff_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-operator-handoff:v1:target=${durable_store_write_target_sha256}:envelope=${durable_store_write_envelope_sha256}:tombstone-plan=${durable_store_write_tombstone_cleanup_plan_sha256}:accepted=true")"
durable_store_write_plan_hash_sha256="$(
  jq -r --arg target_store "$durable_store_target_store_id" --arg handoff "$durable_store_write_operator_handoff_sha256" \
    '"minimal-scoped-memory-real-write-canary-durable-store-write-plan:v1:namespace=\(.approved_namespace):target-store=\($target_store):scope=\(.approved_scope):handoff=\($handoff):write=false"' \
    <<<"$SOURCE_JSON" | shasum -a 256 | awk '{print $1}'
)"
boundary_hash_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-write-plan-boundary:v1:source-ready=true:target=true:envelope=true:wal-receipt-plan=true:readback-plan=true:rollback-plan=true:fixtures=10:accepted=1:denials=30")"
policy_hash_sha256="$(sha256_text "minimal-scoped-memory-real-write-canary-durable-store-write-plan-policy:v1:accept-plan-only:no-durable-memory-write:no-memory-store-mutation:no-wal-write:no-receipt-persist:no-readback:no-rollback:no-tombstone:no-kg:no-provider:no-channel:no-release:no-install")"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary_gate" \
  --arg source_report_sha256 "$source_report_sha256" \
  --arg durable_store_write_target_id "$durable_store_write_target_id" \
  --arg durable_store_target_store_id "$durable_store_target_store_id" \
  --arg durable_store_write_payload_digest_sha256 "$durable_store_write_payload_digest_sha256" \
  --arg durable_store_write_target_sha256 "$durable_store_write_target_sha256" \
  --arg durable_store_write_envelope_sha256 "$durable_store_write_envelope_sha256" \
  --arg durable_store_write_wal_receipt_plan_sha256 "$durable_store_write_wal_receipt_plan_sha256" \
  --arg durable_store_write_readback_plan_sha256 "$durable_store_write_readback_plan_sha256" \
  --arg durable_store_write_rollback_plan_sha256 "$durable_store_write_rollback_plan_sha256" \
  --arg durable_store_write_tombstone_cleanup_plan_sha256 "$durable_store_write_tombstone_cleanup_plan_sha256" \
  --arg durable_store_write_operator_handoff_sha256 "$durable_store_write_operator_handoff_sha256" \
  --arg durable_store_write_plan_hash_sha256 "$durable_store_write_plan_hash_sha256" \
  --arg boundary_hash_sha256 "$boundary_hash_sha256" \
  --arg policy_hash_sha256 "$policy_hash_sha256" \
  --argjson source "$SOURCE_JSON" \
  '
  {
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    source_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary_ready:true,
    source_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_report_sha256:$source_report_sha256,
    source_accepted_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_fixture_count:$source.accepted_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_fixture_count,
    source_blocked_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_fixture_count:$source.blocked_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_fixture_count,
    source_tombstone_cleanup_acceptance_result_accepted_count:$source.tombstone_cleanup_acceptance_result_accepted_count,
    source_tombstone_cleanup_executed_count:$source.tombstone_cleanup_executed_count,
    source_artifact_cleanup_performed_count:$source.artifact_cleanup_performed_count,
    source_durable_memory_store_write_performed_count:$source.durable_memory_store_write_performed_count,
    source_memory_store_write_performed_count:$source.memory_store_write_performed_count,
    approved_namespace:$source.approved_namespace,
    approved_store:$source.approved_store,
    approved_scope:$source.approved_scope,
    source_tombstone_cleanup_acceptance_hash_sha256:$source.tombstone_cleanup_acceptance_hash_sha256,
    source_tombstone_cleanup_receipt_linkage_sha256:$source.tombstone_cleanup_receipt_linkage_sha256,
    source_tombstone_cleanup_target_sha256:$source.tombstone_cleanup_target_sha256,
    durable_store_write_target_id:$durable_store_write_target_id,
    durable_store_target_store_id:$durable_store_target_store_id,
    durable_store_write_payload_digest_sha256:$durable_store_write_payload_digest_sha256,
    durable_store_write_target_sha256:$durable_store_write_target_sha256,
    durable_store_write_envelope_sha256:$durable_store_write_envelope_sha256,
    durable_store_write_wal_receipt_plan_sha256:$durable_store_write_wal_receipt_plan_sha256,
    durable_store_write_readback_plan_sha256:$durable_store_write_readback_plan_sha256,
    durable_store_write_rollback_plan_sha256:$durable_store_write_rollback_plan_sha256,
    durable_store_write_tombstone_cleanup_plan_sha256:$durable_store_write_tombstone_cleanup_plan_sha256,
    durable_store_write_operator_handoff_sha256:$durable_store_write_operator_handoff_sha256,
    durable_store_write_plan_hash_sha256:$durable_store_write_plan_hash_sha256,
    minimal_scoped_memory_real_write_canary_durable_store_write_plan_ready:true,
    minimal_scoped_memory_real_write_canary_durable_store_write_plan_performed:true,
    minimal_scoped_memory_real_write_canary_durable_store_write_plan_accepted:true,
    durable_store_write_plan_performed:true,
    durable_store_write_plan_result_recorded:true,
    durable_store_write_plan_result_accepted:true,
    durable_store_target_bound:true,
    durable_store_write_envelope_bound:true,
    durable_store_write_payload_digest_bound:true,
    durable_store_write_wal_receipt_plan_bound:true,
    durable_store_write_readback_plan_bound:true,
    durable_store_write_rollback_plan_bound:true,
    durable_store_write_tombstone_cleanup_plan_bound:true,
    durable_store_write_operator_handoff_bound:true,
    durable_store_write_plan_receipt_linkage_verified:true,
    durable_store_write_plan_rollback_tombstone_cleanup_verified:true,
    required_minimal_scoped_memory_real_write_canary_durable_store_write_plan_surface_count:12,
    ready_minimal_scoped_memory_real_write_canary_durable_store_write_plan_surface_count:12,
    minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixture_count:10,
    accepted_minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixture_count:1,
    blocked_minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixture_count:9,
    minimal_scoped_memory_real_write_canary_durable_store_write_plan_accepted_count:1,
    durable_store_write_plan_authority_accepted_count:1,
    durable_store_write_plan_result_accepted_count:1,
    denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary_count:30,
    minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary_hash_sha256:$boundary_hash_sha256,
    minimal_scoped_memory_real_write_canary_durable_store_write_plan_policy_hash_sha256:$policy_hash_sha256,
    minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixtures:([range(0;10)] | map({fixture_index:., minimal_scoped_memory_real_write_canary_durable_store_write_plan_accepted:(. == 0)})),
    single_use_nonce_consumed:false,
    explicit_command_dispatched:false,
    durable_store_write_plan_executed:false,
    wal_write_performed:false,
    receipt_persisted:false,
    artifact_cleanup_performed:false,
    post_write_readback_performed:false,
    rollback_performed:false,
    rollback_executed:false,
    tombstone_cleanup_executed:false,
    tombstone_written:false,
    durable_memory_store_read_performed:false,
    durable_memory_store_write_performed:false,
    durable_memory_store_rollback_performed:false,
    memory_store_write_performed:false,
    memory_store_mutated:false,
    raw_payload_plaintext_recorded:false,
    live_kg_write_performed:false,
    provider_invoked:false,
    model_invoked:false,
    credential_read:false,
    channel_send_performed:false,
    external_send_performed:false,
    release_artifact_written:false,
    install_executed:false,
    service_restarted:false,
    active_binary_mutated:false,
    side_effects:{
      durable_store_write_plan_performed:true,
      durable_store_write_plan_result_accepted:true,
      durable_store_write_plan_executed:false,
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
        action:"run_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary_require_live_gate",
        status:"allowed_verification_only",
        writes_durable_memory:false,
        mutates_memory_store:false
      },
      {
        action:"prepare_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary",
        status:"allowed_report_only_next_slice",
        requires_minimal_scoped_memory_real_write_canary_durable_store_write_plan:true,
        writes_durable_memory:false,
        mutates_memory_store:false
      }
    ]
  }'
