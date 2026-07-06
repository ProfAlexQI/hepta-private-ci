#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_BACKEND_DISPATCH_PACKET_REPORT_PATH:-$READINESS_DIR/ui-backend-dispatch-packet-gate.json}"
PACKET_DIR="${HEPTA_UI_BACKEND_DISPATCH_PACKET_DIR:-$READINESS_DIR/backend-dispatch-packet}"
PAYLOAD_DIR="$PACKET_DIR/payload"
EXTRACT_DIR="$PACKET_DIR/extracted"
MANIFEST_PATH="$PACKET_DIR/backend-dispatch-packet-manifest.json"
ARCHIVE_PATH="$PACKET_DIR/backend-dispatch-packet.tar.gz"
PACKET_MARKDOWN_PATH="$PACKET_DIR/backend-dispatch-packet.md"

PLAN_BOUNDARY_REPORT_PATH="$READINESS_DIR/ui-plan-boundary-gate.json"
OPERATOR_BRIEFING_REPORT_PATH="$READINESS_DIR/ui-operator-briefing-gate.json"
BACKEND_PROMOTION_PACKET_REPORT_PATH="$READINESS_DIR/ui-backend-promotion-packet-gate.json"
BACKEND_ALIGNMENT_EVIDENCE_REPORT_PATH="$READINESS_DIR/ui-backend-alignment-evidence-gate.json"
CRITICAL_PATH_PLAN_REPORT_PATH="$READINESS_DIR/ui-critical-path-plan-gate.json"
BACKEND_CONTRACT_ACCEPTANCE_REPORT_PATH="$READINESS_DIR/ui-backend-contract-acceptance-gate.json"
BACKEND_HANDOFF_EXPORT_REPORT_PATH="$READINESS_DIR/ui-backend-handoff-export-gate.json"
BACKEND_HANDOFF_EXPORT_MARKDOWN_PATH="$READINESS_DIR/backend-handoff-export/backend-handoff-export.md"
BACKEND_AGENT_PROBE_PATH="${HEPTA_UI_BACKEND_AGENT_PROBE_PATH:-}"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf '%s is required for the Hepta UI backend dispatch packet gate\n' "$1" >&2
    exit 2
  fi
}

require_report() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required backend dispatch packet input: %s\n' "$path" >&2
    exit 1
  fi
  jq empty "$path" >/dev/null
}

require_file() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required backend dispatch packet file: %s\n' "$path" >&2
    exit 1
  fi
}

file_sha256() {
  shasum -a 256 "$1" | awk '{print $1}'
}

file_bytes() {
  wc -c <"$1" | tr -d ' '
}

copy_payload_file() {
  local name="$1"
  local source_path="$2"
  local relative_path="$3"
  local kind="$4"
  local destination_path="$PAYLOAD_DIR/$relative_path"
  local bytes
  local sha

  require_file "$source_path"
  mkdir -p "$(dirname "$destination_path")"
  cp "$source_path" "$destination_path"
  bytes="$(file_bytes "$destination_path")"
  sha="$(file_sha256 "$destination_path")"

  jq -n \
    --arg name "$name" \
    --arg kind "$kind" \
    --arg source_path "$source_path" \
    --arg relative_path "$relative_path" \
    --arg sha "$sha" \
    --argjson bytes "$bytes" \
    '{
      name:$name,
      kind:$kind,
      source_path:$source_path,
      relative_path:$relative_path,
      bytes:$bytes,
      sha256:$sha,
      copied:true
    }' >>"$PAYLOAD_ITEMS_NDJSON"
}

require_command jq
require_command shasum
require_command tar

require_report "$PLAN_BOUNDARY_REPORT_PATH"
require_report "$OPERATOR_BRIEFING_REPORT_PATH"
require_report "$BACKEND_PROMOTION_PACKET_REPORT_PATH"
require_report "$BACKEND_ALIGNMENT_EVIDENCE_REPORT_PATH"
require_report "$CRITICAL_PATH_PLAN_REPORT_PATH"
require_report "$BACKEND_CONTRACT_ACCEPTANCE_REPORT_PATH"
require_report "$BACKEND_HANDOFF_EXPORT_REPORT_PATH"
require_file "$BACKEND_HANDOFF_EXPORT_MARKDOWN_PATH"
if [[ -n "$BACKEND_AGENT_PROBE_PATH" ]]; then
  require_report "$BACKEND_AGENT_PROBE_PATH"
