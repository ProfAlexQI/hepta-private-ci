#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
PUBLIC_DISTRIBUTION_FINAL_INDEX_REPORT="$ROOT/scripts/hepta-systems-terminal-public-distribution-non-publication-lock-attachment-final-index-report.sh"
PUBLIC_DISTRIBUTION_FINAL_INDEX_GATE="$ROOT/scripts/hepta-systems-terminal-public-distribution-non-publication-lock-attachment-final-index-gate.sh"
NON_ACTIVATION_RELEASE_CLAIM_GATE="$ROOT/scripts/hepta-terminal-non-activation-release-claim-index-gate.sh"
NON_ACTIVATION_RELEASE_CLAIM_DOC="$ROOT/docs/architecture/HEPTA_TERMINAL_NON_ACTIVATION_RELEASE_CLAIM_INDEX_GATE.md"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TERMINAL_PUBLIC_DISTRIBUTION_FINAL_INDEX_TERMINAL_NON_ACTIVATION_RELEASE_CLAIM_INDEX_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-public-distribution-final-index-terminal-non-activation-release-claim-index-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$PUBLIC_DISTRIBUTION_FINAL_INDEX_REPORT" ]] || fail "missing executable terminal public distribution non-publication lock attachment final index report: $PUBLIC_DISTRIBUTION_FINAL_INDEX_REPORT"
[[ -x "$PUBLIC_DISTRIBUTION_FINAL_INDEX_GATE" ]] || fail "missing executable terminal public distribution non-publication lock attachment final index gate: $PUBLIC_DISTRIBUTION_FINAL_INDEX_GATE"
[[ -x "$NON_ACTIVATION_RELEASE_CLAIM_GATE" ]] || fail "missing executable terminal non-activation release claim index gate: $NON_ACTIVATION_RELEASE_CLAIM_GATE"
[[ -f "$NON_ACTIVATION_RELEASE_CLAIM_DOC" ]] || fail "missing terminal non-activation release claim index doc: $NON_ACTIVATION_RELEASE_CLAIM_DOC"
[[ -f "$DOC" ]] || fail "missing terminal non-activation release claim attachment architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the terminal non-activation release claim attachment report"
fi

jq -n \
  --slurpfile public_distribution_final_index <("$PUBLIC_DISTRIBUTION_FINAL_INDEX_REPORT") \
  --arg gate "scripts/hepta-systems-terminal-public-distribution-final-index-terminal-non-activation-release-claim-index-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_TERMINAL_PUBLIC_DISTRIBUTION_FINAL_INDEX_TERMINAL_NON_ACTIVATION_RELEASE_CLAIM_INDEX_2026-06-21.md" \
  '
  ($public_distribution_final_index[0]) as $public |
  [
    "manual_operator_live_cutover_approval_required",
    "terminal_non_activation_release_claim_index_source_probed_not_invoked",
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
    "public_distribution_publication_disabled",
    "release_publication_disabled",
    "release_artifact_write_disabled",
    "public_release_claim_disabled",
    "package_or_release_write_disabled",
    "release_claim_index_persistence_disabled"
  ] as $attachment_blockers |
  ($public.terminal_public_distribution_non_publication_lock_attachment_final_index_ready == true
    and $public.terminal_public_distribution_non_publication_lock_attachment_final_index_blocked == true
    and $public.terminal_public_distribution_non_publication_lock_gate_invoked == false
    and $public.terminal_release_artifact_non_write_lock_gate_invoked == false
    and $public.terminal_release_governance_final_audit_gate_invoked == false
    and $public.terminal_summary_gates_invoked == false
    and $public.terminal_live_gates_invoked == false
    and $public.canonical_gate_wrapper_invoked == false
    and $public.wrapper_target_invoked == false
    and $public.source_successor_consumer_cutover_allowed == false
    and $public.source_canonical_governance_rollback_anchor == "current_canonical_consumer"
    and $public.tool_execution_live_cutover_allowed == false
    and $public.tool_execution_public_ga_allowed == false
    and $public.public_distribution_publication_allowed == false
    and $public.release_publication_allowed == false
    and $public.release_artifact_write_allowed == false
    and $public.public_release_claim_allowed == false
    and $public.package_or_release_write_allowed == false
    and ($public.side_effects | to_entries | all(.value == false))) as $attachment_ready |
  {
    runtime:"hepta",
    surface:"terminal_public_distribution_final_index_terminal_non_activation_release_claim_index",
    plugin_id:$public.plugin_id,
    status:(if $attachment_ready then "ready_blocked" else "blocked" end),
    source_terminal_public_distribution_non_publication_lock_attachment_final_index_surface:$public.surface,
    source_terminal_public_distribution_non_publication_lock_attachment_final_index_ready:$public.terminal_public_distribution_non_publication_lock_attachment_final_index_ready,
    source_terminal_public_distribution_non_publication_lock_attachment_final_index_blocked:$public.terminal_public_distribution_non_publication_lock_attachment_final_index_blocked,
    terminal_non_activation_release_claim_index_attachment_ready:$attachment_ready,
    terminal_non_activation_release_claim_index_attachment_blocked:true,
    terminal_public_distribution_non_publication_lock_attachment_final_index_attached:true,
    terminal_non_activation_release_claim_index_gate_present:true,
    terminal_non_activation_release_claim_index_doc_present:true,
    terminal_non_activation_release_claim_index_gate_invoked:false,
    terminal_public_distribution_non_publication_lock_gate_invoked:false,
    terminal_release_artifact_non_write_lock_gate_invoked:false,
    terminal_release_governance_final_audit_gate_invoked:false,
    terminal_summary_gates_invoked:false,
    terminal_live_gates_invoked:false,
    canonical_gate_wrapper_invoked:false,
    wrapper_target_invoked:false,
    source_successor_consumer_cutover_allowed:false,
    source_canonical_governance_rollback_anchor:$public.source_canonical_governance_rollback_anchor,
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
    release_claim_index_persistence_allowed:false,
    package_or_release_write_allowed:false,
    next_migration_step:"derive_terminal_non_activation_release_claim_index_attachment_readback_without_release_claim_gate_invocation",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      terminal_public_distribution_non_publication_lock_attachment_final_index_report:"scripts/hepta-systems-terminal-public-distribution-non-publication-lock-attachment-final-index-report.sh",
      terminal_public_distribution_non_publication_lock_attachment_final_index_gate:"scripts/hepta-systems-terminal-public-distribution-non-publication-lock-attachment-final-index-gate.sh",
      terminal_non_activation_release_claim_index_gate:"scripts/hepta-terminal-non-activation-release-claim-index-gate.sh",
      terminal_non_activation_release_claim_index_doc:"docs/architecture/HEPTA_TERMINAL_NON_ACTIVATION_RELEASE_CLAIM_INDEX_GATE.md"
    },
    side_effect_free:true,
    side_effects:$public.side_effects
  }'
