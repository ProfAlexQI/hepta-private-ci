#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH:-$READINESS_DIR/ui-backend-receipt-refresh-lock-gate.json}"
LOCK_DIR="${HEPTA_UI_BACKEND_RECEIPT_REFRESH_LOCK_DIR:-$READINESS_DIR/backend-receipt-refresh-lock}"
LOCK_MARKDOWN_PATH="$LOCK_DIR/backend-receipt-refresh-lock.md"

BACKEND_RECEIPT_INTAKE_REPORT_PATH="${HEPTA_UI_BACKEND_RECEIPT_INTAKE_REPORT_PATH:-$READINESS_DIR/ui-backend-receipt-intake-gate.json}"
BACKEND_RECEIPT_ROUNDTRIP_REPORT_PATH="${HEPTA_UI_BACKEND_RECEIPT_ROUNDTRIP_REPORT_PATH:-$READINESS_DIR/ui-backend-receipt-roundtrip-gate.json}"
BACKEND_DISPATCH_PACKET_REPORT_PATH="${HEPTA_UI_BACKEND_DISPATCH_PACKET_REPORT_PATH:-$READINESS_DIR/ui-backend-dispatch-packet-gate.json}"
NATIVE_WINDOW_REPORT_PATH="$READINESS_DIR/native-window-smoke.json"
NATIVE_WINDOW_ROUTE_REPORT_PATH="$READINESS_DIR/native-window-routes-smoke.json"
NATIVE_WINDOW_SECONDARY_REPORT_PATH="$READINESS_DIR/native-window-secondary-smoke.json"
NATIVE_WINDOW_SECONDARY_MOBILE_REPORT_PATH="$READINESS_DIR/native-window-secondary-mobile-smoke.json"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf '%s is required for the Hepta UI backend receipt refresh lock gate\n' "$1" >&2
    exit 2
  fi
}

require_report() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required backend receipt refresh lock input: %s\n' "$path" >&2
    exit 1
  fi
  jq empty "$path" >/dev/null
}

file_sha256() {
  shasum -a 256 "$1" | awk '{print $1}'
}

file_bytes() {
  wc -c <"$1" | tr -d ' '
}

require_command jq
require_command shasum

require_report "$BACKEND_RECEIPT_INTAKE_REPORT_PATH"
require_report "$BACKEND_RECEIPT_ROUNDTRIP_REPORT_PATH"
require_report "$BACKEND_DISPATCH_PACKET_REPORT_PATH"
require_report "$NATIVE_WINDOW_REPORT_PATH"
require_report "$NATIVE_WINDOW_ROUTE_REPORT_PATH"
require_report "$NATIVE_WINDOW_SECONDARY_REPORT_PATH"
require_report "$NATIVE_WINDOW_SECONDARY_MOBILE_REPORT_PATH"

rm -rf "$LOCK_DIR"
mkdir -p "$LOCK_DIR"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-backend-receipt-refresh-lock.XXXXXX")"
REPORT_TMP="$TMP_DIR/backend-receipt-refresh-lock-report.json"
trap 'rm -rf "$TMP_DIR"' EXIT

