#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-terminal-public-distribution-non-publication-lock-attachment-final-index-report.sh"
READBACK_GATE="$ROOT/scripts/hepta-systems-terminal-public-distribution-non-publication-lock-attachment-readback-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TERMINAL_PUBLIC_DISTRIBUTION_NON_PUBLICATION_LOCK_ATTACHMENT_FINAL_INDEX_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-public-distribution-non-publication-lock-attachment-final-index-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable terminal public distribution non-publication lock attachment final index report: $REPORT"
[[ -x "$READBACK_GATE" ]] || fail "missing executable terminal public distribution non-publication lock attachment readback gate: $READBACK_GATE"
[[ -f "$DOC" ]] || fail "missing terminal public distribution non-publication lock attachment final index architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the terminal public distribution non-publication lock attachment final index report"
fi

grep -q 'Terminal Public Distribution Non-Publication Lock Attachment Final Index' "$DOC" \
  || fail "architecture note must document Terminal Public Distribution Non-Publication Lock Attachment Final Index"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that final index does not invoke distribution or live gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "terminal_public_distribution_non_publication_lock_attachment_final_index"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_terminal_public_distribution_non_publication_lock_attachment_readback_surface == "terminal_public_distribution_non_publication_lock_attachment_readback"
  and .source_terminal_public_distribution_non_publication_lock_attachment_readback_ready == true
  and .source_terminal_public_distribution_non_publication_lock_attachment_readback_blocked == true
  and .terminal_public_distribution_non_publication_lock_attachment_final_index_ready == true
  and .terminal_public_distribution_non_publication_lock_attachment_final_index_blocked == true
  and .terminal_release_artifact_non_write_lock_attachment_final_index_attached == true
  and .terminal_public_distribution_non_publication_lock_gate_present == true
  and .terminal_public_distribution_non_publication_lock_gate_invoked == false
  and .terminal_release_artifact_non_write_lock_gate_invoked == false
  and .terminal_release_governance_final_audit_gate_invoked == false
  and .terminal_summary_gates_invoked == false
  and .terminal_live_gates_invoked == false
  and .canonical_gate_wrapper_invoked == false
  and .wrapper_target_invoked == false
  and .source_successor_consumer_cutover_allowed == false
  and .source_canonical_governance_rollback_anchor == "current_canonical_consumer"
  and .final_blocker_count == 18
  and (.final_blockers | index("terminal_public_distribution_non_publication_lock_not_invoked")) != null
  and (.final_blockers | index("public_distribution_publication_disabled")) != null
  and (.final_blockers | index("public_release_claim_disabled")) != null
  and .manual_operator_live_cutover_approval_required == true
  and .terminal_live_url_required == false
  and .long_soak_required == false
  and .tool_execution_live_cutover_allowed == false
  and .tool_execution_public_ga_allowed == false
  and .public_distribution_publication_allowed == false
  and .release_publication_allowed == false
  and .release_artifact_write_allowed == false
  and .public_release_claim_allowed == false
  and .package_or_release_write_allowed == false
  and .next_migration_step == "attach_terminal_public_distribution_non_publication_lock_attachment_final_index_to_terminal_non_activation_release_claim_index_without_distribution_gate_invocation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$READBACK_GATE" >/dev/null

printf 'hepta-systems-terminal-public-distribution-non-publication-lock-attachment-final-index-gate: PASS: terminal public distribution non-publication lock attachment final index is ready but blocked without distribution/release/live invocation\n'
