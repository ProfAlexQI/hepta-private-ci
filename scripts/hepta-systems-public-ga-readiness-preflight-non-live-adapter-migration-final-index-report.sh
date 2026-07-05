#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
READBACK_REPORT="$ROOT/scripts/hepta-systems-public-ga-readiness-preflight-non-live-adapter-migration-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_READINESS_PREFLIGHT_NON_LIVE_ADAPTER_MIGRATION_FINAL_INDEX_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-readiness-preflight-non-live-adapter-migration-final-index-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$READBACK_REPORT" ]] || fail "missing executable Public GA readiness preflight non-live adapter migration readback report: $READBACK_REPORT"
[[ -f "$DOC" ]] || fail "missing Public GA readiness preflight non-live adapter migration final index architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Public GA readiness preflight non-live adapter migration final index report"
fi

jq -n \
  --slurpfile readback <("$READBACK_REPORT") \
  --arg gate "scripts/hepta-systems-public-ga-readiness-preflight-non-live-adapter-migration-final-index-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_READINESS_PREFLIGHT_NON_LIVE_ADAPTER_MIGRATION_FINAL_INDEX_2026-06-21.md" \
  '
  ($readback[0]) as $readback |
  ($readback.public_ga_readiness_preflight_non_live_adapter_migration_readback_ready == true
    and $readback.public_ga_readiness_preflight_non_live_adapter_migration_readback_blocked == true
    and $readback.public_ga_readiness_preflight_migration_basis == "non_live_adapter_final_index"
    and $readback.public_ga_readiness_non_live_adapter_final_index_attached == true
    and $readback.public_ga_readiness_endpoint_inventory_from_adapter == true
    and $readback.public_ga_readiness_target_endpoint_count == 9
    and $readback.public_ga_readiness_live_endpoint_read_required_by_migration == false
    and $readback.public_ga_readiness_script_invoked == false
    and $readback.public_ga_readiness_live_endpoint_read_performed == false
    and $readback.public_ga_readiness_endpoint_curl_performed == false
    and $readback.public_ga_readiness_report_materialized == false
    and $readback.public_ga_readiness_attachment_allowed == false
    and $readback.public_ga_claim_allowed == false
    and $readback.public_ga_claimed == false
    and ($readback.side_effects | to_entries | all(.value == false))) as $final_index_ready |
  {
    runtime:"hepta",
    surface:"public_ga_readiness_preflight_non_live_adapter_migration_final_index",
    plugin_id:$readback.plugin_id,
    status:(if $final_index_ready then "ready_blocked" else "blocked" end),
    source_public_ga_readiness_preflight_non_live_adapter_migration_readback_surface:$readback.surface,
    source_public_ga_readiness_preflight_non_live_adapter_migration_readback_ready:$readback.public_ga_readiness_preflight_non_live_adapter_migration_readback_ready,
    source_public_ga_readiness_preflight_non_live_adapter_migration_readback_blocked:$readback.public_ga_readiness_preflight_non_live_adapter_migration_readback_blocked,
    public_ga_readiness_preflight_non_live_adapter_migration_final_index_ready:$final_index_ready,
    public_ga_readiness_preflight_non_live_adapter_migration_final_index_blocked:true,
    public_ga_readiness_preflight_non_live_adapter_migration_readback_attached:true,
    public_ga_readiness_preflight_migration_basis:$readback.public_ga_readiness_preflight_migration_basis,
    public_ga_readiness_preflight_report_mutated:false,
    public_ga_readiness_non_live_adapter_final_index_attached:true,
    public_ga_readiness_endpoint_inventory_from_adapter:true,
    public_ga_readiness_non_live_endpoint_inventory_ready:true,
    public_ga_readiness_target_endpoint_count:$readback.public_ga_readiness_target_endpoint_count,
    public_ga_readiness_existing_doc_present:$readback.public_ga_readiness_existing_doc_present,
    public_ga_readiness_dedicated_architecture_note_required:$readback.public_ga_readiness_dedicated_architecture_note_required,
    public_ga_readiness_live_endpoint_read_required_by_original_target:true,
    public_ga_readiness_live_endpoint_read_required_by_migration:false,
    public_ga_readiness_script_invoked:false,
    public_ga_readiness_live_endpoint_read_performed:false,
    public_ga_readiness_endpoint_curl_performed:false,
    public_ga_readiness_report_materialized:false,
    public_ga_readiness_attachment_recorded:false,
    public_ga_readiness_attachment_allowed:false,
    terminal_publication_evidence_non_persistence_summary_gate_invoked:false,
    hepta_watchdog_invoked:false,
    terminal_public_distribution_non_publication_lock_gate_invoked:false,
    terminal_denial_index_gate_invoked:false,
    terminal_summary_gates_invoked:false,
    terminal_live_gates_invoked:false,
    canonical_gate_wrapper_invoked:false,
    wrapper_target_invoked:false,
    final_blocker_count:$readback.migration_blocker_count,
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
    operator_approval_recorded:false,
    operator_identity_accepted:false,
    rollback_execution_allowed:false,
    next_migration_step:"attach_public_ga_readiness_preflight_non_live_adapter_migration_final_index_to_public_ga_readiness_attachment_without_readiness_invocation",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      public_ga_readiness_preflight_non_live_adapter_migration_readback_report:"scripts/hepta-systems-public-ga-readiness-preflight-non-live-adapter-migration-readback-report.sh"
    },
    side_effect_free:true,
    side_effects:$readback.side_effects
  }'
