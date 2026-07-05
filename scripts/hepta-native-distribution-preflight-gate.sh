#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

APP_DIR="apps/hepta-native"
CARGO_TOML="$APP_DIR/Cargo.toml"
PACKAGING_DIR="$APP_DIR/packaging"
DMG_SCRIPT="$PACKAGING_DIR/build-macos-dmg.sh"
INFO_PLIST="$PACKAGING_DIR/Info.plist"
ENTITLEMENTS_PLIST="$PACKAGING_DIR/Entitlements.plist"
DMG_BACKGROUND="$PACKAGING_DIR/Hepta Native macOS dmg background.png"
ICNS_PATH="$PACKAGING_DIR/HeptaNative.icns"
PACKAGING_REPORT_PATH="${HEPTA_NATIVE_DISTRIBUTION_PREFLIGHT_PACKAGING_REPORT_PATH:-}"
REPORT_PATH="${HEPTA_NATIVE_DISTRIBUTION_PREFLIGHT_REPORT_PATH:-}"

bool_command() {
  if command -v "$1" >/dev/null 2>&1; then
    printf 'true'
  else
    printf 'false'
  fi
}

bool_xcrun_tool() {
  if command -v xcrun >/dev/null 2>&1 && xcrun -f "$1" >/dev/null 2>&1; then
    printf 'true'
  else
    printf 'false'
  fi
}

bool_marker() {
  local path="$1"
  local marker="$2"
  if grep -Fq -- "$marker" "$path"; then
    printf 'true'
  else
    printf 'false'
  fi
}

plist_value() {
  local path="$1"
  local key="$2"
  /usr/libexec/PlistBuddy -c "Print :$key" "$path" 2>/dev/null || true
}

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

for required in "$CARGO_TOML" "$DMG_SCRIPT" "$INFO_PLIST" "$ENTITLEMENTS_PLIST" "$DMG_BACKGROUND" "$ICNS_PATH"; do
  if [[ ! -s "$required" ]]; then
    echo "missing required distribution preflight file: $required" >&2
    exit 1
  fi
done

bash -n "$DMG_SCRIPT"
plutil -lint "$INFO_PLIST" "$ENTITLEMENTS_PLIST" >/dev/null

product_version="$(sed -n 's/^version[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "$CARGO_TOML" | head -1)"
signing_identity="$(sed -n 's/^signing_identity[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "$CARGO_TOML" | head -1)"
bundle_identifier="$(plist_value "$INFO_PLIST" "CFBundleIdentifier")"
bundle_executable="$(plist_value "$INFO_PLIST" "CFBundleExecutable")"
bundle_name="$(plist_value "$INFO_PLIST" "CFBundleName")"
bundle_package_type="$(plist_value "$INFO_PLIST" "CFBundlePackageType")"
bundle_icon_file="$(plist_value "$INFO_PLIST" "CFBundleIconFile")"
minimum_system_version="$(plist_value "$INFO_PLIST" "LSMinimumSystemVersion")"
url_scheme_hepta="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleURLTypes:0:CFBundleURLSchemes:0' "$INFO_PLIST" 2>/dev/null || true)"
url_scheme_matrix="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleURLTypes:0:CFBundleURLSchemes:1' "$INFO_PLIST" 2>/dev/null || true)"
location_entitlement="$(plist_value "$ENTITLEMENTS_PLIST" "com.apple.security.personal-information.location")"

codesign_available="$(bool_command codesign)"
xcrun_available="$(bool_command xcrun)"
hdiutil_available="$(bool_command hdiutil)"
ditto_available="$(bool_command ditto)"
spctl_available="$(bool_command spctl)"
xattr_available="$(bool_command xattr)"
plutil_available="$(bool_command plutil)"
cargo_packager_available="$(bool_command cargo-packager)"
notarytool_available="$(bool_xcrun_tool notarytool)"
stapler_available="$(bool_xcrun_tool stapler)"

