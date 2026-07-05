#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-persistence-enforcement-rollout-blocker-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-persistence-enforcement-rollout-blocker-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  def durable_fields: [
    "workflow_id",
    "run_id",
    "step_id",
    "checkpoint",
    "replay_key",
    "rollback_anchor",
    "receipt_hash"
  ];
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_persistence_enforcement_rollout_blocker_preview_gate"
  and .schema_version == "work_graph_persistence_enforcement_rollout_blocker_preview_v1"
  and .preview_mode == "read_only_persistence_enforcement_rollout_blocker_preview_no_rollout"
  and .rollout_stage_count == 6
  and (.rollout_stages | length) == .rollout_stage_count
  and (.rollout_stages | map(.id) == [
    "store_persistence_enforcement_rollout",
    "wal_append_enforcement_rollout",
    "checkpoint_write_enforcement_rollout",
    "readback_receipt_enforcement_rollout",
    "replay_execution_enforcement_rollout",
    "external_publication_enforcement_rollout"
  ])
  and (.rollout_stages | all(
    .max_traffic_ppm == 0
    and .enforcement_enabled == false
    and .blocks_release == true
    and (.required_evidence_fields | length) >= 11
    and ((durable_fields - .required_evidence_fields) == [])
  ))
  and .traffic_ramp_blocker_count == 7
  and (.traffic_ramp_blockers | length) == .traffic_ramp_blocker_count
  and (.traffic_ramp_blockers | map(.id) == [
    "ramp_blocked_without_operator_packet",
    "ramp_blocked_without_durable_identity",
    "ramp_blocked_without_shadow_live_match",
    "ramp_blocked_without_kill_switch",
    "ramp_blocked_without_rollback_owner",
    "ramp_blocked_without_release_denial_matrix",
    "ramp_blocked_for_external_publication"
  ])
  and (.traffic_ramp_blockers | all(
    .max_allowed_traffic_ppm == 0
    and .blocks_ramp == true
    and (.applies_to_stage_ids | length) >= 1
  ))
  and .kill_switch_count == 5
  and (.kill_switches | length) == .kill_switch_count
  and (.kill_switches | map(.id) == [
    "kill_store_persistence_rollout",
    "kill_wal_checkpoint_rollout",
    "kill_readback_receipt_rollout",
    "kill_replay_execution_rollout",
    "kill_external_publication_rollout"
  ])
  and (.kill_switches | all(.armed_in_preview == true and (.target_stage_ids | length) >= 1))
  and .operator_enablement_count == 6
  and (.operator_enablements | length) == .operator_enablement_count
  and (.operator_enablements | map(.id) == [
    "operator_enable_store_persistence_rollout",
    "operator_enable_wal_checkpoint_rollout",
    "operator_enable_readback_receipt_rollout",
    "operator_enable_replay_execution_rollout",
    "operator_enable_external_publication_rollout",
    "operator_enable_full_rollout_abort_packet"
  ])
  and (.operator_enablements | all(
    .currently_satisfied == false
    and .approval_recorded == false
    and (.required_fields | length) >= 11
    and (.required_fields | index("workflow_id") != null)
    and (.required_fields | index("receipt_hash") != null)
  ))
  and .rollback_owner_count == 5
  and (.rollback_owners | length) == .rollback_owner_count
  and (.rollback_owners | map(.id) == [
    "rollback_owner_store_persistence",
    "rollback_owner_wal_checkpoint",
    "rollback_owner_receipts",
    "rollback_owner_replay",
    "rollback_owner_external_publication"
  ])
  and (.rollback_owners | all(
    .currently_satisfied == false
    and (.required_receipt_fields | length) >= 11
    and (.required_receipt_fields | index("workflow_id") != null)
    and (.required_receipt_fields | index("receipt_hash") != null)
    and (.owns_stage_ids | length) >= 1
  ))
  and .release_denial_count == 6
  and (.release_denials | length) == .release_denial_count
  and (.release_denials | map(.id) == [
    "deny_store_persistence_enforcement_release",
    "deny_wal_checkpoint_enforcement_release",
    "deny_readback_receipt_enforcement_release",
    "deny_replay_execution_enforcement_release",
    "deny_external_publication_enforcement_release",
    "deny_full_rollout_public_claim"
  ])
  and (.release_denials | all(
    .blocks_release == true
    and .blocks_publication == true
    and (.required_clearance_ids | length) >= 4
    and (.required_clearance_ids | index("ramp_blocked_without_durable_identity") != null)
  ))
  and .durable_identity_evidence.schema_version == "work_graph_durable_identity_preview_v1"
  and .durable_identity_evidence.required_prior_gate == "hepta_work_graph_durable_identity_preview_gate"
  and .durable_identity_evidence.required_field_ids == durable_fields
  and .durable_identity_evidence.required_for_rollout_stage_ids == [
    "store_persistence_enforcement_rollout",
    "wal_append_enforcement_rollout",
    "checkpoint_write_enforcement_rollout",
    "readback_receipt_enforcement_rollout",
    "replay_execution_enforcement_rollout",
    "external_publication_enforcement_rollout"
  ]
  and .durable_identity_evidence.durable_field_count == 7
  and .durable_identity_evidence.preview_binding_count >= 5
  and .durable_identity_evidence.invariant_count >= 7
  and .durable_identity_evidence.currently_satisfied == false
  and .invariant_count == 7
  and (.invariants | length) == .invariant_count
  and (.invariants | map(.id) == [
    "enforcement_rollout_requires_durable_identity_evidence",
    "enforcement_rollout_is_blocked_by_default",
    "traffic_ramp_requires_operator_packet_and_shadow_live_match",
    "kill_switches_precede_any_rollout_stage",
    "rollback_owners_are_explicit_and_unsatisfied",
    "release_and_publication_denied_independently",
    "enforcement_rollout_blocker_preview_has_no_side_effects"
  ])
  and (.invariants | all(.required == true))
  and (.required_prior_gates == [
    "hepta_work_graph_contract_preview_gate",
    "hepta_work_graph_task_result_contract_preview_gate",
    "hepta_work_graph_scheduler_admission_controller_preview_gate",
    "hepta_work_graph_observability_timeline_preview_gate",
    "hepta_work_graph_role_manifest_contract_preview_gate",
    "hepta_work_graph_unified_state_store_preview_gate",
    "hepta_work_graph_adapter_projection_fixture_gate",
    "hepta_work_graph_state_store_persistence_preview_gate",
    "hepta_work_graph_replay_readback_preview_gate",
    "hepta_work_graph_promotion_precondition_preview_gate",
    "hepta_work_graph_activation_enforcement_blocker_preview_gate",
    "hepta_work_graph_shadow_adapter_readback_preview_gate",
    "hepta_work_graph_persistence_feature_flag_preview_gate",
    "hepta_work_graph_persistence_canary_dry_run_preview_gate",
    "hepta_work_graph_persistence_canary_readback_receipt_preview_gate",
    "hepta_work_graph_persistence_promotion_blocker_preview_gate",
    "hepta_work_graph_persistence_shadow_live_readback_comparison_preview_gate",
    "hepta_work_graph_durable_identity_preview_gate"
  ])
  and .recommended_next_gate == "hepta_work_graph_persistence_operator_readiness_packet_preview_gate"
  and .ready_for_operator_readiness_packet_preview == true
  and .ready_for_enforcement_rollout == false
  and .ready_for_live_persistence == false
  and .source_probes.persistence_enforcement_rollout_blocker.rust_module_present == true
  and .source_probes.persistence_enforcement_rollout_blocker.report_script_present == true
  and .source_probes.persistence_enforcement_rollout_blocker.gate_script_present == true
  and .source_probes.persistence_shadow_live_readback_comparison.rust_module_present == true
  and .source_probes.persistence_shadow_live_readback_comparison.gate_script_present == true
  and .source_probes.durable_identity.rust_module_present == true
  and .source_probes.durable_identity.report_script_present == true
  and .source_probes.durable_identity.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_persistence_enforcement_rollout_blocker_preview --lib

echo "Hepta WorkGraph persistence enforcement rollout blocker preview gate passed"
