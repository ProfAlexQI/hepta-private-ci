hepta_browser_run_scenarios() {
  hepta_browser_capture_viewport "desktop" "1365x900"
  hepta_browser_capture_viewport "narrow" "768x900"
  hepta_browser_capture_viewport "mobile" "500x844"
  hepta_browser_capture_viewport "phone320" "320x844"

  density_qa_status=0
  density_qa_json="$(node "$HEPTA_BROWSER_SMOKE_LIB_DIR/density-qa.cjs" "$CHROME_BIN" "$BASE_URL")" \
    || density_qa_status="$?"
  printf '%s\n' "$density_qa_json" >"$OUT_DIR/density-qa.json"

  progressive_qa_status=0
  progressive_qa_json="$(node "$HEPTA_BROWSER_SMOKE_LIB_DIR/progressive-enhancement-qa.cjs" "$CHROME_BIN" "$BASE_URL")" \
    || progressive_qa_status="$?"
  printf '%s\n' "$progressive_qa_json" >"$OUT_DIR/progressive-enhancement-qa.json"
}
