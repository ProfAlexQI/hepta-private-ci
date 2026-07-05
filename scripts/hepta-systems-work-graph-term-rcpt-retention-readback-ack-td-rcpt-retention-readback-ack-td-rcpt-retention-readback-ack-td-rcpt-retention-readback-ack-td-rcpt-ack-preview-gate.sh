#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-term-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-ack-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-term-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-ack-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_gate"
  and .schema_version == "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_v1"
  and .preview_mode == "read_only_terminal_receipt_retention_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_no_recording"
  and .acknowledgement_contract_count == 6
  and (.acknowledgement_contracts | length) == .acknowledgement_contract_count
  and (.acknowledgement_contracts | map(.id) == [
    "operator_terminal_decision_receipt_retention_readback_ack_decision_receipt_acknowledgement",
    "release_owner_terminal_decision_receipt_retention_readback_ack_decision_receipt_acknowledgement",
    "authority_denial_terminal_decision_receipt_retention_readback_ack_receipt_acknowledgement",
    "rollout_denial_terminal_decision_receipt_retention_readback_ack_receipt_acknowledgement",
    "release_publication_denial_terminal_decision_receipt_retention_readback_ack_receipt_acknowledgement",
    "external_delivery_denial_terminal_decision_receipt_retention_readback_ack_receipt_acknowledgement"
  ])
  and (.acknowledgement_contracts | all(
    .acknowledgement_recording_allowed == false
    and .acceptance_allowed == false
    and .authority_grant_allowed == false
    and .public_claim_enabled == false
    and .external_delivery_enabled == false
    and (.source_receipt_ids | length) == 6
  ))
  and .non_acceptance_reason_count == 7
  and (.non_acceptance_reasons | length) == .non_acceptance_reason_count
  and (.non_acceptance_reasons | all(.blocks_acceptance == true and .blocks_authority == true))
  and .recording_denial_count == 7
  and (.recording_denials | length) == .recording_denial_count
  and (.recording_denials | all(
    .blocks_acknowledgement_recording == true
    and .blocks_acceptance == true
    and .blocks_authority == true
    and .blocks_release_publication == true
    and .blocks_public_claim == true
    and .blocks_external_delivery == true
    and (.applies_to_acknowledgement_ids | length) == 6
  ))
  and .expiry_replay_guard_count == 5
  and (.expiry_replay_guards | length) == .expiry_replay_guard_count
  and (.expiry_replay_guards | all(.blocks_acknowledgement_recording == true and (.required_fields | length) >= 3))
  and .local_view_count == 4
  and (.local_views | length) == .local_view_count
  and (.local_views | all(.external_delivery_enabled == false and (.required_fields | length) >= 4))
  and .invariant_count == 6
  and (.invariants | length) == .invariant_count
  and (.invariants | all(.required == true))
  and (.required_prior_gates[-1] == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_gate"
  and .ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview == true
  and .ready_for_operator_acceptance == false
  and .ready_for_live_persistence == false
  and .source_probes.term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement.rust_module_present == true
  and .source_probes.term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement.report_script_present == true
  and .source_probes.term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_acknowledgement.gate_script_present == true
  and .source_probes.term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt.report_script_present == true
  and .source_probes.term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  wg_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_ack_preview --lib

echo "Hepta WorkGraph terminal receipt retention readback acknowledgement terminal decision receipt acknowledgement preview gate passed"
