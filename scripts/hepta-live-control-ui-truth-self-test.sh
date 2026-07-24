#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
source "$REPO_ROOT/scripts/lib/hepta-live-control-ui-truth-v1.sh"
source "$REPO_ROOT/scripts/lib/hepta-public-ga-blockers-v1.sh"

expected_public_ga_blockers="$(hepta_expected_public_ga_blockers_json)"
jq -e '
  length == 10
  and index("gateway_replacement_not_ready") != null
  and index("control_ui_live_truth_not_available_on_active_legacy_schema") != null
  and index("control_ui_product_behavior_evidence_not_bound") != null
  and (unique | length) == length
' >/dev/null <<<"$expected_public_ga_blockers"

assert_rejected() {
  local ga_json="$1"
  local merge_json="$2"
  local mode="$3"
  local label="$4"
  local contract

  contract="$(
    hepta_live_control_ui_truth_contract_json \
      "$ga_json" "$merge_json" "$mode"
  )"
  jq -e '.ready == false and .product_completion_claim_allowed == false' \
    >/dev/null <<<"$contract" || {
    echo "expected Control UI truth fixture rejection: $label" >&2
    return 1
  }
}

legacy_ga='{
  "status":"blocked",
  "public_ga_ready":false,
  "public_ga_claimed":false,
  "operator_approval_required":true,
  "production_replacement_percent":100,
  "blocker_count":1,
  "blockers":["gateway_replacement_not_ready"]
}'
legacy_merge='{
  "status":"attention",
  "public_ga_claimed":false,
  "production_replacement_percent":100,
  "readiness_class":"active_production_replacement_ready",
  "blockers":["telegram_owner_handoff_not_requested"]
}'

strict_legacy="$(
  hepta_live_control_ui_truth_contract_json \
    "$legacy_ga" "$legacy_merge" deployment-consistency
)"
jq -e '
  .ready == false
  and .control_ui_truth_checked == false
  and .schema_mode == "legacy_rejected_in_strict_mode"
  and .legacy_accepted == false
  and .production_semantics_checked == false
  and .product_completion_claim_allowed == false
  and .product_completion_claimed == false
' >/dev/null <<<"$strict_legacy"

active_legacy="$(
  hepta_live_control_ui_truth_contract_json \
    "$legacy_ga" "$legacy_merge" active-health
)"
jq -e '
  .ready == true
  and .legacy_active_only == true
  and .legacy_accepted == true
  and .control_ui_truth_checked == false
  and .schema_mode == "legacy_active_only"
  and .production_semantics_checked == false
  and .reports_sync_scope == "legacy_base_reports_only"
  and .product_completion_claim_allowed == false
  and .product_completion_claimed == false
' >/dev/null <<<"$active_legacy"

partial_ga="$(
  jq -cn '{
    production_replacement_percent:100,
    status:"blocked",
    public_ga_ready:false,
    public_ga_claimed:false,
    operator_approval_required:true,
    blocker_count:1,
    blockers:["gateway_replacement_not_ready"],
    control_ui_product_status:"static_contract_complete"
  }'
)"
partial_active="$(
  hepta_live_control_ui_truth_contract_json \
    "$partial_ga" "$legacy_merge" active-health
)"
jq -e '
  .ready == false
  and .partial_schema_present == true
  and .schema_mode == "partial_or_unknown"
' >/dev/null <<<"$partial_active"

current_evidence='{
  "schema_version":1,
  "static_contract":{"status":"verified","coverage_percent":100,"verified":true,"evidence_ref":"fixture:static-contract"},
  "unit_state":{"status":"not_bound_to_report","coverage_percent":0,"verified":false,"evidence_ref":null},
  "browser_behavior":{"status":"not_bound_to_report","coverage_percent":0,"verified":false,"evidence_ref":null},
  "backend_mutation_readback":{"status":"not_bound_to_report","coverage_percent":0,"verified":false,"evidence_ref":null},
  "live_adapter":{"status":"not_bound_to_report","coverage_percent":0,"verified":false,"evidence_ref":null},
  "overall_evidence_percent":20,
  "all_required_layers_verified":false,
  "boundary":"Source markers and declared smoke commands prove only the static contract. Unit/state runs, real browser behavior, backend mutation/readback, and live-adapter evidence must be bound explicitly before product completion or a 100% live operator surface may be claimed."
}'
current_ga="$(
  jq -cn '{
    production_replacement_percent:94,
    status:"blocked",
    public_ga_ready:false,
    public_ga_claimed:false,
    operator_approval_required:true,
    blocker_count:1,
    blockers:["control_ui_product_behavior_evidence_not_bound"],
    control_ui_product_status:"static_contract_complete",
    control_ui_product_complete:false,
    control_ui_live_operator_surface_percent:0,
    control_ui_overall_evidence_percent:20
  }'
)"
current_merge="$(
  jq -cn --argjson evidence "$current_evidence" '{
    production_replacement_percent:94,
    status:"attention",
    public_ga_claimed:false,
    blockers:["control_ui_product_behavior_evidence_not_bound"],
    control_ui_product_status:"static_contract_complete",
    control_ui_product_complete:false,
    control_ui_live_operator_surface_percent:0,
    control_ui_evidence:$evidence
  }'
)"

