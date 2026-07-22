#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
cd "$ROOT"

PINNED_URL="https://github.com/openai/codex.git"
PINNED_SOURCE_REF="refs/heads/main"
PINNED_MANIFEST="docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-22_R3.json"
PINNED_CUTOFF_REF="refs/remotes/upstream/hepta-intake-20260722-r3"
PINNED_CUTOFF_HEAD="9fc715c0861c956c894a91890b78dc05b304ba29"
PINNED_RANGE_START="88fac6fe108237a105d3203e3508b0d531054312"
PINNED_R2_REF="refs/remotes/upstream/hepta-intake-20260721-r2"
PINNED_R2_HEAD="88fac6fe108237a105d3203e3508b0d531054312"
PINNED_R2_MANIFEST="docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-21_R2.json"
PINNED_R2_MANIFEST_SHA256="41d9f73fe2d3339d1912578be7628a76f546cea2aaa54f1477964cbad2c1c9ca"
PINNED_R1_REF="refs/remotes/upstream/hepta-intake-20260721"
PINNED_R1_HEAD="45ac251e178416ff5c3022457ad8d2778c0d4549"
PINNED_RANGE_DIGEST="c56a557f74a6299f0300b1dc65a69b99d7a42027e0c832984e6ee67f98bc9a45"
PINNED_PATH_DIGEST="388e77f541fb5752993b6287b69ed97cca7faa57e68c479190c573f48bfcb837"
PINNED_NORMALIZED_COMMIT_DIGEST="765ba2e7eaac8d94e936ac4b17289a0ac945bb05469499204a8dfb15f63d24e2"
PINNED_NORMALIZED_FILE_DIGEST="8b6858172f5ba38a968d791caef26ac354d23ecef66f0a95d72810abdb8ae220"

MANIFEST="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_MANIFEST:-$PINNED_MANIFEST}"
CUTOFF_REF="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_CUTOFF_REF:-$PINNED_CUTOFF_REF}"
CUTOFF_HEAD="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_CUTOFF_HEAD:-$PINNED_CUTOFF_HEAD}"
ALLOW_FIXTURE_MANIFEST="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST:-0}"

fail() {
  echo "hepta upstream Codex current intake gate failed: $*" >&2
  exit 1
}

resolve_direct_commit_ref() {
  local label="$1" ref="$2" expected="$3" symbolic_target raw object_type
  symbolic_target="$(git symbolic-ref -q "$ref" 2>/dev/null || true)"
  [[ -z "$symbolic_target" ]] || fail "$label must be a direct ref, found symref to $symbolic_target"
  raw="$(git rev-parse --verify "$ref" 2>/dev/null)" || fail "$label is missing: $ref"
  [[ "$raw" == "$expected" ]] || fail "$label raw OID drifted: expected $expected got $raw"
  object_type="$(git cat-file -t "$raw" 2>/dev/null)" || fail "$label object is unavailable: $raw"
  [[ "$object_type" == "commit" ]] || fail "$label must point directly to a commit, found $object_type"
}

command -v jq >/dev/null || fail "jq is required"
command -v shasum >/dev/null || fail "shasum is required"
[[ "$MANIFEST" == "$PINNED_MANIFEST" || "$ALLOW_FIXTURE_MANIFEST" == "1" ]] || fail "manifest override requires explicit fixture opt-in"
[[ -f "$MANIFEST" ]] || fail "manifest is missing: $MANIFEST"
[[ "$CUTOFF_REF" == "$PINNED_CUTOFF_REF" ]] || fail "cutoff ref does not match pinned r3 ref"
[[ "$CUTOFF_HEAD" == "$PINNED_CUTOFF_HEAD" ]] || fail "cutoff head does not match pinned r3 head"
[[ "$(shasum -a 256 "$PINNED_R2_MANIFEST" | awk '{print $1}')" == "$PINNED_R2_MANIFEST_SHA256" ]] || fail "r2 predecessor manifest hash drifted"
resolve_direct_commit_ref "r1 predecessor cutoff" "$PINNED_R1_REF" "$PINNED_R1_HEAD"
resolve_direct_commit_ref "r2 predecessor cutoff" "$PINNED_R2_REF" "$PINNED_R2_HEAD"
resolve_direct_commit_ref "r3 current cutoff" "$CUTOFF_REF" "$CUTOFF_HEAD"
git merge-base --is-ancestor "$PINNED_R1_HEAD" "$PINNED_R2_HEAD" || fail "r1 predecessor is not an ancestor of r2"
git merge-base --is-ancestor "$PINNED_R2_HEAD" "$CUTOFF_HEAD" || fail "r2 predecessor is not an ancestor of r3"

