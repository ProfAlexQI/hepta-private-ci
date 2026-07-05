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

contract_preview_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_contract_preview.rs
)"
contract_preview_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-contract-preview-report.sh
)"
contract_preview_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-contract-preview-gate.sh
)"
current_inventory_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_current_state_inventory.rs
)"
current_inventory_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-current-state-inventory-report.sh
)"
current_inventory_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-current-state-inventory-gate.sh
)"

jq -n \
  --argjson contract_preview_rust_module_present "$contract_preview_rust_module_present" \
  --argjson contract_preview_report_script_present "$contract_preview_report_script_present" \
  --argjson contract_preview_gate_script_present "$contract_preview_gate_script_present" \
  --argjson current_inventory_rust_module_present "$current_inventory_rust_module_present" \
  --argjson current_inventory_report_script_present "$current_inventory_report_script_present" \
  --argjson current_inventory_gate_script_present "$current_inventory_gate_script_present" \
  '
  def node_type($id; $purpose; $identity; $status; $evidence): {
    id: $id,
    purpose: $purpose,
    required_identity_fields: $identity,
    required_status_fields: $status,
    required_evidence_fields: $evidence
  };
  def edge_type($id; $from; $to; $invariant): {
    id: $id,
    from_node_kinds: $from,
    to_node_kinds: $to,
    invariant: $invariant
  };
  def invariant($id; $reason): {
    id: $id,
    required: true,
    reason: $reason
  };
  def adapter($source; $node_kind; $edge_kinds; $blockers): {
    source_surface_id: $source,
    preview_node_kind: $node_kind,
    preview_edge_kinds: $edge_kinds,
    live_mutation_enabled: false,
    blocker_ids: $blockers
  };
  [
    node_type("plan_step"; "durable projection of update_plan or Plan Mode planning intent"; ["node_id", "source_thread_id", "source_turn_id", "step_index"]; ["status", "owner_agent_path", "blocked_reason"]; ["trace_id", "source_event_id"]),
    node_type("agent_task"; "subagent or delegated thread task with targetable lifecycle status"; ["node_id", "agent_path", "thread_id", "parent_thread_id"]; ["status", "role_id", "budget_state"]; ["trace_id", "last_mailbox_sequence"]),
    node_type("worker_task"; "runtime worker or batch item that may produce artifacts and patches"; ["node_id", "task_id", "workspace_id"]; ["status", "lease_state", "attempt_count"]; ["trace_id", "artifact_ids", "command_run_ids"]),
    node_type("scheduler_run"; "scheduled job run and wake handoff with idempotency/readback metadata"; ["node_id", "job_id", "run_id"]; ["status", "timeout_state", "active_state"]; ["trace_id", "idempotency_key_hash", "readback_evidence_id"]),
    node_type("verification_gate"; "local static, unit, integration, or operator gate result"; ["node_id", "gate_id", "scope"]; ["status", "blocking", "rerun_required"]; ["trace_id", "report_hash", "log_excerpt_hash"]),
    node_type("artifact"; "file, patch, report, evidence bundle, or external handoff material"; ["node_id", "artifact_id", "artifact_kind"]; ["status", "retention_class", "redaction_state"]; ["trace_id", "content_hash", "producer_node_id"]),
    node_type("human_approval"; "operator approval, denial, supersession, or acknowledgement boundary"; ["node_id", "approval_id", "operator_scope"]; ["status", "authority_state", "expiry_state"]; ["trace_id", "request_hash", "decision_hash"]),
    node_type("external_handoff"; "queued or proposed external/channel/gateway action without live execution"; ["node_id", "handoff_id", "target_kind"]; ["status", "policy_state", "delivery_state"]; ["trace_id", "payload_preview_hash", "readback_evidence_id"])
  ] as $node_types
  | [
    edge_type("depends_on"; ["plan_step", "agent_task", "worker_task", "scheduler_run"]; ["plan_step", "agent_task", "worker_task", "verification_gate"]; "target cannot become runnable before all blocking dependencies are terminal-ready"),
    edge_type("spawned_by"; ["agent_task", "worker_task"]; ["plan_step", "agent_task", "scheduler_run"]; "child task must retain a parent trace and source authority"),
    edge_type("produces"; ["artifact"]; ["agent_task", "worker_task", "scheduler_run", "verification_gate"]; "artifact producer and content hash must be recorded before promotion"),
    edge_type("verifies"; ["verification_gate"]; ["plan_step", "agent_task", "worker_task", "artifact"]; "verification cannot promote a node without a report hash and trace id"),
    edge_type("blocks"; ["verification_gate", "human_approval", "external_handoff"]; ["plan_step", "agent_task", "worker_task", "scheduler_run"]; "blocked nodes require an explicit blocker id and unblock condition"),
    edge_type("retries"; ["agent_task", "worker_task", "scheduler_run", "external_handoff"]; ["agent_task", "worker_task", "scheduler_run", "external_handoff"]; "retry edges must preserve original idempotency and increment attempt evidence"),
    edge_type("replaces"; ["plan_step", "artifact", "human_approval", "external_handoff"]; ["plan_step", "artifact", "human_approval", "external_handoff"]; "replacement must supersede older nodes without deleting audit evidence")
  ] as $edge_types
  | [
    invariant("stable_node_identity_required"; "every projected node must have a deterministic node_id before it can be referenced"),
    invariant("source_surface_required"; "every node and edge must carry the source surface that produced it"),
    invariant("trace_id_required"; "plan, spawn, mailbox, tool, artifact, gate, and result evidence must be joinable"),
    invariant("task_result_not_optional_for_terminal_tasks"; "terminal agent, worker, and scheduler nodes must eventually point at a TaskResult"),
    invariant("admission_before_execution"; "execution adapters must not start until dependency, approval, lease, idempotency, and budget checks pass"),
    invariant("preview_gate_is_side_effect_free"; "this preview gate cannot write graph state, spawn agents, call models, or send externally")
  ] as $invariants
  | [
    adapter("update_plan_tool"; "plan_step"; ["depends_on", "blocks", "replaces"]; ["plan_step_identity_projection_missing"]),
    adapter("multi_agent_v2_thread_spawn"; "agent_task"; ["spawned_by", "depends_on", "retries"]; ["agent_task_lifecycle_fact_source_missing"]),
    adapter("agent_jobs_batch_workers"; "worker_task"; ["spawned_by", "produces", "retries"]; ["task_result_contract_not_enforced"]),
    adapter("hepta_runtime_task_board"; "worker_task"; ["depends_on", "blocks", "produces"]; ["task_board_work_graph_adapter_missing"]),
    adapter("hepta_runtime_scheduler_store"; "scheduler_run"; ["depends_on", "retries", "blocks"]; ["scheduler_admission_controller_not_enforced"]),
    adapter("hepta_runtime_agent_harness"; "external_handoff"; ["spawned_by", "produces", "blocks"]; ["agent_harness_work_graph_projection_missing"])
  ] as $adapter_previews
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_contract_preview_gate",
      schema_version: "work_graph_contract_preview_v1",
      preview_mode: "read_only_contract_preview_no_state_store",
      node_type_count: ($node_types | length),
      edge_type_count: ($edge_types | length),
      invariant_count: ($invariants | length),
      adapter_preview_count: ($adapter_previews | length),
      node_types: $node_types,
      edge_types: $edge_types,
      invariants: $invariants,
      adapter_previews: $adapter_previews,
      recommended_next_gate: "hepta_work_graph_task_result_contract_preview_gate",
      ready_for_task_result_contract_preview: true,
      ready_for_scheduler_admission_preview: false,
      ready_for_live_execution: false,
      source_probes: {
        contract_preview_contract: {
          rust_module_present: $contract_preview_rust_module_present,
          report_script_present: $contract_preview_report_script_present,
          gate_script_present: $contract_preview_gate_script_present
        },
        current_state_inventory_contract: {
          rust_module_present: $current_inventory_rust_module_present,
          report_script_present: $current_inventory_report_script_present,
          gate_script_present: $current_inventory_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        runtime_mutation_performed: false,
        scheduler_cutover_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
