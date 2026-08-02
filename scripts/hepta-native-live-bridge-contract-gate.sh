#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

APP_DIR="${HEPTA_NATIVE_LIVE_BRIDGE_APP_DIR:-apps/hepta-native}"
CONTRACT_PATH="${HEPTA_NATIVE_LIVE_BRIDGE_CONTRACT_PATH:-$APP_DIR/hepta-live-bridge-backend-contract-v1.json}"
DOC_PATH="${HEPTA_NATIVE_LIVE_BRIDGE_HANDOFF_PATH:-docs/architecture/HEPTA_NATIVE_LIVE_BRIDGE_BACKEND_HANDOFF_2026-08-02.md}"
BRIDGE_MOD_PATH="${HEPTA_NATIVE_LIVE_BRIDGE_MOD_PATH:-$APP_DIR/src/hepta_bridge/mod.rs}"
BRIDGE_ADAPTER_PATH="${HEPTA_NATIVE_LIVE_BRIDGE_ADAPTER_PATH:-$APP_DIR/src/hepta_bridge/adapter.rs}"
LIVE_POLICY_PATH="${HEPTA_NATIVE_LIVE_BRIDGE_POLICY_PATH:-$APP_DIR/src/hepta_bridge/live_policy.rs}"
VALIDATOR_PATH="${HEPTA_NATIVE_LIVE_BRIDGE_VALIDATOR_PATH:-scripts/lib/hepta-native-live-bridge-envelope-v1.jq}"
GATEWAY_SOURCE_ROOT="${HEPTA_NATIVE_LIVE_BRIDGE_GATEWAY_SOURCE_ROOT:-codex-rs/hepta-native-gateway/src}"
OUTPUT_PATH=""

