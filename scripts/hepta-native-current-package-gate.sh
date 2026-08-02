#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"
# shellcheck source=scripts/lib/hepta-safe-output-v1.sh
source scripts/lib/hepta-safe-output-v1.sh

BUILD=0
BOOTSTRAP_TOOLS=0
LAUNCH_PROBE=0
LAUNCH_PROBE_MODE="default_disabled"
REPORT_PATH=""
STAGE_DIR=""
TARGET_DIR="${HEPTA_NATIVE_CARGO_TARGET_DIR:-${TMPDIR:-/tmp}/hepta-native-current-package-target}"
TOOLS_DIR="${HEPTA_NATIVE_PACKAGING_TOOLS_DIR:-${TMPDIR:-/tmp}/hepta-native-packaging-tools-v1}"
while [[ $# -gt 0 ]]; do
  case "$1" in
    --build) BUILD=1; shift ;;
    --bootstrap-tools) BOOTSTRAP_TOOLS=1; shift ;;
    --launch-probe)
      if [[ "$LAUNCH_PROBE_MODE" == "explicit_disabled" ]]; then
        echo "--launch-probe and --no-launch are mutually exclusive" >&2
        exit 64
      fi
      LAUNCH_PROBE=1
      LAUNCH_PROBE_MODE="explicit_enabled"
      shift
      ;;
    --no-launch)
      if [[ "$LAUNCH_PROBE_MODE" == "explicit_enabled" ]]; then
        echo "--launch-probe and --no-launch are mutually exclusive" >&2
        exit 64
      fi
      LAUNCH_PROBE=0
      LAUNCH_PROBE_MODE="explicit_disabled"
      shift
      ;;
    --output) REPORT_PATH="${2:-}"; shift 2 ;;
    --stage-dir) STAGE_DIR="${2:-}"; shift 2 ;;
    --target-dir) TARGET_DIR="${2:-}"; shift 2 ;;
    --tools-dir) TOOLS_DIR="${2:-}"; shift 2 ;;
    --help|-h)
      cat <<'EOF'
usage: scripts/hepta-native-current-package-gate.sh [--build] [--bootstrap-tools]
       [--launch-probe | --no-launch]
       [--output report.json] [--stage-dir directory] [--target-dir directory]
       [--tools-dir directory]

Without --build the gate validates source and package metadata and reports the
local package boundary as not_ready. --build creates a formal, resource-complete,
current-source unsigned Hepta.app using cargo-packager and Robius. It does not
launch by default. --launch-probe opts into a sandboxed, force-login process-
startup diagnostic; --no-launch remains the explicit non-interactive release-
input lane. The independent window verifier remains responsible for launch,
focus, and local/window promotion evidence. This gate never signs, notarizes,
staples, uploads, or publishes an artifact.
--bootstrap-tools installs exact tool versions under --tools-dir, never globally.
EOF
      exit 0
      ;;
    *) echo "unknown argument: $1" >&2; exit 64 ;;
  esac
done

if [[ -n "$REPORT_PATH" ]]; then
  hepta_safe_output_resolve_file "$ROOT_DIR" "--output" "$REPORT_PATH" || exit $?
  REPORT_PATH="$HEPTA_SAFE_OUTPUT_PATH"
fi
if [[ -z "$STAGE_DIR" ]]; then
  if [[ -n "$REPORT_PATH" ]]; then
    STAGE_DIR="$(dirname "$REPORT_PATH")/native-current-package"
  else
    STAGE_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-native-current-package.XXXXXX")"
  fi
fi

