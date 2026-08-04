#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

TEST_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-native-window-verifier-self-test.XXXXXX")"
trap 'rm -rf "$TEST_DIR"' EXIT

scripts/hepta-ui-native-window-verifier-v1 --help >/dev/null

peekaboo_success_filter='type == "object" and ((.success == true) or (.summary.status == "success"))'
jq -e "$peekaboo_success_filter" <<<'{"success":true,"data":{}}' >/dev/null
jq -e "$peekaboo_success_filter" <<<'{"summary":{"status":"success"},"data":{"windows":[]}}' >/dev/null
if jq -e "$peekaboo_success_filter" <<<'{"success":false,"summary":{"status":"error"}}' >/dev/null; then
  echo "native-window verifier accepted a failed Peekaboo response" >&2
  exit 1
fi

expect_path_rejection() {
  local label="$1"
  shift
  if scripts/hepta-ui-native-window-verifier-v1 "$@" >/dev/null 2>&1; then
    echo "native-window verifier accepted unsafe paths: $label" >&2
    exit 1
  fi
}

symlink_target="$TEST_DIR/symlink-target.json"
symlink_output="$TEST_DIR/symlink-output.json"
printf '%s\n' 'symlink-target-must-not-change' >"$symlink_target"
ln -s "$symlink_target" "$symlink_output"
expect_path_rejection output-symlink \
  --package-report "$TEST_DIR/missing-for-output-symlink.json" \
  --run-nonce 11111111-1111-1111-1111-111111111111 \
  --evidence-dir "$TEST_DIR/evidence-output-symlink" \
  --output "$symlink_output"
[[ "$(cat "$symlink_target")" == 'symlink-target-must-not-change' && -L "$symlink_output" ]] || {
  echo "unsafe output symlink validation changed its target" >&2
  exit 1
}

evidence_symlink_target="$TEST_DIR/evidence-symlink-target"
evidence_symlink="$TEST_DIR/evidence-symlink"
mkdir -p "$evidence_symlink_target"
printf '%s\n' 'evidence-target-must-not-change' >"$evidence_symlink_target/sentinel"
ln -s "$evidence_symlink_target" "$evidence_symlink"
expect_path_rejection evidence-symlink \
  --package-report "$TEST_DIR/missing-for-evidence-symlink.json" \
  --run-nonce 11111111-1111-1111-1111-111111111111 \
  --evidence-dir "$evidence_symlink" \
  --output "$TEST_DIR/evidence-symlink-receipt.json"
[[ "$(cat "$evidence_symlink_target/sentinel")" == 'evidence-target-must-not-change' ]] || {
  echo "unsafe evidence symlink validation changed its target" >&2
  exit 1
}

package_symlink_target="$TEST_DIR/package-symlink-target.json"
package_symlink="$TEST_DIR/package-symlink.json"
printf '%s\n' 'package-target-must-not-change' >"$package_symlink_target"
ln -s "$package_symlink_target" "$package_symlink"
expect_path_rejection package-symlink \
  --package-report "$package_symlink" \
  --run-nonce 11111111-1111-1111-1111-111111111111 \
  --evidence-dir "$TEST_DIR/evidence-package-symlink" \
  --output "$TEST_DIR/package-symlink-receipt.json"
[[ "$(cat "$package_symlink_target")" == 'package-target-must-not-change' ]] || {
  echo "unsafe package symlink validation changed its target" >&2
  exit 1
}

evidence_special="$TEST_DIR/evidence-special-file"
printf '%s\n' 'not-a-directory' >"$evidence_special"
expect_path_rejection evidence-special-file \
  --package-report "$TEST_DIR/missing-for-evidence-special.json" \
  --run-nonce 11111111-1111-1111-1111-111111111111 \
  --evidence-dir "$evidence_special" \
  --output "$TEST_DIR/evidence-special-receipt.json"

output_special="$TEST_DIR/output-special-directory"
mkdir -p "$output_special"
expect_path_rejection output-special-file \
  --package-report "$TEST_DIR/missing-for-output-special.json" \
  --run-nonce 11111111-1111-1111-1111-111111111111 \
  --evidence-dir "$TEST_DIR/evidence-output-special" \
  --output "$output_special"

