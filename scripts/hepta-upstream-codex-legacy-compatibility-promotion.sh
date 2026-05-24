#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_LEGACY_COMPATIBILITY_PROMOTION.md"

echo "[hepta-upstream-codex-legacy-compatibility-promotion] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_legacy_compatibility_promotion -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "legacy compatibility promotion packet missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing legacy compatibility promotion packet: $DOC" >&2
  exit 1
fi

require_doc_text "hepta-cli-tui-parity-promotion-packet"
require_doc_text "Selected changed paths: \`128\`"
require_doc_text "CLI command contract parity ready"
require_doc_text "TUI presentation parity ready"
require_doc_text "code-mode callback boundary ready"
require_doc_text "Terminal helper contract ready"
require_doc_text "Adapter shadow replay ready"
require_doc_text "Operator approval model ready"
require_doc_text "Ready promotion conditions: \`7 / 7\`"
require_doc_text "Promotion packet ready: \`true\`"
require_doc_text "Active CLI/TUI promotion allowed: \`false\`"
require_doc_text "Active TUI presentation promotion allowed: \`false\`"
require_doc_text "Active code-mode promotion allowed: \`false\`"
require_doc_text "No channel delivery"
require_doc_text "No gateway RPC"
require_doc_text "No active Codex engine dependency"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg promotion "hepta-cli-tui-parity-promotion-packet" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-legacy-compatibility-replay.sh" \
    --arg promotion_gate "scripts/hepta-upstream-codex-legacy-compatibility-promotion.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      promotion_id:$promotion,
      manifest:$manifest,
      promotion_packet_path:$doc,
      selected_bucket:{
        id:"legacy-cli-tui-compatibility",
        risk:"p1_compatibility",
        selected_changed_file_count:128
      },
      gates:{
        source_replay_gate:$source_gate,
        promotion_gate:$promotion_gate,
        active_dependency_isolation_gate:$active_dependency_gate
      },
      promotion_conditions:{
        ready_promotion_condition_count:7,
        required_promotion_condition_count:7,
        cli_command_contract_parity_ready:true,
        tui_presentation_parity_ready:true,
        code_mode_callback_boundary_ready:true,
        terminal_helper_contract_ready:true,
        adapter_shadow_replay_ready:true,
        operator_approval_model_ready:true,
        side_effect_boundary_ready:true,
        promotion_packet_ready:true
      },
      active_promotion_decision:{
        active_cli_tui_promotion_allowed:false,
        active_tui_presentation_promotion_allowed:false,
        active_code_mode_promotion_allowed:false,
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

echo "Hepta upstream Codex legacy compatibility promotion gate passed"
