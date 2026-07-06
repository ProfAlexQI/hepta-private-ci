#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_PLAN_BOUNDARY_REPORT_PATH:-}"

READINESS_PATH="$READINESS_DIR/readiness.json"
HANDOFF_PATH="$READINESS_DIR/native-base-gap-backend-handoff.json"
WORK_QUEUE_PATH="$READINESS_DIR/native-base-gap-work-queue.json"
DISTRIBUTION_PREFLIGHT_PATH="$READINESS_DIR/native-distribution-preflight-gate.json"
PRODUCTIZATION_ROLLUP_PATH="$READINESS_DIR/native-productization-blocker-rollup.json"
PACKAGING_PATH="$READINESS_DIR/native-packaging-gate.json"
NATIVE_WINDOW_PATH="$READINESS_DIR/native-window-smoke.json"
NATIVE_WINDOW_ROUTE_PATH="$READINESS_DIR/native-window-routes-smoke.json"
NATIVE_WINDOW_SECONDARY_PATH="$READINESS_DIR/native-window-secondary-smoke.json"
NATIVE_WINDOW_SECONDARY_MOBILE_PATH="$READINESS_DIR/native-window-secondary-mobile-smoke.json"

require_file() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required plan-boundary input: %s\n' "$path" >&2
    exit 1
  fi
}

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-plan-boundary.XXXXXX")"
trap 'rm -rf "$TMP_DIR"' EXIT

json_input_path_or_default() {
  local path="$1"
  local fallback="$2"
  local name="$3"
  if [[ -s "$path" ]]; then
    printf '%s\n' "$path"
  else
    local fallback_path="$TMP_DIR/$name"
    printf '%s\n' "$fallback" >"$fallback_path"
    printf '%s\n' "$fallback_path"
  fi
}

require_file "$HANDOFF_PATH"
require_file "$WORK_QUEUE_PATH"
require_file "$DISTRIBUTION_PREFLIGHT_PATH"
require_file "$PRODUCTIZATION_ROLLUP_PATH"
require_file "$PACKAGING_PATH"

if [[ -s "$READINESS_PATH" ]]; then
  readiness_report_state="ready"
else
  readiness_report_state="in_progress_artifacts_ready"
fi

READINESS_INPUT_PATH="$(json_input_path_or_default "$READINESS_PATH" '{}' "readiness-default.json")"
NATIVE_WINDOW_INPUT_PATH="$(json_input_path_or_default "$NATIVE_WINDOW_PATH" '{"enabled":false,"status":"not_run","blocked_allowed":false,"screenshots":[]}' "native-window-default.json")"
NATIVE_WINDOW_ROUTE_INPUT_PATH="$(json_input_path_or_default "$NATIVE_WINDOW_ROUTE_PATH" '{"enabled":false,"status":"not_run","blocked_allowed":false,"screenshots":[]}' "native-window-route-default.json")"
NATIVE_WINDOW_SECONDARY_INPUT_PATH="$(json_input_path_or_default "$NATIVE_WINDOW_SECONDARY_PATH" '{"enabled":false,"status":"not_run","blocked_allowed":false,"screenshots":[]}' "native-window-secondary-default.json")"
NATIVE_WINDOW_SECONDARY_MOBILE_INPUT_PATH="$(json_input_path_or_default "$NATIVE_WINDOW_SECONDARY_MOBILE_PATH" '{"enabled":false,"status":"not_run","blocked_allowed":false,"screenshots":[]}' "native-window-secondary-mobile-default.json")"

