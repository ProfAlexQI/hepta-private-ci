#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

BUILD=0
REPORT_PATH=""
STAGE_DIR=""
TARGET_DIR="${HEPTA_NATIVE_CARGO_TARGET_DIR:-${TMPDIR:-/tmp}/hepta-native-current-package-target}"
while [[ $# -gt 0 ]]; do
  case "$1" in
    --build) BUILD=1; shift ;;
    --output) REPORT_PATH="${2:-}"; shift 2 ;;
    --stage-dir) STAGE_DIR="${2:-}"; shift 2 ;;
    --target-dir) TARGET_DIR="${2:-}"; shift 2 ;;
    --help|-h)
      cat <<'EOF'
usage: scripts/hepta-native-current-package-gate.sh [--build] [--output report.json]
       [--stage-dir directory] [--target-dir directory]

Without --build the gate validates source and package metadata and reports the
local package boundary as not_ready. --build creates a current-source unsigned
Hepta.app in the evidence stage directory; it never signs, notarizes, staples,
uploads, or publishes an artifact.
EOF
      exit 0
      ;;
    *) echo "unknown argument: $1" >&2; exit 64 ;;
  esac
done

if [[ -n "$REPORT_PATH" ]]; then
  mkdir -p "$(dirname "$REPORT_PATH")"
fi
if [[ -z "$STAGE_DIR" ]]; then
  if [[ -n "$REPORT_PATH" ]]; then
    STAGE_DIR="$(dirname "$REPORT_PATH")/native-current-package"
  else
    STAGE_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-native-current-package.XXXXXX")"
  fi
fi

for command in git jq shasum ruby bash uname; do
  command -v "$command" >/dev/null 2>&1 || { echo "$command is required" >&2; exit 2; }
done
platform="$(uname -s)"
darwin_build_supported=false
if [[ "$platform" == "Darwin" ]]; then
  darwin_build_supported=true
  for command in file codesign strings; do
    command -v "$command" >/dev/null 2>&1 || { echo "$command is required for a Darwin app build" >&2; exit 2; }
  done
fi

# shellcheck source=scripts/lib/hepta-ui-rust-toolchain.sh
source scripts/lib/hepta-ui-rust-toolchain.sh
hepta_ui_activate_rust_toolchain

APP_DIR="apps/hepta-native"
PLIST="$APP_DIR/packaging/Info.plist"
ICON="$APP_DIR/packaging/HeptaNative.icns"
ENTITLEMENTS="$APP_DIR/packaging/Entitlements.plist"
DESKTOP_ENTRY="$APP_DIR/packaging/hepta-native.desktop"
APPSTREAM="$APP_DIR/packaging/ai.hepta.nativeapp.metainfo.xml"
MACOS_SCRIPT="$APP_DIR/packaging/build-macos-dmg.sh"
IOS_SCRIPT="$APP_DIR/packaging/build-ios-testflight.sh"

for required in "$APP_DIR/Cargo.toml" "$APP_DIR/Cargo.lock" "$PLIST" "$ICON" "$ENTITLEMENTS" \
  "$DESKTOP_ENTRY" "$APPSTREAM" "$MACOS_SCRIPT" "$IOS_SCRIPT"; do
  [[ -s "$required" ]] || { echo "missing current package input: $required" >&2; exit 1; }
done

bash -n "$MACOS_SCRIPT" "$IOS_SCRIPT"
ruby -r rexml/document -e 'ARGV.each { |path| REXML::Document.new(File.binread(path)) }' "$PLIST" "$ENTITLEMENTS"
hepta_ui_cargo metadata --manifest-path "$APP_DIR/Cargo.toml" --locked --offline --no-deps --format-version 1 >/dev/null