package_special="$TEST_DIR/package-special-directory"
mkdir -p "$package_special"
expect_path_rejection package-special-file \
  --package-report "$package_special" \
  --run-nonce 11111111-1111-1111-1111-111111111111 \
  --evidence-dir "$TEST_DIR/evidence-package-special" \
  --output "$TEST_DIR/package-special-receipt.json"

shared_package_output="$TEST_DIR/package-is-output.json"
printf '%s\n' 'package-input-must-not-change' >"$shared_package_output"
expect_path_rejection output-equals-package \
  --package-report "$shared_package_output" \
  --run-nonce 11111111-1111-1111-1111-111111111111 \
  --evidence-dir "$TEST_DIR/evidence-output-equals-package" \
  --output "$shared_package_output"
[[ "$(cat "$shared_package_output")" == 'package-input-must-not-change' ]] || {
  echo "output/package collision deleted or changed the input receipt" >&2
  exit 1
}

alias_collision_real="$TEST_DIR/alias-collision-real"
alias_collision_link="$TEST_DIR/alias-collision-link"
mkdir -p "$alias_collision_real"
ln -s "$alias_collision_real" "$alias_collision_link"
printf '%s\n' 'canonical-package-input-must-not-change' >"$alias_collision_real/package.json"
expect_path_rejection canonical-output-equals-package \
  --package-report "$alias_collision_real/package.json" \
  --run-nonce 11111111-1111-1111-1111-111111111111 \
  --evidence-dir "$TEST_DIR/evidence-canonical-output-equals-package" \
  --output "$alias_collision_link/package.json"
[[ "$(cat "$alias_collision_real/package.json")" == 'canonical-package-input-must-not-change' ]] || {
  echo "canonical output/package collision changed the input receipt" >&2
  exit 1
}

dot_collision_parent="$TEST_DIR/dot-collision-parent"
mkdir -p "$dot_collision_parent"
printf '%s\n' 'dot-package-input-must-not-change' >"$dot_collision_parent/package.json"
expect_path_rejection lexical-output-equals-package \
  --package-report "$dot_collision_parent/package.json" \
  --run-nonce 11111111-1111-1111-1111-111111111111 \
  --evidence-dir "$TEST_DIR/evidence-lexical-output-equals-package" \
  --output "$dot_collision_parent/not-created/../package.json"
[[ "$(cat "$dot_collision_parent/package.json")" == 'dot-package-input-must-not-change' && ! -e "$dot_collision_parent/not-created" ]] || {
  echo "lexical output/package collision wrote before rejection" >&2
  exit 1
}

log_collision_evidence="$TEST_DIR/evidence-output-log-collision"
expect_path_rejection output-equals-log \
  --package-report "$TEST_DIR/missing-for-log-collision.json" \
  --run-nonce 11111111-1111-1111-1111-111111111111 \
  --evidence-dir "$log_collision_evidence" \
  --output "$log_collision_evidence/native-window-app.log"

evidence_output_collision="$TEST_DIR/evidence-is-output"
expect_path_rejection output-equals-evidence \
  --package-report "$TEST_DIR/missing-for-evidence-collision.json" \
  --run-nonce 11111111-1111-1111-1111-111111111111 \
  --evidence-dir "$evidence_output_collision" \
  --output "$evidence_output_collision"

package_inside_evidence="$TEST_DIR/evidence-containing-package"
expect_path_rejection package-inside-evidence \
  --package-report "$package_inside_evidence/package.json" \
  --run-nonce 11111111-1111-1111-1111-111111111111 \
  --evidence-dir "$package_inside_evidence" \
  --output "$TEST_DIR/package-inside-evidence-receipt.json"

package_evidence_alias="$TEST_DIR/evidence-containing-package-alias"
ln -s "$package_inside_evidence" "$package_evidence_alias"
expect_path_rejection canonical-package-inside-evidence \
  --package-report "$package_evidence_alias/package.json" \
  --run-nonce 11111111-1111-1111-1111-111111111111 \
  --evidence-dir "$package_inside_evidence" \
  --output "$TEST_DIR/canonical-package-inside-evidence-receipt.json"

