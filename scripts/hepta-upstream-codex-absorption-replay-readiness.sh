#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_ABSORPTION_REPLAY_READINESS.md"

echo "[hepta-upstream-codex-absorption-replay-readiness] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_absorption_replay_readiness -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "absorption/replay readiness packet missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing absorption/replay readiness packet: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-absorption-replay-readiness"
require_doc_text "Ledger changed paths: \`878\`"
require_doc_text "Selected absorption paths: \`716\`"
require_doc_text "Selected buckets: \`4 / 4\`"
require_doc_text "Translation/replay gates: \`4 / 4\`"
require_doc_text "product-governance-translation"
require_doc_text "legacy-compatibility-replay"
require_doc_text "provider-security-replay"
require_doc_text "runtime-appserver-replay"
require_doc_text "No active Codex engine dependency"
require_doc_text "No public release claim"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg readiness "upstream-codex-absorption-replay-readiness" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-diff-ledger.sh" \
    --arg readiness_gate "scripts/hepta-upstream-codex-absorption-replay-readiness.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      readiness_id:$readiness,
      manifest:$manifest,
      readiness_packet_path:$doc,
      source_ledger_gate:$source_gate,
      readiness_gate:$readiness_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      coverage:{
        ledger_changed_file_count:878,
        selected_absorption_changed_file_count:716,
        selected_bucket_count:4,
        required_selected_bucket_count:4,
        absorption_contract_ready_count:4,
        required_absorption_contract_ready_count:4,
        translation_replay_ready_count:4,
        required_translation_replay_ready_count:4,
        p0_replay_ready_count:2,
        required_p0_replay_ready_count:2,
        p1_replay_ready_count:1,
        required_p1_replay_ready_count:1,
        p2_translation_ready_count:1,
        required_p2_translation_ready_count:1,
        readiness_ready:true
      },
      closed_gate_families:[
        "product-governance-absorption",
        "product-governance-translation",
        "legacy-compatibility-absorption",
        "legacy-compatibility-replay",
        "provider-security-absorption",
        "provider-security-replay",
        "runtime-appserver-absorption",
        "runtime-appserver-replay"
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

echo "Hepta upstream Codex absorption/replay readiness gate passed"
