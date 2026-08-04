#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

CONTRACT_PATH="docs/architecture/HEPTA_NATIVE_SPACES_ROOM_MEMBERSHIP_EDGE_BACKEND_CONTRACT_2026-06-15.md"
FIXTURE_CONTRACT_PATH="apps/hepta-native/packaging/native-fixture-contract-v1.json"
READINESS_DIR="${HEPTA_NATIVE_SPACES_ROOM_EDGE_CONTRACT_READINESS_DIR:-${HEPTA_UI_PRODUCT_READINESS_DIR:-}}"
REPORT_PATH="${HEPTA_NATIVE_SPACES_ROOM_EDGE_CONTRACT_REPORT:-}"

require_file() {
  local path="$1"
  if [[ ! -f "$path" ]]; then
    echo "missing required file: $path" >&2
    exit 1
  fi
}

require_contains() {
  local path="$1"
  local needle="$2"
  if ! grep -Fq "$needle" "$path"; then
    echo "missing marker in $path: $needle" >&2
    exit 1
  fi
}

require_file "$CONTRACT_PATH"
require_file "$FIXTURE_CONTRACT_PATH"
require_file "apps/hepta-native/src/home/rooms_list.rs"
require_file "apps/hepta-native/src/home/space_lobby.rs"
require_file "apps/hepta-native/src/home/add_room.rs"
require_file "apps/hepta-native/src/room/room_display_filter.rs"
require_file "apps/hepta-native/src/space_service_sync.rs"
require_file "apps/hepta-native/src/home/hepta_telegram_base_contract.rs"

require_contains "$CONTRACT_PATH" "Re-Knock / Cancel Prior Knock Contract"
require_contains "$CONTRACT_PATH" "re-knock/cancel-prior packet"
require_contains "$CONTRACT_PATH" "Selected Room Removed / Rejoin Contract"
require_contains "$CONTRACT_PATH" "selected-room removed/rejoin packet"
require_contains "$CONTRACT_PATH" "Room-List Pagination Contract"
require_contains "$CONTRACT_PATH" "Room-list Load More pagination packet"
require_contains "$CONTRACT_PATH" "explicit cursor slot"
require_contains "$CONTRACT_PATH" "Space Aggregate Contract"
require_contains "$CONTRACT_PATH" 'no unconfirmed `JoinRoom`, `LeaveRoom`, `Knock`'
require_contains "$CONTRACT_PATH" "visible row prefetch for latest messages must remain separate"
require_contains "$CONTRACT_PATH" "counters must update without sending read receipts"

require_contains "apps/hepta-native/src/home/rooms_list.rs" "re-knock/cancel-prior-knock UI remains unwired"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "no Load more rooms UI"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "Room-list Load More pagination packet"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "room_list_load_more_packet_label"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "load_more_button_slot not_rendered"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "explicit_cursor_slot not_exposed"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "latest_preview_pagination_source Matrix_PaginateTimeline_read_only"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "Selected-room removed/rejoin packet"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "selected_room_removed_rejoin_packet_label"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "replacement_ui_slot not_wired"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "rejoin_request_slot not_built"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "People/Rooms unread/mention aggregate packet"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "section_unread_aggregate_packet_label"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "header_badge_source local_zero_placeholder"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "aggregate_refresh_slot not_built"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "parent_chain_attribution partial_cache_only"
require_contains "apps/hepta-native/src/room/room_display_filter.rs" "SPACE_UNREAD_MENTION_FILTER_LOCAL_ZERO_EVIDENCE"
require_contains "apps/hepta-native/src/room/room_display_filter.rs" "Space unread/mention aggregate packet"
require_contains "apps/hepta-native/src/room/room_display_filter.rs" "room-display-filter zero source"
require_contains "apps/hepta-native/src/home/space_lobby.rs" "UnsubscribeFromSpaceRoomList is service lifecycle cleanup"
require_contains "apps/hepta-native/src/home/space_lobby.rs" "Banned membership edge"
require_contains "apps/hepta-native/src/home/space_lobby.rs" "SPACE_LOBBY_REKNOCK_CANCEL_PRIOR_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/home/space_lobby.rs" "space_lobby_reknock_cancel_prior_packet_label"
require_contains "apps/hepta-native/src/home/space_lobby.rs" "tree_reknock_action_slot not_exposed"
require_contains "apps/hepta-native/src/home/add_room.rs" "ADD_ROOM_REKNOCK_CANCEL_PRIOR_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/home/add_room.rs" "add_room_reknock_cancel_prior_packet_label"
require_contains "apps/hepta-native/src/home/add_room.rs" "cancel_prior_request_slot not_built"
require_contains "apps/hepta-native/src/space_service_sync.rs" "SpaceRequest::PaginateSpaceRoomList"
require_contains "apps/hepta-native/src/space_service_sync.rs" "TODO: handle Knocked spaces"

