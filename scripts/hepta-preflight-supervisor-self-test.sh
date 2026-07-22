#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SUPERVISOR="$ROOT/scripts/hepta-preflight-supervisor"
tmp="$(mktemp -d "${TMPDIR:-/tmp}/hepta-preflight-supervisor-test.XXXXXX")"
trap 'rm -rf "$tmp"' EXIT

fake_resume="$tmp/fake-resume"
calls="$tmp/calls"
mode="$tmp/mode"
printf 'non_resumable\n' >"$mode"

cat >"$fake_resume" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

calls="${HEPTA_SUPERVISOR_TEST_CALLS:?}"
mode="$(cat "${HEPTA_SUPERVISOR_TEST_MODE:?}")"
printf '%s\n' "${1:-run}" >>"$calls"

if [[ "${1:-}" == "--show" ]]; then
  case "$mode" in
    non_resumable)
      jq -n '{status:"running",resumable:false,failure_streak:0,fuse_threshold:3,fuse_armed:false,environment_matches:false,relevant_environment_matches:false,worktree_state_matches:true,log_matches:true}'
      ;;
    fused)
      jq -n '{status:"blocked",resumable:false,failure_streak:3,fuse_threshold:3,fuse_armed:true,environment_matches:true,relevant_environment_matches:true,worktree_state_matches:true,log_matches:true}'
      ;;
    retry)
      attempt_count="$(grep -c '^run$' "$calls" 2>/dev/null || true)"
      if (( attempt_count >= 2 )); then
        jq -n '{status:"complete",resumable:false,failure_streak:0,fuse_threshold:3,fuse_armed:false,environment_matches:true,relevant_environment_matches:true,worktree_state_matches:true,log_matches:true}'
      else
        jq -n --argjson streak "$attempt_count" '{status:"blocked",resumable:true,failure_streak:$streak,fuse_threshold:3,fuse_armed:false,environment_matches:true,relevant_environment_matches:true,worktree_state_matches:true,log_matches:true}'
      fi
      ;;
  esac
  exit 0
fi

[[ "$mode" == "retry" ]] || exit 70
attempt_count="$(grep -c '^run$' "$calls" 2>/dev/null || true)"
if (( attempt_count >= 2 )); then
  exit 0
fi
exit 42
EOF
chmod +x "$fake_resume"

supervisor_env=(
  HEPTA_PREFLIGHT_SUPERVISOR_RESUME="$fake_resume"
  HEPTA_PREFLIGHT_SUPERVISOR_INITIAL_BACKOFF_SECONDS=0
  HEPTA_PREFLIGHT_SUPERVISOR_MAX_BACKOFF_SECONDS=0
  HEPTA_SUPERVISOR_TEST_CALLS="$calls"
  HEPTA_SUPERVISOR_TEST_MODE="$mode"
)

non_resumable_rc=0
env "${supervisor_env[@]}" "$SUPERVISOR" >/dev/null 2>&1 || non_resumable_rc=$?
[[ "$non_resumable_rc" == "22" ]]
[[ "$(grep -c '^--show$' "$calls")" == "1" ]]
[[ "$(grep -c '^run$' "$calls" 2>/dev/null || true)" == "0" ]]

: >"$calls"
printf 'fused\n' >"$mode"
fused_rc=0
env "${supervisor_env[@]}" "$SUPERVISOR" >/dev/null 2>&1 || fused_rc=$?
[[ "$fused_rc" == "21" ]]
[[ "$(grep -c '^--show$' "$calls")" == "1" ]]
[[ "$(grep -c '^run$' "$calls" 2>/dev/null || true)" == "0" ]]

: >"$calls"
printf 'retry\n' >"$mode"
env "${supervisor_env[@]}" "$SUPERVISOR" >/dev/null
[[ "$(grep -c '^run$' "$calls")" == "2" ]]

echo "hepta preflight supervisor self-test passed"
