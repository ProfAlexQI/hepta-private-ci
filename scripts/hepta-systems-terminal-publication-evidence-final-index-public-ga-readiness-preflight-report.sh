#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
PUBLICATION_EVIDENCE_FINAL_INDEX_REPORT="$ROOT/scripts/hepta-systems-terminal-publication-evidence-non-persistence-summary-attachment-final-index-report.sh"
PUBLIC_GA_READINESS_SCRIPT="$ROOT/scripts/hepta-public-ga-readiness.sh"
PUBLIC_GA_READINESS_DOC="$ROOT/docs/architecture/HEPTA_PUBLIC_GA_READINESS.md"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TERMINAL_PUBLICATION_EVIDENCE_FINAL_INDEX_PUBLIC_GA_READINESS_PREFLIGHT_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-publication-evidence-final-index-public-ga-readiness-preflight-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$PUBLICATION_EVIDENCE_FINAL_INDEX_REPORT" ]] || fail "missing executable terminal publication evidence attachment final index report: $PUBLICATION_EVIDENCE_FINAL_INDEX_REPORT"
[[ -x "$PUBLIC_GA_READINESS_SCRIPT" ]] || fail "missing executable public GA readiness script: $PUBLIC_GA_READINESS_SCRIPT"
[[ -f "$DOC" ]] || fail "missing public GA readiness preflight architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the public GA readiness preflight report"
fi