if scripts/hepta-ui-native-window-verifier-v1 \
  --package-report "$TEST_DIR/missing-package.json" \
  --run-nonce 11111111-1111-1111-1111-111111111111 \
  --evidence-dir "$TEST_DIR/evidence" \
  --output "$TEST_DIR/receipt.json" >/dev/null 2>&1; then
  echo "native-window verifier accepted a missing package report" >&2
  exit 1
fi
[[ -s "$TEST_DIR/receipt.json" ]] || {
  echo "missing-package path did not produce a structured failure receipt" >&2
  exit 1
}
jq -e '
  .schema_version == 1
  and .kind == "hepta-ui-native-window-receipt-v1"
  and .producer == "scripts/hepta-ui-native-window-verifier-v1"
  and .status == "not_ready"
  and .native_window_ready == false
  and .independent_promotion_verifier_ready == false
  and .failure.stage == "argument_validation"
  and .failure.exit_code == 1
  and (.failure.detail | contains("package report is not a regular file"))
  and .display_wake.backend == "/usr/bin/caffeinate"
  and .display_wake.active == false
  and .display_wake.pid == null
  and .display_wake.process_start_confirmed == false
  and .display_wake.flags == ["-d", "-i", "-m", "-s", "-u"]
  and (.display_wake.wait_for_pid | type == "number" and . > 0)
  and .display_wake.parent_bound_fail_safe == true
  and .remote_side_effects_performed == false
' "$TEST_DIR/receipt.json" >/dev/null
[[ -f "$TEST_DIR/evidence/native-window-verifier.stdout.log" ]] || {
  echo "failure path did not persist verifier stdout" >&2
  exit 1
}
[[ -s "$TEST_DIR/evidence/native-window-verifier.stderr.log" ]] || {
  echo "failure path did not persist verifier stderr" >&2
  exit 1
}

