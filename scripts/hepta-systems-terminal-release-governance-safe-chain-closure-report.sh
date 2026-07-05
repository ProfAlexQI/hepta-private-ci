#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
RELEASE_GOVERNANCE_FINAL_INDEX_REPORT="$ROOT/scripts/hepta-systems-terminal-release-governance-attachment-final-index-report.sh"
RELEASE_ARTIFACT_FINAL_INDEX_REPORT="$ROOT/scripts/hepta-systems-terminal-release-artifact-non-write-lock-attachment-final-index-report.sh"
PUBLIC_DISTRIBUTION_FINAL_INDEX_REPORT="$ROOT/scripts/hepta-systems-terminal-public-distribution-non-publication-lock-attachment-final-index-report.sh"
RELEASE_CLAIM_FINAL_INDEX_REPORT="$ROOT/scripts/hepta-systems-terminal-non-activation-release-claim-index-attachment-final-index-report.sh"
OPERATOR_READINESS_FINAL_INDEX_REPORT="$ROOT/scripts/hepta-systems-terminal-operator-readiness-non-approval-index-attachment-final-index-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TERMINAL_RELEASE_GOVERNANCE_SAFE_CHAIN_CLOSURE_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-release-governance-safe-chain-closure-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$RELEASE_GOVERNANCE_FINAL_INDEX_REPORT" ]] || fail "missing executable terminal release governance attachment final index report: $RELEASE_GOVERNANCE_FINAL_INDEX_REPORT"
[[ -x "$RELEASE_ARTIFACT_FINAL_INDEX_REPORT" ]] || fail "missing executable terminal release artifact non-write lock attachment final index report: $RELEASE_ARTIFACT_FINAL_INDEX_REPORT"
[[ -x "$PUBLIC_DISTRIBUTION_FINAL_INDEX_REPORT" ]] || fail "missing executable terminal public distribution non-publication lock attachment final index report: $PUBLIC_DISTRIBUTION_FINAL_INDEX_REPORT"
[[ -x "$RELEASE_CLAIM_FINAL_INDEX_REPORT" ]] || fail "missing executable terminal non-activation release claim attachment final index report: $RELEASE_CLAIM_FINAL_INDEX_REPORT"
[[ -x "$OPERATOR_READINESS_FINAL_INDEX_REPORT" ]] || fail "missing executable terminal operator readiness non-approval attachment final index report: $OPERATOR_READINESS_FINAL_INDEX_REPORT"
[[ -f "$DOC" ]] || fail "missing terminal release governance safe chain closure architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the terminal release governance safe chain closure report"
fi

