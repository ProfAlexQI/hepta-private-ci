#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
RESUME="$ROOT/scripts/hepta-preflight-resume"
tmp="$(mktemp -d /tmp/hepta-preflight-resume-self-test.XXXXXX)"
trap '[[ "${HEPTA_RESUME_SELFTEST_KEEP_TMP:-0}" == "1" ]] || rm -rf "$tmp"' EXIT
fixture="$tmp/repo"
mkdir -p "$fixture/scripts" "$fixture/codex-rs" "$fixture/apps/hepta-native"
cp "$RESUME" "$fixture/scripts/hepta-preflight-resume"
chmod +x "$fixture/scripts/hepta-preflight-resume"
cat >"$fixture/scripts/hepta-preflight.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
# hepta-preflight-resume: prelude-start
FIXTURE_REPO_ROOT="$PWD"
RUN_NATIVE="${HEPTA_PREFLIGHT_NATIVE:-0}"
RUN_RELEASE="${HEPTA_PREFLIGHT_RELEASE:-0}"
PREFLIGHT_RELEASE_TARGET_DIR="${CARGO_TARGET_DIR:-target}"
run_preflight_gate() {
  printf '[hepta-preflight] %s\n' "$1"
  "${@:2}"
}
[[ "$FIXTURE_REPO_ROOT" == "$PWD" ]] || exit 48
printf '[fixture] release_target=%s\n' "$PREFLIGHT_RELEASE_TARGET_DIR"
# hepta-preflight-resume: prelude-end
if [[ "${HEPTA_RESUME_TEST_BREAK_STATE:-0}" == "1" ]]; then
  rm -rf "${HEPTA_RESUME_TEST_STATE_DIR:?}"
fi
run_preflight_gate "fixture deterministic gate" true
[[ "$PREFLIGHT_RELEASE_TARGET_DIR" == "${HEPTA_RESUME_TEST_EXPECTED_TARGET:?}" ]] || exit 44
[[ -z "${CODEX_THREAD_ID+x}" ]] || exit 45
if [[ -n "${HEPTA_RESUME_TEST_EXPECTED_PATH+x}" \
  && "${PATH:-}" != "$HEPTA_RESUME_TEST_EXPECTED_PATH" ]]; then
  exit 47
fi
if [[ -n "${RESUME_TEST_FORBIDDEN_ARG0_ROOT:-}" ]]; then
  case ":${PATH:-}:" in
    *":${RESUME_TEST_FORBIDDEN_ARG0_ROOT%/}/codex-arg0"*) exit 46 ;;
  esac
fi
if [[ "${HEPTA_RESUME_TEST_SLEEP_SECONDS:-0}" != "0" ]]; then
  sleep "$HEPTA_RESUME_TEST_SLEEP_SECONDS"
fi
if [[ "${HEPTA_RESUME_TEST_BREAK_LOG:-0}" == "1" ]]; then
  rm -f "${HEPTA_PREFLIGHT_RESUME_LOG:?}"
  mkdir "${HEPTA_PREFLIGHT_RESUME_LOG:?}"
fi
if [[ "${HEPTA_RESUME_TEST_FAIL:-0}" == "1" \
  || ( -n "${HEPTA_RESUME_TEST_FAIL_FILE:-}" && -f "$HEPTA_RESUME_TEST_FAIL_FILE" ) ]]; then
  exit 42
fi
echo "[hepta-preflight] source-bound release gate receipt replay boundary"
echo "[hepta-preflight] fixture dependency security receipt"
true
echo "[hepta-preflight] fixture compatibility receipt"
true
echo "[hepta-preflight] fixture trailing gate"
true
if [[ "$RUN_NATIVE" == "1" ]]; then
  echo "[hepta-preflight] fixture native gate"
  true
else
  echo "[hepta-preflight] native app gates skipped (HEPTA_PREFLIGHT_NATIVE=$RUN_NATIVE)"
fi
if [[ -n "${HEPTA_RESUME_TEST_TERMINAL_REPLAY_SIGNAL:-}" ]]; then
  touch "$HEPTA_RESUME_TEST_TERMINAL_REPLAY_SIGNAL"
  while [[ ! -f "${HEPTA_RESUME_TEST_TERMINAL_REPLAY_CONTINUE:?}" ]]; do
    sleep 0.05
  done
fi
if [[ "$RUN_RELEASE" == "1" ]]; then
  echo "[hepta-preflight] fixture release gate"
  if [[ -n "${HEPTA_RESUME_TEST_TERMINAL_FAIL_FILE:-}" \
    && -f "$HEPTA_RESUME_TEST_TERMINAL_FAIL_FILE" ]]; then
    exit 42
  fi
  true
else
  echo "[hepta-preflight] release build skipped (set HEPTA_PREFLIGHT_RELEASE=1)"
fi
echo "[hepta-preflight] fixture whitespace/status"
true
if [[ "$RUN_RELEASE" == "1" ]]; then
  fixture_provenance='{"fixture":true}'
  fixture_provenance_sha="$(
    printf '%s' "$fixture_provenance" | shasum -a 256 | awk '{print $1}'
  )"
  printf '[hepta-preflight-provenance] %s\n' "$fixture_provenance"
  printf '[hepta-preflight-final] {"artifact_sha256":"%s","build_provenance_sha256":"%s","schema":"hepta_preflight_final_receipt_v1","source_commit":"%s","status":"passed"}\n' \
    "$(printf '1%.0s' {1..64})" \
    "$fixture_provenance_sha" \
    "$(git rev-parse HEAD)"
  if [[ -n "${HEPTA_RESUME_TEST_FAIL_AFTER_PROVENANCE_FILE:-}" \
    && -f "$HEPTA_RESUME_TEST_FAIL_AFTER_PROVENANCE_FILE" ]]; then
    rm -f "$HEPTA_RESUME_TEST_FAIL_AFTER_PROVENANCE_FILE"
    exit 42
  fi
fi
echo "Hepta preflight passed"
EOF
chmod +x "$fixture/scripts/hepta-preflight.sh"
touch \
  "$fixture/codex-rs/Cargo.lock" \
  "$fixture/apps/hepta-native/Cargo.lock" \
  "$fixture/MODULE.bazel.lock"
git -C "$fixture" init -q
git -C "$fixture" add .
git -C "$fixture" \
  -c user.name='Hepta Resume Self-Test' \
  -c user.email='hepta-resume-self-test@invalid' \
  commit -qm 'fixture'
