#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
APPEND_INTAKE_REPORT="$ROOT/scripts/hepta-systems-work-graph-append-only-event-intake-preview-report.sh"
RUST_ADAPTER="$ROOT/codex-rs/hepta-runtime/src/workflow_durable_store_adapter.rs"
RUST_APPEND_PLAN="$ROOT/codex-rs/hepta-runtime/src/workflow_durable_store_append_plan.rs"
RUST_HARNESS="$ROOT/codex-rs/hepta-runtime/src/workflow_durable_store_adapter_harness.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_WORKFLOW_DURABLE_STORE_ADAPTER_2026-06-27.md"

fail() {
  printf 'hepta-systems-workflow-durable-store-adapter-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$APPEND_INTAKE_REPORT" ]] || fail "missing executable append-only intake report: $APPEND_INTAKE_REPORT"
[[ -f "$RUST_ADAPTER" ]] || fail "missing workflow durable store adapter source: $RUST_ADAPTER"
[[ -f "$RUST_APPEND_PLAN" ]] || fail "missing workflow durable store append plan source: $RUST_APPEND_PLAN"
[[ -f "$RUST_HARNESS" ]] || fail "missing workflow durable store adapter harness source: $RUST_HARNESS"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing workflow durable store adapter architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the workflow durable store adapter report"
fi

lib_export_present=false
if grep -q 'hepta_workflow_durable_store_adapter_report' "$LIB_SOURCE" \
  && grep -q 'hepta_workflow_durable_store_append_plan_report' "$LIB_SOURCE" \
  && grep -q 'hepta_workflow_durable_store_adapter_harness_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile intake <("$APPEND_INTAKE_REPORT") \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-workflow-durable-store-adapter-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_WORKFLOW_DURABLE_STORE_ADAPTER_2026-06-27.md" \
  '
  def lease_scope($kind):
    if $kind == "plan_step_event" then "workflow_run_plan_projection_lease"
    elif $kind == "agent_spawn_event" then "workflow_run_agent_spawn_lease"
    elif $kind == "mailbox_delivery_event" then "workflow_run_mailbox_delivery_lease"
    elif $kind == "agent_job_item_event" then "workflow_run_agent_job_item_lease"
    elif $kind == "worker_task_event" then "workflow_run_worker_task_lease"
    elif $kind == "scheduler_run_event" then "workflow_run_scheduler_lease"
    elif $kind == "artifact_event" then "workflow_run_artifact_lease"
    elif $kind == "approval_event" then "workflow_run_approval_projection_lease"
    elif $kind == "task_result_event" then "workflow_run_task_result_lease"
    else "workflow_run_unknown_projection_lease" end;
  def rollback_anchor($kind):
    if $kind == "plan_step_event" then "rollback_to_prior_plan_projection_checkpoint"
    elif $kind == "agent_spawn_event" then "rollback_to_parent_thread_spawn_anchor"
    elif $kind == "mailbox_delivery_event" then "rollback_to_prior_mailbox_sequence_anchor"
    elif $kind == "agent_job_item_event" then "rollback_to_prior_job_item_attempt_anchor"
    elif $kind == "worker_task_event" then "rollback_to_prior_worker_task_attempt_anchor"
    elif $kind == "scheduler_run_event" then "rollback_to_prior_scheduler_lease_anchor"
    elif $kind == "artifact_event" then "rollback_to_prior_artifact_hash_anchor"
    elif $kind == "approval_event" then "rollback_to_prior_approval_scope_anchor"
    elif $kind == "task_result_event" then "rollback_to_prior_task_result_anchor"
    else "rollback_to_prior_unknown_event_anchor" end;
  def adapter_entry($contract): {
    event_contract_id:$contract.id,
    record_kind:$contract.record_kind,
    target_collection_ids:$contract.target_collection_ids,
    required_fields:$contract.required_fields,
    idempotency_key_fields:$contract.idempotency_key_fields,
    adapter_route:"temporal_lite_plan_ready_behind_feature_gate",
    lease_scope:lease_scope($contract.record_kind),
    checkpoint_policy:"checkpoint_metadata_only_no_checkpoint_write",
    replay_validation_policy:"deterministic_replay_validation_metadata_only",
    rollback_anchor:rollback_anchor($contract.record_kind),
    append_policy:"append_plan_only_feature_gate_required",
    feature_gate_required:true,
    feature_gate_enabled:false,
    append_suppressed_by_feature_gate:true,
    noop_receipt_projected:true,
    event_log_write_enabled:false,
    sqlite_write_enabled:false,
    checkpoint_write_enabled:false,
    workflow_execution_enabled:false,
    replay_execution_enabled:false,
    rollback_execution_enabled:false,
    live_execution_enabled:false
  };
  ($intake[0]) as $intake |
  ($intake.event_contracts | map(adapter_entry(.))) as $entries |
  ($intake.status == "ready"
    and $intake.event_contract_count == 9
    and $intake.ready_for_replay_readback_preview == true
    and $intake.ready_for_append_only_store_enablement == false
    and $intake.ready_for_store_persistence == false
    and $intake.ready_for_live_execution == false
    and $lib_export_present
    and ($entries | length) == 9
    and ($entries | all(.feature_gate_required == true and .feature_gate_enabled == false and .append_suppressed_by_feature_gate == true and .noop_receipt_projected == true))
    and ($entries | all(.event_log_write_enabled == false and .sqlite_write_enabled == false and .checkpoint_write_enabled == false and .workflow_execution_enabled == false and .replay_execution_enabled == false and .rollback_execution_enabled == false and .live_execution_enabled == false))) as $adapter_ready |
  {
    runtime:"hepta",
    surface:"workflow_durable_store_adapter",
    status:(if $adapter_ready then "ready" else "blocked" end),
    gate:"hepta_workflow_durable_store_adapter_gate",
    schema_version:"workflow_durable_store_adapter_v1",
    source_append_only_event_intake_surface:$intake.gate,
    source_append_only_event_intake_ready:($intake.status == "ready"),
    source_append_only_event_contract_count:$intake.event_contract_count,
    source_append_plan_surface:"workflow_durable_store_append_plan",
    source_append_plan_ready:$adapter_ready,
    source_adapter_harness_surface:"workflow_durable_store_adapter_harness",
    source_adapter_harness_ready:$adapter_ready,
    lib_export_present:$lib_export_present,
    event_contract_count:$intake.event_contract_count,
    append_plan_count:($entries | length),
    lease_metadata_count:($entries | length),
    idempotency_metadata_count:($entries | length),
    checkpoint_metadata_count:($entries | length),
    replay_validation_count:($entries | length),
    rollback_metadata_count:($entries | length),
    noop_receipt_count:($entries | map(select(.noop_receipt_projected == true)) | length),
    adapter_entry_count:($entries | length),
    feature_gate_required:true,
    feature_gate_enabled:false,
    adapter_contract_ready:$adapter_ready,
    temporal_lite_adapter_ready:$adapter_ready,
    ready_for_event_log_write:false,
    ready_for_sqlite_write:false,
    ready_for_workflow_execution:false,
    ready_for_replay_execution:false,
    ready_for_rollback_execution:false,
    ready_for_live_execution:false,
    entries:$entries,
    blockers:[
      "workflow_durable_store_feature_gate_disabled",
      "workflow_event_log_write_disabled",
      "sqlite_write_disabled",
      "workflow_execution_disabled",
      "replay_execution_disabled",
      "rollback_execution_disabled",
      "live_execution_disabled"
    ],
    next_actions:[
      "phase4_thread_thin_hepta_system_status_e2e_read_only_chain",
      "keep_event_log_sqlite_replay_rollback_and_live_execution_disabled_until_explicit_cutover"
    ],
    next_migration_step:"phase4_thread_thin_hepta_system_status_e2e_read_only_chain",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      adapter:"codex-rs/hepta-runtime/src/workflow_durable_store_adapter.rs",
      append_plan:"codex-rs/hepta-runtime/src/workflow_durable_store_append_plan.rs",
      harness:"codex-rs/hepta-runtime/src/workflow_durable_store_adapter_harness.rs",
      append_only_intake_report:"scripts/hepta-systems-work-graph-append-only-event-intake-preview-report.sh"
    },
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      filesystem_written:false,
      workflow_event_log_mutated:false,
      event_log_written:false,
      sqlite_written:false,
      lease_acquired:false,
      idempotency_index_mutated:false,
      checkpoint_written:false,
      readback_performed:false,
      workflow_execution_started:false,
      replay_executed:false,
      rollback_executed:false,
      provider_invoked:false,
      model_invoked:false,
      gateway_or_auth_mutated:false,
      native_post_mutation_performed:false,
      channel_send_performed:false,
      package_or_release_written:false,
      public_ga_promoted:false
    }
  }'
