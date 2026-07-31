#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

fail() {
  echo "hepta-context-selected-snippet-surface-audit: $*" >&2
  exit 1
}

assert_schema_properties_hide_selected_snippets() {
  local schema_path="$1"
  local properties_path="$2"
  local label="$3"
  local selected_snippet_property

  if ! jq -e "$properties_path" "$schema_path" >/dev/null; then
    fail "$label properties path is missing in $schema_path"
  fi

  selected_snippet_property="$(
    jq -r \
      "$properties_path | keys[] | select(test(\"contextRecallSelectedSnippets|context_recall_selected_snippets|ContextRecallSelectedSnippet|selectedSnippet|selected_snippet\"))" \
      "$schema_path" \
      | head -n 1
  )"
  if [ -n "$selected_snippet_property" ]; then
    fail "$label schema exposes selected-snippet property $selected_snippet_property in $schema_path"
  fi
}

assert_schema_fragment_hide_selected_snippet_markers() {
  local schema_path="$1"
  local fragment_path="$2"
  local label="$3"

  if ! jq -e "$fragment_path" "$schema_path" >/dev/null; then
    fail "$label schema fragment is missing in $schema_path"
  fi

  if jq -e \
    "$fragment_path | tostring | test(\"contextRecallSelectedSnippets|context_recall_selected_snippets|ContextRecallSelectedSnippet|selectedSnippet|selected_snippet\")" \
    "$schema_path" >/dev/null; then
    fail "$label schema references selected-snippet handoff in $schema_path"
  fi
}

array_contains() {
  local needle="$1"
  shift
  local value

  for value in "$@"; do
    if [ "$value" = "$needle" ]; then
      return 0
    fi
  done

  return 1
}

assert_schema_marker_definitions_only_allowlist() {
  local schema_path="$1"
  local definitions_path="$2"
  local label="$3"
  shift 3
  local allowed_definitions=("$@")
  local definition_name

  if ! jq -e "$definitions_path" "$schema_path" >/dev/null; then
    fail "$label definitions path is missing in $schema_path"
  fi

  while IFS= read -r definition_name; do
    if ! array_contains "$definition_name" "${allowed_definitions[@]}"; then
      fail "$label definition $definition_name references selected-snippet handoff in $schema_path"
    fi
  done < <(
    jq -r \
      "$definitions_path | to_entries[] | select((.key | test(\"ContextRecallSelectedSnippet|selectedSnippet|selected_snippet\")) or (.value | tostring | test(\"contextRecallSelectedSnippets|context_recall_selected_snippets|ContextRecallSelectedSnippet|selectedSnippet|selected_snippet\"))) | .key" \
      "$schema_path" \
      | sort -u
  )
}

assert_text_contains() {
  local haystack="$1"
  local needle="$2"
  local label="$3"

  if [[ "$haystack" != *"$needle"* ]]; then
    fail "$label is missing required text: $needle"
  fi
}

assert_text_not_contains() {
  local haystack="$1"
  local needle="$2"
  local label="$3"

  if [[ "$haystack" == *"$needle"* ]]; then
    fail "$label must not contain text: $needle"
  fi
}

assert_text_not_matches() {
  local haystack="$1"
  local pattern="$2"
  local label="$3"

  if printf '%s\n' "$haystack" | grep -E "$pattern" >/dev/null; then
    fail "$label must not match regex: $pattern"
  fi
}

assert_file_fixed_occurrence_count() {
  local file_path="$1"
  local needle="$2"
  local expected_count="$3"
  local label="$4"
  local actual_count

  actual_count="$(
    { rg -o --fixed-strings "$needle" "$file_path" || true; } \
      | wc -l \
      | tr -d '[:space:]'
  )"

  if [ "$actual_count" != "$expected_count" ]; then
    fail "$label expected $expected_count occurrences of $needle, found $actual_count"
  fi
}

assert_rust_struct_fields_match() {
  local file_path="$1"
  local struct_name="$2"
  local label="$3"
  shift 3
  local expected_sorted
  local actual_sorted

  expected_sorted="$(
    printf '%s\n' "$@" \
      | sort
  )"
  actual_sorted="$(
    awk -v struct_name="$struct_name" '
      $0 ~ "^struct " struct_name " \\{" {
        in_struct = 1
        next
      }
      in_struct && /^}/ {
        exit
      }
      in_struct {
        line = $0
        sub(/^[[:space:]]*/, "", line)
        if (line ~ /^[A-Za-z_][A-Za-z0-9_]*:/) {
          sub(/:.*/, "", line)
          print line
        }
      }
    ' "$file_path" \
      | sort
  )"

  if [ "$actual_sorted" != "$expected_sorted" ]; then
    fail "$label field allowlist mismatch; expected $(printf '%s' "$expected_sorted" | tr '\n' ','), got $(printf '%s' "$actual_sorted" | tr '\n' ',')"
  fi
}

assert_rust_struct_header_match() {
  local file_path="$1"
  local struct_name="$2"
  local label="$3"
  local expected_header
  local actual_header

  expected_header="$(
    printf '%s\n%s\n' \
      "#[derive(Debug, Serialize)]" \
      "struct $struct_name {"
  )"
  actual_header="$(
    awk -v struct_name="$struct_name" '
      /^[[:space:]]*#\[/ {
        attrs[++attr_count] = $0
        next
      }
      $0 == "struct " struct_name " {" {
        for (i = 1; i <= attr_count; i++) {
          print attrs[i]
        }
        print $0
        exit
      }
      {
        delete attrs
        attr_count = 0
      }
    ' "$file_path"
  )"

  if [ "$actual_header" != "$expected_header" ]; then
    fail "$label struct header must be exactly #[derive(Debug, Serialize)] with no serde rename/skip/flatten attributes"
  fi
}

assert_rust_const_string_assignment_match() {
  local file_path="$1"
  local const_name="$2"
  local expected_value="$3"
  local label="$4"
  local actual_value

  actual_value="$(
    awk -v const_name="$const_name" '
      $0 ~ "^[[:space:]]*(pub(\\([^)]*\\))?[[:space:]]+)?const[[:space:]]+" const_name ": &str =" {
        line = $0
        if (line !~ /"[^"]*";/) {
          getline line
        }
        if (match(line, /"[^"]*"/)) {
          print substr(line, RSTART + 1, RLENGTH - 2)
        }
        exit
      }
    ' "$file_path"
  )"

  if [ "$actual_value" != "$expected_value" ]; then
    fail "$label const $const_name expected $expected_value, got $actual_value"
  fi
}

assert_rust_function_signature_match() {
  local file_path="$1"
  local function_name="$2"
  local expected_signature="$3"
  local label="$4"
  local expected_normalized
  local actual_normalized

  expected_normalized="$(printf '%s\n' "$expected_signature" | tr -d '[:space:]')"
  actual_normalized="$(
    awk -v function_name="$function_name" '
      $0 ~ "^[[:space:]]*(pub[[:space:]]+)?(async[[:space:]]+)?fn[[:space:]]+" function_name "[[:space:]]*\\(" {
        in_signature = 1
      }
      in_signature {
        line = $0
        sub(/\/\/.*/, "", line)
        printf "%s", line
        if (line ~ /\{/) {
          exit
        }
      }
    ' "$file_path" \
      | tr -d '[:space:]'
  )"

  if [ "$actual_normalized" != "$expected_normalized" ]; then
    fail "$label function signature mismatch; expected $expected_normalized, got $actual_normalized"
  fi
}

