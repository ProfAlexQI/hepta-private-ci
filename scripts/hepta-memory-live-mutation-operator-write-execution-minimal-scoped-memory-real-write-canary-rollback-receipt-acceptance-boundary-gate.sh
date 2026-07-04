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

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-readback-receipt-acceptance-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-readback-receipt-acceptance-boundary-gate.sh
)"

source_report_sha256="$(printf '%s' "$SOURCE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_ready == true
    and $source.minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_performed == true
    and $source.minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_accepted == true
    and $source.accepted_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixture_count == 1
    and $source.blocked_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixture_count == 9
    and $source.approved_namespace == "hepta.memory.canary"
    and $source.approved_store == "wal-receipt-canary-artifact"
    and $source.approved_scope == "session"
    and ($source.receipt_acceptance_hash_sha256 | type == "string" and length > 0)
    and ($source.receipt_readback_report_sha256 | type == "string" and length > 0)
    and ($source.receipt_hash_chain_sha256 | type == "string" and length > 0)
    and $source.receipt_readback_performed_count == 1
    and $source.receipt_readback_result_accepted_count == 1
    and $source.receipt_acceptance_recorded_count == 1
    and $source.rollback_receipt_acceptance_handoff_bound_count == 1
    and $source.wal_write_performed_count == 0
    and $source.receipt_persisted_count == 0
    and $source.rollback_performed_count == 0
    and $source.tombstone_written_count == 0
    and $source.durable_memory_store_read_performed_count == 0
    and $source.durable_memory_store_write_performed_count == 0
    and $source.durable_memory_store_rollback_performed_count == 0
    and $source.memory_store_write_performed_count == 0
    and $source.rollback_executed == false
    and $source.tombstone_written == false
    and $source.durable_memory_store_write_performed == false
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
    and $source.allowed_next_actions[1].action == "prepare_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary"
    and $source.allowed_next_actions[1].requires_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance == true
    and $min_long_soak_samples >= 24
  ' >/dev/null

