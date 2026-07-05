#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_OPERATOR_BRIEFING_REFRESH_REPORT_PATH:-$READINESS_DIR/ui-operator-briefing-refresh-gate.json}"
REFRESH_DIR="${HEPTA_UI_OPERATOR_BRIEFING_REFRESH_DIR:-$READINESS_DIR/operator-briefing-refresh}"
REFRESH_MARKDOWN_PATH="$REFRESH_DIR/operator-briefing-refresh.md"

OPERATOR_BRIEFING_REPORT_PATH="$READINESS_DIR/ui-operator-briefing-gate.json"
FUTURE_PLAN_REFRESH_REPORT_PATH="$READINESS_DIR/ui-future-plan-refresh-gate.json"
BACKEND_DISPATCH_PACKET_REPORT_PATH="$READINESS_DIR/ui-backend-dispatch-packet-gate.json"
BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH="$READINESS_DIR/ui-backend-receipt-refresh-lock-gate.json"
RELEASE_OPERATOR_DRY_RUN_REPORT_PATH="$READINESS_DIR/ui-release-operator-dry-run-gate.json"
EVIDENCE_ARCHIVE_REPORT_PATH="$READINESS_DIR/ui-evidence-archive-gate.json"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf '%s is required for the Hepta UI operator briefing refresh gate\n' "$1" >&2
    exit 2
  fi
}

require_report() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required operator-briefing refresh input: %s\n' "$path" >&2
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

require_report "$OPERATOR_BRIEFING_REPORT_PATH"
require_report "$FUTURE_PLAN_REFRESH_REPORT_PATH"
require_report "$BACKEND_DISPATCH_PACKET_REPORT_PATH"
require_report "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH"
require_report "$RELEASE_OPERATOR_DRY_RUN_REPORT_PATH"
require_report "$EVIDENCE_ARCHIVE_REPORT_PATH"

rm -rf "$REFRESH_DIR"
mkdir -p "$REFRESH_DIR"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-operator-briefing-refresh.XXXXXX")"
REPORT_DRAFT="$TMP_DIR/operator-briefing-refresh-draft.json"
REPORT_TMP="$TMP_DIR/operator-briefing-refresh-report.json"
MARKDOWN_TMP="$TMP_DIR/operator-briefing-refresh.md"
trap 'rm -rf "$TMP_DIR"' EXIT

