#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
cd "$REPO_ROOT"

GATE="scripts/hepta-upstream-codex-current-intake.sh"
OLD_REF="refs/remotes/openai-codex/main"
OLD_HEAD="7d47056ea42636271ac020b86347fbbef49490aa"
OLD_APPS_MCP_RECEIPT="0000000000000000000000000000000000000000"
OLD_PROC_PREFLIGHT_RECEIPT="0000000000000000000000000000000000000000"
CANONICAL_MANIFEST="docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-21_R2.json"
MISSING_R2_SHA="bd92b056ddd91bd7c2ecfea3d8773f7eb5a879a6"
WRONG_CLASS_R2_SHA="e4836f998da166aba456f60d2e74eb79d6e2542b"
SELECTED_R2_SHA="44481a1c4548d1cc0cc3c95aa03b59ec4cba074a"
FORGED_R2_SHA="0000000000000000000000000000000000000000"

fixture_dir="$(mktemp -d /tmp/hepta-current-intake-negative.XXXXXX)"
trap 'rm -rf "$fixture_dir"' EXIT

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

expect_denied \
  "proc_preflight_receipt_drift" \
  "proc preflight receipt does not match pinned receipt" \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_PROC_PREFLIGHT_LOCAL_RECEIPT="$OLD_PROC_PREFLIGHT_RECEIPT"

jq --arg sha "$MISSING_R2_SHA" '
  .deferred_decisions |= map(select(.upstream_commit != $sha))
  | .classification.deferred_decision_count = (.deferred_decisions | length)
' "$CANONICAL_MANIFEST" >"$fixture_dir/missing-r2-sha.json"
expect_denied \
  "missing_r2_deferred_sha" \
  "r2 deferred commit set does not equal observed delta minus selected" \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_MANIFEST="$fixture_dir/missing-r2-sha.json"

jq --arg sha "$FORGED_R2_SHA" '
  .deferred_decisions += [{
    state:"deferred",
    classification:"r2_forged_extra_commit",
    upstream_commit:$sha,
    reason:"negative fixture"
  }]
  | .classification.deferred_decision_count = (.deferred_decisions | length)
' "$CANONICAL_MANIFEST" >"$fixture_dir/forged-r2-sha.json"
expect_denied \
  "forged_extra_r2_deferred_sha" \
  "r2 deferred commit set does not equal observed delta minus selected" \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_MANIFEST="$fixture_dir/forged-r2-sha.json"

jq --arg sha "$SELECTED_R2_SHA" '
  .deferred_decisions += [{
    state:"deferred",
    classification:"r2_selected_proc_overlap",
    upstream_commit:$sha,
    reason:"negative fixture"
  }]
  | .classification.deferred_decision_count = (.deferred_decisions | length)
' "$CANONICAL_MANIFEST" >"$fixture_dir/selected-r2-overlap.json"
expect_denied \
  "selected_r2_also_deferred" \
  "r2 selected and deferred commit sets overlap" \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_MANIFEST="$fixture_dir/selected-r2-overlap.json"

jq --arg sha "$WRONG_CLASS_R2_SHA" '
  (.deferred_decisions[] | select(.upstream_commit == $sha) | .classification) =
    "r2_wrong_classification"
' "$CANONICAL_MANIFEST" >"$fixture_dir/wrong-r2-classification.json"
expect_denied \
  "wrong_r2_deferred_classification" \
  "r2 deferred classification mapping drifted" \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_MANIFEST="$fixture_dir/wrong-r2-classification.json"

jq -n '{
  product:"Hepta",
  status:"ready",
  gate:"hepta_upstream_codex_current_intake_negative_fixture",
  stale_ref_denied:true,
  cutoff_drift_denied:true,
  apps_mcp_receipt_drift_denied:true,
  proc_preflight_receipt_drift_denied:true,
  missing_r2_deferred_sha_denied:true,
  forged_extra_r2_deferred_sha_denied:true,
  selected_r2_also_deferred_denied:true,
  wrong_r2_deferred_classification_denied:true,
  network_access_performed:false,
  ref_mutation_performed:false,
  workspace_mutation_performed:false
}'

echo "Hepta upstream Codex current intake negative fixture passed"
