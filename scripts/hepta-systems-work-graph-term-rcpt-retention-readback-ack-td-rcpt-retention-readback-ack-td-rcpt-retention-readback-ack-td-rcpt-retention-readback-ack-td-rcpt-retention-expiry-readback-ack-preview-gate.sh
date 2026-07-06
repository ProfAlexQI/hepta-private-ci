#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-term-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-expiry-readback-ack-preview-report.sh"

report="$(capture_json_report "hepta-systems-work-graph-term-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-expiry-readback-ack-preview-report" "$REPORT_SCRIPT")"
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
  def has_durable_fields:
    (.required_fields | index("workflow_id") != null)
    and (.required_fields | index("run_id") != null)
    and (.required_fields | index("step_id") != null)
    and (.required_fields | index("checkpoint") != null)
    and (.required_fields | index("replay_key") != null)
    and (.required_fields | index("rollback_anchor") != null)
    and (.required_fields | index("receipt_hash") != null);

  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_gate"
  and .schema_version == "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_v1"
  and .preview_mode == "read_only_terminal_receipt_retention_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_no_recording"
  and .acknowledgement_contract_count == 6
  and (.acknowledgement_contracts | length) == .acknowledgement_contract_count
  and (.acknowledgement_contracts | all(
    .acknowledgement_recording_allowed == false
    and .acceptance_allowed == false
    and .approval_recording_allowed == false
    and .authority_grant_allowed == false
    and .public_claim_enabled == false
    and .external_delivery_enabled == false
    and (.source_readback_receipt_ids | length) == 6
    and (.required_fields | length) >= 11
    and has_durable_fields
  ))
  and .non_acceptance_reason_count == 8
  and (.non_acceptance_reasons | length) == .non_acceptance_reason_count
  and .non_acceptance_reasons[0].id == "durable_identity_evidence_missing"
  and (.non_acceptance_reasons | all(.blocks_acceptance == true and .blocks_approval == true and .blocks_authority == true))
  and .recording_denial_count == 8
  and (.recording_denials | length) == .recording_denial_count
  and .recording_denials[0].id == "deny_durable_identity_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_recording"
  and (.recording_denials | any(.target_record == "durable_identity_terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement_evidence"))
  and (.recording_denials | all(
    .blocks_acknowledgement_recording == true
    and .blocks_receipt_recording == true
    and .blocks_acceptance == true
    and .blocks_authority == true
    and .blocks_release_publication == true
    and .blocks_public_claim == true
    and .blocks_external_delivery == true
    and (.applies_to_acknowledgement_ids | length) == 6
  ))
  and .expiry_replay_guard_count == 5
  and (.expiry_replay_guards | length) == .expiry_replay_guard_count
  and (.expiry_replay_guards | all(.blocks_acknowledgement_recording == true and (.required_fields | length) >= 10 and has_durable_fields))
  and .local_view_count == 4
  and (.local_views | length) == .local_view_count
  and (.local_views | all(.external_delivery_enabled == false and (.required_fields | length) >= 11 and has_durable_fields))
  and .durable_identity_evidence.schema_version == "work_graph_durable_identity_preview_v1"
  and .durable_identity_evidence.required_prior_gate == "hepta_work_graph_durable_identity_preview_gate"
  and .durable_identity_evidence.required_field_ids == durable_fields
  and (.durable_identity_evidence.required_for_acknowledgement_ids | length) == 6
  and .durable_identity_evidence.durable_field_count == 7
  and .durable_identity_evidence.preview_binding_count == 5
  and .durable_identity_evidence.invariant_count == 7
  and .durable_identity_evidence.currently_satisfied == false
  and .invariant_count == 7
  and (.invariants | length) == .invariant_count
  and (.invariants | all(.required == true))
  and (.invariants | any(.id == "terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgements_require_durable_identity_evidence"))
  and (.required_prior_gates[-2] == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_gate")
  and (.required_prior_gates[-1] == "hepta_work_graph_durable_identity_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate"
  and .ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview == true
  and .ready_for_operator_acceptance == false
  and .ready_for_live_persistence == false
  and .source_probes.term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement.rust_module_present == true
  and .source_probes.term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement.report_script_present == true
  and .source_probes.term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement.gate_script_present == true
  and .source_probes.terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipt.report_script_present == true
  and .source_probes.terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipt.gate_script_present == true
  and .source_probes.durable_identity.report_script_present == true
  and .source_probes.durable_identity.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  wg_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_preview --lib

echo "Hepta WorkGraph terminal receipt retention readback acknowledgement terminal decision receipt retention expiry readback acknowledgement preview gate passed"
