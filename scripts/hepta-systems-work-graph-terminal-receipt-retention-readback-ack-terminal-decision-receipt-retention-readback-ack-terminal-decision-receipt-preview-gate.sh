#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-terminal-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-receipt-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-terminal-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-receipt-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_gate"
  and .schema_version == "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_v1"
  and .preview_mode == "read_only_terminal_decision_receipt_retention_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_hash_only_no_recording"
  and .receipt_count == 6
  and (.receipts | length) == .receipt_count
  and (.receipts | all(
    (.receipt_hash_mode | startswith("hash_only_"))
    and (.required_fields | index("workflow_id") != null)
    and (.required_fields | index("receipt_hash") != null)
    and (.required_fields | length) >= 12
    and .persisted == false
    and .receipt_recording_allowed == false
    and .acceptance_allowed == false
    and .external_delivery_enabled == false
    and (.source_terminal_decision_surface_ids | length) == 6
  ))
  and .digest_check_count == 6
  and (.digest_checks | length) == .digest_check_count
  and (.digest_checks | all(
    .blocks_receipt_recording == true
    and (.compared_fields | index("workflow_id") != null)
    and (.compared_fields | index("receipt_hash") != null)
    and (.compared_fields | length) >= 10
  ))
  and .mismatch_denial_count == 8
  and (.mismatch_denials | length) == .mismatch_denial_count
  and (.mismatch_denials | any(.id == "durable_identity_evidence_missing"))
  and (.mismatch_denials | all(
    .blocks_receipt_recording == true
    and .blocks_acceptance == true
    and .blocks_authority == true
    and .blocks_rollout == true
    and .blocks_release_publication == true
    and .blocks_public_claim == true
    and .blocks_external_delivery == true
    and (.applies_to_receipt_ids | length) == 6
  ))
  and .receipt_guard_count == 5
  and (.receipt_guards | length) == .receipt_guard_count
  and (.receipt_guards | all(
    .receipt_recording_allowed == false
    and .promotion_allowed == false
    and .public_claim_allowed == false
    and (.required_fields | index("workflow_id") != null)
    and (.required_fields | index("receipt_hash") != null)
    and (.required_fields | length) >= 10
  ))
  and .local_view_count == 4
  and (.local_views | length) == .local_view_count
  and (.local_views | all(
    .external_delivery_enabled == false
    and (.required_fields | index("workflow_id") != null)
    and (.required_fields | index("receipt_hash") != null)
    and (.required_fields | length) >= 11
  ))
  and .durable_identity_evidence.schema_version == "work_graph_durable_identity_preview_v1"
  and .durable_identity_evidence.required_prior_gate == "hepta_work_graph_durable_identity_preview_gate"
  and .durable_identity_evidence.required_field_ids == ["workflow_id", "run_id", "step_id", "checkpoint", "replay_key", "rollback_anchor", "receipt_hash"]
  and (.durable_identity_evidence.required_for_receipt_ids | length) == 6
  and .durable_identity_evidence.durable_field_count == 7
  and .durable_identity_evidence.preview_binding_count == 5
  and .durable_identity_evidence.invariant_count == 7
  and .durable_identity_evidence.currently_satisfied == false
  and .invariant_count == 7
  and (.invariants | length) == .invariant_count
  and (.invariants | all(.required == true))
  and (.invariants | any(.id == "terminal_receipt_retention_readback_ack_terminal_decision_receipts_require_durable_identity_evidence"))
  and (.required_prior_gates[-2] == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate")
  and (.required_prior_gates[-1] == "hepta_work_graph_durable_identity_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_gate"
  and .ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview == true
  and .ready_for_operator_acceptance == false
  and .ready_for_live_persistence == false
  and .source_probes.terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt.rust_module_present == true
  and .source_probes.terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt.report_script_present == true
  and .source_probes.terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt.gate_script_present == true
  and .source_probes.terminal_decision_receipt_retention_readback_ack_terminal_decision.report_script_present == true
  and .source_probes.terminal_decision_receipt_retention_readback_ack_terminal_decision.gate_script_present == true
  and .source_probes.durable_identity.report_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  wg_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_preview --lib

echo "Hepta WorkGraph terminal decision receipt retention readback acknowledgement terminal decision receipt preview gate passed"