fi

rm -rf "$PAYLOAD_DIR" "$EXTRACT_DIR"
mkdir -p "$PACKET_DIR" "$PAYLOAD_DIR" "$EXTRACT_DIR"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-backend-dispatch-packet.XXXXXX")"
PAYLOAD_ITEMS_NDJSON="$TMP_DIR/payload-items.ndjson"
PAYLOAD_ITEMS_JSON="$TMP_DIR/payload-items.json"
REPORT_TMP="$TMP_DIR/backend-dispatch-packet-report.json"
MARKDOWN_TMP="$TMP_DIR/backend-dispatch-packet.md"
BACKEND_AGENT_PROBE_CAPTURE_PATH="$TMP_DIR/backend-agent-probe.json"
trap 'rm -rf "$TMP_DIR"' EXIT
: >"$PAYLOAD_ITEMS_NDJSON"

if [[ -n "$BACKEND_AGENT_PROBE_PATH" ]]; then
  cp "$BACKEND_AGENT_PROBE_PATH" "$BACKEND_AGENT_PROBE_CAPTURE_PATH"
else
  jq -n '{present:false}' >"$BACKEND_AGENT_PROBE_CAPTURE_PATH"
fi

copy_payload_file "backend_handoff_export_markdown" "$BACKEND_HANDOFF_EXPORT_MARKDOWN_PATH" "backend-handoff-export/backend-handoff-export.md" "backend_export"
copy_payload_file "backend_handoff_export_report" "$BACKEND_HANDOFF_EXPORT_REPORT_PATH" "reports/ui-backend-handoff-export-gate.json" "backend_export"
copy_payload_file "backend_contract_acceptance_report" "$BACKEND_CONTRACT_ACCEPTANCE_REPORT_PATH" "reports/ui-backend-contract-acceptance-gate.json" "backend_acceptance"
copy_payload_file "critical_path_plan_report" "$CRITICAL_PATH_PLAN_REPORT_PATH" "reports/ui-critical-path-plan-gate.json" "critical_path"
copy_payload_file "backend_alignment_evidence_report" "$BACKEND_ALIGNMENT_EVIDENCE_REPORT_PATH" "reports/ui-backend-alignment-evidence-gate.json" "backend_alignment"
copy_payload_file "backend_promotion_packet_report" "$BACKEND_PROMOTION_PACKET_REPORT_PATH" "reports/ui-backend-promotion-packet-gate.json" "backend_promotion"
copy_payload_file "operator_briefing_report" "$OPERATOR_BRIEFING_REPORT_PATH" "reports/ui-operator-briefing-gate.json" "operator_briefing"
copy_payload_file "plan_boundary_report" "$PLAN_BOUNDARY_REPORT_PATH" "reports/ui-plan-boundary-gate.json" "claim_boundary"

jq -s '.' "$PAYLOAD_ITEMS_NDJSON" >"$PAYLOAD_ITEMS_JSON"

jq -n \
  --arg product "Hepta UI" \
  --arg packet_kind "local_backend_dispatch_packet" \
  --arg packet_dir "$PACKET_DIR" \
  --arg payload_dir "$PAYLOAD_DIR" \
  --arg archive_path "$ARCHIVE_PATH" \
  --arg packet_markdown_path "$PACKET_MARKDOWN_PATH" \
  --slurpfile payload_items_file "$PAYLOAD_ITEMS_JSON" \
  '($payload_items_file[0]) as $payload_items
  | {
      product:$product,
      packet_kind:$packet_kind,
      packet_version:1,
      packet_dir:$packet_dir,
      payload_dir:$payload_dir,
      archive_path:$archive_path,
      packet_markdown_path:$packet_markdown_path,
      payload_file_count:($payload_items | length),
      payload_total_bytes:($payload_items | map(.bytes) | add),
      payload_items:$payload_items
    }' >"$MANIFEST_PATH"

