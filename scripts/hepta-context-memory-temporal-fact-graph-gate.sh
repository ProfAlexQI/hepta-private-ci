#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
hepta_core_memory="$repo_root/codex-rs/hepta-core/src/memory.rs"
hepta_core_memory_recall_inspection="$repo_root/codex-rs/hepta-core/src/memory/recall/inspection.rs"
hepta_core_memory_temporal="$repo_root/codex-rs/hepta-core/src/memory/temporal.rs"
hepta_core_memory_temporal_graph="$repo_root/codex-rs/hepta-core/src/memory/temporal/graph.rs"
hepta_core_memory_tests="$repo_root/codex-rs/hepta-core/tests/memory_recall_contracts/temporal.rs"
hepta_memory="$repo_root/codex-rs/hepta-memory/src/lib.rs"
hepta_memory_tests="$repo_root/codex-rs/hepta-memory/src/tests/recall_memory/temporal.rs"
hepta_memory_recall_helpers="$repo_root/codex-rs/hepta-memory/src/recall_helpers/snapshot.rs"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
front_door_gate="$repo_root/scripts/lib/hepta-context-gates-v1/hepta-context-source-aware-compression-front-door.gate"
release_manifest="$repo_root/codex-rs/CONTEXT_LANE_RELEASE_MANIFEST.tsv"
lane="${HEPTA_CARGO_LANE:-${HEPTA_LANE:-hepta-context}}"
target_root="${HEPTA_CARGO_TARGET_ROOT:-$HOME/.openclaw/tmp/cargo-targets}"
target_leaf="$lane"
if [[ "$target_leaf" != hepta-* ]]; then
  target_leaf="hepta-$target_leaf"
fi
export CARGO_TARGET_DIR="${HEPTA_CARGO_TARGET_DIR:-$target_root/$target_leaf}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"

fail() {
  echo "hepta-context-memory-temporal-fact-graph-gate: $*" >&2
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
  "Memory temporal fact graph dry-run" \
  "memory_temporal_fact_graph" \
  "fact_hash" \
  "edge_hash" \
  "provenance" \
  "validity_window" \
  "supersedes" \
  "valid_from_sequence" \
  "invalid_at_sequence" \
  "confidence_basis_points" \
  "dry_run_only=true" \
  "production_write=false" \
  "graph_write=false" \
  "must not write graph facts" \
  "must not write production memory" \
  "must not alter prompt assembly" \
  "hepta-context-memory-temporal-fact-graph-gate.sh" \
  "runtime-activation=disabled"; do
  assert_file_contains "$contracts" "$term" "memory temporal fact graph contract"
done

assert_file_contains "$hepta_core_memory" \
  "CONTEXT_MEMORY_TEMPORAL_FACT_GRAPH_SCHEMA_VERSION" \
  "memory temporal fact graph schema version"
assert_file_contains "$hepta_core_memory_temporal" \
  "mod graph;" \
  "memory temporal fact graph leaf module"
assert_file_contains "$hepta_core_memory_temporal" \
  "pub use graph::ContextMemoryTemporalFactGraphReport" \
  "memory temporal fact graph wrapper re-export"
assert_file_contains "$hepta_core_memory_temporal_graph" \
  "ContextMemoryTemporalFactGraphEdgeKind" \
  "memory temporal fact graph edge kind"
assert_file_contains "$hepta_core_memory_temporal_graph" \
  "ContextMemoryTemporalFactGraphNode" \
  "memory temporal fact graph node"
assert_file_contains "$hepta_core_memory_temporal_graph" \
  "ContextMemoryTemporalFactGraphEdge" \
  "memory temporal fact graph edge"
assert_file_contains "$hepta_core_memory_temporal_graph" \
  "ContextMemoryTemporalFactGraphReport" \
  "memory temporal fact graph report"
assert_file_contains "$hepta_core_memory_recall_inspection" \
  "memory_temporal_fact_graph_report" \
  "memory temporal fact graph recall helper"
assert_file_contains "$hepta_core_memory_tests" \
  "context_recall_memory_temporal_fact_graph_is_payload_light_reversible_and_non_writing" \
  "memory temporal fact graph hepta-core test"

assert_file_contains "$hepta_memory" \
  "mod recall_helpers;" \
  "memory temporal fact graph hepta-memory recall helper module"
assert_file_contains "$hepta_memory_recall_helpers" \
  "recall_context_memory_temporal_fact_graph_report" \
  "memory temporal fact graph hepta-memory helper"
assert_file_contains "$hepta_memory_tests" \
  "store_snapshot_recall_context_memory_temporal_fact_graph_is_payload_light" \
  "memory temporal fact graph hepta-memory snapshot test"
assert_file_contains "$hepta_memory_tests" \
  "store_recall_context_memory_temporal_fact_graph_matches_snapshot_helper" \
  "memory temporal fact graph hepta-memory store test"

assert_file_contains "$debug_gate" \
  "hepta-context-memory-temporal-fact-graph-gate.sh" \
  "memory temporal fact graph debug gate"
assert_file_contains "$preflight_script" \
  "context memory temporal fact graph dry-run gate" \
  "memory temporal fact graph preflight stage"
assert_file_contains "$front_door_gate" \
  "memory_temporal_fact_graph_gate_script" \
  "memory temporal fact graph front-door static check"
assert_file_contains "$release_manifest" \
  "scripts/hepta-context-memory-temporal-fact-graph-gate.sh" \
  "memory temporal fact graph release manifest entry"

assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-temporal-fact-schema-gate.sh" \
  "hepta-context-memory-temporal-fact-graph-gate.sh" \
  "memory temporal fact graph debug gate order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-temporal-fact-graph-gate.sh" \
  "hepta-context-memory-eval-harness-seed-gate.sh" \
  "memory temporal fact graph debug gate order"
assert_line_before \
  "$preflight_script" \
  "context memory temporal fact schema dry-run gate" \
  "context memory temporal fact graph dry-run gate" \
  "memory temporal fact graph preflight order"
assert_line_before \
  "$preflight_script" \
  "context memory temporal fact graph dry-run gate" \
  "context memory eval harness seed gate" \
  "memory temporal fact graph preflight order"

cargo test --manifest-path "$manifest" -p hepta-core \
  memory_temporal_fact_graph \
  --test memory_recall_contracts --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  memory_temporal_fact_graph \
  --lib --message-format=short

echo "context-memory-temporal-fact-graph=pass"
echo "context-memory-temporal-fact-graph.payload-light=pass"
echo "context-memory-temporal-fact-graph.provenance-edges=pass"
echo "context-memory-temporal-fact-graph.validity-window-edges=pass"
echo "context-memory-temporal-fact-graph.supersedes-edges=pass"
echo "context-memory-temporal-fact-graph.production-write=disabled"
echo "context-memory-temporal-fact-graph.graph-write=disabled"
echo "context-memory-temporal-fact-graph.runtime-activation=disabled"
echo "Hepta context memory temporal fact graph gate passed"
