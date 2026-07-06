#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

path_exists() {
  local path="$1"
  [[ -e "$path" ]]
}

bool_for() {
  if "$@"; then
    printf 'true\n'
  else
    printf 'false\n'
  fi
}

unified_state_store_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_unified_state_store.rs
)"
unified_state_store_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-state-store-preview-report.sh
)"
unified_state_store_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-state-store-preview-gate.sh
)"
role_manifest_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_role_manifest_contract.rs
)"
role_manifest_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-role-manifest-contract-preview-report.sh
)"
role_manifest_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-role-manifest-contract-preview-gate.sh
)"

jq -n \
  --argjson unified_state_store_rust_module_present "$unified_state_store_rust_module_present" \
  --argjson unified_state_store_report_script_present "$unified_state_store_report_script_present" \
  --argjson unified_state_store_gate_script_present "$unified_state_store_gate_script_present" \
  --argjson role_manifest_rust_module_present "$role_manifest_rust_module_present" \
  --argjson role_manifest_report_script_present "$role_manifest_report_script_present" \
  --argjson role_manifest_gate_script_present "$role_manifest_gate_script_present" \
  '
  def collection($id; $keys; $purpose): {
    id: $id,
    key_fields: $keys,
    purpose: $purpose
  };
  def index($id; $collection; $fields; $purpose): {
    id: $id,
    collection_id: $collection,
    fields: $fields,
    purpose: $purpose
  };
  def operation($id; $inputs; $purpose): {
    id: $id,
    mutates_store: false,
    required_inputs: $inputs,
    purpose: $purpose
  };
  def invariant($id; $reason): {
    id: $id,
    required: true,
    reason: $reason
  };
  def adapter($source; $collections; $gates; $blockers): {
    source_surface_id: $source,
    projected_collection_ids: $collections,
    required_contract_gates: $gates,
    persistence_enabled: false,
    blocker_ids: $blockers
  };
  [
    collection("nodes"; ["nodeId", "nodeKind"]; "PlanStep, AgentTask, WorkerTask, SchedulerRun, gate, artifact, approval, and handoff nodes"),
    collection("edges"; ["edgeId", "edgeKind", "fromNodeId", "toNodeId"]; "depends_on, spawned_by, produces, verifies, blocks, retries, and replaces edges"),
    collection("taskResults"; ["taskId", "traceId"]; "terminal and non-terminal TaskResult projections"),
    collection("artifacts"; ["artifactId", "producerNodeId"]; "redacted artifact metadata, hashes, and path hints"),
    collection("approvals"; ["approvalId", "operatorScope"]; "operator decisions, expiry, supersession, and authority boundaries"),
    collection("timelineEvents"; ["traceId", "eventKind", "nodeId"]; "redacted trace timeline events for local audit views")
  ] as $collections
  | [
    index("byTraceId"; "nodes"; ["traceId"]; "join every node in one work trace"),
    index("bySourceSurface"; "nodes"; ["sourceSurfaceId"]; "audit which source surface produced a node"),
    index("byStatus"; "nodes"; ["status"]; "find blocked, runnable, terminal, or superseded work"),
    index("byEdgeKind"; "edges"; ["edgeKind"]; "query dependency, spawn, evidence, retry, and replacement paths"),
    index("byTaskResultStatus"; "taskResults"; ["status"]; "find terminal results that need promotion review"),
    index("byTimelineTrace"; "timelineEvents"; ["traceId", "eventKind"]; "render ordered redacted timeline views")
  ] as $indexes
  | [
    operation("preview_project_node"; ["sourceSurfaceId", "nodeKind", "traceId"]; "explain how a source record would become a WorkGraph node"),
    operation("preview_project_edge"; ["edgeKind", "fromNodeId", "toNodeId", "traceId"]; "explain how a relationship would be represented without writing it"),
    operation("preview_validate_task_result"; ["taskId", "status", "traceId"]; "check a TaskResult against the preview schema"),
    operation("preview_explain_admission"; ["nodeId", "dependencyStatus", "budgetState"]; "explain scheduler admission allow or deny decisions"),
    operation("preview_render_timeline"; ["traceId", "redactionState"]; "render a local timeline view from projected events"),
    operation("preview_role_manifest_projection"; ["roleId", "capabilities", "toolPermissions"]; "explain role capability and permission projection")
  ] as $operations
  | [
    invariant("deterministic_identity_required"; "node, edge, result, artifact, approval, and event ids must be deterministic"),
    invariant("append_only_evidence_required"; "promotion cannot delete or rewrite prior evidence"),
    invariant("redacted_payload_only"; "store previews carry hashes and references, not raw secrets or private payloads"),
    invariant("idempotent_projection_required"; "re-running a projection preview must produce the same ids and decisions"),
    invariant("readback_before_promotion"; "terminal state promotion needs readback, verifier, or gate evidence"),
    invariant("preview_store_does_not_persist"; "this gate cannot write graph state or enable adapter enforcement")
  ] as $invariants
  | [
    "hepta_work_graph_contract_preview_gate",
    "hepta_work_graph_task_result_contract_preview_gate",
    "hepta_work_graph_scheduler_admission_controller_preview_gate",
    "hepta_work_graph_observability_timeline_preview_gate",
    "hepta_work_graph_role_manifest_contract_preview_gate"
  ] as $required_gates
  | [
    adapter("update_plan_tool"; ["nodes", "edges", "timelineEvents"]; $required_gates; ["plan_step_store_projection_not_enforced"]),
    adapter("multi_agent_v2_thread_spawn"; ["nodes", "edges", "timelineEvents"]; $required_gates; ["agent_task_store_projection_not_enforced"]),
    adapter("agent_jobs_batch_workers"; ["nodes", "taskResults", "timelineEvents"]; $required_gates; ["agent_job_store_projection_not_enforced"]),
    adapter("hepta_runtime_worker_tasks"; ["nodes", "taskResults", "artifacts", "timelineEvents"]; $required_gates; ["worker_task_store_projection_not_enforced"]),
    adapter("hepta_runtime_scheduler_store"; ["nodes", "edges", "timelineEvents"]; $required_gates; ["scheduler_store_projection_not_enforced"]),
    adapter("hepta_runtime_agent_harness"; ["nodes", "artifacts", "timelineEvents"]; $required_gates; ["agent_harness_store_projection_not_enforced"])
  ] as $adapter_previews
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_unified_state_store_preview_gate",
      schema_version: "work_graph_unified_state_store_preview_v1",
      preview_mode: "read_only_store_shape_preview_no_persistence",
      collection_count: ($collections | length),
      index_count: ($indexes | length),
      operation_count: ($operations | length),
      invariant_count: ($invariants | length),
      adapter_preview_count: ($adapter_previews | length),
      collections: $collections,
      indexes: $indexes,
      operations: $operations,
      invariants: $invariants,
      adapter_previews: $adapter_previews,
      recommended_next_gate: "hepta_work_graph_adapter_projection_fixture_gate",
      ready_for_adapter_projection_fixtures: true,
      ready_for_store_persistence: false,
      ready_for_live_execution: false,
      source_probes: {
        unified_state_store: {
          rust_module_present: $unified_state_store_rust_module_present,
          report_script_present: $unified_state_store_report_script_present,
          gate_script_present: $unified_state_store_gate_script_present
        },
        role_manifest_contract: {
          rust_module_present: $role_manifest_rust_module_present,
          report_script_present: $role_manifest_report_script_present,
          gate_script_present: $role_manifest_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        store_persistence_enabled: false,
        runtime_mutation_performed: false,
        scheduler_cutover_performed: false,
        adapter_projection_enforced: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
