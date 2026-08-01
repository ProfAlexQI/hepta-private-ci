#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_DEMO_EVIDENCE_REPORT_PATH:-$READINESS_DIR/ui-demo-evidence-gate.json}"
MIN_SCREENSHOT_BYTES="${HEPTA_UI_DEMO_EVIDENCE_MIN_SCREENSHOT_BYTES:-10000}"

MANIFEST_PATH="$READINESS_DIR/screenshot-manifest.json"
STATIC_CONTRACT_PATH="$READINESS_DIR/static-contract.json"
DESIGN_SYSTEM_PATH="$READINESS_DIR/ui-design-system-gate.json"
CONTROL_BROWSER_PATH="$READINESS_DIR/control-ui-browser-smoke.json"
CONTROL_REAL_CLICK_V7_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v7-real-click-gate.json"
NATIVE_FIXTURE_PATH="$READINESS_DIR/native-fixture/native-fixture-visual-smoke.json"
PACKAGING_PATH="$READINESS_DIR/native-packaging-gate.json"
DISTRIBUTION_PREFLIGHT_PATH="$READINESS_DIR/native-distribution-preflight-gate.json"
RELEASE_SIGNING_CAPABILITY_PATH="$READINESS_DIR/ui-release-signing-capability-gate.json"
BASE_GAP_DRILLDOWN_PATH="$READINESS_DIR/native-base-gap-drilldown.json"
BASE_GAP_WORK_QUEUE_PATH="$READINESS_DIR/native-base-gap-work-queue.json"
BASE_GAP_BACKEND_HANDOFF_PATH="$READINESS_DIR/native-base-gap-backend-handoff.json"
BACKEND_CONTRACT_GATES_PATH="$READINESS_DIR/native-backend-contract-gates.json"
NON_BASE_EDGE_GATES_PATH="$READINESS_DIR/native-non-base-edge-gates.json"
PRODUCTIZATION_ROLLUP_PATH="$READINESS_DIR/native-productization-blocker-rollup.json"
PLAN_BOUNDARY_PATH="$READINESS_DIR/ui-plan-boundary-gate.json"
NATIVE_WINDOW_PATH="$READINESS_DIR/native-window-smoke.json"
NATIVE_WINDOW_ROUTE_PATH="$READINESS_DIR/native-window-routes-smoke.json"
NATIVE_WINDOW_ROUTE_MOBILE_PATH="$READINESS_DIR/native-window-routes-mobile-smoke.json"
NATIVE_WINDOW_SECONDARY_PATH="$READINESS_DIR/native-window-secondary-smoke.json"
NATIVE_WINDOW_SECONDARY_MOBILE_PATH="$READINESS_DIR/native-window-secondary-mobile-smoke.json"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf '%s is required for the Hepta UI demo evidence gate\n' "$1" >&2
    exit 2
  fi
}

require_file() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required demo-evidence input: %s\n' "$path" >&2
    exit 1
  fi
}

require_command jq
require_command shasum
require_command sips

require_file "$MANIFEST_PATH"
require_file "$STATIC_CONTRACT_PATH"
require_file "$DESIGN_SYSTEM_PATH"
require_file "$CONTROL_BROWSER_PATH"
require_file "$CONTROL_REAL_CLICK_V7_PATH"
require_file "$NATIVE_FIXTURE_PATH"
require_file "$PACKAGING_PATH"
require_file "$DISTRIBUTION_PREFLIGHT_PATH"
require_file "$RELEASE_SIGNING_CAPABILITY_PATH"
require_file "$BASE_GAP_DRILLDOWN_PATH"
require_file "$BASE_GAP_WORK_QUEUE_PATH"
require_file "$BASE_GAP_BACKEND_HANDOFF_PATH"
require_file "$BACKEND_CONTRACT_GATES_PATH"
require_file "$NON_BASE_EDGE_GATES_PATH"
require_file "$PRODUCTIZATION_ROLLUP_PATH"
require_file "$PLAN_BOUNDARY_PATH"

jq -e '
  .status == "ready"
  and .real_click_ready == true
  and .summary.control_real_click_activation.viewport_count == 4
  and .summary.control_real_click_activation.target_count == 26
  and .summary.control_real_click_activation.failure_count == 0
  and .summary.control_real_click_activation.mobile_routes_ready == true
  and .summary.control_real_click_activation.popover_switch_sequence_ready == true
  and .summary.control_real_click_activation.popover_switch_step_count == 26
  and (.control_real_click_activation.viewports | all(
    .ready == true
    and .mobile_pane_routes.ready == true
    and .popover_switch_sequence.ready == true
    and (.targets | all(
      .light_dismiss.ready == true
      and .escape_close.ready == true
      and .escape_close.focus_returned_to_trigger == true
    ))
  ))
