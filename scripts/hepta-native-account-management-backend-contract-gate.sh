#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

CONTRACT_PATH="${HEPTA_NATIVE_ACCOUNT_MANAGEMENT_BACKEND_CONTRACT_PATH:-docs/architecture/HEPTA_NATIVE_ACCOUNT_MANAGEMENT_BACKEND_CONTRACT_2026-06-15.md}"
READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.p6-matrix-link-server-refresh-20260615}"
HANDOFF_PATH="$READINESS_DIR/native-base-gap-backend-handoff.json"

require_file() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required file: %s\n' "$path" >&2
    exit 1
  fi
}

require_marker() {
  local path="$1"
  local marker="$2"
  if ! grep -Fq "$marker" "$path"; then
    printf 'Missing marker in %s: %s\n' "$path" "$marker" >&2
    exit 1
  fi
}

require_file "$CONTRACT_PATH"
require_file "$HANDOFF_PATH"

MARKERS=(
  'HEPTA_NATIVE_ACCOUNT_MANAGEMENT_BACKEND_CONTRACT_READY:true'
  'HEPTA_NATIVE_ACCOUNT_MANAGEMENT_BACKEND_CONTRACT_DATE:2026-06-15'
  'HEPTA_NATIVE_ACCOUNT_MANAGEMENT_BACKEND_CONTRACT_GAP_ID:account_management'
  'HEPTA_NATIVE_ACCOUNT_MANAGEMENT_BACKEND_CONTRACT_LIVE_WIRING:matrix_getowndevice_getdevices_retry_setdisplayname_resubmit_current_device_renamedevice_browser_portal_homeserver_opener_live;password_sso_revoke_trust_cross_session_device_mutations_blocked'
  'HEPTA_NATIVE_ACCOUNT_MANAGEMENT_BACKEND_CONTRACT_BOUNDARY:no-gateway-runtime-provider-auth-telegram-delivery-mutation'
  'Current Device Result'
  'All-Device Directory Result'
  'Display Name Result'
  'Current-Device Rename Result'
  'Account Portal Route Result'
  'Password And SSO Action Result'
  'Cross-Session Revoke/Trust Result'
  'Device Delete/Trust Result'
  'Password/SSO/Revoke/Trust/Delete Result Taxonomy UI Packet'
  'dedicated_portal_operation_id'
  'device_delete_operation_id'
  'audit_redaction'
  'Retry, Stale-Session, And Secret Rules'
  'PositiveConfirmationModal'
)

for marker in "${MARKERS[@]}"; do
  require_marker "$CONTRACT_PATH" "$marker"
done

jq -e '
  (.items | length) == 12
  and (.items | map(select(
    .id == "account_management"
    and .status == "partial_live_backend_contract_remaining"
    and .ui_lane_state == "complete"
    and .next_owner_lane == "backend_contract"
    and .acceptance_state.current_ui_evidence == "machine_checked"
    and .acceptance_state.side_effect_boundary == "locked"
    and .acceptance_state.ui_contract == "ready"
    and .acceptance_state.backend_contract == "required"
    and .acceptance_state.live_wiring == "matrix_getowndevice_getdevices_retry_setdisplayname_resubmit_current_device_renamedevice_browser_portal_homeserver_opener_live;password_sso_revoke_trust_cross_session_device_mutations_blocked"
    and (.required_backend_contracts | index("password/SSO action result") != null)
    and (.required_backend_contracts | index("cross-session revoke/trust result") != null)
    and (.required_backend_contracts | index("device delete/trust and account/profile mutations beyond display name/current-device rename") != null)
  )) | length) == 1
' "$HANDOFF_PATH" >/dev/null

if grep -Eiq '(password|token|sso code|refresh token|pushkey|session secret):[[:space:]]*[^`[:space:]]+' "$CONTRACT_PATH"; then
  printf 'Potential unredacted secret-shaped field in %s\n' "$CONTRACT_PATH" >&2
  exit 1
fi

jq -n \
  --arg status "ready" \
  --arg contract_path "$CONTRACT_PATH" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg backend_handoff_path "$HANDOFF_PATH" \
  '{
    product:"Hepta Native",
    gate:"account_management_backend_contract",
    status:$status,
    contract_path:$contract_path,
    readiness_dir:$readiness_dir,
    backend_handoff_path:$backend_handoff_path,
    gap_id:"account_management",
    verified_live_wiring:"matrix_getowndevice_getdevices_retry_setdisplayname_resubmit_current_device_renamedevice_browser_portal_homeserver_opener_live;password_sso_revoke_trust_cross_session_device_mutations_blocked",
    required_contract_groups:[
      "current device result",
      "all-device directory result",
      "display name result",
      "current-device rename result",
      "account portal route result",
      "password and SSO action result",
      "cross-session revoke/trust result",
      "device delete/trust result",
      "password/SSO/revoke/trust/delete result taxonomy UI packet",
      "secret redaction rules"
    ],
    side_effects:{
      matrix_login:false,
      gateway_call:false,
      provider_invoked:false,
      channel_delivery:false,
      external_mutation:false
    }
  }'
