#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-dtd8-td3-rbtd5-td-rt-td-rt-td-rt-td-rt-td-rba-preview-report.sh"
report="$(capture_json_report "hepta-systems-work-graph-dtd8-td3-rbtd-rbtd-rbtd-rt-rba-td-rt-rba-td-rt-rba-td-rt-rba-td-rt-rba-td-rba-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and (.gate | endswith("_receipt_retention_expiry_readback_acknowledgement_preview_gate"))
  and .acknowledgement_contract_count == 6
  and (.acknowledgement_contracts | all(.hash_only == true and .blocks_acceptance == true and .blocks_authority == true))
  and .non_acceptance_reason_count == 7
  and .recording_denial_count == 7
  and (.recording_denials | all(.blocks_acknowledgement_recording == true and .blocks_receipt_recording == true and .blocks_external_delivery == true))
  and .expiry_replay_guard_count == 5
  and .local_view_count == 4
  and .invariant_count == 6
  and (.required_prior_gates[-1] | endswith("_receipt_retention_expiry_readback_receipt_preview_gate"))
  and (.recommended_next_gate | endswith("_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate"))
  and .ready_for_readback_acknowledgement_replay_preview == true
  and .ready_for_operator_acceptance == false
  and .ready_for_live_persistence == false
  and .source_probes.deep_td8_td3_rbackack_td14_receipt_retention_readback_ack.rust_module_present == true
  and .source_probes.prior_deep_td8_td3_rbackack_td14_receipt_retention_readback_receipt.report_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  wg_dtd8_td3_rbtd_rbtd_rbtd_rt_td_rt_td_rt_td_rt_td_rt_td_rt_td_rt_td_rt_td_rt_td_rt_td_rt_rbackack_preview --lib

echo "Hepta WorkGraph deep td8 td3 rbackack td14 receipt retention readback ack preview gate passed"
