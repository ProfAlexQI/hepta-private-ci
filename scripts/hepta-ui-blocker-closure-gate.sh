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
REPORT_PATH="${HEPTA_UI_BLOCKER_CLOSURE_REPORT_PATH:-$READINESS_DIR/ui-blocker-closure-gate.json}"
CLOSURE_DIR="${HEPTA_UI_BLOCKER_CLOSURE_DIR:-$READINESS_DIR/blocker-closure}"
CLOSURE_MARKDOWN_PATH="$CLOSURE_DIR/blocker-closure.md"
READINESS_DIR="$(hepta_safe_normalize_path readiness "$READINESS_DIR")"
REPORT_PATH="$(hepta_safe_normalize_path report "$REPORT_PATH")"
CLOSURE_DIR="$(hepta_safe_normalize_path closure "$CLOSURE_DIR")"
CLOSURE_MARKDOWN_PATH="$CLOSURE_DIR/blocker-closure.md"
REPORT_PARENT="$(hepta_safe_normalize_path report_parent "$(/usr/bin/dirname "$REPORT_PATH")")"
hepta_safe_require_directory_target readiness "$READINESS_DIR"
hepta_safe_require_directory_target closure "$CLOSURE_DIR"
hepta_safe_require_directory_target report_parent "$REPORT_PARENT"
hepta_safe_require_regular_target report "$REPORT_PATH"
hepta_safe_require_regular_target closure_markdown "$CLOSURE_MARKDOWN_PATH"
if hepta_safe_paths_overlap "$READINESS_DIR" "$REPO_ROOT"; then
  printf 'blocker-closure readiness must not overlap the repository\n' >&2
  exit 64
fi
if ! hepta_safe_is_strict_descendant "$CLOSURE_DIR" "$READINESS_DIR"; then
  printf 'blocker-closure directory must be a strict readiness child\n' >&2
  exit 64
fi
if [[ "$REPORT_PARENT" != "$READINESS_DIR" ]] \
  && ! hepta_safe_is_strict_descendant "$REPORT_PARENT" "$READINESS_DIR"; then
  printf 'blocker-closure report parent must remain inside readiness\n' >&2
  exit 64
fi
if hepta_safe_paths_overlap "$REPORT_PATH" "$CLOSURE_DIR"; then
  printf 'blocker-closure report and managed directory must be disjoint\n' >&2
  exit 64
fi

CURRENT_PLAN_REFRESH_REPORT_PATH="$READINESS_DIR/ui-current-plan-refresh-gate.json"
OPERATOR_BRIEFING_REFRESH_REPORT_PATH="$READINESS_DIR/ui-operator-briefing-refresh-gate.json"
BACKEND_DISPATCH_PACKET_REPORT_PATH="$READINESS_DIR/ui-backend-dispatch-packet-gate.json"
BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH="$READINESS_DIR/ui-backend-receipt-refresh-lock-gate.json"
RELEASE_APPROVAL_INTAKE_REPORT_PATH="$READINESS_DIR/ui-release-approval-intake-gate.json"
RELEASE_SIGNING_CAPABILITY_REPORT_PATH="$READINESS_DIR/ui-release-signing-capability-gate.json"
RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH="$READINESS_DIR/ui-release-artifact-boundary-gate.json"
RELEASE_ARTIFACT_INTAKE_REPORT_PATH="$READINESS_DIR/ui-release-artifact-intake-gate.json"
RELEASE_ARTIFACT_ROUNDTRIP_REPORT_PATH="$READINESS_DIR/ui-release-artifact-roundtrip-gate.json"
TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH="$READINESS_DIR/ui-top-design-referee-refresh-gate.json"
EVIDENCE_ARCHIVE_REPORT_PATH="$READINESS_DIR/ui-evidence-archive-gate.json"
SCREENSHOT_MANIFEST_PATH="$READINESS_DIR/screenshot-manifest.json"
for protected_input in \
  "$CURRENT_PLAN_REFRESH_REPORT_PATH" "$OPERATOR_BRIEFING_REFRESH_REPORT_PATH" \
  "$BACKEND_DISPATCH_PACKET_REPORT_PATH" "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH" \
  "$RELEASE_APPROVAL_INTAKE_REPORT_PATH" "$RELEASE_SIGNING_CAPABILITY_REPORT_PATH" \
  "$RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH" "$RELEASE_ARTIFACT_INTAKE_REPORT_PATH" \
  "$RELEASE_ARTIFACT_ROUNDTRIP_REPORT_PATH" "$TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH" \
  "$EVIDENCE_ARCHIVE_REPORT_PATH" "$SCREENSHOT_MANIFEST_PATH"; do
  if hepta_safe_paths_overlap "$protected_input" "$CLOSURE_DIR" \
    || hepta_safe_paths_overlap "$protected_input" "$REPORT_PATH"; then
    printf 'blocker-closure output overlaps protected input: %s\n' "$protected_input" >&2
    exit 64
  fi
done

HEPTA_UI_GATE_REQUIREMENT_CONTEXT="the Hepta UI blocker closure gate"
HEPTA_UI_REPORT_INPUT_LABEL="blocker closure"
source scripts/lib/hepta-ui-gate-common-v1.sh

require_command jq
require_command shasum

require_report "$CURRENT_PLAN_REFRESH_REPORT_PATH"
require_report "$OPERATOR_BRIEFING_REFRESH_REPORT_PATH"
require_report "$BACKEND_DISPATCH_PACKET_REPORT_PATH"
require_report "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH"
require_report "$RELEASE_APPROVAL_INTAKE_REPORT_PATH"
require_report "$RELEASE_SIGNING_CAPABILITY_REPORT_PATH"
require_report "$RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH"
require_report "$RELEASE_ARTIFACT_INTAKE_REPORT_PATH"
require_report "$RELEASE_ARTIFACT_ROUNDTRIP_REPORT_PATH"
require_report "$TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH"
require_report "$EVIDENCE_ARCHIVE_REPORT_PATH"
require_report "$SCREENSHOT_MANIFEST_PATH"

mkdir -p "$CLOSURE_DIR" "$REPORT_PARENT"
hepta_safe_revalidate_directory closure "$CLOSURE_DIR"
hepta_safe_revalidate_directory report_parent "$REPORT_PARENT"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-blocker-closure.XXXXXX")"
REPORT_DRAFT="$TMP_DIR/blocker-closure-draft.json"
REPORT_TMP="$TMP_DIR/blocker-closure-report.json"
MARKDOWN_TMP="$TMP_DIR/blocker-closure.md"
trap 'rm -rf "$TMP_DIR"' EXIT

