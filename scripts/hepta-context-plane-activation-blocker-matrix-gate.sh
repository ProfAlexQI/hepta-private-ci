#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
hepta_core_memory="$repo_root/codex-rs/hepta-core/src/memory.rs"
hepta_core_memory_context_plane="$repo_root/codex-rs/hepta-core/src/memory/context_plane.rs"
hepta_core_memory_context_plane_activation="$repo_root/codex-rs/hepta-core/src/memory/context_plane/activation.rs"
hepta_core_memory_context_plane_activation_matrix="$repo_root/codex-rs/hepta-core/src/memory/context_plane/activation/matrix.rs"
hepta_core_memory_context_plane_activation_row="$repo_root/codex-rs/hepta-core/src/memory/context_plane/activation/row.rs"
hepta_core_memory_context_plane_activation_target="$repo_root/codex-rs/hepta-core/src/memory/context_plane/activation/target.rs"
hepta_core_memory_tests="$repo_root/codex-rs/hepta-core/src/memory/tests/context_plane_activation.rs"
hepta_memory="$repo_root/codex-rs/hepta-memory/src/lib.rs"
hepta_memory_tests="$repo_root/codex-rs/hepta-memory/src/tests/context_plane/activation_matrix.rs"
hepta_memory_context_plane_helpers="$repo_root/codex-rs/hepta-memory/src/context_plane_helpers.rs"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
blocker_report="$repo_root/scripts/hepta-context-plane-activation-blocker-matrix-report.sh"
lane="${HEPTA_CARGO_LANE:-${HEPTA_LANE:-hepta-context}}"
target_root="${HEPTA_CARGO_TARGET_ROOT:-$HOME/.openclaw/tmp/cargo-targets}"
target_leaf="$lane"
if [[ "$target_leaf" != hepta-* ]]; then
  target_leaf="hepta-$target_leaf"
fi
export CARGO_TARGET_DIR="${HEPTA_CARGO_TARGET_DIR:-$target_root/$target_leaf}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"

