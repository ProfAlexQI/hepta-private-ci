#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
cd "$REPO_ROOT"

PINNED_BASE_HEAD="108234b5ebe6941764a6b8edbb37b2aa04369f07"
PINNED_CUTOFF_REF="refs/remotes/upstream/hepta-intake-20260721-r2"
PINNED_CUTOFF_HEAD="88fac6fe108237a105d3203e3508b0d531054312"
PINNED_MANIFEST="docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-21_R2.json"
PINNED_PREDECESSOR_CUTOFF_REF="refs/remotes/upstream/hepta-intake-20260721"
PINNED_PREDECESSOR_CUTOFF_HEAD="45ac251e178416ff5c3022457ad8d2778c0d4549"
PINNED_PREDECESSOR_MANIFEST="docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-21.json"
PINNED_PREDECESSOR_MANIFEST_SHA256="157274d564f6e4274ad7ce50d9038670ce99b277e9ed481d879243c3404e6882"
PINNED_APPS_MCP_UPSTREAM_COMMIT="6bf4845b60e0abccd0c64690e9c7591e0efb85d8"
PINNED_APPS_MCP_LOCAL_RECEIPT="f983f4ae7fc7e4b224272990106049f30ee472d7"
PINNED_PROC_PREFLIGHT_UPSTREAM_COMMIT="44481a1c4548d1cc0cc3c95aa03b59ec4cba074a"
PINNED_PROC_PREFLIGHT_LOCAL_RECEIPT="c62ce9e2d4ee0ccaa85b50098f41198b44ae17e7"

MANIFEST="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_MANIFEST:-$PINNED_MANIFEST}"
BASE_HEAD="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_BASE_HEAD:-$PINNED_BASE_HEAD}"
CUTOFF_REF="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_CUTOFF_REF:-$PINNED_CUTOFF_REF}"
CUTOFF_HEAD="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_CUTOFF_HEAD:-$PINNED_CUTOFF_HEAD}"
APPS_MCP_LOCAL_RECEIPT="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_APPS_MCP_LOCAL_RECEIPT:-$PINNED_APPS_MCP_LOCAL_RECEIPT}"
PROC_PREFLIGHT_LOCAL_RECEIPT="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_PROC_PREFLIGHT_LOCAL_RECEIPT:-$PINNED_PROC_PREFLIGHT_LOCAL_RECEIPT}"
SKIP_RUST_TESTS="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_SKIP_RUST_TESTS:-0}"
ALLOW_FIXTURE_MANIFEST="${HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_ALLOW_FIXTURE_MANIFEST:-0}"
CODEX_MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"

fail() {
  echo "hepta upstream Codex current intake gate failed: $*" >&2
  exit 1
}

validate_sha() {
  local label="$1"
  local value="$2"
  [[ "$value" =~ ^[0-9a-f]{40}$ ]] || fail "$label must be a lowercase 40-hex commit id, got '$value'"
}

count_matching_paths() {
  local paths="$1"
  local pattern="$2"
  {
    printf '%s\n' "$paths" | grep -E "$pattern" || true
  } | sed '/^$/d' | wc -l | tr -d '[:space:]'
}

pinned_r2_deferred_pairs() {
  printf '%s\n' \
    'bd92b056ddd91bd7c2ecfea3d8773f7eb5a879a6|r2_windows_write_root_acl_integrity' \
    'e4836f998da166aba456f60d2e74eb79d6e2542b|r2_hook_context_spill_limits' \
    '8c41ed33ce3e39460e7b13b14c35e0c39bb5980d|r2_session_start_hook_ordering' \
    'e52c35b0001ea3e4a1744b99c4250a5b1a09e44d|r2_approval_rejection_reason_propagation' \
    'ec3140db1297f3acebec7d6916b329cad3b12693|r2_history_hook_api_test_alignment' \
    'b7e39aa31608b6eaba4f317538a8f82985a9e854|r2_paginated_rollout_lineage_resolution' \
    '19940967bdb5ac04aec5d08ebd465481f1ac964d|r2_threadless_mcp_connection_events' \
    '81e89fa5af13012c8313f032a17b11b9a5170d33|r2_sqlite_test_path_validation' \
    '687f05cb946d10c96f90dd7ce82e11465c6e20a7|r2_agent_job_storage_migration' \
    'cf821e8ec850c6d8380feea0e84859dd8ff54cd0|r2_hook_warning_tui_presentation' \
    '60272096bc125ad7bd8ec26508b19d1e0db2874b|r2_connector_metadata_enrichment' \
    '35c2278dd5c49daf8a4e44468038aed9be9e866e|r2_windows_exec_server_sandboxing' \
    '56c11cf6586c0579e4e3eca14eefb0916b14c78c|r2_shared_skill_model_migration' \
    'fd3c1dc13d0a0941af406e1bc1f697c9d14110ea|r2_remote_compaction_history_optimization' \
    '2be7d3bcd9d1aec2780f0a71fe79cbb5afd877a1|r2_approval_catalog_policy_compatibility' \
    'c9ef7eff005c3299a5a5f0004c34c6a3eedf2564|r2_outbound_proxy_route_resolution' \
    '88fac6fe108237a105d3203e3508b0d531054312|r2_managed_permission_proxy_resolution'
}

