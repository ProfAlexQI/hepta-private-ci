#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
artifact_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-gate.sh"
status_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-gate.sh"
context_contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
front_door_gate="$repo_root/scripts/hepta-context-source-aware-compression-front-door-gate.sh"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
symlink_gate_script="hepta-context-source-aware-compression-front-door-report-status-artifact-export-symlink-gate.sh"
artifact_out_env="HEPTA_CONTEXT_SOURCE_AWARE_COMPRESSION_STATUS_ARTIFACT_OUT"
expected_gates="readiness,operator-approval-evidence,readiness-export,activation-negative-matrix,activation-surface,leak-bait,positive-route-readiness,positive-route-change-detector"
tmp_dir="$(mktemp -d -t hepta-context-source-aware-status-artifact-symlink.XXXXXX)"
artifact_out="$tmp_dir/source-aware-contracts.status"
victim_file="$tmp_dir/symlink-victim.txt"
expected_victim="$tmp_dir/expected-symlink-victim.txt"
gate_output="$tmp_dir/artifact-gate-output.txt"

cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

fail() {
  echo "hepta-context-source-aware-compression-front-door-report-status-artifact-export-symlink-gate: $*" >&2
  if [ -e "$artifact_out" ] || [ -L "$artifact_out" ]; then
    echo "persisted source-aware status artifact path:" >&2
    ls -l "$artifact_out" >&2 || true
    if [ -f "$artifact_out" ] && [ ! -L "$artifact_out" ]; then
      cat "$artifact_out" >&2
    fi
  fi
  if [ -f "$victim_file" ]; then
    echo "symlink victim file:" >&2
    cat "$victim_file" >&2
  fi
  if [ -s "$gate_output" ]; then
    echo "artifact gate output:" >&2
    cat "$gate_output" >&2
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

  actual="$(sed -n "${line_number}p" "$artifact_out")"
  if [ "$actual" != "$expected" ]; then
    fail "persisted status artifact line $line_number expected '$expected', got '$actual'"
  fi
}

assert_persisted_artifact_contract() {
  if [ -L "$artifact_out" ]; then
    fail "persisted status artifact output path must replace the symlink itself"
  fi

  if [ ! -s "$artifact_out" ]; then
    fail "persisted status artifact was not written"
  fi

  line_count="$(wc -l <"$artifact_out" | tr -d ' ')"
  if [ "$line_count" != "5" ]; then
    fail "persisted status artifact expected exactly five source-aware-contracts lines, got $line_count"
  fi

  if grep -F "front-door:" "$artifact_out" >/dev/null; then
    fail "persisted status artifact must not contain front-door diagnostic noise"
  fi

  if grep -F "source-aware-contracts.runtime-activation=enabled" "$artifact_out" >/dev/null; then
    fail "persisted status artifact must not enable runtime activation"
  fi

  if grep -v '^source-aware-contracts' "$artifact_out" >/dev/null; then
    fail "persisted status artifact must contain only source-aware-contracts lines"
  fi

  assert_artifact_line 1 "source-aware-contracts=pass"
  assert_artifact_line 2 "source-aware-contracts.front-door=pass"

  runtime_dirty_line="$(sed -n '3p' "$artifact_out")"
  case "$runtime_dirty_line" in
    source-aware-contracts.runtime-dirty-classifier=none | \
    source-aware-contracts.runtime-dirty-classifier=non-blocking)
      ;;
    *)
      fail "persisted status artifact runtime dirty classifier line malformed: $runtime_dirty_line"
      ;;
  esac

  assert_artifact_line 4 "source-aware-contracts.runtime-activation=disabled"
  assert_artifact_line 5 "source-aware-contracts.gates=$expected_gates"

  HEPTA_CONTEXT_SOURCE_AWARE_COMPRESSION_FRONT_DOOR_REPORT_FILE="$artifact_out" \
    bash "$status_gate" >/dev/null
}

required_contract_terms=(
  "Source-aware compression persisted status artifact export symlink replacement"
  "$artifact_out_env"
  "symlink"
  "replace the symlink itself"
  "must not follow the symlink"
  "victim file"
  "runtime activation disabled"
  "must not become a runtime activation route"
)

for term in "${required_contract_terms[@]}"; do
  assert_file_contains \
    "$context_contracts" \
    "$term" \
    "source-aware compression persisted status artifact export symlink replacement contract"
done

assert_file_contains \
  "$front_door_gate" \
  "$symlink_gate_script" \
  "source-aware compression status artifact export symlink front-door wiring"

assert_file_contains \
  "$debug_gate" \
  "$symlink_gate_script" \
  "source-aware compression status artifact export symlink debug gate"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export symlink replacement" \
  "source-aware compression status artifact export symlink preflight stage"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export writability precheck" \
  "source-aware compression front-door report persisted status artifact export symlink replacement" \
  "source-aware compression status artifact export symlink preflight stage order"

assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export symlink replacement" \
  "source-aware compression readiness checklist gate" \
  "source-aware compression status artifact export symlink preflight stage order"

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

{
  echo "front-door: stale-source-aware-status victim diagnostic"
  echo "source-aware-contracts.runtime-activation=enabled"
  echo "symlink-victim-should-not-change"
} >"$victim_file"
cp "$victim_file" "$expected_victim"
ln -s "$victim_file" "$artifact_out"

if ! env "$artifact_out_env=$artifact_out" bash "$artifact_gate" >"$gate_output" 2>&1; then
  fail "status artifact gate failed with symlink output path"
fi

assert_persisted_artifact_contract

if ! cmp -s "$victim_file" "$expected_victim"; then
  fail "status artifact export must not follow the symlink or mutate the victim file"
fi

echo "Hepta context source-aware compression persisted status artifact export symlink gate passed"