state="$tmp/resume.state"
log="$tmp/resume.log"
target="$tmp/target"
common_env=(
  HEPTA_PREFLIGHT_RESUME_STATE="$state"
  HEPTA_PREFLIGHT_RESUME_LOG="$log"
  HEPTA_PREFLIGHT_RESUME_MAX_SAME_FAILURES=3
  HEPTA_PREFLIGHT_NATIVE=0
  HEPTA_PREFLIGHT_RELEASE=1
  HEPTA_RESUME_TEST_FAIL=1
  CARGO_TARGET_DIR="$target"
  HEPTA_RESUME_TEST_EXPECTED_TARGET="$target"
)
assert_complete_state() {
  jq -e '
    .schema_version == "hepta_preflight_resume_state_v2"
    and .status == "complete"
    and .attempt >= 1
    and .failure_streak == 0
    and .fuse_armed == false
    and .evidence.worktree_clean == true
    and (.evidence.preflight_sha256 | test("^[0-9a-f]{64}$"))
    and (.evidence.environment_sha256 | test("^[0-9a-f]{64}$"))
    and (.evidence.cargo_identity_sha256 | test("^[0-9a-f]{64}$"))
    and (.evidence.rustc_identity_sha256 | test("^[0-9a-f]{64}$"))
    and (.evidence.toolchain_identity_sha256 | test("^[0-9a-f]{64}$"))
    and (.evidence.relevant_environment_sha256 | test("^[0-9a-f]{64}$"))
    and (.evidence.worktree_state_sha256 | test("^[0-9a-f]{64}$"))
    and (.evidence.worktree_id | length) > 0
    and (.evidence.log_sha256 | test("^[0-9a-f]{64}$"))
    and .evidence.log_prefix_sha256 == .evidence.log_sha256
    and .evidence.log_prefix_bytes > 0
    and (.evidence.attempt_log_sha256 | test("^[0-9a-f]{64}$"))
    and .evidence.failure_fingerprint_sha256 == null
    and .last_exit == {preflight:0, tee:0, checkpoint:0, combined:0}
  ' "$1" >/dev/null
}
fixture_head="$(git -C "$fixture" rev-parse HEAD)"
printf '%s\t%s\n' "$fixture_head" "fixture whitespace/status" >"$state"
legacy_show_json="$(env "${common_env[@]}" "$fixture/scripts/hepta-preflight-resume" --show)"
jq -e '
  .status == "legacy_untrusted"
  and .resumable == false
  and .start_line == 1
' >/dev/null <<<"$legacy_show_json"
legacy_rc=0
env "${common_env[@]}" "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 || legacy_rc=$?
[[ "$legacy_rc" == "1" ]] || {
  echo "legacy resume fixture returned $legacy_rc; expected fail-closed exit 1" >&2
  exit 1
}
env "${common_env[@]}" HEPTA_RESUME_TEST_FAIL=0 \
  "$fixture/scripts/hepta-preflight-resume" --reset >/dev/null
assert_complete_state "$state"
grep -q '^\[fixture\] release_target=' "$log"
grep -q '^\[hepta-preflight\] fixture deterministic gate$' "$log"
complete_show_json="$(
  env "${common_env[@]}" HEPTA_RESUME_TEST_FAIL=0 \
    "$fixture/scripts/hepta-preflight-resume" --show
)"
jq -e '
  .status == "complete"
  and .resumable == false
  and .fuse_armed == false
  and .lock_state == "none"
  and .resume_marker == "fixture whitespace/status"
  and .environment_matches == true
  and .preflight_matches == true
  and .toolchain_matches == true
  and .relevant_environment_matches == true
  and .worktree_state_matches == true
  and .worktree_identity_matches == true
  and .worktree_clean == true
  and .log_matches == true
  and .completion_evidence_valid == true
  and .last_exit == {preflight:0, tee:0, checkpoint:0, combined:0}
' >/dev/null <<<"$complete_show_json"
complete_gate_count="$(grep -c '^\[hepta-preflight\] fixture deterministic gate$' "$log")"
env "${common_env[@]}" HEPTA_RESUME_TEST_FAIL=0 \
  "$fixture/scripts/hepta-preflight-resume" >/dev/null
[[ "$(grep -c '^\[hepta-preflight\] fixture deterministic gate$' "$log")" == "$complete_gate_count" ]]
cp "$state" "$state.complete.saved"
jq '.head = "0000000000000000000000000000000000000000"' "$state" >"$state.tmp"
mv "$state.tmp" "$state"
complete_stale_head_rc=0
env "${common_env[@]}" HEPTA_RESUME_TEST_FAIL=0 \
  "$fixture/scripts/hepta-preflight-resume" --show >/dev/null 2>&1 || complete_stale_head_rc=$?
[[ "$complete_stale_head_rc" == "1" ]]
mv "$state.complete.saved" "$state"
complete_stale_environment_show="$(
  env "${common_env[@]}" HEPTA_RESUME_TEST_FAIL=0 CARGO_TARGET_DIR="$tmp/complete-stale-target" \
    "$fixture/scripts/hepta-preflight-resume" --show
)"
jq -e '
  .status == "complete_invalid"
  and .resumable == false
  and .environment_matches == false
' >/dev/null <<<"$complete_stale_environment_show"
complete_stale_environment_rc=0
env "${common_env[@]}" HEPTA_RESUME_TEST_FAIL=0 CARGO_TARGET_DIR="$tmp/complete-stale-target" \
  "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 || complete_stale_environment_rc=$?
[[ "$complete_stale_environment_rc" == "1" ]]
touch "$fixture/COMPLETE_DIRTY_FIXTURE"
complete_dirty_show="$(
  env "${common_env[@]}" HEPTA_RESUME_TEST_FAIL=0 \
    "$fixture/scripts/hepta-preflight-resume" --show
)"
jq -e '
  .status == "complete_invalid"
  and .resumable == false
  and .worktree_clean == false
  and .worktree_state_matches == false
' >/dev/null <<<"$complete_dirty_show"
complete_dirty_rc=0
env "${common_env[@]}" HEPTA_RESUME_TEST_FAIL=0 \
  "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 || complete_dirty_rc=$?
[[ "$complete_dirty_rc" == "1" ]]
rm -f "$fixture/COMPLETE_DIRTY_FIXTURE"
cp "$log" "$log.complete.saved"
printf '%s\n' '[fixture] complete evidence tampered' >>"$log"
complete_tampered_log_show="$(
  env "${common_env[@]}" HEPTA_RESUME_TEST_FAIL=0 \
    "$fixture/scripts/hepta-preflight-resume" --show
)"
jq -e '
  .status == "complete_invalid"
  and .resumable == false
  and .log_matches == false
' >/dev/null <<<"$complete_tampered_log_show"
complete_tampered_log_rc=0
env "${common_env[@]}" HEPTA_RESUME_TEST_FAIL=0 \
  "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 || complete_tampered_log_rc=$?
[[ "$complete_tampered_log_rc" == "1" ]]
mv "$log.complete.saved" "$log"
rm -f "$log"
import_rc=0
env "${common_env[@]}" "$fixture/scripts/hepta-preflight-resume" \
  --import-log "$tmp/untrusted.log" >/dev/null 2>&1 || import_rc=$?
