#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-terminal-release-artifact-non-write-lock-attachment-readback-report.sh"
ATTACHMENT_GATE="$ROOT/scripts/hepta-systems-terminal-release-governance-final-index-terminal-release-artifact-non-write-lock-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TERMINAL_RELEASE_ARTIFACT_NON_WRITE_LOCK_ATTACHMENT_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-release-artifact-non-write-lock-attachment-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable terminal release artifact non-write lock attachment readback report: $REPORT"
[[ -x "$ATTACHMENT_GATE" ]] || fail "missing executable terminal release artifact non-write lock attachment gate: $ATTACHMENT_GATE"
[[ -f "$DOC" ]] || fail "missing terminal release artifact non-write lock attachment readback architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the terminal release artifact non-write lock attachment readback report"
fi

grep -q 'Terminal Release Artifact Non-Write Lock Attachment Readback' "$DOC" \
  || fail "architecture note must document Terminal Release Artifact Non-Write Lock Attachment Readback"
grep -q 'static terminal release artifact non-write lock attachment snapshot' "$DOC" \
  || fail "architecture note must document static terminal release artifact non-write lock attachment snapshot readback"
grep -q 'canonical terminal closure backfeed' "$DOC" \
  || fail "architecture note must document canonical terminal closure backfeed"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that readback does not invoke artifact or live gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "terminal_release_artifact_non_write_lock_attachment_readback"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .readback_mode == "static_terminal_release_artifact_non_write_lock_attachment_snapshot_only"
  and .source_terminal_release_artifact_non_write_lock_attachment_surface == "terminal_release_governance_final_index_terminal_release_artifact_non_write_lock"
  and .source_terminal_release_artifact_non_write_lock_attachment_report_reexecuted == false
  and .source_terminal_release_artifact_non_write_lock_attachment_ready == true
  and .source_terminal_release_artifact_non_write_lock_attachment_blocked == true
  and .terminal_release_artifact_non_write_lock_attachment_readback_ready == true
  and .terminal_release_artifact_non_write_lock_attachment_readback_blocked == true
  and .readback_check_count == 18
  and .terminal_release_artifact_non_write_lock_gate_present == true
  and .terminal_release_artifact_non_write_lock_doc_present == true
  and .terminal_release_artifact_non_write_lock_gate_invoked == false
  and .terminal_release_governance_final_audit_gate_invoked == false
  and .terminal_summary_gates_invoked == false
  and .terminal_live_gates_invoked == false
  and .canonical_gate_wrapper_invoked == false
  and .wrapper_target_invoked == false
  and .source_successor_consumer_cutover_allowed == false
  and .source_canonical_governance_rollback_anchor == "current_canonical_consumer"
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
  and (.source_canonical_governance_tool_execution_closure_backfeed_categories | length) == 4
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "runner_selector" and .blocker_count == 2)
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "dirty_worktree_owner_freeze" and .blocker_count == 2)
  and .manual_operator_live_cutover_approval_required == true
  and .terminal_live_url_required == false
  and .long_soak_required == false
  and .tool_execution_live_cutover_allowed == false
  and .tool_execution_public_ga_allowed == false
  and .release_publication_allowed == false
  and .release_artifact_write_allowed == false
  and .public_release_claim_allowed == false
  and .package_or_release_write_allowed == false
  and .next_migration_step == "derive_terminal_release_artifact_non_write_lock_attachment_final_index_without_artifact_gate_invocation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$ATTACHMENT_GATE" >/dev/null

printf 'hepta-systems-terminal-release-artifact-non-write-lock-attachment-readback-gate: PASS: terminal release artifact non-write lock attachment readback is static and blocks artifact/release/live invocation\n'
