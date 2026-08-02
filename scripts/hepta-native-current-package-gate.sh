#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

BUILD=0
BOOTSTRAP_TOOLS=0
REPORT_PATH=""
STAGE_DIR=""
TARGET_DIR="${HEPTA_NATIVE_CARGO_TARGET_DIR:-${TMPDIR:-/tmp}/hepta-native-current-package-target}"
TOOLS_DIR="${HEPTA_NATIVE_PACKAGING_TOOLS_DIR:-${TMPDIR:-/tmp}/hepta-native-packaging-tools-v1}"
while [[ $# -gt 0 ]]; do
  case "$1" in
    --build) BUILD=1; shift ;;
    --bootstrap-tools) BOOTSTRAP_TOOLS=1; shift ;;
    --output) REPORT_PATH="${2:-}"; shift 2 ;;
    --stage-dir) STAGE_DIR="${2:-}"; shift 2 ;;
    --target-dir) TARGET_DIR="${2:-}"; shift 2 ;;
    --tools-dir) TOOLS_DIR="${2:-}"; shift 2 ;;
    --help|-h)
      cat <<'EOF'
usage: scripts/hepta-native-current-package-gate.sh [--build] [--bootstrap-tools]
       [--output report.json] [--stage-dir directory] [--target-dir directory]
       [--tools-dir directory]

Without --build the gate validates source and package metadata and reports the
local package boundary as not_ready. --build creates and launch-probes a formal,
resource-complete, current-source unsigned Hepta.app using cargo-packager and
Robius. It never signs, notarizes, staples, uploads, or publishes an artifact.
--bootstrap-tools installs exact tool versions under --tools-dir, never globally.
EOF
      exit 0
      ;;
    *) echo "unknown argument: $1" >&2; exit 64 ;;
  esac
done

if [[ -n "$REPORT_PATH" ]]; then mkdir -p "$(dirname "$REPORT_PATH")"; fi
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
  for command in file codesign otool grep plutil open pgrep; do
    command -v "$command" >/dev/null 2>&1 || { echo "$command is required for a Darwin app build" >&2; exit 2; }
  done
fi

# shellcheck source=scripts/lib/hepta-ui-rust-toolchain.sh
source scripts/lib/hepta-ui-rust-toolchain.sh
hepta_ui_activate_rust_toolchain

APP_DIR="apps/hepta-native"
PLIST="$APP_DIR/packaging/Info.plist"
SOURCE_ICON="$APP_DIR/packaging/HeptaNative.icns"
ENTITLEMENTS="$APP_DIR/packaging/Entitlements.plist"
DESKTOP_ENTRY="$APP_DIR/packaging/hepta-native.desktop"
APPSTREAM="$APP_DIR/packaging/ai.hepta.nativeapp.metainfo.xml"
UNSIGNED_SCRIPT="$APP_DIR/packaging/build-macos-unsigned-app.sh"
MACOS_RELEASE_SCRIPT="$APP_DIR/packaging/build-macos-dmg.sh"
IOS_SCRIPT="$APP_DIR/packaging/build-ios-testflight.sh"

for required in "$APP_DIR/Cargo.toml" "$APP_DIR/Cargo.lock" "$PLIST" "$SOURCE_ICON" "$ENTITLEMENTS" \
  "$DESKTOP_ENTRY" "$APPSTREAM" "$UNSIGNED_SCRIPT" "$MACOS_RELEASE_SCRIPT" "$IOS_SCRIPT"; do
  [[ -s "$required" ]] || { echo "missing current package input: $required" >&2; exit 1; }
done

bash -n "$UNSIGNED_SCRIPT" "$MACOS_RELEASE_SCRIPT" "$IOS_SCRIPT"
ruby -r rexml/document -e 'ARGV.each { |path| REXML::Document.new(File.binread(path)) }' "$PLIST" "$ENTITLEMENTS"
metadata="$(hepta_ui_cargo metadata --manifest-path "$APP_DIR/Cargo.toml" --locked --offline --no-deps --format-version 1)"
package_metadata="$(jq -c '.packages[] | select(.name == "hepta-native")' <<<"$metadata")"

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
    puts(array ? REXML::XPath.match(array, "dict/array/string").map(&:text) : [])
  ' "$1"
}

