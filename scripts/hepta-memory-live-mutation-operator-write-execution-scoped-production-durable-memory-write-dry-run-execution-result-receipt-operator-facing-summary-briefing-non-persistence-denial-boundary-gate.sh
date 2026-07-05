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
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-export-query-observability-denial-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-export-query-observability-denial-boundary-gate.sh
)"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_ready == true
    and $source.scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_accepted == true
    and $source.accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_fixture_count == 1
    and $source.blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_fixture_count == 9
    and $source.denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_count == 64
    and $source.dry_run_execution_result_receipt_operator_summary_recorded == false
    and $source.dry_run_execution_result_receipt_authority_promoted_from_observability == false
    and $source.dry_run_execution_executed == false
    and $source.production_durable_memory_store_write_performed == false
    and $source.memory_store_write_performed == false
    and $source.wal_write_performed == false
    and $source.receipt_persisted == false
    and $source.external_send_performed == false
    and $source.allowed_next_actions[1].action == "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary"
    and $source.allowed_next_actions[1].requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary == true
    and $source.allowed_next_actions[1].persists_operator_summary == false
    and $source.allowed_next_actions[1].delivers_operator_summary == false
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
source_export_query_observability_boundary_hash_sha256="$(jq -r '.scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_export_query_observability_policy_hash_sha256="$(jq -r '.scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_policy_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_export_query_observability_result_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_export_query_observability_result_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_export_query_observability_handoff_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_export_query_observability_handoff_hash_sha256 // ""' <<<"$SOURCE_JSON")"

