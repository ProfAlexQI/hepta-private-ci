# Runtime configuration is intentionally evaluated by the canonical entrypoint
# so the established HEPTA_* environment aliases keep their exact semantics.
hepta_browser_configure() {
  local caller_dir
  local reserved_output
  BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
  CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
  OUT_DIR="${HEPTA_BROWSER_SMOKE_DIR:-$(mktemp -d "${TMPDIR:-/tmp}/hepta-browser-visual-smoke.XXXXXX")}"
  VIRTUAL_TIME_BUDGET_MS="${HEPTA_BROWSER_SMOKE_VIRTUAL_TIME_BUDGET_MS:-1500}"
  REPORT_PATH="${HEPTA_BROWSER_SMOKE_REPORT_PATH:-}"

  caller_dir="$(pwd -P)"
  hepta_safe_output_resolve_directory "$caller_dir" "HEPTA_BROWSER_SMOKE_DIR" "$OUT_DIR" || return $?
  OUT_DIR="$HEPTA_SAFE_OUTPUT_PATH"
  mkdir -p "$OUT_DIR" || return 1
  [[ -d "$OUT_DIR" && ! -L "$OUT_DIR" && "$(cd "$OUT_DIR" && pwd -P)" == "$OUT_DIR" ]] || {
    hepta_safe_output_error "browser output directory changed during validation: $OUT_DIR"
    return 64
  }
  if [[ -n "$REPORT_PATH" ]]; then
    hepta_safe_output_resolve_file "$caller_dir" "HEPTA_BROWSER_SMOKE_REPORT_PATH" "$REPORT_PATH" || return $?
    REPORT_PATH="$HEPTA_SAFE_OUTPUT_PATH"
    for reserved_output in desktop.png narrow.png mobile.png phone320.png hepta-agent-logo.png; do
      if [[ "$REPORT_PATH" == "$OUT_DIR/$reserved_output" ]]; then
        hepta_safe_output_error "browser receipt collides with producer output: $REPORT_PATH"
        return 64
      fi
    done
    hepta_safe_output_prepare_parent "$REPORT_PATH" || {
      hepta_safe_output_error "could not prepare browser receipt parent: $REPORT_PATH"
      return 64
    }
  fi

  if [[ ! -x "$CHROME_BIN" ]]; then
    echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
    return 2
  fi
  CHROME_VERSION="$("$CHROME_BIN" --version 2>/dev/null || true)"
  CHROME_SHA256="$(shasum -a 256 "$CHROME_BIN" | awk '{print $1}')"
  if [[ -z "$CHROME_VERSION" || ! "$CHROME_SHA256" =~ ^[0-9a-f]{64}$ ]]; then
    echo "Chrome version or executable digest could not be captured: $CHROME_BIN" >&2
    return 2
  fi
}
