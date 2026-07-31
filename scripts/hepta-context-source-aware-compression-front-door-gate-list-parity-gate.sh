#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
front_door_gate="$repo_root/scripts/lib/hepta-context-gates-v1/hepta-context-source-aware-compression-front-door.gate"
front_door_report="$repo_root/scripts/hepta-context-source-aware-compression-front-door-report.sh"
front_door_report_source="$repo_root/scripts/lib/hepta-context-gates-v1/hepta-context-source-aware-compression-front-door.report"
context_contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"
release_manifest="$repo_root/codex-rs/CONTEXT_LANE_RELEASE_MANIFEST.tsv"
gate_list_parity_script="hepta-context-source-aware-compression-front-door-gate-list-parity-gate.sh"
report_output="$(mktemp -t hepta-context-source-aware-gate-list-parity.XXXXXX)"

cleanup() {
  rm -f "$report_output"
}
trap cleanup EXIT

fail() {
  echo "hepta-context-source-aware-compression-front-door-gate-list-parity-gate: $*" >&2
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

  line="$(awk -v needle="$needle" 'index($0, needle) { print NR; exit }' "$file_path")"
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

derive_front_door_gate_list() {
  awk '
    function map_gate_label(label) {
      if (label == "readiness checklist") {
        return "readiness"
      }
      if (label == "operator approval evidence") {
        return "operator-approval-evidence"
      }
      if (label == "readiness export surface") {
        return "readiness-export"
      }
      if (label == "activation negative matrix") {
        return "activation-negative-matrix"
      }
      if (label == "activation surface audit") {
        return "activation-surface"
      }
      if (label == "leak bait") {
        return "leak-bait"
      }
      if (label == "positive route readiness") {
        return "positive-route-readiness"
      }
      if (label == "positive route implementation-change detector") {
        return "positive-route-change-detector"
      }
      print "unknown front-door gate label: " label > "/dev/stderr"
      exit 2
    }

    $1 == "run_contract_gate" {
      if ((getline label_line) <= 0) {
        print "missing front-door gate label after run_contract_gate" > "/dev/stderr"
        exit 2
      }
      sub(/^[[:space:]]*"/, "", label_line)
      sub(/"[[:space:]]*\\?[[:space:]]*$/, "", label_line)
      print map_gate_label(label_line)
    }
  ' "$front_door_gate"
}

derive_preflight_source_aware_gate_list() {
  awk '
    function map_preflight_label(label) {
      if (label == "source-aware compression readiness checklist gate") {
        return "readiness"
      }
      if (label == "source-aware compression operator approval evidence contract gate") {
        return "operator-approval-evidence"
      }
      if (label == "source-aware compression readiness export surface gate") {
        return "readiness-export"
      }
      if (label == "source-aware compression activation negative matrix contract gate") {
        return "activation-negative-matrix"
      }
      if (label == "source-aware compression activation surface audit") {
        return "activation-surface"
      }
      if (label == "source-aware compression leak bait contract gate") {
        return "leak-bait"
      }
      if (label == "source-aware compression positive route readiness review gate") {
        return "positive-route-readiness"
      }
      if (label == "source-aware compression positive route implementation-change detector") {
        return "positive-route-change-detector"
      }
      return ""
    }

    /run_stage "/ {
      label = $0
      sub(/^.*run_stage "/, "", label)
      sub(/"[[:space:]]*\\?[[:space:]]*$/, "", label)
      token = map_preflight_label(label)
      if (token != "") {
        print token
      }
    }
  ' "$preflight_script"
}

join_csv() {
  awk '
    NF {
      if (seen) {
        printf ","
      }
      printf "%s", $0
      seen = 1
    }
    END {
      printf "\n"
    }
  '
}

actual_gates="$(derive_front_door_gate_list)" || fail "failed to derive source-aware front-door gate list"
actual_gate_count="$(printf '%s\n' "$actual_gates" | awk 'NF { count++ } END { print count + 0 }')"
if [ "$actual_gate_count" != "8" ]; then
  fail "expected 8 source-aware front-door gates, derived $actual_gate_count: $actual_gates"
fi

duplicate_gate="$(printf '%s\n' "$actual_gates" | awk 'NF && seen[$0]++ { print; exit }')"
if [ -n "$duplicate_gate" ]; then
  fail "duplicate derived source-aware front-door gate token: $duplicate_gate"
fi

actual_gates_csv="$(printf '%s\n' "$actual_gates" | join_csv)"
old_seven_gate_csv="readiness,operator-approval-evidence,activation-negative-matrix,activation-surface,leak-bait,positive-route-readiness,positive-route-change-detector"

preflight_gates="$(derive_preflight_source_aware_gate_list)" || fail "failed to derive source-aware preflight gate list"
preflight_gate_count="$(printf '%s\n' "$preflight_gates" | awk 'NF { count++ } END { print count + 0 }')"
if [ "$preflight_gate_count" != "$actual_gate_count" ]; then
  fail "preflight source-aware gate count drift: preflight=$preflight_gate_count front-door=$actual_gate_count preflight-gates=$preflight_gates"
fi

duplicate_preflight_gate="$(printf '%s\n' "$preflight_gates" | awk 'NF && seen[$0]++ { print; exit }')"
if [ -n "$duplicate_preflight_gate" ]; then
  fail "duplicate derived source-aware preflight gate token: $duplicate_preflight_gate"
fi

missing_preflight_gate="$(
  awk '
    NR == FNR && NF {
      expected[$0] = 1
      next
    }
    NF {
      seen[$0] = 1
    }
    END {
      for (gate in expected) {
        if (!seen[gate]) {
          print gate
          exit
        }
      }
    }
  ' <(printf '%s\n' "$actual_gates") <(printf '%s\n' "$preflight_gates")
)"
if [ -n "$missing_preflight_gate" ]; then
  fail "preflight is missing source-aware gate from front-door list: $missing_preflight_gate"
fi

unexpected_preflight_gate="$(
  awk '
    NR == FNR && NF {
      expected[$0] = 1
      next
    }
    NF && !expected[$0] {
      print
      exit
    }
  ' <(printf '%s\n' "$actual_gates") <(printf '%s\n' "$preflight_gates")
)"
if [ -n "$unexpected_preflight_gate" ]; then
  fail "preflight has source-aware gate outside front-door list: $unexpected_preflight_gate"
fi

if ! bash "$front_door_report" >"$report_output" 2>&1; then
  fail "front-door report command failed"
fi

report_gates_count="$(
  awk -F= '$1 == "source-aware-contracts.gates" { count++ } END { print count + 0 }' "$report_output"
)"
if [ "$report_gates_count" != "1" ]; then
  fail "expected exactly one source-aware-contracts.gates line, got $report_gates_count"
fi

report_gates="$(
  awk -F= '$1 == "source-aware-contracts.gates" { print $2; exit }' "$report_output"
)"
if [ "$report_gates" != "$actual_gates_csv" ]; then
  fail "front-door report gate list drift: report=$report_gates actual=$actual_gates_csv"
