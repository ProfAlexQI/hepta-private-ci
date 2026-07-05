#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
PREFLIGHT_REPORT="$ROOT/scripts/hepta-systems-terminal-publication-evidence-final-index-public-ga-readiness-preflight-report.sh"
ADAPTER_FINAL_INDEX_REPORT="$ROOT/scripts/hepta-systems-public-ga-readiness-non-live-source-probe-adapter-final-index-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_READINESS_PREFLIGHT_NON_LIVE_ADAPTER_MIGRATION_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-readiness-preflight-non-live-adapter-migration-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$PREFLIGHT_REPORT" ]] || fail "missing executable Public GA readiness preflight report: $PREFLIGHT_REPORT"
[[ -x "$ADAPTER_FINAL_INDEX_REPORT" ]] || fail "missing executable Public GA readiness non-live adapter final index report: $ADAPTER_FINAL_INDEX_REPORT"
[[ -f "$DOC" ]] || fail "missing Public GA readiness preflight non-live adapter migration architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Public GA readiness preflight non-live adapter migration report"
fi

jq -n \
  --slurpfile preflight <("$PREFLIGHT_REPORT") \
  --slurpfile adapter <("$ADAPTER_FINAL_INDEX_REPORT") \
  --arg gate "scripts/hepta-systems-public-ga-readiness-preflight-non-live-adapter-migration-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_READINESS_PREFLIGHT_NON_LIVE_ADAPTER_MIGRATION_2026-06-21.md" \
  '
  ($preflight[0]) as $preflight |
  ($adapter[0]) as $adapter |
  ($preflight.side_effects + $adapter.side_effects + {
    public_ga_readiness_preflight_report_mutated:false,
    public_ga_readiness_non_live_migration_written:false,
    public_ga_readiness_script_invoked:false,
    public_ga_readiness_live_endpoint_read_performed:false,
    public_ga_readiness_endpoint_curl_performed:false,
    public_ga_readiness_report_materialized:false,
    public_ga_readiness_attachment_recorded:false,
    external_network_read:false,
    public_ga_claim_recorded:false
  }) as $side_effects |
  [
    "source_preflight_preserved_as_live_target_evidence",
    "direct_public_ga_readiness_target_invocation_blocked",
    "non_live_adapter_final_index_ready_but_blocked",
    "public_ga_readiness_dedicated_architecture_note_missing",
    "public_ga_readiness_attachment_still_blocked",
    "public_ga_claim_disabled",
    "public_release_claim_disabled",
    "public_distribution_publication_disabled",
    "publication_evidence_recording_disabled",
    "terminal_publication_evidence_gate_not_invoked",
    "watchdog_not_invoked",
    "terminal_live_gates_not_invoked",
    "live_url_not_contacted",
    "long_soak_not_started",
    "operator_approval_not_recorded",
    "rollback_execution_blocked"
  ] as $migration_blockers |
  ($preflight.public_ga_readiness_preflight_ready == true
    and $preflight.public_ga_readiness_preflight_blocked == true
    and $preflight.public_ga_readiness_live_endpoint_read_required_by_target == true
    and $preflight.public_ga_readiness_script_invoked == false
    and $preflight.public_ga_readiness_live_endpoint_read_performed == false
    and $preflight.public_ga_readiness_endpoint_curl_performed == false
    and $adapter.public_ga_readiness_non_live_source_probe_adapter_final_index_ready == true
    and $adapter.public_ga_readiness_non_live_source_probe_adapter_final_index_blocked == true
    and $adapter.public_ga_readiness_non_live_endpoint_inventory_ready == true
    and $adapter.public_ga_readiness_target_endpoint_count == 9
    and $adapter.public_ga_readiness_script_invoked == false
    and $adapter.public_ga_readiness_live_endpoint_read_performed == false
    and $adapter.public_ga_readiness_endpoint_curl_performed == false
    and $adapter.public_ga_readiness_report_materialized == false
    and $adapter.public_ga_readiness_attachment_allowed == false
    and ($side_effects | to_entries | all(.value == false))) as $migration_ready |
  {
    runtime:"hepta",
    surface:"public_ga_readiness_preflight_non_live_adapter_migration",
    plugin_id:$preflight.plugin_id,
    status:(if $migration_ready then "ready_blocked" else "blocked" end),
    source_public_ga_readiness_preflight_surface:$preflight.surface,
    source_public_ga_readiness_preflight_ready:$preflight.public_ga_readiness_preflight_ready,
    source_public_ga_readiness_preflight_blocked:$preflight.public_ga_readiness_preflight_blocked,
    source_public_ga_readiness_non_live_adapter_final_index_surface:$adapter.surface,
    source_public_ga_readiness_non_live_adapter_final_index_ready:$adapter.public_ga_readiness_non_live_source_probe_adapter_final_index_ready,
    source_public_ga_readiness_non_live_adapter_final_index_blocked:$adapter.public_ga_readiness_non_live_source_probe_adapter_final_index_blocked,
    public_ga_readiness_preflight_non_live_adapter_migration_ready:$migration_ready,
    public_ga_readiness_preflight_non_live_adapter_migration_blocked:true,
    public_ga_readiness_preflight_migration_basis:"non_live_adapter_final_index",
    public_ga_readiness_preflight_report_mutated:false,
    public_ga_readiness_non_live_adapter_final_index_attached:true,
    public_ga_readiness_endpoint_inventory_from_adapter:true,
    public_ga_readiness_non_live_endpoint_inventory_ready:true,
    public_ga_readiness_target_endpoint_count:$adapter.public_ga_readiness_target_endpoint_count,
    public_ga_readiness_existing_doc_present:$adapter.public_ga_readiness_existing_doc_present,
    public_ga_readiness_dedicated_architecture_note_required:$adapter.public_ga_readiness_dedicated_architecture_note_required,
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
    migration_blocker_count:($migration_blockers | length),
    migration_blockers:$migration_blockers,
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
    next_migration_step:"derive_public_ga_readiness_preflight_non_live_adapter_migration_readback_without_public_ga_readiness_invocation",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      public_ga_readiness_preflight_report:"scripts/hepta-systems-terminal-publication-evidence-final-index-public-ga-readiness-preflight-report.sh",
      public_ga_readiness_non_live_adapter_final_index_report:"scripts/hepta-systems-public-ga-readiness-non-live-source-probe-adapter-final-index-report.sh"
    },
    side_effect_free:true,
    side_effects:$side_effects
  }'