assert_report_entrypoints_match() {
  local report_text="$1"
  local label="$2"
  shift 2
  local expected_sorted
  local actual_sorted

  expected_sorted="$(
    printf '%s\n' "$@" \
      | sort
  )"
  actual_sorted="$(
    printf '%s\n' "$report_text" \
      | awk '
        /allowed_runtime_entrypoints: &\[/ { in_array = 1; next }
        in_array && /\],/ { exit }
        in_array { print }
      ' \
      | sed -n 's/^[[:space:]]*"\([^"]*\)",[[:space:]]*$/\1/p' \
      | sort
  )"

  if [ "$actual_sorted" != "$expected_sorted" ]; then
    fail "$label entrypoint allowlist mismatch; expected $(printf '%s' "$expected_sorted" | tr '\n' ','), got $(printf '%s' "$actual_sorted" | tr '\n' ',')"
  fi
}

assert_report_entrypoint_source_block_match() {
  local report_text="$1"
  local label="$2"
  shift 2
  local expected_block
  local actual_block

  expected_block="$(
    printf '%s\n' "$@"
  )"
  actual_block="$(
    printf '%s\n' "$report_text" \
      | awk '
        /allowed_runtime_entrypoints: &\[/ { in_array = 1 }
        in_array {
          line = $0
          sub(/^[[:space:]]*/, "", line)
          sub(/[[:space:]]*$/, "", line)
          print line
        }
        in_array && /\],/ { exit }
      '
  )"

  if [ "$actual_block" != "$expected_block" ]; then
    fail "$label entrypoint source block mismatch; expected $(printf '%s' "$expected_block" | tr '\n' ','), got $(printf '%s' "$actual_block" | tr '\n' ',')"
  fi
}

assert_trimmed_text_block_match() {
  local block_text="$1"
  local label="$2"
  shift 2
  local expected_block
  local actual_block

  expected_block="$(
    printf '%s\n' "$@"
  )"
  actual_block="$(
    printf '%s\n' "$block_text" \
      | sed 's/^[[:space:]]*//; s/[[:space:]]*$//'
  )"

  if [ "$actual_block" != "$expected_block" ]; then
    fail "$label source block mismatch; expected $(printf '%s' "$expected_block" | tr '\n' ','), got $(printf '%s' "$actual_block" | tr '\n' ',')"
  fi
}

assert_report_blockers_match() {
  local report_text="$1"
  local label="$2"
  shift 2
  local expected_sorted
  local actual_sorted

  expected_sorted="$(
    printf '%s\n' "$@" \
      | sort
  )"
  actual_sorted="$(
    printf '%s\n' "$report_text" \
      | sed -n 's/^[[:space:]]*blockers\.push("\([^"]*\)");[[:space:]]*$/\1/p' \
      | sort
  )"

  if [ "$actual_sorted" != "$expected_sorted" ]; then
    fail "$label blocker allowlist mismatch; expected $(printf '%s' "$expected_sorted" | tr '\n' ','), got $(printf '%s' "$actual_sorted" | tr '\n' ',')"
  fi
}

assert_report_scalar_assignments_match() {
  local report_text="$1"
  local label="$2"
  shift 2
  local expected_sorted
  local actual_sorted

  expected_sorted="$(
    printf '%s\n' "$@" \
      | sort
  )"
  actual_sorted="$(
    printf '%s\n' "$report_text" \
      | sed -n 's/^[[:space:]]*\([A-Za-z_][A-Za-z0-9_]*\): \(.*\),[[:space:]]*$/\1=\2/p' \
      | sort
  )"

  if [ "$actual_sorted" != "$expected_sorted" ]; then
    fail "$label scalar assignment allowlist mismatch; expected $(printf '%s' "$expected_sorted" | tr '\n' ','), got $(printf '%s' "$actual_sorted" | tr '\n' ',')"
  fi
}

assert_text_lines_containing_match() {
  local haystack="$1"
  local marker="$2"
  local label="$3"
  shift 3
  local expected_sorted
  local actual_sorted

  expected_sorted="$(
    printf '%s\n' "$@" \
      | sort
  )"
  actual_sorted="$(
    printf '%s\n' "$haystack" \
      | grep -F "$marker" \
      | sed 's/^[[:space:]]*//; s/[[:space:]]*$//' \
      | sort
  )"

  if [ "$actual_sorted" != "$expected_sorted" ]; then
    fail "$label marker $marker allowlist mismatch; expected $(printf '%s' "$expected_sorted" | tr '\n' ','), got $(printf '%s' "$actual_sorted" | tr '\n' ',')"
  fi
}

assert_context_gate_target_dir_contract() {
  local script_path="$1"
  local label="$2"
  local script_text

  script_text="$(<"$script_path")"

  assert_text_contains \
    "$script_text" \
    'target_leaf="$lane"' \
    "$label target-dir contract"
  assert_text_contains \
    "$script_text" \
    'if [[ "$target_leaf" != hepta-* ]]; then' \
    "$label target-dir contract"
  assert_text_contains \
    "$script_text" \
    'target_leaf="hepta-$target_leaf"' \
    "$label target-dir contract"
  assert_text_contains \
    "$script_text" \
    'export CARGO_TARGET_DIR="${HEPTA_CARGO_TARGET_DIR:-$target_root/$target_leaf}"' \
    "$label target-dir contract"
  assert_text_not_contains \
    "$script_text" \
    'export CARGO_TARGET_DIR="${HEPTA_CARGO_TARGET_DIR:-$target_root/hepta-$lane}"' \
    "$label target-dir contract"
}

turn_protocol_source="$repo_root/codex-rs/app-server-protocol/src/protocol/v2/turn.rs"
app_server_readme="$repo_root/codex-rs/app-server/README.md"
native_gateway_root="$repo_root/codex-rs/hepta-native-gateway/src/native_gateway.rs"
native_gateway_source="$(mktemp -t hepta-selected-snippet-native-gateway-source.XXXXXX)"
cat \
  "$repo_root/codex-rs/hepta-native-gateway/src/route_registry.rs" \
  "$repo_root/codex-rs/hepta-native-gateway/src/native_gateway/report_types.rs" \
  "$repo_root/codex-rs/hepta-native-gateway/src/native_gateway/release_ui_reports.rs" \
  "$repo_root/codex-rs/hepta-native-gateway/src/native_gateway/report_registry.rs" \
  "$native_gateway_root" \
  >"$native_gateway_source"
trap 'rm -f "$native_gateway_source"' EXIT
context_gate_cargo_scripts=(
  "$repo_root/scripts/hepta-context-preflight.sh:context preflight"
  "$repo_root/scripts/hepta-context-response-debug-export-gate.sh:response-debug export gate"
  "$repo_root/scripts/hepta-context-prompt-input-summary-gate.sh:prompt-input summary gate"
)
readme_selected_snippet_marker_pattern='contextRecallSelectedSnippets|ContextRecallSelectedSnippet|selectedSnippet|selected_snippet|selected[- ]snippet|selected[- ]recall|recall[- ]snippets?'
stable_turn_start_ts="$repo_root/codex-rs/app-server-protocol/schema/typescript/v2/TurnStartParams.ts"
stable_turn_start_schema_paths=(
  "$repo_root/codex-rs/app-server-protocol/schema/json/v2/TurnStartParams.json:.properties"
  "$repo_root/codex-rs/app-server-protocol/schema/json/ClientRequest.json:.definitions.TurnStartParams.properties"
  "$repo_root/codex-rs/app-server-protocol/schema/json/codex_app_server_protocol.schemas.json:.definitions.v2.TurnStartParams.properties"
  "$repo_root/codex-rs/app-server-protocol/schema/json/codex_app_server_protocol.v2.schemas.json:.definitions.TurnStartParams.properties"
)
selected_snippet_json_marker_allowed_paths=(
  "$repo_root/codex-rs/app-server-protocol/schema/json/ClientRequest.json"
  "$repo_root/codex-rs/app-server-protocol/schema/json/codex_app_server_protocol.schemas.json"
  "$repo_root/codex-rs/app-server-protocol/schema/json/codex_app_server_protocol.v2.schemas.json"
  "$repo_root/codex-rs/app-server-protocol/schema/json/v2/TurnStartParams.json"
)
stable_typescript_request_paths=(
  "$repo_root/codex-rs/app-server-protocol/schema/typescript/ClientRequest.ts"
  "$repo_root/codex-rs/app-server-protocol/schema/typescript/index.ts"
)
selected_snippet_app_server_protocol_source_marker_allowed_paths=(
  "$repo_root/codex-rs/app-server-protocol/src/protocol/v2/tests.rs"
  "$repo_root/codex-rs/app-server-protocol/src/protocol/v2/turn.rs"
)
selected_snippet_app_server_protocol_source_marker_paths=()
while IFS= read -r source_path; do
  selected_snippet_app_server_protocol_source_marker_paths+=("$source_path")
