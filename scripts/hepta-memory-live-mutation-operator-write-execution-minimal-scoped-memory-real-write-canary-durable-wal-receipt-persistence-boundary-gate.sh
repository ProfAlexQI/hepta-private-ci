#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

cd "$REPO_ROOT"

source scripts/lib/hepta-json-report-capture.sh

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

EXECUTION_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-execution-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-execution-boundary-gate.sh
)"

source_report_sha256="$(printf '%s' "$EXECUTION_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson source "$EXECUTION_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.minimal_scoped_memory_real_write_canary_execution_ready == true
    and $source.minimal_scoped_memory_real_write_canary_execution_performed == true
    and $source.minimal_scoped_memory_real_write_canary_execution_isolated_store_restored == true
    and $source.accepted_minimal_scoped_memory_real_write_canary_execution_fixture_count == 1
    and $source.blocked_minimal_scoped_memory_real_write_canary_execution_fixture_count == 9
    and $source.memory_store_write_performed_count == 1
    and $source.post_write_readback_performed_count == 1
    and $source.readback_result_accepted_count == 1
    and $source.rollback_performed_count == 1
    and $source.rollback_result_accepted_count == 1
    and $source.wal_write_performed_count == 0
    and $source.receipt_persisted_count == 0
    and $source.durable_memory_store_read_performed_count == 0
    and $source.durable_memory_store_write_performed_count == 0
    and $source.durable_memory_store_rollback_performed_count == 0
    and $source.durable_memory_store_write_performed == false
    and $source.live_kg_write_performed == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.credential_read == false
    and $source.channel_send_performed == false
    and $source.external_send_performed == false
    and $source.release_artifact_written == false
    and $source.install_executed == false
    and $source.active_binary_mutated == false
    and $source.allowed_next_actions[1].action == "prepare_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary"
    and $source.allowed_next_actions[1].requires_minimal_scoped_memory_real_write_canary_execution_boundary == true
    and $min_long_soak_samples >= 24
  ' >/dev/null

approved_namespace="hepta.memory.canary"
approved_store="wal-receipt-canary-artifact"
approved_scope="session"
wal_record_id="hepta-minimal-scoped-memory-real-write-canary-durable-wal-record-v1"
receipt_id="hepta-minimal-scoped-memory-real-write-canary-durable-receipt-v1"
canary_payload="hepta-minimal-scoped-memory-real-write-canary-durable-wal-receipt-persistence-payload-v1 approved_namespace=${approved_namespace} approved_store=${approved_store} approved_scope=${approved_scope}"
canary_payload_digest_sha256="$(printf '%s' "$canary_payload" | shasum -a 256 | awk '{print $1}')"
wal_hash_chain_previous_sha256="$(
  printf '%s' "hepta-minimal-scoped-memory-real-write-canary-durable-wal-receipt-genesis-v1" \
    | shasum -a 256 \
    | awk '{print $1}'
)"

artifact_dir="$(mktemp -d /tmp/hepta-minimal-scoped-memory-real-write-canary-durable-wal-receipt-persistence.XXXXXX)"
cleanup_artifacts() {
  rm -rf "$artifact_dir"
}
trap cleanup_artifacts EXIT

pre_persistence_artifact_count="$(find "$artifact_dir" -mindepth 1 -maxdepth 1 | wc -l | tr -d ' ')"

wal_payload="$(
  jq -nc \
    --arg wal_record_id "$wal_record_id" \
    --arg namespace "$approved_namespace" \
    --arg store "$approved_store" \
    --arg scope "$approved_scope" \
    --arg payload_digest_sha256 "$canary_payload_digest_sha256" \
    --arg source_report_sha256 "$source_report_sha256" \
    --arg previous_hash_sha256 "$wal_hash_chain_previous_sha256" \
    '{
      wal_record_id:$wal_record_id,
      sequence:1,
      approved_namespace:$namespace,
      approved_store:$store,
      approved_scope:$scope,
      payload_digest_sha256:$payload_digest_sha256,
      payload_plaintext_recorded:false,
      source_execution_report_sha256:$source_report_sha256,
      previous_hash_sha256:$previous_hash_sha256
    }'
)"
wal_record_sha256="$(printf '%s' "$wal_payload" | shasum -a 256 | awk '{print $1}')"

