#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

OUT_DIR="${HEPTA_UI_DESIGN_SYSTEM_GATE_DIR:-$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-design-system.XXXXXX")}"
REPORT_PATH="${HEPTA_UI_DESIGN_SYSTEM_GATE_REPORT_PATH:-$OUT_DIR/hepta-ui-design-system.json}"
mkdir -p "$OUT_DIR"

source scripts/lib/hepta-ui-rust-toolchain.sh

CONTROL_CSS_FILES=(
  apps/hepta-control-ui/light-glass-tokens.generated.css
  apps/hepta-control-ui/styles.legacy.css
  apps/hepta-control-ui/styles.foundation.css
  apps/hepta-control-ui/styles.components.css
  apps/hepta-control-ui/styles.responsive.css
  apps/hepta-control-ui/styles.accessibility.css
)
CONTROL_CSS_BUDGET_BYTES=300000
CONTROL_IMPORTANT_BASELINE=2913
CONTROL_IMPORTANT_BUDGET=2100
ROBRIX_UPSTREAM_COMMIT="a5a664da569c577ab1a3e5a33f45dcc9364954a0"

for css_file in "${CONTROL_CSS_FILES[@]}"; do
  [[ -s "$css_file" ]] || {
    echo "missing Control CSS layer: $css_file" >&2
    exit 1
  }
done

for fixture_token_mapping in \
  'environment|wash' \
  'panel|panel' \
  'text|ink' \
  'muted|muted' \
  'dim|dim' \
  'hairline|hairline' \
  'focus|accent'; do
  IFS='|' read -r token_key fixture_variable <<<"$fixture_token_mapping"
  token_value="$(jq -r --arg key "$token_key" '.color[$key]' design-tokens/hepta-light-glass.tokens.json)"
  rg -Fq -- "--${fixture_variable}: ${token_value};" scripts/hepta-native-fixture-visual-smoke.sh || {
    echo "Native HTML fixture token drift: ${fixture_variable} must use ${token_key}=${token_value}" >&2
    exit 1
  }
done

ruby scripts/hepta-ui-light-glass-token-sync.rb --check >/dev/null
rust_version="$(hepta_ui_rustc --version)"
[[ "$rust_version" == rustc\ 1.95.0* ]] || {
  echo "unexpected UI Rust toolchain: $rust_version" >&2
  exit 1
}

for documented_token in environment panel input text muted dim hairline focus secondaryAccent glassShadow; do
  token_value="$(jq -r --arg key "$documented_token" '.color[$key]' design-tokens/hepta-light-glass.tokens.json)"
  token_value_upper="$(printf '%s' "$token_value" | tr '[:lower:]' '[:upper:]')"
  rg -Fq "$token_value" docs/architecture/HEPTA_UI_LIGHT_TEMPERED_GLASS_STANDARD_2026.md || {
    echo "design standard is missing current CSS token $documented_token=$token_value" >&2
    exit 1
  }
  rg -Fq "$token_value_upper" docs/architecture/HEPTA_UI_LIGHT_TEMPERED_GLASS_STANDARD_2026.md || {
    echo "design standard is missing current Native token $documented_token=$token_value_upper" >&2
    exit 1
  }
done

dim_contrast_json="$(ruby -rjson -e '
  def channel_luminance(channel)
    normalized = channel / 255.0
    normalized <= 0.04045 ? normalized / 12.92 : ((normalized + 0.055) / 1.055) ** 2.4
  end

  def luminance(hex)
    rgb = hex.delete_prefix("#")[0, 6].scan(/../).map { |pair| pair.to_i(16) }
    (0.2126 * channel_luminance(rgb[0])) +
      (0.7152 * channel_luminance(rgb[1])) +
      (0.0722 * channel_luminance(rgb[2]))
  end

  colors = JSON.parse(File.read(ARGV.fetch(0))).fetch("color")
  foreground = luminance(colors.fetch("dim"))
  backgrounds = %w[environment panel input].to_h do |name|
    background = luminance(colors.fetch(name))
    ratio = ([foreground, background].max + 0.05) / ([foreground, background].min + 0.05)
    [name, ratio.round(3)]
  end
  print JSON.generate({foreground: colors.fetch("dim"), backgrounds: backgrounds, minimum: backgrounds.values.min})