done < <(
  rg -l \
    'contextRecallSelectedSnippets|context_recall_selected_snippets|ContextRecallSelectedSnippet|selectedSnippet|selected_snippet|selected[- ]snippet|selected[- ]recall|recall[- ]snippets?' \
    "$repo_root/codex-rs/app-server-protocol/src" \
    | sort
)
selected_snippet_core_protocol_source_marker_allowed_paths=(
  "$repo_root/codex-rs/core/src/context_manager/controller.rs"
  "$repo_root/codex-rs/core/src/context_manager/history_tests.rs"
  "$repo_root/codex-rs/core/src/context_manager/manifest.rs"
  "$repo_root/codex-rs/core/src/context_manager/manifest/classification.rs"
  "$repo_root/codex-rs/core/src/context_manager/manifest/options.rs"
  "$repo_root/codex-rs/core/src/context_manager/manifest/policy.rs"
  "$repo_root/codex-rs/core/src/context_manager/manifest/policy/compression.rs"
  "$repo_root/codex-rs/core/src/context_manager/manifest/rewrite.rs"
  "$repo_root/codex-rs/core/src/context_manager/manifest/selected_recall.rs"
  "$repo_root/codex-rs/core/src/context_manager/manifest/selected_snippet.rs"
  "$repo_root/codex-rs/core/src/context_manager/manifest/tests.rs"
  "$repo_root/codex-rs/core/src/context_manager/source_registry/catalog.rs"
  "$repo_root/codex-rs/core/src/context_manager/source_registry/tests.rs"
  "$repo_root/codex-rs/core/src/event_mapping.rs"
  "$repo_root/codex-rs/core/src/prompt_debug.rs"
  "$repo_root/codex-rs/core/src/session/handlers.rs"
  "$repo_root/codex-rs/core/src/session/mod.rs"
  "$repo_root/codex-rs/core/src/session/tests/contract_part_02.rs"
  "$repo_root/codex-rs/core/src/session/tests/contract_part_03.rs"
  "$repo_root/codex-rs/core/src/session/tests/contract_part_04.rs"
  "$repo_root/codex-rs/protocol/src/protocol.rs"
)
selected_snippet_core_protocol_source_marker_paths=()
while IFS= read -r source_path; do
  selected_snippet_core_protocol_source_marker_paths+=("$source_path")
done < <(
  rg -l \
    '<selected_context_recall>|selected_context_recall|LIVE_RECALL_SELECTED_SNIPPETS|context_recall_selected_snippets|recall_selected_snippets|TurnContextRecallSelectedSnippet|ContextRecallSelectedSnippet|selected[- ]snippet|selected[- ]recall|recall[- ]snippets?' \
    "$repo_root/codex-rs/core/src" \
    "$repo_root/codex-rs/protocol/src" \
    | sort
)
selected_snippet_runtime_source_marker_allowed_paths=(
  "$repo_root/codex-rs/hepta-runtime/src/context_recall_operator_invocation.rs"
  "$repo_root/codex-rs/hepta-runtime/src/context_recall_operator_scheduler.rs"
  "$repo_root/codex-rs/hepta-runtime/src/multi_agent.rs"
  "$repo_root/codex-rs/hepta-runtime/src/query.rs"
  "$repo_root/codex-rs/hepta-runtime/src/query/context_recall_selected_snippet_envelope.rs"
  "$repo_root/codex-rs/hepta-runtime/src/query/context_recall_turn_handoff.rs"
  "$repo_root/codex-rs/hepta-runtime/src/query/tests.rs"
  "$repo_root/codex-rs/hepta-runtime/src/runtime_kernel/context_turn_ops.rs"
  "$repo_root/codex-rs/hepta-runtime/src/runtime_kernel/context_turn_ops/approved_candidate.rs"
  "$repo_root/codex-rs/hepta-runtime/src/runtime_kernel/exports_workgraph.rs"
  "$repo_root/codex-rs/hepta-runtime/src/runtime_kernel/tests.rs"
  "$repo_root/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs"
  "$repo_root/codex-rs/hepta-runtime/src/runtime_kernel/turn_coordinator.rs"
  "$repo_root/codex-rs/hepta-runtime/src/runtime_kernel/types.rs"
  "$repo_root/codex-rs/hepta-runtime/src/worker_tasks.rs"
  "$repo_root/codex-rs/hepta-runtime/src/worker_tasks/tests.rs"
)
selected_snippet_runtime_source_marker_paths=()
while IFS= read -r source_path; do
  selected_snippet_runtime_source_marker_paths+=("$source_path")
done < <(
  rg -l \
    'selected_snippet|selected[- ]snippet|selected[- ]recall|recall[- ]snippets?|selected_context_recall|ContextRecallSelectedSnippet|context_recall_selected_snippets|recall_selected_snippets' \
    "$repo_root/codex-rs/hepta-runtime/src" \
    | sort
)
selected_snippet_external_source_marker_allowed_paths=(
  "$repo_root/codex-rs/app-server/README.md"
  "$repo_root/codex-rs/app-server/src/message_processor_tracing_tests.rs"
  "$repo_root/codex-rs/app-server/src/request_processors.rs"
  "$repo_root/codex-rs/app-server/src/request_processors/turn_processor.rs"
  "$repo_root/codex-rs/app-server/tests/suite/v2/experimental_api.rs"
  "$repo_root/codex-rs/app-server/tests/suite/v2/turn_start.rs"
  "$repo_root/codex-rs/hepta-native-gateway/src/native_gateway/release_ui_reports.rs"
  "$repo_root/codex-rs/hepta-native-gateway/src/native_gateway/report_types.rs"
  "$repo_root/codex-rs/hepta-native-gateway/src/native_gateway/tests/contract_part_01.rs"
  "$repo_root/codex-rs/hepta-native-gateway/src/route_registry.rs"
  "$repo_root/codex-rs/exec/src/lib.rs"
  "$repo_root/codex-rs/exec/src/lib_tests.rs"
  "$repo_root/codex-rs/tui/src/app/thread_routing.rs"
  "$repo_root/codex-rs/tui/src/app_command.rs"
  "$repo_root/codex-rs/tui/src/app_server_session.rs"
)
selected_snippet_external_source_marker_paths=()
while IFS= read -r source_path; do
  selected_snippet_external_source_marker_paths+=("$source_path")