count_nonempty_lines() {
  sed '/^$/d' | wc -l | tr -d '[:space:]'
}

resolve_direct_commit_ref() {
  local label="$1"
  local ref="$2"
  local expected="$3"
  local symbolic_target raw object_type

  symbolic_target="$(git symbolic-ref -q "$ref" 2>/dev/null || true)"
  [[ -z "$symbolic_target" ]] || fail "$label must be a direct ref, found symref to $symbolic_target"
  raw="$(git rev-parse --verify "$ref" 2>/dev/null)" || fail "$label is missing: $ref"
  [[ "$raw" == "$expected" ]] || fail "$label raw OID drifted: expected $expected got $raw"
  object_type="$(git cat-file -t "$raw" 2>/dev/null)" || fail "$label object is unavailable: $raw"
  [[ "$object_type" == "commit" ]] || fail "$label must directly reference a commit object, found $object_type"
  printf '%s\n' "$raw"
}

if [[ "$MANIFEST" != "$PINNED_MANIFEST" ]]; then
  [[ "$ALLOW_FIXTURE_MANIFEST" == "1" && "$SKIP_RUST_TESTS" == "1" ]] || fail "manifest override is not allowed for the canonical gate: $MANIFEST"
fi
[[ -f "$MANIFEST" ]] || fail "missing intake manifest: $MANIFEST"
[[ -f "$PINNED_PREDECESSOR_MANIFEST" ]] || fail "missing predecessor intake manifest: $PINNED_PREDECESSOR_MANIFEST"
[[ "$BASE_HEAD" == "$PINNED_BASE_HEAD" ]] || fail "baseline does not match pinned baseline: expected $PINNED_BASE_HEAD got $BASE_HEAD"
[[ "$CUTOFF_REF" == "$PINNED_CUTOFF_REF" ]] || fail "cutoff ref does not match pinned ref: expected $PINNED_CUTOFF_REF got $CUTOFF_REF"
[[ "$CUTOFF_HEAD" == "$PINNED_CUTOFF_HEAD" ]] || fail "cutoff head does not match pinned cutoff: expected $PINNED_CUTOFF_HEAD got $CUTOFF_HEAD"
[[ "$APPS_MCP_LOCAL_RECEIPT" == "$PINNED_APPS_MCP_LOCAL_RECEIPT" ]] || fail "Apps MCP receipt does not match pinned receipt: expected $PINNED_APPS_MCP_LOCAL_RECEIPT got $APPS_MCP_LOCAL_RECEIPT"
[[ "$PROC_PREFLIGHT_LOCAL_RECEIPT" == "$PINNED_PROC_PREFLIGHT_LOCAL_RECEIPT" ]] || fail "proc preflight receipt does not match pinned receipt: expected $PINNED_PROC_PREFLIGHT_LOCAL_RECEIPT got $PROC_PREFLIGHT_LOCAL_RECEIPT"
validate_sha "baseline" "$BASE_HEAD"
validate_sha "cutoff" "$CUTOFF_HEAD"
validate_sha "Apps MCP upstream commit" "$PINNED_APPS_MCP_UPSTREAM_COMMIT"
validate_sha "Apps MCP local receipt" "$APPS_MCP_LOCAL_RECEIPT"
validate_sha "proc preflight upstream commit" "$PINNED_PROC_PREFLIGHT_UPSTREAM_COMMIT"
validate_sha "proc preflight local receipt" "$PROC_PREFLIGHT_LOCAL_RECEIPT"

