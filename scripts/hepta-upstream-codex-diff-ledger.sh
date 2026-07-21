#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
BASE_HEAD="${HEPTA_UPSTREAM_CODEX_DIFF_BASE_HEAD:-108234b5ebe6941764a6b8edbb37b2aa04369f07}"
TARGET_REF="${HEPTA_UPSTREAM_CODEX_DIFF_TARGET_REF:-refs/remotes/upstream/hepta-intake-20260721-r2}"
TARGET_HEAD="${HEPTA_UPSTREAM_CODEX_DIFF_TARGET_HEAD:-}"
REQUIRE_DESCENDANT="${HEPTA_UPSTREAM_CODEX_DIFF_REQUIRE_DESCENDANT:-1}"

echo "[hepta-upstream-codex-diff-ledger] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_diff_ledger -- --nocapture

validate_sha() {
  local label="$1"
  local value="$2"
  if [[ ! "$value" =~ ^[0-9a-fA-F]{40}$ ]]; then
    echo "invalid $label: expected 40-hex git object id, got '$value'" >&2
    exit 1
  fi
}

validate_sha "HEPTA_UPSTREAM_CODEX_DIFF_BASE_HEAD" "$BASE_HEAD"

if [[ -z "$TARGET_HEAD" ]]; then
  TARGET_HEAD="$(git rev-parse --verify "${TARGET_REF}^{commit}")"
  target_head_source="$TARGET_REF"
else
  validate_sha "HEPTA_UPSTREAM_CODEX_DIFF_TARGET_HEAD" "$TARGET_HEAD"
  target_head_source="env"
fi
validate_sha "target upstream head" "$TARGET_HEAD"

git cat-file -e "${BASE_HEAD}^{commit}"
git cat-file -e "${TARGET_HEAD}^{commit}"

merge_base="$(git merge-base "$BASE_HEAD" "$TARGET_HEAD")"
if [[ "$REQUIRE_DESCENDANT" == "1" && "$merge_base" != "$BASE_HEAD" ]]; then
  echo "target head is not descended from baseline: merge_base=$merge_base base=$BASE_HEAD target=$TARGET_HEAD" >&2
  exit 1
fi

diff_range="${BASE_HEAD}..${TARGET_HEAD}"
commit_count="$(git rev-list --count "$diff_range")"
changed_paths="$(git diff --name-only "$diff_range" -- codex-rs)"
changed_file_count="$(printf '%s\n' "$changed_paths" | sed '/^$/d' | wc -l | tr -d ' ')"
codex_rs_tree_base="$(git rev-parse "${BASE_HEAD}:codex-rs")"
codex_rs_tree_target="$(git rev-parse "${TARGET_HEAD}:codex-rs")"

json_array_from_stdin() {
  jq -R -s 'split("\n") | map(select(length > 0))'
}

bucket_paths_json() {
  local pattern="$1"
  { printf '%s\n' "$changed_paths" | grep -E "$pattern" || true; } | json_array_from_stdin
}

provider_paths="$(
  bucket_paths_json '(^codex-rs/(codex-api|model-provider|login|config|network-proxy|exec|shell-command|windows-sandbox-rs|linux-sandbox|sandboxing)/|sandbox|approval|auth|credential|secret|provider)'
)"
runtime_paths="$(
  bucket_paths_json '(^codex-rs/(app-server|app-server-client|app-server-daemon|app-server-protocol|app-server-transport|core/src/(agent|context|session|state|tasks|tools|unified_exec)|codex-mcp|mcp-server|protocol|thread-store|hooks|exec-server)/|tool|mcp|session|thread)'
)"
compat_paths="$(
  bucket_paths_json '(^codex-rs/(cli|tui|code-mode|terminal-detection|utils/cli)/|codex-cli|codex-tui|legacy command)'
)"
governance_paths="$(
  bucket_paths_json '(^codex-rs/(README.md|docs/|Cargo.toml|Cargo.lock|scripts/)|README|CHANGELOG|package|release|install|npm)'
)"

buckets_json="$(
  jq -n \
    --argjson provider "$provider_paths" \
    --argjson runtime "$runtime_paths" \
    --argjson compat "$compat_paths" \
    --argjson governance "$governance_paths" \
    '[
      {
        id:"provider-credential-sandbox-security",
        risk:"p0_security",
        changed_file_count:($provider | length),
        sample_paths:($provider[0:40]),
        required_action:"classify provider/auth/sandbox/network deltas before active adapter wiring",
        promotion_gate:"security/provider adapter contract review plus active dependency isolation"
      },
      {
        id:"runtime-session-tool-mcp-appserver",
        risk:"p0_runtime",
        changed_file_count:($runtime | length),
        sample_paths:($runtime[0:40]),
        required_action:"classify runtime/session/tool/MCP/app-server deltas before promotion",
        promotion_gate:"adapter behavior-equivalence and shadow-replay gates"
      },
      {
        id:"legacy-cli-tui-compatibility",
        risk:"p1_compatibility",
        changed_file_count:($compat | length),
        sample_paths:($compat[0:40]),
        required_action:"retain CLI/TUI deltas as compatibility intake unless Hepta contracts absorb them",
        promotion_gate:"active hepta-cli cargo tree remains free of tracked Codex engine crates"
      },
      {
        id:"product-doc-release-governance",
        risk:"p2_product",
        changed_file_count:($governance | length),
        sample_paths:($governance[0:40]),
        required_action:"separate product/release deltas from runtime claims",
        promotion_gate:"release claims require governance packet, watchdog, and long soak evidence"
      }
    ]'
)"