script_requires_apple_id="$(bool_marker "$DMG_SCRIPT" "APPLE_ID")"
script_requires_apple_password="$(bool_marker "$DMG_SCRIPT" "APPLE_PASSWORD")"
script_requires_apple_team_id="$(bool_marker "$DMG_SCRIPT" "APPLE_TEAM_ID")"
script_unsets_apple_env_for_unsigned_packager="$(bool_marker "$DMG_SCRIPT" "env -u APPLE_ID -u APPLE_PASSWORD -u APPLE_TEAM_ID cargo packager --release")"
script_reads_signing_identity_from_cargo="$(bool_marker "$DMG_SCRIPT" "signing_identity")"
script_codesign_retry_present="$(bool_marker "$DMG_SCRIPT" "codesign_with_retry")"
script_hardened_runtime_present="$(bool_marker "$DMG_SCRIPT" "--options runtime")"
script_entitlements_present="$(bool_marker "$DMG_SCRIPT" "--entitlements")"
script_timestamp_present="$(bool_marker "$DMG_SCRIPT" "--timestamp")"
script_notary_submit_present="$(bool_marker "$DMG_SCRIPT" "xcrun notarytool submit")"
script_staple_present="$(bool_marker "$DMG_SCRIPT" "xcrun stapler staple")"
script_staple_validate_present="$(bool_marker "$DMG_SCRIPT" "xcrun stapler validate")"
script_spctl_present="$(bool_marker "$DMG_SCRIPT" "spctl --assess")"
script_spctl_hard_fail_present="$(bool_marker "$DMG_SCRIPT" "spctl assessment failed")"
script_dmg_rw_repack_present="$(bool_marker "$DMG_SCRIPT" "hdiutil convert")"
script_signed_app_embed_present="$(bool_marker "$DMG_SCRIPT" 'ditto "$APP_PATH" "$MOUNT_DIR/$APP_BUNDLE"')"
script_cargo_toml_restore_present="$(bool_marker "$DMG_SCRIPT" "Restored Cargo.toml")"
script_notary_keychain_profile_present="$(bool_marker "$DMG_SCRIPT" "HEPTA_NATIVE_NOTARYTOOL_PROFILE")"
script_release_artifact_receipt_present="$(bool_marker "$DMG_SCRIPT" "HEPTA_NATIVE_RELEASE_ARTIFACT_RECEIPT_PATH")"
script_notary_log_capture_present="$(bool_marker "$DMG_SCRIPT" "NOTARY_LOG")"

packaging_unsigned_ready=false
packaging_unsigned_app_path=""
packaging_unsigned_app_exists=false
packaging_unsigned_codesign_status=""
packaging_unsigned_bundle_bytes=0
if [[ -n "$PACKAGING_REPORT_PATH" && -s "$PACKAGING_REPORT_PATH" ]]; then
  packaging_unsigned_ready="$(jq -r '.local_unsigned_app_bundle_probe_ready == true' "$PACKAGING_REPORT_PATH")"
  packaging_unsigned_app_path="$(jq -r '.local_unsigned_app_bundle.app_bundle_path // ""' "$PACKAGING_REPORT_PATH")"
  packaging_unsigned_codesign_status="$(jq -r '.local_unsigned_app_bundle.codesign_status // ""' "$PACKAGING_REPORT_PATH")"
  packaging_unsigned_bundle_bytes="$(jq -r '.local_unsigned_app_bundle.bundle_bytes // 0' "$PACKAGING_REPORT_PATH")"
  if [[ -d "$packaging_unsigned_app_path" ]]; then
    packaging_unsigned_app_exists=true
  fi
fi

info_sha256="$(sha256_file "$INFO_PLIST")"
entitlements_sha256="$(sha256_file "$ENTITLEMENTS_PLIST")"
icns_sha256="$(sha256_file "$ICNS_PATH")"
dmg_script_sha256="$(sha256_file "$DMG_SCRIPT")"
dmg_background_sha256="$(sha256_file "$DMG_BACKGROUND")"

