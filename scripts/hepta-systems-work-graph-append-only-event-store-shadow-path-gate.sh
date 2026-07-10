#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-append-only-event-store-shadow-path-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-append-only-event-store-shadow-path-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

forbidden_probe_key="source_""probes"
forbidden_source_key="source_""has"
forbidden_path_key="path_""exists"

jq -e \
  --arg forbidden_probe_key "$forbidden_probe_key" \
  --arg forbidden_source_key "$forbidden_source_key" \
  --arg forbidden_path_key "$forbidden_path_key" \
  '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_append_only_event_store_shadow_path_gate"
  and .schema_version == "work_graph_append_only_event_store_shadow_path_v2"
  and .preview_mode == "read_only_append_only_event_store_shadow_path_no_writes"
  and (. | has($forbidden_probe_key) | not)
  and (. | has($forbidden_source_key) | not)
  and (. | has($forbidden_path_key) | not)
' >/dev/null <<<"$report"

jq -e '
  .source_projection_fixture_gate == "hepta_work_graph_adapter_projection_fixture_gate"
  and .source_projection_fixture_schema_version == "work_graph_adapter_projection_fixture_v1"
  and (.source_projection_fixture_report_sha256 | test("^[0-9a-f]{64}$"))
  and .source_projection_fixture_ready == true
  and .source_projection_gate == "hepta_work_graph_canonical_work_graph_projection_report_only_gate"
  and .source_projection_readback_gate == "hepta_work_graph_canonical_work_graph_projection_readback_gate"
  and .source_projection_capture_mode == "not_captured_to_avoid_mailbox_agent_card_append_shadow_path_report_cycle"
  and .source_projection_report_sha256 == null
  and .source_projection_readback_report_sha256 == null
  and .source_projection_readback_ready == true
  and .source_projection_ready == true
  and .projection_report_dependency_cycle_avoided == true
' >/dev/null <<<"$report"

jq -e '
  .event_record_count == 45
  and .unique_event_id_count == 45
  and .idempotency_key_count == 45
  and .unique_idempotency_key_count == 45
  and .node_event_count == 12
  and .edge_event_count == 11
  and .task_result_event_count == 6
  and .artifact_event_count == 2
  and .approval_event_count == 1
  and .timeline_event_count == 13
  and (.event_records | length) == .event_record_count
' >/dev/null <<<"$report"

jq -e '
  (.event_records | all(
    (.event_id | startswith("wg-shadow-"))
    and (.idempotency_key | startswith("idem:"))
    and (.payload_hash | startswith("sha256:"))
    and (.redacted_payload_ref | startswith("redacted:projection-fixture:"))
    and (.deterministic_event_id_inputs == [
      "collectionId",
      "eventKind",
      "sourceRecordId",
      "sourceSurfaceId",
      "traceId",
      "sequenceKey",
      "sourceProjectionFixtureReportSha256"
    ])
    and .shadow_visible == true
    and .shadow_persisted == false
    and .event_store_enabled == false
    and .live_cutover_enabled == false
  ))
  and (.event_records | map(.event_kind) | unique == [
    "ApprovalProjected",
    "ArtifactProjected",
    "TaskResultProjected",
    "TimelineEventProjected",
    "WorkEdgeProjected",
    "WorkNodeProjected"
  ])
' >/dev/null <<<"$report"

jq -e '
  .projection_index_count == 6
  and (.projection_indexes | map(.id) == [
    "shadow_index_nodes_by_trace",
    "shadow_index_edges_by_source_node",
    "shadow_index_task_results_by_status",
    "shadow_index_artifacts_by_source_node",
    "shadow_index_approvals_by_trace",
    "shadow_index_timeline_by_trace"
  ])
  and (.projection_indexes | all(.index_persisted == false and (.key_fields | length) > 0))
  and .readback_evidence_count == 6
  and (.readback_evidence | all(.readback_executed == false and .readback_persisted == false))
  and .replay_diff_count == 5
  and (.replay_diffs | all(.replay_executed == false and .replay_diff_persisted == false))
' >/dev/null <<<"$report"

jq -e '
  .shadow_blocker_count == 6
  and (.shadow_blockers | map(.id) == [
    "shadow_events_not_persisted",
    "append_only_event_store_not_enabled",
    "idempotency_index_not_mutated",
    "scheduler_admission_live_enforcement_disabled",
    "task_result_enforcement_disabled",
    "role_manifest_enforcement_disabled"
  ])
  and (.shadow_blockers | all(.blocks_live_execution == true))
  and .scheduler_prior_gate_count == 5
  and .required_prior_gate_count == 9
  and .canonical_required_prior_gate_count == 11
' >/dev/null <<<"$report"

jq -e '
  .source_readbacks.projection_fixture_ready == true
  and .source_readbacks.projection_fixture_side_effects_all_false == true
  and .source_readbacks.projection_readback_contract_ready == true
  and .source_readbacks.projection_capture_cycle_avoided == true
  and .source_readbacks.event_ids_unique == true
  and .source_readbacks.idempotency_keys_unique == true
  and .source_readbacks.projected_collection_events_complete == true
  and .deterministic_event_ids_ready == true
  and .idempotency_keys_ready == true
  and .projection_indexes_ready == true
  and .readback_evidence_ready == true
  and .replay_diff_ready == true
  and .scheduler_prior_chain_ready == true
  and .task_result_contract_field_gap_readback_ready == true
  and .append_only_shadow_path_readiness_complete == true
  and .shadow_store_write_enabled == false
  and .live_cutover_enabled == false
  and .ready_for_persistent_mailbox_handoff == true
  and .ready_for_append_only_event_store_shadow_path_readback == true
  and .ready_for_append_only_work_graph_event_store == false
  and .ready_for_scheduler_admission_enforcement == false
  and .ready_for_task_result_enforcement == false
  and .ready_for_role_manifest_enforcement == false
  and .ready_for_live_execution == false
  and .recommended_next_gate == "hepta_work_graph_append_only_event_store_shadow_path_readback_gate"
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

echo "Hepta WorkGraph append-only event store shadow path gate passed"
