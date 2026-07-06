#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-persistence-shadow-live-readback-comparison-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-persistence-shadow-live-readback-comparison-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_persistence_shadow_live_readback_comparison_preview_gate"
  and .schema_version == "work_graph_persistence_shadow_live_readback_comparison_preview_v1"
  and .preview_mode == "read_only_persistence_shadow_live_readback_comparison_preview_no_live_read"
  and .comparison_surface_count == 6
  and (.comparison_surfaces | length) == .comparison_surface_count
  and (.comparison_surfaces | map(.id) == [
    "work_graph_node_collection_comparison",
    "work_graph_edge_collection_comparison",
    "work_graph_task_result_collection_comparison",
    "work_graph_artifact_collection_comparison",
    "work_graph_approval_collection_comparison",
    "work_graph_timeline_collection_comparison"
  ])
  and (.comparison_surfaces | all(
    .comparison_mode == "hash_only_shadow_to_future_live_digest"
    and .live_read_enabled == false
    and (.shadow_probe_id | length) > 0
    and (.future_live_probe_id | length) > 0
  ))
  and .readback_pair_count == 6
  and (.readback_pairs | length) == .readback_pair_count
  and (.readback_pairs | map(.id) == [
    "compare_node_shadow_to_future_live_digest",
    "compare_edge_shadow_to_future_live_digest",
    "compare_task_result_shadow_to_future_live_digest",
    "compare_artifact_shadow_to_future_live_digest",
    "compare_approval_shadow_to_future_live_digest",
    "compare_timeline_shadow_to_future_live_digest"
  ])
  and (.readback_pairs | all(
    .tolerance == "exact_digest_match_required"
    and .blocks_promotion_on_mismatch == true
    and (.required_digest_fields | length) >= 4
    and (.required_digest_fields | index("workflow_id"))
    and (.required_digest_fields | index("run_id"))
    and (.required_digest_fields | index("step_id"))
    and (.required_digest_fields | index("checkpoint"))
    and (.required_digest_fields | index("replay_key"))
    and (.required_digest_fields | index("rollback_anchor"))
    and (.required_digest_fields | index("receipt_hash"))
  ))
  and .mismatch_classifier_count == 7
  and (.mismatch_classifiers | length) == .mismatch_classifier_count
  and (.mismatch_classifiers | map(.id) == [
    "missing_shadow_digest",
    "durable_identity_digest_missing",
    "future_live_probe_not_authorized",
    "schema_version_drift",
    "collection_count_drift",
    "redaction_state_drift",
    "operator_scope_drift"
  ])
  and (.mismatch_classifiers | all(.quarantine_required == true and (.applies_to_surface_ids | length) >= 1))
  and (.mismatch_classifiers | map(select(.severity == "critical")) | length) == 5
  and .promotion_denial_count == 7
  and (.promotion_denials | length) == .promotion_denial_count
  and (.promotion_denials | map(.id) == [
    "deny_missing_shadow_digest",
    "deny_durable_identity_digest_missing",
    "deny_future_live_probe_without_authorization",
    "deny_schema_version_drift",
    "deny_collection_count_drift",
    "deny_redaction_state_drift",
    "deny_operator_scope_drift"
  ])
  and (.promotion_denials | all(.blocks_promotion == true and (.applies_to_classifier_ids | length) >= 1))
  and .operator_view_count == 4
  and (.operator_views | length) == .operator_view_count
  and (.operator_views | map(.id) == [
    "operator_shadow_live_comparison_summary",
    "auditor_mismatch_classifier_view",
    "rollback_quarantine_preview_view",
    "enforcement_rollout_blocker_view"
  ])
  and (.operator_views | all(
    .external_delivery_enabled == false
    and (.required_fields | length) >= 4
    and (.required_fields | index("workflow_id"))
    and (.required_fields | index("receipt_hash"))
  ))
  and .durable_identity_evidence.schema_version == "work_graph_durable_identity_preview_v1"
  and .durable_identity_evidence.required_prior_gate == "hepta_work_graph_durable_identity_preview_gate"
  and .durable_identity_evidence.required_field_ids == [
    "workflow_id",
    "run_id",
    "step_id",
    "checkpoint",
    "replay_key",
    "rollback_anchor",
    "receipt_hash"
  ]
  and .durable_identity_evidence.required_for_readback_pair_ids == [
    "compare_node_shadow_to_future_live_digest",
    "compare_edge_shadow_to_future_live_digest",
    "compare_task_result_shadow_to_future_live_digest",
    "compare_artifact_shadow_to_future_live_digest",
    "compare_approval_shadow_to_future_live_digest",
    "compare_timeline_shadow_to_future_live_digest"
  ]
  and .durable_identity_evidence.durable_field_count == 7
  and .durable_identity_evidence.preview_binding_count == 5
  and .durable_identity_evidence.invariant_count == 7
  and .durable_identity_evidence.currently_satisfied == false
  and .invariant_count == 7
  and (.invariants | length) == .invariant_count
  and (.invariants | map(.id) == [
    "shadow_live_comparison_requires_durable_identity_evidence",
    "shadow_live_comparison_is_digest_only",
    "live_readback_is_disabled_in_preview",
    "any_mismatch_blocks_promotion",
    "redaction_and_scope_drift_are_critical",
    "operator_views_are_local_only",
    "shadow_live_readback_comparison_preview_has_no_side_effects"
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
    "hepta_work_graph_durable_identity_preview_gate"
  ])
  and .recommended_next_gate == "hepta_work_graph_persistence_enforcement_rollout_blocker_preview_gate"
  and .ready_for_enforcement_rollout_blocker_preview == true
  and .ready_for_live_readback == false
  and .ready_for_live_persistence == false
  and .source_probes.persistence_shadow_live_readback_comparison.rust_module_present == true
  and .source_probes.persistence_shadow_live_readback_comparison.report_script_present == true
  and .source_probes.persistence_shadow_live_readback_comparison.gate_script_present == true
  and .source_probes.persistence_promotion_blocker.rust_module_present == true
  and .source_probes.persistence_promotion_blocker.gate_script_present == true
  and .source_probes.durable_identity.rust_module_present == true
  and .source_probes.durable_identity.report_script_present == true
  and .source_probes.durable_identity.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_persistence_shadow_live_readback_comparison_preview --lib

echo "Hepta WorkGraph persistence shadow/live readback comparison preview gate passed"
