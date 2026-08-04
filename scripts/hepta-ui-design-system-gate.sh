#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

OUT_DIR="${HEPTA_UI_DESIGN_SYSTEM_GATE_DIR:-$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-design-system.XXXXXX")}"
REPORT_PATH="${HEPTA_UI_DESIGN_SYSTEM_GATE_REPORT_PATH:-$OUT_DIR/hepta-ui-design-system.json}"
mkdir -p "$OUT_DIR" "$(dirname "$REPORT_PATH")"

# shellcheck source=scripts/lib/hepta-ui-rust-toolchain.sh
source scripts/lib/hepta-ui-rust-toolchain.sh
hepta_ui_activate_rust_toolchain

TOKEN_SOURCE="design-tokens/hepta-light-glass.tokens.json"
NATIVE_THEME="apps/hepta-native/src/shared/hepta_theme.rs"
CONTROL_TOKENS="apps/hepta-control-ui/light-glass-tokens.generated.css"
CONTROL_CSS_FILES=(
  "$CONTROL_TOKENS"
  apps/hepta-control-ui/styles.legacy.css
  apps/hepta-control-ui/styles.foundation.css
  apps/hepta-control-ui/styles.components.css
  apps/hepta-control-ui/styles.responsive.css
  apps/hepta-control-ui/styles.accessibility.css
)
CONTROL_CSS_BUDGET_BYTES=300000
CONTROL_IMPORTANT_BUDGET=2100

for required in "$TOKEN_SOURCE" "$NATIVE_THEME" "apps/hepta-native/src/shared/mod.rs" \
  "docs/architecture/HEPTA_UI_LIGHT_TEMPERED_GLASS_STANDARD_2026.md" apps/hepta-control-ui/styles.css "${CONTROL_CSS_FILES[@]}"; do
  [[ -s "$required" ]] || { echo "missing design-system input: $required" >&2; exit 1; }
done

jq -e '
  .schemaVersion == 3
  and .rendererPolicy.nativeOutput == "apps/hepta-native/src/shared/hepta_theme.rs"
  and .rendererPolicy.controlOutput == "apps/hepta-control-ui/light-glass-tokens.generated.css"
  and .rendererPolicy.defaultMode == "check"
  and .rendererPolicy.writeRequiresFlag == "--write"
  and (.color.shared | has("text") and has("focus") and has("secondaryAccent"))
  and (.color.native | has("environment") and has("content") and has("glass") and has("input") and has("dim"))
  and (.color.control | has("environment") and has("panel") and has("input") and has("dim"))
  and (.materialLayers | has("environment") and has("stableContent") and has("glassChrome") and has("floatingGlass") and has("limits"))
  and .materialLayers.stableContent.blurPx == 0
  and .materialLayers.limits.maxVisibleBackdropLayers == 2
' "$TOKEN_SOURCE" >/dev/null

token_report="$(scripts/hepta-ui-light-glass-token-sync.rb --check)"
jq -e '.status == "ready" and .mode == "check" and .schema_version == 3' <<<"$token_report" >/dev/null

rg -Fq 'pub mod hepta_theme;' apps/hepta-native/src/shared/mod.rs
if rg -Fq 'light_glass_tokens' apps/hepta-native/src/shared/mod.rs apps/hepta-native/src/lib.rs; then
  echo "retired Native light_glass_tokens module is still registered" >&2
  exit 1
fi
rg -Fq 'COLOR_HEPTA_CONTENT' "$NATIVE_THEME"
rg -Fq 'COLOR_HEPTA_GLASS' "$NATIVE_THEME"
rg -Fq 'HEPTA_RADIUS_PANEL' "$NATIVE_THEME"
rg -Fq 'HEPTA_LAYER_STABLE_BLUR = 0.0' "$NATIVE_THEME"
rg -Fq -- '--hepta-max-visible-backdrop-layers: 2' "$CONTROL_TOKENS"

runtime_css_bytes="$(wc -c "${CONTROL_CSS_FILES[@]}" | awk 'END {print $1}')"
runtime_css_join_separator_bytes="$(( ${#CONTROL_CSS_FILES[@]} - 1 ))"
gateway_stylesheet_body_bytes="$(( runtime_css_bytes + runtime_css_join_separator_bytes ))"
direct_preview_manifest_bytes="$(wc -c < apps/hepta-control-ui/styles.css | tr -d ' ')"
direct_preview_css_payload_bytes="$(( direct_preview_manifest_bytes + runtime_css_bytes ))"
important_count="$(rg -o '!important' "${CONTROL_CSS_FILES[@]}" | wc -l | tr -d ' ')"
[[ "$gateway_stylesheet_body_bytes" -lt "$CONTROL_CSS_BUDGET_BYTES" ]] || { echo "Control gateway stylesheet exceeds $CONTROL_CSS_BUDGET_BYTES bytes" >&2; exit 1; }
[[ "$important_count" -le "$CONTROL_IMPORTANT_BUDGET" ]] || { echo "Control !important count exceeds $CONTROL_IMPORTANT_BUDGET" >&2; exit 1; }

if rg -Fq 'assets/k.png' "${CONTROL_CSS_FILES[@]}" apps/hepta-control-ui/index.html codex-rs/hepta-core/src/control_ui.rs; then
  echo "retired assets/k.png remains in the active Control renderer" >&2
  exit 1