require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_rooms_list_pagination_adapter_local_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_rooms_list_load_more_pagination_packet_ready"
require_contains "$FIXTURE_CONTRACT_PATH" 'data-native-telegram-rooms-list-load-more-pagination-packet=loaded-counts-cursor-result-slots-local'
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_rooms_list_removed_room_selected_state_local_ready"
require_contains "$FIXTURE_CONTRACT_PATH" 'data-native-telegram-rooms-list-selected-room-removed-rejoin-packet=focus-rejoin-slots-local'
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_space_unread_filter_local_zero_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "rooms_list_section_unread_aggregate_packet_preview"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "space_unread_filter_aggregate_packet_preview"
require_contains "$FIXTURE_CONTRACT_PATH" 'data-native-telegram-rooms-list-unread-aggregate-packet=loaded-row-totals-local-zero-header'
require_contains "$FIXTURE_CONTRACT_PATH" 'data-native-telegram-space-unread-filter-aggregate-packet=joined-space-zero-source-local'
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_space_lobby_membership_edge_local_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_add_room_membership_edge_local_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "space_lobby_reknock_cancel_prior_packet_preview"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "add_room_reknock_cancel_prior_packet_preview"
require_contains "$FIXTURE_CONTRACT_PATH" 'data-native-telegram-space-lobby-reknock-cancel-prior-packet=tree-action-cancel-slot-local'
require_contains "$FIXTURE_CONTRACT_PATH" 'data-native-telegram-add-room-reknock-cancel-prior-packet=confirmed-reknock-cancel-slot-local'

readiness_status="not_provided"
readiness_path=""
native_screenshot_count=0
control_screenshot_count=0
packaging_ready=false
backend_contract_gates_ready=false

if [[ -n "$READINESS_DIR" ]]; then
  readiness_path="$READINESS_DIR/readiness.json"
  require_file "$readiness_path"
  readiness_status="$(jq -r '.status // "missing"' "$readiness_path")"
  if [[ "$readiness_status" != "ready" ]]; then
    echo "readiness status is not ready: $readiness_status" >&2
    exit 1
  fi
  native_screenshot_count="$(jq -r '.native.screenshot_count // 0' "$readiness_path")"
  control_screenshot_count="$(jq -r '.control_ui.screenshot_count // 0' "$readiness_path")"
  packaging_ready="$(jq -r '.native.packaging_gate_ready // false' "$readiness_path")"
  backend_contract_gates_ready="$(jq -r '.native_backend_contract_gates_ready // false' "$readiness_path")"
  if (( native_screenshot_count < 28 )); then
    echo "expected at least 28 native fixture screenshots, got $native_screenshot_count" >&2
    exit 1
  fi
  if (( control_screenshot_count < 3 )); then
    echo "expected at least 3 control screenshots, got $control_screenshot_count" >&2
    exit 1
  fi
  if [[ "$packaging_ready" != "true" ]]; then
    echo "native packaging gate not ready" >&2
    exit 1
  fi
  if [[ "$backend_contract_gates_ready" != "true" ]]; then
    echo "backend contract wave gates not ready" >&2
    exit 1
  fi
fi

json_report="$(jq -n \
  --arg product "Hepta Native" \
  --arg gate "spaces_room_membership_edge_backend_contract" \
  --arg status "ready" \
  --arg contract_path "$CONTRACT_PATH" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg readiness_path "$readiness_path" \
  --arg readiness_status "$readiness_status" \
  --argjson native_screenshot_count "$native_screenshot_count" \
  --argjson control_screenshot_count "$control_screenshot_count" \
  --argjson packaging_ready "$packaging_ready" \
  --argjson backend_contract_gates_ready "$backend_contract_gates_ready" \
  '{
    product: $product,
    gate: $gate,
    status: $status,
    contract_path: $contract_path,
    readiness_dir: $readiness_dir,
    readiness_path: $readiness_path,
    readiness_status: $readiness_status,
    verified_groups: [
      "reknock_cancel_prior_knock_contract",
      "selected_room_removed_rejoin_packet_contract",
      "room_list_pagination_contract",
      "room_list_load_more_pagination_packet",
      "space_unread_mention_aggregate_contract"
    ],
    current_ui_state: "read_sync_load_more_and_unread_aggregate_packets_local",
    native_screenshot_count: $native_screenshot_count,
    control_screenshot_count: $control_screenshot_count,
    packaging_ready: $packaging_ready,
    backend_contract_gates_ready: $backend_contract_gates_ready,
    side_effects: {
      matrix_login: false,
      matrix_request_submitted: false,
      membership_mutation: false,
      room_state_mutation: false,
      gateway_call: false,
      provider_invoked: false,
      channel_delivery: false,
      external_mutation: false
    }
  }')"

if [[ -n "$REPORT_PATH" ]]; then
  mkdir -p "$(dirname "$REPORT_PATH")"
  printf '%s\n' "$json_report" > "$REPORT_PATH"
else
  printf '%s\n' "$json_report"
fi
