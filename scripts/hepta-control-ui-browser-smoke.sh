#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

source scripts/lib/hepta-ui-rust-toolchain.sh
# shellcheck source=scripts/lib/hepta-process-identity-v1.sh
source scripts/lib/hepta-process-identity-v1.sh
# shellcheck source=scripts/lib/hepta-safe-output-v1.sh
source scripts/lib/hepta-safe-output-v1.sh

export HEPTA_AUTOLOAD=0
export HEPTA_AUTOSAVE=0
export CARGO_INCREMENTAL=0
HEPTA_CHECK_JSONSCHEMA_BIN="$(scripts/hepta-control-ui-schema-validator-v1 --bootstrap)"
export HEPTA_CHECK_JSONSCHEMA_BIN

MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
EXPLICIT_BIND_ADDR=false
if [[ -n "$BIND_ADDR" ]]; then
  EXPLICIT_BIND_ADDR=true
fi
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-control-ui-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"
REPORT_PATH="${HEPTA_CONTROL_UI_BROWSER_SMOKE_REPORT_PATH:-${HEPTA_BROWSER_SMOKE_REPORT_PATH:-}}"
if [[ -z "$REPORT_PATH" && -n "${HEPTA_BROWSER_SMOKE_DIR:-}" ]]; then
  REPORT_PATH="${HEPTA_BROWSER_SMOKE_DIR%/}/control-ui-browser-smoke.json"
fi

CONTROL_SMOKE_ROOT="$(pwd -P)"
hepta_safe_output_resolve_file "$CONTROL_SMOKE_ROOT" "HEPTA_CONTROL_UI_SERVER_LOG" "$SERVER_LOG" || exit $?
SERVER_LOG="$HEPTA_SAFE_OUTPUT_PATH"
if [[ -n "$REPORT_PATH" ]]; then
  hepta_safe_output_resolve_file "$CONTROL_SMOKE_ROOT" "browser smoke report" "$REPORT_PATH" || exit $?
  REPORT_PATH="$HEPTA_SAFE_OUTPUT_PATH"
  if [[ "$REPORT_PATH" == "$SERVER_LOG" ]]; then
    hepta_safe_output_error "browser report and server log must be different files"
    exit 64
  fi
fi
if [[ -n "${HEPTA_BROWSER_SMOKE_DIR:-}" ]]; then
  hepta_safe_output_resolve_directory "$CONTROL_SMOKE_ROOT" "HEPTA_BROWSER_SMOKE_DIR" "$HEPTA_BROWSER_SMOKE_DIR" || exit $?
  CONTROL_BROWSER_OUTPUT_DIR="$HEPTA_SAFE_OUTPUT_PATH"
  for reserved_output in desktop.png narrow.png mobile.png phone320.png hepta-agent-logo.png; do
    if [[ "$SERVER_LOG" == "$CONTROL_BROWSER_OUTPUT_DIR/$reserved_output" ]]; then
      hepta_safe_output_error "server log collides with browser producer output: $SERVER_LOG"
      exit 64
    fi
  done
fi
hepta_safe_output_prepare_parent "$SERVER_LOG" || {
  hepta_safe_output_error "could not prepare canonical server-log parent"
  exit 64
}
if [[ -n "$REPORT_PATH" ]]; then
  hepta_safe_output_prepare_parent "$REPORT_PATH" || {
    hepta_safe_output_error "could not prepare canonical browser-report parent"
    exit 64
  }
fi

