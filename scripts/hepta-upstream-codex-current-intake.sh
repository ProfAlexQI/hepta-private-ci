#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
cd "$ROOT"

PINNED_URL="https://github.com/openai/codex.git"
PINNED_R3_MANIFEST="docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-22_R3.json"
PINNED_R3_MANIFEST_SHA256="4e0ad42fe7edc0f073d840457f48cf579befbbfd0abb73cc0845778f3122eca6"
PINNED_R3_REF="refs/remotes/upstream/hepta-intake-20260722-r3"
PINNED_R3_HEAD="9fc715c0861c956c894a91890b78dc05b304ba29"

PINNED_R4_MANIFEST="docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R4.json"
PINNED_R4_MANIFEST_SHA256="ec5f48d428f80f9fa1eace82f23f758f0dc71ebe2f5ebc5f04ba05308c32a616"
PINNED_R4_REF="refs/remotes/upstream/hepta-intake-20260724-r4"
PINNED_R4_HEAD="f61b51ddd924643514b33234816a8a2772b1aec7"
PINNED_R4_RANGE_DIGEST="33d5eaeddf0f4e69919df5dde501747ca1abbe10ab30ad3a192a2c64441a6469"
PINNED_R4_PATH_DIGEST="edf243d8bcd9c047d8e03034dcc0090b83b996b6fadf8658fb5fc78e27fb5d15"
PINNED_R4_NORMALIZED_COMMIT_DIGEST="279eaf925d7a23470b6ad3a37d68e60f3ed4c81ef1bbb44c5f259a7985dc9f52"
PINNED_R4_NORMALIZED_FILE_DIGEST="164b64cbc09bb08676974bac32877313b7f77f594d0fc848dfb2d3d9f67b612b"

PINNED_R4_SECURITY="docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R4_SECURITY.json"
PINNED_R4_SECURITY_SHA256="32d1b2fb73abbecd18c7677bac5330c87f99917fb59bc3a7fde4516b017cbea8"
PINNED_R4_PROTOCOL="docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R4_PROTOCOL_APP_SERVER.json"
PINNED_R4_PROTOCOL_SHA256="090f33912ca0947858466a3e725be4d230d1f216fba46de11d7acf6ccf1d8a69"
PINNED_R4_TOOLS="docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R4_TOOLS_APPS_RUNTIME.json"
PINNED_R4_TOOLS_SHA256="29bbaee20b42666ddd4f058537ef9c50d2a71412f9ef1ed6a57dc154ae2e8793"
PINNED_R4_TUI="docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R4_TUI_OTHER.json"
PINNED_R4_TUI_SHA256="37b0fb2e5930fc321c14caf5e3c60c83b3c8a4e249bf21cff40c780d155bc029"
PINNED_R4_FILES="docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R4_FILE_SURFACE.json"
PINNED_R4_FILES_SHA256="332bedaa48b008ae142cdafa2f4402c8bbad645951fd703cdb19c9e75dbeef73"

PINNED_R5_MANIFEST="docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R5.json"
PINNED_R5_MANIFEST_SHA256="dd56ea3130f035714fe14dcf25584161e669bb441a946548273a2ddec741e18f"
PINNED_R5_REF="refs/remotes/upstream/hepta-intake-20260724-r5"
PINNED_R5_HEAD="81da9deb065d7adb283816b19b40f89bcc484276"
PINNED_R5_RANGE_DIGEST="b158a9313653c113ca9bf695cd7209ac929bff6c9c04dfdfcaec9b423a9bd0a5"
PINNED_R5_PATH_DIGEST="3340c084533627f55e6fa28ea4464da9ff2065ad40d34dfc6e4cf1178ea9779f"
PINNED_R5_NORMALIZED_COMMIT_DIGEST="6d7afb49c0a74d70cb7067037e4df1fa89c4cd3561f3211c1aea9768113fd42e"
PINNED_R5_RELATED_PATH_DIGEST="d57fa54f0278a6a032173c402c4dc190ec5bb49b3af48b4a86d10a1656dc4042"

PINNED_R6_MANIFEST="docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R6.json"
PINNED_R6_MANIFEST_SHA256="4e8993154f769ce2f4bdbd078816fe6ce193e64ca12913ca09acec073532bbdc"
PINNED_R6_REF="refs/remotes/upstream/hepta-intake-20260724-r6"
PINNED_R6_HEAD="6c729ef1c1dcfbcbe1bd9d0c2dddde24377ae899"
PINNED_R6_RANGE_DIGEST="049e178e5bb6190f59776a7f3ef6bf924e9e1a5a3c3cc70ba02de4d1acfcfda5"
PINNED_R6_PATH_DIGEST="6402b9e303d68b212faa4162047bd635919051d0ebb7facac6ec5e77445de934"
PINNED_R6_COMMIT_IDENTITY_DIGEST="2cb86a8276e7831da2174db0d5873147a71419ac6adadd7dd5be6298581685d9"
PINNED_R6_NORMALIZED_COMMIT_DIGEST="62990e0160f88e604440469f2ea72fe2c517ecd337a5bca9dce516ebc6520d47"
PINNED_R6_RELATED_PATH_DIGEST="c39553d1d0715dccc7bd4a416c6eddcbe4a71b4c964ee1ed14d2e45991fbcf1f"

PINNED_R7_MANIFEST="docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R7.json"
PINNED_R7_MANIFEST_SHA256="63d15fc05f42605dceeb29899cb357f664ec99e691d24b8d296a0033c359feee"
PINNED_R7_REF="refs/remotes/upstream/hepta-intake-20260724-r7"
PINNED_R7_HEAD="f201c30c52a35f819262865a53df94b6f4ea7a50"
PINNED_R7_RANGE_DIGEST="58b43c4389ea3adf336023a05d08b6b6a4708d4c89eb49b4f2529d83ea1cbc4e"
PINNED_R7_PATH_DIGEST="7db8ff45019f54816b0175ae01cb4a4bd0d09d8886fc7ed93515bcbe27c1a03e"
PINNED_R7_COMMIT_IDENTITY_DIGEST="fb0f8b5c3dbc294a0cdda6c452c249646084d82cf3b33465dc3b116a60e3b40d"
PINNED_R7_NORMALIZED_COMMIT_DIGEST="151ffc8a8db68378663d69b0e28f9de3e442456f28a92fa525114e8603c82209"
PINNED_R7_RELATED_PATH_DIGEST="4610d4f464aa739db64c87ed819562d335b6cebb807815b496e9ec4d550449a8"

PINNED_R8_MANIFEST="docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R8.json"
PINNED_R8_MANIFEST_SHA256="60833a3504bca61ed33f527d6bc9315193540b839650e92280eeedb6cf10dba3"
PINNED_R8_REF="refs/remotes/upstream/hepta-intake-20260724-r8"
PINNED_R8_HEAD="c8957bbf0f79fa29c5e08b8c0b942c12ea3893f2"
PINNED_R8_RANGE_DIGEST="aa5aeda987db109b75fdcb1fb130da8a9996cdfdab87bf897eaa2215b0a8c1f5"
PINNED_R8_PATH_DIGEST="3fdd6e10977611d16a25338b9270615f187a5ef666040dbed2a33147262904e4"
PINNED_R8_COMMIT_IDENTITY_DIGEST="dd8a95f921dd891b5750022edf1fedb44b5fa0e9957f06cd29d78fc786acff6a"
PINNED_R8_NORMALIZED_COMMIT_DIGEST="9e41732419966ebd2c6e1ee54ea4a2d3d0cc3f1393de55f480e5c8920481e800"
PINNED_R8_RELATED_PATH_DIGEST="dac025271875834e3bc7d6fd778054a13105961411324665177b22860ffacf03"

PINNED_R9_MANIFEST="docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R9.json"
PINNED_R9_MANIFEST_SHA256="e8b28c20790abe02a9af0f59fb48eec172298799427d655e5ac23ed3fff64564"
PINNED_R9_REF="refs/remotes/upstream/hepta-intake-20260724-r9"
PINNED_R9_HEAD="000d2540ad73996f3589ae178bfe447bfd67cef2"
PINNED_R9_RANGE_DIGEST="9b8b35976e482e8628f08ec1c40b412fc9f114dc80c577bb429d58ed2db960f8"
PINNED_R9_PATH_DIGEST="30c2d4cf04cd64d547843e61249f52ec5d505a4bc0f1746da8f48e0257fd3c2f"
PINNED_R9_COMMIT_IDENTITY_DIGEST="e420afa25ff20a16c40bb06abf43344644db97ad6b1459b74d881aa10e9cb1b3"
PINNED_R9_NORMALIZED_COMMIT_DIGEST="18eb5611f0d7090aae76aa20278e033f65553d25d94250a4559c5cab8213a3a9"
PINNED_R9_RELATED_PATH_DIGEST="2f78b6bf411fa297a2850b52f3d197c985c34297f547dba72c9b3ad7fbdb4a17"