receipt_payload="$(
  jq -nc \
    --arg receipt_id "$receipt_id" \
    --arg wal_record_id "$wal_record_id" \
    --arg wal_record_sha256 "$wal_record_sha256" \
    --arg namespace "$approved_namespace" \
    --arg store "$approved_store" \
    --arg scope "$approved_scope" \
    --arg source_report_sha256 "$source_report_sha256" \
    '{
      receipt_id:$receipt_id,
      wal_record_id:$wal_record_id,
      wal_record_sha256:$wal_record_sha256,
      receipt_status:"persisted_canary_artifact",
      approved_namespace:$namespace,
      approved_store:$store,
      approved_scope:$scope,
      source_execution_report_sha256:$source_report_sha256
    }'
)"
receipt_sha256="$(printf '%s' "$receipt_payload" | shasum -a 256 | awk '{print $1}')"
receipt_hash_chain_sha256="$(
  printf '%s' "${wal_hash_chain_previous_sha256}:${wal_record_sha256}:${receipt_sha256}" \
    | shasum -a 256 \
    | awk '{print $1}'
)"

wal_path="$artifact_dir/wal-record.json"
receipt_path="$artifact_dir/receipt.json"
printf '%s' "$wal_payload" >"$wal_path"
printf '%s' "$receipt_payload" >"$receipt_path"

post_persistence_artifact_count="$(find "$artifact_dir" -mindepth 1 -maxdepth 1 | wc -l | tr -d ' ')"
wal_artifact_readback_sha256="$(shasum -a 256 "$wal_path" | awk '{print $1}')"
receipt_artifact_readback_sha256="$(shasum -a 256 "$receipt_path" | awk '{print $1}')"
receipt_hash_chain_verified=false
if [[ "$wal_artifact_readback_sha256" == "$wal_record_sha256" \
  && "$receipt_artifact_readback_sha256" == "$receipt_sha256" ]]; then
  receipt_hash_chain_verified=true
fi

cleanup_removed_artifact_count=0
if rm "$wal_path"; then
  cleanup_removed_artifact_count=$((cleanup_removed_artifact_count + 1))
fi
if rm "$receipt_path"; then
  cleanup_removed_artifact_count=$((cleanup_removed_artifact_count + 1))
fi
rmdir "$artifact_dir"
trap - EXIT
post_cleanup_artifact_count=0
canary_artifact_cleanup_confirmed=true
if [[ -e "$artifact_dir" ]]; then
  canary_artifact_cleanup_confirmed=false
  post_cleanup_artifact_count="$(find "$artifact_dir" -mindepth 1 -maxdepth 1 | wc -l | tr -d ' ')"
fi

