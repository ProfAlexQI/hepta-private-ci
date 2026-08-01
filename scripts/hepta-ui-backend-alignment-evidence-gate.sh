#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_BACKEND_ALIGNMENT_EVIDENCE_REPORT_PATH:-$READINESS_DIR/ui-backend-alignment-evidence-gate.json}"

STATIC_CONTRACT_PATH="$READINESS_DIR/static-contract.json"
NATIVE_FIXTURE_REPORT_PATH="$READINESS_DIR/native-fixture/native-fixture-visual-smoke.json"
SCREENSHOT_MANIFEST_PATH="$READINESS_DIR/screenshot-manifest.json"
BASE_GAP_BACKEND_HANDOFF_PATH="$READINESS_DIR/native-base-gap-backend-handoff.json"
BACKEND_CONTRACT_GATES_REPORT_PATH="$READINESS_DIR/native-backend-contract-gates.json"
PRODUCTIZATION_ROLLUP_REPORT_PATH="$READINESS_DIR/native-productization-blocker-rollup.json"
PLAN_BOUNDARY_REPORT_PATH="$READINESS_DIR/ui-plan-boundary-gate.json"
DEMO_EVIDENCE_REPORT_PATH="$READINESS_DIR/ui-demo-evidence-gate.json"
EVIDENCE_ARCHIVE_REPORT_PATH="$READINESS_DIR/ui-evidence-archive-gate.json"
OPERATOR_BRIEFING_REPORT_PATH="$READINESS_DIR/ui-operator-briefing-gate.json"
BACKEND_PROMOTION_PACKET_REPORT_PATH="$READINESS_DIR/ui-backend-promotion-packet-gate.json"
FIXTURE_SOURCE_PATH="scripts/hepta-native-fixture-visual-smoke.sh"

ALIGNMENT_IDS=(
  message_search
  file_upload_send
  media_download_playback
  notifications
  room_settings
)

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf '%s is required for the Hepta UI backend alignment evidence gate\n' "$1" >&2
    exit 2
  fi
}

require_report() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required backend-alignment input: %s\n' "$path" >&2
    exit 1
  fi
  jq empty "$path" >/dev/null
}

require_marker() {
  local path="$1"
  local marker="$2"
  if ! grep -Fq "$marker" "$path"; then
    printf 'Missing backend-alignment marker in %s: %s\n' "$path" "$marker" >&2
    exit 1
  fi
}

file_sha256() {
  shasum -a 256 "$1" | awk '{print $1}'
}

require_command jq
require_command shasum

require_report "$STATIC_CONTRACT_PATH"
require_report "$NATIVE_FIXTURE_REPORT_PATH"
require_report "$SCREENSHOT_MANIFEST_PATH"
require_report "$BASE_GAP_BACKEND_HANDOFF_PATH"
require_report "$BACKEND_CONTRACT_GATES_REPORT_PATH"
require_report "$PRODUCTIZATION_ROLLUP_REPORT_PATH"
require_report "$PLAN_BOUNDARY_REPORT_PATH"
require_report "$DEMO_EVIDENCE_REPORT_PATH"
require_report "$EVIDENCE_ARCHIVE_REPORT_PATH"
require_report "$OPERATOR_BRIEFING_REPORT_PATH"
require_report "$BACKEND_PROMOTION_PACKET_REPORT_PATH"

if [[ ! -s "$FIXTURE_SOURCE_PATH" ]]; then
  printf 'Missing fixture source for backend-alignment evidence: %s\n' "$FIXTURE_SOURCE_PATH" >&2
  exit 1
fi