[[ "$import_rc" == "2" ]] || {
  echo "resume fixture import-log returned $import_rc; expected fail-closed exit 2" >&2
  exit 1
}
run_failure() {
  local rc=0
  env "${common_env[@]}" HEPTA_RESUME_TEST_FAIL=1 \
    "$fixture/scripts/hepta-preflight-resume" "$@" >/dev/null 2>&1 || rc=$?
  [[ "$rc" == "42" ]] || {
    echo "resume fixture failure returned $rc; expected 42" >&2
    exit 1
  }
}
broken_state_dir="$tmp/broken-state"
mkdir -p "$broken_state_dir"
checkpoint_output="$tmp/checkpoint-output.log"
checkpoint_rc=0
env \
  HEPTA_PREFLIGHT_RESUME_STATE="$broken_state_dir/resume.state" \
  HEPTA_PREFLIGHT_RESUME_LOG="$tmp/broken-state-run.log" \
  HEPTA_PREFLIGHT_NATIVE=0 \
  HEPTA_PREFLIGHT_RELEASE=1 \
  CARGO_TARGET_DIR="$target" \
  HEPTA_RESUME_TEST_EXPECTED_TARGET="$target" \
  HEPTA_RESUME_TEST_BREAK_STATE=1 \
  HEPTA_RESUME_TEST_STATE_DIR="$broken_state_dir" \
  "$fixture/scripts/hepta-preflight-resume" >"$checkpoint_output" 2>&1 || checkpoint_rc=$?
[[ "$checkpoint_rc" == "1" ]] || {
  echo "resume fixture checkpoint failure returned $checkpoint_rc; expected 1" >&2
  exit 1
}
[[ ! -e "$broken_state_dir/resume.state" ]]
run_failure --reset
show_json="$(env "${common_env[@]}" "$fixture/scripts/hepta-preflight-resume" --show)"
jq -e '
  .status == "blocked"
  and .resumable == true
  and .attempt == 1
  and .failure_streak == 1
  and .fuse_armed == false
  and .environment_matches == true
  and .preflight_matches == true
  and .log_matches == true
  and .last_exit.preflight == 42
  and .last_exit.combined == 42
' >/dev/null <<<"$show_json"
cp "$log" "$tmp/resume-log.saved"
printf '%s\n' '[fixture] tampered' >>"$log"
tampered_show_json="$(env "${common_env[@]}" "$fixture/scripts/hepta-preflight-resume" --show)"
jq -e '
  .status == "blocked"
  and .resumable == false
  and .log_matches == false
' >/dev/null <<<"$tampered_show_json"
tampered_rc=0
env "${common_env[@]}" HEPTA_RESUME_TEST_FAIL=1 \
  "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 || tampered_rc=$?
[[ "$tampered_rc" == "1" ]]
mv "$tmp/resume-log.saved" "$log"
changed_show_json="$(
  env "${common_env[@]}" CARGO_TARGET_DIR="$tmp/different-target" \
    "$fixture/scripts/hepta-preflight-resume" --show
)"
jq -e '
  .status == "blocked"
  and .resumable == false
  and .environment_matches == false
' >/dev/null <<<"$changed_show_json"
policy_show_json="$(
  env "${common_env[@]}" HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES=25 \
    "$fixture/scripts/hepta-preflight-resume" --show
)"
jq -e '
  .status == "blocked"
  and .resumable == false
  and .environment_matches == false
' >/dev/null <<<"$policy_show_json"
policy_rc=0
env "${common_env[@]}" HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES=25 \
  "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 || policy_rc=$?
[[ "$policy_rc" == "1" ]]
unlisted_policy_show_json="$(
  env "${common_env[@]}" HEPTA_UNLISTED_RESUME_POLICY=changed \
    "$fixture/scripts/hepta-preflight-resume" --show
)"
jq -e '
  .resumable == false
  and .environment_matches == false
  and .relevant_environment_matches == false
' >/dev/null <<<"$unlisted_policy_show_json"
session_state="$tmp/session.state"
session_log="$tmp/session.log"
session_codex_home="$tmp/session-codex-home"
session_hepta_home="$tmp/session-hepta-home"
session_arg0_one="$session_hepta_home/tmp/arg0/codex-arg0-first"
session_arg0_two="$session_hepta_home/tmp/arg0/codex-arg0-second"
session_non_effective_arg0="$session_codex_home/tmp/arg0/codex-arg0-non-effective"
semantic_path_entry="$tmp/semantic-path-entry"
mkdir -p \
  "$session_arg0_one" \
  "$session_arg0_two" \
  "$session_non_effective_arg0" \
  "$semantic_path_entry"
session_env=(
  HEPTA_PREFLIGHT_RESUME_STATE="$session_state"
  HEPTA_PREFLIGHT_RESUME_LOG="$session_log"
  HEPTA_PREFLIGHT_NATIVE=0
  HEPTA_PREFLIGHT_RELEASE=1
  HEPTA_RESUME_TEST_FAIL=1
  CARGO_TARGET_DIR="$target"
  HEPTA_RESUME_TEST_EXPECTED_TARGET="$target"
  HEPTA_RESUME_TEST_EXPECTED_PATH="$PATH"
  RESUME_TEST_FORBIDDEN_ARG0_ROOT="$session_hepta_home/tmp/arg0"
  HEPTA_HOME="$session_hepta_home"
  CODEX_HOME="$session_codex_home"
)
session_rc=0
env "${session_env[@]}" \
  CODEX_THREAD_ID=fixture-thread-one \
  PATH="$session_arg0_one:$PATH" \
  "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 || session_rc=$?
[[ "$session_rc" == "42" ]]
new_thread_show_json="$(
  env "${session_env[@]}" \
    CODEX_THREAD_ID=fixture-thread-two \
    PATH="$session_arg0_two:$PATH" \
    "$fixture/scripts/hepta-preflight-resume" --show
)"
jq -e '
  .status == "blocked"
  and .resumable == true
  and .environment_matches == true
  and .relevant_environment_matches == true
' >/dev/null <<<"$new_thread_show_json"
new_thread_rc=0
env "${session_env[@]}" \
  CODEX_THREAD_ID=fixture-thread-two \
  PATH="$session_arg0_two:$PATH" \
  "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 || new_thread_rc=$?
[[ "$new_thread_rc" == "42" ]]
[[ "$(jq -r '.attempt' "$session_state")" == "2" ]]
non_effective_home_show_json="$(
  env "${session_env[@]}" \
    CODEX_THREAD_ID=fixture-thread-two \
    PATH="$session_non_effective_arg0:$PATH" \
    "$fixture/scripts/hepta-preflight-resume" --show
)"
jq -e '
  .status == "blocked"
  and .resumable == false
  and .environment_matches == false
  and .relevant_environment_matches == false
' >/dev/null <<<"$non_effective_home_show_json"
non_effective_home_rc=0
env "${session_env[@]}" \
  CODEX_THREAD_ID=fixture-thread-two \
  PATH="$session_non_effective_arg0:$PATH" \
  "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 \
  || non_effective_home_rc=$?
[[ "$non_effective_home_rc" == "1" ]]
semantic_path_show_json="$(
  env "${session_env[@]}" \
    CODEX_THREAD_ID=fixture-thread-two \
    PATH="$session_arg0_two:$semantic_path_entry:$PATH" \
    "$fixture/scripts/hepta-preflight-resume" --show
)"
jq -e '
  .status == "blocked"
  and .resumable == false
  and .environment_matches == false
  and .relevant_environment_matches == false
