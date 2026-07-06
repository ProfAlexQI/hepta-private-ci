#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-promotion-precondition-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-promotion-precondition-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_promotion_precondition_preview_gate"
  and .schema_version == "work_graph_promotion_precondition_preview_v1"
  and .preview_mode == "read_only_promotion_precondition_preview_no_promotion"
  and .promotion_target_count == 6
  and (.promotion_targets | length) == .promotion_target_count
  and (.promotion_targets | map(.id) == [
    "terminal_task_result_promotion",
    "scheduler_unblock_promotion",
    "artifact_handoff_promotion",
    "external_handoff_promotion",
    "approval_resolution_promotion",
    "timeline_operator_summary_promotion"
  ])
  and (.promotion_targets | all(.blocked_without_readback == true and .promotes_state == false and (.required_check_ids | length) >= 3))
  and .required_check_count == 15
  and (.required_checks | length) == .required_check_count
  and (.required_checks | map(.id) == [
    "durable_identity_evidence_ready",
    "task_result_schema_valid",
    "task_result_readback_clean",
    "artifact_readback_clean",
    "approval_readback_clean",
    "timeline_readback_clean",
    "dependency_closure_satisfied",
    "lease_and_budget_current",
    "operator_authority_scope_valid",
    "approval_expiry_current",
    "handoff_scope_authorized",
    "no_external_delivery_enabled",
    "artifact_redaction_verified",
    "no_replay_drift_detected",
    "operator_audit_receipt_ready"
  ])
  and (.required_checks | all(.required_before_promotion == true and (.required_evidence_fields | length) >= 3 and (.failure_denial_id | startswith("deny_"))))
  and (.required_checks[] | select(.id == "durable_identity_evidence_ready") | .required_evidence_fields == [
    "workflow_id",
    "run_id",
    "step_id",
    "checkpoint",
    "replay_key",
    "rollback_anchor",
    "receipt_hash"
  ])
  and (.required_checks[] | select(.id == "operator_audit_receipt_ready") | (.required_evidence_fields | index("receipt_hash")))
  and .denial_reason_count == 15
  and (.denial_reasons | length) == .denial_reason_count
  and (.denial_reasons | map(.id) == [
    "deny_durable_identity_evidence_missing",
    "deny_task_result_schema_missing",
    "deny_task_result_readback_missing",
    "deny_artifact_readback_missing",
    "deny_approval_readback_missing",
    "deny_timeline_readback_missing",
    "deny_dependency_closure_unsatisfied",
    "deny_lease_or_budget_stale",
    "deny_operator_authority_invalid",
    "deny_approval_expired",
    "deny_handoff_scope_unauthorized",
    "deny_external_delivery_enabled",
    "deny_redaction_unverified",
    "deny_replay_drift_detected",
    "deny_audit_receipt_missing"
  ])
  and (.denial_reasons | map(select(.severity == "critical")) | length) == 14
  and (.denial_reasons | all((.blocks_target_ids | length) >= 1 and (.operator_message | length) > 0))
  and .audit_receipt_count == 6
  and (.audit_receipts | length) == .audit_receipt_count
  and (.audit_receipts | map(.target_id) == [
    "terminal_task_result_promotion",
    "scheduler_unblock_promotion",
    "artifact_handoff_promotion",
    "external_handoff_promotion",
    "approval_resolution_promotion",
    "timeline_operator_summary_promotion"
  ])
  and (.audit_receipts | all(
    .persistence_enabled == false
    and .external_delivery_enabled == false
    and (.required_fields | index("receiptHash"))
    and (.required_fields | index("receipt_hash"))
    and (.required_fields | index("rollback_anchor"))
    and (.required_fields | index("redactedEvidenceRefs"))
  ))
  and .invariant_count == 7
  and (.invariants | length) == .invariant_count
  and (.invariants | map(.id) == [
    "promotion_requires_durable_identity",
    "promotion_requires_readback",
    "scheduler_unblock_requires_dependency_closure",
    "handoff_promotion_cannot_deliver",
    "approval_resolution_is_visible_not_recorded",
    "audit_receipts_are_redacted_and_non_persistent",
    "promotion_precondition_preview_has_no_side_effects"
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
    "hepta_work_graph_durable_identity_preview_gate"
  ])
  and .durable_identity_evidence.schema_version == "work_graph_durable_identity_preview_v1"
  and .durable_identity_evidence.durable_field_count == 7
  and .durable_identity_evidence.preview_binding_count == 5
  and .durable_identity_evidence.invariant_count == 7
  and .recommended_next_gate == "hepta_work_graph_activation_enforcement_blocker_preview_gate"
  and .ready_for_activation_enforcement_blocker_preview == true
  and .ready_for_promotion_execution == false
  and .ready_for_live_execution == false
  and .source_probes.promotion_precondition.rust_module_present == true
  and .source_probes.promotion_precondition.report_script_present == true
  and .source_probes.promotion_precondition.gate_script_present == true
  and .source_probes.replay_readback.rust_module_present == true
  and .source_probes.replay_readback.report_script_present == true
  and .source_probes.replay_readback.gate_script_present == true
  and .source_probes.durable_identity.rust_module_present == true
  and .source_probes.durable_identity.report_script_present == true
  and .source_probes.durable_identity.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_promotion_precondition_preview --lib

echo "Hepta WorkGraph promotion precondition preview gate passed"