predecessor_manifest_sha256="$(shasum -a 256 "$PINNED_PREDECESSOR_MANIFEST" | awk '{print $1}')"
[[ "$predecessor_manifest_sha256" == "$PINNED_PREDECESSOR_MANIFEST_SHA256" ]] || fail "predecessor intake manifest drifted: expected $PINNED_PREDECESSOR_MANIFEST_SHA256 got $predecessor_manifest_sha256"

jq -e \
  --arg base "$PINNED_BASE_HEAD" \
  --arg cutoff_ref "$PINNED_CUTOFF_REF" \
  --arg cutoff "$PINNED_CUTOFF_HEAD" \
  --arg predecessor_manifest "$PINNED_PREDECESSOR_MANIFEST" \
  --arg predecessor_manifest_sha256 "$PINNED_PREDECESSOR_MANIFEST_SHA256" \
  --arg predecessor_cutoff_ref "$PINNED_PREDECESSOR_CUTOFF_REF" \
  --arg predecessor_cutoff_head "$PINNED_PREDECESSOR_CUTOFF_HEAD" \
  --arg apps_mcp_upstream_commit "$PINNED_APPS_MCP_UPSTREAM_COMMIT" \
  --arg apps_mcp_local_receipt "$PINNED_APPS_MCP_LOCAL_RECEIPT" \
  --arg proc_preflight_upstream_commit "$PINNED_PROC_PREFLIGHT_UPSTREAM_COMMIT" \
  --arg proc_preflight_local_receipt "$PINNED_PROC_PREFLIGHT_LOCAL_RECEIPT" \
  '
    .schema_version == "hepta_upstream_codex_current_intake_v2"
    and .intake_id == "upstream-codex-intake-2026-07-21-r2"
    and .state_model == ["observed", "classified", "absorbed", "deferred"]
    and .predecessor_intake.manifest_path == $predecessor_manifest
    and .predecessor_intake.manifest_sha256 == $predecessor_manifest_sha256
    and .predecessor_intake.cutoff_ref == $predecessor_cutoff_ref
    and .predecessor_intake.cutoff_head == $predecessor_cutoff_head
    and .predecessor_intake.preserved == true
    and .observation.state == "observed"
    and .observation.baseline_head == $base
    and .observation.cutoff_ref == $cutoff_ref
    and .observation.cutoff_head == $cutoff
    and .observation.commit_count > 0
    and .observation.changed_file_count > 0
    and .observation.codex_rs_changed_file_count > 0
    and .observation.bucket_counts_are_non_exclusive == true
    and .observation.network_observation_performed == false
    and .observation.fetch_performed == false
    and .observation.merge_performed == false
    and .observation.rebase_performed == false
    and .classification.state == "classified"
    and .classification.policy == "selective_semantic_absorption_only"
    and .classification.bulk_merge_allowed == false
    and .classification.bulk_rebase_allowed == false
    and .classification.whole_tree_replacement_allowed == false
    and .classification.cargo_lock_replacement_allowed == false
    and .classification.selected_absorption_count == (.selected_absorptions | length)
    and .classification.deferred_decision_count == (.deferred_decisions | length)
    and (.selected_absorptions | length) == 12
    and (.selected_absorptions | all(
      .state == "absorbed"
      and (.classification | type == "string" and length > 0)
      and (.upstream_commit | test("^[0-9a-f]{40}$"))
      and (.local_receipts | type == "array" and length > 0)
      and (.local_receipts | all(test("^[0-9a-f]{40}$")))
      and (.absorption_kind == "semantic_port" or .absorption_kind == "translated_catalog" or .absorption_kind == "local_split")
    ))
    and ([.selected_absorptions[].upstream_commit] | length == (unique | length))
    and ([.selected_absorptions[] | select(
      .state == "absorbed"
      and .classification == "mcp_endpoint_ownership"
      and .upstream_commit == $apps_mcp_upstream_commit
      and .local_receipts == [$apps_mcp_local_receipt]
      and .absorption_kind == "semantic_port"
    )] | length) == 1
    and ([.selected_absorptions[] | select(
      .state == "absorbed"
      and .classification == "history_storage_efficiency"
      and .upstream_commit == $predecessor_cutoff_head
      and .local_receipts == ["31c6065061185de711aa36ee6e9cf7c4a4795821"]
      and .absorption_kind == "semantic_port"
    )] | length) == 1
    and ([.selected_absorptions[] | select(
      .state == "absorbed"
      and .classification == "linux_proc_preflight_filesystem_isolation"
      and .upstream_commit == $proc_preflight_upstream_commit
      and .local_receipts == [$proc_preflight_local_receipt]
      and .absorption_kind == "semantic_port"
    )] | length) == 1
    and (.deferred_decisions | all(
      .state == "deferred"
      and (.classification | type == "string" and length > 0)
      and (.reason | type == "string" and length > 0)
      and (.upstream_commit == null or (.upstream_commit | test("^[0-9a-f]{40}$")))
    ))
    and ([.deferred_decisions[] | select(
      .classification == "mcp_endpoint_ownership"
      or .upstream_commit == $apps_mcp_upstream_commit
    )] | length) == 0
    and .historical_absorption_receipt.state == "historical_receipt"
    and .historical_absorption_receipt.current_intake_freshness_proof == false
    and .boundaries.offline_only == true
    and .boundaries.local_ref_must_match_cutoff == true
    and .boundaries.active_runtime_auto_rebase_allowed == false
    and .boundaries.active_runtime_dependency_allowed == false
    and .boundaries.active_service_restart_allowed == false
    and .boundaries.public_release_claim_allowed == false
  ' "$MANIFEST" >/dev/null || fail "manifest schema or policy boundary is invalid"

