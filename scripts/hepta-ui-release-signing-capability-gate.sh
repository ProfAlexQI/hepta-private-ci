#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.release-signing-capability}"
REPORT_PATH="${HEPTA_UI_RELEASE_SIGNING_CAPABILITY_REPORT_PATH:-$READINESS_DIR/ui-release-signing-capability-gate.json}"
CAPABILITY_DIR="${HEPTA_UI_RELEASE_SIGNING_CAPABILITY_DIR:-$READINESS_DIR/release-signing-capability}"
MARKDOWN_PATH="$CAPABILITY_DIR/release-signing-capability.md"

APP_DIR="apps/hepta-native"
CARGO_TOML="$APP_DIR/Cargo.toml"
PACKAGING_DIR="$APP_DIR/packaging"
DMG_SCRIPT="$PACKAGING_DIR/build-macos-dmg.sh"
INFO_PLIST="$PACKAGING_DIR/Info.plist"
ENTITLEMENTS_PLIST="$PACKAGING_DIR/Entitlements.plist"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf '%s is required for the Hepta UI release signing capability gate\n' "$1" >&2
    exit 2
  fi
}

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

bool_nonempty_env() {
  local name="$1"
  if [[ -n "${!name:-}" ]]; then
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

file_sha256() {
  shasum -a 256 "$1" | awk '{print $1}'
}

file_bytes() {
  wc -c <"$1" | tr -d ' '
}

require_command jq
require_command shasum

for required in "$CARGO_TOML" "$DMG_SCRIPT" "$INFO_PLIST" "$ENTITLEMENTS_PLIST"; do
  if [[ ! -s "$required" ]]; then
    echo "missing required release signing capability input: $required" >&2
    exit 1
  fi
done

bash -n "$DMG_SCRIPT"
plutil -lint "$INFO_PLIST" "$ENTITLEMENTS_PLIST" >/dev/null

rm -rf "$CAPABILITY_DIR"
mkdir -p "$CAPABILITY_DIR" "$(dirname "$REPORT_PATH")"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-release-signing-capability.XXXXXX")"
IDENTITIES_RAW="$TMP_DIR/codesigning-identities.txt"
NOTARY_PROFILE_LOOKUP_RAW="$TMP_DIR/notary-profile-keychain-lookup.txt"
REPORT_TMP="$TMP_DIR/release-signing-capability-report.json"
MARKDOWN_TMP="$TMP_DIR/release-signing-capability.md"
trap 'rm -rf "$TMP_DIR"' EXIT

signing_identity="$(sed -n 's/^signing_identity[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "$CARGO_TOML" | head -1)"
configured_team_id="$(printf '%s\n' "$signing_identity" | sed -n 's/.*(\([A-Z0-9][A-Z0-9]*\)).*/\1/p' | head -1)"
bundle_identifier="$(plist_value "$INFO_PLIST" "CFBundleIdentifier")"
bundle_executable="$(plist_value "$INFO_PLIST" "CFBundleExecutable")"
bundle_name="$(plist_value "$INFO_PLIST" "CFBundleName")"
codesign_available="$(bool_command codesign)"
security_available="$(bool_command security)"
xcrun_available="$(bool_command xcrun)"
notarytool_available="$(bool_xcrun_tool notarytool)"
stapler_available="$(bool_xcrun_tool stapler)"
cargo_packager_available="$(bool_command cargo-packager)"
hdiutil_available="$(bool_command hdiutil)"
ditto_available="$(bool_command ditto)"
spctl_available="$(bool_command spctl)"
xattr_available="$(bool_command xattr)"

if [[ "$security_available" == "true" ]]; then
  security find-identity -p codesigning -v >"$IDENTITIES_RAW" 2>&1 || true
else
  printf 'security tool unavailable\n' >"$IDENTITIES_RAW"
fi

identity_output_sha="$(file_sha256 "$IDENTITIES_RAW")"
identity_output_bytes="$(file_bytes "$IDENTITIES_RAW")"
valid_identity_count="$(awk '/valid identities found/ {print $1}' "$IDENTITIES_RAW" | tail -1)"
if [[ -z "$valid_identity_count" || ! "$valid_identity_count" =~ ^[0-9]+$ ]]; then
  valid_identity_count=0
fi
matching_identity_count=0
if [[ -n "$signing_identity" ]]; then
  matching_identity_count="$( (grep -F -- "$signing_identity" "$IDENTITIES_RAW" || true) | wc -l | tr -d ' ')"
fi

apple_id_present="$(bool_nonempty_env APPLE_ID)"
apple_password_present="$(bool_nonempty_env APPLE_PASSWORD)"
apple_team_id_present="$(bool_nonempty_env APPLE_TEAM_ID)"
notary_profile_present="$(bool_nonempty_env HEPTA_NATIVE_NOTARYTOOL_PROFILE)"
build_script_supports_notary_profile="$(grep -Fq "HEPTA_NATIVE_NOTARYTOOL_PROFILE" "$DMG_SCRIPT" && printf 'true' || printf 'false')"
build_script_writes_artifact_receipt="$(grep -Fq "HEPTA_NATIVE_RELEASE_ARTIFACT_RECEIPT_PATH" "$DMG_SCRIPT" && printf 'true' || printf 'false')"
build_script_spctl_hard_fail="$(grep -Fq "spctl assessment failed" "$DMG_SCRIPT" && printf 'true' || printf 'false')"

notary_profile_keychain_lookup_performed=false
notary_profile_keychain_item_found=false
if [[ "$security_available" == "true" && "$notary_profile_present" == "true" ]]; then
  notary_profile_keychain_lookup_performed=true
  {
    printf 'profile keychain lookup: service=com.apple.gke.notary.tool account=<profile-env>\n'
    set +e
    security find-generic-password \
      -s "com.apple.gke.notary.tool" \
      -a "$HEPTA_NATIVE_NOTARYTOOL_PROFILE"
    profile_lookup_status_a=$?
    printf 'profile keychain lookup: service=com.apple.gke.notary.tool.<profile-env>\n'
    security find-generic-password \
      -s "com.apple.gke.notary.tool.$HEPTA_NATIVE_NOTARYTOOL_PROFILE"
    profile_lookup_status_b=$?
    set -e
    printf 'profile lookup exit statuses: %s %s\n' "$profile_lookup_status_a" "$profile_lookup_status_b"
  } >"$NOTARY_PROFILE_LOOKUP_RAW" 2>&1
  if [[ "$profile_lookup_status_a" -eq 0 || "$profile_lookup_status_b" -eq 0 ]]; then
    notary_profile_keychain_item_found=true
  fi
else
  printf 'notary profile keychain lookup not performed; profile env present=%s security available=%s\n' \
    "$notary_profile_present" "$security_available" >"$NOTARY_PROFILE_LOOKUP_RAW"
fi
notary_profile_lookup_output_sha="$(file_sha256 "$NOTARY_PROFILE_LOOKUP_RAW")"
notary_profile_lookup_output_bytes="$(file_bytes "$NOTARY_PROFILE_LOOKUP_RAW")"

cargo_sha="$(file_sha256 "$CARGO_TOML")"
dmg_script_sha="$(file_sha256 "$DMG_SCRIPT")"
info_sha="$(file_sha256 "$INFO_PLIST")"
entitlements_sha="$(file_sha256 "$ENTITLEMENTS_PLIST")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_release_signing_capability_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg capability_dir "$CAPABILITY_DIR" \
  --arg markdown_path "$MARKDOWN_PATH" \
  --arg cargo_toml "$CARGO_TOML" \
  --arg dmg_script "$DMG_SCRIPT" \
  --arg info_plist "$INFO_PLIST" \
  --arg entitlements_plist "$ENTITLEMENTS_PLIST" \
  --arg cargo_sha "$cargo_sha" \
  --arg dmg_script_sha "$dmg_script_sha" \
  --arg info_sha "$info_sha" \
  --arg entitlements_sha "$entitlements_sha" \
  --arg signing_identity "$signing_identity" \
  --arg configured_team_id "$configured_team_id" \
  --arg bundle_identifier "$bundle_identifier" \
  --arg bundle_executable "$bundle_executable" \
  --arg bundle_name "$bundle_name" \
  --arg identity_output_sha "$identity_output_sha" \
  --argjson identity_output_bytes "$identity_output_bytes" \
  --argjson valid_identity_count "$valid_identity_count" \
  --argjson matching_identity_count "$matching_identity_count" \
  --argjson codesign_available "$codesign_available" \
  --argjson security_available "$security_available" \
  --argjson xcrun_available "$xcrun_available" \
  --argjson notarytool_available "$notarytool_available" \
  --argjson stapler_available "$stapler_available" \
  --argjson cargo_packager_available "$cargo_packager_available" \
  --argjson hdiutil_available "$hdiutil_available" \
  --argjson ditto_available "$ditto_available" \
  --argjson spctl_available "$spctl_available" \
  --argjson xattr_available "$xattr_available" \
  --argjson apple_id_present "$apple_id_present" \
  --argjson apple_password_present "$apple_password_present" \
  --argjson apple_team_id_present "$apple_team_id_present" \
  --argjson notary_profile_present "$notary_profile_present" \
  --arg notary_profile_lookup_output_sha "$notary_profile_lookup_output_sha" \
  --argjson notary_profile_lookup_output_bytes "$notary_profile_lookup_output_bytes" \
  --argjson notary_profile_keychain_lookup_performed "$notary_profile_keychain_lookup_performed" \
  --argjson notary_profile_keychain_item_found "$notary_profile_keychain_item_found" \
  --argjson build_script_supports_notary_profile "$build_script_supports_notary_profile" \
  --argjson build_script_writes_artifact_receipt "$build_script_writes_artifact_receipt" \
  --argjson build_script_spctl_hard_fail "$build_script_spctl_hard_fail" \
  '
  def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
  def distribution_tools_ready:
    $codesign_available
    and $security_available
    and $xcrun_available
    and $notarytool_available
    and $stapler_available
    and $cargo_packager_available
    and $hdiutil_available
    and $ditto_available
    and $spctl_available
    and $xattr_available;
  def configured_identity_ready:
    ($signing_identity | length) > 0
    and ($configured_team_id | test("^[A-Z0-9]{10}$"))
    and $bundle_identifier == "ai.hepta.nativeapp"
    and $bundle_executable == "hepta-native"
    and $bundle_name == "Hepta Native";
  def keychain_identity_ready:
    $security_available
    and $valid_identity_count > 0
    and $matching_identity_count > 0;
  def notary_env_ready:
    $apple_id_present
    and $apple_password_present
    and $apple_team_id_present;
  def notary_profile_ready:
    $notary_profile_present
    and $build_script_supports_notary_profile
    and $notary_profile_keychain_lookup_performed
    and $notary_profile_keychain_item_found;
  def notary_credentials_ready:
    notary_env_ready
    or notary_profile_ready;
  (
    distribution_tools_ready
    and configured_identity_ready
    and sha_ready($cargo_sha)
    and sha_ready($dmg_script_sha)
    and sha_ready($info_sha)
    and sha_ready($entitlements_sha)
    and sha_ready($identity_output_sha)
    and $identity_output_bytes > 0
  ) as $audit_ready
  | (keychain_identity_ready and notary_credentials_ready and distribution_tools_ready) as $execution_prerequisites_ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $audit_ready then "ready" else "failed" end),
      release_signing_capability_gate_ready:$audit_ready,
      capability_kind:"local_release_signing_notary_prerequisite_audit",
      capability_version:1,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      capability_dir:$capability_dir,
      markdown_path:$markdown_path,
      inspected_files:{
        cargo_toml:$cargo_toml,
        dmg_script:$dmg_script,
        info_plist:$info_plist,
        entitlements_plist:$entitlements_plist
      },
      inspected_file_sha256:{
        cargo_toml:$cargo_sha,
        dmg_script:$dmg_script_sha,
        info_plist:$info_sha,
        entitlements_plist:$entitlements_sha
      },
      package_metadata:{
        signing_identity_configured:(($signing_identity | length) > 0),
        signing_identity:$signing_identity,
        configured_team_id:$configured_team_id,
        bundle_identifier:$bundle_identifier,
        bundle_executable:$bundle_executable,
        bundle_name:$bundle_name
      },
      local_tooling:{
        distribution_tools_ready:distribution_tools_ready,
        codesign_available:$codesign_available,
        security_available:$security_available,
        xcrun_available:$xcrun_available,
        notarytool_available:$notarytool_available,
        stapler_available:$stapler_available,
        cargo_packager_available:$cargo_packager_available,
        hdiutil_available:$hdiutil_available,
        ditto_available:$ditto_available,
        spctl_available:$spctl_available,
        xattr_available:$xattr_available
      },
      keychain_identity_lookup:{
        performed:$security_available,
        raw_output_persisted:false,
        output_sha256:$identity_output_sha,
        output_bytes:$identity_output_bytes,
        valid_identity_count:$valid_identity_count,
        matching_configured_identity_count:$matching_identity_count,
        configured_identity_available:keychain_identity_ready
      },
      notary_credentials:{
        required_env_names:["APPLE_ID","APPLE_PASSWORD","APPLE_TEAM_ID"],
        preferred_keychain_profile_env_name:"HEPTA_NATIVE_NOTARYTOOL_PROFILE",
        credential_value_captured:false,
        apple_id_present:$apple_id_present,
        apple_password_present:$apple_password_present,
        apple_team_id_present:$apple_team_id_present,
        all_required_env_present:notary_env_ready,
        optional_keychain_profile_env_name:"HEPTA_NATIVE_NOTARYTOOL_PROFILE",
        optional_keychain_profile_present:$notary_profile_present,
        optional_keychain_profile_keychain_lookup_performed:$notary_profile_keychain_lookup_performed,
        optional_keychain_profile_keychain_item_found:$notary_profile_keychain_item_found,
        optional_keychain_profile_lookup_raw_output_persisted:false,
        optional_keychain_profile_lookup_output_sha256:$notary_profile_lookup_output_sha,
        optional_keychain_profile_lookup_output_bytes:$notary_profile_lookup_output_bytes,
        build_script_uses_direct_apple_env:true,
        build_script_supports_keychain_profile:$build_script_supports_notary_profile,
        keychain_profile_ready:notary_profile_ready,
        notary_credentials_ready:notary_credentials_ready
      },
      release_script_capabilities:{
        notary_keychain_profile_supported:$build_script_supports_notary_profile,
        release_artifact_receipt_output_supported:$build_script_writes_artifact_receipt,
        spctl_failure_is_hard_failure:$build_script_spctl_hard_fail
      },
      release_execution_prerequisites:{
        configured_identity_ready:configured_identity_ready,
        keychain_identity_ready:keychain_identity_ready,
        notary_env_ready:notary_env_ready,
        notary_keychain_profile_ready:notary_profile_ready,
        notary_credentials_ready:notary_credentials_ready,
        release_signing_execution_prerequisites_ready:$execution_prerequisites_ready
      },
      blockers:[
        (if keychain_identity_ready then empty else "developer_id_identity_missing_or_not_matching_configured_identity" end),
        (if notary_credentials_ready then empty else "apple_notary_credentials_missing" end),
        (if distribution_tools_ready then empty else "local_distribution_tooling_missing" end),
        "signed_notarized_stapled_artifact_missing",
        "public_distribution_artifact_not_written"
      ],
      next_validation_sequence:[
        "install_or_unlock_configured_developer_id_application_identity",
        "provide_apple_notary_keychain_profile_or_environment_without_persisting_secret_values",
        "run_apps_hepta_native_packaging_build_macos_dmg",
        "validate_codesign_notarytool_stapler_spctl_outputs",
        "feed_signed_notarized_stapled_artifact_receipt_into_release_artifact_intake",
        "rerun_hepta_ui_product_readiness_gate"
      ],
      release_execution_handoff:{
        handoff_kind:"operator_supplied_release_signing_notary_artifact_handoff",
        handoff_ready:$audit_ready,
        execution_prerequisites_ready:$execution_prerequisites_ready,
        required_operator_inputs:[
          "configured_developer_id_application_identity_available_in_keychain",
          "apple_notary_credentials_as_environment_or_notarytool_keychain_profile",
          "explicit_release_execution_command",
          "release_artifact_receipt_path",
          "signed_notarized_stapled_local_distribution_artifact_receipt"
        ],
        supported_credential_modes:{
          direct_environment:{
            env_names:["APPLE_ID","APPLE_PASSWORD","APPLE_TEAM_ID"],
            all_required_env_present:notary_env_ready
          },
          notarytool_keychain_profile:{
            profile_env_name:"HEPTA_NATIVE_NOTARYTOOL_PROFILE",
            supported_by_build_script:$build_script_supports_notary_profile,
            profile_env_present:$notary_profile_present,
            keychain_profile_ready:notary_profile_ready
          }
        },
        receipt_contract:{
          release_artifact_receipt_output_env_name:"HEPTA_NATIVE_RELEASE_ARTIFACT_RECEIPT_PATH",
          release_artifact_receipt_output_supported:$build_script_writes_artifact_receipt,
          release_artifact_intake_input_env_name:"HEPTA_UI_RELEASE_ARTIFACT_INPUT_PATH",
          release_artifact_intake_gate:"scripts/hepta-ui-release-artifact-intake-gate.sh",
          post_artifact_refresh_gate:"scripts/hepta-ui-product-readiness-gate.sh",
          expected_artifact_kind:"signed_notarized_stapled_artifact",
          public_upload_performed_must_be_false:true,
          public_distribution_claim_requires_intake_refresh:true
        },
        local_verification_commands:[
          "scripts/hepta-ui-release-signing-capability-gate.sh",
          "apps/hepta-native/packaging/build-macos-dmg.sh",
          "scripts/hepta-ui-release-artifact-intake-gate.sh",
          "scripts/hepta-ui-product-readiness-gate.sh"
        ],
        side_effects_must_remain_false_until_explicit_release_command:[
          "credential_value_captured",
          "network_call_performed",
          "notary_submission_performed",
          "app_signed",
          "app_notarized",
          "app_stapled",
          "public_distribution_artifact_written",
          "external_mutation",
          "active_binary_mutation",
          "install_or_restart"
        ],
        claim_boundary_after_handoff:{
          release_artifact_claim_ready:false,
          release_execution_ready:false,
          public_distribution_claim_ready:false,
          release_claim_ready:false,
          live_product_claim_ready:false
        }
      },
      claim_boundary:{
        local_release_signing_capability_audit_ready:$audit_ready,
        release_signing_execution_prerequisites_ready:$execution_prerequisites_ready,
        release_artifact_claim_ready:false,
        release_execution_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false,
        live_product_claim_ready:false
      },
      side_effects:{
        filesystem_read:true,
        local_report_written:true,
        local_markdown_written:true,
        keychain_identity_lookup_performed:$security_available,
        keychain_identity_lookup_raw_output_persisted:false,
        notary_profile_keychain_lookup_performed:$notary_profile_keychain_lookup_performed,
        notary_profile_keychain_lookup_raw_output_persisted:false,
        credential_value_captured:false,
        network_call_performed:false,
        notary_submission_performed:false,
        app_signed:false,
        app_notarized:false,
        app_stapled:false,
        public_distribution_artifact_written:false,
        external_mutation:false
      }
    }' >"$REPORT_TMP"