hepta_safe_output_resolve_directory "$ROOT_DIR" "--stage-dir" "$STAGE_DIR" || exit $?
STAGE_DIR="$HEPTA_SAFE_OUTPUT_PATH"
hepta_safe_output_resolve_directory "$ROOT_DIR" "--target-dir" "$TARGET_DIR" || exit $?
TARGET_DIR="$HEPTA_SAFE_OUTPUT_PATH"
hepta_safe_output_resolve_directory "$ROOT_DIR" "--tools-dir" "$TOOLS_DIR" || exit $?
TOOLS_DIR="$HEPTA_SAFE_OUTPUT_PATH"
if [[ -n "$REPORT_PATH" ]]; then
  for producer_root in "$STAGE_DIR" "$TARGET_DIR" "$TOOLS_DIR" "$ROOT_DIR/apps/hepta-native"; do
    if hepta_safe_output_path_within "$REPORT_PATH" "$producer_root"; then
      hepta_safe_output_error "--output must not overlap producer source/stage/tool path: $producer_root"
      exit 64
    fi
  done
  hepta_safe_output_prepare_parent "$REPORT_PATH" || {
    echo "could not prepare canonical --output parent" >&2
    exit 64
  }
fi

for command in git jq shasum ruby swiftc bash uname; do
  command -v "$command" >/dev/null 2>&1 || { echo "$command is required" >&2; exit 2; }
done
platform="$(uname -s)"
darwin_build_supported=false
if [[ "$platform" == "Darwin" ]]; then
  darwin_build_supported=true
  for command in file codesign otool grep plutil ps; do
    command -v "$command" >/dev/null 2>&1 || { echo "$command is required for a Darwin app build" >&2; exit 2; }
  done
  if [[ "$LAUNCH_PROBE" == "1" && ! -x /usr/bin/sandbox-exec ]]; then
    echo "/usr/bin/sandbox-exec is required for --launch-probe" >&2
    exit 2
  fi
fi

# shellcheck source=scripts/lib/hepta-ui-rust-toolchain.sh
source scripts/lib/hepta-ui-rust-toolchain.sh
# shellcheck source=scripts/lib/hepta-process-identity-v1.sh
source scripts/lib/hepta-process-identity-v1.sh
hepta_ui_activate_rust_toolchain

APP_DIR="apps/hepta-native"
PLIST="$APP_DIR/packaging/Info.plist"
SOURCE_ICON="$APP_DIR/packaging/HeptaNative.icns"
ENTITLEMENTS="$APP_DIR/packaging/Entitlements.plist"
DESKTOP_ENTRY="$APP_DIR/packaging/hepta-native.desktop"
APPSTREAM="$APP_DIR/packaging/ai.hepta.nativeapp.metainfo.xml"
UNSIGNED_SCRIPT="$APP_DIR/packaging/build-macos-unsigned-app.sh"
MACOS_RELEASE_SCRIPT="$APP_DIR/packaging/build-macos-dmg.sh"
MACOS_DMG_CREATE_SCRIPT="$APP_DIR/packaging/create-macos-dmg-from-app.sh"
APP_BUNDLE_FINGERPRINT="$APP_DIR/packaging/app-bundle-fingerprint-v1.rb"
FINDER_BOOKMARK_RESOLVER="$APP_DIR/packaging/resolve-finder-bookmark-v1.swift"
IOS_SCRIPT="$APP_DIR/packaging/build-ios-testflight.sh"
WINDOW_SANDBOX_PROFILE_SCRIPT="scripts/hepta-ui-native-window-sandbox-profile"

for required in "$APP_DIR/Cargo.toml" "$APP_DIR/Cargo.lock" "$PLIST" "$SOURCE_ICON" "$ENTITLEMENTS" \
  "$DESKTOP_ENTRY" "$APPSTREAM" "$UNSIGNED_SCRIPT" "$MACOS_RELEASE_SCRIPT" "$MACOS_DMG_CREATE_SCRIPT" \
  "$APP_BUNDLE_FINGERPRINT" "$FINDER_BOOKMARK_RESOLVER" "$IOS_SCRIPT"; do
  [[ -s "$required" ]] || { echo "missing current package input: $required" >&2; exit 1; }
done

bash -n "$UNSIGNED_SCRIPT" "$MACOS_RELEASE_SCRIPT" "$MACOS_DMG_CREATE_SCRIPT" "$IOS_SCRIPT"
ruby -c "$APP_BUNDLE_FINGERPRINT" >/dev/null
swiftc -parse "$FINDER_BOOKMARK_RESOLVER"
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

