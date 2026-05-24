#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_PROVIDER_SECURITY_REPLAY.md"
EXPECTED_SELECTED_COUNT="${HEPTA_UPSTREAM_CODEX_PROVIDER_SECURITY_EXPECTED_COUNT:-104}"

echo "[hepta-upstream-codex-provider-security-replay] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_provider_security_replay -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "provider/security replay packet missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing provider/security replay packet: $DOC" >&2
  exit 1
fi

require_doc_text "provider-credential-sandbox-security"
require_doc_text "Selected changed paths: \`104\`"
require_doc_text "redacted provider contracts"
require_doc_text "credential redaction"
require_doc_text "approval-policy"
require_doc_text "sandbox and exec"
require_doc_text "network-proxy"
require_doc_text "No credential value read"
require_doc_text "No provider invocation"
require_doc_text "No active provider promotion"
require_doc_text "No active Codex engine dependency"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg replay "upstream-codex-provider-security-replay-packet" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-provider-security-absorption.sh" \
    --arg replay_gate "scripts/hepta-upstream-codex-provider-security-replay.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    --argjson selected_count "$EXPECTED_SELECTED_COUNT" \
    '{
      product:$product,
      status:"ready",
      replay_id:$replay,
      manifest:$manifest,
      replay_packet_path:$doc,
      selected_bucket:{
        id:"provider-credential-sandbox-security",
        risk:"p0_security",
        selected_changed_file_count:$selected_count,
        replay_surface_count:6,
        replay_ready:true
      },
      gates:{
        source_absorption_gate:$source_gate,
        replay_gate:$replay_gate,
        active_dependency_isolation_gate:$active_dependency_gate
      },
      replay_surfaces:[
        "redacted provider catalog and endpoint contracts",
        "auth login config and credential redaction",
        "approval-policy dry-run matrix",
        "sandbox and exec request replay",
        "network-proxy policy replay",
        "side-effect boundary and operator approval replay"
      ],
      absorption_policy:{
        p0_security_review_required:true,
        redacted_provider_contract_ready:true,
        auth_credential_redaction_ready:true,
        approval_policy_replay_ready:true,
        sandbox_exec_replay_ready:true,
        network_policy_replay_ready:true,
        active_provider_promotion_allowed:false,
        active_security_policy_promotion_allowed:false,
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

echo "Hepta upstream Codex provider/security replay gate passed"
