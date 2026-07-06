#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

# Product-readiness first-pass static marker: full_product_root_risk_future_plan_ready:false
# Product-readiness final full-root static marker: full_product_root_risk_future_plan_ready:true
READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.r228-command-palette-item-prismatic-rim-light-glass-targeted-20260626215700}"
CONTROL_BROWSER_REPORT_PATH="${HEPTA_UI_RISK_FUTURE_PLAN_CONTROL_BROWSER_REPORT_PATH:-$READINESS_DIR/control-ui-browser/control-ui-browser-smoke.json}"
OLD_CONTROL_BROWSER_REPORT_PATH="${HEPTA_UI_RISK_FUTURE_PLAN_OLD_CONTROL_BROWSER_REPORT_PATH:-}"
FULL_ROOT_READINESS_REPORT_PATH="${HEPTA_UI_RISK_FUTURE_PLAN_FULL_ROOT_READINESS_REPORT_PATH:-}"
FULL_ROOT_ARTIFACT_SUMMARY_PATH="${HEPTA_UI_RISK_FUTURE_PLAN_FULL_ROOT_ARTIFACT_SUMMARY_PATH:-}"
REPORT_PATH="${HEPTA_UI_RISK_FUTURE_PLAN_POST_R228_REPORT_PATH:-$READINESS_DIR/ui-risk-future-plan-post-r228-gate.json}"
RISK_PLAN_DIR="${HEPTA_UI_RISK_FUTURE_PLAN_POST_R228_DIR:-$READINESS_DIR/risk-future-plan-post-r228}"
RISK_PLAN_MARKDOWN_PATH="$RISK_PLAN_DIR/risk-future-plan-post-r228.md"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf '%s is required for the Hepta UI post-r228 risk/future-plan gate\n' "$1" >&2
    exit 2
  fi
}

require_report() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required post-r228 risk/future-plan input: %s\n' "$path" >&2
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

require_report "$CONTROL_BROWSER_REPORT_PATH"
if [[ -n "$OLD_CONTROL_BROWSER_REPORT_PATH" ]]; then
  require_report "$OLD_CONTROL_BROWSER_REPORT_PATH"
fi

mkdir -p "$RISK_PLAN_DIR"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-post-r228-risk-plan.XXXXXX")"
REPORT_DRAFT="$TMP_DIR/post-r228-risk-plan-draft.json"
REPORT_TMP="$TMP_DIR/post-r228-risk-plan-report.json"
MARKDOWN_TMP="$TMP_DIR/risk-future-plan-post-r228.md"
OLD_REPORT_TMP="$TMP_DIR/old-control-ui-browser-smoke.json"
FULL_ROOT_READINESS_TMP="$TMP_DIR/full-root-readiness.json"
FULL_ROOT_ARTIFACT_SUMMARY_TMP="$TMP_DIR/full-root-artifact-summary.json"
trap 'rm -rf "$TMP_DIR"' EXIT

control_sha="$(file_sha256 "$CONTROL_BROWSER_REPORT_PATH")"
if [[ -n "$OLD_CONTROL_BROWSER_REPORT_PATH" ]]; then
  old_control_sha="$(file_sha256 "$OLD_CONTROL_BROWSER_REPORT_PATH")"
  cp "$OLD_CONTROL_BROWSER_REPORT_PATH" "$OLD_REPORT_TMP"
else
  old_control_sha=""
  printf '{}\n' >"$OLD_REPORT_TMP"
fi

if [[ -n "$FULL_ROOT_READINESS_REPORT_PATH$FULL_ROOT_ARTIFACT_SUMMARY_PATH" ]]; then
  if [[ -z "$FULL_ROOT_READINESS_REPORT_PATH" || -z "$FULL_ROOT_ARTIFACT_SUMMARY_PATH" ]]; then
    printf 'Both full-root readiness and artifact-summary inputs are required when closing post-r228 root wiring\n' >&2
    exit 1
  fi
  require_report "$FULL_ROOT_READINESS_REPORT_PATH"
  require_report "$FULL_ROOT_ARTIFACT_SUMMARY_PATH"
  full_root_readiness_sha="$(file_sha256 "$FULL_ROOT_READINESS_REPORT_PATH")"
  full_root_artifact_summary_sha="$(file_sha256 "$FULL_ROOT_ARTIFACT_SUMMARY_PATH")"
  cp "$FULL_ROOT_READINESS_REPORT_PATH" "$FULL_ROOT_READINESS_TMP"
  cp "$FULL_ROOT_ARTIFACT_SUMMARY_PATH" "$FULL_ROOT_ARTIFACT_SUMMARY_TMP"
