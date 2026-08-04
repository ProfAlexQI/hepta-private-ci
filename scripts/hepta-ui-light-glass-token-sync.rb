#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"

ROOT = Pathname.new(__dir__).join("..").expand_path
SOURCE = ROOT.join("design-tokens/hepta-light-glass.tokens.json")
CONTROL_OUTPUT = ROOT.join("apps/hepta-control-ui/light-glass-tokens.generated.css")
NATIVE_OUTPUT = ROOT.join("apps/hepta-native/src/shared/hepta_theme.rs")

def source_binding
  stdout, stderr, status = Open3.capture3(ROOT.join("scripts/hepta-ui-source-fingerprint").to_s, chdir: ROOT.to_s)
  abort("source fingerprint failed: #{stderr.strip}") unless status.success?
  JSON.parse(stdout)
end

def binding_equal?(left, right)
  %w[head head_tree source_fingerprint].all? { |key| left[key] == right[key] }
end

def abort_usage
  abort("usage: #{File.basename($PROGRAM_NAME)} [--check|--write]\n" \
        "       default mode is --check; --write is the only mutating mode")
end

mode = ARGV.empty? ? :check : case ARGV
                             when ["--check"] then :check
                             when ["--write"] then :write
                             else abort_usage
                             end
binding_before = source_binding

tokens = JSON.parse(SOURCE.read)
abort("unsupported token schema: expected schemaVersion=3") unless tokens["schemaVersion"] == 3

colors = tokens.fetch("color")
shared = colors.fetch("shared")
native = colors.fetch("native")
control = colors.fetch("control")
radii = tokens.fetch("radius")
shared_radii = radii.fetch("shared")
native_radii = radii.fetch("native")
control_radii = radii.fetch("control")
control_motion = tokens.fetch("motion").fetch("control")
native_interaction = tokens.fetch("interaction").fetch("native")
material_layers = tokens.fetch("materialLayers")
environment_layer = material_layers.fetch("environment")
stable_content_layer = material_layers.fetch("stableContent")
glass_chrome_layer = material_layers.fetch("glassChrome")
floating_glass_layer = material_layers.fetch("floatingGlass")
material_limits = material_layers.fetch("limits")
policy = tokens.fetch("rendererPolicy")

expected_policy = {
  "nativeOutput" => NATIVE_OUTPUT.relative_path_from(ROOT).to_s,
  "controlOutput" => CONTROL_OUTPUT.relative_path_from(ROOT).to_s,
  "defaultMode" => "check",
  "writeRequiresFlag" => "--write",
}
policy_errors = expected_policy.each_with_object([]) do |(key, expected), errors|
  errors << "#{key}=#{policy[key].inspect} (expected #{expected.inspect})" unless policy[key] == expected
end
abort("invalid rendererPolicy: #{policy_errors.join(', ')}") unless policy_errors.empty?

required = {
  "color.shared" => [shared, %w[text focus secondaryAccent]],
  "color.native" => [native, %w[
    environment content surface glass glassStrong input muted dim hairline
    hairlineStrong focusHover focusSurface focusSurfaceHover selection success
    successSurface danger dangerSurface warning disabled disabledSurface shadow
  ]],
  "color.control" => [control, %w[
    environment panel input muted dim hairline activeSurface success glassShadow
    glassGlow elevated textStrong mutedStrong accentHover
  ]],
  "radius.shared" => [shared_radii, %w[control floating]],
  "radius.native" => [native_radii, %w[panel]],
  "radius.control" => [control_radii, %w[panel]],
  "motion.control" => [control_motion, %w[fastMs normalMs]],
  "interaction.native" => [native_interaction, %w[minimumTouchTarget]],
  "materialLayers.environment" => [environment_layer, %w[surfaceAlpha hairlineAlpha shadowAlpha radiusPx blurPx]],
  "materialLayers.stableContent" => [stable_content_layer, %w[surfaceAlpha hairlineAlpha shadowAlpha radiusPx blurPx]],
  "materialLayers.glassChrome" => [glass_chrome_layer, %w[surfaceAlpha hairlineAlpha shadowAlpha radiusPx blurPx]],
  "materialLayers.floatingGlass" => [floating_glass_layer, %w[surfaceAlpha hairlineAlpha shadowAlpha radiusPx blurPx]],
  "materialLayers.limits" => [material_limits, %w[maxVisibleBackdropLayers maxStableContentBackdropLayers]],
}
required.each do |group, (values, keys)|
  missing = keys.reject { |key| values.key?(key) }
  abort("missing #{group} tokens: #{missing.join(', ')}") unless missing.empty?