done < <(
  rg -l \
    'contextRecallSelectedSnippets|context_recall_selected_snippets|ContextRecallSelectedSnippet|selectedSnippet|selected_snippet|selected[- ]snippet|selected[- ]recall|recall[- ]snippets?' \
    "$repo_root/codex-rs/app-server/README.md" \
    "$repo_root/codex-rs/app-server/src" \
    "$repo_root/codex-rs/app-server/tests" \
    "$repo_root/codex-rs/hepta-native-gateway/src" \
    "$repo_root/codex-rs/exec/src" \
    "$repo_root/codex-rs/tui/src" \
    | sort
)
selected_snippet_response_debug_source_marker_allowed_paths=(
  "$repo_root/codex-rs/response-debug-context/src/rollout_context.rs"
  "$repo_root/codex-rs/response-debug-context/src/tests.rs"
)
selected_snippet_response_debug_source_marker_paths=()
while IFS= read -r source_path; do
  selected_snippet_response_debug_source_marker_paths+=("$source_path")
done < <(
  rg -l \
    'contextRecallSelectedSnippets|context_recall_selected_snippets|ContextRecallSelectedSnippet|selectedSnippet|selected_snippet|recall_selected_snippets|selected[- ]snippet|selected[- ]recall|recall[- ]snippets?' \
    "$repo_root/codex-rs/response-debug-context/src" \
    | sort
)
selected_snippet_json_marker_paths=()
while IFS= read -r schema_path; do
  selected_snippet_json_marker_paths+=("$schema_path")
done < <(
  rg -l \
    'contextRecallSelectedSnippets|context_recall_selected_snippets|ContextRecallSelectedSnippet|selectedSnippet|selected_snippet' \
    "$repo_root/codex-rs/app-server-protocol/schema/json" \
    | sort
)
selected_snippet_v2_typescript_marker_allowed_paths=(
  "$repo_root/codex-rs/app-server-protocol/schema/typescript/v2/ContextRecallSelectedSnippet.ts"
  "$repo_root/codex-rs/app-server-protocol/schema/typescript/v2/ContextRecallSelectedSnippetEnvelope.ts"
  "$repo_root/codex-rs/app-server-protocol/schema/typescript/v2/ContextRecallSelectedSnippetSafety.ts"
  "$repo_root/codex-rs/app-server-protocol/schema/typescript/v2/index.ts"
)
selected_snippet_v2_typescript_marker_paths=()
while IFS= read -r typescript_path; do
  selected_snippet_v2_typescript_marker_paths+=("$typescript_path")
done < <(
  rg -l \
    'contextRecallSelectedSnippets|context_recall_selected_snippets|ContextRecallSelectedSnippet|selectedSnippet|selected_snippet' \
    "$repo_root/codex-rs/app-server-protocol/schema/typescript/v2" \
    | sort
)
selected_snippet_forbidden_root_typescript_paths=()
while IFS= read -r typescript_path; do
  selected_snippet_forbidden_root_typescript_paths+=("$typescript_path")
done < <(
  rg --files "$repo_root/codex-rs/app-server-protocol/schema/typescript" \
    | rg '\.ts$' \
    | rg -v '/v2/' \
    | sort
)
selected_snippet_forbidden_root_typescript_param_paths=()
while IFS= read -r typescript_param_path; do
  selected_snippet_forbidden_root_typescript_param_paths+=("$typescript_param_path")
done < <(
  rg --files "$repo_root/codex-rs/app-server-protocol/schema/typescript" \
    | rg '/[^/]*Params\.ts$' \
    | rg -v '/v2/' \
    | sort
)
selected_snippet_forbidden_non_v2_individual_schema_paths=()
while IFS= read -r schema_path; do
  selected_snippet_forbidden_non_v2_individual_schema_paths+=("$schema_path")
done < <(
  rg --files "$repo_root/codex-rs/app-server-protocol/schema/json" \
    | rg '/[^/]*Params\.json$' \
    | rg -v '/v2/' \
    | sort
)
selected_snippet_forbidden_individual_schema_params=()
while IFS= read -r request_param; do
  if [ "$request_param" = "TurnStartParams" ]; then
    continue
  fi
  selected_snippet_forbidden_individual_schema_params+=("$request_param")
done < <(
  rg --files "$repo_root/codex-rs/app-server-protocol/schema/json/v2" \
    | sed -n 's#.*/\([^/]*Params\)\.json$#\1#p' \
    | sort -u
)
selected_snippet_forbidden_client_request_params=()
while IFS= read -r request_param; do
  if [ "$request_param" = "TurnStartParams" ]; then
    continue
  fi
  selected_snippet_forbidden_client_request_params+=("$request_param")
done < <(
  jq -r '.definitions | keys[] | select(endswith("Params"))' \
    "$repo_root/codex-rs/app-server-protocol/schema/json/ClientRequest.json" \
    | sort -u
)
selected_snippet_forbidden_protocol_schema_params=()
while IFS= read -r request_param; do
  if [ "$request_param" = "TurnStartParams" ]; then
    continue
  fi
  selected_snippet_forbidden_protocol_schema_params+=("$request_param")
done < <(
  jq -r '.definitions.v2 | keys[] | select(endswith("Params"))' \
    "$repo_root/codex-rs/app-server-protocol/schema/json/codex_app_server_protocol.schemas.json" \
    | sort -u
)
selected_snippet_forbidden_protocol_root_schema_params=()
while IFS= read -r request_param; do
  if [ "$request_param" = "TurnStartParams" ]; then
    continue
  fi
  selected_snippet_forbidden_protocol_root_schema_params+=("$request_param")
done < <(
  jq -r '.definitions | keys[] | select(endswith("Params"))' \
    "$repo_root/codex-rs/app-server-protocol/schema/json/codex_app_server_protocol.schemas.json" \
    | sort -u
)
selected_snippet_forbidden_protocol_v2_schema_params=()
while IFS= read -r request_param; do
  if [ "$request_param" = "TurnStartParams" ]; then
    continue
  fi
  selected_snippet_forbidden_protocol_v2_schema_params+=("$request_param")
done < <(
  jq -r '.definitions | keys[] | select(endswith("Params"))' \
    "$repo_root/codex-rs/app-server-protocol/schema/json/codex_app_server_protocol.v2.schemas.json" \
    | sort -u
)
selected_snippet_forbidden_typescript_param_paths=()
while IFS= read -r typescript_param_path; do
  if [ "$(basename "$typescript_param_path")" = "TurnStartParams.ts" ]; then
    continue
  fi
  selected_snippet_forbidden_typescript_param_paths+=("$typescript_param_path")
done < <(
  rg --files "$repo_root/codex-rs/app-server-protocol/schema/typescript/v2" \
    | rg 'Params\.ts$' \
    | sort
)

if ! rg -n '#\[experimental\("turn/start\.contextRecallSelectedSnippets"\)\]' \
  "$turn_protocol_source" >/dev/null; then
  fail "turn/start.contextRecallSelectedSnippets is no longer explicitly experimental in source"
fi

if ! rg -n 'context_recall_selected_snippets: Option<ContextRecallSelectedSnippetEnvelope>' \
  "$turn_protocol_source" >/dev/null; then
  fail "turn/start selected-snippet source field is missing"
fi

for entry in "${stable_turn_start_schema_paths[@]}"; do
  schema_path="${entry%%:*}"
  properties_path="${entry#*:}"
  assert_schema_properties_hide_selected_snippets \
    "$schema_path" "$properties_path" "stable TurnStartParams"
done

for schema_path in "${selected_snippet_json_marker_paths[@]}"; do
  if ! array_contains "$schema_path" "${selected_snippet_json_marker_allowed_paths[@]}"; then
    fail "JSON schema outside the selected-snippet allowlist references selected-snippet handoff: $schema_path"
  fi
done