' >/dev/null <<<"$semantic_path_show_json"
semantic_path_rc=0
env "${session_env[@]}" \
  CODEX_THREAD_ID=fixture-thread-two \
  PATH="$session_arg0_two:$semantic_path_entry:$PATH" \
  "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 || semantic_path_rc=$?
[[ "$semantic_path_rc" == "1" ]]
env "${session_env[@]}" \
  HEPTA_RESUME_TEST_FAIL=0 \
  CODEX_THREAD_ID=fixture-thread-two \
  PATH="$session_arg0_two:$PATH" \
  "$fixture/scripts/hepta-preflight-resume" --reset >/dev/null
assert_complete_state "$session_state"
codex_fallback_state="$tmp/codex-fallback.state"
codex_fallback_log="$tmp/codex-fallback.log"
codex_fallback_arg0_one="$session_codex_home/tmp/arg0/codex-arg0-fallback-one"
codex_fallback_arg0_two="$session_codex_home/tmp/arg0/codex-arg0-fallback-two"
mkdir -p "$codex_fallback_arg0_one" "$codex_fallback_arg0_two"
codex_fallback_env=(
  HEPTA_PREFLIGHT_RESUME_STATE="$codex_fallback_state"
  HEPTA_PREFLIGHT_RESUME_LOG="$codex_fallback_log"
  HEPTA_PREFLIGHT_NATIVE=0
  HEPTA_PREFLIGHT_RELEASE=1
  HEPTA_RESUME_TEST_FAIL=1
  CARGO_TARGET_DIR="$target"
  HEPTA_RESUME_TEST_EXPECTED_TARGET="$target"
  HEPTA_RESUME_TEST_EXPECTED_PATH="$PATH"
  RESUME_TEST_FORBIDDEN_ARG0_ROOT="$session_codex_home/tmp/arg0"
  HEPTA_HOME=
  CODEX_HOME="$session_codex_home"
)
codex_fallback_rc=0
env "${codex_fallback_env[@]}" \
  CODEX_THREAD_ID=fixture-fallback-thread-one \
  PATH="$codex_fallback_arg0_one:$PATH" \
  "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 \
  || codex_fallback_rc=$?
[[ "$codex_fallback_rc" == "42" ]]
codex_fallback_show_json="$(
  env "${codex_fallback_env[@]}" \
    CODEX_THREAD_ID=fixture-fallback-thread-two \
    PATH="$codex_fallback_arg0_two:$PATH" \
    "$fixture/scripts/hepta-preflight-resume" --show
)"
jq -e '
  .status == "blocked"
  and .resumable == true
  and .environment_matches == true
  and .relevant_environment_matches == true
' >/dev/null <<<"$codex_fallback_show_json"
codex_fallback_resume_rc=0
env "${codex_fallback_env[@]}" \
  CODEX_THREAD_ID=fixture-fallback-thread-two \
  PATH="$codex_fallback_arg0_two:$PATH" \
  "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 \
  || codex_fallback_resume_rc=$?
[[ "$codex_fallback_resume_rc" == "42" ]]
env "${codex_fallback_env[@]}" \
  HEPTA_RESUME_TEST_FAIL=0 \
  CODEX_THREAD_ID=fixture-fallback-thread-two \
  PATH="$codex_fallback_arg0_two:$PATH" \
  "$fixture/scripts/hepta-preflight-resume" --reset >/dev/null
assert_complete_state "$codex_fallback_state"
default_state="$tmp/default-home.state"
default_log="$tmp/default-home.log"
default_output="$tmp/default-home.output"
default_user_home="$tmp/default-user-home"
default_runtime_home="$default_user_home/.hepta"
default_arg0_one="$default_runtime_home/tmp/arg0/codex-arg0-default-one"
default_arg0_two="$default_runtime_home/tmp/arg0/codex-arg0-default-two"
mkdir -p "$default_arg0_one" "$default_arg0_two"
default_env=(
  HEPTA_PREFLIGHT_RESUME_STATE="$default_state"
  HEPTA_PREFLIGHT_RESUME_LOG="$default_log"
  HEPTA_PREFLIGHT_NATIVE=0
  HEPTA_PREFLIGHT_RELEASE=1
  HEPTA_RESUME_TEST_FAIL=1
  CARGO_TARGET_DIR="$target"
  HEPTA_RESUME_TEST_EXPECTED_TARGET="$target"
  HEPTA_RESUME_TEST_EXPECTED_PATH="$PATH"
  RESUME_TEST_FORBIDDEN_ARG0_ROOT="$default_runtime_home/tmp/arg0"
  HOME="$default_user_home"
)
default_rc=0
env -u HEPTA_HOME -u CODEX_HOME "${default_env[@]}" \
  CODEX_THREAD_ID=fixture-default-thread-one \
  PATH="$default_arg0_one:$PATH" \
  /bin/bash "$fixture/scripts/hepta-preflight-resume" >"$default_output" 2>&1 \
  || default_rc=$?
[[ "$default_rc" == "42" ]] || {
  cat "$default_output" >&2
  echo "default-home resume fixture returned $default_rc; expected 42" >&2
  exit 1
}
default_show_json="$(
  env -u HEPTA_HOME -u CODEX_HOME "${default_env[@]}" \
    CODEX_THREAD_ID=fixture-default-thread-two \
    PATH="$default_arg0_two:$PATH" \
    /bin/bash "$fixture/scripts/hepta-preflight-resume" --show
)"
jq -e '
  .status == "blocked"
  and .resumable == true
  and .environment_matches == true
  and .relevant_environment_matches == true
' >/dev/null <<<"$default_show_json"
default_resume_rc=0
env -u HEPTA_HOME -u CODEX_HOME "${default_env[@]}" \
  CODEX_THREAD_ID=fixture-default-thread-two \
  PATH="$default_arg0_two:$PATH" \
  /bin/bash "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 \
  || default_resume_rc=$?
[[ "$default_resume_rc" == "42" ]]
env -u HEPTA_HOME -u CODEX_HOME "${default_env[@]}" \
  HEPTA_RESUME_TEST_FAIL=0 \
  CODEX_THREAD_ID=fixture-default-thread-two \
  PATH="$default_arg0_two:$PATH" \
  /bin/bash "$fixture/scripts/hepta-preflight-resume" --reset >/dev/null
assert_complete_state "$default_state"
unresolved_home_state="$tmp/unresolved-home.state"
unresolved_home_log="$tmp/unresolved-home.log"
unresolved_arg0="$tmp/unresolved-root/codex-arg0-unowned"
mkdir -p "$unresolved_arg0"
unresolved_home_show_json="$(
  env -u HEPTA_HOME -u CODEX_HOME -u HOME \
    HEPTA_PREFLIGHT_RESUME_STATE="$unresolved_home_state" \
    HEPTA_PREFLIGHT_RESUME_LOG="$unresolved_home_log" \
    HEPTA_PREFLIGHT_NATIVE=0 \
    HEPTA_PREFLIGHT_RELEASE=0 \
    CARGO_TARGET_DIR="$target" \
    HEPTA_RESUME_TEST_EXPECTED_TARGET="$target" \
    PATH="$unresolved_arg0:$PATH" \
    /bin/bash "$fixture/scripts/hepta-preflight-resume" --show
)"
jq -e '
  .status == "fresh"
  and .resumable == true
  and .environment_matches == true
