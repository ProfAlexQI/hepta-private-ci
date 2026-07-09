#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
release_manifest="$repo_root/codex-rs/CONTEXT_LANE_RELEASE_MANIFEST.tsv"
report_script="$repo_root/scripts/hepta-context-memory-temporal-graph-shadow-replay-report.sh"
gate_script="$repo_root/scripts/hepta-context-memory-temporal-graph-shadow-replay-gate.sh"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
hepta_core_memory="$repo_root/codex-rs/hepta-core/src/memory.rs"
hepta_core_temporal="$repo_root/codex-rs/hepta-core/src/memory/temporal.rs"
hepta_core_temporal_replay="$repo_root/codex-rs/hepta-core/src/memory/temporal/replay.rs"
hepta_core_memory_tests="$repo_root/codex-rs/hepta-core/src/memory/tests/recall_memory/temporal.rs"
hepta_memory_context_plane_helpers="$repo_root/codex-rs/hepta-memory/src/context_plane_helpers.rs"
hepta_memory_snapshot_helpers="$repo_root/codex-rs/hepta-memory/src/recall_helpers/snapshot.rs"
hepta_memory_store_helpers="$repo_root/codex-rs/hepta-memory/src/recall_helpers/store.rs"
hepta_memory_tests="$repo_root/codex-rs/hepta-memory/src/tests/recall_memory/temporal.rs"
lane="${HEPTA_CARGO_LANE:-${HEPTA_LANE:-hepta-context}}"
target_root="${HEPTA_CARGO_TARGET_ROOT:-$HOME/.openclaw/tmp/cargo-targets}"
target_leaf="$lane"
if [[ "$target_leaf" != hepta-* ]]; then
  target_leaf="hepta-$target_leaf"
fi
export CARGO_TARGET_DIR="${HEPTA_CARGO_TARGET_DIR:-$target_root/$target_leaf}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"
report_output="$(mktemp -t hepta-context-memory-temporal-graph-shadow-replay-report.XXXXXX)"

cleanup() {
  rm -f "$report_output"
}
trap cleanup EXIT

