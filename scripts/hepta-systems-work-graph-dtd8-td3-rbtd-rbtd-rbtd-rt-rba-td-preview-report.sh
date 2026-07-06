#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

prior_report_script="scripts/hepta-systems-work-graph-dtd8-td3-rbtd-rbtd-rbtd-rt-rba-rp-preview-report.sh"
prior_gate_script="scripts/hepta-systems-work-graph-dtd8-td3-rbtd-rbtd-rbtd-rt-rba-rp-preview-gate.sh"
prior_report="$("$ROOT/$prior_report_script")"
GATE="$(jq -r '.recommended_next_gate' <<<"$prior_report")"
BASE="${GATE%_preview_gate}"
SCHEMA="${GATE#hepta_}"
SCHEMA="${SCHEMA%_gate}_v1"
NEXT="${BASE}_receipt_preview_gate"
required_prior_gates="$(jq -c '.required_prior_gates + [.gate]' <<<"$prior_report")"

path_exists() { [[ -e "$1" ]]; }
bool_for() {
  if "$@"; then printf 'true\n'; else printf 'false\n'; fi
}

rust_module="codex-rs/hepta-runtime/src/wg_dtd8_td3_rbtd_rbtd_rbtd_rt_td_preview.rs"
report_script="scripts/hepta-systems-work-graph-dtd8-td3-rbtd-rbtd-rbtd-rt-rba-td-preview-report.sh"
gate_script="scripts/hepta-systems-work-graph-dtd8-td3-rbtd-rbtd-rbtd-rt-rba-td-preview-gate.sh"

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
  def source_ids: [range(0; 6) | "deep_td8_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_surface_\(.)"];
  def item($prefix; $count): [range(0; $count) | {
    id: "\($prefix)_\(.)",
    source_ids: source_ids,
    required_fields: ["priorGate", "terminalDecisionHash", "zeroEffectHash"],
    blocks_terminal_decision_recording: true,
    blocks_persistence_promotion: true,
    blocks_authority_grant: true,
    blocks_rollout: true,
    blocks_release_publication: true,
    blocks_public_claim: true,
    blocks_external_delivery: true,
    required: true
  }];
  {
    product: "Hepta",
    runtime: "hepta",
    status: "ready",
    gate: $gate,
    schema_version: $schema,
    preview_mode: "read_only_deep_td8_td3_rbackack_td4_receipt_retention_readback_ack_terminal_decision_non_promotion_preview_no_promotion",
    terminal_decision_surface_count: 6,
    non_promotion_denial_count: 8,
    authority_guard_count: 6,
    release_delivery_guard_count: 6,
    local_view_count: 4,
    invariant_count: 6,
    required_prior_gates: $required_prior_gates,
    terminal_decision_surfaces: item("deep_td8_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_surface"; 6),
    non_promotion_denials: item("deep_td8_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_non_promotion_denial"; 8),
    authority_guards: item("deep_td8_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_authority_guard"; 6),
    release_delivery_guards: item("deep_td8_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_release_delivery_guard"; 6),
    local_views: item("deep_td8_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_local_view"; 4),
    invariants: item("deep_td8_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_invariant"; 6),
    recommended_next_gate: $next,
    ready_for_terminal_decision_receipt_preview: true,
    ready_for_operator_acceptance: false,
    ready_for_live_persistence: false,
    side_effects: {
      filesystem_written: false,
      graph_state_persisted: false,
      terminal_decision_recorded: false,
      terminal_decision_receipt_recorded: false,
      acknowledgement_recorded: false,
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
      deep_td8_td3_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_td_rcptret_rbackack_terminal_decision: {
        rust_module_present: $rust_module_present,
        report_script_present: $report_script_present,
        gate_script_present: $gate_script_present
      },
      prior_deep_td8_td3_rbackack_td4_receipt_retention_readback_ack_replay: {
        report_script_present: $prior_report_script_present,
        gate_script_present: $prior_gate_script_present
      }
    }
  }
'
