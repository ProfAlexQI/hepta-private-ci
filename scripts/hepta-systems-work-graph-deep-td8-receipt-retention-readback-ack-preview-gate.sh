#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-deep-td8-receipt-retention-readback-ack-preview-report.sh"

report="$(capture_json_report "hepta-systems-work-graph-deep-td8-receipt-retention-readback-ack-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"
GATE="$(jq -r '.gate' <<<"$report")"
NEXT="$(jq -r '.recommended_next_gate' <<<"$report")"
PRIOR="$(jq -r '.required_prior_gates[-1]' <<<"$report")"

jq -e --arg gate "$GATE" --arg next "$NEXT" --arg prior "$PRIOR" '
  .gate == $gate
  and .acknowledgement_contract_count == 6
  and (.acknowledgement_contracts | all(.hash_only == true and .blocks_acceptance == true and (.source_receipt_ids | length) == 6))
  and .non_acceptance_reason_count == 7
  and (.non_acceptance_reasons | all(.blocks_acceptance == true and .blocks_authority == true))
  and .recording_denial_count == 7
  and (.recording_denials | all(.blocks_recording == true and .blocks_external_delivery == true))
  and .expiry_replay_guard_count == 5
  and .local_view_count == 4
  and .invariant_count == 6
  and (.required_prior_gates[-1] == $prior)
  and .recommended_next_gate == $next
  and .ready_for_acknowledgement_replay_idempotency_preview == true
  and .ready_for_operator_acceptance == false
  and .ready_for_live_persistence == false
  and .source_probes.deep_td8_readback_ack.rust_module_present == true
  and .source_probes.prior_readback_receipt.report_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  wg_deep_td8_receipt_retention_readback_ack_preview --lib

echo "Hepta WorkGraph deep td8 receipt retention readback acknowledgement preview gate passed"
