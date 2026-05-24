#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_PROMOTION_READINESS.md"

echo "[hepta-upstream-codex-promotion-readiness] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_promotion_readiness -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "promotion readiness packet missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing promotion readiness packet: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-promotion-readiness"
require_doc_text "Assessed buckets: \`4 / 4\`"
require_doc_text "Absorption/replay source readiness: \`4 / 4\`"
require_doc_text "Completed surface promotion packets: \`3\`"
require_doc_text "Promotable buckets: \`0\`"
require_doc_text "Promotion-blocked buckets: \`4\`"
require_doc_text "Active promotion ready: \`false\`"
require_doc_text "hepta-cli-tui-parity-promotion-packet"
require_doc_text "upstream-codex-provider-security-promotion-packet"
require_doc_text "runtime-appserver-route-event-promotion-packet"
require_doc_text "No active Codex engine dependency"
require_doc_text "No public release claim"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg decision "upstream-codex-promotion-readiness" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-absorption-replay-readiness.sh" \
    --arg promotion_gate "scripts/hepta-upstream-codex-promotion-readiness.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      decision_id:$decision,
      manifest:$manifest,
      decision_packet_path:$doc,
      source_readiness_gate:$source_gate,
      promotion_readiness_gate:$promotion_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      decision:{
        assessed_bucket_count:4,
        required_assessed_bucket_count:4,
        absorption_replay_ready_count:4,
        required_absorption_replay_ready_count:4,
        required_surface_promotion_packet_count:4,
        completed_surface_promotion_packet_count:3,
        promotable_bucket_count:0,
        promotion_blocked_bucket_count:4,
        readiness_source_ready:true,
        active_promotion_ready:false,
        decision_ready:true
      },
      required_promotion_packets:[
        "release-governance-claim-promotion-packet",
        "hepta-cli-tui-parity-promotion-packet",
        "upstream-codex-provider-security-promotion-packet",
        "runtime-appserver-route-event-promotion-packet"
      ],
      promotion_policy:{
        active_runtime_code_wiring_allowed:false,
        active_runtime_dependency_allowed:false,
        active_runtime_auto_rebase_allowed:false,
        active_codex_engine_dependency_allowed:false,
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

echo "Hepta upstream Codex promotion readiness gate passed"