report="$(
  jq -n \
    --arg product "Hepta Native" \
    --arg gate "distribution_preflight_gate" \
    --arg cargo_toml "$CARGO_TOML" \
    --arg dmg_script "$DMG_SCRIPT" \
    --arg info_plist "$INFO_PLIST" \
    --arg entitlements_plist "$ENTITLEMENTS_PLIST" \
    --arg dmg_background "$DMG_BACKGROUND" \
    --arg icns_path "$ICNS_PATH" \
    --arg product_version "$product_version" \
    --arg signing_identity "$signing_identity" \
    --arg bundle_identifier "$bundle_identifier" \
    --arg bundle_executable "$bundle_executable" \
    --arg bundle_name "$bundle_name" \
    --arg bundle_package_type "$bundle_package_type" \
    --arg bundle_icon_file "$bundle_icon_file" \
    --arg minimum_system_version "$minimum_system_version" \
    --arg url_scheme_hepta "$url_scheme_hepta" \
    --arg url_scheme_matrix "$url_scheme_matrix" \
    --arg location_entitlement "$location_entitlement" \
    --arg info_sha256 "$info_sha256" \
    --arg entitlements_sha256 "$entitlements_sha256" \
    --arg icns_sha256 "$icns_sha256" \
    --arg dmg_script_sha256 "$dmg_script_sha256" \
    --arg dmg_background_sha256 "$dmg_background_sha256" \
    --arg packaging_report_path "$PACKAGING_REPORT_PATH" \
    --arg packaging_unsigned_app_path "$packaging_unsigned_app_path" \
    --arg packaging_unsigned_codesign_status "$packaging_unsigned_codesign_status" \
    --argjson codesign_available "$codesign_available" \
    --argjson xcrun_available "$xcrun_available" \
    --argjson hdiutil_available "$hdiutil_available" \
    --argjson ditto_available "$ditto_available" \
    --argjson spctl_available "$spctl_available" \
    --argjson xattr_available "$xattr_available" \
    --argjson plutil_available "$plutil_available" \
    --argjson cargo_packager_available "$cargo_packager_available" \
    --argjson notarytool_available "$notarytool_available" \
    --argjson stapler_available "$stapler_available" \
    --argjson script_requires_apple_id "$script_requires_apple_id" \
    --argjson script_requires_apple_password "$script_requires_apple_password" \
    --argjson script_requires_apple_team_id "$script_requires_apple_team_id" \
    --argjson script_unsets_apple_env_for_unsigned_packager "$script_unsets_apple_env_for_unsigned_packager" \
    --argjson script_reads_signing_identity_from_cargo "$script_reads_signing_identity_from_cargo" \
    --argjson script_codesign_retry_present "$script_codesign_retry_present" \
    --argjson script_hardened_runtime_present "$script_hardened_runtime_present" \
    --argjson script_entitlements_present "$script_entitlements_present" \
    --argjson script_timestamp_present "$script_timestamp_present" \
    --argjson script_notary_submit_present "$script_notary_submit_present" \
    --argjson script_staple_present "$script_staple_present" \
    --argjson script_staple_validate_present "$script_staple_validate_present" \
    --argjson script_spctl_present "$script_spctl_present" \
    --argjson script_spctl_hard_fail_present "$script_spctl_hard_fail_present" \
    --argjson script_dmg_rw_repack_present "$script_dmg_rw_repack_present" \
    --argjson script_signed_app_embed_present "$script_signed_app_embed_present" \
    --argjson script_cargo_toml_restore_present "$script_cargo_toml_restore_present" \
    --argjson script_notary_keychain_profile_present "$script_notary_keychain_profile_present" \
    --argjson script_release_artifact_receipt_present "$script_release_artifact_receipt_present" \
    --argjson script_notary_log_capture_present "$script_notary_log_capture_present" \
    --argjson packaging_unsigned_ready "$packaging_unsigned_ready" \
    --argjson packaging_unsigned_app_exists "$packaging_unsigned_app_exists" \
    --argjson packaging_unsigned_bundle_bytes "$packaging_unsigned_bundle_bytes" \
    '{
      product:$product,
      gate:$gate,
      status:"ready",
      distribution_preflight_gate_ready:true,
      distribution_preflight_mode:"static_distribution_workflow_plus_unsigned_bundle_probe",
      public_distribution_ready:false,
      release_approval_required:true,
      credential_values_read:false,
      keychain_identity_lookup_performed:false,
      network_call_performed:false,
      notary_submission_performed:false,
      public_distribution_artifact_written:false,
      app_signed:false,
      app_notarized:false,
      app_stapled:false,
      inspected_files:{
        cargo_toml:$cargo_toml,
        dmg_script:$dmg_script,
        info_plist:$info_plist,
        entitlements_plist:$entitlements_plist,
        dmg_background:$dmg_background,
        icns_path:$icns_path
      },
      artifact_hashes:{
        info_plist:$info_sha256,
        entitlements_plist:$entitlements_sha256,
        icns:$icns_sha256,
        dmg_script:$dmg_script_sha256,
        dmg_background:$dmg_background_sha256
      },
      package_metadata:{
        product_version:$product_version,
        signing_identity_present:($signing_identity | length > 0),
        signing_identity:$signing_identity,
        bundle_identifier:$bundle_identifier,
        bundle_executable:$bundle_executable,
        bundle_name:$bundle_name,
        bundle_package_type:$bundle_package_type,
        bundle_icon_file:$bundle_icon_file,
        minimum_system_version:$minimum_system_version,
        url_schemes:[$url_scheme_hepta, $url_scheme_matrix],
        location_entitlement_enabled:($location_entitlement == "true")
      },
      local_tooling:{
        codesign_available:$codesign_available,
        xcrun_available:$xcrun_available,
        hdiutil_available:$hdiutil_available,
        ditto_available:$ditto_available,
        spctl_available:$spctl_available,
        xattr_available:$xattr_available,
        plutil_available:$plutil_available,
        cargo_packager_available:$cargo_packager_available,
        notarytool_available:$notarytool_available,
        stapler_available:$stapler_available
      },
      dmg_script_contract:{
        required_credential_env_names:["APPLE_ID", "APPLE_PASSWORD", "APPLE_TEAM_ID"],
        requires_apple_id:$script_requires_apple_id,
        requires_apple_password:$script_requires_apple_password,
        requires_apple_team_id:$script_requires_apple_team_id,
        unsigned_packager_unsets_apple_env:$script_unsets_apple_env_for_unsigned_packager,
        reads_signing_identity_from_cargo:$script_reads_signing_identity_from_cargo,
        codesign_retry_present:$script_codesign_retry_present,
        hardened_runtime_present:$script_hardened_runtime_present,
        entitlements_present:$script_entitlements_present,
        timestamp_present:$script_timestamp_present,
        notary_submit_present:$script_notary_submit_present,
        staple_present:$script_staple_present,
        staple_validate_present:$script_staple_validate_present,
        spctl_assess_present:$script_spctl_present,
        spctl_hard_fail_present:$script_spctl_hard_fail_present,
        dmg_rw_repack_present:$script_dmg_rw_repack_present,
        signed_app_embed_present:$script_signed_app_embed_present,
        cargo_toml_restore_present:$script_cargo_toml_restore_present,
        notary_keychain_profile_present:$script_notary_keychain_profile_present,
        release_artifact_receipt_present:$script_release_artifact_receipt_present,
        notary_log_capture_present:$script_notary_log_capture_present
      },
      unsigned_app_bundle_probe:{
        packaging_report_path:$packaging_report_path,
        ready:$packaging_unsigned_ready,
        app_bundle_path:$packaging_unsigned_app_path,
        app_bundle_exists:$packaging_unsigned_app_exists,
        codesign_status:$packaging_unsigned_codesign_status,
        bundle_bytes:$packaging_unsigned_bundle_bytes
      },
      distribution_static_contract_ready:(
        ($signing_identity | length > 0)
        and $bundle_identifier == "ai.hepta.nativeapp"
        and $bundle_executable == "hepta-native"
        and $bundle_name == "Hepta Native"
        and $bundle_package_type == "APPL"
        and $bundle_icon_file == "HeptaNative.icns"
        and $url_scheme_hepta == "hepta-native"
        and $url_scheme_matrix == "matrix"
        and $script_requires_apple_id
        and $script_requires_apple_password
        and $script_requires_apple_team_id
        and $script_unsets_apple_env_for_unsigned_packager
        and $script_codesign_retry_present
        and $script_hardened_runtime_present
        and $script_entitlements_present
        and $script_timestamp_present
        and $script_notary_submit_present
        and $script_staple_present
        and $script_staple_validate_present
        and $script_spctl_present
        and $script_spctl_hard_fail_present
        and $script_dmg_rw_repack_present
        and $script_signed_app_embed_present
        and $script_cargo_toml_restore_present
        and $script_notary_keychain_profile_present
        and $script_release_artifact_receipt_present
        and $script_notary_log_capture_present
      ),
      local_distribution_tooling_ready:(
        $codesign_available
        and $xcrun_available
        and $hdiutil_available
        and $ditto_available
        and $spctl_available
        and $xattr_available
        and $plutil_available
        and $notarytool_available
        and $stapler_available
      ),
      release_artifact_tooling_ready:($cargo_packager_available),
      blockers:([
        (if $cargo_packager_available then empty else "cargo_packager_missing" end),
        "operator_release_approval_required",
        "apple_credentials_not_read",
        "notary_submission_not_performed",
        "public_distribution_artifact_not_written"
      ]),
      side_effects:{
        filesystem_read:true,
        filesystem_written:false,
        keychain_identity_lookup_performed:false,
        credential_value_read:false,
        network_call_performed:false,
        notary_submission_performed:false,
        app_signed:false,
        app_notarized:false,
        app_stapled:false,
        public_distribution_artifact_written:false,
        external_send_performed:false,
        provider_invoked:false,
        channel_send_performed:false,
        gateway_mutation_performed:false
      }
    }'
)"

