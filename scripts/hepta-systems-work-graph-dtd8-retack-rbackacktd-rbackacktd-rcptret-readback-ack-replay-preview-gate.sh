#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-dtd8-retack-rbackacktd-rbackacktd-rcptret-readback-ack-replay-preview-report.sh"
report="$(capture_json_report "hepta-systems-work-graph-dtd8-retack-rbackacktd-rbackacktd-rcptret-readback-ack-replay-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and (.gate | endswith("_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate"))
  and .replay_scenario_count == 6
  and (.replay_scenarios | all(.blocks_replay_mutation == true and (.source_acknowledgement_ids | length) == 6))
  and .idempotency_guard_count == 7
  and (.idempotency_guards | all(.blocks_replay_mutation == true))
  and .replay_denial_count == 7
  and (.replay_denials | all(.blocks_acknowledgement_recording == true and .blocks_acceptance == true and .blocks_authority == true and .blocks_external_delivery == true))
  and .monotonicity_check_count == 5
  and .local_view_count == 4
  and .invariant_count == 6
  and (.required_prior_gates[-1] | endswith("_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_gate"))
  and (.recommended_next_gate | endswith("_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate"))
  and .ready_for_terminal_decision_non_promotion_preview == true
  and .ready_for_operator_acceptance == false
  and .ready_for_live_persistence == false
  and .source_probes.deep_td8_latest_readback_ack_replay.rust_module_present == true
  and .source_probes.prior_readback_ack.report_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  wg_deep_td8_ret_ack_td_rback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_td_receipt_retention_readback_ack_replay_preview --lib

echo "Hepta WorkGraph deep td8 latest receipt retention readback acknowledgement replay idempotency preview gate passed"
