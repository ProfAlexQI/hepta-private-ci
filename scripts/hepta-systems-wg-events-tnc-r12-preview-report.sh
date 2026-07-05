#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

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

upstream_report="$(
  capture_json_report \
    "hepta-wg-upe-readiness-cutover-terminal-no-cutover-receipt-ack-replay-idempotency-closeout-receipt-ack-replay-idempotency-closeout-receipt-receipt-receipt-receipt-receipt-receipt-receipt-receipt-receipt-receipt-receipt-rerun-preview-report" \
    "$ROOT/scripts/hepta-systems-wg-upe-readiness-cutover-terminal-no-cutover-receipt-ack-replay-idempotency-closeout-receipt-ack-replay-idempotency-closeout-receipt-receipt-receipt-receipt-receipt-receipt-receipt-receipt-receipt-receipt-receipt-rerun-preview-report.sh"
)"

preview_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/wg_events_tnc_r12_preview.rs
)"
preview_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-wg-events-tnc-r12-preview-report.sh
)"
preview_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-wg-events-tnc-r12-preview-gate.sh
)"
upstream_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_upe_readiness_cutover_terminal_no_cutover_receipt_ack_replay_idempotency_closeout_receipt_ack_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_preview.rs
)"
upstream_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-wg-upe-readiness-cutover-terminal-no-cutover-receipt-ack-replay-idempotency-closeout-receipt-ack-replay-idempotency-closeout-receipt-receipt-receipt-receipt-receipt-receipt-receipt-receipt-receipt-receipt-receipt-rerun-preview-gate.sh
)"

