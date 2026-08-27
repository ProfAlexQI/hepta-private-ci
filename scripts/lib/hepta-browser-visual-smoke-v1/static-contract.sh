# Decode stdin with the platform base64 implementation used by the macOS and
# Linux CI lanes.
hepta_browser_decode_base64() {
  if base64 --help 2>&1 | grep -Fq -- '--decode'; then
    base64 --decode
  else
    base64 -D
  fi
}

hepta_browser_configure_schema_validator() {
  HEPTA_BROWSER_SCHEMA_VALIDATOR="${HEPTA_CHECK_JSONSCHEMA_BIN:-}"
  if [[ -z "$HEPTA_BROWSER_SCHEMA_VALIDATOR" ]]; then
    HEPTA_BROWSER_SCHEMA_VALIDATOR="$(command -v check-jsonschema 2>/dev/null || true)"
  elif [[ "$HEPTA_BROWSER_SCHEMA_VALIDATOR" != */* ]]; then
    HEPTA_BROWSER_SCHEMA_VALIDATOR="$(command -v "$HEPTA_BROWSER_SCHEMA_VALIDATOR" 2>/dev/null || true)"
  fi
  if [[ -z "$HEPTA_BROWSER_SCHEMA_VALIDATOR" || ! -x "$HEPTA_BROWSER_SCHEMA_VALIDATOR" \
    || "$("$HEPTA_BROWSER_SCHEMA_VALIDATOR" --version 2>/dev/null)" != "check-jsonschema, version 0.37.4" ]]; then
    echo "Control UI static contract requires check-jsonschema 0.37.4; set HEPTA_CHECK_JSONSCHEMA_BIN to its executable" >&2
    return 1
  fi
}

hepta_browser_validate_digest_report() {
  local route_name="$1"
  local schema_path="$2"
  local summary_path="$OUT_DIR/${route_name}.summary.json"
  local source_schema_path="$OUT_DIR/${route_name}.source.schema.json"
  local source_payload_path="$OUT_DIR/${route_name}.source.json"
  local invalid_summary_path="$OUT_DIR/${route_name}.invalid-summary.json"
  local invalid_source_path="$OUT_DIR/${route_name}.invalid-source.json"
  local snapshot expected_sha expected_size cursor page_count

  "$HEPTA_BROWSER_SCHEMA_VALIDATOR" --check-metaschema "$schema_path" >/dev/null
  curl -fsS "$BASE_URL/api/$route_name" -o "$summary_path"
  "$HEPTA_BROWSER_SCHEMA_VALIDATOR" --schemafile "$schema_path" "$summary_path" >/dev/null

  snapshot="$(jq -er '.full_detail.snapshot' "$summary_path")"
  expected_sha="$(jq -er '.content_sha256' "$summary_path")"
  expected_size="$(jq -er '.full_size_bytes' "$summary_path")"
  if [[ "$snapshot" != "$expected_sha" ]]; then
    echo "$route_name summary snapshot and content digest are not bound" >&2
    return 1
  fi

  cursor=0
  page_count=0
  : >"$source_payload_path"
  while :; do
    local page_path="$OUT_DIR/${route_name}.page.${cursor}.json"
    local encoded_path="$OUT_DIR/${route_name}.page.${cursor}.base64"
    local decoded_path="$OUT_DIR/${route_name}.page.${cursor}.json-bytes"
    local page_cursor page_sha page_size decoded_size complete next_cursor expected_next

    curl -fsS \
      "$BASE_URL/api/$route_name?detail=full&cursor=$cursor&snapshot=$snapshot" \
      -o "$page_path"
    "$HEPTA_BROWSER_SCHEMA_VALIDATOR" --schemafile "$schema_path" "$page_path" >/dev/null

    page_cursor="$(jq -er '.cursor' "$page_path")"
    page_sha="$(jq -er '.content_sha256' "$page_path")"
    page_size="$(jq -er '.page_size_bytes' "$page_path")"
    if [[ "$page_cursor" != "$cursor" || "$page_sha" != "$expected_sha" \
      || "$(jq -er '.full_size_bytes' "$page_path")" != "$expected_size" ]]; then
      echo "$route_name page metadata is not bound to its summary" >&2
      return 1
    fi

    jq -er '.page_data' "$page_path" >"$encoded_path"
    hepta_browser_decode_base64 <"$encoded_path" >"$decoded_path"
    decoded_size="$(wc -c <"$decoded_path" | tr -d ' ')"
    if [[ "$decoded_size" != "$page_size" ]]; then
      echo "$route_name page size does not match its decoded bytes" >&2
      return 1
    fi
    cat "$decoded_path" >>"$source_payload_path"

    page_count=$((page_count + 1))
    if (( page_count > 128 )); then
      echo "$route_name pagination exceeded its bounded page count" >&2
      return 1
    fi
    complete="$(jq -r '.complete' "$page_path")"
    next_cursor="$(jq -r '.next_cursor' "$page_path")"
    if [[ "$complete" == "true" ]]; then
      if [[ "$next_cursor" != "null" ]]; then
        echo "$route_name final page exposes a next cursor" >&2
        return 1
      fi
      break
    fi
    expected_next=$((cursor + page_size))
    if [[ "$next_cursor" != "$expected_next" ]]; then
      echo "$route_name pagination cursor is not byte-contiguous" >&2
      return 1
    fi
    cursor="$next_cursor"
  done

  if [[ "$(wc -c <"$source_payload_path" | tr -d ' ')" != "$expected_size" \
    || "$(shasum -a 256 "$source_payload_path" | awk '{print $1}')" != "$expected_sha" ]]; then
    echo "$route_name reassembled payload is not bound to its summary digest" >&2
    return 1
  fi

  jq '{"$schema": ."$schema", "$ref": "#/$defs/sourcePayload", "$defs": ."$defs"}' \
    "$schema_path" >"$source_schema_path"
  "$HEPTA_BROWSER_SCHEMA_VALIDATOR" --schemafile "$source_schema_path" "$source_payload_path" >/dev/null

  jq '.status = "ready"' "$summary_path" >"$invalid_summary_path"
  if "$HEPTA_BROWSER_SCHEMA_VALIDATOR" --schemafile "$schema_path" "$invalid_summary_path" >/dev/null 2>&1; then
    echo "$route_name schema accepted a contradictory ready summary" >&2
    return 1
  fi
  jq '
    .control_ui_product_complete = true
    | .control_ui_live_operator_surface_percent = 0
    | .control_ui_evidence.overall_evidence_percent = 100
    | .control_ui_evidence.all_required_layers_verified = true
  ' "$source_payload_path" >"$invalid_source_path"
  if "$HEPTA_BROWSER_SCHEMA_VALIDATOR" --schemafile "$source_schema_path" "$invalid_source_path" >/dev/null 2>&1; then
    echo "$route_name schema accepted contradictory product and evidence readiness" >&2
    return 1
  fi
}

# Fetches the immutable source/asset contract and leaves its values available
# to the scenario and receipt stages in the calling shell.
hepta_browser_validate_static_contract() {
hepta_browser_configure_schema_validator
root_html="$(curl -fsS "$BASE_URL/")"
styles_css="$(curl -fsS "$BASE_URL/styles.css")"
gateway_status_html="$(curl -fsS "$BASE_URL/gateway-status")"
root_headers="$OUT_DIR/root.headers"
curl -fsS -D "$root_headers" -o /dev/null "$BASE_URL/"
control_ui_js_file="$OUT_DIR/control-ui.js"
control_ui_js_headers="$OUT_DIR/control-ui-js.headers"
curl -fsS -D "$control_ui_js_headers" "$BASE_URL/control-ui.js" -o "$control_ui_js_file"
control_ui_js="$(<"$control_ui_js_file")"
logo_png="$OUT_DIR/hepta-agent-logo.png"
curl -fsS "$BASE_URL/assets/hepta-agent-logo.png" -o "$logo_png"
merge_json="$(curl -fsS "$BASE_URL/api/hepta-merge-completion")"

for needle in \
  'data-rust-frontend-renderer="hepta-core::control_ui"' \
  'data-no-js-fallback="navigation"' \
  'data-progressive-enhancement="same-origin-read-only"' \
  'data-control-ui-capability-mode="local-read-only"' \
  'data-control-ui-live-adapter-bound="false"' \
  'data-js-artifacts="external-read-only"' \
  'data-telegram-multi-agent-chat="true"' \
  'data-control-ui-product-first="true"' \
  'data-control-ui-primary-path="telegram-chat-shell"' \
  'data-control-ui-telegram-shell="true"' \
  'data-control-ui-top-design-referee="liquid-glass-2026-wcag22-320-reflow"' \
  'data-control-ui-harsh-referee="2026-06-08-liquid-glass-menus-sidebars-scroll-search"' \
  'class="tg-conversation-rail"' \
  'class="tg-thread-panel"' \
  'data-control-ui-secondary-map="collapsed"' \
  'data-control-ui-runtime-rail="local-review-safety-evidence"' \
  'data-control-ui-secondary-nav="collapsed"' \
  'data-control-ui-composer-product-first="true"' \
  'data-mobile-compact-composer="true"' \
  'data-control-ui-composer-more="collapsed"' \
  'data-control-ui-rail-search-input="light-glass"' \
  'data-control-ui-work-rail="product-first"' \
  'data-control-ui-compact-product-path="narrow-mobile"' \
  'data-control-ui-thread-tools-trigger="light-glass"' \
  'data-control-ui-thread-tools-panel="light-glass"' \
  'data-control-ui-composer-tools-trigger="light-glass"' \
  'data-control-ui-composer-tools-panel="light-glass"' \
  'data-control-ui-topbar-action="light-glass"' \
  'data-chat-first-architecture="true"' \
  'data-open-command-palette' \
  'data-control-ui-command-palette-trigger="light-glass"' \
  'id="command-palette"' \
  'data-control-ui-command-palette-surface="light-glass"' \
  'data-control-ui-command-palette-close="light-glass"' \
  'data-control-ui-catalog-mount="palette"' \
  'href="./styles.css"' \
  'defer src="./control-ui.js"' \
  'src="./assets/hepta-agent-logo.png"'; do
  if [[ "$root_html" != *"$needle"* ]]; then
    echo "control UI root is missing expected marker: $needle" >&2
    exit 1
  fi
done

for forbidden in \
  "Native gateway entrypoint running" \
  "<script>" \
  "Rust/no-JS chat workspace" \
  "old JS" \
  "blank module fallback" \
  "NO_REPLY" \
  "mutation=false" \
  "payload hash" \
  "hepta-product-path" \
  "Ask / Plan / Evidence / Approve" \
  "Fixture mode"; do
  if [[ "$root_html" == *"$forbidden"* ]]; then
    echo "control UI root includes forbidden fallback marker: $forbidden" >&2
    exit 1
  fi
done

if ! grep -Fqi "content-security-policy:" "$root_headers" \
  || ! grep -Fqi "script-src 'self';" "$root_headers" \
  || ! grep -Fqi "connect-src 'self';" "$root_headers" \
  || ! grep -Fqi "object-src 'none';" "$root_headers" \
  || ! grep -Fqi "form-action 'none';" "$root_headers" \
  || grep -Fqi "script-src 'self' 'unsafe-inline'" "$root_headers"; then
  echo "control UI response CSP does not enforce same-origin external scripts and connections" >&2
  exit 1
fi

control_ui_base_js_source="apps/hepta-control-ui/control-ui.js"
control_ui_runtime_js_source="apps/hepta-control-ui/control-ui-v4-runtime.js"
control_ui_expected_js_file="$OUT_DIR/control-ui.expected.js"
cat "$control_ui_base_js_source" >"$control_ui_expected_js_file"
printf '\n/* hepta-ui-v4-runtime-bundle-boundary */\n' >>"$control_ui_expected_js_file"
cat "$control_ui_runtime_js_source" >>"$control_ui_expected_js_file"

source_js_sha="$(shasum -a 256 "$control_ui_base_js_source" | awk '{print $1}')"
runtime_js_sha="$(shasum -a 256 "$control_ui_runtime_js_source" | awk '{print $1}')"
expected_bundle_js_sha="$(shasum -a 256 "$control_ui_expected_js_file" | awk '{print $1}')"
served_js_sha="$(shasum -a 256 "$control_ui_js_file" | awk '{print $1}')"
served_js_etag="$(awk 'tolower($1) == "etag:" { gsub(/\r/, ""); sub(/^[^:]+:[[:space:]]*/, ""); print; exit }' "$control_ui_js_headers")"
control_ui_v4_runtime_bound=false
if ! cmp -s "$control_ui_expected_js_file" "$control_ui_js_file" \
  || [[ "$served_js_sha" != "$expected_bundle_js_sha" \
  || "$served_js_etag" != "\"sha256-${expected_bundle_js_sha}\"" ]]; then
  echo "Control UI base/runtime sources, served bundle bytes, and ETag digest are not bound" >&2
  exit 1
fi
control_ui_v4_runtime_bound=true

if [[ "$(grep -Eo ', \"/api/[^\"]+\", (true|false)\]' "$control_ui_js_file" | wc -l | tr -d ' ')" != "21" ]]; then
  echo "control UI JavaScript does not expose exactly 21 fixed read-only report routes" >&2
  exit 1
fi
for marker in \
  'const SNAPSHOT_PATH = "/api/operator-snapshot"' \
  'const COMMAND_CATALOG = Object.freeze([' \
  'const READ_ONLY_ROUTES = Object.freeze(Object.fromEntries(' \
  'typed-command-catalog-v1' \
  'renderCommandCatalog()' \
  'configureRouteViews()' \
  'const UNAVAILABLE_PREVIEW_CONTROLS = Object.freeze([' \
  'configureUnavailablePreviewControls()' \
  'configureLocalJsonPreview()' \
  'configureComposerPickerSearch()' \
  'insertLocalDraftText(' \
  'let commandGeneration = 0' \
  'let activeCommandRequest = null' \
  'new AbortController()' \
  'response.body?.getReader()' \
  'reader.cancel("Response exceeded the local display limit")' \
  'new TextDecoder("utf-8", { fatal: true })' \
  'source_path: path' \
  'headers: { Accept: "application/json" }' \
  'url.origin !== window.location.origin' \
  'hepta-ui-v4-runtime-bundle-boundary' \
  'const READ_STATE_SET = new Set(READ_STATES)' \
  'HeptaUiV4ReadState' \
  'controlUiV4Runtime = "ready"' \
  'controlUiV4RuntimeAuthority = "local-ui-only"' \
  'textContent'; do
  if [[ "$control_ui_js" != *"$marker"* ]]; then
    echo "control UI JavaScript is missing safety marker: $marker" >&2
    exit 1
  fi
done
for forbidden_js in 'innerHTML' 'eval(' 'new Function(' 'http://' 'https://'; do
  if [[ "$control_ui_js" == *"$forbidden_js"* ]]; then
    echo "control UI JavaScript contains forbidden capability: $forbidden_js" >&2
    exit 1
  fi
done

for needle in ".tg-conversation-rail" ".tg-thread-panel" ".command-palette" "safe-area-inset-bottom" "mrog" "data-control-ui-compact-product-path" "data-control-ui-primary-shell-light-glass" "crs" "cwb" "cce" "pce" "ppe" "cpe" "mpb" "ipc" "avr" "rpf" "rcs" "mmp" "tsp" "csh" "rms" "hte" "rsc" "rpe" "mbp" "bsp" "rsp" "fcp" "strong){filter" "--x:0 1px #fff6" "text-shadow:var(--x)" "font-weight:650" "rdlg" "oclg" "data-control-ui-tspcfrg" "body[data-view=chat] .hepta-secondary-map{display:none}" "gar26" "htr26" "cmv" "ctlg" "cplg" "rmlg" "ttlg" "tiblg" "bmslg" "stslg" "talg" "cps" "cpis" "cpt" "cpc" "cpir" "cph" "cprw" "cprr" "cpkc" "cpilg" "cpici" "data-control-ui-command-palette-input=light-glass" "data-control-ui-command-palette-result=light-glass"; do
  if [[ "$styles_css" != *"$needle"* ]]; then
    echo "control UI stylesheet is missing expected rule marker: $needle" >&2
    exit 1
  fi
done

for forbidden_style in \
  'letter-spacing:-' \
  'letter-spacing: -' \
  'radial-gradient(circle' \
  'font-size: clamp('; do
  if [[ "$styles_css" == *"$forbidden_style"* ]]; then
    echo "control UI stylesheet still contains a top-design-forbidden style: $forbidden_style" >&2
    exit 1
  fi
done

if [[ "$gateway_status_html" != *"Native gateway entrypoint running"* ]]; then
  echo "gateway status page no longer exposes the native gateway readiness copy" >&2
  exit 1
fi

logo_bytes="$(wc -c <"$logo_png" | tr -d ' ')"
if [[ "$logo_bytes" -lt 1024 ]]; then
  echo "logo asset is suspiciously small: ${logo_bytes} bytes" >&2
  exit 1
fi
logo_dimensions="$(
  sips -g pixelWidth -g pixelHeight "$logo_png" 2>/dev/null |
    awk '/pixelWidth/ { w=$2 } /pixelHeight/ { h=$2 } END { print w "x" h }'
)"
if [[ "$logo_dimensions" == "x" || "$logo_dimensions" == "0x0" ]]; then
  echo "logo asset dimensions could not be read" >&2
  exit 1
fi

if [[ "$(jq -r '.runtime' <<<"$merge_json")" != "hepta" ]]; then
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

hepta_browser_validate_digest_report \
  control-ui \
  apps/hepta-control-ui/schemas/control-ui.schema.json
hepta_browser_validate_digest_report \
  ui-contract-audit \
  apps/hepta-control-ui/schemas/ui-contract-audit.schema.json
}
