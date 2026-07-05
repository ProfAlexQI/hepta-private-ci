#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
lane="${HEPTA_CARGO_LANE:-${HEPTA_LANE:-hepta-context}}"
target_root="${HEPTA_CARGO_TARGET_ROOT:-$HOME/.openclaw/tmp/cargo-targets}"
target_leaf="$lane"
if [[ "$target_leaf" != hepta-* ]]; then
  target_leaf="hepta-$target_leaf"
fi
export CARGO_TARGET_DIR="${HEPTA_CARGO_TARGET_DIR:-$target_root/$target_leaf}"
export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"

fail() {
  echo "hepta-context-selected-snippet-live-prompt-compression-gate: $*" >&2
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

contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
session_tests="$repo_root/codex-rs/core/src/session/tests.rs"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
release_manifest="$repo_root/codex-rs/CONTEXT_LANE_RELEASE_MANIFEST.tsv"
gate_script="hepta-context-selected-snippet-live-prompt-compression-gate.sh"
new_test="record_context_updates_and_set_reference_context_item_rejects_prompt_unsafe_selected_snippets_under_source_aware_compression_opt_in"
safe_opt_in_test="record_context_updates_and_set_reference_context_item_honors_turn_scoped_source_aware_compression_opt_in"
unsafe_user_turn_test="user_input_with_turn_context_selected_snippets_reject_prompt_unsafe_payload"

echo "hepta-context-selected-snippet-live-prompt-compression-gate: lane=$lane"
echo "hepta-context-selected-snippet-live-prompt-compression-gate: CARGO_TARGET_DIR=$CARGO_TARGET_DIR"

for term in \
  "Selected-snippet live prompt compression gate" \
  "$new_test" \
  "$safe_opt_in_test" \
  "$unsafe_user_turn_test" \
  "$gate_script"
do
  assert_file_contains "$contracts" "$term" "selected-snippet live prompt compression contract"
done

for term in \
  "$new_test" \
  "unsafe-selected-snippet-live-compression-bait" \
  "Feature::SourceAwareCompressionCanary" \
  "insert_source_aware_compression_policy_opt_in_marker" \
  "TurnContextCompressionStageKind::Defragment" \
  "TurnContextCompressionStageKind::Prune" \
  "[context defragmented for budget]" \
  "[context pruned for budget]" \
  "[context summarized for budget]" \
  "<selected_context_recall>"
do
  assert_file_contains "$session_tests" "$term" "selected-snippet live prompt compression core test"
done

assert_file_contains \
  "$debug_gate" \
  "$gate_script" \
  "selected-snippet live prompt compression debug gate"

assert_file_contains \
  "$preflight_script" \
  "selected-snippet live prompt compression gate" \
  "selected-snippet live prompt compression preflight stage"

assert_file_contains \
  "$front_door_gate" \
  "$gate_script" \
  "selected-snippet live prompt compression front-door static coverage"

assert_file_contains \
  "$release_manifest" \
  "scripts/$gate_script" \
  "selected-snippet live prompt compression release manifest"

assert_line_before \
  "$preflight_script" \
  "core session source-aware prompt multi-compression explicit opt-in fixture" \
  "selected-snippet live prompt compression gate" \
  "selected-snippet live prompt compression preflight stage order"

assert_line_before \
  "$preflight_script" \
  "selected-snippet live prompt compression gate" \
  "core context contribution ledger settings-diff fixture" \
  "selected-snippet live prompt compression preflight stage order"

cargo test --manifest-path "$manifest" -p codex-core \
  "$new_test" \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-core \
  "$safe_opt_in_test" \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-core \
  "$unsafe_user_turn_test" \
  --lib --message-format=short

echo "selected-snippet-live-prompt-compression=pass"
echo "selected-snippet-live-prompt-compression.unsafe-snippet=blocked"
echo "selected-snippet-live-prompt-compression.compression-chain=defragment-prune"
echo "selected-snippet-live-prompt-compression.live-prompt=no-leak"
echo "selected-snippet-live-prompt-compression.runtime-activation=disabled"
