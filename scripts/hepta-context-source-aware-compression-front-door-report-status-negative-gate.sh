#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
status_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-gate.sh"
tmp_dir="$(mktemp -d -t hepta-context-source-aware-status-negative.XXXXXX)"

cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

fail() {
  echo "hepta-context-source-aware-compression-front-door-report-status-negative-gate: $*" >&2
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

write_report() {
  local file_path="$1"
  shift
  {
    echo "source-aware-contracts=pass"
    echo "source-aware-contracts.front-door=pass"
    echo "source-aware-contracts.runtime-dirty-classifier=non-blocking"
    echo "source-aware-contracts.runtime-activation=disabled"
    echo "source-aware-contracts.gates=readiness,operator-approval-evidence,readiness-export,activation-negative-matrix,activation-surface,leak-bait,positive-route-readiness,positive-route-change-detector"
    for extra_line in "$@"; do
      echo "$extra_line"
    done
  } >"$file_path"
}

run_synthetic_expect_success() {
  local name="$1"
  local report_path="$tmp_dir/$name.report"

  write_report "$report_path"
  HEPTA_CONTEXT_SOURCE_AWARE_COMPRESSION_FRONT_DOOR_REPORT_FILE="$report_path" \
    bash "$status_gate" >/dev/null
}

run_synthetic_expect_failure() {
  local name="$1"
  local expected_error="$2"
  local report_path="$tmp_dir/$name.report"
  local output_path="$tmp_dir/$name.output"
  shift 2

  write_report "$report_path" "$@"
  case "$name" in
    duplicate-status)
      echo "source-aware-contracts=pass" >>"$report_path"
      ;;
    missing-gate-list)
      grep -v '^source-aware-contracts\.gates=' "$report_path" >"$report_path.tmp"
      mv "$report_path.tmp" "$report_path"
      ;;
    malformed-classifier)
      sed -i.bak 's/^source-aware-contracts\.runtime-dirty-classifier=.*/source-aware-contracts.runtime-dirty-classifier=dirty/' "$report_path"
      rm -f "$report_path.bak"
      ;;
    runtime-activation-enabled)
      sed -i.bak 's/^source-aware-contracts\.runtime-activation=.*/source-aware-contracts.runtime-activation=enabled/' "$report_path"
      rm -f "$report_path.bak"
      ;;
    *)
      fail "unknown synthetic failure case: $name"
      ;;
  esac

  if HEPTA_CONTEXT_SOURCE_AWARE_COMPRESSION_FRONT_DOOR_REPORT_FILE="$report_path" \
    bash "$status_gate" >"$output_path" 2>&1; then
    fail "synthetic bad report unexpectedly passed: $name"
  fi

  assert_file_contains \
    "$output_path" \
    "$expected_error" \
    "synthetic bad report failure for $name"
}

context_contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
negative_gate_script="hepta-context-source-aware-compression-front-door-report-status-negative-gate.sh"

required_contract_terms=(
  "Source-aware compression front-door report status negative harness"
  "duplicate source-aware-contracts"
  "missing gate list"
  "malformed classifier"
  "runtime-activation=enabled"
  "synthetic report"
)

for term in "${required_contract_terms[@]}"; do
  assert_file_contains \
    "$context_contracts" \
    "$term" \
    "source-aware compression front-door report status negative harness contract"
done

assert_file_contains \
  "$debug_gate" \
  "$negative_gate_script" \
  "source-aware compression front-door report status negative debug gate"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression front-door report status negative harness" \
  "source-aware compression front-door report status negative preflight stage"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report status assertion" \
  "source-aware compression front-door report status negative harness" \
  "source-aware compression front-door report status negative preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report status negative harness" \
  "source-aware compression readiness checklist gate" \
  "source-aware compression front-door report status negative preflight stage order"

run_synthetic_expect_success "good"
run_synthetic_expect_failure \
  "duplicate-status" \
  "expected exactly one line: source-aware-contracts=pass"
run_synthetic_expect_failure \
  "missing-gate-list" \
  "expected exactly one source-aware-contracts.gates line"
run_synthetic_expect_failure \
  "malformed-classifier" \
  "expected source-aware-contracts.runtime-dirty-classifier to match none|non-blocking"
run_synthetic_expect_failure \
  "runtime-activation-enabled" \
  "expected source-aware-contracts.runtime-activation=disabled"

echo "Hepta context source-aware compression front-door report status negative gate passed"
