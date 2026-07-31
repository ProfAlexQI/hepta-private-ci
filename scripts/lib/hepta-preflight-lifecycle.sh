#!/usr/bin/env bash

hepta_preflight_run_with_timeout() {
  local max_seconds="$1"
  shift

  perl -MPOSIX -e '
    use strict;
    use warnings;
    my $limit = shift @ARGV;
    my $pid = fork();
    die "fork failed: $!\n" unless defined $pid;
    if ($pid == 0) {
      setpgrp(0, 0);
      exec @ARGV;
      die "exec failed: $!\n";
    }
    local $SIG{ALRM} = sub {
      kill "TERM", -$pid;
      select undef, undef, undef, 0.25;
      kill "KILL", -$pid;
      waitpid($pid, 0);
      exit 124;
    };
    alarm $limit;
    waitpid($pid, 0);
    alarm 0;
    my $status = $?;
    exit(($status & 127) ? 128 + ($status & 127) : ($status >> 8));
  ' "$max_seconds" "$@"
}

hepta_preflight_terminal_status() {
  local exit_code="$1"
  if (( exit_code == 0 )); then
    printf 'passed\n'
  elif [[ -n "${PREFLIGHT_TERMINATING_SIGNAL:-}" ]]; then
    printf 'interrupted\n'
  elif (( exit_code == 124 )); then
    printf 'timed_out\n'
  else
    printf 'failed\n'
  fi
}

hepta_preflight_terminal_receipt_json() {
  local exit_code="$1"
  local ended_at elapsed_seconds gate_elapsed_seconds status
  ended_at="$(date +%s)"
  elapsed_seconds=$((ended_at - PREFLIGHT_STARTED_AT_EPOCH))
  gate_elapsed_seconds=0
  if (( PREFLIGHT_CURRENT_GATE_STARTED_AT_EPOCH > 0 )); then
    gate_elapsed_seconds=$((ended_at - PREFLIGHT_CURRENT_GATE_STARTED_AT_EPOCH))
  fi
  status="$(hepta_preflight_terminal_status "$exit_code")"

  jq -cSn \
    --arg schema hepta_preflight_terminal_receipt_v1 \
    --arg status "$status" \
    --argjson exit_code "$exit_code" \
    --arg signal "${PREFLIGHT_TERMINATING_SIGNAL:-}" \
    --arg source_commit "${PREFLIGHT_SOURCE_COMMIT:-unknown}" \
    --arg current_gate "${PREFLIGHT_CURRENT_GATE:-}" \
    --arg timed_out_gate "${PREFLIGHT_TIMED_OUT_GATE:-}" \
    --argjson timeout_seconds "${HEPTA_PREFLIGHT_GATE_MAX_SECONDS:-1800}" \
    --argjson completed_gate_count "${PREFLIGHT_COMPLETED_GATE_COUNT:-0}" \
    --argjson started_at_epoch "$PREFLIGHT_STARTED_AT_EPOCH" \
    --argjson ended_at_epoch "$ended_at" \
    --argjson elapsed_seconds "$elapsed_seconds" \
    --argjson current_gate_elapsed_seconds "$gate_elapsed_seconds" \
    --argjson final_pass_marker_emitted "${PREFLIGHT_FINAL_PASS_MARKER_EMITTED:-false}" \
    '{
      schema:$schema,
      status:$status,
      exit_code:$exit_code,
      signal:(if $signal == "" then null else $signal end),
      source_commit:$source_commit,
      completed_gate_count:$completed_gate_count,
      current_gate:(if $current_gate == "" then null else $current_gate end),
      timeout:{
        max_seconds:$timeout_seconds,
        timed_out_gate:(if $timed_out_gate == "" then null else $timed_out_gate end)
      },
      timing:{
        started_at_epoch:$started_at_epoch,
        ended_at_epoch:$ended_at_epoch,
        elapsed_seconds:$elapsed_seconds,
        current_gate_elapsed_seconds:$current_gate_elapsed_seconds
      },
      final_pass_marker_emitted:$final_pass_marker_emitted
    }'
}

hepta_preflight_emit_terminal_receipt() {
  local exit_code="$1"
  local receipt temporary receipt_path
  receipt="$(hepta_preflight_terminal_receipt_json "$exit_code")"
  printf '[hepta-preflight-terminal] %s\n' "$receipt"

  receipt_path="${HEPTA_PREFLIGHT_TERMINAL_RECEIPT:-}"
  [[ -n "$receipt_path" ]] || return 0
  mkdir -p "$(dirname "$receipt_path")"
  temporary="${receipt_path}.tmp.$$"
  printf '%s\n' "$receipt" >"$temporary"
  chmod 0444 "$temporary"
  mv -f "$temporary" "$receipt_path"
}

hepta_preflight_lifecycle_self_test() (
  set -euo pipefail
  local root receipt rc
  root="$(mktemp -d /tmp/hepta-preflight-lifecycle.XXXXXX)"
  trap 'rm -rf "$root"' EXIT

  PREFLIGHT_STARTED_AT_EPOCH="$(date +%s)"
  PREFLIGHT_CURRENT_GATE_STARTED_AT_EPOCH="$PREFLIGHT_STARTED_AT_EPOCH"
  PREFLIGHT_CURRENT_GATE="timeout-self-test"
  PREFLIGHT_TIMED_OUT_GATE="timeout-self-test"
  PREFLIGHT_COMPLETED_GATE_COUNT=3
  PREFLIGHT_SOURCE_COMMIT="$(printf 'a%.0s' {1..40})"
  PREFLIGHT_FINAL_PASS_MARKER_EMITTED=false
  HEPTA_PREFLIGHT_GATE_MAX_SECONDS=1
  HEPTA_PREFLIGHT_TERMINAL_RECEIPT="$root/terminal.json"

  rc=0
  hepta_preflight_run_with_timeout 1 sh -c 'sleep 3' || rc=$?
  [[ "$rc" -eq 124 ]]
  hepta_preflight_emit_terminal_receipt "$rc" >/dev/null
  receipt="$(cat "$HEPTA_PREFLIGHT_TERMINAL_RECEIPT")"
  jq -e '
    .schema == "hepta_preflight_terminal_receipt_v1"
    and .status == "timed_out"
    and .exit_code == 124
    and .completed_gate_count == 3
    and .current_gate == "timeout-self-test"
    and .timeout.max_seconds == 1
    and .timeout.timed_out_gate == "timeout-self-test"
    and .final_pass_marker_emitted == false
  ' <<<"$receipt" >/dev/null
  [[ ! -e "$HEPTA_PREFLIGHT_TERMINAL_RECEIPT.tmp.$$" ]]
  printf '%s\n' '{"schema":"hepta_preflight_lifecycle_self_test_v1","status":"ready","timeout_enforced":true,"terminal_receipt_atomic":true}'
)