R4_MANIFEST="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R4_MANIFEST:-$PINNED_R4_MANIFEST}"
R5_MANIFEST="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R5_MANIFEST:-$PINNED_R5_MANIFEST}"
R6_MANIFEST="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R6_MANIFEST:-$PINNED_R6_MANIFEST}"
R7_MANIFEST="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R7_MANIFEST:-$PINNED_R7_MANIFEST}"
R8_MANIFEST="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R8_MANIFEST:-$PINNED_R8_MANIFEST}"
R9_MANIFEST="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R9_MANIFEST:-$PINNED_R9_MANIFEST}"
R4_REF="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R4_REF:-$PINNED_R4_REF}"
R4_HEAD="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R4_HEAD:-$PINNED_R4_HEAD}"
R5_REF="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R5_REF:-$PINNED_R5_REF}"
R5_HEAD="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R5_HEAD:-$PINNED_R5_HEAD}"
R6_REF="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R6_REF:-$PINNED_R6_REF}"
R6_HEAD="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R6_HEAD:-$PINNED_R6_HEAD}"
R7_REF="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R7_REF:-$PINNED_R7_REF}"
R7_HEAD="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R7_HEAD:-$PINNED_R7_HEAD}"
R8_REF="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R8_REF:-$PINNED_R8_REF}"
R8_HEAD="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R8_HEAD:-$PINNED_R8_HEAD}"
R9_REF="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R9_REF:-$PINNED_R9_REF}"
R9_HEAD="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_R9_HEAD:-$PINNED_R9_HEAD}"
ALLOW_FIXTURE_MANIFEST="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST:-0}"

fail() {
  echo "hepta upstream Codex current latest-recorded intake gate failed: $*" >&2
  exit 1
}

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
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

require_file_hash() {
  local label="$1" file="$2" expected="$3"
  [[ -f "$file" ]] || fail "$label is missing: $file"
  [[ "$(sha256_file "$file")" == "$expected" ]] || fail "$label hash drifted"
}

command -v jq >/dev/null || fail "jq is required"
command -v shasum >/dev/null || fail "shasum is required"
command -v cmp >/dev/null || fail "cmp is required"

[[ "$R4_MANIFEST" == "$PINNED_R4_MANIFEST" ]] || fail "R4 manifest override is forbidden"
[[ "$R5_MANIFEST" == "$PINNED_R5_MANIFEST" || "$ALLOW_FIXTURE_MANIFEST" == "1" ]] || fail "R5 manifest override requires explicit fixture opt-in"
[[ "$R6_MANIFEST" == "$PINNED_R6_MANIFEST" || "$ALLOW_FIXTURE_MANIFEST" == "1" ]] || fail "R6 manifest override requires explicit fixture opt-in"
[[ "$R7_MANIFEST" == "$PINNED_R7_MANIFEST" || "$ALLOW_FIXTURE_MANIFEST" == "1" ]] || fail "R7 manifest override requires explicit fixture opt-in"
[[ "$R8_MANIFEST" == "$PINNED_R8_MANIFEST" || "$ALLOW_FIXTURE_MANIFEST" == "1" ]] || fail "R8 manifest override requires explicit fixture opt-in"
[[ "$R9_MANIFEST" == "$PINNED_R9_MANIFEST" || "$ALLOW_FIXTURE_MANIFEST" == "1" ]] || fail "R9 manifest override requires explicit fixture opt-in"
[[ "$R4_REF" == "$PINNED_R4_REF" ]] || fail "R4 ref does not match the pinned frozen ref"
[[ "$R4_HEAD" == "$PINNED_R4_HEAD" ]] || fail "R4 head does not match the pinned cutoff"
[[ "$R5_REF" == "$PINNED_R5_REF" ]] || fail "R5 ref does not match the pinned frozen ref"
[[ "$R5_HEAD" == "$PINNED_R5_HEAD" ]] || fail "R5 head does not match the pinned cutoff"
[[ "$R6_REF" == "$PINNED_R6_REF" ]] || fail "R6 ref does not match the pinned frozen ref"
[[ "$R6_HEAD" == "$PINNED_R6_HEAD" ]] || fail "R6 head does not match the pinned cutoff"
[[ "$R7_REF" == "$PINNED_R7_REF" ]] || fail "R7 ref does not match the pinned frozen ref"
[[ "$R7_HEAD" == "$PINNED_R7_HEAD" ]] || fail "R7 head does not match the pinned cutoff"
[[ "$R8_REF" == "$PINNED_R8_REF" ]] || fail "R8 ref does not match the pinned frozen ref"
[[ "$R8_HEAD" == "$PINNED_R8_HEAD" ]] || fail "R8 head does not match the pinned cutoff"
[[ "$R9_REF" == "$PINNED_R9_REF" ]] || fail "R9 ref does not match the pinned frozen ref"
[[ "$R9_HEAD" == "$PINNED_R9_HEAD" ]] || fail "R9 head does not match the pinned cutoff"

bash "$ROOT/scripts/hepta-upstream-codex-r3-integrity.sh" >/dev/null
require_file_hash "R3 predecessor manifest" "$PINNED_R3_MANIFEST" "$PINNED_R3_MANIFEST_SHA256"
require_file_hash "R4 manifest" "$R4_MANIFEST" "$PINNED_R4_MANIFEST_SHA256"
[[ -f "$R5_MANIFEST" ]] || fail "R5 manifest is missing: $R5_MANIFEST"
if [[ "$R5_MANIFEST" == "$PINNED_R5_MANIFEST" ]]; then
  require_file_hash "R5 manifest" "$R5_MANIFEST" "$PINNED_R5_MANIFEST_SHA256"
fi
[[ -f "$R6_MANIFEST" ]] || fail "R6 manifest is missing: $R6_MANIFEST"
if [[ "$R6_MANIFEST" == "$PINNED_R6_MANIFEST" ]]; then
  require_file_hash "R6 manifest" "$R6_MANIFEST" "$PINNED_R6_MANIFEST_SHA256"
fi
[[ -f "$R7_MANIFEST" ]] || fail "R7 manifest is missing: $R7_MANIFEST"
if [[ "$R7_MANIFEST" == "$PINNED_R7_MANIFEST" ]]; then
  require_file_hash "R7 manifest" "$R7_MANIFEST" "$PINNED_R7_MANIFEST_SHA256"
fi
[[ -f "$R8_MANIFEST" ]] || fail "R8 manifest is missing: $R8_MANIFEST"
if [[ "$R8_MANIFEST" == "$PINNED_R8_MANIFEST" ]]; then
  require_file_hash "R8 manifest" "$R8_MANIFEST" "$PINNED_R8_MANIFEST_SHA256"
fi
[[ -f "$R9_MANIFEST" ]] || fail "R9 manifest is missing: $R9_MANIFEST"
if [[ "$R9_MANIFEST" == "$PINNED_R9_MANIFEST" ]]; then
  require_file_hash "R9 manifest" "$R9_MANIFEST" "$PINNED_R9_MANIFEST_SHA256"
fi
require_file_hash "R4 security shard" "$PINNED_R4_SECURITY" "$PINNED_R4_SECURITY_SHA256"
require_file_hash "R4 protocol/app-server shard" "$PINNED_R4_PROTOCOL" "$PINNED_R4_PROTOCOL_SHA256"
require_file_hash "R4 tools/apps/runtime shard" "$PINNED_R4_TOOLS" "$PINNED_R4_TOOLS_SHA256"
require_file_hash "R4 TUI/other shard" "$PINNED_R4_TUI" "$PINNED_R4_TUI_SHA256"
require_file_hash "R4 file-surface shard" "$PINNED_R4_FILES" "$PINNED_R4_FILES_SHA256"

