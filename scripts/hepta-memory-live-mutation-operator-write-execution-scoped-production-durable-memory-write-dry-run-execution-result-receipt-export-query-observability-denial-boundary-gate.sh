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
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-retention-expiry-garbage-collection-denial-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-retention-expiry-garbage-collection-denial-boundary-gate.sh
)"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_ready == true
    and $source.scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_accepted == true
    and $source.accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_fixture_count == 1
    and $source.blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_fixture_count == 9
    and $source.denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_count == 62
    and $source.dry_run_execution_result_receipt_retention_policy_recorded == false
    and $source.dry_run_execution_result_receipt_garbage_collection_scan_performed == false
    and $source.dry_run_execution_result_receipt_authority_promoted_from_garbage_collection == false
    and $source.dry_run_execution_executed == false
    and $source.production_durable_memory_store_write_performed == false
    and $source.memory_store_write_performed == false
    and $source.wal_write_performed == false
    and $source.receipt_persisted == false
    and $source.external_send_performed == false
    and $source.allowed_next_actions[1].action == "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary"
    and $source.allowed_next_actions[1].requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary == true
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
source_retention_boundary_hash_sha256="$(jq -r '.scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_retention_policy_hash_sha256="$(jq -r '.scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_policy_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_retention_result_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_retention_expiry_garbage_collection_result_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_retention_handoff_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_retention_expiry_garbage_collection_handoff_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_garbage_collection_denial_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_garbage_collection_denial_hash_sha256 // ""' <<<"$SOURCE_JSON")"

