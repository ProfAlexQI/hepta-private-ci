#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-terminal-non-activation-release-claim-index-attachment-readback-report.sh"
ATTACHMENT_GATE="$ROOT/scripts/hepta-systems-terminal-public-distribution-final-index-terminal-non-activation-release-claim-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TERMINAL_NON_ACTIVATION_RELEASE_CLAIM_INDEX_ATTACHMENT_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-non-activation-release-claim-index-attachment-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable terminal non-activation release claim attachment readback report: $REPORT"
[[ -x "$ATTACHMENT_GATE" ]] || fail "missing executable terminal non-activation release claim attachment gate: $ATTACHMENT_GATE"
[[ -f "$DOC" ]] || fail "missing terminal non-activation release claim attachment readback architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the terminal non-activation release claim attachment readback report"
fi

grep -q 'Terminal Non-Activation Release Claim Index Attachment Readback' "$DOC" \
  || fail "architecture note must document Terminal Non-Activation Release Claim Index Attachment Readback"
grep -q 'static terminal non-activation release claim index attachment snapshot' "$DOC" \
  || fail "architecture note must document static terminal non-activation release claim index attachment snapshot readback"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that readback does not invoke release claim or live gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "terminal_non_activation_release_claim_index_attachment_readback"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .readback_mode == "static_terminal_non_activation_release_claim_index_attachment_snapshot_only"
  and .source_terminal_non_activation_release_claim_index_attachment_surface == "terminal_public_distribution_final_index_terminal_non_activation_release_claim_index"
  and .source_terminal_non_activation_release_claim_index_attachment_report_reexecuted == false
  and .source_terminal_non_activation_release_claim_index_attachment_ready == true
  and .source_terminal_non_activation_release_claim_index_attachment_blocked == true
  and .terminal_non_activation_release_claim_index_attachment_readback_ready == true
  and .terminal_non_activation_release_claim_index_attachment_readback_blocked == true
  and .readback_check_count == 16
  and .terminal_non_activation_release_claim_index_gate_present == true
  and .terminal_non_activation_release_claim_index_doc_present == true
  and .terminal_non_activation_release_claim_index_gate_invoked == false
  and .terminal_public_distribution_non_publication_lock_gate_invoked == false
  and .terminal_release_artifact_non_write_lock_gate_invoked == false
  and .terminal_release_governance_final_audit_gate_invoked == false
  and .terminal_summary_gates_invoked == false
  and .terminal_live_gates_invoked == false
  and .canonical_gate_wrapper_invoked == false
  and .wrapper_target_invoked == false
  and .source_successor_consumer_cutover_allowed == false
  and .source_canonical_governance_rollback_anchor == "current_canonical_consumer"
  and .manual_operator_live_cutover_approval_required == true
  and .terminal_live_url_required == false
  and .long_soak_required == false
  and .tool_execution_live_cutover_allowed == false
  and .tool_execution_public_ga_allowed == false
  and .public_distribution_publication_allowed == false
  and .release_publication_allowed == false
  and .release_artifact_write_allowed == false
  and .public_release_claim_allowed == false
  and .release_claim_index_persistence_allowed == false
  and .package_or_release_write_allowed == false
  and .next_migration_step == "derive_terminal_non_activation_release_claim_index_attachment_final_index_without_release_claim_gate_invocation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$ATTACHMENT_GATE" >/dev/null

printf 'hepta-systems-terminal-non-activation-release-claim-index-attachment-readback-gate: PASS: terminal non-activation release claim attachment readback is static and blocks claim/distribution/release/live invocation\n'