current_plan_sha="$(file_sha256 "$CURRENT_PLAN_REFRESH_REPORT_PATH")"
operator_refresh_sha="$(file_sha256 "$OPERATOR_BRIEFING_REFRESH_REPORT_PATH")"
backend_dispatch_sha="$(file_sha256 "$BACKEND_DISPATCH_PACKET_REPORT_PATH")"
receipt_refresh_sha="$(file_sha256 "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH")"
release_approval_sha="$(file_sha256 "$RELEASE_APPROVAL_INTAKE_REPORT_PATH")"
release_signing_sha="$(file_sha256 "$RELEASE_SIGNING_CAPABILITY_REPORT_PATH")"
release_artifact_sha="$(file_sha256 "$RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH")"
release_artifact_intake_sha="$(file_sha256 "$RELEASE_ARTIFACT_INTAKE_REPORT_PATH")"
release_artifact_roundtrip_sha="$(file_sha256 "$RELEASE_ARTIFACT_ROUNDTRIP_REPORT_PATH")"
top_design_sha="$(file_sha256 "$TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH")"
evidence_archive_sha="$(file_sha256 "$EVIDENCE_ARCHIVE_REPORT_PATH")"
screenshot_manifest_sha="$(file_sha256 "$SCREENSHOT_MANIFEST_PATH")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_blocker_closure_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg closure_dir "$CLOSURE_DIR" \
  --arg closure_markdown_path "$CLOSURE_MARKDOWN_PATH" \
  --arg current_plan_path "$CURRENT_PLAN_REFRESH_REPORT_PATH" \
  --arg operator_refresh_path "$OPERATOR_BRIEFING_REFRESH_REPORT_PATH" \
  --arg backend_dispatch_path "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  --arg receipt_refresh_path "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH" \
  --arg release_approval_path "$RELEASE_APPROVAL_INTAKE_REPORT_PATH" \
  --arg release_signing_path "$RELEASE_SIGNING_CAPABILITY_REPORT_PATH" \
  --arg release_artifact_path "$RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH" \
  --arg release_artifact_intake_path "$RELEASE_ARTIFACT_INTAKE_REPORT_PATH" \
  --arg release_artifact_roundtrip_path "$RELEASE_ARTIFACT_ROUNDTRIP_REPORT_PATH" \
  --arg top_design_path "$TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH" \
  --arg evidence_archive_path "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  --arg screenshot_manifest_path "$SCREENSHOT_MANIFEST_PATH" \
  --arg current_plan_sha "$current_plan_sha" \
  --arg operator_refresh_sha "$operator_refresh_sha" \
  --arg backend_dispatch_sha "$backend_dispatch_sha" \
  --arg receipt_refresh_sha "$receipt_refresh_sha" \
  --arg release_approval_sha "$release_approval_sha" \
  --arg release_signing_sha "$release_signing_sha" \
  --arg release_artifact_sha "$release_artifact_sha" \
  --arg release_artifact_intake_sha "$release_artifact_intake_sha" \
  --arg release_artifact_roundtrip_sha "$release_artifact_roundtrip_sha" \
  --arg top_design_sha "$top_design_sha" \
  --arg evidence_archive_sha "$evidence_archive_sha" \
  --arg screenshot_manifest_sha "$screenshot_manifest_sha" \
  --slurpfile current_plan_file "$CURRENT_PLAN_REFRESH_REPORT_PATH" \
  --slurpfile operator_refresh_file "$OPERATOR_BRIEFING_REFRESH_REPORT_PATH" \
  --slurpfile backend_dispatch_file "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  --slurpfile receipt_refresh_file "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH" \
  --slurpfile release_approval_file "$RELEASE_APPROVAL_INTAKE_REPORT_PATH" \
  --slurpfile release_signing_file "$RELEASE_SIGNING_CAPABILITY_REPORT_PATH" \
  --slurpfile release_artifact_file "$RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH" \
  --slurpfile release_artifact_intake_file "$RELEASE_ARTIFACT_INTAKE_REPORT_PATH" \
  --slurpfile release_artifact_roundtrip_file "$RELEASE_ARTIFACT_ROUNDTRIP_REPORT_PATH" \
  --slurpfile top_design_file "$TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH" \
  --slurpfile evidence_archive_file "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  --slurpfile screenshot_manifest_file "$SCREENSHOT_MANIFEST_PATH" \
  '
  ($current_plan_file[0]) as $current
  | ($operator_refresh_file[0]) as $operator_refresh
  | ($backend_dispatch_file[0]) as $dispatch
  | ($receipt_refresh_file[0]) as $receipt_refresh
  | ($release_approval_file[0]) as $release_approval
  | ($release_signing_file[0]) as $release_signing
  | ($release_artifact_file[0]) as $release_artifact
  | ($release_artifact_intake_file[0]) as $release_artifact_intake
  | ($release_artifact_roundtrip_file[0]) as $release_artifact_roundtrip
  | ($top_design_file[0]) as $top_design
  | ($evidence_archive_file[0]) as $archive
  | ($screenshot_manifest_file[0]) as $manifest
	  | def selected_ids: ["message_search","file_upload_send","media_download_playback","notifications","room_settings"];
	    def current_plan_ids: ["r62_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt","release_artifact_roundtrip_and_signed_artifact_gate"];
	    def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
	    def top_design_hard_ready:
	      $top_design.true_window_evidence_mode == "full_hard_true_window"
	      and $top_design.hard_true_window_evidence_ready == true
	      and $top_design.screenshot_manifest.hard_ready == true
	      and $manifest.screenshot_count.native_true_window == 2
	      and $manifest.screenshot_count.native_true_window_route == 4
	      and $manifest.screenshot_count.native_true_window_secondary == 5
	      and $manifest.screenshot_count.native_true_window_secondary_mobile == 5
	      and $manifest.screenshot_count.total >= 60;
	    def top_design_no_window_ready:
	      $top_design.true_window_evidence_mode == "no_window_fixture"
	      and $top_design.no_window_evidence_accepted == true
	      and $top_design.screenshot_manifest.no_window_ready == true
	      and $current.current_minimum_gate.no_window_evidence_accepted == true
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
	      and $release_artifact_intake.release_artifact_state.local_distribution_artifact_written == false
	      and $release_artifact_intake.release_artifact_state.public_distribution_artifact_written == false
	      and $release_artifact_intake.release_artifact_state.public_upload_performed == false
	      and $release_artifact_intake.source_alignment.present_artifact_branch_supported == false
	      and $release_artifact_intake.source_alignment.independent_approval_verifier_contract_ready == false
	      and ($release_artifact_intake.release_artifact_blockers | index("release_artifact_present_branch_unsupported_without_independent_approval_verifier")) != null;
	    def release_artifact_state_ready:
	      release_artifact_waiting_branch_ready;
	    def source_chain_ready:
      $current.current_plan_refresh_gate_ready == true
      and $current.current_minimum_gate.gate_id == "r62_minimum_ui_demo_gate"
      and $current.current_minimum_gate.root_report_replay_required_count_after_current_plan_refresh == 41
      and $current.current_plan_ids == current_plan_ids
      and $current.source_alignment.selected_ids_match == true
      and ($current.source_alignment.real_backend_receipt_present | type) == "boolean"
      and $operator_refresh.operator_briefing_refresh_gate_ready == true
      and ($operator_refresh.updated_critical_risk_count >= 1 and $operator_refresh.updated_critical_risk_count <= 4)
      and $dispatch.backend_dispatch_packet_gate_ready == true
      and $dispatch.selected_packet_ids == selected_ids
      and $dispatch.backend_lane_target.target_repo == "/Users/qianqi/.openclaw/workspace/Hepta"
      and ($dispatch.dispatch_guardrail.backend_agent_available | type) == "boolean"
      and $dispatch.dispatch_guardrail.external_dispatch_performed == false
      and $dispatch.claim_boundary.backend_adapter_promoted == false
      and $dispatch.claim_boundary.readback_evidence_recorded == false
      and ($dispatch.archive_sha256 | test("^[0-9a-f]{64}$"))
      and $dispatch.archive_bytes > 0
      and $receipt_refresh.backend_receipt_refresh_lock_gate_ready == true
      and $receipt_refresh.selected_refresh_ids == selected_ids
      and ($receipt_refresh.receipt_state.real_backend_receipt_present | type) == "boolean"
      and ($receipt_refresh.receipt_state.backend_receipt_valid | type) == "boolean"
      and ($receipt_refresh.claim_boundary.real_backend_receipt_claim_ready | type) == "boolean"
      and ($receipt_refresh.claim_boundary.backend_receipt_claim_ready | type) == "boolean"
      and $receipt_refresh.claim_boundary.live_product_claim_ready == false
      and $release_approval.release_approval_intake_gate_ready == true
      and $release_approval.release_approval_state.waiting_for_release_approval == true
      and $release_approval.release_approval_state.release_approval_present == false
      and $release_approval.release_approval_state.release_approval_valid == false
      and $release_approval.release_approval_state.independent_approval_verifier_ready == false
      and $release_approval.release_approval_state.self_reported_approval_can_authorize_release == false
      and ($release_approval.approval_blockers | index("independent_release_approval_verifier_unavailable")) != null
      and $release_approval.claim_boundary.release_approval_claim_ready == false
      and $release_approval.claim_boundary.release_execution_ready == false
      and $release_signing.audit_status == "ready"
      and $release_signing.capability_status == "blocked"
      and $release_signing.capability_version == 2
      and $release_signing.release_signing_capability_gate_ready == true
      and $release_signing.release_execution_prerequisites.bundle_and_release_script_contract_ready == true
      and ($release_signing.release_execution_prerequisites.keychain_identity_ready | type) == "boolean"
      and ($release_signing.release_execution_prerequisites.notary_credentials_ready | type) == "boolean"
      and ($release_signing.release_execution_prerequisites.release_signing_execution_prerequisites_ready | type) == "boolean"
      and $release_signing.claim_boundary.local_release_signing_capability_audit_ready == true
      and $release_signing.claim_boundary.release_execution_ready == false
      and $release_signing.claim_boundary.public_distribution_claim_ready == false
      and $release_signing.claim_boundary.release_claim_ready == false
      and $release_signing.side_effects.credential_value_captured == false
      and $release_signing.side_effects.notary_submission_performed == false
      and $release_signing.side_effects.app_signed == false
      and $release_signing.side_effects.app_notarized == false
      and $release_signing.side_effects.app_stapled == false
      and $release_signing.side_effects.public_distribution_artifact_written == false
      and $release_signing.side_effects.external_mutation == false
      and $release_artifact.release_artifact_boundary_gate_ready == true
      and $release_artifact.release_artifact_boundary.signed_notarized_stapled_artifact_present == false
      and $release_artifact.release_artifact_boundary.public_distribution_artifact_written == false
      and $release_artifact.release_artifact_boundary.next_required_artifact_gate == "signed_notarized_stapled_artifact_gate"
      and $release_artifact.claim_boundary.release_artifact_claim_ready == false
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
	      and $release_artifact_roundtrip.source_alignment.present_branch_local_distribution_artifact_written == false
      and $release_artifact_roundtrip.source_alignment.present_branch_public_upload_performed == false
      and $release_artifact_roundtrip.source_alignment.root_report_replay_required_count_after_roundtrip == 41
      and $release_artifact_roundtrip.claim_boundary.local_release_artifact_roundtrip_ready == true
      and $release_artifact_roundtrip.claim_boundary.release_artifact_claim_ready == false
      and $release_artifact_roundtrip.claim_boundary.live_product_claim_ready == false
      and $release_artifact_roundtrip.claim_boundary.public_distribution_claim_ready == false
      and $release_artifact_roundtrip.claim_boundary.release_claim_ready == false
      and $release_artifact_roundtrip.side_effects.external_mutation == false
      and $top_design.top_design_referee_refresh_gate_ready == true
      and $top_design.claim_boundary.desktop_mobile_design_claim_ready == true
      and $top_design.claim_boundary.live_product_claim_ready == false
      and $archive.evidence_archive_gate_ready == true
      and $archive.claim_boundary.local_evidence_archive_ready == true
      and $archive.claim_boundary.live_product_claim_ready == false
	      and $archive.all_extracted_items_sha256_match == true
	      and $manifest.screenshot_manifest_ready == true
	      and $manifest.screenshot_count.control_ui == 4
	      and (
	        (top_design_hard_ready and $manifest.screenshot_count.total >= 60)
	        or (top_design_no_window_ready and $manifest.screenshot_count.total >= 44)
	      )
      and sha_ready($current_plan_sha)
      and sha_ready($operator_refresh_sha)
      and sha_ready($backend_dispatch_sha)
      and sha_ready($receipt_refresh_sha)
      and sha_ready($release_approval_sha)
      and sha_ready($release_signing_sha)
      and sha_ready($release_artifact_sha)
      and sha_ready($release_artifact_intake_sha)
      and sha_ready($release_artifact_roundtrip_sha)
      and sha_ready($top_design_sha)
      and sha_ready($evidence_archive_sha)
      and sha_ready($screenshot_manifest_sha);
    def critical_blockers:
      [
        (if $dispatch.dispatch_guardrail.backend_agent_available then empty else {
          id:"backend_agent_dispatch_unavailable_in_this_session",
          owner_lane:"backend_contract",
          state:"blocked",
          evidence:"ui-backend-dispatch-packet-gate.dispatch_guardrail.backend_agent_available=false"
        } end),
        (if $receipt_refresh.receipt_state.real_backend_receipt_present then empty else {
          id:"real_backend_receipt_missing",
          owner_lane:"backend_contract",
          state:"blocked",
          evidence:"ui-backend-receipt-refresh-lock-gate.receipt_state.real_backend_receipt_present=false"
        } end),
        (if $receipt_refresh.receipt_state.real_backend_receipt_present then empty else {
          id:"backend_contract_first_five_not_executed",
          owner_lane:"backend_contract",
          state:"blocked",
          selected_ids:selected_ids,
          target_repo:$dispatch.backend_lane_target.target_repo
        } end),
        (if ($receipt_refresh.refresh_requirements.full_hard_refresh_required == true) then {
          id:"backend_receipt_full_hard_refresh_required",
          owner_lane:"hepta-ui",
          state:"blocked",
          evidence:"ui-backend-receipt-refresh-lock-gate.refresh_requirements.full_hard_refresh_required=true",
          required_commands:$receipt_refresh.refresh_requirements.required_ui_refresh_commands
        } else empty end),
        (if $release_approval.release_approval_state.release_approval_valid then empty else {
          id:"release_approval_missing",
          owner_lane:"release_operator",
          state:"blocked",
          evidence:"ui-release-approval-intake-gate.release_approval_state.release_approval_present=false"
        } end),
        (if $release_approval.release_approval_state.independent_approval_verifier_ready then empty else {
          id:"independent_release_approval_verifier_unavailable",
          owner_lane:"release_operator",
          state:"blocked",
          evidence:"ui-release-approval-intake-gate.release_approval_state.independent_approval_verifier_ready=false"
        } end),
        (if $release_signing.release_execution_prerequisites.keychain_identity_ready then empty else {
          id:"developer_id_identity_missing_or_not_matching_configured_identity",
          owner_lane:"release_operator",
          state:"blocked",
          evidence:"ui-release-signing-capability-gate.release_execution_prerequisites.keychain_identity_ready=false"
        } end),
        (if $release_signing.release_execution_prerequisites.notary_credentials_ready then empty else {
          id:"apple_notary_credentials_missing",
          owner_lane:"release_operator",
          state:"blocked",
          evidence:"ui-release-signing-capability-gate.release_execution_prerequisites.notary_credentials_ready=false"
        } end),
        (if $release_signing.local_tooling.distribution_tools_ready then empty else {
          id:"local_distribution_tooling_missing",
          owner_lane:"hepta-ui",
          state:"blocked",
          evidence:"ui-release-signing-capability-gate.local_tooling.distribution_tools_ready=false"
        } end),
        (if $release_artifact_intake.release_artifact_state.signed_notarized_stapled_artifact_present then empty else {
          id:"signed_notarized_stapled_artifact_missing",
          owner_lane:"release_operator",
          state:"blocked",
          evidence:"ui-release-artifact-intake-gate.release_artifact_state.signed_notarized_stapled_artifact_present=false"
        } end),
        (if $release_artifact_intake.release_artifact_state.public_distribution_artifact_written then empty else {
          id:"public_distribution_artifact_not_written",
          owner_lane:"release_operator",
          state:"blocked",
          evidence:"ui-release-artifact-intake-gate.release_artifact_state.public_distribution_artifact_written=false"
        } end)
      ];
    source_chain_ready as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      blocker_closure_gate_ready:$ready,
      closure_kind:"local_ui_blocker_closure_after_current_plan_refresh",
      closure_version:1,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      closure_dir:$closure_dir,
      closure_markdown_path:$closure_markdown_path,
      source_reports:{
        current_plan_refresh:$current_plan_path,
        operator_briefing_refresh:$operator_refresh_path,
        backend_dispatch_packet:$backend_dispatch_path,
        backend_receipt_refresh_lock:$receipt_refresh_path,
        release_approval_intake:$release_approval_path,
        release_signing_capability:$release_signing_path,
        release_artifact_boundary:$release_artifact_path,
        release_artifact_intake:$release_artifact_intake_path,
        release_artifact_roundtrip:$release_artifact_roundtrip_path,
        top_design_referee_refresh:$top_design_path,
        evidence_archive:$evidence_archive_path,
        screenshot_manifest:$screenshot_manifest_path
      },
      source_report_sha256:{
        current_plan_refresh:$current_plan_sha,
        operator_briefing_refresh:$operator_refresh_sha,
        backend_dispatch_packet:$backend_dispatch_sha,
        backend_receipt_refresh_lock:$receipt_refresh_sha,
        release_approval_intake:$release_approval_sha,
        release_signing_capability:$release_signing_sha,
        release_artifact_boundary:$release_artifact_sha,
        release_artifact_intake:$release_artifact_intake_sha,
        release_artifact_roundtrip:$release_artifact_roundtrip_sha,
        top_design_referee_refresh:$top_design_sha,
        evidence_archive:$evidence_archive_sha,
        screenshot_manifest:$screenshot_manifest_sha
      },
      closure_state:{
        current_minimum_gate_id:$current.current_minimum_gate.gate_id,
        current_plan_ids:$current.current_plan_ids,
        prior_current_plan_root_report_required_count:$current.current_minimum_gate.root_report_replay_required_count_after_current_plan_refresh,
        root_report_replay_required_count_after_blocker_closure:41,
        local_ui_demo_evidence_ready:$current.current_minimum_gate.current_full_hard_evidence_ready,
        desktop_mobile_design_claim_ready:$top_design.claim_boundary.desktop_mobile_design_claim_ready,
        backend_agent_available:$dispatch.dispatch_guardrail.backend_agent_available,
        external_dispatch_performed:$dispatch.dispatch_guardrail.external_dispatch_performed,
        target_backend_repo:$dispatch.backend_lane_target.target_repo,
        dispatch_archive_sha256:$dispatch.archive_sha256,
        dispatch_archive_bytes:$dispatch.archive_bytes,
        selected_ids:selected_ids,
        real_backend_receipt_present:$receipt_refresh.receipt_state.real_backend_receipt_present,
        backend_receipt_valid:$receipt_refresh.receipt_state.backend_receipt_valid,
        backend_receipt_claim_ready:$receipt_refresh.claim_boundary.backend_receipt_claim_ready,
        backend_adapter_promoted:$dispatch.claim_boundary.backend_adapter_promoted,
        readback_evidence_recorded:$dispatch.claim_boundary.readback_evidence_recorded,
        release_approval_present:$release_approval.release_approval_state.release_approval_present,
        release_approval_valid:$release_approval.release_approval_state.release_approval_valid,
        independent_approval_verifier_ready:$release_approval.release_approval_state.independent_approval_verifier_ready,
        self_reported_approval_can_authorize_release:$release_approval.release_approval_state.self_reported_approval_can_authorize_release,
        release_signing_execution_prerequisites_ready:$release_signing.release_execution_prerequisites.release_signing_execution_prerequisites_ready,
        release_signing_keychain_identity_ready:$release_signing.release_execution_prerequisites.keychain_identity_ready,
        release_signing_notary_credentials_ready:$release_signing.release_execution_prerequisites.notary_credentials_ready,
        release_signing_distribution_tools_ready:$release_signing.local_tooling.distribution_tools_ready,
        release_signing_blockers:$release_signing.blockers,
        release_artifact_present:$release_artifact_intake.release_artifact_state.release_artifact_present,
        release_artifact_valid:$release_artifact_intake.release_artifact_state.release_artifact_valid,
        release_artifact_receipt_contract_version:$release_artifact_intake.release_artifact_state.receipt_contract_version,
        release_artifact_evidence_readback_valid:$release_artifact_intake.release_artifact_state.evidence_readback_valid,
        release_artifact_present_artifact_branch_supported:$release_artifact_intake.source_alignment.present_artifact_branch_supported,
        release_artifact_independent_approval_verifier_contract_ready:$release_artifact_intake.source_alignment.independent_approval_verifier_contract_ready,
        signed_notarized_stapled_artifact_present:$release_artifact_intake.release_artifact_state.signed_notarized_stapled_artifact_present,
        local_distribution_artifact_written:$release_artifact_intake.release_artifact_state.local_distribution_artifact_written,
        public_distribution_artifact_written:$release_artifact_intake.release_artifact_state.public_distribution_artifact_written,
        public_upload_performed:$release_artifact_intake.release_artifact_state.public_upload_performed,
        local_release_artifact_roundtrip_ready:$release_artifact_roundtrip.claim_boundary.local_release_artifact_roundtrip_ready,
        release_artifact_roundtrip_present_branch_ready:$release_artifact_roundtrip.source_alignment.present_branch_ready,
        release_artifact_roundtrip_present_artifact_present:$release_artifact_roundtrip.source_alignment.present_branch_release_artifact_present,
        release_artifact_roundtrip_present_artifact_valid:$release_artifact_roundtrip.source_alignment.present_branch_release_artifact_valid,
        release_artifact_roundtrip_present_artifact_branch_supported:$release_artifact_roundtrip.source_alignment.present_artifact_branch_supported,
        release_artifact_roundtrip_independent_approval_verifier_contract_ready:$release_artifact_roundtrip.source_alignment.independent_approval_verifier_contract_ready,
        release_artifact_roundtrip_present_local_distribution_artifact_written:$release_artifact_roundtrip.source_alignment.present_branch_local_distribution_artifact_written,
        release_artifact_roundtrip_present_public_upload_performed:$release_artifact_roundtrip.source_alignment.present_branch_public_upload_performed,
        release_artifact_roundtrip_legacy_simulated_rejected:$release_artifact_roundtrip.source_alignment.legacy_simulated_artifact_rejected,
        release_artifact_roundtrip_v3_valid_branch_selftest_ready:$release_artifact_roundtrip.source_alignment.v3_valid_branch_selftest_ready,
        next_required_artifact_gate:$release_artifact.release_artifact_boundary.next_required_artifact_gate,
        screenshot_total:$manifest.screenshot_count.total
      },
      critical_blockers:critical_blockers,
      critical_blocker_count:(critical_blockers | length),
      future_plan:$current.current_plan,
      next_unblock_sequence:[
        (if $dispatch.dispatch_guardrail.backend_agent_available then empty else "make_hepta_backend_agent_or_backend_lane_execution_available" end),
        (if $receipt_refresh.receipt_state.real_backend_receipt_present then empty else "execute_backend_dispatch_packet_for_first_five_contracts" end),
        (if $receipt_refresh.receipt_state.real_backend_receipt_present then empty else "return_real_backend_receipt_bound_to_dispatch_archive" end),
        (if $receipt_refresh.claim_boundary.backend_receipt_claim_ready then empty else "rerun_no_window_then_full_hard_ui_readiness_with_real_receipt" end),
        (if $release_approval.release_approval_state.independent_approval_verifier_ready then empty else "provision_independent_release_approval_verifier" end),
        (if $release_artifact_intake.release_artifact_state.signed_notarized_stapled_artifact_present then empty else "provision_developer_id_identity_and_notary_profile" end),
        (if $release_artifact_intake.release_artifact_state.signed_notarized_stapled_artifact_present then "complete_release_policy_review_before_any_public_distribution_claim" elif $release_approval.release_approval_state.release_approval_valid then "collect_signed_notarized_stapled_artifact_before_public_distribution" else "collect_release_approval_and_signed_notarized_stapled_artifact_before_public_distribution" end)
      ],
      source_alignment:{
        current_plan_refresh_ready:$current.current_plan_refresh_gate_ready,
        current_plan_ids_match:($current.current_plan_ids == current_plan_ids),
        current_plan_root_report_required_count:$current.current_minimum_gate.root_report_replay_required_count_after_current_plan_refresh,
        operator_briefing_refresh_ready:$operator_refresh.operator_briefing_refresh_gate_ready,
        operator_briefing_refresh_updated_critical_risk_count:$operator_refresh.updated_critical_risk_count,
        backend_dispatch_packet_ready:$dispatch.backend_dispatch_packet_gate_ready,
        backend_agent_available:$dispatch.dispatch_guardrail.backend_agent_available,
        external_dispatch_performed:$dispatch.dispatch_guardrail.external_dispatch_performed,
        backend_receipt_refresh_lock_ready:$receipt_refresh.backend_receipt_refresh_lock_gate_ready,
        real_backend_receipt_present:$receipt_refresh.receipt_state.real_backend_receipt_present,
        backend_receipt_claim_ready:$receipt_refresh.claim_boundary.backend_receipt_claim_ready,
        release_approval_intake_ready:$release_approval.release_approval_intake_gate_ready,
        release_approval_present:$release_approval.release_approval_state.release_approval_present,
        release_approval_valid:$release_approval.release_approval_state.release_approval_valid,
        independent_approval_verifier_ready:$release_approval.release_approval_state.independent_approval_verifier_ready,
        self_reported_approval_can_authorize_release:$release_approval.release_approval_state.self_reported_approval_can_authorize_release,
        release_signing_capability_ready:$release_signing.release_signing_capability_gate_ready,
        release_signing_execution_prerequisites_ready:$release_signing.release_execution_prerequisites.release_signing_execution_prerequisites_ready,
        release_signing_keychain_identity_ready:$release_signing.release_execution_prerequisites.keychain_identity_ready,
        release_signing_notary_credentials_ready:$release_signing.release_execution_prerequisites.notary_credentials_ready,
        release_signing_distribution_tools_ready:$release_signing.local_tooling.distribution_tools_ready,
        release_artifact_boundary_ready:$release_artifact.release_artifact_boundary_gate_ready,
        signed_notarized_stapled_artifact_present:$release_artifact.release_artifact_boundary.signed_notarized_stapled_artifact_present,
        release_artifact_intake_ready:$release_artifact_intake.release_artifact_intake_gate_ready,
        release_artifact_intake_root_report_required_count:$release_artifact_intake.root_report_replay_required_count_after_intake,
	        release_artifact_present:$release_artifact_intake.release_artifact_state.release_artifact_present,
	        release_artifact_valid:$release_artifact_intake.release_artifact_state.release_artifact_valid,
	        release_artifact_intake_receipt_contract_version:$release_artifact_intake.release_artifact_state.receipt_contract_version,
	        release_artifact_intake_evidence_readback_valid:$release_artifact_intake.release_artifact_state.evidence_readback_valid,
            release_artifact_present_artifact_branch_supported:$release_artifact_intake.source_alignment.present_artifact_branch_supported,
            release_artifact_independent_approval_verifier_contract_ready:$release_artifact_intake.source_alignment.independent_approval_verifier_contract_ready,
        release_artifact_intake_signed_notarized_stapled_artifact_present:$release_artifact_intake.release_artifact_state.signed_notarized_stapled_artifact_present,
        release_artifact_intake_local_distribution_artifact_written:$release_artifact_intake.release_artifact_state.local_distribution_artifact_written,
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
        release_artifact_roundtrip_present_local_distribution_artifact_written:$release_artifact_roundtrip.source_alignment.present_branch_local_distribution_artifact_written,
        release_artifact_roundtrip_present_public_upload_performed:$release_artifact_roundtrip.source_alignment.present_branch_public_upload_performed,
        release_artifact_roundtrip_legacy_simulated_rejected:$release_artifact_roundtrip.source_alignment.legacy_simulated_artifact_rejected,
        release_artifact_roundtrip_v3_valid_branch_selftest_ready:$release_artifact_roundtrip.source_alignment.v3_valid_branch_selftest_ready,
        top_design_referee_refresh_ready:$top_design.top_design_referee_refresh_gate_ready,
        desktop_mobile_design_claim_ready:$top_design.claim_boundary.desktop_mobile_design_claim_ready,
        evidence_archive_ready:$archive.evidence_archive_gate_ready,
        screenshot_manifest_ready:$manifest.screenshot_manifest_ready,
        root_report_replay_required_count_after_blocker_closure:41,
        selected_ids_match:(
          $dispatch.selected_packet_ids == selected_ids
          and $receipt_refresh.selected_refresh_ids == selected_ids
          and $current.current_plan[1].selected_ids == selected_ids
        )
      },
      claim_boundary:{
        local_blocker_closure_ready:$ready,
        local_current_plan_refresh_ready:$current.claim_boundary.local_current_plan_refresh_ready,
        local_backend_dispatch_packet_ready:$dispatch.claim_boundary.local_backend_dispatch_packet_ready,
        local_backend_receipt_refresh_lock_ready:$receipt_refresh.claim_boundary.local_backend_receipt_refresh_lock_ready,
        local_release_approval_intake_ready:$release_approval.claim_boundary.local_release_approval_intake_ready,
        local_release_signing_capability_audit_ready:$release_signing.claim_boundary.local_release_signing_capability_audit_ready,
        local_release_artifact_boundary_ready:$release_artifact.claim_boundary.local_release_artifact_boundary_ready,
        local_release_artifact_intake_ready:$release_artifact_intake.claim_boundary.local_release_artifact_intake_ready,
        local_release_artifact_roundtrip_ready:$release_artifact_roundtrip.claim_boundary.local_release_artifact_roundtrip_ready,
        desktop_mobile_design_claim_ready:$top_design.claim_boundary.desktop_mobile_design_claim_ready,
        real_backend_receipt_claim_ready:$receipt_refresh.claim_boundary.real_backend_receipt_claim_ready,
        backend_receipt_claim_ready:$receipt_refresh.claim_boundary.backend_receipt_claim_ready,
        backend_adapter_promoted:false,
        readback_evidence_recorded:false,
        release_approval_claim_ready:$release_approval.claim_boundary.release_approval_claim_ready,
        release_signing_execution_prerequisites_ready:$release_signing.claim_boundary.release_signing_execution_prerequisites_ready,
        release_artifact_claim_ready:false,
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
        local_markdown_written:true,
        local_report_written:true,
        backend_agent_spawned:false,
        backend_repo_write:false,
        matrix_login:false,
        gateway_call:false,
        provider_invoked:false,
        channel_delivery:false,
        credential_value_read:false,
        keychain_identity_lookup_performed:false,
        notary_submission_performed:false,
        app_signed:false,
        app_notarized:false,
        app_stapled:false,
        public_distribution_artifact_written:false,
        public_upload_performed:false,
        external_mutation:false
      }
    }' >"$REPORT_DRAFT"