for mode in deployment-consistency active-health; do
  current_contract="$(
    hepta_live_control_ui_truth_contract_json \
      "$current_ga" "$current_merge" "$mode"
  )"
  jq -e '
    .ready == true
    and .current_truth_ready == true
    and .control_ui_truth_checked == true
    and .schema_mode == "current_truth_v1"
    and .legacy_active_only == false
    and .legacy_accepted == false
    and .production_semantics_checked == true
    and .reports_sync_scope == "full_including_control_ui_truth"
    and .product_completion_claim_allowed == false
    and .product_completion_claimed == false
  ' >/dev/null <<<"$current_contract"
done

ga_ui_keys=(
  control_ui_product_status
  control_ui_product_complete
  control_ui_live_operator_surface_percent
  control_ui_overall_evidence_percent
)
for key in "${ga_ui_keys[@]}"; do
  assert_rejected \
    "$(jq --arg key "$key" 'del(.[$key])' <<<"$current_ga")" \
    "$current_merge" active-health "current GA missing $key"
done

merge_ui_keys=(
  control_ui_product_status
  control_ui_product_complete
  control_ui_live_operator_surface_percent
  control_ui_evidence
)
for key in "${merge_ui_keys[@]}"; do
  assert_rejected \
    "$current_ga" \
    "$(jq --arg key "$key" 'del(.[$key])' <<<"$current_merge")" \
    active-health "current merge missing $key"
done

for filter in \
  '.status = "ready"' \
  '.public_ga_ready = true' \
  '.public_ga_claimed = true' \
  '.operator_approval_required = false' \
  '.blockers = [] | .blocker_count = 0' \
  '.blocker_count = 99'
do
  assert_rejected \
    "$(jq "$filter" <<<"$current_ga")" "$current_merge" \
    active-health "current GA denial truth flip: $filter"
done

for filter in \
  '.status = "ready"' \
  '.public_ga_claimed = true' \
  '.blockers = []'
do
  assert_rejected \
    "$current_ga" "$(jq "$filter" <<<"$current_merge")" \
    active-health "current merge denial truth flip: $filter"
done

for filter in \
  '.production_replacement_percent = null' \
  '.production_replacement_percent = "94"' \
  '.production_replacement_percent = -1' \
  '.production_replacement_percent = 94.5' \
  '.production_replacement_percent = 95'
do
  assert_rejected \
    "$(jq "$filter" <<<"$current_ga")" "$current_merge" \
    active-health "current GA production semantics flip: $filter"
done

for filter in \
  '.production_replacement_percent = null' \
  '.production_replacement_percent = "94"' \
  '.production_replacement_percent = -1' \
  '.production_replacement_percent = 94.5' \
  '.production_replacement_percent = 95'
do
  assert_rejected \
    "$current_ga" "$(jq "$filter" <<<"$current_merge")" \
    active-health "current merge production semantics flip: $filter"
done

for filter in \
  '.control_ui_product_status = "complete"' \
  '.control_ui_product_complete = true' \
  '.control_ui_live_operator_surface_percent = 100' \
  '.control_ui_overall_evidence_percent = 100' \
  '.blockers = ["different_blocker"] | .blocker_count = 1'
do
  assert_rejected \
    "$(jq "$filter" <<<"$current_ga")" "$current_merge" \
    active-health "current GA UI truth flip: $filter"
done

for filter in \
  '.control_ui_product_status = "complete"' \
  '.control_ui_product_complete = true' \
  '.control_ui_live_operator_surface_percent = 100' \
  '.blockers = ["different_blocker"]' \
  '.control_ui_evidence.schema_version = 2' \
  '.control_ui_evidence.static_contract.verified = false' \
  '.control_ui_evidence.static_contract.evidence_ref = null' \
  '.control_ui_evidence.static_contract.evidence_ref = ""' \
  'del(.control_ui_evidence.static_contract.verified)' \
  '.control_ui_evidence.unit_state.verified = true' \
  '.control_ui_evidence.unit_state.evidence_ref = "fixture:unit"' \
  '.control_ui_evidence.browser_behavior.verified = true' \
  '.control_ui_evidence.backend_mutation_readback.evidence_ref = "fixture:backend"' \
  '.control_ui_evidence.live_adapter.verified = true' \
  '.control_ui_evidence.overall_evidence_percent = 100' \
  '.control_ui_evidence.all_required_layers_verified = true' \
  '.control_ui_evidence.boundary = "product complete"'
