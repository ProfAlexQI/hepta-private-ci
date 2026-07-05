#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-readiness-preflight-non-live-adapter-migration-readback-report.sh"
MIGRATION_GATE="$ROOT/scripts/hepta-systems-public-ga-readiness-preflight-non-live-adapter-migration-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_READINESS_PREFLIGHT_NON_LIVE_ADAPTER_MIGRATION_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-readiness-preflight-non-live-adapter-migration-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Public GA readiness preflight non-live adapter migration readback report: $REPORT"
[[ -x "$MIGRATION_GATE" ]] || fail "missing executable Public GA readiness preflight non-live adapter migration gate: $MIGRATION_GATE"
[[ -f "$DOC" ]] || fail "missing Public GA readiness preflight non-live adapter migration readback architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Public GA readiness preflight non-live adapter migration readback report"
fi

grep -q 'Public GA Readiness Preflight Non-Live Adapter Migration Readback' "$DOC" \
  || fail "architecture note must document Public GA Readiness Preflight Non-Live Adapter Migration Readback"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that readback does not invoke Public GA readiness"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_readiness_preflight_non_live_adapter_migration_readback"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_readiness_preflight_non_live_adapter_migration_surface == "public_ga_readiness_preflight_non_live_adapter_migration"
  and .source_public_ga_readiness_preflight_non_live_adapter_migration_ready == true
  and .source_public_ga_readiness_preflight_non_live_adapter_migration_blocked == true
  and .public_ga_readiness_preflight_non_live_adapter_migration_readback_ready == true
  and .public_ga_readiness_preflight_non_live_adapter_migration_readback_blocked == true
  and .public_ga_readiness_preflight_non_live_adapter_migration_attached == true
  and .readback_mode == "static_report_readback_only"
  and .readback_check_count == 16
  and .public_ga_readiness_preflight_migration_basis == "non_live_adapter_final_index"
  and .public_ga_readiness_preflight_report_mutated == false
  and .public_ga_readiness_non_live_adapter_final_index_attached == true
  and .public_ga_readiness_endpoint_inventory_from_adapter == true
  and .public_ga_readiness_target_endpoint_count == 9
  and .public_ga_readiness_live_endpoint_read_required_by_original_target == true
  and .public_ga_readiness_live_endpoint_read_required_by_migration == false
  and .public_ga_readiness_script_invoked == false
  and .public_ga_readiness_live_endpoint_read_performed == false
  and .public_ga_readiness_endpoint_curl_performed == false
  and .public_ga_readiness_report_materialized == false
  and .public_ga_readiness_attachment_recorded == false
  and .public_ga_readiness_attachment_allowed == false
  and .terminal_live_gates_invoked == false
  and .canonical_gate_wrapper_invoked == false
  and .wrapper_target_invoked == false
  and .migration_blocker_count == 16
  and .public_ga_claim_allowed == false
  and .public_ga_claimed == false
  and .operator_approval_recorded == false
  and .operator_identity_accepted == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "derive_public_ga_readiness_preflight_non_live_adapter_migration_final_index_without_public_ga_readiness_invocation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$MIGRATION_GATE" >/dev/null

printf 'hepta-systems-public-ga-readiness-preflight-non-live-adapter-migration-readback-gate: PASS: Public GA readiness preflight non-live adapter migration readback is static without readiness invocation\n'
