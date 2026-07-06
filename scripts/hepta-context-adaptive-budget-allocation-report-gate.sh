#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
protocol="$repo_root/codex-rs/protocol/src/protocol.rs"
context_manifest="$repo_root/codex-rs/core/src/context_manager/manifest.rs"
context_manifest_policy="$repo_root/codex-rs/core/src/context_manager/manifest/policy.rs"
context_manifest_tests="$repo_root/codex-rs/core/src/context_manager/manifest/tests.rs"
budget_planner="$repo_root/codex-rs/core/src/context_manager/budget_planner.rs"
source_registry="$repo_root/codex-rs/core/src/context_manager/source_registry.rs"
source_registry_entry="$repo_root/codex-rs/core/src/context_manager/source_registry/entry.rs"
response_debug_rollout="$repo_root/codex-rs/response-debug-context/src/rollout_context.rs"
response_debug_tests="$repo_root/codex-rs/response-debug-context/src/tests.rs"
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
  echo "hepta-context-adaptive-budget-allocation-report-gate: $*" >&2
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

for term in \
  "manifest adaptive budget-allocation list" \
  "Context adaptive budget allocation dry-run report" \
  "adaptive_budget_allocations" \
  "current_heuristic_action" \
  "proposed_action" \
  "would_drop" \
  "would_compress" \
  "must not change actual prompt assembly" \
  "runtime-activation=disabled" \
  "hepta-context-adaptive-budget-allocation-report-gate.sh"; do
  assert_file_contains "$contracts" "$term" "adaptive budget allocation contract"
done

assert_file_contains "$protocol" \
  "TURN_CONTEXT_ADAPTIVE_BUDGET_ALLOCATION_SCHEMA_VERSION" \
  "adaptive budget allocation protocol schema version"
assert_file_contains "$protocol" \
  "TurnContextAdaptiveBudgetAllocation" \
  "adaptive budget allocation protocol item"
assert_file_contains "$protocol" \
  "TurnContextBudgetAllocationAction" \
  "adaptive budget allocation protocol action"
assert_file_contains "$protocol" \
  "adaptive_budget_allocations_have_integrity" \
  "adaptive budget allocation protocol integrity"

assert_file_contains "$source_registry_entry" \
  "ContextSourceBudgetClass" \
  "adaptive budget allocation source registry budget class"
assert_file_contains "$source_registry_entry" \
  "fn as_str(self) -> &'static str" \
  "adaptive budget allocation source registry budget class string"
assert_file_contains "$context_manifest_policy" \
  "adaptive_budget_allocations_for_pressure" \
  "adaptive budget allocation core dry-run planner call"
assert_file_contains "$budget_planner" \
  "adaptive_budget_allocations_for_pressure" \
  "adaptive budget allocation core dry-run planner"
assert_file_contains "$budget_planner" \
  "context_source_registry_entries()" \
  "adaptive budget allocation registry-backed planner"
assert_file_contains "$budget_planner" \
  "current_heuristic_action" \
  "adaptive budget allocation heuristic comparison"
assert_file_contains "$budget_planner" \
  "would_drop" \
  "adaptive budget allocation drop report"
assert_file_contains "$budget_planner" \
  "would_compress" \
  "adaptive budget allocation compress report"
assert_file_contains "$context_manifest_tests" \
  "turn_context_manifest_records_adaptive_budget_allocations_without_prompt_mutation" \
  "adaptive budget allocation behavior-neutral core test"

assert_file_contains "$response_debug_rollout" \
  "latest_manifest_adaptive_budget_allocation_count" \
  "adaptive budget allocation response-debug summary"
assert_file_contains "$response_debug_rollout" \
  "manifest_adaptive_budget_allocations_invalid" \
  "adaptive budget allocation response-debug audit"
assert_file_contains "$response_debug_tests" \
  "rollout_context_debug_summary_surfaces_adaptive_budget_allocations_without_payloads" \
  "adaptive budget allocation response-debug no-payload test"

cargo test --manifest-path "$manifest" -p codex-protocol \
  turn_context_manifest_adaptive_budget_allocations_are_payload_light_and_hashed \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-core \
  turn_context_manifest_records_adaptive_budget_allocations_without_prompt_mutation \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-response-debug-context \
  rollout_context_debug_summary_surfaces_adaptive_budget_allocations_without_payloads \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-response-debug-context \
  rollout_context_debug_audit_rejects_invalid_adaptive_budget_allocation \
  --lib --message-format=short

echo "context-adaptive-budget-allocation=pass"
echo "context-adaptive-budget-allocation.registry-backed=pass"
echo "context-adaptive-budget-allocation.payload-light=pass"
echo "context-adaptive-budget-allocation.runtime-activation=disabled"
echo "Hepta context adaptive budget allocation report gate passed"
