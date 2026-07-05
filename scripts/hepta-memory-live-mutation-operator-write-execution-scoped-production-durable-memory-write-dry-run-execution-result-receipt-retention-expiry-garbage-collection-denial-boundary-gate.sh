#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-audit-trail-immutable-evidence-denial-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-audit-trail-immutable-evidence-denial-boundary-gate.sh
)"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_ready == true
    and $source.scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_accepted == true
    and $source.accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_fixture_count == 1
    and $source.blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_fixture_count == 9
    and $source.denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_count == 72
    and $source.dry_run_execution_result_receipt_audit_trail_recorded == false
    and $source.dry_run_execution_result_receipt_audit_trail_persisted == false
    and $source.dry_run_execution_result_receipt_immutable_evidence_recorded == false
    and $source.dry_run_execution_result_receipt_immutable_evidence_persisted == false
    and $source.dry_run_execution_result_receipt_hash_chain_recorded == false
    and $source.dry_run_execution_result_receipt_authority_promoted_from_audit_trail == false
    and $source.dry_run_execution_executed == false
    and $source.dry_run_execution_result_receipt_persisted == false
    and $source.production_durable_memory_write_executed == false
    and $source.production_durable_memory_store_write_performed == false
    and $source.actual_production_durable_memory_write_performed == false
    and $source.durable_memory_store_write_performed == false
    and $source.durable_memory_store_read_performed == false
    and $source.durable_memory_store_rollback_performed == false
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
    and $source.service_restarted == false
    and $source.active_binary_mutated == false
    and $source.allowed_next_actions[1].action == "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary"
    and $source.allowed_next_actions[1].requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary == true
    and $source.allowed_next_actions[1].executes_dry_run == false
    and $source.allowed_next_actions[1].writes_production_durable_memory == false
  ' >/dev/null

approved_production_namespace="$(jq -r '.approved_production_namespace // ""' <<<"$SOURCE_JSON")"
approved_production_store="$(jq -r '.approved_production_store // ""' <<<"$SOURCE_JSON")"
approved_production_scope="$(jq -r '.approved_production_scope // ""' <<<"$SOURCE_JSON")"
production_durable_memory_target_id="$(jq -r '.production_durable_memory_target_id // ""' <<<"$SOURCE_JSON")"
production_durable_memory_payload_class="$(jq -r '.production_durable_memory_payload_class // ""' <<<"$SOURCE_JSON")"
operator_packet_scope="$(jq -r '.operator_packet_scope // ""' <<<"$SOURCE_JSON")"
source_report_sha256="$(sha256_text "$SOURCE_JSON")"
source_audit_boundary_hash_sha256="$(jq -r '.scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_audit_policy_hash_sha256="$(jq -r '.scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_policy_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_audit_result_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_audit_evidence_result_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_audit_handoff_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_audit_evidence_handoff_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_audit_trail_matrix_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_audit_trail_denial_matrix_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_immutable_evidence_matrix_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_immutable_evidence_denial_matrix_hash_sha256 // ""' <<<"$SOURCE_JSON")"

