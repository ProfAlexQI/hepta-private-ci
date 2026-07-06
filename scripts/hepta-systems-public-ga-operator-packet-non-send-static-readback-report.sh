#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
NON_SEND_READBACK_REPORT="$ROOT/scripts/hepta-systems-terminal-public-ga-final-index-public-ga-operator-packet-non-send-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_PACKET_NON_SEND_STATIC_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-packet-non-send-static-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$NON_SEND_READBACK_REPORT" ]] || fail "missing executable Public GA operator packet non-send readback report: $NON_SEND_READBACK_REPORT"
[[ -f "$DOC" ]] || fail "missing Public GA operator packet non-send static readback architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Public GA operator packet non-send static readback report"
fi

jq -n \
  --slurpfile source <("$NON_SEND_READBACK_REPORT") \
  --arg gate "scripts/hepta-systems-public-ga-operator-packet-non-send-static-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_PACKET_NON_SEND_STATIC_READBACK_2026-06-21.md" \
  '
  ($source[0]) as $source |
  ($source.public_ga_operator_packet_non_send_readback_ready == true
    and $source.public_ga_operator_packet_non_send_readback_blocked == true
    and $source.public_ga_operator_approval_packet_invoked == false
    and $source.public_ga_operator_compat_wrapper_invoked == false
    and $source.public_ga_operator_packet_live_endpoint_read_performed == false
    and $source.public_ga_operator_packet_endpoint_curl_performed == false
    and $source.public_ga_operator_packet_sent == false
    and $source.public_ga_operator_packet_recorded == false
    and $source.public_ga_operator_packet_accepted == false
    and $source.operator_approval_request_sent == false
    and $source.operator_approval_recorded == false
    and $source.public_ga_claim_allowed == false
    and $source.public_ga_claimed == false
    and $source.source_canonical_governance_tool_execution_closure_backfeed_ready == true
    and $source.source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
    and $source.source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
    and $source.source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
    and $source.source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
    and $source.source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
    and ($source.side_effects | to_entries | all(.value == false))) as $static_readback_ready |
  {
    runtime:"hepta",
    surface:"public_ga_operator_packet_non_send_static_readback",
    plugin_id:$source.plugin_id,
    status:(if $static_readback_ready then "ready_blocked" else "blocked" end),
    source_public_ga_operator_packet_non_send_readback_surface:$source.surface,
    source_public_ga_operator_packet_non_send_readback_ready:$source.public_ga_operator_packet_non_send_readback_ready,
    source_public_ga_operator_packet_non_send_readback_blocked:$source.public_ga_operator_packet_non_send_readback_blocked,
    source_canonical_governance_tool_execution_closure_backfeed_ready:$source.source_canonical_governance_tool_execution_closure_backfeed_ready,
    source_canonical_governance_tool_execution_closure_backfeed_blocker_count:$source.source_canonical_governance_tool_execution_closure_backfeed_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_count:$source.source_canonical_governance_tool_execution_closure_backfeed_category_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_ready_count:$source.source_canonical_governance_tool_execution_closure_backfeed_category_ready_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count:$source.source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_categorization_ready:$source.source_canonical_governance_tool_execution_closure_backfeed_categorization_ready,
    source_canonical_governance_tool_execution_closure_backfeed_categories:$source.source_canonical_governance_tool_execution_closure_backfeed_categories,
    public_ga_operator_packet_non_send_static_readback_ready:$static_readback_ready,
    public_ga_operator_packet_non_send_static_readback_blocked:true,
    public_ga_operator_packet_non_send_readback_attached:true,
    readback_mode:"static_public_ga_operator_packet_non_send_snapshot_only",
    readback_check_count:24,
    terminal_public_ga_non_promotion_summary_final_index_attached:true,
    public_ga_operator_approval_packet_present:true,
    public_ga_operator_compat_wrapper_present:true,
    public_ga_operator_approval_packet_doc_present:true,
    public_ga_operator_packet_target_curl_count:$source.public_ga_operator_packet_target_curl_count,
    public_ga_operator_packet_target_endpoint_count:$source.public_ga_operator_packet_target_endpoint_count,
    public_ga_operator_packet_required_approval_static_count:$source.public_ga_operator_packet_required_approval_static_count,
    public_ga_operator_compat_wrapper_exec_count:$source.public_ga_operator_compat_wrapper_exec_count,
    public_ga_operator_approval_packet_invoked:false,
    public_ga_operator_compat_wrapper_invoked:false,
    public_ga_operator_packet_live_endpoint_read_performed:false,
    public_ga_operator_packet_endpoint_curl_performed:false,
    public_ga_operator_packet_report_materialized:false,
    public_ga_operator_packet_sent:false,
    public_ga_operator_packet_recorded:false,
    public_ga_operator_packet_accepted:false,
    operator_approval_request_sent:false,
    operator_approval_recorded:false,
    operator_identity_accepted:false,
    public_ga_readiness_script_invoked:false,
    public_ga_readiness_live_endpoint_read_performed:false,
    public_ga_readiness_endpoint_curl_performed:false,
    public_ga_readiness_report_materialized:false,
    public_claim_non_promotion_denial_gate_invoked:false,
    terminal_publication_evidence_non_persistence_summary_gate_invoked:false,
    hepta_watchdog_invoked:false,
    terminal_public_distribution_non_publication_lock_gate_invoked:false,
    terminal_denial_index_gate_invoked:false,
    terminal_summary_gates_invoked:false,
    terminal_live_gates_invoked:false,
    canonical_gate_wrapper_invoked:false,
    wrapper_target_invoked:false,
    readback_blocker_count:$source.readback_blocker_count,
    readback_blockers:$source.readback_blockers,
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
    publication_evidence_summary_recorded:false,
    publication_evidence_summary_persisted:false,
    publication_evidence_receipt_persisted:false,
    publication_evidence_ledger_persisted:false,
    rollback_execution_allowed:false,
    next_migration_step:"derive_public_ga_operator_packet_non_send_readback_final_index_without_packet_invocation",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      public_ga_operator_packet_non_send_readback_report:"scripts/hepta-systems-terminal-public-ga-final-index-public-ga-operator-packet-non-send-readback-report.sh"
    },
    side_effect_free:true,
    side_effects:$source.side_effects
  }'
