#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/codex-rs/Cargo.toml"
protocol_root="$repo_root/codex-rs/protocol/src"
protocol_op="$protocol_root/protocol.rs"
protocol_tests="$protocol_root/protocol/tests.rs"
app_server_protocol_turn="$repo_root/codex-rs/app-server-protocol/src/protocol/v2/turn.rs"
app_server_protocol_tests="$repo_root/codex-rs/app-server-protocol/src/protocol/v2/tests.rs"
app_server_turn_processor="$repo_root/codex-rs/app-server/src/request_processors/turn_processor.rs"
app_server_turn_tests="$repo_root/codex-rs/app-server/tests/suite/v2/turn_start.rs"
app_server_experimental_tests="$repo_root/codex-rs/app-server/tests/suite/v2/experimental_api.rs"
tui_app_server_session="$repo_root/codex-rs/tui/src/app_server_session.rs"
tui_app_command="$repo_root/codex-rs/tui/src/app_command.rs"
exec_lib="$repo_root/codex-rs/exec/src/lib.rs"
exec_lib_tests="$repo_root/codex-rs/exec/src/lib_tests.rs"
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
skip_runtime_stages="${HEPTA_CONTEXT_PREFLIGHT_SKIP_RUNTIME:-0}"

case "$skip_runtime_stages" in
  0 | 1)
    ;;
  *)
    echo "hepta-context-selected-snippet-api-surface-gate: HEPTA_CONTEXT_PREFLIGHT_SKIP_RUNTIME must be 0 or 1, got $skip_runtime_stages" >&2
    exit 1
    ;;
esac

fail() {
  echo "hepta-context-selected-snippet-api-surface-gate: $*" >&2
  exit 1
}

run_runtime_stage() {
  local label="$1"
  shift
  if [ "$skip_runtime_stages" = "1" ]; then
    echo "skip: $label (HEPTA_CONTEXT_PREFLIGHT_SKIP_RUNTIME=1)"
    return 0
  fi
  "$@"
}

assert_file_contains() {
  local file_path="$1"
  local needle="$2"
  local label="$3"

  if ! grep -F "$needle" "$file_path" >/dev/null; then
    fail "$label must contain: $needle"
  fi
}