for typescript_path in "${selected_snippet_v2_typescript_marker_paths[@]}"; do
  if ! array_contains "$typescript_path" "${selected_snippet_v2_typescript_marker_allowed_paths[@]}"; then
    fail "v2 TypeScript file outside the selected-snippet helper allowlist references selected-snippet handoff: $typescript_path"
  fi
done

for source_path in "${selected_snippet_app_server_protocol_source_marker_paths[@]}"; do
  if ! array_contains "$source_path" "${selected_snippet_app_server_protocol_source_marker_allowed_paths[@]}"; then
    fail "app-server protocol source outside the selected-snippet marker allowlist references selected-snippet handoff: $source_path"
  fi
done

for source_path in "${selected_snippet_core_protocol_source_marker_paths[@]}"; do
  if ! array_contains "$source_path" "${selected_snippet_core_protocol_source_marker_allowed_paths[@]}"; then
    fail "core/protocol source outside the selected-snippet marker allowlist references selected-snippet live handoff: $source_path"
  fi
done

for source_path in "${selected_snippet_runtime_source_marker_paths[@]}"; do
  if ! array_contains "$source_path" "${selected_snippet_runtime_source_marker_allowed_paths[@]}"; then
    fail "hepta-runtime source outside the selected-snippet marker allowlist references selected-snippet handoff: $source_path"
  fi
done

for source_path in "${selected_snippet_external_source_marker_paths[@]}"; do
  if ! array_contains "$source_path" "${selected_snippet_external_source_marker_allowed_paths[@]}"; then
    fail "external app/native/TUI/exec source outside the selected-snippet marker allowlist references selected-snippet handoff: $source_path"
  fi
done

for source_path in "${selected_snippet_response_debug_source_marker_paths[@]}"; do
  if ! array_contains "$source_path" "${selected_snippet_response_debug_source_marker_allowed_paths[@]}"; then
    fail "response-debug source outside the selected-snippet marker allowlist references selected-snippet export: $source_path"
  fi
done

assert_schema_marker_definitions_only_allowlist \
  "$repo_root/codex-rs/app-server-protocol/schema/json/ClientRequest.json" \
  ".definitions" \
  "ClientRequest JSON schema" \
  "ContextRecallSelectedSnippet" \
  "ContextRecallSelectedSnippetEnvelope" \
  "ContextRecallSelectedSnippetSafety"

assert_schema_marker_definitions_only_allowlist \
  "$repo_root/codex-rs/app-server-protocol/schema/json/codex_app_server_protocol.schemas.json" \
  ".definitions | with_entries(select(.key != \"v2\"))" \
  "root aggregate stable JSON schema"

assert_schema_marker_definitions_only_allowlist \
  "$repo_root/codex-rs/app-server-protocol/schema/json/codex_app_server_protocol.schemas.json" \
  ".definitions.v2" \
  "root aggregate v2 JSON schema" \
  "ContextRecallSelectedSnippet" \
  "ContextRecallSelectedSnippetEnvelope" \
  "ContextRecallSelectedSnippetSafety"

assert_schema_marker_definitions_only_allowlist \
  "$repo_root/codex-rs/app-server-protocol/schema/json/codex_app_server_protocol.v2.schemas.json" \
  ".definitions" \
  "v2 aggregate JSON schema" \
  "ContextRecallSelectedSnippet" \
  "ContextRecallSelectedSnippetEnvelope" \
  "ContextRecallSelectedSnippetSafety"

assert_schema_marker_definitions_only_allowlist \
  "$repo_root/codex-rs/app-server-protocol/schema/json/v2/TurnStartParams.json" \
  ".definitions" \
  "v2 TurnStartParams JSON schema" \
  "ContextRecallSelectedSnippet" \
  "ContextRecallSelectedSnippetEnvelope" \
  "ContextRecallSelectedSnippetSafety"

for request_param in "${selected_snippet_forbidden_individual_schema_params[@]}"; do
  assert_schema_fragment_hide_selected_snippet_markers \
    "$repo_root/codex-rs/app-server-protocol/schema/json/v2/$request_param.json" \
    "." \
    "$request_param individual schema"
done

for schema_path in "${selected_snippet_forbidden_non_v2_individual_schema_paths[@]}"; do
  assert_schema_fragment_hide_selected_snippet_markers \
    "$schema_path" \
    "." \
    "$(basename "$schema_path") non-v2 individual schema"
done

for request_param in "${selected_snippet_forbidden_client_request_params[@]}"; do
  assert_schema_fragment_hide_selected_snippet_markers \
    "$repo_root/codex-rs/app-server-protocol/schema/json/ClientRequest.json" \
    ".definitions.$request_param" \
    "$request_param ClientRequest schema"
done

for request_param in "${selected_snippet_forbidden_protocol_schema_params[@]}"; do
  assert_schema_fragment_hide_selected_snippet_markers \
    "$repo_root/codex-rs/app-server-protocol/schema/json/codex_app_server_protocol.schemas.json" \
    ".definitions.v2.$request_param" \
    "$request_param aggregate schema"
done

for request_param in "${selected_snippet_forbidden_protocol_root_schema_params[@]}"; do
  assert_schema_fragment_hide_selected_snippet_markers \
    "$repo_root/codex-rs/app-server-protocol/schema/json/codex_app_server_protocol.schemas.json" \
    ".definitions.$request_param" \
    "$request_param root aggregate schema"
done

for request_param in "${selected_snippet_forbidden_protocol_v2_schema_params[@]}"; do
  assert_schema_fragment_hide_selected_snippet_markers \
    "$repo_root/codex-rs/app-server-protocol/schema/json/codex_app_server_protocol.v2.schemas.json" \
    ".definitions.$request_param" \
    "$request_param v2 aggregate schema"
done

for typescript_param_path in "${selected_snippet_forbidden_typescript_param_paths[@]}"; do
  if rg -n 'contextRecallSelectedSnippets|context_recall_selected_snippets|ContextRecallSelectedSnippet|selectedSnippet|selected_snippet' \
    "$typescript_param_path"; then
    fail "$(basename "$typescript_param_path") TypeScript references selected-snippet handoff"
  fi
done

if rg -n 'contextRecallSelectedSnippets|context_recall_selected_snippets|ContextRecallSelectedSnippet|selectedSnippet|selected_snippet' \
  "$stable_turn_start_ts"; then
  fail "stable TypeScript TurnStartParams references selected-snippet handoff"
fi

if rg -n 'contextRecallSelectedSnippets|context_recall_selected_snippets|ContextRecallSelectedSnippet|selectedSnippet|selected_snippet' \
  "${stable_typescript_request_paths[@]}"; then
  fail "stable TypeScript request surface references selected-snippet handoff"
fi

for typescript_param_path in "${selected_snippet_forbidden_root_typescript_param_paths[@]}"; do
  if rg -n 'contextRecallSelectedSnippets|context_recall_selected_snippets|ContextRecallSelectedSnippet|selectedSnippet|selected_snippet' \
    "$typescript_param_path"; then
    fail "$(basename "$typescript_param_path") root TypeScript params references selected-snippet handoff"
  fi
done

for typescript_path in "${selected_snippet_forbidden_root_typescript_paths[@]}"; do
  if rg -n 'contextRecallSelectedSnippets|context_recall_selected_snippets|ContextRecallSelectedSnippet|selectedSnippet|selected_snippet' \
    "$typescript_path"; then
    fail "$(basename "$typescript_path") root TypeScript file references selected-snippet handoff"
  fi
done

if [ ! -f "$repo_root/codex-rs/app-server-protocol/schema/typescript/v2/ContextRecallSelectedSnippetEnvelope.ts" ]; then
  fail "selected-snippet helper TypeScript type is missing"
