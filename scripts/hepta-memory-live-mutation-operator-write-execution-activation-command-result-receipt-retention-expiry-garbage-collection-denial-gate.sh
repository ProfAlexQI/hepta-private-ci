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

AUDIT_EVIDENCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-audit-trail-immutable-evidence-denial-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-audit-trail-immutable-evidence-denial-gate.sh
)"

audit_evidence_report_sha256="$(printf '%s' "$AUDIT_EVIDENCE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson source "$AUDIT_EVIDENCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_gate"
    and $source.activation_command_result_receipt_audit_trail_immutable_evidence_mode == "memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial"
    and $source.memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_no_persistence_ready == true
    and $source.source_activation_command_result_receipt_cancellation_supersession_report_sha256 != ""
    and $source.source_activation_command_result_receipt_ordering_monotonicity_report_sha256 != ""
    and $source.source_activation_command_result_receipt_replay_idempotency_report_sha256 != ""
    and $source.source_activation_command_result_receipt_no_persistence_report_sha256 != ""
    and $source.minimum_required_samples >= 24
    and $source.required_activation_command_result_receipt_audit_trail_immutable_evidence_surface_count == 12
    and $source.ready_activation_command_result_receipt_audit_trail_immutable_evidence_surface_count == 12
    and $source.side_effect_free_activation_command_result_receipt_audit_trail_immutable_evidence_surface_count == 12
    and $source.required_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count == 10
    and $source.activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count == 10
    and $source.blocked_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count == 10
    and $source.noop_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count == 10
    and $source.allowed_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count == 0
    and $source.accepted_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count == 0
    and $source.activation_command_result_receipt_audit_trail_allowed == false
    and $source.activation_command_result_receipt_audit_trail_recorded == false
    and $source.activation_command_result_receipt_audit_trail_persisted == false
    and $source.activation_command_result_receipt_immutable_evidence_allowed == false
    and $source.activation_command_result_receipt_immutable_evidence_recorded == false
    and $source.activation_command_result_receipt_immutable_evidence_persisted == false
    and $source.activation_command_result_receipt_hash_chain_recorded == false
    and $source.activation_command_result_receipt_merkle_root_recorded == false
    and $source.activation_command_result_receipt_attestation_recorded == false
    and $source.activation_command_result_receipt_witness_recorded == false
    and $source.activation_command_result_receipt_notary_recorded == false
    and $source.activation_command_result_receipt_ledger_evidence_recorded == false
    and $source.activation_command_result_receipt_index_evidence_recorded == false
    and $source.activation_command_result_receipt_delivery_evidence_recorded == false
    and $source.activation_command_result_receipt_recorded == false
    and $source.activation_command_result_receipt_persisted == false
    and $source.activation_command_result_receipt_accepted == false
    and $source.activation_command_result_receipt_materialized == false
    and $source.activation_command_result_receipt_filesystem_written == false
    and $source.activation_allowed_by_result_receipt_audit_trail == false
    and $source.activation_allowed_by_result_receipt_immutable_evidence == false
    and $source.activation_allowed_by_result_receipt == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and $source.live_mutation_execution_performed == false
    and $source.memory_write_execution_performed == false
    and $source.memory_store_write_performed == false
    and $source.memory_store_write_performed_count == 0
    and $source.memory_store_mutated == false
    and $source.rollback_executed == false
    and $source.secret_material_read == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.external_send_performed == false
    and $source.public_release_published == false
    and $source.release_artifact_written == false
    and $source.install_executed == false
    and $source.launchd_mutated == false
    and $source.service_restarted == false
    and $source.active_binary_mutated == false
    and ($source.activation_command_result_receipt_audit_trail_immutable_evidence_fixtures | length) == 10
    and ($source.activation_command_result_receipt_audit_trail_immutable_evidence_fixtures | all((.audit_evidence_status == "blocked_noop" or .audit_evidence_status == "blocked_evidence_noop") and .audit_trail_allowed == false and .audit_trail_recorded == false and .audit_trail_persisted == false and .immutable_evidence_allowed == false and .immutable_evidence_recorded == false and .immutable_evidence_persisted == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false and .receipt_noop_confirmed == true))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_gate" \
  --arg audit_evidence_report_sha256 "$audit_evidence_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$AUDIT_EVIDENCE_JSON" \
  '
  def blocked_fixture($id; $reason; $extra):
    {
      id:$id,
      retention_requested:true,
      expiry_requested:false,
      garbage_collection_requested:false,
      retention_gc_status:"blocked_noop",
      source_audit_evidence_present:true,
      source_audit_evidence_ready:true,
      retention_policy_allowed:false,
      retention_policy_recorded:false,
      retention_policy_persisted:false,
      retention_policy_materialized:false,
      retention_index_allowed:false,
      retention_index_recorded:false,
      retention_index_persisted:false,
      expiry_allowed:false,
      expiry_recorded:false,
      expiry_persisted:false,
      expiry_scheduler_registered:false,
      expiry_timer_started:false,
      expiry_materialized:false,
      ttl_update_allowed:false,
      ttl_update_recorded:false,
      ttl_extension_allowed:false,
      ttl_extension_recorded:false,
      garbage_collection_allowed:false,
      garbage_collection_scan_performed:false,
      garbage_collection_candidate_recorded:false,
      garbage_collection_decision_recorded:false,
      garbage_collection_persisted:false,
      delete_allowed:false,
      delete_performed:false,
      delete_marker_recorded:false,
      tombstone_recorded:false,
      sweep_allowed:false,
      sweep_performed:false,
      archive_allowed:false,
      archive_written:false,
      compaction_allowed:false,
      compaction_performed:false,
      compaction_artifact_written:false,
      ledger_retention_recorded:false,
      ledger_retention_persisted:false,
      index_retention_recorded:false,
      delivery_retention_recorded:false,
      receipt_recorded:false,
      receipt_persisted:false,
      receipt_accepted:false,
      receipt_materialized:false,
      receipt_filesystem_written:false,
      completion_ack_recorded:false,
      completion_ack_persisted:false,
      completion_ack_accepted:false,
      activation_allowed:false,
      live_mutation_execution_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      rollback_executed:false,
      secret_material_read:false,
      provider_invoked:false,
      model_invoked:false,
      external_send_performed:false,
      public_release_published:false,
      release_artifact_written:false,
      install_executed:false,
      launchd_mutated:false,
      service_restarted:false,
      active_binary_mutated:false,
      receipt_noop_confirmed:true,
      reason:$reason
    } + $extra;
  {
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    activation_command_result_receipt_retention_expiry_garbage_collection_mode:"memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial",
    source_activation_command_result_receipt_audit_trail_immutable_evidence_gate:$source.gate,
    source_activation_command_result_receipt_audit_trail_immutable_evidence_ready:$source.memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready,
    source_activation_command_result_receipt_audit_trail_immutable_evidence_report_sha256:$audit_evidence_report_sha256,
    source_activation_command_result_receipt_cancellation_supersession_ready:$source.memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready,
    source_activation_command_result_receipt_cancellation_supersession_report_sha256:$source.source_activation_command_result_receipt_cancellation_supersession_report_sha256,
    source_activation_command_result_receipt_ordering_monotonicity_ready:$source.memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready,
    source_activation_command_result_receipt_ordering_monotonicity_report_sha256:$source.source_activation_command_result_receipt_ordering_monotonicity_report_sha256,
    source_activation_command_result_receipt_replay_idempotency_ready:$source.memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready,
    source_activation_command_result_receipt_replay_idempotency_report_sha256:$source.source_activation_command_result_receipt_replay_idempotency_report_sha256,
    source_activation_command_result_receipt_no_persistence_ready:$source.memory_write_execution_activation_command_result_receipt_no_persistence_ready,
    source_activation_command_result_receipt_no_persistence_report_sha256:$source.source_activation_command_result_receipt_no_persistence_report_sha256,
    source_activation_command_noop_handoff_ready:$source.memory_write_execution_activation_command_noop_handoff_ready,
    source_activation_command_noop_handoff_report_sha256:$source.source_activation_command_noop_handoff_report_sha256,
    source_memory_write_execution_preflight_report_sha256:$source.source_memory_write_execution_preflight_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_no_persistence_ready:true,
    memory_write_execution_activation_command_noop_handoff_ready:true,
    required_activation_command_result_receipt_retention_expiry_garbage_collection_surface_count:12,
    ready_activation_command_result_receipt_retention_expiry_garbage_collection_surface_count:12,
    side_effect_free_activation_command_result_receipt_retention_expiry_garbage_collection_surface_count:12,
    required_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count:10,
    activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count:10,
    blocked_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count:10,
    noop_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count:10,
    allowed_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count:0,
    accepted_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count:0,
    activation_command_result_receipt_retention_denied_count:10,
    activation_command_result_receipt_expiry_denied_count:10,
    activation_command_result_receipt_garbage_collection_denied_count:10,
    activation_command_result_receipt_retention_performed_count:0,
    activation_command_result_receipt_expiry_performed_count:0,
    activation_command_result_receipt_garbage_collection_performed_count:0,
    activation_command_result_receipt_retention_policy_allowed:false,
    activation_command_result_receipt_retention_policy_recorded:false,
    activation_command_result_receipt_retention_policy_persisted:false,
    activation_command_result_receipt_retention_policy_materialized:false,
    activation_command_result_receipt_retention_index_allowed:false,
    activation_command_result_receipt_retention_index_recorded:false,
    activation_command_result_receipt_retention_index_persisted:false,
    activation_command_result_receipt_expiry_allowed:false,
    activation_command_result_receipt_expiry_recorded:false,
    activation_command_result_receipt_expiry_persisted:false,
    activation_command_result_receipt_expiry_scheduler_registered:false,
    activation_command_result_receipt_expiry_timer_started:false,
    activation_command_result_receipt_expiry_materialized:false,
    activation_command_result_receipt_ttl_update_allowed:false,
    activation_command_result_receipt_ttl_update_recorded:false,
    activation_command_result_receipt_ttl_extension_allowed:false,
    activation_command_result_receipt_ttl_extension_recorded:false,
    activation_command_result_receipt_garbage_collection_allowed:false,
    activation_command_result_receipt_garbage_collection_scan_performed:false,
    activation_command_result_receipt_garbage_collection_candidate_recorded:false,
    activation_command_result_receipt_garbage_collection_decision_recorded:false,
    activation_command_result_receipt_garbage_collection_persisted:false,
    activation_command_result_receipt_delete_allowed:false,
    activation_command_result_receipt_delete_performed:false,
    activation_command_result_receipt_delete_marker_recorded:false,
    activation_command_result_receipt_tombstone_recorded:false,
    activation_command_result_receipt_sweep_allowed:false,
    activation_command_result_receipt_sweep_performed:false,
    activation_command_result_receipt_archive_allowed:false,
    activation_command_result_receipt_archive_written:false,
    activation_command_result_receipt_compaction_allowed:false,
    activation_command_result_receipt_compaction_performed:false,
    activation_command_result_receipt_compaction_artifact_written:false,
    activation_command_result_receipt_ledger_retention_recorded:false,
    activation_command_result_receipt_ledger_retention_persisted:false,
    activation_command_result_receipt_index_retention_recorded:false,
    activation_command_result_receipt_delivery_retention_recorded:false,
    activation_command_result_receipt_recorded:false,
    activation_command_result_receipt_persisted:false,
    activation_command_result_receipt_accepted:false,
    activation_command_result_receipt_materialized:false,
    activation_command_result_receipt_filesystem_written:false,
    activation_command_result_receipt_ledger_written:false,
    activation_command_result_receipt_indexed:false,
    activation_command_result_receipt_enqueued:false,
    activation_command_result_receipt_delivered:false,
    activation_command_completion_ack_recorded:false,
    activation_command_completion_ack_persisted:false,
    activation_command_completion_ack_accepted:false,
    activation_command_completion_ack_delivered:false,
    activation_allowed_by_result_receipt_retention:false,
    activation_allowed_by_result_receipt_expiry:false,
    activation_allowed_by_result_receipt_garbage_collection:false,
    activation_allowed_by_result_receipt_audit_trail:false,
    activation_allowed_by_result_receipt_immutable_evidence:false,
    activation_allowed_by_result_receipt:false,
    activation_allowed:false,
    activation_performed:false,
    live_mutation_execution_ready:false,
    live_mutation_execution_allowed:false,
    live_mutation_execution_performed:false,
    memory_write_execution_allowed:false,
    memory_write_execution_ready:false,
    memory_write_execution_performed:false,
    memory_store_write_path_enabled:false,
    memory_store_write_allowed:false,
    memory_store_write_performed:false,
    memory_store_write_performed_count:0,
    memory_store_mutation_allowed:false,
    memory_store_mutated:false,
    rollback_execution_allowed:false,
    rollback_executed:false,
    raw_payload_plaintext_recorded:false,
    raw_payload_plaintext_persisted:false,
    secret_material_read:false,
    provider_prompt_replay_enabled:false,
    provider_invoked:false,
    model_invoked:false,
    external_send_enabled:false,
    external_send_performed:false,
    public_claim_or_release_artifact_write_enabled:false,
    public_release_published:false,
    public_ga_claimed:false,
    release_artifact_written:false,
    install_executed:false,
    launchd_mutated:false,
    service_restarted:false,
    active_binary_mutated:false,
    activation_command_result_receipt_retention_expiry_garbage_collection_surfaces:[
      "source_audit_trail_immutable_evidence_report_required",
      "retention_policy_request_shape_denied",
      "retention_index_recording_denied",
      "expiry_scheduler_registration_denied",
      "ttl_update_extension_denied",
      "garbage_collection_scan_denied",
      "delete_tombstone_sweep_denied",
      "archive_compaction_denied",
      "ledger_index_delivery_retention_evidence_denied",
      "activation_from_retention_expiry_gc_denied",
      "memory_write_rollback_secret_provider_gc_denied",
      "external_public_install_restart_active_binary_gc_denied"
    ],
    activation_command_result_receipt_retention_expiry_garbage_collection_fixtures:[
      blocked_fixture("activation-result-receipt-retention-missing-source-audit-evidence"; "source_audit_trail_immutable_evidence_report_required"; {
        source_audit_evidence_present:false,
        source_audit_evidence_ready:false
      }),
      blocked_fixture("activation-result-receipt-retention-policy-write-request"; "retention_policy_write_request_denied"; {
        retention_policy_request_shape:"record_blocked_noop_receipt_retention_policy"
      }),
      blocked_fixture("activation-result-receipt-retention-index-record"; "retention_index_recording_denied"; {
        retention_index_requested:true
      }),
      blocked_fixture("activation-result-receipt-expiry-scheduler-timer"; "expiry_scheduler_timer_denied"; {
        retention_requested:false,
        expiry_requested:true,
        retention_gc_status:"blocked_expiry_noop",
        expiry_schedule_requested:true,
        expiry_timer_requested:true
      }),
      blocked_fixture("activation-result-receipt-ttl-update-extension"; "ttl_update_extension_denied"; {
        retention_requested:false,
        expiry_requested:true,
        retention_gc_status:"blocked_expiry_noop",
        ttl_update_requested:true,
        ttl_extension_requested:true
      }),
      blocked_fixture("activation-result-receipt-garbage-collection-scan"; "garbage_collection_scan_denied"; {
        retention_requested:false,
        garbage_collection_requested:true,
        retention_gc_status:"blocked_gc_noop",
        garbage_collection_scan_requested:true
      }),
      blocked_fixture("activation-result-receipt-delete-tombstone-sweep"; "delete_tombstone_sweep_denied"; {
        retention_requested:false,
        garbage_collection_requested:true,
        retention_gc_status:"blocked_gc_noop",
        delete_requested:true,
        tombstone_requested:true,
        sweep_requested:true
      }),
      blocked_fixture("activation-result-receipt-archive-compaction"; "archive_compaction_denied"; {
        retention_requested:false,
        garbage_collection_requested:true,
        retention_gc_status:"blocked_gc_noop",
        archive_requested:true,
        compaction_requested:true
      }),
      blocked_fixture("activation-result-receipt-activation-memory-provider-retention-gc"; "activation_memory_provider_retention_gc_denied"; {
        retention_requested:false,
        expiry_requested:true,
        garbage_collection_requested:true,
        retention_gc_status:"blocked_gc_noop",
        activation_from_retention_gc_requested:true,
        memory_write_gc_evidence_requested:true,
        rollback_gc_evidence_requested:true,
        secret_material_gc_evidence_requested:true,
        provider_prompt_gc_evidence_requested:true
      }),
      blocked_fixture("activation-result-receipt-external-public-install-retention-gc"; "external_public_install_restart_active_binary_retention_gc_denied"; {
        retention_requested:false,
        expiry_requested:true,
        garbage_collection_requested:true,
        retention_gc_status:"blocked_gc_noop",
        ledger_retention_requested:true,
        index_retention_requested:true,
        delivery_retention_requested:true,
        external_send_gc_evidence_requested:true,
        public_claim_gc_evidence_requested:true,
        release_artifact_gc_evidence_requested:true,
        install_gc_evidence_requested:true,
        service_restart_gc_evidence_requested:true,
        active_binary_gc_evidence_requested:true
      })
    ],
    denied_by_activation_command_result_receipt_retention_expiry_garbage_collection:[
      "source_audit_trail_immutable_evidence_report_required",
      "retention_policy_request_acceptance_denied",
      "retention_policy_recording_denied",
      "retention_policy_persistence_denied",
      "retention_index_recording_denied",
      "expiry_request_acceptance_denied",
      "expiry_recording_denied",
      "expiry_scheduler_registration_denied",
      "expiry_timer_start_denied",
      "ttl_update_denied",
      "ttl_extension_denied",
      "garbage_collection_request_acceptance_denied",
      "garbage_collection_scan_denied",
      "garbage_collection_candidate_recording_denied",
      "garbage_collection_decision_recording_denied",
      "delete_marker_recording_denied",
      "tombstone_recording_denied",
      "sweep_execution_denied",
      "archive_write_denied",
      "compaction_execution_denied",
      "ledger_retention_recording_denied",
      "index_retention_recording_denied",
      "delivery_retention_recording_denied",
      "activation_from_retention_expiry_gc_denied",
      "memory_write_gc_denied",
      "rollback_gc_denied",
      "secret_material_gc_denied",
      "provider_prompt_gc_denied",
      "external_public_install_restart_active_binary_gc_denied"
    ],
    side_effects:{
      activation_command_result_receipt_retention_policy_recorded:false,
      activation_command_result_receipt_retention_policy_persisted:false,
      activation_command_result_receipt_retention_policy_materialized:false,
      activation_command_result_receipt_retention_index_recorded:false,
      activation_command_result_receipt_retention_index_persisted:false,
      activation_command_result_receipt_expiry_recorded:false,
      activation_command_result_receipt_expiry_persisted:false,
      activation_command_result_receipt_expiry_scheduler_registered:false,
      activation_command_result_receipt_expiry_timer_started:false,
      activation_command_result_receipt_expiry_materialized:false,
      activation_command_result_receipt_ttl_update_recorded:false,
      activation_command_result_receipt_ttl_extension_recorded:false,
      activation_command_result_receipt_garbage_collection_scan_performed:false,
      activation_command_result_receipt_garbage_collection_candidate_recorded:false,
      activation_command_result_receipt_garbage_collection_decision_recorded:false,
      activation_command_result_receipt_garbage_collection_persisted:false,
      activation_command_result_receipt_delete_performed:false,
      activation_command_result_receipt_delete_marker_recorded:false,
      activation_command_result_receipt_tombstone_recorded:false,
      activation_command_result_receipt_sweep_performed:false,
      activation_command_result_receipt_archive_written:false,
      activation_command_result_receipt_compaction_performed:false,
      activation_command_result_receipt_compaction_artifact_written:false,
      activation_command_result_receipt_ledger_retention_recorded:false,
      activation_command_result_receipt_ledger_retention_persisted:false,
      activation_command_result_receipt_index_retention_recorded:false,
      activation_command_result_receipt_delivery_retention_recorded:false,
      activation_command_result_receipt_audit_trail_recorded:false,
      activation_command_result_receipt_audit_trail_persisted:false,
      activation_command_result_receipt_immutable_evidence_recorded:false,
      activation_command_result_receipt_immutable_evidence_persisted:false,
      activation_command_result_receipt_hash_chain_recorded:false,
      activation_command_result_receipt_merkle_root_recorded:false,
      activation_command_result_receipt_attestation_recorded:false,
      activation_command_result_receipt_witness_recorded:false,
      activation_command_result_receipt_notary_recorded:false,
      activation_command_result_receipt_recorded:false,
      activation_command_result_receipt_persisted:false,
      activation_command_result_receipt_accepted:false,
      activation_command_result_receipt_materialized:false,
      activation_command_result_receipt_filesystem_written:false,
      activation_command_result_receipt_ledger_written:false,
      activation_command_result_receipt_indexed:false,
      activation_command_result_receipt_enqueued:false,
      activation_command_result_receipt_delivered:false,
      activation_command_completion_ack_recorded:false,
      activation_command_completion_ack_persisted:false,
      activation_command_completion_ack_accepted:false,
      activation_command_completion_ack_delivered:false,
      activation_performed:false,
      live_mutation_execution_performed:false,
      memory_write_execution_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      rollback_executed:false,
      raw_payload_inspected:false,
      payload_plaintext_persisted:false,
      secret_file_read:false,
      credential_read:false,
      provider_invoked:false,
      model_invoked:false,
      provider_prompt_replayed:false,
      channel_send_performed:false,
      external_send_performed:false,
      runtime_store_mutated:false,
      gateway_event_enqueued:false,
      capability_registry_mutated:false,
      plugin_registry_mutated:false,
      skill_workshop_written:false,
      filesystem_written:false,
      release_artifact_written:false,
      public_artifact_written:false,
      public_release_published:false,
      public_ga_claimed:false,
      install_executed:false,
      active_binary_mutated:false,
      launchd_mutated:false,
      service_restarted:false
    }
  }')"

