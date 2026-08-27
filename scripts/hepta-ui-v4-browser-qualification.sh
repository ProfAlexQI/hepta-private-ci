#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

OUT_DIR="${HEPTA_UI_V4_BROWSER_OUT_DIR:-artifacts/ui-v4/browser}"
HOST="127.0.0.1"
PORT="${HEPTA_UI_V4_BROWSER_PORT:-}"
SERVER_LOG="${HEPTA_UI_V4_BROWSER_SERVER_LOG:-$OUT_DIR/static-server.log}"

mkdir -p "$OUT_DIR"

ruby scripts/hepta-ui-v4-runtime-js-sync --check
ruby scripts/hepta-ui-v4-runtime-css-sync --check
ruby scripts/hepta-ui-v4-style-lint >"$OUT_DIR/source-style-lint.json"

resolve_chrome() {
  if [[ -n "${HEPTA_CHROME_BIN:-}" && -x "${HEPTA_CHROME_BIN}" ]]; then
    printf '%s\n' "$HEPTA_CHROME_BIN"
    return 0
  fi
  local candidate
  for candidate in \
    "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
    "/Applications/Chromium.app/Contents/MacOS/Chromium" \
    "$(command -v google-chrome-stable 2>/dev/null || true)" \
    "$(command -v google-chrome 2>/dev/null || true)" \
    "$(command -v chromium 2>/dev/null || true)" \
    "$(command -v chromium-browser 2>/dev/null || true)"; do
    if [[ -n "$candidate" && -x "$candidate" ]]; then
      printf '%s\n' "$candidate"
      return 0
    fi
  done
  return 1
}

CHROME_BIN="$(resolve_chrome)" || {
  echo "No supported Chrome/Chromium executable was found" >&2
  exit 2
}

if [[ -z "$PORT" ]]; then
  PORT="$(python3 - <<'PY'
import socket
with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
    sock.bind(("127.0.0.1", 0))
    print(sock.getsockname()[1])
PY
)"
fi

BASE_URL="http://${HOST}:${PORT}/"
python3 -m http.server "$PORT" \
  --bind "$HOST" \
  --directory apps/hepta-control-ui \
  >"$SERVER_LOG" 2>&1 &
server_pid="$!"

cleanup() {
  if kill -0 "$server_pid" 2>/dev/null; then
    kill "$server_pid" 2>/dev/null || true
    wait "$server_pid" 2>/dev/null || true
  fi
}
trap cleanup EXIT INT TERM

ready=false
for _ in $(seq 1 80); do
  if curl -fsS "$BASE_URL" >/dev/null 2>&1; then
    ready=true
    break
  fi
  if ! kill -0 "$server_pid" 2>/dev/null; then
    echo "Static Control UI server exited before readiness" >&2
    tail -n 80 "$SERVER_LOG" >&2 || true
    exit 1
  fi
  sleep 0.1
done

if [[ "$ready" != true ]]; then
  echo "Timed out waiting for source-static Control UI server" >&2
  tail -n 80 "$SERVER_LOG" >&2 || true
  exit 1
fi

node scripts/hepta-ui-v4-browser-matrix.cjs \
  "$CHROME_BIN" \
  "$BASE_URL" \
  "$OUT_DIR" \
  | tee "$OUT_DIR/hepta-ui-v4-browser-qualification.compact.json"

echo "Hepta UI v4 source-static browser qualification passed"