fi

if rg -n "$readme_selected_snippet_marker_pattern" "$app_server_readme" \
  | rg -v 'capabilities\.experimentalApi = true'; then
  fail "app-server docs mention selected-snippet handoff without experimentalApi gate"
fi
if rg -n "$readme_selected_snippet_marker_pattern" "$app_server_readme" \
  | rg -v 'turn/start'; then
  fail "app-server docs mention selected-snippet handoff without turn/start scope"
fi
if rg -n "$readme_selected_snippet_marker_pattern" "$app_server_readme" \
  | rg -v 'for the new turn only|new turn only|non-`turn/start`'; then
  fail "app-server docs mention selected-snippet handoff without turn/start-only scope"
fi

external_surface_paths=(
  "$repo_root/codex-rs/hepta-native-gateway/src"
  "$repo_root/codex-rs/app-server/src"
  "$repo_root/codex-rs/tui/src"
  "$repo_root/codex-rs/exec/src"
)
if rg -n \
  'run_context_recall_operator_invocation_command|ContextRecallOperatorInvocationCommand(Request|Report)|hepta-context-recall-runtime-operator-command|/hepta-context-recall-handoff --execute --json --target' \
  "${external_surface_paths[@]}"; then
  fail "external app/native/TUI/exec surfaces call the runtime command facade"
fi

if rg -n \
  '(run_context_recall_operator_invocation(_command)?\s*\(|run_worker_scheduler_with_context_recall_operator_(handoff|invocation)\s*\(|run_ready_agents_with_context_recall_operator_invocation\s*\(|run_ready_agents_with_context_recall_handoff\s*\(|run_(ready|due)_worker_tasks_with_context_recall_handoff\s*\(|run_demo_turn_in_session_with_context_recall_handoff\s*\()' \
  "${external_surface_paths[@]}"; then
  fail "external app/native/TUI/exec surfaces have live hepta-runtime selected-snippet wiring"
fi

for context_gate_cargo_script in "${context_gate_cargo_scripts[@]}"; do
  assert_context_gate_target_dir_contract \
    "${context_gate_cargo_script%%:*}" \
    "${context_gate_cargo_script#*:}"
done

native_gateway_handoff_report="$(
  awk '
    /^fn hepta_context_recall_worker_scheduler_handoff_report/ { in_report = 1 }
    /^fn hepta_provider_metadata_inventory_report/ { in_report = 0 }
    in_report { print }
  ' "$native_gateway_source"
)"
native_gateway_non_test_source="$(
  awk '
    /^mod tests \{/ { exit }
    { print }
  ' "$native_gateway_source"
)"
native_gateway_handoff_route_handler="$(
  awk '
    /HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_ENDPOINT =>/ { in_route = 1 }
    in_route && /HEPTA_CHANNEL_ADAPTER_STATUS_INVENTORY_ENDPOINT =>/ { exit }
    in_route { print }
  ' "$native_gateway_source"
)"
native_gateway_handoff_control_spec="$(
  awk '
    /ControlUiRouteSpec \{/ {
      in_spec = 1
      spec = ""
    }
    in_spec {
      spec = spec $0 "\n"
    }
    in_spec && /},/ {
      if (spec ~ /pattern: HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_ENDPOINT/) {
        print spec
      }
      in_spec = 0
      spec = ""
    }
  ' "$native_gateway_source"
)"
native_gateway_handoff_approval_env_reads="$(
  {
    rg -n 'env_truthy\(HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_APPROVED_ENV\)' \
      "$native_gateway_source" \
      || true
  } \
    | wc -l \
    | tr -d '[:space:]'
)"
native_gateway_handoff_status_block="$(
  printf '%s\n' "$native_gateway_handoff_report" \
    | awk '
      /^[[:space:]]*status: if operator_approval_enabled \{/ {
        in_status = 1
      }
      in_status {
        print
      }
      in_status && /^[[:space:]]*},[[:space:]]*$/ {
        exit
      }
    '
)"

assert_rust_function_signature_match \
  "$native_gateway_source" \
  "hepta_context_recall_worker_scheduler_handoff_report" \
  "fn hepta_context_recall_worker_scheduler_handoff_report() -> HeptaContextRecallWorkerSchedulerHandoffResponse {" \
  "native gateway selected-snippet dry-run route"
assert_rust_const_string_assignment_match \
  "$native_gateway_source" \
  "HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_ENDPOINT" \
  "/api/hepta-context-recall-worker-scheduler-handoff" \
  "native gateway selected-snippet dry-run route"
assert_text_lines_containing_match \
  "$native_gateway_non_test_source" \
  "/api/hepta-context-recall-worker-scheduler-handoff" \
  "native gateway selected-snippet endpoint path literal production usage" \
  '"/api/hepta-context-recall-worker-scheduler-handoff";'
assert_rust_const_string_assignment_match \
  "$native_gateway_source" \
  "HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_APPROVED_ENV" \
  "HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_APPROVED" \
  "native gateway selected-snippet approval env"
assert_text_lines_containing_match \
  "$native_gateway_non_test_source" \
  "HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_APPROVED_ENV" \
  "native gateway selected-snippet approval env production usage" \
  "const HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_APPROVED_ENV: &str =" \
  "env_truthy(HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_APPROVED_ENV);" \
  "operator_approval_env: HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_APPROVED_ENV,"
assert_text_lines_containing_match \
  "$native_gateway_non_test_source" \
  "HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_ENDPOINT" \
  "native gateway selected-snippet endpoint production usage" \
  "HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_ENDPOINT => {" \
  "pub(crate) const HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_ENDPOINT: &str =" \
  "endpoint: HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_ENDPOINT," \
  "pattern: HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_ENDPOINT,"
assert_text_lines_containing_match \
  "$native_gateway_non_test_source" \
  "/hepta-context-recall-worker-scheduler-handoff --dry-run --json" \
  "native gateway selected-snippet source command production usage" \
  'source_command: "/hepta-context-recall-worker-scheduler-handoff --dry-run --json",' \
  'source_command: "/hepta-context-recall-worker-scheduler-handoff --dry-run --json",'
assert_text_lines_containing_match \
  "$native_gateway_non_test_source" \
  'capability: "hepta-context-recall-worker-scheduler-handoff"' \
  "native gateway selected-snippet ControlUi capability production usage" \
  'capability: "hepta-context-recall-worker-scheduler-handoff",'
assert_text_lines_containing_match \
  "$native_gateway_non_test_source" \
  'side_effect_boundary: "read-only selected-snippet worker scheduler handoff route contract; exposes explicit operator gate status without running workers, invoking models, injecting snippets, writing registries, or promoting stable schema"' \
  "native gateway selected-snippet ControlUi side-effect boundary production usage" \
  'side_effect_boundary: "read-only selected-snippet worker scheduler handoff route contract; exposes explicit operator gate status without running workers, invoking models, injecting snippets, writing registries, or promoting stable schema",'

assert_rust_struct_fields_match \
  "$native_gateway_source" \
  "HeptaContextRecallWorkerSchedulerHandoffResponse" \
  "native gateway selected-snippet dry-run response" \
  "allowed_runtime_entrypoints" \
  "blockers" \
  "compatibility_mode" \
  "default_worker_policy" \
  "endpoint" \
  "legacy_ready_due_scheduler_defaults_disabled" \
  "native_route" \
  "next_runtime_step" \
  "operator_approval_enabled" \
  "operator_approval_env" \
  "operator_approved_policy" \
  "product" \
  "query_payload_exposed" \
  "ready_due_scheduler_variants_available" \
  "report_shape" \
  "route_executes_scheduler" \
  "route_injects_selected_snippets" \
  "route_invokes_model" \
  "route_runs_worker_task" \
  "runtime" \
  "selected_snippet_text_exposed" \
  "side_effect_free" \
  "side_effects" \
  "source_command" \
  "source_ids_exposed" \
  "stable_schema_promoted" \
  "status" \
  "tui_exec_app_server_defaults_none"
