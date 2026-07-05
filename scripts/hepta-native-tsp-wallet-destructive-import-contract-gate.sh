#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

CONTRACT_PATH="docs/architecture/HEPTA_NATIVE_TSP_WALLET_DESTRUCTIVE_IMPORT_BACKEND_CONTRACT_2026-06-15.md"
READINESS_DIR="${HEPTA_NATIVE_TSP_WALLET_CONTRACT_READINESS_DIR:-${HEPTA_UI_PRODUCT_READINESS_DIR:-}}"
REPORT_PATH="${HEPTA_NATIVE_TSP_WALLET_CONTRACT_REPORT:-}"

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
require_file "apps/hepta-native/src/tsp/mod.rs"
require_file "apps/hepta-native/src/tsp/wallet_entry/mod.rs"
require_file "apps/hepta-native/src/tsp/tsp_settings_screen.rs"
require_file "apps/hepta-native/src/tsp/create_wallet_modal.rs"
require_file "apps/hepta-native/src/tsp/create_did_modal.rs"
require_file "apps/hepta-native/src/tsp/verify_user.rs"
require_file "apps/hepta-native/src/home/hepta_telegram_base_contract.rs"

require_contains "$CONTRACT_PATH" "Delete Wallet Contract"
require_contains "$CONTRACT_PATH" "Import Wallet Contract"
require_contains "$CONTRACT_PATH" "Worker Receipt/Result Contract"
require_contains "$CONTRACT_PATH" "Pending Creation Cancel Contract"
require_contains "$CONTRACT_PATH" "Association Cancel/Remove Contract"
require_contains "$CONTRACT_PATH" "no deletion outside the verified wallet path"
require_contains "$CONTRACT_PATH" "Current UI delete preflight/result packet"
require_contains "$CONTRACT_PATH" "filesystem_result_taxonomy"
require_contains "$CONTRACT_PATH" "retry_cancel_policy"
require_contains "$CONTRACT_PATH" "password is handled only in modal-local memory"
require_contains "$CONTRACT_PATH" "Current UI import result taxonomy packet"
require_contains "$CONTRACT_PATH" "stable operation id"
require_contains "$CONTRACT_PATH" "remove association must specify"

