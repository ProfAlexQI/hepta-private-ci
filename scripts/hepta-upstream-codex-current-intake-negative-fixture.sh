#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
cd "$ROOT"
GATE="scripts/hepta-upstream-codex-current-intake.sh"
R3_GATE="scripts/hepta-upstream-codex-r3-integrity.sh"
R3_MANIFEST="docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-22_R3.json"
R5_MANIFEST="docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R5.json"
R6_MANIFEST="docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R6.json"
fixture_dir="$(mktemp -d /tmp/hepta-current-intake-r6-negative.XXXXXX)"
trap 'rm -rf "$fixture_dir"' EXIT

expect_denied() {
  local name="$1"
  shift
  if env "$@" bash "$GATE" >"$fixture_dir/$name.out" 2>"$fixture_dir/$name.err"; then
    echo "Hepta current latest-recorded intake negative fixture unexpectedly accepted $name" >&2
    return 1
  fi
}

expect_denied_with_message() {
  local name="$1"
  local expected_message="$2"
  shift 2
  if env "$@" bash "$GATE" >"$fixture_dir/$name.out" 2>"$fixture_dir/$name.err"; then
    echo "Hepta current latest-recorded intake negative fixture unexpectedly accepted $name" >&2
    return 1
  fi
  if ! grep -Fq "$expected_message" "$fixture_dir/$name.err"; then
    echo "Hepta current latest-recorded intake negative fixture rejected $name for the wrong reason" >&2
    sed -n '1,5p' "$fixture_dir/$name.err" >&2
    return 1
  fi
}

expect_r3_denied() {
  local name="$1"
  shift
  if env "$@" bash "$R3_GATE" >"$fixture_dir/r3-$name.out" 2>"$fixture_dir/r3-$name.err"; then
    echo "Hepta R3 historical integrity negative fixture unexpectedly accepted $name" >&2
    return 1
  fi
}

expect_r3_denied stale_ref \
  HEPTA_UPSTREAM_CODEX_R3_INTEGRITY_CUTOFF_REF=refs/remotes/upstream/hepta-intake-20260721-r2
expect_r3_denied cutoff_sha_mismatch \
  HEPTA_UPSTREAM_CODEX_R3_INTEGRITY_CUTOFF_HEAD=88fac6fe108237a105d3203e3508b0d531054312

jq '.observation.upstream_repository="https://example.invalid/codex.git"' "$R3_MANIFEST" >"$fixture_dir/r3-url.json"
expect_r3_denied url_mismatch \
  HEPTA_UPSTREAM_CODEX_R3_INTEGRITY_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_R3_INTEGRITY_MANIFEST="$fixture_dir/r3-url.json"

jq '.predecessor_intake.manifest_sha256="0000000000000000000000000000000000000000000000000000000000000000"' "$R3_MANIFEST" >"$fixture_dir/r3-predecessor-hash.json"
expect_r3_denied predecessor_hash_mismatch \
  HEPTA_UPSTREAM_CODEX_R3_INTEGRITY_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_R3_INTEGRITY_MANIFEST="$fixture_dir/r3-predecessor-hash.json"

jq '.commit_inventory[0].disposition="imported" | .commit_inventory[0].imported=true' "$R3_MANIFEST" >"$fixture_dir/r3-imported.json"
expect_r3_denied false_import_claim \
  HEPTA_UPSTREAM_CODEX_R3_INTEGRITY_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_R3_INTEGRITY_MANIFEST="$fixture_dir/r3-imported.json"

expect_denied stale_r4_ref \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R4_REF=refs/remotes/upstream/hepta-intake-20260722-r3
expect_denied stale_r5_ref \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R5_REF=refs/remotes/upstream/main
expect_denied_with_message stale_r6_ref "R6 ref does not match the pinned frozen ref" \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R6_REF=refs/remotes/upstream/main
expect_denied r4_head_mismatch \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R4_HEAD=9fc715c0861c956c894a91890b78dc05b304ba29
expect_denied r5_head_mismatch \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R5_HEAD=f61b51ddd924643514b33234816a8a2772b1aec7
expect_denied_with_message r6_head_mismatch "R6 head does not match the pinned cutoff" \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R6_HEAD=81da9deb065d7adb283816b19b40f89bcc484276
expect_denied manifest_override_without_fixture_opt_in \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R5_MANIFEST="$fixture_dir/r5-untrusted.json"
expect_denied_with_message r6_manifest_override_without_fixture_opt_in "R6 manifest override requires explicit fixture opt-in" \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R6_MANIFEST="$fixture_dir/r6-untrusted.json"

jq '.predecessor_intake.manifest_sha256="0000000000000000000000000000000000000000000000000000000000000000"' "$R5_MANIFEST" >"$fixture_dir/r5-predecessor-hash.json"
expect_denied r5_predecessor_hash_mismatch \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R5_MANIFEST="$fixture_dir/r5-predecessor-hash.json"

