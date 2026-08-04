#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_BACKEND_PROMOTION_PACKET_REPORT_PATH:-$READINESS_DIR/ui-backend-promotion-packet-gate.json}"

BASE_GAP_BACKEND_HANDOFF_PATH="$READINESS_DIR/native-base-gap-backend-handoff.json"
BACKEND_CONTRACT_GATES_REPORT_PATH="$READINESS_DIR/native-backend-contract-gates.json"
PRODUCTIZATION_ROLLUP_REPORT_PATH="$READINESS_DIR/native-productization-blocker-rollup.json"
PLAN_BOUNDARY_REPORT_PATH="$READINESS_DIR/ui-plan-boundary-gate.json"
OPERATOR_BRIEFING_REPORT_PATH="$READINESS_DIR/ui-operator-briefing-gate.json"

HEPTA_UI_GATE_REQUIREMENT_CONTEXT="the Hepta UI backend promotion packet gate"
HEPTA_UI_REPORT_INPUT_LABEL="backend-promotion"
source scripts/lib/hepta-ui-gate-common-v1.sh

require_command jq
require_command shasum

require_report "$BASE_GAP_BACKEND_HANDOFF_PATH"
require_report "$BACKEND_CONTRACT_GATES_REPORT_PATH"
require_report "$PRODUCTIZATION_ROLLUP_REPORT_PATH"
require_report "$PLAN_BOUNDARY_REPORT_PATH"
require_report "$OPERATOR_BRIEFING_REPORT_PATH"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-backend-promotion-packet.XXXXXX")"
REPORT_TMP="$TMP_DIR/backend-promotion-packet-report.json"
trap 'rm -rf "$TMP_DIR"' EXIT