assert_rust_struct_header_match \
  "$native_gateway_source" \
  "HeptaContextRecallWorkerSchedulerHandoffResponse" \
  "native gateway selected-snippet dry-run response"
assert_rust_struct_fields_match \
  "$native_gateway_source" \
  "HeptaContextRecallWorkerSchedulerHandoffSideEffects" \
  "native gateway selected-snippet dry-run side effects" \
  "credential_read" \
  "due_scheduler_ran" \
  "external_network_read" \
  "filesystem_written" \
  "gateway_mutation_performed" \
  "message_sent" \
  "model_invoked" \
  "native_post_mutation_performed" \
  "provider_invoked" \
  "ready_scheduler_ran" \
  "selected_snippets_injected" \
  "session_store_mutated" \
  "stable_schema_mutated" \
  "task_registry_mutated" \
  "telegram_read_performed" \
  "worker_task_ran"
assert_rust_struct_header_match \
  "$native_gateway_source" \
  "HeptaContextRecallWorkerSchedulerHandoffSideEffects" \
  "native gateway selected-snippet dry-run side effects"
assert_report_entrypoints_match \
  "$native_gateway_handoff_report" \
  "native gateway selected-snippet dry-run route" \
  "run_due_worker_tasks_with_context_recall_handoff" \
  "run_ready_worker_tasks_with_context_recall_handoff"
assert_report_entrypoint_source_block_match \
  "$native_gateway_handoff_report" \
  "native gateway selected-snippet dry-run route" \
  'allowed_runtime_entrypoints: &[' \
  '"run_ready_worker_tasks_with_context_recall_handoff",' \
  '"run_due_worker_tasks_with_context_recall_handoff",' \
  '],'
assert_report_blockers_match \
  "$native_gateway_handoff_report" \
  "native gateway selected-snippet dry-run route" \
  "context_recall_worker_scheduler_operator_approval_env_disabled" \
  "native_gateway_route_is_plan_only_no_worker_execution"
assert_text_lines_containing_match \
  "$native_gateway_handoff_report" \
  "blockers" \
  "native gateway selected-snippet dry-run route" \
  'blockers,' \
  'blockers.push("context_recall_worker_scheduler_operator_approval_env_disabled");' \
  'blockers.push("native_gateway_route_is_plan_only_no_worker_execution");' \
  "let mut blockers = Vec::new();"
assert_text_lines_containing_match \
  "$native_gateway_non_test_source" \
  "context_recall_worker_scheduler_operator_approval_env_disabled" \
  "native gateway selected-snippet operator approval blocker production usage" \
  'blockers.push("context_recall_worker_scheduler_operator_approval_env_disabled");'
assert_text_lines_containing_match \
  "$native_gateway_non_test_source" \
  "native_gateway_route_is_plan_only_no_worker_execution" \
  "native gateway selected-snippet plan-only blocker production usage" \
  'blockers.push("native_gateway_route_is_plan_only_no_worker_execution");'
assert_report_scalar_assignments_match \
  "$native_gateway_handoff_report" \
  "native gateway selected-snippet dry-run route" \
  'compatibility_mode="native_context_recall_worker_scheduler_handoff_dry_run"' \
  "credential_read=false" \
  'default_worker_policy="Disabled"' \
  "due_scheduler_ran=false" \
  "endpoint=HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_ENDPOINT" \
  "external_network_read=false" \
  "filesystem_written=false" \
  "gateway_mutation_performed=false" \
  "legacy_ready_due_scheduler_defaults_disabled=true" \
  "message_sent=false" \
  "model_invoked=false" \
  "native_post_mutation_performed=false" \
  "native_route=true" \
  "operator_approval_env=HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_APPROVED_ENV" \
  'operator_approved_policy="ExperimentalOperatorApproved"' \
  'next_runtime_step="wire an explicitly approved native/operator caller to these runtime entrypoints outside this read-only route"' \
  'product="Hepta"' \
  "provider_invoked=false" \
  "query_payload_exposed=false" \
  "ready_due_scheduler_variants_available=true" \
  "ready_scheduler_ran=false" \
  'report_shape="policy plus aggregate selected-snippet presence/count and per-run provider rollup only"' \
  "route_executes_scheduler=false" \
  "route_injects_selected_snippets=false" \
  "route_invokes_model=false" \
  "route_runs_worker_task=false" \
  'runtime="hepta"' \
  "selected_snippet_text_exposed=false" \
  "selected_snippets_injected=false" \
  "session_store_mutated=false" \
  "side_effect_free=true" \
  'source_command="/hepta-context-recall-worker-scheduler-handoff --dry-run --json"' \
  "source_ids_exposed=false" \
  "stable_schema_mutated=false" \
  "stable_schema_promoted=false" \
  "task_registry_mutated=false" \
  "telegram_read_performed=false" \
  "tui_exec_app_server_defaults_none=true" \
  "worker_task_ran=false"
assert_text_lines_containing_match \
  "$native_gateway_handoff_report" \
  "operator_approval_enabled" \
  "native gateway selected-snippet dry-run route" \
  "if !operator_approval_enabled {" \
  "let operator_approval_enabled =" \
  "operator_approval_enabled," \
  "status: if operator_approval_enabled {"
assert_text_contains \
  "$native_gateway_handoff_report" \
  'status: if operator_approval_enabled {' \
  "native gateway selected-snippet dry-run route"
assert_trimmed_text_block_match \
  "$native_gateway_handoff_status_block" \
  "native gateway selected-snippet dry-run status branch" \
  'status: if operator_approval_enabled {' \
  '"operator_gate_visible"' \
  '} else {' \
  '"blocked"' \
  '},'
assert_text_contains \
  "$native_gateway_handoff_report" \
  '"operator_gate_visible"' \
  "native gateway selected-snippet dry-run route"
assert_text_contains \
  "$native_gateway_handoff_report" \
  '"blocked"' \
  "native gateway selected-snippet dry-run route"
assert_text_not_contains \
  "$native_gateway_handoff_report" \
  '"ready"' \
  "native gateway selected-snippet dry-run route"
assert_text_not_contains \
  "$native_gateway_handoff_report" \
  '"approved"' \
  "native gateway selected-snippet dry-run route"
assert_text_not_contains \
  "$native_gateway_handoff_report" \
  'run_context_recall_operator_invocation' \
  "native gateway selected-snippet dry-run route"
assert_text_not_contains \
  "$native_gateway_handoff_report" \
  'run_worker_scheduler_with_context_recall_operator' \
  "native gateway selected-snippet dry-run route"
assert_text_not_contains \
  "$native_gateway_handoff_report" \
  'run_ready_agents_with_context_recall_operator' \
  "native gateway selected-snippet dry-run route"
assert_text_not_contains \
  "$native_gateway_handoff_report" \
  'ContextRecallOperatorInvocationCommand' \
  "native gateway selected-snippet dry-run route"
assert_text_not_contains \
  "$native_gateway_handoff_report" \
  'hepta-context-recall-runtime-operator-command' \
  "native gateway selected-snippet dry-run route"