jq -e '
  .status == "ready"
  and .memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready == true
  and .activation_command_result_receipt_retention_expiry_garbage_collection_mode == "memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial"
  and .source_activation_command_result_receipt_audit_trail_immutable_evidence_ready == true
  and .source_activation_command_result_receipt_audit_trail_immutable_evidence_report_sha256 != ""
  and .source_activation_command_result_receipt_cancellation_supersession_ready == true
  and .source_activation_command_result_receipt_ordering_monotonicity_ready == true
  and .source_activation_command_result_receipt_replay_idempotency_ready == true
  and .source_activation_command_result_receipt_no_persistence_ready == true
  and .minimum_required_samples >= 24
  and .required_activation_command_result_receipt_retention_expiry_garbage_collection_surface_count == 12
  and .ready_activation_command_result_receipt_retention_expiry_garbage_collection_surface_count == 12
  and .side_effect_free_activation_command_result_receipt_retention_expiry_garbage_collection_surface_count == 12
  and .required_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count == 10
  and .activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count == 10
  and .blocked_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count == 10
  and .noop_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count == 10
  and .allowed_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count == 0
  and .accepted_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count == 0
  and .activation_command_result_receipt_retention_denied_count == 10
  and .activation_command_result_receipt_expiry_denied_count == 10
  and .activation_command_result_receipt_garbage_collection_denied_count == 10
  and .activation_command_result_receipt_retention_performed_count == 0
  and .activation_command_result_receipt_expiry_performed_count == 0
  and .activation_command_result_receipt_garbage_collection_performed_count == 0
  and .activation_command_result_receipt_retention_policy_allowed == false
  and .activation_command_result_receipt_retention_policy_recorded == false
  and .activation_command_result_receipt_retention_policy_persisted == false
  and .activation_command_result_receipt_retention_policy_materialized == false
  and .activation_command_result_receipt_retention_index_allowed == false
  and .activation_command_result_receipt_retention_index_recorded == false
  and .activation_command_result_receipt_expiry_allowed == false
  and .activation_command_result_receipt_expiry_recorded == false
  and .activation_command_result_receipt_expiry_persisted == false
  and .activation_command_result_receipt_expiry_scheduler_registered == false
  and .activation_command_result_receipt_expiry_timer_started == false
  and .activation_command_result_receipt_ttl_update_allowed == false
  and .activation_command_result_receipt_ttl_extension_allowed == false
  and .activation_command_result_receipt_garbage_collection_allowed == false
  and .activation_command_result_receipt_garbage_collection_scan_performed == false
  and .activation_command_result_receipt_garbage_collection_candidate_recorded == false
  and .activation_command_result_receipt_garbage_collection_decision_recorded == false
  and .activation_command_result_receipt_garbage_collection_persisted == false
  and .activation_command_result_receipt_delete_allowed == false
  and .activation_command_result_receipt_delete_performed == false
  and .activation_command_result_receipt_delete_marker_recorded == false
  and .activation_command_result_receipt_tombstone_recorded == false
  and .activation_command_result_receipt_sweep_allowed == false
  and .activation_command_result_receipt_sweep_performed == false
  and .activation_command_result_receipt_archive_allowed == false
  and .activation_command_result_receipt_archive_written == false
  and .activation_command_result_receipt_compaction_allowed == false
  and .activation_command_result_receipt_compaction_performed == false
  and .activation_command_result_receipt_compaction_artifact_written == false
  and .activation_command_result_receipt_ledger_retention_recorded == false
  and .activation_command_result_receipt_index_retention_recorded == false
  and .activation_command_result_receipt_delivery_retention_recorded == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_result_receipt_materialized == false
  and .activation_command_result_receipt_filesystem_written == false
  and .activation_command_result_receipt_ledger_written == false
  and .activation_command_result_receipt_indexed == false
  and .activation_command_result_receipt_enqueued == false
  and .activation_command_result_receipt_delivered == false
  and .activation_command_completion_ack_recorded == false
  and .activation_command_completion_ack_accepted == false
  and .activation_allowed_by_result_receipt_retention == false
  and .activation_allowed_by_result_receipt_expiry == false
  and .activation_allowed_by_result_receipt_garbage_collection == false
  and .activation_allowed_by_result_receipt_audit_trail == false
  and .activation_allowed_by_result_receipt_immutable_evidence == false
  and .activation_allowed_by_result_receipt == false
  and .activation_allowed == false
  and .activation_performed == false
  and .live_mutation_execution_ready == false
  and .live_mutation_execution_allowed == false
  and .live_mutation_execution_performed == false
  and .memory_write_execution_allowed == false
  and .memory_write_execution_ready == false
  and .memory_write_execution_performed == false
  and .memory_store_write_path_enabled == false
  and .memory_store_write_allowed == false
  and .memory_store_write_performed == false
  and .memory_store_write_performed_count == 0
  and .memory_store_mutation_allowed == false
  and .memory_store_mutated == false
  and .rollback_execution_allowed == false
  and .rollback_executed == false
  and .secret_material_read == false
  and .provider_prompt_replay_enabled == false
  and .provider_invoked == false
  and .model_invoked == false
  and .external_send_enabled == false
  and .external_send_performed == false
  and .public_claim_or_release_artifact_write_enabled == false
  and .public_release_published == false
  and .public_ga_claimed == false
  and .release_artifact_written == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and (.activation_command_result_receipt_retention_expiry_garbage_collection_surfaces | length) == 12
  and (.activation_command_result_receipt_retention_expiry_garbage_collection_fixtures | length) == 10
  and (.activation_command_result_receipt_retention_expiry_garbage_collection_fixtures | all((.retention_gc_status == "blocked_noop" or .retention_gc_status == "blocked_expiry_noop" or .retention_gc_status == "blocked_gc_noop") and .retention_policy_allowed == false and .retention_policy_recorded == false and .retention_policy_persisted == false and .expiry_allowed == false and .expiry_recorded == false and .expiry_scheduler_registered == false and .garbage_collection_allowed == false and .garbage_collection_scan_performed == false and .garbage_collection_decision_recorded == false and .delete_performed == false and .tombstone_recorded == false and .sweep_performed == false and .archive_written == false and .compaction_performed == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .completion_ack_recorded == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false and .receipt_noop_confirmed == true))
  and ([.activation_command_result_receipt_retention_expiry_garbage_collection_fixtures[] | select(.source_audit_evidence_present == false)] | length) == 1
  and ([.activation_command_result_receipt_retention_expiry_garbage_collection_fixtures[] | select(.retention_policy_request_shape == "record_blocked_noop_receipt_retention_policy")] | length) == 1
  and ([.activation_command_result_receipt_retention_expiry_garbage_collection_fixtures[] | select(.retention_index_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_retention_expiry_garbage_collection_fixtures[] | select(.expiry_schedule_requested == true and .expiry_timer_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_retention_expiry_garbage_collection_fixtures[] | select(.ttl_update_requested == true and .ttl_extension_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_retention_expiry_garbage_collection_fixtures[] | select(.garbage_collection_scan_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_retention_expiry_garbage_collection_fixtures[] | select(.delete_requested == true and .tombstone_requested == true and .sweep_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_retention_expiry_garbage_collection_fixtures[] | select(.archive_requested == true and .compaction_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_retention_expiry_garbage_collection_fixtures[] | select(.activation_from_retention_gc_requested == true and .memory_write_gc_evidence_requested == true and .provider_prompt_gc_evidence_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_retention_expiry_garbage_collection_fixtures[] | select(.external_send_gc_evidence_requested == true and .install_gc_evidence_requested == true and .active_binary_gc_evidence_requested == true)] | length) == 1
  and (.denied_by_activation_command_result_receipt_retention_expiry_garbage_collection | length) == 29
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory live mutation operator write execution activation command result receipt retention/expiry/garbage-collection denial gate passed"
