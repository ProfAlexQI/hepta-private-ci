#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
OUT_DIR="${HEPTA_BROWSER_SMOKE_DIR:-$(mktemp -d "${TMPDIR:-/tmp}/hepta-codex-browser-visual-smoke.XXXXXX")}"

if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 2
fi

mkdir -p "$OUT_DIR"

root_html="$(curl -fsS "$BASE_URL/")"
merge_json="$(curl -fsS "$BASE_URL/api/hepta-merge-completion")"

for needle in "Merge completion" "100 / 100 / 100 / 100" "/api/hepta-merge-completion"; do
  if [[ "$root_html" != *"$needle"* ]]; then
    echo "gateway index is missing expected text: $needle" >&2
    exit 1
  fi
done

if [[ "$(jq -r '.runtime' <<<"$merge_json")" != "hepta-codex" ]]; then
  echo "merge completion endpoint runtime mismatch" >&2
  exit 1
fi
if [[ "$(jq -r '.route_matrix_ready' <<<"$merge_json")" != "true" ]]; then
  echo "merge completion endpoint route matrix is not ready" >&2
  exit 1
fi
merge_status="$(jq -r '.status' <<<"$merge_json")"
merge_blockers="$(jq -r '.blockers | length' <<<"$merge_json")"
telegram_live_send_enabled="$(jq -r '.telegram_live_send_enabled' <<<"$merge_json")"
native_post_real_activation_enabled="$(jq -r '.native_post_real_activation_enabled' <<<"$merge_json")"
if [[ "$telegram_live_send_enabled" == "true" || "$native_post_real_activation_enabled" == "true" ]]; then
  if [[ "$merge_status" != "ready" || "$merge_blockers" != "0" ]]; then
    echo "merge completion enables production gates without ready status and zero blockers" >&2
    exit 1
  fi
elif [[ "$telegram_live_send_enabled" != "false" || "$native_post_real_activation_enabled" != "false" ]]; then
  echo "merge completion endpoint returned invalid production gate booleans" >&2
  exit 1
fi

capture_viewport() {
  local name="$1"
  local viewport="$2"
  local width="${viewport%x*}"
  local height="${viewport#*x}"
  local screenshot="$OUT_DIR/${name}.png"
  local profile="$OUT_DIR/profile-${name}"
  local stderr_log="$OUT_DIR/${name}.stderr.log"
  local stdout_log="$OUT_DIR/${name}.stdout.log"

  "$CHROME_BIN" \
    --headless=new \
    --disable-gpu \
    --disable-background-networking \
    --disable-component-update \
    --disable-default-apps \
    --disable-extensions \
    --disable-sync \
    --no-first-run \
    --no-default-browser-check \
    --user-data-dir="$profile" \
    --window-size="$viewport" \
    --screenshot="$screenshot" \
    "$BASE_URL/" >"$stdout_log" 2>"$stderr_log" &

  local chrome_pid="$!"
  local deadline=$((SECONDS + 45))
  while [[ ! -s "$screenshot" && "$SECONDS" -lt "$deadline" ]]; do
    if ! kill -0 "$chrome_pid" 2>/dev/null; then
      break
    fi
    sleep 1
  done

  if kill -0 "$chrome_pid" 2>/dev/null; then
    kill "$chrome_pid" 2>/dev/null || true
    wait "$chrome_pid" 2>/dev/null || true
  else
    wait "$chrome_pid" 2>/dev/null || true
  fi

  if [[ ! -s "$screenshot" ]]; then
    echo "screenshot was not created for viewport $name" >&2
    tail -n 40 "$stderr_log" >&2 || true
    exit 1
  fi

  local dimensions
  dimensions="$(
    sips -g pixelWidth -g pixelHeight "$screenshot" 2>/dev/null |
      awk '/pixelWidth/ { w=$2 } /pixelHeight/ { h=$2 } END { print w "x" h }'
  )"
  if [[ "$dimensions" != "${width}x${height}" ]]; then
    echo "unexpected screenshot dimensions for $name: $dimensions expected ${width}x${height}" >&2
    exit 1
  fi

  local bytes
  bytes="$(wc -c <"$screenshot" | tr -d ' ')"
  if [[ "$bytes" -lt 10000 ]]; then
    echo "screenshot for $name is suspiciously small: ${bytes} bytes" >&2
    exit 1
  fi
}

capture_viewport "desktop" "1365x900"
capture_viewport "mobile" "390x844"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta-codex" \
  --arg base_url "$BASE_URL" \
  --arg output_dir "$OUT_DIR" \
  --argjson telegram_live_send_enabled "$telegram_live_send_enabled" \
  --argjson native_post_real_activation_enabled "$native_post_real_activation_enabled" \
  --arg desktop_sha "$(shasum -a 256 "$OUT_DIR/desktop.png" | awk '{print $1}')" \
  --arg mobile_sha "$(shasum -a 256 "$OUT_DIR/mobile.png" | awk '{print $1}')" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    output_dir:$output_dir,
    browser:"chrome-headless",
    checked_text:["Merge completion","100 / 100 / 100 / 100","/api/hepta-merge-completion"],
    telegram_live_send_enabled:$telegram_live_send_enabled,
    native_post_real_activation_enabled:$native_post_real_activation_enabled,
    screenshots:[
      {name:"desktop", viewport:"1365x900", sha256:$desktop_sha},
      {name:"mobile", viewport:"390x844", sha256:$mobile_sha}
    ],
    side_effects:{
      telegram_read:false,
      telegram_send:false,
      native_post_real_mutation:false,
      provider_invoked:false
    }
  }'

echo "Hepta Codex browser visual smoke passed"
