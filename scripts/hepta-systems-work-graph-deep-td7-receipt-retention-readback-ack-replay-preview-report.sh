#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

BASE="hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion"
GATE="${BASE}_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate"
SCHEMA="${GATE#hepta_}"
SCHEMA="${SCHEMA%_gate}_v1"
NEXT="${BASE}_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate"

path_exists() { [[ -e "$1" ]]; }
bool_for() {
  if "$@"; then printf 'true\n'; else printf 'false\n'; fi
}

rust_module="codex-rs/hepta-runtime/src/wg_deep_td7_receipt_retention_readback_ack_replay_preview.rs"
report_script="scripts/hepta-systems-work-graph-deep-td7-receipt-retention-readback-ack-replay-preview-report.sh"
gate_script="scripts/hepta-systems-work-graph-deep-td7-receipt-retention-readback-ack-replay-preview-gate.sh"
prior_report_script="scripts/hepta-systems-work-graph-deep-td7-receipt-retention-readback-ack-preview-report.sh"
prior_gate_script="scripts/hepta-systems-work-graph-deep-td7-receipt-retention-readback-ack-preview-gate.sh"
required_prior_gates="$("$ROOT/$prior_report_script" | jq -c '.required_prior_gates + [.gate]')"

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
  def ack_ids: [range(0; 6) | "deep_td7_readback_ack_\(.)"];
  def items($prefix; $count): [range(0; $count) | {id: "\($prefix)_\(.)", source_acknowledgement_ids: ack_ids, required_fields: ["idempotencyKey", "priorGateDigest", "zeroEffectHash"], blocks_replay_mutation: true, blocks_acknowledgement_recording: true, blocks_acceptance: true, blocks_authority: true, blocks_rollout: true, blocks_release_publication: true, blocks_public_claim: true, blocks_external_delivery: true, required: true}];
  {
    product: "Hepta",
    runtime: "hepta",
    status: "ready",
    gate: $gate,
    schema_version: $schema,
    preview_mode: "read_only_deep_td7_receipt_retention_readback_acknowledgement_replay_idempotency_preview_no_replay_write",
    replay_scenario_count: 6,
    idempotency_guard_count: 7,
    replay_denial_count: 7,
    monotonicity_check_count: 5,
    local_view_count: 4,
    invariant_count: 6,
    required_prior_gates: $required_prior_gates,
    replay_scenarios: items("deep_td7_readback_ack_replay_scenario"; 6),
    idempotency_guards: items("deep_td7_readback_ack_replay_idempotency_guard"; 7),
    replay_denials: items("deep_td7_readback_ack_replay_denial"; 7),
    monotonicity_checks: items("deep_td7_readback_ack_replay_monotonicity_check"; 5),
    local_views: items("deep_td7_readback_ack_replay_view"; 4),
    invariants: items("deep_td7_readback_ack_replay_invariant"; 6),
    recommended_next_gate: $next,
    ready_for_terminal_decision_non_promotion_preview: true,
    ready_for_operator_acceptance: false,
    ready_for_live_persistence: false,
    side_effects: {
      filesystem_written: false,
      graph_state_persisted: false,
      acknowledgement_replay_recorded: false,
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
      deep_td7_readback_ack_replay: {
        rust_module_present: $rust_module_present,
        report_script_present: $report_script_present,
        gate_script_present: $gate_script_present
      },
      prior_readback_ack: {
        report_script_present: $prior_report_script_present,
        gate_script_present: $prior_gate_script_present
      }
    }
  }
'
