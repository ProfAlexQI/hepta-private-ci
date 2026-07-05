#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_FUTURE_PLAN_REFRESH_REPORT_PATH:-$READINESS_DIR/ui-future-plan-refresh-gate.json}"

PLAN_BOUNDARY_REPORT_PATH="$READINESS_DIR/ui-plan-boundary-gate.json"
DEMO_EVIDENCE_REPORT_PATH="$READINESS_DIR/ui-demo-evidence-gate.json"
CRITICAL_PATH_PLAN_REPORT_PATH="$READINESS_DIR/ui-critical-path-plan-gate.json"
BACKEND_CONTRACT_ACCEPTANCE_REPORT_PATH="$READINESS_DIR/ui-backend-contract-acceptance-gate.json"
BACKEND_HANDOFF_EXPORT_REPORT_PATH="$READINESS_DIR/ui-backend-handoff-export-gate.json"
BACKEND_DISPATCH_PACKET_REPORT_PATH="$READINESS_DIR/ui-backend-dispatch-packet-gate.json"
BACKEND_RECEIPT_INTAKE_REPORT_PATH="$READINESS_DIR/ui-backend-receipt-intake-gate.json"
BACKEND_RECEIPT_ROUNDTRIP_REPORT_PATH="$READINESS_DIR/ui-backend-receipt-roundtrip-gate.json"
BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH="$READINESS_DIR/ui-backend-receipt-refresh-lock-gate.json"
NATIVE_WINDOW_REPORT_PATH="$READINESS_DIR/native-window-smoke.json"
NATIVE_WINDOW_ROUTE_REPORT_PATH="$READINESS_DIR/native-window-routes-smoke.json"
NATIVE_WINDOW_SECONDARY_REPORT_PATH="$READINESS_DIR/native-window-secondary-smoke.json"
NATIVE_WINDOW_SECONDARY_MOBILE_REPORT_PATH="$READINESS_DIR/native-window-secondary-mobile-smoke.json"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf '%s is required for the Hepta UI future-plan refresh gate\n' "$1" >&2
    exit 2
  fi
}

require_report() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required future-plan refresh input: %s\n' "$path" >&2
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
require_report "$CRITICAL_PATH_PLAN_REPORT_PATH"
require_report "$BACKEND_CONTRACT_ACCEPTANCE_REPORT_PATH"
require_report "$BACKEND_HANDOFF_EXPORT_REPORT_PATH"
require_report "$BACKEND_DISPATCH_PACKET_REPORT_PATH"
require_report "$BACKEND_RECEIPT_INTAKE_REPORT_PATH"
require_report "$BACKEND_RECEIPT_ROUNDTRIP_REPORT_PATH"
require_report "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH"
require_report "$NATIVE_WINDOW_REPORT_PATH"
require_report "$NATIVE_WINDOW_ROUTE_REPORT_PATH"
require_report "$NATIVE_WINDOW_SECONDARY_REPORT_PATH"
require_report "$NATIVE_WINDOW_SECONDARY_MOBILE_REPORT_PATH"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-future-plan-refresh.XXXXXX")"
REPORT_TMP="$TMP_DIR/future-plan-refresh-report.json"
trap 'rm -rf "$TMP_DIR"' EXIT