' design-tokens/hepta-light-glass.tokens.json)"
jq -e '.minimum >= 4.8' <<<"$dim_contrast_json" >/dev/null || {
  echo "shared dim text token is below the 4.8:1 safety floor: $dim_contrast_json" >&2
  exit 1
}

runtime_css_bytes="$(wc -c "${CONTROL_CSS_FILES[@]}" | awk 'END {print $1}')"
important_count="$(rg -o '!important' "${CONTROL_CSS_FILES[@]}" | wc -l | tr -d ' ')"
legacy_texture_asset_reference_count="$(
  { rg -o -F 'assets/k.png' "${CONTROL_CSS_FILES[@]}" 2>/dev/null || true; } \
    | wc -l \
    | tr -d ' '
)"
[[ "$runtime_css_bytes" -lt "$CONTROL_CSS_BUDGET_BYTES" ]] || {
  echo "Control runtime CSS exceeds budget: $runtime_css_bytes >= $CONTROL_CSS_BUDGET_BYTES" >&2
  exit 1
}
[[ "$important_count" -le "$CONTROL_IMPORTANT_BUDGET" ]] || {
  echo "Control !important count exceeds the post-refactor budget: $important_count > $CONTROL_IMPORTANT_BUDGET" >&2
  exit 1
}
[[ "$legacy_texture_asset_reference_count" -eq 0 ]] || {
  echo "Control runtime CSS still requests the retired k.png texture: $legacy_texture_asset_reference_count references" >&2
  exit 1
}

for query in \
  'prefers-contrast:more' \
  'forced-colors:active' \
  'prefers-reduced-transparency:reduce' \
  'prefers-reduced-motion:reduce'; do
  rg -q "$query" apps/hepta-control-ui/styles.accessibility.css || {
    echo "missing accessibility media query: $query" >&2
    exit 1
  }
done

static_popover_count="$(rg -o 'popover="auto"' apps/hepta-control-ui/index.html | wc -l | tr -d ' ')"
static_trigger_count="$(rg -o 'popovertarget="[^"]+"' apps/hepta-control-ui/index.html | wc -l | tr -d ' ')"
legacy_palette_anchor_count="$(
  { rg -o 'href="#command-palette"' apps/hepta-control-ui/index.html codex-rs/hepta-core/src/control_ui.rs 2>/dev/null || true; } \
    | wc -l \
    | tr -d ' '
)"
light_root_marker='<html lang="en" dir="auto" data-theme="premium" data-theme-mode="light"'
[[ "$static_popover_count" -eq 8 ]] || {
  echo "expected 8 static auto popovers, found $static_popover_count" >&2
  exit 1
}
[[ "$static_trigger_count" -ge 9 ]] || {
  echo "expected at least 9 static popover triggers, found $static_trigger_count" >&2
  exit 1
}
[[ "$legacy_palette_anchor_count" -eq 0 ]] || {
  echo "legacy command-palette anchors remain: $legacy_palette_anchor_count" >&2
  exit 1
}
for light_root_source in \
  apps/hepta-control-ui/index.html \
  codex-rs/hepta-core/src/control_ui.rs; do
  rg -Fq "$light_root_marker" "$light_root_source" || {
    echo "Control light theme and automatic document direction are not source-backed: $light_root_source" >&2
    exit 1
  }
  if rg -Fq 'data-theme-mode="dark"' "$light_root_source"; then
    echo "stale dark Control theme marker remains: $light_root_source" >&2
    exit 1
  fi
done

rg -Fq 'light-glass-tokens.generated.css' codex-rs/hepta-core/src/control_ui.rs
rg -Fq 'pub mod light_glass_tokens;' apps/hepta-native/src/shared/mod.rs
rg -Fq 'Copyright (c) 2023-2026 Project Robius Developers' apps/hepta-native/LICENSE-MIT
for fixture_token_marker in \
  'COLOR_HEPTA_GLASS_PANEL' \
  'COLOR_HEPTA_GLASS_ACTIVE_SURFACE' \
  'COLOR_HEPTA_GLASS_HAIRLINE' \
  'COLOR_HEPTA_GLASS_SHADOW' \
  'HEPTA_GLASS_CONTROL_RADIUS' \
  'HEPTA_GLASS_PANEL_RADIUS' \
  'HEPTA_GLASS_FLOATING_RADIUS'; do
  rg -Fq "$fixture_token_marker" apps/hepta-native/src/home/hepta_fixture_cockpit.rs || {
    echo "Hepta Native fixture does not consume generated token: $fixture_token_marker" >&2
    exit 1
  }
