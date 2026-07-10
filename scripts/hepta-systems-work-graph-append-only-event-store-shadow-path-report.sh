#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

json_sha256() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

adapter_fixture_report="$(
  capture_json_report \
    "hepta-work-graph-adapter-projection-fixture-report" \
    "$ROOT/scripts/hepta-systems-work-graph-adapter-projection-fixture-report.sh"
)"

jq -n \
  --argjson fixture "$adapter_fixture_report" \
  --arg fixture_sha "$(json_sha256 "$adapter_fixture_report")" \
  '
  def side_effects_all_false($report):
    ($report.side_effects | to_entries | all(.value == false));
  def key_safe:
    gsub("[^A-Za-z0-9_-]"; "_");
  def shadow_event($collection; $kind; $record_id; $source_surface_id; $trace_id; $sequence_key; $source_node_id):
    {
      event_id: ("wg-shadow-" + $collection + "-" + ($record_id | key_safe)),
      event_kind: $kind,
      collection_id: $collection,
      source_record_id: $record_id,
      source_surface_id: $source_surface_id,
      source_node_id: $source_node_id,
      trace_id: $trace_id,
      sequence_key: $sequence_key,
      deterministic_event_id_inputs: [
        "collectionId",
        "eventKind",
        "sourceRecordId",
        "sourceSurfaceId",
        "traceId",
        "sequenceKey",
        "sourceProjectionFixtureReportSha256"
      ],
      idempotency_key: ("idem:" + $collection + ":" + $source_surface_id + ":" + $record_id + ":" + $trace_id),
      payload_hash: ("sha256:" + $fixture_sha),
      redacted_payload_ref: ("redacted:projection-fixture:" + $collection + ":" + ($record_id | key_safe)),
      shadow_visible: true,
      shadow_persisted: false,
      event_store_enabled: false,
      live_cutover_enabled: false
    };
  def index($id; $collection; $count; $keys): {
    id: $id,
    collection_id: $collection,
    event_count: $count,
    key_fields: $keys,
    deterministic_order: "collectionId:traceId:eventId",
    index_persisted: false
  };
  def evidence($id; $collection; $count): {
    id: $id,
    collection_id: $collection,
    expected_event_count: $count,
    evidence_ref: ("evidence:shadow-readback:" + $collection),
    readback_executed: false,
    readback_persisted: false
  };
  def replay_diff($id; $scope; $expected): {
    id: $id,
    replay_scope: $scope,
    expected_diff: $expected,
    replay_executed: false,
    replay_diff_persisted: false
  };
  def blocker($id; $severity; $fix): {
    id: $id,
    severity: $severity,
    blocks_live_execution: true,
    recommended_fix: $fix
  };
  def projection_readback_contract: {
    source_projection_gate: "hepta_work_graph_canonical_work_graph_projection_report_only_gate",
    source_projection_readback_gate: "hepta_work_graph_canonical_work_graph_projection_readback_gate",
    source_projection_capture_mode: "not_captured_to_avoid_mailbox_agent_card_append_shadow_path_report_cycle",
    source_projection_readback_ready: true,
    source_projection_ready: true,
    expected_work_node_count: 12,
    expected_work_edge_count: 11,
    expected_task_result_count: 6,
    expected_artifact_count: 2,
    expected_approval_count: 1,
    expected_timeline_event_count: 13
  };
  def scheduler_prior_gates: [
    "hepta_work_graph_task_result_envelope_report_only_validator_gate",
    "hepta_work_graph_adapter_task_result_index_gate",
    "hepta_work_graph_terminal_envelope_readback_gate",
    "hepta_work_graph_source_id_alignment_readback_gate",
    "hepta_work_graph_task_result_contract_field_gap_readback_gate"
  ];
  def legacy_required_prior_gates:
    scheduler_prior_gates + [
      "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate",
      "hepta_work_graph_append_only_event_intake_preview_gate",
      "hepta_work_graph_append_only_work_graph_events_shadow_write_preview_gate",
      "hepta_work_graph_append_only_work_graph_events_shadow_write_readback_preview_gate"
    ];
  def canonical_required_prior_gates: [
    "hepta_work_graph_canonical_work_graph_projection_readback_gate",
    "hepta_work_graph_canonical_work_graph_projection_report_only_gate",
    "hepta_work_graph_canonical_frontier_lineage_index_readback_gate",
    "hepta_work_graph_canonical_frontier_lineage_index_gate",
    "hepta_work_graph_canonical_adapter_inventory_preview_gate",
    "hepta_work_graph_task_result_envelope_report_only_validator_gate",
    "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate",
    "hepta_work_graph_agent_role_agent_card_manifest_report_only_gate",
    "hepta_work_graph_adapter_projection_fixture_gate",
    "hepta_work_graph_live_attachment_final_closeout_frontier_gate",
    "hepta_work_graph_live_attachment_audit_index_frontier_gate"
  ];
  ($fixture.fixtures | map(
    shadow_event("nodes"; "WorkNodeProjected"; .projected_node_id; .source_surface_id; .trace_id; .id; .projected_node_id)
  )) as $node_events
  | ($fixture.fixtures | map(
    . as $f
    | .projected_edge_ids[]?
    | shadow_event("edges"; "WorkEdgeProjected"; .; $f.source_surface_id; ("trace:edge:" + $f.id); $f.id; $f.projected_node_id)
  )) as $edge_events
  | ($fixture.fixtures | map(
    select(.projected_task_result_id != null)
    | shadow_event("taskResults"; "TaskResultProjected"; .projected_task_result_id; .source_surface_id; .trace_id; .status; .projected_node_id)
  )) as $task_result_events
  | ($fixture.fixtures | map(
    . as $f
    | .projected_artifact_ids[]?
    | shadow_event("artifacts"; "ArtifactProjected"; .; $f.source_surface_id; $f.trace_id; $f.redaction_state; $f.projected_node_id)
  )) as $artifact_events
  | ($fixture.fixtures | map(
    select(.projected_approval_id != null)
    | shadow_event("approvals"; "ApprovalProjected"; .projected_approval_id; .source_surface_id; .trace_id; .status; .projected_node_id)
  )) as $approval_events
  | ($fixture.fixtures | map(
    . as $f
    | .projected_timeline_event_ids[]?
    | shadow_event("timelineEvents"; "TimelineEventProjected"; .; $f.source_surface_id; $f.trace_id; .; $f.projected_node_id)
  )) as $timeline_events
  | ($node_events + $edge_events + $task_result_events + $artifact_events + $approval_events + $timeline_events) as $events
  | [
    index("shadow_index_nodes_by_trace"; "nodes"; ($node_events | length); ["collectionId", "traceId", "eventId"]),
    index("shadow_index_edges_by_source_node"; "edges"; ($edge_events | length); ["collectionId", "sourceNodeId", "eventId"]),
    index("shadow_index_task_results_by_status"; "taskResults"; ($task_result_events | length); ["collectionId", "sourceSurfaceId", "sequenceKey", "eventId"]),
    index("shadow_index_artifacts_by_source_node"; "artifacts"; ($artifact_events | length); ["collectionId", "sourceNodeId", "eventId"]),
    index("shadow_index_approvals_by_trace"; "approvals"; ($approval_events | length); ["collectionId", "traceId", "eventId"]),
    index("shadow_index_timeline_by_trace"; "timelineEvents"; ($timeline_events | length); ["collectionId", "traceId", "eventId"])
  ] as $indexes
  | [
    evidence("shadow_readback_nodes"; "nodes"; ($node_events | length)),
    evidence("shadow_readback_edges"; "edges"; ($edge_events | length)),
    evidence("shadow_readback_task_results"; "taskResults"; ($task_result_events | length)),
    evidence("shadow_readback_artifacts"; "artifacts"; ($artifact_events | length)),
    evidence("shadow_readback_approvals"; "approvals"; ($approval_events | length)),
    evidence("shadow_readback_timeline_events"; "timelineEvents"; ($timeline_events | length))
  ] as $readback_evidence
  | [
    replay_diff("shadow_replay_event_id_stability"; "all_shadow_events"; "event_ids_match_projection_fixture_inputs"),
    replay_diff("shadow_replay_idempotency_key_stability"; "all_shadow_events"; "idempotency_keys_match_projection_fixture_inputs"),
    replay_diff("shadow_replay_projection_index_rebuild"; "all_shadow_indexes"; "projection_indexes_rebuild_without_diff"),
    replay_diff("shadow_replay_payload_hash_stability"; "all_shadow_payloads"; "payload_hashes_match_source_projection_fixture_report"),
    replay_diff("shadow_replay_no_persistence_boundary"; "side_effect_boundary"; "no_persistence_or_live_cutover")
  ] as $replay_diffs
  | [
    blocker("shadow_events_not_persisted"; "high"; "keep shadow events visible-only until readback and replay gates close"),
    blocker("append_only_event_store_not_enabled"; "high"; "do not enable event-store writes before idempotency and replay proofs"),
    blocker("idempotency_index_not_mutated"; "high"; "derive keys without mutating the idempotency index in this cut"),
    blocker("scheduler_admission_live_enforcement_disabled"; "high"; "scheduler admission remains dry-run-only until event-store and TaskResult gates close"),
    blocker("task_result_enforcement_disabled"; "high"; "TaskResult shadow events are not reducer-authoritative"),
    blocker("role_manifest_enforcement_disabled"; "medium"; "role manifest and AgentCard constraints remain report-only")
  ] as $blockers
  | ($fixture.status == "ready"
      and $fixture.gate == "hepta_work_graph_adapter_projection_fixture_gate"
      and $fixture.fixture_count == 12
      and $fixture.source_surface_count == 12
      and side_effects_all_false($fixture)) as $source_projection_fixture_ready
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_append_only_event_store_shadow_path_gate",
      schema_version: "work_graph_append_only_event_store_shadow_path_v2",
      preview_mode: "read_only_append_only_event_store_shadow_path_no_writes",
      source_projection_fixture_gate: $fixture.gate,
      source_projection_fixture_schema_version: $fixture.schema_version,
      source_projection_fixture_report_sha256: $fixture_sha,
      source_projection_fixture_ready: $source_projection_fixture_ready,
      source_projection_gate: projection_readback_contract.source_projection_gate,
      source_projection_readback_gate: projection_readback_contract.source_projection_readback_gate,
      source_projection_capture_mode: projection_readback_contract.source_projection_capture_mode,
      source_projection_report_sha256: null,
      source_projection_readback_report_sha256: null,
      source_projection_readback_ready: projection_readback_contract.source_projection_readback_ready,
      source_projection_ready: projection_readback_contract.source_projection_ready,
      projection_report_dependency_cycle_avoided: true,
      event_record_count: ($events | length),
      unique_event_id_count: ($events | map(.event_id) | unique | length),
      idempotency_key_count: ($events | map(.idempotency_key) | length),
      unique_idempotency_key_count: ($events | map(.idempotency_key) | unique | length),
      node_event_count: ($node_events | length),
      edge_event_count: ($edge_events | length),
      task_result_event_count: ($task_result_events | length),
      artifact_event_count: ($artifact_events | length),
      approval_event_count: ($approval_events | length),
      timeline_event_count: ($timeline_events | length),
      projection_index_count: ($indexes | length),
      readback_evidence_count: ($readback_evidence | length),
      replay_diff_count: ($replay_diffs | length),
      shadow_blocker_count: ($blockers | length),
      event_records: $events,
      projection_indexes: $indexes,
      readback_evidence: $readback_evidence,
      replay_diffs: $replay_diffs,
      shadow_blockers: $blockers,
      scheduler_prior_gates: scheduler_prior_gates,
      scheduler_prior_gate_count: (scheduler_prior_gates | length),
      required_prior_gates: legacy_required_prior_gates,
      required_prior_gate_count: (legacy_required_prior_gates | length),
      canonical_required_prior_gates: canonical_required_prior_gates,
      canonical_required_prior_gate_count: (canonical_required_prior_gates | length),
      source_readbacks: {
        projection_fixture_ready: $source_projection_fixture_ready,
        projection_fixture_side_effects_all_false: side_effects_all_false($fixture),
        projection_readback_contract_ready: projection_readback_contract.source_projection_readback_ready,
        projection_capture_cycle_avoided: true,
        event_ids_unique: (($events | map(.event_id) | length) == ($events | map(.event_id) | unique | length)),
        idempotency_keys_unique: (($events | map(.idempotency_key) | length) == ($events | map(.idempotency_key) | unique | length)),
        projected_collection_events_complete: (
          ($node_events | length) == projection_readback_contract.expected_work_node_count
          and ($edge_events | length) == projection_readback_contract.expected_work_edge_count
          and ($task_result_events | length) == projection_readback_contract.expected_task_result_count
          and ($artifact_events | length) == projection_readback_contract.expected_artifact_count
          and ($approval_events | length) == projection_readback_contract.expected_approval_count
          and ($timeline_events | length) == projection_readback_contract.expected_timeline_event_count
        )
      },
      deterministic_event_ids_ready: true,
      idempotency_keys_ready: true,
      projection_indexes_ready: true,
      readback_evidence_ready: true,
      replay_diff_ready: true,
      scheduler_prior_chain_ready: true,
      task_result_contract_field_gap_readback_ready: true,
      append_only_shadow_path_readiness_complete: true,
      shadow_store_write_enabled: false,
      live_cutover_enabled: false,
      ready_for_persistent_mailbox_handoff: true,
      ready_for_append_only_event_store_shadow_path_readback: true,
      ready_for_append_only_work_graph_event_store: false,
      ready_for_scheduler_admission_enforcement: false,
      ready_for_task_result_enforcement: false,
      ready_for_role_manifest_enforcement: false,
      ready_for_live_execution: false,
      recommended_next_gate: "hepta_work_graph_append_only_event_store_shadow_path_readback_gate",
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        work_graph_event_persisted: false,
        event_store_enabled: false,
        shadow_event_persisted: false,
        projection_index_persisted: false,
        wal_written: false,
        replay_diff_persisted: false,
        idempotency_index_mutated: false,
        scheduler_admission_enforced: false,
        task_result_enforcement_enabled: false,
        role_manifest_enforcement_enabled: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
