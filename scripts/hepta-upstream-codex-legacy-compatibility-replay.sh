#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_LEGACY_COMPATIBILITY_REPLAY.md"
EXPECTED_SELECTED_COUNT="${HEPTA_UPSTREAM_CODEX_LEGACY_COMPATIBILITY_EXPECTED_COUNT:-128}"

echo "[hepta-upstream-codex-legacy-compatibility-replay] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_legacy_compatibility_replay -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "legacy CLI/TUI compatibility replay packet missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing legacy CLI/TUI compatibility replay packet: $DOC" >&2
  exit 1
fi

require_doc_text "legacy-cli-tui-compatibility"
require_doc_text "Selected changed paths: \`128\`"
require_doc_text "CLI command shape"
require_doc_text "TUI presentation"
require_doc_text "code-mode runtime"
require_doc_text "terminal detection"
require_doc_text "No active CLI/TUI promotion"
require_doc_text "No active runtime code wiring"
require_doc_text "No active Codex engine dependency"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg replay "upstream-codex-legacy-compatibility-replay-packet" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-legacy-compatibility-absorption.sh" \
    --arg replay_gate "scripts/hepta-upstream-codex-legacy-compatibility-replay.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    --argjson selected_count "$EXPECTED_SELECTED_COUNT" \
    '{
      product:$product,
      status:"ready",
      replay_id:$replay,
      manifest:$manifest,
      replay_packet_path:$doc,
      selected_bucket:{
        id:"legacy-cli-tui-compatibility",
        risk:"p1_compatibility",
        selected_changed_file_count:$selected_count,
        replay_surface_count:5,
        replay_ready:true
      },
      gates:{
        source_absorption_gate:$source_gate,
        replay_gate:$replay_gate,
        active_dependency_isolation_gate:$active_dependency_gate
      },
      replay_surfaces:[
        "CLI command shape and argument contract replay",
        "TUI presentation and snapshot compatibility replay",
        "code-mode runtime callback and module-loader replay",
        "terminal detection PTY and utils CLI helper replay",
        "active dependency boundary and no-promotion replay"
      ],
      absorption_policy:{
        p1_compatibility_review_required:true,
        cli_command_contract_ready:true,
        tui_presentation_replay_ready:true,
        code_mode_replay_ready:true,
        terminal_helper_replay_ready:true,
        active_cli_tui_promotion_allowed:false,
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

echo "Hepta upstream Codex legacy CLI/TUI compatibility replay gate passed"
