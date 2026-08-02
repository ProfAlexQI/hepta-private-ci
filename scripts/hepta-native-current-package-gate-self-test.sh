#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

GATE="scripts/hepta-native-current-package-gate.sh"
TEST_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-native-current-package-gate-self-test.XXXXXX")"
trap 'rm -rf "$TEST_DIR"' EXIT

fail() {
  echo "hepta-native-current-package-gate self-test failed: $1" >&2
  exit 1
}

run_not_ready_report() {
  local report="$1"
  shift
  local rc=0
  set +e
  /bin/bash "$GATE" "$@" --output "$report" >/dev/null
  rc=$?
  set -e
  [[ "$rc" == "1" ]] || fail "expected non-build gate exit 1, got $rc"
  jq -e '.schema_version == 1 and .kind == "hepta-native-current-package-gate"' \
    "$report" >/dev/null || fail "gate did not write a valid report"
}

/bin/bash -n "$GATE"

help_output="$(/bin/bash "$GATE" --help)"
grep -Fq -- '[--launch-probe | --no-launch]' <<<"$help_output" \
  || fail "help does not expose the explicit launch choice"
grep -Fq -- 'It does not' <<<"$help_output" \
  || fail "help does not state the default non-launch boundary"

set +e
/bin/bash "$GATE" --launch-probe --no-launch \
  >"$TEST_DIR/conflict.stdout" 2>"$TEST_DIR/conflict.stderr"
conflict_rc=$?
set -e
[[ "$conflict_rc" == "64" ]] || fail "conflicting launch switches did not fail with 64"
grep -Fq 'mutually exclusive' "$TEST_DIR/conflict.stderr" \
  || fail "conflicting launch switches did not explain the failure"

symlink_target="$TEST_DIR/output-symlink-target.json"
symlink_output="$TEST_DIR/output-symlink.json"
printf '%s\n' 'package-output-target-must-not-change' >"$symlink_target"
ln -s "$symlink_target" "$symlink_output"
set +e
/bin/bash "$GATE" --output "$symlink_output" \
  >"$TEST_DIR/symlink.stdout" 2>"$TEST_DIR/symlink.stderr"
symlink_rc=$?
set -e
[[ "$symlink_rc" == "64" ]] || fail "symlinked package output did not fail with 64"
[[ "$(cat "$symlink_target")" == 'package-output-target-must-not-change' && -L "$symlink_output" ]] \
  || fail "symlinked package output changed its target"

special_output="$TEST_DIR/output-special-directory"
mkdir -p "$special_output"
set +e
/bin/bash "$GATE" --output "$special_output" \
  >"$TEST_DIR/special.stdout" 2>"$TEST_DIR/special.stderr"
special_rc=$?
set -e
[[ "$special_rc" == "64" ]] || fail "special-file package output did not fail with 64"

overlap_stage="$TEST_DIR/producer-stage"
overlap_output="$overlap_stage/package-receipt.json"
mkdir -p "$overlap_stage"
set +e
/bin/bash "$GATE" --stage-dir "$overlap_stage" --output "$overlap_output" \
  >"$TEST_DIR/overlap.stdout" 2>"$TEST_DIR/overlap.stderr"
overlap_rc=$?
set -e
[[ "$overlap_rc" == "64" && ! -e "$overlap_output" ]] \
  || fail "package output overlapping the producer stage was not rejected"

default_report="$TEST_DIR/default.json"
run_not_ready_report "$default_report"
jq -e '
  .build_requested == false
  and .launch_probe_required == false
  and .package_source_readiness_requires_launch == false
  and .independent_window_verifier_required == true
  and .local_window_promotion_ready == false
  and .launch_probe_mode == "default_disabled"
  and .launch_probe_requested == false
  and .launch_probe_executed == false
  and .process_started == false
  and .staged_app_launch_verified == false
  and .sandbox_profile_applied == false
  and .home_isolated == false
  and .network_denied_by_sandbox == false
  and .keychain_services_denied == false
  and .force_login_argument == false
  and .external_side_effects_performed == false
  and .remote_side_effects_performed == false
  and .side_effect_boundary.mode == "static_source_and_metadata_verification_only"
  and .side_effect_boundary.external_side_effects_performed == false
  and .side_effect_boundary.independent_window_promotion_required == true
  and .launch_probe.failures == []
  and ([.blockers[] | select(startswith("launch_") or startswith("staged_app_launch"))] | length) == 0
' "$default_report" >/dev/null || fail "default report launch truth is invalid"

explicit_disabled_report="$TEST_DIR/explicit-disabled.json"
run_not_ready_report "$explicit_disabled_report" --no-launch
jq -e '
  .launch_probe_mode == "explicit_disabled"
  and .launch_probe_requested == false
  and .launch_probe_executed == false
  and .process_started == false
  and .external_side_effects_performed == false
' "$explicit_disabled_report" >/dev/null || fail "--no-launch report is invalid"

explicit_enabled_report="$TEST_DIR/explicit-enabled-no-build.json"
run_not_ready_report "$explicit_enabled_report" --launch-probe
jq -e '
  .launch_probe_mode == "explicit_enabled"
  and .launch_probe_requested == true
  and .launch_probe_executed == false
  and .process_started == false
  and .external_side_effects_performed == false
  and .launch_probe.failures == ["launch_probe_requires_explicit_build"]
  and ([.blockers[] | select(startswith("launch_") or startswith("staged_app_launch"))] | length) == 0
