#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
BASE_HEAD="${HEPTA_UPSTREAM_CODEX_LATEST_MULTISURFACE_BASE_HEAD:-9f42c89c0112771dc29100a6f3fc904049b2655f}"
TARGET_REF="${HEPTA_UPSTREAM_CODEX_LATEST_MULTISURFACE_TARGET_REF:-refs/remotes/openai-codex/latest}"
TARGET_HEAD="${HEPTA_UPSTREAM_CODEX_LATEST_MULTISURFACE_TARGET_HEAD:-}"
EXPECTED_COMMIT_COUNT="${HEPTA_UPSTREAM_CODEX_LATEST_MULTISURFACE_EXPECTED_COMMITS:-12}"
EXPECTED_CHANGED_FILE_COUNT="${HEPTA_UPSTREAM_CODEX_LATEST_MULTISURFACE_EXPECTED_CHANGED_FILES:-57}"
EXPECTED_RUNTIME_COUNT="${HEPTA_UPSTREAM_CODEX_LATEST_MULTISURFACE_EXPECTED_RUNTIME:-11}"
EXPECTED_COMPAT_COUNT="${HEPTA_UPSTREAM_CODEX_LATEST_MULTISURFACE_EXPECTED_COMPAT:-47}"
EXPECTED_GOVERNANCE_COUNT="${HEPTA_UPSTREAM_CODEX_LATEST_MULTISURFACE_EXPECTED_GOVERNANCE:-2}"
EXPECTED_PROVIDER_COUNT="${HEPTA_UPSTREAM_CODEX_LATEST_MULTISURFACE_EXPECTED_PROVIDER:-0}"

validate_sha() {
  local label="$1"
  local value="$2"
  if [[ ! "$value" =~ ^[0-9a-fA-F]{40}$ ]]; then
    echo "invalid $label: expected 40-hex git object id, got '$value'" >&2
    exit 1
  fi
}

extract_first_json_object() {
  awk '
    BEGIN {
      capture = 0
      depth = 0
    }
    {
      if (!capture && $0 ~ /^[[:space:]]*\{[[:space:]]*$/) {
        capture = 1
      }
      if (capture) {
        print
        line = $0
        open_line = line
        close_line = line
        opens = gsub(/\{/, "", open_line)
        closes = gsub(/\}/, "", close_line)
        depth += opens - closes
        if (depth == 0) {
          exit
        }
      }
    }
  '
}

capture_json_report() {
  local command_name="$1"
  shift

  local output
  output="$("$@")"
  local report
  report="$(printf '%s\n' "$output" | extract_first_json_object)"

  if ! jq -e . >/dev/null <<<"$report"; then
    echo "$command_name did not emit a parseable JSON report" >&2
    exit 1
  fi

  printf '%s\n' "$report"
}

json_array_from_stdin() {
  jq -R -s 'split("\n") | map(select(length > 0))'
}

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

validate_sha "HEPTA_UPSTREAM_CODEX_LATEST_MULTISURFACE_BASE_HEAD" "$BASE_HEAD"
if [[ -z "$TARGET_HEAD" ]]; then
  TARGET_HEAD="$(git rev-parse --verify "${TARGET_REF}^{commit}")"
  target_head_source="$TARGET_REF"
else
  validate_sha "HEPTA_UPSTREAM_CODEX_LATEST_MULTISURFACE_TARGET_HEAD" "$TARGET_HEAD"
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

LEDGER_JSON="$(
  capture_json_report \
    "hepta-upstream-codex-diff-ledger latest multisurface" \
    env HEPTA_UPSTREAM_CODEX_DIFF_BASE_HEAD="$BASE_HEAD" \
      HEPTA_UPSTREAM_CODEX_DIFF_TARGET_REF="$TARGET_REF" \
      HEPTA_UPSTREAM_CODEX_DIFF_TARGET_HEAD="$TARGET_HEAD" \
      scripts/hepta-upstream-codex-diff-ledger.sh
)"

