#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
artifact_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-gate.sh"
context_contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
writability_gate_script="hepta-context-source-aware-compression-front-door-report-status-artifact-export-writability-precheck-gate.sh"
artifact_out_env="HEPTA_CONTEXT_SOURCE_AWARE_COMPRESSION_STATUS_ARTIFACT_OUT"
tmp_dir="$(mktemp -d -t hepta-context-source-aware-status-artifact-writability.XXXXXX)"
readonly_dir="$tmp_dir/read-only-parent"
readonly_artifact="$readonly_dir/source-aware-contracts.status"

cleanup() {
  chmod u+w "$readonly_dir" 2>/dev/null || true
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

fail() {
  echo "hepta-context-source-aware-compression-front-door-report-status-artifact-export-writability-precheck-gate: $*" >&2
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

run_writability_precheck_failure() {
  local output_file="$tmp_dir/unwritable-parent.output"

  if env "$artifact_out_env=$readonly_artifact" bash "$artifact_gate" >"$output_file" 2>&1; then
    fail "unwritable parent directory should have failed"
  fi

  if ! grep -F "status artifact output directory is not writable" "$output_file" >/dev/null; then
    echo "unwritable parent directory output:" >&2
    cat "$output_file" >&2
    fail "unwritable parent directory did not report expected output-path error"
  fi

  if grep -F "front-door:" "$output_file" >/dev/null; then
    echo "unwritable parent directory output:" >&2
    cat "$output_file" >&2
    fail "unwritable parent directory must fail before running the front-door report"
  fi

  if grep -F "source-aware-contracts=" "$output_file" >/dev/null; then
    echo "unwritable parent directory output:" >&2
    cat "$output_file" >&2
    fail "unwritable parent directory must fail before emitting source-aware-contracts status"
  fi

  if grep -F "source-aware-contracts.runtime-activation=enabled" "$output_file" >/dev/null; then
    fail "unwritable parent directory must not enable runtime activation"
  fi
}

required_contract_terms=(
  "Source-aware compression persisted status artifact export writability precheck"
  "$artifact_out_env"
  "unwritable parent directory"
  "before running the real front-door report"
  "before emitting source-aware-contracts status"
  "without creating a final artifact"
  "runtime activation disabled"
  "must not become a runtime activation route"
)

for term in "${required_contract_terms[@]}"; do
  assert_file_contains \
    "$context_contracts" \
    "$term" \
    "source-aware compression persisted status artifact export writability precheck contract"
done

assert_file_contains \
  "$front_door_gate" \
  "$writability_gate_script" \
  "source-aware compression status artifact export writability front-door wiring"

assert_file_contains \
  "$debug_gate" \
  "$writability_gate_script" \
  "source-aware compression status artifact export writability debug gate"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export writability precheck" \
  "source-aware compression status artifact export writability preflight stage"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export atomic replace" \
  "source-aware compression front-door report persisted status artifact export writability precheck" \
  "source-aware compression status artifact export writability preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export writability precheck" \
  "source-aware compression readiness checklist gate" \
  "source-aware compression status artifact export writability preflight stage order"

assert_line_before \
  "$artifact_gate" \
  "status artifact output directory is not writable" \
  'bash "$front_door_report"' \
  "source-aware compression status artifact export writability precheck happens before front-door report"

unexpected_env_refs="$(
  rg -n "$artifact_out_env" "$repo_root/codex-rs" "$repo_root/scripts" | awk -F: '
    $1 !~ /codex-rs\/CONTEXT_DEBUG_CONTRACTS\.md$/ &&
    $1 !~ /scripts\/hepta-context-source-aware-compression-front-door-gate\.sh$/ &&
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

mkdir "$readonly_dir"
chmod u-w "$readonly_dir"

run_writability_precheck_failure

if [ -e "$readonly_artifact" ]; then
  fail "unwritable parent directory case must not create a final artifact: $readonly_artifact"
fi

echo "Hepta context source-aware compression persisted status artifact export writability precheck gate passed"
