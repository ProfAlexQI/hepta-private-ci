#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_BACKEND_RECEIPT_ROUNDTRIP_REPORT_PATH:-$READINESS_DIR/ui-backend-receipt-roundtrip-gate.json}"
ROUNDTRIP_DIR="${HEPTA_UI_BACKEND_RECEIPT_ROUNDTRIP_DIR:-$READINESS_DIR/backend-receipt-roundtrip}"
SIMULATED_RECEIPT_PATH="$ROUNDTRIP_DIR/simulated-backend-receipt.json"
SIMULATED_INTAKE_DIR="$ROUNDTRIP_DIR/simulated-intake"
SIMULATED_INTAKE_REPORT_PATH="$ROUNDTRIP_DIR/ui-backend-receipt-intake-present-gate.json"

BACKEND_RECEIPT_INTAKE_REPORT_PATH="${HEPTA_UI_BACKEND_RECEIPT_INTAKE_REPORT_PATH:-$READINESS_DIR/ui-backend-receipt-intake-gate.json}"
BACKEND_DISPATCH_PACKET_REPORT_PATH="${HEPTA_UI_BACKEND_DISPATCH_PACKET_REPORT_PATH:-$READINESS_DIR/ui-backend-dispatch-packet-gate.json}"
BACKEND_DISPATCH_PACKET_DIR="${HEPTA_UI_BACKEND_DISPATCH_PACKET_DIR:-$READINESS_DIR/backend-dispatch-packet}"
BACKEND_DISPATCH_PACKET_MANIFEST_PATH="$BACKEND_DISPATCH_PACKET_DIR/backend-dispatch-packet-manifest.json"
BACKEND_DISPATCH_PACKET_ARCHIVE_PATH="$BACKEND_DISPATCH_PACKET_DIR/backend-dispatch-packet.tar.gz"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf '%s is required for the Hepta UI backend receipt roundtrip gate\n' "$1" >&2
    exit 2
  fi
}

require_report() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required backend receipt roundtrip input: %s\n' "$path" >&2
    exit 1
  fi
  jq empty "$path" >/dev/null
}

require_file() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required backend receipt roundtrip file: %s\n' "$path" >&2
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

require_report "$BACKEND_RECEIPT_INTAKE_REPORT_PATH"
require_report "$BACKEND_DISPATCH_PACKET_REPORT_PATH"
require_report "$BACKEND_DISPATCH_PACKET_MANIFEST_PATH"
require_file "$BACKEND_DISPATCH_PACKET_ARCHIVE_PATH"

rm -rf "$ROUNDTRIP_DIR"
mkdir -p "$ROUNDTRIP_DIR" "$SIMULATED_INTAKE_DIR"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-backend-receipt-roundtrip.XXXXXX")"
REPORT_TMP="$TMP_DIR/backend-receipt-roundtrip-report.json"
trap 'rm -rf "$TMP_DIR"' EXIT

jq -n \
  --slurpfile dispatch_file "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  '
  ($dispatch_file[0]) as $dispatch
  | {
      receipt_kind:"backend_contract_execution_receipt",
      receipt_version:1,
      receipt_mode:"local_simulated_receipt_roundtrip_only",
      backend_target_repo:$dispatch.backend_lane_target.target_repo,
      owner_lane:"backend_contract",
      dispatch_packet_archive_sha256:$dispatch.archive_sha256,
      selected_receipt_ids:$dispatch.selected_packet_ids,
      simulated_provenance:{
        source:"hepta-ui-backend-receipt-roundtrip-gate",
        backend_agent_dispatch_performed:false,
        backend_repo_mutation_performed:false,
        backend_adapter_promoted:false,
        readback_evidence_recorded_from_backend:false,
        live_runtime_mutation:false,
        external_mutation:false
      },
      receipt_items:($dispatch.selected_packet_ids | map({
        id:.,
        backend_adapter_contract_recorded:true,
        operation_id:("simulated-roundtrip-" + .),
        source_hash:$dispatch.archive_sha256,
        readback_evidence_recorded:true,
        retry_cancel_idempotency_policy_recorded:true,
        stale_target_guard_recorded:true,
        side_effect_review_recorded:true
      })),
      claim_boundary:{
        backend_receipt_claim_ready:false,
        simulated_backend_receipt_claim_ready:true,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false
      },
      side_effects:{
        filesystem_write:true,
        matrix_login:false,
        gateway_call:false,
        provider_invoked:false,
        channel_delivery:false,
        live_runtime_mutation:false,
        external_mutation:false
      }
    }' >"$SIMULATED_RECEIPT_PATH"

