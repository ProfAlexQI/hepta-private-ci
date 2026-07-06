#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
hepta_memory_recall_query_helpers="$repo_root/codex-rs/hepta-memory/src/recall_helpers/query.rs"
hepta_memory_recall_snapshot_helpers="$repo_root/codex-rs/hepta-memory/src/recall_helpers/snapshot.rs"
hepta_memory_recall_core_tests="$repo_root/codex-rs/hepta-memory/src/tests/recall_context_core.rs"
context_manifest="$repo_root/codex-rs/core/src/context_manager/manifest.rs"
context_manifest_tests="$repo_root/codex-rs/core/src/context_manager/manifest/tests.rs"
context_manifest_selected_snippet="$repo_root/codex-rs/core/src/context_manager/manifest/selected_snippet.rs"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
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
  echo "hepta-context-memory-recall-manifest-payload-light-gate: $*" >&2
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
  "Memory recall manifest payload-light gate" \
  "recall snapshot -> recall query" \
  "context manifest chain" \
  "contain raw memory, transcript, control-record, selected snippet source" \
  "metadata, or prior prompt text" \
  "store_snapshot_recall_context_report_is_payload_light_across_query_boundaries" \
  "turn_context_manifest_resolves_recall_provider_rollup_without_payload_text" \
  "turn_context_manifest_resolves_selected_snippets_as_guarded_payload" \
  "hepta-context-memory-recall-manifest-payload-light-gate.sh" \
  "runtime-activation=disabled"; do
  assert_file_contains "$contracts" "$term" \
    "memory recall manifest payload-light contract"
done

assert_file_contains "$hepta_memory_recall_query_helpers" \
  "memory_record_is_recall_control" \
  "memory recall query control filter"
assert_file_contains "$hepta_memory_recall_query_helpers" \
  "MEMORY_RECALL_TOMBSTONE_MARKER" \
  "memory recall query tombstone marker"
assert_file_contains "$hepta_memory_recall_query_helpers" \
  "MEMORY_RECALL_CONFLICT_MARKER" \
  "memory recall query conflict marker"
assert_file_contains "$hepta_memory_recall_snapshot_helpers" \
  "memory_records_matching_recall_query(&self.memories, &memory_query.text)" \
  "memory recall snapshot query helper handoff"
assert_file_contains "$hepta_memory_recall_snapshot_helpers" \
  "pub fn recall_context_report" \
  "memory recall snapshot payload-light report helper"
assert_file_contains "$hepta_memory_recall_core_tests" \
  "store_snapshot_recall_context_report_is_payload_light_across_query_boundaries" \
  "memory recall payload-light report regression test"
assert_file_contains "$hepta_memory_recall_core_tests" \
  "MEMORY_RECALL_TOMBSTONE_MARKER" \
  "memory recall payload-light tombstone leak bait"
assert_file_contains "$hepta_memory_recall_core_tests" \
  "MEMORY_RECALL_CONFLICT_MARKER" \
  "memory recall payload-light conflict leak bait"
assert_file_contains "$context_manifest_tests" \
  "turn_context_manifest_resolves_recall_provider_rollup_without_payload_text" \
  "core manifest recall rollup no-payload test"
assert_file_contains "$context_manifest_tests" \
  "turn_context_manifest_resolves_selected_snippets_as_guarded_payload" \
  "core manifest selected snippet guarded payload test"
assert_file_contains "$context_manifest_selected_snippet" \
  "selected_snippet_envelope_is_manifest_safe" \
  "core manifest selected snippet safety guard"

assert_file_contains "$debug_gate" \
  "hepta-context-memory-recall-manifest-payload-light-gate.sh" \
  "memory recall manifest payload-light debug gate"
assert_file_contains "$preflight_script" \
  "context memory recall manifest payload-light gate" \
  "memory recall manifest payload-light preflight stage"
assert_file_contains "$front_door_gate" \
  "memory_recall_manifest_payload_light_gate_script" \
  "memory recall manifest payload-light front-door static check"
assert_file_contains "$release_manifest" \
  "scripts/hepta-context-memory-recall-manifest-payload-light-gate.sh" \
  "memory recall manifest payload-light release manifest entry"

assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-recall-helper-boundary-gate.sh" \
  "hepta-context-memory-recall-manifest-payload-light-gate.sh" \
  "memory recall manifest payload-light debug gate order"
assert_line_before \
  "$debug_gate" \
  "hepta-context-memory-recall-manifest-payload-light-gate.sh" \
  "hepta-context-memory-taxonomy-report-gate.sh" \
  "memory recall manifest payload-light debug taxonomy order"
assert_line_before \
  "$preflight_script" \
  "context memory recall helper boundary gate" \
  "context memory recall manifest payload-light gate" \
  "memory recall manifest payload-light preflight order"
assert_line_before \
  "$preflight_script" \
  "context memory recall manifest payload-light gate" \
  "context memory taxonomy report gate" \
  "memory recall manifest payload-light taxonomy preflight order"

cargo test --manifest-path "$manifest" -p hepta-memory \
  store_snapshot_recall_context_report_is_payload_light_across_query_boundaries \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-core \
  turn_context_manifest_resolves_recall_provider_rollup_without_payload_text \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-core \
  turn_context_manifest_resolves_selected_snippets_as_guarded_payload \
  --lib --message-format=short

echo "context-memory-recall-manifest-payload-light=pass"
echo "context-memory-recall-manifest-payload-light.memory-report=payload-light"
echo "context-memory-recall-manifest-payload-light.control-record-filter=pass"
echo "context-memory-recall-manifest-payload-light.manifest-rollup=no-payload"
echo "context-memory-recall-manifest-payload-light.selected-snippet=guarded"
echo "context-memory-recall-manifest-payload-light.runtime-activation=disabled"
echo "Hepta context memory recall manifest payload-light gate passed"