grep -Fq -- 'producer:"scripts/hepta-ui-native-window-verifier-v1"' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'scope:"unauthenticated_local_macos_product_shell"' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'and .local_package_ready == true' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'and .package_source_readiness_requires_launch == false' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'and .independent_window_verifier_required == true' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'and .launch_probe_executed == false' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'and .staged_app_launch_verified == false' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- '"$PACKAGE_BINARY" --force-login' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- '/usr/bin/sandbox-exec -f "$SANDBOX_PROFILE"' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- '--mode window --window-id "$WINDOW_ID" --path "$HOST_SCREENSHOT"' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'peekaboo list windows --no-remote --app "PID:$APP_PID"' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'and ((.success == true) or (.summary.status == "success"))' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'WINDOW_BOUNDS_TOLERANCE=32' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'bounds_within_tolerance:$bounds_within_tolerance' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'minimum_capture_size_ready:true' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'for attempt in {1..4}; do' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'peekaboo-image-$attempt' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'capture_attempt:$window_capture_attempt' scripts/hepta-ui-native-window-verifier-v1
[[ "$(grep -Fc -- 'peekaboo_response_succeeded <<<"$WINDOW_LIST"' scripts/hepta-ui-native-window-verifier-v1)" == "2" ]] || {
  echo "native-window verifier does not validate both window-list response stages" >&2
  exit 1
}
grep -Fq -- 'peekaboo window focus \' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- '--no-remote --app "PID:$APP_PID" --window-id "$WINDOW_ID" --json' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'HOME="$ISOLATED_HOME"' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'CFFIXED_USER_HOME="$ISOLATED_HOME"' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'write_failure_receipt()' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'VERIFIER_STDERR_LOG="$EVIDENCE_DIR/native-window-verifier.stderr.log"' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'source scripts/lib/hepta-process-identity-v1.sh' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'hepta_process_terminate_identity_safe' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'hepta_process_terminate_start_safe' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'DISPLAY_WAKE_BACKEND="/usr/bin/caffeinate"' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- '"$DISPLAY_WAKE_BACKEND" -dimsu -w "$DISPLAY_WAKE_PARENT_PID" \' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'DISPLAY_WAKE_PID="$!"' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'if ! kill -0 "$DISPLAY_WAKE_PID" >/dev/null 2>&1; then' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'fail "display wake backend failed to remain active after startup (exit code $startup_rc)" 2' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'CURRENT_STAGE="peekaboo_permissions"' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'CURRENT_STAGE="window_capture"' scripts/hepta-ui-native-window-verifier-v1
[[ "$(grep -Fc -- 'display_wake:{backend:$display_wake_backend' scripts/hepta-ui-native-window-verifier-v1)" == "3" ]] || {
  echo "native-window verifier does not audit display wake in every receipt and manifest" >&2
  exit 1
}
[[ "$(grep -Fc -- 'require_display_wake_active' scripts/hepta-ui-native-window-verifier-v1)" -ge 7 ]] || {
  echo "native-window verifier does not fail closed around permission/window capture" >&2
  exit 1
}
grep -Fq -- 'source scripts/lib/hepta-process-identity-v1.sh' scripts/hepta-control-ui-browser-smoke.sh
grep -Fq -- 'hepta_process_terminate_identity_safe' scripts/hepta-control-ui-browser-smoke.sh
grep -Fq -- 'hepta_process_terminate_start_safe' scripts/hepta-control-ui-browser-smoke.sh
grep -Fq -- 'scripts/hepta-ui-bundle-fingerprint --root "$PACKAGE_APP"' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'scripts/hepta-ui-native-window-sandbox-profile' scripts/hepta-ui-native-window-verifier-v1
grep -Fq -- 'select(.title == "Hepta")' scripts/hepta-ui-native-window-verifier-v1
if rg -n 'ALLOW_BLOCKED|screen_crop|--mode screen|developer-diagnostics|hepta_ui_cargo build|staged_app_launch_verified == true' scripts/hepta-ui-native-window-verifier-v1 >/dev/null; then
  echo "native-window promotion verifier contains a permissive capture fallback" >&2
  exit 1
fi

# A recycled PID is a successful confirmation that the original child stopped,
# but it must never receive TERM or KILL. This exercises the shared cleanup used
# by both the native-window verifier and the browser smoke server.
fake_ps="$TEST_DIR/fake-ps"
fake_kill="$TEST_DIR/fake-kill"
fake_signal_log="$TEST_DIR/fake-signals.log"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'case "$*" in' \
  '  *"lstart="*) printf "%s\\n" "Tue Jan  2 03:04:05 2024" ;;' \
  '  *"state="*) printf "%s\\n" "S" ;;' \
  '  *"command="*) printf "%s\\n" "/usr/bin/unrelated --serve" ;;' \
  '  *) exit 1 ;;' \
  'esac' >"$fake_ps"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'if [[ "${1:-}" == "-0" ]]; then exit 0; fi' \
  'printf "%s\\n" "$*" >>"$FAKE_SIGNAL_LOG"' \
  'exit 0' >"$fake_kill"
chmod 700 "$fake_ps" "$fake_kill"
export FAKE_SIGNAL_LOG="$fake_signal_log"
export HEPTA_PROCESS_PS_BIN="$fake_ps"
export HEPTA_PROCESS_KILL_BIN="$fake_kill"
# shellcheck source=scripts/lib/hepta-process-identity-v1.sh
source scripts/lib/hepta-process-identity-v1.sh
reuse_rc=0
hepta_process_terminate_identity_safe \
  4242 "Mon Jan  1 00:00:00 2024" "/expected/product --serve" 1 0 1 || reuse_rc=$?
[[ "$reuse_rc" == "0" \
  && "$HEPTA_PROCESS_PID_REUSED" == true \
  && "$HEPTA_PROCESS_STOP_CONFIRMED" == true \
  && "$HEPTA_PROCESS_TERM_SENT" == false \
  && "$HEPTA_PROCESS_KILL_SENT" == false \
  && ! -s "$fake_signal_log" ]] || {
  echo "identity-safe cleanup signalled a recycled PID" >&2
  exit 1
}
command_mismatch_rc=0
hepta_process_terminate_identity_safe \
  4242 "Tue Jan  2 03:04:05 2024" "/expected/product --serve" 1 0 1 || command_mismatch_rc=$?