assert_text_not_matches \
  "$native_gateway_handoff_report" \
  'std::fs::|tokio::fs::|fs::write\(|File::create\(|OpenOptions::|write_all\(|std::process::Command|Command::new\(|reqwest::|hyper::Client|ureq::|TcpStream|UnixStream|native_telegram::|send_message\(|send_telegram\(|run_ready_worker_tasks_with_context_recall_handoff\(|run_due_worker_tasks_with_context_recall_handoff\(|run_context_recall_operator_invocation_command\(' \
  "native gateway selected-snippet dry-run route side-effect source"

assert_file_fixed_occurrence_count \
  "$native_gateway_source" \
  '"HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_APPROVED"' \
  1 \
  "native gateway selected-snippet approval env literal"
if [ "$native_gateway_handoff_approval_env_reads" != "1" ]; then
  fail "native gateway selected-snippet approval env must be read only by the dry-run report"
fi
assert_text_contains \
  "$native_gateway_handoff_report" \
  'env_truthy(HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_APPROVED_ENV)' \
  "native gateway selected-snippet approval env"
assert_text_contains \
  "$native_gateway_handoff_report" \
  'operator_approval_enabled,' \
  "native gateway selected-snippet approval env"
assert_text_contains \
  "$native_gateway_handoff_report" \
  '"operator_gate_visible"' \
  "native gateway selected-snippet approval env"
assert_text_not_contains \
  "$native_gateway_handoff_route_handler" \
  'HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_APPROVED_ENV' \
  "native gateway selected-snippet route binding"
assert_text_not_contains \
  "$native_gateway_handoff_route_handler" \
  'operator_approval_enabled' \
  "native gateway selected-snippet route binding"
assert_text_not_contains \
  "$native_gateway_handoff_control_spec" \
  'HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_APPROVED_ENV' \
  "native gateway selected-snippet ControlUi route spec"
assert_text_not_contains \
  "$native_gateway_handoff_control_spec" \
  'operator_approval_enabled' \
  "native gateway selected-snippet ControlUi route spec"

assert_text_contains \
  "$native_gateway_handoff_route_handler" \
  'HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_ENDPOINT => {' \
  "native gateway selected-snippet route binding"
assert_text_contains \
  "$native_gateway_handoff_route_handler" \
  '"200 OK"' \
  "native gateway selected-snippet route binding"
assert_text_contains \
  "$native_gateway_handoff_route_handler" \
  '"application/json; charset=utf-8"' \
  "native gateway selected-snippet route binding"
assert_text_contains \
  "$native_gateway_handoff_route_handler" \
  'json_or_error(&hepta_context_recall_worker_scheduler_handoff_report())' \
  "native gateway selected-snippet route binding"
assert_text_not_contains \
  "$native_gateway_handoff_route_handler" \
  'run_context_recall_operator_invocation' \
  "native gateway selected-snippet route binding"
assert_text_not_contains \
  "$native_gateway_handoff_route_handler" \
  'run_worker_scheduler_with_context_recall' \
  "native gateway selected-snippet route binding"
assert_text_not_contains \
  "$native_gateway_handoff_route_handler" \
  'run_ready_worker_tasks_with_context_recall' \
  "native gateway selected-snippet route binding"
assert_text_not_contains \
  "$native_gateway_handoff_route_handler" \
  'run_due_worker_tasks_with_context_recall' \
  "native gateway selected-snippet route binding"

assert_trimmed_text_block_match \
  "$native_gateway_handoff_route_handler" \
  "native gateway selected-snippet route binding" \
  'HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_ENDPOINT => {' \
  'return (' \
  '"200 OK",' \
  '"application/json; charset=utf-8",' \
  'json_or_error(&hepta_context_recall_worker_scheduler_handoff_report()),' \
  ');' \
  '}'

assert_trimmed_text_block_match \
  "$native_gateway_handoff_control_spec" \
  "native gateway selected-snippet ControlUi route spec" \
  'ControlUiRouteSpec {' \
  'method: "GET",' \
  'pattern: HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_ENDPOINT,' \
  'source_command: "/hepta-context-recall-worker-scheduler-handoff --dry-run --json",' \
  'capability: "hepta-context-recall-worker-scheduler-handoff",' \
  'side_effect_boundary: "read-only selected-snippet worker scheduler handoff route contract; exposes explicit operator gate status without running workers, invoking models, injecting snippets, writing registries, or promoting stable schema",' \
  '},'
assert_text_contains \
  "$native_gateway_handoff_control_spec" \
  'method: "GET"' \
  "native gateway selected-snippet ControlUi route spec"
assert_text_contains \
  "$native_gateway_handoff_control_spec" \
  'pattern: HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_ENDPOINT' \
  "native gateway selected-snippet ControlUi route spec"
assert_text_contains \
  "$native_gateway_handoff_control_spec" \
  'source_command: "/hepta-context-recall-worker-scheduler-handoff --dry-run --json"' \
  "native gateway selected-snippet ControlUi route spec"
assert_text_contains \
  "$native_gateway_handoff_control_spec" \
  'capability: "hepta-context-recall-worker-scheduler-handoff"' \
  "native gateway selected-snippet ControlUi route spec"
assert_text_contains \
  "$native_gateway_handoff_control_spec" \
  'without running workers, invoking models, injecting snippets, writing registries, or promoting stable schema' \
  "native gateway selected-snippet ControlUi route spec"

assert_text_contains \
  "$native_gateway_handoff_report" \
  'source_command: "/hepta-context-recall-worker-scheduler-handoff --dry-run --json"' \
  "native gateway selected-snippet dry-run route"
assert_text_contains \
  "$native_gateway_handoff_report" \
  'blockers.push("native_gateway_route_is_plan_only_no_worker_execution")' \
  "native gateway selected-snippet dry-run route"
assert_text_contains \
  "$native_gateway_handoff_report" \
  "side_effect_free: true" \
  "native gateway selected-snippet dry-run route"
assert_text_contains \
  "$native_gateway_handoff_report" \
  "route_executes_scheduler: false" \
  "native gateway selected-snippet dry-run route"
assert_text_contains \
  "$native_gateway_handoff_report" \
  "route_runs_worker_task: false" \
  "native gateway selected-snippet dry-run route"
assert_text_contains \
  "$native_gateway_handoff_report" \
  "route_invokes_model: false" \
  "native gateway selected-snippet dry-run route"
assert_text_contains \
  "$native_gateway_handoff_report" \
  "route_injects_selected_snippets: false" \
  "native gateway selected-snippet dry-run route"
assert_text_contains \
  "$native_gateway_handoff_report" \
  "stable_schema_promoted: false" \
  "native gateway selected-snippet dry-run route"
assert_text_contains \
  "$native_gateway_handoff_report" \
  "tui_exec_app_server_defaults_none: true" \
  "native gateway selected-snippet dry-run route"
assert_text_contains \
  "$native_gateway_handoff_report" \
  "selected_snippet_text_exposed: false" \
  "native gateway selected-snippet dry-run route"
assert_text_contains \
  "$native_gateway_handoff_report" \
  "source_ids_exposed: false" \
  "native gateway selected-snippet dry-run route"
assert_text_contains \
  "$native_gateway_handoff_report" \
  "query_payload_exposed: false" \
  "native gateway selected-snippet dry-run route"
assert_text_contains \
  "$native_gateway_handoff_report" \
  "selected_snippets_injected: false" \
  "native gateway selected-snippet dry-run route side effects"
assert_text_contains \
  "$native_gateway_handoff_report" \
  "filesystem_written: false" \
  "native gateway selected-snippet dry-run route side effects"

if ! rg -n 'run_context_recall_operator_invocation_command' \
  "$repo_root/codex-rs/hepta-runtime/src/context_recall_operator_invocation.rs" >/dev/null; then
  fail "runtime command facade is missing"
fi

echo "Hepta context selected-snippet surface audit passed"
