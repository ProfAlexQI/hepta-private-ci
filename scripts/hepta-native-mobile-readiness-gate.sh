#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

REPORT_PATH=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --output) REPORT_PATH="${2:-}"; shift 2 ;;
    --help|-h)
      cat <<'EOF'
usage: scripts/hepta-native-mobile-readiness-gate.sh [--output report.json]

Validates the current-source mobile build/package contracts and emits the
known iOS/Android runtime boundaries as explicit machine-readable hard false
values. It never signs, uploads, boots a simulator, or contacts a device.
EOF
      exit 0
      ;;
    *) echo "unknown argument: $1" >&2; exit 64 ;;
  esac
done

for command in git jq ruby rustup shasum ditto strings plutil sips find unzip; do
  command -v "$command" >/dev/null 2>&1 || { echo "$command is required" >&2; exit 2; }
done

SOURCE_BEFORE="$(scripts/hepta-ui-source-fingerprint)"
POLICY_PATH="apps/hepta-native/mobile-readiness-policy-v1.json"
MANIFEST_PATH="apps/hepta-native/Cargo.toml"
CREDENTIAL_PATH="apps/hepta-native/src/persistence/matrix_session_store/credential.rs"
TESTFLIGHT_PATH="apps/hepta-native/packaging/build-ios-testflight.sh"
IOS_SIMULATOR_SMOKE_PATH="scripts/hepta-native-ios-simulator-smoke.sh"
IOS_SIMULATOR_RECEIPT="${HEPTA_NATIVE_IOS_SIMULATOR_RECEIPT:-}"

policy_ready=false
if jq -e '
    .schema_version == 1
    and .kind == "hepta-native-mobile-readiness-policy"
    and .makepad_revision == "c4335cee10b22aca768510c9d072b0ca1bba15c8"
    and .known_upstream_boundaries.ios_accessibility_update_consumed == false
    and .known_upstream_boundaries.android_accessibility_update_consumed == false
    and .known_upstream_boundaries.observed_behavior == "CxOsOp::AccessibilityUpdate(_) => {}"
    and .downstream_boundaries.android_secure_credential_backend_supported == false
    and .downstream_boundaries.android_session_behavior == "fail_closed_relogin_required"
    and .downstream_boundaries.plaintext_credential_fallback_allowed == false
    and .downstream_boundaries.ios_bundle_identifier == "ai.hepta.nativeapp"
    and .downstream_boundaries.ios_product_name == "Hepta"
    and .downstream_boundaries.ios_executable == "hepta-native"
    and .downstream_boundaries.ios_simulator_smoke_signing_performed == false
    and (.promotion_requirements | to_entries | all(.value == true))
  ' "$POLICY_PATH" >/dev/null 2>&1; then
  policy_ready=true
fi

makepad_pin_ready=false
if [[ "$(rg -c 'makepad-widgets = .*rev = "c4335cee10b22aca768510c9d072b0ca1bba15c8"' "$MANIFEST_PATH")" == "1" \
  && "$(rg -c 'makepad-code-editor = .*rev = "c4335cee10b22aca768510c9d072b0ca1bba15c8"' "$MANIFEST_PATH")" == "1" ]]; then
  makepad_pin_ready=true
fi

android_credential_fail_closed_ready=false
if ruby -e '
    text = File.binread(ARGV.fetch(0))
    support = text[/pub\(super\) const SYSTEM_CREDENTIAL_STORE_SUPPORTED: bool = cfg!\(any\((.*?)\)\);/m, 1]
    abort "missing support contract" unless support
    abort "Android unexpectedly declared supported" if support.include?(%q{target_os = "android"})
    abort "missing fail-closed error" unless text.include?("secure Matrix session persistence is unavailable on this platform; re-login is required")
  ' "$CREDENTIAL_PATH" >/dev/null 2>&1; then
  android_credential_fail_closed_ready=true
fi

