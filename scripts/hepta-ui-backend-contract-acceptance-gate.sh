#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_BACKEND_CONTRACT_ACCEPTANCE_REPORT_PATH:-$READINESS_DIR/ui-backend-contract-acceptance-gate.json}"

PLAN_BOUNDARY_REPORT_PATH="$READINESS_DIR/ui-plan-boundary-gate.json"
OPERATOR_BRIEFING_REPORT_PATH="$READINESS_DIR/ui-operator-briefing-gate.json"
BACKEND_PROMOTION_PACKET_REPORT_PATH="$READINESS_DIR/ui-backend-promotion-packet-gate.json"
BACKEND_ALIGNMENT_EVIDENCE_REPORT_PATH="$READINESS_DIR/ui-backend-alignment-evidence-gate.json"
CRITICAL_PATH_PLAN_REPORT_PATH="$READINESS_DIR/ui-critical-path-plan-gate.json"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf '%s is required for the Hepta UI backend-contract acceptance gate\n' "$1" >&2
    exit 2
  fi
}

require_report() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required backend-contract acceptance input: %s\n' "$path" >&2
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

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-backend-contract-acceptance.XXXXXX")"
REPORT_TMP="$TMP_DIR/backend-contract-acceptance-report.json"
trap 'rm -rf "$TMP_DIR"' EXIT

