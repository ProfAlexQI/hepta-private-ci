#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVE_WIRING_PRECONDITION.md"

echo "[hepta-upstream-codex-active-wiring-precondition] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_active_wiring_precondition -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "active-wiring precondition packet missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing active-wiring precondition packet: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-active-wiring-precondition"
require_doc_text "Promotion closure ready: \`true\`"
require_doc_text "All surface promotion packets complete: \`true\`"
require_doc_text "Active promotion denial ready: \`true\`"
require_doc_text "Explicit operator approval required: \`true\`"
require_doc_text "Operator approval recorded: \`false\`"
require_doc_text "Activation request id required: \`true\`"
require_doc_text "Activation request id present: \`false\`"
require_doc_text "Live dependency isolation required: \`true\`"
require_doc_text "Watchdog required: \`true\`"
require_doc_text "Browser smoke required: \`true\`"
require_doc_text "Long soak required: \`true\`"
require_doc_text "Active wiring precondition ready: \`true\`"
require_doc_text "Active wiring allowed: \`false\`"
require_doc_text "Active runtime code wiring allowed: \`false\`"
require_doc_text "Public release claim allowed: \`false\`"
require_doc_text "Public GA claim allowed: \`false\`"
require_doc_text "Release artifact write allowed: \`false\`"
require_doc_text "No gateway RPC"
require_doc_text "No public release publication"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg precondition "upstream-codex-active-wiring-precondition" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg closure_gate "scripts/hepta-upstream-codex-promotion-closure.sh" \
    --arg precondition_gate "scripts/hepta-upstream-codex-active-wiring-precondition.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      precondition_id:$precondition,
      manifest:$manifest,
      precondition_packet_path:$doc,
      source_closure_gate:$closure_gate,
      active_wiring_precondition_gate:$precondition_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      source_closure:{
        promotion_closure_ready:true,
        all_surface_promotion_packets_complete:true,
        active_promotion_denial_ready:true
      },
      required_preconditions:{
        explicit_operator_approval_required:true,
        operator_approval_recorded:false,
        activation_request_id_required:true,
        activation_request_id_present:false,
        live_dependency_isolation_required:true,
        watchdog_required:true,
        browser_smoke_required:true,
        long_soak_required:true,
        active_wiring_precondition_ready:true,
        active_wiring_allowed:false
      },
      denied_active_decisions:{
        active_runtime_code_wiring_allowed:false,
        active_runtime_dependency_allowed:false,
        active_runtime_auto_rebase_allowed:false,
        active_codex_engine_dependency_allowed:false,
        public_release_claim_allowed:false,
        public_ga_claim_allowed:false,
        release_artifact_write_allowed:false
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

echo "Hepta upstream Codex active-wiring precondition gate passed"