changed_paths="$(git diff --name-only "$diff_range" -- codex-rs)"
changed_paths_json="$(printf '%s\n' "$changed_paths" | json_array_from_stdin)"
commit_sample_json="$(
  git log --pretty=format:'%H%x09%s' "$diff_range" --max-count=20 |
    jq -R -s 'split("\n") | map(select(length > 0) | capture("(?<commit>[0-9a-f]+)\t(?<subject>.*)"))'
)"
commit_count="$(git rev-list --count "$diff_range")"
changed_file_count="$(jq 'length' <<<"$changed_paths_json")"
ledger_report_sha256="$(sha256_text "$LEDGER_JSON")"
delta_index_hash_sha256="$(sha256_text "hepta-upstream-codex-latest-multisurface:index:$BASE_HEAD:$TARGET_HEAD:$ledger_report_sha256")"
delta_policy_hash_sha256="$(sha256_text "hepta-upstream-codex-latest-multisurface:policy:$BASE_HEAD:$TARGET_HEAD:$ledger_report_sha256")"
delta_side_effect_hash_sha256="$(sha256_text "hepta-upstream-codex-latest-multisurface:side-effects:$BASE_HEAD:$TARGET_HEAD:$ledger_report_sha256")"

jq -n -e \
  --argjson ledger "$LEDGER_JSON" \
  --argjson expected_commit_count "$EXPECTED_COMMIT_COUNT" \
  --argjson expected_changed_file_count "$EXPECTED_CHANGED_FILE_COUNT" \
  --argjson expected_runtime_count "$EXPECTED_RUNTIME_COUNT" \
  --argjson expected_compat_count "$EXPECTED_COMPAT_COUNT" \
  --argjson expected_governance_count "$EXPECTED_GOVERNANCE_COUNT" \
  --argjson expected_provider_count "$EXPECTED_PROVIDER_COUNT" \
  '
    $ledger.status == "ready"
    and $ledger.inventory.commit_count == $expected_commit_count
    and $ledger.inventory.changed_file_count == $expected_changed_file_count
    and $ledger.inventory.populated_bucket_count == 3
    and $ledger.inventory.all_buckets_populated == false
    and ($ledger.buckets[] | select(.id == "provider-credential-sandbox-security").changed_file_count) == $expected_provider_count
    and ($ledger.buckets[] | select(.id == "runtime-session-tool-mcp-appserver").changed_file_count) == $expected_runtime_count
    and ($ledger.buckets[] | select(.id == "legacy-cli-tui-compatibility").changed_file_count) == $expected_compat_count
    and ($ledger.buckets[] | select(.id == "product-doc-release-governance").changed_file_count) == $expected_governance_count
    and $ledger.absorption_policy.upstream_merge_performed == false
    and $ledger.absorption_policy.active_runtime_dependency_allowed == false
    and $ledger.absorption_policy.active_runtime_auto_rebase_allowed == false
    and ($ledger.side_effects | to_entries | all(.value == false))
  ' >/dev/null

family_inventory_json="$(
  jq -n \
    --argjson paths "$changed_paths_json" \
    '[
      {
        id:"doctor-thread-inventory-audit",
        risk:"p0_runtime_observability",
        required_paths:[
          "codex-rs/cli/src/doctor/thread_inventory.rs",
          "codex-rs/state/src/audit.rs",
          "codex-rs/state/src/lib.rs",
          "codex-rs/cli/src/doctor/background.rs",
          "codex-rs/cli/src/doctor/output.rs"
        ],
        required_action:"translate as bounded diagnostic inventory before any active Hepta runtime query",
        promotion_allowed:false
      },
      {
        id:"appserver-remote-status",
        risk:"p0_runtime_status",
        required_paths:[
          "codex-rs/app-server-client/src/lib.rs",
          "codex-rs/app-server-client/src/remote.rs",
          "codex-rs/app-server-daemon/src/lib.rs",
          "codex-rs/tui/src/status/remote_connection.rs"
        ],
        required_action:"classify remote connection details as display-only status, not Gateway mutation",
        promotion_allowed:false
      },
      {
        id:"tui-markdown-status-stderr",
        risk:"p1_compatibility",
        required_paths:[
          "codex-rs/tui/src/markdown_render.rs",
          "codex-rs/tui/src/tui/terminal_stderr.rs",
          "codex-rs/tui/src/status/card.rs",
          "codex-rs/tui/src/status/mod.rs"
        ],
        required_action:"retain as legacy TUI compatibility intake unless Hepta UI contracts absorb it",
        promotion_allowed:false
      },
      {
        id:"tui-config-trust-cleanup",
        risk:"p1_compatibility",
        required_paths:[
          "codex-rs/tui/src/config_update.rs",
          "codex-rs/tui/src/onboarding/trust_directory.rs",
          "codex-rs/tui/src/startup_hooks_review.rs",
          "codex-rs/tui/src/oss_selection.rs"
        ],
        required_action:"map trust and config cleanup to Hepta policy gates before active startup changes",
        promotion_allowed:false
      },
      {
        id:"process-hardening-macos-malloc-diagnostics",
        risk:"p2_product_governance",
        required_paths:[
          "codex-rs/process-hardening/src/lib.rs",
          "codex-rs/process-hardening/README.md"
        ],
        required_action:"preserve as process-hardening signal without mutating active launchd env",
        promotion_allowed:false
      }
    ]
    | map(. + {
        matched_path_count:(.required_paths | map(select(. as $path | $paths | index($path))) | length),
        ready:(.required_paths | all(. as $path | $paths | index($path)))
      })'
)"

