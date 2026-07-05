#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_BACKEND_HANDOFF_EXPORT_REPORT_PATH:-$READINESS_DIR/ui-backend-handoff-export-gate.json}"
EXPORT_DIR="${HEPTA_UI_BACKEND_HANDOFF_EXPORT_DIR:-$READINESS_DIR/backend-handoff-export}"
EXPORT_MARKDOWN_PATH="$EXPORT_DIR/backend-handoff-export.md"

PLAN_BOUNDARY_REPORT_PATH="$READINESS_DIR/ui-plan-boundary-gate.json"
OPERATOR_BRIEFING_REPORT_PATH="$READINESS_DIR/ui-operator-briefing-gate.json"
BACKEND_PROMOTION_PACKET_REPORT_PATH="$READINESS_DIR/ui-backend-promotion-packet-gate.json"
BACKEND_ALIGNMENT_EVIDENCE_REPORT_PATH="$READINESS_DIR/ui-backend-alignment-evidence-gate.json"
CRITICAL_PATH_PLAN_REPORT_PATH="$READINESS_DIR/ui-critical-path-plan-gate.json"
BACKEND_CONTRACT_ACCEPTANCE_REPORT_PATH="$READINESS_DIR/ui-backend-contract-acceptance-gate.json"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf '%s is required for the Hepta UI backend handoff export gate\n' "$1" >&2
    exit 2
  fi
}

require_report() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required backend handoff export input: %s\n' "$path" >&2
    exit 1
  fi
  jq empty "$path" >/dev/null
}

file_sha256() {
  shasum -a 256 "$1" | awk '{print $1}'
}

require_command jq
require_command shasum

require_report "$PLAN_BOUNDARY_REPORT_PATH"
require_report "$OPERATOR_BRIEFING_REPORT_PATH"
require_report "$BACKEND_PROMOTION_PACKET_REPORT_PATH"
require_report "$BACKEND_ALIGNMENT_EVIDENCE_REPORT_PATH"
require_report "$CRITICAL_PATH_PLAN_REPORT_PATH"
require_report "$BACKEND_CONTRACT_ACCEPTANCE_REPORT_PATH"

mkdir -p "$EXPORT_DIR"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-backend-handoff-export.XXXXXX")"
REPORT_TMP="$TMP_DIR/backend-handoff-export-report.json"
MARKDOWN_TMP="$TMP_DIR/backend-handoff-export.md"
trap 'rm -rf "$TMP_DIR"' EXIT