BASE_URL=""
SERVER_BINARY=""
AUTO_PORTS_TRIED=""
server_pid=""
server_spawn_start_token=""
server_start_token=""
server_command=""
server_termination_confirmed=false
attempt_log_dir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-control-ui-browser-smoke.XXXXXX")"
runtime_fixture_dir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-control-ui-runtime.XXXXXX")"
attempt_log_dir="$(cd "$attempt_log_dir" && pwd -P)"
runtime_fixture_dir="$(cd "$runtime_fixture_dir" && pwd -P)"
runtime_database="$runtime_fixture_dir/outcomes.sqlite3"
runtime_state_database="$runtime_fixture_dir/runtime-state.json"
runtime_key_file="$runtime_fixture_dir/integrity.key"
preference_database="$runtime_fixture_dir/preferences.sqlite3"
preference_integrity_key_file="$runtime_fixture_dir/preference-integrity.key"
preference_auth_key_file="$runtime_fixture_dir/preference-auth.key"
chmod 700 "$runtime_fixture_dir"
(umask 077; printf '%s' '000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f' >"$runtime_key_file")
(umask 077; printf '%s' '202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f' >"$preference_integrity_key_file")
(umask 077; printf '%s' '404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f' >"$preference_auth_key_file")
chmod 600 "$runtime_key_file"
chmod 600 "$preference_integrity_key_file" "$preference_auth_key_file"

producer_files=("$SERVER_LOG")
if [[ -n "$REPORT_PATH" ]]; then producer_files+=("$REPORT_PATH"); fi
for producer_file in "${producer_files[@]}"; do
  if hepta_safe_output_path_within "$producer_file" "$attempt_log_dir" \
    || hepta_safe_output_path_within "$producer_file" "$runtime_fixture_dir"; then
    hepta_safe_output_error "browser output overlaps ephemeral producer fixtures: $producer_file"
    exit 64
  fi
done

prepare_server_binary() {
  # Finish the potentially long compile before choosing an unreserved port.
  local build_messages="$attempt_log_dir/cargo-build.jsonl"
  if ! hepta_ui_cargo build --manifest-path "$MANIFEST" -q -p hepta-cli --bin hepta \
    --message-format=json-render-diagnostics >"$build_messages"; then
    jq -r 'select(.reason == "compiler-message") | .message.rendered // empty' "$build_messages" >&2 || true
    return 1
  fi

  SERVER_BINARY="$(
    jq -r '
      select(
        .reason == "compiler-artifact"
        and .target.name == "hepta"
        and (.target.kind | index("bin"))
        and .executable != null
      )
      | .executable
    ' "$build_messages" | tail -n 1
  )"
  if [[ -z "$SERVER_BINARY" || ! -x "$SERVER_BINARY" ]]; then
    echo "Hepta Control UI server build did not produce an executable" >&2
    return 1
  fi
}

select_auto_bind_addr() {
  local port
  for port in 7374 7375 7376 7377 7378; do
    case " $AUTO_PORTS_TRIED " in
      *" $port "*) continue ;;
    esac
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      BASE_URL="http://${BIND_ADDR}"
      AUTO_PORTS_TRIED="${AUTO_PORTS_TRIED}${AUTO_PORTS_TRIED:+ }${port}"
      return 0
    fi
  done

  echo "no untried free local port found for Hepta Control UI browser smoke" >&2
  return 75
}