' "$CONTROL_REAL_CLICK_V7_PATH" >/dev/null

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-demo-evidence.XXXXXX")"
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

file_sha256() {
  shasum -a 256 "$1" | awk '{print $1}'
}

image_dimensions() {
  sips -g pixelWidth -g pixelHeight "$1" 2>/dev/null |
    awk '/pixelWidth/{w=$2}/pixelHeight/{h=$2} END{if (w && h) print w "x" h}'
}

REPORT_ITEMS_NDJSON="$TMP_DIR/report-items.ndjson"
SCREENSHOT_ITEMS_NDJSON="$TMP_DIR/screenshot-items.ndjson"
REPORT_ITEMS_JSON="$TMP_DIR/report-items.json"
SCREENSHOT_ITEMS_JSON="$TMP_DIR/screenshot-items.json"
SCREENSHOT_CANDIDATES="$TMP_DIR/screenshot-candidates.ndjson"
REPORT_TMP="$TMP_DIR/demo-evidence-report.json"
: >"$REPORT_ITEMS_NDJSON"
: >"$SCREENSHOT_ITEMS_NDJSON"

report_file_json() {
  local group="$1"
  local name="$2"
  local path="$3"
  local required="$4"
  local present=false
  local bytes=0
  local sha=""
  local json_valid=false

  if [[ -s "$path" ]]; then
    present=true
    bytes="$(wc -c <"$path" | tr -d ' ')"
    sha="$(file_sha256 "$path")"
    if jq empty "$path" >/dev/null 2>&1; then
      json_valid=true
    fi
  fi

  jq -n \
    --arg group "$group" \
    --arg name "$name" \
    --arg path "$path" \
    --arg sha "$sha" \
    --argjson required "$required" \
    --argjson present "$present" \
    --argjson bytes "$bytes" \
    --argjson json_valid "$json_valid" \
    '{
      group:$group,
      name:$name,
      path:$path,
      required:$required,
      present:$present,
      bytes:$bytes,
      sha256:$sha,
      json_valid:$json_valid,
      ready:(($required | not) or ($present and $bytes > 0 and ($sha | test("^[0-9a-f]{64}$")) and $json_valid))
    }'
}

screenshot_file_json() {
  local group="$1"
  local name="$2"
  local path="$3"
  local expected_sha="$4"
  local expected_bytes="$5"
  local expected_dimensions="$6"
  local required="$7"
  local present=false
  local bytes=0
  local sha=""
  local dimensions=""

  if [[ -s "$path" ]]; then
    present=true
    bytes="$(wc -c <"$path" | tr -d ' ')"
    sha="$(file_sha256 "$path")"
    dimensions="$(image_dimensions "$path")"
  fi

  jq -n \
    --arg group "$group" \
    --arg name "$name" \
    --arg path "$path" \
    --arg expected_sha "$expected_sha" \
    --arg sha "$sha" \
    --arg expected_dimensions "$expected_dimensions" \
    --arg dimensions "$dimensions" \
    --argjson expected_bytes "${expected_bytes:-0}" \
    --argjson min_bytes "$MIN_SCREENSHOT_BYTES" \
    --argjson required "$required" \
    --argjson present "$present" \
    --argjson bytes "$bytes" \
    '{
      group:$group,
      name:$name,
      path:$path,
      required:$required,
      present:$present,
      bytes:$bytes,
      expected_bytes:$expected_bytes,
      min_bytes:$min_bytes,
      byte_floor_ready:($bytes >= $min_bytes and ($expected_bytes == 0 or $bytes == $expected_bytes)),
      sha256:$sha,
      expected_sha256:$expected_sha,
      sha256_match:(($expected_sha | length) == 0 or $sha == $expected_sha),
      dimensions:$dimensions,
      expected_dimensions:$expected_dimensions,
      dimensions_readable:($dimensions | test("^[0-9]+x[0-9]+$")),
      dimensions_match:(($expected_dimensions | length) == 0 or $dimensions == $expected_dimensions),
      ready:(
        ($required | not)
        or (
          $present
          and $bytes >= $min_bytes
          and (($expected_bytes == 0) or ($bytes == $expected_bytes))
          and (($sha | test("^[0-9a-f]{64}$")) and (($expected_sha | length) == 0 or $sha == $expected_sha))
          and ($dimensions | test("^[0-9]+x[0-9]+$"))
          and (($expected_dimensions | length) == 0 or $dimensions == $expected_dimensions)
        )
      )
    }'
}