operator_briefing_sha="$(file_sha256 "$OPERATOR_BRIEFING_REPORT_PATH")"
future_plan_sha="$(file_sha256 "$FUTURE_PLAN_REFRESH_REPORT_PATH")"
backend_dispatch_sha="$(file_sha256 "$BACKEND_DISPATCH_PACKET_REPORT_PATH")"
backend_receipt_refresh_lock_sha="$(file_sha256 "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH")"
release_operator_sha="$(file_sha256 "$RELEASE_OPERATOR_DRY_RUN_REPORT_PATH")"
evidence_archive_sha="$(file_sha256 "$EVIDENCE_ARCHIVE_REPORT_PATH")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_operator_briefing_refresh_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg refresh_dir "$REFRESH_DIR" \
  --arg refresh_markdown_path "$REFRESH_MARKDOWN_PATH" \
  --arg operator_briefing_path "$OPERATOR_BRIEFING_REPORT_PATH" \
  --arg future_plan_path "$FUTURE_PLAN_REFRESH_REPORT_PATH" \
  --arg backend_dispatch_path "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  --arg backend_receipt_refresh_lock_path "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH" \
  --arg release_operator_path "$RELEASE_OPERATOR_DRY_RUN_REPORT_PATH" \
  --arg evidence_archive_path "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  --arg operator_briefing_sha "$operator_briefing_sha" \
  --arg future_plan_sha "$future_plan_sha" \
  --arg backend_dispatch_sha "$backend_dispatch_sha" \
  --arg backend_receipt_refresh_lock_sha "$backend_receipt_refresh_lock_sha" \
  --arg release_operator_sha "$release_operator_sha" \
  --arg evidence_archive_sha "$evidence_archive_sha" \
  --slurpfile operator_briefing_file "$OPERATOR_BRIEFING_REPORT_PATH" \
  --slurpfile future_plan_file "$FUTURE_PLAN_REFRESH_REPORT_PATH" \
  --slurpfile backend_dispatch_file "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  --slurpfile backend_receipt_refresh_lock_file "$BACKEND_RECEIPT_REFRESH_LOCK_REPORT_PATH" \
  --slurpfile release_operator_file "$RELEASE_OPERATOR_DRY_RUN_REPORT_PATH" \
  --slurpfile evidence_archive_file "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  '
  ($operator_briefing_file[0]) as $operator
  | ($future_plan_file[0]) as $future
  | ($backend_dispatch_file[0]) as $dispatch
  | ($backend_receipt_refresh_lock_file[0]) as $refresh_lock
  | ($release_operator_file[0]) as $release_dry_run
  | ($evidence_archive_file[0]) as $archive
  | def selected_ids: ["message_search","file_upload_send","media_download_playback","notifications","room_settings"];
    def plan_ids: ["r52_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt"];
    def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
    def backend_receipt_claim_ready:
      $refresh_lock.claim_boundary.real_backend_receipt_claim_ready == true
      and $refresh_lock.claim_boundary.backend_receipt_claim_ready == true
      and $refresh_lock.refresh_requirements.full_hard_refresh_required == false
      and $refresh_lock.refresh_requirements.full_hard_refresh_ready == true
      and $future.backend_receipt_refresh_contract.backend_receipt_claim_ready == true;
    def updated_critical_risks:
      [
        (if backend_receipt_claim_ready then empty else {
          id:"real_backend_receipt_missing",
          severity:"blocker",
          owner_lane:"backend_contract",
          selected_ids:selected_ids
        } end),
        (if backend_receipt_claim_ready then empty else {
          id:"backend_contract_first_five_not_executed",
          severity:"blocker",
          owner_lane:"backend_contract",
          target_repo:$dispatch.backend_lane_target.target_repo,
          dispatch_archive_sha256:$dispatch.archive_sha256
        } end),
        (if backend_receipt_claim_ready then empty else {
          id:"ui_refresh_after_real_receipt_required",
          severity:"guardrail",
          owner_lane:"hepta-ui",
          required_commands:$refresh_lock.refresh_requirements.required_ui_refresh_commands
        } end),
        {
          id:"release_public_distribution_not_approved",
          severity:"blocker",
          owner_lane:"release_operator",
          operator_approval_recorded:$release_dry_run.operator_packet.operator_approval_recorded,
          public_distribution_artifact_written:$release_dry_run.operator_packet.public_distribution_artifact_written
        }
      ];
    def forbidden_claims:
      (
        (if backend_receipt_claim_ready then [] else ["real_backend_receipt_ready"] end)
        + ["live_product_ready","public_distribution_ready","release_ready"]
      );
    def source_chain_ready:
      $operator.operator_briefing_gate_ready == true
      and $operator.critical_risk_count == 3
      and $operator.backend_remaining_contract_count == 12
      and ($operator.answer_guardrail.forbidden_claims | index("live_product_ready") != null)
      and ($operator.answer_guardrail.forbidden_claims | index("public_distribution_ready") != null)
      and ($operator.answer_guardrail.forbidden_claims | index("release_ready") != null)
      and $future.future_plan_refresh_gate_ready == true
      and ($future.future_plan | map(.id)) == plan_ids
      and $future.r52_minimum_gate.root_report_replay_required_count == 32
      and $future.backend_receipt_refresh_contract.selected_ids == selected_ids
      and ($future.backend_receipt_refresh_contract.real_backend_receipt_present | type) == "boolean"
      and ($future.backend_receipt_refresh_contract.backend_receipt_claim_ready | type) == "boolean"
      and $future.backend_receipt_refresh_contract.simulated_branch_not_promoted == true
      and ($future.backend_receipt_refresh_contract.required_ui_refresh_commands | length) == 2
      and $dispatch.backend_dispatch_packet_gate_ready == true
      and $dispatch.selected_packet_ids == selected_ids
      and ($dispatch.archive_sha256 | test("^[0-9a-f]{64}$"))
      and $dispatch.archive_bytes > 0
      and $dispatch.claim_boundary.local_backend_dispatch_packet_ready == true
      and $dispatch.claim_boundary.backend_adapter_promoted == false
      and $dispatch.claim_boundary.readback_evidence_recorded == false
      and $dispatch.claim_boundary.live_runtime_mutation == false
      and $refresh_lock.backend_receipt_refresh_lock_gate_ready == true
      and $refresh_lock.selected_refresh_ids == selected_ids
      and $refresh_lock.receipt_state.real_backend_receipt_present == $future.backend_receipt_refresh_contract.real_backend_receipt_present
      and $refresh_lock.claim_boundary.real_backend_receipt_claim_ready == $future.claim_boundary.real_backend_receipt_claim_ready
      and $refresh_lock.claim_boundary.backend_receipt_claim_ready == $future.claim_boundary.backend_receipt_claim_ready
      and $refresh_lock.claim_boundary.live_product_claim_ready == false
      and $release_dry_run.release_operator_dry_run_gate_ready == true
      and $release_dry_run.operator_packet.operator_approval_recorded == false
      and $release_dry_run.operator_packet.public_distribution_artifact_written == false
      and $release_dry_run.claim_boundary.release_claim_ready == false
      and $archive.evidence_archive_gate_ready == true
      and $archive.claim_boundary.local_evidence_archive_ready == true
      and $archive.all_extracted_items_sha256_match == true
      and sha_ready($operator_briefing_sha)
      and sha_ready($future_plan_sha)
      and sha_ready($backend_dispatch_sha)
      and sha_ready($backend_receipt_refresh_lock_sha)
      and sha_ready($release_operator_sha)
      and sha_ready($evidence_archive_sha);
    source_chain_ready as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      operator_briefing_refresh_gate_ready:$ready,
      briefing_refresh_kind:"local_ui_operator_briefing_after_future_plan_refresh",
      briefing_refresh_version:1,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      refresh_dir:$refresh_dir,
      refresh_markdown_path:$refresh_markdown_path,
      source_reports:{
        operator_briefing:$operator_briefing_path,
        future_plan_refresh:$future_plan_path,
        backend_dispatch_packet:$backend_dispatch_path,
        backend_receipt_refresh_lock:$backend_receipt_refresh_lock_path,
        release_operator_dry_run:$release_operator_path,
        evidence_archive:$evidence_archive_path
      },
      source_report_sha256:{
        operator_briefing:$operator_briefing_sha,
        future_plan_refresh:$future_plan_sha,
        backend_dispatch_packet:$backend_dispatch_sha,
        backend_receipt_refresh_lock:$backend_receipt_refresh_lock_sha,
        release_operator_dry_run:$release_operator_sha,
        evidence_archive:$evidence_archive_sha
      },
      current_state:{
        prior_operator_briefing_ready:$operator.operator_briefing_gate_ready,
        prior_operator_briefing_risk_count:$operator.critical_risk_count,
        future_plan_refresh_ready:$future.future_plan_refresh_gate_ready,
        source_future_plan_root_report_required_count:$future.r52_minimum_gate.root_report_replay_required_count,
        root_report_replay_required_count_after_refresh:33,
        r52_minimum_full_hard_evidence_ready:$future.r52_minimum_gate.current_full_hard_evidence_ready,
        real_backend_receipt_present:$refresh_lock.receipt_state.real_backend_receipt_present,
        backend_receipt_claim_ready:$refresh_lock.claim_boundary.backend_receipt_claim_ready,
        backend_contract_remaining_count:$operator.backend_remaining_contract_count,
        release_operator_approval_recorded:$release_dry_run.operator_packet.operator_approval_recorded,
        release_claim_ready:$release_dry_run.claim_boundary.release_claim_ready,
        evidence_archive_sha256:$archive.archive_sha256,
        evidence_archive_bytes:$archive.archive_bytes
      },
      backend_dispatch_pointer:{
        selected_ids:selected_ids,
        target_repo:$dispatch.backend_lane_target.target_repo,
        archive_path:$dispatch.archive_path,
        archive_sha256:$dispatch.archive_sha256,
        archive_bytes:$dispatch.archive_bytes,
        packet_markdown_path:$dispatch.packet_markdown_path,
        backend_adapter_promoted:$dispatch.claim_boundary.backend_adapter_promoted,
        readback_evidence_recorded:$dispatch.claim_boundary.readback_evidence_recorded,
        live_runtime_mutation:$dispatch.claim_boundary.live_runtime_mutation
      },
      receipt_refresh_pointer:{
        selected_ids:selected_ids,
        real_backend_receipt_present:$refresh_lock.receipt_state.real_backend_receipt_present,
        simulated_receipt_input_present:$refresh_lock.receipt_state.simulated_receipt_input_present,
        simulated_branch_not_promoted:$refresh_lock.misclaim_lock.simulated_receipt_not_promoted_to_backend_receipt,
        hard_true_window_refresh_ready:$refresh_lock.refresh_requirements.hard_true_window_refresh_ready,
        full_hard_refresh_required:$refresh_lock.refresh_requirements.full_hard_refresh_required,
        required_ui_refresh_commands:$refresh_lock.refresh_requirements.required_ui_refresh_commands
      },
      refreshed_operator_briefing:{
        current_plan_source:"ui-future-plan-refresh-gate.json",
        current_plan_ids:($future.future_plan | map(.id)),
        next_plan:$future.future_plan,
        updated_critical_risks:updated_critical_risks
      },
      updated_critical_risk_count:(updated_critical_risks | length),
      current_next_plan_ids:($future.future_plan | map(.id)),
      current_next_plan:$future.future_plan,
      source_alignment:{
        operator_briefing_ready:$operator.operator_briefing_gate_ready,
        future_plan_refresh_ready:$future.future_plan_refresh_gate_ready,
        backend_dispatch_packet_ready:$dispatch.backend_dispatch_packet_gate_ready,
        backend_receipt_refresh_lock_ready:$refresh_lock.backend_receipt_refresh_lock_gate_ready,
        release_operator_dry_run_ready:$release_dry_run.release_operator_dry_run_gate_ready,
        evidence_archive_ready:$archive.evidence_archive_gate_ready,
        selected_ids_match:(
          $dispatch.selected_packet_ids == selected_ids
          and $refresh_lock.selected_refresh_ids == selected_ids
          and $future.backend_receipt_refresh_contract.selected_ids == selected_ids
        ),
        plan_ids_match:(($future.future_plan | map(.id)) == plan_ids),
        real_receipt_state_match:(
          $future.backend_receipt_refresh_contract.real_backend_receipt_present
          == $refresh_lock.receipt_state.real_backend_receipt_present
        )
      },
      answer_guardrail:{
        allowed_summary:"current local UI readiness, backend dispatch packet pointer, receipt waiting state, and release dry-run denial state",
        forbidden_claims:forbidden_claims,
        closeout_required_fields:["artifact_path","root_report_replay_count","updated_critical_risk_count","dispatch_archive_sha256","backend_receipt_full_hard_refresh_required"]
      },
      claim_boundary:{
        local_operator_briefing_refresh_ready:$ready,
        local_future_plan_refresh_ready:$future.claim_boundary.local_future_plan_refresh_ready,
        local_backend_receipt_refresh_lock_ready:$refresh_lock.claim_boundary.local_backend_receipt_refresh_lock_ready,
        real_backend_receipt_claim_ready:$refresh_lock.claim_boundary.real_backend_receipt_claim_ready,
        backend_receipt_claim_ready:$refresh_lock.claim_boundary.backend_receipt_claim_ready,
        backend_adapter_promoted:false,
        readback_evidence_recorded:false,
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
        external_mutation:false
      }
    }' >"$REPORT_DRAFT"

