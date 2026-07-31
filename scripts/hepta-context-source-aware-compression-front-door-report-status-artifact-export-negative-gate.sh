#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
artifact_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-gate.sh"
context_contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
front_door_gate="$repo_root/scripts/lib/hepta-context-gates-v1/hepta-context-source-aware-compression-front-door.gate"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
negative_gate_script="hepta-context-source-aware-compression-front-door-report-status-artifact-export-negative-gate.sh"
artifact_out_env="HEPTA_CONTEXT_SOURCE_AWARE_COMPRESSION_STATUS_ARTIFACT_OUT"
tmp_dir="$(mktemp -d -t hepta-context-source-aware-status-artifact-negative.XXXXXX)"

cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

fail() {
  echo "hepta-context-source-aware-compression-front-door-report-status-artifact-export-negative-gate: $*" >&2
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

run_expected_failure() {
  local label="$1"
  local artifact_out_path="$2"
  local expected_error="$3"
  local output_file="$tmp_dir/$label.output"

  if env "$artifact_out_env=$artifact_out_path" bash "$artifact_gate" >"$output_file" 2>&1; then
    fail "$label should have failed"
  fi

  if ! grep -F "$expected_error" "$output_file" >/dev/null; then
    echo "$label output:" >&2
    cat "$output_file" >&2
    fail "$label did not report expected error: $expected_error"
  fi
}

required_contract_terms=(
  "Source-aware compression persisted status artifact export negative matrix"
  "$artifact_out_env"
  "directory target"
  "missing parent directory"
  "runtime activation disabled"
  "must not become a runtime activation route"
)

for term in "${required_contract_terms[@]}"; do
  assert_file_contains \
    "$context_contracts" \
    "$term" \
    "source-aware compression persisted status artifact export negative matrix contract"
done

assert_file_contains \
  "$front_door_gate" \
  "$negative_gate_script" \
  "source-aware compression status artifact export negative front-door wiring"

assert_file_contains \
  "$debug_gate" \
  "$negative_gate_script" \
  "source-aware compression status artifact export negative debug gate"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export negative matrix" \
  "source-aware compression status artifact export negative preflight stage"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export" \
  "source-aware compression front-door report persisted status artifact export negative matrix" \
  "source-aware compression status artifact export negative preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export negative matrix" \
  "source-aware compression readiness checklist gate" \
  "source-aware compression status artifact export negative preflight stage order"

unexpected_env_refs="$(
  rg -n "$artifact_out_env" "$repo_root/codex-rs" "$repo_root/scripts" | awk -F: '
    $1 !~ /codex-rs\/CONTEXT_DEBUG_CONTRACTS\.md$/ &&
    $1 !~ /scripts\/hepta-context-source-aware-compression-front-door-gate\.sh$/ &&
    $1 !~ /scripts\/lib\/hepta-context-gates-v1\/hepta-context-source-aware-compression-front-door\.gate$/ &&
    $1 !~ /scripts\/hepta-context-source-aware-compression-front-door-report-status-artifact-gate\.sh$/ &&
    $1 !~ /scripts\/hepta-context-source-aware-compression-front-door-report-status-artifact-export-gate\.sh$/ &&
    $1 !~ /scripts\/hepta-context-source-aware-compression-front-door-report-status-artifact-export-negative-gate\.sh$/ &&
    $1 !~ /scripts\/hepta-context-source-aware-compression-front-door-report-status-artifact-export-precheck-gate\.sh$/ &&
    $1 !~ /scripts\/hepta-context-source-aware-compression-front-door-report-status-artifact-export-idempotence-gate\.sh$/ &&
    $1 !~ /scripts\/hepta-context-source-aware-compression-front-door-report-status-artifact-export-atomic-gate\.sh$/ &&
    $1 !~ /scripts\/hepta-context-source-aware-compression-front-door-report-status-artifact-export-writability-precheck-gate\.sh$/ &&
    $1 !~ /scripts\/hepta-context-source-aware-compression-front-door-report-status-artifact-export-symlink-gate\.sh$/ &&
    $1 !~ /scripts\/hepta-context-source-aware-compression-front-door-report-status-artifact-export-hardlink-gate\.sh$/ {
      print
      exit
    }
  '
)"

if [ -n "$unexpected_env_refs" ]; then
  fail "$artifact_out_env may only appear in the status artifact export contract/gates: $unexpected_env_refs"
fi

run_expected_failure \
  "directory-target" \
  "$tmp_dir" \
  "status artifact output path must be a file, got directory"

missing_parent_path="$tmp_dir/missing-parent/source-aware-contracts.status"
run_expected_failure \
  "missing-parent-directory" \
  "$missing_parent_path" \
  "status artifact output directory does not exist"

if [ -e "$missing_parent_path" ]; then
  fail "missing parent directory case must not create an artifact: $missing_parent_path"
fi

echo "Hepta context source-aware compression persisted status artifact export negative gate passed"