plan_boundary_sha="$(file_sha256 "$PLAN_BOUNDARY_REPORT_PATH")"
demo_evidence_sha="$(file_sha256 "$DEMO_EVIDENCE_REPORT_PATH")"
critical_path_sha="$(file_sha256 "$CRITICAL_PATH_PLAN_REPORT_PATH")"
backend_acceptance_sha="$(file_sha256 "$BACKEND_CONTRACT_ACCEPTANCE_REPORT_PATH")"
backend_handoff_sha="$(file_sha256 "$BACKEND_HANDOFF_EXPORT_REPORT_PATH")"
backend_dispatch_sha="$(file_sha256 "$BACKEND_DISPATCH_PACKET_REPORT_PATH")"
backend_receipt_intake_sha="$(file_sha256 "$BACKEND_RECEIPT_INTAKE_REPORT_PATH")"
backend_receipt_roundtrip_sha="$(file_sha256 "$BACKEND_RECEIPT_ROUNDTRIP_REPORT_PATH")"
backend_receipt_refresh_lock_sha="$(file_sha256 "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH")"
native_window_sha="$(file_sha256 "$NATIVE_WINDOW_REPORT_PATH")"
native_window_route_sha="$(file_sha256 "$NATIVE_WINDOW_ROUTE_REPORT_PATH")"
native_window_secondary_sha="$(file_sha256 "$NATIVE_WINDOW_SECONDARY_REPORT_PATH")"
native_window_secondary_mobile_sha="$(file_sha256 "$NATIVE_WINDOW_SECONDARY_MOBILE_REPORT_PATH")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_future_plan_refresh_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg plan_boundary_path "$PLAN_BOUNDARY_REPORT_PATH" \
  --arg demo_evidence_path "$DEMO_EVIDENCE_REPORT_PATH" \
  --arg critical_path_path "$CRITICAL_PATH_PLAN_REPORT_PATH" \
  --arg backend_acceptance_path "$BACKEND_CONTRACT_ACCEPTANCE_REPORT_PATH" \
  --arg backend_handoff_path "$BACKEND_HANDOFF_EXPORT_REPORT_PATH" \
  --arg backend_dispatch_path "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  --arg backend_receipt_intake_path "$BACKEND_RECEIPT_INTAKE_REPORT_PATH" \
  --arg backend_receipt_roundtrip_path "$BACKEND_RECEIPT_ROUNDTRIP_REPORT_PATH" \
  --arg backend_receipt_refresh_lock_path "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH" \
  --arg native_window_path "$NATIVE_WINDOW_REPORT_PATH" \
  --arg native_window_route_path "$NATIVE_WINDOW_ROUTE_REPORT_PATH" \
  --arg native_window_secondary_path "$NATIVE_WINDOW_SECONDARY_REPORT_PATH" \
  --arg native_window_secondary_mobile_path "$NATIVE_WINDOW_SECONDARY_MOBILE_REPORT_PATH" \
  --arg plan_boundary_sha "$plan_boundary_sha" \
  --arg demo_evidence_sha "$demo_evidence_sha" \
  --arg critical_path_sha "$critical_path_sha" \
  --arg backend_acceptance_sha "$backend_acceptance_sha" \
  --arg backend_handoff_sha "$backend_handoff_sha" \
  --arg backend_dispatch_sha "$backend_dispatch_sha" \
  --arg backend_receipt_intake_sha "$backend_receipt_intake_sha" \
  --arg backend_receipt_roundtrip_sha "$backend_receipt_roundtrip_sha" \
  --arg backend_receipt_refresh_lock_sha "$backend_receipt_refresh_lock_sha" \
  --arg native_window_sha "$native_window_sha" \
  --arg native_window_route_sha "$native_window_route_sha" \
  --arg native_window_secondary_sha "$native_window_secondary_sha" \
  --arg native_window_secondary_mobile_sha "$native_window_secondary_mobile_sha" \
  --slurpfile plan_boundary_file "$PLAN_BOUNDARY_REPORT_PATH" \
  --slurpfile demo_evidence_file "$DEMO_EVIDENCE_REPORT_PATH" \
  --slurpfile critical_path_file "$CRITICAL_PATH_PLAN_REPORT_PATH" \
  --slurpfile backend_acceptance_file "$BACKEND_CONTRACT_ACCEPTANCE_REPORT_PATH" \
  --slurpfile backend_handoff_file "$BACKEND_HANDOFF_EXPORT_REPORT_PATH" \
  --slurpfile backend_dispatch_file "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  --slurpfile backend_receipt_intake_file "$BACKEND_RECEIPT_INTAKE_REPORT_PATH" \
  --slurpfile backend_receipt_roundtrip_file "$BACKEND_RECEIPT_ROUNDTRIP_REPORT_PATH" \
  --slurpfile backend_receipt_refresh_lock_file "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH" \
  --slurpfile native_window_file "$NATIVE_WINDOW_REPORT_PATH" \
  --slurpfile native_window_route_file "$NATIVE_WINDOW_ROUTE_REPORT_PATH" \
  --slurpfile native_window_secondary_file "$NATIVE_WINDOW_SECONDARY_REPORT_PATH" \
  --slurpfile native_window_secondary_mobile_file "$NATIVE_WINDOW_SECONDARY_MOBILE_REPORT_PATH" \
  '
  ($plan_boundary_file[0]) as $plan
  | ($demo_evidence_file[0]) as $demo
  | ($critical_path_file[0]) as $critical
  | ($backend_acceptance_file[0]) as $acceptance
  | ($backend_handoff_file[0]) as $handoff
  | ($backend_dispatch_file[0]) as $dispatch
  | ($backend_receipt_intake_file[0]) as $intake
  | ($backend_receipt_roundtrip_file[0]) as $roundtrip
  | ($backend_receipt_refresh_lock_file[0]) as $refresh_lock
  | ($native_window_file[0]) as $window
  | ($native_window_route_file[0]) as $window_route
  | ($native_window_secondary_file[0]) as $window_secondary
  | ($native_window_secondary_mobile_file[0]) as $window_secondary_mobile
  | def selected_ids: ["message_search","file_upload_send","media_download_playback","notifications","room_settings"];
    def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
    def hard_true_window_ready:
      $refresh_lock.refresh_requirements.hard_true_window_refresh_ready == true
      and $window.enabled == true
      and $window.status == "ready"
      and (($window.screenshots // []) | length) == 2
      and $window_route.enabled == true
      and $window_route.status == "ready"
      and $window_route.screenshot_count == 4
      and $window_route.route_screenshot_unique_count == 4
      and $window_route.route_content_probe_ready == true
      and $window_secondary.enabled == true
      and $window_secondary.status == "ready"
      and $window_secondary.screenshot_count == 5
      and $window_secondary.surface_screenshot_unique_count == 5
      and $window_secondary_mobile.enabled == true
      and $window_secondary_mobile.status == "ready"
      and $window_secondary_mobile.screenshot_count == 5
      and $window_secondary_mobile.surface_screenshot_unique_count == 5
      and $window_secondary_mobile.mobile_secondary_content_probe_ready == true
      and $window_secondary_mobile.mobile_secondary_content_visible_count >= 5;
    def backend_receipt_claim_state_valid:
      (
        $refresh_lock.claim_boundary.backend_receipt_claim_ready == true
        and $refresh_lock.claim_boundary.real_backend_receipt_claim_ready == true
        and $refresh_lock.claim_boundary.backend_receipt_full_hard_refresh_ready == true
        and $refresh_lock.refresh_requirements.full_hard_refresh_required == false
        and $refresh_lock.refresh_requirements.full_hard_refresh_ready == true
        and hard_true_window_ready
      )
      or (
        $refresh_lock.claim_boundary.backend_receipt_claim_ready == false
        and $refresh_lock.claim_boundary.real_backend_receipt_claim_ready == false
        and $refresh_lock.claim_boundary.backend_receipt_full_hard_refresh_ready == false
      );
    def source_chain_ready:
      $plan.plan_boundary_gate_ready == true
      and ($plan.next_plan | length) == 3
      and $demo.demo_evidence_gate_ready == true
      and $critical.critical_path_plan_gate_ready == true
      and $critical.future_plan_count == 3
      and $critical.current_backend_selected_ids == selected_ids
      and $acceptance.backend_contract_acceptance_gate_ready == true
      and $acceptance.selected_acceptance_ids == selected_ids
      and $acceptance.future_plan_link.critical_path_plan_id == "backend_contract_first_five"
      and $acceptance.future_plan_link.hepta_ui_after_backend_refresh == "hepta_ui_hard_evidence_refresh"
      and $handoff.backend_handoff_export_gate_ready == true
      and $handoff.selected_export_ids == selected_ids
      and $handoff.backend_lane_target.target_repo == "/Users/qianqi/.openclaw/workspace/Hepta"
      and $dispatch.backend_dispatch_packet_gate_ready == true
      and $dispatch.selected_packet_ids == selected_ids
      and $intake.backend_receipt_intake_gate_ready == true
      and $intake.selected_receipt_ids == selected_ids
      and $roundtrip.backend_receipt_roundtrip_gate_ready == true
      and $roundtrip.selected_roundtrip_ids == selected_ids
      and $roundtrip.source_alignment.backend_receipt_present_branch_ready == true
      and $roundtrip.source_alignment.simulated_receipt_ready == true
      and $refresh_lock.backend_receipt_refresh_lock_gate_ready == true
      and $refresh_lock.selected_refresh_ids == selected_ids
      and $refresh_lock.misclaim_lock.simulated_receipt_not_promoted_to_backend_receipt == true
      and backend_receipt_claim_state_valid
      and sha_ready($plan_boundary_sha)
      and sha_ready($demo_evidence_sha)
      and sha_ready($critical_path_sha)
      and sha_ready($backend_acceptance_sha)
      and sha_ready($backend_handoff_sha)
      and sha_ready($backend_dispatch_sha)
      and sha_ready($backend_receipt_intake_sha)
      and sha_ready($backend_receipt_roundtrip_sha)
      and sha_ready($backend_receipt_refresh_lock_sha)
      and sha_ready($native_window_sha)
      and sha_ready($native_window_route_sha)
      and sha_ready($native_window_secondary_sha)
      and sha_ready($native_window_secondary_mobile_sha);
    (
      source_chain_ready
      and $plan.live_product_claim.remaining_backend_contract_count == 12
      and $plan.claim_boundary.live_product_claim_ready == false
      and $plan.claim_boundary.public_distribution_claim_ready == false
      and $plan.claim_boundary.release_claim_ready == false
      and backend_receipt_claim_state_valid
      and $refresh_lock.claim_boundary.live_product_claim_ready == false
      and $refresh_lock.claim_boundary.public_distribution_claim_ready == false
      and $refresh_lock.claim_boundary.release_claim_ready == false
    ) as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      future_plan_refresh_gate_ready:$ready,
      plan_kind:"local_ui_future_plan_refresh_after_backend_receipt_lock",
      plan_version:1,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      source_reports:{
        plan_boundary:$plan_boundary_path,
        demo_evidence:$demo_evidence_path,
        critical_path_plan:$critical_path_path,
        backend_contract_acceptance:$backend_acceptance_path,
        backend_handoff_export:$backend_handoff_path,
        backend_dispatch_packet:$backend_dispatch_path,
        backend_receipt_intake:$backend_receipt_intake_path,
        backend_receipt_roundtrip:$backend_receipt_roundtrip_path,
        backend_receipt_refresh_lock:$backend_receipt_refresh_lock_path,
        native_window:$native_window_path,
        native_window_route:$native_window_route_path,
        native_window_secondary:$native_window_secondary_path,
        native_window_secondary_mobile:$native_window_secondary_mobile_path
      },
      source_report_sha256:{
        plan_boundary:$plan_boundary_sha,
        demo_evidence:$demo_evidence_sha,
        critical_path_plan:$critical_path_sha,
        backend_contract_acceptance:$backend_acceptance_sha,
        backend_handoff_export:$backend_handoff_sha,
        backend_dispatch_packet:$backend_dispatch_sha,
        backend_receipt_intake:$backend_receipt_intake_sha,
        backend_receipt_roundtrip:$backend_receipt_roundtrip_sha,
        backend_receipt_refresh_lock:$backend_receipt_refresh_lock_sha,
        native_window:$native_window_sha,
        native_window_route:$native_window_route_sha,
        native_window_secondary:$native_window_secondary_sha,
        native_window_secondary_mobile:$native_window_secondary_mobile_sha
      },
      r52_minimum_gate:{
        gate_id:"r52_equivalent_hard_ui_demo_gate",
        defined:true,
        current_full_hard_evidence_ready:hard_true_window_ready,
        root_report_replay_required_count:32,
        main_true_window_required:2,
        route_true_window_required:4,
        route_unique_required:4,
        desktop_secondary_required:5,
        mobile_secondary_required:5,
        mobile_content_probe_required:true,
        backend_receipt_roundtrip_required:true,
        backend_receipt_refresh_lock_required:true,
        blocked_allowed_required:false,
        counts:{
          main:(($window.screenshots // []) | length),
          route:($window_route.screenshot_count // 0),
          route_unique:($window_route.route_screenshot_unique_count // 0),
          desktop_secondary:($window_secondary.screenshot_count // 0),
          desktop_secondary_unique:($window_secondary.surface_screenshot_unique_count // 0),
          mobile_secondary:($window_secondary_mobile.screenshot_count // 0),
          mobile_secondary_unique:($window_secondary_mobile.surface_screenshot_unique_count // 0),
          mobile_secondary_content_visible_count:($window_secondary_mobile.mobile_secondary_content_visible_count // 0)
        }
      },
      backend_receipt_refresh_contract:{
        selected_ids:selected_ids,
        real_backend_receipt_present:$refresh_lock.receipt_state.real_backend_receipt_present,
        backend_receipt_claim_ready:$refresh_lock.claim_boundary.backend_receipt_claim_ready,
        simulated_branch_available:$refresh_lock.misclaim_lock.simulated_receipt_branch_available,
        simulated_branch_not_promoted:$refresh_lock.misclaim_lock.simulated_receipt_not_promoted_to_backend_receipt,
        required_ui_refresh_commands:$refresh_lock.refresh_requirements.required_ui_refresh_commands,
        full_hard_refresh_required:$refresh_lock.refresh_requirements.full_hard_refresh_required,
        full_hard_refresh_ready:$refresh_lock.refresh_requirements.full_hard_refresh_ready
      },
      future_plan:[
        {
          priority:1,
          id:"r52_minimum_ui_demo_gate",
          owner_lane:"hepta-ui",
          action:"keep r52-equivalent hard readiness as the minimum UI demo gate before any public demo claim",
          required_evidence:["main_true_window_2","route_true_window_4_unique_with_content_probe","desktop_secondary_5_unique","mobile_secondary_5_unique_with_content_probe","root_report_replay_32","backend_receipt_roundtrip_green","backend_receipt_refresh_lock_green"],
          current_full_hard_evidence_ready:hard_true_window_ready
        },
        {
          priority:2,
          id:"backend_real_receipt_return",
          owner_lane:"backend_contract",
          action:"execute the first five backend dispatch items and return a real receipt matching receipt template plus refresh-lock requirements",
          selected_ids:selected_ids,
          target_repo:"/Users/qianqi/.openclaw/workspace/Hepta"
        },
        {
          priority:3,
          id:"ui_refresh_after_real_receipt",
          owner_lane:"hepta-ui",
          action:"rerun no-window and full-hard readiness with the real backend receipt before claiming backend receipt acceptance",
          required_commands:$refresh_lock.refresh_requirements.required_ui_refresh_commands
        }
      ],
      future_plan_count:3,
      stale_plan_lock:{
        plan_boundary_old_minimum_id:$plan.next_plan[0].id,
        refreshed_minimum_id:"r52_minimum_ui_demo_gate",
        refreshed_plan_supersedes_plan_boundary_next_plan:true,
        plan_boundary_next_plan_kept_for_legacy_replay:true
      },
      claim_boundary:{
        local_future_plan_refresh_ready:$ready,
        local_backend_receipt_refresh_lock_ready:$refresh_lock.claim_boundary.local_backend_receipt_refresh_lock_ready,
        real_backend_receipt_claim_ready:$refresh_lock.claim_boundary.real_backend_receipt_claim_ready,
        backend_receipt_claim_ready:$refresh_lock.claim_boundary.backend_receipt_claim_ready,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false,
        backend_adapter_promoted:false,
        live_runtime_mutation:false,
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
  and .future_plan_refresh_gate_ready == true
  and .plan_kind == "local_ui_future_plan_refresh_after_backend_receipt_lock"
  and .plan_version == 1
  and .r52_minimum_gate.defined == true
  and .r52_minimum_gate.root_report_replay_required_count == 32
  and .r52_minimum_gate.backend_receipt_roundtrip_required == true
  and .r52_minimum_gate.backend_receipt_refresh_lock_required == true
  and .backend_receipt_refresh_contract.selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and (.backend_receipt_refresh_contract.real_backend_receipt_present | type) == "boolean"
  and (.backend_receipt_refresh_contract.backend_receipt_claim_ready | type) == "boolean"
  and .backend_receipt_refresh_contract.simulated_branch_not_promoted == true
  and (
    (
      .backend_receipt_refresh_contract.backend_receipt_claim_ready == true
      and .backend_receipt_refresh_contract.full_hard_refresh_required == false
      and .backend_receipt_refresh_contract.full_hard_refresh_ready == true
      and .r52_minimum_gate.current_full_hard_evidence_ready == true
      and .claim_boundary.real_backend_receipt_claim_ready == true
      and .claim_boundary.backend_receipt_claim_ready == true
    )
    or
    (
      .backend_receipt_refresh_contract.backend_receipt_claim_ready == false
      and .claim_boundary.real_backend_receipt_claim_ready == false
      and .claim_boundary.backend_receipt_claim_ready == false
    )
  )
  and (.backend_receipt_refresh_contract.required_ui_refresh_commands | length) == 2
  and .future_plan_count == 3
  and .future_plan[0].id == "r52_minimum_ui_demo_gate"
  and .future_plan[1].id == "backend_real_receipt_return"
  and .future_plan[2].id == "ui_refresh_after_real_receipt"
  and .future_plan[1].selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .future_plan[1].target_repo == "/Users/qianqi/.openclaw/workspace/Hepta"
  and (.future_plan[2].required_commands | length) == 2
  and .stale_plan_lock.refreshed_minimum_id == "r52_minimum_ui_demo_gate"
  and .stale_plan_lock.refreshed_plan_supersedes_plan_boundary_next_plan == true
  and .claim_boundary.local_future_plan_refresh_ready == true
  and .claim_boundary.local_backend_receipt_refresh_lock_ready == true
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.backend_adapter_promoted == false
  and .claim_boundary.live_runtime_mutation == false
  and .claim_boundary.external_actions_allowed == false
  and .claim_boundary.public_upload_performed == false
  and .claim_boundary.signing_notarization_performed == false
  and .side_effects.external_mutation == false
' "$REPORT_TMP" >/dev/null

mkdir -p "$(dirname "$REPORT_PATH")"
cp "$REPORT_TMP" "$REPORT_PATH"
cat "$REPORT_TMP"
