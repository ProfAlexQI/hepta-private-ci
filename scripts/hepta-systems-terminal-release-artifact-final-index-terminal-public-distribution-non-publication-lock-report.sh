#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
ARTIFACT_FINAL_INDEX_REPORT="$ROOT/scripts/hepta-systems-terminal-release-artifact-non-write-lock-attachment-final-index-report.sh"
ARTIFACT_FINAL_INDEX_GATE="$ROOT/scripts/hepta-systems-terminal-release-artifact-non-write-lock-attachment-final-index-gate.sh"
PUBLIC_DISTRIBUTION_LOCK_GATE="$ROOT/scripts/hepta-terminal-public-distribution-non-publication-lock-gate.sh"
PUBLIC_DISTRIBUTION_LOCK_DOC="$ROOT/docs/architecture/HEPTA_TERMINAL_PUBLIC_DISTRIBUTION_NON_PUBLICATION_LOCK_GATE.md"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TERMINAL_RELEASE_ARTIFACT_FINAL_INDEX_TERMINAL_PUBLIC_DISTRIBUTION_NON_PUBLICATION_LOCK_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-release-artifact-final-index-terminal-public-distribution-non-publication-lock-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$ARTIFACT_FINAL_INDEX_REPORT" ]] || fail "missing executable terminal release artifact non-write lock attachment final index report: $ARTIFACT_FINAL_INDEX_REPORT"
[[ -x "$ARTIFACT_FINAL_INDEX_GATE" ]] || fail "missing executable terminal release artifact non-write lock attachment final index gate: $ARTIFACT_FINAL_INDEX_GATE"
[[ -x "$PUBLIC_DISTRIBUTION_LOCK_GATE" ]] || fail "missing executable terminal public distribution non-publication lock gate: $PUBLIC_DISTRIBUTION_LOCK_GATE"
[[ -f "$PUBLIC_DISTRIBUTION_LOCK_DOC" ]] || fail "missing terminal public distribution non-publication lock doc: $PUBLIC_DISTRIBUTION_LOCK_DOC"
[[ -f "$DOC" ]] || fail "missing terminal public distribution non-publication lock attachment architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the terminal public distribution non-publication lock attachment report"
fi

jq -n \
  --slurpfile artifact_final_index <("$ARTIFACT_FINAL_INDEX_REPORT") \
  --arg gate "scripts/hepta-systems-terminal-release-artifact-final-index-terminal-public-distribution-non-publication-lock-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_TERMINAL_RELEASE_ARTIFACT_FINAL_INDEX_TERMINAL_PUBLIC_DISTRIBUTION_NON_PUBLICATION_LOCK_2026-06-21.md" \
  '
  ($artifact_final_index[0]) as $artifact |
  [
    "manual_operator_live_cutover_approval_required",
    "terminal_public_distribution_non_publication_lock_source_probed_not_invoked",
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
    "public_distribution_publication_disabled",
    "release_publication_disabled",
    "release_artifact_write_disabled",
    "public_release_claim_disabled",
    "package_or_release_write_disabled"
  ] as $attachment_blockers |
  ($artifact.terminal_release_artifact_non_write_lock_attachment_final_index_ready == true
    and $artifact.terminal_release_artifact_non_write_lock_attachment_final_index_blocked == true
    and $artifact.terminal_release_artifact_non_write_lock_gate_invoked == false
    and $artifact.terminal_release_governance_final_audit_gate_invoked == false
    and $artifact.terminal_summary_gates_invoked == false
    and $artifact.terminal_live_gates_invoked == false
    and $artifact.canonical_gate_wrapper_invoked == false
    and $artifact.wrapper_target_invoked == false
    and $artifact.source_successor_consumer_cutover_allowed == false
    and $artifact.source_canonical_governance_rollback_anchor == "current_canonical_consumer"
    and $artifact.tool_execution_live_cutover_allowed == false
    and $artifact.tool_execution_public_ga_allowed == false
    and $artifact.release_publication_allowed == false
    and $artifact.release_artifact_write_allowed == false
    and $artifact.public_release_claim_allowed == false
    and $artifact.package_or_release_write_allowed == false
    and ($artifact.side_effects | to_entries | all(.value == false))) as $attachment_ready |
  {
    runtime:"hepta",
    surface:"terminal_release_artifact_final_index_terminal_public_distribution_non_publication_lock",
    plugin_id:$artifact.plugin_id,
    status:(if $attachment_ready then "ready_blocked" else "blocked" end),
    source_terminal_release_artifact_non_write_lock_attachment_final_index_surface:$artifact.surface,
    source_terminal_release_artifact_non_write_lock_attachment_final_index_ready:$artifact.terminal_release_artifact_non_write_lock_attachment_final_index_ready,
    source_terminal_release_artifact_non_write_lock_attachment_final_index_blocked:$artifact.terminal_release_artifact_non_write_lock_attachment_final_index_blocked,
    terminal_public_distribution_non_publication_lock_attachment_ready:$attachment_ready,
    terminal_public_distribution_non_publication_lock_attachment_blocked:true,
    terminal_release_artifact_non_write_lock_attachment_final_index_attached:true,
    terminal_public_distribution_non_publication_lock_gate_present:true,
    terminal_public_distribution_non_publication_lock_doc_present:true,
    terminal_public_distribution_non_publication_lock_gate_invoked:false,
    terminal_release_artifact_non_write_lock_gate_invoked:false,
    terminal_release_governance_final_audit_gate_invoked:false,
    terminal_summary_gates_invoked:false,
    terminal_live_gates_invoked:false,
    canonical_gate_wrapper_invoked:false,
    wrapper_target_invoked:false,
    source_successor_consumer_cutover_allowed:false,
    source_canonical_governance_rollback_anchor:$artifact.source_canonical_governance_rollback_anchor,
    attachment_blocker_count:($attachment_blockers | length),
    attachment_blockers:$attachment_blockers,
    manual_operator_live_cutover_approval_required:true,
    terminal_live_url_required:false,
    long_soak_required:false,
    tool_execution_live_cutover_allowed:false,
    tool_execution_public_ga_allowed:false,
    public_distribution_publication_allowed:false,
    release_publication_allowed:false,
    release_artifact_write_allowed:false,
    public_release_claim_allowed:false,
    package_or_release_write_allowed:false,
    next_migration_step:"derive_terminal_public_distribution_non_publication_lock_attachment_readback_without_distribution_gate_invocation",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      terminal_release_artifact_non_write_lock_attachment_final_index_report:"scripts/hepta-systems-terminal-release-artifact-non-write-lock-attachment-final-index-report.sh",
      terminal_release_artifact_non_write_lock_attachment_final_index_gate:"scripts/hepta-systems-terminal-release-artifact-non-write-lock-attachment-final-index-gate.sh",
      terminal_public_distribution_non_publication_lock_gate:"scripts/hepta-terminal-public-distribution-non-publication-lock-gate.sh",
      terminal_public_distribution_non_publication_lock_doc:"docs/architecture/HEPTA_TERMINAL_PUBLIC_DISTRIBUTION_NON_PUBLICATION_LOCK_GATE.md"
    },
    side_effect_free:true,
    side_effects:$artifact.side_effects
  }'