[[ "$command_mismatch_rc" == "75" \
  && "$HEPTA_PROCESS_TERM_SENT" == false \
  && "$HEPTA_PROCESS_KILL_SENT" == false \
  && ! -s "$fake_signal_log" ]] || {
  echo "identity-safe cleanup signalled a command-mismatched PID" >&2
  exit 1
}

# A same-start child that has already exited can remain observable as a zombie
# until this parent calls wait. Its command may have changed to <defunct>, but
# it cannot execute and must not receive TERM/KILL or be mistaken for a live
# command-mismatched/recycled process.
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'case "$*" in' \
  '  *"lstart="*) printf "%s\\n" "Tue Jan  2 03:04:05 2024" ;;' \
  '  *"state="*) printf "%s\\n" "ZN" ;;' \
  '  *"command="*) printf "%s\\n" "<defunct>" ;;' \
  '  *) exit 1 ;;' \
  'esac' >"$fake_ps"
chmod 700 "$fake_ps"
: >"$fake_signal_log"
zombie_exit_race_rc=0
hepta_process_terminate_identity_safe \
  4242 "Tue Jan  2 03:04:05 2024" "/expected/product --serve" 1 0 1 \
  || zombie_exit_race_rc=$?
[[ "$zombie_exit_race_rc" == "0" \
  && "$HEPTA_PROCESS_STOP_CONFIRMED" == true \
  && "$HEPTA_PROCESS_PID_REUSED" == false \
  && "$HEPTA_PROCESS_TERM_SENT" == false \
  && "$HEPTA_PROCESS_KILL_SENT" == false \
  && ! -s "$fake_signal_log" ]] || {
  echo "identity-safe cleanup mishandled the same-start zombie exit race" >&2
  exit 1
}

# The child may exit between kill -0 and the identity read. The helper must
# confirm that it is now gone without signalling the PID, while continuing to
# reject an identity read failure for a still-live process.
fake_alive="$TEST_DIR/fake-race-alive"
: >"$fake_alive"
: >"$fake_signal_log"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'rm -f "$FAKE_ALIVE_FILE"' \
  'exit 1' >"$fake_ps"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'if [[ "${1:-}" == "-0" ]]; then [[ -e "$FAKE_ALIVE_FILE" ]]; exit; fi' \
  'printf "%s\\n" "$*" >>"$FAKE_SIGNAL_LOG"' \
  'exit 0' >"$fake_kill"
chmod 700 "$fake_ps" "$fake_kill"
export FAKE_ALIVE_FILE="$fake_alive"
identity_exit_race_rc=0
hepta_process_terminate_identity_safe \
  4242 "Tue Jan  2 03:04:05 2024" "/expected/product --serve" 1 0 1 \
  || identity_exit_race_rc=$?
[[ "$identity_exit_race_rc" == "0" \
  && "$HEPTA_PROCESS_STOP_CONFIRMED" == true \
  && "$HEPTA_PROCESS_TERM_SENT" == false \
  && "$HEPTA_PROCESS_KILL_SENT" == false \
  && ! -s "$fake_signal_log" ]] || {
  echo "identity-safe cleanup mishandled the exit-before-identity race" >&2
  exit 1
}
unset HEPTA_PROCESS_PS_BIN HEPTA_PROCESS_KILL_BIN FAKE_SIGNAL_LOG FAKE_ALIVE_FILE