directory_metrics() {
  ruby -r digest -r json -e '
    root = File.expand_path(ARGV.fetch(0))
    exclude_mobile = ARGV[1] == "exclude-mobile"
    files = Dir.glob(File.join(root, "**", "*"), File::FNM_DOTMATCH)
      .select { |path| File.file?(path) }
      .map { |path| [path.delete_prefix(root + "/"), path] }
      .reject { |relative, _| exclude_mobile && (relative.start_with?("android/") || relative.start_with?("ios/")) }
      .sort_by(&:first)
    digest = Digest::SHA256.new
    bytes = 0
    files.each do |relative, path|
      size = File.size(path)
      bytes += size
      digest << relative << "\0" << size.to_s << "\0" << Digest::SHA256.file(path).hexdigest << "\n"
    end
    print JSON.generate({file_count: files.length, bytes: bytes, manifest_sha256: digest.hexdigest})
  ' "$1" "${2:-}"
}

bundle_id="$(plist_value "$PLIST" CFBundleIdentifier)"
bundle_name="$(plist_value "$PLIST" CFBundleName)"
bundle_display_name="$(plist_value "$PLIST" CFBundleDisplayName)"
bundle_executable="$(plist_value "$PLIST" CFBundleExecutable)"
bundle_icon="$(plist_value "$PLIST" CFBundleIconFile)"
bundle_type="$(plist_value "$PLIST" CFBundlePackageType)"
source_requires_carbon="$(plutil -extract LSRequiresCarbon raw "$PLIST" 2>/dev/null || true)"
url_schemes="$(plist_url_schemes "$PLIST")"
url_hepta="$(printf '%s\n' "$url_schemes" | sed -n '1p')"
url_matrix="$(printf '%s\n' "$url_schemes" | sed -n '2p')"
product_name="$(jq -r '.metadata.packager.product_name // empty' <<<"$package_metadata")"
package_version="$(jq -r '.version' <<<"$package_metadata")"
packager_version="$(jq -r '.metadata["hepta-packaging"].cargo_packager_version // empty' <<<"$package_metadata")"
robius_packaging_version="$(jq -r '.metadata["hepta-packaging"].robius_packaging_commands_version // empty' <<<"$package_metadata")"
packaging_rust_toolchain="$(jq -r '.metadata["hepta-packaging"].rust_toolchain // empty' <<<"$package_metadata")"
formal_script_metadata="$(jq -r '.metadata["hepta-packaging"].formal_unsigned_macos_script // empty' <<<"$package_metadata")"
declared_resources="$(jq -c '.metadata.packager.resources // []' <<<"$package_metadata")"
declared_resource_count="$(jq 'length' <<<"$declared_resources")"
declared_resource_targets_ready=false
if jq -e '
  length == 2
  and ([.[].target] | sort) == ["hepta-native", "makepad_widgets"]
  and all(.[]; (.src | startswith("./dist/resources/")))
' >/dev/null <<<"$declared_resources"; then
  declared_resource_targets_ready=true
fi

static_ready=false
if [[ "$bundle_id" == "ai.hepta.nativeapp" && "$bundle_name" == "Hepta" && "$bundle_display_name" == "Hepta" \
  && "$bundle_executable" == "hepta-native" && "$bundle_icon" == "Hepta.icns" && "$bundle_type" == "APPL" \
  && "$source_requires_carbon" == "false" \
  && "$url_hepta" == "hepta-native" && "$url_matrix" == "matrix" && "$product_name" == "Hepta" \
  && "$packager_version" == "0.11.8" && "$robius_packaging_version" == "0.3.3" \
  && "$packaging_rust_toolchain" == "1.95.0" \
  && "$formal_script_metadata" == "./packaging/build-macos-unsigned-app.sh" \
  && "$declared_resource_targets_ready" == "true" ]]; then
  static_ready=true
fi

source_binding_before="$(scripts/hepta-ui-source-fingerprint)"
artifact_json='null'
resource_reports='[]'
collector_source_reports='[]'
formal_pipeline_exit_code=-1
formal_unsigned_packaging_pipeline_ready=false
resources_complete=false
collector_sources_complete=false
artifact_head_embedded=false
binary_rpath_ready=false
bundle_unsigned=false
linker_adhoc_signature=false
launch_probe_ready=false
launch_probe_seconds=0
launch_exit_code='null'
launch_via_launch_services=false
build_log=""
launch_log=""