jq -r '
  "# Hepta UI Backend Dispatch Packet\n\n"
  + "- Kind: \(.packet_kind)\n"
  + "- Payload files: \(.payload_file_count)\n"
  + "- Payload bytes: \(.payload_total_bytes)\n"
  + "- Archive: \(.archive_path)\n"
  + "- Claim boundary: local packet only; backend adapter=false; readback=false; live runtime=false; live product=false; public distribution=false; release=false\n\n"
  + "## Payload\n\n"
  + (.payload_items | map("- `\(.relative_path)` \(.bytes) bytes \(.sha256)") | join("\n"))
' "$MANIFEST_PATH" >"$MARKDOWN_TMP"
cp "$MARKDOWN_TMP" "$PACKET_MARKDOWN_PATH"

COPYFILE_DISABLE=1 tar -czf "$ARCHIVE_PATH" -C "$PAYLOAD_DIR" .
tar -xzf "$ARCHIVE_PATH" -C "$EXTRACT_DIR"

while IFS=$'\t' read -r relative_path expected_sha expected_bytes; do
  extracted_path="$EXTRACT_DIR/$relative_path"
  require_file "$extracted_path"
  actual_sha="$(file_sha256 "$extracted_path")"
  actual_bytes="$(file_bytes "$extracted_path")"
  if [[ "$actual_sha" != "$expected_sha" || "$actual_bytes" != "$expected_bytes" ]]; then
    printf 'Backend dispatch packet extracted mismatch: %s\n' "$relative_path" >&2
    exit 1
  fi
done < <(jq -r '.payload_items[] | [.relative_path, .sha256, (.bytes|tostring)] | @tsv' "$MANIFEST_PATH")

