#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
BASE_HEAD="${HEPTA_UPSTREAM_CODEX_DIFF_BASE_HEAD:-108234b5ebe6941764a6b8edbb37b2aa04369f07}"
TARGET_REF="${HEPTA_UPSTREAM_CODEX_DIFF_TARGET_REF:-refs/remotes/openai-codex/main}"
PINNED_TARGET_HEAD="7d47056ea42636271ac020b86347fbbef49490aa"
TARGET_HEAD="${HEPTA_UPSTREAM_CODEX_DIFF_TARGET_HEAD:-$PINNED_TARGET_HEAD}"
EXPECTED_SELECTED_COUNT="${HEPTA_UPSTREAM_CODEX_LEGACY_COMPAT_EXPECTED_COUNT:-128}"

echo "[hepta-upstream-codex-legacy-compatibility-absorption] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_legacy_compatibility -- --nocapture

validate_sha() {
  local label="$1"
  local value="$2"
  if [[ ! "$value" =~ ^[0-9a-fA-F]{40}$ ]]; then
    echo "invalid $label: expected 40-hex git object id, got '$value'" >&2
    exit 1
  fi
}

validate_sha "HEPTA_UPSTREAM_CODEX_DIFF_BASE_HEAD" "$BASE_HEAD"
if [[ -z "${HEPTA_UPSTREAM_CODEX_DIFF_TARGET_HEAD:-}" ]]; then
  target_head_source="pinned_archive_manifest:$TARGET_REF"
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
    grep -E '(^codex-rs/(cli|tui|code-mode|terminal-detection|utils/cli)/|codex-cli|codex-tui|legacy command)' |
    json_array_from_stdin
)"
selected_count="$(jq 'length' <<<"$selected_paths_json")"

paths_match_contract="$(
  jq --argjson expected "$EXPECTED_SELECTED_COUNT" \
    'length == $expected
     and all(.[]; startswith("codex-rs/"))
     and any(.[]; startswith("codex-rs/cli/"))
     and any(.[]; startswith("codex-rs/tui/"))
     and any(.[]; startswith("codex-rs/code-mode/"))' \
    <<<"$selected_paths_json"
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg absorption "upstream-codex-legacy-compatibility-absorption-contract" \
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
    --argjson selected_paths "$selected_paths_json" \
    --argjson paths_match_contract "$paths_match_contract" \
    '{
      product:$product,
      status:(if $paths_match_contract then "ready" else "attention" end),
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
        id:"legacy-cli-tui-compatibility",
        risk:"p1_compatibility",
        selected_changed_file_count:$selected_count,
        expected_changed_file_count:$expected_selected_count,
        selected_paths_match_contract:$paths_match_contract,
        sample_paths:$selected_paths[0:40]
      },
      absorption_policy:{
        retained_as_compatibility_snapshot:true,
        requires_hepta_command_contract:true,
        active_cli_tui_promotion_allowed:false,
        active_runtime_code_wiring_allowed:false,
        active_runtime_dependency_allowed:false,
        active_runtime_auto_rebase_allowed:false,
        public_release_claim_allowed:false,
        required_next_gates:[
          "map legacy CLI/TUI deltas to explicit Hepta command contracts",
          "keep active hepta-cli cargo tree free of tracked Codex engine crates",
          "run behavior-equivalence and shadow-replay before promotion",
          "do not promote compatibility UI behavior without Hepta-native parity"
        ]
      },
      gates:{
        source_ledger_gate:"scripts/hepta-upstream-codex-diff-ledger.sh",
        absorption_gate:"scripts/hepta-upstream-codex-legacy-compatibility-absorption.sh",
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

if [[ "$paths_match_contract" != "true" ]]; then
  echo "legacy compatibility absorption contract incomplete: selected_count=$selected_count expected=$EXPECTED_SELECTED_COUNT" >&2
  exit 1
fi

echo "Hepta upstream Codex legacy compatibility absorption contract gate passed"
