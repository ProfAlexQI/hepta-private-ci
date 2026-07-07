#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
release_manifest="$repo_root/codex-rs/CONTEXT_LANE_RELEASE_MANIFEST.tsv"
report_script="$repo_root/scripts/hepta-context-memory-temporal-graph-shadow-eval-report.sh"
gate_script="$repo_root/scripts/hepta-context-memory-temporal-graph-shadow-eval-gate.sh"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
hepta_core_memory="$repo_root/codex-rs/hepta-core/src/memory.rs"
hepta_core_eval_harness="$repo_root/codex-rs/hepta-core/src/memory/eval_harness.rs"
hepta_core_temporal_graph_shadow="$repo_root/codex-rs/hepta-core/src/memory/eval_harness/temporal_graph_shadow.rs"
hepta_core_memory_tests="$repo_root/codex-rs/hepta-core/src/memory/tests/eval_harness.rs"
hepta_memory_context_plane_helpers="$repo_root/codex-rs/hepta-memory/src/context_plane_helpers.rs"
hepta_memory_tests="$repo_root/codex-rs/hepta-memory/src/tests/context_memory.rs"
lane="${HEPTA_CARGO_LANE:-${HEPTA_LANE:-hepta-context}}"
target_root="${HEPTA_CARGO_TARGET_ROOT:-$HOME/.openclaw/tmp/cargo-targets}"
target_leaf="$lane"
if [[ "$target_leaf" != hepta-* ]]; then
  target_leaf="hepta-$target_leaf"
fi
export CARGO_TARGET_DIR="${HEPTA_CARGO_TARGET_DIR:-$target_root/$target_leaf}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"
report_output="$(mktemp -t hepta-context-memory-temporal-graph-shadow-eval-report.XXXXXX)"

cleanup() {
  rm -f "$report_output"
}
trap cleanup EXIT

