#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SINGLE_RENDER_REPORT="$ROOT/scripts/hepta-systems-matrix-report-single-render-cache-boundary-readback-report.sh"
KILL_SWITCH_BOUNDARY_REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-kill-switch-rehearsal-boundary-readback-report.sh"
CLOSURE_INDEX_REPORT="$ROOT/scripts/hepta-systems-tool-execution-live-cutover-closure-index-report.sh"
EVIDENCE_PLAN_REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-collection-plan-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/controlled_live_operator_readiness_dashboard.rs"
EVIDENCE_ACCEPTANCE_PACKET_SOURCE="$ROOT/codex-rs/hepta-runtime/src/status_canary_evidence_acceptance_packet.rs"
EVIDENCE_SOURCE_ADAPTER_SOURCE="$ROOT/codex-rs/hepta-runtime/src/status_canary_evidence_source_adapter.rs"
EVIDENCE_SOURCE_REASON_PACKET_SOURCE="$ROOT/codex-rs/hepta-runtime/src/status_canary_evidence_source_reason_packet.rs"
EVIDENCE_SOURCE_READBACK_SOURCE="$ROOT/codex-rs/hepta-runtime/src/status_canary_evidence_source_readback.rs"
EVIDENCE_SOURCE_VALIDATOR_SOURCE="$ROOT/codex-rs/hepta-runtime/src/status_canary_evidence_source_validator.rs"
START_GUARD_SOURCE="$ROOT/codex-rs/hepta-runtime/src/status_canary_start_guard.rs"
START_REQUEST_GATE_SOURCE="$ROOT/codex-rs/hepta-runtime/src/status_canary_start_request_gate.rs"
RUNNER_ADAPTER_SOURCE="$ROOT/codex-rs/hepta-runtime/src/status_canary_runner_adapter.rs"
RUNNER_START_SURFACE_SOURCE="$ROOT/codex-rs/hepta-runtime/src/status_canary_runner_start_surface.rs"
RUNNER_ENTRY_BOUNDARY_SOURCE="$ROOT/codex-rs/hepta-runtime/src/status_canary_runner_entry_boundary.rs"
RUNNER_ENTRY_ADAPTER_SOURCE="$ROOT/codex-rs/hepta-runtime/src/status_canary_runner_entry_adapter.rs"
RUNNER_BINDING_GUARD_SOURCE="$ROOT/codex-rs/hepta-runtime/src/status_canary_runner_binding_guard.rs"
RUNNER_DRY_RUN_SELECTOR_SOURCE="$ROOT/codex-rs/hepta-runtime/src/status_canary_runner_dry_run_selector.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_OPERATOR_READINESS_DASHBOARD_2026-06-27.md"