require_contains "apps/hepta-native/src/tsp/wallet_entry/mod.rs" "TSP_WALLET_DELETE_BLOCKED_METADATA_EVIDENCE"
require_contains "apps/hepta-native/src/tsp/wallet_entry/mod.rs" "TSP_WALLET_DELETE_PREFLIGHT_RESULT_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/tsp/wallet_entry/mod.rs" "Clicking Delete Wallet still only emits a warning popup"
require_contains "apps/hepta-native/src/tsp/wallet_entry/mod.rs" "tsp_delete_wallet_preflight_result_packet_label"
require_contains "apps/hepta-native/src/tsp/wallet_entry/mod.rs" "path_validation_slot backend_required_exists_regular_app_owned_single_scope"
require_contains "apps/hepta-native/src/tsp/wallet_entry/mod.rs" "filesystem_result_taxonomy"
require_contains "apps/hepta-native/src/tsp/wallet_entry/mod.rs" "retry_cancel_policy confirmation_gated_idempotent_retry_cancel_sends_no_request"
require_contains "apps/hepta-native/src/tsp/wallet_entry/mod.rs" "No TspRequest::DeleteWallet"
require_contains "apps/hepta-native/src/tsp/tsp_settings_screen.rs" "TSP_WALLET_IMPORT_BLOCKED_METADATA_EVIDENCE"
require_contains "apps/hepta-native/src/tsp/tsp_settings_screen.rs" "TSP_WALLET_IMPORT_PREFLIGHT_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/tsp/tsp_settings_screen.rs" "TSP_WALLET_IMPORT_RESULT_TAXONOMY_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/tsp/tsp_settings_screen.rs" "TSP_WORKER_RECEIPT_RESULT_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/tsp/tsp_settings_screen.rs" "starts no file picker"
require_contains "apps/hepta-native/src/tsp/tsp_settings_screen.rs" "password_state not_collected"
require_contains "apps/hepta-native/src/tsp/tsp_settings_screen.rs" "persistence_result not_started"
require_contains "apps/hepta-native/src/tsp/tsp_settings_screen.rs" "tsp_import_wallet_result_taxonomy_packet_label"
require_contains "apps/hepta-native/src/tsp/tsp_settings_screen.rs" "operation_id_slot not_assigned"
require_contains "apps/hepta-native/src/tsp/tsp_settings_screen.rs" "vault_open_result opened|invalid_password|unsupported_vault|corrupted_database|already_imported|duplicate_path|permission_denied not_wired"
require_contains "apps/hepta-native/src/tsp/tsp_settings_screen.rs" "audit_redaction_policy no_password_token_private_vid_key_material_raw_path"
require_contains "apps/hepta-native/src/tsp/tsp_settings_screen.rs" "worker_receipt Cx_post_action"
require_contains "apps/hepta-native/src/tsp/tsp_settings_screen.rs" "result_mapping success_error_canceled_stale_local_taxonomy"
require_contains "apps/hepta-native/src/tsp/tsp_settings_screen.rs" "No new TspRequest"
require_contains "apps/hepta-native/src/tsp/mod.rs" "TSP_DELETE_WALLET_REQUEST_BLOCKED_EVIDENCE"
require_contains "apps/hepta-native/src/tsp/mod.rs" "TSP_DELETE_WALLET_RESULT_TAXONOMY_EVIDENCE"
require_contains "apps/hepta-native/src/tsp/mod.rs" "DeleteWallet request ignored"
require_contains "apps/hepta-native/src/tsp/mod.rs" "result_taxonomy request_state blocked_before_execution"
require_contains "apps/hepta-native/src/tsp/mod.rs" "does not delete files"
require_contains "apps/hepta-native/src/tsp/create_wallet_modal.rs" "TSP_WALLET_PENDING_CANCEL_OPERATION_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/tsp/create_wallet_modal.rs" "operation_id missing_backend_contract"
require_contains "apps/hepta-native/src/tsp/create_wallet_modal.rs" "stale_result_policy backend_operation_id_required"
require_contains "apps/hepta-native/src/tsp/create_did_modal.rs" "TSP_DID_PENDING_CANCEL_OPERATION_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/tsp/create_did_modal.rs" "operation_id missing_backend_contract"
require_contains "apps/hepta-native/src/tsp/create_did_modal.rs" "stale_result_policy backend_operation_id_required"
require_contains "apps/hepta-native/src/tsp/verify_user.rs" "TSP_ASSOCIATION_BLOCKED_METADATA_EVIDENCE"
require_contains "apps/hepta-native/src/tsp/verify_user.rs" "TSP_ASSOCIATION_CANCEL_REMOVE_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/tsp/verify_user.rs" "request_id missing_backend_contract"
require_contains "apps/hepta-native/src/tsp/verify_user.rs" "receive_loop_scope backend_required"
require_contains "apps/hepta-native/src/tsp/verify_user.rs" "Remove TSP Association is not implemented"

require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_tsp_wallet_delete_blocked_metadata_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_tsp_wallet_delete_preflight_result_packet_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_tsp_pending_cancel_operation_packet_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_tsp_wallet_import_blocked_metadata_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_tsp_wallet_import_preflight_packet_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_tsp_wallet_import_result_taxonomy_packet_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_tsp_worker_receipt_result_packet_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_tsp_association_cancel_local_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_tsp_association_cancel_remove_packet_ready"

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
  --arg gate "tsp_wallet_destructive_import_backend_contract" \
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
      "delete_wallet_destructive_contract",
      "delete_wallet_preflight_result_taxonomy_packet",
      "import_wallet_preflight_result_contract",
      "import_wallet_result_taxonomy_packet",
      "worker_receipt_result_packet",
      "pending_creation_cancel_contract",
      "association_cancel_remove_contract"
    ],
    current_ui_state: "visible_delete_preflight_result_taxonomy_import_preflight_result_taxonomy_worker_receipt_pending_cancel_and_association_operation_packets_warning_only",
    native_screenshot_count: $native_screenshot_count,
    control_screenshot_count: $control_screenshot_count,
    packaging_ready: $packaging_ready,
    backend_contract_gates_ready: $backend_contract_gates_ready,
    side_effects: {
      matrix_login: false,
      matrix_request_submitted: false,
      wallet_file_deleted: false,
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
