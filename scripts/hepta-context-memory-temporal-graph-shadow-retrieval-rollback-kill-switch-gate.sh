#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
release_manifest="$repo_root/codex-rs/CONTEXT_LANE_RELEASE_MANIFEST.tsv"
report_script="$repo_root/scripts/hepta-context-memory-temporal-graph-shadow-retrieval-rollback-kill-switch-report.sh"
gate_script="$repo_root/scripts/hepta-context-memory-temporal-graph-shadow-retrieval-rollback-kill-switch-gate.sh"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
hepta_core_memory="$repo_root/codex-rs/hepta-core/src/memory.rs"
hepta_core_temporal="$repo_root/codex-rs/hepta-core/src/memory/temporal.rs"
hepta_core_temporal_rollback_kill_switch="$repo_root/codex-rs/hepta-core/src/memory/temporal/rollback_kill_switch.rs"
hepta_core_status_entry="$repo_root/codex-rs/hepta-core/src/memory/context_plane/status/entry.rs"
hepta_core_activation_target="$repo_root/codex-rs/hepta-core/src/memory/context_plane/activation/target.rs"
hepta_core_operator="$repo_root/codex-rs/hepta-core/src/memory/context_plane/operator.rs"
hepta_memory_context_plane_helpers="$repo_root/codex-rs/hepta-memory/src/context_plane_helpers.rs"
hepta_memory_snapshot_helpers="$repo_root/codex-rs/hepta-memory/src/recall_helpers/snapshot.rs"
hepta_memory_store_helpers="$repo_root/codex-rs/hepta-memory/src/recall_helpers/store.rs"
lane="${HEPTA_CARGO_LANE:-${HEPTA_LANE:-hepta-context}}"
target_root="${HEPTA_CARGO_TARGET_ROOT:-$HOME/.openclaw/tmp/cargo-targets}"
target_leaf="$lane"
if [[ "$target_leaf" != hepta-* ]]; then
  target_leaf="hepta-$target_leaf"
fi
export CARGO_TARGET_DIR="${HEPTA_CARGO_TARGET_DIR:-$target_root/$target_leaf}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"
report_output="$(mktemp -t hepta-context-memory-temporal-graph-shadow-retrieval-rollback-kill-switch-report.XXXXXX)"

cleanup() {
  rm -f "$report_output"
}
trap cleanup EXIT

