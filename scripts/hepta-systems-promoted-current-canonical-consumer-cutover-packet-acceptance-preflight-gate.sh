#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-promoted-current-canonical-consumer-cutover-packet-acceptance-preflight-report.sh"
READBACK_REPORT="$ROOT/scripts/hepta-systems-promoted-current-canonical-consumer-cutover-packet-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PROMOTED_CURRENT_CANONICAL_CONSUMER_CUTOVER_PACKET_ACCEPTANCE_PREFLIGHT_2026-06-21.md"

fail() {
  printf 'hepta-systems-promoted-current-canonical-consumer-cutover-packet-acceptance-preflight-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable promoted current canonical consumer cutover packet acceptance preflight report: $REPORT"
[[ -x "$READBACK_REPORT" ]] || fail "missing executable promoted current canonical consumer cutover packet readback report: $READBACK_REPORT"
[[ -f "$DOC" ]] || fail "missing promoted current canonical consumer cutover packet acceptance preflight architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the promoted current canonical consumer cutover packet acceptance preflight report"
fi

grep -q 'Promoted Current Canonical Consumer Cutover Packet Acceptance Preflight' "$DOC" \
  || fail "architecture note must document Promoted Current Canonical Consumer Cutover Packet Acceptance Preflight"
grep -q 'acceptance remains blocked' "$DOC" \
  || fail "architecture note must document acceptance remains blocked"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that the preflight does not invoke the alias"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "promoted_current_canonical_consumer_cutover_packet_acceptance_preflight"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .source_packet_readback_surface == "promoted_current_canonical_consumer_cutover_packet_readback"
  and .source_packet_readback_ready == true
  and .acceptance_preflight_ready == true
  and .cutover_packet_acceptance_allowed == false
  and .successor_consumer_cutover_allowed == false
  and .acceptance_requirement_count == 5
  and .acceptance_satisfied_requirement_count == 1
  and .acceptance_missing_requirement_count == 4
  and (.acceptance_requirements | any(.id == "packet_readback_ready" and .satisfied == true))
  and (.acceptance_requirements | any(.id == "operator_live_cutover_approval_recorded" and .satisfied == false))
  and (.acceptance_requirements | any(.id == "packet_recorded" and .satisfied == false))
  and (.acceptance_requirements | any(.id == "packet_accepted" and .satisfied == false))
  and (.acceptance_requirements | any(.id == "successor_consumer_cutover_allowed" and .satisfied == false))
  and .acceptance_blocker_count == 10
  and (.acceptance_blockers | index("operator_live_cutover_approval_missing")) != null
  and (.acceptance_blockers | index("cutover_packet_not_recorded")) != null
  and (.acceptance_blockers | index("cutover_packet_not_accepted")) != null
  and .cutover_packet_recorded == false
  and .cutover_packet_accepted == false
  and .operator_live_cutover_approval_recorded == false
  and .current_canonical_consumer_replaced_in_place == false
  and .current_canonical_consumer_mutated == false
  and .promoted_current_canonical_consumer_mutated == false
  and .rollback_anchor == "current_canonical_consumer"
  and .execution_enabled_count == 0
  and .public_ga_enabled_count == 0
  and .canonical_gate_wrapper_invoked == false
  and .wrapper_target_invoked == false
  and .capability_matrix_gate_invoked == false
  and .terminal_live_gate_invoked == false
  and .live_url_required == false
  and .long_soak_required == false
  and .manual_operator_live_cutover_approval_required == true
  and .tool_execution_live_cutover_allowed == false
  and .tool_execution_public_ga_allowed == false
  and .next_migration_step == "derive_terminal_successor_canonical_consumer_cutover_final_gate_without_live_invocation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$READBACK_REPORT" | jq -e '
  .surface == "promoted_current_canonical_consumer_cutover_packet_readback"
  and .terminal_successor_canonical_consumer_cutover_packet_readback_ready == true
  and .cutover_packet_recorded == false
  and .cutover_packet_accepted == false
  and .canonical_gate_wrapper_invoked == false
  and .wrapper_target_invoked == false
' >/dev/null

printf 'hepta-systems-promoted-current-canonical-consumer-cutover-packet-acceptance-preflight-gate: PASS: terminal successor consumer cutover packet acceptance remains blocked without live invocation\n'
