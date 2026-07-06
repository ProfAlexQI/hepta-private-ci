#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-terminal-denial-index-final-index-terminal-publication-evidence-non-persistence-summary-report.sh"
TERMINAL_DENIAL_FINAL_INDEX_GATE="$ROOT/scripts/hepta-systems-terminal-denial-index-attachment-final-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TERMINAL_DENIAL_INDEX_FINAL_INDEX_TERMINAL_PUBLICATION_EVIDENCE_NON_PERSISTENCE_SUMMARY_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-denial-index-final-index-terminal-publication-evidence-non-persistence-summary-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable terminal publication evidence non-persistence summary attachment report: $REPORT"
[[ -x "$TERMINAL_DENIAL_FINAL_INDEX_GATE" ]] || fail "missing executable terminal denial index attachment final index gate: $TERMINAL_DENIAL_FINAL_INDEX_GATE"
[[ -f "$DOC" ]] || fail "missing terminal publication evidence non-persistence summary attachment architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the terminal publication evidence non-persistence summary attachment report"
fi

grep -q 'Terminal Denial Index Final Index To Terminal Publication Evidence Non-Persistence Summary' "$DOC" \
  || fail "architecture note must document Terminal Denial Index Final Index To Terminal Publication Evidence Non-Persistence Summary"
grep -q 'source-probe' "$DOC" \
  || fail "architecture note must document source-probe mode"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that attachment does not invoke publication evidence or live gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "terminal_denial_index_final_index_terminal_publication_evidence_non_persistence_summary"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_terminal_denial_index_attachment_final_index_surface == "terminal_denial_index_attachment_final_index"
  and .source_terminal_denial_index_attachment_final_index_ready == true
  and .source_terminal_denial_index_attachment_final_index_blocked == true
  and .terminal_publication_evidence_non_persistence_summary_attachment_ready == true
  and .terminal_publication_evidence_non_persistence_summary_attachment_blocked == true
  and .terminal_denial_index_attachment_final_index_attached == true
  and .terminal_publication_evidence_non_persistence_summary_gate_present == true
  and .terminal_publication_evidence_non_persistence_summary_doc_present == true
  and .terminal_publication_evidence_non_persistence_summary_gate_invoked == false
  and .hepta_watchdog_invoked == false
  and .terminal_denial_index_gate_invoked == false
  and .terminal_denial_index_recorded == false
  and .terminal_denial_index_persisted == false
  and .terminal_denial_index_materialized == false
  and .terminal_denial_index_filesystem_written == false
  and .terminal_public_distribution_non_publication_lock_gate_invoked == false
  and .terminal_summary_gates_invoked == false
  and .terminal_live_gates_invoked == false
  and .canonical_gate_wrapper_invoked == false
  and .wrapper_target_invoked == false
  and .attachment_blocker_count == 28
  and (.attachment_blockers | index("terminal_publication_evidence_non_persistence_summary_not_invoked")) != null
  and (.attachment_blockers | index("publication_evidence_summary_persistence_disabled")) != null
  and (.attachment_blockers | index("publication_evidence_public_claim_disabled")) != null
  and (.attachment_blockers | index("watchdog_not_invoked")) != null
  and .manual_operator_live_cutover_approval_required == true
  and .terminal_live_url_required == false
  and .long_soak_required == false
  and .tool_execution_live_cutover_allowed == false
  and .tool_execution_public_ga_allowed == false
  and .public_distribution_publication_allowed == false
  and .public_distribution_artifact_write_allowed == false
  and .public_release_claim_allowed == false
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .public_ga_claimed == false
  and .external_public_claim_performed == false
  and .external_public_distribution_performed == false
  and .publication_evidence_summary_recorded == false
  and .publication_evidence_summary_persisted == false
  and .publication_evidence_summary_materialized == false
  and .publication_evidence_summary_filesystem_written == false
  and .publication_evidence_receipt_persisted == false
  and .publication_evidence_ledger_persisted == false
  and .publication_evidence_external_send_performed == false
  and .publication_evidence_public_claim_performed == false
  and .publication_evidence_public_distribution_performed == false
  and .publication_evidence_artifact_written == false
  and .operator_approval_recorded == false
  and .operator_identity_accepted == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "derive_terminal_publication_evidence_non_persistence_summary_attachment_readback_without_summary_gate_invocation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$TERMINAL_DENIAL_FINAL_INDEX_GATE" >/dev/null

printf 'hepta-systems-terminal-denial-index-final-index-terminal-publication-evidence-non-persistence-summary-gate: PASS: terminal publication evidence non-persistence summary is source-probed without publication/live invocation\n'
