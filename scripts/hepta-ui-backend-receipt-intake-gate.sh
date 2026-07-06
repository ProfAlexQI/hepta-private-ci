#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_BACKEND_RECEIPT_INTAKE_REPORT_PATH:-$READINESS_DIR/ui-backend-receipt-intake-gate.json}"
INTAKE_DIR="${HEPTA_UI_BACKEND_RECEIPT_INTAKE_DIR:-$READINESS_DIR/backend-receipt-intake}"
RECEIPT_INPUT_PATH="${HEPTA_UI_BACKEND_RECEIPT_INPUT_PATH:-}"
TEMPLATE_PATH="$INTAKE_DIR/backend-receipt-template.json"
MARKDOWN_PATH="$INTAKE_DIR/backend-receipt-intake.md"
ACCEPTED_RECEIPT_INPUT_PATH="$INTAKE_DIR/backend-receipt-input.accepted.json"

BACKEND_DISPATCH_PACKET_REPORT_PATH="${HEPTA_UI_BACKEND_DISPATCH_PACKET_REPORT_PATH:-$READINESS_DIR/ui-backend-dispatch-packet-gate.json}"
BACKEND_DISPATCH_PACKET_DIR="${HEPTA_UI_BACKEND_DISPATCH_PACKET_DIR:-$READINESS_DIR/backend-dispatch-packet}"
BACKEND_DISPATCH_PACKET_MANIFEST_PATH="$BACKEND_DISPATCH_PACKET_DIR/backend-dispatch-packet-manifest.json"
BACKEND_DISPATCH_PACKET_ARCHIVE_PATH="$BACKEND_DISPATCH_PACKET_DIR/backend-dispatch-packet.tar.gz"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf '%s is required for the Hepta UI backend receipt intake gate\n' "$1" >&2
    exit 2
  fi
}

require_report() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required backend receipt intake input: %s\n' "$path" >&2
    exit 1
  fi
  jq empty "$path" >/dev/null
}

require_file() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required backend receipt intake file: %s\n' "$path" >&2
    exit 1
  fi
}

file_sha256() {
  shasum -a 256 "$1" | awk '{print $1}'
}

file_bytes() {
  wc -c <"$1" | tr -d ' '
}

require_command jq
require_command shasum

require_report "$BACKEND_DISPATCH_PACKET_REPORT_PATH"
require_report "$BACKEND_DISPATCH_PACKET_MANIFEST_PATH"
require_file "$BACKEND_DISPATCH_PACKET_ARCHIVE_PATH"

mkdir -p "$INTAKE_DIR"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-backend-receipt-intake.XXXXXX")"
REPORT_TMP="$TMP_DIR/backend-receipt-intake-report.json"
RECEIPT_CAPTURE_PATH="$TMP_DIR/backend-receipt-input.json"
trap 'rm -rf "$TMP_DIR"' EXIT

receipt_present=false
receipt_input_path_json=null
receipt_captured_input_path_json=null
receipt_sha_json=null
receipt_bytes=0

if [[ -n "$RECEIPT_INPUT_PATH" ]]; then
  require_report "$RECEIPT_INPUT_PATH"
  cp "$RECEIPT_INPUT_PATH" "$RECEIPT_CAPTURE_PATH"
  cp "$RECEIPT_INPUT_PATH" "$ACCEPTED_RECEIPT_INPUT_PATH"
  receipt_present=true
  receipt_input_path_json="$(jq -n --arg path "$RECEIPT_INPUT_PATH" '$path')"
  receipt_captured_input_path_json="$(jq -n --arg path "$ACCEPTED_RECEIPT_INPUT_PATH" '$path')"
  receipt_sha_json="$(jq -n --arg sha "$(file_sha256 "$RECEIPT_INPUT_PATH")" '$sha')"
  receipt_bytes="$(file_bytes "$RECEIPT_INPUT_PATH")"
else
  rm -f "$ACCEPTED_RECEIPT_INPUT_PATH"
  jq -n '{present:false}' >"$RECEIPT_CAPTURE_PATH"
fi

dispatch_report_sha="$(file_sha256 "$BACKEND_DISPATCH_PACKET_REPORT_PATH")"
dispatch_manifest_sha="$(file_sha256 "$BACKEND_DISPATCH_PACKET_MANIFEST_PATH")"
dispatch_archive_sha="$(file_sha256 "$BACKEND_DISPATCH_PACKET_ARCHIVE_PATH")"
dispatch_archive_bytes="$(file_bytes "$BACKEND_DISPATCH_PACKET_ARCHIVE_PATH")"

