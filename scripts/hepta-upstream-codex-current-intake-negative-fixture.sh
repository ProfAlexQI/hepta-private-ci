#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
cd "$REPO_ROOT"

GATE="scripts/hepta-upstream-codex-current-intake.sh"
OLD_REF="refs/remotes/openai-codex/main"
OLD_HEAD="7d47056ea42636271ac020b86347fbbef49490aa"
OLD_APPS_MCP_RECEIPT="0000000000000000000000000000000000000000"

expect_denied() {
  local fixture_id="$1"
  local expected_message="$2"
  shift 2

  local fixture_output
  if fixture_output="$(env HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_SKIP_RUST_TESTS=1 "$@" "$GATE" 2>&1)"; then
    echo "negative fixture unexpectedly passed: $fixture_id" >&2
    exit 1
  fi

  if ! grep -Fq "$expected_message" <<<"$fixture_output"; then
    echo "negative fixture failed for the wrong reason: $fixture_id" >&2
    printf '%s\n' "$fixture_output" >&2
    exit 1
  fi
}

expect_denied \
  "stale_ref" \
  "cutoff ref does not match pinned ref" \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_CUTOFF_REF="$OLD_REF"

expect_denied \
  "cutoff_drift" \
  "cutoff head does not match pinned cutoff" \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_CUTOFF_HEAD="$OLD_HEAD"

expect_denied \
  "apps_mcp_receipt_drift" \
  "Apps MCP receipt does not match pinned receipt" \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_APPS_MCP_LOCAL_RECEIPT="$OLD_APPS_MCP_RECEIPT"

jq -n '{
  product:"Hepta",
  status:"ready",
  gate:"hepta_upstream_codex_current_intake_negative_fixture",
  stale_ref_denied:true,
  cutoff_drift_denied:true,
  apps_mcp_receipt_drift_denied:true,
  network_access_performed:false,
  ref_mutation_performed:false,
  workspace_mutation_performed:false
}'

echo "Hepta upstream Codex current intake negative fixture passed"