done
if rg -q '#xF5FCFF|#x7FC8DE|border_radius: 7\.0' apps/hepta-native/src/home/hepta_fixture_cockpit.rs; then
  echo "Hepta Native fixture retains pre-token light-glass card values" >&2
  exit 1
fi
if rg -q 'draw_icon\.svg: \(HEPTA_INTAKE_ICON_' apps/hepta-native/src/shared/room_input_popup_menu.rs; then
  echo "Robrix intake icon is not qualified in the Makepad script scope" >&2
  exit 1
fi

robrix_commit_marker_count="$(rg -l "$ROBRIX_UPSTREAM_COMMIT" \
  apps/hepta-native/src/shared/slash_commands.rs \
  apps/hepta-native/src/shared/file_upload_modal.rs \
  apps/hepta-native/src/home/upload_progress.rs \
  apps/hepta-native/src/shared/attachment_download.rs \
  apps/hepta-native/src/shared/mention_popup.rs \
  apps/hepta-native/src/shared/room_input_popup_menu.rs | wc -l | tr -d ' ')"
[[ "$robrix_commit_marker_count" -eq 6 ]] || {
  echo "Robrix provenance is incomplete: $robrix_commit_marker_count/6 modules" >&2
  exit 1
}

jq -n \
  --arg rust_toolchain "$rust_version" \
  --arg robrix_upstream_commit "$ROBRIX_UPSTREAM_COMMIT" \
  --argjson runtime_css_bytes "$runtime_css_bytes" \
  --argjson runtime_css_budget_bytes "$CONTROL_CSS_BUDGET_BYTES" \
  --argjson important_count "$important_count" \
  --argjson important_budget "$CONTROL_IMPORTANT_BUDGET" \
  --argjson important_baseline "$CONTROL_IMPORTANT_BASELINE" \
  --argjson legacy_texture_asset_reference_count "$legacy_texture_asset_reference_count" \
  --argjson css_layer_count "${#CONTROL_CSS_FILES[@]}" \
  --argjson static_popover_count "$static_popover_count" \
  --argjson static_trigger_count "$static_trigger_count" \
  --argjson robrix_module_count "$robrix_commit_marker_count" \
  --argjson dim_text_contrast "$dim_contrast_json" \
  '{
    status:"ready",
    product:"Hepta shared desktop/mobile UI design system",
    token_source:"design-tokens/hepta-light-glass.tokens.json",
    generated_token_sync_ready:true,
    documentation_token_sync_ready:true,
    dim_text_contrast:$dim_text_contrast,
    rust_toolchain:$rust_toolchain,
    control:{
      css_layer_count:$css_layer_count,
      runtime_css_bytes:$runtime_css_bytes,
      runtime_css_budget_bytes:$runtime_css_budget_bytes,
      important_count:$important_count,
      important_budget:$important_budget,
      important_audit_baseline:$important_baseline,
      legacy_texture_asset_reference_count:$legacy_texture_asset_reference_count,
      retired_texture_asset_free:true,
      accessibility_media_queries_ready:true,
      static_light_theme_ready:true,
      renderer_light_theme_ready:true,
      document_direction_source_ready:true,
      static_auto_popover_count:$static_popover_count,
      static_popover_trigger_count:$static_trigger_count,
      legacy_command_palette_anchor_count:0
    },
    native:{
      generated_tokens_registered:true,
      html_fixture_token_sync_ready:true,
      fixture_generated_tokens_consumed:true,
      fixture_unified_radius_scale_ready:true,
      fixture_key_surface_shadows_ready:true,
      makepad_intake_icon_scope_ready:true
    },
    robrix:{
      upstream_commit:$robrix_upstream_commit,
      selective_module_count:$robrix_module_count,
      license:"MIT",
      license_notice_current:true
    }
  }' >"$REPORT_PATH"

echo "Hepta UI design-system gate passed: $REPORT_PATH"
