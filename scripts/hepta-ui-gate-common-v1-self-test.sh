#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

LIBRARY="scripts/lib/hepta-ui-gate-common-v1.sh"
for command_name in bash cmp jq mktemp rg shasum wc; do
  command -v "$command_name" >/dev/null 2>&1 || {
    printf '%s is required for the Hepta UI gate common helper self-test\n' "$command_name" >&2
    exit 2
  }
done

tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-gate-common-v1.XXXXXX")"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

valid_report="$tmp_dir/valid.json"
invalid_report="$tmp_dir/invalid.json"
stdout_path="$tmp_dir/stdout"
stderr_path="$tmp_dir/stderr"
expected_path="$tmp_dir/expected"
printf '{"status":"ready"}\n' >"$valid_report"
printf '{not-json}\n' >"$invalid_report"

HEPTA_UI_GATE_REQUIREMENT_CONTEXT="the Hepta UI helper self-test gate" \
HEPTA_UI_REPORT_INPUT_LABEL="helper self-test" \
  bash -c 'source "$1"; require_command jq; require_report "$2"' \
  _ "$LIBRARY" "$valid_report"

expected_sha="$(shasum -a 256 "$valid_report" | awk '{print $1}')"
actual_sha="$(
  HEPTA_UI_GATE_REQUIREMENT_CONTEXT="the Hepta UI helper self-test gate" \
  HEPTA_UI_REPORT_INPUT_LABEL="helper self-test" \
    bash -c 'source "$1"; file_sha256 "$2"' _ "$LIBRARY" "$valid_report"
)"
[[ "$actual_sha" == "$expected_sha" ]]

expected_bytes="$(wc -c <"$valid_report" | tr -d ' ')"
actual_bytes="$(
  HEPTA_UI_GATE_REQUIREMENT_CONTEXT="the Hepta UI helper self-test gate" \
  HEPTA_UI_REPORT_INPUT_LABEL="helper self-test" \
    bash -c 'source "$1"; file_bytes "$2"' _ "$LIBRARY" "$valid_report"
)"
[[ "$actual_bytes" == "$expected_bytes" ]]

if HEPTA_UI_GATE_REQUIREMENT_CONTEXT="the Hepta UI helper self-test gate" \
  HEPTA_UI_REPORT_INPUT_LABEL="helper self-test" \
  bash -c 'source "$1"; require_command hepta-ui-command-that-does-not-exist' \
    _ "$LIBRARY" >"$stdout_path" 2>"$stderr_path"; then
  printf 'shared require_command unexpectedly accepted a missing command\n' >&2
  exit 1
else
  status=$?
fi
[[ "$status" -eq 2 ]]
[[ ! -s "$stdout_path" ]]
printf '%s\n' \
  'hepta-ui-command-that-does-not-exist is required for the Hepta UI helper self-test gate' \
  >"$expected_path"
cmp -s "$expected_path" "$stderr_path"

missing_report="$tmp_dir/missing.json"
if HEPTA_UI_GATE_REQUIREMENT_CONTEXT="the Hepta UI helper self-test gate" \
  HEPTA_UI_REPORT_INPUT_LABEL="helper self-test" \
  bash -c 'source "$1"; require_report "$2"' \
    _ "$LIBRARY" "$missing_report" >"$stdout_path" 2>"$stderr_path"; then
  printf 'shared require_report unexpectedly accepted a missing report\n' >&2
  exit 1
else
  status=$?
fi
[[ "$status" -eq 1 ]]
[[ ! -s "$stdout_path" ]]
printf 'Missing required helper self-test input: %s\n' "$missing_report" >"$expected_path"
cmp -s "$expected_path" "$stderr_path"

if HEPTA_UI_GATE_REQUIREMENT_CONTEXT="the Hepta UI helper self-test gate" \
  HEPTA_UI_REPORT_INPUT_LABEL="helper self-test" \
  bash -c 'source "$1"; require_report "$2"' \
    _ "$LIBRARY" "$invalid_report" >/dev/null 2>&1; then
  printf 'shared require_report unexpectedly accepted invalid JSON\n' >&2
  exit 1
