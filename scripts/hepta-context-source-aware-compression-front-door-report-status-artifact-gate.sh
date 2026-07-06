#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
front_door_report="$repo_root/scripts/hepta-context-source-aware-compression-front-door-report.sh"
status_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-gate.sh"
report_output="$(mktemp -t hepta-context-source-aware-front-door-report-output.XXXXXX)"
status_artifact="$(mktemp -t hepta-context-source-aware-front-door-status-artifact.XXXXXX)"
artifact_out_path="${HEPTA_CONTEXT_SOURCE_AWARE_COMPRESSION_STATUS_ARTIFACT_OUT:-}"
expected_gates="readiness,operator-approval-evidence,readiness-export,activation-negative-matrix,activation-surface,leak-bait,positive-route-readiness,positive-route-change-detector"

cleanup() {
  rm -f "$report_output" "$status_artifact"
}
trap cleanup EXIT

fail() {
  echo "hepta-context-source-aware-compression-front-door-report-status-artifact-gate: $*" >&2
  if [ -s "$report_output" ]; then
    echo "front-door report output:" >&2
    cat "$report_output" >&2
  fi
  if [ -s "$status_artifact" ]; then
    echo "source-aware status artifact:" >&2
    cat "$status_artifact" >&2
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

assert_artifact_line() {
  local line_number="$1"
  local expected="$2"
  local actual

  actual="$(sed -n "${line_number}p" "$status_artifact")"
  if [ "$actual" != "$expected" ]; then
    fail "status artifact line $line_number expected '$expected', got '$actual'"
  fi
}

precheck_artifact_output_path() {
  if [ -z "$artifact_out_path" ]; then
    return
  fi

  if [ -d "$artifact_out_path" ]; then
    fail "status artifact output path must be a file, got directory: $artifact_out_path"
  fi

  artifact_out_dir="$(dirname "$artifact_out_path")"
  if [ ! -d "$artifact_out_dir" ]; then
    fail "status artifact output directory does not exist: $artifact_out_dir"
  fi

  if [ ! -w "$artifact_out_dir" ]; then
    fail "status artifact output directory is not writable: $artifact_out_dir"
  fi
}

persist_status_artifact() {
  if [ -z "$artifact_out_path" ]; then
    return
  fi

  local artifact_out_dir
  local artifact_out_base
  local artifact_out_tmp

  artifact_out_dir="$(dirname "$artifact_out_path")"
  artifact_out_base="$(basename "$artifact_out_path")"
  artifact_out_tmp="$(mktemp "$artifact_out_dir/.${artifact_out_base}.tmp.XXXXXX")"

  if ! cp "$status_artifact" "$artifact_out_tmp"; then
    rm -f "$artifact_out_tmp"
    fail "failed to write status artifact temporary file: $artifact_out_tmp"
  fi

  if ! mv -f "$artifact_out_tmp" "$artifact_out_path"; then
    rm -f "$artifact_out_tmp"
    fail "failed to replace status artifact output path: $artifact_out_path"
  fi
}

context_contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
artifact_gate_script="hepta-context-source-aware-compression-front-door-report-status-artifact-gate.sh"

required_contract_terms=(
  "Source-aware compression front-door report status artifact consumer"
  "exactly the five allowlisted source-aware-contracts status lines"
  "no front-door diagnostic noise"
  "real front-door report"
  "runtime activation disabled"
)

for term in "${required_contract_terms[@]}"; do
  assert_file_contains \
    "$context_contracts" \
    "$term" \
    "source-aware compression front-door report status artifact contract"
done

assert_file_contains \
  "$debug_gate" \
  "$artifact_gate_script" \
  "source-aware compression front-door report status artifact debug gate"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression front-door report status artifact consumer" \
  "source-aware compression front-door report status artifact preflight stage"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report status fixture matrix" \
  "source-aware compression front-door report status artifact consumer" \
  "source-aware compression front-door report status artifact preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report status artifact consumer" \
  "source-aware compression readiness checklist gate" \
  "source-aware compression front-door report status artifact preflight stage order"

precheck_artifact_output_path

if ! bash "$front_door_report" >"$report_output" 2>&1; then
  fail "front-door report command failed"
fi

awk 'index($0, "source-aware-contracts") == 1 { print }' "$report_output" >"$status_artifact"

line_count="$(wc -l <"$status_artifact" | tr -d ' ')"
if [ "$line_count" != "5" ]; then
  fail "status artifact expected exactly five source-aware-contracts lines, got $line_count"
fi

if grep -F "front-door:" "$status_artifact" >/dev/null; then
  fail "status artifact must not contain front-door diagnostic noise"
fi

if grep -F "hepta-context-source-aware-compression-front-door-gate" "$status_artifact" >/dev/null; then
  fail "status artifact must not contain front-door gate diagnostic noise"
fi

if grep -v '^source-aware-contracts' "$status_artifact" >/dev/null; then
  fail "status artifact must contain only source-aware-contracts lines"
fi

assert_artifact_line 1 "source-aware-contracts=pass"
assert_artifact_line 2 "source-aware-contracts.front-door=pass"

runtime_dirty_line="$(sed -n '3p' "$status_artifact")"
case "$runtime_dirty_line" in
  source-aware-contracts.runtime-dirty-classifier=none | \
  source-aware-contracts.runtime-dirty-classifier=non-blocking)
    ;;
  *)
    fail "status artifact runtime dirty classifier line malformed: $runtime_dirty_line"
    ;;
esac

assert_artifact_line 4 "source-aware-contracts.runtime-activation=disabled"
assert_artifact_line 5 "source-aware-contracts.gates=$expected_gates"

HEPTA_CONTEXT_SOURCE_AWARE_COMPRESSION_FRONT_DOOR_REPORT_FILE="$status_artifact" \
  bash "$status_gate" >/dev/null

persist_status_artifact

echo "Hepta context source-aware compression front-door report status artifact gate passed"