assert_unique_protocol_owner() {
  local pattern="$1"
  local expected_owner="$2"
  local label="$3"
  local owners

  owners="$(rg -l "$pattern" "$protocol_root" -g '*.rs' || true)"
  if [[ "$(printf '%s\n' "$owners" | sed '/^$/d' | wc -l | tr -d ' ')" != "1" \
    || "$owners" != "$expected_owner" ]]; then
    fail "$label must have exactly one protocol owner: $expected_owner"
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

gate_script="hepta-context-selected-snippet-api-surface-gate.sh"
surface_audit_script="hepta-context-selected-snippet-surface-audit.sh"
protocol_request_filter="user_input_with_turn_context_"
app_server_protocol_roundtrip_test="turn_start_params_round_trip_context_recall_selected_snippets"
app_server_protocol_conversion_filter="context_recall_selected_snippets_from_core"
tui_helper_filter="context_recall_selected_snippets_for_turn_start"
tui_no_log_test="user_turn_selected_snippets_are_not_serialized"
exec_helper_filter="context_recall_selected_snippets_for_turn_start"
app_server_handoff_filter="context_recall_selected_snippets_v2"
app_server_experimental_test="turn_start_context_recall_selected_snippets_requires_experimental_api_capability"
app_server_history_no_leak_test="turn_start_source_aware_compression_canary_thread_history_hides_routing_metadata"

echo "hepta-context-selected-snippet-api-surface-gate: lane=$lane"
echo "hepta-context-selected-snippet-api-surface-gate: CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
echo "hepta-context-selected-snippet-api-surface-gate: skip-runtime-stages=$skip_runtime_stages"

for term in \
  "Selected-snippet API/caller surface gate" \
  "app-server/TUI/exec caller surfaces" \
  "selected-snippet-api-surface=pass" \
  "$surface_audit_script" \
  "$gate_script" \
  "$protocol_request_filter" \
  "$app_server_protocol_roundtrip_test" \
  "$app_server_protocol_conversion_filter" \
  "$tui_helper_filter" \
  "$tui_no_log_test" \
  "$exec_helper_filter" \
  "$app_server_handoff_filter" \
  "$app_server_experimental_test" \
  "$app_server_history_no_leak_test"
do
  assert_file_contains "$contracts" "$term" \
    "selected-snippet API/caller surface contract"
done

for term in \
  "user_input_with_turn_context_deserializes_without_selected_snippet_handoff" \
  "user_input_with_turn_context_serializes_selected_snippet_handoff" \
  "context_recall_selected_snippets: Some(selected_snippets.clone())" \
  ".get(\"source_id\")" \
  "selected_snippets.has_shadow_integrity()"
do
  assert_file_contains "$protocol_tests" "$term" \
    "core protocol selected-snippet request contract"
done
assert_file_contains "$protocol_op" \
  "context_recall_selected_snippets: Option<TurnContextRecallSelectedSnippetEnvelope>" \
  "core protocol selected-snippet request field"
assert_unique_protocol_owner \
  '^        context_recall_selected_snippets: Option<TurnContextRecallSelectedSnippetEnvelope>,' \
  "$protocol_op" \
  "core protocol selected-snippet request field"
assert_unique_protocol_owner \
  '^fn user_input_with_turn_context_deserializes_without_selected_snippet_handoff\(' \
  "$protocol_tests" \
  "core protocol selected-snippet absent handoff test"
assert_unique_protocol_owner \
  '^fn user_input_with_turn_context_serializes_selected_snippet_handoff\(' \
  "$protocol_tests" \
  "core protocol selected-snippet serialized handoff test"

for term in \
  "pub context_recall_selected_snippets: Option<ContextRecallSelectedSnippetEnvelope>" \
  "from_core_for_experimental_client" \
  ".filter(CoreTurnContextRecallSelectedSnippetEnvelope::has_shadow_integrity)" \
  ".map(Self::from_core)"
do
  assert_file_contains "$app_server_protocol_turn" "$term" \
    "app-server v2 selected-snippet turn/start protocol"
done

for term in \
  "$app_server_protocol_roundtrip_test" \
  "$app_server_protocol_conversion_filter" \
  "turn/start.contextRecallSelectedSnippets" \
  "selected_snippet_count" \
  "source_id"
do
  assert_file_contains "$app_server_protocol_tests" "$term" \
    "app-server v2 selected-snippet protocol tests"
done

for term in \
  "contextRecallSelectedSnippets must be bounded and source-safe" \
  "context_recall_selected_snippets.is_some()" \
  "Op::UserInputWithTurnContext" \
  "context_recall_selected_snippets,"
do
  assert_file_contains "$app_server_turn_processor" "$term" \
    "app-server selected-snippet request handoff"
done

for term in \
  "turn_start_accepts_context_recall_selected_snippets_v2" \
  "turn_start_rejects_invalid_context_recall_selected_snippets_v2" \
  "$app_server_history_no_leak_test" \
  "assert_no_source_aware_compression_routing_metadata" \
  "source-memory-id" \
  "[hepta-memory:"
do
  assert_file_contains "$app_server_turn_tests" "$term" \
    "app-server selected-snippet turn/start handoff tests"
done

assert_file_contains "$app_server_experimental_tests" "$app_server_experimental_test" \
  "app-server selected-snippet experimental API test"
assert_file_contains "$app_server_experimental_tests" \
  "turn/start.contextRecallSelectedSnippets" \
  "app-server selected-snippet experimental API reason"

for term in \
  "CONTEXT_RECALL_SELECTED_SNIPPETS_EXPERIMENTAL_API_ENABLED" \
  "context_recall_selected_snippets_for_turn_start_with_opt_in" \
  "from_core_for_experimental_client" \
  "context_recall_selected_snippets_for_turn_start_maps_valid_opted_in_envelope" \
  "context_recall_selected_snippets_for_turn_start_requires_opt_in_and_integrity"
do
  assert_file_contains "$tui_app_server_session" "$term" \
    "TUI selected-snippet turn/start helper"
done

for term in \
  "$tui_no_log_test" \
  "context_recall_selected_snippets" \
  "contextRecallSelectedSnippets" \
  "bounded memory" \
  "snippetHash"
do
  assert_file_contains "$tui_app_command" "$term" \
    "TUI selected-snippet outbound log no-leak test"
done

for term in \
  "CONTEXT_RECALL_SELECTED_SNIPPETS_EXPERIMENTAL_API_ENABLED" \
  "context_recall_selected_snippets_for_turn_start_with_opt_in" \
  "from_core_for_experimental_client"
do
  assert_file_contains "$exec_lib" "$term" \
    "exec selected-snippet turn/start helper"
done

for term in \
  "context_recall_selected_snippets_for_turn_start_maps_valid_opted_in_envelope" \
  "context_recall_selected_snippets_for_turn_start_requires_opt_in_and_integrity" \
  "[redacted-query] bounded memory"
do
  assert_file_contains "$exec_lib_tests" "$term" \
    "exec selected-snippet helper tests"
done

assert_file_contains "$debug_gate" "$gate_script" \
  "selected-snippet API/caller surface debug gate"
assert_file_contains "$preflight_script" \
  "selected snippet API/caller surface gate" \
  "selected-snippet API/caller surface preflight stage"
assert_file_contains "$front_door_gate" "$gate_script" \
  "selected-snippet API/caller surface front-door static coverage"
assert_file_contains "$release_manifest" "scripts/$gate_script" \
  "selected-snippet API/caller surface release manifest"

assert_line_before "$preflight_script" \
  "selected snippet default surface/schema audit" \
  "selected snippet API/caller surface gate" \
  "selected-snippet API/caller surface preflight order"
assert_line_before "$preflight_script" \
  "selected snippet API/caller surface gate" \
  "native gateway context-recall worker scheduler route fixture" \
  "selected-snippet API/caller surface preflight order"
assert_line_before "$debug_gate" \
  "hepta-context-runtime-provider-rollup-manifest-handoff-gate.sh" \
  "$gate_script" \
  "selected-snippet API/caller surface debug gate order"

bash "$repo_root/scripts/$surface_audit_script"

cargo test --manifest-path "$manifest" -p codex-protocol \
  "$protocol_request_filter" \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-app-server-protocol \
  "$app_server_protocol_roundtrip_test" \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-app-server-protocol \
  "$app_server_protocol_conversion_filter" \
  --lib --message-format=short

run_runtime_stage "codex-tui selected-snippet opt-in helper fixture" \
  cargo test --manifest-path "$manifest" -p codex-tui \
  "$tui_helper_filter" \
  --lib --message-format=short

run_runtime_stage "codex-tui selected-snippet outbound command no-log fixture" \
  cargo test --manifest-path "$manifest" -p codex-tui \
  "$tui_no_log_test" \
  --lib --message-format=short

cargo test --manifest-path "$manifest" -p codex-exec \
  "$exec_helper_filter" \
  --lib --message-format=short

run_runtime_stage "codex-app-server selected-snippet turn-start handoff fixture" \
  cargo test --manifest-path "$manifest" -p codex-app-server \
  "$app_server_handoff_filter" \
  --test all --message-format=short -- --test-threads=1

run_runtime_stage "codex-app-server selected-snippet experimental opt-in fixture" \
  cargo test --manifest-path "$manifest" -p codex-app-server \
  "$app_server_experimental_test" \
  --test all --message-format=short

run_runtime_stage "codex-app-server source-aware compression thread-history no-leak fixture" \
  cargo test --manifest-path "$manifest" -p codex-app-server \
  "$app_server_history_no_leak_test" \
  --test all --message-format=short

echo "selected-snippet-api-surface=pass"
echo "selected-snippet-api-surface.default-surface=audit-pass"
echo "selected-snippet-api-surface.protocol=typed-request-handoff"
echo "selected-snippet-api-surface.app-server=experimental-turn-start-only"
echo "selected-snippet-api-surface.tui=opt-in-no-log"
echo "selected-snippet-api-surface.exec=opt-in"
echo "selected-snippet-api-surface.history=no-routing-metadata"
echo "selected-snippet-api-surface.runtime-activation=disabled"
if [ "$skip_runtime_stages" = "1" ]; then
  echo "selected-snippet-api-surface.runtime-coupled-fixtures=skipped"
fi
echo "Hepta context selected-snippet API/caller surface gate passed"