plan_boundary_sha="$(file_sha256 "$PLAN_BOUNDARY_REPORT_PATH")"
operator_briefing_sha="$(file_sha256 "$OPERATOR_BRIEFING_REPORT_PATH")"
backend_promotion_sha="$(file_sha256 "$BACKEND_PROMOTION_PACKET_REPORT_PATH")"
backend_alignment_sha="$(file_sha256 "$BACKEND_ALIGNMENT_EVIDENCE_REPORT_PATH")"
critical_path_sha="$(file_sha256 "$CRITICAL_PATH_PLAN_REPORT_PATH")"
backend_acceptance_sha="$(file_sha256 "$BACKEND_CONTRACT_ACCEPTANCE_REPORT_PATH")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_backend_handoff_export_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg export_dir "$EXPORT_DIR" \
  --arg export_markdown_path "$EXPORT_MARKDOWN_PATH" \
  --arg plan_boundary_path "$PLAN_BOUNDARY_REPORT_PATH" \
  --arg operator_briefing_path "$OPERATOR_BRIEFING_REPORT_PATH" \
  --arg backend_promotion_path "$BACKEND_PROMOTION_PACKET_REPORT_PATH" \
  --arg backend_alignment_path "$BACKEND_ALIGNMENT_EVIDENCE_REPORT_PATH" \
  --arg critical_path_path "$CRITICAL_PATH_PLAN_REPORT_PATH" \
  --arg backend_acceptance_path "$BACKEND_CONTRACT_ACCEPTANCE_REPORT_PATH" \
  --arg plan_boundary_sha "$plan_boundary_sha" \
  --arg operator_briefing_sha "$operator_briefing_sha" \
  --arg backend_promotion_sha "$backend_promotion_sha" \
  --arg backend_alignment_sha "$backend_alignment_sha" \
  --arg critical_path_sha "$critical_path_sha" \
  --arg backend_acceptance_sha "$backend_acceptance_sha" \
  --slurpfile plan_boundary_file "$PLAN_BOUNDARY_REPORT_PATH" \
  --slurpfile operator_briefing_file "$OPERATOR_BRIEFING_REPORT_PATH" \
  --slurpfile backend_promotion_file "$BACKEND_PROMOTION_PACKET_REPORT_PATH" \
  --slurpfile backend_alignment_file "$BACKEND_ALIGNMENT_EVIDENCE_REPORT_PATH" \
  --slurpfile critical_path_file "$CRITICAL_PATH_PLAN_REPORT_PATH" \
  --slurpfile backend_acceptance_file "$BACKEND_CONTRACT_ACCEPTANCE_REPORT_PATH" \
  '
  ($plan_boundary_file[0]) as $plan
  | ($operator_briefing_file[0]) as $operator
  | ($backend_promotion_file[0]) as $promotion
  | ($backend_alignment_file[0]) as $alignment
  | ($critical_path_file[0]) as $critical
  | ($backend_acceptance_file[0]) as $acceptance
  | def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
    def selected_ids: ["message_search","file_upload_send","media_download_playback","notifications","room_settings"];
    def source_chain_ready:
      $plan.plan_boundary_gate_ready == true
      and $plan.live_product_claim.remaining_backend_contract_count == 12
      and $operator.operator_briefing_gate_ready == true
      and $operator.backend_priority_ids[0:5] == selected_ids
      and $promotion.backend_promotion_packet_gate_ready == true
      and $promotion.selected_priority_ids == selected_ids
      and $alignment.backend_alignment_evidence_gate_ready == true
      and $alignment.selected_alignment_ids == selected_ids
      and $critical.critical_path_plan_gate_ready == true
      and $critical.current_backend_selected_ids == selected_ids
      and $acceptance.backend_contract_acceptance_gate_ready == true
      and $acceptance.selected_acceptance_ids == selected_ids
      and $acceptance.acceptance_item_count == 5
      and $acceptance.acceptance_ready_count == 5
      and ($acceptance.acceptance_items | all(.owner_lane == "backend_contract"))
      and ($acceptance.acceptance_items | all(.next_owner_lane == "backend_contract"))
      and ($acceptance.acceptance_items | all(.status == "partial_live_backend_contract_remaining"))
      and ($acceptance.acceptance_items | all((.acceptance_required_evidence | length) == 8))
      and ($acceptance.acceptance_items | all(.current_backend_completion.backend_adapter_promoted == false))
      and ($acceptance.acceptance_items | all(.current_backend_completion.readback_evidence_recorded == false))
      and $acceptance.promotion_exit_guard.active_backend_promotion_allowed == false
      and $acceptance.claim_boundary.backend_adapter_promoted == false
      and $acceptance.claim_boundary.live_runtime_mutation == false
      and $acceptance.claim_boundary.live_product_claim_ready == false
      and $acceptance.claim_boundary.public_distribution_claim_ready == false
      and $acceptance.claim_boundary.release_claim_ready == false
      and $acceptance.side_effects.external_mutation == false
      and sha_ready($plan_boundary_sha)
      and sha_ready($operator_briefing_sha)
      and sha_ready($backend_promotion_sha)
      and sha_ready($backend_alignment_sha)
      and sha_ready($critical_path_sha)
      and sha_ready($backend_acceptance_sha);
    source_chain_ready as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      backend_handoff_export_gate_ready:$ready,
      export_kind:"local_backend_lane_execution_export",
      export_version:1,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      export_dir:$export_dir,
      export_markdown_path:$export_markdown_path,
      source_reports:{
        plan_boundary:$plan_boundary_path,
        operator_briefing:$operator_briefing_path,
        backend_promotion_packet:$backend_promotion_path,
        backend_alignment_evidence:$backend_alignment_path,
        critical_path_plan:$critical_path_path,
        backend_contract_acceptance:$backend_acceptance_path
      },
      source_report_sha256:{
        plan_boundary:$plan_boundary_sha,
        operator_briefing:$operator_briefing_sha,
        backend_promotion_packet:$backend_promotion_sha,
        backend_alignment_evidence:$backend_alignment_sha,
        critical_path_plan:$critical_path_sha,
        backend_contract_acceptance:$backend_acceptance_sha
      },
      backend_lane_target:{
        target_repo:"/Users/qianqi/.openclaw/workspace/Hepta",
        owner_lane:"backend_contract",
        selected_export_ids:selected_ids,
        next_action:"implement backend adapters only after adapter contract, operation/source hash, readback evidence, retry/cancel/idempotency policy, stale-target guard, and side-effect review are recorded"
      },
      hepta_ui_after_backend:{
        repo:"/Users/qianqi/.openclaw/workspace/Hepta-ui",
        required_refresh_commands:[
          "HEPTA_UI_PRODUCT_READINESS_DIR=/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.after-backend-nowindow scripts/hepta-ui-product-readiness-gate.sh",
          "HEPTA_UI_PRODUCT_READINESS_INCLUDE_NATIVE_WINDOW_SMOKE=1 HEPTA_UI_PRODUCT_READINESS_INCLUDE_NATIVE_WINDOW_ROUTE_SMOKE=1 HEPTA_UI_PRODUCT_READINESS_INCLUDE_NATIVE_WINDOW_SECONDARY_SMOKE=1 HEPTA_UI_PRODUCT_READINESS_INCLUDE_NATIVE_WINDOW_SECONDARY_MOBILE_SMOKE=1 HEPTA_UI_PRODUCT_READINESS_DIR=/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.after-backend-full-hard scripts/hepta-ui-product-readiness-gate.sh"
        ],
        hard_claim_guard:"public demo/live-product claims stay false until refreshed no-window and full-hard artifacts pass after backend changes"
      },
      export_item_count:($acceptance.acceptance_items | length),
      export_ready_count:($acceptance.acceptance_items | map(select(.handoff_acceptance_ready == true)) | length),
      selected_export_ids:selected_ids,
      backend_remaining_contract_count:$plan.live_product_claim.remaining_backend_contract_count,
      export_items:($acceptance.acceptance_items | map({
        id,
        priority,
        owner_lane,
        status,
        backend_contract_gate,
        backend_contract_next_slice,
        required_backend_contracts,
        required_contract_groups,
        acceptance_required_evidence,
        backend_exit_criteria,
        current_backend_completion,
        verification_commands,
        promotion_blocker,
        promotion_requires,
        side_effects
      })),
      dispatch_guardrail:{
        local_export_ready:$ready,
        external_dispatch_performed:false,
        backend_adapter_promoted:false,
        readback_evidence_recorded:false,
        side_effect_review_recorded:false,
        live_runtime_mutation:false,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false
      },
      claim_boundary:{
        local_backend_handoff_export_ready:$ready,
        local_backend_contract_acceptance_ready:$acceptance.claim_boundary.local_backend_contract_acceptance_ready,
        local_critical_path_plan_ready:$critical.claim_boundary.local_critical_path_plan_ready,
        active_backend_promotion_performed:false,
        backend_adapter_promoted:false,
        readback_evidence_recorded:false,
        side_effect_review_recorded:false,
        live_runtime_mutation:false,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false,
        external_actions_allowed:false,
        public_upload_performed:false,
        signing_notarization_performed:false
      },
      side_effects:{
        filesystem_read:true,
        local_report_written:true,
        local_markdown_export_written:true,
        matrix_login:false,
        gateway_call:false,
        provider_invoked:false,
        channel_delivery:false,
        backend_adapter_promoted:false,
        live_runtime_mutation:false,
        external_mutation:false
      }
    }' >"$REPORT_TMP"