testflight_source_contract_ready=false
if ruby -e '
    text = File.binread(ARGV.fetch(0))
    required = [
      %q{APP="${APP:-nativeapp}"},
      %q{CARGO_PACKAGE="${CARGO_PACKAGE:-hepta-native}"},
      %q{PRODUCT_NAME="Hepta"},
      %q{APP_BUNDLE="$BUILD_DIR/${CARGO_PACKAGE}.app"},
      %q{rm -rf "$APP_BUNDLE" "$SCENT"},
      %q{run-device -p "$CARGO_PACKAGE" --locked --release},
      %q{scripts/hepta-native-mobile-cargo},
      %q{https://github.com/ProfAlexQI/Hepta/commit/$SOURCE_HEAD},
      %q{set_or_add CFBundleDisplayName string "$PRODUCT_NAME"},
      %q{set_or_add CFBundleName string "$PRODUCT_NAME"},
      %q{[[ "$BUILT_IDENTIFIER" == "ai.hepta.nativeapp" ]]},
      %q{[[ "$BUILT_EXECUTABLE" == "hepta-native" ]]},
      %q{[[ "$BUILT_DISPLAY_NAME" == "$PRODUCT_NAME" ]]},
      %q{[[ "$BUILT_BUNDLE_NAME" == "$PRODUCT_NAME" ]]},
      %q{compiled_asset_catalog_ready:true},
      %q{stale_artifact_accepted:false},
    ]
    abort "missing fail-closed TestFlight contract" unless required.all? { |needle| text.include?(needle) }
    abort "TestFlight script still suppresses a command failure" if text.include?("|| true")
  ' "$TESTFLIGHT_PATH" >/dev/null 2>&1; then
  testflight_source_contract_ready=true
fi

ios_simulator_smoke_source_shape_ready=false
if ruby -e '
    text = File.binread(ARGV.fetch(0))
    required = [
      %q{BUNDLE_IDENTIFIER="ai.hepta.nativeapp"},
      %q{PRODUCT_NAME="Hepta"},
      %q{CARGO_PACKAGE="hepta-native"},
      %q{scripts/hepta-native-mobile-cargo},
      %q{rm -rf "$APP_BUNDLE" "$SCENT"},
      %q{build -p "$CARGO_PACKAGE" --locked --release},
      %q{https://github.com/ProfAlexQI/Hepta/commit/$SOURCE_HEAD},
      %q{compiled_asset_catalog_ready:true},
      %q{xcrun simctl install},
      %q{xcrun simctl launch --terminate-running-process},
      %q{xcrun simctl io},
      %q{signing:{performed:false}},
      %q{ios_real_device_verified:false},
      %q{safe_area_verified:false},
      %q{software_keyboard_verified:false},
      %q{voiceover_verified:false},
      %q{rtl_verified:false},
      %q{dynamic_type_verified:false},
    ]
    abort "missing iOS simulator source contract" unless required.all? { |needle| text.include?(needle) }
    forbidden_commands = [
      /^\s*xcodebuild\s+.*-download/m,
      /^\s*xcrun\s+simctl\s+create\b/m,
      /^\s*codesign\b/m,
      /^\s*security\b/m,
    ]
    abort "iOS simulator script contains a forbidden mutation command" if forbidden_commands.any? { |pattern| text.match?(pattern) }
  ' "$IOS_SIMULATOR_SMOKE_PATH" >/dev/null 2>&1; then
  ios_simulator_smoke_source_shape_ready=true
fi

ios_simulator_smoke_source_contract='{"status":"not_ready"}'
ios_simulator_smoke_source_contract_ready=false
if [[ "$ios_simulator_smoke_source_shape_ready" == true ]] \
  && ios_simulator_smoke_source_contract="$($IOS_SIMULATOR_SMOKE_PATH --contract-only 2>/dev/null)" \
  && jq -e '
    .schema_version == 1
    and .kind == "hepta-native-ios-simulator-smoke-source-contract"
    and .status == "ready"
    and .producer == "scripts/hepta-native-ios-simulator-smoke.sh"
    and .build_wrapper == "scripts/hepta-native-mobile-cargo"
    and .receipt_kind == "hepta-native-ios-simulator-smoke-receipt"
    and .identity.bundle_identifier == "ai.hepta.nativeapp"
    and .identity.display_name == "Hepta"
    and .identity.name == "Hepta"
    and .identity.executable == "hepta-native"
    and (.requirements | to_entries | all(.value == true))
    and (.forbidden_actions | to_entries | all(.value == false))
    and .external_side_effects_performed == false
  ' >/dev/null <<<"$ios_simulator_smoke_source_contract"; then
  ios_simulator_smoke_source_contract_ready=true
fi

toolchain_report='{"status":"not_ready"}'
toolchain_wrapper_ready=false
if toolchain_report="$(scripts/hepta-native-mobile-cargo --print-toolchain-contract 2>/dev/null)" \
  && jq -e '
    .schema_version == 1
    and .kind == "hepta-native-mobile-cargo-toolchain-contract"
    and .status == "ready"
    and .cargo_makepad_requested_channel == "stable"
    and .resolved_toolchain == "1.95.0"
    and (.rustc | startswith("rustc 1.95.0 "))
    and .cargo_makepad.repository == "https://github.com/kevinaboos/makepad.git"
    and .cargo_makepad.revision == "c4335cee10b22aca768510c9d072b0ca1bba15c8"
    and (.cargo_makepad.binary_sha256 | test("^[0-9a-f]{64}$"))
    and .cargo_makepad.exact_revision_source_marker_ready == true
    and .cargo_makepad.custom_android_manifest_help_contract_ready == true
    and .cargo_makepad.global_cargo_makepad_used == false
    and .user_global_stable_mutated == false
  ' >/dev/null <<<"$toolchain_report"; then
  toolchain_wrapper_ready=true
fi

icon_report='{"status":"not_ready"}'
ios_icon_contract_ready=false
if icon_report="$(scripts/hepta-native-ios-icons verify 2>/dev/null)" \
  && jq -e '
    .schema_version == 1
    and .kind == "hepta-native-ios-icon-contract"
    and .status == "ready"
    and .canonical.path == "apps/hepta-native/resources/icon_1024.png"
    and .canonical.pixels == 1024
    and .canonical.png_color_type == 2
    and .canonical.alpha == false
    and .app_store_marketing_icon_opaque == true
    and (.generated | length == 5)
    and (.generated | all(.png_color_type == 2 and .alpha == false and .canonical_source == "apps/hepta-native/resources/icon_1024.png"))
  ' >/dev/null <<<"$icon_report"; then
  ios_icon_contract_ready=true
fi

installed_targets="$(rustup target list --toolchain 1.95.0 --installed 2>/dev/null || true)"
ios_toolchain_targets_ready=false
if grep -Fxq aarch64-apple-ios <<<"$installed_targets" \
  && grep -Fxq aarch64-apple-ios-sim <<<"$installed_targets"; then
  ios_toolchain_targets_ready=true
fi
android_toolchain_target_ready=false
if grep -Fxq aarch64-linux-android <<<"$installed_targets"; then android_toolchain_target_ready=true; fi

ios_distribution_identity_available=false
ios_distribution_identity_count=0
if command -v security >/dev/null 2>&1; then
  ios_distribution_identity_count="$(security find-identity -v -p codesigning 2>/dev/null | awk '/Apple Distribution/ {count++} END {print count+0}')"
  [[ "$ios_distribution_identity_count" -gt 0 ]] && ios_distribution_identity_available=true
fi

SOURCE_AFTER="$(scripts/hepta-ui-source-fingerprint)"
source_stable=false
if [[ "$(jq -r '.head' <<<"$SOURCE_BEFORE")" == "$(jq -r '.head' <<<"$SOURCE_AFTER")" \
  && "$(jq -r '.head_tree' <<<"$SOURCE_BEFORE")" == "$(jq -r '.head_tree' <<<"$SOURCE_AFTER")" \
  && "$(jq -r '.source_fingerprint' <<<"$SOURCE_BEFORE")" == "$(jq -r '.source_fingerprint' <<<"$SOURCE_AFTER")" ]]; then
  source_stable=true
fi

ios_simulator_receipt_supplied=false
ios_simulator_receipt_ready=false
ios_simulator_receipt_status="missing"
ios_simulator_receipt_summary="$(jq -n \
  --arg path "$IOS_SIMULATOR_RECEIPT" \
  '{supplied:false,path:$path,status:"missing",ready:false}')"

verify_ios_simulator_artifact() {
  local receipt="$1" archive="$2" extract_root app_bundle_count app_bundle plist binary mode evidence_path evidence_sha
  # Reject absolute paths, parent traversal, backslashes, and control bytes
  # before extraction. The smoke producer emits only portable slash-separated
  # app-bundle members, so these shapes are never needed by valid evidence.
  if ! unzip -Z -1 "$archive" 2>/dev/null | ruby -e '
      entries = STDIN.each_line.map(&:chomp)
      abort if entries.empty?
      entries.each do |entry|
        abort if entry.empty? || entry.start_with?("/") || entry.include?("\\")
        abort if entry.bytes.any? { |byte| byte < 0x20 || byte == 0x7f }
        abort if entry.split("/").include?("..")
      end
    '; then
    return 1
  fi

  extract_root="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ios-sim-receipt.XXXXXX")"
  if ! ditto -x -k "$archive" "$extract_root" >/dev/null 2>&1; then
    rm -rf "$extract_root"
    return 1
  fi
  # No archive member may redirect later hash/metadata reads outside the
  # extraction root. This is checked before discovering or opening the app.
  if [[ -n "$(find "$extract_root" -type l -print -quit)" ]]; then
    rm -rf "$extract_root"
    return 1
  fi
  app_bundle_count="$(find "$extract_root" -maxdepth 2 -type d -name 'hepta-native.app' | wc -l | tr -d '[:space:]')"
  [[ "$app_bundle_count" == "1" ]] || { rm -rf "$extract_root"; return 1; }
  app_bundle="$(find "$extract_root" -maxdepth 2 -type d -name 'hepta-native.app' -print -quit)"
  plist="$app_bundle/Info.plist"
  binary="$app_bundle/hepta-native"
  if [[ ! -s "$plist" || ! -x "$binary" ]] \
    || ! plutil -lint "$plist" >/dev/null 2>&1 \
    || [[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$plist" 2>/dev/null)" != "ai.hepta.nativeapp" ]] \
    || [[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleDisplayName' "$plist" 2>/dev/null)" != "Hepta" ]] \
    || [[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleName' "$plist" 2>/dev/null)" != "Hepta" ]] \
    || [[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$plist" 2>/dev/null)" != "hepta-native" ]] \
    || ! strings "$binary" | grep -F "https://github.com/ProfAlexQI/Hepta/commit/$(jq -r '.source_binding.head' "$receipt")" >/dev/null; then
    rm -rf "$extract_root"
    return 1
  fi

  mode="$(jq -r '.asset_catalog.mode' "$receipt")"
  evidence_path="$(jq -r '.asset_catalog.evidence.path' "$receipt")"
  evidence_sha="$(jq -r '.asset_catalog.evidence.sha256' "$receipt")"
  if [[ ! -s "$app_bundle/$evidence_path" \
    || "$(shasum -a 256 "$app_bundle/$evidence_path" | awk '{print $1}')" != "$evidence_sha" ]]; then
    rm -rf "$extract_root"
    return 1
  fi
  if [[ "$mode" == "actool_info_and_opaque_icon_outputs" ]]; then
    if ! plutil -lint "$app_bundle/actool-Info.plist" >/dev/null 2>&1 \
      || [[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIcons:CFBundlePrimaryIcon:CFBundleIconName' "$app_bundle/actool-Info.plist" 2>/dev/null)" != "AppIcon" ]] \
      || [[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIcons~ipad:CFBundlePrimaryIcon:CFBundleIconName' "$app_bundle/actool-Info.plist" 2>/dev/null)" != "AppIcon" ]]; then
      rm -rf "$extract_root"
      return 1
    fi
    while IFS=$'\t' read -r icon_path icon_sha icon_width icon_height; do
      if [[ ! -s "$app_bundle/$icon_path" \
        || "$(shasum -a 256 "$app_bundle/$icon_path" | awk '{print $1}')" != "$icon_sha" \
        || "$(sips -g pixelWidth "$app_bundle/$icon_path" 2>/dev/null | awk '/pixelWidth:/ {print $2}')" != "$icon_width" \
        || "$(sips -g pixelHeight "$app_bundle/$icon_path" 2>/dev/null | awk '/pixelHeight:/ {print $2}')" != "$icon_height" \
        || "$(sips -g hasAlpha "$app_bundle/$icon_path" 2>/dev/null | awk '/hasAlpha:/ {print $2}')" != "no" ]]; then
        rm -rf "$extract_root"
        return 1
      fi
    done < <(jq -r '.asset_catalog.icon_outputs[] | [.path,.sha256,(.width|tostring),(.height|tostring)] | @tsv' "$receipt")
  fi
  rm -rf "$extract_root"
}

if [[ -n "$IOS_SIMULATOR_RECEIPT" ]]; then
  ios_simulator_receipt_supplied=true
  ios_simulator_receipt_status="invalid"
  if [[ -s "$IOS_SIMULATOR_RECEIPT" ]] \
    && jq -e \
      --arg head "$(jq -r '.head' <<<"$SOURCE_AFTER")" \
      --arg tree "$(jq -r '.head_tree' <<<"$SOURCE_AFTER")" \
      --arg fingerprint "$(jq -r '.source_fingerprint' <<<"$SOURCE_AFTER")" '
        .schema_version == 1
        and .kind == "hepta-native-ios-simulator-smoke-receipt"
        and .producer == "scripts/hepta-native-ios-simulator-smoke.sh"
        and .status == "ready"
        and .ready == true
        and .source_binding.head == $head
        and .source_binding.head_tree == $tree
        and .source_binding.source_fingerprint == $fingerprint
        and .source_binding.worktree_clean == true
        and .source_binding.repository_worktree_clean == true
        and .device.state == "Booted"
        and .device.is_available == true
        and (.device.udid | type == "string" and length > 0)
        and .artifact.format == "zip"
        and .artifact.stale_artifact_accepted == false
        and (.artifact.path | type == "string" and startswith("/"))
        and (.artifact.sha256 | test("^[0-9a-f]{64}$"))
        and .screenshot.format == "png"
        and (.screenshot.path | type == "string" and startswith("/"))
        and (.screenshot.sha256 | test("^[0-9a-f]{64}$"))
        and .screenshot.width > 0
        and .screenshot.height > 0
        and .bundle.identifier == "ai.hepta.nativeapp"
        and .bundle.display_name == "Hepta"
        and .bundle.name == "Hepta"
        and .bundle.executable == "hepta-native"
        and .asset_catalog.compiled_asset_catalog_ready == true
        and (.asset_catalog.evidence.sha256 | test("^[0-9a-f]{64}$"))
        and (
          if .asset_catalog.mode == "assets_car" then
            .asset_catalog.evidence.path == "Assets.car"
            and ((.asset_catalog.icon_outputs // []) | length == 0)
          elif .asset_catalog.mode == "actool_info_and_opaque_icon_outputs" then
            .asset_catalog.evidence.path == "actool-Info.plist"
            and (.asset_catalog.icon_outputs | length == 4)
            and (.asset_catalog.icon_outputs | map(.path) | unique | length == 4)
            and (.asset_catalog.icon_outputs | map({path,width,height,alpha}) | sort_by(.path)) == ([
              {path:"AppIcon60x60@2x.png",width:120,height:120,alpha:false},
              {path:"AppIcon60x60@3x.png",width:180,height:180,alpha:false},
              {path:"AppIcon76x76@2x~ipad.png",width:152,height:152,alpha:false},
              {path:"AppIcon83.5x83.5@2x~ipad.png",width:167,height:167,alpha:false}
            ] | sort_by(.path))
            and (.asset_catalog.icon_outputs | all(.sha256 | test("^[0-9a-f]{64}$")))
          else false end
        )
        and .launch.ready == true
        and .launch.install_succeeded == true
        and .launch.launch_succeeded == true
        and .launch.pid > 0
        and (.launch.app_container | type == "string" and length > 0)
        and .signing.performed == false
        and (.forbidden_actions_performed | to_entries | all(.value == false))
        and (.hard_boundaries | to_entries | all(.value == false))
        and .toolchain.status == "ready"
        and .toolchain.resolved_toolchain == "1.95.0"
        and .toolchain.cargo_makepad.revision == "c4335cee10b22aca768510c9d072b0ca1bba15c8"
      ' "$IOS_SIMULATOR_RECEIPT" >/dev/null 2>&1; then
    receipt_artifact_path="$(jq -r '.artifact.path' "$IOS_SIMULATOR_RECEIPT")"
    receipt_artifact_sha256="$(jq -r '.artifact.sha256' "$IOS_SIMULATOR_RECEIPT")"
    receipt_screenshot_path="$(jq -r '.screenshot.path' "$IOS_SIMULATOR_RECEIPT")"
    receipt_screenshot_sha256="$(jq -r '.screenshot.sha256' "$IOS_SIMULATOR_RECEIPT")"
    if [[ -s "$receipt_artifact_path" && -s "$receipt_screenshot_path" \
      && "$(shasum -a 256 "$receipt_artifact_path" | awk '{print $1}')" == "$receipt_artifact_sha256" \
      && "$(shasum -a 256 "$receipt_screenshot_path" | awk '{print $1}')" == "$receipt_screenshot_sha256" ]] \
      && ruby -e 'abort unless File.binread(ARGV.fetch(0), 4).start_with?("PK")' "$receipt_artifact_path" >/dev/null 2>&1 \
      && ruby -e 'abort unless File.binread(ARGV.fetch(0), 8) == "\x89PNG\r\n\x1a\n".b' "$receipt_screenshot_path" >/dev/null 2>&1 \
      && verify_ios_simulator_artifact "$IOS_SIMULATOR_RECEIPT" "$receipt_artifact_path"; then
      ios_simulator_receipt_ready=true
      ios_simulator_receipt_status="ready"
    fi
  fi
  ios_simulator_receipt_summary="$(jq -n \
    --arg path "$IOS_SIMULATOR_RECEIPT" \
    --arg status "$ios_simulator_receipt_status" \
    --argjson ready "$ios_simulator_receipt_ready" \
    '{supplied:true,path:$path,status:$status,ready:$ready}')"
fi

source_contract_ready=false
if [[ "$source_stable" == true \
  && "$policy_ready" == true \
  && "$makepad_pin_ready" == true \
  && "$android_credential_fail_closed_ready" == true \
  && "$testflight_source_contract_ready" == true \
  && "$ios_simulator_smoke_source_contract_ready" == true \
  && "$toolchain_wrapper_ready" == true \
  && "$ios_icon_contract_ready" == true \
  && "$ios_toolchain_targets_ready" == true \
  && "$android_toolchain_target_ready" == true ]]; then
  source_contract_ready=true
fi

report="$(jq -n \
  --arg generated_at_utc "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --argjson source_binding "$SOURCE_AFTER" \
  --argjson source_stable "$source_stable" \
  --argjson policy_ready "$policy_ready" \
  --argjson makepad_pin_ready "$makepad_pin_ready" \
  --argjson credential_ready "$android_credential_fail_closed_ready" \
  --argjson testflight_ready "$testflight_source_contract_ready" \
  --argjson ios_simulator_smoke_source_ready "$ios_simulator_smoke_source_contract_ready" \
  --argjson ios_simulator_smoke_source_contract "$ios_simulator_smoke_source_contract" \
  --argjson ios_simulator_receipt_supplied "$ios_simulator_receipt_supplied" \
  --argjson ios_simulator_receipt_ready "$ios_simulator_receipt_ready" \
  --argjson ios_simulator_receipt_summary "$ios_simulator_receipt_summary" \
  --argjson toolchain_ready "$toolchain_wrapper_ready" \
  --argjson toolchain "$toolchain_report" \
  --argjson icons_ready "$ios_icon_contract_ready" \
  --argjson icons "$icon_report" \
  --argjson ios_targets "$ios_toolchain_targets_ready" \
  --argjson android_target "$android_toolchain_target_ready" \
  --argjson identity_available "$ios_distribution_identity_available" \
  --argjson identity_count "$ios_distribution_identity_count" \
  --argjson source_ready "$source_contract_ready" '
    {
      schema_version:1,
      kind:"hepta-native-mobile-readiness-gate",
      producer:"scripts/hepta-native-mobile-readiness-gate.sh",
      generated_at_utc:$generated_at_utc,
      status:(if $source_ready then "source_contract_ready" else "not_ready" end),
      source_binding:$source_binding,
      source_stable_during_run:$source_stable,
      mobile_source_contract_ready:$source_ready,
      checks:{
        policy_contract_ready:$policy_ready,
        pinned_makepad_revision_ready:$makepad_pin_ready,
        cargo_makepad_exact_toolchain_wrapper_ready:$toolchain_ready,
        testflight_fail_closed_current_source_contract_ready:$testflight_ready,
        ios_simulator_smoke_source_contract_ready:$ios_simulator_smoke_source_ready,
        ios_opaque_canonical_icon_contract_ready:$icons_ready,
        android_credential_fail_closed_contract_ready:$credential_ready,
        ios_pinned_toolchain_targets_installed:$ios_targets,
        android_pinned_toolchain_target_installed:$android_target
      },
      toolchain:$toolchain,
      ios_icons:$icons,
      ios_simulator_smoke_source_contract:$ios_simulator_smoke_source_contract,
      ios_simulator_runtime_evidence:$ios_simulator_receipt_summary,
      signing_preflight:{apple_distribution_identity_available:$identity_available,apple_distribution_identity_count:$identity_count,signing_performed:false},
      hard_boundaries:{
        ios_accessibility_update_consumed:false,
        android_accessibility_update_consumed:false,
        android_secure_session_persistence_ready:false,
        plaintext_credential_fallback_allowed:false,
        ios_simulator_runtime_verified:$ios_simulator_receipt_ready,
        ios_real_device_verified:false,
        android_real_device_verified:false,
        voiceover_verified:false,
        talkback_verified:false,
        software_keyboard_verified:false,
        safe_area_verified:false,
        rtl_verified:false,
        dynamic_type_or_font_scale_verified:false,
        mobile_full_product_ready:false,
        mobile_public_ga_ready:false
      },
      external_side_effects_performed:false,
      blockers:([if $source_stable then empty else "source_changed_during_mobile_gate" end,if $policy_ready then empty else "mobile_policy_contract_not_ready" end,if $makepad_pin_ready then empty else "makepad_revision_not_pinned" end,if $toolchain_ready then empty else "cargo_makepad_exact_toolchain_wrapper_not_ready" end,if $testflight_ready then empty else "testflight_current_source_fail_closed_contract_not_ready" end,if $ios_simulator_smoke_source_ready then empty else "ios_simulator_smoke_source_contract_not_ready" end,if $icons_ready then empty else "ios_opaque_icon_contract_not_ready" end,if $credential_ready then empty else "android_credential_fail_closed_contract_not_ready" end,if $ios_targets then empty else "ios_1_95_targets_not_installed" end,if $android_target then empty else "android_1_95_target_not_installed" end,if $identity_available then empty else "apple_distribution_identity_not_available" end,"pinned_makepad_ios_accessibility_update_discarded","pinned_makepad_android_accessibility_update_discarded","android_secure_credential_backend_not_supported",if $ios_simulator_receipt_ready then empty elif $ios_simulator_receipt_supplied then "ios_simulator_receipt_invalid" else "ios_simulator_receipt_missing" end,"ios_real_device_receipt_missing","android_real_device_receipt_missing","voiceover_receipt_missing","talkback_receipt_missing","software_keyboard_receipt_missing","safe_area_receipt_missing","rtl_receipt_missing","dynamic_type_or_font_scale_receipt_missing"])
    }
  ')"

if [[ -n "$REPORT_PATH" ]]; then
  mkdir -p "$(dirname "$REPORT_PATH")"
  printf '%s\n' "$report" >"$REPORT_PATH"
fi
printf '%s\n' "$report"
if [[ "$ios_simulator_receipt_supplied" == true && "$ios_simulator_receipt_ready" != true ]]; then
  exit 1
fi
jq -e '.mobile_source_contract_ready == true' <<<"$report" >/dev/null