archive_sha="$(file_sha256 "$ARCHIVE_PATH")"
archive_bytes="$(file_bytes "$ARCHIVE_PATH")"
manifest_sha="$(file_sha256 "$MANIFEST_PATH")"
manifest_bytes="$(file_bytes "$MANIFEST_PATH")"
packet_markdown_sha="$(file_sha256 "$PACKET_MARKDOWN_PATH")"
packet_markdown_bytes="$(file_bytes "$PACKET_MARKDOWN_PATH")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_backend_dispatch_packet_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg packet_dir "$PACKET_DIR" \
  --arg payload_dir "$PAYLOAD_DIR" \
  --arg extract_dir "$EXTRACT_DIR" \
  --arg manifest_path "$MANIFEST_PATH" \
  --arg archive_path "$ARCHIVE_PATH" \
  --arg packet_markdown_path "$PACKET_MARKDOWN_PATH" \
  --arg archive_sha "$archive_sha" \
  --arg manifest_sha "$manifest_sha" \
  --arg packet_markdown_sha "$packet_markdown_sha" \
  --argjson archive_bytes "$archive_bytes" \
  --argjson manifest_bytes "$manifest_bytes" \
  --argjson packet_markdown_bytes "$packet_markdown_bytes" \
  --slurpfile manifest_file "$MANIFEST_PATH" \
  --slurpfile plan_boundary_file "$PLAN_BOUNDARY_REPORT_PATH" \
  --slurpfile operator_briefing_file "$OPERATOR_BRIEFING_REPORT_PATH" \
  --slurpfile backend_promotion_file "$BACKEND_PROMOTION_PACKET_REPORT_PATH" \
  --slurpfile backend_alignment_file "$BACKEND_ALIGNMENT_EVIDENCE_REPORT_PATH" \
  --slurpfile critical_path_file "$CRITICAL_PATH_PLAN_REPORT_PATH" \
  --slurpfile backend_acceptance_file "$BACKEND_CONTRACT_ACCEPTANCE_REPORT_PATH" \
  --slurpfile backend_handoff_export_file "$BACKEND_HANDOFF_EXPORT_REPORT_PATH" \
  --slurpfile backend_agent_probe_file "$BACKEND_AGENT_PROBE_CAPTURE_PATH" \
  '
  ($manifest_file[0]) as $manifest
  | ($plan_boundary_file[0]) as $plan
  | ($operator_briefing_file[0]) as $operator
  | ($backend_promotion_file[0]) as $promotion
  | ($backend_alignment_file[0]) as $alignment
  | ($critical_path_file[0]) as $critical
  | ($backend_acceptance_file[0]) as $acceptance
  | ($backend_handoff_export_file[0]) as $export
  | ($backend_agent_probe_file[0]) as $agent_probe
  | def selected_ids: ["message_search","file_upload_send","media_download_playback","notifications","room_settings"];
    def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
    def controlled_backend_agent_available:
      (
        ($agent_probe.status // "") == "ok"
        and (($agent_probe.sessionKey // "") | test("^agent:hepta-backend:"))
      )
      or
      (
        ($agent_probe.reply // "") == "HEPTA_UI_SESSIONS_SEND_PROBE_OK"
        and (($agent_probe.sessionKey // "") | test("^agent:hepta-backend:"))
      );
    def source_chain_ready:
      $plan.plan_boundary_gate_ready == true
      and $operator.operator_briefing_gate_ready == true
      and $promotion.backend_promotion_packet_gate_ready == true
      and $alignment.backend_alignment_evidence_gate_ready == true
      and $critical.critical_path_plan_gate_ready == true
      and $acceptance.backend_contract_acceptance_gate_ready == true
      and $export.backend_handoff_export_gate_ready == true
      and $export.selected_export_ids == selected_ids
      and $export.export_item_count == 5
      and $export.export_ready_count == 5
      and $export.backend_lane_target.target_repo == "/Users/qianqi/.openclaw/workspace/Hepta"
      and $export.dispatch_guardrail.external_dispatch_performed == false
      and $export.dispatch_guardrail.backend_adapter_promoted == false
      and $export.dispatch_guardrail.live_runtime_mutation == false
      and $export.claim_boundary.local_backend_handoff_export_ready == true
      and $export.claim_boundary.live_product_claim_ready == false
      and $export.claim_boundary.public_distribution_claim_ready == false
      and $export.claim_boundary.release_claim_ready == false
      and $export.side_effects.external_mutation == false
      and $acceptance.claim_boundary.local_backend_contract_acceptance_ready == true
      and $acceptance.claim_boundary.backend_adapter_promoted == false
      and $acceptance.claim_boundary.live_runtime_mutation == false
      and $acceptance.claim_boundary.live_product_claim_ready == false
      and $plan.live_product_claim.remaining_backend_contract_count == 12;
    def payload_ready:
      $manifest.packet_kind == "local_backend_dispatch_packet"
      and $manifest.packet_version == 1
      and $manifest.payload_file_count == 8
      and ($manifest.payload_items | length) == 8
      and ($manifest.payload_items | all(.copied == true and .bytes > 0 and (.sha256 | test("^[0-9a-f]{64}$"))))
      and ($manifest.payload_items | map(.relative_path) | unique | length) == 8
      and sha_ready($archive_sha)
      and $archive_bytes > 0
      and sha_ready($manifest_sha)
      and $manifest_bytes > 0
      and sha_ready($packet_markdown_sha)
      and $packet_markdown_bytes > 0;
    (source_chain_ready and payload_ready) as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      backend_dispatch_packet_gate_ready:$ready,
      packet_kind:"local_backend_dispatch_packet",
      packet_version:1,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      packet_dir:$packet_dir,
      payload_dir:$payload_dir,
      extract_dir:$extract_dir,
      manifest_path:$manifest_path,
      archive_path:$archive_path,
      archive_sha256:$archive_sha,
      archive_bytes:$archive_bytes,
      packet_markdown_path:$packet_markdown_path,
      packet_markdown_sha256:$packet_markdown_sha,
      packet_markdown_bytes:$packet_markdown_bytes,
      manifest_sha256:$manifest_sha,
      manifest_bytes:$manifest_bytes,
      payload_file_count:$manifest.payload_file_count,
      payload_total_bytes:$manifest.payload_total_bytes,
      extracted_file_count:$manifest.payload_file_count,
      all_extracted_files_sha256_match:true,
      selected_packet_ids:selected_ids,
      packet_item_count:$export.export_item_count,
      packet_ready_count:$export.export_ready_count,
      backend_lane_target:$export.backend_lane_target,
      hepta_ui_after_backend:$export.hepta_ui_after_backend,
      backend_agent_probe:{
        evidence_path:(if ($agent_probe.present // true) == false then null else env.HEPTA_UI_BACKEND_AGENT_PROBE_PATH end),
        status:($agent_probe.status // null),
        reply:($agent_probe.reply // null),
        session_key:($agent_probe.sessionKey // null),
        controlled_sessions_send_ready:controlled_backend_agent_available
      },
      manifest:$manifest,
      dispatch_guardrail:{
        local_dispatch_packet_ready:$ready,
        external_dispatch_performed:false,
        backend_agent_available:controlled_backend_agent_available,
        backend_adapter_promoted:false,
        readback_evidence_recorded:false,
        side_effect_review_recorded:false,
        live_runtime_mutation:false,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false
      },
      claim_boundary:{
        local_backend_dispatch_packet_ready:$ready,
        local_backend_handoff_export_ready:$export.claim_boundary.local_backend_handoff_export_ready,
        local_backend_contract_acceptance_ready:$acceptance.claim_boundary.local_backend_contract_acceptance_ready,
        active_backend_promotion_performed:false,
        backend_adapter_promoted:false,
        readback_evidence_recorded:false,
        side_effect_review_recorded:false,
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
        local_payload_written:true,
        local_manifest_written:true,
        local_archive_written:true,
        local_extract_verification:true,
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
  and .backend_dispatch_packet_gate_ready == true
  and .packet_kind == "local_backend_dispatch_packet"
  and .packet_version == 1
  and .selected_packet_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .packet_item_count == 5
  and .packet_ready_count == 5
  and .payload_file_count == 8
  and .extracted_file_count == 8
  and .all_extracted_files_sha256_match == true
  and (.archive_sha256 | test("^[0-9a-f]{64}$"))
  and .archive_bytes > 0
  and (.manifest_sha256 | test("^[0-9a-f]{64}$"))
  and .manifest_bytes > 0
  and (.packet_markdown_sha256 | test("^[0-9a-f]{64}$"))
  and .packet_markdown_bytes > 0
  and .backend_lane_target.target_repo == "/Users/qianqi/.openclaw/workspace/Hepta"
  and .backend_lane_target.owner_lane == "backend_contract"
  and (.hepta_ui_after_backend.required_refresh_commands | length) == 2
  and (.backend_agent_probe.controlled_sessions_send_ready | type) == "boolean"
  and .dispatch_guardrail.local_dispatch_packet_ready == true
  and .dispatch_guardrail.external_dispatch_performed == false
  and (.dispatch_guardrail.backend_agent_available | type) == "boolean"
  and .dispatch_guardrail.backend_adapter_promoted == false
  and .dispatch_guardrail.live_runtime_mutation == false
  and .dispatch_guardrail.live_product_claim_ready == false
  and .claim_boundary.local_backend_dispatch_packet_ready == true
  and .claim_boundary.backend_adapter_promoted == false
  and .claim_boundary.readback_evidence_recorded == false
  and .claim_boundary.live_runtime_mutation == false
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and .side_effects.local_archive_written == true
  and .side_effects.local_extract_verification == true
  and .side_effects.backend_adapter_promoted == false
  and .side_effects.live_runtime_mutation == false
  and .side_effects.external_mutation == false
' "$REPORT_TMP" >/dev/null

mkdir -p "$(dirname "$REPORT_PATH")"
cp "$REPORT_TMP" "$REPORT_PATH"
cat "$REPORT_TMP"