jq -r '
  "# Hepta UI Blocker Closure\n\n"
  + "- Kind: \(.closure_kind)\n"
  + "- Status: \(.status)\n"
  + "- Current minimum gate: \(.closure_state.current_minimum_gate_id)\n"
  + "- Root replay after this gate: \(.closure_state.root_report_replay_required_count_after_blocker_closure)\n"
  + "- Dispatch archive SHA-256: \(.closure_state.dispatch_archive_sha256)\n"
  + "- Backend agent available in local dispatch gate: \(.closure_state.backend_agent_available)\n"
  + "- Real backend receipt present: \(.closure_state.real_backend_receipt_present)\n"
  + "- Release approval valid: \(.closure_state.release_approval_valid)\n"
  + "- Developer ID identity ready: \(.closure_state.release_signing_keychain_identity_ready)\n"
  + "- Apple notary credentials ready: \(.closure_state.release_signing_notary_credentials_ready)\n"
  + "- Signed/notarized/stapled artifact present: \(.closure_state.signed_notarized_stapled_artifact_present)\n"
  + "- Release artifact roundtrip present branch valid: \(.closure_state.release_artifact_roundtrip_present_artifact_valid)\n"
  + "- Critical blocker count: \(.critical_blocker_count)\n\n"
  + "## Blockers\n\n"
  + (.critical_blockers | map("- `\(.id)` -> \(.owner_lane) / \(.state)") | join("\n"))
  + "\n\n## Next Unblock Sequence\n\n"
  + (.next_unblock_sequence | map("- `\(.)`") | join("\n"))
  + "\n"
