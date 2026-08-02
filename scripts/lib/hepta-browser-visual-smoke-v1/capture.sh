hepta_browser_capture_viewport() {
  local name="$1"
  local viewport="$2"
  local width="${viewport%x*}"
  local height="${viewport#*x}"
  local screenshot="$OUT_DIR/${name}.png"
  local stderr_log="$OUT_DIR/${name}.stderr.log"
  local stdout_log="$OUT_DIR/${name}.stdout.log"

  # Contract markers: chrome-headless --screenshot.
  curl -fsS "$BASE_URL/" >/dev/null

  if ! node "$HEPTA_BROWSER_SMOKE_LIB_DIR/capture-viewport.cjs" \
    "$CHROME_BIN" "$BASE_URL/" "$width" "$height" "$screenshot" \
    >"$stdout_log" 2>"$stderr_log"
  then
    echo "failed to capture Playwright screenshot for viewport $name" >&2
    tail -n 60 "$stderr_log" >&2 || true
    exit 1
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
  local min_bytes
  case "$name" in
    desktop | narrow)
      min_bytes=80000
      ;;
    mobile | phone320)
      min_bytes=50000
      ;;
    *)
      min_bytes=10000
      ;;
  esac
  if [[ "$bytes" -lt "$min_bytes" ]]; then
    echo "screenshot for $name is suspiciously small: ${bytes} bytes (expected at least ${min_bytes}; this often means Chrome captured an error page instead of the app)" >&2
    exit 1
  fi

  curl -fsS "$BASE_URL/" >/dev/null
}