jq -e \
  --arg url "$PINNED_URL" \
  --arg r3_manifest "$PINNED_R3_MANIFEST" \
  --arg r3_manifest_sha "$PINNED_R3_MANIFEST_SHA256" \
  --arg r3_head "$PINNED_R3_HEAD" \
  --arg r4_head "$PINNED_R4_HEAD" \
  --arg range_digest "$PINNED_R4_RANGE_DIGEST" \
  --arg path_digest "$PINNED_R4_PATH_DIGEST" \
  --arg normalized_commit_digest "$PINNED_R4_NORMALIZED_COMMIT_DIGEST" \
  --arg normalized_file_digest "$PINNED_R4_NORMALIZED_FILE_DIGEST" '
    .schema_version == "hepta_upstream_codex_current_intake_v4"
    and .intake_id == "upstream-codex-intake-2026-07-24-r4"
    and .predecessor_intake == {
      manifest_path:$r3_manifest,
      manifest_sha256:$r3_manifest_sha,
      cutoff_head:$r3_head,
      preserved:true,
      modified_by_r4:false
    }
    and .observation.upstream_repository == $url
    and .observation.target_branch == "main"
    and .observation.observed_upstream_head == $r4_head
    and .observation.range_start_exclusive == $r3_head
    and .observation.range_end_inclusive == $r4_head
    and .observation.commit_count == 97
    and .observation.merge_commit_count == 0
    and .observation.net_changed_file_count == 744
    and .observation.history_relationship == "no_merge_base_unrelated_roots"
    and .observation.ordinary_merge_allowed == false
    and .observation.ordinary_rebase_allowed == false
    and .observation.direct_cherry_pick_default == false
    and .observation.merge_performed == false
    and .observation.rebase_performed == false
    and .observation.cherry_pick_performed == false
    and .observation.range_identity == {algorithm:"sha256(git_rev_list_reverse_range_lf)",digest:$range_digest}
    and .observation.file_surface_identity == {algorithm:"sha256(c_locale_sorted_name_status_lf)",digest:$path_digest}
    and .observation.normalized_commit_inventory_identity == {algorithm:"sha256(json_generate_sha_sorted_inventory)",digest:$normalized_commit_digest}
    and .observation.normalized_file_surface_identity == {algorithm:"sha256(json_generate_path_sorted_surface)",digest:$normalized_file_digest}
    and .classification_summary.status_counts == {candidate:41,deferred:34,rejected:22,imported:0}
    and .validation_contract.expected_status_counts == {candidate:41,deferred:34,rejected:22,imported:0}
    and .validation_contract.sharded_commit_count == 97
    and .validation_contract.sharded_file_count == 744
  ' "$R4_MANIFEST" >/dev/null || fail "R4 manifest contract drifted"

r4_shards=(
  "$PINNED_R4_SECURITY"
  "$PINNED_R4_PROTOCOL"
  "$PINNED_R4_TOOLS"
  "$PINNED_R4_TUI"
)
jq -se '
  [.[].commit_inventory[]] as $items
  | ($items | length) == 97
  and ([$items[].upstream_commit] | unique | length) == 97
  and ([$items[] | select(.status == "candidate")] | length) == 41
  and ([$items[] | select(.status == "deferred")] | length) == 34
  and ([$items[] | select(.status == "rejected")] | length) == 22
  and ([$items[] | select(.status == "imported" or .imported == true)] | length) == 0
  and ([$items[] | select(
    (has("upstream_commit")
      and has("primary_category")
      and has("status")
      and has("reason")
      and has("related_files")
      and has("semantic_absorption_recommendation")
      and has("imported")) | not
  )] | length) == 0
' "${r4_shards[@]}" >/dev/null || fail "R4 sharded commit inventory drifted"

r4_range="$PINNED_R3_HEAD..$R4_HEAD"
r4_commit_count="$(git rev-list --count "$r4_range")"
r4_merge_count="$(git rev-list --merges --count "$r4_range")"
r4_changed_file_count="$(git diff --name-only "$r4_range" | wc -l | tr -d '[:space:]')"
r4_range_digest="$(git rev-list --reverse "$r4_range" | shasum -a 256 | awk '{print $1}')"
r4_path_digest="$(git diff --name-status "$r4_range" | LC_ALL=C sort | shasum -a 256 | awk '{print $1}')"
[[ "$r4_commit_count" == "97" && "$r4_merge_count" == "0" ]] || fail "R4 commit inventory count drifted"
[[ "$r4_changed_file_count" == "744" ]] || fail "R4 file inventory count drifted"
[[ "$r4_range_digest" == "$PINNED_R4_RANGE_DIGEST" ]] || fail "R4 range digest drifted"
[[ "$r4_path_digest" == "$PINNED_R4_PATH_DIGEST" ]] || fail "R4 path surface digest drifted"

r4_expected_commits="$(mktemp)"
r4_manifest_commits="$(mktemp)"
r4_manifest_files="$(mktemp)"
r5_expected_commits="$(mktemp)"
r5_manifest_commits="$(mktemp)"
r5_expected_files="$(mktemp)"
r5_manifest_files="$(mktemp)"
r5_commit_expected_files="$(mktemp)"
r5_commit_manifest_files="$(mktemp)"
r6_expected_commits="$(mktemp)"
r6_manifest_commits="$(mktemp)"
r6_expected_files="$(mktemp)"
r6_manifest_files="$(mktemp)"
r6_commit_expected_files="$(mktemp)"
r6_commit_manifest_files="$(mktemp)"
r7_expected_commits="$(mktemp)"
r7_manifest_commits="$(mktemp)"
r7_expected_files="$(mktemp)"
r7_manifest_files="$(mktemp)"
r7_commit_expected_files="$(mktemp)"
r7_commit_manifest_files="$(mktemp)"
r8_expected_commits="$(mktemp)"
r8_manifest_commits="$(mktemp)"
r8_expected_files="$(mktemp)"
r8_manifest_files="$(mktemp)"
r8_commit_expected_files="$(mktemp)"
r8_commit_manifest_files="$(mktemp)"
r9_expected_commits="$(mktemp)"
r9_manifest_commits="$(mktemp)"
r9_expected_files="$(mktemp)"
r9_manifest_files="$(mktemp)"
r9_commit_expected_files="$(mktemp)"
r9_commit_manifest_files="$(mktemp)"
trap 'rm -f "$r4_expected_commits" "$r4_manifest_commits" "$r4_manifest_files" "$r5_expected_commits" "$r5_manifest_commits" "$r5_expected_files" "$r5_manifest_files" "$r5_commit_expected_files" "$r5_commit_manifest_files" "$r6_expected_commits" "$r6_manifest_commits" "$r6_expected_files" "$r6_manifest_files" "$r6_commit_expected_files" "$r6_commit_manifest_files" "$r7_expected_commits" "$r7_manifest_commits" "$r7_expected_files" "$r7_manifest_files" "$r7_commit_expected_files" "$r7_commit_manifest_files" "$r8_expected_commits" "$r8_manifest_commits" "$r8_expected_files" "$r8_manifest_files" "$r8_commit_expected_files" "$r8_commit_manifest_files" "$r9_expected_commits" "$r9_manifest_commits" "$r9_expected_files" "$r9_manifest_files" "$r9_commit_expected_files" "$r9_commit_manifest_files"' EXIT

git log --reverse --format='%H%x09%aI%x09%s' "$r4_range" | LC_ALL=C sort >"$r4_expected_commits"
jq -sr '.[].commit_inventory[] | [.upstream_commit,.authored_at,.title] | @tsv' "${r4_shards[@]}" | LC_ALL=C sort >"$r4_manifest_commits"
cmp -s "$r4_expected_commits" "$r4_manifest_commits" || fail "R4 commit shards do not match the complete range"
jq -r '.file_surface[] | if has("old_path") then [.status,.old_path,.path] else [.status,.path] end | @tsv' "$PINNED_R4_FILES" >"$r4_manifest_files"
git diff --name-status "$r4_range" | cmp -s - "$r4_manifest_files" || fail "R4 file-surface shard does not match the complete range"

