#!/bin/bash -p
set +x
PS4='+ '
set -euo pipefail
unset BASH_ENV ENV CDPATH GLOBIGNORE RUBYOPT RUBYLIB GEM_HOME GEM_PATH BUNDLE_GEMFILE BUNDLE_PATH
SYSTEM_PATH="/usr/bin:/bin:/usr/sbin:/sbin"
PATH="$SYSTEM_PATH"
export PATH

cd "$(/usr/bin/dirname "$0")/.."
REPO_ROOT="$(pwd -P)"
. "$REPO_ROOT/scripts/lib/hepta-safe-managed-output-v1.sh"

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_CURRENT_PLAN_REFRESH_REPORT_PATH:-$READINESS_DIR/ui-current-plan-refresh-gate.json}"
REFRESH_DIR="${HEPTA_UI_CURRENT_PLAN_REFRESH_DIR:-$READINESS_DIR/current-plan-refresh}"
READINESS_DIR="$(hepta_safe_normalize_path readiness "$READINESS_DIR")"
REPORT_PATH="$(hepta_safe_normalize_path report "$REPORT_PATH")"
REFRESH_DIR="$(hepta_safe_normalize_path refresh "$REFRESH_DIR")"
REFRESH_MARKDOWN_PATH="$REFRESH_DIR/current-plan-refresh.md"
REPORT_PARENT="$(hepta_safe_normalize_path report_parent "$(/usr/bin/dirname "$REPORT_PATH")")"
hepta_safe_require_directory_target readiness "$READINESS_DIR"
hepta_safe_require_directory_target refresh "$REFRESH_DIR"
hepta_safe_require_directory_target report_parent "$REPORT_PARENT"
hepta_safe_require_regular_target report "$REPORT_PATH"
hepta_safe_require_regular_target refresh_markdown "$REFRESH_MARKDOWN_PATH"
if hepta_safe_paths_overlap "$READINESS_DIR" "$REPO_ROOT"; then
  printf 'current-plan readiness must not overlap the repository\n' >&2
  exit 64
fi
if ! hepta_safe_is_strict_descendant "$REFRESH_DIR" "$READINESS_DIR"; then
  printf 'current-plan refresh directory must be a strict readiness child\n' >&2
  exit 64
fi
if [[ "$REPORT_PARENT" != "$READINESS_DIR" ]] \
  && ! hepta_safe_is_strict_descendant "$REPORT_PARENT" "$READINESS_DIR"; then
  printf 'current-plan report parent must remain inside readiness\n' >&2
  exit 64
fi
if hepta_safe_paths_overlap "$REPORT_PATH" "$REFRESH_DIR"; then
  printf 'current-plan report and refresh directory must be disjoint\n' >&2
  exit 64
fi

FUTURE_PLAN_REFRESH_REPORT_PATH="$READINESS_DIR/ui-future-plan-refresh-gate.json"
OPERATOR_BRIEFING_REFRESH_REPORT_PATH="$READINESS_DIR/ui-operator-briefing-refresh-gate.json"
RELEASE_APPROVAL_INTAKE_REPORT_PATH="$READINESS_DIR/ui-release-approval-intake-gate.json"
TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH="$READINESS_DIR/ui-top-design-referee-refresh-gate.json"
RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH="$READINESS_DIR/ui-release-artifact-boundary-gate.json"
RELEASE_ARTIFACT_INTAKE_REPORT_PATH="$READINESS_DIR/ui-release-artifact-intake-gate.json"
RELEASE_ARTIFACT_ROUNDTRIP_REPORT_PATH="$READINESS_DIR/ui-release-artifact-roundtrip-gate.json"
BACKEND_DISPATCH_PACKET_REPORT_PATH="$READINESS_DIR/ui-backend-dispatch-packet-gate.json"
BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH="$READINESS_DIR/ui-backend-receipt-refresh-lock-gate.json"
EVIDENCE_ARCHIVE_REPORT_PATH="$READINESS_DIR/ui-evidence-archive-gate.json"
SCREENSHOT_MANIFEST_PATH="$READINESS_DIR/screenshot-manifest.json"
for protected_input in \
  "$FUTURE_PLAN_REFRESH_REPORT_PATH" "$OPERATOR_BRIEFING_REFRESH_REPORT_PATH" \
  "$RELEASE_APPROVAL_INTAKE_REPORT_PATH" "$TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH" \
  "$RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH" "$RELEASE_ARTIFACT_INTAKE_REPORT_PATH" \
  "$RELEASE_ARTIFACT_ROUNDTRIP_REPORT_PATH" "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH" "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  "$SCREENSHOT_MANIFEST_PATH"; do
  if hepta_safe_paths_overlap "$protected_input" "$REFRESH_DIR" \
    || hepta_safe_paths_overlap "$protected_input" "$REPORT_PATH"; then
    printf 'current-plan output overlaps protected input: %s\n' "$protected_input" >&2
    exit 64
  fi
done

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf '%s is required for the Hepta UI current-plan refresh gate\n' "$1" >&2
    exit 2
  fi
}

require_report() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required current-plan refresh input: %s\n' "$path" >&2
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

require_report "$FUTURE_PLAN_REFRESH_REPORT_PATH"
require_report "$OPERATOR_BRIEFING_REFRESH_REPORT_PATH"
require_report "$RELEASE_APPROVAL_INTAKE_REPORT_PATH"
require_report "$TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH"
require_report "$RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH"
require_report "$RELEASE_ARTIFACT_INTAKE_REPORT_PATH"
require_report "$RELEASE_ARTIFACT_ROUNDTRIP_REPORT_PATH"
require_report "$BACKEND_DISPATCH_PACKET_REPORT_PATH"
require_report "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH"
require_report "$EVIDENCE_ARCHIVE_REPORT_PATH"
require_report "$SCREENSHOT_MANIFEST_PATH"