if [[ "$BUILD" == "1" && "$darwin_build_supported" == "true" ]]; then
  mkdir -p "$STAGE_DIR"
  build_log="$STAGE_DIR/formal-unsigned-build.log"
  build_args=(--stage-dir "$STAGE_DIR" --target-dir "$TARGET_DIR" --tools-dir "$TOOLS_DIR")
  if [[ "$BOOTSTRAP_TOOLS" == "1" ]]; then build_args+=(--bootstrap-tools); fi
  set +e
  "$UNSIGNED_SCRIPT" "${build_args[@]}" >"$build_log" 2>&1
  formal_pipeline_exit_code=$?
  set -e

  app_bundle="$STAGE_DIR/Hepta.app"
  collected_resources="$STAGE_DIR/collected-resources"
  binary="$app_bundle/Contents/MacOS/hepta-native"
  bundled_plist="$app_bundle/Contents/Info.plist"

  if [[ "$formal_pipeline_exit_code" == "0" && -x "$binary" && -s "$bundled_plist" && -d "$collected_resources" ]]; then
    resources_complete=true
    while IFS=$'\t' read -r src target; do
      relative_src="${src#./dist/resources/}"
      collected_dir="$collected_resources/$relative_src"
      bundled_dir="$app_bundle/Contents/Resources/$target"
      collected_metrics='null'
      bundled_metrics='null'
      exact=false
      if [[ "$relative_src" != "$src" && -d "$collected_dir" && -d "$bundled_dir" ]]; then
        collected_metrics="$(directory_metrics "$collected_dir")"
        bundled_metrics="$(directory_metrics "$bundled_dir")"
        if [[ "$(jq -r '.file_count' <<<"$collected_metrics")" -gt 0 && "$collected_metrics" == "$bundled_metrics" ]]; then exact=true; fi
      fi
      [[ "$exact" == "true" ]] || resources_complete=false
      resource_reports="$(jq -c \
        --arg src "$src" --arg target "$target" \
        --argjson collected "$collected_metrics" --argjson bundled "$bundled_metrics" --argjson exact "$exact" \
        '. + [{declared_source:$src,bundle_target:$target,collected:$collected,bundled:$bundled,byte_exact:$exact}]' \
        <<<"$resource_reports")"
    done < <(jq -r '.[] | [.src, .target] | @tsv' <<<"$declared_resources")

    # Independently bind Robius' collected trees back to their canonical source
    # directories, not merely to cargo-packager's second copy.
    canonical_app_metrics="$(directory_metrics "$APP_DIR/resources" exclude-mobile)"
    collected_app_metrics='null'
    app_collector_exact=false
    if [[ -d "$collected_resources/hepta-native/resources" ]]; then
      collected_app_metrics="$(directory_metrics "$collected_resources/hepta-native/resources")"
      if [[ "$canonical_app_metrics" == "$collected_app_metrics" ]]; then app_collector_exact=true; fi
    fi
    collector_source_reports="$(jq -c --arg kind app_resources --argjson source "$canonical_app_metrics" \
      --argjson collected "$collected_app_metrics" --argjson exact "$app_collector_exact" \
      '. + [{kind:$kind,source:$source,collected:$collected,byte_exact:$exact}]' <<<"$collector_source_reports")"
    collector_sources_complete="$app_collector_exact"

    while IFS=$'\t' read -r target; do
      makepad_crate="${target#makepad_}"
      makepad_crate="${makepad_crate//_/-}"
      path_file="$TARGET_DIR/release/makepad-$makepad_crate.path"
      source_metrics='null'
      collected_metrics='null'
      exact=false
      if [[ -s "$path_file" ]]; then
        makepad_source="$(tr -d '\r\n' <"$path_file")/resources"
        collected_makepad="$collected_resources/$target/resources"
        if [[ -d "$makepad_source" && -d "$collected_makepad" ]]; then
          source_metrics="$(directory_metrics "$makepad_source")"
          collected_metrics="$(directory_metrics "$collected_makepad")"
          if [[ "$(jq -r '.file_count' <<<"$source_metrics")" -gt 0 && "$source_metrics" == "$collected_metrics" ]]; then exact=true; fi
        fi
      fi
      [[ "$exact" == "true" ]] || collector_sources_complete=false
      collector_source_reports="$(jq -c --arg kind "$target" --argjson source "$source_metrics" \
        --argjson collected "$collected_metrics" --argjson exact "$exact" \
        '. + [{kind:$kind,source:$source,collected:$collected,byte_exact:$exact}]' <<<"$collector_source_reports")"
    done < <(jq -r '.[] | .target | select(startswith("makepad_"))' <<<"$declared_resources")

    expected_head="$(jq -r '.head' <<<"$source_binding_before")"
    if LC_ALL=C grep -aFq -- "$expected_head" "$binary"; then artifact_head_embedded=true; fi
    if otool -l "$binary" | grep -Fq '@executable_path/../Frameworks'; then binary_rpath_ready=true; fi
    codesign_details="$(codesign -dv --verbose=4 "$app_bundle" 2>&1 || true)"
    if grep -Fq 'Signature=adhoc' <<<"$codesign_details" \
      && grep -Fq 'TeamIdentifier=not set' <<<"$codesign_details" \
      && ! grep -Fq 'Authority=' <<<"$codesign_details" \
      && [[ ! -e "$app_bundle/Contents/_CodeSignature" ]]; then
      linker_adhoc_signature=true
      bundle_unsigned=true
    fi
    bundle_requires_carbon="$(plutil -extract LSRequiresCarbon raw "$bundled_plist" 2>/dev/null || true)"

    launch_log="$STAGE_DIR/launch-probe.log"
    launch_stderr="$STAGE_DIR/launch-probe.stderr.log"
    launch_home="$STAGE_DIR/launch-home"
    mkdir -p "$launch_home"
    start_epoch="$(date +%s)"
    set +e
    env -u APPLE_ID -u APPLE_PASSWORD -u APPLE_TEAM_ID \
      -u APPLE_CERTIFICATE -u APPLE_CERTIFICATE_PASSWORD \
      open -n -g \
        --env "HOME=$launch_home" --env "TMPDIR=$launch_home" \
        --env "HTTPS_PROXY=http://127.0.0.1:9" --env "HTTP_PROXY=http://127.0.0.1:9" \
        --env "ALL_PROXY=http://127.0.0.1:9" --env "NO_PROXY=localhost,127.0.0.1" \
        -o "$launch_log" --stderr "$launch_stderr" "$app_bundle"
    open_exit_code=$?
    set -e
    launch_pid=""
    if [[ "$open_exit_code" == "0" ]]; then
      for _ in {1..40}; do
        launch_pid="$(pgrep -f "$binary" | head -n 1 || true)"
        [[ -z "$launch_pid" ]] || break
        sleep 0.25
      done
    fi
    if [[ -n "$launch_pid" ]]; then launch_via_launch_services=true; fi
    for _ in {1..20}; do
      if [[ -z "$launch_pid" ]] || ! kill -0 "$launch_pid" >/dev/null 2>&1; then break; fi
      sleep 0.25
    done
    launch_probe_seconds=$(( $(date +%s) - start_epoch ))
    if [[ -n "$launch_pid" ]] && kill -0 "$launch_pid" >/dev/null 2>&1; then
      launch_probe_ready=true
      kill -TERM "$launch_pid" >/dev/null 2>&1 || true
      for _ in {1..20}; do
        if ! kill -0 "$launch_pid" >/dev/null 2>&1; then break; fi
        sleep 0.1
      done
      kill -KILL "$launch_pid" >/dev/null 2>&1 || true
    elif [[ -n "$launch_pid" ]]; then
      launch_exit_code=1
    fi

    binary_kind="$(file -b "$binary")"
    binary_sha="$(shasum -a 256 "$binary" | awk '{print $1}')"
    plist_sha="$(shasum -a 256 "$bundled_plist" | awk '{print $1}')"
    icon_path="$app_bundle/Contents/Resources/Hepta.icns"
    icon_sha=""
    [[ ! -s "$icon_path" ]] || icon_sha="$(shasum -a 256 "$icon_path" | awk '{print $1}')"
    bundle_bytes="$(find "$app_bundle" -type f -exec wc -c {} \; | awk '{sum += $1} END {print sum + 0}')"
    artifact_json="$(jq -n \
      --arg path "$app_bundle" --arg binary_kind "$binary_kind" --arg binary_sha256 "$binary_sha" \
      --arg info_plist_sha256 "$plist_sha" --arg icon_sha256 "$icon_sha" \
      --argjson bytes "$bundle_bytes" --argjson head_embedded "$artifact_head_embedded" \
      --arg expected_head "$expected_head" --argjson rpath_ready "$binary_rpath_ready" \
      --argjson unsigned "$bundle_unsigned" --argjson linker_adhoc "$linker_adhoc_signature" \
      --arg bundle_requires_carbon "$bundle_requires_carbon" --argjson launch_services "$launch_via_launch_services" \
      --arg build_log "$build_log" --arg launch_log "$launch_log" --arg launch_stderr "$launch_stderr" \
      '{probe_type:"formal_cargo_packager_robius_unsigned_app",path:$path,bytes:$bytes,binary_kind:$binary_kind,binary_sha256:$binary_sha256,info_plist_sha256:$info_plist_sha256,icon_sha256:$icon_sha256,expected_head:$expected_head,full_head_embedded:$head_embedded,makepad_bundle_rpath_ready:$rpath_ready,ls_requires_carbon:($bundle_requires_carbon == "true"),bundle_signature:(if $unsigned then "linker_adhoc_no_developer_id" else "unexpected_developer_identity" end),linker_adhoc_signature:$linker_adhoc,developer_id_signed:false,launch_via_launch_services:$launch_services,build_log:$build_log,launch_log:$launch_log,launch_stderr_log:$launch_stderr}')"

    if [[ "$static_ready" == "true" && "$resources_complete" == "true" \
      && "$collector_sources_complete" == "true" && "$artifact_head_embedded" == "true" \
      && "$binary_rpath_ready" == "true" && "$bundle_unsigned" == "true" \
      && "$bundle_requires_carbon" == "false" && "$launch_via_launch_services" == "true" \
      && "$launch_probe_ready" == "true" && "$binary_kind" == Mach-O\ *executable* \
      && "$bundle_bytes" -gt 1000000 && -n "$icon_sha" ]]; then
      formal_unsigned_packaging_pipeline_ready=true
    fi
  fi
