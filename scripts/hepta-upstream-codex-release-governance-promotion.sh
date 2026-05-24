#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_RELEASE_GOVERNANCE_PROMOTION.md"

echo "[hepta-upstream-codex-release-governance-promotion] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_release_governance_promotion -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "release-governance promotion packet missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing release-governance promotion packet: $DOC" >&2
  exit 1
fi

require_doc_text "release-governance-claim-promotion-packet"
require_doc_text "Selected changed paths: \`22\`"
require_doc_text "Release claim taxonomy ready"
require_doc_text "Package and install-context governance ready"
require_doc_text "Plugin marketplace policy ready"
require_doc_text "Operator approval model ready"
require_doc_text "Watchdog and soak evidence ready"
require_doc_text "Public claim boundary ready"
require_doc_text "Ready promotion conditions: \`7 / 7\`"
require_doc_text "Promotion packet ready: \`true\`"
require_doc_text "Public release claim allowed: \`false\`"
require_doc_text "Public GA claim allowed: \`false\`"
require_doc_text "Release artifact write allowed: \`false\`"
require_doc_text "No channel delivery"
require_doc_text "No gateway RPC"
require_doc_text "No public release claim"
require_doc_text "No public release publication"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg promotion "release-governance-claim-promotion-packet" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-product-governance-translation.sh" \
    --arg promotion_gate "scripts/hepta-upstream-codex-release-governance-promotion.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      promotion_id:$promotion,
      manifest:$manifest,
      promotion_packet_path:$doc,
      selected_bucket:{
        id:"product-doc-release-governance",
        risk:"p2_product",
        selected_changed_file_count:22
      },
      gates:{
        source_translation_gate:$source_gate,
        promotion_gate:$promotion_gate,
        active_dependency_isolation_gate:$active_dependency_gate
      },
      promotion_conditions:{
        ready_promotion_condition_count:7,
        required_promotion_condition_count:7,
        release_claim_taxonomy_ready:true,
        package_install_context_ready:true,
        plugin_marketplace_policy_ready:true,
        operator_approval_model_ready:true,
        watchdog_soak_evidence_ready:true,
        public_claim_boundary_ready:true,
        side_effect_boundary_ready:true,
        promotion_packet_ready:true
      },
      active_promotion_decision:{
        public_release_claim_allowed:false,
        public_ga_claim_allowed:false,
        release_artifact_write_allowed:false,
        active_runtime_code_wiring_allowed:false,
        active_runtime_dependency_allowed:false,
        upstream_auto_rebase_allowed:false
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

echo "Hepta upstream Codex release-governance promotion gate passed"
