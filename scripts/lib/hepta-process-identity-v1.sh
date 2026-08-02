# Shared Bash 3.2-compatible helpers for terminating a process without ever
# signalling a PID that has been recycled for a different process.

hepta_process_reset_termination_result() {
  HEPTA_PROCESS_ACTUAL_START_TOKEN=""
  HEPTA_PROCESS_ACTUAL_COMMAND=""
  HEPTA_PROCESS_TERM_IDENTITY_VERIFIED=false
  HEPTA_PROCESS_KILL_IDENTITY_VERIFIED=false
  HEPTA_PROCESS_TERM_SENT=false
  HEPTA_PROCESS_KILL_SENT=false
  HEPTA_PROCESS_PID_REUSED=false
  HEPTA_PROCESS_STOP_CONFIRMED=false
}

hepta_process_is_alive() {
  local pid="$1"
  local kill_bin="${HEPTA_PROCESS_KILL_BIN:-/bin/kill}"
  "$kill_bin" -0 "$pid" >/dev/null 2>&1
}

hepta_process_read_identity() {
  local pid="$1"
  local ps_bin="${HEPTA_PROCESS_PS_BIN:-/bin/ps}"
  local start_token=""
  local command=""

  [[ "$pid" =~ ^[0-9]+$ ]] || return 64
  start_token="$("$ps_bin" -p "$pid" -o lstart= 2>/dev/null | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')"
  command="$("$ps_bin" -p "$pid" -o command= 2>/dev/null | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')"
  [[ -n "$start_token" && -n "$command" ]] || return 1
  HEPTA_PROCESS_ACTUAL_START_TOKEN="$start_token"
  HEPTA_PROCESS_ACTUAL_COMMAND="$command"
}

hepta_process_identity_matches() {
  local pid="$1"
  local expected_start_token="$2"
  local expected_command="$3"
  [[ -n "$expected_start_token" && -n "$expected_command" ]] || return 64
  hepta_process_read_identity "$pid" || return 1
  [[ "$HEPTA_PROCESS_ACTUAL_START_TOKEN" == "$expected_start_token" \
    && "$HEPTA_PROCESS_ACTUAL_COMMAND" == "$expected_command" ]]
}