fail() {
  echo "hepta-context-memory-temporal-graph-shadow-eval-gate: $*" >&2
  if [ -s "$report_output" ]; then
    echo "temporal graph shadow eval report output:" >&2
    cat "$report_output" >&2
  fi
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

assert_report_line() {
  local expected="$1"
  if ! grep -F -x "$expected" "$report_output" >/dev/null; then
    fail "temporal graph shadow eval report must contain line: $expected"
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

bash "$report_script" >"$report_output"

assert_report_line "temporal-graph-shadow-eval=pass"
assert_report_line "temporal-graph-shadow-eval.payload-light=pass"
assert_report_line "temporal-graph-shadow-eval.schema=1"
assert_report_line "temporal-graph-shadow-eval.mode=deterministic-shadow"
assert_report_line "temporal-graph-shadow-eval.fixture-count=4"
assert_report_line "temporal-graph-shadow-eval.fixture-pass-count=4"
assert_report_line "temporal-graph-shadow-eval.positive-fixture-count=3"
assert_report_line "temporal-graph-shadow-eval.negative-fixture-count=1"
assert_report_line "temporal-graph-shadow-eval.node-coverage-floor-basis-points=10000"
assert_report_line "temporal-graph-shadow-eval.edge-coverage-floor-basis-points=10000"
assert_report_line "temporal-graph-shadow-eval.validity-window-floor-basis-points=10000"
assert_report_line "temporal-graph-shadow-eval.supersedes-floor-basis-points=10000"
assert_report_line "temporal-graph-shadow-eval.latency-max-ms=100"
assert_report_line "temporal-graph-shadow-eval.regret-max-basis-points=0"
assert_report_line "temporal-graph-shadow-eval.min-positive-node-coverage-basis-points=10000"
assert_report_line "temporal-graph-shadow-eval.min-positive-edge-coverage-basis-points=10000"
assert_report_line "temporal-graph-shadow-eval.min-positive-validity-window-coverage-basis-points=10000"
assert_report_line "temporal-graph-shadow-eval.min-positive-supersedes-coverage-basis-points=10000"
assert_report_line "temporal-graph-shadow-eval.max-positive-latency-ms=47"
assert_report_line "temporal-graph-shadow-eval.max-positive-regret-basis-points=0"
assert_report_line "temporal-graph-shadow-eval.regression-fixture=blocked"
assert_report_line "temporal-graph-shadow-eval.operator-approval=required"
assert_report_line "temporal-graph-shadow-eval.production-route=disabled"
assert_report_line "temporal-graph-shadow-eval.graph-write=disabled"
assert_report_line "temporal-graph-shadow-eval.runtime-activation=disabled"

for term in \
  "Temporal graph shadow eval" \
  "hepta-context-memory-temporal-graph-shadow-eval-report.sh" \
  "hepta-context-memory-temporal-graph-shadow-eval-gate.sh" \
  "deterministic-shadow" \
  "ContextMemoryTemporalGraphShadowEvalReport" \
  "context_memory_temporal_graph_shadow_eval_report" \
  "topology_coverage" \
  "validity_window_replay" \
  "supersedes_replay" \
  "regression_guard" \
  "node-coverage-floor-basis-points" \
  "edge-coverage-floor-basis-points" \
  "validity-window-floor-basis-points" \
  "supersedes-floor-basis-points" \
  "latency-max-ms" \
  "regret-max-basis-points" \
  "regression fixture"; do
  assert_file_contains "$contracts" "$term" "temporal graph shadow eval contract"
done

assert_file_contains "$hepta_core_memory" \
  "CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_EVAL_SCHEMA_VERSION" \
  "temporal graph shadow eval schema version"
assert_file_contains "$hepta_core_memory" \
  "pub use eval_harness::ContextMemoryTemporalGraphShadowEvalReport" \
  "temporal graph shadow eval public re-export"
assert_file_contains "$hepta_core_eval_harness" \
  "mod temporal_graph_shadow" \
  "temporal graph shadow eval module boundary"
assert_file_contains "$hepta_core_eval_harness" \
  "pub use temporal_graph_shadow::ContextMemoryTemporalGraphShadowEvalReport" \
  "temporal graph shadow eval wrapper re-export"
assert_file_contains "$hepta_core_temporal_graph_shadow" \
  "ContextMemoryTemporalGraphShadowEvalReport" \
  "temporal graph shadow eval rust report"
assert_file_contains "$hepta_core_temporal_graph_shadow" \
  "ContextMemoryTemporalGraphShadowEvalFixtureResult" \
  "temporal graph shadow eval rust fixture"
assert_file_contains "$hepta_core_temporal_graph_shadow" \
  "pub fn has_temporal_graph_shadow_integrity" \
  "temporal graph shadow eval integrity gate"
assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_temporal_graph_shadow_eval_tracks_metrics_without_activation" \
  "temporal graph shadow eval hepta-core positive test"
assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_temporal_graph_shadow_eval_blocks_regression_drift" \
  "temporal graph shadow eval hepta-core regression test"

assert_file_contains "$hepta_memory_context_plane_helpers" \
  "context_memory_temporal_graph_shadow_eval_report" \
  "temporal graph shadow eval hepta-memory helper"
assert_file_contains "$hepta_memory_tests" \
  "store_snapshot_context_memory_temporal_graph_shadow_eval_is_payload_light" \
  "temporal graph shadow eval hepta-memory snapshot test"
assert_file_contains "$hepta_memory_tests" \
  "store_context_memory_temporal_graph_shadow_eval_matches_snapshot_helper" \
  "temporal graph shadow eval hepta-memory store test"

assert_file_contains "$debug_gate" "hepta-context-memory-temporal-graph-shadow-eval-gate.sh" \
  "temporal graph shadow eval debug gate"
assert_file_contains "$preflight_script" "context memory temporal graph shadow eval gate" \
  "temporal graph shadow eval preflight stage"
assert_file_contains "$release_manifest" "codex-rs/hepta-core/src/memory/eval_harness/temporal_graph_shadow.rs" \
  "temporal graph shadow eval rust release manifest"
assert_file_contains "$release_manifest" "scripts/hepta-context-memory-temporal-graph-shadow-eval-report.sh" \
  "temporal graph shadow eval report release manifest"
assert_file_contains "$release_manifest" "scripts/hepta-context-memory-temporal-graph-shadow-eval-gate.sh" \
  "temporal graph shadow eval gate release manifest"
assert_file_contains "$front_door_gate" "memory_temporal_graph_shadow_eval_gate_script" \
  "temporal graph shadow eval front-door static check"

assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-temporal-fact-graph-gate.sh" \
  "hepta-context-memory-temporal-graph-shadow-eval-gate.sh" \
  "temporal graph shadow eval debug order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-temporal-graph-shadow-eval-gate.sh" \
  "hepta-context-memory-eval-harness-seed-gate.sh" \
  "temporal graph shadow eval debug order"
assert_line_before \
  "$preflight_script" \
  "context memory temporal fact graph dry-run gate" \
  "context memory temporal graph shadow eval gate" \
  "temporal graph shadow eval preflight order"
assert_line_before \
  "$preflight_script" \
  "context memory temporal graph shadow eval gate" \
  "context memory eval harness seed gate" \
  "temporal graph shadow eval preflight order"

leak_pattern='(prompt_text|transcript_text|memory_text|answer_text|session_id|memory_id|source_id|trace_id|query_text|query_payload|tool_args|tool_outputs|raw_payload|raw_graph_payload|entity_hash|fact_hash=|edge_hash=|operator_identity|runtime-activation=enabled|production-route=enabled|operator-activation=enabled|graph-write=enabled|production-write=enabled)'
if grep -E "$leak_pattern" "$report_output" >/dev/null; then
  fail "temporal graph shadow eval report leaked payload or activation state"
fi

bash -n "$report_script"
bash -n "$gate_script"

cargo test --manifest-path "$manifest" -p hepta-core \
  temporal_graph_shadow \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  temporal_graph_shadow \
  --lib --message-format=short

echo "temporal-graph-shadow-eval=pass"
echo "temporal-graph-shadow-eval.payload-light=pass"
echo "temporal-graph-shadow-eval.fixtures=4"
echo "temporal-graph-shadow-eval.regression-fixture=blocked"
echo "temporal-graph-shadow-eval.graph-write=disabled"
echo "temporal-graph-shadow-eval.runtime-activation=disabled"
