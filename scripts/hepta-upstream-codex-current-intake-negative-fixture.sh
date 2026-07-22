#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
cd "$ROOT"
GATE="scripts/hepta-upstream-codex-current-intake.sh"
MANIFEST="docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-22_R3.json"
fixture_dir="$(mktemp -d /tmp/hepta-current-intake-r3-negative.XXXXXX)"
trap 'rm -rf "$fixture_dir"' EXIT

expect_denied() {
  local name="$1"
  shift
  if env "$@" "$GATE" >"$fixture_dir/$name.out" 2>"$fixture_dir/$name.err"; then
    echo "Hepta current intake negative fixture unexpectedly accepted $name" >&2
    return 1
  fi
}

expect_denied stale_ref \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_CUTOFF_REF=refs/remotes/upstream/hepta-intake-20260721-r2
expect_denied cutoff_sha_mismatch \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_CUTOFF_HEAD=88fac6fe108237a105d3203e3508b0d531054312

jq '.observation.upstream_repository="https://example.invalid/codex.git"' "$MANIFEST" >"$fixture_dir/url.json"
expect_denied url_mismatch \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_MANIFEST="$fixture_dir/url.json"

jq '.observation.target_source_ref="refs/heads/next"' "$MANIFEST" >"$fixture_dir/source-ref.json"
expect_denied source_ref_mismatch \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_MANIFEST="$fixture_dir/source-ref.json"

jq '.observation.discovered_remote_head="0000000000000000000000000000000000000000"' "$MANIFEST" >"$fixture_dir/remote-head.json"
expect_denied remote_head_sha_mismatch \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_MANIFEST="$fixture_dir/remote-head.json"

jq '.observation.discovery_command="git fetch upstream main"' "$MANIFEST" >"$fixture_dir/floating-fetch.json"
expect_denied floating_fetch \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_MANIFEST="$fixture_dir/floating-fetch.json"

jq '.predecessor_intake.manifest_sha256="0000000000000000000000000000000000000000000000000000000000000000"' "$MANIFEST" >"$fixture_dir/predecessor-hash.json"
expect_denied predecessor_hash_mismatch \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_MANIFEST="$fixture_dir/predecessor-hash.json"

jq '.commit_inventory[0].disposition="imported" | .commit_inventory[0].imported=true' "$MANIFEST" >"$fixture_dir/imported.json"
expect_denied false_import_claim \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_MANIFEST="$fixture_dir/imported.json"

jq 'del(.commit_inventory[0])' "$MANIFEST" >"$fixture_dir/incomplete-commits.json"
expect_denied incomplete_commit_surface \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_MANIFEST="$fixture_dir/incomplete-commits.json"

jq 'del(.file_surface[0])' "$MANIFEST" >"$fixture_dir/incomplete-files.json"
expect_denied incomplete_file_surface \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_MANIFEST="$fixture_dir/incomplete-files.json"

jq -n '{schema:"hepta_upstream_codex_current_intake_negative_fixture_v3",status:"ready",stale_ref_denied:true,cutoff_sha_mismatch_denied:true,url_mismatch_denied:true,source_ref_mismatch_denied:true,remote_head_sha_mismatch_denied:true,floating_fetch_denied:true,predecessor_hash_mismatch_denied:true,false_import_claim_denied:true,incomplete_commit_surface_denied:true,incomplete_file_surface_denied:true,network_access_performed:false,ref_mutation_performed:false,workspace_mutation_performed:false}'
echo "Hepta upstream Codex current intake negative fixture passed"
