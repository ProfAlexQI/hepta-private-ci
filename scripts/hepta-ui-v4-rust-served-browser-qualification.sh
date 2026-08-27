#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

source scripts/lib/hepta-ui-rust-toolchain.sh
# shellcheck source=scripts/lib/hepta-process-identity-v1.sh
source scripts/lib/hepta-process-identity-v1.sh
# shellcheck source=scripts/lib/hepta-safe-output-v1.sh
source scripts/lib/hepta-safe-output-v1.sh

ROOT="$(pwd -P)"
OUT_DIR="${HEPTA_UI_V4_RUST_SERVED_BROWSER_OUT:-artifacts/ui-v4-rust-served-browser}"
STARTUP_TIMEOUT_SEC="${HEPTA_UI_V4_RUST_SERVED_STARTUP_TIMEOUT_SEC:-900}"
HOST="127.0.0.1"
PORTS="${HEPTA_UI_V4_RUST_SERVED_PORTS:-7380 7381 7382 7383 7384}"
MANIFEST="codex-rs/Cargo.toml"

hepta_safe_output_resolve_directory "$ROOT" "HEPTA_UI_V4_RUST_SERVED_BROWSER_OUT" "$OUT_DIR" || exit $?
OUT_DIR="$HEPTA_SAFE_OUTPUT_PATH"
mkdir -p "$OUT_DIR"

export HEPTA_AUTOLOAD=0
export HEPTA_AUTOSAVE=0
export CARGO_INCREMENTAL=0

work_dir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-v4-rust-served.XXXXXX")"
work_dir="$(cd "$work_dir" && pwd -P)"
runtime_dir="$work_dir/runtime"
mkdir -m 700 "$runtime_dir"
server_log="$work_dir/server.log"
build_messages="$work_dir/cargo-build.jsonl"

runtime_database="$runtime_dir/outcomes.sqlite3"
runtime_state_database="$runtime_dir/runtime-state.json"
runtime_key_file="$runtime_dir/integrity.key"
preference_database="$runtime_dir/preferences.sqlite3"
preference_integrity_key_file="$runtime_dir/preference-integrity.key"
preference_auth_key_file="$runtime_dir/preference-auth.key"

(
  umask 077
  printf '%s' '000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f' \
    >"$runtime_key_file"
  printf '%s' '202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f' \
    >"$preference_integrity_key_file"
  printf '%s' '404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f' \
    >"$preference_auth_key_file"
)
chmod 600 "$runtime_key_file" "$preference_integrity_key_file" "$preference_auth_key_file"

server_pid=""
server_start_token=""
server_command=""
server_stop_confirmed=false

port_available() {
  python3 - "$HOST" "$1" <<'PY'
import socket
import sys
host = sys.argv[1]
port = int(sys.argv[2])
with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
    sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    try:
        sock.bind((host, port))
    except OSError:
        raise SystemExit(1)
PY
}

select_bind_addr() {
  local port
  for port in $PORTS; do
    if port_available "$port"; then
      BIND_ADDR="${HOST}:${port}"
      BASE_URL="http://${BIND_ADDR}"
      export BIND_ADDR BASE_URL
      return 0
    fi
  done
  echo "no free bounded loopback port available for UI v4 qualification" >&2
  return 75
}

resolve_browser() {
  if [[ -n "${HEPTA_CHROME_BIN:-}" && -x "$HEPTA_CHROME_BIN" ]]; then
    return 0
  fi

  local candidate
  for candidate in google-chrome google-chrome-stable chromium chromium-browser; do
    if command -v "$candidate" >/dev/null 2>&1; then
      HEPTA_CHROME_BIN="$(command -v "$candidate")"
      export HEPTA_CHROME_BIN
      return 0
    fi
  done

  if [[ -n "${NODE_PATH:-}" ]]; then
    candidate="$(node -e 'try { process.stdout.write(require("playwright").chromium.executablePath()) } catch (_) {}')"
    if [[ -n "$candidate" && -x "$candidate" ]]; then
      HEPTA_CHROME_BIN="$candidate"
      export HEPTA_CHROME_BIN
      return 0
    fi
  fi

  candidate="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
  if [[ -x "$candidate" ]]; then
    HEPTA_CHROME_BIN="$candidate"
    export HEPTA_CHROME_BIN
    return 0
  fi

  echo "no executable Chromium/Chrome binary available" >&2
  return 2
}

stop_server() {
  local cleanup_rc=0
  server_stop_confirmed=false
  if [[ -z "$server_pid" ]]; then
    server_stop_confirmed=true
    return 0
  fi
  if ! hepta_process_is_alive "$server_pid"; then
    wait "$server_pid" 2>/dev/null || true
    server_pid=""
    server_stop_confirmed=true
    return 0
  fi
  if [[ -z "$server_start_token" || -z "$server_command" ]]; then
    echo "refusing to stop an unbound qualification server process" >&2
    return 74
  fi
  hepta_process_terminate_identity_safe \
    "$server_pid" "$server_start_token" "$server_command" 30 0.2 10 || cleanup_rc=$?
  if [[ "$cleanup_rc" == "0" && "$HEPTA_PROCESS_STOP_CONFIRMED" == true ]]; then
    wait "$server_pid" 2>/dev/null || true
    server_pid=""
    server_stop_confirmed=true
    return 0
  fi
  echo "identity-safe qualification server cleanup failed (rc=$cleanup_rc)" >&2
  return "${cleanup_rc:-1}"
}

