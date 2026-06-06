#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
RELEASE_BIN="${HEPTA_RELEASE_BIN:-${HEPTA_CODEX_RELEASE_BIN:-$HOME/.local/opt/hepta/bin/hepta}}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

require_unsigned_integer() {
  local name="$1"
  local value="$2"

  case "$value" in
    ''|*[!0-9]*)
      echo "$name must be an unsigned integer" >&2
      exit 2
      ;;
  esac
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

AUDIT_EVIDENCE_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial-gate.sh
)"

retention_expiry_garbage_collection_fixtures_json="$(
  jq -n '
    def retention_gc_fixture($id; $status; $reason; $extra):
      {
        fixture_id: $id,
        retention_gc_status: $status,
        source_audit_trail_immutable_evidence_present: true,
        source_audit_trail_immutable_evidence_ready: true,
        retention_requested: true,
        expiry_requested: false,
        garbage_collection_requested: false,
        retention_policy_allowed: false,
        retention_policy_recorded: false,
        retention_policy_persisted: false,
        retention_policy_materialized: false,
        retention_policy_filesystem_written: false,
        retention_index_allowed: false,
        retention_index_recorded: false,
        retention_index_persisted: false,
        expiry_allowed: false,
        expiry_recorded: false,
        expiry_persisted: false,
        expiry_scheduler_registered: false,
        expiry_timer_started: false,
        expiry_materialized: false,
        ttl_update_allowed: false,
        ttl_update_recorded: false,
        ttl_extension_allowed: false,
        ttl_extension_recorded: false,
        garbage_collection_allowed: false,
        garbage_collection_scan_performed: false,
        garbage_collection_candidate_recorded: false,
        garbage_collection_decision_recorded: false,
        garbage_collection_persisted: false,
        delete_allowed: false,
        delete_performed: false,
        delete_marker_recorded: false,
        tombstone_recorded: false,
        sweep_allowed: false,
        sweep_performed: false,
        archive_allowed: false,
        archive_written: false,
        compaction_allowed: false,
        compaction_performed: false,
        compaction_artifact_written: false,
        ledger_retention_recorded: false,
        ledger_retention_persisted: false,
        index_retention_recorded: false,
        index_retention_persisted: false,
        delivery_retention_recorded: false,
        delivery_retention_persisted: false,
        audit_trail_recorded: false,
        audit_trail_persisted: false,
        immutable_evidence_recorded: false,
        immutable_evidence_persisted: false,
        hash_chain_recorded: false,
        merkle_root_recorded: false,
        attestation_recorded: false,
        witness_recorded: false,
        notary_recorded: false,
        activation_command_result_receipt_recorded: false,
        activation_command_result_receipt_persisted: false,
        activation_command_result_receipt_accepted: false,
        activation_command_result_receipt_materialized: false,
        activation_command_result_receipt_filesystem_written: false,
        activation_command_completion_ack_recorded: false,
        activation_command_completion_ack_persisted: false,
        activation_command_completion_ack_accepted: false,
        operator_approval_from_retention_accepted: false,
        operator_approval_from_expiry_accepted: false,
        operator_approval_from_garbage_collection_accepted: false,
        activation_from_retention_allowed: false,
        activation_from_expiry_allowed: false,
        activation_from_garbage_collection_allowed: false,
        activation_from_receipt_allowed: false,
        activation_command_allowed: false,
        activation_command_enabled: false,
        activation_command_invoked: false,
        activation_command_dispatched: false,
        activation_request_accepted: false,
        activation_request_recorded: false,
        activation_request_persisted: false,
        activation_request_executed: false,
        operator_approval_recorded: false,
        dispatch_performed: false,
        execution_performed: false,
        context_injection_performed: false,
        provider_invoked: false,
        model_invoked: false,
        provider_prompt_replayed: false,
        memory_store_write_performed: false,
        memory_store_mutated: false,
        external_kg_adapter_read_performed: false,
        live_kg_write_performed: false,
        readback_evidence_recorded: false,
        readback_evidence_persisted: false,
        rollback_executed: false,
        credential_read: false,
        secret_file_read: false,
        auth_secret_read: false,
        channel_send_performed: false,
        telegram_send_performed: false,
        external_send_performed: false,
        public_release_claimed: false,
        public_ga_claimed: false,
        release_artifact_written: false,
        install_executed: false,
        launchd_mutated: false,
        service_restarted: false,
        active_binary_mutated: false,
        upstream_fetch_performed: false,
        upstream_merge_performed: false,
        receipt_noop_confirmed: true,
        denial_reason: $reason
      } + $extra;
    [
      retention_gc_fixture("missing-source-audit-trail-immutable-evidence-report"; "blocked_noop"; "source_audit_trail_immutable_evidence_report_required"; {source_audit_trail_immutable_evidence_present: false, source_audit_trail_immutable_evidence_ready: false}),
      retention_gc_fixture("retention-policy-write-request"; "blocked_noop"; "retention_policy_write_request_denied"; {retention_policy_request_shape: "record_blocked_noop_receipt_retention_policy"}),
      retention_gc_fixture("retention-index-record-request"; "blocked_noop"; "retention_index_recording_denied"; {retention_index_requested: true}),
      retention_gc_fixture("expiry-scheduler-timer-request"; "blocked_expiry_noop"; "expiry_scheduler_timer_denied"; {retention_requested: false, expiry_requested: true, expiry_schedule_requested: true, expiry_timer_requested: true}),
      retention_gc_fixture("ttl-update-extension-request"; "blocked_expiry_noop"; "ttl_update_extension_denied"; {retention_requested: false, expiry_requested: true, ttl_update_requested: true, ttl_extension_requested: true}),
      retention_gc_fixture("garbage-collection-scan-request"; "blocked_gc_noop"; "garbage_collection_scan_denied"; {retention_requested: false, garbage_collection_requested: true, garbage_collection_scan_requested: true}),
      retention_gc_fixture("delete-tombstone-sweep-request"; "blocked_gc_noop"; "delete_tombstone_sweep_denied"; {retention_requested: false, garbage_collection_requested: true, delete_requested: true, tombstone_requested: true, sweep_requested: true}),
      retention_gc_fixture("archive-compaction-request"; "blocked_gc_noop"; "archive_compaction_denied"; {retention_requested: false, garbage_collection_requested: true, archive_requested: true, compaction_requested: true}),
      retention_gc_fixture("activation-provider-memory-kg-retention-gc-attempt"; "blocked_gc_noop"; "activation_provider_memory_kg_retention_gc_denied"; {retention_requested: false, expiry_requested: true, garbage_collection_requested: true, activation_from_retention_gc_requested: true, provider_prompt_gc_evidence_requested: true, memory_store_gc_evidence_requested: true, external_kg_gc_evidence_requested: true, live_kg_gc_evidence_requested: true, readback_gc_evidence_requested: true}),
      retention_gc_fixture("rollback-secret-external-public-install-retention-gc-attempt"; "blocked_gc_noop"; "rollback_secret_external_public_install_retention_gc_denied"; {retention_requested: false, expiry_requested: true, garbage_collection_requested: true, ledger_retention_requested: true, index_retention_requested: true, delivery_retention_requested: true, rollback_gc_evidence_requested: true, credential_secret_gc_evidence_requested: true, external_send_gc_evidence_requested: true, public_claim_gc_evidence_requested: true, release_artifact_gc_evidence_requested: true, install_gc_evidence_requested: true, service_restart_gc_evidence_requested: true, active_binary_gc_evidence_requested: true, upstream_gc_evidence_requested: true})
    ]
  '
)"