usage() {
  cat <<'EOF'
Usage: scripts/hepta-native-live-bridge-contract-gate.sh [--output PATH]

Audits the current read-only gateway response shapes and writes a structured,
fail-closed backend blocker receipt. This gate never performs a network request
and never produces a live readiness receipt.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --output)
      [[ $# -ge 2 ]] || { printf '%s\n' 'missing value for --output' >&2; exit 64; }
      OUTPUT_PATH="$2"
      shift 2
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      printf 'unknown argument: %s\n' "$1" >&2
      usage >&2
      exit 64
      ;;
  esac
done

require_file() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'missing required live-bridge contract file: %s\n' "$path" >&2
    exit 1
  fi
}

require_marker() {
  local path="$1"
  local marker="$2"
  if ! grep -Fq -- "$marker" "$path"; then
    printf 'missing live-bridge contract marker in %s: %s\n' "$path" "$marker" >&2
    exit 1
  fi
}

for path in \
  "$CONTRACT_PATH" \
  "$DOC_PATH" \
  "$BRIDGE_MOD_PATH" \
  "$BRIDGE_ADAPTER_PATH" \
  "$LIVE_POLICY_PATH" \
  "$VALIDATOR_PATH"
do
  require_file "$path"
done

if [[ ! -d "$GATEWAY_SOURCE_ROOT" ]]; then
  printf 'missing gateway source root: %s\n' "$GATEWAY_SOURCE_ROOT" >&2
  exit 1
fi

jq -e '
  .schema == "hepta-native-live-bridge-backend-contract-v1"
  and .contract_version == 1
  and .owner_handoff.ui_lane == "hepta-ui"
  and .owner_handoff.backend_lane_required == true
  and .owner_handoff.runtime_gateway_auth_files_modified_by_ui_lane == false
  and .canonical_endpoint.method == "GET"
  and .canonical_endpoint.path == "/api/hepta-native-bridge/v1/snapshot"
  and .canonical_endpoint.loopback_only == true
  and .canonical_endpoint.explicit_opt_in_required == true
  and .canonical_endpoint.matrix_login_required == true
  and .canonical_endpoint.authenticated_session_binding_required == true
  and .canonical_endpoint.mutation_allowed == false
  and .response_contract.rust_type == "hepta_native::hepta_bridge::BridgeUpdate"
  and .response_contract.update_type == "snapshot"
  and .response_contract.schema_version == 1
  and .response_contract.raw_source_payload_allowed == false
  and .native_capabilities.snapshot == "only_after_all_preflight_checks"
  and .native_capabilities.subscribe == false
  and .native_capabilities.prepare == false
  and .native_capabilities.confirm == false
  and .native_capabilities.reject == false
  and .native_capabilities.cancel == false
  and (.candidate_endpoint_audit | length) == 6
  and (.candidate_endpoint_audit | all(.authoritative_bridge_snapshot == false))
  and ([.candidate_endpoint_audit[].path] | sort) == ([
    "/api/activity",
    "/api/approvals",
    "/api/gateway-runtime",
    "/api/operator-snapshot",
    "/api/session-activity",
    "/api/task/<task_id>"
  ] | sort)
  and .current_implementation.canonical_endpoint_registered == false
  and .current_implementation.authoritative_envelope_available == false
  and .current_implementation.authenticated_session_binding_available == false
  and .current_implementation.live_adapter_available == false
  and .current_implementation.live_receipt_available == false
  and .current_implementation.production_default_adapter == "disabled"
  and .current_implementation.hepta_live_bridge_ready == false
  and (.promotion_requirements | length) >= 8
' "$CONTRACT_PATH" >/dev/null

for marker in \
  'HEPTA_NATIVE_LIVE_BRIDGE_BACKEND_HANDOFF_VERSION:1' \
  'HEPTA_NATIVE_LIVE_BRIDGE_CANONICAL_ENDPOINT:GET /api/hepta-native-bridge/v1/snapshot' \
  'HEPTA_NATIVE_LIVE_BRIDGE_LOOPBACK_ONLY:true' \
  'HEPTA_NATIVE_LIVE_BRIDGE_EXPLICIT_OPT_IN_REQUIRED:true' \
  'HEPTA_NATIVE_LIVE_BRIDGE_MATRIX_LOGIN_REQUIRED:true' \
  'HEPTA_NATIVE_LIVE_BRIDGE_MUTATIONS_ENABLED:false' \
  'HEPTA_NATIVE_LIVE_BRIDGE_CURRENT_READY:false' \
  'HEPTA_NATIVE_LIVE_BRIDGE_BOUNDARY:no-runtime-gateway-auth-mutation-implemented-by-ui-lane'
do
  require_marker "$DOC_PATH" "$marker"
done

for marker in \
  'DisabledBridgeAdapter' \
  'GuardedBridgeAdapter<DisabledBridgeAdapter>' \
  'GuardedBridgeAdapter::disabled()'
do
  require_marker "$BRIDGE_MOD_PATH" "$marker"
done

require_marker "$BRIDGE_ADAPTER_PATH" 'pub(crate) struct DisabledBridgeAdapter'
require_marker "$BRIDGE_ADAPTER_PATH" 'BridgeCapabilities::default()'

for marker in \
  'HEPTA_LIVE_BRIDGE_SNAPSHOT_PATH' \
  'MatrixSessionNotAuthenticated' \
  'ExplicitOptInMissing' \
  'EndpointNotLoopback' \
  'AuthenticatedSessionBindingMissing' \
  'AuthoritativeSnapshotContractMissing' \
  'snapshot: self.snapshot_enabled' \
  'prepare: false' \
  'confirm: false' \
  'reject: false' \
  'cancel: false'
do
  require_marker "$LIVE_POLICY_PATH" "$marker"
done

require_marker "$VALIDATOR_PATH" 'hepta_native_live_bridge_envelope_v1_valid'

for endpoint in \
  '/api/operator-snapshot' \
  '/api/session-activity' \
  '/api/task/<task_id>' \
  '/api/approvals' \
  '/api/activity' \
  '/api/gateway-runtime'
do
  if ! grep -R -Fq -- "$endpoint" "$GATEWAY_SOURCE_ROOT"; then
    printf 'audited gateway endpoint is missing from source: %s\n' "$endpoint" >&2
    exit 1
  fi
done

# These source markers establish the real response classifications. They are
# deliberately checked in gateway source, not inferred from route names.
for marker in \
  'native_sessions_json("/session-activity --json", "native_session_activity")' \
  'Some(task_id)' \
  'pending_approval_count = 0usize' \
  'native_events_json(NativeEventSurface::Activity, None)' \
  'native_gateway_json(context.options, &context.telegram_plugin)'
do
  if ! grep -R -Fq -- "$marker" "$GATEWAY_SOURCE_ROOT"; then
    printf 'gateway response-shape audit marker is missing: %s\n' "$marker" >&2
    exit 1
  fi
done

canonical_endpoint="$(jq -r '.canonical_endpoint.path' "$CONTRACT_PATH")"
if grep -R -Fq -- "$canonical_endpoint" "$GATEWAY_SOURCE_ROOT"; then
  printf '%s\n' 'canonical endpoint appeared in gateway source; the blocked-state contract must be replaced by an authenticated live integration review' >&2
  exit 1
fi

source_commit="$(git rev-parse HEAD 2>/dev/null || printf '%s' unknown)"
if [[ -n "$(git status --porcelain --untracked-files=normal 2>/dev/null || true)" ]]; then
  worktree_dirty=true
else
  worktree_dirty=false
fi
candidate_endpoint_audit="$(jq -c '.candidate_endpoint_audit' "$CONTRACT_PATH")"
promotion_requirements="$(jq -c '.promotion_requirements' "$CONTRACT_PATH")"

receipt="$(jq -n \
  --arg schema 'hepta-native-live-bridge-blocker-receipt-v1' \
  --arg status 'blocked' \
  --arg source_commit "$source_commit" \
  --arg contract_path "$CONTRACT_PATH" \
  --arg handoff_path "$DOC_PATH" \
  --arg live_policy_path "$LIVE_POLICY_PATH" \
  --arg envelope_validator_path "$VALIDATOR_PATH" \
  --arg canonical_endpoint "$canonical_endpoint" \
  --argjson worktree_dirty "$worktree_dirty" \
  --argjson candidate_endpoint_audit "$candidate_endpoint_audit" \
  --argjson promotion_requirements "$promotion_requirements" \
  '{
    schema:$schema,
    receipt_version:1,
    product:"Hepta Native",
    status:$status,
    contract_gate_ready:true,
    backend_handoff_ready:true,
    hepta_live_bridge_ready:false,
    source:{
      commit:$source_commit,
      worktree_dirty:$worktree_dirty
    },
    evidence:{
      contract_path:$contract_path,
      handoff_path:$handoff_path,
      live_policy_path:$live_policy_path,
      envelope_validator_path:$envelope_validator_path
    },
    canonical_endpoint:{
      method:"GET",
      path:$canonical_endpoint,
      registered:false,
      loopback_only:true,
      explicit_opt_in_required:true,
      matrix_login_required:true,
      authenticated_session_binding_available:false
    },
    capabilities:{
      snapshot:false,
      subscribe:false,
      prepare:false,
      confirm:false,
      reject:false,
      cancel:false
    },
    candidate_endpoint_audit:$candidate_endpoint_audit,
    actual_request:{
      performed:false,
      endpoint:null,
      method:null,
      http_status:null,
      request_descriptor_sha256:null,
      response_bytes:null,
      response_sha256:null,
      session_match:null,
      correlation_match:null
    },
    blockers:[
      "canonical_snapshot_endpoint_not_registered",
      "authoritative_bridge_update_envelope_not_available",
      "authenticated_native_session_binding_not_available",
      "snapshot_only_live_adapter_not_implemented",
      "actual_live_request_not_performed",
      "actual_live_receipt_not_available"
    ],
    promotion_requirements:$promotion_requirements,
    side_effects:{
      network_request_performed:false,
      matrix_login_performed:false,
      provider_invoked:false,
      channel_delivery_performed:false,
      cursor_written:false,
      gateway_mutation_performed:false,
      external_mutation_performed:false
    },
    claim_boundaries:{
      source_audit_is_live_receipt:false,
      synthetic_envelope_is_live_receipt:false,
      legacy_report_endpoint_is_task_truth:false,
      matrix_event_is_hepta_receipt:false,
      public_or_live_readiness_claimed:false
    }
  }')"

if [[ -n "$OUTPUT_PATH" ]]; then
  output_dir="$(dirname "$OUTPUT_PATH")"
  mkdir -p "$output_dir"
  temp_output="$(mktemp "$output_dir/.hepta-native-live-bridge-blocker.XXXXXX")"
  printf '%s\n' "$receipt" > "$temp_output"
  mv "$temp_output" "$OUTPUT_PATH"
else
  printf '%s\n' "$receipt"
fi