# Return 0 only when the original process is confirmed stopped (or the PID is
# now owned by a different start token). Return 75 for a same-start command
# mismatch and 76 when a matching process survives KILL. No TERM/KILL is ever
# sent unless PID, lstart, and command were re-read and matched immediately
# beforehand.
hepta_process_terminate_identity_safe() {
  local pid="$1"
  local expected_start_token="$2"
  local expected_command="$3"
  local grace_checks="${4:-30}"
  local grace_delay="${5:-0.1}"
  local kill_checks="${6:-20}"
  local kill_bin="${HEPTA_PROCESS_KILL_BIN:-/bin/kill}"
  local sleep_bin="${HEPTA_PROCESS_SLEEP_BIN:-/bin/sleep}"
  local attempt

  hepta_process_reset_termination_result
  [[ "$pid" =~ ^[0-9]+$ && -n "$expected_start_token" && -n "$expected_command" ]] || return 64

  if ! hepta_process_is_alive "$pid"; then
    HEPTA_PROCESS_STOP_CONFIRMED=true
    return 0
  fi
  if ! hepta_process_read_identity "$pid"; then
    return 74
  fi
  if [[ "$HEPTA_PROCESS_ACTUAL_START_TOKEN" != "$expected_start_token" ]]; then
    HEPTA_PROCESS_PID_REUSED=true
    HEPTA_PROCESS_STOP_CONFIRMED=true
    return 0
  fi
  [[ "$HEPTA_PROCESS_ACTUAL_COMMAND" == "$expected_command" ]] || return 75

  HEPTA_PROCESS_TERM_IDENTITY_VERIFIED=true
  if ! "$kill_bin" -TERM "$pid" >/dev/null 2>&1; then
    if ! hepta_process_is_alive "$pid"; then
      HEPTA_PROCESS_STOP_CONFIRMED=true
      return 0
    fi
    return 74
  fi
  HEPTA_PROCESS_TERM_SENT=true

  attempt=0
  while (( attempt < grace_checks )); do
    if ! hepta_process_is_alive "$pid"; then
      HEPTA_PROCESS_STOP_CONFIRMED=true
      return 0
    fi
    if ! hepta_process_read_identity "$pid"; then
      return 74
    fi
    if [[ "$HEPTA_PROCESS_ACTUAL_START_TOKEN" != "$expected_start_token" ]]; then
      HEPTA_PROCESS_PID_REUSED=true
      HEPTA_PROCESS_STOP_CONFIRMED=true
      return 0
    fi
    [[ "$HEPTA_PROCESS_ACTUAL_COMMAND" == "$expected_command" ]] || return 75
    "$sleep_bin" "$grace_delay"
    attempt=$((attempt + 1))
  done

  if ! hepta_process_is_alive "$pid"; then
    HEPTA_PROCESS_STOP_CONFIRMED=true
    return 0
  fi
  if ! hepta_process_read_identity "$pid"; then
    return 74
  fi
  if [[ "$HEPTA_PROCESS_ACTUAL_START_TOKEN" != "$expected_start_token" ]]; then
    HEPTA_PROCESS_PID_REUSED=true
    HEPTA_PROCESS_STOP_CONFIRMED=true
    return 0
  fi
  [[ "$HEPTA_PROCESS_ACTUAL_COMMAND" == "$expected_command" ]] || return 75

  HEPTA_PROCESS_KILL_IDENTITY_VERIFIED=true
  if ! "$kill_bin" -KILL "$pid" >/dev/null 2>&1; then
    if ! hepta_process_is_alive "$pid"; then
      HEPTA_PROCESS_STOP_CONFIRMED=true
      return 0
    fi
    return 74
  fi
  HEPTA_PROCESS_KILL_SENT=true

  attempt=0
  while (( attempt < kill_checks )); do
    if ! hepta_process_is_alive "$pid"; then
      HEPTA_PROCESS_STOP_CONFIRMED=true
      return 0
    fi
    if hepta_process_read_identity "$pid" \
      && [[ "$HEPTA_PROCESS_ACTUAL_START_TOKEN" != "$expected_start_token" ]]; then
      HEPTA_PROCESS_PID_REUSED=true
      HEPTA_PROCESS_STOP_CONFIRMED=true
      return 0
    fi
    "$sleep_bin" "$grace_delay"
    attempt=$((attempt + 1))
  done
  return 76
}

# A just-spawned sandbox wrapper may exec into the product before its final
# command can be captured. The immutable lstart token remains the anchor; each
# TERM/KILL still uses an exact command snapshot through the helper above.
hepta_process_terminate_start_safe() {
  local pid="$1"
  local expected_start_token="$2"
  local grace_checks="${3:-30}"
  local grace_delay="${4:-0.1}"
  local kill_checks="${5:-20}"
  local attempt=0
  local status=75
  local observed_command=""

  [[ "$pid" =~ ^[0-9]+$ && -n "$expected_start_token" ]] || return 64
  while (( attempt < 4 )); do
    hepta_process_reset_termination_result
    if ! hepta_process_is_alive "$pid"; then
      HEPTA_PROCESS_STOP_CONFIRMED=true
      return 0
    fi
    if ! hepta_process_read_identity "$pid"; then
      return 74
    fi
    if [[ "$HEPTA_PROCESS_ACTUAL_START_TOKEN" != "$expected_start_token" ]]; then
      HEPTA_PROCESS_PID_REUSED=true
      HEPTA_PROCESS_STOP_CONFIRMED=true
      return 0
    fi
    observed_command="$HEPTA_PROCESS_ACTUAL_COMMAND"
    status=0
    hepta_process_terminate_identity_safe "$pid" "$expected_start_token" "$observed_command" \
      "$grace_checks" "$grace_delay" "$kill_checks" || status=$?
    [[ "$status" == "75" ]] || return "$status"
    attempt=$((attempt + 1))
  done
  return "$status"
}