SOURCE_MARKERS=(
  'native_telegram_message_search_server_packet_clipboard_ready:true'
  'native_telegram_message_search_matrix_contract_packet_ready:true'
  'native_telegram_message_search_remote_result_taxonomy_packet_ready:true'
  'native_telegram_message_search_server_pagination_live_ready:true'
  'native_telegram_message_search_loaded_scope_filters_live_ready:true'
  'native_telegram_attachment_accepted_queue_timeline_cancel_bridge_ready:true'
  'native_telegram_attachment_sdk_queue_contract_packet_ready:true'
  'native_telegram_attachment_queue_progress_result_taxonomy_packet_ready:true'
  'native_telegram_attachment_timeline_cancel_local_send_ready:true'
  'native_telegram_media_operation_packet_drilldown_ready:true'
  'native_telegram_media_playback_queue_contract_packet_ready:true'
  'native_telegram_media_playback_result_taxonomy_packet_ready:true'
  'native_telegram_media_inline_playback_queue_boundary_ready:true'
  'native_telegram_notifications_rule_packet_drilldown_ready:true'
  'native_telegram_notifications_rule_contract_packet_ready:true'
  'native_telegram_notifications_result_taxonomy_packet_ready:true'
  'native_telegram_notifications_retry_confirmation_ready:true'
  'native_telegram_room_settings_field_mutation_packet_drilldown_ready:true'
  'native_telegram_room_settings_field_mutation_contract_packet_ready:true'
  'native_telegram_room_settings_power_member_result_taxonomy_packet_ready:true'
  'native_telegram_room_settings_field_edit_intent_controls_ready:true'
)

for marker in "${SOURCE_MARKERS[@]}"; do
  require_marker "$FIXTURE_SOURCE_PATH" "$marker"
done

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-backend-alignment-evidence.XXXXXX")"
REPORT_TMP="$TMP_DIR/backend-alignment-evidence-report.json"
SOURCE_MARKERS_JSON="$TMP_DIR/source-markers.json"
trap 'rm -rf "$TMP_DIR"' EXIT

printf '%s\n' "${SOURCE_MARKERS[@]}" | jq -R . | jq -s . >"$SOURCE_MARKERS_JSON"

