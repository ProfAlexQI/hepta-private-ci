#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
front_door_report="$repo_root/scripts/hepta-context-source-aware-compression-front-door-report.sh"
synthetic_report_file="${HEPTA_CONTEXT_SOURCE_AWARE_COMPRESSION_FRONT_DOOR_REPORT_FILE:-}"
report_output="$(mktemp -t hepta-context-source-aware-front-door-report.XXXXXX)"

cleanup() {
  rm -f "$report_output"
}
trap cleanup EXIT

fail() {
  echo "hepta-context-source-aware-compression-front-door-report-status-gate: $*" >&2
  if [ -s "$report_output" ]; then
    echo "front-door report output:" >&2
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

assert_exact_line_once() {
  local expected="$1"
  local label="$2"
  local count

  count="$(grep -x -F "$expected" "$report_output" | wc -l | tr -d ' ')"
  if [ "$count" != "1" ]; then
    fail "$label expected exactly one line: $expected"
  fi
}

assert_key_value_once() {
  local key="$1"
  local expected_value="$2"
  local label="$3"
  local count
  local actual_line

  count="$(awk -v prefix="$key=" 'index($0, prefix) == 1 { count++ } END { print count + 0 }' "$report_output")"
  if [ "$count" != "1" ]; then
    fail "$label expected exactly one $key line"
  fi
  actual_line="$(awk -v prefix="$key=" 'index($0, prefix) == 1 { print; exit }' "$report_output")"
  if [ "$actual_line" != "$key=$expected_value" ]; then
    fail "$label expected $key=$expected_value, got $actual_line"
  fi
}

assert_key_value_regex_once() {
  local key="$1"
  local pattern="$2"
  local label="$3"
  local count
  local actual_line
  local value

  count="$(awk -v prefix="$key=" 'index($0, prefix) == 1 { count++ } END { print count + 0 }' "$report_output")"
  if [ "$count" != "1" ]; then
    fail "$label expected exactly one $key line"
  fi
  actual_line="$(awk -v prefix="$key=" 'index($0, prefix) == 1 { print; exit }' "$report_output")"
  value="${actual_line#"$key="}"
  if ! printf '%s\n' "$value" | grep -E -x "$pattern" >/dev/null; then
    fail "$label expected $key to match $pattern, got $value"
  fi
}

assert_no_unknown_machine_readable_status_keys() {
  local unexpected

  unexpected="$(
    awk '
      BEGIN {
        allowed["source-aware-contracts"] = 1
        allowed["source-aware-contracts.front-door"] = 1
        allowed["source-aware-contracts.runtime-dirty-classifier"] = 1
        allowed["source-aware-contracts.runtime-activation"] = 1
        allowed["source-aware-contracts.gates"] = 1
      }
      index($0, "source-aware-contracts") == 1 {
        key = $0
        sub(/=.*/, "", key)
        if (!(key in allowed)) {
          print $0
          exit
        }
      }
    ' "$report_output"
  )"

  if [ -n "$unexpected" ]; then
    fail "unexpected machine-readable source-aware-contracts key: $unexpected"
  fi
}

status_key_line_number() {
  local key="$1"
  local line

  line="$(awk -v prefix="$key=" 'index($0, prefix) == 1 { print NR; exit }' "$report_output")"
  if [ -z "$line" ]; then
    fail "source-aware compression report status order missing $key line"
  fi
  printf '%s\n' "$line"
}

assert_status_key_before() {
  local before_key="$1"
  local after_key="$2"
  local before_line
  local after_line

  before_line="$(status_key_line_number "$before_key")"
  after_line="$(status_key_line_number "$after_key")"
  if [ "$before_line" -ge "$after_line" ]; then
    fail "source-aware compression report status order expected $before_key before $after_key"
  fi
}

assert_machine_readable_status_order() {
  assert_status_key_before \
    "source-aware-contracts" \
    "source-aware-contracts.front-door"
  assert_status_key_before \
    "source-aware-contracts.front-door" \
    "source-aware-contracts.runtime-dirty-classifier"
  assert_status_key_before \
    "source-aware-contracts.runtime-dirty-classifier" \
    "source-aware-contracts.runtime-activation"
  assert_status_key_before \
    "source-aware-contracts.runtime-activation" \
    "source-aware-contracts.gates"
}

context_contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
status_gate_script="hepta-context-source-aware-compression-front-door-report-status-gate.sh"
expected_gates="readiness,operator-approval-evidence,readiness-export,activation-negative-matrix,activation-surface,leak-bait,positive-route-readiness,positive-route-change-detector"

required_contract_terms=(
  "Source-aware compression front-door report status assertion"
  "source-aware-contracts=pass"
  "source-aware-contracts.front-door=pass"
  "source-aware-contracts.runtime-dirty-classifier=none|non-blocking"
  "source-aware-contracts.runtime-activation=disabled"
  "$expected_gates"
  "unknown extra source-aware-contracts.* key"
  "stable machine-readable status order"
)

for term in "${required_contract_terms[@]}"; do
  assert_file_contains \
    "$context_contracts" \
    "$term" \
    "source-aware compression front-door report status assertion contract"
done

assert_file_contains \
  "$debug_gate" \
  "$status_gate_script" \
  "source-aware compression front-door report status debug gate"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression front-door report status assertion" \
  "source-aware compression front-door report status preflight stage"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door machine-readable report" \
  "source-aware compression front-door report status assertion" \
  "source-aware compression front-door report status preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report status assertion" \
  "source-aware compression readiness checklist gate" \
  "source-aware compression front-door report status preflight stage order"

if [ -n "$synthetic_report_file" ]; then
  if [ ! -f "$synthetic_report_file" ]; then
    fail "synthetic front-door report file does not exist: $synthetic_report_file"
  fi
  cat "$synthetic_report_file" >"$report_output"
else
  if ! bash "$front_door_report" >"$report_output" 2>&1; then
    fail "front-door report command failed"
  fi
fi

assert_no_unknown_machine_readable_status_keys

assert_exact_line_once \
  "source-aware-contracts=pass" \
  "source-aware compression report status"

assert_key_value_once \
  "source-aware-contracts.front-door" \
  "pass" \
  "source-aware compression front-door report status"

assert_key_value_regex_once \
  "source-aware-contracts.runtime-dirty-classifier" \
  "none|non-blocking" \
  "source-aware compression runtime dirty classifier"

assert_key_value_once \
  "source-aware-contracts.runtime-activation" \
  "disabled" \
  "source-aware compression runtime activation status"

assert_key_value_once \
  "source-aware-contracts.gates" \
  "$expected_gates" \
  "source-aware compression report gate list"

assert_machine_readable_status_order

echo "Hepta context source-aware compression front-door report status gate passed"