jq -r '
  "# Hepta UI Operator Briefing Refresh\n\n"
  + "- Kind: \(.briefing_refresh_kind)\n"
  + "- Prior operator briefing risk count: \(.current_state.prior_operator_briefing_risk_count)\n"
  + "- Updated critical risk count: \(.updated_critical_risk_count)\n"
  + "- Current plan ids: \(.current_next_plan_ids | join(","))\n"
  + "- Real backend receipt present: \(.current_state.real_backend_receipt_present)\n"
  + "- Backend dispatch archive: \(.backend_dispatch_pointer.archive_sha256)\n"
  + "- Root replay required after this refresh: \(.current_state.root_report_replay_required_count_after_refresh)\n"
  + "- Claim boundary: real_backend_receipt=\(.claim_boundary.real_backend_receipt_claim_ready); backend_adapter=false; live_product=false; public_distribution=false; release=false\n\n"
  + "## Current Critical Risks\n\n"
  + (.refreshed_operator_briefing.updated_critical_risks
      | map("- `\(.id)` \(.severity) owner=\(.owner_lane)")
      | join("\n"))
  + "\n\n## Next Plan\n\n"
  + (.current_next_plan
      | map("- \(.priority). `\(.id)` owner=\(.owner_lane)")
      | join("\n"))
  + "\n"
