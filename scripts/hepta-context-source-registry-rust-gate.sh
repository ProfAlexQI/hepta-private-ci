#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
rust_registry="$repo_root/codex-rs/core/src/context_manager/source_registry.rs"
rust_registry_catalog="$repo_root/codex-rs/core/src/context_manager/source_registry/catalog.rs"
rust_registry_entry="$repo_root/codex-rs/core/src/context_manager/source_registry/entry.rs"
rust_registry_health="$repo_root/codex-rs/core/src/context_manager/source_registry/health.rs"
rust_registry_tests="$repo_root/codex-rs/core/src/context_manager/source_registry/tests.rs"
context_manifest_classifier="$repo_root/codex-rs/core/src/context_manager/manifest/classification.rs"
context_manifest_policy_candidate="$repo_root/codex-rs/core/src/context_manager/manifest/policy/candidate.rs"
context_manifest_policy_compression="$repo_root/codex-rs/core/src/context_manager/manifest/policy/compression.rs"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
lane="${HEPTA_CARGO_LANE:-${HEPTA_LANE:-hepta-context}}"
target_root="${HEPTA_CARGO_TARGET_ROOT:-$HOME/.openclaw/tmp/cargo-targets}"
target_leaf="$lane"
if [[ "$target_leaf" != hepta-* ]]; then
  target_leaf="hepta-$target_leaf"
fi
export CARGO_TARGET_DIR="${HEPTA_CARGO_TARGET_DIR:-$target_root/$target_leaf}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"

fail() {
  echo "hepta-context-source-registry-rust-gate: $*" >&2
  exit 1
}

assert_file_contains() {
  local file_path="$1"
  local needle="$2"
  local label="$3"

  if ! grep -F "$needle" "$file_path" >/dev/null; then
    fail "$label must contain: $needle"
  fi
}

assert_file_not_contains() {
  local file_path="$1"
  local needle="$2"
  local label="$3"

  if grep -F "$needle" "$file_path" >/dev/null; then
    fail "$label must not contain stale hard-coded text: $needle"
  fi
}

assert_file_contains "$rust_registry" "mod catalog;" \
  "context source registry catalog module"
assert_file_contains "$rust_registry" "mod entry;" \
  "context source registry entry module"
assert_file_contains "$rust_registry" "mod health;" \
  "context source registry health module"
assert_file_contains "$rust_registry" "pub(crate) use catalog::context_source_registry_entries" \
  "context source registry parent re-export"
assert_file_contains "$rust_registry_catalog" "ContextSourceRegistryEntry" \
  "context source registry catalog entries"
assert_file_contains "$rust_registry_entry" "ContextSourceBudgetClass" \
  "context source registry entry metadata"
for term in \
  "ContextSourceTtl" \
  "ContextSourceVolatility" \
  "ContextSourceTrustClass" \
  "ContextSourceRedactionPolicy" \
  "ContextSourceQualityMetric" \
  "ContextSourceActivationGuard" \
  "ContextSourceRollbackPolicy"; do
  assert_file_contains "$rust_registry_entry" "$term" \
    "context source registry descriptor metadata"
done
assert_file_contains "$rust_registry_health" "ContextSourceRegistryHealthReport" \
  "context source registry health report"
assert_file_contains "$rust_registry_health" "live_activation_route_count" \
  "context source registry health live activation guard"
assert_file_contains "$rust_registry_tests" "include_str!(\"../../../../CONTEXT_SOURCE_REGISTRY.tsv\")" \
  "context source registry TSV sync test"
assert_file_contains "$context_manifest_classifier" "registered_contribution_classification" \
  "context source registry manifest classifier lookup"
assert_file_contains "$context_manifest_policy_candidate" "source_aware_omit_priority(" \
  "context source registry manifest priority lookup"
assert_file_contains "$context_manifest_policy_compression" "source_aware_compression_kind(" \
  "context source registry manifest compression lookup"

for term in \
  "Context source registry Rust resolver" \
  "source_registry.rs" \
  "source_registry/health.rs" \
  "single registry lookup" \
  "must not carry a second hard-coded omit-priority" \
  "must not change prompt assembly by itself" \
  "live-activation-routes=0" \
  "runtime-activation=disabled" \
  "hepta-context-source-registry-health-gate.sh" \
  "hepta-context-source-registry-rust-gate.sh"; do
  assert_file_contains "$contracts" "$term" "context source registry rust contract"
done

for stale in \
  '"extension_developer_capabilities") => Some(10)' \
  '"available_plugins") => Some(20)' \
  '"apps") => Some(30)' \
  '"available_skills") => Some(40)' \
  '"selected_context_recall") => Some(50)' \
  '"available_plugins" | "apps" | "available_skills"'; do
  assert_file_not_contains "$context_manifest_policy_candidate" "$stale" \
    "context source registry manifest priority lookup"
  assert_file_not_contains "$context_manifest_policy_compression" "$stale" \
    "context source registry manifest compression lookup"
done

cargo test --manifest-path "$manifest" -p codex-core \
  context_manager::source_registry::tests::context_source_registry_entries_are_sorted_and_complete \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-core \
  context_manager::source_registry::tests::context_source_registry_matches_catalog_tsv \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-core \
  context_manager::source_registry::tests::context_source_registry_exposes_source_aware_budget_policy \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-core \
  context_manager::source_registry::tests::context_source_registry_health_report_is_payload_light_and_non_activating \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-core \
  source_aware_budget_candidate_priority_is_tier_guarded \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-core \
  turn_context_manifest_records_compression_candidates_without_prompt_mutation \
  --lib --message-format=short

echo "context-source-registry-rust=pass"
echo "context-source-registry-rust.catalog-sync=pass"
echo "context-source-registry-rust.health-report=pass"
echo "context-source-registry-rust.manifest-lookup=pass"
echo "context-source-registry-rust.runtime-activation=disabled"
echo "Hepta context source registry rust resolver gate passed"