fi

for query in prefers-contrast:more forced-colors:active prefers-reduced-transparency:reduce prefers-reduced-motion:reduce; do
  rg -q "$query" apps/hepta-control-ui/styles.accessibility.css || { echo "missing accessibility media query: $query" >&2; exit 1; }
done

contrast_json="$(ruby -rjson - "$TOKEN_SOURCE" <<'RUBY'
def luminance(value)
  channels = value.delete_prefix("#")[0, 6].scan(/../).map { |pair| pair.to_i(16) / 255.0 }
  linear = channels.map { |channel| channel <= 0.04045 ? channel / 12.92 : ((channel + 0.055) / 1.055) ** 2.4 }
  (0.2126 * linear[0]) + (0.7152 * linear[1]) + (0.0722 * linear[2])
end
def contrast(foreground, background)
  values = [luminance(foreground), luminance(background)]
  ((values.max + 0.05) / (values.min + 0.05)).round(3)
end
tokens = JSON.parse(File.read(ARGV.fetch(0))).fetch("color")
control = tokens.fetch("control")
native = tokens.fetch("native")
control_samples = %w[environment panel input].to_h { |role| [role, contrast(control.fetch("dim"), control.fetch(role))] }
native_samples = %w[environment content surface glass glassStrong input].to_h { |role| [role, contrast(native.fetch("dim"), native.fetch(role))] }
puts JSON.generate({ control: control_samples, native: native_samples, minimum: (control_samples.values + native_samples.values).min })
RUBY
)"
jq -e '.minimum >= 4.5 and (.control | [.[]] | min) >= 4.8' <<<"$contrast_json" >/dev/null || {
  echo "semantic dim text contrast is below renderer floor: $contrast_json" >&2
  exit 1
}

static_popover_count="$(rg -o 'popover="auto"' apps/hepta-control-ui/index.html | wc -l | tr -d ' ')"
static_trigger_count="$(rg -o 'popovertarget="[^"]+"' apps/hepta-control-ui/index.html | wc -l | tr -d ' ')"
[[ "$static_popover_count" -gt 0 && "$static_trigger_count" -ge "$static_popover_count" ]] || {
  echo "Control native popover contract is incomplete" >&2
  exit 1
}

rust_toolchain="$(hepta_ui_rustc --version)"
source_binding="$(scripts/hepta-ui-source-fingerprint)"
jq -n \
  --arg rust_toolchain "$rust_toolchain" --argjson source_binding "$source_binding" --argjson token_report "$token_report" \
  --argjson contrast "$contrast_json" --argjson runtime_css_bytes "$runtime_css_bytes" \
  --argjson runtime_css_file_count "${#CONTROL_CSS_FILES[@]}" --argjson runtime_css_join_separator_bytes "$runtime_css_join_separator_bytes" \
  --argjson gateway_stylesheet_body_bytes "$gateway_stylesheet_body_bytes" --argjson direct_preview_manifest_bytes "$direct_preview_manifest_bytes" \
  --argjson direct_preview_css_payload_bytes "$direct_preview_css_payload_bytes" \
  --argjson runtime_css_budget_bytes "$CONTROL_CSS_BUDGET_BYTES" --argjson important_count "$important_count" \
  --argjson important_budget "$CONTROL_IMPORTANT_BUDGET" --argjson static_popover_count "$static_popover_count" \
  --argjson static_trigger_count "$static_trigger_count" '
  {
    schema_version:2,
    kind:"hepta-ui-design-system-gate",
    status:"ready",
    source_binding:$source_binding,
    token_source:"design-tokens/hepta-light-glass.tokens.json",
    token_schema_version:3,
    generated_token_sync_ready:true,
    token_report:$token_report,
    dim_text_contrast:$contrast,
    rust_toolchain:$rust_toolchain,
    control:{
      runtime_css_bytes:$runtime_css_bytes,
      runtime_css_bytes_compatibility_alias_for:"css_bytes.leaf_source_bytes",
      runtime_css_budget_bytes:$runtime_css_budget_bytes,
      gateway_stylesheet_body_budget_bytes:$runtime_css_budget_bytes,
      css_bytes:{schema_version:1,measurement:"uncompressed_file_bytes_excluding_http_headers",leaf_file_count:$runtime_css_file_count,leaf_source_bytes:$runtime_css_bytes,gateway_join_separator:"\\n",gateway_join_separator_bytes:$runtime_css_join_separator_bytes,gateway_stylesheet_body_bytes:$gateway_stylesheet_body_bytes,direct_preview_manifest_bytes:$direct_preview_manifest_bytes,direct_preview_css_payload_bytes:$direct_preview_css_payload_bytes},
      important_count:$important_count,important_budget:$important_budget,retired_texture_asset_free:true,accessibility_media_queries_ready:true,static_auto_popover_count:$static_popover_count,static_popover_trigger_count:$static_trigger_count
    },
    native:{semantic_theme_registered:true,retired_light_glass_module_absent:true},
    external_side_effects_performed:false
  }' >"$REPORT_PATH"

echo "Hepta UI design-system gate passed: $REPORT_PATH"