retention_policy_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-retention-policy-denial:v1:source=${source_audit_result_hash_sha256}:retention-policy=false:record=false:persist=false:materialize=false:filesystem=false"
)"
retention_index_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-retention-index-denial:v1:policy=${retention_policy_denial_hash_sha256}:index=false:record=false:persist=false"
)"
expiry_lifecycle_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-expiry-lifecycle-denial:v1:retention=${retention_index_denial_hash_sha256}:ttl=false:expiry=false:scheduler=false:timer=false:ack=false:persist=false"
)"
garbage_collection_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-garbage-collection-denial:v1:expiry=${expiry_lifecycle_denial_hash_sha256}:queue=false:scan=false:candidate=false:decision=false:delete=false:tombstone=false:sweep=false"
)"
archive_compaction_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-archive-compaction-denial:v1:gc=${garbage_collection_denial_hash_sha256}:archive=false:compaction=false:artifact=false"
)"
retention_evidence_handoff_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-retention-expiry-garbage-collection-handoff:v1:archive=${archive_compaction_denial_hash_sha256}:next=export-query-observability-denial-boundary"
)"
retention_result_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-retention-expiry-garbage-collection-result:v1:retention=${retention_policy_denial_hash_sha256}:expiry=${expiry_lifecycle_denial_hash_sha256}:gc=${garbage_collection_denial_hash_sha256}:handoff=${retention_evidence_handoff_hash_sha256}:accepted=true:record=false:persist=false:delete=false:authority=false:execution=false:production-write=false"
)"
retention_boundary_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-retention-expiry-garbage-collection-denial-boundary:v1:source=${source_report_sha256}:result=${retention_result_hash_sha256}:fixtures=10:accepted=1:denials=62:retention=false:expiry=false:gc=false:delete=false:archive=false:compaction=false:authority=false:dry-run-executed=false:production-write=false"
)"
retention_boundary_policy_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-retention-expiry-garbage-collection-denial-policy:v1:bind-source-audit-evidence-no-retention-policy-no-index-no-ttl-no-expiry-scheduler-no-gc-queue-no-scan-no-delete-no-tombstone-no-sweep-no-archive-no-compaction-no-authority-no-execution-no-production-write-no-kg-no-provider-no-channel-no-release-no-install"
)"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_gate" \
  --arg endpoint "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-retention-expiry-garbage-collection-denial-boundary" \
  --arg source_command "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-retention-expiry-garbage-collection-denial-boundary --json" \
  --arg approved_production_namespace "$approved_production_namespace" \
  --arg approved_production_store "$approved_production_store" \
  --arg approved_production_scope "$approved_production_scope" \
  --arg production_durable_memory_target_id "$production_durable_memory_target_id" \
  --arg production_durable_memory_payload_class "$production_durable_memory_payload_class" \
  --arg operator_packet_scope "$operator_packet_scope" \
  --arg source_report_sha256 "$source_report_sha256" \
  --arg source_audit_boundary_hash_sha256 "$source_audit_boundary_hash_sha256" \
  --arg source_audit_policy_hash_sha256 "$source_audit_policy_hash_sha256" \
  --arg source_audit_result_hash_sha256 "$source_audit_result_hash_sha256" \
  --arg source_audit_handoff_hash_sha256 "$source_audit_handoff_hash_sha256" \
  --arg source_audit_trail_matrix_hash_sha256 "$source_audit_trail_matrix_hash_sha256" \
  --arg source_immutable_evidence_matrix_hash_sha256 "$source_immutable_evidence_matrix_hash_sha256" \
  --arg retention_policy_denial_hash_sha256 "$retention_policy_denial_hash_sha256" \
  --arg retention_index_denial_hash_sha256 "$retention_index_denial_hash_sha256" \
  --arg expiry_lifecycle_denial_hash_sha256 "$expiry_lifecycle_denial_hash_sha256" \
  --arg garbage_collection_denial_hash_sha256 "$garbage_collection_denial_hash_sha256" \
  --arg archive_compaction_denial_hash_sha256 "$archive_compaction_denial_hash_sha256" \
  --arg retention_evidence_handoff_hash_sha256 "$retention_evidence_handoff_hash_sha256" \
  --arg retention_result_hash_sha256 "$retention_result_hash_sha256" \
  --arg retention_boundary_hash_sha256 "$retention_boundary_hash_sha256" \
  --arg retention_boundary_policy_hash_sha256 "$retention_boundary_policy_hash_sha256" \
  --argjson source "$SOURCE_JSON" \
  '
  def zero_fields($keys): reduce $keys[] as $k ({}; .[$k]=false | .[$k + "_count"]=0);
  def true_count_fields($keys): reduce $keys[] as $k ({}; .[$k]=true | .[$k + "_count"]=1);
  ([
    "source_audit_trail_immutable_evidence_denial_boundary_required",
    "source_audit_evidence_result_required",
    "dry_run_execution_result_receipt_retention_policy_request_denied",
    "dry_run_execution_result_receipt_retention_index_denied",
    "dry_run_execution_result_receipt_ttl_lease_update_extension_denied",
    "dry_run_execution_result_receipt_expiry_timestamp_scheduler_timer_ack_denied",
    "dry_run_execution_result_receipt_expiry_state_persistence_denied",
    "dry_run_execution_result_receipt_garbage_collection_queue_scan_candidate_denied",
    "dry_run_execution_result_receipt_garbage_collection_decision_state_denied",
    "dry_run_execution_result_receipt_delete_tombstone_sweep_denied",
    "dry_run_execution_result_receipt_archive_compaction_denied",
    "dry_run_execution_result_receipt_audit_immutable_hash_attestation_retention_denied",
    "dry_run_execution_result_receipt_ledger_index_delivery_retention_denied",
    "dry_run_execution_result_receipt_memory_kg_provider_channel_retention_denied",
    "dry_run_execution_result_receipt_release_install_active_binary_gc_denied",
    "dry_run_execution_production_write_and_authority_forbidden_on_retention_gc_route"
  ]) as $surfaces
  | ([
    "source_audit_trail_immutable_evidence_denial_boundary_required",
    "source_audit_evidence_result_hash_required",
    "source_audit_evidence_policy_hash_required",
    "source_audit_evidence_handoff_hash_required",
    "approved_production_namespace_required",
    "approved_production_store_required",
    "approved_production_scope_required",
    "production_durable_memory_target_required",
    "retention_policy_request_acceptance_denied",
    "retention_policy_recording_denied",
    "retention_policy_persistence_denied",
    "retention_policy_materialization_denied",
    "retention_policy_filesystem_write_denied",
    "retention_index_recording_denied",
    "retention_index_persistence_denied",
    "ttl_lease_recording_denied",
    "ttl_lease_persistence_denied",
    "ttl_update_denied",
    "ttl_extension_denied",
    "expiry_timestamp_recording_denied",
    "expiry_scheduler_registration_denied",
    "expiry_timer_start_denied",
    "expiry_ack_recording_denied",
    "expiry_state_persistence_denied",
    "garbage_collection_queue_recording_denied",
    "garbage_collection_queue_persistence_denied",
    "garbage_collection_scan_denied",
    "garbage_collection_candidate_recording_denied",
    "garbage_collection_decision_recording_denied",
    "garbage_collection_state_persistence_denied",
    "delete_marker_gc_denied",
    "tombstone_gc_denied",
    "sweep_execution_denied",
    "archive_write_denied",
    "compaction_execution_denied",
    "compaction_artifact_write_denied",
    "audit_evidence_retention_recording_denied",
    "immutable_evidence_retention_persistence_denied",
    "hash_attestation_retention_denied",
    "ledger_index_delivery_retention_denied",
    "result_receipt_retention_authority_promotion_denied",
    "retention_expiry_gc_authority_promotion_denied",
    "dry_run_execution_execution_denied",
    "dry_run_execution_result_receipt_persistence_denied",
    "production_write_execution_denied",
    "production_durable_memory_backend_write_denied",
    "durable_memory_backend_read_or_rollback_denied",
    "memory_store_mutation_denied",
    "wal_write_denied",
    "receipt_persistence_denied",
    "post_write_readback_denied",
    "rollback_execution_denied",
    "tombstone_write_denied",
    "raw_payload_plaintext_denied",
    "kg_live_write_denied",
    "provider_model_invocation_denied",
    "credential_secret_read_denied",
    "channel_external_send_denied",
    "release_public_artifact_write_denied",
    "install_restart_authority_denied",
    "active_binary_mutation_denied",
    "unrestricted_full_live_activation_denied"
  ]) as $denials
  | ([
    "dry_run_execution_result_receipt_retention_policy_recorded",
    "dry_run_execution_result_receipt_retention_policy_persisted",
    "dry_run_execution_result_receipt_retention_index_recorded",
    "dry_run_execution_result_receipt_expiry_scheduler_registered",
    "dry_run_execution_result_receipt_expiry_timer_started",
    "dry_run_execution_result_receipt_garbage_collection_queue_recorded",
    "dry_run_execution_result_receipt_garbage_collection_scan_performed",
    "dry_run_execution_result_receipt_delete_marker_garbage_collected",
    "dry_run_execution_result_receipt_tombstone_garbage_collected",
    "dry_run_execution_result_receipt_sweep_performed",
    "dry_run_execution_result_receipt_archive_written",
    "dry_run_execution_result_receipt_compaction_performed",
    "dry_run_execution_result_receipt_authority_promoted_from_retention_policy",
    "dry_run_execution_result_receipt_authority_promoted_from_garbage_collection",
    "dry_run_execution_result_receipt_persisted",
    "dry_run_execution_executed",
    "production_durable_memory_write_executed",
    "production_durable_memory_store_write_performed",
    "actual_production_durable_memory_write_performed",
    "durable_memory_store_write_performed",
    "durable_memory_store_read_performed",
    "durable_memory_store_rollback_performed",
    "memory_store_write_performed",
    "wal_write_performed",
    "receipt_persisted",
    "post_write_readback_performed",
    "rollback_executed",
    "rollback_performed",
    "tombstone_cleanup_executed",
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
  ]) as $false_keys
  | ([
    "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_performed",
    "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_result_recorded",
    "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_result_accepted",
    "source_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_accepted",
    "dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_matrix_bound",
    "dry_run_execution_result_receipt_retention_policy_request_denied",
    "dry_run_execution_result_receipt_retention_index_denied",
    "dry_run_execution_result_receipt_ttl_lease_update_extension_denied",
    "dry_run_execution_result_receipt_expiry_request_denied",
    "dry_run_execution_result_receipt_expiry_scheduler_timer_denied",
    "dry_run_execution_result_receipt_garbage_collection_request_denied",
    "dry_run_execution_result_receipt_garbage_collection_scan_denied",
    "dry_run_execution_result_receipt_delete_tombstone_sweep_denied",
    "dry_run_execution_result_receipt_archive_compaction_denied",
    "dry_run_execution_result_receipt_retention_gc_authority_denied",
    "dry_run_execution_result_receipt_retention_expiry_garbage_collection_handoff_bound"
  ]) as $true_keys
  | ($source + {
    product: $product,
    runtime: $runtime,
    status: "ready",
    gate: $gate,
    endpoint: $endpoint,
    source_command: $source_command,
    base_url: $base_url,
    memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_ready: true,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_ready: true,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_performed: true,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_accepted: true,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_mode: "dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_no_retention_no_expiry_no_gc_no_delete_no_archive_no_compaction_no_authority_no_execution_no_production_durable_memory_mutation",
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_ready: true,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_report_sha256: $source_report_sha256,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_accepted_count: 1,
    source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_fixture_count: 1,
    source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_fixture_count: 9,
    source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_count: 72,
    approved_production_namespace: $approved_production_namespace,
    approved_production_store: $approved_production_store,
    approved_production_scope: $approved_production_scope,
    production_durable_memory_target_id: $production_durable_memory_target_id,
    production_durable_memory_payload_class: $production_durable_memory_payload_class,
    operator_packet_scope: $operator_packet_scope,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_hash_sha256: $source_audit_boundary_hash_sha256,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_policy_hash_sha256: $source_audit_policy_hash_sha256,
    source_dry_run_execution_result_receipt_audit_evidence_result_hash_sha256: $source_audit_result_hash_sha256,
    source_dry_run_execution_result_receipt_audit_evidence_handoff_hash_sha256: $source_audit_handoff_hash_sha256,
    source_dry_run_execution_result_receipt_audit_trail_denial_matrix_hash_sha256: $source_audit_trail_matrix_hash_sha256,
    source_dry_run_execution_result_receipt_immutable_evidence_denial_matrix_hash_sha256: $source_immutable_evidence_matrix_hash_sha256,
    dry_run_execution_result_receipt_retention_policy_denial_hash_sha256: $retention_policy_denial_hash_sha256,
    dry_run_execution_result_receipt_retention_index_denial_hash_sha256: $retention_index_denial_hash_sha256,
    dry_run_execution_result_receipt_expiry_lifecycle_denial_hash_sha256: $expiry_lifecycle_denial_hash_sha256,
    dry_run_execution_result_receipt_garbage_collection_denial_hash_sha256: $garbage_collection_denial_hash_sha256,
    dry_run_execution_result_receipt_archive_compaction_denial_hash_sha256: $archive_compaction_denial_hash_sha256,
    dry_run_execution_result_receipt_retention_expiry_garbage_collection_handoff_hash_sha256: $retention_evidence_handoff_hash_sha256,
    dry_run_execution_result_receipt_retention_expiry_garbage_collection_result_hash_sha256: $retention_result_hash_sha256,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_hash_sha256: $retention_boundary_hash_sha256,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_policy_hash_sha256: $retention_boundary_policy_hash_sha256,
    required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_surface_count: ($surfaces|length),
    ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_surface_count: ($surfaces|length),
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_surfaces: $surfaces,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_fixture_count: 10,
    accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_fixture_count: 1,
    blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_fixture_count: 9,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_fixtures: (
      [{id:"scoped-production-durable-memory-write-dry-run-execution-result-receipt-retention-expiry-garbage-collection-denial", scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_accepted:true}]
      + (["missing-audit-evidence-source","retention-policy-record-request-attempt","retention-index-request-attempt","ttl-lease-update-extension-attempt","expiry-scheduler-timer-ack-attempt","garbage-collection-queue-scan-attempt","delete-tombstone-sweep-attempt","archive-compaction-attempt","memory-kg-provider-channel-release-install-gc-evidence-attempt"] | map({id:., scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_accepted:false}))
    ),
    denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary: $denials,
    denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_count: ($denials|length),
    allowed_next_actions: [
      {
        action: "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_require_live_gate",
        status: "allowed_verification_only",
        accepts_retention_expiry_garbage_collection_denial_matrix: true,
        records_retention_policy: false,
        performs_garbage_collection_scan: false,
        deletes_memory: false,
        performs_compaction: false,
        executes_dry_run: false,
        writes_production_durable_memory: false
      },
      {
        action: "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary",
        status: "requires_separate_result_receipt_export_query_observability_denial_gate",
        requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary: true,
        performs_garbage_collection_scan: false,
        deletes_memory: false,
        executes_dry_run: false,
        writes_production_durable_memory: false
      }
    ]
  })
  + true_count_fields($true_keys)
  + zero_fields($false_keys)
  | .source_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_bound = true
  | .dry_run_execution_result_receipt_retention_expiry_garbage_collection_persistence_forbidden = true
  | .dry_run_execution_execution_forbidden_on_retention_gc_route = true
  | .production_write_execution_forbidden_on_retention_gc_route = true
  | .production_durable_memory_write_forbidden = true
  | .memory_store_mutation_forbidden = true
  | .kg_live_write_forbidden = true
  | .provider_model_invocation_forbidden = true
  | .credential_channel_public_release_forbidden = true
  | .install_restart_active_binary_mutation_forbidden = true
  | .side_effects = (true_count_fields($true_keys) + zero_fields($false_keys))
  '