jq '.observation.range_end_inclusive="0000000000000000000000000000000000000000"' "$R5_MANIFEST" >"$fixture_dir/r5-range-end.json"
expect_denied r5_range_end_mismatch \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R5_MANIFEST="$fixture_dir/r5-range-end.json"

jq 'del(.commit_inventory[0])' "$R5_MANIFEST" >"$fixture_dir/r5-incomplete-commits.json"
expect_denied r5_incomplete_commit_inventory \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R5_MANIFEST="$fixture_dir/r5-incomplete-commits.json"

jq '.commit_inventory[1]=.commit_inventory[0]' "$R5_MANIFEST" >"$fixture_dir/r5-duplicate-commit.json"
expect_denied r5_duplicate_commit_inventory \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R5_MANIFEST="$fixture_dir/r5-duplicate-commit.json"

jq '.commit_inventory[0].status="imported" | .commit_inventory[0].imported=true | .classification_summary.status_counts={candidate:1,deferred:0,rejected:0,imported:1}' "$R5_MANIFEST" >"$fixture_dir/r5-imported.json"
expect_denied r5_false_import_claim \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R5_MANIFEST="$fixture_dir/r5-imported.json"

jq '.claims.merge_performed=true' "$R5_MANIFEST" >"$fixture_dir/r5-merge-claim.json"
expect_denied r5_false_merge_claim \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R5_MANIFEST="$fixture_dir/r5-merge-claim.json"

jq 'del(.commit_inventory[0].related_files[0])' "$R5_MANIFEST" >"$fixture_dir/r5-incomplete-files.json"
expect_denied r5_incomplete_file_surface \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R5_MANIFEST="$fixture_dir/r5-incomplete-files.json"

jq '.predecessor_intake.manifest_sha256="0000000000000000000000000000000000000000000000000000000000000000"' "$R6_MANIFEST" >"$fixture_dir/r6-predecessor-hash.json"
expect_denied_with_message r6_predecessor_hash_mismatch "R6 manifest contract drifted" \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R6_MANIFEST="$fixture_dir/r6-predecessor-hash.json"

jq '.observation.range_identity.digest="0000000000000000000000000000000000000000000000000000000000000000"' "$R6_MANIFEST" >"$fixture_dir/r6-range-hash.json"
expect_denied_with_message r6_range_hash_mismatch "R6 manifest contract drifted" \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R6_MANIFEST="$fixture_dir/r6-range-hash.json"

jq 'del(.commit_inventory[0])' "$R6_MANIFEST" >"$fixture_dir/r6-incomplete-commits.json"
expect_denied_with_message r6_incomplete_commit_inventory "R6 manifest contract drifted" \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R6_MANIFEST="$fixture_dir/r6-incomplete-commits.json"

jq '.commit_inventory[1]=.commit_inventory[0]' "$R6_MANIFEST" >"$fixture_dir/r6-duplicate-commit.json"
expect_denied_with_message r6_duplicate_commit_inventory "R6 manifest contract drifted" \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R6_MANIFEST="$fixture_dir/r6-duplicate-commit.json"

jq '.commit_inventory[0].status="imported" | .commit_inventory[0].imported=true | .commit_inventory[0].imported_evidence={local_commit:"0000000000000000000000000000000000000000"} | .classification_summary.status_counts={candidate:1,deferred:0,rejected:0,imported:1}' "$R6_MANIFEST" >"$fixture_dir/r6-imported.json"
expect_denied_with_message r6_false_import_claim "R6 manifest contract drifted" \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R6_MANIFEST="$fixture_dir/r6-imported.json"

jq '.claims.live_enablement_performed=true' "$R6_MANIFEST" >"$fixture_dir/r6-live-claim.json"
expect_denied_with_message r6_false_live_claim "R6 manifest contract drifted" \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R6_MANIFEST="$fixture_dir/r6-live-claim.json"

jq 'del(.commit_inventory[0].related_files[0])' "$R6_MANIFEST" >"$fixture_dir/r6-incomplete-files.json"
expect_denied_with_message r6_incomplete_file_surface "R6 manifest contract drifted" \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST=1 \
  HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R6_MANIFEST="$fixture_dir/r6-incomplete-files.json"

jq -n '{
  schema:"hepta_upstream_codex_current_intake_negative_fixture_v6",
  status:"ready",
  r3_historical_integrity_preserved:true,
  stale_r4_ref_denied:true,
  stale_or_floating_r5_ref_denied:true,
  stale_or_floating_r6_ref_denied:true,
  cutoff_sha_mismatch_denied:true,
  untrusted_manifest_override_denied:true,
  predecessor_hash_mismatch_denied:true,
  range_hash_mismatch_denied:true,
  range_end_mismatch_denied:true,
  incomplete_commit_inventory_denied:true,
  duplicate_commit_inventory_denied:true,
  false_import_claim_denied:true,
  false_merge_claim_denied:true,
  false_live_claim_denied:true,
  incomplete_file_surface_denied:true,
  network_access_performed:false,
  ref_mutation_performed:false,
  workspace_mutation_performed:false
}'
echo "Hepta upstream Codex current latest-recorded intake negative fixture passed"