' "$REPORT_DRAFT" >"$MARKDOWN_TMP"

hepta_safe_atomic_replace "$MARKDOWN_TMP" "$CLOSURE_MARKDOWN_PATH" blocker_closure_markdown

closure_markdown_sha="$(file_sha256 "$CLOSURE_MARKDOWN_PATH")"
closure_markdown_bytes="$(file_bytes "$CLOSURE_MARKDOWN_PATH")"

jq \
  --arg closure_markdown_sha "$closure_markdown_sha" \
  --argjson closure_markdown_bytes "$closure_markdown_bytes" \
  '. + {
    closure_markdown_sha256:$closure_markdown_sha,
    closure_markdown_bytes:$closure_markdown_bytes
  }' "$REPORT_DRAFT" >"$REPORT_TMP"

jq -e '
  .status == "ready"
  and .blocker_closure_gate_ready == true
  and .closure_kind == "local_ui_blocker_closure_after_current_plan_refresh"
  and .closure_version == 1
  and .closure_state.current_minimum_gate_id == "r62_minimum_ui_demo_gate"
  and .closure_state.prior_current_plan_root_report_required_count == 41
  and .closure_state.root_report_replay_required_count_after_blocker_closure == 41
  and (.closure_state.backend_agent_available | type) == "boolean"
  and .closure_state.external_dispatch_performed == false
  and (.closure_state.real_backend_receipt_present | type) == "boolean"
  and (.closure_state.backend_receipt_valid | type) == "boolean"
  and (.closure_state.backend_receipt_claim_ready | type) == "boolean"
  and .closure_state.backend_adapter_promoted == false
  and .closure_state.readback_evidence_recorded == false
  and .closure_state.release_approval_present == false
  and .closure_state.release_approval_valid == false
  and .closure_state.independent_approval_verifier_ready == false
  and .closure_state.self_reported_approval_can_authorize_release == false
  and (.critical_blockers | map(.id) | index("release_approval_missing")) != null
  and (.critical_blockers | map(.id) | index("independent_release_approval_verifier_unavailable")) != null
  and .claim_boundary.release_approval_claim_ready == false
  and .closure_state.release_artifact_present == false
  and .closure_state.release_artifact_valid == false
  and .closure_state.release_artifact_receipt_contract_version == 0
  and .closure_state.release_artifact_evidence_readback_valid == false
  and .closure_state.release_artifact_present_artifact_branch_supported == false
  and .closure_state.release_artifact_independent_approval_verifier_contract_ready == false
  and .closure_state.signed_notarized_stapled_artifact_present == false
  and .closure_state.local_distribution_artifact_written == false
  and .closure_state.public_distribution_artifact_written == false
  and .closure_state.public_upload_performed == false
  and .closure_state.local_release_artifact_roundtrip_ready == true
  and .closure_state.release_artifact_roundtrip_present_branch_ready == false
  and .closure_state.release_artifact_roundtrip_present_artifact_present == false
  and .closure_state.release_artifact_roundtrip_present_artifact_valid == false
  and .closure_state.release_artifact_roundtrip_present_artifact_branch_supported == false
  and .closure_state.release_artifact_roundtrip_independent_approval_verifier_contract_ready == false
  and .closure_state.release_artifact_roundtrip_present_local_distribution_artifact_written == false
  and .closure_state.release_artifact_roundtrip_present_public_upload_performed == false
  and .closure_state.release_artifact_roundtrip_legacy_simulated_rejected == true
  and .closure_state.release_artifact_roundtrip_v3_valid_branch_selftest_ready == true
  and .closure_state.next_required_artifact_gate == "signed_notarized_stapled_artifact_gate"
  and .closure_state.selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .critical_blocker_count == (.critical_blockers | length)
  and (.critical_blocker_count >= 0 and .critical_blocker_count <= 10)
  and (
    (
      .closure_state.backend_agent_available == true
      and (.critical_blockers | map(.id) | index("backend_agent_dispatch_unavailable_in_this_session")) == null
    )
    or (
      .closure_state.backend_agent_available == false
      and (.critical_blockers | map(.id) | index("backend_agent_dispatch_unavailable_in_this_session")) != null
    )
  )
  and (
    (
      .closure_state.real_backend_receipt_present == true
      and (.critical_blockers | map(.id) | index("real_backend_receipt_missing")) == null
      and (.critical_blockers | map(.id) | index("backend_contract_first_five_not_executed")) == null
      and (
        (
          .closure_state.backend_receipt_claim_ready == true
          and (.critical_blockers | map(.id) | index("backend_receipt_full_hard_refresh_required")) == null
        )
        or
        (
          .closure_state.backend_receipt_claim_ready == false
          and (.critical_blockers | map(.id) | index("backend_receipt_full_hard_refresh_required")) != null
        )
      )
    )
    or
    (
      .closure_state.real_backend_receipt_present == false
      and (.critical_blockers | map(.id) | index("real_backend_receipt_missing")) != null
      and (.critical_blockers | map(.id) | index("backend_contract_first_five_not_executed")) != null
    )
  )
  and .closure_state.signed_notarized_stapled_artifact_present == false
  and (.critical_blockers | map(.id) | index("signed_notarized_stapled_artifact_missing")) != null
  and (
    (
      .closure_state.release_signing_keychain_identity_ready == false
      and (.critical_blockers | map(.id) | index("developer_id_identity_missing_or_not_matching_configured_identity")) != null
    )
    or (
      .closure_state.release_signing_keychain_identity_ready == true
      and (.critical_blockers | map(.id) | index("developer_id_identity_missing_or_not_matching_configured_identity")) == null
    )
  )
  and (
    (
      .closure_state.release_signing_notary_credentials_ready == false
      and (.critical_blockers | map(.id) | index("apple_notary_credentials_missing")) != null
    )
    or (
      .closure_state.release_signing_notary_credentials_ready == true
      and (.critical_blockers | map(.id) | index("apple_notary_credentials_missing")) == null
    )
  )
  and .closure_state.public_distribution_artifact_written == false
  and (.critical_blockers | map(.id) | index("public_distribution_artifact_not_written")) != null
  and (.future_plan | map(.id)) == ["r62_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt","release_artifact_roundtrip_and_signed_artifact_gate"]
  and ((.next_unblock_sequence | length) >= 1 and (.next_unblock_sequence | length) <= 7)
  and .source_alignment.current_plan_refresh_ready == true
  and .source_alignment.current_plan_ids_match == true
  and .source_alignment.current_plan_root_report_required_count == 41
  and .source_alignment.backend_dispatch_packet_ready == true
  and (.source_alignment.backend_agent_available | type) == "boolean"
  and .source_alignment.external_dispatch_performed == false
  and .source_alignment.backend_receipt_refresh_lock_ready == true
  and (.source_alignment.real_backend_receipt_present | type) == "boolean"
  and (.source_alignment.backend_receipt_claim_ready | type) == "boolean"
  and .source_alignment.release_approval_intake_ready == true
  and .source_alignment.release_approval_present == .closure_state.release_approval_present
  and .source_alignment.release_approval_valid == .closure_state.release_approval_valid
  and .source_alignment.independent_approval_verifier_ready == .closure_state.independent_approval_verifier_ready
  and .source_alignment.self_reported_approval_can_authorize_release == .closure_state.self_reported_approval_can_authorize_release
  and .source_alignment.release_signing_capability_ready == true
  and .source_alignment.release_signing_keychain_identity_ready == .closure_state.release_signing_keychain_identity_ready
  and .source_alignment.release_signing_notary_credentials_ready == .closure_state.release_signing_notary_credentials_ready
  and .source_alignment.release_artifact_boundary_ready == true
  and .source_alignment.signed_notarized_stapled_artifact_present == false
  and .source_alignment.release_artifact_intake_ready == true
  and .source_alignment.release_artifact_intake_root_report_required_count == 37
  and .source_alignment.release_artifact_present == .closure_state.release_artifact_present
  and .source_alignment.release_artifact_valid == .closure_state.release_artifact_valid
  and .source_alignment.release_artifact_intake_receipt_contract_version == .closure_state.release_artifact_receipt_contract_version
  and .source_alignment.release_artifact_intake_evidence_readback_valid == .closure_state.release_artifact_evidence_readback_valid
  and .source_alignment.release_artifact_intake_signed_notarized_stapled_artifact_present == .closure_state.signed_notarized_stapled_artifact_present
  and .source_alignment.release_artifact_intake_local_distribution_artifact_written == .closure_state.local_distribution_artifact_written
  and .source_alignment.release_artifact_intake_public_distribution_artifact_written == .closure_state.public_distribution_artifact_written
  and .source_alignment.release_artifact_intake_public_upload_performed == .closure_state.public_upload_performed
  and .source_alignment.release_artifact_roundtrip_ready == true
  and .source_alignment.release_artifact_roundtrip_root_report_required_count == 41
  and .source_alignment.release_artifact_roundtrip_waiting_branch_ready == true
  and .source_alignment.release_artifact_present_artifact_branch_supported == false
  and .source_alignment.release_artifact_independent_approval_verifier_contract_ready == false
  and .source_alignment.release_artifact_roundtrip_present_branch_ready == false
  and .source_alignment.release_artifact_roundtrip_present_artifact_present == false
  and .source_alignment.release_artifact_roundtrip_present_artifact_valid == false
  and .source_alignment.release_artifact_roundtrip_present_artifact_branch_supported == false
  and .source_alignment.release_artifact_roundtrip_independent_approval_verifier_contract_ready == false
  and .source_alignment.release_artifact_roundtrip_present_local_distribution_artifact_written == false
  and .source_alignment.release_artifact_roundtrip_present_public_upload_performed == false
  and .source_alignment.release_artifact_roundtrip_legacy_simulated_rejected == true
  and .source_alignment.release_artifact_roundtrip_v3_valid_branch_selftest_ready == true
  and .source_alignment.root_report_replay_required_count_after_blocker_closure == 41
  and .source_alignment.selected_ids_match == true
  and .claim_boundary.local_blocker_closure_ready == true
  and .claim_boundary.real_backend_receipt_claim_ready == .closure_state.backend_receipt_claim_ready
  and .claim_boundary.backend_receipt_claim_ready == .closure_state.backend_receipt_claim_ready
  and .claim_boundary.backend_adapter_promoted == false
  and .claim_boundary.readback_evidence_recorded == false
  and .claim_boundary.release_artifact_claim_ready == false
  and .claim_boundary.local_release_artifact_roundtrip_ready == true
  and .claim_boundary.release_signing_execution_prerequisites_ready == .closure_state.release_signing_execution_prerequisites_ready
  and .claim_boundary.release_execution_ready == false
  and .claim_boundary.live_runtime_mutation == false
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.public_upload_performed == false
  and .side_effects.local_markdown_written == true
  and .side_effects.backend_agent_spawned == false
  and .side_effects.backend_repo_write == false
  and .side_effects.matrix_login == false
  and .side_effects.gateway_call == false
  and .side_effects.provider_invoked == false
  and .side_effects.channel_delivery == false
  and .side_effects.notary_submission_performed == false
  and .side_effects.app_signed == false
  and .side_effects.public_distribution_artifact_written == false
  and .side_effects.public_upload_performed == false
  and .side_effects.external_mutation == false
  and (.closure_markdown_sha256 | test("^[0-9a-f]{64}$"))
  and .closure_markdown_bytes > 0
' "$REPORT_TMP" >/dev/null

hepta_safe_atomic_replace "$REPORT_TMP" "$REPORT_PATH" blocker_closure_report
cat "$REPORT_PATH"
