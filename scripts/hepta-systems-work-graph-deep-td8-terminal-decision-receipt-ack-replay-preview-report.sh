#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

path_exists() { [[ -e "$1" ]]; }
bool_for() {
  if "$@"; then printf 'true\n'; else printf 'false\n'; fi
}

prior_report_script="scripts/hepta-systems-work-graph-deep-td8-terminal-decision-receipt-ack-preview-report.sh"
prior_gate_script="scripts/hepta-systems-work-graph-deep-td8-terminal-decision-receipt-ack-preview-gate.sh"
prior_report="$("$ROOT/$prior_report_script")"
GATE="$(jq -r '.recommended_next_gate' <<<"$prior_report")"
BASE="${GATE%_receipt_acknowledgement_replay_idempotency_preview_gate}"
SCHEMA="${GATE#hepta_}"
SCHEMA="${SCHEMA%_gate}_v1"
NEXT="${BASE}_receipt_retention_expiry_preview_gate"
required_prior_gates="$(jq -c '.required_prior_gates + [.gate]' <<<"$prior_report")"

rust_module="codex-rs/hepta-runtime/src/wg_deep_td8_terminal_decision_receipt_ack_replay_preview.rs"
report_script="scripts/hepta-systems-work-graph-deep-td8-terminal-decision-receipt-ack-replay-preview-report.sh"
gate_script="scripts/hepta-systems-work-graph-deep-td8-terminal-decision-receipt-ack-replay-preview-gate.sh"

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
  def ack_ids: [range(0; 6) | "deep_td8_terminal_decision_receipt_ack_\(.)"];
  def scenario_ids: [range(0; 6) | "deep_td8_receipt_ack_replay_scenario_\(.)"];
  def scenario($i): {
    id: "deep_td8_receipt_ack_replay_scenario_\($i)",
    source_acknowledgement_ids: ack_ids,
    replay_mode: (["duplicate_receipt", "duplicate_acknowledgement", "stale_receipt_digest", "superseded_receipt_scope", "cross_scope_acknowledgement", "out_of_order_acknowledgement"][$i]),
    acknowledgement_recording_allowed: false,
    receipt_recording_allowed: false,
    mutation_allowed: false
  };
  def idempotency_guard($i): {
    id: "deep_td8_receipt_ack_idempotency_guard_\($i)",
    required_fields: ["idempotencyKey", "priorGateDigest", "zeroEffectHash"],
    blocks_replay_mutation: true
  };
  def replay_denial($i): {
    id: "deep_td8_receipt_ack_replay_denial_\($i)",
    applies_to_replay_scenario_ids: scenario_ids,
    reason: "deep td8 receipt acknowledgement replay cannot mutate state",
    blocks_acknowledgement_recording: true,
    blocks_receipt_recording: true,
    blocks_acceptance: true,
    blocks_authority: true,
    blocks_rollout: true,
    blocks_release_publication: true,
    blocks_public_claim: true,
    blocks_external_delivery: true
  };
  def monotonicity_check($i): {
    id: "deep_td8_receipt_ack_monotonicity_check_\($i)",
    compared_fields: ["currentGate", "priorGateDigest", "nextGate"],
    blocks_out_of_order_replay: true
  };
  def local_view($i): {
    id: "deep_td8_receipt_ack_replay_local_view_\($i)",
    audience: (["operator", "release_owner", "auditor", "system"][$i]),
    required_fields: ["replayScenarioId", "idempotencyKey", "mutationAllowed", "nextGate"],
    external_delivery_enabled: false
  };
  def invariant($i): {
    id: "deep_td8_receipt_ack_replay_invariant_\($i)",
    required: true,
    reason: "deep td8 receipt acknowledgement replay is idempotent and zero-effect"
  };
  {
    product: "Hepta",
    runtime: "hepta",
    status: "ready",
    gate: $gate,
    schema_version: $schema,
    preview_mode: "read_only_deep_td8_terminal_decision_receipt_acknowledgement_replay_idempotency_preview_no_replay_write",
    replay_scenario_count: 6,
    idempotency_guard_count: 7,
    replay_denial_count: 7,
    monotonicity_check_count: 5,
    local_view_count: 4,
    invariant_count: 6,
    required_prior_gates: $required_prior_gates,
    replay_scenarios: [range(0; 6) | scenario(.)],
    idempotency_guards: [range(0; 7) | idempotency_guard(.)],
    replay_denials: [range(0; 7) | replay_denial(.)],
    monotonicity_checks: [range(0; 5) | monotonicity_check(.)],
    local_views: [range(0; 4) | local_view(.)],
    invariants: [range(0; 6) | invariant(.)],
    recommended_next_gate: $next,
    ready_for_receipt_retention_expiry_preview: true,
    ready_for_operator_acceptance: false,
    ready_for_live_persistence: false,
    side_effects: {
      filesystem_written: false,
      graph_state_persisted: false,
      terminal_decision_recorded: false,
      terminal_decision_receipt_recorded: false,
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
      deep_td8_terminal_decision_receipt_ack_replay: {
        rust_module_present: $rust_module_present,
        report_script_present: $report_script_present,
        gate_script_present: $gate_script_present
      },
      prior_deep_td8_terminal_decision_receipt_ack: {
        report_script_present: $prior_report_script_present,
        gate_script_present: $prior_gate_script_present
      }
    }
  }
'