single_artifact_release_contract_ready=false
if grep -Fq 'consumed_exact_formal_app:true' "$MACOS_RELEASE_SCRIPT" \
  && grep -Fq 'built_second_product_app:false' "$MACOS_RELEASE_SCRIPT" \
  && grep -Fq 'HEPTA_NATIVE_UNSIGNED_APP_RECEIPT_PATH' "$MACOS_RELEASE_SCRIPT" \
  && grep -Fq 'scripts/hepta-native-current-package-gate.sh' "$MACOS_RELEASE_SCRIPT" \
  && grep -Fq -- '--no-launch' "$MACOS_RELEASE_SCRIPT" \
  && grep -Fq 'create-macos-dmg-from-app.sh' "$MACOS_RELEASE_SCRIPT" \
  && grep -Fq 'app-bundle-fingerprint-v1.rb' "$MACOS_RELEASE_SCRIPT" \
  && grep -Fq 'resolve-finder-bookmark-v1.swift' "$MACOS_RELEASE_SCRIPT" \
  && grep -Fq 'dmg_mounted_read_only:true' "$MACOS_RELEASE_SCRIPT" \
  && grep -Fq 'formal_app_contains_symlinks_or_unsupported_entries' "$MACOS_RELEASE_SCRIPT" \
  && ! grep -Eq 'cargo[[:space:]]+packager|cargo-packager[^[:alnum:]_-].*--release' "$MACOS_RELEASE_SCRIPT" \
  && grep -Fq 'never builds' "$MACOS_DMG_CREATE_SCRIPT"; then
  single_artifact_release_contract_ready=true
fi

static_ready=false
if [[ "$bundle_id" == "ai.hepta.nativeapp" && "$bundle_name" == "Hepta" && "$bundle_display_name" == "Hepta" \
  && "$bundle_executable" == "hepta-native" && "$bundle_icon" == "Hepta.icns" && "$bundle_type" == "APPL" \
  && "$source_requires_carbon" == "false" \
  && "$url_hepta" == "hepta-native" && "$url_matrix" == "matrix" && "$product_name" == "Hepta" \
  && "$packager_version" == "0.11.8" && "$robius_packaging_version" == "0.3.3" \
  && "$packaging_rust_toolchain" == "1.95.0" \
  && "$formal_script_metadata" == "./packaging/build-macos-unsigned-app.sh" \
  && "$declared_resource_targets_ready" == "true" \
  && "$single_artifact_release_contract_ready" == "true" ]]; then
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
launch_probe_executed=false
launch_process_started=false
launch_probe_seconds=0
launch_exit_code='null'
launch_via_launch_services=false
launch_pid='null'
launch_executable=""
launch_command=""
launch_spawn_start_token=""
launch_start_token=""
launch_pid_identity_verified=false
launch_pid_revalidated_before_term=false
launch_pid_revalidated_before_kill=false
launch_terminated_by_gate=false
launch_kill_escalation_performed=false
launch_cleanup_confirmed=false
launch_cleanup_exit_code='null'
launch_force_login_argument=false
launch_sandbox_profile=""
launch_sandbox_profile_applied=false
launch_home=""
launch_home_isolated=false
launch_real_product_data_path_denied=false
launch_real_product_cache_path_denied=false
launch_network_denied_by_sandbox=false
launch_keychain_services_denied=false
build_log=""
launch_log=""
launch_stderr=""
bundle_fingerprint='null'

