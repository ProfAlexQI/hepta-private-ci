#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-terminal-publication-evidence-non-persistence-summary-attachment-readback-report.sh"
ATTACHMENT_GATE="$ROOT/scripts/hepta-systems-terminal-denial-index-final-index-terminal-publication-evidence-non-persistence-summary-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TERMINAL_PUBLICATION_EVIDENCE_NON_PERSISTENCE_SUMMARY_ATTACHMENT_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-publication-evidence-non-persistence-summary-attachment-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable terminal publication evidence non-persistence summary attachment readback report: $REPORT"
[[ -x "$ATTACHMENT_GATE" ]] || fail "missing executable terminal publication evidence non-persistence summary attachment gate: $ATTACHMENT_GATE"
[[ -f "$DOC" ]] || fail "missing terminal publication evidence non-persistence summary attachment readback architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the terminal publication evidence non-persistence summary attachment readback report"
fi

grep -q 'Terminal Publication Evidence Non-Persistence Summary Attachment Readback' "$DOC" \
  || fail "architecture note must document Terminal Publication Evidence Non-Persistence Summary Attachment Readback"
grep -q 'static readback' "$DOC" \
  || fail "architecture note must document static readback"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that readback does not invoke publication evidence or live gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "terminal_publication_evidence_non_persistence_summary_attachment_readback"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .readback_mode == "static_terminal_publication_evidence_non_persistence_summary_attachment_snapshot_only"
  and .source_terminal_publication_evidence_non_persistence_summary_attachment_surface == "terminal_denial_index_final_index_terminal_publication_evidence_non_persistence_summary"
  and .source_terminal_publication_evidence_non_persistence_summary_attachment_report_reexecuted == false
  and .source_terminal_publication_evidence_non_persistence_summary_attachment_ready == true
  and .source_terminal_publication_evidence_non_persistence_summary_attachment_blocked == true
  and .terminal_publication_evidence_non_persistence_summary_attachment_readback_ready == true
  and .terminal_publication_evidence_non_persistence_summary_attachment_readback_blocked == true
  and .readback_check_count == 25
  and .terminal_publication_evidence_non_persistence_summary_gate_present == true
  and .terminal_publication_evidence_non_persistence_summary_doc_present == true
  and .terminal_publication_evidence_non_persistence_summary_gate_invoked == false
  and .hepta_watchdog_invoked == false
  and .terminal_denial_index_gate_invoked == false
  and .terminal_public_distribution_non_publication_lock_gate_invoked == false
  and .terminal_live_gates_invoked == false
  and .canonical_gate_wrapper_invoked == false
  and .wrapper_target_invoked == false
  and .attachment_blocker_count == 28
  and .public_distribution_publication_allowed == false
  and .public_distribution_artifact_write_allowed == false
  and .public_release_claim_allowed == false
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .public_ga_claimed == false
  and .publication_evidence_summary_recorded == false
  and .publication_evidence_summary_persisted == false
  and .publication_evidence_summary_materialized == false
  and .publication_evidence_summary_filesystem_written == false
  and .publication_evidence_receipt_persisted == false
  and .publication_evidence_ledger_persisted == false
  and .operator_approval_recorded == false
  and .operator_identity_accepted == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "derive_terminal_publication_evidence_non_persistence_summary_attachment_final_index_without_summary_gate_invocation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$ATTACHMENT_GATE" >/dev/null

printf 'hepta-systems-terminal-publication-evidence-non-persistence-summary-attachment-readback-gate: PASS: terminal publication evidence attachment readback is static without publication/live invocation\n'
