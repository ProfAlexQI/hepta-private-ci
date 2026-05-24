#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_PROMOTION_CLOSURE.md"

echo "[hepta-upstream-codex-promotion-closure] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_promotion_closure -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "promotion closure packet missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing promotion closure packet: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-promotion-closure-denial"
require_doc_text "Required surface promotion packets: \`4\`"
require_doc_text "Completed surface promotion packets: \`4\`"
require_doc_text "All surface promotion packets complete: \`true\`"
require_doc_text "Promotable buckets: \`0\`"
require_doc_text "Promotion-blocked buckets: \`4\`"
require_doc_text "Active promotion ready: \`false\`"
require_doc_text "Active promotion denial ready: \`true\`"
require_doc_text "Closure ready: \`true\`"
require_doc_text "release-governance-claim-promotion-packet"
require_doc_text "hepta-cli-tui-parity-promotion-packet"
require_doc_text "upstream-codex-provider-security-promotion-packet"
require_doc_text "runtime-appserver-route-event-promotion-packet"
require_doc_text "Active runtime code wiring allowed: \`false\`"
require_doc_text "Public release claim allowed: \`false\`"
require_doc_text "Public GA claim allowed: \`false\`"
require_doc_text "Release artifact write allowed: \`false\`"
require_doc_text "No gateway RPC"
require_doc_text "No public release publication"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg closure "upstream-codex-promotion-closure-denial" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg readiness_gate "scripts/hepta-upstream-codex-promotion-readiness.sh" \
    --arg closure_gate "scripts/hepta-upstream-codex-promotion-closure.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      closure_id:$closure,
      manifest:$manifest,
      closure_packet_path:$doc,
      source_promotion_readiness_gate:$readiness_gate,
      closure_gate:$closure_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      closure:{
        required_surface_promotion_packet_count:4,
        completed_surface_promotion_packet_count:4,
        all_surface_promotion_packets_complete:true,
        promotable_bucket_count:0,
        promotion_blocked_bucket_count:4,
        active_promotion_ready:false,
        active_promotion_denial_ready:true,
        closure_ready:true
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
      closure_invariants:[
        "all four required surface promotion packets are complete",
        "zero selected upstream Codex buckets are promotable by default",
        "all four selected upstream Codex buckets remain active-promotion blocked",
        "active Hepta runtime keeps zero tracked Codex engine dependencies",
        "public release and public GA claims remain operator-gated"
      ],
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

echo "Hepta upstream Codex promotion closure gate passed"