jq -e \
  --arg url "$PINNED_URL" \
  --arg r4_manifest "$PINNED_R4_MANIFEST" \
  --arg r4_manifest_sha "$PINNED_R4_MANIFEST_SHA256" \
  --arg r4_head "$PINNED_R4_HEAD" \
  --arg r5_head "$PINNED_R5_HEAD" \
  --arg range_digest "$PINNED_R5_RANGE_DIGEST" \
  --arg path_digest "$PINNED_R5_PATH_DIGEST" '
    .schema_version == "hepta_upstream_codex_current_intake_v5"
    and .intake_id == "upstream-codex-intake-2026-07-24-r5"
    and .predecessor_intake == {
      manifest_path:$r4_manifest,
      manifest_sha256:$r4_manifest_sha,
      cutoff_head:$r4_head,
      preserved:true,
      modified_by_r5:false
    }
    and .observation.state == "observed_and_classified"
    and .observation.upstream_repository == $url
    and .observation.target_branch == "main"
    and .observation.observed_upstream_head == $r5_head
    and .observation.range_start_exclusive == $r4_head
    and .observation.range_end_inclusive == $r5_head
    and .observation.commit_count == 2
    and .observation.merge_commit_count == 0
    and .observation.net_changed_file_count == 28
    and .observation.net_insertions == 1191
    and .observation.net_deletions == 87
    and .observation.commit_file_touches == 28
    and .observation.history_relationship == "no_merge_base_unrelated_roots"
    and .observation.ordinary_merge_allowed == false
    and .observation.ordinary_rebase_allowed == false
    and .observation.direct_cherry_pick_default == false
    and .observation.required_integration_mode == "selective_semantic_transplant_with_behavioral_evidence"
    and .observation.range_identity == {algorithm:"sha256(git_rev_list_reverse_range_lf)",digest:$range_digest}
    and .observation.file_surface_identity == {algorithm:"sha256(c_locale_sorted_name_status_lf)",digest:$path_digest}
    and .classification_summary == {
      commit_count:2,
      status_counts:{candidate:2,deferred:0,rejected:0,imported:0},
      priority_counts:{P1:2},
      category_counts:{tools_apps_runtime:2}
    }
    and ([.commit_inventory[].upstream_commit] | length == 2 and length == (unique | length))
    and ([.commit_inventory[] | select(.status != "candidate" or .imported != false or .imported_evidence != null)] | length) == 0
    and .selective_semantic_transplant.ordinary_merge_rebase == "forbidden_no_merge_base"
    and .selective_semantic_transplant.direct_cherry_pick == "not_default"
    and .claims == {
      upstream_fully_consumed:false,
      source_imported_at_observation:false,
      merge_performed:false,
      rebase_performed:false,
      cherry_pick_performed:false,
      deployment_performed:false,
      live_enablement_performed:false
    }
  ' "$R5_MANIFEST" >/dev/null || fail "R5 manifest contract drifted"

r5_normalized_commit_digest="$(jq -cS '.commit_inventory' "$R5_MANIFEST" | shasum -a 256 | awk '{print $1}')"
r5_related_path_digest="$(jq -r '[.commit_inventory[].related_files[]] | unique[]' "$R5_MANIFEST" | shasum -a 256 | awk '{print $1}')"
[[ "$r5_normalized_commit_digest" == "$PINNED_R5_NORMALIZED_COMMIT_DIGEST" ]] || fail "R5 normalized commit inventory digest drifted"
[[ "$r5_related_path_digest" == "$PINNED_R5_RELATED_PATH_DIGEST" ]] || fail "R5 related path inventory digest drifted"

r5_range="$R4_HEAD..$R5_HEAD"
r5_commit_count="$(git rev-list --count "$r5_range")"
r5_merge_count="$(git rev-list --merges --count "$r5_range")"
r5_changed_file_count="$(git diff --name-only "$r5_range" | wc -l | tr -d '[:space:]')"
r5_range_digest="$(git rev-list --reverse "$r5_range" | shasum -a 256 | awk '{print $1}')"
r5_path_digest="$(git diff --name-status "$r5_range" | LC_ALL=C sort | shasum -a 256 | awk '{print $1}')"
[[ "$r5_commit_count" == "2" && "$r5_merge_count" == "0" ]] || fail "R5 commit inventory count drifted"
[[ "$r5_changed_file_count" == "28" ]] || fail "R5 file inventory count drifted"
[[ "$r5_range_digest" == "$PINNED_R5_RANGE_DIGEST" ]] || fail "R5 range digest drifted"
[[ "$r5_path_digest" == "$PINNED_R5_PATH_DIGEST" ]] || fail "R5 path surface digest drifted"

git log --reverse --format='%H%x09%aI' "$r5_range" >"$r5_expected_commits"
jq -r '.commit_inventory[] | [.upstream_commit,.authored_at] | @tsv' "$R5_MANIFEST" >"$r5_manifest_commits"
cmp -s "$r5_expected_commits" "$r5_manifest_commits" || fail "R5 commit identity inventory does not match the complete range"
git diff --name-only "$r5_range" | LC_ALL=C sort >"$r5_expected_files"
jq -r '[.commit_inventory[].related_files[]] | unique[]' "$R5_MANIFEST" >"$r5_manifest_files"
cmp -s "$r5_expected_files" "$r5_manifest_files" || fail "R5 related path inventory does not match the complete range"

while IFS= read -r r5_commit; do
  git show --format= --name-only "$r5_commit" | sed '/^$/d' | LC_ALL=C sort -u >"$r5_commit_expected_files"
  jq -r --arg commit "$r5_commit" '.commit_inventory[] | select(.upstream_commit == $commit) | .related_files[]' "$R5_MANIFEST" | LC_ALL=C sort -u >"$r5_commit_manifest_files"
  cmp -s "$r5_commit_expected_files" "$r5_commit_manifest_files" || fail "R5 per-commit path inventory drifted for $r5_commit"
done < <(git rev-list --reverse "$r5_range")

jq -e \
  --arg url "$PINNED_URL" \
  --arg r5_manifest "$PINNED_R5_MANIFEST" \
  --arg r5_manifest_sha "$PINNED_R5_MANIFEST_SHA256" \
  --arg r5_head "$PINNED_R5_HEAD" \
  --arg r6_head "$PINNED_R6_HEAD" \
  --arg range_digest "$PINNED_R6_RANGE_DIGEST" \
  --arg path_digest "$PINNED_R6_PATH_DIGEST" \
  --arg commit_identity_digest "$PINNED_R6_COMMIT_IDENTITY_DIGEST" \
  --arg normalized_commit_digest "$PINNED_R6_NORMALIZED_COMMIT_DIGEST" \
  --arg related_path_digest "$PINNED_R6_RELATED_PATH_DIGEST" '
    .schema_version == "hepta_upstream_codex_current_intake_v6"
    and .intake_id == "upstream-codex-intake-2026-07-24-r6"
    and .predecessor_intake == {
      manifest_path:$r5_manifest,
      manifest_sha256:$r5_manifest_sha,
      cutoff_head:$r5_head,
      preserved:true,
      modified_by_r6:false
    }
    and .observation.state == "observed_and_classified"
    and .observation.upstream_repository == $url
    and .observation.target_branch == "main"
    and .observation.target_ref == "refs/remotes/upstream/main"
    and .observation.observed_upstream_head == $r6_head
    and .observation.range_start_exclusive == $r5_head
    and .observation.range_end_inclusive == $r6_head
    and .observation.commit_count == 2
    and .observation.merge_commit_count == 0
    and .observation.net_changed_file_count == 11
    and .observation.net_insertions == 465
    and .observation.net_deletions == 47
    and .observation.commit_file_touches == 14
    and .observation.history_relationship == "no_merge_base_unrelated_roots"
    and .observation.predecessor_relationship == "linear_ancestor"
    and .observation.ordinary_merge_allowed == false
    and .observation.ordinary_rebase_allowed == false
    and .observation.direct_cherry_pick_default == false
    and .observation.required_integration_mode == "selective_semantic_transplant_with_behavioral_evidence"
    and .observation.range_identity == {algorithm:"sha256(git_rev_list_reverse_range_lf)",digest:$range_digest}
    and .observation.file_surface_identity == {algorithm:"sha256(c_locale_sorted_name_status_lf)",digest:$path_digest}
    and .observation.commit_identity == {algorithm:"sha256(git_log_reverse_sha_authored_at_title_lf)",digest:$commit_identity_digest}
    and .observation.normalized_commit_inventory_identity == {algorithm:"sha256(jq_canonical_sorted_commit_inventory)",digest:$normalized_commit_digest}
    and .observation.related_path_inventory_identity == {algorithm:"sha256(c_locale_sorted_unique_related_paths_lf)",digest:$related_path_digest}
    and .classification_summary == {
      commit_count:2,
      status_counts:{candidate:2,deferred:0,rejected:0,imported:0},
      priority_counts:{P1:2},
      category_counts:{tools_apps_runtime:2}
    }
    and ([.commit_inventory[].upstream_commit] | length == 2 and length == (unique | length))
    and (.commit_inventory | all(.[]; (.related_files | length) == .changed_file_count))
    and ([.commit_inventory[] | select(.status != "candidate" or .imported != false or .imported_evidence != null)] | length) == 0
    and .selective_semantic_transplant.ordinary_merge_rebase == "forbidden_no_merge_base"
    and .selective_semantic_transplant.direct_cherry_pick == "not_default"
    and [.selective_semantic_transplant.ordered_slices[].id] == [
      "r6-p1-mcp-auth-refresh-consistency",
      "r6-p1-mcp-prewarm-coalescing"
    ]
    and .claims == {
      upstream_fully_consumed:false,
      source_imported_at_observation:false,
      merge_performed:false,
      rebase_performed:false,
      cherry_pick_performed:false,
      deployment_performed:false,
      live_enablement_performed:false
    }
  ' "$R6_MANIFEST" >/dev/null || fail "R6 manifest contract drifted"

