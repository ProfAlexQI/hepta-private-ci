#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_PROVIDER_SECURITY_PROMOTION.md"

echo "[hepta-upstream-codex-provider-security-promotion] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_provider_security_promotion -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "provider/security promotion packet missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing provider/security promotion packet: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-provider-security-promotion-packet"
require_doc_text "Selected changed paths: \`104\`"
require_doc_text "Redacted provider contract ready"
require_doc_text "Auth credential redaction ready"
require_doc_text "Approval-policy replay ready"
require_doc_text "Sandbox and exec replay ready"
require_doc_text "Network policy replay ready"
require_doc_text "Operator approval model ready"
require_doc_text "Ready promotion conditions: \`7 / 7\`"
require_doc_text "Promotion packet ready: \`true\`"
require_doc_text "Active provider promotion allowed: \`false\`"
require_doc_text "No credential value read"
require_doc_text "No provider invocation"
require_doc_text "No active Codex engine dependency"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg promotion "upstream-codex-provider-security-promotion-packet" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-provider-security-replay.sh" \
    --arg promotion_gate "scripts/hepta-upstream-codex-provider-security-promotion.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      promotion_id:$promotion,
      manifest:$manifest,
      promotion_packet_path:$doc,
      selected_bucket:{
        id:"provider-credential-sandbox-security",
        risk:"p0_security",
        selected_changed_file_count:104
      },
      gates:{
        source_replay_gate:$source_gate,
        promotion_gate:$promotion_gate,
        active_dependency_isolation_gate:$active_dependency_gate
      },
      promotion_conditions:{
        ready_promotion_condition_count:7,
        required_promotion_condition_count:7,
        redacted_provider_contract_ready:true,
        auth_credential_redaction_ready:true,
        approval_policy_replay_ready:true,
        sandbox_exec_replay_ready:true,
        network_policy_replay_ready:true,
        operator_approval_model_ready:true,
        side_effect_boundary_ready:true,
        promotion_packet_ready:true
      },
      active_promotion_decision:{
        active_provider_promotion_allowed:false,
        active_security_policy_promotion_allowed:false,
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

echo "Hepta upstream Codex provider/security promotion gate passed"