REPORT_TMP="$TMP_DIR/plan-boundary-report.json"
jq -n \
    --arg product "Hepta UI" \
    --arg runtime "hepta" \
    --arg gate "hepta_ui_plan_boundary_gate" \
    --arg readiness_dir "$READINESS_DIR" \
    --arg readiness_report_state "$readiness_report_state" \
    --arg readiness_path "$READINESS_PATH" \
    --arg handoff_path "$HANDOFF_PATH" \
    --arg work_queue_path "$WORK_QUEUE_PATH" \
    --arg distribution_preflight_path "$DISTRIBUTION_PREFLIGHT_PATH" \
    --arg productization_rollup_path "$PRODUCTIZATION_ROLLUP_PATH" \
    --arg packaging_path "$PACKAGING_PATH" \
    --arg native_window_path "$NATIVE_WINDOW_PATH" \
    --arg native_window_route_path "$NATIVE_WINDOW_ROUTE_PATH" \
    --arg native_window_secondary_path "$NATIVE_WINDOW_SECONDARY_PATH" \
    --arg native_window_secondary_mobile_path "$NATIVE_WINDOW_SECONDARY_MOBILE_PATH" \
    --slurpfile readiness_file "$READINESS_INPUT_PATH" \
    --slurpfile handoff_file "$HANDOFF_PATH" \
    --slurpfile work_queue_file "$WORK_QUEUE_PATH" \
    --slurpfile distribution_file "$DISTRIBUTION_PREFLIGHT_PATH" \
    --slurpfile rollup_file "$PRODUCTIZATION_ROLLUP_PATH" \
    --slurpfile packaging_file "$PACKAGING_PATH" \
    --slurpfile native_window_file "$NATIVE_WINDOW_INPUT_PATH" \
    --slurpfile native_window_route_file "$NATIVE_WINDOW_ROUTE_INPUT_PATH" \
    --slurpfile native_window_secondary_file "$NATIVE_WINDOW_SECONDARY_INPUT_PATH" \
    --slurpfile native_window_secondary_mobile_file "$NATIVE_WINDOW_SECONDARY_MOBILE_INPUT_PATH" \
    '
    ($readiness_file[0]) as $readiness
    | ($handoff_file[0]) as $handoff
    | ($work_queue_file[0]) as $work_queue
    | ($distribution_file[0]) as $distribution
    | ($rollup_file[0]) as $rollup
    | ($packaging_file[0]) as $packaging
    | ($native_window_file[0]) as $native_window
    | ($native_window_route_file[0]) as $native_window_route
    | ($native_window_secondary_file[0]) as $native_window_secondary
    | ($native_window_secondary_mobile_file[0]) as $native_window_secondary_mobile
    |
    def enabled($gate): $gate.enabled == true;
    def screenshots($gate): (($gate.screenshots // []) | length);
    def route_content_ready:
      enabled($native_window_route)
      and $native_window_route.status == "ready"
      and ($native_window_route.blocked_allowed // false) != true
      and $native_window_route.true_window_capture_performed == true
      and $native_window_route.route_content_probe_ready == true
      and $native_window_route.route_top_design_referee_ready == true
      and $native_window_route.route_count == 4
      and $native_window_route.route_screenshot_unique_count == 4
      and $native_window_route.screenshot_count == 4
      and (($native_window_route.screenshots // []) | all(.visual_probe.route_content_ready == true))
      and $native_window_route.native_app_log_error_free == true;
    def main_window_ready:
      enabled($native_window)
      and $native_window.status == "ready"
      and ($native_window.blocked_allowed // false) != true
      and $native_window.true_window_capture_performed == true
      and screenshots($native_window) == 2
      and $native_window.native_app_log_error_free == true;
    def desktop_secondary_ready:
      enabled($native_window_secondary)
      and $native_window_secondary.status == "ready"
      and ($native_window_secondary.blocked_allowed // false) != true
      and $native_window_secondary.true_window_capture_performed == true
      and $native_window_secondary.surface_count == 5
      and $native_window_secondary.surface_screenshot_unique_count == 5
      and $native_window_secondary.screenshot_count == 5
      and $native_window_secondary.native_app_log_error_free == true;
    def mobile_secondary_ready:
      enabled($native_window_secondary_mobile)
      and $native_window_secondary_mobile.status == "ready"
      and ($native_window_secondary_mobile.blocked_allowed // false) != true
      and $native_window_secondary_mobile.true_window_capture_performed == true
      and $native_window_secondary_mobile.mobile_secondary_content_probe_ready == true
      and $native_window_secondary_mobile.mobile_secondary_content_visible_count >= 5
      and $native_window_secondary_mobile.surface_count == 5
      and $native_window_secondary_mobile.surface_screenshot_unique_count == 5
      and $native_window_secondary_mobile.screenshot_count == 5
      and (($native_window_secondary_mobile.screenshots // []) | all(.visual_probe.mobile_secondary_content_ready == true))
      and $native_window_secondary_mobile.native_app_log_error_free == true;
    def local_fixture_demo_ready:
      ($readiness.status == "ready" or $readiness_report_state == "in_progress_artifacts_ready")
      and $handoff.native_base_gap_backend_handoff_ready == true
      and $handoff.handoff_count == 12
      and $work_queue.native_base_gap_work_queue_ready == true
      and $work_queue.item_count == 12
      and $distribution.distribution_preflight_gate_ready == true
      and $distribution.public_distribution_ready == false
      and $rollup.productization_blocker_rollup_ready == true
      and $packaging.local_packaging_gate_ready == true
      and $packaging.local_unsigned_app_bundle_probe_ready == true;
    def backend_ids: ($handoff.items | map(.id) | sort);
    def backend_priority: ($handoff.items | sort_by(.priority) | map({priority,id, next_owner_lane, status, live_wiring:.acceptance_state.live_wiring}));
    def release_blockers: ($distribution.blockers // []);
    {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:"ready",
      plan_boundary_gate_ready:true,
      readiness_dir:$readiness_dir,
      readiness_report_state:$readiness_report_state,
      source_reports:{
        readiness:$readiness_path,
        backend_handoff:$handoff_path,
        work_queue:$work_queue_path,
        distribution_preflight:$distribution_preflight_path,
        productization_rollup:$productization_rollup_path,
        packaging:$packaging_path,
        native_window_main:$native_window_path,
        native_window_route:$native_window_route_path,
        native_window_secondary:$native_window_secondary_path,
        native_window_secondary_mobile:$native_window_secondary_mobile_path
      },
      claim_boundary:{
        local_fixture_demo_ready:local_fixture_demo_ready,
        r33_minimum_hard_demo_ready:(main_window_ready and route_content_ready and desktop_secondary_ready and mobile_secondary_ready),
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false,
        external_actions_allowed:false,
        backend_live_wiring_claim_allowed:false
      },
      local_readiness_claim:{
        ready:local_fixture_demo_ready,
        mode:"local_ui_fixture_packaging_contract_readiness",
        hard_true_window_gate_ready:(main_window_ready and route_content_ready and desktop_secondary_ready and mobile_secondary_ready),
        hard_true_window_gate_required_for_public_demo:true,
        hard_true_window_gate:{
          main_window_ready:main_window_ready,
          main_window_screenshot_count:screenshots($native_window),
          route_content_ready:route_content_ready,
          route_screenshot_count:screenshots($native_window_route),
          route_unique_count:($native_window_route.route_screenshot_unique_count // 0),
          desktop_secondary_ready:desktop_secondary_ready,
          desktop_secondary_screenshot_count:screenshots($native_window_secondary),
          desktop_secondary_unique_count:($native_window_secondary.surface_screenshot_unique_count // 0),
          mobile_secondary_ready:mobile_secondary_ready,
          mobile_secondary_content_probe_ready:($native_window_secondary_mobile.mobile_secondary_content_probe_ready // false),
          mobile_secondary_content_visible_count:($native_window_secondary_mobile.mobile_secondary_content_visible_count // 0),
          mobile_secondary_screenshot_count:screenshots($native_window_secondary_mobile),
          mobile_secondary_unique_count:($native_window_secondary_mobile.surface_screenshot_unique_count // 0)
        }
      },
      live_product_claim:{
        ready:false,
        blocked:true,
        blocked_by:["partial_live_backend_contract_remaining"],
        remaining_backend_contract_count:($handoff.items | length),
        remaining_backend_contract_ids:backend_ids,
        next_owner_lane:$handoff.next_owner_lane,
        ui_lane_state:$handoff.ui_lane_state,
        priority_order:backend_priority
      },
      release_claim:{
        ready:false,
        blocked:true,
        public_distribution_ready:$distribution.public_distribution_ready,
        blocked_by:release_blockers,
        release_approval_required:$distribution.release_approval_required,
        external_action_required:true,
        external_action_allowed:false,
        credential_values_read:$distribution.credential_values_read,
        network_call_performed:$distribution.network_call_performed,
        notary_submission_performed:$distribution.notary_submission_performed,
        public_distribution_artifact_written:$distribution.public_distribution_artifact_written,
        app_signed:$distribution.app_signed,
        app_notarized:$distribution.app_notarized,
        app_stapled:$distribution.app_stapled
      },
      next_plan:[
        {
          priority:1,
          id:"minimum_ui_demo_gate",
          owner_lane:"hepta-ui",
          action:"keep r33-equivalent hard readiness as the minimum local UI demo gate before public demo claims",
          required_evidence:"main 2 screenshots, route 4 unique with route content probe, desktop secondary 5 unique, mobile secondary 5 unique with content probe, blocked_allowed=false"
        },
        {
          priority:2,
          id:"backend_contract_promotion",
          owner_lane:"backend_contract",
          action:"promote remaining partial-live backend contracts in priority order",
          first_five:($handoff.items | sort_by(.priority) | map(.id) | .[0:5])
        },
        {
          priority:3,
          id:"release_artifact_gate",
          owner_lane:"release_operator",
          action:"add explicit release-approved signed/notarized/stapled artifact evidence before release or GA claims",
          blockers:release_blockers
        }
      ],
      side_effects:{
        local_loopback_server_spawned:($readiness.side_effects.local_loopback_server_spawned // $packaging.runner.local_loopback_spawned // false),
        matrix_login:false,
        gateway_call:false,
        provider_invoked:false,
        channel_delivery:false,
        external_mutation:false
      }
    }' >"$REPORT_TMP"

jq -e '
  .status == "ready"
  and .plan_boundary_gate_ready == true
  and .claim_boundary.local_fixture_demo_ready == true
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and .claim_boundary.backend_live_wiring_claim_allowed == false
  and .local_readiness_claim.ready == true
  and .local_readiness_claim.hard_true_window_gate_required_for_public_demo == true
  and .live_product_claim.ready == false
  and .live_product_claim.blocked == true
  and .live_product_claim.remaining_backend_contract_count == 12
  and .live_product_claim.next_owner_lane == "backend_contract"
  and .live_product_claim.ui_lane_state == "complete"
  and (.live_product_claim.remaining_backend_contract_ids | length) == 12
  and (.live_product_claim.remaining_backend_contract_ids | index("message_search")) != null
  and (.live_product_claim.remaining_backend_contract_ids | index("file_upload_send")) != null
  and (.live_product_claim.priority_order | length) == 12
  and (.live_product_claim.priority_order[0].id == "message_search")
  and (.live_product_claim.priority_order[1].id == "file_upload_send")
  and (.live_product_claim.priority_order[2].id == "media_download_playback")
  and (.live_product_claim.priority_order[3].id == "notifications")
  and (.live_product_claim.priority_order[4].id == "room_settings")
  and .release_claim.ready == false
  and .release_claim.blocked == true
  and .release_claim.public_distribution_ready == false
  and .release_claim.release_approval_required == true
  and .release_claim.external_action_required == true
  and .release_claim.external_action_allowed == false
  and .release_claim.credential_values_read == false
  and .release_claim.network_call_performed == false
  and .release_claim.notary_submission_performed == false
  and .release_claim.public_distribution_artifact_written == false
  and .release_claim.app_signed == false
  and .release_claim.app_notarized == false
  and .release_claim.app_stapled == false
  and (.release_claim.blocked_by | index("operator_release_approval_required")) != null
  and (.release_claim.blocked_by | index("apple_credentials_not_read")) != null
  and (.release_claim.blocked_by | index("notary_submission_not_performed")) != null
  and (.release_claim.blocked_by | index("public_distribution_artifact_not_written")) != null
  and (.next_plan | length) == 3
  and .next_plan[0].id == "minimum_ui_demo_gate"
  and .next_plan[1].id == "backend_contract_promotion"
  and .next_plan[2].id == "release_artifact_gate"
  and .side_effects.matrix_login == false
  and .side_effects.gateway_call == false
  and .side_effects.provider_invoked == false
  and .side_effects.channel_delivery == false
  and .side_effects.external_mutation == false
' "$REPORT_TMP" >/dev/null

if [[ -n "$REPORT_PATH" ]]; then
  cp "$REPORT_TMP" "$REPORT_PATH"
else
  cat "$REPORT_TMP"
fi