fail() {
  printf 'hepta-systems-controlled-live-operator-readiness-dashboard-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SINGLE_RENDER_REPORT" ]] || fail "missing executable single-render matrix report: $SINGLE_RENDER_REPORT"
[[ -x "$KILL_SWITCH_BOUNDARY_REPORT" ]] || fail "missing executable Phase 5n kill-switch rehearsal boundary report: $KILL_SWITCH_BOUNDARY_REPORT"
[[ -x "$CLOSURE_INDEX_REPORT" ]] || fail "missing executable live cutover closure index report: $CLOSURE_INDEX_REPORT"
[[ -x "$EVIDENCE_PLAN_REPORT" ]] || fail "missing executable required evidence collection plan report: $EVIDENCE_PLAN_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Phase 6 Rust source: $RUST_SOURCE"
[[ -f "$EVIDENCE_ACCEPTANCE_PACKET_SOURCE" ]] || fail "missing status canary evidence acceptance packet source: $EVIDENCE_ACCEPTANCE_PACKET_SOURCE"
[[ -f "$EVIDENCE_SOURCE_ADAPTER_SOURCE" ]] || fail "missing status canary evidence source adapter source: $EVIDENCE_SOURCE_ADAPTER_SOURCE"
[[ -f "$EVIDENCE_SOURCE_REASON_PACKET_SOURCE" ]] || fail "missing status canary evidence source reason packet source: $EVIDENCE_SOURCE_REASON_PACKET_SOURCE"
[[ -f "$EVIDENCE_SOURCE_READBACK_SOURCE" ]] || fail "missing status canary evidence source readback source: $EVIDENCE_SOURCE_READBACK_SOURCE"
[[ -f "$EVIDENCE_SOURCE_VALIDATOR_SOURCE" ]] || fail "missing status canary evidence source validator source: $EVIDENCE_SOURCE_VALIDATOR_SOURCE"
[[ -f "$START_GUARD_SOURCE" ]] || fail "missing status canary start guard source: $START_GUARD_SOURCE"
[[ -f "$START_REQUEST_GATE_SOURCE" ]] || fail "missing status canary start request gate source: $START_REQUEST_GATE_SOURCE"
[[ -f "$RUNNER_ADAPTER_SOURCE" ]] || fail "missing status canary runner adapter source: $RUNNER_ADAPTER_SOURCE"
[[ -f "$RUNNER_START_SURFACE_SOURCE" ]] || fail "missing status canary runner start surface source: $RUNNER_START_SURFACE_SOURCE"
[[ -f "$RUNNER_ENTRY_BOUNDARY_SOURCE" ]] || fail "missing status canary runner entry boundary source: $RUNNER_ENTRY_BOUNDARY_SOURCE"
[[ -f "$RUNNER_ENTRY_ADAPTER_SOURCE" ]] || fail "missing status canary runner entry adapter source: $RUNNER_ENTRY_ADAPTER_SOURCE"
[[ -f "$RUNNER_BINDING_GUARD_SOURCE" ]] || fail "missing status canary runner binding guard source: $RUNNER_BINDING_GUARD_SOURCE"
[[ -f "$RUNNER_DRY_RUN_SELECTOR_SOURCE" ]] || fail "missing status canary runner dry-run selector source: $RUNNER_DRY_RUN_SELECTOR_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Phase 6 architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the controlled-live operator readiness dashboard report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

render_report() {
  local name="$1"
  local script="$2"
  "$script" >"$tmpdir/$name.json" || fail "failed to render $name from $script"
  jq -e . "$tmpdir/$name.json" >/dev/null || fail "invalid JSON rendered by $script"
}

render_report single_render "$SINGLE_RENDER_REPORT"
render_report kill_switch "$KILL_SWITCH_BOUNDARY_REPORT"
render_report closure_index "$CLOSURE_INDEX_REPORT"
render_report evidence_plan "$EVIDENCE_PLAN_REPORT"

lib_export_present=false
if grep -q 'controlled_live_operator_readiness_dashboard_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

evidence_acceptance_packet_source_present=false
if grep -q 'status_canary_evidence_acceptance_packet' "$RUST_SOURCE" \
  && grep -q 'STATUS_CANARY_EVIDENCE_ACCEPTANCE_PACKET_ID' "$EVIDENCE_ACCEPTANCE_PACKET_SOURCE" \
  && grep -q 'status_canary_evidence_acceptance_packet' "$LIB_SOURCE"; then
  evidence_acceptance_packet_source_present=true
fi

evidence_source_adapter_source_present=false
if grep -q 'status_canary_evidence_source_adapter' "$RUST_SOURCE" \
  && grep -q 'STATUS_CANARY_EVIDENCE_SOURCE_ADAPTER_ID' "$EVIDENCE_SOURCE_ADAPTER_SOURCE" \
  && grep -q 'status_canary_evidence_source_adapter' "$LIB_SOURCE"; then
  evidence_source_adapter_source_present=true
fi

evidence_source_reason_packet_source_present=false
if grep -q 'status_canary_evidence_source_reason_packet' "$RUST_SOURCE" \
  && grep -q 'STATUS_CANARY_EVIDENCE_SOURCE_REASON_PACKET_ID' "$EVIDENCE_SOURCE_REASON_PACKET_SOURCE" \
  && grep -q 'status_canary_evidence_source_reason_packet' "$LIB_SOURCE"; then
  evidence_source_reason_packet_source_present=true
fi

evidence_source_readback_source_present=false
if grep -q 'status_canary_evidence_source_readback' "$RUST_SOURCE" \
  && grep -q 'STATUS_CANARY_EVIDENCE_SOURCE_READBACK_ID' "$EVIDENCE_SOURCE_READBACK_SOURCE" \
  && grep -q 'status_canary_evidence_source_readback' "$LIB_SOURCE"; then
  evidence_source_readback_source_present=true
fi

evidence_source_validator_source_present=false
if grep -q 'status_canary_evidence_source_validator' "$RUST_SOURCE" \
  && grep -q 'STATUS_CANARY_EVIDENCE_SOURCE_VALIDATOR_ID' "$EVIDENCE_SOURCE_VALIDATOR_SOURCE" \
  && grep -q 'status_canary_evidence_source_validator' "$LIB_SOURCE"; then
  evidence_source_validator_source_present=true
fi

start_guard_source_present=false
if grep -q 'status_canary_start_guard' "$RUST_SOURCE" \
  && grep -q 'STATUS_CANARY_START_GUARD_ID' "$START_GUARD_SOURCE" \
  && grep -q 'status_canary_start_guard' "$LIB_SOURCE"; then
  start_guard_source_present=true
fi

start_request_gate_source_present=false
if grep -q 'status_canary_start_request_gate' "$RUST_SOURCE" \
  && grep -q 'STATUS_CANARY_START_REQUEST_GATE_ID' "$START_REQUEST_GATE_SOURCE" \
  && grep -q 'status_canary_start_request_gate' "$LIB_SOURCE"; then
  start_request_gate_source_present=true
fi

runner_adapter_source_present=false
if grep -q 'status_canary_runner_adapter_plan' "$RUST_SOURCE" \
  && grep -q 'STATUS_CANARY_RUNNER_ADAPTER_ID' "$RUNNER_ADAPTER_SOURCE" \
  && grep -q 'status_canary_runner_adapter_plan' "$LIB_SOURCE"; then
  runner_adapter_source_present=true
fi

runner_start_surface_source_present=false
if grep -q 'status_canary_runner_start_surface_plan' "$RUST_SOURCE" \
  && grep -q 'STATUS_CANARY_RUNNER_START_SURFACE_ID' "$RUNNER_START_SURFACE_SOURCE" \
  && grep -q 'status_canary_runner_start_surface_plan' "$LIB_SOURCE"; then
  runner_start_surface_source_present=true
fi

runner_entry_boundary_source_present=false
if grep -q 'status_canary_runner_entry_boundary_plan' "$RUST_SOURCE" \
  && grep -q 'STATUS_CANARY_RUNNER_ENTRY_BOUNDARY_ID' "$RUNNER_ENTRY_BOUNDARY_SOURCE" \
  && grep -q 'status_canary_runner_entry_boundary_plan' "$LIB_SOURCE"; then
  runner_entry_boundary_source_present=true
fi

runner_entry_adapter_source_present=false
if grep -q 'status_canary_runner_entry_adapter_plan' "$RUST_SOURCE" \
  && grep -q 'STATUS_CANARY_RUNNER_ENTRY_ADAPTER_ID' "$RUNNER_ENTRY_ADAPTER_SOURCE" \
  && grep -q 'status_canary_runner_entry_adapter_plan' "$LIB_SOURCE"; then
  runner_entry_adapter_source_present=true
fi

runner_binding_guard_source_present=false
if grep -q 'status_canary_runner_binding_guard_plan' "$RUST_SOURCE" \
  && grep -q 'STATUS_CANARY_RUNNER_BINDING_GUARD_ID' "$RUNNER_BINDING_GUARD_SOURCE" \
  && grep -q 'status_canary_runner_binding_guard_plan' "$LIB_SOURCE"; then
  runner_binding_guard_source_present=true
fi

runner_dry_run_selector_source_present=false
if grep -q 'status_canary_runner_dry_run_selector_plan' "$RUST_SOURCE" \
  && grep -q 'STATUS_CANARY_RUNNER_DRY_RUN_SELECTOR_ID' "$RUNNER_DRY_RUN_SELECTOR_SOURCE" \
  && grep -q 'status_canary_runner_dry_run_selector_plan' "$LIB_SOURCE"; then
  runner_dry_run_selector_source_present=true
fi

jq -n \
  --slurpfile single_render "$tmpdir/single_render.json" \
  --slurpfile kill_switch "$tmpdir/kill_switch.json" \
  --slurpfile closure_index "$tmpdir/closure_index.json" \
  --slurpfile evidence_plan "$tmpdir/evidence_plan.json" \
  --argjson lib_export_present "$lib_export_present" \
  --argjson evidence_acceptance_packet_source_present "$evidence_acceptance_packet_source_present" \
  --argjson evidence_source_adapter_source_present "$evidence_source_adapter_source_present" \
  --argjson evidence_source_reason_packet_source_present "$evidence_source_reason_packet_source_present" \
  --argjson evidence_source_readback_source_present "$evidence_source_readback_source_present" \
  --argjson evidence_source_validator_source_present "$evidence_source_validator_source_present" \
  --argjson start_guard_source_present "$start_guard_source_present" \
  --argjson start_request_gate_source_present "$start_request_gate_source_present" \
  --argjson runner_adapter_source_present "$runner_adapter_source_present" \
  --argjson runner_start_surface_source_present "$runner_start_surface_source_present" \
  --argjson runner_entry_boundary_source_present "$runner_entry_boundary_source_present" \
  --argjson runner_entry_adapter_source_present "$runner_entry_adapter_source_present" \
  --argjson runner_binding_guard_source_present "$runner_binding_guard_source_present" \
  --argjson runner_dry_run_selector_source_present "$runner_dry_run_selector_source_present" \
  --arg gate "scripts/hepta-systems-controlled-live-operator-readiness-dashboard-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_OPERATOR_READINESS_DASHBOARD_2026-06-27.md" \
  '
  def dashboard_key($id):
    "controlled_live.operator_dashboard." + $id;
  def dashboard_route($id):
    "readback://controlled-live/operator-dashboard/" + ($id | gsub("_"; "-"));
  def status_canary_closure_key($id):
    "controlled_live.status_canary.evidence_closure." + $id;
  def status_canary_closure_route($id):
    "readback://controlled-live/status-canary/evidence-closure/" + ($id | gsub("_"; "-"));
  def status_canary_action_kind($id):
    if $id == "dirty_worktree_boundary" then "clean_worktree_snapshot_required"
    elif $id == "operator_live_approval_missing" then "operator_live_approval_packet_required"
    elif $id == "fresh_soak_readback_missing" then "fresh_status_canary_soak_readback_required"
    elif $id == "credential_boundary_attestation_missing" then "credential_boundary_attestation_required"
    elif $id == "gateway_native_telegram_post_boundary_approval_missing" then "transport_boundary_approval_required"
    elif $id == "rollback_rehearsal_missing" then "rollback_rehearsal_packet_required"
    elif $id == "kill_switch_rehearsal_missing" then "kill_switch_rehearsal_packet_required"
    else "unknown_status_canary_evidence_action_required"
    end;
  ($single_render[0]) as $single_render |
  ($kill_switch[0]) as $kill |
  ($closure_index[0]) as $closure |
  ($evidence_plan[0]) as $evidence_plan |
  ($kill.entries | map({
    source_blocker_id,
    dashboard_key:dashboard_key(.source_blocker_id),
    dashboard_route:dashboard_route(.source_blocker_id),
    source_readback_route:.kill_switch_rehearsal_boundary_route,
    operator_display_order,
    operator_status,
    evidence_state:.kill_switch_rehearsal_evidence_state,
    owner,
    risk_bucket,
    operator_label,
    required_evidence,
    operator_visible:true,
    queryable:true,
    diffable:true,
    acceptance_allowed:false,
    waiver_allowed:false,
    evidence_recording_allowed:false,
    credential_read_allowed:false,
    transport_mutation_allowed:false,
    persistence_allowed:false,
    live_mutation_allowed:false
  })) as $entries |
  ($entries | map(select(.operator_visible == true)) | length) as $operator_visible_blocker_count |
  ($entries | map(select(.evidence_state == "missing")) | length) as $missing_evidence_blocker_count |
  ($entries | map(select(.acceptance_allowed == true)) | length) as $accepted_blocker_count |
  ($entries | map(select(.waiver_allowed == true)) | length) as $waived_blocker_count |
  ($entries | map(select(.evidence_recording_allowed == true)) | length) as $evidence_recorded_count |
  ($evidence_plan.entries | map({
    source_blocker_id,
    selected_status_canary_tool_id:"preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp",
    preflight_only_connector_tool_id:"preview:connector:hepta-system@hepta-local:hepta_system_local_app",
    closure_key:status_canary_closure_key(.source_blocker_id),
    closure_route:status_canary_closure_route(.source_blocker_id),
    action_kind:status_canary_action_kind(.source_blocker_id),
    operator_label,
    required_evidence,
    evidence_state:"missing",
    operator_visible:true,
    action_required:true,
    canary_start_blocked:true,
    evidence_recorded:false,
    evidence_waived:false,
    evidence_expired:false,
    evidence_invalid:false,
    evidence_recording_allowed:false,
    waiver_allowed:false,
    credential_read_allowed:false,
    transport_mutation_allowed:false,
    persistence_allowed:false,
    live_mutation_allowed:false
  })) as $status_canary_evidence_closure_entries |
  ($status_canary_evidence_closure_entries | map(select(.operator_visible == true
    and .action_required == true
    and .canary_start_blocked == true
    and .evidence_state == "missing"
    and .evidence_recorded == false
    and .evidence_recording_allowed == false
    and .waiver_allowed == false
    and .credential_read_allowed == false
    and .transport_mutation_allowed == false
    and .persistence_allowed == false
    and .live_mutation_allowed == false)) | length) as $status_canary_evidence_closure_ready_count |
  ($status_canary_evidence_closure_entries | map(select(.evidence_state == "missing")) | length) as $status_canary_evidence_closure_missing_count |
  ($status_canary_evidence_closure_entries | map(select(.evidence_recorded == true)) | length) as $status_canary_evidence_closure_recorded_count |
  ($status_canary_evidence_closure_entries | map(select(.evidence_waived == true)) | length) as $status_canary_evidence_closure_waived_count |
  ($status_canary_evidence_closure_entries | map(select(.action_required == true)) | length) as $status_canary_evidence_closure_actionable_precondition_count |
  ($status_canary_evidence_closure_entries | length) as $status_canary_evidence_packet_item_count |
  ($status_canary_evidence_closure_missing_count) as $status_canary_evidence_packet_missing_count |
  ($status_canary_evidence_closure_recorded_count) as $status_canary_evidence_packet_recorded_count |
  ($status_canary_evidence_closure_waived_count) as $status_canary_evidence_packet_waived_count |
  0 as $status_canary_evidence_packet_expired_count |
  0 as $status_canary_evidence_packet_invalid_count |
  0 as $status_canary_evidence_packet_decision_reason_audit_count |
  0 as $status_canary_evidence_packet_decision_reason_audit_ready_count |
  0 as $status_canary_evidence_packet_decision_reason_audit_rejected_count |
  false as $status_canary_evidence_packet_complete |
  true as $status_canary_start_blocked_by_evidence_packet |
  false as $status_canary_start_allowed_by_evidence_packet |
  "status_canary_evidence_packet_blocked_missing_evidence" as $status_canary_evidence_packet_guard_route |
  true as $status_canary_evidence_acceptance_packet_ready |
  "status-canary-evidence-acceptance-packet/hepta-system-status/v1" as $status_canary_evidence_acceptance_packet_id |
  "status_canary_evidence_acceptance_packet_ready_no_decision_requests" as $status_canary_evidence_acceptance_packet_route |
  0 as $status_canary_evidence_acceptance_request_count |
  0 as $status_canary_evidence_acceptance_known_request_count |
  0 as $status_canary_evidence_acceptance_unknown_request_count |
  0 as $status_canary_evidence_acceptance_duplicate_request_count |
  0 as $status_canary_evidence_acceptance_request_source_validator_bound_count |
  0 as $status_canary_evidence_acceptance_request_source_validator_contract_audit_ready_count |
  0 as $status_canary_evidence_acceptance_request_reason_audit_count |
  0 as $status_canary_evidence_acceptance_request_reason_audit_ready_count |
  0 as $status_canary_evidence_acceptance_request_reason_audit_rejected_count |
  0 as $status_canary_evidence_acceptance_accepted_decision_count |
  0 as $status_canary_evidence_acceptance_rejected_decision_count |
  0 as $status_canary_evidence_acceptance_generated_override_count |
  0 as $status_canary_evidence_acceptance_generated_override_reason_audit_ready_count |
  true as $status_canary_evidence_source_adapter_report_ready |
  "status-canary-evidence-source-adapter/hepta-system-status/v1" as $status_canary_evidence_source_adapter_id |
  "status_canary_evidence_source_adapter_ready_no_inputs" as $status_canary_evidence_source_adapter_route |
  7 as $status_canary_evidence_source_adapter_count |
  0 as $status_canary_evidence_source_adapter_input_count |
  0 as $status_canary_evidence_source_adapter_generated_fixture_count |
  7 as $status_canary_evidence_source_adapter_missing_input_count |
  7 as $status_canary_evidence_source_adapter_metadata_contract_count |
  7 as $status_canary_evidence_source_adapter_metadata_contract_ready_count |
  21 as $status_canary_evidence_source_adapter_input_contract_field_count |
  70 as $status_canary_evidence_source_adapter_readback_fixture_contract_field_count |
  7 as $status_canary_evidence_source_adapter_required_field_validator_count |
  7 as $status_canary_evidence_source_adapter_required_field_validator_ready_count |
  0 as $status_canary_evidence_source_adapter_required_field_rejected_count |
  0 as $status_canary_evidence_source_adapter_missing_required_field_count |
  true as $status_canary_evidence_source_reason_packet_report_ready |
  "status-canary-evidence-source-reason-packet/hepta-system-status/v1" as $status_canary_evidence_source_reason_packet_id |
  "status_canary_evidence_source_reason_packet_ready_no_adapter_inputs" as $status_canary_evidence_source_reason_packet_route |
  7 as $status_canary_evidence_source_reason_packet_source_count |
  28 as $status_canary_evidence_source_decision_reason_count |
  28 as $status_canary_evidence_source_decision_reason_ready_count |
  84 as $status_canary_evidence_source_decision_required_field_count |
  84 as $status_canary_evidence_source_missing_required_field_reason_count |
  28 as $status_canary_evidence_source_adapter_input_missing_reason_count |
  0 as $status_canary_evidence_source_adapter_input_other_decision_reason_count |
  0 as $status_canary_evidence_source_adapter_rejection_reason_count |
  0 as $status_canary_evidence_source_fixture_generation_allowed_count |
  28 as $status_canary_evidence_source_fixture_generation_blocked_count |
  true as $status_canary_evidence_source_readback_report_ready |
  "status-canary-evidence-source-readback/hepta-system-status/v1" as $status_canary_evidence_source_readback_id |
  "status_canary_evidence_source_readback_ready_no_fixtures" as $status_canary_evidence_source_readback_route |
  0 as $status_canary_evidence_source_readback_fixture_count |
  0 as $status_canary_evidence_source_readback_observation_count |
  7 as $status_canary_evidence_source_readback_missing_observation_count |
  7 as $status_canary_evidence_source_readback_contract_audit_count |
  7 as $status_canary_evidence_source_readback_contract_audit_ready_count |
  0 as $status_canary_evidence_source_readback_fixture_contract_audit_ready_count |
  true as $status_canary_evidence_source_readback_reason_packet_bound |
  true as $status_canary_evidence_source_readback_reason_packet_ready |
  $status_canary_evidence_source_reason_packet_route as $status_canary_evidence_source_readback_reason_packet_route |
  0 as $status_canary_evidence_source_readback_fixture_reason_audit_count |
  0 as $status_canary_evidence_source_readback_fixture_reason_audit_ready_count |
  0 as $status_canary_evidence_source_readback_fixture_reason_audit_rejected_count |
  true as $status_canary_evidence_source_validator_ready |
  "status-canary-evidence-source-validator/hepta-system-status/v1" as $status_canary_evidence_source_validator_id |
  "status_canary_evidence_source_validator_ready_no_observations" as $status_canary_evidence_source_validator_route |
  0 as $status_canary_evidence_source_validator_contract_audit_count |
  0 as $status_canary_evidence_source_validator_contract_audit_ready_count |
  0 as $status_canary_evidence_source_validator_contract_audit_rejected_count |
  0 as $status_canary_evidence_source_validator_reason_audit_count |
  0 as $status_canary_evidence_source_validator_reason_audit_ready_count |
  0 as $status_canary_evidence_source_validator_reason_audit_rejected_count |
  0 as $status_canary_evidence_source_observation_count |
  7 as $status_canary_evidence_source_missing_count |
  0 as $status_canary_evidence_source_validated_count |
  0 as $status_canary_evidence_source_rejected_count |
  0 as $status_canary_evidence_source_generated_request_count |
  false as $status_canary_start_guard_switch_enabled |
  0 as $status_canary_start_guard_evidence_packet_reason_audit_count |
  0 as $status_canary_start_guard_evidence_packet_reason_audit_ready_count |
  0 as $status_canary_start_guard_evidence_packet_reason_audit_rejected_count |
  true as $status_canary_start_guard_evidence_packet_reason_audit_ready |
  true as $status_canary_start_guard_blocked |
  false as $status_canary_start_guard_allowed |
  "status_canary_start_blocked_missing_evidence_packet" as $status_canary_start_guard_route |
  false as $status_canary_start_request_present |
  "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp" as $status_canary_start_request_requested_tool_id |
  true as $status_canary_start_request_selected_status_canary |
  false as $status_canary_start_request_preflight_only_connector |
  true as $status_canary_start_request_source_start_guard_reason_audit_ready |
  true as $status_canary_start_request_blocked |
  false as $status_canary_start_request_allowed |
  "status_canary_start_request_blocked_no_request" as $status_canary_start_request_gate_route |
  false as $status_canary_runner_adapter_request_present |
  true as $status_canary_runner_adapter_source_gate_bound |
  true as $status_canary_runner_adapter_source_start_guard_reason_audit_ready |
  false as $status_canary_runner_adapter_source_start_request_allowed |
  true as $status_canary_runner_adapter_blocked |
  false as $status_canary_runner_adapter_allowed |
  "status_canary_runner_adapter_blocked_no_runner_request" as $status_canary_runner_adapter_route |
  false as $status_canary_runner_start_request_present |
  true as $status_canary_runner_start_surface_source_adapter_bound |
  true as $status_canary_runner_start_surface_source_start_guard_reason_audit_ready |
  false as $status_canary_runner_start_surface_source_adapter_allowed |
  true as $status_canary_runner_start_surface_blocked |
  false as $status_canary_runner_start_surface_allowed |
  "status_canary_runner_start_surface_blocked_no_start_request" as $status_canary_runner_start_surface_route |
  false as $status_canary_runner_entry_request_present |
  true as $status_canary_runner_entry_boundary_source_start_surface_bound |
  true as $status_canary_runner_entry_boundary_source_start_guard_reason_audit_ready |
  false as $status_canary_runner_entry_boundary_source_start_surface_allowed |
  true as $status_canary_runner_entry_boundary_blocked |
  false as $status_canary_runner_entry_boundary_allowed |
  "status_canary_runner_entry_boundary_blocked_no_entry_request" as $status_canary_runner_entry_boundary_route |
  false as $status_canary_runner_entry_adapter_request_present |
  true as $status_canary_runner_entry_adapter_source_boundary_bound |
  true as $status_canary_runner_entry_adapter_source_start_guard_reason_audit_ready |
  false as $status_canary_runner_entry_adapter_source_boundary_allowed |
  true as $status_canary_runner_entry_adapter_blocked |
  false as $status_canary_runner_entry_adapter_allowed |
  "status_canary_runner_entry_adapter_blocked_no_adapter_request" as $status_canary_runner_entry_adapter_route |
  false as $status_canary_runner_binding_request_present |
  true as $status_canary_runner_binding_guard_source_entry_adapter_bound |
  true as $status_canary_runner_binding_guard_source_start_guard_reason_audit_ready |
  false as $status_canary_runner_binding_guard_source_entry_adapter_allowed |
  true as $status_canary_runner_binding_guard_blocked |
  false as $status_canary_runner_binding_guard_allowed |
  "status_canary_runner_binding_guard_blocked_no_binding_request" as $status_canary_runner_binding_guard_route |
  false as $status_canary_runner_dry_run_selector_request_present |
  true as $status_canary_runner_dry_run_selector_source_binding_guard_bound |
  true as $status_canary_runner_dry_run_selector_source_start_guard_reason_audit_ready |
  false as $status_canary_runner_dry_run_selector_source_binding_guard_allowed |
  true as $status_canary_runner_dry_run_selector_blocked |
  false as $status_canary_runner_dry_run_selector_allowed |
  "status_canary_runner_dry_run_selector_blocked_no_selector_request" as $status_canary_runner_dry_run_selector_route |
  ($evidence_plan.evidence_collection_plan_ready == true
    and $status_canary_evidence_packet_item_count == 7
    and $status_canary_evidence_packet_missing_count == 7
    and $status_canary_evidence_packet_recorded_count == 0
    and $status_canary_evidence_packet_waived_count == 0
    and $status_canary_evidence_packet_expired_count == 0
    and $status_canary_evidence_packet_invalid_count == 0
    and $status_canary_evidence_packet_decision_reason_audit_count == 0
    and $status_canary_evidence_packet_decision_reason_audit_ready_count == 0
    and $status_canary_evidence_packet_decision_reason_audit_rejected_count == 0
    and $status_canary_start_blocked_by_evidence_packet == true
    and $status_canary_start_allowed_by_evidence_packet == false) as $status_canary_evidence_packet_ready |
  ($evidence_acceptance_packet_source_present == true
    and $status_canary_evidence_acceptance_packet_ready == true
    and $status_canary_evidence_acceptance_packet_id == "status-canary-evidence-acceptance-packet/hepta-system-status/v1"
    and $status_canary_evidence_acceptance_packet_route == "status_canary_evidence_acceptance_packet_ready_no_decision_requests"
    and $status_canary_evidence_acceptance_request_count == 0
    and $status_canary_evidence_acceptance_known_request_count == 0
    and $status_canary_evidence_acceptance_unknown_request_count == 0
    and $status_canary_evidence_acceptance_duplicate_request_count == 0
    and $status_canary_evidence_acceptance_request_source_validator_bound_count == 0
    and $status_canary_evidence_acceptance_request_source_validator_contract_audit_ready_count == 0
    and $status_canary_evidence_acceptance_request_reason_audit_count == 0
    and $status_canary_evidence_acceptance_request_reason_audit_ready_count == 0
    and $status_canary_evidence_acceptance_request_reason_audit_rejected_count == 0
    and $status_canary_evidence_acceptance_accepted_decision_count == 0
    and $status_canary_evidence_acceptance_rejected_decision_count == 0
    and $status_canary_evidence_acceptance_generated_override_count == 0
    and $status_canary_evidence_acceptance_generated_override_reason_audit_ready_count == 0
    and $status_canary_evidence_packet_missing_count == 7
    and $status_canary_evidence_packet_recorded_count == 0
    and $status_canary_evidence_packet_waived_count == 0
    and $status_canary_evidence_packet_expired_count == 0
    and $status_canary_evidence_packet_invalid_count == 0) as $status_canary_evidence_acceptance_ready |
  ($evidence_source_adapter_source_present == true
    and $status_canary_evidence_source_adapter_report_ready == true
    and $status_canary_evidence_source_adapter_id == "status-canary-evidence-source-adapter/hepta-system-status/v1"
    and $status_canary_evidence_source_adapter_route == "status_canary_evidence_source_adapter_ready_no_inputs"
    and $status_canary_evidence_source_adapter_count == 7
    and $status_canary_evidence_source_adapter_input_count == 0
    and $status_canary_evidence_source_adapter_generated_fixture_count == 0
    and $status_canary_evidence_source_adapter_missing_input_count == 7
    and $status_canary_evidence_source_adapter_metadata_contract_count == 7
    and $status_canary_evidence_source_adapter_metadata_contract_ready_count == 7
    and $status_canary_evidence_source_adapter_input_contract_field_count == 21
    and $status_canary_evidence_source_adapter_readback_fixture_contract_field_count == 70
    and $status_canary_evidence_source_adapter_required_field_validator_count == 7
    and $status_canary_evidence_source_adapter_required_field_validator_ready_count == 7
    and $status_canary_evidence_source_adapter_required_field_rejected_count == 0
    and $status_canary_evidence_source_adapter_missing_required_field_count == 0) as $status_canary_evidence_source_adapter_ready |
  ($evidence_source_reason_packet_source_present == true
    and $status_canary_evidence_source_adapter_ready == true
    and $status_canary_evidence_source_reason_packet_report_ready == true
    and $status_canary_evidence_source_reason_packet_id == "status-canary-evidence-source-reason-packet/hepta-system-status/v1"
    and $status_canary_evidence_source_reason_packet_route == "status_canary_evidence_source_reason_packet_ready_no_adapter_inputs"
    and $status_canary_evidence_source_reason_packet_source_count == 7
    and $status_canary_evidence_source_decision_reason_count == 28
    and $status_canary_evidence_source_decision_reason_ready_count == 28
    and $status_canary_evidence_source_decision_required_field_count == 84
    and $status_canary_evidence_source_missing_required_field_reason_count == 84
    and $status_canary_evidence_source_adapter_input_missing_reason_count == 28
    and $status_canary_evidence_source_adapter_input_other_decision_reason_count == 0
    and $status_canary_evidence_source_adapter_rejection_reason_count == 0
    and $status_canary_evidence_source_fixture_generation_allowed_count == 0
    and $status_canary_evidence_source_fixture_generation_blocked_count == 28) as $status_canary_evidence_source_reason_packet_ready |
  ($evidence_source_readback_source_present == true
    and $status_canary_evidence_source_adapter_ready == true
    and $status_canary_evidence_source_reason_packet_ready == true
    and $status_canary_evidence_source_readback_report_ready == true
    and $status_canary_evidence_source_readback_id == "status-canary-evidence-source-readback/hepta-system-status/v1"
    and $status_canary_evidence_source_readback_route == "status_canary_evidence_source_readback_ready_no_fixtures"
    and $status_canary_evidence_source_readback_fixture_count == 0
    and $status_canary_evidence_source_readback_observation_count == 0
    and $status_canary_evidence_source_readback_missing_observation_count == 7
    and $status_canary_evidence_source_readback_contract_audit_count == 7
    and $status_canary_evidence_source_readback_contract_audit_ready_count == 7
    and $status_canary_evidence_source_readback_fixture_contract_audit_ready_count == 0
    and $status_canary_evidence_source_readback_reason_packet_bound == true
    and $status_canary_evidence_source_readback_reason_packet_ready == true
    and $status_canary_evidence_source_readback_reason_packet_route == "status_canary_evidence_source_reason_packet_ready_no_adapter_inputs"
    and $status_canary_evidence_source_readback_fixture_reason_audit_count == 0
    and $status_canary_evidence_source_readback_fixture_reason_audit_ready_count == 0
    and $status_canary_evidence_source_readback_fixture_reason_audit_rejected_count == 0
    and $status_canary_evidence_acceptance_request_count == 0
    and $status_canary_evidence_acceptance_generated_override_count == 0) as $status_canary_evidence_source_readback_ready |
  ($evidence_source_validator_source_present == true
    and $status_canary_evidence_source_readback_ready == true
    and $status_canary_evidence_source_validator_ready == true
    and $status_canary_evidence_source_validator_id == "status-canary-evidence-source-validator/hepta-system-status/v1"
    and $status_canary_evidence_source_validator_route == "status_canary_evidence_source_validator_ready_no_observations"
    and $status_canary_evidence_source_validator_contract_audit_count == 0
    and $status_canary_evidence_source_validator_contract_audit_ready_count == 0
    and $status_canary_evidence_source_validator_contract_audit_rejected_count == 0
    and $status_canary_evidence_source_validator_reason_audit_count == 0
    and $status_canary_evidence_source_validator_reason_audit_ready_count == 0
    and $status_canary_evidence_source_validator_reason_audit_rejected_count == 0
    and $status_canary_evidence_source_observation_count == 0
    and $status_canary_evidence_source_missing_count == 7
    and $status_canary_evidence_source_validated_count == 0
    and $status_canary_evidence_source_rejected_count == 0
    and $status_canary_evidence_source_generated_request_count == 0
    and $status_canary_evidence_acceptance_request_count == 0
    and $status_canary_evidence_acceptance_generated_override_count == 0) as $status_canary_evidence_source_ready |
  ($start_guard_source_present == true
    and $status_canary_evidence_packet_ready == true
    and $status_canary_evidence_packet_complete == false
    and $status_canary_evidence_packet_missing_count == 7
    and $status_canary_evidence_packet_recorded_count == 0
    and $status_canary_evidence_packet_waived_count == 0
    and $status_canary_evidence_packet_expired_count == 0
    and $status_canary_evidence_packet_invalid_count == 0
    and $status_canary_start_guard_evidence_packet_reason_audit_count == 0
    and $status_canary_start_guard_evidence_packet_reason_audit_ready_count == 0
    and $status_canary_start_guard_evidence_packet_reason_audit_rejected_count == 0
    and $status_canary_start_guard_evidence_packet_reason_audit_ready == true
    and $status_canary_start_guard_switch_enabled == false
    and $status_canary_start_guard_blocked == true
    and $status_canary_start_guard_allowed == false
    and $status_canary_start_guard_route == "status_canary_start_blocked_missing_evidence_packet") as $status_canary_start_guard_ready |
  ($start_request_gate_source_present == true
    and $status_canary_start_guard_ready == true
    and $status_canary_start_request_present == false
    and $status_canary_start_request_requested_tool_id == "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp"
    and $status_canary_start_request_selected_status_canary == true
    and $status_canary_start_request_preflight_only_connector == false
    and $status_canary_start_request_source_start_guard_reason_audit_ready == true
    and $status_canary_start_request_blocked == true
    and $status_canary_start_request_allowed == false
    and $status_canary_start_request_gate_route == "status_canary_start_request_blocked_no_request") as $status_canary_start_request_gate_ready |
  ($runner_adapter_source_present == true
    and $status_canary_start_request_gate_ready == true
    and $status_canary_runner_adapter_request_present == false
    and $status_canary_runner_adapter_source_gate_bound == true
    and $status_canary_runner_adapter_source_start_guard_reason_audit_ready == true
    and $status_canary_runner_adapter_source_start_request_allowed == false
    and $status_canary_runner_adapter_blocked == true
    and $status_canary_runner_adapter_allowed == false
    and $status_canary_runner_adapter_route == "status_canary_runner_adapter_blocked_no_runner_request") as $status_canary_runner_adapter_ready |
  ($runner_start_surface_source_present == true
    and $status_canary_runner_adapter_ready == true
    and $status_canary_runner_start_request_present == false
    and $status_canary_runner_start_surface_source_adapter_bound == true
    and $status_canary_runner_start_surface_source_start_guard_reason_audit_ready == true
    and $status_canary_runner_start_surface_source_adapter_allowed == false
    and $status_canary_runner_start_surface_blocked == true
    and $status_canary_runner_start_surface_allowed == false
    and $status_canary_runner_start_surface_route == "status_canary_runner_start_surface_blocked_no_start_request") as $status_canary_runner_start_surface_ready |
  ($runner_entry_boundary_source_present == true
    and $status_canary_runner_start_surface_ready == true
    and $status_canary_runner_entry_request_present == false
    and $status_canary_runner_entry_boundary_source_start_surface_bound == true
    and $status_canary_runner_entry_boundary_source_start_guard_reason_audit_ready == true
    and $status_canary_runner_entry_boundary_source_start_surface_allowed == false
    and $status_canary_runner_entry_boundary_blocked == true
    and $status_canary_runner_entry_boundary_allowed == false
    and $status_canary_runner_entry_boundary_route == "status_canary_runner_entry_boundary_blocked_no_entry_request") as $status_canary_runner_entry_boundary_ready |
  ($runner_entry_adapter_source_present == true
    and $status_canary_runner_entry_boundary_ready == true
    and $status_canary_runner_entry_adapter_request_present == false
    and $status_canary_runner_entry_adapter_source_boundary_bound == true
    and $status_canary_runner_entry_adapter_source_start_guard_reason_audit_ready == true
    and $status_canary_runner_entry_adapter_source_boundary_allowed == false
    and $status_canary_runner_entry_adapter_blocked == true
    and $status_canary_runner_entry_adapter_allowed == false
    and $status_canary_runner_entry_adapter_route == "status_canary_runner_entry_adapter_blocked_no_adapter_request") as $status_canary_runner_entry_adapter_ready |
  ($runner_binding_guard_source_present == true
    and $status_canary_runner_entry_adapter_ready == true
    and $status_canary_runner_binding_request_present == false
    and $status_canary_runner_binding_guard_source_entry_adapter_bound == true
    and $status_canary_runner_binding_guard_source_start_guard_reason_audit_ready == true
    and $status_canary_runner_binding_guard_source_entry_adapter_allowed == false
    and $status_canary_runner_binding_guard_blocked == true
    and $status_canary_runner_binding_guard_allowed == false
    and $status_canary_runner_binding_guard_route == "status_canary_runner_binding_guard_blocked_no_binding_request") as $status_canary_runner_binding_guard_ready |
  ($runner_dry_run_selector_source_present == true
    and $status_canary_runner_binding_guard_ready == true
    and $status_canary_runner_dry_run_selector_request_present == false
    and $status_canary_runner_dry_run_selector_source_binding_guard_bound == true
    and $status_canary_runner_dry_run_selector_source_start_guard_reason_audit_ready == true
    and $status_canary_runner_dry_run_selector_source_binding_guard_allowed == false
    and $status_canary_runner_dry_run_selector_blocked == true
    and $status_canary_runner_dry_run_selector_allowed == false
    and $status_canary_runner_dry_run_selector_route == "status_canary_runner_dry_run_selector_blocked_no_selector_request") as $status_canary_runner_dry_run_selector_ready |
  ($single_render.single_render_cache_boundary_readback_ready == true
    and $single_render.source_matrix_ready == true
    and $single_render.source_matrix_capability_count == 104
    and $single_render.source_matrix_ready_count == 104
    and $single_render.source_live_enabled_count == 0
    and $single_render.source_all_live_paths_blocked == true
    and $single_render.dashboard_consumer_rewired == true
    and $kill.kill_switch_rehearsal_boundary_readback_ready == true
    and $kill.kill_switch_rehearsal_boundary_entry_count == 7
    and $kill.kill_switch_rehearsal_boundary_ready_count == 7
    and $kill.kill_switch_rehearsal_boundary_closed_count == 7
    and $kill.kill_switch_rehearsal_execution_blocked_count == 7
    and $kill.kill_switch_mutation_blocked_count == 7
    and $kill.kill_switch_rehearsal_recording_blocked_count == 7
    and $kill.kill_switch_rehearsal_receipt_persistence_blocked_count == 7
    and $kill.kill_switch_rehearsal_evidence_missing_count == 7
    and $kill.evidence_recorded_count == 0
    and $kill.blocker_waived_count == 0
    and $kill.approval_request_sent == false
    and $kill.approval_accepted == false
    and $kill.credential_read_allowed == false
    and $kill.transport_mutation_allowed == false
    and $kill.packet_persisted == false
    and $kill.attachment_persisted == false
    and $kill.readback_persisted == false
    and $kill.live_execution_allowed == false
    and $closure.tool_execution_live_cutover_closure_index_ready == true
    and $closure.tool_execution_live_cutover_allowed == false
    and $closure.tool_execution_public_ga_allowed == false
    and $closure.closure_blocker_count == 17
    and $closure.closure_blocker_category_count == 4
    and $closure.closure_blocker_category_ready_count == 4
    and $closure.closure_blocker_category_blocker_count == 17
    and $closure.closure_blocker_categorization_ready == true
    and $closure.final_gate_ready_count == 2
    and $closure.selected_status_canary_count == 1
    and $closure.preflight_only_non_selected_count == 1
    and $closure.explicit_live_cutover_approval_missing_count == 1
    and $closure.live_cutover_blocked_count == 1
    and $closure.execution_switch_blocked_count == 1
    and $closure.rollback_execution_blocked_count == 1
    and $closure.result_receipt_write_blocked_count == 1
    and $evidence_plan.evidence_collection_plan_ready == true
    and $lib_export_present == true
    and ($entries | length) == 7
    and $operator_visible_blocker_count == 7
    and $missing_evidence_blocker_count == 7
    and $accepted_blocker_count == 0
    and $waived_blocker_count == 0
    and $evidence_recorded_count == 0
    and ($status_canary_evidence_closure_entries | length) == 7
    and $status_canary_evidence_closure_ready_count == 7
    and $status_canary_evidence_closure_missing_count == 7
    and $status_canary_evidence_closure_recorded_count == 0
    and $status_canary_evidence_closure_waived_count == 0
    and $status_canary_evidence_closure_actionable_precondition_count == 7
    and $status_canary_evidence_packet_ready == true
    and $status_canary_evidence_packet_item_count == 7
    and $status_canary_evidence_packet_missing_count == 7
    and $status_canary_evidence_packet_recorded_count == 0
    and $status_canary_evidence_packet_waived_count == 0
    and $status_canary_evidence_packet_expired_count == 0
    and $status_canary_evidence_packet_invalid_count == 0
    and $status_canary_evidence_packet_complete == false
    and $status_canary_start_blocked_by_evidence_packet == true
    and $status_canary_start_allowed_by_evidence_packet == false
    and $status_canary_evidence_packet_guard_route == "status_canary_evidence_packet_blocked_missing_evidence"
    and $status_canary_evidence_acceptance_ready == true
    and $status_canary_evidence_source_adapter_ready == true
    and $status_canary_evidence_source_reason_packet_ready == true
    and $status_canary_evidence_source_readback_ready == true
    and $status_canary_evidence_source_ready == true
    and $status_canary_start_guard_ready == true
    and $status_canary_start_request_gate_ready == true
    and $status_canary_runner_adapter_ready == true
    and $status_canary_runner_start_surface_ready == true
    and $status_canary_runner_entry_boundary_ready == true
    and $status_canary_runner_entry_adapter_ready == true
    and $status_canary_runner_binding_guard_ready == true
    and $status_canary_runner_dry_run_selector_ready == true
    and ($entries | all(.queryable == true
      and .diffable == true
      and .operator_status == "blocked_missing_evidence"
      and .evidence_state == "missing"
      and .acceptance_allowed == false
      and .waiver_allowed == false
      and .evidence_recording_allowed == false
      and .credential_read_allowed == false
      and .transport_mutation_allowed == false
      and .persistence_allowed == false
      and .live_mutation_allowed == false))) as $dashboard_ready |
  {
    runtime:"hepta",
    surface:"controlled_live_operator_readiness_dashboard",
    status:(if $dashboard_ready then "ready_blocked" else "blocked" end),
    gate:"controlled_live_operator_readiness_dashboard_gate",
    schema_version:"controlled_live_operator_readiness_dashboard_v1",
    plugin_id:"hepta-system@hepta-local",
    source_single_render_cache_boundary_ready:$single_render.single_render_cache_boundary_readback_ready,
    source_matrix_ready:$single_render.source_matrix_ready,
    source_matrix_capability_count:$single_render.source_matrix_capability_count,
    source_matrix_capability_ready_count:$single_render.source_matrix_ready_count,
    source_matrix_live_enabled_count:$single_render.source_live_enabled_count,
    source_matrix_all_live_paths_blocked:$single_render.source_all_live_paths_blocked,
    source_matrix_next_migration_step:$single_render.recommended_next_gate,
    source_kill_switch_boundary_readback_ready:$kill.kill_switch_rehearsal_boundary_readback_ready,
    source_kill_switch_boundary_entry_count:$kill.kill_switch_rehearsal_boundary_entry_count,
    source_kill_switch_boundary_ready_count:$kill.kill_switch_rehearsal_boundary_ready_count,
    source_live_cutover_closure_index_surface:$closure.surface,
    source_live_cutover_closure_index_ready:$closure.tool_execution_live_cutover_closure_index_ready,
    source_live_cutover_final_gate_ready_count:$closure.final_gate_ready_count,
    source_live_cutover_closure_blocker_count:$closure.closure_blocker_count,
    source_live_cutover_closure_blocker_category_count:$closure.closure_blocker_category_count,
    source_live_cutover_closure_blocker_category_ready_count:$closure.closure_blocker_category_ready_count,
    source_live_cutover_closure_blocker_category_blocker_count:$closure.closure_blocker_category_blocker_count,
    source_live_cutover_closure_blocker_categorization_ready:$closure.closure_blocker_categorization_ready,
    source_live_cutover_closure_blocker_categories:$closure.closure_blocker_categories,
    source_required_evidence_collection_plan_ready:$evidence_plan.evidence_collection_plan_ready,
    status_canary_final_guard_present:true,
    status_canary_tool_id:"preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp",
    preflight_only_connector_tool_id:"preview:connector:hepta-system@hepta-local:hepta_system_local_app",
    status_canary_candidate_count:$closure.candidate_count,
    selected_status_canary_count:$closure.selected_status_canary_count,
    preflight_only_non_selected_count:$closure.preflight_only_non_selected_count,
    status_canary_final_gate_ready_count:$closure.final_gate_ready_count,
    status_canary_final_guard_live_blocked_count:$closure.live_cutover_blocked_count,
    status_canary_final_guard_approval_missing_count:$closure.explicit_live_cutover_approval_missing_count,
    status_canary_final_guard_live_enabled:false,
    status_canary_final_guard_tool_invocation_enabled:false,
    status_canary_final_guard_ledger_write_enabled:false,
    status_canary_evidence_packet_ready:$status_canary_evidence_packet_ready,
    status_canary_evidence_packet_id:"status-canary-evidence-packet/hepta-system-status/v1",
    status_canary_evidence_packet_item_count:$status_canary_evidence_packet_item_count,
    status_canary_evidence_packet_missing_count:$status_canary_evidence_packet_missing_count,
    status_canary_evidence_packet_recorded_count:$status_canary_evidence_packet_recorded_count,
    status_canary_evidence_packet_waived_count:$status_canary_evidence_packet_waived_count,
    status_canary_evidence_packet_expired_count:$status_canary_evidence_packet_expired_count,
    status_canary_evidence_packet_invalid_count:$status_canary_evidence_packet_invalid_count,
    status_canary_evidence_packet_decision_reason_audit_count:$status_canary_evidence_packet_decision_reason_audit_count,
    status_canary_evidence_packet_decision_reason_audit_ready_count:$status_canary_evidence_packet_decision_reason_audit_ready_count,
    status_canary_evidence_packet_decision_reason_audit_rejected_count:$status_canary_evidence_packet_decision_reason_audit_rejected_count,
    status_canary_evidence_packet_complete:$status_canary_evidence_packet_complete,
    status_canary_start_blocked_by_evidence_packet:$status_canary_start_blocked_by_evidence_packet,
    status_canary_start_allowed_by_evidence_packet:$status_canary_start_allowed_by_evidence_packet,
    status_canary_evidence_packet_guard_route:$status_canary_evidence_packet_guard_route,
    status_canary_evidence_acceptance_packet_ready:$status_canary_evidence_acceptance_ready,
    status_canary_evidence_acceptance_packet_id:$status_canary_evidence_acceptance_packet_id,
    status_canary_evidence_acceptance_packet_route:$status_canary_evidence_acceptance_packet_route,
    status_canary_evidence_acceptance_request_count:$status_canary_evidence_acceptance_request_count,
    status_canary_evidence_acceptance_known_request_count:$status_canary_evidence_acceptance_known_request_count,
    status_canary_evidence_acceptance_unknown_request_count:$status_canary_evidence_acceptance_unknown_request_count,
    status_canary_evidence_acceptance_duplicate_request_count:$status_canary_evidence_acceptance_duplicate_request_count,
    status_canary_evidence_acceptance_request_source_validator_bound_count:$status_canary_evidence_acceptance_request_source_validator_bound_count,
    status_canary_evidence_acceptance_request_source_validator_contract_audit_ready_count:$status_canary_evidence_acceptance_request_source_validator_contract_audit_ready_count,
    status_canary_evidence_acceptance_request_reason_audit_count:$status_canary_evidence_acceptance_request_reason_audit_count,
    status_canary_evidence_acceptance_request_reason_audit_ready_count:$status_canary_evidence_acceptance_request_reason_audit_ready_count,
    status_canary_evidence_acceptance_request_reason_audit_rejected_count:$status_canary_evidence_acceptance_request_reason_audit_rejected_count,
    status_canary_evidence_acceptance_accepted_decision_count:$status_canary_evidence_acceptance_accepted_decision_count,
    status_canary_evidence_acceptance_rejected_decision_count:$status_canary_evidence_acceptance_rejected_decision_count,
    status_canary_evidence_acceptance_generated_override_count:$status_canary_evidence_acceptance_generated_override_count,
    status_canary_evidence_acceptance_generated_override_reason_audit_ready_count:$status_canary_evidence_acceptance_generated_override_reason_audit_ready_count,
    status_canary_evidence_source_adapter_ready:$status_canary_evidence_source_adapter_ready,
    status_canary_evidence_source_adapter_id:$status_canary_evidence_source_adapter_id,
    status_canary_evidence_source_adapter_route:$status_canary_evidence_source_adapter_route,
    status_canary_evidence_source_adapter_count:$status_canary_evidence_source_adapter_count,
    status_canary_evidence_source_adapter_input_count:$status_canary_evidence_source_adapter_input_count,
    status_canary_evidence_source_adapter_generated_fixture_count:$status_canary_evidence_source_adapter_generated_fixture_count,
    status_canary_evidence_source_adapter_missing_input_count:$status_canary_evidence_source_adapter_missing_input_count,
    status_canary_evidence_source_adapter_metadata_contract_count:$status_canary_evidence_source_adapter_metadata_contract_count,
    status_canary_evidence_source_adapter_metadata_contract_ready_count:$status_canary_evidence_source_adapter_metadata_contract_ready_count,
    status_canary_evidence_source_adapter_input_contract_field_count:$status_canary_evidence_source_adapter_input_contract_field_count,
    status_canary_evidence_source_adapter_readback_fixture_contract_field_count:$status_canary_evidence_source_adapter_readback_fixture_contract_field_count,
    status_canary_evidence_source_adapter_required_field_validator_count:$status_canary_evidence_source_adapter_required_field_validator_count,
    status_canary_evidence_source_adapter_required_field_validator_ready_count:$status_canary_evidence_source_adapter_required_field_validator_ready_count,
    status_canary_evidence_source_adapter_required_field_rejected_count:$status_canary_evidence_source_adapter_required_field_rejected_count,
    status_canary_evidence_source_adapter_missing_required_field_count:$status_canary_evidence_source_adapter_missing_required_field_count,
    status_canary_evidence_source_reason_packet_ready:$status_canary_evidence_source_reason_packet_ready,
    status_canary_evidence_source_reason_packet_id:$status_canary_evidence_source_reason_packet_id,
    status_canary_evidence_source_reason_packet_route:$status_canary_evidence_source_reason_packet_route,
    status_canary_evidence_source_reason_packet_source_count:$status_canary_evidence_source_reason_packet_source_count,
    status_canary_evidence_source_decision_reason_count:$status_canary_evidence_source_decision_reason_count,
    status_canary_evidence_source_decision_reason_ready_count:$status_canary_evidence_source_decision_reason_ready_count,
    status_canary_evidence_source_decision_required_field_count:$status_canary_evidence_source_decision_required_field_count,
    status_canary_evidence_source_missing_required_field_reason_count:$status_canary_evidence_source_missing_required_field_reason_count,
    status_canary_evidence_source_adapter_input_missing_reason_count:$status_canary_evidence_source_adapter_input_missing_reason_count,
    status_canary_evidence_source_adapter_input_other_decision_reason_count:$status_canary_evidence_source_adapter_input_other_decision_reason_count,
    status_canary_evidence_source_adapter_rejection_reason_count:$status_canary_evidence_source_adapter_rejection_reason_count,
    status_canary_evidence_source_fixture_generation_allowed_count:$status_canary_evidence_source_fixture_generation_allowed_count,
    status_canary_evidence_source_fixture_generation_blocked_count:$status_canary_evidence_source_fixture_generation_blocked_count,
    status_canary_evidence_source_readback_ready:$status_canary_evidence_source_readback_ready,
    status_canary_evidence_source_readback_id:$status_canary_evidence_source_readback_id,
    status_canary_evidence_source_readback_route:$status_canary_evidence_source_readback_route,
    status_canary_evidence_source_readback_fixture_count:$status_canary_evidence_source_readback_fixture_count,
    status_canary_evidence_source_readback_observation_count:$status_canary_evidence_source_readback_observation_count,
    status_canary_evidence_source_readback_missing_observation_count:$status_canary_evidence_source_readback_missing_observation_count,
    status_canary_evidence_source_readback_contract_audit_count:$status_canary_evidence_source_readback_contract_audit_count,
    status_canary_evidence_source_readback_contract_audit_ready_count:$status_canary_evidence_source_readback_contract_audit_ready_count,
    status_canary_evidence_source_readback_fixture_contract_audit_ready_count:$status_canary_evidence_source_readback_fixture_contract_audit_ready_count,
    status_canary_evidence_source_readback_reason_packet_bound:$status_canary_evidence_source_readback_reason_packet_bound,
    status_canary_evidence_source_readback_reason_packet_ready:$status_canary_evidence_source_readback_reason_packet_ready,
    status_canary_evidence_source_readback_reason_packet_route:$status_canary_evidence_source_readback_reason_packet_route,
    status_canary_evidence_source_readback_fixture_reason_audit_count:$status_canary_evidence_source_readback_fixture_reason_audit_count,
    status_canary_evidence_source_readback_fixture_reason_audit_ready_count:$status_canary_evidence_source_readback_fixture_reason_audit_ready_count,
    status_canary_evidence_source_readback_fixture_reason_audit_rejected_count:$status_canary_evidence_source_readback_fixture_reason_audit_rejected_count,
    status_canary_evidence_source_validator_ready:$status_canary_evidence_source_ready,
    status_canary_evidence_source_validator_id:$status_canary_evidence_source_validator_id,
    status_canary_evidence_source_validator_route:$status_canary_evidence_source_validator_route,
    status_canary_evidence_source_validator_contract_audit_count:$status_canary_evidence_source_validator_contract_audit_count,
    status_canary_evidence_source_validator_contract_audit_ready_count:$status_canary_evidence_source_validator_contract_audit_ready_count,
    status_canary_evidence_source_validator_contract_audit_rejected_count:$status_canary_evidence_source_validator_contract_audit_rejected_count,
    status_canary_evidence_source_validator_reason_audit_count:$status_canary_evidence_source_validator_reason_audit_count,
    status_canary_evidence_source_validator_reason_audit_ready_count:$status_canary_evidence_source_validator_reason_audit_ready_count,
    status_canary_evidence_source_validator_reason_audit_rejected_count:$status_canary_evidence_source_validator_reason_audit_rejected_count,
    status_canary_evidence_source_observation_count:$status_canary_evidence_source_observation_count,
    status_canary_evidence_source_missing_count:$status_canary_evidence_source_missing_count,
    status_canary_evidence_source_validated_count:$status_canary_evidence_source_validated_count,
    status_canary_evidence_source_rejected_count:$status_canary_evidence_source_rejected_count,
    status_canary_evidence_source_generated_request_count:$status_canary_evidence_source_generated_request_count,
    status_canary_start_guard_ready:$status_canary_start_guard_ready,
    status_canary_start_guard_id:"status-canary-start-guard/hepta-system-status/v1",
    status_canary_start_guard_route:$status_canary_start_guard_route,
    status_canary_start_guard_switch_enabled:$status_canary_start_guard_switch_enabled,
    status_canary_start_guard_evidence_packet_reason_audit_count:$status_canary_start_guard_evidence_packet_reason_audit_count,
    status_canary_start_guard_evidence_packet_reason_audit_ready_count:$status_canary_start_guard_evidence_packet_reason_audit_ready_count,
    status_canary_start_guard_evidence_packet_reason_audit_rejected_count:$status_canary_start_guard_evidence_packet_reason_audit_rejected_count,
    status_canary_start_guard_evidence_packet_reason_audit_ready:$status_canary_start_guard_evidence_packet_reason_audit_ready,
    status_canary_start_guard_blocked:$status_canary_start_guard_blocked,
    status_canary_start_guard_allowed:$status_canary_start_guard_allowed,
    status_canary_start_request_gate_ready:$status_canary_start_request_gate_ready,
    status_canary_start_request_gate_id:"status-canary-start-request-gate/hepta-system-status/v1",
    status_canary_start_request_gate_route:$status_canary_start_request_gate_route,
    status_canary_start_request_present:$status_canary_start_request_present,
    status_canary_start_request_requested_tool_id:$status_canary_start_request_requested_tool_id,
    status_canary_start_request_selected_status_canary:$status_canary_start_request_selected_status_canary,
    status_canary_start_request_preflight_only_connector:$status_canary_start_request_preflight_only_connector,
    status_canary_start_request_source_start_guard_reason_audit_ready:$status_canary_start_request_source_start_guard_reason_audit_ready,
    status_canary_start_request_blocked:$status_canary_start_request_blocked,
    status_canary_start_request_allowed:$status_canary_start_request_allowed,
    status_canary_runner_adapter_ready:$status_canary_runner_adapter_ready,
    status_canary_runner_adapter_id:"status-canary-runner-adapter/hepta-system-status/v1",
    status_canary_runner_adapter_route:$status_canary_runner_adapter_route,
    status_canary_runner_adapter_request_present:$status_canary_runner_adapter_request_present,
    status_canary_runner_adapter_source_gate_bound:$status_canary_runner_adapter_source_gate_bound,
    status_canary_runner_adapter_source_start_guard_reason_audit_ready:$status_canary_runner_adapter_source_start_guard_reason_audit_ready,
    status_canary_runner_adapter_source_start_request_allowed:$status_canary_runner_adapter_source_start_request_allowed,
    status_canary_runner_adapter_blocked:$status_canary_runner_adapter_blocked,
    status_canary_runner_adapter_allowed:$status_canary_runner_adapter_allowed,
    status_canary_runner_start_surface_ready:$status_canary_runner_start_surface_ready,
    status_canary_runner_start_surface_id:"status-canary-runner-start-surface/hepta-system-status/v1",
    status_canary_runner_start_surface_route:$status_canary_runner_start_surface_route,
    status_canary_runner_start_request_present:$status_canary_runner_start_request_present,
    status_canary_runner_start_surface_source_adapter_bound:$status_canary_runner_start_surface_source_adapter_bound,
    status_canary_runner_start_surface_source_start_guard_reason_audit_ready:$status_canary_runner_start_surface_source_start_guard_reason_audit_ready,
    status_canary_runner_start_surface_source_adapter_allowed:$status_canary_runner_start_surface_source_adapter_allowed,
    status_canary_runner_start_surface_blocked:$status_canary_runner_start_surface_blocked,
    status_canary_runner_start_surface_allowed:$status_canary_runner_start_surface_allowed,
    status_canary_runner_entry_boundary_ready:$status_canary_runner_entry_boundary_ready,
    status_canary_runner_entry_boundary_id:"status-canary-runner-entry-boundary/hepta-system-status/v1",
    status_canary_runner_entry_boundary_route:$status_canary_runner_entry_boundary_route,
    status_canary_runner_entry_request_present:$status_canary_runner_entry_request_present,
    status_canary_runner_entry_boundary_source_start_surface_bound:$status_canary_runner_entry_boundary_source_start_surface_bound,
    status_canary_runner_entry_boundary_source_start_guard_reason_audit_ready:$status_canary_runner_entry_boundary_source_start_guard_reason_audit_ready,
    status_canary_runner_entry_boundary_source_start_surface_allowed:$status_canary_runner_entry_boundary_source_start_surface_allowed,
    status_canary_runner_entry_boundary_blocked:$status_canary_runner_entry_boundary_blocked,
    status_canary_runner_entry_boundary_allowed:$status_canary_runner_entry_boundary_allowed,
    status_canary_runner_entry_adapter_ready:$status_canary_runner_entry_adapter_ready,
    status_canary_runner_entry_adapter_id:"status-canary-runner-entry-adapter/hepta-system-status/v1",
    status_canary_runner_entry_adapter_route:$status_canary_runner_entry_adapter_route,
    status_canary_runner_entry_adapter_request_present:$status_canary_runner_entry_adapter_request_present,
    status_canary_runner_entry_adapter_source_boundary_bound:$status_canary_runner_entry_adapter_source_boundary_bound,
    status_canary_runner_entry_adapter_source_start_guard_reason_audit_ready:$status_canary_runner_entry_adapter_source_start_guard_reason_audit_ready,
    status_canary_runner_entry_adapter_source_boundary_allowed:$status_canary_runner_entry_adapter_source_boundary_allowed,
    status_canary_runner_entry_adapter_blocked:$status_canary_runner_entry_adapter_blocked,
    status_canary_runner_entry_adapter_allowed:$status_canary_runner_entry_adapter_allowed,
    status_canary_runner_binding_guard_ready:$status_canary_runner_binding_guard_ready,
    status_canary_runner_binding_guard_id:"status-canary-runner-binding-guard/hepta-system-status/v1",
    status_canary_runner_binding_guard_route:$status_canary_runner_binding_guard_route,
    status_canary_runner_binding_request_present:$status_canary_runner_binding_request_present,
    status_canary_runner_binding_guard_source_entry_adapter_bound:$status_canary_runner_binding_guard_source_entry_adapter_bound,
    status_canary_runner_binding_guard_source_start_guard_reason_audit_ready:$status_canary_runner_binding_guard_source_start_guard_reason_audit_ready,
    status_canary_runner_binding_guard_source_entry_adapter_allowed:$status_canary_runner_binding_guard_source_entry_adapter_allowed,
    status_canary_runner_binding_guard_blocked:$status_canary_runner_binding_guard_blocked,
    status_canary_runner_binding_guard_allowed:$status_canary_runner_binding_guard_allowed,
    status_canary_runner_dry_run_selector_ready:$status_canary_runner_dry_run_selector_ready,
    status_canary_runner_dry_run_selector_id:"status-canary-runner-dry-run-selector/hepta-system-status/v1",
    status_canary_runner_dry_run_selector_route:$status_canary_runner_dry_run_selector_route,
    status_canary_runner_dry_run_selector_request_present:$status_canary_runner_dry_run_selector_request_present,
    status_canary_runner_dry_run_selector_source_binding_guard_bound:$status_canary_runner_dry_run_selector_source_binding_guard_bound,
    status_canary_runner_dry_run_selector_source_start_guard_reason_audit_ready:$status_canary_runner_dry_run_selector_source_start_guard_reason_audit_ready,
    status_canary_runner_dry_run_selector_source_binding_guard_allowed:$status_canary_runner_dry_run_selector_source_binding_guard_allowed,
    status_canary_runner_dry_run_selector_blocked:$status_canary_runner_dry_run_selector_blocked,
    status_canary_runner_dry_run_selector_allowed:$status_canary_runner_dry_run_selector_allowed,
    status_canary_evidence_closure_entry_count:($status_canary_evidence_closure_entries | length),
    status_canary_evidence_closure_ready_count:$status_canary_evidence_closure_ready_count,
    status_canary_evidence_closure_missing_count:$status_canary_evidence_closure_missing_count,
    status_canary_evidence_closure_recorded_count:$status_canary_evidence_closure_recorded_count,
    status_canary_evidence_closure_waived_count:$status_canary_evidence_closure_waived_count,
    status_canary_evidence_closure_actionable_precondition_count:$status_canary_evidence_closure_actionable_precondition_count,
    lib_export_present:$lib_export_present,
    capability_row_count:$single_render.source_matrix_capability_count,
    capability_ready_count:$single_render.source_matrix_ready_count,
    live_enabled_count:$single_render.source_live_enabled_count,
    all_live_paths_blocked:$single_render.source_all_live_paths_blocked,
    blocker_entry_count:($entries | length),
    operator_visible_blocker_count:$operator_visible_blocker_count,
    missing_evidence_blocker_count:$missing_evidence_blocker_count,
    accepted_blocker_count:$accepted_blocker_count,
    waived_blocker_count:$waived_blocker_count,
    evidence_recorded_count:$evidence_recorded_count,
    approval_request_sent:false,
    approval_accepted:false,
    credential_read_allowed:false,
    transport_mutation_allowed:false,
    persistence_allowed:false,
    live_execution_allowed:false,
    dashboard_ready:$dashboard_ready,
    entries:$entries,
    status_canary_evidence_closure_entries:$status_canary_evidence_closure_entries,
    next_actions:[
      "close_controlled_live_evidence_before_status_canary_start",
      "keep_status_canary_final_guard_read_only",
      "keep_connector_candidate_preflight_only"
    ],
    next_migration_step:"close_controlled_live_evidence_before_status_canary_start",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      approval_requested:false,
      approval_accepted:false,
      approval_recorded:false,
      evidence_recorded:false,
      evidence_persisted:false,
      blocker_waived:false,
      credential_read:false,
      transport_mutated:false,
      packet_persisted:false,
      attachment_persisted:false,
      readback_persisted:false,
      ledger_written:false,
      workflow_event_log_written:false,
      sqlite_written:false,
      provider_invoked:false,
      model_invoked:false,
      live_execution_started:false
    }
  }'