if [[ "$SKIP_RUST_TESTS" != "1" ]]; then
  echo "[hepta-upstream-codex-current-intake] report contract tests"
  cargo test --offline --manifest-path "$CODEX_MANIFEST" -q -p hepta-core \
    upstream_codex_current_intake -- --nocapture
fi

resolved_base="$(git rev-parse --verify "$BASE_HEAD")" || fail "pinned baseline commit is unavailable"
resolved_ref="$(resolve_direct_commit_ref "pinned local cutoff ref" "$CUTOFF_REF" "$CUTOFF_HEAD")"
resolved_cutoff="$(git rev-parse --verify "$CUTOFF_HEAD")" || fail "pinned cutoff commit is unavailable"
resolved_predecessor_ref="$(resolve_direct_commit_ref "predecessor cutoff ref" "$PINNED_PREDECESSOR_CUTOFF_REF" "$PINNED_PREDECESSOR_CUTOFF_HEAD")"
[[ "$resolved_base" == "$BASE_HEAD" ]] || fail "baseline object resolves to $resolved_base instead of $BASE_HEAD"
[[ "$(git cat-file -t "$resolved_base")" == "commit" ]] || fail "baseline must be a commit object"
[[ "$resolved_cutoff" == "$CUTOFF_HEAD" ]] || fail "cutoff object resolves to $resolved_cutoff instead of $CUTOFF_HEAD"
[[ "$(git cat-file -t "$resolved_cutoff")" == "commit" ]] || fail "cutoff must be a commit object"
git merge-base --is-ancestor "$BASE_HEAD" "$CUTOFF_HEAD" || fail "cutoff is not descended from the pinned baseline"
git merge-base --is-ancestor "$PINNED_PREDECESSOR_CUTOFF_HEAD" "$CUTOFF_HEAD" || fail "r2 cutoff does not descend from the preserved predecessor cutoff"