public_ga_readiness_curl_count="$(grep -c 'curl -fsS' "$PUBLIC_GA_READINESS_SCRIPT" || true)"
public_ga_readiness_api_endpoint_count="$(grep -o '"/api/[^"]*"' "$PUBLIC_GA_READINESS_SCRIPT" | wc -l | tr -d '[:space:]')"
if [[ -f "$PUBLIC_GA_READINESS_DOC" ]]; then
  public_ga_readiness_doc_present=true
else
  public_ga_readiness_doc_present=false
fi

jq -n \
  --slurpfile publication <("$PUBLICATION_EVIDENCE_FINAL_INDEX_REPORT") \
  --argjson public_ga_readiness_curl_count "$public_ga_readiness_curl_count" \
  --argjson public_ga_readiness_api_endpoint_count "$public_ga_readiness_api_endpoint_count" \
  --argjson public_ga_readiness_doc_present "$public_ga_readiness_doc_present" \
  --arg gate "scripts/hepta-systems-terminal-publication-evidence-final-index-public-ga-readiness-preflight-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_TERMINAL_PUBLICATION_EVIDENCE_FINAL_INDEX_PUBLIC_GA_READINESS_PREFLIGHT_2026-06-21.md" \
  '
  ($publication[0]) as $source |
  ($source.side_effects + {
    public_ga_readiness_script_invoked:false,
    public_ga_readiness_live_endpoint_read_performed:false,
    public_ga_readiness_endpoint_curl_performed:false,
    public_ga_readiness_report_materialized:false,
    external_network_read:false,
    public_ga_claim_recorded:false
  }) as $side_effects |
  [
    "public_ga_readiness_script_not_invoked",
    "public_ga_readiness_live_endpoint_read_required_by_target",
    "public_ga_readiness_non_live_readback_adapter_required",
    "public_ga_readiness_dedicated_architecture_note_missing",
    "terminal_publication_evidence_non_persistence_summary_not_invoked",
    "publication_evidence_summary_recording_disabled",
    "publication_evidence_summary_persistence_disabled",
    "publication_evidence_public_claim_disabled",
    "publication_evidence_public_distribution_disabled",
    "public_ga_claim_disabled",
    "public_release_claim_disabled",
    "public_distribution_publication_disabled",
    "public_distribution_artifact_write_disabled",
    "terminal_live_gates_not_invoked",
    "live_url_not_contacted",
    "long_soak_not_started",
    "operator_approval_not_recorded"
  ] as $preflight_blockers |
  ($source.terminal_publication_evidence_non_persistence_summary_attachment_final_index_ready == true
    and $source.terminal_publication_evidence_non_persistence_summary_attachment_final_index_blocked == true
    and $source.terminal_publication_evidence_non_persistence_summary_gate_invoked == false
    and $source.hepta_watchdog_invoked == false
    and $source.terminal_public_distribution_non_publication_lock_gate_invoked == false
    and $source.terminal_live_gates_invoked == false
    and $source.canonical_gate_wrapper_invoked == false
    and $source.wrapper_target_invoked == false
    and $source.public_distribution_publication_allowed == false
    and $source.public_distribution_artifact_write_allowed == false
    and $source.public_release_claim_allowed == false
    and $source.public_ga_claim_allowed == false
    and $source.public_release_published == false
    and $source.public_ga_claimed == false
    and $source.publication_evidence_summary_recorded == false
    and $source.publication_evidence_summary_persisted == false
    and $source.publication_evidence_receipt_persisted == false
    and $source.publication_evidence_ledger_persisted == false
    and $public_ga_readiness_curl_count >= 1
    and $public_ga_readiness_api_endpoint_count >= 1
    and ($side_effects | to_entries | all(.value == false))) as $preflight_ready |
  {
    runtime:"hepta",
    surface:"terminal_publication_evidence_final_index_public_ga_readiness_preflight",
    plugin_id:$source.plugin_id,
    status:(if $preflight_ready then "ready_blocked" else "blocked" end),
    source_terminal_publication_evidence_non_persistence_summary_attachment_final_index_surface:$source.surface,
    source_terminal_publication_evidence_non_persistence_summary_attachment_final_index_ready:$source.terminal_publication_evidence_non_persistence_summary_attachment_final_index_ready,
    source_terminal_publication_evidence_non_persistence_summary_attachment_final_index_blocked:$source.terminal_publication_evidence_non_persistence_summary_attachment_final_index_blocked,
    public_ga_readiness_preflight_ready:$preflight_ready,
    public_ga_readiness_preflight_blocked:true,
    public_ga_readiness_source_probe_ready:true,
    public_ga_readiness_script_present:true,
    public_ga_readiness_existing_doc_present:$public_ga_readiness_doc_present,
    public_ga_readiness_live_endpoint_read_required_by_target:true,
    public_ga_readiness_curl_count:$public_ga_readiness_curl_count,
    public_ga_readiness_api_endpoint_count:$public_ga_readiness_api_endpoint_count,
    public_ga_readiness_script_invoked:false,
    public_ga_readiness_live_endpoint_read_performed:false,
    public_ga_readiness_endpoint_curl_performed:false,
    public_ga_readiness_report_materialized:false,
    public_ga_readiness_attachment_allowed:false,
    non_live_readback_adapter_required:true,
    dedicated_public_ga_readiness_architecture_note_required:($public_ga_readiness_doc_present == false),
    terminal_publication_evidence_non_persistence_summary_gate_invoked:false,
    hepta_watchdog_invoked:false,
    terminal_public_distribution_non_publication_lock_gate_invoked:false,
    terminal_denial_index_gate_invoked:false,
    terminal_summary_gates_invoked:false,
    terminal_live_gates_invoked:false,
    canonical_gate_wrapper_invoked:false,
    wrapper_target_invoked:false,
    preflight_blocker_count:($preflight_blockers | length),
    preflight_blockers:$preflight_blockers,
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
    next_migration_step:"create_public_ga_readiness_non_live_source_probe_adapter_before_attachment",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      terminal_publication_evidence_non_persistence_summary_attachment_final_index_report:"scripts/hepta-systems-terminal-publication-evidence-non-persistence-summary-attachment-final-index-report.sh",
      public_ga_readiness_script:"scripts/hepta-public-ga-readiness.sh",
      public_ga_readiness_expected_doc:"docs/architecture/HEPTA_PUBLIC_GA_READINESS.md"
    },
    side_effect_free:true,
    side_effects:$side_effects
  }'