fi

gate_list_owner_files=(
  "$front_door_report_source"
  "$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-gate.sh"
  "$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-negative-gate.sh"
  "$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-fixture-matrix-gate.sh"
  "$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-gate.sh"
  "$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-export-gate.sh"
  "$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-export-idempotence-gate.sh"
  "$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-export-atomic-gate.sh"
  "$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-export-symlink-gate.sh"
  "$repo_root/scripts/hepta-context-source-aware-compression-front-door-report-status-artifact-export-hardlink-gate.sh"
)

for owner_file in "${gate_list_owner_files[@]}"; do
  assert_file_contains \
    "$owner_file" \
    "$actual_gates_csv" \
    "source-aware front-door gate-list owner $(basename "$owner_file")"
  if grep -F "$old_seven_gate_csv" "$owner_file" >/dev/null; then
    fail "source-aware front-door gate-list owner $(basename "$owner_file") still contains old seven-gate summary"
  fi
done

assert_file_contains \
  "$context_contracts" \
  "Source-aware compression front-door gate-list parity" \
  "source-aware compression front-door gate-list parity contract"
assert_file_contains \
  "$context_contracts" \
  "$actual_gates_csv" \
  "source-aware compression front-door gate-list parity contract"
assert_file_contains \
  "$context_contracts" \
  "the same eight gate tokens must be derivable" \
  "source-aware compression front-door gate-list parity preflight contract"

assert_file_contains \
  "$debug_gate" \
  "$gate_list_parity_script" \
  "source-aware compression front-door gate-list parity debug gate"
assert_line_before \
  "$debug_gate" \
  "hepta-context-source-aware-compression-front-door-report-status-artifact-export-hardlink-gate.sh" \
  "$gate_list_parity_script" \
  "source-aware compression front-door gate-list parity debug gate order"
assert_line_before \
  "$debug_gate" \
  "$gate_list_parity_script" \
  "hepta-context-source-aware-compression-readiness-gate.sh" \
  "source-aware compression front-door gate-list parity debug gate order"

assert_file_contains \
  "$preflight_script" \
  "source-aware compression front-door gate-list parity" \
  "source-aware compression front-door gate-list parity preflight stage"
assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door report persisted status artifact export hardlink replacement" \
  "source-aware compression front-door gate-list parity" \
  "source-aware compression front-door gate-list parity preflight stage order"
assert_line_before \
  "$preflight_script" \
  "source-aware compression front-door gate-list parity" \
  "source-aware compression readiness checklist gate" \
  "source-aware compression front-door gate-list parity preflight stage order"

assert_file_contains \
  "$front_door_gate" \
  "$gate_list_parity_script" \
  "source-aware compression front-door gate-list parity front-door static coverage"
assert_file_contains \
  "$release_manifest" \
  "scripts/$gate_list_parity_script" \
  "source-aware compression front-door gate-list parity release manifest"

echo "source-aware-front-door-gate-list-parity=pass"
echo "source-aware-front-door-gate-list-parity.gate-count=$actual_gate_count"
echo "source-aware-front-door-gate-list-parity.gates=$actual_gates_csv"
echo "source-aware-front-door-gate-list-parity.preflight-gates=$actual_gates_csv"
echo "source-aware-front-door-gate-list-parity.runtime-activation=disabled"
echo "Hepta context source-aware compression front-door gate-list parity gate passed"