jq -r '
  "# Hepta UI Release Signing Capability\n\n"
  + "- Gate: `\(.gate)`\n"
  + "- Status: `\(.status)`\n"
  + "- Configured identity: `\(.package_metadata.signing_identity)`\n"
  + "- Valid code-signing identities: \(.keychain_identity_lookup.valid_identity_count)\n"
  + "- Matching configured identities: \(.keychain_identity_lookup.matching_configured_identity_count)\n"
  + "- Apple notary env present: \(.notary_credentials.all_required_env_present)\n"
  + "- Apple notary keychain profile present: \(.notary_credentials.optional_keychain_profile_present)\n"
  + "- Apple notary keychain profile item found: \(.notary_credentials.optional_keychain_profile_keychain_item_found)\n"
  + "- Apple notary credentials ready: \(.notary_credentials.notary_credentials_ready)\n"
  + "- Release signing prerequisites ready: \(.release_execution_prerequisites.release_signing_execution_prerequisites_ready)\n"
  + "- Release receipt output supported: \(.release_script_capabilities.release_artifact_receipt_output_supported)\n"
  + "- `spctl` failure is hard failure: \(.release_script_capabilities.spctl_failure_is_hard_failure)\n"
  + "- Handoff ready: \(.release_execution_handoff.handoff_ready)\n"
  + "- Release artifact receipt output env: `\(.release_execution_handoff.receipt_contract.release_artifact_receipt_output_env_name)`\n"
  + "- Release artifact intake input env: `\(.release_execution_handoff.receipt_contract.release_artifact_intake_input_env_name)`\n"
  + "- Release/public/live claims remain false.\n\n"
  + "## Blockers\n\n"
  + (.blockers | map("- `" + . + "`") | join("\n"))
  + "\n\n## Next Validation Sequence\n\n"
  + (.next_validation_sequence | map("- `" + . + "`") | join("\n"))
  + "\n"
