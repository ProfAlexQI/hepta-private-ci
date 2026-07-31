#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
hepta_core_memory="$repo_root/codex-rs/hepta-core/src/memory.rs"
hepta_core_memory_tests="$repo_root/codex-rs/hepta-core/tests/memory_recall_quality.rs"
hepta_core_recall_quality_gate="$repo_root/codex-rs/hepta-core/src/memory/recall_quality_gate.rs"
hepta_core_recall_quality_fixture="$repo_root/codex-rs/hepta-core/src/memory/recall_quality_gate/fixture.rs"
hepta_core_recall_quality_report="$repo_root/codex-rs/hepta-core/src/memory/recall_quality_gate/report.rs"
hepta_memory="$repo_root/codex-rs/hepta-memory/src/lib.rs"
hepta_memory_tests="$repo_root/codex-rs/hepta-memory/src/tests/context_memory.rs"
hepta_memory_context_plane_helpers="$repo_root/codex-rs/hepta-memory/src/context_plane_helpers.rs"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
front_door_gate="$repo_root/scripts/lib/hepta-context-gates-v1/hepta-context-source-aware-compression-front-door.gate"
lane="${HEPTA_CARGO_LANE:-${HEPTA_LANE:-hepta-context}}"
target_root="${HEPTA_CARGO_TARGET_ROOT:-$HOME/.openclaw/tmp/cargo-targets}"
target_leaf="$lane"
if [[ "$target_leaf" != hepta-* ]]; then
  target_leaf="hepta-$target_leaf"
fi
export CARGO_TARGET_DIR="${HEPTA_CARGO_TARGET_DIR:-$target_root/$target_leaf}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"

