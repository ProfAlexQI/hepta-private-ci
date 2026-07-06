#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

prior_report_script="scripts/hepta-systems-work-graph-dtd8-td3-rbtd-rbtd-rbtd-rt-rba-td-rt-rba-td-rt-rba-td-rt-rba-td-rt-exp-preview-report.sh"
prior_gate_script="scripts/hepta-systems-work-graph-dtd8-td3-rbtd-rbtd-rbtd-rt-rba-td-rt-rba-td-rt-rba-td-rt-rba-td-rt-exp-preview-gate.sh"
prior_report="$("$ROOT/$prior_report_script")"
GATE="$(jq -r '.recommended_next_gate' <<<"$prior_report")"
BASE="${GATE%_receipt_retention_expiry_readback_receipt_preview_gate}"
SCHEMA="${GATE#hepta_}"
SCHEMA="${SCHEMA%_gate}_v1"
NEXT="${BASE}_receipt_retention_expiry_readback_acknowledgement_preview_gate"
required_prior_gates="$(jq -c '.required_prior_gates + [.gate]' <<<"$prior_report")"

path_exists() { [[ -e "$1" ]]; }
bool_for() {
  if "$@"; then printf 'true\n'; else printf 'false\n'; fi
}

rust_module="codex-rs/hepta-runtime/src/wg_dtd8_td3_rbtd_rbtd_rbtd_rt_td_rt_td_rt_td_rt_td_rt_rbrcpt_preview.rs"
report_script="scripts/hepta-systems-work-graph-dtd8-td3-rbtd-rbtd-rbtd-rt-rba-td-rt-rba-td-rt-rba-td-rt-rba-td-rt-rbr-preview-report.sh"
gate_script="scripts/hepta-systems-work-graph-dtd8-td3-rbtd-rbtd-rbtd-rt-rba-td-rt-rba-td-rt-rba-td-rt-rba-td-rt-rbr-preview-gate.sh"

jq -n \
  --arg gate "$GATE" \
  --arg schema "$SCHEMA" \
  --arg next "$NEXT" \
  --argjson required_prior_gates "$required_prior_gates" \
  --argjson rust_module_present "$(bool_for path_exists "$rust_module")" \
  --argjson report_script_present "$(bool_for path_exists "$report_script")" \
  --argjson gate_script_present "$(bool_for path_exists "$gate_script")" \
  --argjson prior_report_script_present "$(bool_for path_exists "$prior_report_script")" \
  --argjson prior_gate_script_present "$(bool_for path_exists "$prior_gate_script")" \
  '
  def item($prefix; $count): [range(0; $count) | {
    id: "\($prefix)_\(.)",
    required_fields: ["retentionGate", "readbackDigest", "zeroEffectHash"],
    hash_only: true,
    blocks_recording: true,
    blocks_external_delivery: true,
    required: true
  }];
  {
    product: "Hepta",
    runtime: "hepta",
    status: "ready",
    gate: $gate,
    schema_version: $schema,
    preview_mode: "read_only_deep_td8_td3_rbackack_td7_receipt_retention_readback_receipt_preview_hash_only_no_recording",
    receipt_count: 6,
    digest_check_count: 6,
    mismatch_denial_count: 7,
    receipt_guard_count: 5,
    local_view_count: 4,
    invariant_count: 6,
    required_prior_gates: $required_prior_gates,
    receipts: item("deep_td8_td3_rbackack_td7_rcptret_rbrcpt_receipt"; 6),
    digest_checks: item("deep_td8_td3_rbackack_td7_rcptret_rbrcpt_digest_check"; 6),
    mismatch_denials: item("deep_td8_td3_rbackack_td7_rcptret_rbrcpt_mismatch_denial"; 7),
    receipt_guards: item("deep_td8_td3_rbackack_td7_rcptret_rbrcpt_receipt_guard"; 5),
    local_views: item("deep_td8_td3_rbackack_td7_rcptret_rbrcpt_local_view"; 4),
    invariants: item("deep_td8_td3_rbackack_td7_rcptret_rbrcpt_invariant"; 6),
    recommended_next_gate: $next,
    ready_for_readback_acknowledgement_preview: true,
    ready_for_operator_acceptance: false,
    ready_for_live_persistence: false,
    side_effects: {
      filesystem_written: false,
      graph_state_persisted: false,
      readback_receipt_persisted: false,
      receipt_recorded: false,
      acknowledgement_recorded: false,
      operator_acceptance_recorded: false,
      authority_granted: false,
      wal_written: false,
      checkpoint_written: false,
      release_published: false,
      public_claim_recorded: false,
      external_send_performed: false,
      model_invoked: false
    },
    source_probes: {
      deep_td8_td3_rbackack_td7_receipt_retention_readback_receipt: {
        rust_module_present: $rust_module_present,
        report_script_present: $report_script_present,
        gate_script_present: $gate_script_present
      },
      prior_deep_td8_td3_rbackack_td7_receipt_retention_expiry: {
        report_script_present: $prior_report_script_present,
        gate_script_present: $prior_gate_script_present
      }
    }
  }
'