else
  full_root_readiness_sha=""
  full_root_artifact_summary_sha=""
  printf '{}\n' >"$FULL_ROOT_READINESS_TMP"
  printf '{}\n' >"$FULL_ROOT_ARTIFACT_SUMMARY_TMP"
fi

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_post_r228_risk_future_plan_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg risk_plan_dir "$RISK_PLAN_DIR" \
  --arg risk_plan_markdown_path "$RISK_PLAN_MARKDOWN_PATH" \
  --arg control_browser_report_path "$CONTROL_BROWSER_REPORT_PATH" \
  --arg old_control_browser_report_path "$OLD_CONTROL_BROWSER_REPORT_PATH" \
  --arg full_root_readiness_report_path "$FULL_ROOT_READINESS_REPORT_PATH" \
  --arg full_root_artifact_summary_path "$FULL_ROOT_ARTIFACT_SUMMARY_PATH" \
  --arg control_sha "$control_sha" \
  --arg old_control_sha "$old_control_sha" \
  --arg full_root_readiness_sha "$full_root_readiness_sha" \
  --arg full_root_artifact_summary_sha "$full_root_artifact_summary_sha" \
  --slurpfile control_file "$CONTROL_BROWSER_REPORT_PATH" \
  --slurpfile old_control_file "$OLD_REPORT_TMP" \
  --slurpfile full_root_readiness_file "$FULL_ROOT_READINESS_TMP" \
  --slurpfile full_root_artifact_summary_file "$FULL_ROOT_ARTIFACT_SUMMARY_TMP" \
  '
  ($control_file[0]) as $control
  | ($old_control_file[0]) as $old
  | ($full_root_readiness_file[0]) as $full_root_readiness
  | ($full_root_artifact_summary_file[0]) as $full_root_artifact_summary
  | def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
    def current_gate_id: "r228_command_palette_item_prismatic_rim_light_glass_minimum_ui_demo_gate";
    def latest_plan_ids: [
      current_gate_id,
      "full_root_risk_future_plan_wiring_post_r228",
      "backend_delivery_receipt_return",
      "backend_real_receipt_return",
      "ui_refresh_after_real_receipt",
      "release_artifact_roundtrip_and_signed_artifact_gate"
    ];
    def required_top_level_fields: [
      "control_ui_harsh_2026_ready",
      "control_ui_command_palette_ready",
      "control_ui_command_palette_surface_light_glass_ready",
      "control_ui_command_palette_surface_prismatic_perimeter_light_glass_ready",
      "control_ui_command_palette_backdrop_caustic_veil_light_glass_ready",
      "control_ui_command_palette_input_light_glass_ready",
      "control_ui_command_palette_input_text_prismatic_etch_light_glass_ready",
      "control_ui_command_palette_input_placeholder_prismatic_etch_light_glass_ready",
      "control_ui_command_palette_input_row_prismatic_separator_light_glass_ready",
      "control_ui_command_palette_input_icon_light_glass_ready",
      "control_ui_command_palette_input_icon_prismatic_light_glass_ready",
      "control_ui_command_palette_close_light_glass_ready",
      "control_ui_command_palette_close_prismatic_icon_light_glass_ready",
      "control_ui_command_palette_results_well_light_glass_ready",
      "control_ui_command_palette_results_well_prismatic_rim_light_glass_ready",
      "control_ui_command_palette_item_light_glass_ready",
      "control_ui_command_palette_item_label_prismatic_etch_light_glass_ready",
      "control_ui_command_palette_item_hover_prismatic_light_glass_ready",
      "control_ui_command_palette_item_prismatic_rim_light_glass_ready",
      "control_ui_command_palette_kind_chip_light_glass_ready"
    ];
    def density_key($field): ($field | sub("^control_ui_"; ""));
    def required_density_fields: (required_top_level_fields | map(density_key(.)) | map(select(. != "harsh_2026_ready")));
    def four_viewport_ready:
      (($control.screenshots // []) | map(.name) | sort) == ["desktop","mobile","narrow","phone320"];
    def current_required_fields_ready:
      all(required_top_level_fields[]; $control[.] == true)
      and all(required_density_fields[]; $control.density_qa[.] == true);
    def old_evidence_rejected:
      ($old_control_browser_report_path | length) > 0
      and (($old.control_ui_command_palette_item_prismatic_rim_light_glass_ready // false) != true)
      and (($old.density_qa.command_palette_item_prismatic_rim_light_glass_ready // false) != true);
    def artifact_post_r228_ready($artifact):
      $artifact.ui_post_r228_risk_future_plan_gate_ready == true
      and $artifact.post_r228_risk_future_plan_latest_minimum_gate_id == current_gate_id
      and $artifact.post_r228_risk_future_plan_required_top_level_field_count == 20
      and $artifact.post_r228_risk_future_plan_required_density_field_count == 19
      and $artifact.post_r228_risk_future_plan_old_evidence_rejected == true
      and $artifact.post_r228_risk_future_plan_four_viewport_ready == true
      and (($artifact.post_r228_risk_future_plan_rim_detail_count // 0) >= 4)
      and $artifact.post_r228_risk_future_plan_live_product_claim_ready == false
      and $artifact.post_r228_risk_future_plan_public_distribution_claim_ready == false
      and $artifact.post_r228_risk_future_plan_release_claim_ready == false
      and $artifact.post_r228_risk_future_plan_external_actions_allowed == false;
    def full_root_paths_supplied:
      (($full_root_readiness_report_path | length) > 0)
      and (($full_root_artifact_summary_path | length) > 0);
    def full_root_post_r228_wiring_ready:
      full_root_paths_supplied
      and sha_ready($full_root_readiness_sha)
      and sha_ready($full_root_artifact_summary_sha)
      and $full_root_readiness.status == "ready"
      and $full_root_readiness.artifact_summary_ready == true
      and $full_root_readiness.local_root_report_replay_ready == true
      and (($full_root_readiness.root_report_replay_count // 0) >= 43)
      and $full_root_readiness.live_product_claim_ready == false
      and $full_root_readiness.public_distribution_claim_ready == false
      and $full_root_readiness.release_claim_ready == false
      and $full_root_artifact_summary.artifact_summary_ready == true
      and $full_root_artifact_summary.local_root_report_replay_ready == true
      and (($full_root_artifact_summary.root_report_replay_count // 0) >= 43)
      and $full_root_artifact_summary.live_product_claim_ready == false
      and $full_root_artifact_summary.public_distribution_claim_ready == false
      and $full_root_artifact_summary.release_claim_ready == false
      and artifact_post_r228_ready($full_root_readiness.artifact_summary)
      and artifact_post_r228_ready($full_root_artifact_summary);
    def backend_delivery_receipt_ready:
      full_root_post_r228_wiring_ready
      and ($full_root_artifact_summary.backend_delivery_audit_delivery_receipt_present // false) == true
      and ($full_root_artifact_summary.backend_delivery_audit_delivery_receipt_valid // false) == true
      and ($full_root_artifact_summary.backend_delivery_audit_backend_delivery_claim_ready // false) == true;
    def backend_real_receipt_ready:
      full_root_post_r228_wiring_ready
      and ($full_root_artifact_summary.backend_receipt_valid // false) == true
      and ($full_root_artifact_summary.backend_receipt_present // false) == true;
    def release_artifact_roundtrip_ready:
      full_root_post_r228_wiring_ready
      and ($full_root_artifact_summary.release_artifact_valid_for_release_claim // false) == true
      and ($full_root_artifact_summary.real_release_artifact_receipt_present // false) == true;
    def full_root_blocker:
      {
        id:"full_root_risk_future_plan_wiring_post_r228",
        owner_lane:"hepta-ui",
        state:"targeted_post_r228_gate_ready_full_product_root_still_needs_wiring"
      };
    def non_ui_blockers: [
      (if backend_delivery_receipt_ready then empty else {
        id:"backend_delivery_receipt_return",
        owner_lane:"backend_contract",
        state:"not_claimed_by_ui_lane"
      } end),
      (if backend_real_receipt_ready then empty else {
        id:"backend_real_receipt_return",
        owner_lane:"backend_contract",
        state:"not_claimed_by_ui_lane"
      } end),
      (if release_artifact_roundtrip_ready then empty else {
        id:"release_artifact_roundtrip_and_signed_artifact_gate",
        owner_lane:"release_operator",
        state:"not_claimed_by_ui_lane"
      } end)
    ];
    def critical_blockers:
      (if full_root_post_r228_wiring_ready then [] else [full_root_blocker] end) + non_ui_blockers;
    def next_unblock_sequence:
      if full_root_post_r228_wiring_ready then
        [
          (if backend_delivery_receipt_ready then empty else "return_backend_delivery_receipt_bound_to_dispatch_archive" end),
          (if backend_real_receipt_ready then empty else "execute_backend_dispatch_packet_for_first_five_contracts_and_return_real_receipt" end),
          (if backend_real_receipt_ready then empty else "rerun_ui_after_real_backend_receipt" end),
          (if release_artifact_roundtrip_ready then empty else "collect_real_signed_notarized_stapled_artifact_before_public_distribution" end)
        ]
      else
        [
          "wire_post_r228_fields_into_full_root_risk_future_plan",
          "rerun_full_product_readiness_or_explain_blocker",
          "continue_ui_residual_visual_gates_while_waiting_for_backend_release_receipts",
          "rerun_ui_after_real_backend_receipt"
        ]
      end;
    (
      $control.status == "ready"
      and current_required_fields_ready
      and four_viewport_ready
      and sha_ready($control_sha)
      and old_evidence_rejected
      and sha_ready($old_control_sha)
    ) as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      risk_future_plan_post_r228_gate_ready:$ready,
      plan_kind:"local_ui_post_r228_command_palette_item_prismatic_rim_risk_future_plan_refresh",
      plan_version:1,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      risk_plan_dir:$risk_plan_dir,
      risk_plan_markdown_path:$risk_plan_markdown_path,
      source_reports:{
        control_browser:$control_browser_report_path,
        old_control_browser:$old_control_browser_report_path,
        full_root_readiness:$full_root_readiness_report_path,
        full_root_artifact_summary:$full_root_artifact_summary_path
      },
      source_report_sha256:{
        control_browser:$control_sha,
        old_control_browser:$old_control_sha,
        full_root_readiness:$full_root_readiness_sha,
        full_root_artifact_summary:$full_root_artifact_summary_sha
      },
      latest_minimum_gate:{
        gate_id:current_gate_id,
        current_artifact_evidence_ready:current_required_fields_ready,
        current_evidence_mode:"targeted_control_ui_browser_four_viewport",
        old_evidence_rejected:old_evidence_rejected,
        control_ui_status:$control.status,
        control_ui_harsh_2026_ready:$control.control_ui_harsh_2026_ready,
        required_top_level_fields:required_top_level_fields,
        required_density_fields:required_density_fields,
        required_top_level_field_count:(required_top_level_fields | length),
        required_density_field_count:(required_density_fields | length),
        four_viewport_ready:four_viewport_ready,
        screenshots:($control.screenshots // [] | map({name,path,bytes,dimensions})),
        command_palette_item_prismatic_rim_ready:$control.control_ui_command_palette_item_prismatic_rim_light_glass_ready,
        command_palette_item_prismatic_rim_detail_count:(
          [($control.density_qa.results // [])[] as $result
            | ($result.translucent_glass_details // [])[]
            | select(.command_palette_item_prismatic_rim_ready == true)]
          | length
        ),
        command_palette_required_ready:current_required_fields_ready
      },
      latest_plan_ids:latest_plan_ids,
      latest_plan:[
        {
          priority:1,
          id:current_gate_id,
          owner_lane:"hepta-ui",
          action:"treat r228 command palette item default prismatic rim as the current Control UI minimum visual gate",
          required_evidence:["control_ui_browser_4_viewports","post_r228_command_palette_prismatic_surface_backdrop_input_icon_close_results_well_kind_item_default_and_hover_gates","old_r227_evidence_rejected"]
        },
        {
          priority:2,
          id:"full_root_risk_future_plan_wiring_post_r228",
          owner_lane:"hepta-ui",
          status:(if full_root_post_r228_wiring_ready then "closed" else "open" end),
          action:"wire post-r228 command palette fields into the full product-readiness risk/future-plan and root replay chain",
          required_evidence:["risk_future_plan_latest_minimum_gate_id_r228","root_replay_contains_post_r228_command_palette_fields","full_product_readiness_no_backend_claim"]
        },
        {
          priority:3,
          id:"backend_delivery_receipt_return",
          owner_lane:"backend_contract",
          action:"return or bind a backend dispatch delivery receipt before any backend handoff delivery claim"
        },
        {
          priority:4,
          id:"backend_real_receipt_return",
          owner_lane:"backend_contract",
          action:"execute the selected backend dispatch items and return a real backend receipt bound to the dispatch archive"
        },
        {
          priority:5,
          id:"ui_refresh_after_real_receipt",
          owner_lane:"hepta-ui",
          action:"rerun UI readiness after a real backend receipt, without promoting backend/runtime changes from this lane"
        },
        {
          priority:6,
          id:"release_artifact_roundtrip_and_signed_artifact_gate",
          owner_lane:"release_operator",
          action:"record release approval plus signed/notarized/stapled artifact evidence before any public distribution claim"
        }
      ],
      latest_plan_count:6,
      critical_blockers:critical_blockers,
      critical_blocker_count:(critical_blockers | length),
      next_unblock_sequence:next_unblock_sequence,
      source_alignment:{
        control_browser_ready:($control.status == "ready"),
        control_ui_harsh_2026_ready:$control.control_ui_harsh_2026_ready,
        post_r228_command_palette_required_ready:current_required_fields_ready,
        old_r227_evidence_rejected:old_evidence_rejected,
        four_viewport_ready:four_viewport_ready,
        control_json_sha256_ready:sha_ready($control_sha),
        old_control_json_sha256_ready:sha_ready($old_control_sha),
        full_root_inputs_supplied:full_root_paths_supplied,
        full_root_post_r228_wiring_ready:full_root_post_r228_wiring_ready,
        full_root_readiness_json_sha256_ready:(if full_root_paths_supplied then sha_ready($full_root_readiness_sha) else false end),
        full_root_artifact_summary_json_sha256_ready:(if full_root_paths_supplied then sha_ready($full_root_artifact_summary_sha) else false end),
        full_root_readiness_ready:($full_root_readiness.status == "ready"),
        full_root_artifact_summary_ready:($full_root_artifact_summary.artifact_summary_ready == true),
        full_root_readiness_root_replay_ready:($full_root_readiness.local_root_report_replay_ready == true),
        full_root_artifact_summary_root_replay_ready:($full_root_artifact_summary.local_root_report_replay_ready == true),
        full_root_readiness_post_r228_fields_ready:artifact_post_r228_ready($full_root_readiness.artifact_summary),
        full_root_artifact_summary_post_r228_fields_ready:artifact_post_r228_ready($full_root_artifact_summary),
        backend_delivery_receipt_ready:backend_delivery_receipt_ready,
        backend_real_receipt_ready:backend_real_receipt_ready,
        release_artifact_roundtrip_ready:release_artifact_roundtrip_ready
      },
      claim_boundary:{
        local_post_r228_risk_future_plan_ready:$ready,
        local_control_ui_targeted_evidence_ready:current_required_fields_ready,
        full_product_root_risk_future_plan_ready:full_root_post_r228_wiring_ready,
        backend_delivery_claim_ready:backend_delivery_receipt_ready,
        real_backend_receipt_claim_ready:backend_real_receipt_ready,
        backend_receipt_claim_ready:backend_real_receipt_ready,
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
        local_markdown_written:true,
        local_report_written:true,
        backend_agent_spawned:false,
        backend_repo_write:false,
        matrix_login:false,
        gateway_call:false,
        provider_invoked:false,
        channel_delivery:false,
        credential_value_read:false,
        notary_submission_performed:false,
        app_signed:false,
        app_notarized:false,
        app_stapled:false,
        public_distribution_artifact_written:false,
        external_mutation:false
      }
    }' >"$REPORT_DRAFT"

jq -r '
  def plan_lines:
    (.latest_plan | map("- P\(.priority) `\(.id)` (\(.owner_lane)): \(.action)\n") | join(""));
  def blocker_lines:
    (.critical_blockers | map("- `\(.id)` (\(.owner_lane)): \(.state)\n") | join(""));
  def next_lines:
    (.next_unblock_sequence | map("- \(.)\n") | join(""));
  "# Hepta UI Post-r228 Risk / Future Plan\n\n"
  + "- Status: \(.status)\n"
  + "- Latest minimum gate: \(.latest_minimum_gate.gate_id)\n"
  + "- Evidence mode: \(.latest_minimum_gate.current_evidence_mode)\n"
  + "- Current artifact evidence ready: \(.latest_minimum_gate.current_artifact_evidence_ready)\n"
  + "- Old evidence rejected: \(.latest_minimum_gate.old_evidence_rejected)\n"
  + "- Four viewport evidence ready: \(.latest_minimum_gate.four_viewport_ready)\n"
  + "- Critical blockers: \(.critical_blocker_count)\n\n"
  + "## Latest Plan\n\n"
  + plan_lines
  + "\n## Critical Blockers\n\n"
  + blocker_lines
  + "\n## Next Unblock Sequence\n\n"
  + next_lines
' "$REPORT_DRAFT" >"$MARKDOWN_TMP"

markdown_sha="$(file_sha256 "$MARKDOWN_TMP")"
markdown_bytes="$(file_bytes "$MARKDOWN_TMP")"

jq \
  --arg markdown_sha "$markdown_sha" \
  --argjson markdown_bytes "$markdown_bytes" \
  '. + {risk_plan_markdown_sha256:$markdown_sha, risk_plan_markdown_bytes:$markdown_bytes}' \
  "$REPORT_DRAFT" >"$REPORT_TMP"

jq -e '
  .status == "ready"
  and .risk_future_plan_post_r228_gate_ready == true
  and .plan_kind == "local_ui_post_r228_command_palette_item_prismatic_rim_risk_future_plan_refresh"
  and .plan_version == 1
  and .latest_minimum_gate.gate_id == "r228_command_palette_item_prismatic_rim_light_glass_minimum_ui_demo_gate"
  and .latest_minimum_gate.current_artifact_evidence_ready == true
  and .latest_minimum_gate.current_evidence_mode == "targeted_control_ui_browser_four_viewport"
  and .latest_minimum_gate.old_evidence_rejected == true
  and .latest_minimum_gate.four_viewport_ready == true
  and .latest_minimum_gate.control_ui_harsh_2026_ready == true
  and .latest_minimum_gate.required_top_level_field_count == 20
  and .latest_minimum_gate.required_density_field_count == 19
  and .latest_minimum_gate.command_palette_item_prismatic_rim_ready == true
  and .latest_minimum_gate.command_palette_item_prismatic_rim_detail_count >= 4
  and .latest_plan_count == 6
  and .latest_plan_ids == ["r228_command_palette_item_prismatic_rim_light_glass_minimum_ui_demo_gate","full_root_risk_future_plan_wiring_post_r228","backend_delivery_receipt_return","backend_real_receipt_return","ui_refresh_after_real_receipt","release_artifact_roundtrip_and_signed_artifact_gate"]
  and .critical_blocker_count == (.critical_blockers | length)
  and (
    if .claim_boundary.full_product_root_risk_future_plan_ready then
      (.critical_blockers | map(.id) | index("full_root_risk_future_plan_wiring_post_r228") | not)
      and (.next_unblock_sequence | length) >= 1
      and (.next_unblock_sequence | length) <= 4
    else
      (.critical_blockers | map(.id) | index("full_root_risk_future_plan_wiring_post_r228") != null)
      and (.next_unblock_sequence | length) == 4
    end
  )
  and .source_alignment.post_r228_command_palette_required_ready == true
  and .source_alignment.old_r227_evidence_rejected == true
  and .source_alignment.four_viewport_ready == true
  and .source_alignment.full_root_post_r228_wiring_ready == .claim_boundary.full_product_root_risk_future_plan_ready
  and .source_alignment.backend_delivery_receipt_ready == .claim_boundary.backend_delivery_claim_ready
  and .source_alignment.backend_real_receipt_ready == .claim_boundary.backend_receipt_claim_ready
  and (.source_alignment.release_artifact_roundtrip_ready | type) == "boolean"
  and .claim_boundary.local_post_r228_risk_future_plan_ready == true
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and .side_effects.backend_repo_write == false
  and .side_effects.external_mutation == false
  and (.risk_plan_markdown_sha256 | test("^[0-9a-f]{64}$"))
  and .risk_plan_markdown_bytes > 0
' "$REPORT_TMP" >/dev/null

cp "$MARKDOWN_TMP" "$RISK_PLAN_MARKDOWN_PATH"
cp "$REPORT_TMP" "$REPORT_PATH"
cat "$REPORT_TMP"
