#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-audit-evidence-final-index-retention-expiry-gc-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-audit-evidence-final-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_AUDIT_EVIDENCE_FINAL_INDEX_RETENTION_EXPIRY_GC_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-audit-evidence-final-index-retention-expiry-gc-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Public GA operator identity/session retention expiry GC attachment report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable Public GA operator identity/session audit evidence final index gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing Public GA operator identity/session retention expiry GC architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Public GA operator identity/session retention expiry GC attachment report"
fi

grep -q 'Public GA Operator Identity/Session Retention Expiry GC Attachment' "$DOC" \
  || fail "architecture note must document Public GA Operator Identity/Session Retention Expiry GC Attachment"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that attachment does not invoke retention gates"
grep -q 'canonical terminal closure backfeed' "$DOC" \
  || fail "architecture note must document canonical terminal closure backfeed"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_retention_expiry_gc_attachment"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_audit_evidence_final_index_surface == "public_ga_operator_identity_session_audit_evidence_final_index"
  and .source_public_ga_operator_identity_session_audit_evidence_final_index_ready == true
  and .source_public_ga_operator_identity_session_audit_evidence_final_index_blocked == true
  and .public_ga_operator_identity_session_audit_evidence_final_index_attached == true
  and .public_ga_operator_identity_session_retention_expiry_gc_attachment_ready == true
  and .public_ga_operator_identity_session_retention_expiry_gc_attachment_blocked == true
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "runner_selector" and .blocker_count == 2)
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "dirty_worktree_owner_freeze" and .blocker_count == 2)
  and .operator_identity_session_retention_expiry_gc_denial_gate_present == true
  and .operator_identity_session_retention_expiry_gc_denial_doc_present == true
  and .operator_identity_session_retention_expiry_gc_static_mention_count >= 10
  and .operator_identity_session_retention_expiry_gc_denial_gate_invoked == false
  and .operator_identity_session_reinstatement_audit_evidence_denial_gate_invoked == false
  and .long_soak_required_by_source_retention_gate == true
  and .long_soak_started == false
  and .retention_policy_recorded == false
  and .retention_policy_persisted == false
  and .ttl_lease_recorded == false
  and .expiry_timestamp_recorded == false
  and .expiry_timer_started == false
  and .garbage_collection_queue_recorded == false
  and .garbage_collection_scan_performed == false
  and .garbage_collection_decision_recorded == false
  and .archive_recorded == false
  and .compaction_recorded == false
  and .retention_expiry_gc_authority_derived == false
  and .attachment_blocker_count == 38
  and .manual_operator_live_cutover_approval_required == true
  and .public_ga_claim_allowed == false
  and .public_ga_claimed == false
  and .public_release_published == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "derive_public_ga_operator_identity_session_retention_expiry_gc_readback_without_retention"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-audit-evidence-final-index-retention-expiry-gc-gate: PASS: Public GA operator identity/session retention expiry GC attachment is ready but blocked without evidence\n'