fail() {
  echo "hepta-context-plane-activation-blocker-matrix-gate: $*" >&2
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

line_number_of() {
  local file_path="$1"
  local needle="$2"
  local line

  line="$(grep -n -F "$needle" "$file_path" | head -n 1 | cut -d: -f1 || true)"
  if [ -z "$line" ]; then
    fail "$file_path is missing required text: $needle"
  fi
  printf '%s\n' "$line"
}

assert_line_before() {
  local file_path="$1"
  local before_needle="$2"
  local after_needle="$3"
  local label="$4"
  local before_line
  local after_line

  before_line="$(line_number_of "$file_path" "$before_needle")"
  after_line="$(line_number_of "$file_path" "$after_needle")"
  if [ "$before_line" -ge "$after_line" ]; then
    fail "$label expected '$before_needle' before '$after_needle'"
  fi
}

for term in \
  "Context Plane activation blocker matrix" \
  "context-plane-activation-blockers=pass" \
  "source_registry" \
  "adaptive_budget_allocation" \
  "memory_taxonomy" \
  "memory_formation_receipts" \
  "memory_formation_queue" \
  "memory_temporal_facts" \
  "memory_temporal_fact_graph" \
  "eval_harness_seed" \
  "adaptive_allocator_eval_shadow" \
  "recall_quality_gate" \
  "memory_provider_boundary" \
  "recall_quality_blocking_reason_count" \
  "recall_quality_blocking_reasons" \
  "context-plane-activation-blockers.recall-quality-blocking-reason-count=0" \
  "context-plane-activation-blockers.recall-quality-blocking-reasons=none" \
  "source_aware_front_door" \
  "operator_approval" \
  "adaptive_budget_allocation_shadow_only" \
  "memory_provider_boundary_shadow_only" \
  "source_aware_front_door_disabled" \
  "operator_approval_missing" \
  "side_effect_flag_enabled" \
  "must not contain prompt text" \
  "must not contain transcript text" \
  "must not contain memory text" \
  "must not contain answer text" \
  "no production memory writes" \
  "no graph writes" \
  "no runtime activation" \
  "no adaptive allocator runtime activation" \
  "no source-aware runtime activation" \
  "no prompt assembly changes" \
  "no operator activation allowance" \
  "hepta-context-plane-activation-blocker-matrix-report.sh" \
  "hepta-context-plane-activation-blocker-matrix-gate.sh" \
  "runtime-activation=disabled"; do
  assert_file_contains "$contracts" "$term" "context plane activation blocker matrix contract"
done

assert_file_contains "$hepta_core_memory" \
  "CONTEXT_PLANE_ACTIVATION_BLOCKER_SCHEMA_VERSION" \
  "context plane activation blocker schema version"
assert_file_contains "$hepta_core_memory_context_plane" \
  "mod activation" \
  "context plane activation module"
assert_file_contains "$hepta_core_memory_context_plane" \
  "pub use activation::" \
  "context plane activation re-export"
assert_file_contains "$hepta_core_memory_context_plane_activation" \
  "mod target" \
  "context plane activation target module"
assert_file_contains "$hepta_core_memory_context_plane_activation" \
  "mod row" \
  "context plane activation row module"
assert_file_contains "$hepta_core_memory_context_plane_activation" \
  "mod matrix" \
  "context plane activation matrix module"
assert_file_contains "$hepta_core_memory_context_plane_activation_target" \
  "ContextPlaneActivationTarget" \
  "context plane activation target enum"
assert_file_contains "$hepta_core_memory_context_plane_activation_target" \
  "ContextPlaneActivationBlockerReason" \
  "context plane activation blocker reason enum"
assert_file_contains "$hepta_core_memory_context_plane_activation_matrix" \
  "ContextPlaneActivationBlockerMatrix" \
  "context plane activation blocker matrix"
assert_file_contains "$hepta_core_memory_context_plane_activation_matrix" \
  "pub fn from_status(status: &ContextPlaneStatusReport) -> Self" \
  "context plane activation blocker constructor"
assert_file_contains "$hepta_core_memory_tests" \
  "context_plane_activation_blocker_matrix_explains_disabled_runtime_activation" \
  "context plane activation blocker hepta-core test"
assert_file_contains "$hepta_core_memory_tests" \
  "context_plane_activation_blocker_matrix_blocks_side_effect_flags_without_activation" \
  "context plane activation blocker side-effect negative hepta-core test"
assert_file_contains "$hepta_core_memory_tests" \
  "context_plane_activation_blocker_matrix_rolls_up_recall_quality_blockers_without_payloads" \
  "context plane activation blocker recall-quality no-payload rollup hepta-core test"
assert_file_contains "$hepta_core_memory_context_plane_activation_row" \
  "ContextPlaneActivationBlockerReason::SideEffectFlagEnabled" \
  "context plane activation blocker side-effect reason"

assert_file_contains "$hepta_memory" \
  "mod context_plane_helpers" \
  "context plane activation blocker hepta-memory helper module"
assert_file_contains "$hepta_memory_context_plane_helpers" \
  "context_plane_activation_blocker_matrix" \
  "context plane activation blocker hepta-memory helper"
assert_file_contains "$hepta_memory_tests" \
  "store_snapshot_context_plane_activation_blocker_matrix_is_payload_light" \
  "context plane activation blocker hepta-memory snapshot test"
assert_file_contains "$hepta_memory_tests" \
  "store_context_plane_activation_blocker_matrix_matches_snapshot_helper" \
  "context plane activation blocker hepta-memory store test"

assert_file_contains "$debug_gate" \
  "hepta-context-plane-activation-blocker-matrix-gate.sh" \
  "context plane activation blocker debug gate"
assert_file_contains "$preflight_script" \
  "context plane activation blocker matrix gate" \
  "context plane activation blocker preflight stage"
assert_file_contains "$front_door_gate" \
  "context_plane_activation_blocker_matrix_gate_script" \
  "context plane activation blocker front-door static check"

assert_line_before \
  "$debug_gate" \
  "hepta-context-plane-status-report-gate.sh" \
  "hepta-context-plane-activation-blocker-matrix-gate.sh" \
  "context plane activation blocker debug gate order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-plane-activation-blocker-matrix-gate.sh" \
  "hepta-context-source-aware-compression-front-door-report.sh" \
  "context plane activation blocker debug front-door order"
assert_line_before \
  "$preflight_script" \
  "context plane status/export report gate" \
  "context plane activation blocker matrix gate" \
  "context plane activation blocker preflight order"
assert_line_before \
  "$preflight_script" \
  "context plane activation blocker matrix gate" \
  "source-aware compression front-door machine-readable report" \
  "context plane activation blocker front-door preflight order"

expected_status="$(cat <<'STATUS'
context-plane-activation-blockers=pass
context-plane-activation-blockers.schema=2
context-plane-activation-blockers.rows=13
context-plane-activation-blockers.satisfied=9
context-plane-activation-blockers.blockers=4
context-plane-activation-blockers.source-registry=ready
context-plane-activation-blockers.adaptive-budget-allocation=blocked:adaptive_budget_allocation_shadow_only
context-plane-activation-blockers.memory-taxonomy=ready
context-plane-activation-blockers.memory-formation-receipts=ready
context-plane-activation-blockers.memory-formation-queue=ready
context-plane-activation-blockers.memory-temporal-facts=ready
context-plane-activation-blockers.memory-temporal-fact-graph=ready
context-plane-activation-blockers.eval-harness-seed=ready
context-plane-activation-blockers.adaptive-allocator-eval-shadow=shadow-threshold-pass
context-plane-activation-blockers.recall-quality-gate=ready
context-plane-activation-blockers.recall-quality-blocking-reason-count=0
context-plane-activation-blockers.recall-quality-blocking-reasons=none
context-plane-activation-blockers.memory-provider-boundary=blocked:memory_provider_boundary_shadow_only
context-plane-activation-blockers.source-aware-front-door=blocked:source_aware_front_door_disabled
context-plane-activation-blockers.operator-approval=blocked:operator_approval_missing
context-plane-activation-blockers.activation-allowed=disabled
context-plane-activation-blockers.runtime-activation=disabled
context-plane-activation-blockers.adaptive-allocator-runtime-activation=disabled
context-plane-activation-blockers.source-aware-runtime-activation=disabled
context-plane-activation-blockers.production-write=disabled
context-plane-activation-blockers.graph-write=disabled
context-plane-activation-blockers.prompt-assembly-change=disabled
context-plane-activation-blockers.operator-activation=disabled
STATUS
)"
actual_status="$(bash "$blocker_report")"
if [ "$actual_status" != "$expected_status" ]; then
  fail "context plane activation blocker report output changed"
fi

if printf '%s\n' "$actual_status" | grep -E 'prompt_text|transcript_text|memory_text|answer_text|source_id|session_id|memory_id|trace_id|query_text|tool_args|entity_hash|supersedes|idempotency|activation-allowed=enabled|runtime-activation=enabled|production-write=enabled|graph-write=enabled' >/dev/null; then
  fail "context plane activation blocker report leaked payload or enabled activation"
fi

cargo test --manifest-path "$manifest" -p hepta-core \
  context_plane_activation \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  context_plane_activation \
  --lib --message-format=short

echo "context-plane-activation-blocker-matrix=pass"
echo "context-plane-activation-blocker-matrix.payload-light=pass"
echo "context-plane-activation-blocker-matrix.runtime-activation=disabled"
echo "Hepta context plane activation blocker matrix gate passed"