r6_normalized_commit_digest="$(jq -cS '.commit_inventory' "$R6_MANIFEST" | shasum -a 256 | awk '{print $1}')"
r6_related_path_digest="$(jq -r '[.commit_inventory[].related_files[]] | unique[]' "$R6_MANIFEST" | shasum -a 256 | awk '{print $1}')"
[[ "$r6_normalized_commit_digest" == "$PINNED_R6_NORMALIZED_COMMIT_DIGEST" ]] || fail "R6 normalized commit inventory digest drifted"
[[ "$r6_related_path_digest" == "$PINNED_R6_RELATED_PATH_DIGEST" ]] || fail "R6 related path inventory digest drifted"

jq -e \
  --arg url "$PINNED_URL" \
  --arg r6_manifest "$PINNED_R6_MANIFEST" \
  --arg r6_manifest_sha "$PINNED_R6_MANIFEST_SHA256" \
  --arg r6_head "$PINNED_R6_HEAD" \
  --arg r7_head "$PINNED_R7_HEAD" \
  --arg range_digest "$PINNED_R7_RANGE_DIGEST" \
  --arg path_digest "$PINNED_R7_PATH_DIGEST" \
  --arg commit_identity_digest "$PINNED_R7_COMMIT_IDENTITY_DIGEST" \
  --arg normalized_commit_digest "$PINNED_R7_NORMALIZED_COMMIT_DIGEST" \
  --arg related_path_digest "$PINNED_R7_RELATED_PATH_DIGEST" '
    .schema_version == "hepta_upstream_codex_current_intake_v7"
    and .intake_id == "upstream-codex-intake-2026-07-24-r7"
    and .predecessor_intake == {
      manifest_path:$r6_manifest,
      manifest_sha256:$r6_manifest_sha,
      cutoff_head:$r6_head,
      preserved:true,
      modified_by_r7:false
    }
    and .observation.state == "observed_and_classified"
    and .observation.upstream_repository == $url
    and .observation.target_branch == "main"
    and .observation.target_ref == "refs/remotes/upstream/main"
    and .observation.observed_upstream_head == $r7_head
    and .observation.range_start_exclusive == $r6_head
    and .observation.range_end_inclusive == $r7_head
    and .observation.commit_count == 1
    and .observation.merge_commit_count == 0
    and .observation.net_changed_file_count == 3
    and .observation.net_insertions == 65
    and .observation.net_deletions == 8
    and .observation.commit_file_touches == 3
    and .observation.history_relationship == "no_merge_base_unrelated_roots"
    and .observation.predecessor_relationship == "linear_ancestor"
    and .observation.ordinary_merge_allowed == false
    and .observation.ordinary_rebase_allowed == false
    and .observation.direct_cherry_pick_default == false
    and .observation.required_integration_mode == "selective_semantic_transplant_with_behavioral_evidence"
    and .observation.range_identity == {algorithm:"sha256(git_rev_list_reverse_range_lf)",digest:$range_digest}
    and .observation.file_surface_identity == {algorithm:"sha256(c_locale_sorted_name_status_lf)",digest:$path_digest}
    and .observation.commit_identity == {algorithm:"sha256(git_log_reverse_sha_authored_at_title_lf)",digest:$commit_identity_digest}
    and .observation.normalized_commit_inventory_identity == {algorithm:"sha256(jq_canonical_sorted_commit_inventory)",digest:$normalized_commit_digest}
    and .observation.related_path_inventory_identity == {algorithm:"sha256(c_locale_sorted_unique_related_paths_lf)",digest:$related_path_digest}
    and .classification_summary == {
      commit_count:1,
      status_counts:{candidate:1,deferred:0,rejected:0,imported:0},
      priority_counts:{P1:1},
      category_counts:{tools_apps_runtime:1}
    }
    and ([.commit_inventory[].upstream_commit] | length == 1 and length == (unique | length))
    and (.commit_inventory | all(.[]; (.related_files | length) == .changed_file_count))
    and ([.commit_inventory[] | select(.status != "candidate" or .imported != false or .imported_evidence != null)] | length) == 0
    and .selective_semantic_transplant.ordinary_merge_rebase == "forbidden_no_merge_base"
    and .selective_semantic_transplant.direct_cherry_pick == "not_default"
    and [.selective_semantic_transplant.ordered_slices[].id] == [
      "r7-p1-explicit-mcp-reconnect"
    ]
    and .claims == {
      upstream_fully_consumed:false,
      source_imported_at_observation:false,
      merge_performed:false,
      rebase_performed:false,
      cherry_pick_performed:false,
      deployment_performed:false,
      live_enablement_performed:false
    }
  ' "$R7_MANIFEST" >/dev/null || fail "R7 manifest contract drifted"

r7_normalized_commit_digest="$(jq -cS '.commit_inventory' "$R7_MANIFEST" | shasum -a 256 | awk '{print $1}')"
r7_related_path_digest="$(jq -r '[.commit_inventory[].related_files[]] | unique[]' "$R7_MANIFEST" | shasum -a 256 | awk '{print $1}')"
[[ "$r7_normalized_commit_digest" == "$PINNED_R7_NORMALIZED_COMMIT_DIGEST" ]] || fail "R7 normalized commit inventory digest drifted"
[[ "$r7_related_path_digest" == "$PINNED_R7_RELATED_PATH_DIGEST" ]] || fail "R7 related path inventory digest drifted"

jq -e \
  --arg url "$PINNED_URL" \
  --arg r7_manifest "$PINNED_R7_MANIFEST" \
  --arg r7_manifest_sha "$PINNED_R7_MANIFEST_SHA256" \
  --arg r7_head "$PINNED_R7_HEAD" \
  --arg r8_head "$PINNED_R8_HEAD" \
  --arg range_digest "$PINNED_R8_RANGE_DIGEST" \
  --arg path_digest "$PINNED_R8_PATH_DIGEST" \
  --arg commit_identity_digest "$PINNED_R8_COMMIT_IDENTITY_DIGEST" \
  --arg normalized_commit_digest "$PINNED_R8_NORMALIZED_COMMIT_DIGEST" \
  --arg related_path_digest "$PINNED_R8_RELATED_PATH_DIGEST" '
    .schema_version == "hepta_upstream_codex_current_intake_v8"
    and .intake_id == "upstream-codex-intake-2026-07-24-r8"
    and .predecessor_intake == {
      manifest_path:$r7_manifest,
      manifest_sha256:$r7_manifest_sha,
      cutoff_head:$r7_head,
      preserved:true,
      modified_by_r8:false
    }
    and .observation.state == "observed_and_classified"
    and .observation.upstream_repository == $url
    and .observation.target_branch == "main"
    and .observation.target_ref == "refs/remotes/upstream/main"
    and .observation.observed_upstream_head == $r8_head
    and .observation.range_start_exclusive == $r7_head
    and .observation.range_end_inclusive == $r8_head
    and .observation.commit_count == 1
    and .observation.merge_commit_count == 0
    and .observation.net_changed_file_count == 6
    and .observation.net_insertions == 81
    and .observation.net_deletions == 63
    and .observation.commit_file_touches == 6
    and .observation.history_relationship == "no_merge_base_unrelated_roots"
    and .observation.predecessor_relationship == "linear_ancestor"
    and .observation.ordinary_merge_allowed == false
    and .observation.ordinary_rebase_allowed == false
    and .observation.direct_cherry_pick_default == false
    and .observation.required_integration_mode == "selective_semantic_transplant_with_behavioral_evidence"
    and .observation.range_identity == {algorithm:"sha256(git_rev_list_reverse_range_lf)",digest:$range_digest}
    and .observation.file_surface_identity == {algorithm:"sha256(c_locale_sorted_name_status_lf)",digest:$path_digest}
    and .observation.commit_identity == {algorithm:"sha256(git_log_reverse_sha_authored_at_title_lf)",digest:$commit_identity_digest}
    and .observation.normalized_commit_inventory_identity == {algorithm:"sha256(jq_canonical_sorted_commit_inventory)",digest:$normalized_commit_digest}
    and .observation.related_path_inventory_identity == {algorithm:"sha256(c_locale_sorted_unique_related_paths_lf)",digest:$related_path_digest}
    and .classification_summary == {
      commit_count:1,
      status_counts:{candidate:1,deferred:0,rejected:0,imported:0},
      priority_counts:{P1:1},
      category_counts:{tools_apps_runtime:1}
    }
    and ([.commit_inventory[].upstream_commit] | length == 1 and length == (unique | length))
    and (.commit_inventory | all(.[]; (.related_files | length) == .changed_file_count))
    and ([.commit_inventory[] | select(.status != "candidate" or .imported != false or .imported_evidence != null)] | length) == 0
    and .selective_semantic_transplant.ordinary_merge_rebase == "forbidden_no_merge_base"
    and .selective_semantic_transplant.direct_cherry_pick == "not_default"
    and [.selective_semantic_transplant.ordered_slices[].id] == [
      "r8-p1-mcp-refresh-coordinator"
    ]
    and .claims == {
      upstream_fully_consumed:false,
      source_imported_at_observation:false,
      merge_performed:false,
      rebase_performed:false,
      cherry_pick_performed:false,
      deployment_performed:false,
      live_enablement_performed:false
    }
  ' "$R8_MANIFEST" >/dev/null || fail "R8 manifest contract drifted"

