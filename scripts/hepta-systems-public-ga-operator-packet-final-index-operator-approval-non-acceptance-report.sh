#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-packet-non-send-readback-final-index-report.sh"
NON_ACCEPTANCE_GATE="$ROOT/scripts/hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-non-acceptance-gate.sh"
NON_ACCEPTANCE_DOC="$ROOT/docs/architecture/HEPTA_CORE_ACTIVATION_OPERATOR_APPROVAL_GAP_LEDGER_SUMMARY_BRIEFING_ACKNOWLEDGEMENT_NON_ACCEPTANCE_GATE.md"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable Public GA operator packet non-send final index report: $SOURCE_REPORT" >&2
  exit 1
}
[[ -f "$NON_ACCEPTANCE_GATE" ]] || {
  echo "missing operator approval acknowledgement non-acceptance gate: $NON_ACCEPTANCE_GATE" >&2
  exit 1
}
[[ -f "$NON_ACCEPTANCE_DOC" ]] || {
  echo "missing operator approval acknowledgement non-acceptance doc: $NON_ACCEPTANCE_DOC" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the Public GA operator approval non-acceptance attachment report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_packet_non_send_readback_final_index"
  and .public_ga_operator_packet_non_send_readback_final_index_ready == true
  and .public_ga_operator_packet_non_send_readback_final_index_blocked == true
  and .public_ga_operator_approval_packet_invoked == false
  and .public_ga_operator_packet_sent == false
  and .operator_approval_request_sent == false
  and .operator_approval_recorded == false
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
' <<<"$source_json" >/dev/null

non_acceptance_static_mention_count="$(
  grep -Ec 'operator_approval_(recorded|accepted)|operator_identity_accepted|external_send_performed|telegram_send_performed|acknowledgement_accepted' "$NON_ACCEPTANCE_GATE" || true
)"

jq -n \
  --argjson source "$source_json" \
  --argjson non_acceptance_static_mention_count "$non_acceptance_static_mention_count" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_approval_non_acceptance_attachment",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_packet_non_send_readback_final_index_surface: $source.surface,
    source_public_ga_operator_packet_non_send_readback_final_index_ready: $source.public_ga_operator_packet_non_send_readback_final_index_ready,
    source_public_ga_operator_packet_non_send_readback_final_index_blocked: $source.public_ga_operator_packet_non_send_readback_final_index_blocked,
    source_canonical_governance_tool_execution_closure_backfeed_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_ready,
    source_canonical_governance_tool_execution_closure_backfeed_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_ready_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_ready_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_categorization_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_categorization_ready,
    source_canonical_governance_tool_execution_closure_backfeed_categories: $source.source_canonical_governance_tool_execution_closure_backfeed_categories,
    public_ga_operator_packet_non_send_readback_final_index_attached: true,
    public_ga_operator_approval_non_acceptance_attachment_ready: true,
    public_ga_operator_approval_non_acceptance_attachment_blocked: true,
    operator_approval_non_acceptance_gate_present: true,
    operator_approval_non_acceptance_doc_present: true,
    operator_approval_non_acceptance_static_mention_count: $non_acceptance_static_mention_count,
    operator_approval_non_acceptance_gate_invoked: false,
    operator_approval_gap_ledger_summary_gate_invoked: false,
    long_soak_required_by_source_non_acceptance_gate: true,
    long_soak_started: false,
    public_ga_operator_packet_target_curl_count: $source.public_ga_operator_packet_target_curl_count,
    public_ga_operator_packet_target_endpoint_count: $source.public_ga_operator_packet_target_endpoint_count,
    public_ga_operator_packet_required_approval_static_count: $source.public_ga_operator_packet_required_approval_static_count,
    public_ga_operator_approval_packet_invoked: false,
    public_ga_operator_compat_wrapper_invoked: false,
    public_ga_operator_packet_live_endpoint_read_performed: false,
    public_ga_operator_packet_endpoint_curl_performed: false,
    public_ga_operator_packet_sent: false,
    public_ga_operator_packet_recorded: false,
    public_ga_operator_packet_accepted: false,
    operator_approval_request_sent: false,
    operator_approval_recorded: false,
    operator_approval_accepted: false,
    operator_identity_accepted: false,
    operator_acknowledgement_accepted: false,
    telegram_send_performed: false,
    external_send_performed: false,
    public_ga_readiness_script_invoked: false,
    public_claim_non_promotion_denial_gate_invoked: false,
    terminal_live_gates_invoked: false,
    attachment_blocker_count: 20,
    manual_operator_live_cutover_approval_required: true,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_approval_non_acceptance_readback_without_packet_send",
    local_gate: "scripts/hepta-systems-public-ga-operator-packet-final-index-operator-approval-non-acceptance-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_PACKET_FINAL_INDEX_OPERATOR_APPROVAL_NON_ACCEPTANCE_2026-06-21.md",
    source_files: {
      public_ga_operator_packet_non_send_readback_final_index_report: "scripts/hepta-systems-public-ga-operator-packet-non-send-readback-final-index-report.sh",
      operator_approval_acknowledgement_non_acceptance_gate: "scripts/hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-non-acceptance-gate.sh",
      operator_approval_acknowledgement_non_acceptance_doc: "docs/architecture/HEPTA_CORE_ACTIVATION_OPERATOR_APPROVAL_GAP_LEDGER_SUMMARY_BRIEFING_ACKNOWLEDGEMENT_NON_ACCEPTANCE_GATE.md"
    },
    side_effect_free: true,
    side_effects: {
      report_written: false,
      git_index_mutated: false,
      public_ga_operator_approval_packet_invoked: false,
      public_ga_operator_compat_wrapper_invoked: false,
      public_ga_operator_packet_endpoint_curl_performed: false,
      public_ga_operator_packet_live_endpoint_read_performed: false,
      public_ga_operator_packet_sent: false,
      public_ga_operator_packet_recorded: false,
      public_ga_operator_packet_accepted: false,
      operator_approval_non_acceptance_gate_invoked: false,
      operator_approval_gap_ledger_summary_gate_invoked: false,
      operator_approval_request_sent: false,
      operator_approval_recorded: false,
      operator_approval_accepted: false,
      operator_identity_accepted: false,
      operator_acknowledgement_accepted: false,
      telegram_send_performed: false,
      external_send_performed: false,
      long_soak_started: false,
      terminal_live_gate_invoked: false,
      terminal_live_url_contacted: false,
      public_ga_readiness_script_invoked: false,
      public_claim_non_promotion_denial_gate_invoked: false,
      public_ga_claim_recorded: false,
      public_ga_promoted: false,
      public_release_published: false,
      rollback_executed: false,
      external_network_read: false
    }
  }'
