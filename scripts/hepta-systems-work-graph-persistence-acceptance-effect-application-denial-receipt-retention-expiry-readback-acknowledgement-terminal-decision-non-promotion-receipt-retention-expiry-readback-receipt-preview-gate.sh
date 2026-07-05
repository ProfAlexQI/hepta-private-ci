#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-retention-expiry-readback-receipt-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-retention-expiry-readback-receipt-preview-report" "$REPORT_SCRIPT")"
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
  and .gate == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_gate"
  and .schema_version == "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_v1"
  and .preview_mode == "read_only_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_no_receipt_write"
  and .readback_receipt_count == 6
  and (.readback_receipts | length) == .readback_receipt_count
  and (.readback_receipts | map(.id) == [
    "terminal_receipt_retention_policy_readback_receipt",
    "terminal_receipt_expiry_guard_readback_receipt",
    "terminal_receipt_supersession_guard_readback_receipt",
    "terminal_receipt_gc_denial_readback_receipt",
    "terminal_receipt_zero_effect_digest_readback_receipt",
    "terminal_receipt_release_public_claim_denial_readback_receipt"
  ])
  and (.readback_receipts | all(
    .redaction_state == "hash_only_redacted"
    and .persistence_enabled == false
    and .external_delivery_enabled == false
    and (.required_fields | length) >= 15
    and (.required_fields | index("workflow_id") != null)
    and (.required_fields | index("receipt_hash") != null)
  ))
  and .digest_check_count == 7
  and (.digest_checks | length) == .digest_check_count
  and (.digest_checks | map(.id) == [
    "check_durable_identity_digest",
    "check_terminal_receipt_retention_policy_digest",
    "check_terminal_receipt_expiry_guard_digest",
    "check_terminal_receipt_supersession_digest",
    "check_terminal_receipt_gc_denial_digest",
    "check_terminal_receipt_zero_effect_digest",
    "check_terminal_receipt_prior_gate_digest"
  ])
  and (.digest_checks | all(
    .blocks_receipt_acceptance == true
    and (.compared_fields | length) >= 7
    and (.compared_fields | index("workflow_id") != null)
    and (.compared_fields | index("receipt_hash") != null)
  ))
  and .mismatch_denial_count == 8
  and (.mismatch_denials | length) == .mismatch_denial_count
  and (.mismatch_denials | map(.id) == [
    "durable_identity_evidence_missing",
    "missing_terminal_receipt_retention_policy_digest",
    "expired_terminal_receipt_replayed",
    "superseded_terminal_receipt_scope_replayed",
    "terminal_receipt_gc_tombstone_persistence_attempted",
    "terminal_receipt_zero_effect_digest_nonzero",
    "terminal_receipt_public_claim_attempted",
    "terminal_receipt_external_delivery_attempted"
  ])
  and (.mismatch_denials | all(
    .blocks_acceptance == true
    and .blocks_persistence == true
    and (.applies_to_receipt_ids | length) == 6
  ))
  and .receipt_guard_count == 5
  and (.receipt_guards | length) == .receipt_guard_count
  and (.receipt_guards | all(
    .blocks_recording == true
    and (.required_fields | length) >= 10
    and (.required_fields | index("workflow_id") != null)
    and (.required_fields | index("receipt_hash") != null)
  ))
  and .local_view_count == 4
  and (.local_views | length) == .local_view_count
  and (.local_views | all(
    .external_delivery_enabled == false
    and (.required_fields | length) >= 11
    and (.required_fields | index("workflow_id") != null)
    and (.required_fields | index("receipt_hash") != null)
  ))
  and .durable_identity_evidence.schema_version == "work_graph_durable_identity_preview_v1"
  and .durable_identity_evidence.required_prior_gate == "hepta_work_graph_durable_identity_preview_gate"
  and .durable_identity_evidence.required_field_ids == durable_fields
  and .durable_identity_evidence.required_for_readback_receipt_ids == [
    "terminal_receipt_retention_policy_readback_receipt",
    "terminal_receipt_expiry_guard_readback_receipt",
    "terminal_receipt_supersession_guard_readback_receipt",
    "terminal_receipt_gc_denial_readback_receipt",
    "terminal_receipt_zero_effect_digest_readback_receipt",
    "terminal_receipt_release_public_claim_denial_readback_receipt"
  ]
  and .durable_identity_evidence.durable_field_count == 7
  and .durable_identity_evidence.preview_binding_count == 5
  and .durable_identity_evidence.invariant_count == 7
  and .durable_identity_evidence.currently_satisfied == false
  and .invariant_count == 7
  and (.invariants | length) == .invariant_count
  and (.invariants | map(.id) == [
    "terminal_receipt_retention_readback_receipts_require_durable_identity_evidence",
    "terminal_receipt_retention_readback_receipts_are_hash_only",
    "terminal_receipt_retention_readback_receipts_are_non_persistent",
    "terminal_receipt_retention_readback_receipts_block_acceptance",
    "terminal_receipt_retention_readback_receipts_block_gc_mutation",
    "terminal_receipt_retention_readback_views_are_local_only",
    "terminal_receipt_retention_readback_preview_has_no_side_effects"
  ])
  and (.invariants | all(.required == true))
  and (.required_prior_gates[-2] == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_gate")
  and (.required_prior_gates[-1] == "hepta_work_graph_durable_identity_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_gate"
  and .ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview == true
  and .ready_for_operator_acceptance == false
  and .ready_for_live_persistence == false
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt.rust_module_present == true
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt.report_script_present == true
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt.gate_script_present == true
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry.rust_module_present == true
  and .source_probes.persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry.gate_script_present == true
  and .source_probes.durable_identity.rust_module_present == true
  and .source_probes.durable_identity.report_script_present == true
  and .source_probes.durable_identity.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview --lib

echo "Hepta WorkGraph persistence acceptance effect application denial receipt retention expiry readback acknowledgement terminal decision non-promotion receipt retention expiry readback receipt preview gate passed"