jq -n \
  --slurpfile dispatch_file "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  '
  ($dispatch_file[0]) as $dispatch
  | {
      receipt_kind:"backend_contract_execution_receipt",
      receipt_version:1,
      backend_target_repo:$dispatch.backend_lane_target.target_repo,
      owner_lane:"backend_contract",
      dispatch_packet_archive_sha256:$dispatch.archive_sha256,
      selected_receipt_ids:$dispatch.selected_packet_ids,
      required_receipt_item_fields:[
        "id",
        "backend_adapter_contract_recorded",
        "operation_id",
        "source_hash",
        "readback_evidence_recorded",
        "retry_cancel_idempotency_policy_recorded",
        "stale_target_guard_recorded",
        "side_effect_review_recorded"
      ],
      receipt_items:($dispatch.selected_packet_ids | map({
        id:.,
        backend_adapter_contract_recorded:false,
        operation_id:"",
        source_hash:"",
        readback_evidence_recorded:false,
        retry_cancel_idempotency_policy_recorded:false,
        stale_target_guard_recorded:false,
        side_effect_review_recorded:false
      })),
      refreshed_ui_readiness_required:{
        no_window_command:$dispatch.hepta_ui_after_backend.required_refresh_commands[0],
        full_hard_command:$dispatch.hepta_ui_after_backend.required_refresh_commands[1]
      },
      claim_boundary:{
        backend_receipt_claim_ready:false,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false
      },
      side_effects:{
        external_mutation:false,
        matrix_login:false,
        gateway_call:false,
        provider_invoked:false,
        channel_delivery:false
      }
    }' >"$TEMPLATE_PATH"

jq -r '
  "# Hepta UI Backend Receipt Intake\n\n"
  + "- Kind: local backend receipt intake contract\n"
  + "- Dispatch archive SHA: \(.dispatch_packet_archive_sha256)\n"
  + "- Selected ids: \(.selected_receipt_ids | join(", "))\n"
  + "- Receipt input env: `HEPTA_UI_BACKEND_RECEIPT_INPUT_PATH`\n"
  + "- Claim boundary: receipt intake does not make live product, release, or public distribution ready.\n\n"
  + "## Required Receipt Fields\n\n"
  + (.required_receipt_item_fields | map("- `\(.)`") | join("\n"))
  + "\n\n## UI Refresh Commands After Backend Receipt\n\n"
  + "- `\(.refreshed_ui_readiness_required.no_window_command)`\n"
  + "- `\(.refreshed_ui_readiness_required.full_hard_command)`\n"
' "$TEMPLATE_PATH" >"$MARKDOWN_PATH"