hard_true_window_required="$(jq -r '.claim_boundary.r33_minimum_hard_demo_ready == true' "$PLAN_BOUNDARY_PATH")"
if [[ "$hard_true_window_required" == "true" ]]; then
  hard_required_json=true
else
  hard_required_json=false
fi

for spec in \
  "core|screenshot_manifest|$MANIFEST_PATH|true" \
  "core|static_contract|$STATIC_CONTRACT_PATH|true" \
  "core|ui_design_system|$DESIGN_SYSTEM_PATH|true" \
  "control|browser_smoke|$CONTROL_BROWSER_PATH|true" \
  "control|native_popover_real_click_v7|$CONTROL_REAL_CLICK_V7_PATH|true" \
  "native|fixture_visual_smoke|$NATIVE_FIXTURE_PATH|true" \
  "native|packaging|$PACKAGING_PATH|true" \
  "native|distribution_preflight|$DISTRIBUTION_PREFLIGHT_PATH|true" \
  "release|signing_capability|$RELEASE_SIGNING_CAPABILITY_PATH|true" \
  "native|base_gap_drilldown|$BASE_GAP_DRILLDOWN_PATH|true" \
  "native|base_gap_work_queue|$BASE_GAP_WORK_QUEUE_PATH|true" \
  "native|base_gap_backend_handoff|$BASE_GAP_BACKEND_HANDOFF_PATH|true" \
  "native|backend_contract_gates|$BACKEND_CONTRACT_GATES_PATH|true" \
  "native|non_base_edge_gates|$NON_BASE_EDGE_GATES_PATH|true" \
  "native|productization_blocker_rollup|$PRODUCTIZATION_ROLLUP_PATH|true" \
  "ui|plan_boundary|$PLAN_BOUNDARY_PATH|true" \
  "native_true_window|main|$NATIVE_WINDOW_PATH|$hard_required_json" \
  "native_true_window|route|$NATIVE_WINDOW_ROUTE_PATH|$hard_required_json" \
  "native_true_window|route_mobile|$NATIVE_WINDOW_ROUTE_MOBILE_PATH|$hard_required_json" \
  "native_true_window|secondary_desktop|$NATIVE_WINDOW_SECONDARY_PATH|$hard_required_json" \
  "native_true_window|secondary_mobile|$NATIVE_WINDOW_SECONDARY_MOBILE_PATH|$hard_required_json"; do
  IFS='|' read -r group name path required <<<"$spec"
  report_file_json "$group" "$name" "$path" "$required" >>"$REPORT_ITEMS_NDJSON"
done

NATIVE_WINDOW_INPUT_PATH="$(json_input_path_or_default "$NATIVE_WINDOW_PATH" '{"screenshots":[]}' "native-window-default.json")"
NATIVE_WINDOW_ROUTE_INPUT_PATH="$(json_input_path_or_default "$NATIVE_WINDOW_ROUTE_PATH" '{"screenshots":[]}' "native-window-route-default.json")"
NATIVE_WINDOW_ROUTE_MOBILE_INPUT_PATH="$(json_input_path_or_default "$NATIVE_WINDOW_ROUTE_MOBILE_PATH" '{"screenshots":[]}' "native-window-route-mobile-default.json")"
NATIVE_WINDOW_SECONDARY_INPUT_PATH="$(json_input_path_or_default "$NATIVE_WINDOW_SECONDARY_PATH" '{"screenshots":[]}' "native-window-secondary-default.json")"
NATIVE_WINDOW_SECONDARY_MOBILE_INPUT_PATH="$(json_input_path_or_default "$NATIVE_WINDOW_SECONDARY_MOBILE_PATH" '{"screenshots":[]}' "native-window-secondary-mobile-default.json")"