intake_sha="$(file_sha256 "$BACKEND_RECEIPT_INTAKE_REPORT_PATH")"
roundtrip_sha="$(file_sha256 "$BACKEND_RECEIPT_ROUNDTRIP_REPORT_PATH")"
dispatch_sha="$(file_sha256 "$BACKEND_DISPATCH_PACKET_REPORT_PATH")"
native_window_sha="$(file_sha256 "$NATIVE_WINDOW_REPORT_PATH")"
native_window_route_sha="$(file_sha256 "$NATIVE_WINDOW_ROUTE_REPORT_PATH")"
native_window_secondary_sha="$(file_sha256 "$NATIVE_WINDOW_SECONDARY_REPORT_PATH")"
native_window_secondary_mobile_sha="$(file_sha256 "$NATIVE_WINDOW_SECONDARY_MOBILE_REPORT_PATH")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_backend_receipt_refresh_lock_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg lock_dir "$LOCK_DIR" \
  --arg lock_markdown_path "$LOCK_MARKDOWN_PATH" \
  --arg intake_report_path "$BACKEND_RECEIPT_INTAKE_REPORT_PATH" \
  --arg roundtrip_report_path "$BACKEND_RECEIPT_ROUNDTRIP_REPORT_PATH" \
  --arg dispatch_report_path "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  --arg native_window_report_path "$NATIVE_WINDOW_REPORT_PATH" \
  --arg native_window_route_report_path "$NATIVE_WINDOW_ROUTE_REPORT_PATH" \
  --arg native_window_secondary_report_path "$NATIVE_WINDOW_SECONDARY_REPORT_PATH" \
  --arg native_window_secondary_mobile_report_path "$NATIVE_WINDOW_SECONDARY_MOBILE_REPORT_PATH" \
  --arg intake_sha "$intake_sha" \
  --arg roundtrip_sha "$roundtrip_sha" \
  --arg dispatch_sha "$dispatch_sha" \
  --arg native_window_sha "$native_window_sha" \
  --arg native_window_route_sha "$native_window_route_sha" \
  --arg native_window_secondary_sha "$native_window_secondary_sha" \
  --arg native_window_secondary_mobile_sha "$native_window_secondary_mobile_sha" \
  --slurpfile intake_file "$BACKEND_RECEIPT_INTAKE_REPORT_PATH" \
  --slurpfile roundtrip_file "$BACKEND_RECEIPT_ROUNDTRIP_REPORT_PATH" \
  --slurpfile dispatch_file "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  --slurpfile native_window_file "$NATIVE_WINDOW_REPORT_PATH" \
  --slurpfile native_window_route_file "$NATIVE_WINDOW_ROUTE_REPORT_PATH" \
  --slurpfile native_window_secondary_file "$NATIVE_WINDOW_SECONDARY_REPORT_PATH" \
  --slurpfile native_window_secondary_mobile_file "$NATIVE_WINDOW_SECONDARY_MOBILE_REPORT_PATH" \
  '
  ($intake_file[0]) as $intake
  | ($roundtrip_file[0]) as $roundtrip
  | ($dispatch_file[0]) as $dispatch
  | ($native_window_file[0]) as $native_window
  | ($native_window_route_file[0]) as $native_window_route
  | ($native_window_secondary_file[0]) as $native_window_secondary
  | ($native_window_secondary_mobile_file[0]) as $native_window_secondary_mobile
  | def selected_ids: ["message_search","file_upload_send","media_download_playback","notifications","room_settings"];
    def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
    def simulated_receipt_input:
      $intake.backend_receipt_present == true
      and (
        (($intake.receipt_preview.receipt_mode // "") == "local_simulated_receipt_roundtrip_only")
        or (($intake.receipt_preview.simulated_provenance.source // "") == "hepta-ui-backend-receipt-roundtrip-gate")
      );
    def real_backend_receipt_present:
      $intake.backend_receipt_present == true
      and $intake.backend_receipt_valid == true
      and (simulated_receipt_input | not);
    def hard_true_window_refresh_ready:
      $native_window.enabled == true
      and $native_window.status == "ready"
      and (($native_window.screenshots // []) | length) == 2
      and $native_window.native_app_log_error_free == true
      and $native_window_route.enabled == true
      and $native_window_route.status == "ready"
      and $native_window_route.route_count == 4
      and $native_window_route.screenshot_count == 4
      and $native_window_route.route_screenshot_unique_count == 4
      and $native_window_route.route_content_probe_ready == true
      and $native_window_route.native_app_log_error_free == true
      and $native_window_secondary.enabled == true
      and $native_window_secondary.status == "ready"
      and $native_window_secondary.surface_count == 5
      and $native_window_secondary.screenshot_count == 5
      and $native_window_secondary.surface_screenshot_unique_count == 5
      and $native_window_secondary.native_app_log_error_free == true
      and $native_window_secondary_mobile.enabled == true
      and $native_window_secondary_mobile.status == "ready"
      and $native_window_secondary_mobile.surface_count == 5
      and $native_window_secondary_mobile.screenshot_count == 5
      and $native_window_secondary_mobile.surface_screenshot_unique_count == 5
      and $native_window_secondary_mobile.mobile_secondary_content_probe_ready == true
      and $native_window_secondary_mobile.mobile_secondary_content_visible_count >= 5
      and $native_window_secondary_mobile.native_app_log_error_free == true;
    def source_chain_ready:
      $dispatch.backend_dispatch_packet_gate_ready == true
      and $dispatch.selected_packet_ids == selected_ids
      and $intake.backend_receipt_intake_gate_ready == true
      and $intake.selected_receipt_ids == selected_ids
      and $intake.dispatch_packet_archive_sha256 == $dispatch.archive_sha256
      and $intake.dispatch_packet_archive_bytes == $dispatch.archive_bytes
      and $roundtrip.backend_receipt_roundtrip_gate_ready == true
      and $roundtrip.selected_roundtrip_ids == selected_ids
      and $roundtrip.dispatch_packet_archive_sha256 == $dispatch.archive_sha256
      and $roundtrip.source_alignment.backend_receipt_waiting_branch_ready == true
      and $roundtrip.source_alignment.backend_receipt_present_branch_ready == true
      and $roundtrip.source_alignment.simulated_receipt_ready == true
      and sha_ready($intake_sha)
      and sha_ready($roundtrip_sha)
      and sha_ready($dispatch_sha)
      and sha_ready($native_window_sha)
      and sha_ready($native_window_route_sha)
      and sha_ready($native_window_secondary_sha)
      and sha_ready($native_window_secondary_mobile_sha);
    def receipt_state_ready:
      (
        $intake.backend_receipt_present == false
        and $intake.waiting_for_backend_receipt == true
        and $intake.backend_receipt_valid == false
        and $intake.claim_boundary.backend_receipt_claim_ready == false
      )
      or (
        $intake.backend_receipt_present == true
        and $intake.waiting_for_backend_receipt == false
        and $intake.backend_receipt_valid == true
        and $intake.receipt_item_count == 5
        and $intake.receipt_ready_count == 5
      );
    def product_backend_receipt_claim_ready:
      real_backend_receipt_present and hard_true_window_refresh_ready;
    (
      source_chain_ready
      and receipt_state_ready
      and $roundtrip.claim_boundary.backend_receipt_claim_ready == false
      and $roundtrip.claim_boundary.simulated_backend_receipt_branch_ready == true
      and $intake.claim_boundary.live_product_claim_ready == false
      and $intake.claim_boundary.public_distribution_claim_ready == false
      and $intake.claim_boundary.release_claim_ready == false
      and $roundtrip.claim_boundary.live_product_claim_ready == false
      and $roundtrip.claim_boundary.public_distribution_claim_ready == false
      and $roundtrip.claim_boundary.release_claim_ready == false
    ) as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      backend_receipt_refresh_lock_gate_ready:$ready,
      refresh_lock_kind:"local_backend_receipt_refresh_and_misclaim_lock",
      refresh_lock_version:1,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      lock_dir:$lock_dir,
      lock_markdown_path:$lock_markdown_path,
      source_reports:{
        backend_receipt_intake:$intake_report_path,
        backend_receipt_roundtrip:$roundtrip_report_path,
        backend_dispatch_packet:$dispatch_report_path,
        native_window:$native_window_report_path,
        native_window_route:$native_window_route_report_path,
        native_window_secondary:$native_window_secondary_report_path,
        native_window_secondary_mobile:$native_window_secondary_mobile_report_path
      },
      source_report_sha256:{
        backend_receipt_intake:$intake_sha,
        backend_receipt_roundtrip:$roundtrip_sha,
        backend_dispatch_packet:$dispatch_sha,
        native_window:$native_window_sha,
        native_window_route:$native_window_route_sha,
        native_window_secondary:$native_window_secondary_sha,
        native_window_secondary_mobile:$native_window_secondary_mobile_sha
      },
      selected_refresh_ids:selected_ids,
      receipt_state:{
        backend_receipt_present:$intake.backend_receipt_present,
        waiting_for_backend_receipt:$intake.waiting_for_backend_receipt,
        backend_receipt_valid:$intake.backend_receipt_valid,
        simulated_receipt_input_present:simulated_receipt_input,
        real_backend_receipt_present:real_backend_receipt_present,
        receipt_mode:($intake.receipt_preview.receipt_mode // null),
        receipt_input_sha256:($intake.receipt_input_sha256 // null),
        dispatch_packet_archive_sha256:$intake.dispatch_packet_archive_sha256,
        dispatch_archive_match:($intake.dispatch_packet_archive_sha256 == $dispatch.archive_sha256),
        receipt_item_count:$intake.receipt_item_count,
        receipt_ready_count:$intake.receipt_ready_count
      },
      refresh_requirements:{
        required_ui_refresh_commands:$intake.required_ui_refresh_commands,
        no_window_refresh_command:$intake.required_ui_refresh_commands[0],
        full_hard_refresh_command:$intake.required_ui_refresh_commands[1],
        receipt_json_refresh_ready:($intake.backend_receipt_present == true and $intake.backend_receipt_valid == true),
        hard_true_window_refresh_ready:hard_true_window_refresh_ready,
        full_hard_refresh_required:(real_backend_receipt_present and (hard_true_window_refresh_ready | not)),
        full_hard_refresh_ready:(real_backend_receipt_present and hard_true_window_refresh_ready),
        hard_true_window_counts:{
          main:(($native_window.screenshots // []) | length),
          route:($native_window_route.screenshot_count // 0),
          desktop_secondary:($native_window_secondary.screenshot_count // 0),
          mobile_secondary:($native_window_secondary_mobile.screenshot_count // 0),
          route_unique:($native_window_route.route_screenshot_unique_count // 0),
          desktop_secondary_unique:($native_window_secondary.surface_screenshot_unique_count // 0),
          mobile_secondary_unique:($native_window_secondary_mobile.surface_screenshot_unique_count // 0),
          mobile_secondary_content_visible_count:($native_window_secondary_mobile.mobile_secondary_content_visible_count // 0)
        }
      },
      misclaim_lock:{
        simulated_receipt_branch_available:$roundtrip.claim_boundary.simulated_backend_receipt_branch_ready,
        simulated_receipt_not_promoted_to_backend_receipt:((simulated_receipt_input | not) or (real_backend_receipt_present == false)),
        roundtrip_backend_receipt_claim_ready:$roundtrip.claim_boundary.backend_receipt_claim_ready,
        live_claim_blocked_until_real_receipt_and_hard_refresh:true,
        public_release_claims_blocked:true
      },
      source_alignment:{
        backend_dispatch_packet_ready:$dispatch.backend_dispatch_packet_gate_ready,
        backend_receipt_intake_ready:$intake.backend_receipt_intake_gate_ready,
        backend_receipt_roundtrip_ready:$roundtrip.backend_receipt_roundtrip_gate_ready,
        selected_ids_match:($dispatch.selected_packet_ids == selected_ids and $intake.selected_receipt_ids == selected_ids and $roundtrip.selected_roundtrip_ids == selected_ids),
        dispatch_archive_match:($intake.dispatch_packet_archive_sha256 == $dispatch.archive_sha256 and $roundtrip.dispatch_packet_archive_sha256 == $dispatch.archive_sha256),
        receipt_state_ready:receipt_state_ready,
        hard_true_window_refresh_ready:hard_true_window_refresh_ready
      },
      claim_boundary:{
        local_backend_receipt_refresh_lock_ready:$ready,
        local_backend_receipt_intake_ready:$intake.claim_boundary.local_backend_receipt_intake_ready,
        local_backend_receipt_roundtrip_ready:$roundtrip.claim_boundary.local_backend_receipt_roundtrip_ready,
        simulated_backend_receipt_branch_ready:$roundtrip.claim_boundary.simulated_backend_receipt_branch_ready,
        real_backend_receipt_claim_ready:product_backend_receipt_claim_ready,
        backend_receipt_claim_ready:product_backend_receipt_claim_ready,
        backend_receipt_full_hard_refresh_ready:product_backend_receipt_claim_ready,
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
        local_markdown_written:true,
        matrix_login:false,
        gateway_call:false,
        provider_invoked:false,
        channel_delivery:false,
        live_runtime_mutation:false,
        external_mutation:false
      }
    }' >"$REPORT_TMP"

jq -r '
  "# Hepta UI Backend Receipt Refresh Lock\n\n"
  + "- Kind: \(.refresh_lock_kind)\n"
  + "- Selected ids: \(.selected_refresh_ids | join(", "))\n"
  + "- Backend receipt present: \(.receipt_state.backend_receipt_present)\n"
  + "- Real backend receipt present: \(.receipt_state.real_backend_receipt_present)\n"
  + "- Simulated receipt input present: \(.receipt_state.simulated_receipt_input_present)\n"
  + "- Hard true-window refresh ready: \(.refresh_requirements.hard_true_window_refresh_ready)\n"
  + "- Full-hard refresh required: \(.refresh_requirements.full_hard_refresh_required)\n"
  + "- Live/public/release claims remain false.\n\n"
  + "## Required UI Refresh Commands\n\n"
  + "- `\(.refresh_requirements.no_window_refresh_command)`\n"
  + "- `\(.refresh_requirements.full_hard_refresh_command)`\n"
' "$REPORT_TMP" >"$LOCK_MARKDOWN_PATH"

lock_markdown_sha="$(file_sha256 "$LOCK_MARKDOWN_PATH")"
lock_markdown_bytes="$(file_bytes "$LOCK_MARKDOWN_PATH")"

jq \
  --arg lock_markdown_sha "$lock_markdown_sha" \
  --argjson lock_markdown_bytes "$lock_markdown_bytes" \
  '. + {
    lock_markdown_sha256:$lock_markdown_sha,
    lock_markdown_bytes:$lock_markdown_bytes
  }' "$REPORT_TMP" >"$REPORT_TMP.with-markdown"
mv "$REPORT_TMP.with-markdown" "$REPORT_TMP"

jq -e '
  .status == "ready"
  and .backend_receipt_refresh_lock_gate_ready == true
  and .refresh_lock_kind == "local_backend_receipt_refresh_and_misclaim_lock"
  and .refresh_lock_version == 1
  and .selected_refresh_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and (.source_report_sha256.backend_receipt_intake | test("^[0-9a-f]{64}$"))
  and (.source_report_sha256.backend_receipt_roundtrip | test("^[0-9a-f]{64}$"))
  and (.source_report_sha256.backend_dispatch_packet | test("^[0-9a-f]{64}$"))
  and (.lock_markdown_sha256 | test("^[0-9a-f]{64}$"))
  and .lock_markdown_bytes > 0
  and .receipt_state.dispatch_archive_match == true
  and .source_alignment.backend_dispatch_packet_ready == true
  and .source_alignment.backend_receipt_intake_ready == true
  and .source_alignment.backend_receipt_roundtrip_ready == true
  and .source_alignment.selected_ids_match == true
  and .source_alignment.dispatch_archive_match == true
  and .source_alignment.receipt_state_ready == true
  and .misclaim_lock.simulated_receipt_branch_available == true
  and .misclaim_lock.roundtrip_backend_receipt_claim_ready == false
  and .misclaim_lock.live_claim_blocked_until_real_receipt_and_hard_refresh == true
  and .claim_boundary.local_backend_receipt_refresh_lock_ready == true
  and .claim_boundary.local_backend_receipt_intake_ready == true
  and .claim_boundary.local_backend_receipt_roundtrip_ready == true
  and .claim_boundary.simulated_backend_receipt_branch_ready == true
  and .claim_boundary.live_runtime_mutation == false
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and .side_effects.local_markdown_written == true
  and .side_effects.live_runtime_mutation == false
  and .side_effects.external_mutation == false
' "$REPORT_TMP" >/dev/null

mkdir -p "$(dirname "$REPORT_PATH")"
cp "$REPORT_TMP" "$REPORT_PATH"
cat "$REPORT_TMP"