all_buckets_populated="$(
  jq 'all(.[]; .changed_file_count > 0)' <<<"$buckets_json"
)"
populated_bucket_count="$(
  jq '[.[] | select(.changed_file_count > 0)] | length' <<<"$buckets_json"
)"

commit_sample_json="$(
  git log --pretty=format:'%H%x09%s' "$diff_range" --max-count=30 |
    jq -R -s 'split("\n") | map(select(length > 0) | capture("(?<commit>[0-9a-f]+)\t(?<subject>.*)"))'
)"
path_sample_json="$(
  printf '%s\n' "$changed_paths" | sed -n '1,80p' | json_array_from_stdin
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg ledger "upstream-codex-diff-range-ledger" \
    --arg upstream "https://github.com/openai/codex" \
    --arg manifest "$MANIFEST" \
    --arg base_head "$BASE_HEAD" \
    --arg target_head "$TARGET_HEAD" \
    --arg target_head_source "$target_head_source" \
    --arg target_ref "$TARGET_REF" \
    --arg diff_range "$diff_range" \
    --arg merge_base "$merge_base" \
    --arg codex_rs_tree_base "$codex_rs_tree_base" \
    --arg codex_rs_tree_target "$codex_rs_tree_target" \
    --argjson commit_count "$commit_count" \
    --argjson changed_file_count "$changed_file_count" \
    --argjson require_descendant "$(if [[ "$REQUIRE_DESCENDANT" == "1" ]]; then echo true; else echo false; fi)" \
    --argjson all_buckets_populated "$all_buckets_populated" \
    --argjson populated_bucket_count "$populated_bucket_count" \
    --argjson buckets "$buckets_json" \
    --argjson commit_sample "$commit_sample_json" \
    --argjson path_sample "$path_sample_json" \
    '{
      product:$product,
      status:(if ($commit_count > 0 and $changed_file_count > 0 and $populated_bucket_count > 0) then "ready" else "attention" end),
      ledger_id:$ledger,
      upstream_repository:$upstream,
      manifest:$manifest,
      baseline_upstream_head:$base_head,
      target_upstream_head:$target_head,
      target_head_source:$target_head_source,
      target_ref:$target_ref,
      candidate_diff_range:$diff_range,
      merge_base:$merge_base,
      target_descends_from_baseline:($merge_base == $base_head),
      require_descendant:$require_descendant,
      codex_rs_tree:{
        baseline:$codex_rs_tree_base,
        target:$codex_rs_tree_target
      },
      inventory:{
        commit_count:$commit_count,
        changed_file_count:$changed_file_count,
        all_buckets_populated:$all_buckets_populated,
        populated_bucket_count:$populated_bucket_count,
        narrow_delta_ready:($commit_count > 0 and $changed_file_count > 0 and $populated_bucket_count > 0),
        commit_sample:$commit_sample,
        changed_path_sample:$path_sample
      },
      buckets:$buckets,
      gates:{
        snapshot_gate:"scripts/hepta-upstream-codex-snapshot.sh",
        diff_ledger_gate:"scripts/hepta-upstream-codex-diff-ledger.sh",
        sync_lane_gate:"scripts/hepta-upstream-codex-sync-lane.sh",
        active_dependency_isolation_gate:"scripts/hepta-active-service-dependency-isolation.sh"
      },
      absorption_policy:{
        upstream_fetch_performed:false,
        upstream_merge_performed:false,
        upstream_checkout_performed:false,
        workspace_write:false,
        active_service_restart:false,
        auto_apply_allowed:false,
        active_runtime_dependency_allowed:false,
        active_runtime_auto_rebase_allowed:false,
        public_release_claim_allowed:false
      },
      side_effects:{
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

if [[ "$commit_count" -le 0 || "$changed_file_count" -le 0 || "$populated_bucket_count" -le 0 ]]; then
  echo "diff ledger is incomplete: commits=$commit_count changed_files=$changed_file_count populated_bucket_count=$populated_bucket_count all_buckets_populated=$all_buckets_populated" >&2
  exit 1
fi

echo "Hepta upstream Codex diff ledger gate passed"