jq -c \
  --argjson hard_required "$hard_required_json" \
  --slurpfile manifest "$MANIFEST_PATH" \
  --slurpfile native_window "$NATIVE_WINDOW_INPUT_PATH" \
  --slurpfile route "$NATIVE_WINDOW_ROUTE_INPUT_PATH" \
  --slurpfile route_mobile "$NATIVE_WINDOW_ROUTE_MOBILE_INPUT_PATH" \
  --slurpfile secondary "$NATIVE_WINDOW_SECONDARY_INPUT_PATH" \
  --slurpfile mobile "$NATIVE_WINDOW_SECONDARY_MOBILE_INPUT_PATH" \
  -n '
    def shot($group; $required; $s):
      {
        group:$group,
        name:($s.name // ""),
        path:($s.path // ""),
        expected_sha256:($s.sha256 // ""),
        expected_bytes:($s.bytes // 0),
        expected_dimensions:($s.dimensions // ""),
        required:$required
      };
    (
      [($manifest[0].key_screenshots // [])[] as $s | shot("key_screenshots"; true; $s)]
      + [($native_window[0].screenshots // [])[] as $s | shot("native_true_window_main"; $hard_required; $s)]
      + [($route[0].screenshots // [])[] as $s | shot("native_true_window_route"; $hard_required; $s)]
      + [($route_mobile[0].screenshots // [])[] as $s | shot("native_true_window_route_mobile"; $hard_required; $s)]
      + [($secondary[0].screenshots // [])[] as $s | shot("native_true_window_secondary_desktop"; $hard_required; $s)]
      + [($mobile[0].screenshots // [])[] as $s | shot("native_true_window_secondary_mobile"; $hard_required; $s)]
    )[]
  ' >"$SCREENSHOT_CANDIDATES"

while IFS= read -r candidate; do
  [[ -n "$candidate" ]] || continue
  group="$(jq -r '.group' <<<"$candidate")"
  name="$(jq -r '.name' <<<"$candidate")"
  path="$(jq -r '.path' <<<"$candidate")"
  expected_sha="$(jq -r '.expected_sha256' <<<"$candidate")"
  expected_bytes="$(jq -r '.expected_bytes' <<<"$candidate")"
  expected_dimensions="$(jq -r '.expected_dimensions' <<<"$candidate")"
  required="$(jq -r '.required' <<<"$candidate")"
  screenshot_file_json "$group" "$name" "$path" "$expected_sha" "$expected_bytes" "$expected_dimensions" "$required" >>"$SCREENSHOT_ITEMS_NDJSON"
done <"$SCREENSHOT_CANDIDATES"

jq -s '.' "$REPORT_ITEMS_NDJSON" >"$REPORT_ITEMS_JSON"
jq -s '.' "$SCREENSHOT_ITEMS_NDJSON" >"$SCREENSHOT_ITEMS_JSON"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_demo_evidence_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "${REPORT_PATH:-}" \
  --arg screenshot_manifest_path "$MANIFEST_PATH" \
  --arg plan_boundary_path "$PLAN_BOUNDARY_PATH" \
  --argjson min_screenshot_bytes "$MIN_SCREENSHOT_BYTES" \
  --slurpfile reports "$REPORT_ITEMS_JSON" \
  --slurpfile screenshots "$SCREENSHOT_ITEMS_JSON" \
  --slurpfile manifest "$MANIFEST_PATH" \
  --slurpfile design_system "$DESIGN_SYSTEM_PATH" \
  --slurpfile plan "$PLAN_BOUNDARY_PATH" \
  --slurpfile route_mobile_report "$NATIVE_WINDOW_ROUTE_MOBILE_INPUT_PATH" \
  --slurpfile packaging "$PACKAGING_PATH" \
  --slurpfile distribution "$DISTRIBUTION_PREFLIGHT_PATH" \
  '
    ($reports[0]) as $reports
    | ($screenshots[0]) as $screenshots
    | ($manifest[0]) as $manifest
    | ($design_system[0]) as $design_system
    | ($plan[0]) as $plan
    | ($route_mobile_report[0]) as $route_mobile_report
    | ($packaging[0]) as $packaging
    | ($distribution[0]) as $distribution
    | def required_reports: $reports | map(select(.required == true));
    def required_screenshots: $screenshots | map(select(.required == true));
    def required_group_count($group): (required_screenshots | map(select(.group == $group and .present == true)) | length);
    def group_count($group): ($screenshots | map(select(.group == $group and .present == true)) | length);
    def required_screenshots_ready: (required_screenshots | all(.ready == true));
    def report_evidence_ready: (required_reports | all(.ready == true));
    def key_screenshot_ready:
      required_group_count("key_screenshots") >= 24
      and ($manifest.key_screenshot_count // 0) >= 24
      and required_screenshots_ready;
    def hard_true_window_evidence_ready:
      ($plan.claim_boundary.r33_minimum_hard_demo_ready == true)
      and required_group_count("native_true_window_main") == 2
      and required_group_count("native_true_window_route") == 4
      and required_group_count("native_true_window_route_mobile") == 4
      and $route_mobile_report.status == "ready"
      and $route_mobile_report.native_makepad_mobile_route_variants_ready == true
      and $route_mobile_report.mobile_host_window_ready == true
      and required_group_count("native_true_window_secondary_desktop") == 5
      and required_group_count("native_true_window_secondary_mobile") == 5
      and required_screenshots_ready;
    def local_fixture_demo_evidence_ready:
      report_evidence_ready
      and key_screenshot_ready
      and $design_system.status == "ready"
      and $design_system.generated_token_sync_ready == true
      and $design_system.documentation_token_sync_ready == true
      and $design_system.native.fixture_generated_tokens_consumed == true
      and $plan.claim_boundary.local_fixture_demo_ready == true
      and $packaging.local_packaging_gate_ready == true
      and $packaging.local_unsigned_app_bundle_probe_ready == true
      and $distribution.distribution_preflight_gate_ready == true
      and $distribution.public_distribution_ready == false
      and $plan.claim_boundary.live_product_claim_ready == false
      and $plan.claim_boundary.public_distribution_claim_ready == false
      and $plan.claim_boundary.release_claim_ready == false;
    {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:"ready",
      demo_evidence_gate_ready:(
        local_fixture_demo_evidence_ready
        and (($plan.claim_boundary.r33_minimum_hard_demo_ready != true) or hard_true_window_evidence_ready)
      ),
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      screenshot_manifest_path:$screenshot_manifest_path,
      plan_boundary_path:$plan_boundary_path,
      min_screenshot_bytes:$min_screenshot_bytes,
      report_evidence:{
        required_report_count:(required_reports | length),
        observed_report_count:($reports | map(select(.present == true)) | length),
        all_required_reports_present:(required_reports | all(.present == true)),
        all_required_json_reports_valid:(required_reports | all(.json_valid == true)),
        all_required_reports_sha256_ready:(required_reports | all(.sha256 | test("^[0-9a-f]{64}$"))),
        items:$reports
      },
      screenshot_evidence:{
        required_screenshot_count:(required_screenshots | length),
        observed_screenshot_count:($screenshots | map(select(.present == true)) | length),
        key_screenshot_count:group_count("key_screenshots"),
        native_true_window_main_count:group_count("native_true_window_main"),
        native_true_window_route_count:group_count("native_true_window_route"),
        native_true_window_route_mobile_count:group_count("native_true_window_route_mobile"),
        native_true_window_secondary_desktop_count:group_count("native_true_window_secondary_desktop"),
        native_true_window_secondary_mobile_count:group_count("native_true_window_secondary_mobile"),
        all_required_screenshots_present:(required_screenshots | all(.present == true)),
        all_required_screenshots_byte_floor_ready:(required_screenshots | all(.byte_floor_ready == true)),
        all_required_screenshots_sha256_match:(required_screenshots | all(.sha256_match == true)),
        all_required_screenshots_dimensions_readable:(required_screenshots | all(.dimensions_readable == true)),
        all_required_screenshots_dimensions_match:(required_screenshots | all(.dimensions_match == true)),
        items:$screenshots
      },
      claim_boundary:{
        local_fixture_demo_evidence_ready:local_fixture_demo_evidence_ready,
        hard_true_window_required:($plan.claim_boundary.r33_minimum_hard_demo_ready == true),
        r33_hard_demo_evidence_ready:hard_true_window_evidence_ready,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false,
        external_actions_allowed:false,
        backend_live_wiring_claim_allowed:false
      },
      side_effects:{
        matrix_login:false,
        gateway_call:false,
        provider_invoked:false,
        channel_delivery:false,
        external_mutation:false
      }
    }
  ' >"$REPORT_TMP"

jq -e '
  .status == "ready"
  and .demo_evidence_gate_ready == true
  and .report_evidence.required_report_count >= 13
  and .report_evidence.all_required_reports_present == true
  and .report_evidence.all_required_json_reports_valid == true
  and .report_evidence.all_required_reports_sha256_ready == true
  and .screenshot_evidence.key_screenshot_count >= 24
  and .screenshot_evidence.all_required_screenshots_present == true
  and .screenshot_evidence.all_required_screenshots_byte_floor_ready == true
  and .screenshot_evidence.all_required_screenshots_sha256_match == true
  and .screenshot_evidence.all_required_screenshots_dimensions_readable == true
  and .screenshot_evidence.all_required_screenshots_dimensions_match == true
  and .claim_boundary.local_fixture_demo_evidence_ready == true
  and (
    (.claim_boundary.hard_true_window_required != true)
    or (
      .claim_boundary.r33_hard_demo_evidence_ready == true
      and .screenshot_evidence.native_true_window_main_count == 2
      and .screenshot_evidence.native_true_window_route_count == 4
      and .screenshot_evidence.native_true_window_route_mobile_count == 4
      and .screenshot_evidence.native_true_window_secondary_desktop_count == 5
      and .screenshot_evidence.native_true_window_secondary_mobile_count == 5
    )
  )
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
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
