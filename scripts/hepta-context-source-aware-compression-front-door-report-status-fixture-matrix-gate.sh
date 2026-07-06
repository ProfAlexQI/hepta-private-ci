#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
status_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-gate.sh"
tmp_dir="$(mktemp -d -t hepta-context-source-aware-status-matrix.XXXXXX)"
expected_gates="readiness,operator-approval-evidence,readiness-export,activation-negative-matrix,activation-surface,leak-bait,positive-route-readiness,positive-route-change-detector"

cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

fail() {
  echo "hepta-context-source-aware-compression-front-door-report-status-fixture-matrix-gate: $*" >&2
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

write_good_report() {
  local file_path="$1"

  {
    echo "front-door: synthetic preamble"
    echo "source-aware-contracts=pass"
    echo "source-aware-contracts.front-door=pass"
    echo "source-aware-contracts.runtime-dirty-classifier=non-blocking"
    echo "source-aware-contracts.runtime-activation=disabled"
    echo "source-aware-contracts.gates=$expected_gates"
  } >"$file_path"
}

write_reordered_report() {
  local file_path="$1"

  {
    echo "front-door: synthetic preamble"
    echo "source-aware-contracts=pass"
    echo "source-aware-contracts.runtime-dirty-classifier=non-blocking"
    echo "source-aware-contracts.front-door=pass"
    echo "source-aware-contracts.runtime-activation=disabled"
    echo "source-aware-contracts.gates=$expected_gates"
  } >"$file_path"
}

run_synthetic_expect_success() {
  local name="$1"
  local report_path="$tmp_dir/$name.report"

  write_good_report "$report_path"
  HEPTA_CONTEXT_SOURCE_AWARE_COMPRESSION_FRONT_DOOR_REPORT_FILE="$report_path" \
    bash "$status_gate" >/dev/null
}

run_synthetic_expect_failure() {
  local name="$1"
  local expected_error="$2"
  local report_path="$tmp_dir/$name.report"
  local output_path="$tmp_dir/$name.output"

  case "$name" in
    unknown-extra-status-key)
      write_good_report "$report_path"
      echo "source-aware-contracts.unexpected=present" >>"$report_path"
      ;;
    reordered-machine-readable-block)
      write_reordered_report "$report_path"
      ;;
    duplicated-machine-readable-block)
      write_good_report "$report_path"
      write_good_report "$report_path.second"
      cat "$report_path.second" >>"$report_path"
      ;;
    *)
      fail "unknown synthetic fixture matrix case: $name"
      ;;
  esac

  if HEPTA_CONTEXT_SOURCE_AWARE_COMPRESSION_FRONT_DOOR_REPORT_FILE="$report_path" \
    bash "$status_gate" >"$output_path" 2>&1; then
    fail "synthetic fixture matrix bad report unexpectedly passed: $name"
  fi

  assert_file_contains \
    "$output_path" \
    "$expected_error" \
    "synthetic fixture matrix failure for $name"
}

context_contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
fixture_matrix_script="hepta-context-source-aware-compression-front-door-report-status-fixture-matrix-gate.sh"

required_contract_terms=(
  "Source-aware compression front-door report status fixture matrix"
  "unknown extra source-aware-contracts.* key"
  "reordered machine-readable block"
  "duplicated machine-readable block"
  "synthetic input seam"
  "stable machine-readable status order"
)

for term in "${required_contract_terms[@]}"; do
  assert_file_contains \
    "$context_contracts" \
    "$term" \
    "source-aware compression front-door report status fixture matrix contract"
done

assert_file_contains \
  "$debug_gate" \
  "$fixture_matrix_script" \
  "source-aware compression front-door report status fixture matrix debug gate"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression front-door report status fixture matrix" \
  "source-aware compression front-door report status fixture matrix preflight stage"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report status negative harness" \
  "source-aware compression front-door report status fixture matrix" \
  "source-aware compression front-door report status fixture matrix preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report status fixture matrix" \
  "source-aware compression readiness checklist gate" \
  "source-aware compression front-door report status fixture matrix preflight stage order"

run_synthetic_expect_success "good"
run_synthetic_expect_failure \
  "unknown-extra-status-key" \
  "unexpected machine-readable source-aware-contracts key"
run_synthetic_expect_failure \
  "reordered-machine-readable-block" \
  "source-aware compression report status order expected source-aware-contracts.front-door before source-aware-contracts.runtime-dirty-classifier"
run_synthetic_expect_failure \
  "duplicated-machine-readable-block" \
  "expected exactly one line: source-aware-contracts=pass"

echo "Hepta context source-aware compression front-door report status fixture matrix gate passed"