fi

source_binding="$(scripts/hepta-ui-source-fingerprint)"
source_stable=false
if [[ "$(jq -r '.head' <<<"$source_binding_before")" == "$(jq -r '.head' <<<"$source_binding")" \
  && "$(jq -r '.head_tree' <<<"$source_binding_before")" == "$(jq -r '.head_tree' <<<"$source_binding")" \
  && "$(jq -r '.source_fingerprint' <<<"$source_binding_before")" == "$(jq -r '.source_fingerprint' <<<"$source_binding")" ]]; then
  source_stable=true
fi

current_source_build_ready=false
local_package_ready=false
status="not_ready"
if [[ "$formal_unsigned_packaging_pipeline_ready" == "true" && "$source_stable" == "true" \
  && "$(jq -r '.worktree_clean' <<<"$source_binding")" == "true" ]]; then
  current_source_build_ready=true
  local_package_ready=true
  status="ready"
fi

report="$(jq -n \
  --arg status "$status" --argjson source_binding "$source_binding" --argjson source_binding_before "$source_binding_before" \
  --argjson source_stable "$source_stable" --arg rust_toolchain "$(hepta_ui_rustc --version)" \
  --arg platform "$platform" --argjson darwin_build_supported "$darwin_build_supported" \
  --arg bundle_id "$bundle_id" --arg bundle_name "$bundle_name" --arg bundle_display_name "$bundle_display_name" \
  --arg bundle_executable "$bundle_executable" --arg bundle_icon "$bundle_icon" --arg bundle_type "$bundle_type" \
  --arg source_requires_carbon "$source_requires_carbon" \
  --arg url_hepta "$url_hepta" --arg url_matrix "$url_matrix" --arg product_name "$product_name" \
  --arg package_version "$package_version" --arg packager_version "$packager_version" \
  --arg robius_packaging_version "$robius_packaging_version" --arg tools_dir "$TOOLS_DIR" \
  --argjson declared_resource_count "$declared_resource_count" \
  --argjson declared_resource_targets_ready "$declared_resource_targets_ready" --argjson static_ready "$static_ready" \
  --argjson build_requested "$BUILD" --argjson bootstrap_tools "$BOOTSTRAP_TOOLS" \
  --argjson formal_pipeline_exit_code "$formal_pipeline_exit_code" \
  --argjson formal_ready "$formal_unsigned_packaging_pipeline_ready" \
  --argjson resources_complete "$resources_complete" --argjson collector_sources_complete "$collector_sources_complete" \
  --argjson resource_reports "$resource_reports" --argjson collector_source_reports "$collector_source_reports" \
  --argjson launch_probe_ready "$launch_probe_ready" --argjson launch_probe_seconds "$launch_probe_seconds" \
  --argjson launch_via_launch_services "$launch_via_launch_services" \
  --argjson launch_exit_code "$launch_exit_code" --argjson current_source_build_ready "$current_source_build_ready" \
  --argjson local_package_ready "$local_package_ready" --argjson artifact "$artifact_json" \
  '{
    # Keep schema v1 for the current-readiness consumer; all formal packaging
    # evidence fields added here are backwards-compatible extensions.
    schema_version:1,
    kind:"hepta-native-current-package-gate",
    status:$status,
    source_binding:$source_binding,
    source_binding_before:$source_binding_before,
    source_stable_during_run:$source_stable,
    rust_toolchain:$rust_toolchain,
    host_platform:$platform,
    darwin_app_build_supported:$darwin_build_supported,
    package_metadata:{product_name:$product_name,version:$package_version,bundle_identifier:$bundle_id,bundle_name:$bundle_name,bundle_display_name:$bundle_display_name,bundle_executable:$bundle_executable,bundle_icon_file:$bundle_icon,bundle_package_type:$bundle_type,ls_requires_carbon:($source_requires_carbon == "true"),url_schemes:[$url_hepta,$url_matrix]},
    packaging_tools:{cargo_packager_version:$packager_version,robius_packaging_commands_version:$robius_packaging_version,temporary_tools_root:$tools_dir,bootstrap_requested:($bootstrap_tools == 1)},
    static_package_contract_ready:$static_ready,
    build_requested:($build_requested == 1),
    formal_pipeline_exit_code:$formal_pipeline_exit_code,
    declared_resource_tree_count:$declared_resource_count,
    declared_resource_targets_ready:$declared_resource_targets_ready,
    declared_resources_byte_exact:$resources_complete,
    collector_sources_byte_exact:$collector_sources_complete,
    resource_trees:$resource_reports,
    collector_source_trees:$collector_source_reports,
    artifact_build_probe_ready:$formal_ready,
    binary_bundle_probe_ready:$formal_ready,
    formal_unsigned_packaging_pipeline_ready:$formal_ready,
    current_source_build_ready:$current_source_build_ready,
    staged_app_launch_verified:$launch_probe_ready,
    launch_via_launch_services:$launch_via_launch_services,
    launch_probe_seconds:$launch_probe_seconds,
    launch_exit_code:$launch_exit_code,
    local_package_ready:$local_package_ready,
    artifact:$artifact,
    signed:false,
    notarized:false,
    stapled:false,
    public_distribution_ready:false,
    public_ga_ready:false,
    external_side_effects_performed:false,
    blockers:([if $source_binding.worktree_clean then empty else "ui_source_worktree_dirty" end,if $source_stable then empty else "ui_source_changed_during_package_gate" end,if $static_ready then empty else "package_metadata_contract_not_ready" end,if (($build_requested == 1) and ($darwin_build_supported == false)) then "darwin_app_build_not_supported_on_host" else empty end,if ($build_requested == 1) then (if $formal_pipeline_exit_code == 0 then empty else "formal_unsigned_packaging_command_failed" end) else "formal_unsigned_packaging_not_requested" end,if $resources_complete then empty else "declared_package_resources_not_byte_exact" end,if $collector_sources_complete then empty else "robius_collector_sources_not_byte_exact" end,if $formal_ready then empty else "formal_unsigned_packaging_pipeline_not_ready" end,if $launch_via_launch_services then empty else "launch_services_bundle_launch_not_verified" end,if $launch_probe_ready then empty else "staged_app_launch_not_verified" end,"developer_id_signing_not_performed","notarization_not_performed","stapling_not_performed","public_distribution_not_authorized"])
  }')"

if [[ -n "$REPORT_PATH" ]]; then printf '%s\n' "$report" >"$REPORT_PATH"; else printf '%s\n' "$report"; fi
[[ "$local_package_ready" == "true" ]]