# Browser log and receipt paths are validated before compilation/capture. A
# symlink target or producer screenshot must remain untouched on rejection.
browser_log_target="$TEST_DIR/browser-log-target.log"
browser_log_symlink="$TEST_DIR/browser-log-symlink.log"
printf '%s\n' 'browser-log-target-must-not-change' >"$browser_log_target"
ln -s "$browser_log_target" "$browser_log_symlink"
if HEPTA_BROWSER_SMOKE_DIR= \
  HEPTA_BROWSER_SMOKE_REPORT_PATH= \
  HEPTA_CONTROL_UI_BROWSER_SMOKE_REPORT_PATH= \
  HEPTA_CONTROL_UI_SERVER_LOG="$browser_log_symlink" \
  scripts/hepta-control-ui-browser-smoke.sh >/dev/null 2>&1; then
  echo "Control UI browser smoke accepted a symlinked server log" >&2
  exit 1
fi
[[ "$(cat "$browser_log_target")" == 'browser-log-target-must-not-change' && -L "$browser_log_symlink" ]] || {
  echo "Control UI browser smoke changed a symlinked server-log target" >&2
  exit 1
}

browser_log_special="$TEST_DIR/browser-log-special"
mkdir -p "$browser_log_special"
if HEPTA_BROWSER_SMOKE_DIR= \
  HEPTA_BROWSER_SMOKE_REPORT_PATH= \
  HEPTA_CONTROL_UI_BROWSER_SMOKE_REPORT_PATH= \
  HEPTA_CONTROL_UI_SERVER_LOG="$browser_log_special" \
  scripts/hepta-control-ui-browser-smoke.sh >/dev/null 2>&1; then
  echo "Control UI browser smoke accepted a special-file server log" >&2
  exit 1
fi

shared_browser_output="$TEST_DIR/shared-browser-output.json"
printf '%s\n' preserve-shared-browser-output >"$shared_browser_output"
if HEPTA_BROWSER_SMOKE_DIR= \
  HEPTA_BROWSER_SMOKE_REPORT_PATH= \
  HEPTA_CONTROL_UI_BROWSER_SMOKE_REPORT_PATH="$shared_browser_output" \
  HEPTA_CONTROL_UI_SERVER_LOG="$shared_browser_output" \
  scripts/hepta-control-ui-browser-smoke.sh >/dev/null 2>&1; then
  echo "Control UI browser smoke accepted a report/server-log collision" >&2
  exit 1
fi
[[ "$(cat "$shared_browser_output")" == preserve-shared-browser-output ]] || {
  echo "Control UI browser report/server-log collision changed the shared file" >&2
  exit 1
}

browser_output_dir="$TEST_DIR/browser-producer-output"
mkdir -p "$browser_output_dir"
if HEPTA_BROWSER_SMOKE_DIR="$browser_output_dir" \
  HEPTA_BROWSER_SMOKE_REPORT_PATH= \
  HEPTA_CONTROL_UI_BROWSER_SMOKE_REPORT_PATH= \
  HEPTA_CONTROL_UI_SERVER_LOG="$browser_output_dir/phone320.png" \
  scripts/hepta-control-ui-browser-smoke.sh >/dev/null 2>&1; then
  echo "Control UI browser smoke accepted a server-log/screenshot collision" >&2
  exit 1
fi
[[ ! -e "$browser_output_dir/phone320.png" ]] || {
  echo "Control UI browser smoke wrote a reserved screenshot during rejection" >&2
  exit 1
}

browser_receipt_target="$TEST_DIR/browser-receipt-target.json"
browser_receipt_symlink="$TEST_DIR/browser-receipt-symlink.json"
printf '%s\n' 'browser-receipt-target-must-not-change' >"$browser_receipt_target"
ln -s "$browser_receipt_target" "$browser_receipt_symlink"
if (
  # shellcheck source=scripts/lib/hepta-safe-output-v1.sh
  source scripts/lib/hepta-safe-output-v1.sh
  # shellcheck source=scripts/lib/hepta-browser-visual-smoke-v1/config.sh
  source scripts/lib/hepta-browser-visual-smoke-v1/config.sh
  export HEPTA_BROWSER_SMOKE_DIR="$TEST_DIR/browser-config-output"
  export HEPTA_BROWSER_SMOKE_REPORT_PATH="$browser_receipt_symlink"
  hepta_browser_configure
) >/dev/null 2>&1; then
  echo "browser visual smoke accepted a symlinked receipt" >&2
  exit 1