jq -e '
  .status == "ready"
  and .distribution_preflight_gate_ready == true
  and .distribution_preflight_mode == "static_distribution_workflow_plus_unsigned_bundle_probe"
  and .distribution_static_contract_ready == true
  and .package_metadata.bundle_identifier == "ai.hepta.nativeapp"
  and .package_metadata.bundle_executable == "hepta-native"
  and .package_metadata.bundle_name == "Hepta Native"
  and .package_metadata.bundle_package_type == "APPL"
  and (.package_metadata.url_schemes | index("hepta-native") != null)
  and (.package_metadata.url_schemes | index("matrix") != null)
  and .dmg_script_contract.requires_apple_id == true
  and .dmg_script_contract.requires_apple_password == true
  and .dmg_script_contract.requires_apple_team_id == true
  and .dmg_script_contract.unsigned_packager_unsets_apple_env == true
  and .dmg_script_contract.codesign_retry_present == true
  and .dmg_script_contract.hardened_runtime_present == true
  and .dmg_script_contract.entitlements_present == true
  and .dmg_script_contract.notary_submit_present == true
  and .dmg_script_contract.staple_present == true
  and .dmg_script_contract.spctl_assess_present == true
  and .dmg_script_contract.spctl_hard_fail_present == true
  and .dmg_script_contract.notary_keychain_profile_present == true
  and .dmg_script_contract.release_artifact_receipt_present == true
  and .dmg_script_contract.notary_log_capture_present == true
  and .unsigned_app_bundle_probe.ready == true
  and .unsigned_app_bundle_probe.app_bundle_exists == true
  and .unsigned_app_bundle_probe.codesign_status == "unsigned_expected"
  and .unsigned_app_bundle_probe.bundle_bytes > 1000000
  and .public_distribution_ready == false
  and .credential_values_read == false
  and .keychain_identity_lookup_performed == false
  and .network_call_performed == false
  and .notary_submission_performed == false
  and .public_distribution_artifact_written == false
  and .app_signed == false
  and .app_notarized == false
  and .app_stapled == false
  and (.blockers | index("operator_release_approval_required") != null)
  and (.blockers | index("apple_credentials_not_read") != null)
  and (.blockers | index("notary_submission_not_performed") != null)
  and (.blockers | index("public_distribution_artifact_not_written") != null)
  and .side_effects.credential_value_read == false
  and .side_effects.network_call_performed == false
  and .side_effects.notary_submission_performed == false
  and .side_effects.public_distribution_artifact_written == false
' <<<"$report" >/dev/null

printf '%s\n' "$report"

if [[ -n "$REPORT_PATH" ]]; then
  mkdir -p "$(dirname "$REPORT_PATH")"
  printf '%s\n' "$report" >"$REPORT_PATH"
fi

echo "Hepta native distribution preflight gate passed"
