#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_CRITICAL_PATH_PLAN_REPORT_PATH:-$READINESS_DIR/ui-critical-path-plan-gate.json}"

PLAN_BOUNDARY_REPORT_PATH="$READINESS_DIR/ui-plan-boundary-gate.json"
DEMO_EVIDENCE_REPORT_PATH="$READINESS_DIR/ui-demo-evidence-gate.json"
EVIDENCE_ARCHIVE_REPORT_PATH="$READINESS_DIR/ui-evidence-archive-gate.json"
RELEASE_OPERATOR_DRY_RUN_REPORT_PATH="$READINESS_DIR/ui-release-operator-dry-run-gate.json"
OPERATOR_BRIEFING_REPORT_PATH="$READINESS_DIR/ui-operator-briefing-gate.json"
BACKEND_PROMOTION_PACKET_REPORT_PATH="$READINESS_DIR/ui-backend-promotion-packet-gate.json"
BACKEND_ALIGNMENT_EVIDENCE_REPORT_PATH="$READINESS_DIR/ui-backend-alignment-evidence-gate.json"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf '%s is required for the Hepta UI critical-path plan gate\n' "$1" >&2
    exit 2
  fi
}

require_report() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required critical-path input: %s\n' "$path" >&2
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
require_report "$DEMO_EVIDENCE_REPORT_PATH"
require_report "$EVIDENCE_ARCHIVE_REPORT_PATH"
require_report "$RELEASE_OPERATOR_DRY_RUN_REPORT_PATH"
require_report "$OPERATOR_BRIEFING_REPORT_PATH"
require_report "$BACKEND_PROMOTION_PACKET_REPORT_PATH"
require_report "$BACKEND_ALIGNMENT_EVIDENCE_REPORT_PATH"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-critical-path-plan.XXXXXX")"
REPORT_TMP="$TMP_DIR/critical-path-plan-report.json"
trap 'rm -rf "$TMP_DIR"' EXIT