template_sha="$(file_sha256 "$TEMPLATE_PATH")"
template_bytes="$(file_bytes "$TEMPLATE_PATH")"
markdown_sha="$(file_sha256 "$MARKDOWN_PATH")"
markdown_bytes="$(file_bytes "$MARKDOWN_PATH")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_backend_receipt_intake_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg intake_dir "$INTAKE_DIR" \
  --arg template_path "$TEMPLATE_PATH" \
  --arg markdown_path "$MARKDOWN_PATH" \
  --arg dispatch_report_path "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  --arg dispatch_manifest_path "$BACKEND_DISPATCH_PACKET_MANIFEST_PATH" \
  --arg dispatch_archive_path "$BACKEND_DISPATCH_PACKET_ARCHIVE_PATH" \
  --arg dispatch_report_sha "$dispatch_report_sha" \
  --arg dispatch_manifest_sha "$dispatch_manifest_sha" \
  --arg dispatch_archive_sha "$dispatch_archive_sha" \
  --arg template_sha "$template_sha" \
  --arg markdown_sha "$markdown_sha" \
  --argjson dispatch_archive_bytes "$dispatch_archive_bytes" \
  --argjson template_bytes "$template_bytes" \
  --argjson markdown_bytes "$markdown_bytes" \
  --argjson receipt_present "$receipt_present" \
  --argjson receipt_input_path "$receipt_input_path_json" \
  --argjson receipt_captured_input_path "$receipt_captured_input_path_json" \
  --argjson receipt_sha "$receipt_sha_json" \
  --argjson receipt_bytes "$receipt_bytes" \
  --slurpfile dispatch_file "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  --slurpfile manifest_file "$BACKEND_DISPATCH_PACKET_MANIFEST_PATH" \
  --slurpfile template_file "$TEMPLATE_PATH" \
  --slurpfile receipt_file "$RECEIPT_CAPTURE_PATH" \
  '
  ($dispatch_file[0]) as $dispatch
  | ($manifest_file[0]) as $manifest
  | ($template_file[0]) as $template
  | ($receipt_file[0]) as $receipt
  | def selected_ids: ["message_search","file_upload_send","media_download_playback","notifications","room_settings"];
    def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
    def source_chain_ready:
      $dispatch.backend_dispatch_packet_gate_ready == true
      and $dispatch.status == "ready"
      and $dispatch.packet_kind == "local_backend_dispatch_packet"
      and $dispatch.packet_version == 1
      and $dispatch.selected_packet_ids == selected_ids
      and $dispatch.packet_item_count == 5
      and $dispatch.packet_ready_count == 5
      and $dispatch.payload_file_count == 8
      and $dispatch.all_extracted_files_sha256_match == true
      and $dispatch.archive_sha256 == $dispatch_archive_sha
      and $dispatch.archive_bytes == $dispatch_archive_bytes
      and $dispatch.backend_lane_target.target_repo == "/Users/qianqi/.openclaw/workspace/Hepta"
      and $dispatch.backend_lane_target.owner_lane == "backend_contract"
      and ($dispatch.hepta_ui_after_backend.required_refresh_commands | length) == 2
      and $dispatch.claim_boundary.local_backend_dispatch_packet_ready == true
      and $dispatch.claim_boundary.backend_adapter_promoted == false
      and $dispatch.claim_boundary.readback_evidence_recorded == false
      and $dispatch.claim_boundary.live_runtime_mutation == false
      and $dispatch.claim_boundary.live_product_claim_ready == false
      and $dispatch.claim_boundary.public_distribution_claim_ready == false
      and $dispatch.claim_boundary.release_claim_ready == false
      and $dispatch.side_effects.external_mutation == false
      and $manifest.packet_kind == "local_backend_dispatch_packet"
      and $manifest.payload_file_count == 8
      and sha_ready($dispatch_report_sha)
      and sha_ready($dispatch_manifest_sha)
      and sha_ready($dispatch_archive_sha);
    def intake_contract_ready:
      $template.receipt_kind == "backend_contract_execution_receipt"
      and $template.receipt_version == 1
      and $template.backend_target_repo == "/Users/qianqi/.openclaw/workspace/Hepta"
      and $template.owner_lane == "backend_contract"
      and $template.dispatch_packet_archive_sha256 == $dispatch.archive_sha256
      and $template.selected_receipt_ids == selected_ids
      and ($template.receipt_items | length) == 5
      and ($template.receipt_items | map(.id)) == selected_ids
      and ($template.required_receipt_item_fields | length) == 8
      and sha_ready($template_sha)
      and $template_bytes > 0
      and sha_ready($markdown_sha)
      and $markdown_bytes > 0;
    def receipt_present_ready:
      $receipt_present == true
      and $receipt.receipt_kind == "backend_contract_execution_receipt"
      and $receipt.receipt_version == 1
      and $receipt.backend_target_repo == "/Users/qianqi/.openclaw/workspace/Hepta"
      and $receipt.dispatch_packet_archive_sha256 == $dispatch.archive_sha256
      and $receipt.selected_receipt_ids == selected_ids
      and ($receipt.receipt_items | length) == 5
      and ($receipt.receipt_items | map(.id)) == selected_ids
      and ($receipt.receipt_items | all(.backend_adapter_contract_recorded == true))
      and ($receipt.receipt_items | all((.operation_id // "") | length > 0))
      and ($receipt.receipt_items | all((.source_hash // "") | test("^[0-9a-f]{64}$")))
      and ($receipt.receipt_items | all(.readback_evidence_recorded == true))
      and ($receipt.receipt_items | all(.retry_cancel_idempotency_policy_recorded == true))
      and ($receipt.receipt_items | all(.stale_target_guard_recorded == true))
      and ($receipt.receipt_items | all(.side_effect_review_recorded == true))
      and ($receipt.claim_boundary.live_product_claim_ready // false) == false
      and ($receipt.claim_boundary.public_distribution_claim_ready // false) == false
      and ($receipt.claim_boundary.release_claim_ready // false) == false
      and ($receipt.side_effects.external_mutation // false) == false
      and sha_ready($receipt_sha)
      and $receipt_bytes > 0;
    def receipt_claim($name):
      if $receipt_present then (($receipt.claim_boundary[$name] // false) == true) else false end;
    (source_chain_ready and intake_contract_ready and (($receipt_present == false) or receipt_present_ready)) as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      backend_receipt_intake_gate_ready:$ready,
      intake_kind:"local_backend_receipt_intake_contract",
      intake_version:1,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      intake_dir:$intake_dir,
      template_path:$template_path,
      template_sha256:$template_sha,
      template_bytes:$template_bytes,
      markdown_path:$markdown_path,
      markdown_sha256:$markdown_sha,
      markdown_bytes:$markdown_bytes,
      source_reports:{
        backend_dispatch_packet:$dispatch_report_path,
        backend_dispatch_packet_manifest:$dispatch_manifest_path,
        backend_dispatch_packet_archive:$dispatch_archive_path
      },
      source_report_sha256:{
        backend_dispatch_packet:$dispatch_report_sha,
        backend_dispatch_packet_manifest:$dispatch_manifest_sha,
        backend_dispatch_packet_archive:$dispatch_archive_sha
      },
      dispatch_packet_archive_sha256:$dispatch.archive_sha256,
      dispatch_packet_archive_bytes:$dispatch.archive_bytes,
      selected_receipt_ids:selected_ids,
      receipt_item_count:(if $receipt_present then ($receipt.receipt_items | length) else 0 end),
      receipt_ready_count:(if $receipt_present then ($receipt.receipt_items | map(select(
        .backend_adapter_contract_recorded == true
        and ((.operation_id // "") | length > 0)
        and ((.source_hash // "") | test("^[0-9a-f]{64}$"))
        and .readback_evidence_recorded == true
        and .retry_cancel_idempotency_policy_recorded == true
        and .stale_target_guard_recorded == true
        and .side_effect_review_recorded == true
      )) | length) else 0 end),
      backend_receipt_present:$receipt_present,
      backend_receipt_valid:(if $receipt_present then receipt_present_ready else false end),
      waiting_for_backend_receipt:($receipt_present == false),
      receipt_input_path:$receipt_input_path,
      receipt_captured_input_path:$receipt_captured_input_path,
      receipt_input_sha256:$receipt_sha,
      receipt_input_bytes:$receipt_bytes,
      required_receipt_item_fields:$template.required_receipt_item_fields,
      required_ui_refresh_commands:$dispatch.hepta_ui_after_backend.required_refresh_commands,
      template:$template,
      receipt_preview:(if $receipt_present then $receipt else null end),
      claim_boundary:{
        local_backend_receipt_intake_ready:$ready,
        local_backend_dispatch_packet_ready:$dispatch.claim_boundary.local_backend_dispatch_packet_ready,
        backend_receipt_claim_ready:(if $receipt_present then receipt_present_ready else false end),
        backend_adapter_promoted:receipt_claim("backend_adapter_promoted"),
        readback_evidence_recorded:(if $receipt_present then (receipt_present_ready and (($receipt.claim_boundary.readback_evidence_recorded // true) == true)) else false end),
        side_effect_review_recorded:(if $receipt_present then (receipt_present_ready and (($receipt.claim_boundary.side_effect_review_recorded // true) == true)) else false end),
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
        local_template_written:true,
        local_markdown_written:true,
        backend_receipt_file_read:$receipt_present,
        matrix_login:false,
        gateway_call:false,
        provider_invoked:false,
        channel_delivery:false,
        live_runtime_mutation:false,
        external_mutation:false
      }
    }' >"$REPORT_TMP"

jq -e '
  .status == "ready"
  and .backend_receipt_intake_gate_ready == true
  and .intake_kind == "local_backend_receipt_intake_contract"
  and .intake_version == 1
  and .selected_receipt_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and (.template_sha256 | test("^[0-9a-f]{64}$"))
  and .template_bytes > 0
  and (.markdown_sha256 | test("^[0-9a-f]{64}$"))
  and .markdown_bytes > 0
  and (.dispatch_packet_archive_sha256 | test("^[0-9a-f]{64}$"))
  and .dispatch_packet_archive_bytes > 0
  and (.required_receipt_item_fields | length) == 8
  and (.required_ui_refresh_commands | length) == 2
  and (
    (
      .backend_receipt_present == false
      and .waiting_for_backend_receipt == true
      and .backend_receipt_valid == false
      and .receipt_item_count == 0
      and .receipt_ready_count == 0
    )
    or (
      .backend_receipt_present == true
      and .waiting_for_backend_receipt == false
      and .backend_receipt_valid == true
      and .receipt_item_count == 5
      and .receipt_ready_count == 5
      and (.receipt_input_sha256 | test("^[0-9a-f]{64}$"))
      and .receipt_input_bytes > 0
    )
  )
  and .claim_boundary.local_backend_receipt_intake_ready == true
  and .claim_boundary.local_backend_dispatch_packet_ready == true
  and .claim_boundary.live_runtime_mutation == false
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and .side_effects.local_template_written == true
  and .side_effects.local_markdown_written == true
  and .side_effects.live_runtime_mutation == false
  and .side_effects.external_mutation == false
' "$REPORT_TMP" >/dev/null

mkdir -p "$(dirname "$REPORT_PATH")"
cp "$REPORT_TMP" "$REPORT_PATH"
cat "$REPORT_TMP"