mkdir -p "$REFRESH_DIR" "$REPORT_PARENT"
hepta_safe_revalidate_directory refresh "$REFRESH_DIR"
hepta_safe_revalidate_directory report_parent "$REPORT_PARENT"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-current-plan-refresh.XXXXXX")"
REPORT_DRAFT="$TMP_DIR/current-plan-refresh-draft.json"
REPORT_TMP="$TMP_DIR/current-plan-refresh-report.json"
MARKDOWN_TMP="$TMP_DIR/current-plan-refresh.md"
trap 'rm -rf "$TMP_DIR"' EXIT

future_plan_sha="$(file_sha256 "$FUTURE_PLAN_REFRESH_REPORT_PATH")"
operator_refresh_sha="$(file_sha256 "$OPERATOR_BRIEFING_REFRESH_REPORT_PATH")"
release_approval_sha="$(file_sha256 "$RELEASE_APPROVAL_INTAKE_REPORT_PATH")"
top_design_sha="$(file_sha256 "$TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH")"
release_artifact_sha="$(file_sha256 "$RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH")"
release_artifact_intake_sha="$(file_sha256 "$RELEASE_ARTIFACT_INTAKE_REPORT_PATH")"
release_artifact_roundtrip_sha="$(file_sha256 "$RELEASE_ARTIFACT_ROUNDTRIP_REPORT_PATH")"
backend_dispatch_sha="$(file_sha256 "$BACKEND_DISPATCH_PACKET_REPORT_PATH")"
receipt_refresh_sha="$(file_sha256 "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH")"
evidence_archive_sha="$(file_sha256 "$EVIDENCE_ARCHIVE_REPORT_PATH")"
screenshot_manifest_sha="$(file_sha256 "$SCREENSHOT_MANIFEST_PATH")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_current_plan_refresh_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg refresh_dir "$REFRESH_DIR" \
  --arg refresh_markdown_path "$REFRESH_MARKDOWN_PATH" \
  --arg future_plan_path "$FUTURE_PLAN_REFRESH_REPORT_PATH" \
  --arg operator_refresh_path "$OPERATOR_BRIEFING_REFRESH_REPORT_PATH" \
  --arg release_approval_path "$RELEASE_APPROVAL_INTAKE_REPORT_PATH" \
  --arg top_design_path "$TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH" \
  --arg release_artifact_path "$RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH" \
  --arg release_artifact_intake_path "$RELEASE_ARTIFACT_INTAKE_REPORT_PATH" \
  --arg release_artifact_roundtrip_path "$RELEASE_ARTIFACT_ROUNDTRIP_REPORT_PATH" \
  --arg backend_dispatch_path "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  --arg receipt_refresh_path "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH" \
  --arg evidence_archive_path "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  --arg screenshot_manifest_path "$SCREENSHOT_MANIFEST_PATH" \
  --arg future_plan_sha "$future_plan_sha" \
  --arg operator_refresh_sha "$operator_refresh_sha" \
  --arg release_approval_sha "$release_approval_sha" \
  --arg top_design_sha "$top_design_sha" \
  --arg release_artifact_sha "$release_artifact_sha" \
  --arg release_artifact_intake_sha "$release_artifact_intake_sha" \
  --arg release_artifact_roundtrip_sha "$release_artifact_roundtrip_sha" \
  --arg backend_dispatch_sha "$backend_dispatch_sha" \
  --arg receipt_refresh_sha "$receipt_refresh_sha" \
  --arg evidence_archive_sha "$evidence_archive_sha" \
  --arg screenshot_manifest_sha "$screenshot_manifest_sha" \
  --slurpfile future_plan_file "$FUTURE_PLAN_REFRESH_REPORT_PATH" \
  --slurpfile operator_refresh_file "$OPERATOR_BRIEFING_REFRESH_REPORT_PATH" \
  --slurpfile release_approval_file "$RELEASE_APPROVAL_INTAKE_REPORT_PATH" \
  --slurpfile top_design_file "$TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH" \
  --slurpfile release_artifact_file "$RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH" \
  --slurpfile release_artifact_intake_file "$RELEASE_ARTIFACT_INTAKE_REPORT_PATH" \
  --slurpfile release_artifact_roundtrip_file "$RELEASE_ARTIFACT_ROUNDTRIP_REPORT_PATH" \
  --slurpfile backend_dispatch_file "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  --slurpfile receipt_refresh_file "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH" \
  --slurpfile evidence_archive_file "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  --slurpfile screenshot_manifest_file "$SCREENSHOT_MANIFEST_PATH" \
  '
  ($future_plan_file[0]) as $future
  | ($operator_refresh_file[0]) as $operator_refresh
  | ($release_approval_file[0]) as $release_approval
  | ($top_design_file[0]) as $top_design
  | ($release_artifact_file[0]) as $release_artifact
  | ($release_artifact_intake_file[0]) as $release_artifact_intake
  | ($release_artifact_roundtrip_file[0]) as $release_artifact_roundtrip
  | ($backend_dispatch_file[0]) as $dispatch
  | ($receipt_refresh_file[0]) as $receipt_refresh
  | ($evidence_archive_file[0]) as $archive
  | ($screenshot_manifest_file[0]) as $manifest
  | def selected_ids: ["message_search","file_upload_send","media_download_playback","notifications","room_settings"];
	    def old_plan_ids: ["r52_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt"];
	    def current_plan_ids: ["r62_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt","release_artifact_roundtrip_and_signed_artifact_gate"];
	    def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
	    def top_design_hard_ready:
	      $top_design.true_window_evidence_mode == "full_hard_true_window"
	      and $top_design.hard_true_window_evidence_ready == true
	      and $top_design.referee_matrix.true_window_routes.hard_ready == true
	      and $top_design.referee_matrix.true_window_routes.content_probe_ready == true
	      and $top_design.referee_matrix.true_window_secondary_desktop.hard_ready == true
	      and $top_design.referee_matrix.true_window_secondary_mobile.hard_ready == true
	      and $top_design.referee_matrix.true_window_secondary_mobile.content_probe_ready == true
	      and $top_design.referee_matrix.true_window_secondary_mobile.content_visible_count >= 10
	      and $top_design.screenshot_manifest.hard_ready == true
	      and $manifest.screenshot_count.native_true_window == 2
	      and $manifest.screenshot_count.native_true_window_route == 4
	      and $manifest.screenshot_count.native_true_window_secondary == 5
	      and $manifest.screenshot_count.native_true_window_secondary_mobile == 5
	      and $manifest.screenshot_count.total >= 60;
	    def top_design_no_window_ready:
	      $top_design.true_window_evidence_mode == "no_window_fixture"
	      and $top_design.no_window_evidence_accepted == true
	      and $top_design.referee_matrix.true_window_routes.no_window_accepted == true
	      and $top_design.referee_matrix.true_window_secondary_desktop.no_window_accepted == true
	      and $top_design.referee_matrix.true_window_secondary_mobile.no_window_accepted == true
	      and $top_design.screenshot_manifest.no_window_ready == true
	      and (($manifest.screenshot_count.native_true_window // 0) == 0)
	      and (($manifest.screenshot_count.native_true_window_route // 0) == 0)
	      and (($manifest.screenshot_count.native_true_window_secondary // 0) == 0)
	      and (($manifest.screenshot_count.native_true_window_secondary_mobile // 0) == 0)
	      and $manifest.screenshot_count.total >= 44;
	    def release_artifact_waiting_branch_ready:
	      $release_artifact_intake.release_artifact_state.waiting_for_release_artifact == true
	      and $release_artifact_intake.release_artifact_state.release_artifact_present == false
	      and $release_artifact_intake.release_artifact_state.release_artifact_valid == false
	      and $release_artifact_intake.release_artifact_state.receipt_contract_version == 0
	      and $release_artifact_intake.release_artifact_state.evidence_readback_valid == false
	      and $release_artifact_intake.release_artifact_state.signed_notarized_stapled_artifact_present == false
	      and $release_artifact_intake.release_artifact_state.public_distribution_artifact_written == false
	      and $release_artifact_intake.release_artifact_state.public_upload_performed == false
	      and $release_artifact_intake.source_alignment.present_artifact_branch_supported == false
	      and $release_artifact_intake.source_alignment.independent_approval_verifier_contract_ready == false
	      and ($release_artifact_intake.release_artifact_blockers | index("signed_notarized_stapled_artifact_missing") != null)
	      and ($release_artifact_intake.release_artifact_blockers | index("public_distribution_artifact_not_written") != null)
	      and ($release_artifact_intake.release_artifact_blockers | index("release_artifact_present_branch_unsupported_without_independent_approval_verifier") != null);
	    def release_artifact_state_ready:
	      release_artifact_waiting_branch_ready;
	    def source_chain_ready:
      $future.future_plan_refresh_gate_ready == true
      and ($future.future_plan | map(.id)) == old_plan_ids
      and $future.r52_minimum_gate.root_report_replay_required_count == 32
      and $operator_refresh.operator_briefing_refresh_gate_ready == true
      and ($operator_refresh.updated_critical_risk_count >= 1 and $operator_refresh.updated_critical_risk_count <= 4)
      and $operator_refresh.current_next_plan_ids == old_plan_ids
      and $release_approval.release_approval_intake_gate_ready == true
      and $release_approval.release_approval_state.waiting_for_release_approval == true
      and $release_approval.release_approval_state.release_approval_present == false
      and $release_approval.release_approval_state.release_approval_valid == false
      and $release_approval.release_approval_state.independent_approval_verifier_ready == false
      and $release_approval.release_approval_state.self_reported_approval_can_authorize_release == false
      and ($release_approval.approval_blockers | index("independent_release_approval_verifier_unavailable") != null)
      and $release_approval.claim_boundary.release_approval_claim_ready == false
	      and $release_approval.release_approval_state.root_report_replay_required_count_after_intake == 34
	      and $top_design.top_design_referee_refresh_gate_ready == true
	      and $top_design.referee_matrix.control_ui.persisted_phone320_screenshot_ready == true
	      and (top_design_hard_ready or top_design_no_window_ready)
	      and $top_design.current_state.root_report_replay_required_count_after_top_design_refresh == 35
      and $release_artifact.release_artifact_boundary_gate_ready == true
      and $release_artifact.release_artifact_boundary.root_report_replay_required_count_after_boundary == 36
      and $release_artifact.release_artifact_boundary.next_required_artifact_gate == "signed_notarized_stapled_artifact_gate"
      and $release_artifact.release_artifact_boundary.signed_notarized_stapled_artifact_present == false
      and $release_artifact.release_artifact_boundary.public_distribution_artifact_written == false
      and $release_artifact_intake.release_artifact_intake_gate_ready == true
      and $release_artifact_intake.intake_version == 3
      and $release_artifact_intake.root_report_replay_required_count_after_intake == 37
      and release_artifact_state_ready
      and $release_artifact_intake.claim_boundary.release_artifact_claim_ready == false
      and $release_artifact_intake.claim_boundary.public_distribution_claim_ready == false
      and $release_artifact_intake.claim_boundary.release_claim_ready == false
      and $release_artifact_roundtrip.release_artifact_roundtrip_gate_ready == true
      and $release_artifact_roundtrip.roundtrip_kind == "release_artifact_v3_fail_closed_contract_replay"
      and $release_artifact_roundtrip.roundtrip_version == 3
      and $release_artifact_roundtrip.roundtrip_ready_count == 2
      and $release_artifact_roundtrip.source_alignment.waiting_branch_ready == true
      and $release_artifact_roundtrip.source_alignment.simulated_artifact_ready == false
      and $release_artifact_roundtrip.source_alignment.legacy_simulated_artifact_rejected == true
      and $release_artifact_roundtrip.source_alignment.v3_valid_branch_selftest_ready == true
	      and $release_artifact_roundtrip.source_alignment.present_artifact_branch_supported == false
	      and $release_artifact_roundtrip.source_alignment.independent_approval_verifier_contract_ready == false
	      and $release_artifact_roundtrip.source_alignment.present_branch_ready == false
	      and $release_artifact_roundtrip.source_alignment.present_branch_release_artifact_present == false
	      and $release_artifact_roundtrip.source_alignment.present_branch_release_artifact_valid == false
      and $release_artifact_roundtrip.source_alignment.root_report_replay_required_count_after_roundtrip == 41
      and $release_artifact_roundtrip.claim_boundary.local_release_artifact_roundtrip_ready == true
      and $release_artifact_roundtrip.claim_boundary.release_artifact_claim_ready == false
      and $release_artifact_roundtrip.claim_boundary.live_product_claim_ready == false
      and $release_artifact_roundtrip.claim_boundary.public_distribution_claim_ready == false
      and $release_artifact_roundtrip.claim_boundary.release_claim_ready == false
      and $release_artifact_roundtrip.side_effects.external_mutation == false
      and $dispatch.backend_dispatch_packet_gate_ready == true
      and $dispatch.selected_packet_ids == selected_ids
      and $dispatch.backend_lane_target.target_repo == "/Users/qianqi/.openclaw/workspace/Hepta"
      and $receipt_refresh.backend_receipt_refresh_lock_gate_ready == true
      and $receipt_refresh.selected_refresh_ids == selected_ids
      and ($receipt_refresh.receipt_state.real_backend_receipt_present | type) == "boolean"
      and ($receipt_refresh.claim_boundary.real_backend_receipt_claim_ready | type) == "boolean"
      and ($receipt_refresh.claim_boundary.backend_receipt_claim_ready | type) == "boolean"
      and ($receipt_refresh.refresh_requirements.required_ui_refresh_commands | length) == 2
      and $archive.evidence_archive_gate_ready == true
      and $archive.claim_boundary.local_evidence_archive_ready == true
      and $archive.all_extracted_items_sha256_match == true
	      and $manifest.screenshot_manifest_ready == true
	      and $manifest.screenshot_count.control_ui == 4
	      and (
	        (top_design_hard_ready and $manifest.screenshot_count.total >= 60)
	        or (top_design_no_window_ready and $manifest.screenshot_count.total >= 44)
	      )
      and sha_ready($future_plan_sha)
      and sha_ready($operator_refresh_sha)
      and sha_ready($release_approval_sha)
      and sha_ready($top_design_sha)
      and sha_ready($release_artifact_sha)
      and sha_ready($release_artifact_intake_sha)
      and sha_ready($release_artifact_roundtrip_sha)
      and sha_ready($backend_dispatch_sha)
      and sha_ready($receipt_refresh_sha)
      and sha_ready($evidence_archive_sha)
      and sha_ready($screenshot_manifest_sha);
    (
      source_chain_ready
      and $release_artifact.claim_boundary.release_artifact_claim_ready == false
      and $release_artifact.claim_boundary.live_product_claim_ready == false
      and $release_artifact.claim_boundary.public_distribution_claim_ready == false
      and $release_artifact.claim_boundary.release_claim_ready == false
      and $top_design.claim_boundary.desktop_mobile_design_claim_ready == true
      and $top_design.claim_boundary.live_product_claim_ready == false
      and $operator_refresh.claim_boundary.real_backend_receipt_claim_ready == $receipt_refresh.claim_boundary.real_backend_receipt_claim_ready
      and $operator_refresh.claim_boundary.backend_adapter_promoted == false
    ) as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      current_plan_refresh_gate_ready:$ready,
      plan_kind:"local_ui_current_plan_refresh_after_release_artifact_roundtrip",
      plan_version:1,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      refresh_dir:$refresh_dir,
      refresh_markdown_path:$refresh_markdown_path,
      source_reports:{
        legacy_future_plan_refresh:$future_plan_path,
        operator_briefing_refresh:$operator_refresh_path,
        release_approval_intake:$release_approval_path,
        top_design_referee_refresh:$top_design_path,
        release_artifact_boundary:$release_artifact_path,
        release_artifact_intake:$release_artifact_intake_path,
        release_artifact_roundtrip:$release_artifact_roundtrip_path,
        backend_dispatch_packet:$backend_dispatch_path,
        backend_receipt_refresh_lock:$receipt_refresh_path,
        evidence_archive:$evidence_archive_path,
        screenshot_manifest:$screenshot_manifest_path
      },
      source_report_sha256:{
        legacy_future_plan_refresh:$future_plan_sha,
        operator_briefing_refresh:$operator_refresh_sha,
        release_approval_intake:$release_approval_sha,
        top_design_referee_refresh:$top_design_sha,
        release_artifact_boundary:$release_artifact_sha,
        release_artifact_intake:$release_artifact_intake_sha,
        release_artifact_roundtrip:$release_artifact_roundtrip_sha,
        backend_dispatch_packet:$backend_dispatch_sha,
        backend_receipt_refresh_lock:$receipt_refresh_sha,
        evidence_archive:$evidence_archive_sha,
        screenshot_manifest:$screenshot_manifest_sha
      },
      legacy_plan_snapshot:{
        legacy_plan_ids:($future.future_plan | map(.id)),
        legacy_minimum_gate_id:$future.future_plan[0].id,
        legacy_root_report_replay_required_count:$future.r52_minimum_gate.root_report_replay_required_count,
        legacy_kept_for_replay:true
      },
	      current_minimum_gate:{
	        gate_id:"r62_minimum_ui_demo_gate",
	        current_full_hard_evidence_ready:top_design_hard_ready,
	        current_evidence_mode:$top_design.true_window_evidence_mode,
	        no_window_evidence_accepted:top_design_no_window_ready,
	        root_report_replay_required_count_after_current_plan_refresh:41,
        control_phone320_required:true,
        control_phone320_ready:$top_design.referee_matrix.control_ui.persisted_phone320_screenshot_ready,
        control_phone320_sha256:$top_design.referee_matrix.control_ui.persisted_phone320_screenshot.sha256,
        main_true_window_required:2,
        route_true_window_required:4,
        route_unique_required:4,
        route_content_probe_required:true,
        desktop_secondary_required:5,
        mobile_secondary_required:5,
        mobile_content_probe_required:true,
        release_approval_intake_required:true,
        top_design_referee_refresh_required:true,
        release_artifact_boundary_required:true,
        release_artifact_intake_required:true,
        release_artifact_roundtrip_required:true,
        backend_receipt_roundtrip_required:true,
        backend_receipt_refresh_lock_required:true,
        signed_notarized_stapled_artifact_required_for_release:true,
        counts:$manifest.screenshot_count
      },
      current_plan:[
        {
          priority:1,
          id:"r62_minimum_ui_demo_gate",
          owner_lane:"hepta-ui",
          action:"keep r62 roundtrip-backed readiness as the minimum UI demo gate before public demo or release claims",
          required_evidence:[
            "control_ui_phone320_playwright_system_chrome",
            "main_true_window_2",
            "route_true_window_4_unique_with_content_probe",
            "desktop_secondary_5_unique",
            "mobile_secondary_5_unique_with_content_probe",
            "release_approval_intake_green",
            "top_design_referee_refresh_green",
            "release_artifact_boundary_green",
            "release_artifact_intake_green",
            "release_artifact_roundtrip_green",
            "root_report_replay_41"
	          ],
	          current_full_hard_evidence_ready:top_design_hard_ready,
	          current_evidence_mode:$top_design.true_window_evidence_mode
	        },
        {
          priority:2,
          id:"backend_real_receipt_return",
          owner_lane:"backend_contract",
          action:(if $receipt_refresh.claim_boundary.backend_receipt_claim_ready then "keep the accepted backend receipt bound to the dispatch packet and the completed full-hard refresh" else "execute the first five backend dispatch items and return a real receipt matching the receipt template plus refresh-lock requirements" end),
          selected_ids:selected_ids,
          target_repo:"/Users/qianqi/.openclaw/workspace/Hepta",
          dispatch_archive_sha256:$dispatch.archive_sha256
        },
        {
          priority:3,
          id:"ui_refresh_after_real_receipt",
          owner_lane:"hepta-ui",
          action:(if $receipt_refresh.claim_boundary.backend_receipt_claim_ready then "keep the no-window and full-hard readiness refresh bound to the accepted backend receipt" else "rerun no-window and full-hard readiness with the real backend receipt before claiming backend receipt acceptance" end),
          required_commands:$receipt_refresh.refresh_requirements.required_ui_refresh_commands
        },
        {
          priority:4,
          id:"release_artifact_roundtrip_and_signed_artifact_gate",
          owner_lane:"release_operator",
          action:"record explicit release approval, intake signed/notarized/stapled artifact evidence through the roundtrip branch, then refresh UI readiness before any release/public distribution claim",
          next_required_artifact_gate:$release_artifact.release_artifact_boundary.next_required_artifact_gate,
          intake_template_sha256:$release_artifact_intake.template_sha256,
          waiting_for_release_artifact:$release_artifact_intake.release_artifact_state.waiting_for_release_artifact,
          release_artifact_present:$release_artifact_intake.release_artifact_state.release_artifact_present,
          release_artifact_valid:$release_artifact_intake.release_artifact_state.release_artifact_valid,
          signed_notarized_stapled_artifact_present:$release_artifact_intake.release_artifact_state.signed_notarized_stapled_artifact_present,
          local_distribution_artifact_written:$release_artifact_intake.release_artifact_state.local_distribution_artifact_written,
          public_distribution_artifact_written:$release_artifact_intake.release_artifact_state.public_distribution_artifact_written,
          public_upload_performed:$release_artifact_intake.release_artifact_state.public_upload_performed,
          local_roundtrip_ready:$release_artifact_roundtrip.claim_boundary.local_release_artifact_roundtrip_ready,
          roundtrip_present_branch_ready:$release_artifact_roundtrip.source_alignment.present_branch_ready,
          roundtrip_present_branch_valid:$release_artifact_roundtrip.source_alignment.present_branch_release_artifact_valid,
          roundtrip_legacy_simulated_rejected:$release_artifact_roundtrip.source_alignment.legacy_simulated_artifact_rejected,
          roundtrip_v3_valid_branch_selftest_ready:$release_artifact_roundtrip.source_alignment.v3_valid_branch_selftest_ready,
          roundtrip_legacy_artifact_sha256:$release_artifact_roundtrip.source_report_sha256.legacy_v1_simulated_artifact,
          roundtrip_legacy_rejection_report_sha256:$release_artifact_roundtrip.source_report_sha256.legacy_v1_rejection_intake,
          roundtrip_v3_selftest_log_sha256:$release_artifact_roundtrip.source_report_sha256.v3_intake_selftest_log,
          blockers:[
            (if $release_approval.release_approval_state.release_approval_valid then empty else "operator_release_approval_required" end),
            "independent_release_approval_verifier_unavailable",
            "release_artifact_present_branch_unsupported_without_independent_approval_verifier",
            (if $release_artifact_intake.release_artifact_state.signed_notarized_stapled_artifact_present then empty else "signed_notarized_stapled_artifact_missing" end),
            (if $release_artifact_intake.release_artifact_state.public_distribution_artifact_written then empty else "public_distribution_artifact_not_written" end),
            (if $receipt_refresh.receipt_state.real_backend_receipt_present then empty else "real_backend_receipt_missing" end)
          ]
        }
      ],
      current_plan_count:4,
      current_plan_ids:current_plan_ids,
      source_alignment:{
        legacy_future_plan_refresh_ready:$future.future_plan_refresh_gate_ready,
        legacy_plan_ids:($future.future_plan | map(.id)),
        operator_briefing_refresh_ready:$operator_refresh.operator_briefing_refresh_gate_ready,
        operator_briefing_refresh_plan_ids:$operator_refresh.current_next_plan_ids,
        release_approval_intake_ready:$release_approval.release_approval_intake_gate_ready,
        release_approval_waiting_for_approval:$release_approval.release_approval_state.waiting_for_release_approval,
        release_approval_present:$release_approval.release_approval_state.release_approval_present,
        release_approval_valid:$release_approval.release_approval_state.release_approval_valid,
        independent_approval_verifier_ready:$release_approval.release_approval_state.independent_approval_verifier_ready,
        self_reported_approval_can_authorize_release:$release_approval.release_approval_state.self_reported_approval_can_authorize_release,
        top_design_referee_refresh_ready:$top_design.top_design_referee_refresh_gate_ready,
        top_design_control_phone320_ready:$top_design.referee_matrix.control_ui.persisted_phone320_screenshot_ready,
        release_artifact_boundary_ready:$release_artifact.release_artifact_boundary_gate_ready,
        release_artifact_boundary_root_report_required_count:$release_artifact.release_artifact_boundary.root_report_replay_required_count_after_boundary,
        release_artifact_intake_ready:$release_artifact_intake.release_artifact_intake_gate_ready,
        release_artifact_intake_root_report_required_count:$release_artifact_intake.root_report_replay_required_count_after_intake,
        release_artifact_intake_waiting_for_artifact:$release_artifact_intake.release_artifact_state.waiting_for_release_artifact,
	        release_artifact_intake_artifact_present:$release_artifact_intake.release_artifact_state.release_artifact_present,
	        release_artifact_intake_artifact_valid:$release_artifact_intake.release_artifact_state.release_artifact_valid,
	        release_artifact_intake_receipt_contract_version:$release_artifact_intake.release_artifact_state.receipt_contract_version,
	        release_artifact_intake_evidence_readback_valid:$release_artifact_intake.release_artifact_state.evidence_readback_valid,
            release_artifact_intake_present_artifact_branch_supported:$release_artifact_intake.source_alignment.present_artifact_branch_supported,
            release_artifact_intake_independent_approval_verifier_contract_ready:$release_artifact_intake.source_alignment.independent_approval_verifier_contract_ready,
        release_artifact_intake_signed_notarized_stapled_artifact_present:$release_artifact_intake.release_artifact_state.signed_notarized_stapled_artifact_present,
        release_artifact_intake_public_distribution_artifact_written:$release_artifact_intake.release_artifact_state.public_distribution_artifact_written,
        release_artifact_intake_public_upload_performed:$release_artifact_intake.release_artifact_state.public_upload_performed,
        release_artifact_roundtrip_ready:$release_artifact_roundtrip.release_artifact_roundtrip_gate_ready,
        release_artifact_roundtrip_root_report_required_count:$release_artifact_roundtrip.source_alignment.root_report_replay_required_count_after_roundtrip,
        release_artifact_roundtrip_waiting_branch_ready:$release_artifact_roundtrip.source_alignment.waiting_branch_ready,
	        release_artifact_roundtrip_present_branch_ready:$release_artifact_roundtrip.source_alignment.present_branch_ready,
	        release_artifact_roundtrip_present_artifact_present:$release_artifact_roundtrip.source_alignment.present_branch_release_artifact_present,
	        release_artifact_roundtrip_present_artifact_valid:$release_artifact_roundtrip.source_alignment.present_branch_release_artifact_valid,
            release_artifact_roundtrip_present_artifact_branch_supported:$release_artifact_roundtrip.source_alignment.present_artifact_branch_supported,
            release_artifact_roundtrip_independent_approval_verifier_contract_ready:$release_artifact_roundtrip.source_alignment.independent_approval_verifier_contract_ready,
        release_artifact_roundtrip_legacy_simulated_rejected:$release_artifact_roundtrip.source_alignment.legacy_simulated_artifact_rejected,
        release_artifact_roundtrip_v3_valid_branch_selftest_ready:$release_artifact_roundtrip.source_alignment.v3_valid_branch_selftest_ready,
        backend_dispatch_packet_ready:$dispatch.backend_dispatch_packet_gate_ready,
        backend_dispatch_packet_archive_sha256:$dispatch.archive_sha256,
        backend_receipt_refresh_lock_ready:$receipt_refresh.backend_receipt_refresh_lock_gate_ready,
        real_backend_receipt_present:$receipt_refresh.receipt_state.real_backend_receipt_present,
        real_backend_receipt_claim_ready:$receipt_refresh.claim_boundary.real_backend_receipt_claim_ready,
        backend_receipt_claim_ready:$receipt_refresh.claim_boundary.backend_receipt_claim_ready,
        evidence_archive_ready:$archive.evidence_archive_gate_ready,
        screenshot_manifest_ready:$manifest.screenshot_manifest_ready,
        selected_ids_match:($dispatch.selected_packet_ids == selected_ids and $receipt_refresh.selected_refresh_ids == selected_ids),
        current_plan_supersedes_legacy_plan:true
      },
      claim_boundary:{
        local_current_plan_refresh_ready:$ready,
        local_release_artifact_boundary_ready:$release_artifact.claim_boundary.local_release_artifact_boundary_ready,
        desktop_mobile_design_claim_ready:$top_design.claim_boundary.desktop_mobile_design_claim_ready,
        real_backend_receipt_claim_ready:$receipt_refresh.claim_boundary.real_backend_receipt_claim_ready,
        backend_receipt_claim_ready:$receipt_refresh.claim_boundary.backend_receipt_claim_ready,
        backend_adapter_promoted:false,
        readback_evidence_recorded:false,
        release_approval_claim_ready:$release_approval.claim_boundary.release_approval_claim_ready,
        release_artifact_claim_ready:false,
        local_release_artifact_roundtrip_ready:$release_artifact_roundtrip.claim_boundary.local_release_artifact_roundtrip_ready,
        release_execution_ready:false,
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
        local_markdown_written:true,
        matrix_login:false,
        gateway_call:false,
        provider_invoked:false,
        channel_delivery:false,
        backend_adapter_promoted:false,
        live_runtime_mutation:false,
        public_distribution_artifact_written:false,
        external_mutation:false
      }
    }' >"$REPORT_DRAFT"

jq -r '
  "# Hepta UI Current Plan Refresh\n\n"
  + "- Kind: \(.plan_kind)\n"
  + "- Current plan count: \(.current_plan_count)\n"
  + "- Current plan ids: \(.current_plan_ids | join(","))\n"
  + "- Legacy plan ids: \(.legacy_plan_snapshot.legacy_plan_ids | join(","))\n"
  + "- Root replay required after this refresh: \(.current_minimum_gate.root_report_replay_required_count_after_current_plan_refresh)\n"
  + "- Release artifact next gate: \(.current_plan[3].next_required_artifact_gate)\n"
  + "- Release artifact roundtrip ready: \(.source_alignment.release_artifact_roundtrip_ready)\n"
  + "- Release artifact present branch valid: \(.source_alignment.release_artifact_roundtrip_present_artifact_valid)\n"
  + "- Real backend receipt present: \(.source_alignment.real_backend_receipt_present)\n"
  + "- Dispatch archive: \(.source_alignment.backend_dispatch_packet_archive_sha256)\n"
  + "- Claim boundary: real_backend_receipt=\(.claim_boundary.real_backend_receipt_claim_ready); backend_adapter=false; release_artifact=false; live_product=false; public_distribution=false; release=false\n\n"
  + "## Current Plan\n\n"
  + (.current_plan
      | map("- \(.priority). `\(.id)` owner=\(.owner_lane)")
      | join("\n"))
  + "\n"
' "$REPORT_DRAFT" >"$MARKDOWN_TMP"

hepta_safe_atomic_replace "$MARKDOWN_TMP" "$REFRESH_MARKDOWN_PATH" current_plan_markdown
markdown_sha="$(file_sha256 "$REFRESH_MARKDOWN_PATH")"
markdown_bytes="$(file_bytes "$REFRESH_MARKDOWN_PATH")"

jq \
  --arg markdown_sha "$markdown_sha" \
  --argjson markdown_bytes "$markdown_bytes" \
  '. + {
    refresh_markdown_sha256:$markdown_sha,
    refresh_markdown_bytes:$markdown_bytes
  }' "$REPORT_DRAFT" >"$REPORT_TMP"

jq -e '
  .status == "ready"
  and .current_plan_refresh_gate_ready == true
  and .plan_kind == "local_ui_current_plan_refresh_after_release_artifact_roundtrip"
  and .plan_version == 1
  and .legacy_plan_snapshot.legacy_plan_ids == ["r52_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt"]
  and .legacy_plan_snapshot.legacy_root_report_replay_required_count == 32
  and .current_minimum_gate.gate_id == "r62_minimum_ui_demo_gate"
  and .current_minimum_gate.root_report_replay_required_count_after_current_plan_refresh == 41
  and .current_minimum_gate.control_phone320_ready == true
  and .current_minimum_gate.release_approval_intake_required == true
  and .current_minimum_gate.top_design_referee_refresh_required == true
  and .current_minimum_gate.release_artifact_boundary_required == true
  and .current_minimum_gate.release_artifact_intake_required == true
  and .current_minimum_gate.release_artifact_roundtrip_required == true
  and .current_minimum_gate.signed_notarized_stapled_artifact_required_for_release == true
  and .current_plan_count == 4
  and .current_plan_ids == ["r62_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt","release_artifact_roundtrip_and_signed_artifact_gate"]
  and .current_plan[1].selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .current_plan[1].target_repo == "/Users/qianqi/.openclaw/workspace/Hepta"
  and (.current_plan[1].dispatch_archive_sha256 | test("^[0-9a-f]{64}$"))
  and (.current_plan[2].required_commands | length) == 2
  and .current_plan[3].next_required_artifact_gate == "signed_notarized_stapled_artifact_gate"
  and (.current_plan[3].waiting_for_release_artifact | type) == "boolean"
  and (.current_plan[3].release_artifact_present | type) == "boolean"
  and (.current_plan[3].release_artifact_valid | type) == "boolean"
  and .current_plan[3].local_roundtrip_ready == true
  and .current_plan[3].roundtrip_present_branch_ready == false
  and .current_plan[3].roundtrip_present_branch_valid == false
  and .current_plan[3].roundtrip_legacy_simulated_rejected == true
  and .current_plan[3].roundtrip_v3_valid_branch_selftest_ready == true
  and (.current_plan[3].intake_template_sha256 | test("^[0-9a-f]{64}$"))
  and (.current_plan[3].roundtrip_legacy_artifact_sha256 | test("^[0-9a-f]{64}$"))
  and (.current_plan[3].roundtrip_legacy_rejection_report_sha256 | test("^[0-9a-f]{64}$"))
  and (.current_plan[3].roundtrip_v3_selftest_log_sha256 | test("^[0-9a-f]{64}$"))
  and .current_plan[3].waiting_for_release_artifact == true
  and .current_plan[3].release_artifact_present == false
  and .current_plan[3].release_artifact_valid == false
  and .current_plan[3].signed_notarized_stapled_artifact_present == false
  and .current_plan[3].local_distribution_artifact_written == false
  and .current_plan[3].public_distribution_artifact_written == false
  and .current_plan[3].public_upload_performed == false
  and (.current_plan[3].blockers | index("signed_notarized_stapled_artifact_missing") != null)
  and (.current_plan[3].blockers | index("public_distribution_artifact_not_written") != null)
  and (.current_plan[3].blockers | index("release_artifact_present_branch_unsupported_without_independent_approval_verifier") != null)
  and .source_alignment.current_plan_supersedes_legacy_plan == true
  and .source_alignment.release_approval_waiting_for_approval == true
  and .source_alignment.release_approval_present == false
  and .source_alignment.release_approval_valid == false
  and .source_alignment.independent_approval_verifier_ready == false
  and .source_alignment.self_reported_approval_can_authorize_release == false
  and (.current_plan[3].blockers | index("operator_release_approval_required") != null)
  and (.current_plan[3].blockers | index("independent_release_approval_verifier_unavailable") != null)
  and .claim_boundary.release_approval_claim_ready == false
  and .source_alignment.release_artifact_boundary_ready == true
  and .source_alignment.release_artifact_boundary_root_report_required_count == 36
  and .source_alignment.release_artifact_intake_ready == true
  and .source_alignment.release_artifact_intake_root_report_required_count == 37
  and .source_alignment.release_artifact_intake_waiting_for_artifact == true
  and .source_alignment.release_artifact_intake_artifact_present == false
  and .source_alignment.release_artifact_intake_artifact_valid == false
  and .source_alignment.release_artifact_intake_receipt_contract_version == 0
  and .source_alignment.release_artifact_intake_evidence_readback_valid == false
  and .source_alignment.release_artifact_intake_present_artifact_branch_supported == false
  and .source_alignment.release_artifact_intake_independent_approval_verifier_contract_ready == false
  and .source_alignment.release_artifact_intake_signed_notarized_stapled_artifact_present == false
  and .source_alignment.release_artifact_intake_public_distribution_artifact_written == false
  and .source_alignment.release_artifact_intake_public_upload_performed == false
  and .source_alignment.release_artifact_roundtrip_ready == true
  and .source_alignment.release_artifact_roundtrip_root_report_required_count == 41
  and .source_alignment.release_artifact_roundtrip_waiting_branch_ready == true
  and .source_alignment.release_artifact_roundtrip_present_branch_ready == false
  and .source_alignment.release_artifact_roundtrip_present_artifact_present == false
  and .source_alignment.release_artifact_roundtrip_present_artifact_valid == false
  and .source_alignment.release_artifact_roundtrip_present_artifact_branch_supported == false
  and .source_alignment.release_artifact_roundtrip_independent_approval_verifier_contract_ready == false
  and .source_alignment.release_artifact_roundtrip_legacy_simulated_rejected == true
  and .source_alignment.release_artifact_roundtrip_v3_valid_branch_selftest_ready == true
  and .source_alignment.selected_ids_match == true
  and (.source_alignment.real_backend_receipt_present | type) == "boolean"
  and (.source_alignment.real_backend_receipt_claim_ready | type) == "boolean"
  and (.source_alignment.backend_receipt_claim_ready | type) == "boolean"
  and (.refresh_markdown_sha256 | test("^[0-9a-f]{64}$"))
  and .refresh_markdown_bytes > 0
  and .claim_boundary.local_current_plan_refresh_ready == true
  and .claim_boundary.real_backend_receipt_claim_ready == .source_alignment.real_backend_receipt_claim_ready
  and .claim_boundary.backend_receipt_claim_ready == .source_alignment.backend_receipt_claim_ready
  and .claim_boundary.backend_adapter_promoted == false
  and .claim_boundary.release_artifact_claim_ready == false
  and .claim_boundary.local_release_artifact_roundtrip_ready == true
  and .claim_boundary.release_execution_ready == false
  and .claim_boundary.live_runtime_mutation == false
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and .claim_boundary.public_upload_performed == false
  and .claim_boundary.signing_notarization_performed == false
  and .side_effects.local_markdown_written == true
  and .side_effects.external_mutation == false
' "$REPORT_TMP" >/dev/null

hepta_safe_atomic_replace "$REPORT_TMP" "$REPORT_PATH" current_plan_report
cat "$REPORT_TMP"