jq -e 'all(.[]; .ready == true and .promotion_allowed == false)' <<<"$family_inventory_json" >/dev/null

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg gate "hepta_upstream_codex_latest_multisurface_absorption_gate" \
    --arg upstream "https://github.com/openai/codex" \
    --arg manifest "$MANIFEST" \
    --arg base_head "$BASE_HEAD" \
    --arg target_head "$TARGET_HEAD" \
    --arg target_head_source "$target_head_source" \
    --arg target_ref "$TARGET_REF" \
    --arg diff_range "$diff_range" \
    --arg merge_base "$merge_base" \
    --arg ledger_report_sha256 "$ledger_report_sha256" \
    --arg delta_index_hash_sha256 "$delta_index_hash_sha256" \
    --arg delta_policy_hash_sha256 "$delta_policy_hash_sha256" \
    --arg delta_side_effect_hash_sha256 "$delta_side_effect_hash_sha256" \
    --argjson commit_count "$commit_count" \
    --argjson changed_file_count "$changed_file_count" \
    --argjson expected_commit_count "$EXPECTED_COMMIT_COUNT" \
    --argjson expected_changed_file_count "$EXPECTED_CHANGED_FILE_COUNT" \
    --argjson expected_runtime_count "$EXPECTED_RUNTIME_COUNT" \
    --argjson expected_compat_count "$EXPECTED_COMPAT_COUNT" \
    --argjson expected_governance_count "$EXPECTED_GOVERNANCE_COUNT" \
    --argjson expected_provider_count "$EXPECTED_PROVIDER_COUNT" \
    --argjson ledger "$LEDGER_JSON" \
    --argjson commit_sample "$commit_sample_json" \
    --argjson family_inventory "$family_inventory_json" \
    '
      ([
        "latest_delta_direct_merge_denied",
        "latest_delta_active_runtime_auto_rebase_denied",
        "latest_delta_active_dependency_mutation_denied",
        "latest_delta_gateway_mutation_denied",
        "latest_delta_doctor_thread_inventory_live_query_denied",
        "latest_delta_remote_status_active_wiring_denied",
        "latest_delta_tui_compatibility_promotion_denied",
        "latest_delta_process_hardening_launchd_env_mutation_denied",
        "latest_delta_provider_model_invocation_denied",
        "latest_delta_channel_delivery_denied",
        "latest_delta_public_claim_denied",
        "latest_delta_release_artifact_write_denied",
        "latest_delta_evidence_persistence_denied"
      ]) as $denied
      | {
        product:$product,
        runtime:$runtime,
        status:"ready",
        gate:$gate,
        upstream_repository:$upstream,
        manifest:$manifest,
        latest_multisurface_schema_version:"latest_multisurface_delta_absorption_v1",
        baseline_upstream_head:$base_head,
        target_upstream_head:$target_head,
        target_head_source:$target_head_source,
        target_ref:$target_ref,
        candidate_diff_range:$diff_range,
        merge_base:$merge_base,
        target_descends_from_baseline:($merge_base == $base_head),
        source_ledger_report_sha256:$ledger_report_sha256,
        delta_index_hash_sha256:$delta_index_hash_sha256,
        delta_policy_hash_sha256:$delta_policy_hash_sha256,
        delta_side_effect_hash_sha256:$delta_side_effect_hash_sha256,
        latest_multisurface_absorption_ready:true,
        latest_multisurface_decision:"classified_as_oracle_only_without_merge_rebase_or_active_wiring",
        commit_count:$commit_count,
        expected_commit_count:$expected_commit_count,
        changed_file_count:$changed_file_count,
        expected_changed_file_count:$expected_changed_file_count,
        provider_security_changed_file_count:($ledger.buckets[] | select(.id == "provider-credential-sandbox-security").changed_file_count),
        runtime_appserver_changed_file_count:($ledger.buckets[] | select(.id == "runtime-session-tool-mcp-appserver").changed_file_count),
        legacy_cli_tui_changed_file_count:($ledger.buckets[] | select(.id == "legacy-cli-tui-compatibility").changed_file_count),
        product_governance_changed_file_count:($ledger.buckets[] | select(.id == "product-doc-release-governance").changed_file_count),
        expected_provider_security_changed_file_count:$expected_provider_count,
        expected_runtime_appserver_changed_file_count:$expected_runtime_count,
        expected_legacy_cli_tui_changed_file_count:$expected_compat_count,
        expected_product_governance_changed_file_count:$expected_governance_count,
        populated_bucket_count:$ledger.inventory.populated_bucket_count,
        all_buckets_populated:$ledger.inventory.all_buckets_populated,
        family_count:($family_inventory | length),
        ready_family_count:($family_inventory | map(select(.ready == true)) | length),
        activation_blocking_family_count:($family_inventory | map(select(.promotion_allowed == false)) | length),
        family_inventory:$family_inventory,
        commit_sample:$commit_sample,
        required_follow_on_gates:[
          "doctor thread inventory must stay redacted and local-only before active route exposure",
          "remote status display must not mutate Gateway state",
          "TUI markdown/status/stderr changes remain compatibility intake unless Hepta UI absorbs them",
          "process-hardening malloc diagnostics must not mutate launchd environment by default",
          "active hepta-cli dependency isolation must remain green"
        ],
        active_runtime_promotion_allowed:false,
        active_appserver_promotion_allowed:false,
        active_tui_promotion_allowed:false,
        active_process_hardening_env_mutation_allowed:false,
        upstream_fetch_performed_by_gate:false,
        upstream_merge_performed:false,
        upstream_checkout_performed:false,
        active_runtime_auto_rebase_allowed:false,
        active_runtime_dependency_allowed:false,
        active_binary_mutation_allowed:false,
        active_service_restart_allowed:false,
        launchd_mutation_allowed:false,
        provider_model_invocation_allowed:false,
        channel_delivery_allowed:false,
        public_release_claim_allowed:false,
        public_ga_claim_allowed:false,
        release_artifact_write_allowed:false,
        evidence_persistence_allowed:false,
        denied_by_latest_multisurface_absorption:$denied,
        latest_multisurface_denied_by_count:($denied | length),
        side_effects:{
          upstream_fetch_performed:false,
          upstream_merge_performed:false,
          upstream_checkout_performed:false,
          workspace_write:false,
          active_binary_mutated:false,
          active_service_restart:false,
          launchd_mutated:false,
          gateway_mutation_performed:false,
          provider_invoked:false,
          model_invoked:false,
          channel_send_performed:false,
          release_artifact_written:false,
          public_release_published:false,
          public_ga_claimed:false,
          evidence_persisted:false,
          credential_value_read:false,
          secret_file_read:false
        }
      }'
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .latest_multisurface_absorption_ready == true
  and .commit_count == .expected_commit_count
  and .changed_file_count == .expected_changed_file_count
  and .provider_security_changed_file_count == .expected_provider_security_changed_file_count
  and .runtime_appserver_changed_file_count == .expected_runtime_appserver_changed_file_count
  and .legacy_cli_tui_changed_file_count == .expected_legacy_cli_tui_changed_file_count
  and .product_governance_changed_file_count == .expected_product_governance_changed_file_count
  and .populated_bucket_count == 3
  and .all_buckets_populated == false
  and .family_count == 5
  and .ready_family_count == 5
  and .activation_blocking_family_count == 5
  and .active_runtime_promotion_allowed == false
  and .upstream_merge_performed == false
  and .active_runtime_auto_rebase_allowed == false
  and .active_runtime_dependency_allowed == false
  and .active_service_restart_allowed == false
  and .public_release_claim_allowed == false
  and .release_artifact_write_allowed == false
  and .latest_multisurface_denied_by_count == 13
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta upstream Codex latest multisurface absorption gate passed"