jq -e '
  .status == "ready"
  and .backend_handoff_export_gate_ready == true
  and .export_kind == "local_backend_lane_execution_export"
  and .export_version == 1
  and .selected_export_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .export_item_count == 5
  and .export_ready_count == 5
  and (.export_items | length) == 5
  and (.export_items | all(.owner_lane == "backend_contract"))
  and (.export_items | all(.status == "partial_live_backend_contract_remaining"))
  and (.export_items | all((.acceptance_required_evidence | length) == 8))
  and (.export_items | all(.current_backend_completion.backend_adapter_promoted == false))
  and (.export_items | all(.current_backend_completion.readback_evidence_recorded == false))
  and .backend_lane_target.owner_lane == "backend_contract"
  and .backend_lane_target.selected_export_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and (.hepta_ui_after_backend.required_refresh_commands | length) == 2
  and .dispatch_guardrail.local_export_ready == true
  and .dispatch_guardrail.external_dispatch_performed == false
  and .dispatch_guardrail.backend_adapter_promoted == false
  and .dispatch_guardrail.live_runtime_mutation == false
  and .dispatch_guardrail.live_product_claim_ready == false
  and .claim_boundary.local_backend_handoff_export_ready == true
  and .claim_boundary.backend_adapter_promoted == false
  and .claim_boundary.readback_evidence_recorded == false
  and .claim_boundary.live_runtime_mutation == false
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and .side_effects.local_markdown_export_written == true
  and .side_effects.backend_adapter_promoted == false
  and .side_effects.live_runtime_mutation == false
  and .side_effects.external_mutation == false
' "$REPORT_TMP" >/dev/null

jq -r '
  "# Hepta UI Backend Handoff Export\n\n"
  + "- Kind: \(.export_kind)\n"
  + "- Selected ids: \(.selected_export_ids | join(", "))\n"
  + "- Export items: \(.export_item_count)/\(.export_ready_count)\n"
  + "- Backend target repo: \(.backend_lane_target.target_repo)\n"
  + "- Backend owner lane: \(.backend_lane_target.owner_lane)\n"
  + "- Claim boundary: backend_adapter=false, readback=false, live_runtime=false, live_product=false, public_distribution=false, release=false\n\n"
  + "## Backend Exit Evidence\n\n"
  + (.export_items | map("- P\(.priority) `\(.id)`: \(.acceptance_required_evidence | join("; "))") | join("\n"))
  + "\n\n## Required UI Refresh After Backend Changes\n\n"
  + (.hepta_ui_after_backend.required_refresh_commands | map("- `\(.)`") | join("\n"))
' "$REPORT_TMP" >"$MARKDOWN_TMP"

if [[ ! -s "$MARKDOWN_TMP" ]]; then
  echo "Backend handoff export markdown was not written" >&2
  exit 1
fi

cp "$MARKDOWN_TMP" "$EXPORT_MARKDOWN_PATH"
mkdir -p "$(dirname "$REPORT_PATH")"
cp "$REPORT_TMP" "$REPORT_PATH"
cat "$REPORT_TMP"