export_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-export-denial:v1:source=${source_retention_result_hash_sha256}:export-request=false:snapshot=false:file=false:stream=false:delivery=false"
)"
query_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-query-denial:v1:export=${export_denial_hash_sha256}:register=false:execute=false:result=false:index=false:cache=false:search-index=false"
)"
observability_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-observability-denial:v1:query=${query_denial_hash_sha256}:metric=false:log=false:trace=false:span=false:event=false:dashboard=false:alert=false:slo=false:operator-summary=false:readback=false"
)"
export_query_observability_handoff_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-export-query-observability-handoff:v1:observability=${observability_denial_hash_sha256}:next=operator-facing-summary-briefing-non-persistence-denial-boundary"
)"
export_query_observability_result_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-export-query-observability-result:v1:export=${export_denial_hash_sha256}:query=${query_denial_hash_sha256}:observability=${observability_denial_hash_sha256}:handoff=${export_query_observability_handoff_hash_sha256}:accepted=true:persist=false:authority=false:execution=false:production-write=false"
)"
export_query_observability_boundary_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-export-query-observability-denial-boundary:v1:source=${source_report_sha256}:result=${export_query_observability_result_hash_sha256}:fixtures=10:accepted=1:denials=64:export=false:query=false:observability=false:authority=false:dry-run-executed=false:production-write=false"
)"
export_query_observability_policy_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-export-query-observability-denial-policy:v1:bind-source-retention-gc-no-export-no-query-no-search-index-no-metric-no-log-no-trace-no-event-no-dashboard-no-alert-no-slo-no-operator-summary-no-readback-evidence-no-authority-no-execution-no-production-write-no-kg-no-provider-no-channel-no-release-no-install"
)"
export_query_observability_matrix_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-export-query-observability-denial-matrix:v1:export=${export_denial_hash_sha256}:query=${query_denial_hash_sha256}:observability=${observability_denial_hash_sha256}:denials=64"
)"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_gate" \
  --arg endpoint "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-export-query-observability-denial-boundary" \
  --arg source_command "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-export-query-observability-denial-boundary --json" \
  --arg approved_production_namespace "$approved_production_namespace" \
  --arg approved_production_store "$approved_production_store" \
  --arg approved_production_scope "$approved_production_scope" \
  --arg production_durable_memory_target_id "$production_durable_memory_target_id" \
  --arg production_durable_memory_payload_class "$production_durable_memory_payload_class" \
  --arg operator_packet_scope "$operator_packet_scope" \
  --arg source_report_sha256 "$source_report_sha256" \
  --arg source_retention_boundary_hash_sha256 "$source_retention_boundary_hash_sha256" \
  --arg source_retention_policy_hash_sha256 "$source_retention_policy_hash_sha256" \
  --arg source_retention_result_hash_sha256 "$source_retention_result_hash_sha256" \
  --arg source_retention_handoff_hash_sha256 "$source_retention_handoff_hash_sha256" \
  --arg source_garbage_collection_denial_hash_sha256 "$source_garbage_collection_denial_hash_sha256" \
  --arg export_denial_hash_sha256 "$export_denial_hash_sha256" \
  --arg query_denial_hash_sha256 "$query_denial_hash_sha256" \
  --arg observability_denial_hash_sha256 "$observability_denial_hash_sha256" \
  --arg export_query_observability_matrix_hash_sha256 "$export_query_observability_matrix_hash_sha256" \
  --arg export_query_observability_handoff_hash_sha256 "$export_query_observability_handoff_hash_sha256" \
  --arg export_query_observability_result_hash_sha256 "$export_query_observability_result_hash_sha256" \
  --arg export_query_observability_boundary_hash_sha256 "$export_query_observability_boundary_hash_sha256" \
  --arg export_query_observability_policy_hash_sha256 "$export_query_observability_policy_hash_sha256" \
  --argjson source "$SOURCE_JSON" \
  '
  def zero_fields($keys): reduce $keys[] as $k ({}; .[$k]=false | .[$k + "_count"]=0);
  def true_count_fields($keys): reduce $keys[] as $k ({}; .[$k]=true | .[$k + "_count"]=1);
  ([
    "source_retention_expiry_garbage_collection_denial_boundary_required",
    "source_retention_expiry_garbage_collection_result_required",
    "dry_run_execution_result_receipt_export_request_denied",
    "dry_run_execution_result_receipt_export_snapshot_denied",
    "dry_run_execution_result_receipt_export_file_stream_denied",
    "dry_run_execution_result_receipt_query_registration_denied",
    "dry_run_execution_result_receipt_query_execution_denied",
    "dry_run_execution_result_receipt_query_result_recording_denied",
    "dry_run_execution_result_receipt_query_index_cache_denied",
    "dry_run_execution_result_receipt_observability_metric_log_denied",
    "dry_run_execution_result_receipt_observability_trace_span_event_denied",
    "dry_run_execution_result_receipt_dashboard_alert_slo_denied",
    "dry_run_execution_result_receipt_operator_summary_readback_denied",
    "dry_run_execution_result_receipt_ledger_index_delivery_observability_denied",
    "dry_run_execution_result_receipt_memory_kg_provider_channel_observability_denied",
    "dry_run_execution_production_write_and_authority_forbidden_on_export_query_observability_route"
  ]) as $surfaces
  | ([
    "source_retention_expiry_garbage_collection_denial_boundary_required",
    "source_retention_expiry_garbage_collection_result_hash_required",
    "source_retention_expiry_garbage_collection_policy_hash_required",
    "source_retention_expiry_garbage_collection_handoff_hash_required",
    "approved_production_namespace_required",
    "approved_production_store_required",
    "approved_production_scope_required",
    "production_durable_memory_target_required",
    "export_request_acceptance_denied",
    "export_request_recording_denied",
    "export_request_persistence_denied",
    "export_snapshot_materialization_denied",
    "export_file_write_denied",
    "export_stream_open_denied",
    "export_delivery_denied",
    "export_query_authority_promotion_denied",
    "query_registration_denied",
    "query_execution_denied",
    "query_result_recording_denied",
    "query_result_persistence_denied",
    "query_result_materialization_denied",
    "query_endpoint_materialization_denied",
    "query_index_recording_denied",
    "query_cache_write_denied",
    "search_index_write_denied",
    "observability_metric_recording_denied",
    "observability_log_recording_denied",
    "observability_trace_recording_denied",
    "observability_span_recording_denied",
    "observability_event_recording_denied",
    "observability_dashboard_materialization_denied",
    "observability_alert_registration_denied",
    "observability_slo_recording_denied",
    "operator_summary_recording_denied",
    "operator_summary_persistence_denied",
    "operator_summary_delivery_denied",
    "readback_evidence_recording_denied",
    "ledger_observability_recording_denied",
    "index_observability_recording_denied",
    "delivery_observability_recording_denied",
    "result_receipt_export_authority_promotion_denied",
    "result_receipt_query_authority_promotion_denied",
    "result_receipt_observability_authority_promotion_denied",
    "dry_run_execution_execution_denied",
    "dry_run_execution_result_receipt_persistence_denied",
    "retention_expiry_gc_state_mutation_denied",
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
    "dry_run_execution_result_receipt_export_recorded",
    "dry_run_execution_result_receipt_export_persisted",
    "dry_run_execution_result_receipt_export_snapshot_materialized",
    "dry_run_execution_result_receipt_export_file_written",
    "dry_run_execution_result_receipt_export_stream_opened",
    "dry_run_execution_result_receipt_query_registered",
    "dry_run_execution_result_receipt_query_executed",
    "dry_run_execution_result_receipt_query_result_recorded",
    "dry_run_execution_result_receipt_query_index_recorded",
    "dry_run_execution_result_receipt_query_cache_written",
    "dry_run_execution_result_receipt_observability_metric_recorded",
    "dry_run_execution_result_receipt_observability_log_recorded",
    "dry_run_execution_result_receipt_observability_trace_recorded",
    "dry_run_execution_result_receipt_observability_event_recorded",
    "dry_run_execution_result_receipt_observability_dashboard_materialized",
    "dry_run_execution_result_receipt_observability_alert_registered",
    "dry_run_execution_result_receipt_observability_slo_recorded",
    "dry_run_execution_result_receipt_operator_summary_recorded",
    "dry_run_execution_result_receipt_readback_evidence_recorded",
    "dry_run_execution_result_receipt_authority_promoted_from_export",
    "dry_run_execution_result_receipt_authority_promoted_from_query",
    "dry_run_execution_result_receipt_authority_promoted_from_observability",
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
  ]) as $zero_keys
  | ([
    "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_performed",
    "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_result_recorded",
    "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_result_accepted",
    "source_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_accepted",
    "dry_run_execution_result_receipt_export_query_observability_denial_matrix_bound",
    "dry_run_execution_result_receipt_export_request_denied",
    "dry_run_execution_result_receipt_export_file_stream_denied",
    "dry_run_execution_result_receipt_query_registration_execution_denied",
    "dry_run_execution_result_receipt_query_index_cache_denied",
    "dry_run_execution_result_receipt_observability_metric_log_trace_event_denied",
    "dry_run_execution_result_receipt_dashboard_alert_slo_denied",
    "dry_run_execution_result_receipt_operator_summary_readback_denied",
    "dry_run_execution_result_receipt_export_query_observability_authority_denied",
    "dry_run_execution_result_receipt_export_query_observability_handoff_bound"
  ]) as $true_keys
  | ([
    {
      id:"scoped-production-durable-memory-write-dry-run-execution-result-receipt-export-query-observability-denial",
      fixture_id:"scoped-production-durable-memory-write-dry-run-execution-result-receipt-export-query-observability-denial",
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_accepted:true,
      reason:"dry_run_execution_result_receipt_export_query_observability_denial_bound_without_reporting_surface_persistence_authority_execution_or_production_write",
      dry_run_execution_result_receipt_export_recorded:false,
      dry_run_execution_result_receipt_query_registered:false,
      dry_run_execution_result_receipt_observability_metric_recorded:false,
      dry_run_execution_executed:false,
      production_durable_memory_store_write_performed:false,
      external_send_performed:false
    }
  ] + ([
    "missing-retention-expiry-garbage-collection-source",
    "export-request-snapshot-attempt",
    "export-file-stream-attempt",
    "query-registration-execution-attempt",
    "query-result-index-cache-attempt",
    "observability-metric-log-attempt",
    "observability-trace-span-event-attempt",
    "dashboard-alert-slo-attempt",
    "operator-summary-readback-authority-attempt"
  ] | map({
      id:., fixture_id:.,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_accepted:false,
      reason:"blocked_noop",
      dry_run_execution_result_receipt_export_recorded:false,
      dry_run_execution_result_receipt_query_registered:false,
      dry_run_execution_result_receipt_observability_metric_recorded:false,
      dry_run_execution_executed:false,
      production_durable_memory_store_write_performed:false,
      external_send_performed:false
    }))) as $fixtures
  | ($source
  + {
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    endpoint:$endpoint,
    source_command:$source_command,
    native_route:false,
    script_gate:true,
    compatibility_mode:"source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_status",
    side_effect_free:false,
    external_side_effect_free:true,
    audit_date:"2026-07-05",
    memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_ready:true,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_ready:true,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_performed:true,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_accepted:true,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_mode:"dry_run_execution_result_receipt_export_query_observability_denial_boundary_no_export_no_query_no_observability_no_dashboard_no_alert_no_operator_summary_no_authority_no_execution_no_production_durable_memory_mutation",
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_ready:true,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_report_sha256:$source_report_sha256,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_accepted_count:1,
    source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_fixture_count:1,
    source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_fixture_count:9,
    source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_count:62,
    approved_production_namespace:$approved_production_namespace,
    approved_production_store:$approved_production_store,
    approved_production_scope:$approved_production_scope,
    production_durable_memory_target_id:$production_durable_memory_target_id,
    production_durable_memory_payload_class:$production_durable_memory_payload_class,
    operator_packet_scope:$operator_packet_scope,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_hash_sha256:$source_retention_boundary_hash_sha256,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_policy_hash_sha256:$source_retention_policy_hash_sha256,
    source_dry_run_execution_result_receipt_retention_expiry_garbage_collection_result_hash_sha256:$source_retention_result_hash_sha256,
    source_dry_run_execution_result_receipt_retention_expiry_garbage_collection_handoff_hash_sha256:$source_retention_handoff_hash_sha256,
    source_dry_run_execution_result_receipt_garbage_collection_denial_hash_sha256:$source_garbage_collection_denial_hash_sha256,
    dry_run_execution_result_receipt_export_denial_hash_sha256:$export_denial_hash_sha256,
    dry_run_execution_result_receipt_query_denial_hash_sha256:$query_denial_hash_sha256,
    dry_run_execution_result_receipt_observability_denial_hash_sha256:$observability_denial_hash_sha256,
    dry_run_execution_result_receipt_export_query_observability_denial_matrix_hash_sha256:$export_query_observability_matrix_hash_sha256,
    dry_run_execution_result_receipt_export_query_observability_handoff_hash_sha256:$export_query_observability_handoff_hash_sha256,
    dry_run_execution_result_receipt_export_query_observability_result_hash_sha256:$export_query_observability_result_hash_sha256,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_hash_sha256:$export_query_observability_boundary_hash_sha256,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_policy_hash_sha256:$export_query_observability_policy_hash_sha256,
    required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_surface_count:($surfaces | length),
    ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_surface_count:($surfaces | length),
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_surfaces:$surfaces,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_fixture_count:($fixtures | length),
    accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_fixture_count:1,
    blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_fixture_count:9,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_fixtures:$fixtures,
    denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary:$denials,
    denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_count:($denials | length),
    allowed_next_actions:[
      {
        action:"run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_require_live_gate",
        status:"allowed_verification_only",
        accepts_export_query_observability_denial_matrix:true,
        exports_receipt:false,
        materializes_export_snapshot:false,
        opens_export_stream:false,
        registers_query:false,
        executes_query:false,
        records_query_result:false,
        writes_search_index:false,
        records_observability:false,
        materializes_dashboard:false,
        registers_alert:false,
        records_slo:false,
        records_operator_summary:false,
        records_readback_evidence:false,
        promotes_authority:false,
        executes_dry_run:false,
        persists_dry_run_result_receipt:false,
        writes_production_durable_memory:false,
        writes_memory_store:false,
        writes_wal:false,
        persists_receipt:false
      },
      {
        action:"prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary",
        status:"requires_separate_result_receipt_operator_summary_briefing_denial_gate",
        requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary:true,
        exports_receipt:false,
        registers_query:false,
        records_observability:false,
        persists_operator_summary:false,
        delivers_operator_summary:false,
        executes_dry_run:false,
        writes_production_durable_memory:false,
        persists_dry_run_result_receipt:false
      }
    ]
  }
  + zero_fields($zero_keys)
  + true_count_fields($true_keys)
  + {
    source_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_bound:true,
    dry_run_execution_result_receipt_export_denial_bound:true,
    dry_run_execution_result_receipt_query_denial_bound:true,
    dry_run_execution_result_receipt_observability_denial_bound:true,
    dry_run_execution_result_receipt_export_query_observability_persistence_forbidden:true,
    dry_run_execution_execution_forbidden_on_export_query_observability_route:true,
    production_write_execution_forbidden_on_export_query_observability_route:true,
    production_durable_memory_write_forbidden:true,
    memory_store_mutation_forbidden:true,
    kg_live_write_forbidden:true,
    provider_model_invocation_forbidden:true,
    credential_channel_public_release_forbidden:true,
    install_restart_active_binary_mutation_forbidden:true
  }) as $report
  | $report + {side_effects: (($report.side_effects // {}) + zero_fields($zero_keys) + true_count_fields($true_keys))}
  '