plist_value() {
  ruby -r rexml/document -e '
    document = REXML::Document.new(File.binread(ARGV[0]))
    dictionary = REXML::XPath.first(document, "/plist/dict")
    key = dictionary&.elements&.to_a&.find { |element| element.name == "key" && element.text == ARGV[1] }
    value = key&.next_element
    print(value&.text.to_s)
  ' "$1" "$2"
}
plist_url_schemes() {
  ruby -r rexml/document -e '
    document = REXML::Document.new(File.binread(ARGV[0]))
    dictionary = REXML::XPath.first(document, "/plist/dict")
    key = dictionary&.elements&.to_a&.find { |element| element.name == "key" && element.text == "CFBundleURLTypes" }
    array = key&.next_element
    schemes_key = REXML::XPath.first(array, "dict/key[.=\"CFBundleURLSchemes\"]")
    schemes = schemes_key&.next_element
    puts(schemes ? schemes.get_elements("string").map(&:text) : [])
  ' "$1"
}
bundle_id="$(plist_value "$PLIST" CFBundleIdentifier)"
bundle_name="$(plist_value "$PLIST" CFBundleName)"
bundle_display_name="$(plist_value "$PLIST" CFBundleDisplayName)"
bundle_executable="$(plist_value "$PLIST" CFBundleExecutable)"
bundle_icon="$(plist_value "$PLIST" CFBundleIconFile)"
bundle_type="$(plist_value "$PLIST" CFBundlePackageType)"
mapfile_compat="$(plist_url_schemes "$PLIST")"
url_hepta="$(printf '%s\n' "$mapfile_compat" | sed -n '1p')"
url_matrix="$(printf '%s\n' "$mapfile_compat" | sed -n '2p')"
product_name="$(sed -n 's/^product_name[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "$APP_DIR/Cargo.toml" | head -1)"
package_version="$(hepta_ui_cargo metadata --manifest-path "$APP_DIR/Cargo.toml" --locked --offline --no-deps --format-version 1 | jq -r '.packages[] | select(.name=="hepta-native") | .version')"

static_ready=false
if [[ "$bundle_id" == "ai.hepta.nativeapp" && "$bundle_name" == "Hepta" && "$bundle_display_name" == "Hepta" \
  && "$bundle_executable" == "hepta-native" && "$bundle_icon" == "HeptaNative.icns" && "$bundle_type" == "APPL" \
  && "$url_hepta" == "hepta-native" && "$url_matrix" == "matrix" && "$product_name" == "Hepta" ]]; then
  static_ready=true
fi

source_binding_before="$(scripts/hepta-ui-source-fingerprint)"
artifact_json='null'
build_ready=false
artifact_head_embedded=false

if [[ "$BUILD" == "1" && "$darwin_build_supported" == "true" ]]; then
  mkdir -p "$TARGET_DIR" "$STAGE_DIR"
  CARGO_TARGET_DIR="$TARGET_DIR" hepta_ui_cargo build --manifest-path "$APP_DIR/Cargo.toml" --locked --offline --bin hepta-native
  binary="$TARGET_DIR/debug/hepta-native"
  [[ -x "$binary" ]] || { echo "missing built Native binary: $binary" >&2; exit 1; }

  app_bundle="$STAGE_DIR/Hepta.app"
  [[ ! -e "$app_bundle" ]] || { echo "refusing to replace existing evidence bundle: $app_bundle" >&2; exit 1; }
  mkdir -p "$app_bundle/Contents/MacOS" "$app_bundle/Contents/Resources"
  cp "$PLIST" "$app_bundle/Contents/Info.plist"
  cp "$ICON" "$app_bundle/Contents/Resources/HeptaNative.icns"
  cp "$binary" "$app_bundle/Contents/MacOS/hepta-native"
  chmod 755 "$app_bundle/Contents/MacOS/hepta-native"

  binary_kind="$(file -b "$app_bundle/Contents/MacOS/hepta-native")"
  expected_head="$(jq -r '.head' <<<"$source_binding_before")"
  if strings -a "$app_bundle/Contents/MacOS/hepta-native" | grep -Fq "$expected_head"; then
    artifact_head_embedded=true
  fi
  binary_sha="$(shasum -a 256 "$app_bundle/Contents/MacOS/hepta-native" | awk '{print $1}')"
  plist_sha="$(shasum -a 256 "$app_bundle/Contents/Info.plist" | awk '{print $1}')"
  icon_sha="$(shasum -a 256 "$app_bundle/Contents/Resources/HeptaNative.icns" | awk '{print $1}')"
  bundle_bytes="$(find "$app_bundle" -type f -exec wc -c {} \; | awk '{sum += $1} END {print sum + 0}')"
  codesign_status="unsigned"
  if codesign --verify --deep --strict "$app_bundle" >/dev/null 2>&1; then
    codesign_status="signed"
  fi
  if [[ "$binary_kind" == Mach-O\ *executable* && "$bundle_bytes" -gt 1000000 && "$artifact_head_embedded" == "true" ]]; then
    build_ready=true
  fi
  artifact_json="$(jq -n \
    --arg path "$app_bundle" --arg binary_kind "$binary_kind" --arg binary_sha256 "$binary_sha" \
    --arg info_plist_sha256 "$plist_sha" --arg icon_sha256 "$icon_sha" --arg codesign_status "$codesign_status" \
    --argjson bytes "$bundle_bytes" \
    --argjson head_embedded "$artifact_head_embedded" --arg expected_head "$expected_head" \
    '{probe_type:"binary_bundle_probe_not_formal_package",path:$path,bytes:$bytes,binary_kind:$binary_kind,binary_sha256:$binary_sha256,info_plist_sha256:$info_plist_sha256,icon_sha256:$icon_sha256,codesign_status:$codesign_status,expected_head:$expected_head,full_head_embedded:$head_embedded}')"
