#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
runtime_query="$repo_root/codex-rs/hepta-runtime/src/query.rs"
runtime_turn_handoff="$repo_root/codex-rs/hepta-runtime/src/query/context_recall_turn_handoff.rs"
runtime_query_tests="$repo_root/codex-rs/hepta-runtime/src/query/tests.rs"
runtime_context_turn_ops="$repo_root/codex-rs/hepta-runtime/src/runtime_kernel/context_turn_ops.rs"
runtime_tool_support="$repo_root/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs"
runtime_tests="$repo_root/codex-rs/hepta-runtime/src/runtime_kernel/tests.rs"
session_tests="$repo_root/codex-rs/core/src/session/tests/contract_part_03.rs"
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
  echo "hepta-context-runtime-provider-rollup-manifest-handoff-gate: $*" >&2
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

gate_script="hepta-context-runtime-provider-rollup-manifest-handoff-gate.sh"
runtime_rollup_test="context_recall_provider_rollup_maps_runtime_recall_to_payload_light_counts"
runtime_turn_handoff_test="context_recall_turn_handoff_packages_rollup_and_opted_in_core_snippets"
runtime_native_handoff_test="native_turn_messages_with_context_recall_handoff_consumes_opted_in_runtime_handoff_without_leak"
core_combined_test="record_context_updates_and_set_reference_context_item_consumes_turn_scoped_recall_rollup_and_selected_snippets_without_drift"

echo "hepta-context-runtime-provider-rollup-manifest-handoff-gate: lane=$lane"
echo "hepta-context-runtime-provider-rollup-manifest-handoff-gate: CARGO_TARGET_DIR=$CARGO_TARGET_DIR"

for term in \
  "Runtime provider rollup manifest handoff gate" \
  "RuntimeContextRecallTurnHandoff" \
  "provider rollup and optional selected-snippet core envelope cannot drift" \
  "$runtime_rollup_test" \
  "$runtime_turn_handoff_test" \
  "$runtime_native_handoff_test" \
  "$core_combined_test" \
  "$gate_script"
do
  assert_file_contains "$contracts" "$term" \
    "runtime provider rollup manifest handoff contract"
done

for term in \
  "mod context_recall_turn_handoff" \
  "RuntimeContextRecallTurnHandoff" \
  "impl std::fmt::Debug for RuntimeContextRecallTurnHandoff"
do
  assert_file_contains "$runtime_query" "$term" \
    "runtime provider rollup single-slice handoff implementation"
done

for term in \
  "$runtime_rollup_test" \
  "$runtime_turn_handoff_test"
do
  assert_file_contains "$runtime_query_tests" "$term" \
    "runtime provider rollup single-slice handoff regression"
done

for term in \
  "fn build(" \
  "let provider_rollup = context_recall_provider_rollup::build(slice);" \
  "then(|| context_recall_selected_snippet_envelope::build(slice, query_text))" \
  "RuntimeContextRecallTurnHandoff"
do
  assert_file_contains "$runtime_turn_handoff" "$term" \
    "runtime provider rollup single-slice handoff implementation"
done

for term in \
  "native_turn_messages_with_context_recall_handoff" \
  "context_recall_turn_handoff(" \
  "native_selected_snippet_prompt_count"
do
  assert_file_contains "$runtime_context_turn_ops" "$term" \
    "runtime native selected-snippet handoff implementation"
done

for term in \
  "native_selected_snippet_prompt_count" \
  "query_payload"
do
  assert_file_contains "$runtime_tool_support" "$term" \
    "runtime native selected-snippet handoff implementation"
done

for term in \
  "$runtime_native_handoff_test" \
  "selected_context_recall_block" \
  "source_memory_ids" \
  "query_payload"
do
  assert_file_contains "$runtime_tests" "$term" \
    "runtime native selected-snippet handoff regression"
done

for term in \
  "$core_combined_test" \
  "turn_context.extension_data.insert(recall_selection.clone())" \
  ".extension_data" \
  ".insert(selected_snippets.clone())" \
  "refreshed_recall_selection" \
  "matches(\"<selected_context_recall>\")" \
  "history_after_combined" \
  "source-memory-id" \
  "[hepta-memory:" \
  "needle"
do
  assert_file_contains "$session_tests" "$term" \
    "core session combined recall rollup selected-snippet handoff regression"
done

assert_file_contains "$debug_gate" "$gate_script" \
  "runtime provider rollup manifest handoff debug gate"
assert_file_contains "$preflight_script" \
  "runtime provider rollup manifest handoff gate" \
  "runtime provider rollup manifest handoff preflight stage"
assert_file_contains "$front_door_gate" "$gate_script" \
  "runtime provider rollup manifest handoff front-door static coverage"
assert_file_contains "$release_manifest" "scripts/$gate_script" \
  "runtime provider rollup manifest handoff release manifest"

assert_line_before "$preflight_script" \
  "hepta-runtime selected snippet turn handoff fixtures" \
  "runtime provider rollup manifest handoff gate" \
  "runtime provider rollup manifest handoff preflight order"
assert_line_before "$preflight_script" \
  "runtime provider rollup manifest handoff gate" \
  "hepta-runtime native selected snippet request assembly fixtures" \
  "runtime provider rollup manifest handoff preflight order"
assert_line_before "$debug_gate" \
  "hepta-context-selected-snippet-live-prompt-compression-gate.sh" \
  "$gate_script" \
  "runtime provider rollup manifest handoff debug gate order"

cargo test --manifest-path "$manifest" -p hepta-runtime \
  "$runtime_rollup_test" \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-runtime \
  "$runtime_turn_handoff_test" \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p hepta-runtime \
  "$runtime_native_handoff_test" \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-core \
  "$core_combined_test" \
  --lib --message-format=short

echo "runtime-provider-rollup-manifest-handoff=pass"
echo "runtime-provider-rollup-manifest-handoff.provider-rollup=payload-light"
echo "runtime-provider-rollup-manifest-handoff.turn-handoff=single-slice"
echo "runtime-provider-rollup-manifest-handoff.selected-snippet=guarded"
echo "runtime-provider-rollup-manifest-handoff.live-prompt=no-duplicate"
echo "runtime-provider-rollup-manifest-handoff.runtime-activation=disabled"
echo "Hepta context runtime provider rollup manifest handoff gate passed"
