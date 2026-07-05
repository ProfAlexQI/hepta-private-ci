#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-deep-td8-retention-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-receipt-retention-expiry-preview-report.sh"

report="$(capture_json_report "hepta-systems-work-graph-deep-td8-retention-ack-terminal-decision-receipt-retention-expiry-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"
GATE="$(jq -r '.gate' <<<"$report")"
NEXT="$(jq -r '.recommended_next_gate' <<<"$report")"
PRIOR="$(jq -r '.required_prior_gates[-1]' <<<"$report")"

jq -e --arg gate "$GATE" --arg next "$NEXT" --arg prior "$PRIOR" '
  .gate == $gate
  and (.gate | endswith("_terminal_decision_non_promotion_receipt_retention_expiry_preview_gate"))
  and .retention_policy_count == 6
  and (.retention_policies | all(.mutation_allowed == false and .required == true))
  and .expiry_guard_count == 6
  and .supersession_guard_count == 5
  and (.expiry_guards + .supersession_guards | all(.mutation_allowed == false))
  and .garbage_collection_denial_count == 6
  and (.garbage_collection_denials | all(.mutation_allowed == false))
  and .local_view_count == 4
  and .invariant_count == 6
  and (.invariants | all(.required == true))
  and (.required_prior_gates[-1] == $prior)
  and (.required_prior_gates[-1] | endswith("_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_gate"))
  and .recommended_next_gate == $next
  and (.recommended_next_gate | endswith("_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_gate"))
  and .ready_for_readback_receipt_preview == true
  and .ready_for_operator_acceptance == false
  and .ready_for_live_persistence == false
  and .source_probes.deep_td8_ret_ack_td_receipt_retention_expiry.rust_module_present == true
  and .source_probes.prior_ack_replay.report_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  wg_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_expiry_preview --lib

echo "Hepta WorkGraph deep td8 retention-ack terminal-decision receipt retention expiry preview gate passed"