' >/dev/null <<<"$unresolved_home_show_json"
symlink_state="$tmp/symlink-home.state"
symlink_log="$tmp/symlink-home.log"
symlink_home="$tmp/stable-hepta-home"
symlink_real_a="$tmp/hepta-home-real-a"
symlink_real_b="$tmp/hepta-home-real-b"
mkdir -p \
  "$symlink_real_a/tmp/arg0/codex-arg0-symlink-one" \
  "$symlink_real_b/tmp/arg0/codex-arg0-symlink-two"
symlink_real_a_physical="$(cd "$symlink_real_a" && pwd -P)"
symlink_real_b_physical="$(cd "$symlink_real_b" && pwd -P)"
symlink_arg0_one="$symlink_real_a_physical/tmp/arg0/codex-arg0-symlink-one"
symlink_arg0_two="$symlink_real_b_physical/tmp/arg0/codex-arg0-symlink-two"
ln -s "$symlink_real_a" "$symlink_home"
symlink_env=(
  HEPTA_PREFLIGHT_RESUME_STATE="$symlink_state"
  HEPTA_PREFLIGHT_RESUME_LOG="$symlink_log"
  HEPTA_PREFLIGHT_NATIVE=0
  HEPTA_PREFLIGHT_RELEASE=1
  HEPTA_RESUME_TEST_FAIL=1
  CARGO_TARGET_DIR="$target"
  HEPTA_RESUME_TEST_EXPECTED_TARGET="$target"
  HEPTA_HOME="$symlink_home"
  CODEX_HOME="$session_codex_home"
)
symlink_rc=0
env "${symlink_env[@]}" \
  PATH="$symlink_arg0_one:$PATH" \
  "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 || symlink_rc=$?
[[ "$symlink_rc" == "42" ]]
rm -f "$symlink_home"
ln -s "$symlink_real_b" "$symlink_home"
symlink_show_json="$(
  env "${symlink_env[@]}" \
    PATH="$symlink_arg0_two:$PATH" \
    "$fixture/scripts/hepta-preflight-resume" --show
)"
jq -e '
  .status == "blocked"
  and .resumable == false
  and .environment_matches == false
  and .relevant_environment_matches == false
' >/dev/null <<<"$symlink_show_json"
symlink_resume_rc=0
env "${symlink_env[@]}" \
  PATH="$symlink_arg0_two:$PATH" \
  "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 \
  || symlink_resume_rc=$?
[[ "$symlink_resume_rc" == "1" ]]
env "${symlink_env[@]}" \
  HEPTA_RESUME_TEST_FAIL=0 \
  PATH="$symlink_arg0_two:$PATH" \
  "$fixture/scripts/hepta-preflight-resume" --reset >/dev/null
assert_complete_state "$symlink_state"
cp "$state" "$state.worktree-id.saved"
jq '.evidence.worktree_id = "different-worktree"' "$state" >"$state.tmp"
mv "$state.tmp" "$state"
worktree_identity_show_json="$(env "${common_env[@]}" "$fixture/scripts/hepta-preflight-resume" --show)"
jq -e '
  .resumable == false
  and .worktree_identity_matches == false
' >/dev/null <<<"$worktree_identity_show_json"
worktree_identity_rc=0
env "${common_env[@]}" "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 \
  || worktree_identity_rc=$?
[[ "$worktree_identity_rc" == "1" ]]
mv "$state.worktree-id.saved" "$state"
touch "$fixture/UNTRACKED_RESUME_FIXTURE"
dirty_show_json="$(env "${common_env[@]}" "$fixture/scripts/hepta-preflight-resume" --show)"
jq -e '
  .resumable == false
  and .worktree_clean == false
  and .worktree_state_matches == false
' >/dev/null <<<"$dirty_show_json"
dirty_rc=0
env "${common_env[@]}" "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 || dirty_rc=$?
[[ "$dirty_rc" == "1" ]]
rm -f "$fixture/UNTRACKED_RESUME_FIXTURE"
run_failure
run_failure
jq -e '
  .schema_version == "hepta_preflight_resume_state_v2"
  and .status == "blocked"
  and .attempt == 3
  and .failure_streak == 3
  and .fuse_threshold == 3
  and .fuse_armed == true
  and (.evidence.preflight_sha256 | test("^[0-9a-f]{64}$"))
  and (.evidence.environment_sha256 | test("^[0-9a-f]{64}$"))
  and (.evidence.cargo_identity_sha256 | test("^[0-9a-f]{64}$"))
  and (.evidence.rustc_identity_sha256 | test("^[0-9a-f]{64}$"))
  and (.evidence.toolchain_identity_sha256 | test("^[0-9a-f]{64}$"))
  and (.evidence.relevant_environment_sha256 | test("^[0-9a-f]{64}$"))
  and (.evidence.worktree_state_sha256 | test("^[0-9a-f]{64}$"))
  and (.evidence.log_sha256 | test("^[0-9a-f]{64}$"))
  and (.evidence.log_prefix_sha256 | test("^[0-9a-f]{64}$"))
  and .evidence.log_prefix_bytes >= 0
  and (.evidence.attempt_log_sha256 | test("^[0-9a-f]{64}$"))
  and (.evidence.failure_fingerprint_sha256 | test("^[0-9a-f]{64}$"))
' "$state" >/dev/null
fused_rc=0
env "${common_env[@]}" HEPTA_RESUME_TEST_FAIL=1 \
  "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 || fused_rc=$?
[[ "$fused_rc" == "75" ]] || {
  echo "resume fixture fuse returned $fused_rc; expected 75" >&2
  exit 1
}
[[ "$(jq -r '.attempt' "$state")" == "3" ]]
[[ "$(grep -c '^\[fixture\] release_target=' "$log")" == "3" ]]
env "${common_env[@]}" HEPTA_RESUME_TEST_FAIL=0 \
  "$fixture/scripts/hepta-preflight-resume" --reset >/dev/null
assert_complete_state "$state"
grep -q '^Hepta preflight passed$' "$log"
rm -f "$state" "$log"
run_failure
jq '
  .status = "running"
  | .fuse_armed = false
  | .last_exit = null
  | .evidence.log_sha256 = null
' "$state" >"$state.tmp"
mv "$state.tmp" "$state"
run_failure
jq -e '
  .status == "blocked"
  and .attempt == 2
  and .failure_streak == 2
' "$state" >/dev/null
env "${common_env[@]}" HEPTA_RESUME_TEST_FAIL=0 \
  "$fixture/scripts/hepta-preflight-resume" --reset >/dev/null