jq -e \
  --arg url "$PINNED_URL" \
  --arg source_ref "$PINNED_SOURCE_REF" \
  --arg cutoff_ref "$PINNED_CUTOFF_REF" \
  --arg cutoff_head "$PINNED_CUTOFF_HEAD" \
  --arg range_start "$PINNED_RANGE_START" \
  --arg predecessor_manifest "$PINNED_R2_MANIFEST" \
  --arg predecessor_sha256 "$PINNED_R2_MANIFEST_SHA256" \
  --arg predecessor_ref "$PINNED_R2_REF" \
  --arg range_digest "$PINNED_RANGE_DIGEST" \
  --arg path_digest "$PINNED_PATH_DIGEST" \
  --arg commit_digest "$PINNED_NORMALIZED_COMMIT_DIGEST" \
  --arg file_digest "$PINNED_NORMALIZED_FILE_DIGEST" '
  .schema_version == "hepta_upstream_codex_current_intake_v3"
  and .intake_id == "upstream-codex-intake-2026-07-22-r3"
  and .schema_evolution == {mode:"additive_versioned",predecessor_schema:"hepta_upstream_codex_current_intake_v2",compatibility:"v2 predecessor remains immutable and hash-bound"}
  and .predecessor_intake == {manifest_path:$predecessor_manifest,manifest_sha256:$predecessor_sha256,cutoff_ref:$predecessor_ref,cutoff_head:$range_start,preserved:true}
  and .observation.upstream_repository == $url
  and .observation.target_branch == "main"
  and .observation.target_source_ref == $source_ref
  and .observation.discovery_command == "git ls-remote https://github.com/openai/codex refs/heads/main"
  and .observation.discovered_remote_head == $cutoff_head
  and .observation.exact_sha_fetch_performed == true
  and .observation.fetch_source == $cutoff_head
  and .observation.temporary_intake_ref == "refs/hepta/intake/tmp/upstream-codex-20260722-9fc715c0"
  and .observation.cutoff_ref == $cutoff_ref
  and .observation.cutoff_head == $cutoff_head
  and .observation.range_start == $range_start
  and .observation.range_end == $cutoff_head
  and .observation.range_identity == {algorithm:"sha256(git_rev_list_reverse_range_lf)",digest:$range_digest}
  and .observation.file_surface_identity == {algorithm:"sha256(c_locale_sorted_name_status_lf)",digest:$path_digest}
  and .observation.normalized_commit_inventory_sha256 == $commit_digest
  and .observation.normalized_file_surface_sha256 == $file_digest
  and .classification.policy == "inventory_only_selective_semantic_follow_up"
  and .classification.disposition_counts == {deferred:74,rejected:9,imported:0}
  and ([.commit_inventory[].upstream_commit] | length == 83 and length == (unique|length))
  and ([.commit_inventory[] | select(.disposition == "deferred")] | length) == 74
  and ([.commit_inventory[] | select(.disposition == "rejected")] | length) == 9
  and ([.commit_inventory[] | select(.disposition == "imported" or .imported or .integrated or .tested)] | length) == 0
  and ([.candidate_follow_up_slices[] | select(.status != "candidate_only_not_started")] | length) == 0
  and .boundaries == {inventory_only:true,upstream_code_imported:false,upstream_code_integrated:false,upstream_code_tested:false,bulk_merge_performed:false,bulk_rebase_performed:false,cherry_pick_performed:false,whole_tree_copy_performed:false,cargo_lock_replaced:false,module_lock_replaced:false,product_code_modified:false,follow_up_started:false}
  ' "$MANIFEST" >/dev/null || fail "r3 manifest contract drifted"