plan_boundary_sha="$(file_sha256 "$PLAN_BOUNDARY_REPORT_PATH")"
demo_evidence_sha="$(file_sha256 "$DEMO_EVIDENCE_REPORT_PATH")"
evidence_archive_sha="$(file_sha256 "$EVIDENCE_ARCHIVE_REPORT_PATH")"
release_operator_sha="$(file_sha256 "$RELEASE_OPERATOR_DRY_RUN_REPORT_PATH")"
operator_briefing_sha="$(file_sha256 "$OPERATOR_BRIEFING_REPORT_PATH")"
backend_promotion_sha="$(file_sha256 "$BACKEND_PROMOTION_PACKET_REPORT_PATH")"
backend_alignment_sha="$(file_sha256 "$BACKEND_ALIGNMENT_EVIDENCE_REPORT_PATH")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_critical_path_plan_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg plan_boundary_path "$PLAN_BOUNDARY_REPORT_PATH" \
  --arg demo_evidence_path "$DEMO_EVIDENCE_REPORT_PATH" \
  --arg evidence_archive_path "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  --arg release_operator_path "$RELEASE_OPERATOR_DRY_RUN_REPORT_PATH" \
  --arg operator_briefing_path "$OPERATOR_BRIEFING_REPORT_PATH" \
  --arg backend_promotion_path "$BACKEND_PROMOTION_PACKET_REPORT_PATH" \
  --arg backend_alignment_path "$BACKEND_ALIGNMENT_EVIDENCE_REPORT_PATH" \
  --arg plan_boundary_sha "$plan_boundary_sha" \
  --arg demo_evidence_sha "$demo_evidence_sha" \
  --arg evidence_archive_sha "$evidence_archive_sha" \
  --arg release_operator_sha "$release_operator_sha" \
  --arg operator_briefing_sha "$operator_briefing_sha" \
  --arg backend_promotion_sha "$backend_promotion_sha" \
  --arg backend_alignment_sha "$backend_alignment_sha" \
  --slurpfile plan_boundary_file "$PLAN_BOUNDARY_REPORT_PATH" \
  --slurpfile demo_evidence_file "$DEMO_EVIDENCE_REPORT_PATH" \
  --slurpfile evidence_archive_file "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  --slurpfile release_operator_file "$RELEASE_OPERATOR_DRY_RUN_REPORT_PATH" \
  --slurpfile operator_briefing_file "$OPERATOR_BRIEFING_REPORT_PATH" \
  --slurpfile backend_promotion_file "$BACKEND_PROMOTION_PACKET_REPORT_PATH" \
  --slurpfile backend_alignment_file "$BACKEND_ALIGNMENT_EVIDENCE_REPORT_PATH" \
  '
  ($plan_boundary_file[0]) as $plan
  | ($demo_evidence_file[0]) as $demo
  | ($evidence_archive_file[0]) as $archive
  | ($release_operator_file[0]) as $release_dry_run
  | ($operator_briefing_file[0]) as $operator
  | ($backend_promotion_file[0]) as $promotion
  | ($backend_alignment_file[0]) as $alignment
  | def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
    def selected_ids: ["message_search","file_upload_send","media_download_playback","notifications","room_settings"];
    def release_blockers: ($plan.release_claim.blocked_by // []);
    def source_chain_ready:
      $plan.plan_boundary_gate_ready == true
      and $plan.claim_boundary.local_fixture_demo_ready == true
      and $plan.live_product_claim.remaining_backend_contract_count == 12
      and $plan.live_product_claim.next_owner_lane == "backend_contract"
      and $plan.live_product_claim.ready == false
      and $plan.claim_boundary.live_product_claim_ready == false
      and $plan.claim_boundary.public_distribution_claim_ready == false
      and $plan.claim_boundary.release_claim_ready == false
      and ($plan.next_plan | length) == 3
      and $demo.demo_evidence_gate_ready == true
      and $demo.claim_boundary.local_fixture_demo_evidence_ready == true
      and $archive.evidence_archive_gate_ready == true
      and $archive.claim_boundary.local_evidence_archive_ready == true
      and $archive.all_extracted_items_sha256_match == true
      and ($archive.archive_sha256 | test("^[0-9a-f]{64}$"))
      and $archive.archive_bytes > 0
      and $release_dry_run.release_operator_dry_run_gate_ready == true
      and $release_dry_run.claim_boundary.local_release_operator_dry_run_ready == true
      and $release_dry_run.denial_case_count == 4
      and $release_dry_run.allowed_dry_run_case_count == 1
      and $release_dry_run.operator_packet.operator_approval_recorded == false
      and $release_dry_run.operator_packet.credential_values_read == false
      and $release_dry_run.operator_packet.notary_submission_performed == false
      and $release_dry_run.operator_packet.public_distribution_artifact_written == false
      and $release_dry_run.claim_boundary.release_execution_ready == false
      and $release_dry_run.claim_boundary.release_claim_ready == false
      and $release_dry_run.claim_boundary.public_distribution_claim_ready == false
      and $operator.operator_briefing_gate_ready == true
      and $operator.critical_risk_count == 3
      and $operator.backend_remaining_contract_count == 12
      and ($operator.backend_priority_ids | length) == 12
      and $operator.backend_priority_ids[0:5] == selected_ids
      and $promotion.backend_promotion_packet_gate_ready == true
      and $promotion.selected_priority_ids == selected_ids
      and $promotion.priority_packet_count == 5
      and ($promotion.priority_packets | all(.next_owner_lane == "backend_contract"))
      and ($promotion.priority_packets | all(.promote_requires_backend_adapter == true))
      and ($promotion.priority_packets | all(.promote_requires_readback_evidence == true))
      and ($promotion.priority_packets | all(.active_promotion_performed == false))
      and $alignment.backend_alignment_evidence_gate_ready == true
      and $alignment.selected_alignment_ids == selected_ids
      and $alignment.alignment_item_count == 5
      and ($alignment.alignment_items | all(.alignment_ready == true))
      and ($alignment.alignment_items | all(.next_owner_lane == "backend_contract"))
      and ($alignment.alignment_items | all(.status == "partial_live_backend_contract_remaining"))
      and $alignment.claim_boundary.backend_adapter_promoted == false
      and $alignment.claim_boundary.live_runtime_mutation == false
      and $alignment.claim_boundary.live_product_claim_ready == false
      and sha_ready($plan_boundary_sha)
      and sha_ready($demo_evidence_sha)
      and sha_ready($evidence_archive_sha)
      and sha_ready($release_operator_sha)
      and sha_ready($operator_briefing_sha)
      and sha_ready($backend_promotion_sha)
      and sha_ready($backend_alignment_sha);
    source_chain_ready as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      critical_path_plan_gate_ready:$ready,
      plan_kind:"local_ui_critical_path_plan",
      plan_version:1,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      source_reports:{
        plan_boundary:$plan_boundary_path,
        demo_evidence:$demo_evidence_path,
        evidence_archive:$evidence_archive_path,
        release_operator_dry_run:$release_operator_path,
        operator_briefing:$operator_briefing_path,
        backend_promotion_packet:$backend_promotion_path,
        backend_alignment_evidence:$backend_alignment_path
      },
      source_report_sha256:{
        plan_boundary:$plan_boundary_sha,
        demo_evidence:$demo_evidence_sha,
        evidence_archive:$evidence_archive_sha,
        release_operator_dry_run:$release_operator_sha,
        operator_briefing:$operator_briefing_sha,
        backend_promotion_packet:$backend_promotion_sha,
        backend_alignment_evidence:$backend_alignment_sha
      },
      current_state:{
        local_fixture_demo_ready:$plan.claim_boundary.local_fixture_demo_ready,
        local_fixture_demo_evidence_ready:$demo.claim_boundary.local_fixture_demo_evidence_ready,
        hard_true_window_required_for_public_demo:$demo.claim_boundary.hard_true_window_required,
        hard_true_window_gate_currently_ready:($demo.claim_boundary.r33_hard_demo_evidence_ready // false),
        local_evidence_archive_ready:$archive.claim_boundary.local_evidence_archive_ready,
        evidence_archive_sha256:$archive.archive_sha256,
        local_release_operator_dry_run_ready:$release_dry_run.claim_boundary.local_release_operator_dry_run_ready,
        backend_alignment_evidence_ready:$alignment.claim_boundary.local_backend_alignment_evidence_ready,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false
      },
      critical_blockers:[
        {
          id:"live_backend_contracts_remaining",
          severity:"blocker",
          owner_lane:"backend_contract",
          remaining_count:$plan.live_product_claim.remaining_backend_contract_count,
          current_backend_selected_ids:selected_ids,
          ui_lane_state:$plan.live_product_claim.ui_lane_state,
          current_alignment_evidence_ready:$alignment.claim_boundary.local_backend_alignment_evidence_ready,
          backend_adapter_promoted:false
        },
        {
          id:"release_public_distribution_not_approved",
          severity:"blocker",
          owner_lane:"release_operator",
          blockers:release_blockers,
          denial_case_count:$release_dry_run.denial_case_count,
          operator_approval_recorded:$release_dry_run.operator_packet.operator_approval_recorded,
          credential_values_read:false,
          notary_submission_performed:false,
          public_distribution_artifact_written:false
        },
        {
          id:"hard_true_window_required_for_public_demo",
          severity:"guardrail",
          owner_lane:"hepta-ui",
          hard_true_window_required:($demo.claim_boundary.hard_true_window_required // false),
          current_hard_evidence_ready:($demo.claim_boundary.r33_hard_demo_evidence_ready // false),
          public_demo_claim_allowed:false
        }
      ],
      critical_blocker_count:3,
      current_backend_selected_ids:selected_ids,
      backend_priority_ids:$operator.backend_priority_ids,
      backend_alignment_items:($alignment.alignment_items | map({
        id,
        priority,
        backend_contract_gate,
        next_owner_lane,
        status,
        alignment_ready,
        required_backend_contract_count,
        promotion_requires,
        active_promotion_performed,
        promotion_blocker
      })),
      future_plan:[
        {
          priority:1,
          id:"backend_contract_first_five",
          owner_lane:"backend_contract",
          action:"promote selected backend-owned adapters only after adapter contract, readback evidence, and side-effect review",
          selected_ids:selected_ids
        },
        {
          priority:2,
          id:"hepta_ui_hard_evidence_refresh",
          owner_lane:"hepta-ui",
          action:"refresh no-window and full-hard readiness after backend changes; keep public-demo claims gated by hard true-window evidence"
        },
        {
          priority:3,
          id:"release_operator_after_approval",
          owner_lane:"release_operator",
          action:"run signed/notarized/stapled/public artifact evidence only after explicit release approval",
          blockers:release_blockers
        }
      ],
      future_plan_count:3,
      answer_guardrail:{
        allowed_summary:"local UI readiness, critical blockers, backend handoff evidence, and release dry-run denial only",
        forbidden_claims:["live_product_ready","public_distribution_ready","release_ready"],
        required_closeout_fields:["critical_path_report","backend_selected_ids","critical_blocker_count","claim_boundary_false"]
      },
      claim_boundary:{
        local_critical_path_plan_ready:$ready,
        local_fixture_demo_ready:$plan.claim_boundary.local_fixture_demo_ready,
        local_evidence_archive_ready:$archive.claim_boundary.local_evidence_archive_ready,
        local_release_operator_dry_run_ready:$release_dry_run.claim_boundary.local_release_operator_dry_run_ready,
        local_backend_alignment_evidence_ready:$alignment.claim_boundary.local_backend_alignment_evidence_ready,
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
  and .critical_path_plan_gate_ready == true
  and .plan_kind == "local_ui_critical_path_plan"
  and .plan_version == 1
  and .critical_blocker_count == 3
  and (.critical_blockers | map(.id) | index("live_backend_contracts_remaining") != null)
  and (.critical_blockers | map(.id) | index("release_public_distribution_not_approved") != null)
  and (.critical_blockers | map(.id) | index("hard_true_window_required_for_public_demo") != null)
  and .current_backend_selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and (.backend_priority_ids | length) == 12
  and .backend_priority_ids[0:5] == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and (.backend_alignment_items | length) == 5
  and (.backend_alignment_items | all(.alignment_ready == true))
  and (.backend_alignment_items | all(.next_owner_lane == "backend_contract"))
  and (.backend_alignment_items | all(.required_backend_contract_count >= 5))
  and .future_plan_count == 3
  and .future_plan[0].id == "backend_contract_first_five"
  and .future_plan[1].id == "hepta_ui_hard_evidence_refresh"
  and .future_plan[2].id == "release_operator_after_approval"
  and .current_state.local_fixture_demo_ready == true
  and .current_state.local_evidence_archive_ready == true
  and (.current_state.evidence_archive_sha256 | test("^[0-9a-f]{64}$"))
  and .current_state.local_release_operator_dry_run_ready == true
  and .current_state.backend_alignment_evidence_ready == true
  and .current_state.live_product_claim_ready == false
  and .current_state.public_distribution_claim_ready == false
  and .current_state.release_claim_ready == false
  and (.answer_guardrail.forbidden_claims | index("live_product_ready") != null)
  and (.answer_guardrail.forbidden_claims | index("public_distribution_ready") != null)
  and (.answer_guardrail.forbidden_claims | index("release_ready") != null)
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