fake_bin="$tmp/fake-bin"
mkdir -p "$fake_bin"
write_fake_tool() {
  local tool="$1"
  local version="$2"
  cat >"$fake_bin/$tool" <<EOF
#!/usr/bin/env bash
printf '%s\\n' '$tool-$version'
EOF
  chmod +x "$fake_bin/$tool"
}
write_fake_tool cargo v1
write_fake_tool rustc v1
write_fake_tool rustup v1
identity_state="$tmp/identity.state"
identity_log="$tmp/identity.log"
identity_env=(
  HEPTA_PREFLIGHT_RESUME_STATE="$identity_state"
  HEPTA_PREFLIGHT_RESUME_LOG="$identity_log"
  HEPTA_PREFLIGHT_NATIVE=0
  HEPTA_PREFLIGHT_RELEASE=1
  HEPTA_RESUME_TEST_FAIL=1
  CARGO_TARGET_DIR="$target"
  HEPTA_RESUME_TEST_EXPECTED_TARGET="$target"
  PATH="$fake_bin:$PATH"
)
identity_rc=0
env "${identity_env[@]}" HEPTA_RESUME_TEST_FAIL=1 \
  "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 || identity_rc=$?
[[ "$identity_rc" == "42" ]]
write_fake_tool cargo v2
identity_show="$(env "${identity_env[@]}" "$fixture/scripts/hepta-preflight-resume" --show)"
jq -e '
  .resumable == false
  and .environment_matches == false
  and .toolchain_matches == false
' >/dev/null <<<"$identity_show"
identity_resume_rc=0
env "${identity_env[@]}" "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 \
  || identity_resume_rc=$?
[[ "$identity_resume_rc" == "1" ]]
env "${identity_env[@]}" HEPTA_RESUME_TEST_FAIL=0 \
  "$fixture/scripts/hepta-preflight-resume" --reset >/dev/null
fresh_lock_state="$tmp/fresh-lock.state"
fresh_lock_log="$tmp/fresh-lock.log"
fresh_lock_process_start="$(
  LC_ALL=C TZ=UTC ps -o lstart= -p $$ 2>/dev/null \
    | sed 's/^[[:space:]]*//; s/[[:space:]]*$//'
)"
mkdir "$fresh_lock_state.lock"
jq -n \
  --arg host "$(hostname 2>/dev/null || uname -n)" \
  --arg process_start "$fresh_lock_process_start" \
  --arg head "$fixture_head" \
  --arg worktree "$fixture" \
  --argjson pid "$$" \
  --argjson uid "$(id -u)" \
  '{
    schema_version:"hepta_preflight_resume_lock_v1",
    run_id:"fresh-live-fixture",
    pid:$pid,
    uid:$uid,
    host:$host,
    process_start:$process_start,
    head:$head,
    worktree:$worktree,
    worktree_id:"fresh-live-fixture",
    created_at:"2001-01-01T00:00:00Z"
  }' >"$fresh_lock_state.lock/owner.json"
fresh_lock_show="$(
  env \
    HEPTA_PREFLIGHT_RESUME_STATE="$fresh_lock_state" \
    HEPTA_PREFLIGHT_RESUME_LOG="$fresh_lock_log" \
    HEPTA_PREFLIGHT_NATIVE=0 \
    HEPTA_PREFLIGHT_RELEASE=0 \
    CARGO_TARGET_DIR="$target" \
    HEPTA_RESUME_TEST_EXPECTED_TARGET="$target" \
    "$fixture/scripts/hepta-preflight-resume" --show
)"
jq -e '
  .status == "fresh"
  and .lock_state == "live"
  and .resumable == false
' >/dev/null <<<"$fresh_lock_show"
rm -f "$fresh_lock_state.lock/owner.json"
rmdir "$fresh_lock_state.lock"
concurrent_state="$tmp/concurrent.state"
concurrent_log="$tmp/concurrent.log"
concurrent_output="$tmp/concurrent.output"
concurrent_env=(
  HEPTA_PREFLIGHT_RESUME_STATE="$concurrent_state"
  HEPTA_PREFLIGHT_RESUME_LOG="$concurrent_log"
  HEPTA_PREFLIGHT_NATIVE=0
  HEPTA_PREFLIGHT_RELEASE=0
  CARGO_TARGET_DIR="$target"
  HEPTA_RESUME_TEST_EXPECTED_TARGET="$target"
  HEPTA_RESUME_TEST_SLEEP_SECONDS=3
)
env "${concurrent_env[@]}" "$fixture/scripts/hepta-preflight-resume" \
  >"$concurrent_output" 2>&1 &
concurrent_pid=$!
for _ in $(seq 1 100); do
  [[ -f "$concurrent_state.lock/owner.json" ]] && break
  sleep 0.05
done
[[ -f "$concurrent_state.lock/owner.json" ]]
jq -e --argjson pid "$concurrent_pid" '
  .schema_version == "hepta_preflight_resume_lock_v1"
  and .pid == $pid
  and (.uid | type) == "number"
  and (.process_start | length) > 0
' "$concurrent_state.lock/owner.json" >/dev/null
for _ in $(seq 1 100); do
  if [[ -f "$concurrent_state" ]] \
    && jq -e '.status == "running"' "$concurrent_state" >/dev/null 2>&1; then
    break
  fi
  sleep 0.05
done
concurrent_show="$(
  env "${concurrent_env[@]}" "$fixture/scripts/hepta-preflight-resume" --show
)"
jq -e '
  .status == "running"
  and .status != "complete"
  and .lock_state == "live"
  and .resumable == false
' >/dev/null <<<"$concurrent_show"
concurrent_second_rc=0
env "${concurrent_env[@]}" "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 \
  || concurrent_second_rc=$?
[[ "$concurrent_second_rc" == "75" ]]
wait "$concurrent_pid"
[[ ! -e "$concurrent_state.lock" ]]
stale_state="$tmp/stale.state"
stale_log="$tmp/stale.log"
mkdir "$stale_state.lock"
jq -n \
  --arg host "$(hostname 2>/dev/null || uname -n)" \
  --arg head "$fixture_head" \
  --arg worktree "$fixture" \
  --argjson uid "$(id -u)" \
  '{
    schema_version:"hepta_preflight_resume_lock_v1",
    run_id:"stale-fixture",
    pid:99999999,
    uid:$uid,
    host:$host,
    process_start:"Mon Jan  1 00:00:00 2001",
    head:$head,
    worktree:$worktree,
    worktree_id:"stale-fixture",
    created_at:"2001-01-01T00:00:00Z"
  }' >"$stale_state.lock/owner.json"
env \
  HEPTA_PREFLIGHT_RESUME_STATE="$stale_state" \
  HEPTA_PREFLIGHT_RESUME_LOG="$stale_log" \
  HEPTA_PREFLIGHT_NATIVE=0 \
  HEPTA_PREFLIGHT_RELEASE=0 \
  CARGO_TARGET_DIR="$target" \
  HEPTA_RESUME_TEST_EXPECTED_TARGET="$target" \
  "$fixture/scripts/hepta-preflight-resume" >/dev/null
