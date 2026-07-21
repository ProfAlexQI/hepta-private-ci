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
if [[ "${HEPTA_RESUME_TEST_FAIL:-0}" == "1" ]]; then
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

if [[ "$RUN_RELEASE" == "1" ]]; then
  echo "[hepta-preflight] fixture release gate"
  true
else
  echo "[hepta-preflight] release build skipped (set HEPTA_PREFLIGHT_RELEASE=1)"
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
  CARGO_TARGET_DIR="$target"
  HEPTA_RESUME_TEST_EXPECTED_TARGET="$target"
)

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
grep -q 'checkpoint=1' "$checkpoint_output"

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
  and (.evidence.log_sha256 | test("^[0-9a-f]{64}$"))
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

echo "Hepta preflight resume self-test passed"