diff_range="${BASE_HEAD}..${CUTOFF_HEAD}"
commit_count="$(git rev-list --count "$diff_range")"
changed_paths="$(git diff --name-only "$diff_range")"
codex_rs_changed_paths="$(git diff --name-only "$diff_range" -- codex-rs)"
changed_file_count="$(printf '%s\n' "$changed_paths" | sed '/^$/d' | wc -l | tr -d '[:space:]')"
codex_rs_changed_file_count="$(printf '%s\n' "$codex_rs_changed_paths" | sed '/^$/d' | wc -l | tr -d '[:space:]')"
provider_count="$(count_matching_paths "$codex_rs_changed_paths" '(^codex-rs/(codex-api|model-provider|login|config|network-proxy|exec|shell-command|windows-sandbox-rs|linux-sandbox|sandboxing)/|sandbox|approval|auth|credential|secret|provider)')"
runtime_count="$(count_matching_paths "$codex_rs_changed_paths" '(^codex-rs/(app-server|app-server-client|app-server-daemon|app-server-protocol|app-server-transport|core/src/(agent|context|session|state|tasks|tools|unified_exec)|codex-mcp|mcp-server|protocol|thread-store|hooks|exec-server)/|tool|mcp|session|thread)')"
compat_count="$(count_matching_paths "$codex_rs_changed_paths" '(^codex-rs/(cli|tui|code-mode|terminal-detection|utils/cli)/|codex-cli|codex-tui|legacy command)')"
governance_count="$(count_matching_paths "$codex_rs_changed_paths" '(^codex-rs/(README.md|docs/|Cargo.toml|Cargo.lock|scripts/)|README|CHANGELOG|package|release|install|npm)')"

expected_commit_count="$(jq -r '.observation.commit_count' "$MANIFEST")"
expected_changed_file_count="$(jq -r '.observation.changed_file_count' "$MANIFEST")"
expected_codex_rs_changed_file_count="$(jq -r '.observation.codex_rs_changed_file_count' "$MANIFEST")"
expected_provider_count="$(jq -r '.observation.bucket_observations.provider_credential_sandbox_security' "$MANIFEST")"
expected_runtime_count="$(jq -r '.observation.bucket_observations.runtime_session_tool_mcp_appserver' "$MANIFEST")"
expected_compat_count="$(jq -r '.observation.bucket_observations.legacy_cli_tui_compatibility' "$MANIFEST")"
expected_governance_count="$(jq -r '.observation.bucket_observations.product_doc_release_governance' "$MANIFEST")"

[[ "$commit_count" == "$expected_commit_count" ]] || fail "commit inventory drifted: expected $expected_commit_count got $commit_count"
[[ "$changed_file_count" == "$expected_changed_file_count" ]] || fail "changed-file inventory drifted: expected $expected_changed_file_count got $changed_file_count"
[[ "$codex_rs_changed_file_count" == "$expected_codex_rs_changed_file_count" ]] || fail "codex-rs inventory drifted: expected $expected_codex_rs_changed_file_count got $codex_rs_changed_file_count"
[[ "$provider_count" == "$expected_provider_count" ]] || fail "provider/security bucket drifted: expected $expected_provider_count got $provider_count"
[[ "$runtime_count" == "$expected_runtime_count" ]] || fail "runtime/app-server bucket drifted: expected $expected_runtime_count got $runtime_count"
[[ "$compat_count" == "$expected_compat_count" ]] || fail "CLI/TUI compatibility bucket drifted: expected $expected_compat_count got $compat_count"
[[ "$governance_count" == "$expected_governance_count" ]] || fail "product/governance bucket drifted: expected $expected_governance_count got $governance_count"

r2_observed_commits="$(git rev-list "${PINNED_PREDECESSOR_CUTOFF_HEAD}..${CUTOFF_HEAD}" | LC_ALL=C sort)"
r2_observed_commits_json="$(printf '%s\n' "$r2_observed_commits" | jq -Rsc 'split("\n") | map(select(length > 0))')"
all_selected_commits="$(jq -r '.selected_absorptions[].upstream_commit' "$MANIFEST" | LC_ALL=C sort)"
all_unique_selected_commits="$(printf '%s\n' "$all_selected_commits" | sed '/^$/d' | LC_ALL=C sort -u)"
all_deferred_commits="$(
  jq -r '.deferred_decisions[] | select(.upstream_commit != null) | .upstream_commit' "$MANIFEST" \
    | LC_ALL=C sort
)"
all_unique_deferred_commits="$(printf '%s\n' "$all_deferred_commits" | sed '/^$/d' | LC_ALL=C sort -u)"
all_deferred_commit_count="$(printf '%s\n' "$all_deferred_commits" | count_nonempty_lines)"
all_unique_deferred_commit_count="$(printf '%s\n' "$all_unique_deferred_commits" | count_nonempty_lines)"
selected_deferred_overlap="$(
  comm -12 \
    <(printf '%s\n' "$all_unique_selected_commits" | sed '/^$/d') \
    <(printf '%s\n' "$all_unique_deferred_commits" | sed '/^$/d')
)"