r8_normalized_commit_digest="$(jq -cS '.commit_inventory' "$R8_MANIFEST" | shasum -a 256 | awk '{print $1}')"
r8_related_path_digest="$(jq -r '[.commit_inventory[].related_files[]] | unique[]' "$R8_MANIFEST" | shasum -a 256 | awk '{print $1}')"
[[ "$r8_normalized_commit_digest" == "$PINNED_R8_NORMALIZED_COMMIT_DIGEST" ]] || fail "R8 normalized commit inventory digest drifted"
[[ "$r8_related_path_digest" == "$PINNED_R8_RELATED_PATH_DIGEST" ]] || fail "R8 related path inventory digest drifted"

jq -e \
  --arg url "$PINNED_URL" \
  --arg r8_manifest "$PINNED_R8_MANIFEST" \
  --arg r8_manifest_sha "$PINNED_R8_MANIFEST_SHA256" \
  --arg r8_head "$PINNED_R8_HEAD" \
  --arg r9_head "$PINNED_R9_HEAD" \
  --arg range_digest "$PINNED_R9_RANGE_DIGEST" \
  --arg path_digest "$PINNED_R9_PATH_DIGEST" \
  --arg commit_identity_digest "$PINNED_R9_COMMIT_IDENTITY_DIGEST" \
  --arg normalized_commit_digest "$PINNED_R9_NORMALIZED_COMMIT_DIGEST" \
  --arg related_path_digest "$PINNED_R9_RELATED_PATH_DIGEST" '
    .schema_version == "hepta_upstream_codex_current_intake_v9"
    and .intake_id == "upstream-codex-intake-2026-07-24-r9"
    and .predecessor_intake == {
      manifest_path:$r8_manifest,
      manifest_sha256:$r8_manifest_sha,
      cutoff_head:$r8_head,
      preserved:true,
      modified_by_r9:false
    }
    and .observation.state == "observed_and_classified"
    and .observation.upstream_repository == $url
    and .observation.target_branch == "main"
    and .observation.target_ref == "refs/remotes/upstream/main"
    and .observation.observed_upstream_head == $r9_head
    and .observation.range_start_exclusive == $r8_head
    and .observation.range_end_inclusive == $r9_head
    and .observation.commit_count == 9
    and .observation.merge_commit_count == 0
    and .observation.net_changed_file_count == 56
    and .observation.net_insertions == 2439
    and .observation.net_deletions == 303
    and .observation.commit_file_touches == 68
    and .observation.history_relationship == "no_merge_base_unrelated_roots"
    and .observation.predecessor_relationship == "linear_ancestor"
    and .observation.ordinary_merge_allowed == false
    and .observation.ordinary_rebase_allowed == false
    and .observation.direct_cherry_pick_default == false
    and .observation.required_integration_mode == "selective_semantic_transplant_with_behavioral_evidence"
    and .observation.range_identity == {algorithm:"sha256(git_rev_list_reverse_range_lf)",digest:$range_digest}
    and .observation.file_surface_identity == {algorithm:"sha256(c_locale_sorted_name_status_lf)",digest:$path_digest}
    and .observation.commit_identity == {algorithm:"sha256(git_log_reverse_sha_authored_at_title_lf)",digest:$commit_identity_digest}
    and .observation.normalized_commit_inventory_identity == {algorithm:"sha256(jq_canonical_sorted_commit_inventory)",digest:$normalized_commit_digest}
    and .observation.related_path_inventory_identity == {algorithm:"sha256(c_locale_sorted_unique_related_paths_lf)",digest:$related_path_digest}
    and .classification_summary == {
      commit_count:9,
      status_counts:{candidate:4,deferred:3,rejected:2,imported:0},
      priority_counts:{P0:3,P1:2,P2:2,P3:2},
      category_counts:{skill_extensions:4,test_reliability:2,hooks_runtime:1,tools_apps_runtime:2}
    }
    and ([.commit_inventory[].upstream_commit] | length == 9 and length == (unique | length))
    and (.commit_inventory | all(.[]; (.related_files | length) == .changed_file_count))
    and ([.commit_inventory[] | select(.imported != false or .imported_evidence != null)] | length) == 0
    and ([.commit_inventory[] | select(.status == "candidate")] | length) == 4
    and ([.commit_inventory[] | select(.status == "deferred")] | length) == 3
    and ([.commit_inventory[] | select(.status == "rejected")] | length) == 2
    and .selective_semantic_transplant.ordinary_merge_rebase == "forbidden_no_merge_base"
    and .selective_semantic_transplant.direct_cherry_pick == "not_default"
    and [.selective_semantic_transplant.ordered_slices[].id] == [
      "r9-p0-executor-skill-authority",
      "r9-p0-current-mcp-authority",
      "r9-p1-hook-broken-pipe-output",
      "r9-p1-mcp-runtime-refresh-window",
      "r9-p2-extension-warning-and-path-compaction",
      "r9-p3-upstream-test-stabilization"
    ]
    and .claims == {
      upstream_fully_consumed:false,
      source_imported_at_observation:false,
      merge_performed:false,
      rebase_performed:false,
      cherry_pick_performed:false,
      deployment_performed:false,
      live_enablement_performed:false
    }
  ' "$R9_MANIFEST" >/dev/null || fail "R9 manifest contract drifted"

r9_normalized_commit_digest="$(jq -cS '.commit_inventory' "$R9_MANIFEST" | shasum -a 256 | awk '{print $1}')"
r9_related_path_digest="$(jq -r '[.commit_inventory[].related_files[]] | unique[]' "$R9_MANIFEST" | shasum -a 256 | awk '{print $1}')"
[[ "$r9_normalized_commit_digest" == "$PINNED_R9_NORMALIZED_COMMIT_DIGEST" ]] || fail "R9 normalized commit inventory digest drifted"
[[ "$r9_related_path_digest" == "$PINNED_R9_RELATED_PATH_DIGEST" ]] || fail "R9 related path inventory digest drifted"

resolve_direct_commit_ref "R3 historical cutoff" "$PINNED_R3_REF" "$PINNED_R3_HEAD"
resolve_direct_commit_ref "R4 frozen cutoff" "$R4_REF" "$R4_HEAD"
resolve_direct_commit_ref "R5 historical cutoff" "$R5_REF" "$R5_HEAD"
resolve_direct_commit_ref "R6 historical cutoff" "$R6_REF" "$R6_HEAD"
resolve_direct_commit_ref "R7 historical cutoff" "$R7_REF" "$R7_HEAD"
resolve_direct_commit_ref "R8 frozen cutoff" "$R8_REF" "$R8_HEAD"
resolve_direct_commit_ref "R9 frozen cutoff" "$R9_REF" "$R9_HEAD"
git merge-base --is-ancestor "$PINNED_R3_HEAD" "$R4_HEAD" || fail "R3 is not an ancestor of R4"
git merge-base --is-ancestor "$R4_HEAD" "$R5_HEAD" || fail "R4 is not an ancestor of R5"
git merge-base --is-ancestor "$R5_HEAD" "$R6_HEAD" || fail "R5 is not an ancestor of R6"
git merge-base --is-ancestor "$R6_HEAD" "$R7_HEAD" || fail "R6 is not an ancestor of R7"
git merge-base --is-ancestor "$R7_HEAD" "$R8_HEAD" || fail "R7 is not an ancestor of R8"
git merge-base --is-ancestor "$R8_HEAD" "$R9_HEAD" || fail "R8 is not an ancestor of R9"