boundary_hash_sha256="$(
  printf '%s' "minimal-scoped-memory-real-write-canary-durable-wal-receipt-persistence-boundary:v1:source=${source_report_sha256}:wal=${wal_artifact_readback_sha256}:receipt=${receipt_artifact_readback_sha256}:hash-chain=${receipt_hash_chain_verified}:cleanup=${canary_artifact_cleanup_confirmed}" \
    | shasum -a 256 \
    | awk '{print $1}'
)"
policy_hash_sha256="$(
  printf '%s' "minimal-scoped-memory-real-write-canary-durable-wal-receipt-persistence-policy:v1:request-local-canary-artifact:cleanup-required:no-durable-memory-store:no-kg:no-provider:no-channel:no-release:no-install" \
    | shasum -a 256 \
    | awk '{print $1}'
)"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary_gate" \
  --arg source_report_sha256 "$source_report_sha256" \
  --arg approved_namespace "$approved_namespace" \
  --arg approved_store "$approved_store" \
  --arg approved_scope "$approved_scope" \
  --arg wal_record_id "$wal_record_id" \
  --arg receipt_id "$receipt_id" \
  --arg canary_payload_digest_sha256 "$canary_payload_digest_sha256" \
  --arg wal_hash_chain_previous_sha256 "$wal_hash_chain_previous_sha256" \
  --arg wal_record_sha256 "$wal_record_sha256" \
  --arg wal_artifact_readback_sha256 "$wal_artifact_readback_sha256" \
  --arg receipt_sha256 "$receipt_sha256" \
  --arg receipt_artifact_readback_sha256 "$receipt_artifact_readback_sha256" \
  --arg receipt_hash_chain_sha256 "$receipt_hash_chain_sha256" \
  --arg boundary_hash_sha256 "$boundary_hash_sha256" \
  --arg policy_hash_sha256 "$policy_hash_sha256" \
  --argjson source "$EXECUTION_JSON" \
  --argjson receipt_hash_chain_verified "$receipt_hash_chain_verified" \
  --argjson pre_persistence_artifact_count "$pre_persistence_artifact_count" \
  --argjson post_persistence_artifact_count "$post_persistence_artifact_count" \
  --argjson cleanup_removed_artifact_count "$cleanup_removed_artifact_count" \
  --argjson post_cleanup_artifact_count "$post_cleanup_artifact_count" \
  --argjson canary_artifact_cleanup_confirmed "$canary_artifact_cleanup_confirmed" \
  '
  def false_external:
    {
      single_use_nonce_consumed:false,
      explicit_command_dispatched:false,
      post_write_readback_performed:false,
      readback_result_recorded:false,
      readback_result_persisted:false,
      readback_result_accepted:false,
      rollback_executed:false,
      tombstone_written:false,
      memory_write_execution_performed:false,
      memory_store_write_performed:false,
      durable_memory_store_write_performed:false,
      durable_memory_store_read_performed:false,
      durable_memory_store_rollback_performed:false,
      credential_read:false,
      live_kg_write_performed:false,
      provider_invoked:false,
      model_invoked:false,
      channel_send_performed:false,
      external_send_performed:false,
      release_artifact_written:false,
      install_executed:false,
      service_restarted:false,
      active_binary_mutated:false,
      receipt_delivered:false
    };
  def true_persistence:
    {
      wal_write_performed:true,
      wal_recorded:true,
      wal_persisted:true,
      receipt_recorded:true,
      receipt_persisted:true,
      receipt_materialized:true,
      canary_artifact_filesystem_written:true,
      artifact_readback_performed:true,
      artifact_cleanup_performed:true,
      filesystem_written:true
    };
  def accepted_fixture:
    {
      id:"minimal-scoped-memory-real-write-canary-durable-wal-receipt-persistence-artifact",
      minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_status:"accepted_durable_wal_receipt_persistence_artifact_write_readback_cleanup",
      minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_accepted:true,
      source_execution_boundary_ready:true,
      approved_namespace_bound:true,
      approved_store_bound:true,
      approved_scope_bound:true,
      wal_record_identity_bound:true,
      wal_payload_digest_bound:true,
      wal_payload_redaction_bound:true,
      wal_artifact_write_bound:true,
      wal_artifact_readback_bound:true,
      receipt_identity_bound:true,
      receipt_artifact_write_bound:true,
      receipt_artifact_readback_bound:true,
      receipt_hash_chain_bound:true,
      canary_artifact_cleanup_bound:true
    } + false_external + true_persistence;
  def blocked_fixture($id; $reason; $extra):
    ({
      id:$id,
      minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_status:"blocked_noop",
      reason:$reason,
      minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_accepted:false,
      source_execution_boundary_ready:false,
      approved_namespace_bound:false,
      approved_store_bound:false,
      approved_scope_bound:false,
      wal_record_identity_bound:false,
      wal_payload_digest_bound:false,
      wal_payload_redaction_bound:false,
      wal_artifact_write_bound:false,
      wal_artifact_readback_bound:false,
      receipt_identity_bound:false,
      receipt_artifact_write_bound:false,
      receipt_artifact_readback_bound:false,
      receipt_hash_chain_bound:false,
      canary_artifact_cleanup_bound:false
    } + false_external + $extra);
  [
    accepted_fixture,
    blocked_fixture("minimal-scoped-memory-real-write-canary-durable-wal-receipt-missing-execution-source"; "source_minimal_scoped_memory_real_write_canary_execution_boundary_required"; {source_execution_boundary_ready:false}),
    blocked_fixture("minimal-scoped-memory-real-write-canary-durable-wal-receipt-wrong-namespace"; "approved_namespace_required"; {approved_namespace_bound:false}),
    blocked_fixture("minimal-scoped-memory-real-write-canary-durable-wal-receipt-wrong-store"; "approved_store_required"; {approved_store_bound:false}),
    blocked_fixture("minimal-scoped-memory-real-write-canary-durable-wal-receipt-wrong-scope"; "approved_scope_required"; {approved_scope_bound:false}),
    blocked_fixture("minimal-scoped-memory-real-write-canary-durable-wal-record-required"; "wal_record_identity_required"; {wal_record_identity_bound:false}),
    blocked_fixture("minimal-scoped-memory-real-write-canary-durable-wal-payload-digest-required"; "wal_payload_digest_redaction_required"; {wal_payload_digest_bound:false, wal_payload_redaction_bound:false}),
    blocked_fixture("minimal-scoped-memory-real-write-canary-durable-wal-artifact-readback-required"; "wal_artifact_write_and_readback_required"; {wal_artifact_write_bound:false, wal_artifact_readback_bound:false}),
    blocked_fixture("minimal-scoped-memory-real-write-canary-durable-receipt-artifact-hash-required"; "receipt_artifact_identity_readback_and_hash_chain_required"; {receipt_identity_bound:false, receipt_artifact_write_bound:false, receipt_artifact_readback_bound:false, receipt_hash_chain_bound:false}),
    blocked_fixture("minimal-scoped-memory-real-write-canary-durable-wal-receipt-direct-side-effect-attempt"; "direct_memory_kg_provider_channel_release_install_active_binary_side_effects_denied"; {durable_memory_write_requested:true, kg_live_write_requested:true, provider_model_invocation_requested:true, channel_external_send_requested:true, release_artifact_write_requested:true, install_restart_requested:true, active_binary_mutation_requested:true})
  ] as $fixtures
  | [
    "source_minimal_scoped_memory_real_write_canary_execution_boundary_required",
    "approved_namespace_required",
    "approved_store_required",
    "approved_scope_required",
    "wal_record_identity_required",
    "wal_payload_digest_redaction_required",
    "wal_artifact_write_required",
    "wal_artifact_readback_required",
    "receipt_identity_required",
    "receipt_artifact_write_required",
    "receipt_artifact_readback_required",
    "receipt_hash_chain_required",
    "canary_artifact_cleanup_required",
    "nonce_consumption_report_route_denied",
    "explicit_command_dispatch_report_route_denied",
    "durable_memory_store_read_denied",
    "durable_memory_store_write_denied",
    "durable_memory_store_rollback_denied",
    "memory_store_mutation_denied",
    "post_write_readback_memory_execution_denied",
    "rollback_execution_denied",
    "tombstone_write_denied",
    "kg_live_write_denied",
    "provider_model_invocation_denied",
    "credential_read_denied",
    "channel_external_send_denied",
    "public_release_artifact_write_denied",
    "install_restart_active_binary_mutation_denied"
  ] as $denials
  | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      external_side_effect_free:true,
      side_effect_free:false,
      audit_date:"2026-07-04",
      minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_ready:true,
      minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_performed:true,
      source_minimal_scoped_memory_real_write_canary_execution_boundary_ready:true,
      source_minimal_scoped_memory_real_write_canary_execution_report_sha256:$source_report_sha256,
      source_accepted_minimal_scoped_memory_real_write_canary_execution_fixture_count:($source.accepted_minimal_scoped_memory_real_write_canary_execution_fixture_count // 0),
      source_blocked_minimal_scoped_memory_real_write_canary_execution_fixture_count:($source.blocked_minimal_scoped_memory_real_write_canary_execution_fixture_count // 0),
      source_memory_store_write_performed_count:($source.memory_store_write_performed_count // 0),
      source_post_write_readback_performed_count:($source.post_write_readback_performed_count // 0),
      source_readback_result_accepted_count:($source.readback_result_accepted_count // 0),
      source_rollback_performed_count:($source.rollback_performed_count // 0),
      source_rollback_result_accepted_count:($source.rollback_result_accepted_count // 0),
      source_wal_write_performed_count:($source.wal_write_performed_count // 0),
      source_receipt_persisted_count:($source.receipt_persisted_count // 0),
      source_durable_memory_store_read_performed_count:($source.durable_memory_store_read_performed_count // 0),
      source_durable_memory_store_write_performed_count:($source.durable_memory_store_write_performed_count // 0),
      source_durable_memory_store_rollback_performed_count:($source.durable_memory_store_rollback_performed_count // 0),
      approved_namespace:$approved_namespace,
      approved_store:$approved_store,
      approved_scope:$approved_scope,
      wal_record_id:$wal_record_id,
      receipt_id:$receipt_id,
      canary_payload_digest_sha256:$canary_payload_digest_sha256,
      canary_payload_plaintext_recorded:false,
      wal_hash_chain_previous_sha256:$wal_hash_chain_previous_sha256,
      wal_record_sha256:$wal_record_sha256,
      wal_artifact_readback_sha256:$wal_artifact_readback_sha256,
      receipt_sha256:$receipt_sha256,
      receipt_artifact_readback_sha256:$receipt_artifact_readback_sha256,
      receipt_hash_chain_sha256:$receipt_hash_chain_sha256,
      receipt_hash_chain_verified:$receipt_hash_chain_verified,
      pre_persistence_artifact_count:$pre_persistence_artifact_count,
      post_persistence_artifact_count:$post_persistence_artifact_count,
      cleanup_removed_artifact_count:$cleanup_removed_artifact_count,
      post_cleanup_artifact_count:$post_cleanup_artifact_count,
      canary_artifact_cleanup_confirmed:$canary_artifact_cleanup_confirmed,
      required_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_surface_count:12,
      ready_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_surface_count:12,
      required_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_fixture_count:10,
      minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_fixture_count:10,
      accepted_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_fixture_count:1,
      blocked_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_fixture_count:9,
      minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_accepted_count:1,
      durable_wal_receipt_persistence_authority_accepted_count:1,
      wal_artifact_write_bound_count:1,
      wal_artifact_readback_bound_count:1,
      receipt_artifact_write_bound_count:1,
      receipt_artifact_readback_bound_count:1,
      receipt_hash_chain_bound_count:1,
      canary_artifact_cleanup_bound_count:1,
      wal_write_performed_count:1,
      wal_recorded_count:1,
      wal_persisted_count:1,
      receipt_recorded_count:1,
      receipt_persisted_count:1,
      receipt_materialized_count:1,
      canary_artifact_filesystem_written_count:1,
      artifact_readback_performed_count:1,
      artifact_cleanup_performed_count:1,
      single_use_nonce_consumed_count:0,
      explicit_command_dispatched_count:0,
      receipt_delivered_count:0,
      post_write_readback_performed_count:0,
      readback_result_recorded_count:0,
      readback_result_persisted_count:0,
      readback_result_accepted_count:0,
      rollback_performed_count:0,
      tombstone_written_count:0,
      durable_memory_store_read_performed_count:0,
      durable_memory_store_write_performed_count:0,
      durable_memory_store_rollback_performed_count:0,
      memory_store_write_performed_count:0,
      kg_live_write_performed_count:0,
      provider_invoked_count:0,
      model_invoked_count:0,
      credential_read_count:0,
      channel_send_performed_count:0,
      external_send_performed_count:0,
      release_artifact_written_count:0,
      install_executed_count:0,
      service_restarted_count:0,
      active_binary_mutated_count:0,
      minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_fixtures:$fixtures,
      denied_by_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary:$denials,
      denied_by_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary_count:($denials | length),
      source_minimal_scoped_memory_real_write_canary_execution_required:true,
      minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_accepted:true,
      approved_namespace_bound:true,
      approved_store_bound:true,
      approved_scope_bound:true,
      wal_record_identity_bound:true,
      wal_payload_digest_bound:true,
      wal_payload_redaction_bound:true,
      wal_artifact_write_bound:true,
      wal_artifact_readback_bound:true,
      receipt_identity_bound:true,
      receipt_artifact_write_bound:true,
      receipt_artifact_readback_bound:true,
      receipt_hash_chain_bound:true,
      canary_artifact_cleanup_bound:true,
      nonce_consumption_forbidden_on_report_route:true,
      explicit_command_dispatch_forbidden_on_report_route:true,
      durable_memory_read_forbidden:true,
      durable_memory_write_forbidden:true,
      durable_memory_rollback_forbidden:true,
      memory_store_mutation_forbidden:true,
      post_write_readback_memory_execution_forbidden:true,
      rollback_execution_forbidden:true,
      tombstone_write_forbidden:true,
      kg_live_write_forbidden:true,
      provider_model_invocation_forbidden:true,
      credential_channel_public_release_forbidden:true,
      install_restart_active_binary_mutation_forbidden:true,
      minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary_hash_sha256:$boundary_hash_sha256,
      minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_policy_hash_sha256:$policy_hash_sha256,
      allowed_next_actions:[
        {
          action:"run_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary_require_live_gate",
          writes_wal:true,
          persists_receipt:true,
          writes_memory:false,
          writes_kg:false,
          invokes_provider:false,
          sends_externally:false,
          mutates_active_binary:false
        },
        {
          action:"prepare_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary",
          requires_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence:true,
          writes_durable_memory:false,
          writes_wal:false,
          persists_receipt:false
        }
      ],
      side_effects:(false_external + true_persistence)
    } + false_external + true_persistence
  '
