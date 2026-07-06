#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-dtd8-rbackacktd-rcptret-rbackack-td2-receipt-preview-report.sh"
report="$(capture_json_report "hepta-systems-work-graph-dtd8-rbackacktd-rcptret-rbackack-td2-receipt-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and (.gate | endswith("_terminal_decision_non_promotion_receipt_preview_gate"))
  and .receipt_count == 6
  and (.receipts | all(.hash_only == true and .blocks_receipt_recording == true and .blocks_acceptance == true and .blocks_authority == true and .blocks_external_delivery == true))
  and .digest_check_count == 6
  and .mismatch_denial_count == 7
  and (.mismatch_denials | all(.blocks_receipt_recording == true and .blocks_acceptance == true and .blocks_authority == true))
  and .receipt_guard_count == 5
  and .local_view_count == 4
  and .invariant_count == 6
  and (.required_prior_gates[-1] | endswith("_terminal_decision_non_promotion_preview_gate"))
  and (.recommended_next_gate | endswith("_terminal_decision_non_promotion_receipt_acknowledgement_preview_gate"))
  and .ready_for_terminal_decision_receipt_acknowledgement_preview == true
  and .ready_for_operator_acceptance == false
  and .ready_for_live_persistence == false
  and .source_probes.latest_terminal_decision_receipt.rust_module_present == true
  and .source_probes.prior_latest_terminal_decision.report_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  wg_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_preview --lib

echo "Hepta WorkGraph deep td8 latest terminal decision receipt preview gate passed"