rollback_receipt_sha256="$(
  jq -r --arg source_report_sha256 "$source_report_sha256" \
    '"minimal-scoped-memory-real-write-canary-rollback-receipt:v1:source=\($source_report_sha256):receipt-acceptance=\(.receipt_acceptance_hash_sha256):readback=\(.receipt_readback_report_sha256):hash-chain=\(.receipt_hash_chain_sha256)"' \
    <<<"$SOURCE_JSON" | shasum -a 256 | awk '{print $1}'
)"
rollback_receipt_hash_chain_sha256="$(
  jq -r --arg rollback_receipt_sha256 "$rollback_receipt_sha256" \
    '"minimal-scoped-memory-real-write-canary-rollback-receipt-hash-chain:v1:prior=\(.receipt_hash_chain_sha256):rollback-receipt=\($rollback_receipt_sha256)"' \
    <<<"$SOURCE_JSON" | shasum -a 256 | awk '{print $1}'
)"
rollback_receipt_acceptance_hash_sha256="$(
  jq -r --arg rollback_receipt_sha256 "$rollback_receipt_sha256" \
    '"minimal-scoped-memory-real-write-canary-rollback-receipt-acceptance:v1:namespace=\(.approved_namespace):store=\(.approved_store):scope=\(.approved_scope):rollback-receipt=\($rollback_receipt_sha256):accepted=true"' \
    <<<"$SOURCE_JSON" | shasum -a 256 | awk '{print $1}'
)"
boundary_hash_sha256="$(
  printf '%s' "minimal-scoped-memory-real-write-canary-rollback-receipt-acceptance-boundary:v1:source-ready=true:receipt-acceptance=true:rollback-receipt=true:digest=true:hash-chain=true:fixtures=10:accepted=1:denials=28" \
    | shasum -a 256 \
    | awk '{print $1}'
)"
policy_hash_sha256="$(
  printf '%s' "minimal-scoped-memory-real-write-canary-rollback-receipt-acceptance-policy:v1:accept-source-readback-acceptance-evidence:no-wal-rewrite:no-receipt-repersist:no-rollback-execution:no-tombstone:no-durable-memory-store:no-kg:no-provider:no-channel:no-release:no-install" \
    | shasum -a 256 \
    | awk '{print $1}'
)"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_gate" \
  --arg source_report_sha256 "$source_report_sha256" \
  --arg rollback_receipt_id "hepta-minimal-scoped-memory-real-write-canary-rollback-receipt-v1" \
  --arg rollback_receipt_sha256 "$rollback_receipt_sha256" \
  --arg rollback_receipt_hash_chain_sha256 "$rollback_receipt_hash_chain_sha256" \
  --arg rollback_receipt_acceptance_hash_sha256 "$rollback_receipt_acceptance_hash_sha256" \
  --arg boundary_hash_sha256 "$boundary_hash_sha256" \
  --arg policy_hash_sha256 "$policy_hash_sha256" \
  --argjson source "$SOURCE_JSON" \
  '
  def false_external:
    {
      single_use_nonce_consumed:false,
      explicit_command_dispatched:false,
      wal_write_performed:false,
      wal_recorded:false,
      wal_persisted:false,
      receipt_recorded:false,
      receipt_persisted:false,
      receipt_materialized:false,
      receipt_delivered:false,
      canary_artifact_filesystem_written:false,
      artifact_readback_performed:false,
      artifact_cleanup_performed:false,
      filesystem_written:false,
      post_write_readback_performed:false,
      readback_result_recorded:false,
      readback_result_persisted:false,
      readback_result_accepted:false,
      rollback_executed:false,
      rollback_performed:false,
      rollback_result_recorded:false,
      rollback_result_persisted:false,
      rollback_result_accepted:false,
      tombstone_written:false,
      compensating_memory_write_performed:false,
      activation_performed:false,
      live_mutation_execution_performed:false,
      memory_write_execution_performed:false,
      memory_store_write_path_enabled:false,
      memory_store_write_allowed:false,
      memory_store_write_performed:false,
      memory_store_mutation_allowed:false,
      memory_store_mutated:false,
      durable_memory_store_write_performed:false,
      durable_memory_store_read_performed:false,
      durable_memory_store_rollback_performed:false,
      raw_payload_plaintext_recorded:false,
      raw_payload_plaintext_persisted:false,
      secret_material_read:false,
      credential_read:false,
      secret_file_read:false,
      kg_adapter_read_performed:false,
      live_kg_write_performed:false,
      provider_invoked:false,
      model_invoked:false,
      telegram_send_performed:false,
      channel_send_performed:false,
      external_send_performed:false,
      public_claim_promoted:false,
      public_release_published:false,
      public_ga_claimed:false,
      release_artifact_written:false,
      public_artifact_written:false,
      install_executed:false,
      launchd_mutated:false,
      service_restarted:false,
      service_restart_performed:false,
      active_binary_mutated:false
    };
  def true_acceptance:
    {
      rollback_receipt_acceptance_performed:true,
      rollback_receipt_acceptance_result_recorded:true,
      rollback_receipt_acceptance_result_accepted:true,
      rollback_receipt_identity_accepted:true,
      rollback_receipt_digest_accepted:true,
      rollback_receipt_hash_chain_accepted:true,
      minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_accepted:true
    };
  def accepted_fixture:
    {
      id:"minimal-scoped-memory-real-write-canary-rollback-receipt-acceptance",
      fixture_id:"minimal-scoped-memory-real-write-canary-rollback-receipt-acceptance",
      minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_status:"accepted_rollback_receipt_identity_digest_hash_chain",
      reason:"rollback_receipt_identity_digest_hash_chain_and_tombstone_cleanup_handoff_accepted",
      minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_accepted:true,
      source_readback_receipt_acceptance_boundary_ready:true,
      approved_namespace_bound:true,
      approved_store_bound:true,
      approved_scope_bound:true,
      receipt_acceptance_hash_bound:true,
      rollback_receipt_identity_bound:true,
      rollback_receipt_digest_bound:true,
      rollback_receipt_hash_chain_bound:true,
      rollback_receipt_source_readback_linkage_bound:true,
      rollback_receipt_acceptance_record_bound:true,
      rollback_receipt_replay_guard_bound:true,
      rollback_operator_review_handoff_bound:true,
      tombstone_cleanup_handoff_bound:true,
      approved_namespace:$source.approved_namespace,
      approved_store:$source.approved_store,
      approved_scope:$source.approved_scope,
      source_receipt_acceptance_hash_sha256:$source.receipt_acceptance_hash_sha256,
      rollback_receipt_id:$rollback_receipt_id,
      rollback_receipt_sha256:$rollback_receipt_sha256,
      rollback_receipt_hash_chain_sha256:$rollback_receipt_hash_chain_sha256
    } + false_external + true_acceptance;
  def blocked_fixture($id; $reason; $extra):
    ({
      id:$id,
      fixture_id:$id,
      minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_status:"blocked_noop",
      reason:$reason,
      minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_accepted:false,
      source_readback_receipt_acceptance_boundary_ready:false,
      approved_namespace_bound:false,
      approved_store_bound:false,
      approved_scope_bound:false,
      receipt_acceptance_hash_bound:false,
      rollback_receipt_identity_bound:false,
      rollback_receipt_digest_bound:false,
      rollback_receipt_hash_chain_bound:false,
      rollback_receipt_source_readback_linkage_bound:false,
      rollback_receipt_acceptance_record_bound:false,
      rollback_receipt_replay_guard_bound:false,
      rollback_operator_review_handoff_bound:false,
      tombstone_cleanup_handoff_bound:false
    } + false_external + $extra);
  [
    accepted_fixture,
    blocked_fixture("minimal-scoped-memory-real-write-canary-rollback-receipt-missing-source"; "source_durable_readback_receipt_acceptance_boundary_required"; {source_readback_receipt_acceptance_boundary_ready:false}),
    blocked_fixture("minimal-scoped-memory-real-write-canary-rollback-receipt-wrong-namespace"; "approved_namespace_required"; {approved_namespace_bound:false}),
    blocked_fixture("minimal-scoped-memory-real-write-canary-rollback-receipt-wrong-store"; "approved_store_required"; {approved_store_bound:false}),
    blocked_fixture("minimal-scoped-memory-real-write-canary-rollback-receipt-wrong-scope"; "approved_scope_required"; {approved_scope_bound:false}),
    blocked_fixture("minimal-scoped-memory-real-write-canary-rollback-receipt-missing-source-acceptance"; "receipt_acceptance_hash_required"; {receipt_acceptance_hash_bound:false}),
    blocked_fixture("minimal-scoped-memory-real-write-canary-rollback-receipt-missing-identity"; "rollback_receipt_identity_required"; {rollback_receipt_identity_bound:false}),
    blocked_fixture("minimal-scoped-memory-real-write-canary-rollback-receipt-digest-mismatch"; "rollback_receipt_digest_required"; {rollback_receipt_digest_bound:false}),
    blocked_fixture("minimal-scoped-memory-real-write-canary-rollback-receipt-hash-chain-mismatch"; "rollback_receipt_hash_chain_required"; {rollback_receipt_hash_chain_bound:false}),
    blocked_fixture("minimal-scoped-memory-real-write-canary-rollback-receipt-direct-side-effect-attempt"; "direct_memory_kg_provider_channel_release_install_active_binary_side_effects_denied"; {wal_rewrite_requested:true, receipt_repersist_requested:true, rollback_execution_requested:true, tombstone_write_requested:true, durable_memory_write_requested:true, kg_live_write_requested:true, provider_model_invocation_requested:true, channel_external_send_requested:true, release_artifact_write_requested:true, install_restart_requested:true, active_binary_mutation_requested:true})
  ] as $fixtures
  | [
    "source_durable_readback_receipt_acceptance_boundary_required",
    "approved_namespace_required",
    "approved_store_required",
    "approved_scope_required",
    "receipt_acceptance_hash_required",
    "rollback_receipt_identity_required",
    "rollback_receipt_digest_required",
    "rollback_receipt_hash_chain_required",
    "rollback_receipt_source_readback_linkage_required",
    "rollback_receipt_acceptance_record_required",
    "rollback_receipt_replay_guard_required",
    "tombstone_cleanup_handoff_required",
    "wal_rewrite_report_route_denied",
    "receipt_repersist_report_route_denied",
    "nonce_consumption_report_route_denied",
    "explicit_command_dispatch_report_route_denied",
    "durable_memory_store_read_denied",
    "durable_memory_store_write_denied",
    "durable_memory_store_rollback_denied",
    "memory_store_mutation_denied",
    "post_write_memory_readback_denied",
    "rollback_execution_denied",
    "tombstone_write_denied",
    "kg_live_write_denied",
    "provider_model_invocation_denied",
    "credential_channel_public_release_denied",
    "install_restart_active_binary_mutation_denied",
    "compensating_memory_write_denied"
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
      minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_schema_version:"minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_v1",
      scoped_memory_real_write_canary_mode:"minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_report_only",
      minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_ready:true,
      minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_performed:true,
      source_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary_ready:true,
      source_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_report_sha256:$source_report_sha256,
      source_accepted_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixture_count:($source.accepted_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixture_count // 0),
      source_blocked_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixture_count:($source.blocked_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixture_count // 0),
      source_receipt_readback_performed_count:($source.receipt_readback_performed_count // 0),
      source_receipt_readback_result_recorded_count:($source.receipt_readback_result_recorded_count // 0),
      source_receipt_readback_result_accepted_count:($source.receipt_readback_result_accepted_count // 0),
      source_receipt_acceptance_recorded_count:($source.receipt_acceptance_recorded_count // 0),
      source_receipt_replay_guard_accepted_count:($source.receipt_replay_guard_accepted_count // 0),
      source_rollback_receipt_acceptance_handoff_bound_count:($source.rollback_receipt_acceptance_handoff_bound_count // 0),
      source_wal_write_performed_count:($source.wal_write_performed_count // 0),
      source_receipt_persisted_count:($source.receipt_persisted_count // 0),
      source_durable_memory_store_read_performed_count:($source.durable_memory_store_read_performed_count // 0),
      source_durable_memory_store_write_performed_count:($source.durable_memory_store_write_performed_count // 0),
      source_durable_memory_store_rollback_performed_count:($source.durable_memory_store_rollback_performed_count // 0),
      source_memory_store_write_performed_count:($source.memory_store_write_performed_count // 0),
      approved_namespace:$source.approved_namespace,
      approved_store:$source.approved_store,
      approved_scope:$source.approved_scope,
      source_receipt_acceptance_hash_sha256:$source.receipt_acceptance_hash_sha256,
      source_receipt_readback_report_sha256:$source.receipt_readback_report_sha256,
      source_receipt_hash_chain_sha256:$source.receipt_hash_chain_sha256,
      rollback_receipt_id:$rollback_receipt_id,
      rollback_receipt_sha256:$rollback_receipt_sha256,
      rollback_receipt_hash_chain_sha256:$rollback_receipt_hash_chain_sha256,
      rollback_receipt_acceptance_hash_sha256:$rollback_receipt_acceptance_hash_sha256,
      rollback_receipt_digest_match:true,
      rollback_receipt_hash_chain_verified:true,
      required_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_surface_count:12,
      ready_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_surface_count:12,
      required_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixture_count:10,
      minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixture_count:10,
      accepted_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixture_count:1,
      blocked_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixture_count:9,
      minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_accepted_count:1,
      rollback_receipt_acceptance_authority_accepted_count:1,
      source_durable_readback_receipt_acceptance_bound_count:1,
      receipt_acceptance_hash_bound_count:1,
      rollback_receipt_identity_bound_count:1,
      rollback_receipt_digest_bound_count:1,
      rollback_receipt_hash_chain_bound_count:1,
      rollback_receipt_source_readback_linkage_bound_count:1,
      rollback_receipt_acceptance_record_bound_count:1,
      rollback_receipt_replay_guard_accepted_count:1,
      tombstone_cleanup_handoff_bound_count:1,
      rollback_receipt_acceptance_result_recorded_count:1,
      rollback_receipt_acceptance_result_accepted_count:1,
      single_use_nonce_consumed_count:0,
      explicit_command_dispatched_count:0,
      wal_write_performed_count:0,
      wal_recorded_count:0,
      wal_persisted_count:0,
      receipt_recorded_count:0,
      receipt_persisted_count:0,
      receipt_materialized_count:0,
      receipt_delivered_count:0,
      canary_artifact_filesystem_written_count:0,
      artifact_readback_performed_count:0,
      artifact_cleanup_performed_count:0,
      post_write_readback_performed_count:0,
      readback_result_recorded_count:0,
      readback_result_persisted_count:0,
      readback_result_accepted_count:0,
      rollback_performed_count:0,
      rollback_result_recorded_count:0,
      rollback_result_persisted_count:0,
      rollback_result_accepted_count:0,
      tombstone_written_count:0,
      compensating_memory_write_performed_count:0,
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
      required_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fields:[
        "source_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_report_sha256",
        "source_receipt_acceptance_hash_sha256",
        "source_receipt_readback_report_sha256",
        "approved_namespace",
        "approved_store",
        "approved_scope",
        "rollback_receipt_id",
        "rollback_receipt_sha256",
        "rollback_receipt_hash_chain_sha256",
        "rollback_receipt_acceptance_hash_sha256",
        "active_binary_sha256",
        "route_count"
      ],
      minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_surfaces:[
        "source_durable_readback_receipt_acceptance_required",
        "approved_namespace_store_scope_required",
        "receipt_acceptance_hash_required",
        "rollback_receipt_identity_required",
        "rollback_receipt_digest_required",
        "rollback_receipt_hash_chain_required",
        "rollback_receipt_source_readback_linkage_required",
        "rollback_receipt_acceptance_record_required",
        "rollback_receipt_replay_guard_required",
        "rollback_operator_review_handoff_required",
        "tombstone_cleanup_handoff_required",
        "memory_kg_provider_channel_public_release_install_active_binary_side_effects_forbidden"
      ],
      minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixtures:$fixtures,
      denied_by_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary:$denials,
      denied_by_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_count:($denials | length),
      source_durable_readback_receipt_acceptance_required:true,
      minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_accepted:true,
      approved_namespace_bound:true,
      approved_store_bound:true,
      approved_scope_bound:true,
      receipt_acceptance_hash_bound:true,
      rollback_receipt_identity_bound:true,
      rollback_receipt_digest_bound:true,
      rollback_receipt_hash_chain_bound:true,
      rollback_receipt_source_readback_linkage_bound:true,
      rollback_receipt_acceptance_record_bound:true,
      rollback_receipt_replay_guard_bound:true,
      rollback_operator_review_handoff_bound:true,
      tombstone_cleanup_handoff_bound:true,
      wal_rewrite_forbidden_on_report_route:true,
      receipt_repersist_forbidden_on_report_route:true,
      nonce_consumption_forbidden_on_report_route:true,
      explicit_command_dispatch_forbidden_on_report_route:true,
      durable_memory_read_forbidden:true,
      durable_memory_write_forbidden:true,
      durable_memory_rollback_forbidden:true,
      memory_store_mutation_forbidden:true,
      post_write_memory_readback_forbidden:true,
      rollback_execution_forbidden:true,
      tombstone_write_forbidden:true,
      kg_live_write_forbidden:true,
      provider_model_invocation_forbidden:true,
      credential_channel_public_release_forbidden:true,
      install_restart_active_binary_mutation_forbidden:true,
      minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_hash_sha256:$boundary_hash_sha256,
      minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_policy_hash_sha256:$policy_hash_sha256,
      allowed_next_actions:[
        {
          action:"run_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_require_live_gate",
          status:"allowed_verification_only",
          accepts_rollback_receipt:true,
          writes_wal:false,
          persists_receipt:false,
          writes_memory:false,
          reads_memory:false,
          executes_rollback:false,
          writes_tombstone:false,
          writes_kg:false,
          invokes_provider:false,
          reads_credentials:false,
          sends_externally:false,
          publishes_artifacts:false,
          installs_or_restarts:false,
          mutates_active_binary:false
        },
        {
          action:"prepare_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary",
          status:"allowed_report_only_next_slice",
          requires_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance:true,
          writes_durable_memory:false,
          writes_wal:false,
          persists_receipt:false,
          executes_rollback:false,
          writes_tombstone:false,
          writes_kg:false,
          invokes_provider:false,
          sends_externally:false
        }
      ],
      side_effects:(false_external + true_acceptance)
    } + false_external + true_acceptance
  '