launch_pid_identity_matches() {
  local candidate_pid="$1"
  local expected_start_token="$2"
  local expected_command="$3"
  hepta_process_identity_matches "$candidate_pid" "$expected_start_token" "$expected_command"
}

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
    if otool_details="$(otool -l "$binary")" \
      && grep -Fq '@executable_path/../Frameworks' <<<"$otool_details"; then
      binary_rpath_ready=true
    fi
    codesign_details="$(codesign -dv --verbose=4 "$app_bundle" 2>&1 || true)"
    if grep -Fq 'Signature=adhoc' <<<"$codesign_details" \
      && grep -Fq 'TeamIdentifier=not set' <<<"$codesign_details" \
      && ! grep -Fq 'Authority=' <<<"$codesign_details" \
      && [[ ! -e "$app_bundle/Contents/_CodeSignature" ]]; then
      linker_adhoc_signature=true
      bundle_unsigned=true
    fi
    bundle_requires_carbon="$(plutil -extract LSRequiresCarbon raw "$bundled_plist" 2>/dev/null || true)"

    if [[ "$LAUNCH_PROBE" == "1" ]]; then
      launch_log="$STAGE_DIR/launch-probe.log"
      launch_stderr="$STAGE_DIR/launch-probe.stderr.log"
      launch_sandbox_root="$STAGE_DIR/launch-sandbox"
      mkdir -p "$launch_sandbox_root"
      launch_sandbox_root="$(cd "$launch_sandbox_root" && pwd -P)"
      launch_home="$launch_sandbox_root/home"
      mkdir -p "$launch_home/Library/Application Support" "$launch_home/Library/Caches" \
        "$launch_home/Library/Preferences" "$launch_home/.config" \
        "$launch_home/.cache" "$launch_home/.local/share"
      launch_sandbox_profile="$launch_sandbox_root/native-current-package-launch.sb"
      launch_original_home="${HOME:-}"
      launch_real_product_data_dir="${launch_original_home}/Library/Application Support/ai.hepta.hepta-native"
      launch_real_product_cache_dir="${launch_original_home}/Library/Caches/ai.hepta.hepta-native"

      if [[ "$launch_original_home" == /* && -x "$WINDOW_SANDBOX_PROFILE_SCRIPT" ]] \
        && "$WINDOW_SANDBOX_PROFILE_SCRIPT" \
          --data-dir "$launch_real_product_data_dir" \
          --cache-dir "$launch_real_product_cache_dir" \
          --scratch-dir "$launch_sandbox_root" \
          --output "$launch_sandbox_profile"; then
        launch_sandbox_profile_applied=true
        launch_home_isolated=true
        launch_real_product_data_path_denied=true
        launch_real_product_cache_path_denied=true
        launch_network_denied_by_sandbox=true
        launch_keychain_services_denied=true
        launch_force_login_argument=true
        launch_command="$binary --force-login"
        start_epoch="$(date +%s)"
        launch_probe_executed=true
        (
          unset APPLE_ID APPLE_PASSWORD APPLE_TEAM_ID APPLE_CERTIFICATE APPLE_CERTIFICATE_PASSWORD
          exec /usr/bin/sandbox-exec -f "$launch_sandbox_profile" /usr/bin/env \
            HOME="$launch_home" \
            CFFIXED_USER_HOME="$launch_home" \
            XDG_CONFIG_HOME="$launch_home/.config" \
            XDG_CACHE_HOME="$launch_home/.cache" \
            XDG_DATA_HOME="$launch_home/.local/share" \
            TMPDIR="$launch_sandbox_root" \
            HTTPS_PROXY=http://127.0.0.1:9 \
            HTTP_PROXY=http://127.0.0.1:9 \
            ALL_PROXY=http://127.0.0.1:9 \
            NO_PROXY=localhost,127.0.0.1 \
            "$binary" --force-login
        ) >"$launch_log" 2>"$launch_stderr" &
        launch_pid=$!

        for _ in {1..40}; do
          if ! hepta_process_is_alive "$launch_pid"; then break; fi
          candidate_start_token=""
          candidate_command=""
          if hepta_process_read_identity "$launch_pid"; then
            candidate_start_token="$HEPTA_PROCESS_ACTUAL_START_TOKEN"
            candidate_command="$HEPTA_PROCESS_ACTUAL_COMMAND"
            if [[ -z "$launch_spawn_start_token" ]]; then
              launch_spawn_start_token="$candidate_start_token"
            fi
          fi
          if [[ -n "$candidate_start_token" \
            && "$candidate_start_token" == "$launch_spawn_start_token" \
            && "$candidate_command" == "$launch_command" ]]; then
            launch_start_token="$candidate_start_token"
            launch_executable="$binary"
            launch_process_started=true
            launch_pid_identity_verified=true
            break
          fi
          sleep 0.25
        done

        if [[ "$launch_process_started" == "true" ]]; then
          launch_probe_ready=true
          for _ in {1..20}; do
            if ! launch_pid_identity_matches "$launch_pid" "$launch_start_token" "$launch_command"; then
              launch_probe_ready=false
              break
            fi
            sleep 0.25
          done
        fi
        launch_probe_seconds=$(( $(date +%s) - start_epoch ))

        cleanup_rc=0
        hepta_process_reset_termination_result
        if ! hepta_process_is_alive "$launch_pid"; then
          HEPTA_PROCESS_STOP_CONFIRMED=true
        elif [[ -n "$launch_start_token" ]]; then
          hepta_process_terminate_identity_safe \
            "$launch_pid" "$launch_start_token" "$launch_command" 20 0.1 20 || cleanup_rc=$?
        elif [[ -n "$launch_spawn_start_token" ]]; then
          # The wrapper was observed but did not reach the expected executable
          # identity. Anchor cleanup to its immutable start token and revalidate
          # the current command immediately before every signal.
          hepta_process_terminate_start_safe \
            "$launch_pid" "$launch_spawn_start_token" 20 0.1 20 || cleanup_rc=$?
        else
          cleanup_rc=74
          HEPTA_PROCESS_STOP_CONFIRMED=false
        fi
        launch_cleanup_exit_code="$cleanup_rc"
        launch_pid_revalidated_before_term="$HEPTA_PROCESS_TERM_IDENTITY_VERIFIED"
        launch_pid_revalidated_before_kill="$HEPTA_PROCESS_KILL_IDENTITY_VERIFIED"
        launch_terminated_by_gate="$HEPTA_PROCESS_TERM_SENT"
        launch_kill_escalation_performed="$HEPTA_PROCESS_KILL_SENT"
        if [[ "$cleanup_rc" == "0" && "$HEPTA_PROCESS_STOP_CONFIRMED" == true ]]; then
          launch_cleanup_confirmed=true
          set +e
          wait "$launch_pid"
          launch_exit_code=$?
          set -e
        fi
      fi
    fi

    binary_kind="$(file -b "$binary")"
    binary_sha="$(shasum -a 256 "$binary" | awk '{print $1}')"
    plist_sha="$(shasum -a 256 "$bundled_plist" | awk '{print $1}')"
    icon_path="$app_bundle/Contents/Resources/Hepta.icns"
    icon_sha=""
    [[ ! -s "$icon_path" ]] || icon_sha="$(shasum -a 256 "$icon_path" | awk '{print $1}')"
    bundle_bytes="$(find "$app_bundle" -type f -exec wc -c {} \; | awk '{sum += $1} END {print sum + 0}')"
    bundle_fingerprint="$(ruby "$APP_BUNDLE_FINGERPRINT" "$app_bundle")"
    artifact_json="$(jq -n \
      --arg path "$app_bundle" --arg binary_kind "$binary_kind" --arg binary_sha256 "$binary_sha" \
      --arg info_plist_sha256 "$plist_sha" --arg icon_sha256 "$icon_sha" \
      --argjson bytes "$bundle_bytes" --argjson head_embedded "$artifact_head_embedded" \
      --arg expected_head "$expected_head" --argjson rpath_ready "$binary_rpath_ready" \
      --argjson unsigned "$bundle_unsigned" --argjson linker_adhoc "$linker_adhoc_signature" \
      --arg bundle_requires_carbon "$bundle_requires_carbon" --argjson launch_services "$launch_via_launch_services" \
      --argjson launch_pid "$launch_pid" --arg launch_executable "$launch_executable" --arg launch_start_token "$launch_start_token" \
      --arg launch_command "$launch_command" --arg launch_sandbox_profile "$launch_sandbox_profile" \
      --argjson launch_probe_executed "$launch_probe_executed" --argjson process_started "$launch_process_started" \
      --argjson launch_sandbox_profile_applied "$launch_sandbox_profile_applied" \
      --argjson launch_force_login_argument "$launch_force_login_argument" \
      --argjson launch_cleanup_confirmed "$launch_cleanup_confirmed" \
      --argjson launch_cleanup_exit_code "$launch_cleanup_exit_code" \
      --arg build_log "$build_log" --arg launch_log "$launch_log" --arg launch_stderr "$launch_stderr" \
      --argjson bundle_fingerprint "$bundle_fingerprint" \
      '{probe_type:"formal_cargo_packager_robius_unsigned_app",path:$path,bytes:$bytes,binary_kind:$binary_kind,binary_sha256:$binary_sha256,info_plist_sha256:$info_plist_sha256,icon_sha256:$icon_sha256,bundle_fingerprint:$bundle_fingerprint,expected_head:$expected_head,full_head_embedded:$head_embedded,makepad_bundle_rpath_ready:$rpath_ready,ls_requires_carbon:($bundle_requires_carbon == "true"),bundle_signature:(if $unsigned then "linker_adhoc_no_developer_id" else "unexpected_developer_identity" end),linker_adhoc_signature:$linker_adhoc,developer_id_signed:false,launch_probe_executed:$launch_probe_executed,process_started:$process_started,launch_method:(if $launch_probe_executed then "direct_sandboxed_executable" else "not_executed" end),launch_via_launch_services:$launch_services,launch_pid:$launch_pid,launch_executable:(if $launch_executable == "" then null else $launch_executable end),launch_command:(if $launch_command == "" then null else $launch_command end),launch_start_token:(if $launch_start_token == "" then null else $launch_start_token end),force_login_argument:$launch_force_login_argument,sandbox_profile_applied:$launch_sandbox_profile_applied,identity_safe_process_termination_confirmed:$launch_cleanup_confirmed,process_cleanup_exit_code:$launch_cleanup_exit_code,sandbox_profile:(if $launch_sandbox_profile == "" then null else $launch_sandbox_profile end),build_log:$build_log,launch_log:$launch_log,launch_stderr_log:$launch_stderr}')"

    if [[ "$static_ready" == "true" && "$resources_complete" == "true" \
      && "$collector_sources_complete" == "true" && "$artifact_head_embedded" == "true" \
      && "$binary_rpath_ready" == "true" && "$bundle_unsigned" == "true" \
      && "$bundle_requires_carbon" == "false" \
      && "$(jq -r '.symlinks_rejected and .supported_entry_types_only' <<<"$bundle_fingerprint")" == "true" \
      && "$binary_kind" == Mach-O\ *executable* \
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
  && "$(jq -r '.worktree_clean and .repository_worktree_clean' <<<"$source_binding")" == "true" ]]; then
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
  --argjson single_artifact_release_contract_ready "$single_artifact_release_contract_ready" \
  --argjson build_requested "$BUILD" --argjson bootstrap_tools "$BOOTSTRAP_TOOLS" \
  --argjson launch_probe_requested "$LAUNCH_PROBE" --arg launch_probe_mode "$LAUNCH_PROBE_MODE" \
  --argjson formal_pipeline_exit_code "$formal_pipeline_exit_code" \
  --argjson formal_ready "$formal_unsigned_packaging_pipeline_ready" \
  --argjson resources_complete "$resources_complete" --argjson collector_sources_complete "$collector_sources_complete" \
  --argjson resource_reports "$resource_reports" --argjson collector_source_reports "$collector_source_reports" \
  --argjson launch_probe_ready "$launch_probe_ready" --argjson launch_probe_executed "$launch_probe_executed" \
  --argjson launch_process_started "$launch_process_started" --argjson launch_probe_seconds "$launch_probe_seconds" \
  --argjson launch_via_launch_services "$launch_via_launch_services" --argjson launch_pid "$launch_pid" \
  --arg launch_executable "$launch_executable" --arg launch_command "$launch_command" \
  --arg launch_start_token "$launch_start_token" --arg launch_sandbox_profile "$launch_sandbox_profile" \
  --arg launch_home "$launch_home" \
  --argjson launch_pid_identity_verified "$launch_pid_identity_verified" \
  --argjson launch_pid_revalidated_before_term "$launch_pid_revalidated_before_term" \
  --argjson launch_pid_revalidated_before_kill "$launch_pid_revalidated_before_kill" \
  --argjson launch_terminated_by_gate "$launch_terminated_by_gate" \
  --argjson launch_kill_escalation_performed "$launch_kill_escalation_performed" \
  --argjson launch_cleanup_confirmed "$launch_cleanup_confirmed" \
  --argjson launch_cleanup_exit_code "$launch_cleanup_exit_code" \
  --argjson launch_force_login_argument "$launch_force_login_argument" \
  --argjson launch_sandbox_profile_applied "$launch_sandbox_profile_applied" \
  --argjson launch_home_isolated "$launch_home_isolated" \
  --argjson launch_real_product_data_path_denied "$launch_real_product_data_path_denied" \
  --argjson launch_real_product_cache_path_denied "$launch_real_product_cache_path_denied" \
  --argjson launch_network_denied_by_sandbox "$launch_network_denied_by_sandbox" \
  --argjson launch_keychain_services_denied "$launch_keychain_services_denied" \
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
    repository_worktree_clean:$source_binding.repository_worktree_clean,
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
    single_artifact_release_contract_ready:$single_artifact_release_contract_ready,
    declared_resources_byte_exact:$resources_complete,
    collector_sources_byte_exact:$collector_sources_complete,
    resource_trees:$resource_reports,
    collector_source_trees:$collector_source_reports,
    artifact_build_probe_ready:$formal_ready,
    binary_bundle_probe_ready:$formal_ready,
    formal_unsigned_packaging_pipeline_ready:$formal_ready,
    current_source_build_ready:$current_source_build_ready,
    launch_probe_required:false,
    package_source_readiness_requires_launch:false,
    independent_window_verifier_required:true,
    local_window_promotion_ready:false,
    launch_probe_mode:$launch_probe_mode,
    launch_probe_requested:($launch_probe_requested == 1),
    launch_probe_executed:$launch_probe_executed,
    process_started:$launch_process_started,
    staged_app_launch_verified:$launch_probe_ready,
    window_promotion_verified:false,
    launch_via_launch_services:$launch_via_launch_services,
    launch_probe_seconds:$launch_probe_seconds,
    launch_exit_code:$launch_exit_code,
    sandbox_profile_applied:$launch_sandbox_profile_applied,
    home_isolated:$launch_home_isolated,
    real_product_data_path_denied:$launch_real_product_data_path_denied,
    real_product_cache_path_denied:$launch_real_product_cache_path_denied,
    network_denied_by_sandbox:$launch_network_denied_by_sandbox,
    keychain_services_denied:$launch_keychain_services_denied,
    force_login_argument:$launch_force_login_argument,
    launch_probe:{
      scope:"sandboxed_process_start_diagnostic_only_not_window_promotion",
      requested:($launch_probe_requested == 1),
      mode:$launch_probe_mode,
      executed:$launch_probe_executed,
      process_started:$launch_process_started,
      process_survived_observation_window:$launch_probe_ready,
      method:(if $launch_probe_executed then "direct_sandboxed_executable" else "not_executed" end),
      via_launch_services:$launch_via_launch_services,
      pid:$launch_pid,
      executable:(if $launch_executable == "" then null else $launch_executable end),
      command:(if $launch_command == "" then null else $launch_command end),
      start_token:(if $launch_start_token == "" then null else $launch_start_token end),
      pid_identity_verified:$launch_pid_identity_verified,
      pid_revalidated_before_term:$launch_pid_revalidated_before_term,
      pid_revalidated_before_kill:$launch_pid_revalidated_before_kill,
      terminated_by_gate:$launch_terminated_by_gate,
      kill_escalation_performed:$launch_kill_escalation_performed,
      identity_safe_process_termination_confirmed:$launch_cleanup_confirmed,
      process_cleanup_exit_code:$launch_cleanup_exit_code,
      exit_code:$launch_exit_code,
      seconds:$launch_probe_seconds,
      isolation:{
        sandbox_profile:(if $launch_sandbox_profile == "" then null else $launch_sandbox_profile end),
        sandbox_profile_applied:$launch_sandbox_profile_applied,
        isolated_home:(if $launch_home == "" then null else $launch_home end),
        home_isolated:$launch_home_isolated,
        real_product_data_path_denied:$launch_real_product_data_path_denied,
        real_product_cache_path_denied:$launch_real_product_cache_path_denied,
        network_denied_by_sandbox:$launch_network_denied_by_sandbox,
        keychain_services_denied:$launch_keychain_services_denied,
        force_login_argument:$launch_force_login_argument
      },
      failures:([if (($launch_probe_requested == 1) and ($build_requested != 1)) then "launch_probe_requires_explicit_build" else empty end,if (($launch_probe_requested == 1) and ($build_requested == 1) and ($launch_probe_executed == false)) then "launch_probe_not_executed" else empty end,if ($launch_probe_executed and ($launch_process_started == false)) then "sandboxed_product_process_not_started" else empty end,if ($launch_process_started and ($launch_probe_ready == false)) then "sandboxed_product_process_did_not_survive_observation_window" else empty end,if ($launch_probe_executed and ($launch_cleanup_confirmed == false)) then "identity_safe_process_termination_not_confirmed" else empty end])
    },
    local_package_ready:$local_package_ready,
    artifact:$artifact,
    signed:false,
    notarized:false,
    stapled:false,
    public_distribution_ready:false,
    public_ga_ready:false,
    external_side_effects_performed:$launch_probe_executed,
    remote_side_effects_performed:false,
    side_effect_boundary:{
      mode:(if $launch_probe_executed then "sandboxed_local_product_process_probe" elif ($build_requested == 1) then "local_artifact_build_only" else "static_source_and_metadata_verification_only" end),
      local_artifact_files_may_be_written:($build_requested == 1),
      local_launch_scratch_may_be_written:$launch_probe_executed,
      external_side_effects_performed:$launch_probe_executed,
      remote_side_effects_performed:false,
      launch_probe_executed:$launch_probe_executed,
      process_started:$launch_process_started,
      real_home_environment_isolated:$launch_home_isolated,
      real_product_data_path_denied:$launch_real_product_data_path_denied,
      real_product_cache_path_denied:$launch_real_product_cache_path_denied,
      network_denied_by_sandbox:$launch_network_denied_by_sandbox,
      keychain_services_denied:$launch_keychain_services_denied,
      force_login_argument:$launch_force_login_argument,
      matrix_login_performed:false,
      provider_or_channel_mutation_performed:false,
      signing_notarization_stapling_or_publication_performed:false,
      independent_window_promotion_required:true
    },
    blockers:([if $source_binding.worktree_clean then empty else "ui_source_worktree_dirty" end,if $source_binding.repository_worktree_clean then empty else "repository_worktree_dirty" end,if $source_stable then empty else "ui_source_changed_during_package_gate" end,if $static_ready then empty else "package_metadata_contract_not_ready" end,if (($build_requested == 1) and ($darwin_build_supported == false)) then "darwin_app_build_not_supported_on_host" else empty end,if ($build_requested == 1) then (if $formal_pipeline_exit_code == 0 then empty else "formal_unsigned_packaging_command_failed" end) else "formal_unsigned_packaging_not_requested" end,if $resources_complete then empty else "declared_package_resources_not_byte_exact" end,if $collector_sources_complete then empty else "robius_collector_sources_not_byte_exact" end,if $formal_ready then empty else "formal_unsigned_packaging_pipeline_not_ready" end,"developer_id_signing_not_performed","notarization_not_performed","stapling_not_performed","public_distribution_not_authorized"])
  }')"

if [[ -n "$REPORT_PATH" ]]; then
  hepta_safe_output_atomic_write_text "$REPORT_PATH" "$report" || {
    echo "could not atomically write package report" >&2
    exit 1
  }
else
  printf '%s\n' "$report"
fi
[[ "$local_package_ready" == "true" \
  && ( "$LAUNCH_PROBE" != "1" \
    || ( "$launch_probe_ready" == "true" && "$launch_cleanup_confirmed" == "true" ) ) ]]
