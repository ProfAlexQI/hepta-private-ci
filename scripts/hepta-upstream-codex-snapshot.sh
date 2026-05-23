#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
UPSTREAM_REMOTE="${HEPTA_UPSTREAM_CODEX_REMOTE:-https://github.com/openai/codex}"
OBSERVE_REMOTE="${HEPTA_UPSTREAM_CODEX_SNAPSHOT_OBSERVE_REMOTE:-0}"
REQUIRE_OBSERVED_HEAD="${HEPTA_UPSTREAM_CODEX_SNAPSHOT_REQUIRE_OBSERVED_HEAD:-0}"
REQUIRE_DIFF_RANGE="${HEPTA_UPSTREAM_CODEX_SNAPSHOT_REQUIRE_DIFF_RANGE:-0}"
BASE_HEAD="${HEPTA_UPSTREAM_CODEX_BASE_HEAD:-}"
TARGET_HEAD="${HEPTA_UPSTREAM_CODEX_TARGET_HEAD:-}"

echo "[hepta-upstream-codex-snapshot] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_snapshot -- --nocapture

validate_sha() {
  local label="$1"
  local value="$2"
  if [[ -n "$value" && ! "$value" =~ ^[0-9a-fA-F]{40}$ ]]; then
    echo "invalid $label: expected 40-hex git object id, got '$value'" >&2
    exit 1
  fi
}

validate_sha "HEPTA_UPSTREAM_CODEX_BASE_HEAD" "$BASE_HEAD"
validate_sha "HEPTA_UPSTREAM_CODEX_TARGET_HEAD" "$TARGET_HEAD"

target_head_source="unset"
remote_observation_performed=false
if [[ -n "$TARGET_HEAD" ]]; then
  target_head_source="env"
elif [[ "$OBSERVE_REMOTE" == "1" ]]; then
  echo "[hepta-upstream-codex-snapshot] observing upstream HEAD with git ls-remote"
  TARGET_HEAD="$(git ls-remote "$UPSTREAM_REMOTE" HEAD | awk 'NR == 1 { print $1 }')"
  validate_sha "observed upstream HEAD" "$TARGET_HEAD"
  target_head_source="git_ls_remote_head"
  remote_observation_performed=true
fi

if [[ "$REQUIRE_OBSERVED_HEAD" == "1" && -z "$TARGET_HEAD" ]]; then
  echo "observed upstream head is required but no HEPTA_UPSTREAM_CODEX_TARGET_HEAD was provided and remote observation is disabled" >&2
  exit 1
fi

if [[ "$REQUIRE_DIFF_RANGE" == "1" && ( -z "$BASE_HEAD" || -z "$TARGET_HEAD" ) ]]; then
  echo "diff range is required but HEPTA_UPSTREAM_CODEX_BASE_HEAD or target upstream head is missing" >&2
  exit 1
fi

local_repo_head="$(git rev-parse HEAD)"
local_branch="$(git branch --show-current || true)"
codex_rs_tree="$(git rev-parse HEAD:codex-rs)"
tracked_dirty_count="$(git status --porcelain --untracked-files=no | wc -l | tr -d ' ')"

if [[ -n "$BASE_HEAD" && -n "$TARGET_HEAD" ]]; then
  diff_range="${BASE_HEAD}..${TARGET_HEAD}"
else
  diff_range=""
fi

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg lane "upstream-codex-snapshot-intake" \
    --arg upstream "$UPSTREAM_REMOTE" \
    --arg manifest "$MANIFEST" \
    --arg local_head "$local_repo_head" \
    --arg local_branch "$local_branch" \
    --arg codex_tree "$codex_rs_tree" \
    --arg base_head "$BASE_HEAD" \
    --arg target_head "$TARGET_HEAD" \
    --arg target_head_source "$target_head_source" \
    --arg diff_range "$diff_range" \
    --argjson tracked_dirty_count "$tracked_dirty_count" \
    --argjson observe_remote "$(if [[ "$OBSERVE_REMOTE" == "1" ]]; then echo true; else echo false; fi)" \
    --argjson remote_observation_performed "$remote_observation_performed" \
    --argjson require_observed_head "$(if [[ "$REQUIRE_OBSERVED_HEAD" == "1" ]]; then echo true; else echo false; fi)" \
    --argjson require_diff_range "$(if [[ "$REQUIRE_DIFF_RANGE" == "1" ]]; then echo true; else echo false; fi)" \
    '{
      product:$product,
      status:"ready",
      lane_id:$lane,
      upstream_repository:$upstream,
      manifest:$manifest,
      compatibility_snapshot_role:"ingestion_and_regression_oracle",
      local_snapshot:{
        hepta_repo_head:$local_head,
        hepta_branch:$local_branch,
        codex_rs_tree:$codex_tree,
        tracked_dirty_count:$tracked_dirty_count,
        tracked_worktree_clean:($tracked_dirty_count == 0)
      },
      upstream_snapshot:{
        observe_remote_requested:$observe_remote,
        remote_observation_performed:$remote_observation_performed,
        target_head_source:$target_head_source,
        observed_upstream_head:($target_head | if . == "" then null else . end),
        base_upstream_head:($base_head | if . == "" then null else . end),
        candidate_diff_range:($diff_range | if . == "" then null else . end),
        observed_head_required:$require_observed_head,
        diff_range_required:$require_diff_range,
        diff_range_ready:($diff_range != "")
      },
      required_classification_buckets:[
        "provider_credential_sandbox_security",
        "runtime_session_tool_mcp_appserver",
        "legacy_cli_tui_compatibility",
        "product_doc_release_governance"
      ],
      absorption_policy:{
        upstream_fetch_performed:false,
        upstream_merge_performed:false,
        upstream_checkout_performed:false,
        auto_apply_allowed:false,
        active_runtime_dependency_allowed:false,
        active_runtime_auto_rebase_allowed:false,
        public_release_claim_allowed:false,
        dependency_isolation_gate:"scripts/hepta-active-service-dependency-isolation.sh"
      },
      side_effects:{
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
echo "Hepta upstream Codex snapshot gate passed"
