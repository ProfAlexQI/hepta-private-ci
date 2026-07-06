#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
PREFLIGHT_REPORT="$ROOT/scripts/hepta-systems-terminal-publication-evidence-final-index-public-ga-readiness-preflight-report.sh"
PUBLIC_GA_READINESS_SCRIPT="$ROOT/scripts/hepta-public-ga-readiness.sh"
PUBLIC_GA_READINESS_DOC="$ROOT/docs/architecture/HEPTA_PUBLIC_GA_READINESS.md"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_READINESS_NON_LIVE_SOURCE_PROBE_ADAPTER_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-readiness-non-live-source-probe-adapter-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$PREFLIGHT_REPORT" ]] || fail "missing executable Public GA readiness preflight report: $PREFLIGHT_REPORT"
[[ -x "$PUBLIC_GA_READINESS_SCRIPT" ]] || fail "missing executable public GA readiness script: $PUBLIC_GA_READINESS_SCRIPT"
[[ -f "$DOC" ]] || fail "missing Public GA readiness non-live source-probe adapter architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Public GA readiness non-live source-probe adapter report"
fi

PUBLIC_GA_READINESS_CURL_COUNT="$(grep -c 'curl -fsS' "$PUBLIC_GA_READINESS_SCRIPT" || true)"
PUBLIC_GA_READINESS_ENDPOINTS_JSON="$(
  grep 'curl -fsS' "$PUBLIC_GA_READINESS_SCRIPT" \
    | sed -E 's/.*"\$BASE_URL([^"]+)".*/\1/' \
    | jq -R . \
    | jq -s .
)"
PUBLIC_GA_READINESS_ENDPOINT_COUNT="$(jq 'length' <<<"$PUBLIC_GA_READINESS_ENDPOINTS_JSON")"
if [[ -f "$PUBLIC_GA_READINESS_DOC" ]]; then
  PUBLIC_GA_READINESS_DOC_PRESENT=true
else
  PUBLIC_GA_READINESS_DOC_PRESENT=false
fi

jq -n \
  --slurpfile preflight <("$PREFLIGHT_REPORT") \
  --argjson public_ga_readiness_curl_count "$PUBLIC_GA_READINESS_CURL_COUNT" \
  --argjson public_ga_readiness_endpoint_count "$PUBLIC_GA_READINESS_ENDPOINT_COUNT" \
  --argjson public_ga_readiness_endpoints "$PUBLIC_GA_READINESS_ENDPOINTS_JSON" \
  --argjson public_ga_readiness_doc_present "$PUBLIC_GA_READINESS_DOC_PRESENT" \
  --arg gate "scripts/hepta-systems-public-ga-readiness-non-live-source-probe-adapter-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_READINESS_NON_LIVE_SOURCE_PROBE_ADAPTER_2026-06-21.md" \
  '
  ($preflight[0]) as $preflight |
  ($preflight.side_effects + {
    public_ga_readiness_non_live_adapter_written:false,
    public_ga_readiness_script_invoked:false,
    public_ga_readiness_live_endpoint_read_performed:false,
    public_ga_readiness_endpoint_curl_performed:false,
    public_ga_readiness_report_materialized:false,
    public_ga_readiness_attachment_recorded:false,
    external_network_read:false,
    public_ga_claim_recorded:false
  }) as $side_effects |
  [
    "public_ga_readiness_non_live_source_probe_only",
    "public_ga_readiness_script_not_invoked",
    "public_ga_readiness_live_endpoint_read_blocked",
    "public_ga_readiness_endpoint_curl_blocked",
    "public_ga_readiness_report_materialization_blocked",
    "public_ga_readiness_dedicated_architecture_note_missing",
    "public_ga_readiness_attachment_still_blocked",
    "public_ga_claim_disabled",
    "public_release_claim_disabled",
    "public_distribution_publication_disabled",
    "terminal_live_gates_not_invoked",
    "live_url_not_contacted",
    "long_soak_not_started",
    "operator_approval_not_recorded"
  ] as $adapter_blockers |
  ($preflight.public_ga_readiness_preflight_ready == true
    and $preflight.public_ga_readiness_preflight_blocked == true
    and $preflight.public_ga_readiness_script_present == true
    and $preflight.public_ga_readiness_script_invoked == false
    and $preflight.public_ga_readiness_live_endpoint_read_performed == false
    and $preflight.public_ga_readiness_endpoint_curl_performed == false
    and $preflight.public_ga_readiness_report_materialized == false
    and $preflight.public_ga_readiness_attachment_allowed == false
    and $preflight.non_live_readback_adapter_required == true
    and $public_ga_readiness_curl_count == $public_ga_readiness_endpoint_count
    and $public_ga_readiness_endpoint_count == 9
    and ($side_effects | to_entries | all(.value == false))) as $adapter_ready |
  {
    runtime:"hepta",
    surface:"public_ga_readiness_non_live_source_probe_adapter",
    plugin_id:$preflight.plugin_id,
    status:(if $adapter_ready then "ready_blocked" else "blocked" end),
    source_public_ga_readiness_preflight_surface:$preflight.surface,
    source_public_ga_readiness_preflight_ready:$preflight.public_ga_readiness_preflight_ready,
    source_public_ga_readiness_preflight_blocked:$preflight.public_ga_readiness_preflight_blocked,
    public_ga_readiness_non_live_source_probe_adapter_ready:$adapter_ready,
    public_ga_readiness_non_live_source_probe_adapter_blocked:true,
    public_ga_readiness_non_live_endpoint_inventory_ready:$adapter_ready,
    public_ga_readiness_script_present:true,
    public_ga_readiness_existing_doc_present:$public_ga_readiness_doc_present,
    public_ga_readiness_dedicated_architecture_note_required:($public_ga_readiness_doc_present == false),
    public_ga_readiness_target_curl_count:$public_ga_readiness_curl_count,
    public_ga_readiness_target_endpoint_count:$public_ga_readiness_endpoint_count,
    public_ga_readiness_target_endpoints:$public_ga_readiness_endpoints,
    public_ga_readiness_script_invoked:false,
    public_ga_readiness_live_endpoint_read_performed:false,
    public_ga_readiness_endpoint_curl_performed:false,
    public_ga_readiness_report_materialized:false,
    public_ga_readiness_attachment_recorded:false,
    public_ga_readiness_attachment_allowed:false,
    non_live_readback_adapter_available:true,
    terminal_publication_evidence_non_persistence_summary_gate_invoked:false,
    hepta_watchdog_invoked:false,
    terminal_public_distribution_non_publication_lock_gate_invoked:false,
    terminal_denial_index_gate_invoked:false,
    terminal_summary_gates_invoked:false,
    terminal_live_gates_invoked:false,
    canonical_gate_wrapper_invoked:false,
    wrapper_target_invoked:false,
    adapter_blocker_count:($adapter_blockers | length),
    adapter_blockers:$adapter_blockers,
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
    next_migration_step:"derive_public_ga_readiness_non_live_source_probe_adapter_readback_without_public_ga_readiness_invocation",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      public_ga_readiness_preflight_report:"scripts/hepta-systems-terminal-publication-evidence-final-index-public-ga-readiness-preflight-report.sh",
      public_ga_readiness_script:"scripts/hepta-public-ga-readiness.sh",
      public_ga_readiness_expected_doc:"docs/architecture/HEPTA_PUBLIC_GA_READINESS.md"
    },
    side_effect_free:true,
    side_effects:$side_effects
  }'
