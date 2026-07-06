#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
TERMINAL_DENIAL_FINAL_INDEX_REPORT="$ROOT/scripts/hepta-systems-terminal-denial-index-attachment-final-index-report.sh"
PUBLICATION_EVIDENCE_GATE="$ROOT/scripts/hepta-terminal-publication-evidence-non-persistence-summary-gate.sh"
PUBLICATION_EVIDENCE_DOC="$ROOT/docs/architecture/HEPTA_TERMINAL_PUBLICATION_EVIDENCE_NON_PERSISTENCE_SUMMARY_GATE.md"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TERMINAL_DENIAL_INDEX_FINAL_INDEX_TERMINAL_PUBLICATION_EVIDENCE_NON_PERSISTENCE_SUMMARY_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-denial-index-final-index-terminal-publication-evidence-non-persistence-summary-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$TERMINAL_DENIAL_FINAL_INDEX_REPORT" ]] || fail "missing executable terminal denial index attachment final index report: $TERMINAL_DENIAL_FINAL_INDEX_REPORT"
[[ -x "$PUBLICATION_EVIDENCE_GATE" ]] || fail "missing executable terminal publication evidence non-persistence summary gate: $PUBLICATION_EVIDENCE_GATE"
[[ -f "$PUBLICATION_EVIDENCE_DOC" ]] || fail "missing terminal publication evidence non-persistence summary doc: $PUBLICATION_EVIDENCE_DOC"
[[ -f "$DOC" ]] || fail "missing terminal publication evidence non-persistence summary attachment architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the terminal publication evidence non-persistence summary attachment report"
fi