end

native_minimum_touch_target = Integer(native_interaction.fetch("minimumTouchTarget"))
abort("invalid interaction.native.minimumTouchTarget=#{native_minimum_touch_target}") unless native_minimum_touch_target == 48

def rgba!(value, name)
  normalized = String(value).downcase
  raise "invalid RGBA token #{name}=#{value.inspect}" unless normalized.match?(/\A#[0-9a-f]{8}\z/)
  normalized
end

all_color_groups = { "shared" => shared, "native" => native, "control" => control }
all_color_groups.each do |group, values|
  values.each { |name, value| rgba!(value, "color.#{group}.#{name}") }
end

[environment_layer, stable_content_layer, glass_chrome_layer, floating_glass_layer].each do |layer|
  %w[surfaceAlpha hairlineAlpha shadowAlpha].each do |key|
    value = Float(layer.fetch(key))
    abort("invalid material alpha #{key}=#{value}") unless value.between?(0.0, 1.0)
  end
  %w[radiusPx blurPx].each { |key| abort("invalid material metric #{key}") if Integer(layer.fetch(key)).negative? }
end

def makepad_color(value)
  "#x#{value.delete_prefix('#').upcase}"
end

def vec4_color(value)
  channels = value.delete_prefix("#").scan(/../).map { |part| part.to_i(16) }
  formatted = channels.map do |byte|
    number = (byte / 255.0).round(3)
    text = format("%.3f", number).sub(/0+\z/, "").sub(/\.\z/, ".0")
    text == "0" ? "0.0" : text
  end
  "vec4(#{formatted.join(', ')})"
end

def css_hex(value)
  value.downcase
end

control_text = shared.fetch("text")
control_focus = shared.fetch("focus")
control_secondary = shared.fetch("secondaryAccent")

css = <<~CSS
  /* @generated by scripts/hepta-ui-light-glass-token-sync.rb; do not edit. */
  :root {
    --hepta-glass-environment: #{css_hex(control.fetch('environment'))};
    --hepta-glass-panel: #{css_hex(control.fetch('panel'))};
    --hepta-glass-input: #{css_hex(control.fetch('input'))};
    --hepta-glass-text: #{css_hex(control_text)};
    --hepta-glass-muted: #{css_hex(control.fetch('muted'))};
    --hepta-glass-dim: #{css_hex(control.fetch('dim'))};
    --hepta-glass-hairline: #{css_hex(control.fetch('hairline'))};
    --hepta-glass-focus: #{css_hex(control_focus)};
    --hepta-glass-secondary-accent: #{css_hex(control_secondary)};
    --hepta-glass-active-surface: #{css_hex(control.fetch('activeSurface'))};
    --hepta-glass-success: #{css_hex(control.fetch('success'))};
    --hepta-glass-shadow: #{css_hex(control.fetch('glassShadow'))};
    --hepta-glass-glow: #{css_hex(control.fetch('glassGlow'))};
    --hepta-glass-control-radius: #{Integer(shared_radii.fetch('control'))}px;
    --hepta-glass-panel-radius: #{Integer(control_radii.fetch('panel'))}px;
    --hepta-glass-floating-radius: #{Integer(shared_radii.fetch('floating'))}px;
    --hepta-glass-motion-fast: #{Integer(control_motion.fetch('fastMs'))}ms;
    --hepta-glass-motion-normal: #{Integer(control_motion.fetch('normalMs'))}ms;
    --hepta-layer-stable-alpha: #{stable_content_layer.fetch('surfaceAlpha')};
    --hepta-layer-stable-hairline-alpha: #{stable_content_layer.fetch('hairlineAlpha')};
    --hepta-layer-stable-shadow-alpha: #{stable_content_layer.fetch('shadowAlpha')};
    --hepta-layer-stable-radius: #{Integer(stable_content_layer.fetch('radiusPx'))}px;
    --hepta-layer-chrome-alpha: #{glass_chrome_layer.fetch('surfaceAlpha')};
    --hepta-layer-chrome-hairline-alpha: #{glass_chrome_layer.fetch('hairlineAlpha')};
    --hepta-layer-chrome-shadow-alpha: #{glass_chrome_layer.fetch('shadowAlpha')};
    --hepta-layer-chrome-radius: #{Integer(glass_chrome_layer.fetch('radiusPx'))}px;
    --hepta-layer-chrome-blur: #{Integer(glass_chrome_layer.fetch('blurPx'))}px;
    --hepta-layer-floating-alpha: #{floating_glass_layer.fetch('surfaceAlpha')};
    --hepta-layer-floating-hairline-alpha: #{floating_glass_layer.fetch('hairlineAlpha')};
    --hepta-layer-floating-shadow-alpha: #{floating_glass_layer.fetch('shadowAlpha')};
    --hepta-layer-floating-radius: #{Integer(floating_glass_layer.fetch('radiusPx'))}px;
    --hepta-layer-floating-blur: #{Integer(floating_glass_layer.fetch('blurPx'))}px;
    --hepta-max-visible-backdrop-layers: #{Integer(material_limits.fetch('maxVisibleBackdropLayers'))};
  }

  html[data-theme-mode="light"] {
    color-scheme: light;
    --bg: var(--hepta-glass-environment);
    --bg-elevated: #{css_hex(control.fetch('elevated')[0, 7])};
    --bg-hover: var(--hepta-glass-active-surface);
    --card: var(--hepta-glass-panel);
    --card-foreground: var(--hepta-glass-text);
    --popover: var(--hepta-glass-input);
    --panel: var(--hepta-glass-panel);
    --panel-strong: var(--hepta-glass-input);
    --panel-hover: var(--hepta-glass-active-surface);
    --chrome: var(--hepta-glass-panel);
    --chrome-strong: var(--hepta-glass-input);
    --text: var(--hepta-glass-text);
    --text-strong: #{css_hex(control.fetch('textStrong')[0, 7])};
    --chat-text: var(--hepta-glass-text);
    --muted: var(--hepta-glass-muted);
    --muted-strong: #{css_hex(control.fetch('mutedStrong')[0, 7])};
    --muted-foreground: var(--hepta-glass-muted);
    --border: var(--hepta-glass-hairline);
    --border-strong: color-mix(in srgb, var(--hepta-glass-focus) 38%, var(--hepta-glass-hairline));
    --border-hover: var(--hepta-glass-focus);
    --input: var(--hepta-glass-input);
    --ring: var(--hepta-glass-focus);
    --accent: var(--hepta-glass-focus);
    --accent-hover: #{css_hex(control.fetch('accentHover')[0, 7])};
    --accent-muted: color-mix(in srgb, var(--hepta-glass-focus) 72%, transparent);
    --accent-subtle: color-mix(in srgb, var(--hepta-glass-focus) 10%, transparent);
    --accent-glow: var(--hepta-glass-glow);
    --primary: var(--hepta-glass-focus);
    --primary-foreground: #ffffff;
    --secondary: var(--hepta-glass-panel);
    --secondary-foreground: var(--hepta-glass-text);
    --accent-2: var(--hepta-glass-secondary-accent);
    --ok: var(--hepta-glass-success);
    --focus: color-mix(in srgb, var(--hepta-glass-focus) 22%, transparent);
    --grid-line: color-mix(in srgb, var(--hepta-glass-focus) 5%, transparent);
    --shadow-sm: 0 1px 2px var(--hepta-glass-shadow);
    --shadow-md: 0 4px 14px var(--hepta-glass-shadow);
    --shadow-lg: 0 12px 30px var(--hepta-glass-shadow);
    --shadow-xl: 0 20px 42px var(--hepta-glass-shadow);
    --radius-md: var(--hepta-glass-control-radius);
    --radius-lg: var(--hepta-glass-panel-radius);
    --radius-xl: var(--hepta-glass-floating-radius);
    --duration-fast: var(--hepta-glass-motion-fast);
    --duration-normal: var(--hepta-glass-motion-normal);
  }
CSS

native_text = shared.fetch("text")
native_focus = shared.fetch("focus")
native_rust = <<~RUST
  //! Hepta's semantic light-glass palette.
  //!
  //! Message and content surfaces stay nearly opaque. Translucent color,
  //! hairlines, and soft highlights are reserved for navigation, the composer,
  //! and floating controls so the interface still reads as a chat product.

  use makepad_widgets::*;

  script_mod! {
      use mod.prelude.widgets.*
      use mod.widgets.*

      mod.widgets.COLOR_HEPTA_ENVIRONMENT = #{makepad_color(native.fetch('environment'))}
      mod.widgets.COLOR_HEPTA_CONTENT = #{makepad_color(native.fetch('content'))}
      mod.widgets.COLOR_HEPTA_SURFACE = #{makepad_color(native.fetch('surface'))}
      mod.widgets.COLOR_HEPTA_GLASS = #{makepad_color(native.fetch('glass'))}
      mod.widgets.COLOR_HEPTA_GLASS_STRONG = #{makepad_color(native.fetch('glassStrong'))}
      mod.widgets.COLOR_HEPTA_INPUT = #{makepad_color(native.fetch('input'))}

      mod.widgets.COLOR_HEPTA_TEXT = #{makepad_color(native_text)}
      mod.widgets.COLOR_HEPTA_MUTED = #{makepad_color(native.fetch('muted'))}
      // This is also used for placeholders and timestamps. Keep it at >= 4.5:1
      // against the lightest Hepta surfaces instead of treating it as decoration.
      mod.widgets.COLOR_HEPTA_DIM = #{makepad_color(native.fetch('dim'))}

      mod.widgets.COLOR_HEPTA_HAIRLINE = #{makepad_color(native.fetch('hairline'))}
      mod.widgets.COLOR_HEPTA_HAIRLINE_STRONG = #{makepad_color(native.fetch('hairlineStrong'))}
      mod.widgets.COLOR_HEPTA_FOCUS = #{makepad_color(native_focus)}
      mod.widgets.COLOR_HEPTA_FOCUS_HOVER = #{makepad_color(native.fetch('focusHover'))}
      mod.widgets.COLOR_HEPTA_FOCUS_SURFACE = #{makepad_color(native.fetch('focusSurface'))}
      mod.widgets.COLOR_HEPTA_FOCUS_SURFACE_HOVER = #{makepad_color(native.fetch('focusSurfaceHover'))}
      mod.widgets.COLOR_HEPTA_SELECTION = #{makepad_color(native.fetch('selection'))}

      mod.widgets.COLOR_HEPTA_SUCCESS = #{makepad_color(native.fetch('success'))}
      mod.widgets.COLOR_HEPTA_SUCCESS_SURFACE = #{makepad_color(native.fetch('successSurface'))}
      mod.widgets.COLOR_HEPTA_DANGER = #{makepad_color(native.fetch('danger'))}
      mod.widgets.COLOR_HEPTA_DANGER_SURFACE = #{makepad_color(native.fetch('dangerSurface'))}
      mod.widgets.COLOR_HEPTA_WARNING = #{makepad_color(native.fetch('warning'))}
      mod.widgets.COLOR_HEPTA_DISABLED = #{makepad_color(native.fetch('disabled'))}
      mod.widgets.COLOR_HEPTA_DISABLED_SURFACE = #{makepad_color(native.fetch('disabledSurface'))}
      mod.widgets.COLOR_HEPTA_SHADOW = #{makepad_color(native.fetch('shadow'))}

      mod.widgets.HEPTA_RADIUS_CONTROL = #{Integer(shared_radii.fetch('control'))}.0
      mod.widgets.HEPTA_RADIUS_PANEL = #{Integer(native_radii.fetch('panel'))}.0
      mod.widgets.HEPTA_RADIUS_FLOATING = #{Integer(shared_radii.fetch('floating'))}.0
      // Login controls use the stricter Android target on every platform so the
      // shared tree never drops below 48 logical points.
      mod.widgets.HEPTA_TOUCH_TARGET = #{native_minimum_touch_target}.0
      mod.widgets.HEPTA_LAYER_STABLE_ALPHA = #{stable_content_layer.fetch('surfaceAlpha')}
      mod.widgets.HEPTA_LAYER_STABLE_BLUR = #{Integer(stable_content_layer.fetch('blurPx'))}.0
      mod.widgets.HEPTA_LAYER_CHROME_ALPHA = #{glass_chrome_layer.fetch('surfaceAlpha')}
      mod.widgets.HEPTA_LAYER_CHROME_BLUR = #{Integer(glass_chrome_layer.fetch('blurPx'))}.0
      mod.widgets.HEPTA_LAYER_FLOATING_ALPHA = #{floating_glass_layer.fetch('surfaceAlpha')}
      mod.widgets.HEPTA_LAYER_FLOATING_BLUR = #{Integer(floating_glass_layer.fetch('blurPx'))}.0
  }

  pub const COLOR_HEPTA_CONTENT: Vec4 = #{vec4_color(native.fetch('content'))};
  pub const HEPTA_TOUCH_TARGET: f64 = #{native_minimum_touch_target}.0;
  pub const COLOR_HEPTA_FOCUS: Vec4 = #{vec4_color(native_focus)};
  pub const COLOR_HEPTA_FOCUS_HOVER: Vec4 = #{vec4_color(native.fetch('focusHover'))};
  pub const COLOR_HEPTA_SUCCESS: Vec4 = #{vec4_color(native.fetch('success'))};
RUST

outputs = { CONTROL_OUTPUT => css, NATIVE_OUTPUT => native_rust }
if mode == :check
  stale = outputs.each_with_object([]) do |(path, expected), entries|
    next if path.file? && path.binread == expected
    entries << {
      "path" => path.relative_path_from(ROOT).to_s,
      "exists" => path.file?,
      "actual_bytes" => path.file? ? path.size : 0,
      "expected_bytes" => expected.bytesize,
    }
  end
  unless stale.empty?
    warn JSON.pretty_generate({ status: "not_ready", reason: "generated_tokens_stale", stale: stale })
    exit 1
  end
  binding_after = source_binding
  puts JSON.generate({ kind: "hepta-ui-light-glass-token-sync", status: "ready", mode: "check", schema_version: 3,
                       source_binding_before: binding_before, source_binding: binding_after,
                       source_stable_during_run: binding_equal?(binding_before, binding_after),
                       outputs: outputs.keys.map { |path| path.relative_path_from(ROOT).to_s } })
else
  outputs.each do |path, content|
    path.dirname.mkpath
    path.binwrite(content)
  end
  binding_after = source_binding
  puts JSON.generate({ kind: "hepta-ui-light-glass-token-sync", status: "ready", mode: "write", schema_version: 3,
                       source_binding_before: binding_before, source_binding: binding_after,
                       source_stable_during_run: binding_equal?(binding_before, binding_after),
                       outputs: outputs.keys.map { |path| path.relative_path_from(ROOT).to_s } })
end