plan_boundary_sha="$(file_sha256 "$PLAN_BOUNDARY_REPORT_PATH")"
operator_briefing_sha="$(file_sha256 "$OPERATOR_BRIEFING_REPORT_PATH")"
backend_promotion_sha="$(file_sha256 "$BACKEND_PROMOTION_PACKET_REPORT_PATH")"
backend_alignment_sha="$(file_sha256 "$BACKEND_ALIGNMENT_EVIDENCE_REPORT_PATH")"
critical_path_sha="$(file_sha256 "$CRITICAL_PATH_PLAN_REPORT_PATH")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_backend_contract_acceptance_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg plan_boundary_path "$PLAN_BOUNDARY_REPORT_PATH" \
  --arg operator_briefing_path "$OPERATOR_BRIEFING_REPORT_PATH" \
  --arg backend_promotion_path "$BACKEND_PROMOTION_PACKET_REPORT_PATH" \
  --arg backend_alignment_path "$BACKEND_ALIGNMENT_EVIDENCE_REPORT_PATH" \
  --arg critical_path_path "$CRITICAL_PATH_PLAN_REPORT_PATH" \
  --arg plan_boundary_sha "$plan_boundary_sha" \
  --arg operator_briefing_sha "$operator_briefing_sha" \
  --arg backend_promotion_sha "$backend_promotion_sha" \
  --arg backend_alignment_sha "$backend_alignment_sha" \
  --arg critical_path_sha "$critical_path_sha" \
  --slurpfile plan_boundary_file "$PLAN_BOUNDARY_REPORT_PATH" \
  --slurpfile operator_briefing_file "$OPERATOR_BRIEFING_REPORT_PATH" \
  --slurpfile backend_promotion_file "$BACKEND_PROMOTION_PACKET_REPORT_PATH" \
  --slurpfile backend_alignment_file "$BACKEND_ALIGNMENT_EVIDENCE_REPORT_PATH" \
  --slurpfile critical_path_file "$CRITICAL_PATH_PLAN_REPORT_PATH" \
  '
  ($plan_boundary_file[0]) as $plan
  | ($operator_briefing_file[0]) as $operator
  | ($backend_promotion_file[0]) as $promotion
  | ($backend_alignment_file[0]) as $alignment
  | ($critical_path_file[0]) as $critical
  | def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
    def selected_ids: ["message_search","file_upload_send","media_download_playback","notifications","room_settings"];
    def acceptance_required_evidence: [
      "typed backend adapter contract",
      "operation id and source hash",
      "backend readback evidence",
      "retry, cancel, and idempotency policy",
      "stale-target guard",
      "side-effect review",
      "refreshed no-window Hepta UI product-readiness artifact",
      "refreshed full-hard true-window Hepta UI product-readiness artifact"
    ];
    def refresh_commands: [
      "HEPTA_UI_PRODUCT_READINESS_DIR=/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.after-backend-nowindow scripts/hepta-ui-product-readiness-gate.sh",
      "HEPTA_UI_PRODUCT_READINESS_INCLUDE_NATIVE_WINDOW_SMOKE=1 HEPTA_UI_PRODUCT_READINESS_INCLUDE_NATIVE_WINDOW_ROUTE_SMOKE=1 HEPTA_UI_PRODUCT_READINESS_INCLUDE_NATIVE_WINDOW_SECONDARY_SMOKE=1 HEPTA_UI_PRODUCT_READINESS_INCLUDE_NATIVE_WINDOW_SECONDARY_MOBILE_SMOKE=1 HEPTA_UI_PRODUCT_READINESS_DIR=/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.after-backend-full-hard scripts/hepta-ui-product-readiness-gate.sh"
    ];
    def acceptance_items:
      ($alignment.alignment_items | map(
        . as $alignment_item
        | ($promotion.priority_packets[] | select(.id == $alignment_item.id)) as $promotion_item
        | {
            id:$alignment_item.id,
            priority:$alignment_item.priority,
            owner_lane:"backend_contract",
            next_owner_lane:$alignment_item.next_owner_lane,
            status:$alignment_item.status,
            handoff_acceptance_ready:true,
            ui_lane_state:$alignment_item.ui_lane_state,
            backend_contract_gate:$alignment_item.backend_contract_gate,
            backend_contract_next_slice:$alignment_item.backend_contract_next_slice,
            current_ui_evidence:$alignment_item.current_ui_evidence,
            live_wiring:$alignment_item.live_wiring,
            required_backend_contract_count:$alignment_item.required_backend_contract_count,
            required_backend_contracts:$alignment_item.required_backend_contracts,
            required_contract_groups:$alignment_item.required_contract_groups,
            fixture_source_markers:$alignment_item.fixture_source_markers,
            fixture_source_marker_count:$alignment_item.fixture_source_marker_count,
            acceptance_required_evidence:acceptance_required_evidence,
            backend_exit_criteria:{
              backend_adapter_contract:true,
              operation_id_and_source_hash:true,
              readback_evidence:true,
              retry_cancel_idempotency_policy:true,
              stale_target_guard:true,
              side_effect_review:true,
              refreshed_no_window_readiness:true,
              refreshed_full_hard_readiness:true
            },
            current_backend_completion:{
              backend_adapter_promoted:false,
              readback_evidence_recorded:false,
              side_effect_review_recorded:false,
              live_product_claim_ready:false
            },
            verification_commands:($promotion_item.verification_commands + refresh_commands),
            promotion_blocker:$alignment_item.promotion_blocker,
            promotion_requires:$alignment_item.promotion_requires,
            active_promotion_performed:false,
            side_effects:$alignment_item.side_effects
          }
      ));
    def source_chain_ready:
      $plan.plan_boundary_gate_ready == true
      and $plan.live_product_claim.remaining_backend_contract_count == 12
      and $plan.claim_boundary.live_product_claim_ready == false
      and $plan.claim_boundary.public_distribution_claim_ready == false
      and $plan.claim_boundary.release_claim_ready == false
      and $operator.operator_briefing_gate_ready == true
      and $operator.backend_remaining_contract_count == 12
      and $operator.backend_priority_ids[0:5] == selected_ids
      and $promotion.backend_promotion_packet_gate_ready == true
      and $promotion.selected_priority_ids == selected_ids
      and $promotion.priority_packet_count == 5
      and ($promotion.priority_packets | all(.next_owner_lane == "backend_contract"))
      and ($promotion.priority_packets | all(.promote_requires_backend_adapter == true))
      and ($promotion.priority_packets | all(.promote_requires_readback_evidence == true))
      and ($promotion.priority_packets | all(.promote_requires_side_effect_review == true))
      and ($promotion.priority_packets | all(.active_promotion_performed == false))
      and $alignment.backend_alignment_evidence_gate_ready == true
      and $alignment.selected_alignment_ids == selected_ids
      and $alignment.alignment_item_count == 5
      and ($alignment.alignment_items | all(.alignment_ready == true))
      and ($alignment.alignment_items | all(.next_owner_lane == "backend_contract"))
      and ($alignment.alignment_items | all(.status == "partial_live_backend_contract_remaining"))
      and ($alignment.alignment_items | all(.required_backend_contract_count >= 5))
      and $critical.critical_path_plan_gate_ready == true
      and $critical.current_backend_selected_ids == selected_ids
      and $critical.future_plan[0].id == "backend_contract_first_five"
      and $critical.claim_boundary.active_backend_promotion_performed == false
      and $critical.claim_boundary.backend_adapter_promoted == false
      and $critical.claim_boundary.live_runtime_mutation == false
      and $critical.claim_boundary.live_product_claim_ready == false
      and sha_ready($plan_boundary_sha)
      and sha_ready($operator_briefing_sha)
      and sha_ready($backend_promotion_sha)
      and sha_ready($backend_alignment_sha)
      and sha_ready($critical_path_sha);
    source_chain_ready as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      backend_contract_acceptance_gate_ready:$ready,
      acceptance_kind:"local_backend_contract_acceptance_handoff",
      acceptance_version:1,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      source_reports:{
        plan_boundary:$plan_boundary_path,
        operator_briefing:$operator_briefing_path,
        backend_promotion_packet:$backend_promotion_path,
        backend_alignment_evidence:$backend_alignment_path,
        critical_path_plan:$critical_path_path
      },
      source_report_sha256:{
        plan_boundary:$plan_boundary_sha,
        operator_briefing:$operator_briefing_sha,
        backend_promotion_packet:$backend_promotion_sha,
        backend_alignment_evidence:$backend_alignment_sha,
        critical_path_plan:$critical_path_sha
      },
      selected_acceptance_ids:selected_ids,
      acceptance_item_count:(acceptance_items | length),
      acceptance_ready_count:(acceptance_items | map(select(.handoff_acceptance_ready == true)) | length),
      backend_remaining_contract_count:$plan.live_product_claim.remaining_backend_contract_count,
      acceptance_items:acceptance_items,
      future_plan_link:{
        critical_path_plan_id:$critical.future_plan[0].id,
        next_owner_lane:"backend_contract",
        hepta_ui_after_backend_refresh:$critical.future_plan[1].id,
        release_operator_after_approval:$critical.future_plan[2].id
      },
      promotion_exit_guard:{
        active_backend_promotion_allowed:false,
        backend_adapter_promoted:false,
        readback_evidence_recorded:false,
        side_effect_review_recorded:false,
        live_runtime_mutation:false,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false
      },
      answer_guardrail:{
        allowed_claim:"local_backend_contract_acceptance_handoff_ready",
        forbidden_claims:["backend_adapter_promoted","live_runtime_mutated","live_product_ready","public_distribution_ready","release_ready"],
        next_visible_summary:"UI handoff is ready; backend contract promotion remains backend-owned until adapter/readback/side-effect evidence exists."
      },
      claim_boundary:{
        local_backend_contract_acceptance_ready:$ready,
        local_backend_promotion_packet_ready:$promotion.claim_boundary.local_backend_promotion_packet_ready,
        local_backend_alignment_evidence_ready:$alignment.claim_boundary.local_backend_alignment_evidence_ready,
        local_critical_path_plan_ready:$critical.claim_boundary.local_critical_path_plan_ready,
        active_backend_promotion_performed:false,
        backend_adapter_promoted:false,
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
  and .backend_contract_acceptance_gate_ready == true
  and .acceptance_kind == "local_backend_contract_acceptance_handoff"
  and .acceptance_version == 1
  and .backend_remaining_contract_count == 12
  and .selected_acceptance_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .acceptance_item_count == 5
  and .acceptance_ready_count == 5
  and (.acceptance_items | length) == 5
  and (.acceptance_items | all(.owner_lane == "backend_contract"))
  and (.acceptance_items | all(.next_owner_lane == "backend_contract"))
  and (.acceptance_items | all(.status == "partial_live_backend_contract_remaining"))
  and (.acceptance_items | all(.handoff_acceptance_ready == true))
  and (.acceptance_items | all(.required_backend_contract_count >= 5))
  and (.acceptance_items | all(.fixture_source_marker_count >= 4))
  and (.acceptance_items | all((.acceptance_required_evidence | length) == 8))
  and (.acceptance_items | all(.backend_exit_criteria.backend_adapter_contract == true))
  and (.acceptance_items | all(.backend_exit_criteria.readback_evidence == true))
  and (.acceptance_items | all(.backend_exit_criteria.refreshed_full_hard_readiness == true))
  and (.acceptance_items | all(.current_backend_completion.backend_adapter_promoted == false))
  and (.acceptance_items | all(.current_backend_completion.readback_evidence_recorded == false))
  and (.acceptance_items | all(.current_backend_completion.live_product_claim_ready == false))
  and (.acceptance_items | all((.verification_commands | length) >= 8))
  and .future_plan_link.critical_path_plan_id == "backend_contract_first_five"
  and .future_plan_link.hepta_ui_after_backend_refresh == "hepta_ui_hard_evidence_refresh"
  and .promotion_exit_guard.active_backend_promotion_allowed == false
  and .promotion_exit_guard.backend_adapter_promoted == false
  and .promotion_exit_guard.live_runtime_mutation == false
  and .promotion_exit_guard.live_product_claim_ready == false
  and (.answer_guardrail.forbidden_claims | index("backend_adapter_promoted") != null)
  and (.answer_guardrail.forbidden_claims | index("live_product_ready") != null)
  and .claim_boundary.local_backend_contract_acceptance_ready == true
  and .claim_boundary.local_backend_promotion_packet_ready == true
  and .claim_boundary.local_backend_alignment_evidence_ready == true
  and .claim_boundary.local_critical_path_plan_ready == true
  and .claim_boundary.active_backend_promotion_performed == false
  and .claim_boundary.backend_adapter_promoted == false
  and .claim_boundary.live_runtime_mutation == false
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and .claim_boundary.public_upload_performed == false
  and .claim_boundary.signing_notarization_performed == false
  and .side_effects.backend_adapter_promoted == false
  and .side_effects.live_runtime_mutation == false
  and .side_effects.external_mutation == false
' "$REPORT_TMP" >/dev/null

mkdir -p "$(dirname "$REPORT_PATH")"
cp "$REPORT_TMP" "$REPORT_PATH"
cat "$REPORT_TMP"