fi
[[ "$(cat "$browser_receipt_target")" == 'browser-receipt-target-must-not-change' && -L "$browser_receipt_symlink" ]] || {
  echo "browser visual smoke changed a symlinked receipt target" >&2
  exit 1
}

browser_receipt_special="$TEST_DIR/browser-receipt-special"
mkdir -p "$browser_receipt_special"
if (
  # shellcheck source=scripts/lib/hepta-safe-output-v1.sh
  source scripts/lib/hepta-safe-output-v1.sh
  # shellcheck source=scripts/lib/hepta-browser-visual-smoke-v1/config.sh
  source scripts/lib/hepta-browser-visual-smoke-v1/config.sh
  export HEPTA_BROWSER_SMOKE_DIR="$TEST_DIR/browser-config-special-output"
  export HEPTA_BROWSER_SMOKE_REPORT_PATH="$browser_receipt_special"
  hepta_browser_configure
) >/dev/null 2>&1; then
  echo "browser visual smoke accepted a special-file receipt" >&2
  exit 1
fi

if (
  # shellcheck source=scripts/lib/hepta-safe-output-v1.sh
  source scripts/lib/hepta-safe-output-v1.sh
  # shellcheck source=scripts/lib/hepta-browser-visual-smoke-v1/config.sh
  source scripts/lib/hepta-browser-visual-smoke-v1/config.sh
  export HEPTA_BROWSER_SMOKE_DIR="$browser_output_dir"
  export HEPTA_BROWSER_SMOKE_REPORT_PATH="$browser_output_dir/phone320.png"
  hepta_browser_configure
) >/dev/null 2>&1; then
  echo "browser visual smoke accepted a receipt/screenshot collision" >&2
  exit 1
fi
[[ ! -e "$browser_output_dir/phone320.png" ]] || {
  echo "browser visual smoke changed a reserved screenshot during rejection" >&2
  exit 1
}

grep -Fq -- 'source "$script_dir/lib/hepta-safe-output-v1.sh"' scripts/hepta-browser-visual-smoke.sh
grep -Fq -- 'hepta_safe_output_atomic_write_text "$REPORT_PATH" "$report"' scripts/lib/hepta-browser-visual-smoke-v1/receipt.sh
grep -Fq -- 'source scripts/lib/hepta-safe-output-v1.sh' scripts/hepta-control-ui-browser-smoke.sh
grep -Fq -- 'hepta_safe_output_install_temp "$server_log_temp" "$SERVER_LOG"' scripts/hepta-control-ui-browser-smoke.sh

bundle="$TEST_DIR/Hepta.app"
mkdir -p "$bundle/Contents/MacOS"
printf '%s\n' one >"$bundle/Contents/MacOS/hepta-native"
first_fingerprint="$(scripts/hepta-ui-bundle-fingerprint --root "$bundle")"
printf '%s\n' two >"$bundle/Contents/MacOS/hepta-native"
second_fingerprint="$(scripts/hepta-ui-bundle-fingerprint --root "$bundle")"
[[ "$first_fingerprint" =~ ^[0-9a-f]{64}$ && "$second_fingerprint" =~ ^[0-9a-f]{64}$ && "$first_fingerprint" != "$second_fingerprint" ]] || {
  echo "bundle fingerprint did not detect artifact drift" >&2
  exit 1
}

scratch="$TEST_DIR/scratch"
mkdir -p "$scratch"
profile="$TEST_DIR/native-window.sb"
scripts/hepta-ui-native-window-sandbox-profile \
  --data-dir "$TEST_DIR/product-data" \
  --cache-dir "$TEST_DIR/product-cache" \
  --scratch-dir "$scratch" \
  --output "$profile"
[[ "$(stat -f %Lp "$profile")" == "600" ]] || { echo "sandbox profile mode is not 600" >&2; exit 1; }
grep -Fq -- '(deny network*)' "$profile"
grep -Fq -- '(global-name "com.apple.securityd")' "$profile"
/usr/bin/sandbox-exec -f "$profile" /usr/bin/true

echo "hepta-ui native-window verifier fail-closed self-test: PASS"