[[ ! -e "$stale_state.lock" ]]
race_state="$tmp/race.state"
race_log="$tmp/race.log"
race_signal="$tmp/race.quarantined"
race_bin="$tmp/race-bin"
mkdir "$race_state.lock" "$race_bin"
jq -n \
  --arg host "$(hostname 2>/dev/null || uname -n)" \
  --arg head "$fixture_head" \
  --arg worktree "$fixture" \
  --argjson uid "$(id -u)" \
  '{
    schema_version:"hepta_preflight_resume_lock_v1",
    run_id:"race-stale-fixture",
    pid:99999999,
    uid:$uid,
    host:$host,
    process_start:"Mon Jan  1 00:00:00 2001",
    head:$head,
    worktree:$worktree,
    worktree_id:"race-stale-fixture",
    created_at:"2001-01-01T00:00:00Z"
  }' >"$race_state.lock/owner.json"
cat >"$race_bin/mv" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
/bin/mv "$@"
if [[ "$#" == "2" \
  && "$1" == "${HEPTA_RESUME_TEST_RACE_STATE:?}.lock" \
  && "$2" == "${HEPTA_RESUME_TEST_RACE_STATE:?}.lock.stale."* ]]; then
  touch "${HEPTA_RESUME_TEST_RACE_SIGNAL:?}"
  sleep 3
fi
EOF
chmod +x "$race_bin/mv"
race_env=(
  HEPTA_PREFLIGHT_RESUME_STATE="$race_state"
  HEPTA_PREFLIGHT_RESUME_LOG="$race_log"
  HEPTA_PREFLIGHT_NATIVE=0
  HEPTA_PREFLIGHT_RELEASE=0
  HEPTA_RESUME_TEST_RACE_STATE="$race_state"
  HEPTA_RESUME_TEST_RACE_SIGNAL="$race_signal"
  CARGO_TARGET_DIR="$target"
  HEPTA_RESUME_TEST_EXPECTED_TARGET="$target"
  PATH="$race_bin:$PATH"
)
env "${race_env[@]}" "$fixture/scripts/hepta-preflight-resume" \
  >"$tmp/race-recovery.output" 2>&1 &
race_recovery_pid=$!
for _ in $(seq 1 100); do
  [[ -f "$race_signal" ]] && break
  sleep 0.05
done
[[ -f "$race_signal" ]]
[[ -d "$race_state.lock.reclaim" ]]
[[ ! -e "$race_state.lock" ]]
race_contender_rc=0
env "${race_env[@]}" "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 \
  || race_contender_rc=$?
[[ "$race_contender_rc" == "75" ]]
race_recovery_rc=0
wait "$race_recovery_pid" || race_recovery_rc=$?
[[ "$race_recovery_rc" == "0" ]]
[[ ! -e "$race_state.lock" ]]
[[ ! -e "$race_state.lock.reclaim" ]]
orphan_state="$tmp/orphan.state"
orphan_log="$tmp/orphan.log"
mkdir "$orphan_state.lock" "$orphan_state.lock.reclaim"
jq -n \
  --arg host "$(hostname 2>/dev/null || uname -n)" \
  --arg head "$fixture_head" \
  --arg worktree "$fixture" \
  --argjson uid "$(id -u)" \
  '{
    schema_version:"hepta_preflight_resume_lock_v1",
    run_id:"orphan-fixture",
    pid:99999999,
    uid:$uid,
    host:$host,
    process_start:"Mon Jan  1 00:00:00 2001",
    head:$head,
    worktree:$worktree,
    worktree_id:"orphan-fixture",
    created_at:"2001-01-01T00:00:00Z"
  }' >"$orphan_state.lock/owner.json"
orphan_rc=0
env \
  HEPTA_PREFLIGHT_RESUME_STATE="$orphan_state" \
  HEPTA_PREFLIGHT_RESUME_LOG="$orphan_log" \
  HEPTA_PREFLIGHT_NATIVE=0 \
  HEPTA_PREFLIGHT_RELEASE=0 \
  CARGO_TARGET_DIR="$target" \
  HEPTA_RESUME_TEST_EXPECTED_TARGET="$target" \
  "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 || orphan_rc=$?
[[ "$orphan_rc" == "75" ]]
[[ -d "$orphan_state.lock.reclaim" ]]
rm -f "$orphan_state.lock/owner.json"
rmdir "$orphan_state.lock.reclaim" "$orphan_state.lock"
log_failure_state="$tmp/log-failure.state"
log_failure_log="$tmp/log-failure.log"
log_failure_env=(
  HEPTA_PREFLIGHT_RESUME_STATE="$log_failure_state"
  HEPTA_PREFLIGHT_RESUME_LOG="$log_failure_log"
  HEPTA_PREFLIGHT_NATIVE=0
  HEPTA_PREFLIGHT_RELEASE=0
  CARGO_TARGET_DIR="$target"
  HEPTA_RESUME_TEST_EXPECTED_TARGET="$target"
)
log_failure_rc=0
env "${log_failure_env[@]}" HEPTA_RESUME_TEST_BREAK_LOG=1 HEPTA_RESUME_TEST_FAIL=1 \
  "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 || log_failure_rc=$?
[[ "$log_failure_rc" == "42" ]]
jq -e '
  .status == "blocked"
  and .failure_streak == 1
  and .last_exit.preflight == 42
  and .evidence.log_sha256 == null
  and (.evidence.log_prefix_sha256 | test("^[0-9a-f]{64}$"))
  and .evidence.log_prefix_bytes > 0
  and (.evidence.attempt_log_sha256 | test("^[0-9a-f]{64}$"))
' "$log_failure_state" >/dev/null
[[ -d "$log_failure_log" ]]
log_failure_show="$(
  env "${log_failure_env[@]}" HEPTA_RESUME_TEST_BREAK_LOG=1 HEPTA_RESUME_TEST_FAIL=1 \
    "$fixture/scripts/hepta-preflight-resume" --show
)"
jq -e '
  .status == "blocked"
  and .resumable == false
  and .log_matches == false
' >/dev/null <<<"$log_failure_show"
rmdir "$log_failure_log"
env "${log_failure_env[@]}" "$fixture/scripts/hepta-preflight-resume" --reset >/dev/null
post_provenance_state="$tmp/post-provenance.state"
post_provenance_log="$tmp/post-provenance.log"
post_provenance_fail="$tmp/post-provenance.fail"
post_provenance_env=(
  HEPTA_PREFLIGHT_RESUME_STATE="$post_provenance_state"
  HEPTA_PREFLIGHT_RESUME_LOG="$post_provenance_log"
  HEPTA_PREFLIGHT_NATIVE=1
  HEPTA_PREFLIGHT_RELEASE=1
  HEPTA_RESUME_TEST_FAIL_AFTER_PROVENANCE_FILE="$post_provenance_fail"
  CARGO_TARGET_DIR="$target"
  HEPTA_RESUME_TEST_EXPECTED_TARGET="$target"
)
touch "$post_provenance_fail"
post_provenance_rc=0
env "${post_provenance_env[@]}" "$fixture/scripts/hepta-preflight-resume" \
  >/dev/null 2>&1 || post_provenance_rc=$?