fail() {
  echo "hepta-context-memory-recall-quality-gate: $*" >&2
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
  "Context memory recall quality gate" \
  "recall_quality_gate" \
  "gate_pass" \
  "fixture_matrix" \
  "schema version 2" \
  "both seeded fixture kinds represented exactly once" \
  "minimum recall coverage 7000 basis points" \
  "minimum precision 7000 basis points" \
  "missing critical fact limit 2" \
  "fixture pass count equal to fixture count" \
  "zero fixture blocked count" \
  "zero blocking reason count" \
  "zero missing-critical-fact regressions" \
  "zero recall regressions" \
  "zero precision regressions" \
  "blocking_reasons" \
  "blocking_reason_count" \
  "missing_critical_fact_regression" \
  "recall_coverage_regression" \
  "precision_regression" \
  "answer_quality_regression" \
  "side_effect_flag_enabled" \
  "codex-rs/hepta-core/src/memory/recall_quality_gate.rs" \
  "missing_critical_fact" \
  "precision" \
  "answer-quality regression" \
  "must not contain prompt text" \
  "must not contain transcript text" \
  "must not contain memory text" \
  "must not contain answer text" \
  "zero safety leaks" \
  "zero answer-quality regressions" \
  "no production memory writes" \
  "no graph writes" \
  "no runtime activation" \
  "no adaptive allocator runtime activation" \
  "no source-aware runtime activation" \
  "no prompt assembly changes" \
  "no operator activation allowance" \
  "hepta-context-memory-recall-quality-gate.sh" \
  "runtime-activation=disabled"; do
  assert_file_contains "$contracts" "$term" "context memory recall quality gate contract"
done

assert_file_contains "$hepta_core_memory" \
  "CONTEXT_MEMORY_RECALL_QUALITY_GATE_SCHEMA_VERSION" \
  "context memory recall quality gate schema version"
assert_file_contains "$hepta_core_memory" \
  "CONTEXT_MEMORY_RECALL_QUALITY_GATE_SCHEMA_VERSION: u32 = 2" \
  "context memory recall quality gate schema version 2"
assert_file_contains "$hepta_core_memory" \
  "mod recall_quality_gate" \
  "context memory recall quality gate module boundary"
assert_file_contains "$hepta_core_memory" \
  "pub use recall_quality_gate::ContextMemoryRecallQualityGateReport" \
  "context memory recall quality gate public re-export"
assert_file_contains "$hepta_core_recall_quality_gate" \
  "ContextMemoryRecallQualityGateVerdict" \
  "context memory recall quality gate verdict"
assert_file_contains "$hepta_core_recall_quality_gate" \
  "pub use fixture::ContextMemoryRecallQualityFixtureGateReport" \
  "context memory recall quality gate fixture re-export"
assert_file_contains "$hepta_core_recall_quality_gate" \
  "pub use fixture::ContextMemoryRecallQualityGateBlockerReason" \
  "context memory recall quality gate blocker reason re-export"
assert_file_contains "$hepta_core_recall_quality_gate" \
  "pub use report::ContextMemoryRecallQualityGateReport" \
  "context memory recall quality gate report re-export"
assert_file_contains "$hepta_core_recall_quality_report" \
  "ContextMemoryRecallQualityGateReport" \
  "context memory recall quality gate report"
assert_file_contains "$hepta_core_recall_quality_fixture" \
  "ContextMemoryRecallQualityGateBlockerReason" \
  "context memory recall quality gate blocker reason enum"
assert_file_contains "$hepta_core_recall_quality_fixture" \
  "ContextMemoryRecallQualityFixtureGateReport" \
  "context memory recall quality gate fixture matrix report"
assert_file_contains "$hepta_core_recall_quality_fixture" \
  "blocking_reasons_for_fixture" \
  "context memory recall quality gate blocker reason derivation"
assert_file_contains "$hepta_core_recall_quality_report" \
  "pub fn from_shadow(shadow: &ContextMemoryAdaptiveAllocatorEvalShadowReport) -> Self" \
  "context memory recall quality gate shadow adapter"
assert_file_contains "$hepta_core_recall_quality_report" \
  "has_quality_gate_integrity" \
  "context memory recall quality gate integrity"
assert_file_contains "$hepta_core_recall_quality_report" \
  "ContextMemoryRecallQualityGateVerdict::GatePass" \
  "context memory recall quality gate pass verdict"
assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_recall_quality_gate_enforces_thresholds_without_activation" \
  "context memory recall quality gate hepta-core test"
assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_recall_quality_gate_fixture_matrix_blocks_regressions_without_activation" \
  "context memory recall quality gate regression matrix hepta-core test"
assert_file_contains "$hepta_core_memory_tests" \
  "context_memory_recall_quality_gate_delta_matrix_blocks_answer_quality_and_side_effects" \
  "context memory recall quality gate delta matrix hepta-core test"

assert_file_contains "$hepta_memory" \
  "mod context_plane_helpers" \
  "context memory recall quality gate hepta-memory helper module"
assert_file_contains "$hepta_memory_context_plane_helpers" \
  "context_memory_recall_quality_gate_report" \
  "context memory recall quality gate hepta-memory helper"
assert_file_contains "$hepta_memory_tests" \
  "store_snapshot_context_memory_recall_quality_gate_is_payload_light" \
  "context memory recall quality gate hepta-memory snapshot test"
assert_file_contains "$hepta_memory_tests" \
  "store_context_memory_recall_quality_gate_matches_snapshot_helper" \
  "context memory recall quality gate hepta-memory store test"

assert_file_contains "$debug_gate" \
  "hepta-context-memory-recall-quality-gate.sh" \
  "context memory recall quality gate debug gate"
assert_file_contains "$preflight_script" \
  "context memory recall quality gate" \
  "context memory recall quality gate preflight stage"
assert_file_contains "$front_door_gate" \
  "memory_recall_quality_gate_script" \
  "context memory recall quality gate front-door static check"

assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-adaptive-allocator-eval-shadow-gate.sh" \
  "hepta-context-memory-recall-quality-gate.sh" \
  "context memory recall quality gate debug gate order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-recall-quality-gate.sh" \
  "hepta-context-plane-status-report-gate.sh" \
  "context memory recall quality gate debug status order"
assert_line_before \
  "$preflight_script" \
  "context memory adaptive allocator eval shadow gate" \
  "context memory recall quality gate" \
  "context memory recall quality gate preflight order"
assert_line_before \
  "$preflight_script" \
  "context memory recall quality gate" \
  "context plane status/export report gate" \
  "context memory recall quality gate status preflight order"

cargo test --manifest-path "$manifest" -p hepta-core \
  recall_quality_gate \
  --test memory_recall_quality --message-format=short

cargo test --manifest-path "$manifest" -p hepta-memory \
  recall_quality_gate \
  --lib --message-format=short

echo "context-memory-recall-quality-gate=pass"
echo "context-memory-recall-quality-gate.verdict=gate_pass"
echo "context-memory-recall-quality-gate.schema-version=2"
echo "context-memory-recall-quality-gate.fixture-matrix=enabled"
echo "context-memory-recall-quality-gate.fixture-blocked-count=0"
echo "context-memory-recall-quality-gate.blocking-reason-count=0"
echo "context-memory-recall-quality-gate.missing-critical-fact-regressions=0"
echo "context-memory-recall-quality-gate.recall-regressions=0"
echo "context-memory-recall-quality-gate.precision-regressions=0"
echo "context-memory-recall-quality-gate.delta-matrix=enabled"
echo "context-memory-recall-quality-gate.missing-critical-fact-limit=2"
echo "context-memory-recall-quality-gate.runtime-activation=disabled"
echo "Hepta context memory recall quality gate passed"
