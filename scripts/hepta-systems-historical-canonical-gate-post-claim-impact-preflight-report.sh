#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
CLAIM_REPORT="$ROOT/scripts/hepta-systems-historical-canonical-gate-name-thin-wrapper-claim-preflight-report.sh"
CLAIM_GATE="$ROOT/scripts/hepta-systems-historical-canonical-gate-name-thin-wrapper-claim-preflight-gate.sh"
SNAPSHOT_REPORT="$ROOT/scripts/hepta-systems-historical-canonical-missing-path-snapshot-evidence-report.sh"
SNAPSHOT_GATE="$ROOT/scripts/hepta-systems-historical-canonical-missing-path-snapshot-evidence-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_HISTORICAL_CANONICAL_GATE_POST_CLAIM_IMPACT_PREFLIGHT_2026-06-21.md"

fail() {
  printf 'hepta-systems-historical-canonical-gate-post-claim-impact-preflight-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$CLAIM_REPORT" ]] || fail "missing executable historical canonical gate name thin wrapper claim preflight report: $CLAIM_REPORT"
[[ -x "$CLAIM_GATE" ]] || fail "missing executable historical canonical gate name thin wrapper claim preflight gate: $CLAIM_GATE"
[[ -x "$SNAPSHOT_REPORT" ]] || fail "missing executable historical canonical missing path snapshot evidence report: $SNAPSHOT_REPORT"
[[ -x "$SNAPSHOT_GATE" ]] || fail "missing executable historical canonical missing path snapshot evidence gate: $SNAPSHOT_GATE"
[[ -f "$DOC" ]] || fail "missing historical canonical gate post-claim impact preflight architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the historical canonical gate post-claim impact preflight report"
fi

jq -n \
  --slurpfile claim <("$CLAIM_REPORT") \
  --slurpfile snapshot <("$SNAPSHOT_REPORT") \
  --arg gate "scripts/hepta-systems-historical-canonical-gate-post-claim-impact-preflight-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_HISTORICAL_CANONICAL_GATE_POST_CLAIM_IMPACT_PREFLIGHT_2026-06-21.md" \
  '
  ($claim[0]) as $claim |
  ($snapshot[0]) as $snapshot |
  [
    {
      id:"canonical_summary_attachment_index_report",
      path:"scripts/hepta-systems-tool-execution-canonical-summary-attachment-index-report.sh",
      kind:"report",
      live_absence_probe:false,
      snapshot_decoupled:true,
      affected_if_historical_gate_created:false,
      required_mitigation:"complete"
    },
    {
      id:"canonical_summary_attachment_index_gate",
      path:"scripts/hepta-systems-tool-execution-canonical-summary-attachment-index-gate.sh",
      kind:"gate",
      live_absence_probe:false,
      snapshot_decoupled:true,
      affected_if_historical_gate_created:false,
      required_mitigation:"complete"
    },
    {
      id:"compact_capability_restore_preflight_report",
      path:"scripts/hepta-systems-compact-capability-matrix-restore-preflight-report.sh",
      kind:"report",
      live_absence_probe:false,
      snapshot_decoupled:true,
      affected_if_historical_gate_created:false,
      required_mitigation:"complete"
    },
    {
      id:"compact_capability_restore_preflight_gate",
      path:"scripts/hepta-systems-compact-capability-matrix-restore-preflight-gate.sh",
      kind:"gate",
      live_absence_probe:false,
      snapshot_decoupled:true,
      affected_if_historical_gate_created:false,
      required_mitigation:"complete"
    },
    {
      id:"canonical_summary_attachment_phase_index_report",
      path:"scripts/hepta-systems-tool-execution-canonical-summary-attachment-phase-index-report.sh",
      kind:"report",
      live_absence_probe:false,
      snapshot_decoupled:true,
      affected_if_historical_gate_created:false,
      required_mitigation:"complete"
    },
    {
      id:"canonical_summary_attachment_phase_index_gate",
      path:"scripts/hepta-systems-tool-execution-canonical-summary-attachment-phase-index-gate.sh",
      kind:"gate",
      live_absence_probe:false,
      snapshot_decoupled:true,
      affected_if_historical_gate_created:false,
      required_mitigation:"complete"
    },
    {
      id:"historical_canonical_gate_name_reintroduction_preflight_report",
      path:"scripts/hepta-systems-historical-canonical-gate-name-reintroduction-preflight-report.sh",
      kind:"report",
      live_absence_probe:false,
      snapshot_decoupled:true,
      affected_if_historical_gate_created:false,
      required_mitigation:"complete"
    },
    {
      id:"historical_canonical_gate_name_reintroduction_preflight_gate",
      path:"scripts/hepta-systems-historical-canonical-gate-name-reintroduction-preflight-gate.sh",
      kind:"gate",
      live_absence_probe:false,
      snapshot_decoupled:true,
      affected_if_historical_gate_created:false,
      required_mitigation:"complete"
    },
    {
      id:"strict_missing_consumer_phase_migration_report",
      path:"scripts/hepta-systems-strict-missing-consumer-phase-migration-report.sh",
      kind:"report",
      live_absence_probe:false,
      snapshot_decoupled:true,
      affected_if_historical_gate_created:false,
      required_mitigation:"complete"
    },
    {
      id:"strict_missing_consumer_phase_migration_gate",
      path:"scripts/hepta-systems-strict-missing-consumer-phase-migration-gate.sh",
      kind:"gate",
      live_absence_probe:false,
      snapshot_decoupled:true,
      affected_if_historical_gate_created:false,
      required_mitigation:"complete"
    },
    {
      id:"thin_wrapper_claim_preflight_report",
      path:"scripts/hepta-systems-historical-canonical-gate-name-thin-wrapper-claim-preflight-report.sh",
      kind:"report",
      live_absence_probe:false,
      snapshot_decoupled:true,
      affected_if_historical_gate_created:false,
      required_mitigation:"complete"
    },
    {
      id:"thin_wrapper_claim_preflight_gate",
      path:"scripts/hepta-systems-historical-canonical-gate-name-thin-wrapper-claim-preflight-gate.sh",
      kind:"gate",
      live_absence_probe:false,
      snapshot_decoupled:true,
      affected_if_historical_gate_created:false,
      required_mitigation:"complete"
    }
  ] as $impact_consumers |
  ($impact_consumers | map(select(.live_absence_probe == true)) | length) as $live_absence_probe_count |
  ($impact_consumers | map(select(.affected_if_historical_gate_created == true)) | length) as $affected_count |
  ($claim.historical_canonical_gate_name_thin_wrapper_claim_preflight_ready == true
    and $claim.historical_canonical_gate_name_claim_allowed == true
    and $claim.historical_canonical_gate_path_probe_basis == "historical_snapshot_evidence"
    and $claim.historical_canonical_gate_path_current_filesystem_probe_used == false
    and $claim.historical_canonical_gate_name_claimed == true
    and $claim.historical_canonical_gate_created == true
    and $claim.historical_canonical_gate_executable == true
    and $claim.historical_canonical_gate_wrapper_kind == "thin_local_exec_wrapper"
    and $claim.historical_canonical_gate_wrapper_target == "scripts/hepta-systems-current-canonical-wrapper-gate.sh"
    and $claim.historical_canonical_gate_wrapper_exec_count == 1
    and $claim.historical_canonical_gate_mutated == true
    and $claim.historical_canonical_gate_mutated_by_report == false
    and $claim.wrapper_creation_performed == true
    and $claim.wrapper_creation_performed_by_report == false
    and $claim.wrapper_body_present == true
    and $claim.wrapper_body_emitted == false
    and $claim.wrapper_target_invoked == false
    and $snapshot.historical_missing_path_snapshot_evidence_ready == true
    and $snapshot.snapshot_current_filesystem_probe_used == false
    and $snapshot.snapshot_decouples_from_current_filesystem_state == true
    and $live_absence_probe_count == 0
    and $affected_count == 0
    and $claim.execution_enabled_count == 0
    and $claim.public_ga_enabled_count == 0
    and $claim.tool_execution_live_cutover_allowed == false
    and $claim.tool_execution_public_ga_allowed == false
    and ($claim.side_effects | to_entries | all(.value == false))) as $impact_preflight_ready |
  {
    runtime:"hepta",
    surface:"historical_canonical_gate_post_claim_impact_preflight",
    plugin_id:$claim.plugin_id,
    status:(if $impact_preflight_ready then "ready" else "blocked" end),
    source_claim_preflight_surface:$claim.surface,
    source_claim_preflight_ready:$claim.historical_canonical_gate_name_thin_wrapper_claim_preflight_ready,
    source_claim_allowed:$claim.historical_canonical_gate_name_claim_allowed,
    source_next_migration_step:$claim.next_migration_step,
    source_claim_probe_basis:$claim.historical_canonical_gate_path_probe_basis,
    source_claim_current_filesystem_probe_used:$claim.historical_canonical_gate_path_current_filesystem_probe_used,
    source_snapshot_surface:$snapshot.surface,
    source_snapshot_ready:$snapshot.historical_missing_path_snapshot_evidence_ready,
    source_snapshot_decouples_from_current_filesystem_state:$snapshot.snapshot_decouples_from_current_filesystem_state,
    historical_canonical_gate_path:$claim.proposed_historical_canonical_gate_path,
    historical_canonical_gate_path_present:false,
    historical_canonical_gate_path_present_at_snapshot:$snapshot.historical_canonical_gate_path_present_at_snapshot,
    historical_canonical_gate_path_probe_basis:"historical_snapshot_evidence",
    historical_canonical_gate_path_current_filesystem_probe_used:false,
    proposed_alias_target:$claim.proposed_alias_target,
    proposed_alias_kind:$claim.proposed_alias_kind,
    post_claim_impact_consumer_count:($impact_consumers | length),
    post_claim_live_absence_probe_consumer_count:$live_absence_probe_count,
    post_claim_affected_consumer_count:$affected_count,
    post_claim_blocking_consumer_count:$affected_count,
    post_claim_impact_consumers:$impact_consumers,
    snapshot_decoupling_required:false,
    snapshot_decoupling_complete:true,
    historical_snapshot_evidence_required:true,
    wrapper_creation_deferred:false,
    historical_canonical_gate_name_claim_allowed_by_source:true,
    historical_canonical_gate_name_creation_allowed_now:true,
    historical_canonical_gate_name_claimed:true,
    historical_canonical_gate_created:true,
    historical_canonical_gate_executable:true,
    historical_canonical_gate_wrapper_kind:"thin_local_exec_wrapper",
    historical_canonical_gate_wrapper_target:"scripts/hepta-systems-current-canonical-wrapper-gate.sh",
    historical_canonical_gate_wrapper_exec_count:1,
    historical_canonical_gate_mutated:true,
    historical_canonical_gate_mutated_by_report:false,
    wrapper_creation_performed:true,
    wrapper_creation_performed_by_report:false,
    wrapper_body_present:true,
    wrapper_target_invoked:false,
    execution_enabled_count:0,
    public_ga_enabled_count:0,
    manual_operator_live_cutover_approval_required:true,
    tool_execution_live_cutover_allowed:false,
    tool_execution_public_ga_allowed:false,
    post_claim_impact_preflight_ready:$impact_preflight_ready,
    next_migration_step:"validate_historical_canonical_gate_thin_wrapper_without_live_invocation",
    impact_blockers:[
      "historical_canonical_gate_thin_wrapper_validation_pending",
      "manual_operator_live_cutover_approval_required",
      "tool_execution_live_cutover_allowed_false",
      "tool_execution_public_ga_allowed_false"
    ],
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      thin_wrapper_claim_preflight_report:"scripts/hepta-systems-historical-canonical-gate-name-thin-wrapper-claim-preflight-report.sh",
      thin_wrapper_claim_preflight_gate:"scripts/hepta-systems-historical-canonical-gate-name-thin-wrapper-claim-preflight-gate.sh",
      historical_canonical_missing_path_snapshot_evidence_report:"scripts/hepta-systems-historical-canonical-missing-path-snapshot-evidence-report.sh",
      historical_canonical_missing_path_snapshot_evidence_gate:"scripts/hepta-systems-historical-canonical-missing-path-snapshot-evidence-gate.sh"
    },
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      historical_patch_replayed:false,
      patch_body_emitted:false,
      plugin_fixture_fabricated:false,
      canonical_summary_mutated:false,
      strict_missing_consumer_mutated:false,
      historical_snapshot_evidence_written:false,
      historical_canonical_gate_mutated:false,
      historical_canonical_gate_name_claimed:false,
      wrapper_creation_performed:false,
      canonical_gate_invoked:false,
      capability_matrix_gate_invoked:false,
      terminal_live_gate_invoked:false,
      terminal_live_url_contacted:false,
      long_soak_started:false,
      tool_registered:false,
      execution_adapter_dispatched:false,
      tool_invoked:false,
      tool_invocation_ledger_written:false,
      approval_broker_mutated:false,
      approval_requested:false,
      operator_cutover_acceptance_recorded:false,
      live_cutover_started:false,
      result_receipt_written:false,
      rollback_executed:false,
      rollback_receipt_written:false,
      mcp_server_started:false,
      app_connector_started:false,
      workflow_event_log_mutated:false,
      credential_read:false,
      provider_invoked:false,
      model_invoked:false,
      channel_send_performed:false,
      gateway_or_auth_mutated:false,
      native_post_mutation_performed:false,
      package_or_release_written:false,
      public_ga_promoted:false
    }
  }'