fail() {
  echo "hepta-context-memory-temporal-graph-shadow-replay-gate: $*" >&2
  if [ -s "$report_output" ]; then
    echo "temporal graph shadow replay report output:" >&2
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
    fail "temporal graph shadow replay report must contain line: $expected"
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

for line in \
  "temporal-graph-shadow-replay=pass" \
  "temporal-graph-shadow-replay.payload-light=pass" \
  "temporal-graph-shadow-replay.schema=1" \
  "temporal-graph-shadow-replay.source-store-schema=1" \
  "temporal-graph-shadow-replay.mode=approval-gated-shadow-wal-replay" \
  "temporal-graph-shadow-replay.node-count=5" \
  "temporal-graph-shadow-replay.edge-count=10" \
  "temporal-graph-shadow-replay.provenance-replay-count=5" \
  "temporal-graph-shadow-replay.bitemporal-validity-replay-count=5" \
  "temporal-graph-shadow-replay.fact-invalidation-replay-count=0" \
  "temporal-graph-shadow-replay.supersede-tombstone-replay-count=0" \
  "temporal-graph-shadow-replay.stage-required-count=6" \
  "temporal-graph-shadow-replay.stage-projected-count=6" \
  "temporal-graph-shadow-replay.replay-digest-count=6" \
  "temporal-graph-shadow-replay.freshness-pass-count=6" \
  "temporal-graph-shadow-replay.replay-guard-pass-count=6" \
  "temporal-graph-shadow-replay.stale-replay-rejected-count=6" \
  "temporal-graph-shadow-replay.operator-approval=required" \
  "temporal-graph-shadow-replay.operator-approval-recorded-count=0" \
  "temporal-graph-shadow-replay.recorded-receipt-count=0" \
  "temporal-graph-shadow-replay.persisted-receipt-count=0" \
  "temporal-graph-shadow-replay.production-route=disabled" \
  "temporal-graph-shadow-replay.production-write-count=0" \
  "temporal-graph-shadow-replay.graph-write-count=0" \
  "temporal-graph-shadow-replay.hot-path-write=disabled" \
  "temporal-graph-shadow-replay.prompt-assembly-change=disabled" \
  "temporal-graph-shadow-replay.runtime-activation=disabled" \
  "temporal-graph-shadow-replay.operator-activation=disabled"; do
  assert_report_line "$line"
done

for term in \
  "Temporal graph shadow replay surface" \
  "approval-gated shadow temporal graph WAL replay" \
  "memory_temporal_graph_shadow_replay" \
  "ContextMemoryTemporalGraphShadowReplayReport" \
  "context_memory_temporal_graph_shadow_replay_report" \
  "recall_context_memory_temporal_graph_shadow_replay_report" \
  "wal_replay_digest" \
  "provenance_replay_count" \
  "bitemporal_validity_replay_count" \
  "fact_invalidation_replay_count" \
  "supersede_tombstone_replay_count" \
  "digest_freshness_replayed" \
  "replay_guard_replayed" \
  "stale_replay_rejected" \
  "recorded_receipt=false" \
  "persisted_receipt=false" \
  "production_write=false" \
  "graph_write=false" \
  "must not persist receipts" \
  "must not write graph facts" \
  "must not write production memory" \
  "must not alter prompt assembly" \
  "hepta-context-memory-temporal-graph-shadow-replay-report.sh" \
  "hepta-context-memory-temporal-graph-shadow-replay-gate.sh"; do
  assert_file_contains "$contracts" "$term" "temporal graph shadow replay contract"
done

assert_file_contains "$hepta_core_memory" \
  "CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_REPLAY_SCHEMA_VERSION" \
  "temporal graph shadow replay schema version"
assert_file_contains "$hepta_core_memory" \
  "ContextMemoryTemporalGraphShadowReplayReport" \
  "temporal graph shadow replay public re-export"
assert_file_contains "$hepta_core_temporal" \
  "mod replay;" \
  "temporal graph shadow replay module boundary"
assert_file_contains "$hepta_core_temporal" \
  "pub use replay::ContextMemoryTemporalGraphShadowReplayReport" \
  "temporal graph shadow replay wrapper re-export"
assert_file_contains "$hepta_core_temporal_replay" \
  "ContextMemoryTemporalGraphShadowReplayReport" \
  "temporal graph shadow replay rust report"
assert_file_contains "$hepta_core_temporal_replay" \
  "pub fn from_shadow_store" \
  "temporal graph shadow replay store constructor"
assert_file_contains "$hepta_core_temporal_replay" \
  "pub fn has_shadow_replay_integrity" \
  "temporal graph shadow replay integrity gate"
assert_file_contains "$hepta_core_memory_tests" \
  "context_recall_memory_temporal_graph_shadow_replay_is_gateable_and_non_persistent" \
  "temporal graph shadow replay hepta-core test"

assert_file_contains "$hepta_memory_context_plane_helpers" \
  "context_memory_temporal_graph_shadow_replay_report" \
  "temporal graph shadow replay hepta-memory context-plane helper"
assert_file_contains "$hepta_memory_snapshot_helpers" \
  "recall_context_memory_temporal_graph_shadow_replay_report" \
  "temporal graph shadow replay hepta-memory snapshot helper"
assert_file_contains "$hepta_memory_store_helpers" \
  "recall_context_memory_temporal_graph_shadow_replay_report" \
  "temporal graph shadow replay hepta-memory store helper"
assert_file_contains "$hepta_memory_tests" \
  "store_snapshot_recall_context_memory_temporal_graph_shadow_replay_is_payload_light" \
  "temporal graph shadow replay hepta-memory snapshot test"
assert_file_contains "$hepta_memory_tests" \
  "store_recall_context_memory_temporal_graph_shadow_replay_matches_snapshot_helper" \
  "temporal graph shadow replay hepta-memory store test"

assert_file_contains "$debug_gate" "hepta-context-memory-temporal-graph-shadow-replay-gate.sh" \
  "temporal graph shadow replay debug gate"
assert_file_contains "$preflight_script" "context memory temporal graph shadow replay gate" \
  "temporal graph shadow replay preflight stage"
assert_file_contains "$release_manifest" "codex-rs/hepta-core/src/memory/temporal/replay.rs" \
  "temporal graph shadow replay rust release manifest"
assert_file_contains "$release_manifest" "scripts/hepta-context-memory-temporal-graph-shadow-replay-report.sh" \
  "temporal graph shadow replay report release manifest"
assert_file_contains "$release_manifest" "scripts/hepta-context-memory-temporal-graph-shadow-replay-gate.sh" \
  "temporal graph shadow replay gate release manifest"
assert_file_contains "$front_door_gate" "memory_temporal_graph_shadow_replay_gate_script" \
  "temporal graph shadow replay front-door static check"

assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-temporal-graph-shadow-store-gate.sh" \
  "hepta-context-memory-temporal-graph-shadow-replay-gate.sh" \
  "temporal graph shadow replay debug order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-temporal-graph-shadow-replay-gate.sh" \
  "hepta-context-memory-eval-harness-seed-gate.sh" \
  "temporal graph shadow replay debug order"
assert_line_before \
  "$preflight_script" \
  "context memory temporal graph shadow store skeleton gate" \
  "context memory temporal graph shadow replay gate" \
  "temporal graph shadow replay preflight order"
assert_line_before \
  "$preflight_script" \
  "context memory temporal graph shadow replay gate" \
  "context memory eval harness seed gate" \
  "temporal graph shadow replay preflight order"

leak_pattern='(prompt_text|transcript_text|memory_text|answer_text|session_id|memory_id|source_id|trace_id|query_text|query_payload|tool_args|tool_outputs|raw_payload|raw_graph_payload|entity_hash|fact_hash|edge_hash|operator_identity|runtime-activation=enabled|production-route=enabled|operator-activation=enabled|graph-write=enabled|production-write=enabled|recorded-receipt-count=[1-9]|persisted-receipt-count=[1-9])'
if grep -E "$leak_pattern" "$report_output" >/dev/null; then
  fail "temporal graph shadow replay report leaked payload, write, receipt, or activation state"
fi

bash -n "$report_script"
bash -n "$gate_script"

cargo test --manifest-path "$manifest" -p hepta-core \
  temporal_graph_shadow_replay \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  temporal_graph_shadow_replay \
  --lib --message-format=short

echo "temporal-graph-shadow-replay=pass"
echo "temporal-graph-shadow-replay.payload-light=pass"
echo "temporal-graph-shadow-replay.stage-projected-count=6"
echo "temporal-graph-shadow-replay.recorded-receipt-count=0"
echo "temporal-graph-shadow-replay.persisted-receipt-count=0"
echo "temporal-graph-shadow-replay.production-write=disabled"
echo "temporal-graph-shadow-replay.graph-write=disabled"
echo "temporal-graph-shadow-replay.runtime-activation=disabled"