' "$REPORT_DRAFT" >"$MARKDOWN_TMP"

cp "$MARKDOWN_TMP" "$REFRESH_MARKDOWN_PATH"
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
  and .operator_briefing_refresh_gate_ready == true
  and .briefing_refresh_kind == "local_ui_operator_briefing_after_future_plan_refresh"
  and .briefing_refresh_version == 1
  and .current_state.prior_operator_briefing_risk_count == 3
  and .current_state.source_future_plan_root_report_required_count == 32
  and .current_state.root_report_replay_required_count_after_refresh == 33
  and (.current_state.real_backend_receipt_present | type) == "boolean"
  and (.current_state.backend_receipt_claim_ready | type) == "boolean"
  and .updated_critical_risk_count == (.refreshed_operator_briefing.updated_critical_risks | length)
  and (.updated_critical_risk_count >= 1 and .updated_critical_risk_count <= 4)
  and .current_next_plan_ids == ["r52_minimum_ui_demo_gate","backend_real_receipt_return","ui_refresh_after_real_receipt"]
  and .backend_dispatch_pointer.selected_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and (.backend_dispatch_pointer.archive_sha256 | test("^[0-9a-f]{64}$"))
  and .backend_dispatch_pointer.archive_bytes > 0
  and .backend_dispatch_pointer.backend_adapter_promoted == false
  and .backend_dispatch_pointer.readback_evidence_recorded == false
  and (.receipt_refresh_pointer.real_backend_receipt_present | type) == "boolean"
  and .receipt_refresh_pointer.simulated_branch_not_promoted == true
  and (.receipt_refresh_pointer.required_ui_refresh_commands | length) == 2
  and .source_alignment.operator_briefing_ready == true
  and .source_alignment.future_plan_refresh_ready == true
  and .source_alignment.backend_dispatch_packet_ready == true
  and .source_alignment.backend_receipt_refresh_lock_ready == true
  and .source_alignment.selected_ids_match == true
  and .source_alignment.plan_ids_match == true
  and .source_alignment.real_receipt_state_match == true
  and (
    (
      .current_state.backend_receipt_claim_ready == true
      and (.answer_guardrail.forbidden_claims | index("real_backend_receipt_ready") == null)
    )
    or
    (
      .current_state.backend_receipt_claim_ready == false
      and (.answer_guardrail.forbidden_claims | index("real_backend_receipt_ready") != null)
    )
  )
  and (.answer_guardrail.forbidden_claims | index("live_product_ready") != null)
  and (.refresh_markdown_sha256 | test("^[0-9a-f]{64}$"))
  and .refresh_markdown_bytes > 0
  and .claim_boundary.local_operator_briefing_refresh_ready == true
  and .claim_boundary.real_backend_receipt_claim_ready == .current_state.backend_receipt_claim_ready
  and .claim_boundary.backend_receipt_claim_ready == .current_state.backend_receipt_claim_ready
  and .claim_boundary.backend_adapter_promoted == false
  and .claim_boundary.readback_evidence_recorded == false
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

mkdir -p "$(dirname "$REPORT_PATH")"
cp "$REPORT_TMP" "$REPORT_PATH"
cat "$REPORT_TMP"
