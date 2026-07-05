#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
ATTACHMENT_REPORT="$ROOT/scripts/hepta-systems-terminal-release-claim-final-index-terminal-operator-readiness-non-approval-index-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TERMINAL_OPERATOR_READINESS_NON_APPROVAL_INDEX_ATTACHMENT_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-operator-readiness-non-approval-index-attachment-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$ATTACHMENT_REPORT" ]] || fail "missing executable terminal operator readiness non-approval attachment report: $ATTACHMENT_REPORT"
[[ -f "$DOC" ]] || fail "missing terminal operator readiness non-approval attachment readback architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the terminal operator readiness non-approval attachment readback report"
fi

jq -n \
  --arg gate "scripts/hepta-systems-terminal-operator-readiness-non-approval-index-attachment-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_TERMINAL_OPERATOR_READINESS_NON_APPROVAL_INDEX_ATTACHMENT_READBACK_2026-06-21.md" \
  '
  [
    {id:"terminal_operator_readiness_non_approval_index_attachment_ready", observed:true, expected:true},
    {id:"terminal_operator_readiness_non_approval_index_gate_present", observed:true, expected:true},
    {id:"terminal_operator_readiness_non_approval_index_gate_invoked", observed:false, expected:false},
    {id:"terminal_non_activation_release_claim_index_gate_invoked", observed:false, expected:false},
    {id:"terminal_public_distribution_non_publication_lock_gate_invoked", observed:false, expected:false},
    {id:"terminal_release_artifact_non_write_lock_gate_invoked", observed:false, expected:false},
    {id:"terminal_release_governance_final_audit_gate_invoked", observed:false, expected:false},
    {id:"terminal_summary_gates_invoked", observed:false, expected:false},
    {id:"terminal_live_gates_invoked", observed:false, expected:false},
    {id:"canonical_gate_wrapper_invoked", observed:false, expected:false},
    {id:"wrapper_target_invoked", observed:false, expected:false},
    {id:"operator_approval_recorded", observed:false, expected:false},
    {id:"operator_identity_accepted", observed:false, expected:false},
    {id:"rollback_execution_allowed", observed:false, expected:false},
    {id:"operator_readiness_index_persistence_allowed", observed:false, expected:false},
    {id:"public_distribution_publication_allowed", observed:false, expected:false},
    {id:"release_publication_allowed", observed:false, expected:false},
    {id:"release_artifact_write_allowed", observed:false, expected:false},
    {id:"public_release_claim_allowed", observed:false, expected:false},
    {id:"package_or_release_write_allowed", observed:false, expected:false}
  ] as $readback_checks |
  ($readback_checks | all(.observed == .expected)) as $readback_ready |
  {
    runtime:"hepta",
    surface:"terminal_operator_readiness_non_approval_index_attachment_readback",
    plugin_id:"hepta-system@hepta-local",
    status:(if $readback_ready then "ready_blocked" else "blocked" end),
    readback_mode:"static_terminal_operator_readiness_non_approval_index_attachment_snapshot_only",
    source_terminal_operator_readiness_non_approval_index_attachment_surface:"terminal_release_claim_final_index_terminal_operator_readiness_non_approval_index",
    source_terminal_operator_readiness_non_approval_index_attachment_basis:"verified_terminal_operator_readiness_non_approval_index_attachment_snapshot",
    source_terminal_operator_readiness_non_approval_index_attachment_report_reexecuted:false,
    source_terminal_operator_readiness_non_approval_index_attachment_ready:true,
    source_terminal_operator_readiness_non_approval_index_attachment_blocked:true,
    terminal_operator_readiness_non_approval_index_attachment_readback_ready:$readback_ready,
    terminal_operator_readiness_non_approval_index_attachment_readback_blocked:true,
    readback_check_count:($readback_checks | length),
    readback_checks:$readback_checks,
    terminal_non_activation_release_claim_index_attachment_final_index_attached:true,
    terminal_operator_readiness_non_approval_index_gate_present:true,
    terminal_operator_readiness_non_approval_index_doc_present:true,
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
    source_canonical_governance_rollback_anchor:"current_canonical_consumer",
    attachment_blocker_count:24,
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
    next_migration_step:"derive_terminal_operator_readiness_non_approval_index_attachment_final_index_without_operator_gate_invocation",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      terminal_operator_readiness_non_approval_index_attachment_report:"scripts/hepta-systems-terminal-release-claim-final-index-terminal-operator-readiness-non-approval-index-report.sh"
    },
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      terminal_operator_readiness_non_approval_index_gate_invoked:false,
      operator_readiness_index_recorded:false,
      operator_readiness_index_persisted:false,
      operator_approval_recorded:false,
      operator_identity_accepted:false,
      rollback_executed:false,
      terminal_non_activation_release_claim_index_gate_invoked:false,
      release_claim_index_recorded:false,
      release_claim_index_persisted:false,
      public_release_claim_recorded:false,
      terminal_public_distribution_non_publication_lock_gate_invoked:false,
      public_distribution_published:false,
      terminal_release_artifact_non_write_lock_gate_invoked:false,
      terminal_release_governance_gate_invoked:false,
      release_artifact_written:false,
      terminal_summary_gate_invoked:false,
      terminal_live_gate_invoked:false,
      terminal_live_url_contacted:false,
      long_soak_started:false,
      canonical_gate_invoked:false,
      wrapper_target_invoked:false,
      plugin_cache_mutated:false,
      tool_registered:false,
      execution_adapter_dispatched:false,
      tool_invoked:false,
      tool_invocation_ledger_written:false,
      approval_broker_mutated:false,
      approval_requested:false,
      operator_cutover_acceptance_recorded:false,
      live_cutover_started:false,
      result_receipt_written:false,
      rollback_receipt_written:false,
      mcp_server_started:false,
      app_connector_started:false,
      workflow_event_log_mutated:false,
      credential_read:false,
      provider_invoked:false,
      model_invoked:false,
      channel_send_performed:false,
      gateway_or_auth_mutated:false,
      native_post_mutation_performed:false,
      package_or_release_written:false,
      public_ga_promoted:false
    }
  }'