start_server() {
  local outcome_mode="bootstrap-new"
  if [[ "$EXPLICIT_BIND_ADDR" == "true" ]]; then
    BASE_URL="http://${BIND_ADDR}"
  elif ! select_auto_bind_addr; then
    return 75
  fi
  if [[ -e "$runtime_database" ]]; then
    outcome_mode="open-existing"
  fi
  local preference_mode="bootstrap-new"
  if [[ -e "$preference_database" ]]; then
    preference_mode="open-existing"
  fi
  local server_log_temp=""
  hepta_safe_output_make_temp "$SERVER_LOG" ".hepta-control-server-log" || return $?
  server_log_temp="$HEPTA_SAFE_OUTPUT_TEMP"
  if ! exec 9>"$server_log_temp"; then
    rm -f "$server_log_temp"
    return 1
  fi
  if ! hepta_safe_output_install_temp "$server_log_temp" "$SERVER_LOG"; then
    exec 9>&-
    rm -f "$server_log_temp"
    return 1
  fi
  HEPTA_RUNTIME_OUTCOME_DATABASE="$runtime_database" \
    HEPTA_RUNTIME_STATE_DATABASE="$runtime_state_database" \
    HEPTA_RUNTIME_INTEGRITY_KEY_FILE="$runtime_key_file" \
    HEPTA_RUNTIME_OUTCOME_MODE="$outcome_mode" \
    HEPTA_PREFERENCE_DATABASE="$preference_database" \
    HEPTA_PREFERENCE_INTEGRITY_KEY_FILE="$preference_integrity_key_file" \
    HEPTA_PREFERENCE_INGRESS_AUTH_KEY_FILE="$preference_auth_key_file" \
    HEPTA_PREFERENCE_STORE_MODE="$preference_mode" \
    "$SERVER_BINARY" --serve-ui "$BIND_ADDR" \
    >&9 2>&1 &
  server_pid="$!"
  exec 9>&-
  server_command="$SERVER_BINARY --serve-ui $BIND_ADDR"
  server_spawn_start_token=""
  server_start_token=""
  server_termination_confirmed=false
  local identity_attempt=0
  while (( identity_attempt < 40 )); do
    if ! hepta_process_is_alive "$server_pid"; then
      return 0
    fi
    if hepta_process_read_identity "$server_pid"; then
      if [[ -z "$server_spawn_start_token" ]]; then
        server_spawn_start_token="$HEPTA_PROCESS_ACTUAL_START_TOKEN"
      fi
      if [[ "$HEPTA_PROCESS_ACTUAL_START_TOKEN" == "$server_spawn_start_token" \
        && "$HEPTA_PROCESS_ACTUAL_COMMAND" == "$server_command" ]]; then
        server_start_token="$HEPTA_PROCESS_ACTUAL_START_TOKEN"
        return 0
      fi
    fi
    sleep 0.05
    identity_attempt=$((identity_attempt + 1))
  done
  echo "Hepta Control UI server PID could not be bound to lstart and command" >&2
  stop_server || true
  return 1
}

stop_server() {
  local pid="${server_pid:-}"
  local cleanup_rc=0
  server_termination_confirmed=false
  if [[ -z "$pid" ]]; then
    server_termination_confirmed=true
    return 0
  fi
  if ! hepta_process_is_alive "$pid"; then
    wait "$pid" 2>/dev/null || true
    server_pid=""
    server_termination_confirmed=true
    return 0
  fi
  if [[ -n "$server_start_token" && -n "$server_command" ]]; then
    hepta_process_terminate_identity_safe \
      "$pid" "$server_start_token" "$server_command" 20 0.2 10 || cleanup_rc=$?
  elif [[ -n "$server_spawn_start_token" ]]; then
    hepta_process_terminate_start_safe \
      "$pid" "$server_spawn_start_token" 20 0.2 10 || cleanup_rc=$?
  else
    cleanup_rc=74
  fi
  if [[ "$cleanup_rc" == "0" && "$HEPTA_PROCESS_STOP_CONFIRMED" == true ]]; then
    wait "$pid" 2>/dev/null || true
    server_pid=""
    server_spawn_start_token=""
    server_start_token=""
    server_command=""
    server_termination_confirmed=true
    return 0
  fi
  echo "refusing unsafe Control UI server cleanup for PID $pid (rc=$cleanup_rc)" >&2
  return "${cleanup_rc:-1}"
}

cleanup() {
  if stop_server; then
    rm -rf "$attempt_log_dir"
    rm -rf "$runtime_fixture_dir"
  else
    echo "retaining Control UI smoke fixtures because server termination was not confirmed" >&2
  fi
}
trap cleanup EXIT

server_bind_failed() {
  grep -Eiq 'failed to bind|Address already in use|EADDRINUSE|os error (48|98)' "$SERVER_LOG"
}

