#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_RUNTIME_APPSERVER_REPLAY.md"
EXPECTED_SELECTED_COUNT="${HEPTA_UPSTREAM_CODEX_RUNTIME_APPSERVER_EXPECTED_COUNT:-462}"

echo "[hepta-upstream-codex-runtime-appserver-replay] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_runtime_appserver_replay -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "runtime/app-server replay packet missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing runtime/app-server replay packet: $DOC" >&2
  exit 1
fi

require_doc_text "runtime-session-tool-mcp-appserver"
require_doc_text "Selected changed paths: \`462\`"
require_doc_text "app-server protocol"
require_doc_text "session thread-store"
require_doc_text "tool invocation"
require_doc_text "MCP client server"
require_doc_text "exec-server hook"
require_doc_text "No active runtime promotion"
require_doc_text "No active app-server promotion"
require_doc_text "No active tool/MCP promotion"
require_doc_text "No active Codex engine dependency"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg replay "upstream-codex-runtime-appserver-replay-packet" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-runtime-appserver-absorption.sh" \
    --arg replay_gate "scripts/hepta-upstream-codex-runtime-appserver-replay.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    --argjson selected_count "$EXPECTED_SELECTED_COUNT" \
    '{
      product:$product,
      status:"ready",
      replay_id:$replay,
      manifest:$manifest,
      replay_packet_path:$doc,
      selected_bucket:{
        id:"runtime-session-tool-mcp-appserver",
        risk:"p0_runtime",
        selected_changed_file_count:$selected_count,
        replay_surface_count:7,
        replay_ready:true
      },
      gates:{
        source_absorption_gate:$source_gate,
        replay_gate:$replay_gate,
        active_dependency_isolation_gate:$active_dependency_gate
      },
      replay_surfaces:[
        "app-server protocol schema and route-event replay",
        "app-server daemon and transport boundary replay",
        "session thread-store and lifecycle replay",
        "tool invocation and tool-policy replay",
        "MCP client server and request-envelope replay",
        "exec-server hook and runtime event-loop replay",
        "side-effect boundary and active dependency isolation replay"
      ],
      absorption_policy:{
        p0_runtime_review_required:true,
        app_server_protocol_replay_ready:true,
        session_thread_replay_ready:true,
        tool_mcp_replay_ready:true,
        exec_hook_replay_ready:true,
        active_runtime_promotion_allowed:false,
        active_app_server_promotion_allowed:false,
        active_tool_mcp_promotion_allowed:false,
        active_runtime_code_wiring_allowed:false,
        active_runtime_dependency_allowed:false,
        active_runtime_auto_rebase_allowed:false,
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

echo "Hepta upstream Codex runtime/app-server replay gate passed"
