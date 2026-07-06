#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-dtd8-retack-td-rback-ack-td-rcpt-ret-rback-ack-td-rcpt-ret-rback-ack-td-receipt-ack-replay-preview-report.sh"
report="$(capture_json_report "hepta-systems-work-graph-dtd8-retack-td-rback-ack-td-rcpt-ret-rback-ack-td-rcpt-ret-rback-ack-td-receipt-ack-replay-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and (.gate | endswith("_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_gate"))
  and .replay_scenario_count == 6
  and (.replay_scenarios | all(.blocks_replay_mutation == true and .blocks_acknowledgement_recording == true and .blocks_receipt_recording == true and .blocks_acceptance == true and .blocks_authority == true and .blocks_external_delivery == true))
  and .idempotency_guard_count == 7
  and (.idempotency_guards | all(.blocks_replay_mutation == true))
  and .replay_denial_count == 7
  and (.replay_denials | all(.blocks_acknowledgement_recording == true and .blocks_receipt_recording == true and .blocks_acceptance == true and .blocks_authority == true and .blocks_external_delivery == true))
  and .monotonicity_check_count == 5
  and .local_view_count == 4
  and .invariant_count == 6
  and (.required_prior_gates[-1] | endswith("_terminal_decision_non_promotion_receipt_acknowledgement_preview_gate"))
  and (.recommended_next_gate | endswith("_terminal_decision_non_promotion_receipt_retention_expiry_preview_gate"))
  and .ready_for_receipt_retention_expiry_preview == true
  and .ready_for_operator_acceptance == false
  and .ready_for_live_persistence == false
  and .source_probes.deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_ack_replay.rust_module_present == true
  and .source_probes.prior_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_ack.report_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  wg_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_ack_replay_preview --lib

echo "Hepta WorkGraph deep td8 retention-ack terminal-decision readback-ack terminal decision receipt acknowledgement replay preview gate passed"