wait_for_server() {
  local deadline=$((SECONDS + STARTUP_TIMEOUT_SEC))
  local root_probe=""
  local listening_message="Hepta native gateway listening on ${BASE_URL}/"
  until kill -0 "$server_pid" 2>/dev/null \
    && grep -Fq "$listening_message" "$SERVER_LOG" \
    && root_probe="$(curl -fsS "$BASE_URL/" 2>/dev/null)" \
    && [[ "$root_probe" == *'data-rust-rendered-control-ui="true"'* ]]; do
    if ! kill -0 "$server_pid" 2>/dev/null; then
      wait "$server_pid" 2>/dev/null || true
      server_pid=""
      if [[ "$EXPLICIT_BIND_ADDR" != "true" ]] && server_bind_failed; then
        echo "Hepta Control UI server could not bind $BIND_ADDR; retrying with another local port." >&2
        tail -n 80 "$SERVER_LOG" >&2 || true
        return 75
      fi
      echo "Hepta Control UI server exited before browser smoke was ready" >&2
      tail -n 80 "$SERVER_LOG" >&2 || true
      return 1
    fi
    if [[ "$SECONDS" -ge "$deadline" ]]; then
      echo "timed out waiting for Hepta Control UI server at $BASE_URL" >&2
      tail -n 80 "$SERVER_LOG" >&2 || true
      return 1
    fi
    sleep 1
  done
}

retryable_browser_smoke_failure() {
  local stderr_log="$1"
  grep -Eiq 'Failed to connect|Connection refused|Couldn.t connect to server|Broken pipe|write body|Connection reset by peer|screenshot .* suspiciously small|captured an error page' "$stderr_log" "$SERVER_LOG"
}

run_browser_smoke_once() {
  local attempt="$1"
  local stdout_log="$attempt_log_dir/browser-smoke-${attempt}.stdout.log"
  local stderr_log="$attempt_log_dir/browser-smoke-${attempt}.stderr.log"
  if HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_BROWSER_SMOKE_REPORT_PATH="$REPORT_PATH" \
    ./scripts/hepta-browser-visual-smoke.sh >"$stdout_log" 2>"$stderr_log"; then
    if [[ -n "$REPORT_PATH" && ! -s "$REPORT_PATH" ]]; then
      echo "Hepta Control UI browser smoke did not write report: $REPORT_PATH" >&2
      cat "$stdout_log" >&2 || true
      cat "$stderr_log" >&2 || true
      return 1
    fi
    local subresource_error_count
    subresource_error_count="$(grep -Ec 'RuntimeKernel request preflight rejected GET /(assets/[^ ]+|favicon\.ico)' "$SERVER_LOG" || true)"
    if [[ "$subresource_error_count" -ne 0 ]]; then
      echo "Hepta Control UI browser smoke observed ${subresource_error_count} rejected subresource requests" >&2
      grep -E 'RuntimeKernel request preflight rejected GET /(assets/[^ ]+|favicon\.ico)' "$SERVER_LOG" >&2 || true
      return 1
    fi
    if [[ -n "$REPORT_PATH" ]]; then
      local augmented_report_json
      augmented_report_json="$(jq -c '. + {subresource_error_count:0,subresource_requests_clean:true}' "$REPORT_PATH")"
      hepta_safe_output_atomic_write_text "$REPORT_PATH" "$augmented_report_json" || {
        echo "could not atomically augment Control UI browser report" >&2
        return 1
      }
    fi
    cat "$stdout_log"
    if [[ -s "$stderr_log" ]]; then
      cat "$stderr_log" >&2
    fi
    return 0
  fi

  if retryable_browser_smoke_failure "$stderr_log"; then
    echo "Hepta Control UI browser smoke hit a retryable server/capture race on attempt ${attempt}; restarting local server." >&2
    tail -n 40 "$stderr_log" >&2 || true
    tail -n 40 "$SERVER_LOG" >&2 || true
    return 75
  fi

  cat "$stdout_log" >&2 || true
  cat "$stderr_log" >&2 || true
  tail -n 80 "$SERVER_LOG" >&2 || true
  return 1
}

prepare_server_binary

for attempt in 1 2 3; do
  status=0
  if start_server; then
    if wait_for_server; then
      if run_browser_smoke_once "$attempt"; then
        if ! stop_server; then
          echo "Hepta Control UI browser smoke passed but server cleanup was not identity-safe" >&2
          exit 1
        fi
        echo "Hepta Control UI browser screenshot gate passed"
        exit 0
      else
        status="$?"
      fi
    else
      status="$?"
    fi
  else
    status="$?"
  fi
  if ! stop_server; then
    exit 1
  fi
  if [[ "$status" != "75" || "$attempt" == "3" ]]; then
    exit "$status"
  fi
done