static_sha="$(file_sha256 "$STATIC_CONTRACT_PATH")"
native_fixture_sha="$(file_sha256 "$NATIVE_FIXTURE_REPORT_PATH")"
screenshot_manifest_sha="$(file_sha256 "$SCREENSHOT_MANIFEST_PATH")"
handoff_sha="$(file_sha256 "$BASE_GAP_BACKEND_HANDOFF_PATH")"
backend_contract_sha="$(file_sha256 "$BACKEND_CONTRACT_GATES_REPORT_PATH")"
rollup_sha="$(file_sha256 "$PRODUCTIZATION_ROLLUP_REPORT_PATH")"
plan_boundary_sha="$(file_sha256 "$PLAN_BOUNDARY_REPORT_PATH")"
demo_evidence_sha="$(file_sha256 "$DEMO_EVIDENCE_REPORT_PATH")"
evidence_archive_sha="$(file_sha256 "$EVIDENCE_ARCHIVE_REPORT_PATH")"
operator_briefing_sha="$(file_sha256 "$OPERATOR_BRIEFING_REPORT_PATH")"
backend_promotion_sha="$(file_sha256 "$BACKEND_PROMOTION_PACKET_REPORT_PATH")"
fixture_source_sha="$(file_sha256 "$FIXTURE_SOURCE_PATH")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_backend_alignment_evidence_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg static_path "$STATIC_CONTRACT_PATH" \
  --arg native_fixture_path "$NATIVE_FIXTURE_REPORT_PATH" \
  --arg screenshot_manifest_path "$SCREENSHOT_MANIFEST_PATH" \
  --arg handoff_path "$BASE_GAP_BACKEND_HANDOFF_PATH" \
  --arg backend_contract_path "$BACKEND_CONTRACT_GATES_REPORT_PATH" \
  --arg rollup_path "$PRODUCTIZATION_ROLLUP_REPORT_PATH" \
  --arg plan_boundary_path "$PLAN_BOUNDARY_REPORT_PATH" \
  --arg demo_evidence_path "$DEMO_EVIDENCE_REPORT_PATH" \
  --arg evidence_archive_path "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  --arg operator_briefing_path "$OPERATOR_BRIEFING_REPORT_PATH" \
  --arg backend_promotion_path "$BACKEND_PROMOTION_PACKET_REPORT_PATH" \
  --arg fixture_source_path "$FIXTURE_SOURCE_PATH" \
  --arg static_sha "$static_sha" \
  --arg native_fixture_sha "$native_fixture_sha" \
  --arg screenshot_manifest_sha "$screenshot_manifest_sha" \
  --arg handoff_sha "$handoff_sha" \
  --arg backend_contract_sha "$backend_contract_sha" \
  --arg rollup_sha "$rollup_sha" \
  --arg plan_boundary_sha "$plan_boundary_sha" \
  --arg demo_evidence_sha "$demo_evidence_sha" \
  --arg evidence_archive_sha "$evidence_archive_sha" \
  --arg operator_briefing_sha "$operator_briefing_sha" \
  --arg backend_promotion_sha "$backend_promotion_sha" \
  --arg fixture_source_sha "$fixture_source_sha" \
  --slurpfile static_file "$STATIC_CONTRACT_PATH" \
  --slurpfile native_fixture_file "$NATIVE_FIXTURE_REPORT_PATH" \
  --slurpfile screenshot_manifest_file "$SCREENSHOT_MANIFEST_PATH" \
  --slurpfile handoff_file "$BASE_GAP_BACKEND_HANDOFF_PATH" \
  --slurpfile backend_contract_file "$BACKEND_CONTRACT_GATES_REPORT_PATH" \
  --slurpfile rollup_file "$PRODUCTIZATION_ROLLUP_REPORT_PATH" \
  --slurpfile plan_boundary_file "$PLAN_BOUNDARY_REPORT_PATH" \
  --slurpfile demo_evidence_file "$DEMO_EVIDENCE_REPORT_PATH" \
  --slurpfile evidence_archive_file "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  --slurpfile operator_briefing_file "$OPERATOR_BRIEFING_REPORT_PATH" \
  --slurpfile backend_promotion_file "$BACKEND_PROMOTION_PACKET_REPORT_PATH" \
  --slurpfile source_markers_file "$SOURCE_MARKERS_JSON" \
  '
  ($static_file[0]) as $static
  | ($native_fixture_file[0]) as $native_fixture
  | ($screenshot_manifest_file[0]) as $manifest
  | ($handoff_file[0]) as $handoff
  | ($backend_contract_file[0]) as $backend_contract
  | ($rollup_file[0]) as $rollup
  | ($plan_boundary_file[0]) as $plan
  | ($demo_evidence_file[0]) as $demo
  | ($evidence_archive_file[0]) as $archive
  | ($operator_briefing_file[0]) as $operator
  | ($backend_promotion_file[0]) as $promotion
  | ($source_markers_file[0]) as $source_markers
  | def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
    def alignment_ids: ["message_search","file_upload_send","media_download_playback","notifications","room_settings"];
    def gate_for($id):
      if $id == "message_search" then "remote_result_rich_ux_backend_contract"
      elif $id == "file_upload_send" then "queue_media_permission_backend_contract"
      elif $id == "media_download_playback" then "queue_media_permission_backend_contract"
      elif $id == "notifications" then "notifications_backend_contract"
      elif $id == "room_settings" then "room_settings_backend_contract"
      else "unknown"
      end;
    def markers_for($id):
      if $id == "message_search" then [
        "native_telegram_message_search_server_packet_clipboard_ready:true",
        "native_telegram_message_search_matrix_contract_packet_ready:true",
        "native_telegram_message_search_remote_result_taxonomy_packet_ready:true",
        "native_telegram_message_search_server_pagination_live_ready:true",
        "native_telegram_message_search_loaded_scope_filters_live_ready:true"
      ]
      elif $id == "file_upload_send" then [
        "native_telegram_attachment_accepted_queue_timeline_cancel_bridge_ready:true",
        "native_telegram_attachment_sdk_queue_contract_packet_ready:true",
        "native_telegram_attachment_queue_progress_result_taxonomy_packet_ready:true",
        "native_telegram_attachment_timeline_cancel_local_send_ready:true"
      ]
      elif $id == "media_download_playback" then [
        "native_telegram_media_operation_packet_drilldown_ready:true",
        "native_telegram_media_playback_queue_contract_packet_ready:true",
        "native_telegram_media_playback_result_taxonomy_packet_ready:true",
        "native_telegram_media_inline_playback_queue_boundary_ready:true"
      ]
      elif $id == "notifications" then [
        "native_telegram_notifications_rule_packet_drilldown_ready:true",
        "native_telegram_notifications_rule_contract_packet_ready:true",
        "native_telegram_notifications_result_taxonomy_packet_ready:true",
        "native_telegram_notifications_retry_confirmation_ready:true"
      ]
      elif $id == "room_settings" then [
        "native_telegram_room_settings_field_mutation_packet_drilldown_ready:true",
        "native_telegram_room_settings_field_mutation_contract_packet_ready:true",
        "native_telegram_room_settings_power_member_result_taxonomy_packet_ready:true",
        "native_telegram_room_settings_field_edit_intent_controls_ready:true"
      ]
      else []
      end;
    def required_contract_groups_for($id):
      if $id == "message_search" then [
        "remote result cursor",
        "message search date pins scope sort",
        "message search remote date pins scope full result taxonomy"
      ]
      elif $id == "file_upload_send" then [
        "shared queue result",
        "file upload accepted queue",
        "file upload accepted queue/progress/result taxonomy packet"
      ]
      elif $id == "media_download_playback" then [
        "shared queue result",
        "media playback decrypt decode queue",
        "media playback decrypt/decode/opener/queue result taxonomy packet"
      ]
      elif $id == "notifications" then [
        "room mode result",
        "keyword rule mutation result",
        "timed/global/pusher result taxonomy packet"
      ]
      elif $id == "room_settings" then [
        "room identity and refresh result",
        "power/member result taxonomy ui packet",
        "stale-room retry guard"
      ]
      else []
      end;
    def contract_gate_ready($id):
      (gate_for($id)) as $gate
      | ($backend_contract.waves | map(select(.gate == $gate and .status == "ready")) | length) == 1
      and (required_contract_groups_for($id) | all(. as $group |
        ($backend_contract.waves[] | select(.gate == $gate) | .required_contract_groups | index($group)) != null
      ));
    def promotion_packet_for($id): ($promotion.priority_packets[] | select(.id == $id));
    def handoff_item_for($id): ($handoff.items[] | select(.id == $id));
    def alignment_item($id):
      (handoff_item_for($id)) as $handoff_item
      | (promotion_packet_for($id)) as $packet
      | {
          id:$id,
          priority:$handoff_item.priority,
          backend_contract_gate:gate_for($id),
          alignment_ready:(
            $handoff_item.status == "partial_live_backend_contract_remaining"
            and $handoff_item.ui_lane_state == "complete"
            and $handoff_item.next_owner_lane == "backend_contract"
            and ($handoff_item.required_backend_contracts | length) >= 5
            and $handoff_item.side_effects.external_mutation == false
            and $packet.promote_requires_backend_adapter == true
            and $packet.promote_requires_readback_evidence == true
            and $packet.promote_requires_side_effect_review == true
            and $packet.active_promotion_performed == false
            and contract_gate_ready($id)
            and (markers_for($id) | all(. as $marker | ($source_markers | index($marker)) != null))
          ),
          ui_lane_state:$handoff_item.ui_lane_state,
          next_owner_lane:$handoff_item.next_owner_lane,
          status:$handoff_item.status,
          live_wiring:$handoff_item.acceptance_state.live_wiring,
          current_ui_evidence:$packet.current_ui_evidence,
          backend_contract_next_slice:$packet.backend_contract_next_slice,
          promotion_blocker:$packet.promotion_blocker,
          required_backend_contract_count:($handoff_item.required_backend_contracts | length),
          required_backend_contracts:$handoff_item.required_backend_contracts,
          required_contract_groups:required_contract_groups_for($id),
          fixture_source_markers:markers_for($id),
          fixture_source_marker_count:(markers_for($id) | length),
          promotion_requires:{
            backend_adapter:$packet.promote_requires_backend_adapter,
            readback_evidence:$packet.promote_requires_readback_evidence,
            side_effect_review:$packet.promote_requires_side_effect_review
          },
          active_promotion_performed:false,
          side_effects:$handoff_item.side_effects
        };
    def source_chain_ready:
      $static.static_contract_ready == true
      and $static.marker_count >= 3642
      and $native_fixture.status == "ready"
      and $native_fixture.native_secondary_product_surfaces_ready == true
      and $manifest.screenshot_manifest_ready == true
      and $handoff.native_base_gap_backend_handoff_ready == true
      and $handoff.handoff_count == 12
      and ($handoff.items | length) == 12
      and $backend_contract.backend_contract_waves_ready == true
      and $backend_contract.verified_gap_count == 12
      and $rollup.productization_blocker_rollup_ready == true
      and $plan.plan_boundary_gate_ready == true
      and $plan.live_product_claim.remaining_backend_contract_count == 12
      and $plan.live_product_claim.ready == false
      and $demo.demo_evidence_gate_ready == true
      and $demo.claim_boundary.local_fixture_demo_evidence_ready == true
      and $archive.evidence_archive_gate_ready == true
      and $archive.claim_boundary.local_evidence_archive_ready == true
      and $operator.operator_briefing_gate_ready == true
      and $operator.backend_remaining_contract_count == 12
      and $promotion.backend_promotion_packet_gate_ready == true
      and $promotion.selected_priority_ids == alignment_ids
      and ($promotion.priority_packets | length) == 5
      and sha_ready($static_sha)
      and sha_ready($native_fixture_sha)
      and sha_ready($screenshot_manifest_sha)
      and sha_ready($handoff_sha)
      and sha_ready($backend_contract_sha)
      and sha_ready($rollup_sha)
      and sha_ready($plan_boundary_sha)
      and sha_ready($demo_evidence_sha)
      and sha_ready($evidence_archive_sha)
      and sha_ready($operator_briefing_sha)
      and sha_ready($backend_promotion_sha)
      and sha_ready($fixture_source_sha);
    (alignment_ids | map(alignment_item(.))) as $items
  | (source_chain_ready and ($items | all(.alignment_ready == true))) as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      backend_alignment_evidence_gate_ready:$ready,
      alignment_kind:"local_backend_handoff_alignment_evidence",
      alignment_version:1,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      selected_alignment_ids:alignment_ids,
      alignment_item_count:($items | length),
      alignment_items:$items,
      source_reports:{
        static_contract:$static_path,
        native_fixture_visual_smoke:$native_fixture_path,
        screenshot_manifest:$screenshot_manifest_path,
        backend_handoff:$handoff_path,
        backend_contract_gates:$backend_contract_path,
        productization_rollup:$rollup_path,
        plan_boundary:$plan_boundary_path,
        demo_evidence:$demo_evidence_path,
        evidence_archive:$evidence_archive_path,
        operator_briefing:$operator_briefing_path,
        backend_promotion_packet:$backend_promotion_path,
        fixture_marker_source:$fixture_source_path
      },
      source_report_sha256:{
        static_contract:$static_sha,
        native_fixture_visual_smoke:$native_fixture_sha,
        screenshot_manifest:$screenshot_manifest_sha,
        backend_handoff:$handoff_sha,
        backend_contract_gates:$backend_contract_sha,
        productization_rollup:$rollup_sha,
        plan_boundary:$plan_boundary_sha,
        demo_evidence:$demo_evidence_sha,
        evidence_archive:$evidence_archive_sha,
        operator_briefing:$operator_briefing_sha,
        backend_promotion_packet:$backend_promotion_sha,
        fixture_marker_source:$fixture_source_sha
      },
      visual_evidence:{
        local_fixture_ready:$native_fixture.native_secondary_product_surfaces_ready,
        local_fixture_case_count:$native_fixture.secondary_product_surfaces.case_count,
        demo_evidence_ready:$demo.demo_evidence_gate_ready,
        required_report_count:$demo.report_evidence.required_report_count,
        required_screenshot_count:$demo.screenshot_evidence.required_screenshot_count,
        hard_true_window_required:$demo.claim_boundary.hard_true_window_required,
        hard_true_window_evidence_ready:$demo.claim_boundary.r33_hard_demo_evidence_ready,
        evidence_archive_ready:$archive.evidence_archive_gate_ready,
        evidence_archive_sha256:$archive.archive_sha256
      },
      future_plan_alignment:{
        backend_contract_remaining_count:$plan.live_product_claim.remaining_backend_contract_count,
        next_owner_lane:$plan.live_product_claim.next_owner_lane,
        backend_priority_ids:($handoff.items | sort_by(.priority) | map(.id)),
        selected_alignment_ids:alignment_ids,
        next_plan:$plan.next_plan
      },
      acceptance_guardrail:{
        active_backend_promotion_allowed:false,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false,
        required_before_backend_can_promote:[
          "typed adapter contract for each selected handoff item",
          "operation id/source hash/readback evidence",
          "retry cancel idempotency and stale-target guard",
          "side-effect review and redaction behavior",
          "focused Native tests and refreshed readiness artifact"
        ]
      },
      claim_boundary:{
        local_backend_alignment_evidence_ready:$ready,
        local_backend_promotion_packet_ready:$promotion.claim_boundary.local_backend_promotion_packet_ready,
        active_backend_promotion_performed:false,
        backend_adapter_promoted:false,
        live_runtime_mutation:false,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false,
        external_actions_allowed:false
      },
      side_effects:{
        filesystem_read:true,
        fixture_source_read:true,
        local_report_written:true,
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
  and .backend_alignment_evidence_gate_ready == true
  and .alignment_kind == "local_backend_handoff_alignment_evidence"
  and .alignment_version == 1
  and .selected_alignment_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .alignment_item_count == 5
  and (.alignment_items | length) == 5
  and (.alignment_items | all(.alignment_ready == true))
  and (.alignment_items | all(.next_owner_lane == "backend_contract"))
  and (.alignment_items | all(.status == "partial_live_backend_contract_remaining"))
  and (.alignment_items | all(.required_backend_contract_count >= 5))
  and (.alignment_items | all(.fixture_source_marker_count >= 4))
  and (.alignment_items | map(select(.id == "message_search" and .backend_contract_gate == "remote_result_rich_ux_backend_contract")) | length) == 1
  and (.alignment_items | map(select(.id == "file_upload_send" and .backend_contract_gate == "queue_media_permission_backend_contract")) | length) == 1
  and (.alignment_items | map(select(.id == "media_download_playback" and .backend_contract_gate == "queue_media_permission_backend_contract")) | length) == 1
  and (.alignment_items | map(select(.id == "notifications" and .backend_contract_gate == "notifications_backend_contract")) | length) == 1
  and (.alignment_items | map(select(.id == "room_settings" and .backend_contract_gate == "room_settings_backend_contract")) | length) == 1
  and .visual_evidence.local_fixture_ready == true
  and .visual_evidence.local_fixture_case_count == 15
  and .visual_evidence.demo_evidence_ready == true
  and .visual_evidence.required_report_count >= 13
  and .visual_evidence.required_screenshot_count >= 23
  and .visual_evidence.evidence_archive_ready == true
  and (.visual_evidence.evidence_archive_sha256 | test("^[0-9a-f]{64}$"))
  and .future_plan_alignment.backend_contract_remaining_count == 12
  and .future_plan_alignment.next_owner_lane == "backend_contract"
  and .future_plan_alignment.selected_alignment_ids == ["message_search","file_upload_send","media_download_playback","notifications","room_settings"]
  and .acceptance_guardrail.active_backend_promotion_allowed == false
  and .claim_boundary.local_backend_alignment_evidence_ready == true
  and .claim_boundary.local_backend_promotion_packet_ready == true
  and .claim_boundary.active_backend_promotion_performed == false
  and .claim_boundary.backend_adapter_promoted == false
  and .claim_boundary.live_runtime_mutation == false
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and .side_effects.backend_adapter_promoted == false
  and .side_effects.live_runtime_mutation == false
  and .side_effects.external_mutation == false
' "$REPORT_TMP" >/dev/null

mkdir -p "$(dirname "$REPORT_PATH")"
cp "$REPORT_TMP" "$REPORT_PATH"
cat "$REPORT_TMP"