summary_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-denial:v1:source=${source_export_query_observability_result_hash_sha256}:record=false:persist=false:materialize=false:deliver=false:authority=false"
)"
briefing_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-operator-briefing-denial:v1:summary=${summary_denial_hash_sha256}:record=false:persist=false:materialize=false:deliver=false:authority=false"
)"
readout_ack_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-operator-readout-ack-denial:v1:briefing=${briefing_denial_hash_sha256}:readout=false:handoff=false:ack=false:decision=false:status=false:authority=false"
)"
operator_summary_briefing_handoff_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-briefing-handoff:v1:readout_ack=${readout_ack_denial_hash_sha256}:next=final-operator-acknowledgement-non-acceptance-denial-boundary"
)"
operator_summary_briefing_result_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-briefing-result:v1:summary=${summary_denial_hash_sha256}:briefing=${briefing_denial_hash_sha256}:readout_ack=${readout_ack_denial_hash_sha256}:handoff=${operator_summary_briefing_handoff_hash_sha256}:accepted=true:persist=false:delivery=false:authority=false:execution=false:production-write=false"
)"
operator_summary_briefing_boundary_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-briefing-non-persistence-denial-boundary:v1:source=${source_report_sha256}:result=${operator_summary_briefing_result_hash_sha256}:accepted=1:blocked=9:summary=false:briefing=false:ack=false:authority=false:execution=false:production-write=false"
)"
operator_summary_briefing_policy_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-briefing-non-persistence-denial-policy:v1:bind-source-export-query-observability-no-summary-recording-no-briefing-recording-no-materialization-no-filesystem-no-channel-no-readout-no-handoff-no-final-ack-no-decision-status-no-authority-no-execution-no-production-write-no-kg-no-provider-no-release-no-install"
)"
operator_summary_briefing_matrix_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-briefing-denial-matrix:v1:summary=${summary_denial_hash_sha256}:briefing=${briefing_denial_hash_sha256}:readout_ack=${readout_ack_denial_hash_sha256}:fixtures=10"
)"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_gate" \
  --arg endpoint "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-operator-facing-summary-briefing-non-persistence-denial-boundary" \
  --arg source_command "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-operator-facing-summary-briefing-non-persistence-denial-boundary --json" \
  --arg approved_production_namespace "$approved_production_namespace" \
  --arg approved_production_store "$approved_production_store" \
  --arg approved_production_scope "$approved_production_scope" \
  --arg production_durable_memory_target_id "$production_durable_memory_target_id" \
  --arg production_durable_memory_payload_class "$production_durable_memory_payload_class" \
  --arg operator_packet_scope "$operator_packet_scope" \
  --arg source_report_sha256 "$source_report_sha256" \
  --arg source_export_query_observability_boundary_hash_sha256 "$source_export_query_observability_boundary_hash_sha256" \
  --arg source_export_query_observability_policy_hash_sha256 "$source_export_query_observability_policy_hash_sha256" \
  --arg source_export_query_observability_result_hash_sha256 "$source_export_query_observability_result_hash_sha256" \
  --arg source_export_query_observability_handoff_hash_sha256 "$source_export_query_observability_handoff_hash_sha256" \
  --arg summary_denial_hash_sha256 "$summary_denial_hash_sha256" \
  --arg briefing_denial_hash_sha256 "$briefing_denial_hash_sha256" \
  --arg readout_ack_denial_hash_sha256 "$readout_ack_denial_hash_sha256" \
  --arg operator_summary_briefing_matrix_hash_sha256 "$operator_summary_briefing_matrix_hash_sha256" \
  --arg operator_summary_briefing_handoff_hash_sha256 "$operator_summary_briefing_handoff_hash_sha256" \
  --arg operator_summary_briefing_result_hash_sha256 "$operator_summary_briefing_result_hash_sha256" \
  --arg operator_summary_briefing_boundary_hash_sha256 "$operator_summary_briefing_boundary_hash_sha256" \
  --arg operator_summary_briefing_policy_hash_sha256 "$operator_summary_briefing_policy_hash_sha256" \
  --argjson source "$SOURCE_JSON" \
  '
  def with_false_counts($keys):
    reduce $keys[] as $k ({}; .[$k]=false | .[$k + "_count"]=0);
  def with_true_counts($keys):
    reduce $keys[] as $k ({}; .[$k]=true | .[$k + "_count"]=1);
  def fixture($id; $status; $reason; $accepted; $extra):
    {
      id: $id,
      fixture_id: $id,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_status: $status,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_accepted: $accepted,
      reason: $reason,
      source_export_query_observability_present: true,
      source_export_query_observability_ready: true,
      summary_briefing_noop_confirmed: true,
      operator_summary_requested: false,
      operator_briefing_requested: false,
      operator_summary_materialization_requested: false,
      operator_briefing_materialization_requested: false,
      operator_summary_persistence_requested: false,
      operator_summary_filesystem_write_requested: false,
      operator_briefing_persistence_requested: false,
      operator_briefing_filesystem_write_requested: false,
      operator_readout_requested: false,
      operator_handoff_requested: false,
      final_operator_acknowledgement_requested: false,
      terminal_operator_decision_requested: false,
      terminal_operator_status_requested: false,
      channel_delivery_requested: false,
      telegram_send_requested: false,
      authority_promotion_requested: false,
      dry_run_execution_requested: false,
      production_write_requested: false,
      memory_write_summary_requested: false,
      rollback_summary_requested: false,
      secret_material_summary_requested: false,
      provider_prompt_summary_requested: false,
      external_send_summary_requested: false,
      public_claim_summary_requested: false,
      release_artifact_summary_requested: false,
      install_summary_requested: false,
      service_restart_summary_requested: false,
      active_binary_summary_requested: false,
      operator_summary_recorded: false,
      operator_summary_persisted: false,
      operator_summary_materialized: false,
      operator_summary_filesystem_written: false,
      operator_summary_delivered: false,
      operator_briefing_recorded: false,
      operator_briefing_persisted: false,
      operator_briefing_materialized: false,
      operator_briefing_filesystem_written: false,
      operator_briefing_delivered: false,
      operator_readout_recorded: false,
      operator_handoff_recorded: false,
      final_operator_acknowledgement_recorded: false,
      final_operator_acknowledgement_accepted: false,
      terminal_operator_decision_recorded: false,
      terminal_operator_status_recorded: false,
      authority_promoted: false,
      telegram_send_performed: false,
      channel_send_performed: false,
      external_send_performed: false,
      dry_run_execution_executed: false,
      production_durable_memory_store_write_performed: false,
      memory_store_write_performed: false,
      wal_write_performed: false,
      receipt_persisted: false,
      rollback_executed: false,
      provider_invoked: false,
      model_invoked: false,
      credential_read: false,
      release_artifact_written: false,
      install_executed: false,
      service_restarted: false,
      active_binary_mutated: false
    } + $extra;
  ([
    "source_export_query_observability_denial_boundary_required",
    "source_export_query_observability_result_required",
    "operator_facing_summary_request_denied",
    "operator_briefing_request_denied",
    "operator_facing_summary_materialization_denied",
    "operator_briefing_materialization_denied",
    "operator_facing_summary_persistence_denied",
    "operator_briefing_persistence_denied",
    "operator_facing_summary_delivery_denied",
    "operator_briefing_delivery_denied",
    "operator_readout_handoff_denied",
    "final_acknowledgement_decision_status_denied",
    "operator_summary_briefing_authority_promotion_denied",
    "dry_run_execution_production_write_and_authority_forbidden_on_operator_summary_briefing_route"
  ]) as $surfaces
  | ([
    "source_export_query_observability_denial_boundary_required",
    "source_export_query_observability_result_hash_required",
    "source_export_query_observability_policy_hash_required",
    "source_export_query_observability_handoff_hash_required",
    "approved_production_namespace_required",
    "approved_production_store_required",
    "approved_production_scope_required",
    "production_durable_memory_target_required",
    "operator_facing_summary_request_acceptance_denied",
    "operator_facing_summary_recording_denied",
    "operator_facing_summary_persistence_denied",
    "operator_facing_summary_materialization_denied",
    "operator_facing_summary_filesystem_write_denied",
    "operator_facing_summary_delivery_denied",
    "operator_facing_summary_channel_delivery_denied",
    "operator_briefing_request_acceptance_denied",
    "operator_briefing_recording_denied",
    "operator_briefing_persistence_denied",
    "operator_briefing_materialization_denied",
    "operator_briefing_filesystem_write_denied",
    "operator_briefing_delivery_denied",
    "operator_briefing_channel_delivery_denied",
    "operator_readout_recording_denied",
    "operator_readout_persistence_denied",
    "operator_readout_materialization_denied",
    "operator_readout_delivery_denied",
    "operator_readout_readback_evidence_denied",
    "operator_handoff_recording_denied",
    "operator_handoff_persistence_denied",
    "operator_handoff_delivery_denied",
    "final_operator_acknowledgement_recording_denied",
    "final_operator_acknowledgement_persistence_denied",
    "final_operator_acknowledgement_acceptance_denied",
    "final_operator_acknowledgement_delivery_denied",
    "terminal_operator_decision_recording_denied",
    "terminal_operator_decision_persistence_denied",
    "terminal_operator_decision_acceptance_denied",
    "terminal_operator_status_recording_denied",
    "terminal_operator_status_persistence_denied",
    "terminal_operator_status_promotion_denied",
    "result_receipt_operator_summary_authority_promotion_denied",
    "result_receipt_operator_briefing_authority_promotion_denied",
    "result_receipt_operator_readout_authority_promotion_denied",
    "result_receipt_final_acknowledgement_authority_promotion_denied",
    "dry_run_execution_execution_denied",
    "dry_run_execution_result_receipt_persistence_denied",
    "export_query_observability_state_mutation_denied",
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
    "telegram_channel_delivery_denied",
    "external_send_denied",
    "release_public_artifact_write_denied",
    "install_restart_authority_denied",
    "active_binary_mutation_denied",
    "unrestricted_full_live_activation_denied"
  ]) as $denials
  | ([
    "dry_run_execution_result_receipt_operator_facing_summary_recorded",
    "dry_run_execution_result_receipt_operator_facing_summary_persisted",
    "dry_run_execution_result_receipt_operator_facing_summary_materialized",
    "dry_run_execution_result_receipt_operator_facing_summary_filesystem_written",
    "dry_run_execution_result_receipt_operator_facing_summary_delivered",
    "dry_run_execution_result_receipt_operator_briefing_recorded",
    "dry_run_execution_result_receipt_operator_briefing_persisted",
    "dry_run_execution_result_receipt_operator_briefing_materialized",
    "dry_run_execution_result_receipt_operator_briefing_filesystem_written",
    "dry_run_execution_result_receipt_operator_briefing_delivered",
    "dry_run_execution_result_receipt_operator_readout_recorded",
    "dry_run_execution_result_receipt_operator_handoff_recorded",
    "dry_run_execution_result_receipt_final_operator_acknowledgement_recorded",
    "dry_run_execution_result_receipt_final_operator_acknowledgement_accepted",
    "dry_run_execution_result_receipt_terminal_operator_decision_recorded",
    "dry_run_execution_result_receipt_terminal_operator_status_recorded",
    "dry_run_execution_result_receipt_authority_promoted_from_operator_summary",
    "dry_run_execution_result_receipt_authority_promoted_from_operator_briefing",
    "dry_run_execution_result_receipt_authority_promoted_from_final_acknowledgement",
    "dry_run_execution_executed",
    "production_durable_memory_write_executed",
    "production_durable_memory_store_write_performed",
    "actual_production_durable_memory_write_performed",
    "durable_memory_store_write_performed",
    "memory_store_write_performed",
    "memory_store_mutated",
    "wal_write_performed",
    "receipt_persisted",
    "rollback_executed",
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
    "active_binary_mutated",
    "filesystem_written"
  ]) as $false_keys
  | ([
    "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_performed",
    "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_result_recorded",
    "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_result_accepted",
    "source_dry_run_execution_result_receipt_export_query_observability_denial_boundary_bound",
    "dry_run_execution_result_receipt_operator_facing_summary_request_denied",
    "dry_run_execution_result_receipt_operator_briefing_request_denied",
    "dry_run_execution_result_receipt_operator_summary_briefing_materialization_denied",
    "dry_run_execution_result_receipt_operator_summary_briefing_persistence_denied",
    "dry_run_execution_result_receipt_operator_summary_briefing_delivery_denied",
    "dry_run_execution_result_receipt_operator_readout_handoff_denied",
    "dry_run_execution_result_receipt_final_acknowledgement_decision_status_denied",
    "dry_run_execution_result_receipt_operator_summary_briefing_authority_denied",
    "dry_run_execution_result_receipt_operator_summary_briefing_handoff_bound"
  ]) as $true_keys
  | ([
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-briefing-report-only-denial"; "accepted_non_persistent_operator_summary_briefing_denial"; "source_export_query_observability_denial_bound_without_summary_briefing_persistence_delivery_or_authority"; true; {}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-briefing-missing-source"; "blocked_noop"; "source_export_query_observability_report_required"; false; {source_export_query_observability_present:false, source_export_query_observability_ready:false, operator_summary_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-request"; "blocked_summary_noop"; "operator_facing_summary_request_shape_denied"; false; {operator_summary_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-operator-briefing-request"; "blocked_briefing_noop"; "operator_briefing_request_shape_denied"; false; {operator_briefing_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-briefing-materialization-request"; "blocked_materialization_noop"; "summary_briefing_materialization_denied"; false; {operator_summary_requested:true, operator_briefing_requested:true, operator_summary_materialization_requested:true, operator_briefing_materialization_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-briefing-persistence-filesystem-write-request"; "blocked_persistence_noop"; "summary_briefing_persistence_filesystem_write_denied"; false; {operator_summary_requested:true, operator_briefing_requested:true, operator_summary_persistence_requested:true, operator_summary_filesystem_write_requested:true, operator_briefing_persistence_requested:true, operator_briefing_filesystem_write_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-briefing-channel-delivery-request"; "blocked_delivery_noop"; "summary_briefing_channel_delivery_denied"; false; {operator_summary_requested:true, operator_briefing_requested:true, channel_delivery_requested:true, telegram_send_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-operator-readout-handoff-request"; "blocked_readout_noop"; "operator_readout_handoff_denied"; false; {operator_readout_requested:true, operator_handoff_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-final-ack-decision-status-request"; "blocked_ack_decision_status_noop"; "final_acknowledgement_decision_status_denied"; false; {final_operator_acknowledgement_requested:true, terminal_operator_decision_requested:true, terminal_operator_status_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-briefing-authority-memory-provider-external-request"; "blocked_authority_noop"; "operator_summary_briefing_authority_memory_provider_external_denied"; false; {operator_summary_requested:true, operator_briefing_requested:true, authority_promotion_requested:true, dry_run_execution_requested:true, production_write_requested:true, memory_write_summary_requested:true, rollback_summary_requested:true, secret_material_summary_requested:true, provider_prompt_summary_requested:true, external_send_summary_requested:true, public_claim_summary_requested:true, release_artifact_summary_requested:true, install_summary_requested:true, service_restart_summary_requested:true, active_binary_summary_requested:true})
  ]) as $fixtures
  | {
    product: $product,
    runtime: $runtime,
    status: "ready",
    base_url: $base_url,
    gate: $gate,
    endpoint: $endpoint,
    source_command: $source_command,
    native_route: true,
    side_effect_free: true,
    audit_date: "2026-07-05",
    memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_schema_version: "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_v1",
    memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_ready: true,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready: true,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_accepted: true,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_mode: "dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_no_summary_no_briefing_no_delivery_no_ack_no_authority_no_execution_no_production_durable_memory_mutation",
    approved_production_namespace: $approved_production_namespace,
    approved_production_store: $approved_production_store,
    approved_production_scope: $approved_production_scope,
    production_durable_memory_target_id: $production_durable_memory_target_id,
    production_durable_memory_payload_class: $production_durable_memory_payload_class,
    operator_packet_scope: $operator_packet_scope,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_ready: true,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_accepted_count: 1,
    source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_fixture_count: 1,
    source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_fixture_count: 9,
    source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_count: 64,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_report_sha256: $source_report_sha256,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_hash_sha256: $source_export_query_observability_boundary_hash_sha256,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_policy_hash_sha256: $source_export_query_observability_policy_hash_sha256,
    source_dry_run_execution_result_receipt_export_query_observability_result_hash_sha256: $source_export_query_observability_result_hash_sha256,
    source_dry_run_execution_result_receipt_export_query_observability_handoff_hash_sha256: $source_export_query_observability_handoff_hash_sha256,
    dry_run_execution_result_receipt_operator_summary_denial_hash_sha256: $summary_denial_hash_sha256,
    dry_run_execution_result_receipt_operator_briefing_denial_hash_sha256: $briefing_denial_hash_sha256,
    dry_run_execution_result_receipt_operator_readout_ack_denial_hash_sha256: $readout_ack_denial_hash_sha256,
    dry_run_execution_result_receipt_operator_summary_briefing_denial_matrix_hash_sha256: $operator_summary_briefing_matrix_hash_sha256,
    dry_run_execution_result_receipt_operator_summary_briefing_handoff_hash_sha256: $operator_summary_briefing_handoff_hash_sha256,
    dry_run_execution_result_receipt_operator_summary_briefing_result_hash_sha256: $operator_summary_briefing_result_hash_sha256,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_hash_sha256: $operator_summary_briefing_boundary_hash_sha256,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_policy_hash_sha256: $operator_summary_briefing_policy_hash_sha256,
    required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_surface_count: ($surfaces | length),
    ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_surface_count: ($surfaces | length),
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_surfaces: $surfaces,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixture_count: ($fixtures | length),
    accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixture_count: ([$fixtures[] | select(.scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_accepted == true)] | length),
    blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixture_count: ([$fixtures[] | select(.scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_accepted == false)] | length),
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixtures: $fixtures,
    denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary: $denials,
    denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_count: ($denials | length),
    allowed_next_actions: [
      {
        action: "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_require_live_gate",
        status: "allowed_verification_only",
        records_operator_summary: false,
        persists_operator_summary: false,
        materializes_operator_summary: false,
        writes_operator_summary_filesystem: false,
        records_operator_briefing: false,
        persists_operator_briefing: false,
        materializes_operator_briefing: false,
        writes_operator_briefing_filesystem: false,
        records_operator_readout: false,
        records_final_acknowledgement: false,
        records_terminal_decision: false,
        delivers_notification: false,
        sends_telegram: false,
        promotes_authority: false,
        executes_dry_run: false,
        writes_production_durable_memory: false,
        writes_memory_store: false,
        writes_wal: false,
        persists_receipt: false
      },
      {
        action: "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary",
        status: "requires_separate_result_receipt_final_operator_acknowledgement_denial_gate",
        requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary: true,
        accepts_operator_acknowledgement: false,
        persists_acknowledgement: false,
        records_terminal_decision: false,
        promotes_authority: false,
        executes_dry_run: false,
        writes_production_durable_memory: false,
        persists_dry_run_result_receipt: false
      }
    ]
  }
  + with_false_counts($false_keys)
  + with_true_counts($true_keys)
  + {
    source_dry_run_execution_result_receipt_export_query_observability_denial_boundary_bound: true,
    approved_production_namespace_bound: true,
    approved_production_store_bound: true,
    approved_production_scope_bound: true,
    production_durable_memory_target_bound: true,
    dry_run_execution_result_receipt_operator_summary_denial_bound: true,
    dry_run_execution_result_receipt_operator_briefing_denial_bound: true,
    dry_run_execution_result_receipt_operator_readout_ack_denial_bound: true,
    dry_run_execution_result_receipt_operator_facing_summary_briefing_request_denied: true,
    dry_run_execution_result_receipt_operator_summary_briefing_persistence_forbidden: true,
    dry_run_execution_execution_forbidden_on_operator_summary_briefing_route: true,
    dry_run_execution_result_receipt_persistence_forbidden_on_operator_summary_briefing_route: true,
    production_write_execution_forbidden_on_operator_summary_briefing_route: true,
    production_durable_memory_write_forbidden: true,
    memory_store_mutation_forbidden: true,
    kg_live_write_forbidden: true,
    provider_model_invocation_forbidden: true,
    credential_channel_public_release_forbidden: true,
    install_restart_active_binary_mutation_forbidden: true,
    side_effects: (
      with_false_counts($false_keys)
      | with_entries(select(.value | type != "number"))
      + {
        scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_performed: true,
        scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_result_accepted: true
      }
    )
  }
  '