r6_range="$R5_HEAD..$R6_HEAD"
r6_commit_count="$(git rev-list --count "$r6_range")"
r6_merge_count="$(git rev-list --merges --count "$r6_range")"
r6_changed_file_count="$(git diff --name-only "$r6_range" | wc -l | tr -d '[:space:]')"
r6_net_insertions="$(git diff --numstat "$r6_range" | awk '{insertions += $1} END {print insertions + 0}')"
r6_net_deletions="$(git diff --numstat "$r6_range" | awk '{deletions += $2} END {print deletions + 0}')"
r6_commit_file_touches="$(
  while IFS= read -r r6_commit; do
    git show --format= --name-only "$r6_commit" | sed '/^$/d'
  done < <(git rev-list --reverse "$r6_range") | wc -l | tr -d '[:space:]'
)"
r6_range_digest="$(git rev-list --reverse "$r6_range" | shasum -a 256 | awk '{print $1}')"
r6_path_digest="$(git diff --name-status "$r6_range" | LC_ALL=C sort | shasum -a 256 | awk '{print $1}')"
r6_commit_identity_digest="$(git log --reverse --format='%H%x09%aI%x09%s' "$r6_range" | shasum -a 256 | awk '{print $1}')"
[[ "$r6_commit_count" == "2" && "$r6_merge_count" == "0" ]] || fail "R6 commit inventory count drifted"
[[ "$r6_changed_file_count" == "11" ]] || fail "R6 file inventory count drifted"
[[ "$r6_net_insertions" == "465" && "$r6_net_deletions" == "47" ]] || fail "R6 net line counts drifted"
[[ "$r6_commit_file_touches" == "14" ]] || fail "R6 commit file-touch count drifted"
[[ "$r6_range_digest" == "$PINNED_R6_RANGE_DIGEST" ]] || fail "R6 range digest drifted"
[[ "$r6_path_digest" == "$PINNED_R6_PATH_DIGEST" ]] || fail "R6 path surface digest drifted"
[[ "$r6_commit_identity_digest" == "$PINNED_R6_COMMIT_IDENTITY_DIGEST" ]] || fail "R6 commit identity digest drifted"

git log --reverse --format='%H%x09%aI%x09%s' "$r6_range" >"$r6_expected_commits"
jq -r '.commit_inventory[] | [.upstream_commit,.authored_at,.title] | @tsv' "$R6_MANIFEST" >"$r6_manifest_commits"
cmp -s "$r6_expected_commits" "$r6_manifest_commits" || fail "R6 commit identity inventory does not match the complete range"
git diff --name-only "$r6_range" | LC_ALL=C sort >"$r6_expected_files"
jq -r '[.commit_inventory[].related_files[]] | unique[]' "$R6_MANIFEST" >"$r6_manifest_files"
cmp -s "$r6_expected_files" "$r6_manifest_files" || fail "R6 related path inventory does not match the complete range"

while IFS= read -r r6_commit; do
  git show --format= --name-only "$r6_commit" | sed '/^$/d' | LC_ALL=C sort -u >"$r6_commit_expected_files"
  jq -r --arg commit "$r6_commit" '.commit_inventory[] | select(.upstream_commit == $commit) | .related_files[]' "$R6_MANIFEST" | LC_ALL=C sort -u >"$r6_commit_manifest_files"
  cmp -s "$r6_commit_expected_files" "$r6_commit_manifest_files" || fail "R6 per-commit path inventory drifted for $r6_commit"
done < <(git rev-list --reverse "$r6_range")

r7_range="$R6_HEAD..$R7_HEAD"
r7_commit_count="$(git rev-list --count "$r7_range")"
r7_merge_count="$(git rev-list --merges --count "$r7_range")"
r7_changed_file_count="$(git diff --name-only "$r7_range" | wc -l | tr -d '[:space:]')"
r7_net_insertions="$(git diff --numstat "$r7_range" | awk '{insertions += $1} END {print insertions + 0}')"
r7_net_deletions="$(git diff --numstat "$r7_range" | awk '{deletions += $2} END {print deletions + 0}')"
r7_commit_file_touches="$(
  while IFS= read -r r7_commit; do
    git show --format= --name-only "$r7_commit" | sed '/^$/d'
  done < <(git rev-list --reverse "$r7_range") | wc -l | tr -d '[:space:]'
)"
r7_range_digest="$(git rev-list --reverse "$r7_range" | shasum -a 256 | awk '{print $1}')"
r7_path_digest="$(git diff --name-status "$r7_range" | LC_ALL=C sort | shasum -a 256 | awk '{print $1}')"
r7_commit_identity_digest="$(git log --reverse --format='%H%x09%aI%x09%s' "$r7_range" | shasum -a 256 | awk '{print $1}')"
[[ "$r7_commit_count" == "1" && "$r7_merge_count" == "0" ]] || fail "R7 commit inventory count drifted"
[[ "$r7_changed_file_count" == "3" ]] || fail "R7 file inventory count drifted"
[[ "$r7_net_insertions" == "65" && "$r7_net_deletions" == "8" ]] || fail "R7 net line counts drifted"
[[ "$r7_commit_file_touches" == "3" ]] || fail "R7 commit file-touch count drifted"
[[ "$r7_range_digest" == "$PINNED_R7_RANGE_DIGEST" ]] || fail "R7 range digest drifted"
[[ "$r7_path_digest" == "$PINNED_R7_PATH_DIGEST" ]] || fail "R7 path surface digest drifted"
[[ "$r7_commit_identity_digest" == "$PINNED_R7_COMMIT_IDENTITY_DIGEST" ]] || fail "R7 commit identity digest drifted"

git log --reverse --format='%H%x09%aI%x09%s' "$r7_range" >"$r7_expected_commits"
jq -r '.commit_inventory[] | [.upstream_commit,.authored_at,.title] | @tsv' "$R7_MANIFEST" >"$r7_manifest_commits"
cmp -s "$r7_expected_commits" "$r7_manifest_commits" || fail "R7 commit identity inventory does not match the complete range"
git diff --name-only "$r7_range" | LC_ALL=C sort >"$r7_expected_files"
jq -r '[.commit_inventory[].related_files[]] | unique[]' "$R7_MANIFEST" >"$r7_manifest_files"
cmp -s "$r7_expected_files" "$r7_manifest_files" || fail "R7 related path inventory does not match the complete range"

while IFS= read -r r7_commit; do
  git show --format= --name-only "$r7_commit" | sed '/^$/d' | LC_ALL=C sort -u >"$r7_commit_expected_files"
  jq -r --arg commit "$r7_commit" '.commit_inventory[] | select(.upstream_commit == $commit) | .related_files[]' "$R7_MANIFEST" | LC_ALL=C sort -u >"$r7_commit_manifest_files"
  cmp -s "$r7_commit_expected_files" "$r7_commit_manifest_files" || fail "R7 per-commit path inventory drifted for $r7_commit"
done < <(git rev-list --reverse "$r7_range")

