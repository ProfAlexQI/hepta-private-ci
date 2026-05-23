#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_PRODUCT_GOVERNANCE_TRANSLATION.md"
EXPECTED_SELECTED_COUNT="${HEPTA_UPSTREAM_CODEX_PRODUCT_GOVERNANCE_EXPECTED_COUNT:-22}"

echo "[hepta-upstream-codex-product-governance-translation] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_product_governance_translation -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "translation packet missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing translation packet: $DOC" >&2
  exit 1
fi

require_doc_text "product-doc-release-governance"
require_doc_text "Selected changed paths: \`22\`"
require_doc_text "Cargo.toml"
require_doc_text "Cargo.lock"
require_doc_text "install-context"
require_doc_text "request_plugin_install"
require_doc_text "list_available_plugins_to_install"
require_doc_text "P0 security/runtime"
require_doc_text "operator approval packet"
require_doc_text "long soak"
require_doc_text "No raw upstream document or package-policy copy"
require_doc_text "No active Codex engine dependency"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg translation "upstream-codex-product-governance-translation-packet" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-product-governance-absorption.sh" \
    --arg translation_gate "scripts/hepta-upstream-codex-product-governance-translation.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    --argjson selected_count "$EXPECTED_SELECTED_COUNT" \
    '{
      product:$product,
      status:"ready",
      translation_id:$translation,
      manifest:$manifest,
      translation_packet_path:$doc,
      selected_bucket:{
        id:"product-doc-release-governance",
        risk:"p2_product",
        selected_changed_file_count:$selected_count,
        translated_surface_count:5,
        translation_ready:true
      },
      gates:{
        source_absorption_gate:$source_gate,
        translation_gate:$translation_gate,
        active_dependency_isolation_gate:$active_dependency_gate
      },
      hepta_actions:[
        "package and install-context deltas become Hepta packaging-governance inputs",
        "README and protocol deltas become Hepta route/gate language",
        "plugin request and marketplace deltas require operator approval before live mutation",
        "sandbox, exec, network, and app-server docs stay behind P0 security/runtime review",
        "release-facing claims require clean preflight, watchdog, browser smoke, operator packet, and long soak"
      ],
      absorption_policy:{
        requires_hepta_translation:true,
        raw_upstream_doc_copy_allowed:false,
        raw_upstream_package_policy_copy_allowed:false,
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

echo "Hepta upstream Codex product governance translation gate passed"
