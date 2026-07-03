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

SCOPED_CANARY_APPROVAL_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-scoped-memory-real-write-canary-operator-approval-packet-nonce-command-dry-run-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-scoped-memory-real-write-canary-operator-approval-packet-nonce-command-dry-run-gate.sh
)"

source_report_sha256="$(printf '%s' "$SCOPED_CANARY_APPROVAL_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson source "$SCOPED_CANARY_APPROVAL_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_live_mutation_operator_write_execution_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_gate"
    and $source.scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_ready == true
    and $source.source_activation_command_result_receipt_release_artifact_publication_ready == true
    and $source.scoped_memory_real_write_canary_operator_approval_fixture_count == 10
    and $source.accepted_scoped_memory_real_write_canary_operator_approval_fixture_count == 0
    and $source.scoped_memory_real_write_canary_approval_packet_accepted_count == 0
    and $source.single_use_nonce_consumed_count == 0
    and $source.explicit_command_dispatched_count == 0
    and $source.wal_write_performed_count == 0
    and $source.receipt_persisted_count == 0
    and $source.post_write_readback_performed_count == 0
    and $source.memory_store_write_performed_count == 0
    and $source.fresh_operator_approval_packet_accepted == false
    and $source.single_use_nonce_consumed == false
    and $source.explicit_command_dispatched == false
    and $source.durable_memory_store_write_performed == false
    and ($source.durable_memory_store_read_performed // false) == false
    and $source.memory_store_mutated == false
    and $source.live_kg_write_performed == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.credential_read == false
    and $source.channel_send_performed == false
    and $source.external_send_performed == false
    and $source.release_artifact_written == false
    and $source.install_executed == false
    and $source.active_binary_mutated == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_live_mutation_operator_write_execution_scoped_memory_real_write_canary_readback_validation_dry_run_gate" \
    --arg source_report_sha256 "$source_report_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$SCOPED_CANARY_APPROVAL_JSON" \
    '
    def blocked_fixture($id; $status; $reason; $extra):
      {
        id:$id,
        scoped_memory_real_write_canary_readback_status:$status,
        source_scoped_canary_approval_nonce_command_present:true,
        source_scoped_canary_approval_nonce_command_ready:true,
        readback_plan_requested:false,
        receipt_linkage_requested:false,
        payload_digest_compare_requested:false,
        namespace_store_scope_check_requested:false,
        redaction_proof_requested:false,
        secret_plaintext_scan_requested:false,
        stale_read_guard_requested:false,
        phantom_read_guard_requested:false,
        operator_review_handoff_requested:false,
        rollback_tombstone_handoff_requested:false,
        durable_memory_read_requested:false,
        durable_memory_write_requested:false,
        memory_store_mutation_requested:false,
        rollback_execution_requested:false,
        kg_live_write_requested:false,
        provider_model_invocation_requested:false,
        credential_read_requested:false,
        channel_external_send_requested:false,
        public_claim_requested:false,
        release_artifact_write_requested:false,
        install_restart_requested:false,
        active_binary_mutation_requested:false,
        approval_packet_accepted:false,
        single_use_nonce_consumed:false,
        explicit_command_dispatched:false,
        receipt_persisted:false,
        post_write_readback_plan_accepted:false,
        post_write_readback_performed:false,
        readback_result_recorded:false,
        readback_result_persisted:false,
        readback_result_accepted:false,
        readback_payload_digest_compared:false,
        readback_payload_digest_matched:false,
        readback_redaction_proof_accepted:false,
        readback_secret_plaintext_scan_performed:false,
        readback_secret_plaintext_found:false,
        stale_read_guard_accepted:false,
        phantom_read_guard_accepted:false,
        operator_review_handoff_accepted:false,
        rollback_tombstone_handoff_accepted:false,
        rollback_executed:false,
        tombstone_written:false,
        activation_allowed:false,
        memory_write_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        durable_memory_store_write_performed:false,
        durable_memory_store_read_performed:false,
        durable_memory_store_rollback_performed:false,
        live_kg_write_performed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_read:false,
        channel_send_performed:false,
        external_send_performed:false,
        release_artifact_written:false,
        install_executed:false,
        active_binary_mutated:false,
        scoped_canary_readback_dry_run_noop_confirmed:true,
        reason:$reason
      } + $extra;
    [
      blocked_fixture("scoped-memory-real-write-canary-readback-missing-approval-source"; "blocked_source_noop"; "scoped_canary_approval_nonce_command_dry_run_source_required"; {source_scoped_canary_approval_nonce_command_present:false, source_scoped_canary_approval_nonce_command_ready:false, readback_plan_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-readback-plan-required"; "blocked_plan_noop"; "post_write_readback_validation_plan_required"; {readback_plan_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-readback-receipt-linkage-required"; "blocked_receipt_noop"; "wal_receipt_linkage_required_before_readback"; {receipt_linkage_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-readback-payload-digest-compare-required"; "blocked_digest_noop"; "payload_digest_comparison_required_before_acceptance"; {payload_digest_compare_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-readback-namespace-store-scope-required"; "blocked_scope_noop"; "canary_namespace_store_scope_match_required"; {namespace_store_scope_check_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-readback-redaction-secret-scan-required"; "blocked_redaction_noop"; "redaction_proof_and_secret_plaintext_scan_required"; {redaction_proof_requested:true, secret_plaintext_scan_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-readback-stale-phantom-guards-required"; "blocked_consistency_noop"; "stale_and_phantom_read_guards_required"; {stale_read_guard_requested:true, phantom_read_guard_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-readback-operator-review-required"; "blocked_review_noop"; "operator_review_handoff_required_before_acceptance"; {operator_review_handoff_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-readback-rollback-handoff-required"; "blocked_rollback_noop"; "rollback_tombstone_handoff_required_before_real_write_canary"; {rollback_tombstone_handoff_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-readback-direct-side-effect-attempt"; "blocked_execution_noop"; "direct_read_write_rollback_kg_provider_channel_release_install_active_binary_side_effects_denied"; {durable_memory_read_requested:true, durable_memory_write_requested:true, memory_store_mutation_requested:true, rollback_execution_requested:true, kg_live_write_requested:true, provider_model_invocation_requested:true, credential_read_requested:true, channel_external_send_requested:true, public_claim_requested:true, release_artifact_write_requested:true, install_restart_requested:true, active_binary_mutation_requested:true})
    ] as $fixtures
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      scoped_memory_real_write_canary_mode:"scoped_memory_real_write_canary_readback_validation_dry_run_no_read_no_write",
      source_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_gate:$source.gate,
      source_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_ready:$source.scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_ready,
      source_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_report_sha256:$source_report_sha256,
      source_scoped_memory_real_write_canary_operator_approval_fixture_count:$source.scoped_memory_real_write_canary_operator_approval_fixture_count,
      source_accepted_scoped_memory_real_write_canary_operator_approval_fixture_count:$source.accepted_scoped_memory_real_write_canary_operator_approval_fixture_count,
      source_scoped_memory_real_write_canary_approval_packet_accepted_count:$source.scoped_memory_real_write_canary_approval_packet_accepted_count,
      source_single_use_nonce_consumed_count:$source.single_use_nonce_consumed_count,
      source_explicit_command_dispatched_count:$source.explicit_command_dispatched_count,
      source_post_write_readback_performed_count:$source.post_write_readback_performed_count,
      source_memory_store_write_performed_count:$source.memory_store_write_performed_count,
      minimum_required_samples:$min_long_soak_samples,
      scoped_memory_real_write_canary_readback_validation_dry_run_ready:true,
      required_scoped_memory_real_write_canary_readback_surface_count:12,
      ready_scoped_memory_real_write_canary_readback_surface_count:12,
      side_effect_free_scoped_memory_real_write_canary_readback_surface_count:12,
      required_scoped_memory_real_write_canary_readback_fixture_count:10,
      scoped_memory_real_write_canary_readback_fixture_count:($fixtures | length),
      blocked_scoped_memory_real_write_canary_readback_fixture_count:($fixtures | length),
      noop_scoped_memory_real_write_canary_readback_fixture_count:($fixtures | length),
      allowed_scoped_memory_real_write_canary_readback_fixture_count:0,
      accepted_scoped_memory_real_write_canary_readback_fixture_count:0,
      readback_plan_accepted_count:0,
      readback_performed_count:0,
      readback_result_recorded_count:0,
      readback_result_persisted_count:0,
      readback_result_accepted_count:0,
      readback_payload_digest_compared_count:0,
      readback_redaction_proof_accepted_count:0,
      readback_secret_plaintext_scan_performed_count:0,
      durable_memory_store_read_performed_count:0,
      memory_store_write_performed_count:0,
      rollback_tombstone_handoff_accepted_count:0,
      post_write_readback_plan_required:true,
      receipt_linkage_required:true,
      payload_digest_comparison_required:true,
      redaction_secret_scan_required:true,
      rollback_tombstone_handoff_required:true,
      durable_memory_read_forbidden:true,
      durable_memory_write_forbidden:true,
      rollback_execution_forbidden:true,
      scoped_memory_real_write_canary_readback_fixtures:$fixtures,
      denied_by_scoped_memory_real_write_canary_readback_validation_dry_run:[
        "source_scoped_canary_approval_nonce_command_dry_run_boundary_required",
        "fresh_operator_approval_packet_not_accepted",
        "single_use_nonce_not_consumed",
        "explicit_command_not_dispatched",
        "wal_receipt_not_persisted",
        "post_write_readback_plan_not_accepted",
        "durable_memory_store_read_denied",
        "readback_result_not_recorded",
        "readback_result_not_persisted",
        "readback_payload_digest_not_compared",
        "redaction_secret_scan_not_accepted",
        "rollback_tombstone_handoff_not_accepted",
        "memory_kg_provider_channel_release_install_active_binary_denied"
      ],
      denied_by_scoped_memory_real_write_canary_readback_validation_dry_run_count:13,
      fresh_operator_approval_packet_accepted:false,
      single_use_nonce_consumed:false,
      explicit_command_dispatched:false,
      receipt_persisted:false,
      post_write_readback_plan_accepted:false,
      post_write_readback_performed:false,
      readback_result_recorded:false,
      readback_result_persisted:false,
      readback_result_accepted:false,
      readback_payload_digest_compared:false,
      readback_secret_plaintext_scan_performed:false,
      readback_secret_plaintext_found:false,
      rollback_tombstone_handoff_accepted:false,
      activation_allowed:false,
      memory_write_execution_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      durable_memory_store_write_performed:false,
      durable_memory_store_read_performed:false,
      durable_memory_store_rollback_performed:false,
      live_kg_write_performed:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      channel_send_performed:false,
      external_send_performed:false,
      release_artifact_written:false,
      install_executed:false,
      active_binary_mutated:false,
      side_effects:{
        durable_memory_store_read_performed:false,
        durable_memory_store_write_performed:false,
        durable_memory_store_rollback_performed:false,
        memory_store_mutated:false,
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
        filesystem_written:false
      }
    }'
)"

jq -e '
  .status == "ready"
  and .scoped_memory_real_write_canary_readback_validation_dry_run_ready == true
  and .source_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_ready == true
  and .scoped_memory_real_write_canary_readback_fixture_count == 10
  and .accepted_scoped_memory_real_write_canary_readback_fixture_count == 0
  and .readback_performed_count == 0
  and .durable_memory_store_read_performed_count == 0
  and .memory_store_write_performed_count == 0
  and .durable_memory_store_write_performed == false
  and .durable_memory_store_read_performed == false
  and .durable_memory_store_rollback_performed == false
  and .memory_store_mutated == false
  and .live_kg_write_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .release_artifact_written == false
  and .install_executed == false
  and .active_binary_mutated == false
  and (.scoped_memory_real_write_canary_readback_fixtures | length) == 10
  and (.scoped_memory_real_write_canary_readback_fixtures | all((.scoped_memory_real_write_canary_readback_status | startswith("blocked")) and .post_write_readback_performed == false and .durable_memory_store_read_performed == false and .durable_memory_store_write_performed == false and .rollback_executed == false and .memory_store_mutated == false and .scoped_canary_readback_dry_run_noop_confirmed == true))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta scoped Memory real-write canary readback validation dry-run gate passed" >&2