audit_evidence_report_sha256="$(sha256_text "$AUDIT_EVIDENCE_JSON")"
audit_evidence_contract_hash_sha256="$(jq -r '.audit_trail_immutable_evidence_contract_hash_sha256' <<<"$AUDIT_EVIDENCE_JSON")"
audit_evidence_policy_hash_sha256="$(jq -r '.audit_trail_immutable_evidence_policy_hash_sha256' <<<"$AUDIT_EVIDENCE_JSON")"
retention_expiry_garbage_collection_fixtures_sha256="$(sha256_text "$retention_expiry_garbage_collection_fixtures_json")"
retention_expiry_garbage_collection_contract_hash_sha256="$(
  sha256_text "hepta-canary-operator-review-acknowledgement-activation-command-result-receipt-retention-expiry-garbage-collection-denial:v1:source=$audit_evidence_report_sha256:audit=$audit_evidence_contract_hash_sha256:fixtures=$retention_expiry_garbage_collection_fixtures_sha256:retention=0:expiry=0:gc=0:delete=0:authority=0:live=0"
)"
retention_expiry_garbage_collection_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-harness-operator-review-acknowledgement-activation-command-result-receipt-retention-expiry-garbage-collection-denial:v1:no-retention:no-expiry:no-gc:no-delete:no-archive:no-authority:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_side_effects=false;fixtures=10;retention=0;expiry=0;gc=0;delete=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$AUDIT_EVIDENCE_JSON" \
  --argjson fixtures "$retention_expiry_garbage_collection_fixtures_json" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_gate"
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready == true
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status == "blocked"
    and $source.audit_trail_immutable_evidence_fixture_count == 10
    and $source.blocked_audit_trail_immutable_evidence_fixture_count == 10
    and $source.noop_audit_trail_immutable_evidence_fixture_count == 10
    and $source.allowed_audit_trail_immutable_evidence_fixture_count == 0
    and $source.accepted_audit_trail_immutable_evidence_fixture_count == 0
    and $source.audit_trail_performed_count == 0
    and $source.immutable_evidence_performed_count == 0
    and $source.hash_chain_recorded_count == 0
    and $source.merkle_root_recorded_count == 0
    and $source.attestation_recorded_count == 0
    and $source.witness_recorded_count == 0
    and $source.notary_recorded_count == 0
    and $source.activation_command_result_receipt_audit_trail_recorded == false
    and $source.activation_command_result_receipt_audit_trail_persisted == false
    and $source.activation_command_result_receipt_immutable_evidence_recorded == false
    and $source.activation_command_result_receipt_immutable_evidence_persisted == false
    and $source.activation_command_result_receipt_hash_chain_recorded == false
    and $source.activation_command_result_receipt_attestation_recorded == false
    and $source.activation_command_result_receipt_recorded == false
    and $source.activation_command_result_receipt_persisted == false
    and $source.activation_command_result_receipt_accepted == false
    and $source.activation_command_completion_ack_recorded == false
    and $source.activation_command_completion_ack_accepted == false
    and $source.operator_approval_from_audit_trail_accepted == false
    and $source.operator_approval_from_immutable_evidence_accepted == false
    and $source.activation_from_audit_trail_allowed == false
    and $source.activation_from_immutable_evidence_allowed == false
    and $source.activation_from_receipt_allowed == false
    and $source.activation_command_enabled == false
    and $source.activation_command_invoked == false
    and $source.activation_command_dispatched == false
    and $source.activation_request_accepted == false
    and $source.activation_request_executed == false
    and $source.dispatch_performed_count == 0
    and $source.execution_performed_count == 0
    and $source.context_injection_performed_count == 0
    and $source.provider_invoked_count == 0
    and $source.model_invoked_count == 0
    and $source.memory_store_write_performed_count == 0
    and $source.external_kg_adapter_read_performed_count == 0
    and $source.live_kg_write_performed_count == 0
    and $source.credential_read_count == 0
    and $source.secret_file_read_count == 0
    and $source.channel_send_performed_count == 0
    and $source.install_performed_count == 0
    and $source.service_restarted_count == 0
    and $source.active_binary_mutated_count == 0
    and $source.upstream_fetch_performed_count == 0
    and $source.upstream_merge_performed_count == 0
    and ($source.side_effects | to_entries | all(.value == false))
    and ($source.allowed_next_actions | any(.action == "stage_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial" and .status == "allowed_report_only_next_slice" and .performs_retention == false and .performs_gc == false and .mutates_runtime == false and .invokes_model == false and .writes_memory_or_kg == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      (.retention_gc_status == "blocked_noop" or .retention_gc_status == "blocked_expiry_noop" or .retention_gc_status == "blocked_gc_noop")
      and .retention_policy_allowed == false
      and .retention_policy_recorded == false
      and .retention_policy_persisted == false
      and .retention_index_recorded == false
      and .expiry_allowed == false
      and .expiry_recorded == false
      and .expiry_scheduler_registered == false
      and .expiry_timer_started == false
      and .garbage_collection_allowed == false
      and .garbage_collection_scan_performed == false
      and .garbage_collection_decision_recorded == false
      and .delete_performed == false
      and .delete_marker_recorded == false
      and .tombstone_recorded == false
      and .sweep_performed == false
      and .archive_written == false
      and .compaction_performed == false
      and .ledger_retention_recorded == false
      and .index_retention_recorded == false
      and .delivery_retention_recorded == false
      and .activation_command_result_receipt_recorded == false
      and .activation_command_result_receipt_persisted == false
      and .activation_command_result_receipt_accepted == false
      and .activation_command_completion_ack_recorded == false
      and .operator_approval_from_retention_accepted == false
      and .activation_from_retention_allowed == false
      and .activation_from_expiry_allowed == false
      and .activation_from_garbage_collection_allowed == false
      and .activation_command_enabled == false
      and .dispatch_performed == false
      and .execution_performed == false
      and .context_injection_performed == false
      and .provider_invoked == false
      and .model_invoked == false
      and .memory_store_write_performed == false
      and .external_kg_adapter_read_performed == false
      and .live_kg_write_performed == false
      and .credential_read == false
      and .secret_file_read == false
      and .channel_send_performed == false
      and .install_executed == false
      and .service_restarted == false
      and .active_binary_mutated == false
      and .upstream_fetch_performed == false
      and .upstream_merge_performed == false
      and .receipt_noop_confirmed == true
    ))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_gate" \
    --arg audit_evidence_report_sha256 "$audit_evidence_report_sha256" \
    --arg audit_evidence_contract_hash_sha256 "$audit_evidence_contract_hash_sha256" \
    --arg audit_evidence_policy_hash_sha256 "$audit_evidence_policy_hash_sha256" \
    --arg retention_expiry_garbage_collection_fixtures_sha256 "$retention_expiry_garbage_collection_fixtures_sha256" \
    --arg retention_expiry_garbage_collection_contract_hash_sha256 "$retention_expiry_garbage_collection_contract_hash_sha256" \
    --arg retention_expiry_garbage_collection_policy_hash_sha256 "$retention_expiry_garbage_collection_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$AUDIT_EVIDENCE_JSON" \
    --argjson fixtures "$retention_expiry_garbage_collection_fixtures_json" \
    '
      ($source.denied_by_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence + [
        "source_audit_trail_immutable_evidence_report_required",
        "retention_policy_request_acceptance_denied",
        "retention_policy_recording_denied",
        "retention_policy_persistence_denied",
        "retention_policy_materialization_denied",
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
        "delete_execution_denied",
        "delete_marker_recording_denied",
        "tombstone_recording_denied",
        "sweep_execution_denied",
        "archive_write_denied",
        "compaction_execution_denied",
        "ledger_retention_recording_denied",
        "index_retention_recording_denied",
        "delivery_retention_recording_denied",
        "operator_approval_from_retention_expiry_gc_denied",
        "activation_from_retention_expiry_gc_denied",
        "provider_model_memory_kg_gc_evidence_denied",
        "rollback_secret_external_public_install_restart_active_binary_gc_denied"
      ]) as $denials |
      {
        product: $product,
        runtime: $runtime,
        status: "ready",
        base_url: $base_url,
        gate: $gate,
        operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_schema_version: "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_v1",
        operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready: true,
        operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_status: "blocked",
        retention_expiry_garbage_collection_mode: "stdout_only_retention_expiry_garbage_collection_denial_no_schedule_no_scan_no_delete_no_authority_no_live",
        retention_expiry_garbage_collection_decision: "blocked_noop_activation_command_result_receipt_cannot_be_retained_expired_garbage_collected_or_deleted_into_authority",
        minimum_required_samples: $min_long_soak_samples,
        source_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_gate: $source.gate,
        source_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_status: $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status,
        source_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_report_sha256: $audit_evidence_report_sha256,
        source_audit_trail_immutable_evidence_contract_hash_sha256: $audit_evidence_contract_hash_sha256,
        source_audit_trail_immutable_evidence_policy_hash_sha256: $audit_evidence_policy_hash_sha256,
        retention_expiry_garbage_collection_fixtures_sha256: $retention_expiry_garbage_collection_fixtures_sha256,
        retention_expiry_garbage_collection_contract_hash_sha256: $retention_expiry_garbage_collection_contract_hash_sha256,
        retention_expiry_garbage_collection_policy_hash_sha256: $retention_expiry_garbage_collection_policy_hash_sha256,
        side_effect_hash_sha256: $side_effect_hash_sha256,
        source_audit_trail_immutable_evidence_fixture_count: $source.audit_trail_immutable_evidence_fixture_count,
        source_blocked_audit_trail_immutable_evidence_fixture_count: $source.blocked_audit_trail_immutable_evidence_fixture_count,
        source_accepted_audit_trail_immutable_evidence_fixture_count: $source.accepted_audit_trail_immutable_evidence_fixture_count,
        source_audit_trail_performed_count: $source.audit_trail_performed_count,
        source_immutable_evidence_performed_count: $source.immutable_evidence_performed_count,
        source_hash_chain_recorded_count: $source.hash_chain_recorded_count,
        source_attestation_recorded_count: $source.attestation_recorded_count,
        retention_expiry_garbage_collection_surface_count: 12,
        retention_expiry_garbage_collection_surface_ready_count: 12,
        retention_expiry_garbage_collection_side_effect_free_surface_count: 12,
        retention_expiry_garbage_collection_fixtures: $fixtures,
        retention_expiry_garbage_collection_fixture_count: ($fixtures | length),
        blocked_retention_expiry_garbage_collection_fixture_count: ($fixtures | length),
        noop_retention_expiry_garbage_collection_fixture_count: ($fixtures | length),
        allowed_retention_expiry_garbage_collection_fixture_count: 0,
        accepted_retention_expiry_garbage_collection_fixture_count: 0,
        retention_denied_count: ($fixtures | length),
        expiry_denied_count: ($fixtures | length),
        garbage_collection_denied_count: ($fixtures | length),
        retention_performed_count: 0,
        expiry_performed_count: 0,
        garbage_collection_performed_count: 0,
        delete_performed_count: 0,
        archive_written_count: 0,
        compaction_performed_count: 0,
        activation_command_result_receipt_retention_policy_allowed: false,
        activation_command_result_receipt_retention_policy_recorded: false,
        activation_command_result_receipt_retention_policy_persisted: false,
        activation_command_result_receipt_retention_policy_materialized: false,
        activation_command_result_receipt_retention_policy_filesystem_written: false,
        activation_command_result_receipt_retention_index_allowed: false,
        activation_command_result_receipt_retention_index_recorded: false,
        activation_command_result_receipt_retention_index_persisted: false,
        activation_command_result_receipt_expiry_allowed: false,
        activation_command_result_receipt_expiry_recorded: false,
        activation_command_result_receipt_expiry_persisted: false,
        activation_command_result_receipt_expiry_scheduler_registered: false,
        activation_command_result_receipt_expiry_timer_started: false,
        activation_command_result_receipt_expiry_materialized: false,
        activation_command_result_receipt_ttl_update_allowed: false,
        activation_command_result_receipt_ttl_update_recorded: false,
        activation_command_result_receipt_ttl_extension_allowed: false,
        activation_command_result_receipt_ttl_extension_recorded: false,
        activation_command_result_receipt_garbage_collection_allowed: false,
        activation_command_result_receipt_garbage_collection_scan_performed: false,
        activation_command_result_receipt_garbage_collection_candidate_recorded: false,
        activation_command_result_receipt_garbage_collection_decision_recorded: false,
        activation_command_result_receipt_garbage_collection_persisted: false,
        activation_command_result_receipt_delete_allowed: false,
        activation_command_result_receipt_delete_performed: false,
        activation_command_result_receipt_delete_marker_recorded: false,
        activation_command_result_receipt_tombstone_recorded: false,
        activation_command_result_receipt_sweep_allowed: false,
        activation_command_result_receipt_sweep_performed: false,
        activation_command_result_receipt_archive_allowed: false,
        activation_command_result_receipt_archive_written: false,
        activation_command_result_receipt_compaction_allowed: false,
        activation_command_result_receipt_compaction_performed: false,
        activation_command_result_receipt_compaction_artifact_written: false,
        activation_command_result_receipt_ledger_retention_recorded: false,
        activation_command_result_receipt_ledger_retention_persisted: false,
        activation_command_result_receipt_index_retention_recorded: false,
        activation_command_result_receipt_index_retention_persisted: false,
        activation_command_result_receipt_delivery_retention_recorded: false,
        activation_command_result_receipt_delivery_retention_persisted: false,
        activation_command_result_receipt_recorded: false,
        activation_command_result_receipt_persisted: false,
        activation_command_result_receipt_accepted: false,
        activation_command_result_receipt_materialized: false,
        activation_command_result_receipt_filesystem_written: false,
        activation_command_completion_ack_recorded: false,
        activation_command_completion_ack_accepted: false,
        operator_approval_from_retention_accepted: false,
        operator_approval_from_expiry_accepted: false,
        operator_approval_from_garbage_collection_accepted: false,
        activation_allowed_by_result_receipt_retention: false,
        activation_allowed_by_result_receipt_expiry: false,
        activation_allowed_by_result_receipt_garbage_collection: false,
        activation_allowed_by_result_receipt_audit_trail: false,
        activation_allowed_by_result_receipt_immutable_evidence: false,
        activation_allowed_by_result_receipt: false,
        activation_command_allowed: false,
        activation_command_enabled: false,
        activation_command_invoked: false,
        activation_command_dispatched: false,
        activation_request_accepted: false,
        activation_request_recorded: false,
        activation_request_persisted: false,
        activation_request_executed: false,
        operator_approval_recorded: false,
        dispatch_performed_count: 0,
        execution_performed_count: 0,
        context_injection_performed_count: 0,
        provider_invoked_count: 0,
        model_invoked_count: 0,
        memory_store_write_performed_count: 0,
        external_kg_adapter_read_performed_count: 0,
        live_kg_write_performed_count: 0,
        readback_evidence_recorded_count: 0,
        credential_read_count: 0,
        secret_file_read_count: 0,
        channel_send_performed_count: 0,
        install_performed_count: 0,
        service_restarted_count: 0,
        active_binary_mutated_count: 0,
        upstream_fetch_performed_count: 0,
        upstream_merge_performed_count: 0,
        canary_harness_armed: false,
        canary_harness_executable: false,
        canary_live_enabled: false,
        allowed_next_actions: [
          {
            action: "review_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial",
            status: "allowed_report_only",
            performs_retention: false,
            performs_expiry: false,
            performs_gc: false,
            deletes_receipt: false,
            mutates_runtime: false,
            invokes_model: false,
            writes_memory_or_kg: false
          },
          {
            action: "stage_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial",
            status: "allowed_report_only_next_slice",
            performs_retention: false,
            performs_expiry: false,
            performs_gc: false,
            deletes_receipt: false,
            mutates_runtime: false,
            invokes_model: false,
            writes_memory_or_kg: false
          }
        ],
        denied_by_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection: $denials,
        denied_by_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_count: ($denials | length),
        side_effects: {
          workspace_written: false,
          filesystem_written: false,
          retention_policy_recorded: false,
          retention_policy_persisted: false,
          retention_index_recorded: false,
          expiry_recorded: false,
          expiry_scheduler_registered: false,
          expiry_timer_started: false,
          ttl_update_recorded: false,
          ttl_extension_recorded: false,
          garbage_collection_scan_performed: false,
          garbage_collection_candidate_recorded: false,
          garbage_collection_decision_recorded: false,
          delete_performed: false,
          delete_marker_recorded: false,
          tombstone_recorded: false,
          sweep_performed: false,
          archive_written: false,
          compaction_performed: false,
          ledger_retention_recorded: false,
          index_retention_recorded: false,
          delivery_retention_recorded: false,
          activation_command_result_receipt_recorded: false,
          activation_command_result_receipt_persisted: false,
          activation_command_result_receipt_accepted: false,
          activation_command_completion_ack_recorded: false,
          activation_command_enabled: false,
          activation_command_invoked: false,
          activation_command_dispatched: false,
          operator_approval_recorded: false,
          dispatch_performed: false,
          execution_performed: false,
          context_injection_performed: false,
          provider_invoked: false,
          model_invoked: false,
          memory_store_write_performed: false,
          memory_store_mutated: false,
          external_kg_adapter_read_performed: false,
          live_kg_write_performed: false,
          credential_read: false,
          secret_file_read: false,
          channel_send_performed: false,
          telegram_send_performed: false,
          external_send_performed: false,
          public_claim_performed: false,
          release_artifact_written: false,
          install_performed: false,
          service_restarted: false,
          active_binary_mutated: false,
          upstream_fetch_performed: false,
          upstream_merge_performed: false
        }
      }
    '
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_status == "blocked"
  and .source_audit_trail_immutable_evidence_fixture_count == 10
  and .source_accepted_audit_trail_immutable_evidence_fixture_count == 0
  and .source_audit_trail_performed_count == 0
  and .source_immutable_evidence_performed_count == 0
  and .retention_expiry_garbage_collection_fixture_count == 10
  and .blocked_retention_expiry_garbage_collection_fixture_count == 10
  and .noop_retention_expiry_garbage_collection_fixture_count == 10
  and .allowed_retention_expiry_garbage_collection_fixture_count == 0
  and .accepted_retention_expiry_garbage_collection_fixture_count == 0
  and .retention_performed_count == 0
  and .expiry_performed_count == 0
  and .garbage_collection_performed_count == 0
  and .delete_performed_count == 0
  and .activation_command_result_receipt_retention_policy_allowed == false
  and .activation_command_result_receipt_retention_policy_recorded == false
  and .activation_command_result_receipt_retention_policy_persisted == false
  and .activation_command_result_receipt_retention_index_recorded == false
  and .activation_command_result_receipt_expiry_allowed == false
  and .activation_command_result_receipt_expiry_recorded == false
  and .activation_command_result_receipt_expiry_scheduler_registered == false
  and .activation_command_result_receipt_expiry_timer_started == false
  and .activation_command_result_receipt_garbage_collection_allowed == false
  and .activation_command_result_receipt_garbage_collection_scan_performed == false
  and .activation_command_result_receipt_garbage_collection_decision_recorded == false
  and .activation_command_result_receipt_delete_performed == false
  and .activation_command_result_receipt_tombstone_recorded == false
  and .activation_command_result_receipt_sweep_performed == false
  and .activation_command_result_receipt_archive_written == false
  and .activation_command_result_receipt_compaction_performed == false
  and .activation_command_result_receipt_ledger_retention_recorded == false
  and .activation_command_result_receipt_index_retention_recorded == false
  and .activation_command_result_receipt_delivery_retention_recorded == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_completion_ack_recorded == false
  and .activation_command_completion_ack_accepted == false
  and .operator_approval_from_retention_accepted == false
  and .operator_approval_from_expiry_accepted == false
  and .operator_approval_from_garbage_collection_accepted == false
  and .activation_allowed_by_result_receipt_retention == false
  and .activation_allowed_by_result_receipt_expiry == false
  and .activation_allowed_by_result_receipt_garbage_collection == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_request_accepted == false
  and .activation_request_executed == false
  and .dispatch_performed_count == 0
  and .execution_performed_count == 0
  and .context_injection_performed_count == 0
  and .provider_invoked_count == 0
  and .model_invoked_count == 0
  and .memory_store_write_performed_count == 0
  and .external_kg_adapter_read_performed_count == 0
  and .live_kg_write_performed_count == 0
  and .credential_read_count == 0
  and .secret_file_read_count == 0
  and .channel_send_performed_count == 0
  and .install_performed_count == 0
  and .service_restarted_count == 0
  and .active_binary_mutated_count == 0
  and .upstream_fetch_performed_count == 0
  and .upstream_merge_performed_count == 0
  and .canary_harness_armed == false
  and .canary_harness_executable == false
  and .canary_live_enabled == false
  and (.retention_expiry_garbage_collection_fixtures | all(
    (.retention_gc_status == "blocked_noop" or .retention_gc_status == "blocked_expiry_noop" or .retention_gc_status == "blocked_gc_noop")
    and .retention_policy_recorded == false
    and .retention_policy_persisted == false
    and .expiry_recorded == false
    and .expiry_scheduler_registered == false
    and .garbage_collection_scan_performed == false
    and .delete_performed == false
    and .tombstone_recorded == false
    and .archive_written == false
    and .compaction_performed == false
    and .activation_command_result_receipt_accepted == false
    and .operator_approval_from_retention_accepted == false
    and .activation_from_retention_allowed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .external_kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .receipt_noop_confirmed == true
  ))
  and .denied_by_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_count >= 220
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG operator canary controlled request harness operator review acknowledgement activation command result receipt retention/expiry/garbage-collection denial gate passed"