handoff_sha="$(file_sha256 "$BASE_GAP_BACKEND_HANDOFF_PATH")"
backend_contract_sha="$(file_sha256 "$BACKEND_CONTRACT_GATES_REPORT_PATH")"
rollup_sha="$(file_sha256 "$PRODUCTIZATION_ROLLUP_REPORT_PATH")"
plan_boundary_sha="$(file_sha256 "$PLAN_BOUNDARY_REPORT_PATH")"
operator_briefing_sha="$(file_sha256 "$OPERATOR_BRIEFING_REPORT_PATH")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_backend_promotion_packet_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg handoff_path "$BASE_GAP_BACKEND_HANDOFF_PATH" \
  --arg backend_contract_path "$BACKEND_CONTRACT_GATES_REPORT_PATH" \
  --arg rollup_path "$PRODUCTIZATION_ROLLUP_REPORT_PATH" \
  --arg plan_boundary_path "$PLAN_BOUNDARY_REPORT_PATH" \
  --arg operator_briefing_path "$OPERATOR_BRIEFING_REPORT_PATH" \
  --arg handoff_sha "$handoff_sha" \
  --arg backend_contract_sha "$backend_contract_sha" \
  --arg rollup_sha "$rollup_sha" \
  --arg plan_boundary_sha "$plan_boundary_sha" \
  --arg operator_briefing_sha "$operator_briefing_sha" \
  --slurpfile handoff_file "$BASE_GAP_BACKEND_HANDOFF_PATH" \
  --slurpfile backend_contract_file "$BACKEND_CONTRACT_GATES_REPORT_PATH" \
  --slurpfile rollup_file "$PRODUCTIZATION_ROLLUP_REPORT_PATH" \
  --slurpfile plan_boundary_file "$PLAN_BOUNDARY_REPORT_PATH" \
  --slurpfile operator_briefing_file "$OPERATOR_BRIEFING_REPORT_PATH" \
  '
  ($handoff_file[0]) as $handoff
  | ($backend_contract_file[0]) as $backend_contract
  | ($rollup_file[0]) as $rollup
  | ($plan_boundary_file[0]) as $plan
  | ($operator_briefing_file[0]) as $operator_briefing
  | def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
    def priority_items: ($handoff.items | sort_by(.priority));
    def top_priority_ids: (priority_items | map(.id) | .[0:5]);
    def packet_item:
      {
        id,
        priority,
        owner_lane:"backend_contract",
        ui_lane_state,
        next_owner_lane,
        status,
        current_ui_evidence,
        backend_contract_next_slice,
        promotion_blocker,
        required_backend_contract_count:(.required_backend_contracts | length),
        required_backend_contracts,
        live_wiring:.acceptance_state.live_wiring,
        promote_requires_backend_adapter:true,
        promote_requires_readback_evidence:true,
        promote_requires_side_effect_review:true,
        active_promotion_performed:false,
        verification_commands:[
          "focused Rust tests for the touched Native module",
          "cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_telegram_base_ -- --nocapture",
          "bash -n for touched scripts/gates",
          "git diff --check",
          "scripts/hepta-native-fixture-visual-smoke.sh",
          "scripts/hepta-ui-product-readiness-gate.sh",
          "refresh native-base-gap-backend-handoff.json and ui-backend-promotion-packet-gate.json"
        ],
        side_effects
      };
    def sources_ready:
      $handoff.native_base_gap_backend_handoff_ready == true
      and $handoff.handoff_count == 12
      and ($handoff.items | length) == 12
      and ($handoff.items | all(.status == "partial_live_backend_contract_remaining"))
      and ($handoff.items | all(.ui_lane_state == "complete"))
      and ($handoff.items | all(.next_owner_lane == "backend_contract"))
      and ($handoff.items | all((.required_backend_contracts | length) >= 5))
      and ($handoff.items | all(.side_effects.external_mutation == false))
      and $backend_contract.backend_contract_waves_ready == true
      and $backend_contract.verified_gap_count == 12
      and ($backend_contract.waves | length) == 6
      and ($backend_contract.waves | all(.status == "ready"))
      and $rollup.productization_blocker_rollup_ready == true
      and $rollup.base_gap_backend_handoff.handoff_count == 12
      and $plan.plan_boundary_gate_ready == true
      and $plan.live_product_claim.remaining_backend_contract_count == 12
      and $plan.live_product_claim.ready == false
      and $plan.claim_boundary.live_product_claim_ready == false
      and $plan.claim_boundary.public_distribution_claim_ready == false
      and $plan.claim_boundary.release_claim_ready == false
      and $operator_briefing.operator_briefing_gate_ready == true
      and $operator_briefing.backend_remaining_contract_count == 12
      and ($operator_briefing.backend_priority_ids | length) == 12
      and $operator_briefing.backend_priority_ids[0] == "message_search"
      and $operator_briefing.backend_priority_ids[1] == "file_upload_send"
      and $operator_briefing.backend_priority_ids[2] == "media_download_playback"
      and sha_ready($handoff_sha)
      and sha_ready($backend_contract_sha)
      and sha_ready($rollup_sha)
      and sha_ready($plan_boundary_sha)
      and sha_ready($operator_briefing_sha);
    sources_ready as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      backend_promotion_packet_gate_ready:$ready,
      packet_kind:"local_backend_contract_promotion_packet",
      packet_version:1,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      source_reports:{
        backend_handoff:$handoff_path,
        backend_contract_gates:$backend_contract_path,
        productization_rollup:$rollup_path,
        plan_boundary:$plan_boundary_path,
        operator_briefing:$operator_briefing_path
      },
      source_report_sha256:{
        backend_handoff:$handoff_sha,
        backend_contract_gates:$backend_contract_sha,
        productization_rollup:$rollup_sha,
        plan_boundary:$plan_boundary_sha,
        operator_briefing:$operator_briefing_sha
      },
      total_contract_count:($handoff.items | length),
      backend_remaining_contract_count:$plan.live_product_claim.remaining_backend_contract_count,
      priority_packet_count:5,
      selected_priority_ids:top_priority_ids,
      all_priority_ids:(priority_items | map(.id)),
      priority_packets:(priority_items | .[0:5] | map(packet_item)),
      backlog_packets:(priority_items | map(packet_item)),
      acceptance_guardrail:{
        active_backend_promotion_allowed:false,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false,
        required_before_promotion:[
          "typed backend adapter contract",
          "operation id and source hash",
          "readback evidence",
          "retry/cancel/idempotency policy",
          "stale-target guard",
          "audit redaction behavior",
          "focused tests and refreshed product-readiness artifact"
        ]
      },
      claim_boundary:{
        local_backend_promotion_packet_ready:$ready,
        backend_contract_handoff_ready:$handoff.native_base_gap_backend_handoff_ready,
        backend_contract_waves_ready:$backend_contract.backend_contract_waves_ready,
        active_backend_promotion_performed:false,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false,
        external_actions_allowed:false
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
  and .backend_promotion_packet_gate_ready == true
  and .packet_kind == "local_backend_contract_promotion_packet"
  and .packet_version == 1
  and .total_contract_count == 12
  and .backend_remaining_contract_count == 12
  and .priority_packet_count == 5
  and (.selected_priority_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"])
  and (.all_priority_ids | length) == 12
  and (.priority_packets | length) == 5
  and (.backlog_packets | length) == 12
  and (.priority_packets | all(.owner_lane == "backend_contract"))
  and (.priority_packets | all(.next_owner_lane == "backend_contract"))
  and (.priority_packets | all(.status == "partial_live_backend_contract_remaining"))
  and (.priority_packets | all(.promote_requires_backend_adapter == true))
  and (.priority_packets | all(.promote_requires_readback_evidence == true))
  and (.priority_packets | all(.active_promotion_performed == false))
  and (.priority_packets | all(.required_backend_contract_count >= 5))
  and (.priority_packets | all((.verification_commands | length) >= 6))
  and .acceptance_guardrail.active_backend_promotion_allowed == false
  and .acceptance_guardrail.live_product_claim_ready == false
  and .acceptance_guardrail.public_distribution_claim_ready == false
  and .acceptance_guardrail.release_claim_ready == false
  and .claim_boundary.local_backend_promotion_packet_ready == true
  and .claim_boundary.active_backend_promotion_performed == false
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and .side_effects.backend_adapter_promoted == false
  and .side_effects.live_runtime_mutation == false
  and .side_effects.external_mutation == false
' "$REPORT_TMP" >/dev/null

mkdir -p "$(dirname "$REPORT_PATH")"
cp "$REPORT_TMP" "$REPORT_PATH"
cat "$REPORT_TMP"