cleanup() {
  if stop_server; then
    rm -rf "$work_dir"
  else
    echo "retaining qualification work directory because server termination was not confirmed: $work_dir" >&2
  fi
}
trap cleanup EXIT

select_bind_addr
resolve_browser

if ! hepta_ui_cargo build --manifest-path "$MANIFEST" -q -p hepta-cli --bin hepta \
  --message-format=json-render-diagnostics >"$build_messages"; then
  jq -r 'select(.reason == "compiler-message") | .message.rendered // empty' \
    "$build_messages" >&2 || true
  exit 1
fi

server_binary="$(
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
if [[ -z "$server_binary" || ! -x "$server_binary" ]]; then
  echo "hepta build did not produce an executable server binary" >&2
  exit 1
fi

HEPTA_RUNTIME_OUTCOME_DATABASE="$runtime_database" \
HEPTA_RUNTIME_STATE_DATABASE="$runtime_state_database" \
HEPTA_RUNTIME_INTEGRITY_KEY_FILE="$runtime_key_file" \
HEPTA_RUNTIME_OUTCOME_MODE="bootstrap-new" \
HEPTA_PREFERENCE_DATABASE="$preference_database" \
HEPTA_PREFERENCE_INTEGRITY_KEY_FILE="$preference_integrity_key_file" \
HEPTA_PREFERENCE_INGRESS_AUTH_KEY_FILE="$preference_auth_key_file" \
HEPTA_PREFERENCE_STORE_MODE="bootstrap-new" \
"$server_binary" --serve-ui "$BIND_ADDR" >"$server_log" 2>&1 &
server_pid="$!"
server_command="$server_binary --serve-ui $BIND_ADDR"

for _ in $(seq 1 80); do
  if ! hepta_process_is_alive "$server_pid"; then
    break
  fi
  if hepta_process_read_identity "$server_pid" \
    && [[ "$HEPTA_PROCESS_ACTUAL_COMMAND" == "$server_command" ]]; then
    server_start_token="$HEPTA_PROCESS_ACTUAL_START_TOKEN"
    break
  fi
  sleep 0.05
done
if [[ -z "$server_start_token" ]]; then
  echo "qualification server PID could not be bound to its start token and command" >&2
  tail -n 80 "$server_log" >&2 || true
  exit 1
fi

deadline=$((SECONDS + STARTUP_TIMEOUT_SEC))
until kill -0 "$server_pid" 2>/dev/null \
  && curl -fsS "$BASE_URL/" | grep -Fq 'data-rust-rendered-control-ui="true"'; do
  if ! kill -0 "$server_pid" 2>/dev/null; then
    echo "qualification server exited before readiness" >&2
    tail -n 120 "$server_log" >&2 || true
    exit 1
  fi
  if (( SECONDS >= deadline )); then
    echo "timed out waiting for qualification server at $BASE_URL" >&2
    tail -n 120 "$server_log" >&2 || true
    exit 1
  fi
  sleep 1
done

export HEPTA_LIVE_URL="$BASE_URL"
export HEPTA_UI_V4_RUST_SERVED_BROWSER_OUT="$OUT_DIR"
export HEPTA_CANDIDATE_COMMIT="$(git rev-parse HEAD)"
export HEPTA_CANDIDATE_TREE="$(git rev-parse 'HEAD^{tree}')"

node scripts/hepta-ui-v4-rust-served-browser-qualification.cjs "$BASE_URL" \
  | tee "$OUT_DIR/stdout-receipt.json"

receipt="$OUT_DIR/HEPTA_UI_V4_RUST_SERVED_BROWSER_QUALIFICATION_RECEIPT.json"
if [[ ! -s "$receipt" ]]; then
  echo "Rust-served browser qualification did not emit its receipt" >&2
  exit 1
fi

jq -e \
  --arg commit "$HEPTA_CANDIDATE_COMMIT" \
  --arg tree "$HEPTA_CANDIDATE_TREE" '
  .status == "PASS_RUST_SERVED_BROWSER_CONTRACT"
  and .scope == "RUST_SERVED_LOOPBACK_LOCAL_READ_ONLY"
  and .candidate.commit == $commit
  and .candidate.tree == $tree
  and .candidate.commitBound == true
  and .candidate.treeBound == true
  and .fixture == false
  and .runtimeAssetInjectedForQualification == false
  and .rustServedRuntimeAssetBound == true
  and .browserValidation == true
  and .rustRuntimeValidation == true
  and .deviceValidation == false
  and .productionAuthority == false
  and .effectAuthority == false
  and .liveAdapterAuthority == false
  and .operatorAcceptance == false
  and .promotion == false
  and .release == false
  and .server.servedAssets.script.exactBytesBound == true
  and .server.servedAssets.script.etagBound == true
  and .server.servedAssets.script.runtimeBound == true
  and (.results | length) == 10
  and ([.results[] | select(.status != "PASS_RUST_SERVED_BROWSER_CONTRACT")] | length) == 0
  and ([.results[] | select(.network.crossOrigin != 0 or .network.nonGet != 0)] | length) == 0
  and (.failures | length) == 0
' "$receipt" >/dev/null

if ! stop_server; then
  exit 1
fi

echo "Hepta UI v4 Rust-served browser qualification passed"