[[ "$post_provenance_rc" == "42" ]]
[[ "$(grep -c '^\[hepta-preflight-provenance\] ' "$post_provenance_log")" == "1" ]]
[[ "$(grep -c '^\[hepta-preflight-final\] ' "$post_provenance_log")" == "1" ]]
[[ "$(grep -c '^Hepta preflight passed$' "$post_provenance_log" || true)" == "0" ]]
env "${post_provenance_env[@]}" "$fixture/scripts/hepta-preflight-resume" >/dev/null
assert_complete_state "$post_provenance_state"
[[ "$(grep -c '^\[hepta-preflight-provenance\] ' "$post_provenance_log")" == "2" ]]
[[ "$(grep -c '^\[hepta-preflight-final\] ' "$post_provenance_log")" == "2" ]]
[[ "$(grep -c '^Hepta preflight passed$' "$post_provenance_log")" == "1" ]]
tail -n 2 "$post_provenance_log" | sed -n '1p' \
  | grep -q '^\[hepta-preflight-final\] '
tail -n 1 "$post_provenance_log" | grep -qx 'Hepta preflight passed'
run_terminal_resume_case() {
  local name="$1"
  local native="$2"
  local release="$3"
  local marker="$4"
  local case_state="$tmp/terminal-$name.state"
  local case_log="$tmp/terminal-$name.log"
  local fail_file="$tmp/terminal-$name.fail"
  local case_env=(
    HEPTA_PREFLIGHT_RESUME_STATE="$case_state"
    HEPTA_PREFLIGHT_RESUME_LOG="$case_log"
    HEPTA_PREFLIGHT_NATIVE="$native"
    HEPTA_PREFLIGHT_RELEASE="$release"
    CARGO_TARGET_DIR="$target"
    HEPTA_RESUME_TEST_EXPECTED_TARGET="$target"
    HEPTA_RESUME_TEST_FAIL_FILE="$fail_file"
  )
  local case_rc=0
  touch "$fail_file"
  env "${case_env[@]}" \
    "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 || case_rc=$?
  [[ "$case_rc" == "42" ]]
  rm -f "$fail_file"
  jq --arg marker "$marker" '
    .status = "running"
    | .checkpoint.marker = $marker
    | .failure_streak = 0
    | .fuse_armed = false
    | .last_exit = null
    | .evidence.log_sha256 = null
  ' "$case_state" >"$case_state.tmp"
  mv "$case_state.tmp" "$case_state"
  env "${case_env[@]}" "$fixture/scripts/hepta-preflight-resume" >/dev/null
  assert_complete_state "$case_state"
  [[ "$(grep -c '^\[hepta-preflight\] fixture deterministic gate$' "$case_log")" == "1" ]]
  [[ "$(grep -c '^\[fixture\] release_target=' "$case_log")" == "2" ]]
  grep -Fqx \
    "[hepta-preflight] source-bound release gate receipt replay boundary" \
    "$case_log"
  grep -Fqx "[hepta-preflight] $marker" "$case_log"
}
run_terminal_resume_case dependency-receipt 1 1 "fixture dependency security receipt"
run_terminal_resume_case compatibility-receipt 1 1 "fixture compatibility receipt"
run_terminal_resume_case native-on 1 1 "fixture native gate"
run_terminal_resume_case native-skip 0 1 "native app gates skipped (HEPTA_PREFLIGHT_NATIVE=0)"
run_terminal_resume_case release-on 1 1 "fixture release gate"
run_terminal_resume_case release-skip 0 0 "release build skipped (set HEPTA_PREFLIGHT_RELEASE=1)"
run_terminal_resume_case terminal-tail 1 1 "fixture whitespace/status"
terminal_streak_state="$tmp/terminal-streak.state"
terminal_streak_log="$tmp/terminal-streak.log"
terminal_streak_initial_fail="$tmp/terminal-streak-initial.fail"
terminal_streak_target_fail="$tmp/terminal-streak-target.fail"
terminal_streak_signal="$tmp/terminal-streak.signal"
terminal_streak_continue="$tmp/terminal-streak.continue"
terminal_streak_env=(
  HEPTA_PREFLIGHT_RESUME_STATE="$terminal_streak_state"
  HEPTA_PREFLIGHT_RESUME_LOG="$terminal_streak_log"
  HEPTA_PREFLIGHT_RESUME_MAX_SAME_FAILURES=3
  HEPTA_PREFLIGHT_NATIVE=1
  HEPTA_PREFLIGHT_RELEASE=1
  HEPTA_RESUME_TEST_FAIL_FILE="$terminal_streak_initial_fail"
  HEPTA_RESUME_TEST_TERMINAL_FAIL_FILE="$terminal_streak_target_fail"
  HEPTA_RESUME_TEST_TERMINAL_REPLAY_SIGNAL="$terminal_streak_signal"
  HEPTA_RESUME_TEST_TERMINAL_REPLAY_CONTINUE="$terminal_streak_continue"
  CARGO_TARGET_DIR="$target"
  HEPTA_RESUME_TEST_EXPECTED_TARGET="$target"
)
touch "$terminal_streak_initial_fail"
terminal_streak_initial_rc=0
env "${terminal_streak_env[@]}" "$fixture/scripts/hepta-preflight-resume" \
  >/dev/null 2>&1 || terminal_streak_initial_rc=$?
[[ "$terminal_streak_initial_rc" == "42" ]]
rm -f "$terminal_streak_initial_fail"
jq '
  .status = "running"
  | .checkpoint.marker = "fixture release gate"
  | .failure_streak = 2
  | .fuse_armed = false
  | .last_exit = null
  | .evidence.log_sha256 = null
' "$terminal_streak_state" >"$terminal_streak_state.tmp"
mv "$terminal_streak_state.tmp" "$terminal_streak_state"
touch "$terminal_streak_target_fail"
env "${terminal_streak_env[@]}" "$fixture/scripts/hepta-preflight-resume" \
  >"$tmp/terminal-streak.output" 2>&1 &
terminal_streak_pid=$!
for _ in $(seq 1 100); do
  [[ -f "$terminal_streak_signal" ]] && break
  sleep 0.05
done
[[ -f "$terminal_streak_signal" ]]
jq -e '
  .status == "running"
  and .checkpoint.marker == "fixture release gate"
  and .failure_streak == 2
' "$terminal_streak_state" >/dev/null
touch "$terminal_streak_continue"
terminal_streak_rc=0
wait "$terminal_streak_pid" || terminal_streak_rc=$?
[[ "$terminal_streak_rc" == "42" ]]
jq -e '
  .status == "blocked"
  and .checkpoint.marker == "fixture release gate"
  and .failure_streak == 3
  and .fuse_armed == true
' "$terminal_streak_state" >/dev/null
echo "Hepta preflight resume self-test passed"
