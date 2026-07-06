#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
READBACK_REPORT="$ROOT/scripts/hepta-systems-terminal-operator-readiness-non-approval-index-attachment-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TERMINAL_OPERATOR_READINESS_NON_APPROVAL_INDEX_ATTACHMENT_FINAL_INDEX_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-operator-readiness-non-approval-index-attachment-final-index-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$READBACK_REPORT" ]] || fail "missing executable terminal operator readiness non-approval attachment readback report: $READBACK_REPORT"
[[ -f "$DOC" ]] || fail "missing terminal operator readiness non-approval attachment final index architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the terminal operator readiness non-approval attachment final index report"
fi

jq -n \
  --slurpfile readback <("$READBACK_REPORT") \
  --arg gate "scripts/hepta-systems-terminal-operator-readiness-non-approval-index-attachment-final-index-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_TERMINAL_OPERATOR_READINESS_NON_APPROVAL_INDEX_ATTACHMENT_FINAL_INDEX_2026-06-21.md" \
  '
  ($readback[0]) as $readback |
  [
    "manual_operator_live_cutover_approval_required",
    "terminal_operator_readiness_non_approval_index_not_invoked",
    "terminal_non_activation_release_claim_index_not_invoked",
    "terminal_public_distribution_non_publication_lock_not_invoked",
    "terminal_release_artifact_non_write_lock_not_invoked",
    "terminal_release_governance_final_audit_not_invoked",
    "terminal_summary_gates_not_invoked",
    "terminal_live_gates_not_invoked",
    "canonical_successor_consumer_cutover_disallowed",
    "current_canonical_consumer_rollback_anchor_retained",
    "canonical_gate_not_invoked",
    "wrapper_target_not_invoked",
    "live_url_not_contacted",
    "long_soak_not_started",
    "public_ga_disabled",
    "operator_approval_not_recorded",
    "operator_identity_not_accepted",
    "rollback_execution_disabled",
    "operator_readiness_index_persistence_disabled",
    "public_distribution_publication_disabled",
    "release_publication_disabled",
    "release_artifact_write_disabled",
    "public_release_claim_disabled",
    "package_or_release_write_disabled"
  ] as $final_blockers |
  ($readback.terminal_operator_readiness_non_approval_index_attachment_readback_ready == true
    and $readback.terminal_operator_readiness_non_approval_index_attachment_readback_blocked == true
    and $readback.terminal_operator_readiness_non_approval_index_gate_present == true
    and $readback.terminal_operator_readiness_non_approval_index_gate_invoked == false
    and $readback.terminal_non_activation_release_claim_index_gate_invoked == false
    and $readback.terminal_public_distribution_non_publication_lock_gate_invoked == false
    and $readback.terminal_release_artifact_non_write_lock_gate_invoked == false
    and $readback.terminal_release_governance_final_audit_gate_invoked == false
    and $readback.terminal_summary_gates_invoked == false
    and $readback.terminal_live_gates_invoked == false
    and $readback.canonical_gate_wrapper_invoked == false
    and $readback.wrapper_target_invoked == false
    and $readback.source_successor_consumer_cutover_allowed == false
    and $readback.source_canonical_governance_rollback_anchor == "current_canonical_consumer"
    and $readback.tool_execution_live_cutover_allowed == false
    and $readback.tool_execution_public_ga_allowed == false
    and $readback.public_distribution_publication_allowed == false
    and $readback.release_publication_allowed == false
    and $readback.release_artifact_write_allowed == false
    and $readback.public_release_claim_allowed == false
    and $readback.release_claim_index_persistence_allowed == false
    and $readback.package_or_release_write_allowed == false
    and $readback.operator_approval_recorded == false
    and $readback.operator_identity_accepted == false
    and $readback.rollback_execution_allowed == false
    and $readback.operator_readiness_index_persistence_allowed == false
    and ($readback.side_effects | to_entries | all(.value == false))) as $final_index_ready |
  {
    runtime:"hepta",
    surface:"terminal_operator_readiness_non_approval_index_attachment_final_index",
    plugin_id:$readback.plugin_id,
    status:(if $final_index_ready then "ready_blocked" else "blocked" end),
    source_terminal_operator_readiness_non_approval_index_attachment_readback_surface:$readback.surface,
    source_terminal_operator_readiness_non_approval_index_attachment_readback_ready:$readback.terminal_operator_readiness_non_approval_index_attachment_readback_ready,
    source_terminal_operator_readiness_non_approval_index_attachment_readback_blocked:$readback.terminal_operator_readiness_non_approval_index_attachment_readback_blocked,
    terminal_operator_readiness_non_approval_index_attachment_final_index_ready:$final_index_ready,
    terminal_operator_readiness_non_approval_index_attachment_final_index_blocked:true,
    terminal_non_activation_release_claim_index_attachment_final_index_attached:true,
    terminal_operator_readiness_non_approval_index_gate_present:true,
    terminal_operator_readiness_non_approval_index_gate_invoked:false,
    terminal_non_activation_release_claim_index_gate_invoked:false,
    terminal_public_distribution_non_publication_lock_gate_invoked:false,
    terminal_release_artifact_non_write_lock_gate_invoked:false,
    terminal_release_governance_final_audit_gate_invoked:false,
    terminal_summary_gates_invoked:false,
    terminal_live_gates_invoked:false,
    canonical_gate_wrapper_invoked:false,
    wrapper_target_invoked:false,
    source_successor_consumer_cutover_allowed:false,
    source_canonical_governance_rollback_anchor:$readback.source_canonical_governance_rollback_anchor,
    final_blocker_count:($final_blockers | length),
    final_blockers:$final_blockers,
    manual_operator_live_cutover_approval_required:true,
    terminal_live_url_required:false,
    long_soak_required:false,
    tool_execution_live_cutover_allowed:false,
    tool_execution_public_ga_allowed:false,
    public_distribution_publication_allowed:false,
    release_publication_allowed:false,
    release_artifact_write_allowed:false,
    public_release_claim_allowed:false,
    release_claim_index_persistence_allowed:false,
    package_or_release_write_allowed:false,
    operator_approval_recorded:false,
    operator_identity_accepted:false,
    rollback_execution_allowed:false,
    operator_readiness_index_persistence_allowed:false,
    next_migration_step:"derive_terminal_release_governance_safe_chain_closure_without_operator_gate_invocation",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      terminal_operator_readiness_non_approval_index_attachment_readback_report:"scripts/hepta-systems-terminal-operator-readiness-non-approval-index-attachment-readback-report.sh"
    },
    side_effect_free:true,
    side_effects:$readback.side_effects
  }'
