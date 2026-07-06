#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
prompt_debug="$repo_root/codex-rs/core/src/prompt_debug.rs"
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
  echo "hepta-context-prompt-input-summary-gate: $*" >&2
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

gate_script="hepta-context-prompt-input-summary-gate.sh"
smoke_test="build_prompt_input_includes_context_and_user_message"
shadow_no_leak_test="build_prompt_input_from_session_consumes_context_manifest_without_shadow_leak"

echo "hepta-context-prompt-input-summary-gate: lane=$lane"
echo "hepta-context-prompt-input-summary-gate: CARGO_TARGET_DIR=$CARGO_TARGET_DIR"

for term in \
  "Prompt Input Gate" \
  "$shadow_no_leak_test" \
  "prompt-input.context-manifest=no-leak" \
  "prompt-input.live-selected-snippet=guarded" \
  "$gate_script"
do
  assert_file_contains "$contracts" "$term" "prompt-input context-manifest contract"
done

for term in \
  "$shadow_no_leak_test" \
  "build_prompt_input_from_session_with_manifest_options" \
  "TurnContextManifestOptions" \
  "ContextRecallProviderRollup" \
  "ContextRecallSelectedSnippetEnvelope" \
  "<selected_context_recall>" \
  "recall_selection" \
  "recall_selected_snippets" \
  "source-memory-id" \
  "[hepta-memory:"
do
  assert_file_contains "$prompt_debug" "$term" \
    "prompt-input context-manifest no-leak regression"
done

assert_file_contains "$debug_gate" "$gate_script" \
  "prompt-input context-manifest debug gate"
assert_file_contains "$preflight_script" "context prompt-input gate" \
  "prompt-input context-manifest preflight stage"
assert_file_contains "$front_door_gate" "$gate_script" \
  "prompt-input context-manifest front-door static coverage"
assert_file_contains "$release_manifest" "scripts/$gate_script" \
  "prompt-input context-manifest release manifest"

cargo test --manifest-path "$manifest" -p codex-core --test all \
  "$smoke_test" \
  --message-format=short

cargo test --manifest-path "$manifest" -p codex-core \
  "$shadow_no_leak_test" \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-core \
  selected_snippets \
  --lib --message-format=short

echo "prompt-input=pass"
echo "prompt-input.debug-smoke=pass"
echo "prompt-input.context-manifest=no-leak"
echo "prompt-input.live-selected-snippet=guarded"
echo "prompt-input.runtime-activation=disabled"
echo "Hepta context prompt-input gate passed"