jq -n \
  --slurpfile denial_final_index <("$TERMINAL_DENIAL_FINAL_INDEX_REPORT") \
  --arg gate "scripts/hepta-systems-terminal-denial-index-final-index-terminal-publication-evidence-non-persistence-summary-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_TERMINAL_DENIAL_INDEX_FINAL_INDEX_TERMINAL_PUBLICATION_EVIDENCE_NON_PERSISTENCE_SUMMARY_2026-06-21.md" \
  '
  ($denial_final_index[0]) as $source |
  ($source.side_effects + {
    terminal_publication_evidence_non_persistence_summary_gate_invoked:false,
    hepta_watchdog_invoked:false,
    publication_evidence_summary_recorded:false,
    publication_evidence_summary_persisted:false,
    publication_evidence_summary_materialized:false,
    publication_evidence_summary_filesystem_written:false,
    publication_evidence_receipt_persisted:false,
    publication_evidence_ledger_persisted:false,
    publication_evidence_external_send_performed:false,
    publication_evidence_public_claim_performed:false,
    publication_evidence_public_distribution_performed:false,
    publication_evidence_artifact_written:false
  }) as $side_effects |
  [
    "manual_operator_live_cutover_approval_required",
    "terminal_publication_evidence_non_persistence_summary_not_invoked",
    "terminal_denial_index_not_invoked",
    "terminal_denial_index_recording_disabled",
    "terminal_denial_index_persistence_disabled",
    "terminal_denial_index_materialization_disabled",
    "terminal_denial_index_filesystem_write_disabled",
    "publication_evidence_summary_recording_disabled",
    "publication_evidence_summary_persistence_disabled",
    "publication_evidence_summary_materialization_disabled",
    "publication_evidence_summary_filesystem_write_disabled",
    "publication_evidence_receipt_persistence_disabled",
    "publication_evidence_ledger_persistence_disabled",
    "publication_evidence_external_send_disabled",
    "publication_evidence_public_claim_disabled",
    "publication_evidence_public_distribution_disabled",
    "publication_evidence_artifact_write_disabled",
    "watchdog_not_invoked",
    "terminal_live_gates_not_invoked",
    "canonical_gate_not_invoked",
    "wrapper_target_not_invoked",
    "live_url_not_contacted",
    "long_soak_not_started",
    "public_ga_disabled",
    "operator_approval_not_recorded",
    "operator_identity_not_accepted",
    "rollback_execution_disabled",
    "public_release_claim_disabled"
  ] as $attachment_blockers |
  ($source.terminal_denial_index_attachment_final_index_ready == true
    and $source.terminal_denial_index_attachment_final_index_blocked == true
    and $source.terminal_denial_index_gate_invoked == false
    and $source.terminal_denial_index_recorded == false
    and $source.terminal_denial_index_persisted == false
    and $source.terminal_denial_index_materialized == false
    and $source.terminal_denial_index_filesystem_written == false
    and $source.terminal_summary_gates_invoked == false
    and $source.terminal_live_gates_invoked == false
    and $source.canonical_gate_wrapper_invoked == false
    and $source.wrapper_target_invoked == false
    and $source.tool_execution_live_cutover_allowed == false
    and $source.tool_execution_public_ga_allowed == false
    and $source.public_release_claim_allowed == false
    and $source.operator_approval_recorded == false
    and $source.operator_identity_accepted == false
    and $source.rollback_execution_allowed == false
    and ($side_effects | to_entries | all(.value == false))) as $attachment_ready |
  {
    runtime:"hepta",
    surface:"terminal_denial_index_final_index_terminal_publication_evidence_non_persistence_summary",
    plugin_id:$source.plugin_id,
    status:(if $attachment_ready then "ready_blocked" else "blocked" end),
    source_terminal_denial_index_attachment_final_index_surface:$source.surface,
    source_terminal_denial_index_attachment_final_index_ready:$source.terminal_denial_index_attachment_final_index_ready,
    source_terminal_denial_index_attachment_final_index_blocked:$source.terminal_denial_index_attachment_final_index_blocked,
    terminal_publication_evidence_non_persistence_summary_attachment_ready:$attachment_ready,
    terminal_publication_evidence_non_persistence_summary_attachment_blocked:true,
    terminal_denial_index_attachment_final_index_attached:true,
    terminal_publication_evidence_non_persistence_summary_gate_present:true,
    terminal_publication_evidence_non_persistence_summary_doc_present:true,
    terminal_publication_evidence_non_persistence_summary_gate_invoked:false,
    hepta_watchdog_invoked:false,
    terminal_denial_index_gate_invoked:false,
    terminal_denial_index_recorded:false,
    terminal_denial_index_persisted:false,
    terminal_denial_index_materialized:false,
    terminal_denial_index_filesystem_written:false,
    terminal_public_distribution_non_publication_lock_gate_invoked:false,
    terminal_summary_gates_invoked:false,
    terminal_live_gates_invoked:false,
    canonical_gate_wrapper_invoked:false,
    wrapper_target_invoked:false,
    attachment_blocker_count:($attachment_blockers | length),
    attachment_blockers:$attachment_blockers,
    manual_operator_live_cutover_approval_required:true,
    terminal_live_url_required:false,
    long_soak_required:false,
    tool_execution_live_cutover_allowed:false,
    tool_execution_public_ga_allowed:false,
    public_distribution_publication_allowed:false,
    public_distribution_artifact_write_allowed:false,
    public_release_claim_allowed:false,
    public_ga_claim_allowed:false,
    public_release_published:false,
    public_ga_claimed:false,
    external_public_claim_performed:false,
    external_public_distribution_performed:false,
    publication_evidence_summary_recorded:false,
    publication_evidence_summary_persisted:false,
    publication_evidence_summary_materialized:false,
    publication_evidence_summary_filesystem_written:false,
    publication_evidence_receipt_persisted:false,
    publication_evidence_ledger_persisted:false,
    publication_evidence_external_send_performed:false,
    publication_evidence_public_claim_performed:false,
    publication_evidence_public_distribution_performed:false,
    publication_evidence_artifact_written:false,
    operator_approval_recorded:false,
    operator_identity_accepted:false,
    rollback_execution_allowed:false,
    next_migration_step:"derive_terminal_publication_evidence_non_persistence_summary_attachment_readback_without_summary_gate_invocation",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      terminal_denial_index_attachment_final_index_report:"scripts/hepta-systems-terminal-denial-index-attachment-final-index-report.sh",
      terminal_publication_evidence_non_persistence_summary_gate:"scripts/hepta-terminal-publication-evidence-non-persistence-summary-gate.sh",
      terminal_publication_evidence_non_persistence_summary_doc:"docs/architecture/HEPTA_TERMINAL_PUBLICATION_EVIDENCE_NON_PERSISTENCE_SUMMARY_GATE.md"
    },
    side_effect_free:true,
    side_effects:$side_effects
  }'