range="$PINNED_RANGE_START..$CUTOFF_HEAD"
commit_count="$(git rev-list --count "$range")"
merge_count="$(git rev-list --merges --count "$range")"
changed_file_count="$(git diff --name-only "$range" | wc -l | tr -d '[:space:]')"
codex_rs_changed_file_count="$(git diff --name-only "$range" | awk '$0 ~ /^codex-rs\// {n++} END {print n+0}')"
range_digest="$(git rev-list --reverse "$range" | shasum -a 256 | awk '{print $1}')"
path_digest="$(git diff --name-status "$range" | LC_ALL=C sort | shasum -a 256 | awk '{print $1}')"
[[ "$commit_count" == "83" && "$merge_count" == "0" ]] || fail "r3 commit inventory count drifted"
[[ "$changed_file_count" == "522" && "$codex_rs_changed_file_count" == "508" ]] || fail "r3 file inventory count drifted"
[[ "$range_digest" == "$PINNED_RANGE_DIGEST" ]] || fail "r3 range digest drifted"
[[ "$path_digest" == "$PINNED_PATH_DIGEST" ]] || fail "r3 path surface digest drifted"

expected_commits="$(mktemp)"
manifest_commits="$(mktemp)"
manifest_files="$(mktemp)"
trap 'rm -f "$expected_commits" "$manifest_commits" "$manifest_files"' EXIT
git log --reverse --format='%H%x09%aI%x09%s' "$range" >"$expected_commits"
jq -r '.commit_inventory[] | [.upstream_commit,.authored_at,.title] | @tsv' "$MANIFEST" >"$manifest_commits"
cmp -s "$expected_commits" "$manifest_commits" || fail "r3 commit inventory does not match the complete range"
jq -r '.file_surface[] | if has("old_path") then [.status,.old_path,.path] else [.status,.path] end | @tsv' "$MANIFEST" >"$manifest_files"
git diff --name-status "$range" | cmp -s - "$manifest_files" || fail "r3 file surface does not match the complete range"

normalized_commit_digest="$(jq -cS '.commit_inventory' "$MANIFEST" | shasum -a 256 | awk '{print $1}')"
normalized_file_digest="$(jq -cS '.file_surface' "$MANIFEST" | shasum -a 256 | awk '{print $1}')"
[[ "$normalized_commit_digest" == "$PINNED_NORMALIZED_COMMIT_DIGEST" ]] || fail "normalized commit inventory digest drifted"
[[ "$normalized_file_digest" == "$PINNED_NORMALIZED_FILE_DIGEST" ]] || fail "normalized file surface digest drifted"

jq -n \
  --arg schema hepta_upstream_codex_current_intake_verify_v3 \
  --arg status ready \
  --arg manifest "$MANIFEST" \
  --arg cutoff_ref "$CUTOFF_REF" \
  --arg cutoff_head "$CUTOFF_HEAD" \
  --arg range_digest "$range_digest" \
  --arg path_digest "$path_digest" \
  --argjson commit_count "$commit_count" \
  --argjson changed_file_count "$changed_file_count" \
  '{schema:$schema,status:$status,manifest:$manifest,cutoff_ref:$cutoff_ref,cutoff_head:$cutoff_head,commit_count:$commit_count,changed_file_count:$changed_file_count,range_digest:$range_digest,path_digest:$path_digest,imported_count:0,integrated_count:0,tested_count:0,predecessor_refs_preserved:true,network_access_performed:false,workspace_product_code_modified:false}'

echo "Hepta upstream Codex current intake gate passed"
