#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_RUNTIME_APPSERVER_PROMOTION.md"

echo "[hepta-upstream-codex-runtime-appserver-promotion] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_runtime_appserver_promotion -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "runtime/app-server promotion packet missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing runtime/app-server promotion packet: $DOC" >&2
  exit 1
fi

require_doc_text "runtime-appserver-route-event-promotion-packet"
require_doc_text "Selected changed paths: \`462\`"
require_doc_text "App-server route and event contract ready"
require_doc_text "Session thread lifecycle contract ready"
require_doc_text "Tool/MCP request envelope ready"
require_doc_text "Exec hook event-loop replay ready"
require_doc_text "Adapter shadow replay ready"
require_doc_text "Operator approval model ready"
require_doc_text "Ready promotion conditions: \`7 / 7\`"
require_doc_text "Promotion packet ready: \`true\`"
require_doc_text "Active runtime promotion allowed: \`false\`"
require_doc_text "Active app-server promotion allowed: \`false\`"
require_doc_text "Active tool/MCP promotion allowed: \`false\`"
require_doc_text "No channel delivery"
require_doc_text "No gateway RPC"
require_doc_text "No active Codex engine dependency"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg promotion "runtime-appserver-route-event-promotion-packet" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-runtime-appserver-replay.sh" \
    --arg promotion_gate "scripts/hepta-upstream-codex-runtime-appserver-promotion.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      promotion_id:$promotion,
      manifest:$manifest,
      promotion_packet_path:$doc,
      selected_bucket:{
        id:"runtime-session-tool-mcp-appserver",
        risk:"p0_runtime",
        selected_changed_file_count:462
      },
      gates:{
        source_replay_gate:$source_gate,
        promotion_gate:$promotion_gate,
        active_dependency_isolation_gate:$active_dependency_gate
      },
      promotion_conditions:{
        ready_promotion_condition_count:7,
        required_promotion_condition_count:7,
        app_server_route_event_contract_ready:true,
        session_thread_lifecycle_contract_ready:true,
        tool_mcp_request_envelope_ready:true,
        exec_hook_event_loop_replay_ready:true,
        adapter_shadow_replay_ready:true,
        operator_approval_model_ready:true,
        side_effect_boundary_ready:true,
        promotion_packet_ready:true
      },
      active_promotion_decision:{
        active_runtime_promotion_allowed:false,
        active_app_server_promotion_allowed:false,
        active_tool_mcp_promotion_allowed:false,
        active_runtime_code_wiring_allowed:false,
        active_runtime_dependency_allowed:false,
        public_release_claim_allowed:false
      },
      side_effects:{
        upstream_fetch_performed:false,
        upstream_merge_performed:false,
        upstream_checkout_performed:false,
        workspace_write:false,
        active_service_restart:false,
        credential_value_read:false,
        secret_file_read:false,
        provider_invocation:false,
        channel_delivery:false,
        gateway_rpc:false,
        public_release:false
      }
    }'
)"

printf '%s\n' "$report"

echo "Hepta upstream Codex runtime/app-server promotion gate passed"