[[ "$all_deferred_commit_count" == "$all_unique_deferred_commit_count" ]] || fail "deferred upstream commit list contains duplicate SHAs"
[[ -z "$selected_deferred_overlap" ]] || fail "selected and deferred upstream commit sets overlap: $selected_deferred_overlap"

r2_selected_commits="$(
  comm -12 \
    <(printf '%s\n' "$r2_observed_commits" | sed '/^$/d') \
    <(printf '%s\n' "$all_unique_selected_commits" | sed '/^$/d')
)"
r2_deferred_commits="$(
  comm -12 \
    <(printf '%s\n' "$r2_observed_commits" | sed '/^$/d') \
    <(printf '%s\n' "$all_unique_deferred_commits" | sed '/^$/d')
)"
r2_expected_deferred_commits="$(
  comm -23 \
    <(printf '%s\n' "$r2_observed_commits" | sed '/^$/d') \
    <(printf '%s\n' "$r2_selected_commits" | sed '/^$/d')
)"
r2_expected_deferred_pairs="$(pinned_r2_deferred_pairs | LC_ALL=C sort)"
r2_actual_deferred_pairs="$(
  jq -r --argjson observed "$r2_observed_commits_json" '
    .deferred_decisions[] as $decision
    | select(
        $decision.upstream_commit != null
        and ($observed | index($decision.upstream_commit)) != null
      )
    | "\($decision.upstream_commit)|\($decision.classification)"
  ' "$MANIFEST" | LC_ALL=C sort
)"
r2_named_deferred_pairs="$(
  jq -r '
    .deferred_decisions[]
    | select(.classification | startswith("r2_"))
    | "\(.upstream_commit)|\(.classification)"
  ' "$MANIFEST" | LC_ALL=C sort
)"

r2_observed_commit_count="$(printf '%s\n' "$r2_observed_commits" | count_nonempty_lines)"
r2_selected_commit_count="$(printf '%s\n' "$r2_selected_commits" | count_nonempty_lines)"
r2_deferred_commit_count="$(printf '%s\n' "$r2_deferred_commits" | count_nonempty_lines)"

[[ "$r2_observed_commit_count" == "18" ]] || fail "r2 observed delta count drifted: expected 18 got $r2_observed_commit_count"
[[ "$r2_selected_commit_count" == "1" && "$r2_selected_commits" == "$PINNED_PROC_PREFLIGHT_UPSTREAM_COMMIT" ]] || fail "r2 selected commit set must contain only $PINNED_PROC_PREFLIGHT_UPSTREAM_COMMIT"
[[ "$r2_deferred_commits" == "$r2_expected_deferred_commits" ]] || fail "r2 deferred commit set does not equal observed delta minus selected"
[[ "$r2_actual_deferred_pairs" == "$r2_expected_deferred_pairs" ]] || fail "r2 deferred classification mapping drifted"
[[ "$r2_named_deferred_pairs" == "$r2_expected_deferred_pairs" ]] || fail "r2 deferred classification set is not closed"

while IFS= read -r decision; do
  upstream_commit="$(jq -r '.upstream_commit' <<<"$decision")"
  git cat-file -e "${upstream_commit}^{commit}" || fail "selected upstream commit is unavailable: $upstream_commit"
  git merge-base --is-ancestor "$BASE_HEAD" "$upstream_commit" || fail "selected upstream commit predates or diverges from baseline: $upstream_commit"
  git merge-base --is-ancestor "$upstream_commit" "$CUTOFF_HEAD" || fail "selected upstream commit is outside cutoff: $upstream_commit"

  while IFS= read -r local_receipt; do
    git cat-file -e "${local_receipt}^{commit}" || fail "local absorption receipt is unavailable: $local_receipt"
    git merge-base --is-ancestor "$local_receipt" HEAD || fail "local absorption receipt is not in the current Hepta history: $local_receipt"
  done < <(jq -r '.local_receipts[]' <<<"$decision")
done < <(jq -c '.selected_absorptions[]' "$MANIFEST")

while IFS= read -r upstream_commit; do
  [[ "$upstream_commit" == "null" ]] && continue
  git cat-file -e "${upstream_commit}^{commit}" || fail "deferred upstream commit is unavailable: $upstream_commit"
  git merge-base --is-ancestor "$BASE_HEAD" "$upstream_commit" || fail "deferred upstream commit predates or diverges from baseline: $upstream_commit"
  git merge-base --is-ancestor "$upstream_commit" "$CUTOFF_HEAD" || fail "deferred upstream commit is outside cutoff: $upstream_commit"