fi

source_binding="$(scripts/hepta-ui-source-fingerprint)"
worktree_clean="$(jq -r '.worktree_clean' <<<"$source_binding")"
source_stable=false
if [[ "$(jq -r '.head' <<<"$source_binding_before")" == "$(jq -r '.head' <<<"$source_binding")" \
  && "$(jq -r '.head_tree' <<<"$source_binding_before")" == "$(jq -r '.head_tree' <<<"$source_binding")" \
  && "$(jq -r '.source_fingerprint' <<<"$source_binding_before")" == "$(jq -r '.source_fingerprint' <<<"$source_binding")" ]]; then
  source_stable=true
fi
binary_bundle_probe_ready=false
if [[ "$build_ready" == "true" && "$source_stable" == "true" ]]; then
  binary_bundle_probe_ready=true
fi

# The probe above is intentionally not a Robius/cargo-packager product bundle:
# resources and launch behavior have not been proven. It cannot promote local
# package readiness until the formal unsigned packaging pipeline is wired here.
formal_unsigned_packaging_pipeline_ready=false
local_package_ready=false
status="not_ready"

report="$(jq -n \
  --arg status "$status" --argjson source_binding "$source_binding" --argjson source_binding_before "$source_binding_before" \
  --argjson source_stable "$source_stable" --arg rust_toolchain "$(hepta_ui_rustc --version)" \
  --arg platform "$platform" --argjson darwin_build_supported "$darwin_build_supported" \
  --arg bundle_id "$bundle_id" --arg bundle_name "$bundle_name" --arg bundle_display_name "$bundle_display_name" \
  --arg bundle_executable "$bundle_executable" --arg bundle_icon "$bundle_icon" --arg bundle_type "$bundle_type" \
  --arg url_hepta "$url_hepta" --arg url_matrix "$url_matrix" --arg product_name "$product_name" \
  --arg package_version "$package_version" --argjson static_ready "$static_ready" --argjson build_requested "$BUILD" \
  --argjson build_ready "$build_ready" --argjson binary_bundle_probe_ready "$binary_bundle_probe_ready" \
  --argjson local_package_ready "$local_package_ready" --argjson artifact "$artifact_json" \
  '{
    schema_version:1,
    kind:"hepta-native-current-package-gate",
    status:$status,
    source_binding:$source_binding,
    source_binding_before:$source_binding_before,
    source_stable_during_run:$source_stable,
    rust_toolchain:$rust_toolchain,
    host_platform:$platform,
    darwin_app_build_supported:$darwin_build_supported,
    package_metadata:{product_name:$product_name,version:$package_version,bundle_identifier:$bundle_id,bundle_name:$bundle_name,bundle_display_name:$bundle_display_name,bundle_executable:$bundle_executable,bundle_icon_file:$bundle_icon,bundle_package_type:$bundle_type,url_schemes:[$url_hepta,$url_matrix]},
    static_package_contract_ready:$static_ready,
    build_requested:($build_requested == 1),
    artifact_build_probe_ready:$build_ready,
    binary_bundle_probe_ready:$binary_bundle_probe_ready,
    formal_unsigned_packaging_pipeline_ready:false,
    current_source_build_ready:false,
    local_package_ready:$local_package_ready,
    artifact:$artifact,
    signed:false,
    notarized:false,
    stapled:false,
    public_distribution_ready:false,
    public_ga_ready:false,
    external_side_effects_performed:false,
    blockers:([if $source_binding.worktree_clean then empty else "ui_source_worktree_dirty" end, if $source_stable then empty else "ui_source_changed_during_package_gate" end, if $static_ready then empty else "package_metadata_contract_not_ready" end, if (($build_requested == 1) and ($darwin_build_supported == false)) then "darwin_app_build_not_supported_on_host" else empty end, if ($build_requested == 1) then (if $binary_bundle_probe_ready then empty else "binary_bundle_probe_not_ready" end) else "binary_bundle_probe_not_requested" end, "formal_unsigned_packaging_pipeline_not_run", "staged_app_launch_not_verified", "developer_id_signing_not_performed", "notarization_not_performed", "stapling_not_performed", "public_distribution_not_authorized"])
  }')"

if [[ -n "$REPORT_PATH" ]]; then printf '%s\n' "$report" >"$REPORT_PATH"; else printf '%s\n' "$report"; fi
[[ "$local_package_ready" == "true" ]]