jq -n \
  --slurpfile release_governance <("$RELEASE_GOVERNANCE_FINAL_INDEX_REPORT") \
  --slurpfile release_artifact <("$RELEASE_ARTIFACT_FINAL_INDEX_REPORT") \
  --slurpfile public_distribution <("$PUBLIC_DISTRIBUTION_FINAL_INDEX_REPORT") \
  --slurpfile release_claim <("$RELEASE_CLAIM_FINAL_INDEX_REPORT") \
  --slurpfile operator_readiness <("$OPERATOR_READINESS_FINAL_INDEX_REPORT") \
  --arg gate "scripts/hepta-systems-terminal-release-governance-safe-chain-closure-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_TERMINAL_RELEASE_GOVERNANCE_SAFE_CHAIN_CLOSURE_2026-06-21.md" \
  '
  ($release_governance[0]) as $governance |
  ($release_artifact[0]) as $artifact |
  ($public_distribution[0]) as $distribution |
  ($release_claim[0]) as $claim |
  ($operator_readiness[0]) as $operator |
  [
    "terminal_release_governance_attachment_final_index",
    "terminal_release_artifact_non_write_lock_attachment_final_index",
    "terminal_public_distribution_non_publication_lock_attachment_final_index",
    "terminal_non_activation_release_claim_index_attachment_final_index",
    "terminal_operator_readiness_non_approval_index_attachment_final_index"
  ] as $safe_chain_sources |
  [
    $governance.terminal_release_governance_attachment_final_index_ready,
    $artifact.terminal_release_artifact_non_write_lock_attachment_final_index_ready,
    $distribution.terminal_public_distribution_non_publication_lock_attachment_final_index_ready,
    $claim.terminal_non_activation_release_claim_index_attachment_final_index_ready,
    $operator.terminal_operator_readiness_non_approval_index_attachment_final_index_ready
  ] as $source_ready_flags |
  [
    "manual_operator_live_cutover_approval_required",
    "terminal_release_governance_final_audit_not_invoked",
    "terminal_release_artifact_non_write_lock_not_invoked",
    "terminal_public_distribution_non_publication_lock_not_invoked",
    "terminal_non_activation_release_claim_index_not_invoked",
    "terminal_operator_readiness_non_approval_index_not_invoked",
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
    "release_claim_index_persistence_disabled",
    "package_or_release_write_disabled"
  ] as $closure_blockers |
  ($source_ready_flags | map(select(. == true)) | length) as $safe_chain_ready_source_count |
  ($safe_chain_ready_source_count == 5
    and $governance.terminal_release_governance_attachment_final_index_blocked == true
    and $artifact.terminal_release_artifact_non_write_lock_attachment_final_index_blocked == true
    and $distribution.terminal_public_distribution_non_publication_lock_attachment_final_index_blocked == true
    and $claim.terminal_non_activation_release_claim_index_attachment_final_index_blocked == true
    and $operator.terminal_operator_readiness_non_approval_index_attachment_final_index_blocked == true
    and $operator.terminal_release_governance_final_audit_gate_invoked == false
    and $operator.terminal_release_artifact_non_write_lock_gate_invoked == false
    and $operator.terminal_public_distribution_non_publication_lock_gate_invoked == false
    and $operator.terminal_non_activation_release_claim_index_gate_invoked == false
    and $operator.terminal_operator_readiness_non_approval_index_gate_invoked == false
    and $operator.terminal_summary_gates_invoked == false
    and $operator.terminal_live_gates_invoked == false
    and $operator.canonical_gate_wrapper_invoked == false
    and $operator.wrapper_target_invoked == false
    and $operator.source_successor_consumer_cutover_allowed == false
    and $operator.source_canonical_governance_rollback_anchor == "current_canonical_consumer"
    and $operator.tool_execution_live_cutover_allowed == false
    and $operator.tool_execution_public_ga_allowed == false
    and $operator.public_distribution_publication_allowed == false
    and $operator.release_publication_allowed == false
    and $operator.release_artifact_write_allowed == false
    and $operator.public_release_claim_allowed == false
    and $operator.release_claim_index_persistence_allowed == false
    and $operator.package_or_release_write_allowed == false
    and $operator.operator_approval_recorded == false
    and $operator.operator_identity_accepted == false
    and $operator.rollback_execution_allowed == false
    and $operator.operator_readiness_index_persistence_allowed == false
    and $artifact.source_canonical_governance_tool_execution_closure_backfeed_ready == true
    and $artifact.source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
    and $artifact.source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
    and $artifact.source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
    and $artifact.source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
    and $artifact.source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
    and ($operator.side_effects | to_entries | all(.value == false))) as $closure_ready |
  {
    runtime:"hepta",
    surface:"terminal_release_governance_safe_chain_closure",
    plugin_id:$operator.plugin_id,
    status:(if $closure_ready then "ready_blocked" else "blocked" end),
    terminal_release_governance_safe_chain_closure_ready:$closure_ready,
    terminal_release_governance_safe_chain_closure_blocked:true,
    safe_chain_source_count:($safe_chain_sources | length),
    safe_chain_ready_source_count:$safe_chain_ready_source_count,
    safe_chain_sources:$safe_chain_sources,
    source_terminal_release_governance_attachment_final_index_ready:$governance.terminal_release_governance_attachment_final_index_ready,
    source_terminal_release_artifact_non_write_lock_attachment_final_index_ready:$artifact.terminal_release_artifact_non_write_lock_attachment_final_index_ready,
    source_terminal_public_distribution_non_publication_lock_attachment_final_index_ready:$distribution.terminal_public_distribution_non_publication_lock_attachment_final_index_ready,
    source_terminal_non_activation_release_claim_index_attachment_final_index_ready:$claim.terminal_non_activation_release_claim_index_attachment_final_index_ready,
    source_terminal_operator_readiness_non_approval_index_attachment_final_index_ready:$operator.terminal_operator_readiness_non_approval_index_attachment_final_index_ready,
    terminal_release_governance_final_audit_gate_invoked:false,
    terminal_release_artifact_non_write_lock_gate_invoked:false,
    terminal_public_distribution_non_publication_lock_gate_invoked:false,
    terminal_non_activation_release_claim_index_gate_invoked:false,
    terminal_operator_readiness_non_approval_index_gate_invoked:false,
    terminal_summary_gates_invoked:false,
    terminal_live_gates_invoked:false,
    canonical_gate_wrapper_invoked:false,
    wrapper_target_invoked:false,
    source_successor_consumer_cutover_allowed:false,
    source_canonical_governance_rollback_anchor:$operator.source_canonical_governance_rollback_anchor,
    source_canonical_governance_tool_execution_closure_backfeed_ready:$artifact.source_canonical_governance_tool_execution_closure_backfeed_ready,
    source_canonical_governance_tool_execution_closure_backfeed_blocker_count:$artifact.source_canonical_governance_tool_execution_closure_backfeed_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_count:$artifact.source_canonical_governance_tool_execution_closure_backfeed_category_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_ready_count:$artifact.source_canonical_governance_tool_execution_closure_backfeed_category_ready_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count:$artifact.source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_categorization_ready:$artifact.source_canonical_governance_tool_execution_closure_backfeed_categorization_ready,
    source_canonical_governance_tool_execution_closure_backfeed_categories:$artifact.source_canonical_governance_tool_execution_closure_backfeed_categories,
    closure_blocker_count:($closure_blockers | length),
    closure_blockers:$closure_blockers,
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
    next_migration_step:"derive_terminal_release_governance_safe_chain_closure_readback_without_operator_gate_invocation",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      terminal_release_governance_attachment_final_index_report:"scripts/hepta-systems-terminal-release-governance-attachment-final-index-report.sh",
      terminal_release_artifact_non_write_lock_attachment_final_index_report:"scripts/hepta-systems-terminal-release-artifact-non-write-lock-attachment-final-index-report.sh",
      terminal_public_distribution_non_publication_lock_attachment_final_index_report:"scripts/hepta-systems-terminal-public-distribution-non-publication-lock-attachment-final-index-report.sh",
      terminal_non_activation_release_claim_index_attachment_final_index_report:"scripts/hepta-systems-terminal-non-activation-release-claim-index-attachment-final-index-report.sh",
      terminal_operator_readiness_non_approval_index_attachment_final_index_report:"scripts/hepta-systems-terminal-operator-readiness-non-approval-index-attachment-final-index-report.sh"
    },
    side_effect_free:true,
    side_effects:$operator.side_effects
  }'