r8_range="$R7_HEAD..$R8_HEAD"
r8_commit_count="$(git rev-list --count "$r8_range")"
r8_merge_count="$(git rev-list --merges --count "$r8_range")"
r8_changed_file_count="$(git diff --name-only "$r8_range" | wc -l | tr -d '[:space:]')"
r8_net_insertions="$(git diff --numstat "$r8_range" | awk '{insertions += $1} END {print insertions + 0}')"
r8_net_deletions="$(git diff --numstat "$r8_range" | awk '{deletions += $2} END {print deletions + 0}')"
r8_commit_file_touches="$(
  while IFS= read -r r8_commit; do
    git show --format= --name-only "$r8_commit" | sed '/^$/d'
  done < <(git rev-list --reverse "$r8_range") | wc -l | tr -d '[:space:]'
)"
r8_range_digest="$(git rev-list --reverse "$r8_range" | shasum -a 256 | awk '{print $1}')"
r8_path_digest="$(git diff --name-status "$r8_range" | LC_ALL=C sort | shasum -a 256 | awk '{print $1}')"
r8_commit_identity_digest="$(git log --reverse --format='%H%x09%aI%x09%s' "$r8_range" | shasum -a 256 | awk '{print $1}')"
[[ "$r8_commit_count" == "1" && "$r8_merge_count" == "0" ]] || fail "R8 commit inventory count drifted"
[[ "$r8_changed_file_count" == "6" ]] || fail "R8 file inventory count drifted"
[[ "$r8_net_insertions" == "81" && "$r8_net_deletions" == "63" ]] || fail "R8 net line counts drifted"
[[ "$r8_commit_file_touches" == "6" ]] || fail "R8 commit file-touch count drifted"
[[ "$r8_range_digest" == "$PINNED_R8_RANGE_DIGEST" ]] || fail "R8 range digest drifted"
[[ "$r8_path_digest" == "$PINNED_R8_PATH_DIGEST" ]] || fail "R8 path surface digest drifted"
[[ "$r8_commit_identity_digest" == "$PINNED_R8_COMMIT_IDENTITY_DIGEST" ]] || fail "R8 commit identity digest drifted"

git log --reverse --format='%H%x09%aI%x09%s' "$r8_range" >"$r8_expected_commits"
jq -r '.commit_inventory[] | [.upstream_commit,.authored_at,.title] | @tsv' "$R8_MANIFEST" >"$r8_manifest_commits"
cmp -s "$r8_expected_commits" "$r8_manifest_commits" || fail "R8 commit identity inventory does not match the complete range"
git diff --name-only "$r8_range" | LC_ALL=C sort >"$r8_expected_files"
jq -r '[.commit_inventory[].related_files[]] | unique[]' "$R8_MANIFEST" >"$r8_manifest_files"
cmp -s "$r8_expected_files" "$r8_manifest_files" || fail "R8 related path inventory does not match the complete range"

while IFS= read -r r8_commit; do
  git show --format= --name-only "$r8_commit" | sed '/^$/d' | LC_ALL=C sort -u >"$r8_commit_expected_files"
  jq -r --arg commit "$r8_commit" '.commit_inventory[] | select(.upstream_commit == $commit) | .related_files[]' "$R8_MANIFEST" | LC_ALL=C sort -u >"$r8_commit_manifest_files"
  cmp -s "$r8_commit_expected_files" "$r8_commit_manifest_files" || fail "R8 per-commit path inventory drifted for $r8_commit"
done < <(git rev-list --reverse "$r8_range")

r9_range="$R8_HEAD..$R9_HEAD"
r9_commit_count="$(git rev-list --count "$r9_range")"
r9_merge_count="$(git rev-list --merges --count "$r9_range")"
r9_changed_file_count="$(git diff --name-only "$r9_range" | wc -l | tr -d '[:space:]')"
r9_net_insertions="$(git diff --numstat "$r9_range" | awk '{insertions += $1} END {print insertions + 0}')"
r9_net_deletions="$(git diff --numstat "$r9_range" | awk '{deletions += $2} END {print deletions + 0}')"
r9_commit_file_touches="$(
  while IFS= read -r r9_commit; do
    git show --format= --name-only "$r9_commit" | sed '/^$/d'
  done < <(git rev-list --reverse "$r9_range") | wc -l | tr -d '[:space:]'
)"
r9_range_digest="$(git rev-list --reverse "$r9_range" | shasum -a 256 | awk '{print $1}')"
r9_path_digest="$(git diff --name-status "$r9_range" | LC_ALL=C sort | shasum -a 256 | awk '{print $1}')"
r9_commit_identity_digest="$(git log --reverse --format='%H%x09%aI%x09%s' "$r9_range" | shasum -a 256 | awk '{print $1}')"
[[ "$r9_commit_count" == "9" && "$r9_merge_count" == "0" ]] || fail "R9 commit inventory count drifted"
[[ "$r9_changed_file_count" == "56" ]] || fail "R9 file inventory count drifted"
[[ "$r9_net_insertions" == "2439" && "$r9_net_deletions" == "303" ]] || fail "R9 net line counts drifted"
[[ "$r9_commit_file_touches" == "68" ]] || fail "R9 commit file-touch count drifted"
[[ "$r9_range_digest" == "$PINNED_R9_RANGE_DIGEST" ]] || fail "R9 range digest drifted"
[[ "$r9_path_digest" == "$PINNED_R9_PATH_DIGEST" ]] || fail "R9 path surface digest drifted"
[[ "$r9_commit_identity_digest" == "$PINNED_R9_COMMIT_IDENTITY_DIGEST" ]] || fail "R9 commit identity digest drifted"

git log --reverse --format='%H%x09%aI%x09%s' "$r9_range" >"$r9_expected_commits"
jq -r '.commit_inventory[] | [.upstream_commit,.authored_at,.title] | @tsv' "$R9_MANIFEST" >"$r9_manifest_commits"
cmp -s "$r9_expected_commits" "$r9_manifest_commits" || fail "R9 commit identity inventory does not match the complete range"
git diff --name-only "$r9_range" | LC_ALL=C sort >"$r9_expected_files"
jq -r '[.commit_inventory[].related_files[]] | unique[]' "$R9_MANIFEST" >"$r9_manifest_files"
cmp -s "$r9_expected_files" "$r9_manifest_files" || fail "R9 related path inventory does not match the complete range"

while IFS= read -r r9_commit; do
  git show --format= --name-only "$r9_commit" | sed '/^$/d' | LC_ALL=C sort -u >"$r9_commit_expected_files"
  jq -r --arg commit "$r9_commit" '.commit_inventory[] | select(.upstream_commit == $commit) | .related_files[]' "$R9_MANIFEST" | LC_ALL=C sort -u >"$r9_commit_manifest_files"
  cmp -s "$r9_commit_expected_files" "$r9_commit_manifest_files" || fail "R9 per-commit path inventory drifted for $r9_commit"
done < <(git rev-list --reverse "$r9_range")

jq -n \
  --arg schema hepta_upstream_codex_current_intake_verify_v9 \
  --arg status ready \
  --arg historical_manifest "$PINNED_R3_MANIFEST" \
  --arg r4_manifest "$R4_MANIFEST" \
  --arg r5_manifest "$R5_MANIFEST" \
  --arg r6_manifest "$R6_MANIFEST" \
  --arg r7_manifest "$R7_MANIFEST" \
  --arg r8_manifest "$R8_MANIFEST" \
  --arg manifest "$R9_MANIFEST" \
  --arg r4_ref "$R4_REF" \
  --arg r4_head "$R4_HEAD" \
  --arg r5_ref "$R5_REF" \
  --arg r5_head "$R5_HEAD" \
  --arg r6_ref "$R6_REF" \
  --arg r6_head "$R6_HEAD" \
  --arg r7_ref "$R7_REF" \
  --arg r7_head "$R7_HEAD" \
  --arg r8_ref "$R8_REF" \
  --arg r8_head "$R8_HEAD" \
  --arg r9_ref "$R9_REF" \
  --arg r9_head "$R9_HEAD" \
  --arg range_digest "$r9_range_digest" \
  --arg path_digest "$r9_path_digest" \
  --arg commit_identity_digest "$r9_commit_identity_digest" \
  --argjson commit_count "$r9_commit_count" \
  --argjson changed_file_count "$r9_changed_file_count" \
  '{schema:$schema,status:$status,role:"latest_recorded_intake",historical_manifest:$historical_manifest,r4_manifest:$r4_manifest,r5_manifest:$r5_manifest,r6_manifest:$r6_manifest,r7_manifest:$r7_manifest,r8_manifest:$r8_manifest,manifest:$manifest,r4_ref:$r4_ref,r4_head:$r4_head,r5_ref:$r5_ref,r5_head:$r5_head,r6_ref:$r6_ref,r6_head:$r6_head,r7_ref:$r7_ref,r7_head:$r7_head,r8_ref:$r8_ref,r8_head:$r8_head,r9_ref:$r9_ref,r9_head:$r9_head,commit_count:$commit_count,changed_file_count:$changed_file_count,range_digest:$range_digest,path_digest:$path_digest,commit_identity_digest:$commit_identity_digest,imported_count:0,predecessor_chain_preserved:true,network_freshness_checked:false,network_access_performed:false,workspace_product_code_modified:false,merge_performed:false,rebase_performed:false,cherry_pick_performed:false,deployment_performed:false,live_enablement_performed:false}'

echo "Hepta upstream Codex current latest-recorded intake gate passed"