jq -n \
  --argjson upstream "$upstream_report" \
  --argjson preview_rust_module_present "$preview_rust_module_present" \
  --argjson preview_report_script_present "$preview_report_script_present" \
  --argjson preview_gate_script_present "$preview_gate_script_present" \
  --argjson upstream_rust_module_present "$upstream_rust_module_present" \
  --argjson upstream_gate_script_present "$upstream_gate_script_present" \
  '
  def unique_order: reduce .[] as $item ([]; if index($item) then . else . + [$item] end);
  def stage_ids: [
    "work_graph_events_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout",
    "work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_non_recording",
    "work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_non_send_boundary",
    "work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_git_mutation_boundary",
    "work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_no_enablement_regression_guard",
    "work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_terminal_frontier_mapping"
  ];
  def evidence_fields: [
    "source_surface_id",
    "source_category",
    "event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_decision_ref",
    "terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_id",
    "non_recording_replay_idempotency_confirmation_id",
    "non_send_replay_idempotency_boundary_id",
    "git_mutation_replay_idempotency_boundary_id",
    "no_enablement_regression_replay_idempotency_id",
    "residual_source_blocker_ids",
    "next_required_gate"
  ];
  def plan_id($source):
    $source + "_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt";
  def sources_for($blocker_id):
    [$upstream.decision_deltas[]
      | select(.residual_source_blocker_ids | index($blocker_id))
      | .source_surface_id] | unique_order;
  def stage($id; $priority; $category; $sources; $contracts): {
      id: $id,
      priority: $priority,
      category: $category,
      affected_source_surface_ids: $sources,
      required_contract_ref_ids: $contracts,
      expected_runtime_state: "preview_only_no_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt",
      prerequisite_gate_ids: ["hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_preview_gate"],
      contract_ready_preview: true,
      persists_work_graph_events_after_preview: false,
      enables_event_store_after_preview: false,
      writes_wal_after_preview: false,
      writes_checkpoint_after_preview: false,
      executes_replay_after_preview: false,
      executes_readback_after_preview: false,
      enforces_adapter_projection_after_preview: false,
      mutates_runtime_after_preview: false
    };
  def guard($id; $severity; $scope): {
      id: $id,
      severity: $severity,
      guard_scope: $scope,
      required_before_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt: true,
      satisfied_by_preview: false
    };
  def blocker($id; $severity; $category; $sources; $stages; $plans; $fix): {
      id: $id,
      severity: $severity,
      category: $category,
      affected_source_surface_ids: $sources,
      affected_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_stage_ids: $stages,
      affected_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_plan_ids: $plans,
      required_before_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt: true,
      recommended_fix: $fix
    };
  ($upstream.decision_deltas | map(select(.work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_enforcement_decision == "allow_preview_only"))) as $source_decisions
  | ($source_decisions | map(.source_surface_id)) as $all_sources
  | ($source_decisions | map({
      source_surface_id: .source_surface_id,
      source_category: .source_category,
      event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_plan_id: plan_id(.source_surface_id),
      previous_enforcement_decision: .work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_enforcement_decision,
      event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_state: "work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_ready_preview",
      required_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_stage_ids: stage_ids,
      expected_evidence_field_ids: evidence_fields,
      residual_source_blocker_ids: .residual_source_blocker_ids,
      event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview: true,
      non_recording_replay_idempotency_ready_preview: true,
      non_send_replay_idempotency_boundary_ready_preview: true,
      git_mutation_replay_idempotency_boundary_ready_preview: true,
      terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_frontier_ready_preview: true,
      applies_to_runtime: false,
      persists_work_graph_events: false,
      enables_event_store: false,
      writes_wal: false,
      writes_checkpoint: false,
      executes_replay: false,
      executes_readback: false,
      enforces_adapter_projection: false,
      mutates_runtime: false
    })) as $plans
  | ($plans | map(.event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_plan_id)) as $plan_ids
  | [
      stage("work_graph_events_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout"; "critical"; "event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt"; $all_sources; ["terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_contract_ready","terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_rerun_ready","residual_blocker_zero_contract_ready","terminal_no_cutover_runtime_boundary_ready","terminal_acknowledgement_prerequisite_ready"]),
      stage("work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_non_recording"; "critical"; "non_recording_replay_idempotency_confirmation"; $all_sources; ["non_recording_replay_idempotency_confirmed","approval_recording_disabled_acknowledgement_ready","no_cutover_authorization_recorded_acknowledgement_ready","operator_review_not_recorded_acknowledgement_ready","side_effect_report_all_false_ready"]),
      stage("work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_non_send_boundary"; "critical"; "non_send_replay_idempotency_boundary"; $all_sources; ["non_send_replay_idempotency_boundary_ready","external_send_disabled_acknowledgement_ready","model_invocation_disabled_acknowledgement_ready","agent_spawn_disabled_acknowledgement_ready","terminal_delivery_disabled_acknowledgement_ready"]),
      stage("work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_git_mutation_boundary"; "critical"; "git_mutation_replay_idempotency_boundary"; $all_sources; ["git_mutation_replay_idempotency_boundary_ready","git_add_commit_push_disabled_acknowledgement_ready","event_store_activation_disabled_acknowledgement_ready","event_store_promotion_disabled_acknowledgement_ready","durable_store_switch_disabled_acknowledgement_ready","append_only_store_enablement_disabled_acknowledgement_ready"]),
      stage("work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_no_enablement_regression_guard"; "critical"; "no_enablement_regression_guard"; $all_sources; ["no_enablement_regression_replay_idempotency_ready","work_graph_events_append_disabled_acknowledgement_ready","wal_checkpoint_no_write_acknowledgement_ready","timeline_append_noop_acknowledgement_ready","graph_state_persistence_disabled_acknowledgement_ready"]),
      stage("work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_terminal_frontier_mapping"; "high"; "terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_frontier_mapping"; $all_sources; ["residual_blocker_zero_mapping_ready","terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_frontier_mapping_ready","terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_preview_frontier_ready","no_enablement_regression_mapping_ready","terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_blocker_mapping_ready"])
    ] as $stages
  | [
      guard("work_graph_events_persistence_disabled"; "critical"; "event_store"),
      guard("event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_disabled"; "critical"; "event_store"),
      guard("wal_write_disabled"; "critical"; "wal"),
      guard("checkpoint_write_disabled"; "critical"; "checkpoint"),
      guard("replay_execution_disabled"; "critical"; "replay"),
      guard("readback_execution_disabled"; "critical"; "readback"),
      guard("adapter_projection_enforcement_disabled"; "critical"; "adapter_projection"),
      guard("git_mutation_disabled"; "critical"; "git"),
      guard("approval_recording_disabled"; "high"; "operator_review"),
      guard("side_effect_lock_not_established"; "critical"; "side_effect_lock"),
      guard("no_agent_spawn_or_external_effect"; "high"; "external_effects")
    ] as $guards
  | [
      blocker("append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_readback_missing"; "medium"; "readback_preview"; $all_sources; stage_ids; $plan_ids; "run terminal no-cutover receipt acknowledgement replay idempotency closeout readback before applying terminal no-enable outcomes")
    ] as $blockers
  | ($upstream.required_prior_gates + [$upstream.gate]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "blocked",
      gate: "hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_preview_gate",
      schema_version: "work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_preview_v1",
      preview_mode: "read_only_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_preview_no_persistence",
      upstream_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_gate: "hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_preview_gate",
      source_surface_count: ($source_decisions | length),
      event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_plan_count: ($plans | length),
      event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_stage_count: ($stages | length),
      event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_stage_source_ref_count: ($stages | map(.affected_source_surface_ids | length) | add),
      event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_stage_contract_ref_count: ($stages | map(.required_contract_ref_ids | length) | add),
      event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_plan_stage_ref_count: ($plans | map(.required_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_stage_ids | length) | add),
      event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_plan_evidence_field_ref_count: ($plans | map(.expected_evidence_field_ids | length) | add),
      append_only_work_graph_events_primary_blocked_source_count: (sources_for("append_only_work_graph_events_disabled") | length),
      replay_readback_execution_blocked_source_count: (sources_for("replay_readback_execution_disabled") | length),
      runtime_adapter_enforcement_blocked_source_count: (sources_for("runtime_canonical_adapter_enforcement_disabled") | length),
      guard_count: ($guards | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_plans: $plans,
      event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_stage_plans: $stages,
      guards: $guards,
      blockers: $blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_readback_preview_gate",
      ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_readback_preview: true,
      ready_for_append_only_work_graph_events: false,
      ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt: false,
      ready_for_replay_readback_execution: false,
      ready_for_runtime_adapter_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_preview: {
          rust_module_present: $preview_rust_module_present,
          report_script_present: $preview_report_script_present,
          gate_script_present: $preview_gate_script_present
        },
        work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun: {
          rust_module_present: $upstream_rust_module_present,
          gate_script_present: $upstream_gate_script_present,
          upstream_gate: ($upstream.gate == "hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_preview_gate")
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        work_graph_events_persisted: false,
        event_store_enabled: false,
        wal_written: false,
        checkpoint_written: false,
        replay_executed: false,
        readback_executed: false,
        adapter_projection_enforced: false,
        runtime_mutation_performed: false,
        approval_recorded: false,
        terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_established: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