' "$explicit_enabled_report" >/dev/null \
  || fail "explicit launch request without a built artifact is not fail-closed"

grep -Fq 'LAUNCH_PROBE=0' "$GATE" || fail "launch probe is not disabled by default"
grep -Fq 'exec /usr/bin/sandbox-exec -f "$launch_sandbox_profile" /usr/bin/env' "$GATE" \
  || fail "launch probe does not reuse the macOS sandbox profile"
grep -Fq '"$binary" --force-login' "$GATE" \
  || fail "launch probe does not force the login surface"
grep -Fq 'external_side_effects_performed:$launch_probe_executed' "$GATE" \
  || fail "external side-effect truth is not conditional on probe execution"
if grep -Eq 'external_side_effects_performed:[[:space:]]*false' "$GATE"; then
  fail "external side effects are still reported as an unconditional false"
fi
if grep -Fq 'launch_requirement_ready' "$GATE"; then
  fail "package/source readiness still depends on the optional launch probe"
fi
grep -Fq 'source scripts/lib/hepta-process-identity-v1.sh' "$GATE" \
  || fail "package launch cleanup does not use the shared identity helper"
grep -Fq 'source scripts/lib/hepta-safe-output-v1.sh' "$GATE" \
  || fail "package report does not use the shared safe-output helper"
grep -Fq 'hepta_safe_output_atomic_write_text "$REPORT_PATH" "$report"' "$GATE" \
  || fail "package report is not installed atomically in its destination directory"
grep -Fq 'hepta_process_terminate_identity_safe' "$GATE" \
  || fail "package launch cleanup does not revalidate exact process identity"
grep -Fq 'hepta_process_terminate_start_safe' "$GATE" \
  || fail "package launch cleanup cannot stop an unmatched sandbox wrapper safely"
grep -Fq '"$candidate_start_token" == "$launch_spawn_start_token"' "$GATE" \
  || fail "package launch identity can cross a recycled PID start token"
if rg -n 'kill[[:space:]]+-(TERM|KILL)[[:space:]]+"?\$launch_pid' "$GATE" >/dev/null; then
  fail "package launch cleanup contains a naked PID signal"
fi

# The unmatched-wrapper path must stop the exact same-start child, while a
# recycled PID must be treated as already stopped and receive no signal.
fake_ps="$TEST_DIR/fake-ps"
fake_kill="$TEST_DIR/fake-kill"
fake_alive="$TEST_DIR/fake-alive"
fake_signal_log="$TEST_DIR/fake-signals.log"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'case "$*" in' \
  '  *"lstart="*) printf "%s\\n" "Wed Jan  3 04:05:06 2024" ;;' \
  '  *"command="*) printf "%s\\n" "/usr/bin/sandbox-exec -f probe.sb /product/hepta-native --force-login" ;;' \
  '  *) exit 1 ;;' \
  'esac' >"$fake_ps"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'if [[ "${1:-}" == "-0" ]]; then [[ -e "$FAKE_ALIVE_FILE" ]]; exit; fi' \
  'printf "%s\\n" "$*" >>"$FAKE_SIGNAL_LOG"' \
  'rm -f "$FAKE_ALIVE_FILE"' \
  'exit 0' >"$fake_kill"
chmod 700 "$fake_ps" "$fake_kill"
export FAKE_ALIVE_FILE="$fake_alive"
export FAKE_SIGNAL_LOG="$fake_signal_log"
export HEPTA_PROCESS_PS_BIN="$fake_ps"
export HEPTA_PROCESS_KILL_BIN="$fake_kill"
# shellcheck source=scripts/lib/hepta-process-identity-v1.sh
source scripts/lib/hepta-process-identity-v1.sh
: >"$fake_alive"
wrapper_cleanup_rc=0
hepta_process_terminate_start_safe 5252 "Wed Jan  3 04:05:06 2024" 1 0 1 \
  || wrapper_cleanup_rc=$?
[[ "$wrapper_cleanup_rc" == "0" \
  && "$HEPTA_PROCESS_STOP_CONFIRMED" == true \
  && "$HEPTA_PROCESS_TERM_IDENTITY_VERIFIED" == true \
  && "$HEPTA_PROCESS_TERM_SENT" == true \
  && "$HEPTA_PROCESS_KILL_SENT" == false \
  && "$(cat "$fake_signal_log")" == "-TERM 5252" ]] \
  || fail "same-start unmatched wrapper was not terminated safely"

# Swap the fake identity to a different start token without changing the PID.
sed -i '' 's/Wed Jan  3 04:05:06 2024/Thu Jan  4 05:06:07 2024/' "$fake_ps"
: >"$fake_alive"
signals_before="$(wc -l <"$fake_signal_log" | tr -d ' ')"
reuse_cleanup_rc=0
hepta_process_terminate_start_safe 5252 "Wed Jan  3 04:05:06 2024" 1 0 1 \
  || reuse_cleanup_rc=$?
signals_after="$(wc -l <"$fake_signal_log" | tr -d ' ')"
[[ "$reuse_cleanup_rc" == "0" \
  && "$HEPTA_PROCESS_PID_REUSED" == true \
  && "$HEPTA_PROCESS_STOP_CONFIRMED" == true \
  && "$signals_after" == "$signals_before" ]] \
  || fail "start-safe cleanup signalled a recycled PID"
unset HEPTA_PROCESS_PS_BIN HEPTA_PROCESS_KILL_BIN FAKE_ALIVE_FILE FAKE_SIGNAL_LOG

echo "hepta-native-current-package-gate self-test passed"
