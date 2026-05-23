#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
REQUIRE_LIVE="${HEPTA_UPSTREAM_CODEX_SYNC_REQUIRE_LIVE:-0}"

echo "[hepta-upstream-codex-sync-lane] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_sync_lane -- --nocapture

echo "[hepta-upstream-codex-sync-lane] active service dependency isolation"
isolation_raw="$(mktemp)"
isolation_json="$(mktemp)"
trap 'rm -f "$isolation_raw" "$isolation_json"' EXIT
if [[ "$REQUIRE_LIVE" == "1" ]]; then
  HEPTA_ACTIVE_SERVICE_DEPENDENCY_ISOLATION_REQUIRE_LIVE=1 \
    scripts/hepta-active-service-dependency-isolation.sh >"$isolation_raw"
else
  HEPTA_ACTIVE_SERVICE_DEPENDENCY_ISOLATION_LIVE=0 \
    scripts/hepta-active-service-dependency-isolation.sh >"$isolation_raw"
fi
awk '/^Hepta active service dependency isolation gate passed$/ { exit } { print }' \
  "$isolation_raw" >"$isolation_json"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg lane "upstream-codex-sync-lane" \
    --arg upstream "https://github.com/openai/codex" \
    --arg manifest "$MANIFEST" \
    --argjson require_live "$(if [[ "$REQUIRE_LIVE" == "1" ]]; then echo true; else echo false; fi)" \
    --slurpfile isolation "$isolation_json" \
    '{
      product:$product,
      status:"ready",
      lane_id:$lane,
      upstream_repository:$upstream,
      manifest:$manifest,
      sync_mode:"classify_then_absorb_then_gate",
      compatibility_snapshot_role:"ingestion_and_regression_oracle",
      upstream_fetch_performed:false,
      upstream_latest_claimed:false,
      upstream_merge_performed:false,
      active_runtime_auto_rebase_allowed:false,
      active_runtime_codex_engine_dependency_allowed:false,
      require_live:$require_live,
      active_dependency_isolation:($isolation[0] // null),
      required_next_steps:[
        "record upstream head before fetch/merge",
        "classify provider credential sandbox session tool MCP TUI diffs",
        "materialize Hepta absorption contracts before active wiring",
        "keep active hepta-cli dependency isolation green",
        "require long soak and release-governance evidence before any public claim"
      ],
      side_effects:{
        upstream_network_fetch:false,
        workspace_write:false,
        active_service_restart:false,
        credential_read:false,
        provider_invocation:false,
        channel_delivery:false,
        public_release:false
      }
    }'
)"

printf '%s\n' "$report"
echo "Hepta upstream Codex sync lane gate passed"