done < <(jq -r '.deferred_decisions[].upstream_commit' "$MANIFEST")

manifest_sha256="$(shasum -a 256 "$MANIFEST" | awk '{print $1}')"
selected_absorption_count="$(jq '.selected_absorptions | length' "$MANIFEST")"
deferred_decision_count="$(jq '.deferred_decisions | length' "$MANIFEST")"

jq -n \
  --arg product "Hepta" \
  --arg gate "hepta_upstream_codex_current_intake" \
  --arg manifest "$MANIFEST" \
  --arg manifest_sha256 "$manifest_sha256" \
  --arg baseline_head "$BASE_HEAD" \
  --arg cutoff_ref "$CUTOFF_REF" \
  --arg cutoff_head "$CUTOFF_HEAD" \
  --arg diff_range "$diff_range" \
  --arg predecessor_manifest "$PINNED_PREDECESSOR_MANIFEST" \
  --arg predecessor_manifest_sha256 "$PINNED_PREDECESSOR_MANIFEST_SHA256" \
  --arg predecessor_cutoff_ref "$PINNED_PREDECESSOR_CUTOFF_REF" \
  --arg predecessor_cutoff_head "$PINNED_PREDECESSOR_CUTOFF_HEAD" \
  --argjson commit_count "$commit_count" \
  --argjson changed_file_count "$changed_file_count" \
  --argjson codex_rs_changed_file_count "$codex_rs_changed_file_count" \
  --argjson provider_count "$provider_count" \
  --argjson runtime_count "$runtime_count" \
  --argjson compat_count "$compat_count" \
  --argjson governance_count "$governance_count" \
  --argjson selected_absorption_count "$selected_absorption_count" \
  --argjson deferred_decision_count "$deferred_decision_count" \
  --argjson r2_observed_commit_count "$r2_observed_commit_count" \
  --argjson r2_selected_commit_count "$r2_selected_commit_count" \
  --argjson r2_deferred_commit_count "$r2_deferred_commit_count" \
  '{
    product:$product,
    status:"ready",
    gate:$gate,
    manifest:$manifest,
    manifest_sha256:$manifest_sha256,
    states:{observation:"observed", classification:"classified", selected:"absorbed", remainder:"deferred"},
    baseline_head:$baseline_head,
    cutoff_ref:$cutoff_ref,
    cutoff_head:$cutoff_head,
    diff_range:$diff_range,
    predecessor_intake:{
      manifest:$predecessor_manifest,
      manifest_sha256:$predecessor_manifest_sha256,
      cutoff_ref:$predecessor_cutoff_ref,
      cutoff_head:$predecessor_cutoff_head,
      preserved:true
    },
    inventory:{
      commit_count:$commit_count,
      changed_file_count:$changed_file_count,
      codex_rs_changed_file_count:$codex_rs_changed_file_count,
      bucket_counts_are_non_exclusive:true,
      provider_credential_sandbox_security:$provider_count,
      runtime_session_tool_mcp_appserver:$runtime_count,
      legacy_cli_tui_compatibility:$compat_count,
      product_doc_release_governance:$governance_count
    },
    decisions:{
      selected_absorption_count:$selected_absorption_count,
      deferred_decision_count:$deferred_decision_count,
      r2_delta:{
        observed_commit_count:$r2_observed_commit_count,
        selected_commit_count:$r2_selected_commit_count,
        deferred_commit_count:$r2_deferred_commit_count,
        observed_minus_selected_equals_deferred:true,
        duplicate_deferred_commits:false,
        selected_deferred_overlap:false,
        classification_mapping_pinned:true
      },
      full_range_absorption_claimed:false
    },
    policy:{
      offline_only:true,
      local_ref_matches_cutoff:true,
      upstream_fetch_performed:false,
      upstream_merge_performed:false,
      upstream_rebase_performed:false,
      whole_tree_replacement_performed:false,
      cargo_lock_replacement_performed:false,
      active_service_restart_performed:false,
      public_release_claimed:false
    }
  }'

echo "Hepta upstream Codex current intake gate passed"