fail() {
  echo "hepta-context-memory-temporal-graph-shadow-retrieval-rollback-kill-switch-gate: $*" >&2
  if [ -s "$report_output" ]; then
    echo "temporal graph shadow retrieval rollback/kill-switch report output:" >&2
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
    fail "temporal graph shadow retrieval rollback/kill-switch report must contain line: $expected"
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
  "temporal-graph-shadow-retrieval-rollback-kill-switch=pass" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.payload-light=pass" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.schema=1" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.source-retrieval-canary-guard-schema=1" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.mode=shadow-retrieval-rollback-kill-switch" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.fixture-count=5" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.stage-required-count=6" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.stage-projected-count=6" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.canary-guard-pass-count=5" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.operator-approval-required-count=5" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.operator-approval-recorded-count=0" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.feature-flag-registered-count=5" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.feature-flag-enabled-count=0" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.kill-switch-registered-count=5" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.kill-switch-readback-count=5" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.kill-switch-pass-count=5" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-rehearsal-required-count=5" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-rehearsal-readback-count=5" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-rehearsal-pass-count=5" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.route-denial-count=5" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-write-denial-count=5" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.canary-route-opened-count=0" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.digest-count=6" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.freshness-pass-count=6" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.replay-guard-pass-count=6" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.stale-replay-rejected-count=6" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.aggregate-counters-only=pass" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.llm-rerank=disabled" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.graph-persistence=disabled" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.production-route=disabled" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.production-write-count=0" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.graph-write-count=0" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-write-count=0" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.hot-path-write=disabled" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.prompt-assembly-change=disabled" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.runtime-activation=disabled" \
  "temporal-graph-shadow-retrieval-rollback-kill-switch.operator-activation=disabled"; do
  assert_report_line "$line"
done

for term in \
  "Temporal graph shadow retrieval rollback/kill-switch evidence surface" \
  "shadow temporal graph retrieval rollback kill-switch" \
  "memory_temporal_graph_shadow_retrieval_rollback_kill_switch" \
  "ContextMemoryTemporalGraphShadowRetrievalRollbackKillSwitchReport" \
  "context_memory_temporal_graph_shadow_retrieval_rollback_kill_switch_report" \
  "recall_context_memory_temporal_graph_shadow_retrieval_rollback_kill_switch_report" \
  "evidence_fixture_count" \
  "evidence_stage_projected_count" \
  "canary_guard_pass_count" \
  "kill_switch_readback_count" \
  "rollback_rehearsal_readback_count" \
  "route_denial_count" \
  "rollback_write_denial_count" \
  "canary_route_opened_count" \
  "rollback_write=false" \
  "aggregate_counters_only" \
  "llm_rerank=false" \
  "graph_persistence=false" \
  "production_route=false" \
  "production_write=false" \
  "graph_write=false" \
  "must not export retrieval payloads" \
  "must not open a canary route" \
  "must not write rollback state" \
  "must not persist graph facts" \
  "must not enable LLM rerank" \
  "must not alter production recall routing" \
  "hepta-context-memory-temporal-graph-shadow-retrieval-rollback-kill-switch-report.sh" \
  "hepta-context-memory-temporal-graph-shadow-retrieval-rollback-kill-switch-gate.sh"; do
  assert_file_contains "$contracts" "$term" "temporal graph shadow retrieval rollback/kill-switch contract"
done

assert_file_contains "$hepta_core_memory" \
  "CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_RETRIEVAL_ROLLBACK_KILL_SWITCH_SCHEMA_VERSION" \
  "temporal graph shadow retrieval rollback/kill-switch schema version"
assert_file_contains "$hepta_core_memory" \
  "ContextMemoryTemporalGraphShadowRetrievalRollbackKillSwitchReport" \
  "temporal graph shadow retrieval rollback/kill-switch public re-export"
assert_file_contains "$hepta_core_temporal" \
  "mod rollback_kill_switch;" \
  "temporal graph shadow retrieval rollback/kill-switch module boundary"
assert_file_contains "$hepta_core_temporal" \
  "pub use rollback_kill_switch::ContextMemoryTemporalGraphShadowRetrievalRollbackKillSwitchReport" \
  "temporal graph shadow retrieval rollback/kill-switch wrapper re-export"
assert_file_contains "$hepta_core_temporal_rollback_kill_switch" \
  "ContextMemoryTemporalGraphShadowRetrievalRollbackKillSwitchReport" \
  "temporal graph shadow retrieval rollback/kill-switch rust report"
assert_file_contains "$hepta_core_temporal_rollback_kill_switch" \
  "pub fn from_retrieval_canary_guard" \
  "temporal graph shadow retrieval rollback/kill-switch canary constructor"
assert_file_contains "$hepta_core_temporal_rollback_kill_switch" \
  "pub fn has_retrieval_rollback_kill_switch_integrity" \
  "temporal graph shadow retrieval rollback/kill-switch integrity gate"
assert_file_contains "$hepta_core_status_entry" \
  "from_temporal_graph_shadow_retrieval_rollback_kill_switch" \
  "temporal graph shadow retrieval rollback/kill-switch status entry"
assert_file_contains "$hepta_core_activation_target" \
  "MemoryTemporalGraphShadowRetrievalRollbackKillSwitch" \
  "temporal graph shadow retrieval rollback/kill-switch activation target"
assert_file_contains "$hepta_core_activation_target" \
  "TemporalGraphShadowRetrievalRollbackKillSwitchShadowOnly" \
  "temporal graph shadow retrieval rollback/kill-switch activation blocker"
assert_file_contains "$hepta_core_operator" \
  "memory_temporal_graph_shadow_retrieval_rollback_kill_switch_route_denial_count" \
  "temporal graph shadow retrieval rollback/kill-switch operator counter"

assert_file_contains "$hepta_memory_context_plane_helpers" \
  "context_memory_temporal_graph_shadow_retrieval_rollback_kill_switch_report" \
  "temporal graph shadow retrieval rollback/kill-switch hepta-memory context-plane helper"
assert_file_contains "$hepta_memory_snapshot_helpers" \
  "recall_context_memory_temporal_graph_shadow_retrieval_rollback_kill_switch_report" \
  "temporal graph shadow retrieval rollback/kill-switch hepta-memory snapshot helper"
assert_file_contains "$hepta_memory_store_helpers" \
  "recall_context_memory_temporal_graph_shadow_retrieval_rollback_kill_switch_report" \
  "temporal graph shadow retrieval rollback/kill-switch hepta-memory store helper"

assert_file_contains "$debug_gate" "hepta-context-memory-temporal-graph-shadow-retrieval-rollback-kill-switch-gate.sh" \
  "temporal graph shadow retrieval rollback/kill-switch debug gate"
assert_file_contains "$preflight_script" "context memory temporal graph shadow retrieval rollback/kill-switch gate" \
  "temporal graph shadow retrieval rollback/kill-switch preflight stage"
assert_file_contains "$release_manifest" "codex-rs/hepta-core/src/memory/temporal/rollback_kill_switch.rs" \
  "temporal graph shadow retrieval rollback/kill-switch rust release manifest"
assert_file_contains "$release_manifest" "scripts/hepta-context-memory-temporal-graph-shadow-retrieval-rollback-kill-switch-report.sh" \
  "temporal graph shadow retrieval rollback/kill-switch report release manifest"
assert_file_contains "$release_manifest" "scripts/hepta-context-memory-temporal-graph-shadow-retrieval-rollback-kill-switch-gate.sh" \
  "temporal graph shadow retrieval rollback/kill-switch gate release manifest"
assert_file_contains "$front_door_gate" "memory_temporal_graph_shadow_retrieval_rollback_kill_switch_gate_script" \
  "temporal graph shadow retrieval rollback/kill-switch front-door static check"

assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-temporal-graph-shadow-retrieval-canary-guard-gate.sh" \
  "hepta-context-memory-temporal-graph-shadow-retrieval-rollback-kill-switch-gate.sh" \
  "temporal graph shadow retrieval rollback/kill-switch debug order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-temporal-graph-shadow-retrieval-rollback-kill-switch-gate.sh" \
  "hepta-context-memory-eval-harness-seed-gate.sh" \
  "temporal graph shadow retrieval rollback/kill-switch debug order"
assert_line_before \
  "$preflight_script" \
  "context memory temporal graph shadow retrieval canary guard gate" \
  "context memory temporal graph shadow retrieval rollback/kill-switch gate" \
  "temporal graph shadow retrieval rollback/kill-switch preflight order"
assert_line_before \
  "$preflight_script" \
  "context memory temporal graph shadow retrieval rollback/kill-switch gate" \
  "context memory eval harness seed gate" \
  "temporal graph shadow retrieval rollback/kill-switch preflight order"

leak_pattern='(prompt_text|transcript_text|memory_text|answer_text|session_id|memory_id|source_id|trace_id|query_text|query_payload|candidate_text|candidate_payload|candidate_id|path_payload|tool_args|tool_outputs|raw_payload|raw_graph_payload|entity_hash|fact_hash|edge_hash|operator_identity|llm-rerank=enabled|graph-persistence=enabled|production-route=enabled|runtime-activation=enabled|production-write=enabled|graph-write=enabled|operator-activation=enabled|canary-route-opened-count=[1-9]|feature-flag-enabled-count=[1-9]|operator-approval-recorded-count=[1-9]|production-write-count=[1-9]|graph-write-count=[1-9]|rollback-write-count=[1-9])'
if grep -E "$leak_pattern" "$report_output" >/dev/null; then
  fail "temporal graph shadow retrieval rollback/kill-switch report leaked payload, route, write, persistence, rerank, or activation state"
fi

bash -n "$report_script"
bash -n "$gate_script"

cargo test --manifest-path "$manifest" -p hepta-core \
  context_plane \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  context_plane \
  --lib --message-format=short
