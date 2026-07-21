#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
RESUME="$ROOT/scripts/hepta-preflight-resume"

tmp="$(mktemp -d /tmp/hepta-preflight-resume-self-test.XXXXXX)"
trap 'rm -rf "$tmp"' EXIT

fixture="$tmp/repo"
mkdir -p "$fixture/scripts" "$fixture/codex-rs" "$fixture/apps/hepta-native"
cp "$RESUME" "$fixture/scripts/hepta-preflight-resume"
chmod +x "$fixture/scripts/hepta-preflight-resume"

cat >"$fixture/scripts/hepta-preflight.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

RUN_NATIVE="${HEPTA_PREFLIGHT_NATIVE:-0}"
RUN_RELEASE="${HEPTA_PREFLIGHT_RELEASE:-0}"
PREFLIGHT_RELEASE_TARGET_DIR="${CARGO_TARGET_DIR:-target}"
printf '[fixture] release_target=%s\n' "$PREFLIGHT_RELEASE_TARGET_DIR"
# hepta-preflight-resume: prelude-end

if [[ "${HEPTA_RESUME_TEST_BREAK_STATE:-0}" == "1" ]]; then
  rm -rf "${HEPTA_RESUME_TEST_STATE_DIR:?}"
fi
echo "[hepta-preflight] fixture deterministic gate"
[[ "$PREFLIGHT_RELEASE_TARGET_DIR" == "${HEPTA_RESUME_TEST_EXPECTED_TARGET:?}" ]] || exit 44
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
[[ ! -e "$state" ]]
grep -q '^\[fixture\] release_target=' "$log"
grep -q '^\[hepta-preflight\] fixture deterministic gate$' "$log"
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
    "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 || rc=$?
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

run_failure

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

# Codex injects a thread correlation id and a random per-process arg0 alias
# directory. Neither selects preflight inputs, so an exact relaunch from a new
# thread remains resumable. All non-arg0 PATH entries stay fail-closed.
session_state="$tmp/session.state"
session_log="$tmp/session.log"
session_home="$tmp/session-home"
session_hepta_home="$tmp/session-hepta-home"
session_arg0_one="$session_home/tmp/arg0/codex-arg0-first"
session_arg0_two="$session_home/tmp/arg0/codex-arg0-second"
semantic_path_entry="$tmp/semantic-path-entry"
mkdir -p \
  "$session_arg0_one" \
  "$session_arg0_two" \
  "$session_hepta_home" \
  "$semantic_path_entry"
session_env=(
  HEPTA_PREFLIGHT_RESUME_STATE="$session_state"
  HEPTA_PREFLIGHT_RESUME_LOG="$session_log"
  HEPTA_PREFLIGHT_NATIVE=0
  HEPTA_PREFLIGHT_RELEASE=1
  HEPTA_RESUME_TEST_FAIL=1
  CARGO_TARGET_DIR="$target"
  HEPTA_RESUME_TEST_EXPECTED_TARGET="$target"
  HEPTA_HOME="$session_hepta_home"
  CODEX_HOME="$session_home"
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
[[ ! -e "$session_state" ]]

# The state is bound to this worktree, not merely to a shared HEAD and clean
# porcelain digest.
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
[[ ! -e "$state" ]]
grep -q '^Hepta preflight passed$' "$log"

# A same-marker interrupted state retains the prior streak instead of silently
# resetting the retry fuse.
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

# Resolved tool identity is part of the environment binding even when PATH is
# unchanged and only the executable output changes.
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

# A live receipt is non-resumable even during the short acquisition window
# before the owner has written its first running state.
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

# A live owner serializes concurrent runners and publishes an inspectable
# receipt. The second runner must not execute any fixture gate.
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
concurrent_second_rc=0
env "${concurrent_env[@]}" "$fixture/scripts/hepta-preflight-resume" >/dev/null 2>&1 \
  || concurrent_second_rc=$?
[[ "$concurrent_second_rc" == "75" ]]
wait "$concurrent_pid"
[[ ! -e "$concurrent_state.lock" ]]

# A same-host owner receipt with a conclusively dead PID is recoverable. No
# unknown or live owner path is used by this recovery fixture.
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

# Every contender must honor the guard during the quarantine -> replacement
# window. A PATH fixture pauses the stale-owner recovery immediately after its
# atomic rename so a second runner can probe that exact race.
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

# An orphaned reclaim guard is availability-blocking but fail-closed. A losing
# contender must not delete a guard that it did not create.
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

# The blocked state must survive even when the cumulative log path becomes
# unwritable after the pipeline has opened it.
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

# Terminal checkpoints always regenerate the complete native/release branch,
# keeping both if/else blocks syntactically balanced for every profile.
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
  [[ ! -e "$case_state" ]]
  [[ "$(grep -c '^\[hepta-preflight\] fixture deterministic gate$' "$case_log")" == "1" ]]
  [[ "$(grep -c '^\[fixture\] release_target=' "$case_log")" == "2" ]]
  grep -Fqx "[hepta-preflight] $marker" "$case_log"
}

run_terminal_resume_case native-on 1 1 "fixture native gate"
run_terminal_resume_case native-skip 0 1 "native app gates skipped (HEPTA_PREFLIGHT_NATIVE=0)"
run_terminal_resume_case release-on 1 1 "fixture release gate"
run_terminal_resume_case release-skip 0 0 "release build skipped (set HEPTA_PREFLIGHT_RELEASE=1)"
run_terminal_resume_case terminal-tail 1 1 "fixture whitespace/status"

# A terminal suffix is structurally replayed from the opening native `if`.
# Earlier replay markers must not replace the original failed checkpoint or
# clear its streak before the target is reached; the next target failure must
# therefore arm the third-attempt fuse.
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