fi

if bash -c 'source "$1"' _ "$LIBRARY" >/dev/null 2>"$stderr_path"; then
  printf 'shared helper unexpectedly accepted missing configuration\n' >&2
  exit 1
else
  status=$?
fi
[[ "$status" -eq 2 ]]
rg -Fq 'HEPTA_UI_GATE_REQUIREMENT_CONTEXT is required' "$stderr_path"

consumer_specs=(
  'scripts/hepta-ui-backend-alignment-evidence-gate.sh|the Hepta UI backend alignment evidence gate|backend-alignment'
  'scripts/hepta-ui-backend-delivery-audit-gate.sh|the Hepta UI backend delivery audit gate|backend delivery audit'
  'scripts/hepta-ui-backend-delivery-receipt-roundtrip-gate.sh|the Hepta UI backend delivery receipt roundtrip gate|backend delivery receipt roundtrip'
  'scripts/hepta-ui-backend-handoff-export-gate.sh|the Hepta UI backend handoff export gate|backend handoff export'
  'scripts/hepta-ui-backend-promotion-packet-gate.sh|the Hepta UI backend promotion packet gate|backend-promotion'
  'scripts/hepta-ui-backend-receipt-intake-gate.sh|the Hepta UI backend receipt intake gate|backend receipt intake'
  'scripts/hepta-ui-blocker-closure-gate.sh|the Hepta UI blocker closure gate|blocker closure'
  'scripts/hepta-ui-cross-agent-visibility-gate.sh|the Hepta UI cross-agent visibility gate|cross-agent visibility'
  'scripts/hepta-ui-current-plan-refresh-gate.sh|the Hepta UI current-plan refresh gate|current-plan refresh'
  'scripts/hepta-ui-operator-briefing-gate.sh|the Hepta UI operator briefing gate|operator-briefing'
  'scripts/hepta-ui-release-approval-intake-gate.sh|the Hepta UI release approval intake gate|release approval intake'
  'scripts/hepta-ui-release-artifact-boundary-gate.sh|the Hepta UI release artifact boundary gate|release artifact boundary'
  'scripts/hepta-ui-release-artifact-intake-gate.sh|the Hepta UI release artifact intake gate|release artifact intake'
  'scripts/hepta-ui-release-artifact-roundtrip-gate.sh|the Hepta UI release artifact roundtrip gate|release artifact roundtrip'
  'scripts/hepta-ui-release-signing-capability-gate.sh|the Hepta UI release signing capability gate|release signing capability'
  'scripts/hepta-ui-risk-future-plan-gate.sh|the Hepta UI risk/future-plan gate|risk/future-plan'
  'scripts/hepta-ui-root-report-replay-gate.sh|the Hepta UI root-report replay gate|root-report replay'
)
for consumer_spec in "${consumer_specs[@]}"; do
  IFS='|' read -r consumer_script requirement_context report_label <<<"$consumer_spec"
  [[ -s "$consumer_script" ]]
  rg -Fxq "HEPTA_UI_GATE_REQUIREMENT_CONTEXT=\"$requirement_context\"" "$consumer_script"
  rg -Fxq "HEPTA_UI_REPORT_INPUT_LABEL=\"$report_label\"" "$consumer_script"
  [[ "$(rg -c -F 'source scripts/lib/hepta-ui-gate-common-v1.sh' "$consumer_script")" -eq 1 ]]
  if rg -n '^(require_command|require_report|file_sha256|file_bytes)\(\)' "$consumer_script" >/dev/null; then
    printf 'shared helper consumer restored a local helper definition: %s\n' "$consumer_script" >&2
    exit 1
  fi
done

printf '{"schema":"hepta_ui_gate_common_v1_self_test","status":"ready","negative_cases":4,"consumer_count":17}\n'
