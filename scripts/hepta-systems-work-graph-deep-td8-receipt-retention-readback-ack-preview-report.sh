#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

prior_report_script="scripts/hepta-systems-work-graph-deep-td8-receipt-retention-readback-receipt-preview-report.sh"
prior_gate_script="scripts/hepta-systems-work-graph-deep-td8-receipt-retention-readback-receipt-preview-gate.sh"
prior_report="$("$ROOT/$prior_report_script")"
GATE="$(jq -r '.recommended_next_gate' <<<"$prior_report")"
BASE="${GATE%_receipt_retention_expiry_readback_acknowledgement_preview_gate}"
SCHEMA="${GATE#hepta_}"
SCHEMA="${SCHEMA%_gate}_v1"
NEXT="${BASE}_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate"

path_exists() { [[ -e "$1" ]]; }
bool_for() {
  if "$@"; then printf 'true\n'; else printf 'false\n'; fi
}

rust_module="codex-rs/hepta-runtime/src/wg_deep_td8_receipt_retention_readback_ack_preview.rs"
report_script="scripts/hepta-systems-work-graph-deep-td8-receipt-retention-readback-ack-preview-report.sh"
gate_script="scripts/hepta-systems-work-graph-deep-td8-receipt-retention-readback-ack-preview-gate.sh"
required_prior_gates="$(jq -c '.required_prior_gates + [.gate]' <<<"$prior_report")"

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
  def items($prefix; $count): [range(0; $count) | {id: "\($prefix)_\(.)", source_receipt_ids: [range(0; 6) | "deep_td8_readback_receipt_\(.)"], required_fields: ["readbackReceiptHash", "scopeEpoch", "zeroEffectHash"], hash_only: true, blocks_recording: true, blocks_acceptance: true, blocks_authority: true, blocks_external_delivery: true, required: true}];
  {
    product: "Hepta",
    runtime: "hepta",
    status: "ready",
    gate: $gate,
    schema_version: $schema,
    preview_mode: "read_only_deep_td8_receipt_retention_readback_acknowledgement_preview_no_recording",
    acknowledgement_contract_count: 6,
    non_acceptance_reason_count: 7,
    recording_denial_count: 7,
    expiry_replay_guard_count: 5,
    local_view_count: 4,
    invariant_count: 6,
    required_prior_gates: $required_prior_gates,
    acknowledgement_contracts: items("deep_td8_readback_ack_contract"; 6),
    non_acceptance_reasons: items("deep_td8_readback_ack_non_acceptance"; 7),
    recording_denials: items("deep_td8_readback_ack_recording_denial"; 7),
    expiry_replay_guards: items("deep_td8_readback_ack_expiry_replay_guard"; 5),
    local_views: items("deep_td8_readback_ack_view"; 4),
    invariants: items("deep_td8_readback_ack_invariant"; 6),
    recommended_next_gate: $next,
    ready_for_acknowledgement_replay_idempotency_preview: true,
    ready_for_operator_acceptance: false,
    ready_for_live_persistence: false,
    side_effects: {
      filesystem_written: false,
      graph_state_persisted: false,
      readback_receipt_persisted: false,
      acknowledgement_recorded: false,
      receipt_recorded: false,
      operator_acceptance_recorded: false,
      approval_recorded: false,
      authority_granted: false,
      live_persistence_enabled: false,
      wal_written: false,
      checkpoint_written: false,
      rollout_started: false,
      release_published: false,
      public_claim_recorded: false,
      external_send_performed: false,
      model_invoked: false
    },
    source_probes: {
      deep_td8_readback_ack: {
        rust_module_present: $rust_module_present,
        report_script_present: $report_script_present,
        gate_script_present: $gate_script_present
      },
      prior_readback_receipt: {
        report_script_present: $prior_report_script_present,
        gate_script_present: $prior_gate_script_present
      }
    }
  }
'