' "$REPORT_TMP" >"$MARKDOWN_TMP"

markdown_sha="$(file_sha256 "$MARKDOWN_TMP")"
markdown_bytes="$(file_bytes "$MARKDOWN_TMP")"

jq \
  --arg markdown_sha "$markdown_sha" \
  --argjson markdown_bytes "$markdown_bytes" \
  '. + {markdown_sha256:$markdown_sha, markdown_bytes:$markdown_bytes}' \
  "$REPORT_TMP" >"$REPORT_TMP.with-markdown"
mv "$REPORT_TMP.with-markdown" "$REPORT_TMP"

jq -e '
  .status == "ready"
  and .release_signing_capability_gate_ready == true
  and .capability_kind == "local_release_signing_notary_prerequisite_audit"
  and .package_metadata.signing_identity_configured == true
  and .package_metadata.bundle_identifier == "ai.hepta.nativeapp"
  and .package_metadata.bundle_executable == "hepta-native"
  and .package_metadata.bundle_name == "Hepta Native"
  and .local_tooling.distribution_tools_ready == true
  and .keychain_identity_lookup.performed == true
  and (.keychain_identity_lookup.output_sha256 | test("^[0-9a-f]{64}$"))
  and .keychain_identity_lookup.output_bytes > 0
  and (.release_execution_prerequisites.keychain_identity_ready | type) == "boolean"
  and (.release_execution_prerequisites.notary_env_ready | type) == "boolean"
  and (.release_execution_prerequisites.notary_keychain_profile_ready | type) == "boolean"
  and (.release_execution_prerequisites.notary_credentials_ready | type) == "boolean"
  and (.release_execution_prerequisites.release_signing_execution_prerequisites_ready | type) == "boolean"
  and (.notary_credentials.optional_keychain_profile_keychain_lookup_performed | type) == "boolean"
  and (.notary_credentials.optional_keychain_profile_keychain_item_found | type) == "boolean"
  and (.notary_credentials.optional_keychain_profile_lookup_output_sha256 | test("^[0-9a-f]{64}$"))
  and .notary_credentials.optional_keychain_profile_lookup_output_bytes > 0
  and .notary_credentials.optional_keychain_profile_lookup_raw_output_persisted == false
  and .release_script_capabilities.notary_keychain_profile_supported == true
  and .release_script_capabilities.release_artifact_receipt_output_supported == true
  and .release_script_capabilities.spctl_failure_is_hard_failure == true
  and .release_execution_handoff.handoff_kind == "operator_supplied_release_signing_notary_artifact_handoff"
  and .release_execution_handoff.handoff_ready == true
  and (.release_execution_handoff.execution_prerequisites_ready | type) == "boolean"
  and (.release_execution_handoff.required_operator_inputs | length) == 5
  and .release_execution_handoff.supported_credential_modes.direct_environment.env_names == ["APPLE_ID","APPLE_PASSWORD","APPLE_TEAM_ID"]
  and .release_execution_handoff.supported_credential_modes.notarytool_keychain_profile.profile_env_name == "HEPTA_NATIVE_NOTARYTOOL_PROFILE"
  and .release_execution_handoff.supported_credential_modes.notarytool_keychain_profile.supported_by_build_script == true
  and .release_execution_handoff.receipt_contract.release_artifact_receipt_output_env_name == "HEPTA_NATIVE_RELEASE_ARTIFACT_RECEIPT_PATH"
  and .release_execution_handoff.receipt_contract.release_artifact_receipt_output_supported == true
  and .release_execution_handoff.receipt_contract.release_artifact_intake_input_env_name == "HEPTA_UI_RELEASE_ARTIFACT_INPUT_PATH"
  and .release_execution_handoff.receipt_contract.release_artifact_intake_gate == "scripts/hepta-ui-release-artifact-intake-gate.sh"
  and .release_execution_handoff.receipt_contract.post_artifact_refresh_gate == "scripts/hepta-ui-product-readiness-gate.sh"
  and .release_execution_handoff.receipt_contract.expected_artifact_kind == "signed_notarized_stapled_artifact"
  and .release_execution_handoff.receipt_contract.public_upload_performed_must_be_false == true
  and (.release_execution_handoff.local_verification_commands | index("apps/hepta-native/packaging/build-macos-dmg.sh") != null)
  and (.release_execution_handoff.side_effects_must_remain_false_until_explicit_release_command | index("credential_value_captured") != null)
  and (.release_execution_handoff.side_effects_must_remain_false_until_explicit_release_command | index("notary_submission_performed") != null)
  and (.release_execution_handoff.side_effects_must_remain_false_until_explicit_release_command | index("active_binary_mutation") != null)
  and .release_execution_handoff.claim_boundary_after_handoff.release_artifact_claim_ready == false
  and .release_execution_handoff.claim_boundary_after_handoff.release_execution_ready == false
  and .release_execution_handoff.claim_boundary_after_handoff.public_distribution_claim_ready == false
  and .release_execution_handoff.claim_boundary_after_handoff.release_claim_ready == false
  and .release_execution_handoff.claim_boundary_after_handoff.live_product_claim_ready == false
  and .claim_boundary.local_release_signing_capability_audit_ready == true
  and .claim_boundary.release_artifact_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.live_product_claim_ready == false
  and (.markdown_sha256 | test("^[0-9a-f]{64}$"))
  and .markdown_bytes > 0
  and .side_effects.keychain_identity_lookup_performed == true
  and .side_effects.keychain_identity_lookup_raw_output_persisted == false
  and (.side_effects.notary_profile_keychain_lookup_performed | type) == "boolean"
  and .side_effects.notary_profile_keychain_lookup_raw_output_persisted == false
  and .side_effects.credential_value_captured == false
  and .side_effects.network_call_performed == false
  and .side_effects.notary_submission_performed == false
  and .side_effects.app_signed == false
  and .side_effects.app_notarized == false
  and .side_effects.app_stapled == false
  and .side_effects.public_distribution_artifact_written == false
  and .side_effects.external_mutation == false
' "$REPORT_TMP" >/dev/null

cp "$REPORT_TMP" "$REPORT_PATH"
cp "$MARKDOWN_TMP" "$MARKDOWN_PATH"
cat "$REPORT_PATH"
