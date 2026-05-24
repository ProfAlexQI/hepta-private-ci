#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
BASE_HEAD="${HEPTA_UPSTREAM_CODEX_DIFF_BASE_HEAD:-108234b5ebe6941764a6b8edbb37b2aa04369f07}"
TARGET_REF="${HEPTA_UPSTREAM_CODEX_DIFF_TARGET_REF:-refs/remotes/openai-codex/main}"
TARGET_HEAD="${HEPTA_UPSTREAM_CODEX_DIFF_TARGET_HEAD:-}"
EXPECTED_SELECTED_COUNT="${HEPTA_UPSTREAM_CODEX_RUNTIME_APPSERVER_EXPECTED_COUNT:-462}"

echo "[hepta-upstream-codex-runtime-appserver-absorption] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_runtime_appserver -- --nocapture

validate_sha() {
  local label="$1"
  local value="$2"
  if [[ ! "$value" =~ ^[0-9a-fA-F]{40}$ ]]; then
    echo "invalid $label: expected 40-hex git object id, got '$value'" >&2
    exit 1
  fi
}

validate_sha "HEPTA_UPSTREAM_CODEX_DIFF_BASE_HEAD" "$BASE_HEAD"
if [[ -z "$TARGET_HEAD" ]]; then
  TARGET_HEAD="$(git rev-parse --verify "${TARGET_REF}^{commit}")"
  target_head_source="$TARGET_REF"
else
  validate_sha "HEPTA_UPSTREAM_CODEX_DIFF_TARGET_HEAD" "$TARGET_HEAD"
  target_head_source="env"
fi
validate_sha "target upstream head" "$TARGET_HEAD"

git cat-file -e "${BASE_HEAD}^{commit}"
git cat-file -e "${TARGET_HEAD}^{commit}"

diff_range="${BASE_HEAD}..${TARGET_HEAD}"
merge_base="$(git merge-base "$BASE_HEAD" "$TARGET_HEAD")"
if [[ "$merge_base" != "$BASE_HEAD" ]]; then
  echo "target head is not descended from baseline: merge_base=$merge_base base=$BASE_HEAD target=$TARGET_HEAD" >&2
  exit 1
fi

json_array_from_stdin() {
  jq -R -s 'split("\n") | map(select(length > 0))'
}

selected_paths_json="$(
  git diff --name-only "$diff_range" -- codex-rs |
    grep -E '(^codex-rs/(app-server|app-server-client|app-server-daemon|app-server-protocol|app-server-transport|core/src/(agent|context|session|state|tasks|tools|unified_exec)|codex-mcp|mcp-server|protocol|thread-store|hooks|exec-server)/|tool|mcp|session|thread)' |
    json_array_from_stdin
)"
selected_count="$(jq 'length' <<<"$selected_paths_json")"

paths_match_contract="$(
  jq --argjson expected "$EXPECTED_SELECTED_COUNT" \
    'length == $expected
     and all(.[]; startswith("codex-rs/"))
     and any(.[]; startswith("codex-rs/app-server/"))
     and any(.[]; startswith("codex-rs/app-server-protocol/"))
     and any(.[]; startswith("codex-rs/app-server-daemon/"))
     and any(.[]; startswith("codex-rs/app-server-transport/"))
     and any(.[]; startswith("codex-rs/core/src/tools/"))
     and any(.[]; startswith("codex-rs/core/src/session"))
     and any(.[]; startswith("codex-rs/core/src/state"))
     and any(.[]; startswith("codex-rs/codex-mcp/"))
     and any(.[]; startswith("codex-rs/mcp-server/"))
     and any(.[]; startswith("codex-rs/thread-store/"))
     and any(.[]; startswith("codex-rs/hooks/"))
     and any(.[]; startswith("codex-rs/exec-server/"))' \
    <<<"$selected_paths_json"
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg absorption "upstream-codex-runtime-appserver-absorption-contract" \
    --arg upstream "https://github.com/openai/codex" \
    --arg manifest "$MANIFEST" \
    --arg base_head "$BASE_HEAD" \
    --arg target_head "$TARGET_HEAD" \
    --arg target_head_source "$target_head_source" \
    --arg target_ref "$TARGET_REF" \
    --arg diff_range "$diff_range" \
    --arg merge_base "$merge_base" \
    --argjson selected_count "$selected_count" \
    --argjson expected_selected_count "$EXPECTED_SELECTED_COUNT" \
    --argjson selected_paths "$selected_paths_json" \
    --argjson paths_match_contract "$paths_match_contract" \
    '{
      product:$product,
      status:(if $paths_match_contract then "ready" else "attention" end),
      absorption_id:$absorption,
      upstream_repository:$upstream,
      manifest:$manifest,
      baseline_upstream_head:$base_head,
      target_upstream_head:$target_head,
      target_head_source:$target_head_source,
      target_ref:$target_ref,
      candidate_diff_range:$diff_range,
      merge_base:$merge_base,
      selected_bucket:{
        id:"runtime-session-tool-mcp-appserver",
        risk:"p0_runtime",
        selected_changed_file_count:$selected_count,
        expected_changed_file_count:$expected_selected_count,
        selected_paths_match_contract:$paths_match_contract,
        required_path_families:[
          "codex-rs/app-server",
          "codex-rs/app-server-protocol",
          "codex-rs/app-server-daemon",
          "codex-rs/app-server-transport",
          "codex-rs/core/src/tools",
          "codex-rs/core/src/session",
          "codex-rs/core/src/state",
          "codex-rs/codex-mcp",
          "codex-rs/mcp-server",
          "codex-rs/thread-store",
          "codex-rs/hooks",
          "codex-rs/exec-server"
        ],
        sample_paths:$selected_paths[0:80]
      },
      absorption_policy:{
        p0_runtime_review_required:true,
        requires_adapter_contract:true,
        requires_session_thread_replay:true,
        requires_tool_mcp_replay:true,
        requires_app_server_protocol_replay:true,
        requires_exec_hook_replay:true,
        active_runtime_promotion_allowed:false,
        active_app_server_promotion_allowed:false,
        active_tool_mcp_promotion_allowed:false,
        active_runtime_code_wiring_allowed:false,
        active_runtime_dependency_allowed:false,
        active_runtime_auto_rebase_allowed:false,
        public_release_claim_allowed:false,
        required_next_gates:[
          "map app-server protocol deltas to Hepta route and event contracts",
          "run session and thread-store replay before lifecycle promotion",
          "run tool and MCP replay before invocation promotion",
          "run exec and hook replay before runtime event-loop promotion",
          "keep active hepta-cli cargo tree free of tracked Codex engine crates"
        ]
      },
      gates:{
        source_ledger_gate:"scripts/hepta-upstream-codex-diff-ledger.sh",
        absorption_gate:"scripts/hepta-upstream-codex-runtime-appserver-absorption.sh",
        active_dependency_isolation_gate:"scripts/hepta-active-service-dependency-isolation.sh"
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

if [[ "$paths_match_contract" != "true" ]]; then
  echo "runtime/app-server absorption contract incomplete: selected_count=$selected_count expected=$EXPECTED_SELECTED_COUNT" >&2
  exit 1
fi

echo "Hepta upstream Codex runtime/app-server absorption contract gate passed"
