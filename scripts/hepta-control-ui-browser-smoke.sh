#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

export HEPTA_AUTOLOAD=0
export HEPTA_AUTOSAVE=0
export CARGO_INCREMENTAL=0

MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-control-ui-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"
REPORT_PATH="${HEPTA_CONTROL_UI_BROWSER_SMOKE_REPORT_PATH:-${HEPTA_BROWSER_SMOKE_REPORT_PATH:-}}"
if [[ -z "$REPORT_PATH" && -n "${HEPTA_BROWSER_SMOKE_DIR:-}" ]]; then
  REPORT_PATH="${HEPTA_BROWSER_SMOKE_DIR%/}/control-ui-browser-smoke.json"
fi

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7374 7375 7376 7377 7378; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi

if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI browser smoke" >&2
  exit 1
fi

BASE_URL="http://${BIND_ADDR}"
server_pid=""
attempt_log_dir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-control-ui-browser-smoke.XXXXXX")"
runtime_fixture_dir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-control-ui-runtime.XXXXXX")"
runtime_database="$runtime_fixture_dir/outcomes.sqlite3"
runtime_key_file="$runtime_fixture_dir/integrity.key"
chmod 700 "$runtime_fixture_dir"
(umask 077; printf '%s' '000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f' >"$runtime_key_file")
chmod 600 "$runtime_key_file"

start_server() {
  local outcome_mode="bootstrap-new"
  if [[ -e "$runtime_database" ]]; then
    outcome_mode="open-existing"
  fi
  : >"$SERVER_LOG"
  HEPTA_RUNTIME_OUTCOME_DATABASE="$runtime_database" \
    HEPTA_RUNTIME_INTEGRITY_KEY_FILE="$runtime_key_file" \
    HEPTA_RUNTIME_OUTCOME_MODE="$outcome_mode" \
    cargo run --manifest-path "$MANIFEST" -q -p hepta-cli --bin hepta -- --serve-ui "$BIND_ADDR" \
    >"$SERVER_LOG" 2>&1 &
  server_pid="$!"
}

cleanup() {
  if [[ -n "${server_pid:-}" ]] && kill -0 "$server_pid" 2>/dev/null; then
    kill "$server_pid" 2>/dev/null || true
    for _ in {1..20}; do
      if ! kill -0 "$server_pid" 2>/dev/null; then
        wait "$server_pid" 2>/dev/null || true
        break
      fi
      sleep 0.2
    done
    kill -9 "$server_pid" 2>/dev/null || true
    wait "$server_pid" 2>/dev/null || true
  fi
  rm -rf "$attempt_log_dir"
  rm -rf "$runtime_fixture_dir"
}
trap cleanup EXIT

wait_for_server() {
  local deadline=$((SECONDS + STARTUP_TIMEOUT_SEC))
  local root_probe=""
  until root_probe="$(curl -fsS "$BASE_URL/" 2>/dev/null)" && [[ "$root_probe" == *'data-rust-rendered-control-ui="true"'* ]]; do
    if ! kill -0 "$server_pid" 2>/dev/null; then
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

start_server
wait_for_server

for attempt in 1 2 3; do
  if run_browser_smoke_once "$attempt"; then
    echo "Hepta Control UI browser screenshot gate passed"
    exit 0
  else
    status="$?"
  fi
  if [[ "$status" != "75" || "$attempt" == "3" ]]; then
    exit "$status"
  fi
  cleanup
  mkdir -p "$attempt_log_dir"
  start_server
  wait_for_server
done
