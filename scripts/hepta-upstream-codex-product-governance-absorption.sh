#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
BASE_HEAD="${HEPTA_UPSTREAM_CODEX_DIFF_BASE_HEAD:-108234b5ebe6941764a6b8edbb37b2aa04369f07}"
TARGET_REF="${HEPTA_UPSTREAM_CODEX_DIFF_TARGET_REF:-refs/remotes/openai-codex/main}"
TARGET_HEAD="${HEPTA_UPSTREAM_CODEX_DIFF_TARGET_HEAD:-}"
EXPECTED_SELECTED_COUNT="${HEPTA_UPSTREAM_CODEX_PRODUCT_GOVERNANCE_EXPECTED_COUNT:-22}"

echo "[hepta-upstream-codex-product-governance-absorption] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_product_governance -- --nocapture

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

diff_range="${BASE_HEAD}..${TARGET_HEAD}"
merge_base="$(git merge-base "$BASE_HEAD" "$TARGET_HEAD")"
if [[ "$merge_base" != "$BASE_HEAD" ]]; then
  echo "target head is not descended from baseline: merge_base=$merge_base base=$BASE_HEAD target=$TARGET_HEAD" >&2
  exit 1
fi

json_array_from_stdin() {
  jq -R -s 'split("\n") | map(select(length > 0))'
}

selected_paths_json="$(
  git diff --name-only "$diff_range" -- codex-rs |
    grep -E '(^codex-rs/(README.md|docs/|Cargo.toml|Cargo.lock|scripts/)|README|CHANGELOG|package|release|install|npm)' |
    json_array_from_stdin
)"
selected_count="$(jq 'length' <<<"$selected_paths_json")"

commit_sample_json="$(
  git log --pretty=format:'%H%x09%s' "$diff_range" -- \
    codex-rs/README.md \
    codex-rs/docs \
    codex-rs/Cargo.toml \
    codex-rs/Cargo.lock \
    codex-rs/app-server/README.md \
    codex-rs/exec-server/README.md \
    codex-rs/linux-sandbox/README.md \
    codex-rs/network-proxy/README.md \
    codex-rs/tools/README.md \
    codex-rs/install-context \
    codex-rs/skills/src/assets/samples/plugin-creator/references/installing-and-updating.md \
    codex-rs/core/tests/suite/request_plugin_install.rs \
    codex-rs/core/src/tools/handlers/request_plugin_install.rs \
    codex-rs/core/src/tools/handlers/request_plugin_install_spec.rs \
    codex-rs/core/src/tools/handlers/list_available_plugins_to_install.rs \
    codex-rs/core/src/tools/handlers/list_available_plugins_to_install_spec.rs \
    codex-rs/core-plugins/src/remote/remote_installed_plugin_sync.rs |
    jq -R -s 'split("\n") | map(select(length > 0) | capture("(?<commit>[0-9a-f]+)\t(?<subject>.*)"))'
)"
commit_sample_count="$(jq 'length' <<<"$commit_sample_json")"

selected_paths_match_contract="$(
  jq --argjson expected "$EXPECTED_SELECTED_COUNT" \
    'length == $expected
     and all(.[]; startswith("codex-rs/"))
     and any(.[]; . == "codex-rs/README.md")
     and any(.[]; . == "codex-rs/Cargo.lock")
     and any(.[]; contains("request_plugin_install"))' \
    <<<"$selected_paths_json"
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg absorption "upstream-codex-product-governance-absorption-contract" \
    --arg upstream "https://github.com/openai/codex" \
    --arg manifest "$MANIFEST" \
    --arg base_head "$BASE_HEAD" \
    --arg target_head "$TARGET_HEAD" \
    --arg target_head_source "$target_head_source" \
    --arg target_ref "$TARGET_REF" \
    --arg diff_range "$diff_range" \
    --arg merge_base "$merge_base" \
    --argjson selected_count "$selected_count" \
    --argjson expected_selected_count "$EXPECTED_SELECTED_COUNT" \
    --argjson commit_sample_count "$commit_sample_count" \
    --argjson selected_paths "$selected_paths_json" \
    --argjson commit_sample "$commit_sample_json" \
    --argjson selected_paths_match_contract "$selected_paths_match_contract" \
    '{
      product:$product,
      status:(if ($selected_paths_match_contract and $commit_sample_count > 0) then "ready" else "attention" end),
      absorption_id:$absorption,
      upstream_repository:$upstream,
      manifest:$manifest,
      baseline_upstream_head:$base_head,
      target_upstream_head:$target_head,
      target_head_source:$target_head_source,
      target_ref:$target_ref,
      candidate_diff_range:$diff_range,
      merge_base:$merge_base,
      selected_bucket:{
        id:"product-doc-release-governance",
        risk:"p2_product",
        selected_as_first_absorption_contract:true,
        selected_changed_file_count:$selected_count,
        expected_changed_file_count:$expected_selected_count,
        selected_paths_match_contract:$selected_paths_match_contract,
        commit_sample_count:$commit_sample_count,
        selected_paths:$selected_paths,
        commit_sample:$commit_sample
      },
      absorption_policy:{
        requires_hepta_translation:true,
        raw_upstream_doc_copy_allowed:false,
        raw_upstream_package_policy_copy_allowed:false,
        active_runtime_code_wiring_allowed:false,
        active_runtime_dependency_allowed:false,
        active_runtime_auto_rebase_allowed:false,
        public_release_claim_allowed:false,
        required_next_gates:[
          "translate upstream product/docs/package deltas into Hepta release-governance wording",
          "keep active dependency isolation green",
          "run clean preflight before any absorption patch",
          "require watchdog and long soak before release claims"
        ]
      },
      gates:{
        source_ledger_gate:"scripts/hepta-upstream-codex-diff-ledger.sh",
        absorption_gate:"scripts/hepta-upstream-codex-product-governance-absorption.sh",
        active_dependency_isolation_gate:"scripts/hepta-active-service-dependency-isolation.sh"
      },
      side_effects:{
        upstream_fetch_performed:false,
        upstream_merge_performed:false,
        upstream_checkout_performed:false,
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

if [[ "$selected_paths_match_contract" != "true" || "$commit_sample_count" -le 0 ]]; then
  echo "product-governance absorption contract incomplete: selected_count=$selected_count expected=$EXPECTED_SELECTED_COUNT commit_sample_count=$commit_sample_count" >&2
  exit 1
fi

echo "Hepta upstream Codex product governance absorption contract gate passed"