env \
  HEPTA_UI_PRODUCT_READINESS_DIR="$READINESS_DIR" \
  HEPTA_UI_BACKEND_RECEIPT_INTAKE_REPORT_PATH="$SIMULATED_INTAKE_REPORT_PATH" \
  HEPTA_UI_BACKEND_RECEIPT_INTAKE_DIR="$SIMULATED_INTAKE_DIR" \
  HEPTA_UI_BACKEND_RECEIPT_INPUT_PATH="$SIMULATED_RECEIPT_PATH" \
  HEPTA_UI_BACKEND_DISPATCH_PACKET_REPORT_PATH="$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  HEPTA_UI_BACKEND_DISPATCH_PACKET_DIR="$BACKEND_DISPATCH_PACKET_DIR" \
  ./scripts/hepta-ui-backend-receipt-intake-gate.sh >/dev/null

require_report "$SIMULATED_INTAKE_REPORT_PATH"

waiting_intake_sha="$(file_sha256 "$BACKEND_RECEIPT_INTAKE_REPORT_PATH")"
dispatch_report_sha="$(file_sha256 "$BACKEND_DISPATCH_PACKET_REPORT_PATH")"
dispatch_manifest_sha="$(file_sha256 "$BACKEND_DISPATCH_PACKET_MANIFEST_PATH")"
dispatch_archive_sha="$(file_sha256 "$BACKEND_DISPATCH_PACKET_ARCHIVE_PATH")"
simulated_receipt_sha="$(file_sha256 "$SIMULATED_RECEIPT_PATH")"
simulated_intake_sha="$(file_sha256 "$SIMULATED_INTAKE_REPORT_PATH")"
dispatch_archive_bytes="$(file_bytes "$BACKEND_DISPATCH_PACKET_ARCHIVE_PATH")"
simulated_receipt_bytes="$(file_bytes "$SIMULATED_RECEIPT_PATH")"
simulated_intake_bytes="$(file_bytes "$SIMULATED_INTAKE_REPORT_PATH")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_backend_receipt_roundtrip_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg roundtrip_dir "$ROUNDTRIP_DIR" \
  --arg waiting_intake_report_path "$BACKEND_RECEIPT_INTAKE_REPORT_PATH" \
  --arg simulated_receipt_path "$SIMULATED_RECEIPT_PATH" \
  --arg simulated_intake_report_path "$SIMULATED_INTAKE_REPORT_PATH" \
  --arg dispatch_report_path "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  --arg dispatch_manifest_path "$BACKEND_DISPATCH_PACKET_MANIFEST_PATH" \
  --arg dispatch_archive_path "$BACKEND_DISPATCH_PACKET_ARCHIVE_PATH" \
  --arg waiting_intake_sha "$waiting_intake_sha" \
  --arg dispatch_report_sha "$dispatch_report_sha" \
  --arg dispatch_manifest_sha "$dispatch_manifest_sha" \
  --arg dispatch_archive_sha "$dispatch_archive_sha" \
  --arg simulated_receipt_sha "$simulated_receipt_sha" \
  --arg simulated_intake_sha "$simulated_intake_sha" \
  --argjson dispatch_archive_bytes "$dispatch_archive_bytes" \
  --argjson simulated_receipt_bytes "$simulated_receipt_bytes" \
  --argjson simulated_intake_bytes "$simulated_intake_bytes" \
  --slurpfile waiting_intake_file "$BACKEND_RECEIPT_INTAKE_REPORT_PATH" \
  --slurpfile dispatch_file "$BACKEND_DISPATCH_PACKET_REPORT_PATH" \
  --slurpfile simulated_receipt_file "$SIMULATED_RECEIPT_PATH" \
  --slurpfile simulated_intake_file "$SIMULATED_INTAKE_REPORT_PATH" \
  '
  ($waiting_intake_file[0]) as $waiting
  | ($dispatch_file[0]) as $dispatch
  | ($simulated_receipt_file[0]) as $receipt
  | ($simulated_intake_file[0]) as $present
  | def selected_ids: ["message_search","file_upload_send","media_download_playback","notifications","room_settings"];
    def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
    def waiting_branch_ready:
      $waiting.backend_receipt_intake_gate_ready == true
      and $waiting.status == "ready"
      and $waiting.selected_receipt_ids == selected_ids
      and (
        (
          $waiting.backend_receipt_present == false
          and $waiting.waiting_for_backend_receipt == true
          and $waiting.backend_receipt_valid == false
        )
        or (
          $waiting.backend_receipt_present == true
          and $waiting.waiting_for_backend_receipt == false
          and $waiting.backend_receipt_valid == true
          and $waiting.receipt_item_count == 5
          and $waiting.receipt_ready_count == 5
          and $waiting.dispatch_packet_archive_sha256 == $dispatch.archive_sha256
          and $waiting.dispatch_packet_archive_bytes == $dispatch.archive_bytes
        )
      )
      and $waiting.claim_boundary.local_backend_receipt_intake_ready == true
      and $waiting.claim_boundary.live_product_claim_ready == false
      and $waiting.claim_boundary.public_distribution_claim_ready == false
      and $waiting.claim_boundary.release_claim_ready == false;
    def simulated_receipt_ready:
      $receipt.receipt_kind == "backend_contract_execution_receipt"
      and $receipt.receipt_version == 1
      and $receipt.receipt_mode == "local_simulated_receipt_roundtrip_only"
      and $receipt.backend_target_repo == "/Users/qianqi/.openclaw/workspace/Hepta"
      and $receipt.dispatch_packet_archive_sha256 == $dispatch.archive_sha256
      and $receipt.selected_receipt_ids == selected_ids
      and ($receipt.receipt_items | length) == 5
      and ($receipt.receipt_items | map(.id)) == selected_ids
      and ($receipt.receipt_items | all(.backend_adapter_contract_recorded == true))
      and ($receipt.receipt_items | all((.operation_id // "") | startswith("simulated-roundtrip-")))
      and ($receipt.receipt_items | all((.source_hash // "") == $dispatch.archive_sha256))
      and ($receipt.receipt_items | all(.readback_evidence_recorded == true))
      and ($receipt.receipt_items | all(.retry_cancel_idempotency_policy_recorded == true))
      and ($receipt.receipt_items | all(.stale_target_guard_recorded == true))
      and ($receipt.receipt_items | all(.side_effect_review_recorded == true))
      and $receipt.simulated_provenance.backend_agent_dispatch_performed == false
      and $receipt.simulated_provenance.backend_repo_mutation_performed == false
      and $receipt.simulated_provenance.backend_adapter_promoted == false
      and $receipt.simulated_provenance.live_runtime_mutation == false
      and $receipt.simulated_provenance.external_mutation == false
      and $receipt.claim_boundary.backend_receipt_claim_ready == false
      and $receipt.claim_boundary.simulated_backend_receipt_claim_ready == true
      and $receipt.claim_boundary.live_product_claim_ready == false
      and $receipt.claim_boundary.public_distribution_claim_ready == false
      and $receipt.claim_boundary.release_claim_ready == false;
    def present_branch_ready:
      $present.backend_receipt_intake_gate_ready == true
      and $present.status == "ready"
      and $present.selected_receipt_ids == selected_ids
      and $present.backend_receipt_present == true
      and $present.waiting_for_backend_receipt == false
      and $present.backend_receipt_valid == true
      and $present.receipt_item_count == 5
      and $present.receipt_ready_count == 5
      and $present.dispatch_packet_archive_sha256 == $dispatch.archive_sha256
      and $present.dispatch_packet_archive_bytes == $dispatch.archive_bytes
      and $present.receipt_input_sha256 == $simulated_receipt_sha
      and $present.receipt_input_bytes == $simulated_receipt_bytes
      and $present.claim_boundary.local_backend_receipt_intake_ready == true
      and $present.claim_boundary.backend_receipt_claim_ready == true
      and $present.claim_boundary.live_product_claim_ready == false
      and $present.claim_boundary.public_distribution_claim_ready == false
      and $present.claim_boundary.release_claim_ready == false
      and $present.side_effects.external_mutation == false;
    (
      $dispatch.backend_dispatch_packet_gate_ready == true
      and $dispatch.selected_packet_ids == selected_ids
      and $dispatch.archive_sha256 == $dispatch_archive_sha
      and $dispatch.archive_bytes == $dispatch_archive_bytes
      and waiting_branch_ready
      and simulated_receipt_ready
      and present_branch_ready
      and sha_ready($waiting_intake_sha)
      and sha_ready($dispatch_report_sha)
      and sha_ready($dispatch_manifest_sha)
      and sha_ready($dispatch_archive_sha)
      and sha_ready($simulated_receipt_sha)
      and sha_ready($simulated_intake_sha)
      and $simulated_receipt_bytes > 0
      and $simulated_intake_bytes > 0
    ) as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      backend_receipt_roundtrip_gate_ready:$ready,
      roundtrip_kind:"local_backend_receipt_valid_branch_replay",
      roundtrip_version:1,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      roundtrip_dir:$roundtrip_dir,
      source_reports:{
        waiting_intake:$waiting_intake_report_path,
        backend_dispatch_packet:$dispatch_report_path,
        backend_dispatch_packet_manifest:$dispatch_manifest_path,
        backend_dispatch_packet_archive:$dispatch_archive_path,
        simulated_receipt:$simulated_receipt_path,
        simulated_receipt_intake:$simulated_intake_report_path
      },
      source_report_sha256:{
        waiting_intake:$waiting_intake_sha,
        backend_dispatch_packet:$dispatch_report_sha,
        backend_dispatch_packet_manifest:$dispatch_manifest_sha,
        backend_dispatch_packet_archive:$dispatch_archive_sha,
        simulated_receipt:$simulated_receipt_sha,
        simulated_receipt_intake:$simulated_intake_sha
      },
      selected_roundtrip_ids:selected_ids,
      roundtrip_item_count:5,
      roundtrip_ready_count:$present.receipt_ready_count,
      dispatch_packet_archive_sha256:$dispatch.archive_sha256,
      dispatch_packet_archive_bytes:$dispatch.archive_bytes,
      waiting_receipt_state:{
        backend_receipt_present:$waiting.backend_receipt_present,
        waiting_for_backend_receipt:$waiting.waiting_for_backend_receipt,
        backend_receipt_valid:$waiting.backend_receipt_valid
      },
      simulated_receipt_state:{
        receipt_mode:$receipt.receipt_mode,
        backend_receipt_present:$present.backend_receipt_present,
        waiting_for_backend_receipt:$present.waiting_for_backend_receipt,
        backend_receipt_valid:$present.backend_receipt_valid,
        receipt_item_count:$present.receipt_item_count,
        receipt_ready_count:$present.receipt_ready_count
      },
      source_alignment:{
        backend_dispatch_packet_ready:$dispatch.backend_dispatch_packet_gate_ready,
        backend_receipt_waiting_branch_ready:waiting_branch_ready,
        backend_receipt_present_branch_ready:present_branch_ready,
        simulated_receipt_ready:simulated_receipt_ready,
        selected_ids_match:($dispatch.selected_packet_ids == selected_ids and $waiting.selected_receipt_ids == selected_ids and $present.selected_receipt_ids == selected_ids),
        dispatch_archive_match:($present.dispatch_packet_archive_sha256 == $dispatch.archive_sha256)
      },
      claim_boundary:{
        local_backend_receipt_roundtrip_ready:$ready,
        local_backend_receipt_intake_ready:$waiting.claim_boundary.local_backend_receipt_intake_ready,
        simulated_backend_receipt_branch_ready:present_branch_ready,
        backend_receipt_claim_ready:false,
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
        local_simulated_receipt_written:true,
        local_simulated_intake_written:true,
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
  and .backend_receipt_roundtrip_gate_ready == true
  and .roundtrip_kind == "local_backend_receipt_valid_branch_replay"
  and .roundtrip_version == 1
  and .selected_roundtrip_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .roundtrip_item_count == 5
  and .roundtrip_ready_count == 5
  and (.dispatch_packet_archive_sha256 | test("^[0-9a-f]{64}$"))
  and .dispatch_packet_archive_bytes > 0
  and (
    (
      .waiting_receipt_state.backend_receipt_present == false
      and .waiting_receipt_state.waiting_for_backend_receipt == true
      and .waiting_receipt_state.backend_receipt_valid == false
    )
    or (
      .waiting_receipt_state.backend_receipt_present == true
      and .waiting_receipt_state.waiting_for_backend_receipt == false
      and .waiting_receipt_state.backend_receipt_valid == true
    )
  )
  and .simulated_receipt_state.receipt_mode == "local_simulated_receipt_roundtrip_only"
  and .simulated_receipt_state.backend_receipt_present == true
  and .simulated_receipt_state.waiting_for_backend_receipt == false
  and .simulated_receipt_state.backend_receipt_valid == true
  and .simulated_receipt_state.receipt_item_count == 5
  and .simulated_receipt_state.receipt_ready_count == 5
  and .source_alignment.backend_dispatch_packet_ready == true
  and .source_alignment.backend_receipt_waiting_branch_ready == true
  and .source_alignment.backend_receipt_present_branch_ready == true
  and .source_alignment.simulated_receipt_ready == true
  and .source_alignment.selected_ids_match == true
  and .source_alignment.dispatch_archive_match == true
  and .claim_boundary.local_backend_receipt_roundtrip_ready == true
  and .claim_boundary.local_backend_receipt_intake_ready == true
  and .claim_boundary.simulated_backend_receipt_branch_ready == true
  and .claim_boundary.backend_receipt_claim_ready == false
  and .claim_boundary.live_runtime_mutation == false
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and .side_effects.local_simulated_receipt_written == true
  and .side_effects.local_simulated_intake_written == true
  and .side_effects.live_runtime_mutation == false
  and .side_effects.external_mutation == false
' "$REPORT_TMP" >/dev/null

mkdir -p "$(dirname "$REPORT_PATH")"
cp "$REPORT_TMP" "$REPORT_PATH"
cat "$REPORT_TMP"