do
  assert_rejected \
    "$current_ga" "$(jq "$filter" <<<"$current_merge")" \
    active-health "current merge evidence flip: $filter"
done

invalid_current_ga="$(
  jq '.control_ui_product_complete = true' <<<"$current_ga"
)"
invalid_active="$(
  hepta_live_control_ui_truth_contract_json \
    "$invalid_current_ga" "$current_merge" active-health
)"
jq -e '
  .ready == false
  and .current_schema_present == true
  and .current_truth_ready == false
  and .legacy_active_only == false
' >/dev/null <<<"$invalid_active"

for cross_generation in \
  "$(hepta_live_control_ui_truth_contract_json "$current_ga" "$legacy_merge" active-health)" \
  "$(hepta_live_control_ui_truth_contract_json "$legacy_ga" "$current_merge" active-health)"
do
  jq -e '
    .ready == false
    and .partial_schema_present == true
    and .schema_mode == "partial_or_unknown"
    and .legacy_accepted == false
  ' >/dev/null <<<"$cross_generation"
done

null_current_ga="$(
  jq -cn '{
    status:"blocked",
    public_ga_ready:false,
    public_ga_claimed:false,
    operator_approval_required:true,
    production_replacement_percent:100,
    blocker_count:1,
    blockers:["gateway_replacement_not_ready"],
    control_ui_product_status:null,
    control_ui_product_complete:null,
    control_ui_live_operator_surface_percent:null,
    control_ui_overall_evidence_percent:null
  }'
)"
null_current_merge="$(
  jq -cn '{
    status:"attention",
    public_ga_claimed:false,
    production_replacement_percent:100,
    blockers:["telegram_owner_handoff_not_requested"],
    control_ui_product_status:null,
    control_ui_product_complete:null,
    control_ui_live_operator_surface_percent:null,
    control_ui_evidence:null
  }'
)"
null_active="$(
  hepta_live_control_ui_truth_contract_json \
    "$null_current_ga" "$null_current_merge" active-health
)"
jq -e '
  .ready == false
  and .current_schema_present == true
  and .current_truth_ready == false
  and .legacy_accepted == false
' >/dev/null <<<"$null_active"

unknown_mode="$(
  hepta_live_control_ui_truth_contract_json \
    "$current_ga" "$current_merge" candidate-artifact
)"
jq -e '
  .ready == false
  and .mode_known == false
  and .schema_mode == "unknown_mode"
  and .control_ui_truth_checked == false
  and .product_completion_claim_allowed == false
' >/dev/null <<<"$unknown_mode"

for invalid_legacy_ga in \
  "$(jq '.status = "ready"' <<<"$legacy_ga")" \
  "$(jq '.public_ga_ready = true' <<<"$legacy_ga")" \
  "$(jq '.public_ga_claimed = true' <<<"$legacy_ga")" \
  "$(jq '.operator_approval_required = false' <<<"$legacy_ga")" \
  "$(jq '.production_replacement_percent = null' <<<"$legacy_ga")" \
  "$(jq '.production_replacement_percent = 99' <<<"$legacy_ga")" \
  "$(jq '.blocker_count = 99' <<<"$legacy_ga")" \
  "$(jq '.blockers = []' <<<"$legacy_ga")"
do
  jq -e '.ready == false and .legacy_accepted == false' >/dev/null \
    <<<"$(
      hepta_live_control_ui_truth_contract_json \
        "$invalid_legacy_ga" "$legacy_merge" active-health
    )"
done

for invalid_legacy_merge in \
  "$(jq '.status = "ready"' <<<"$legacy_merge")" \
  "$(jq '.public_ga_claimed = true' <<<"$legacy_merge")" \
  "$(jq '.production_replacement_percent = "100"' <<<"$legacy_merge")" \
  "$(jq '.production_replacement_percent = 99' <<<"$legacy_merge")" \
  "$(jq '.readiness_class = "static_contract_ready_production_in_progress"' <<<"$legacy_merge")" \
  "$(jq '.blockers = []' <<<"$legacy_merge")"
do
  jq -e '.ready == false and .legacy_accepted == false' >/dev/null \
    <<<"$(
      hepta_live_control_ui_truth_contract_json \
        "$legacy_ga" "$invalid_legacy_merge" active-health
    )"
done

echo "Hepta live Control UI truth fixture self-test passed"
