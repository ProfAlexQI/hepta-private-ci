#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
BASE_HEAD="${HEPTA_UPSTREAM_CODEX_DIFF_BASE_HEAD:-108234b5ebe6941764a6b8edbb37b2aa04369f07}"
TARGET_REF="${HEPTA_UPSTREAM_CODEX_DIFF_TARGET_REF:-refs/remotes/openai-codex/main}"
TARGET_HEAD="${HEPTA_UPSTREAM_CODEX_DIFF_TARGET_HEAD:-}"
EXPECTED_SELECTED_COUNT="${HEPTA_UPSTREAM_CODEX_PROVIDER_SECURITY_EXPECTED_COUNT:-104}"

echo "[hepta-upstream-codex-provider-security-absorption] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_provider_security -- --nocapture

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
    grep -E '(^codex-rs/(codex-api|model-provider|login|config|network-proxy|exec|shell-command|windows-sandbox-rs|linux-sandbox|sandboxing)/|sandbox|approval|auth|credential|secret|provider)' |
    json_array_from_stdin
)"
selected_count="$(jq 'length' <<<"$selected_paths_json")"

paths_match_contract="$(
  jq --argjson expected "$EXPECTED_SELECTED_COUNT" \
    'length == $expected
     and all(.[]; startswith("codex-rs/"))
     and any(.[]; startswith("codex-rs/codex-api/"))
     and any(.[]; startswith("codex-rs/model-provider/"))
     and any(.[]; startswith("codex-rs/login/"))
     and any(.[]; startswith("codex-rs/config/"))
     and any(.[]; startswith("codex-rs/exec/"))
     and any(.[]; startswith("codex-rs/linux-sandbox/"))
     and any(.[]; startswith("codex-rs/windows-sandbox-rs/"))
     and any(.[]; startswith("codex-rs/network-proxy/"))
     and any(.[]; startswith("codex-rs/sandboxing/"))' \
    <<<"$selected_paths_json"
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg absorption "upstream-codex-provider-security-absorption-contract" \
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
        id:"provider-credential-sandbox-security",
        risk:"p0_security",
        selected_changed_file_count:$selected_count,
        expected_changed_file_count:$expected_selected_count,
        selected_paths_match_contract:$paths_match_contract,
        required_path_families:[
          "codex-rs/codex-api",
          "codex-rs/model-provider",
          "codex-rs/login",
          "codex-rs/config",
          "codex-rs/exec",
          "codex-rs/linux-sandbox",
          "codex-rs/windows-sandbox-rs",
          "codex-rs/network-proxy",
          "codex-rs/sandboxing"
        ],
        sample_paths:$selected_paths[0:60]
      },
      absorption_policy:{
        p0_security_review_required:true,
        requires_provider_contract:true,
        requires_auth_credential_redaction:true,
        requires_sandbox_exec_replay:true,
        requires_network_policy_replay:true,
        active_provider_promotion_allowed:false,
        active_security_policy_promotion_allowed:false,
        active_runtime_code_wiring_allowed:false,
        active_runtime_dependency_allowed:false,
        active_runtime_auto_rebase_allowed:false,
        public_release_claim_allowed:false,
        required_next_gates:[
          "map provider and auth deltas to Hepta redacted provider contracts",
          "run sandbox and exec replay before policy promotion",
          "run network-proxy policy replay before any live network allowance",
          "keep active hepta-cli cargo tree free of tracked Codex engine crates",
          "require operator approval packet and long soak before release claims"
        ]
      },
      gates:{
        source_ledger_gate:"scripts/hepta-upstream-codex-diff-ledger.sh",
        absorption_gate:"scripts/hepta-upstream-codex-provider-security-absorption.sh",
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
  echo "provider/security absorption contract incomplete: selected_count=$selected_count expected=$EXPECTED_SELECTED_COUNT" >&2
  exit 1
fi

echo "Hepta upstream Codex provider/security absorption contract gate passed"
