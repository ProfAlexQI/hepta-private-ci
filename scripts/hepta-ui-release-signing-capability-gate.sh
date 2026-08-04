#!/bin/bash -p
set +x
PS4='+ '
set -euo pipefail

CAPTURED_SIGNING_IDENTITY="${HEPTA_SIGNING_IDENTITY:-}"
CAPTURED_EXPECTED_TEAM_ID="${HEPTA_EXPECTED_TEAM_ID:-}"
CAPTURED_NOTARY_PROFILE="${HEPTA_NOTARY_PROFILE:-${HEPTA_NATIVE_NOTARYTOOL_PROFILE:-}}"
CAPTURED_APPLE_TEAM_ID="${APPLE_TEAM_ID:-}"
CAPTURED_APPLE_ID_PRESENT=false
CAPTURED_APPLE_PASSWORD_PRESENT=false
CAPTURED_APPLE_TEAM_ID_PRESENT=false
[[ -z "${APPLE_ID:-}" ]] || CAPTURED_APPLE_ID_PRESENT=true
[[ -z "${APPLE_PASSWORD:-}" ]] || CAPTURED_APPLE_PASSWORD_PRESENT=true
[[ -z "${APPLE_TEAM_ID:-}" ]] || CAPTURED_APPLE_TEAM_ID_PRESENT=true
unset HEPTA_SIGNING_IDENTITY HEPTA_EXPECTED_TEAM_ID
unset HEPTA_NOTARY_PROFILE HEPTA_NATIVE_NOTARYTOOL_PROFILE
unset APPLE_ID APPLE_PASSWORD APPLE_TEAM_ID
unset BASH_ENV ENV CDPATH GLOBIGNORE
unset RUBYOPT RUBYLIB GEM_HOME GEM_PATH BUNDLE_GEMFILE BUNDLE_PATH
export -n CAPTURED_SIGNING_IDENTITY CAPTURED_EXPECTED_TEAM_ID CAPTURED_NOTARY_PROFILE CAPTURED_APPLE_TEAM_ID
export -n CAPTURED_APPLE_ID_PRESENT CAPTURED_APPLE_PASSWORD_PRESENT CAPTURED_APPLE_TEAM_ID_PRESENT
SYSTEM_PATH="/usr/bin:/bin:/usr/sbin:/sbin"
CANONICAL_HOME="$(/usr/bin/ruby -retc -e 'print Etc.getpwuid(Process.uid).dir')"
[[ "$CANONICAL_HOME" == /* && -d "$CANONICAL_HOME" && ! -L "$CANONICAL_HOME" ]] || exit 2
PATH="$SYSTEM_PATH"
HOME="$CANONICAL_HOME"
TMPDIR="/private/tmp"
export PATH HOME TMPDIR

cd "$(/usr/bin/dirname "$0")/.."
REPO_ROOT="$(pwd -P)"
. "$REPO_ROOT/scripts/lib/hepta-safe-managed-output-v1.sh"

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
BUNDLE_FINGERPRINT_HELPER="$PACKAGING_DIR/app-bundle-fingerprint-v1.rb"
FINDER_BOOKMARK_RESOLVER="$PACKAGING_DIR/resolve-finder-bookmark-v1.swift"
RELEASE_APPROVAL_VERIFIER="scripts/hepta-ui-release-execution-approval-verifier-v1"
RELEASE_APPROVAL_TRUST_POLICY="$PACKAGING_DIR/release-execution-approval-trust-v1.json"

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
  if [[ -x /usr/bin/xcrun && ! -L /usr/bin/xcrun ]] && /usr/bin/xcrun -f "$1" >/dev/null 2>&1; then
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

require_command awk
require_command jq
require_command plutil
require_command ruby
require_command shasum
require_command swift
require_command swiftc

READINESS_DIR="$(hepta_safe_normalize_path readiness "$READINESS_DIR")"
REPORT_PATH="$(hepta_safe_normalize_path report "$REPORT_PATH")"
CAPABILITY_DIR="$(hepta_safe_normalize_path capability "$CAPABILITY_DIR")"
MARKDOWN_PATH="$CAPABILITY_DIR/release-signing-capability.md"
REPORT_PARENT="$(hepta_safe_normalize_path report_parent "$(/usr/bin/dirname "$REPORT_PATH")")"
hepta_safe_require_directory_target readiness "$READINESS_DIR"
hepta_safe_require_directory_target capability "$CAPABILITY_DIR"
hepta_safe_require_directory_target report_parent "$REPORT_PARENT"
hepta_safe_require_regular_target report "$REPORT_PATH"
hepta_safe_require_regular_target capability_markdown "$MARKDOWN_PATH"
if hepta_safe_paths_overlap "$READINESS_DIR" "$REPO_ROOT"; then
  printf 'release signing readiness must not overlap the repository\n' >&2
  exit 64
fi
if ! hepta_safe_is_strict_descendant "$CAPABILITY_DIR" "$READINESS_DIR"; then
  printf 'release signing capability directory must be a strict readiness child\n' >&2
  exit 64
fi
if [[ "$REPORT_PARENT" != "$READINESS_DIR" ]] \
  && ! hepta_safe_is_strict_descendant "$REPORT_PARENT" "$READINESS_DIR"; then
  printf 'release signing report parent must remain inside readiness\n' >&2
  exit 64
fi
if hepta_safe_paths_overlap "$REPORT_PATH" "$CAPABILITY_DIR"; then
  printf 'release signing report and managed directory must be disjoint\n' >&2
  exit 64
fi
for protected_input in \
  "$CARGO_TOML" "$DMG_SCRIPT" "$INFO_PLIST" "$ENTITLEMENTS_PLIST" \
  "$BUNDLE_FINGERPRINT_HELPER" "$FINDER_BOOKMARK_RESOLVER" \
  "$RELEASE_APPROVAL_VERIFIER" "$RELEASE_APPROVAL_TRUST_POLICY"; do
  if hepta_safe_paths_overlap "$protected_input" "$CAPABILITY_DIR" \
    || hepta_safe_paths_overlap "$protected_input" "$REPORT_PATH"; then
    printf 'release signing output overlaps protected source: %s\n' "$protected_input" >&2
    exit 64
  fi
done

for required in "$CARGO_TOML" "$DMG_SCRIPT" "$INFO_PLIST" "$ENTITLEMENTS_PLIST" "$BUNDLE_FINGERPRINT_HELPER" "$FINDER_BOOKMARK_RESOLVER" "$RELEASE_APPROVAL_VERIFIER" "$RELEASE_APPROVAL_TRUST_POLICY"; do
  if [[ ! -s "$required" ]]; then
    echo "missing required release signing capability input: $required" >&2
    exit 1
  fi
done

bash -n "$DMG_SCRIPT"
bash -n "$RELEASE_APPROVAL_VERIFIER"
ruby -c "$BUNDLE_FINGERPRINT_HELPER" >/dev/null
swiftc -parse "$FINDER_BOOKMARK_RESOLVER"
plutil -lint "$INFO_PLIST" "$ENTITLEMENTS_PLIST" >/dev/null

/bin/mkdir -p "$CAPABILITY_DIR" "$REPORT_PARENT"
hepta_safe_revalidate_directory capability "$CAPABILITY_DIR"
hepta_safe_revalidate_directory report_parent "$REPORT_PARENT"

TMP_DIR="$(/usr/bin/mktemp -d /private/tmp/hepta-ui-release-signing-capability.XXXXXX)"
IDENTITIES_RAW="$TMP_DIR/codesigning-identities.txt"
NOTARY_PROFILE_LOOKUP_RAW="$TMP_DIR/notary-profile-keychain-lookup.txt"
REPORT_TMP="$TMP_DIR/release-signing-capability-report.json"
MARKDOWN_TMP="$TMP_DIR/release-signing-capability.md"
trap '/bin/rm -rf "$TMP_DIR"' EXIT

signing_identity="${CAPTURED_SIGNING_IDENTITY:-$(awk -F '"' '/^signing_identity[[:space:]]*=/ { print $2; exit }' "$CARGO_TOML")}"
configured_team_id=""
if [[ "$signing_identity" =~ ^Developer[[:space:]]ID[[:space:]]Application:.+[[:space:]]\(([A-Z0-9]{10})\)$ ]]; then
  configured_team_id="${BASH_REMATCH[1]}"
fi
trusted_team_id="$CAPTURED_EXPECTED_TEAM_ID"
if [[ "$trusted_team_id" =~ ^[A-Z0-9]{10}$ ]]; then
  trusted_team_id_valid=true
else
  trusted_team_id_valid=false
fi
if [[ -n "$configured_team_id" && "$configured_team_id" == "$trusted_team_id" ]]; then
  configured_team_id_matches_trusted=true
else
  configured_team_id_matches_trusted=false
fi
bundle_identifier="$(plist_value "$INFO_PLIST" "CFBundleIdentifier")"
bundle_executable="$(plist_value "$INFO_PLIST" "CFBundleExecutable")"
bundle_name="$(plist_value "$INFO_PLIST" "CFBundleName")"
codesign_available="$(bool_command codesign)"
security_available="$(bool_command security)"
xcrun_available="$(bool_command xcrun)"
notarytool_available="$(bool_xcrun_tool notarytool)"
stapler_available="$(bool_xcrun_tool stapler)"
hdiutil_available="$(bool_command hdiutil)"
ditto_available="$(bool_command ditto)"
spctl_available="$(bool_command spctl)"
xattr_available="$(bool_command xattr)"
mount_available="$(bool_command mount)"
ruby_available="$(bool_command ruby)"
swift_available="$(bool_command swift)"

if [[ "$security_available" == "true" ]]; then
  /usr/bin/security find-identity -p codesigning -v >"$IDENTITIES_RAW" 2>&1 || true
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
if [[ -n "$signing_identity" && "$trusted_team_id_valid" == "true" ]]; then
  matching_identity_count="$(/usr/bin/ruby -e '
    expected_identity, expected_team, path = ARGV
    count = File.readlines(path, chomp: true).count do |line|
      match = line.match(/^\s*\d+\)\s+[0-9A-Fa-f]{40}\s+"([^"]+)"\s*$/)
      next false unless match && match[1] == expected_identity
      team = match[1].match(/\(([A-Z0-9]{10})\)\z/)
      team && team[1] == expected_team
    end
    print count
  ' "$signing_identity" "$trusted_team_id" "$IDENTITIES_RAW")"
fi

apple_id_present="$CAPTURED_APPLE_ID_PRESENT"
apple_password_present="$CAPTURED_APPLE_PASSWORD_PRESENT"
apple_team_id_present="$CAPTURED_APPLE_TEAM_ID_PRESENT"
direct_apple_credentials_present=false
if [[ "$apple_id_present" == true || "$apple_password_present" == true || "$apple_team_id_present" == true ]]; then direct_apple_credentials_present=true; fi
apple_team_id_matches_trusted=false
if [[ "$apple_team_id_present" == false || "$CAPTURED_APPLE_TEAM_ID" == "$trusted_team_id" ]]; then apple_team_id_matches_trusted=true; fi
notary_profile_name="$CAPTURED_NOTARY_PROFILE"
if [[ -n "$notary_profile_name" ]]; then notary_profile_present=true; else notary_profile_present=false; fi
build_script_supports_notary_profile="$(grep -Fq "HEPTA_NATIVE_NOTARYTOOL_PROFILE" "$DMG_SCRIPT" && printf 'true' || printf 'false')"
build_script_writes_artifact_receipt="$(grep -Fq "HEPTA_NATIVE_RELEASE_ARTIFACT_RECEIPT_PATH" "$DMG_SCRIPT" && printf 'true' || printf 'false')"
build_script_spctl_hard_fail="$(grep -Fq "spctl assessment failed" "$DMG_SCRIPT" && printf 'true' || printf 'false')"
build_script_consumes_exact_formal_app="$(grep -Fq 'consumed_exact_formal_app:true' "$DMG_SCRIPT" && grep -Fq 'built_second_product_app:false' "$DMG_SCRIPT" && printf 'true' || printf 'false')"
build_script_requires_unsigned_receipt="$(grep -Fq 'HEPTA_NATIVE_UNSIGNED_APP_RECEIPT_PATH' "$DMG_SCRIPT" && grep -Fq 'scripts/hepta-native-current-package-gate.sh' "$DMG_SCRIPT" && printf 'true' || printf 'false')"
build_script_requires_accepted_notary_result="$(grep -Fq '.status == "Accepted"' "$DMG_SCRIPT" && printf 'true' || printf 'false')"
build_script_binds_full_bundle_fingerprint="$(grep -Fq 'source_app_bundle_fingerprint' "$DMG_SCRIPT" && grep -Fq 'formal_app_contains_symlinks_or_unsupported_entries' "$DMG_SCRIPT" && printf 'true' || printf 'false')"
build_script_recomputes_private_copy="$(grep -Fq 'PRIVATE_COPY_FINGERPRINT' "$DMG_SCRIPT" && grep -Fq 'release_input_changed_before_signing' "$DMG_SCRIPT" && printf 'true' || printf 'false')"
build_script_uses_nonlaunch_release_input="$(grep -Fq -- '--no-launch' "$DMG_SCRIPT" && printf 'true' || printf 'false')"
build_script_verifies_readonly_dmg_payload="$(grep -Fq 'dmg_mounted_read_only:true' "$DMG_SCRIPT" && grep -Fq 'mounted_app_bundle_fingerprint' "$DMG_SCRIPT" && grep -Fq 'applications_alias_resolved_target' "$DMG_SCRIPT" && grep -Fq 'resolve-finder-bookmark-v1.swift' "$DMG_SCRIPT" && printf 'true' || printf 'false')"
build_script_v3_notary_truth="$(grep -Fq 'artifact_version:3' "$DMG_SCRIPT" && grep -Fq 'notary_submission_may_have_occurred' "$DMG_SCRIPT" && grep -Fq 'notarytool_submit_log_bytes' "$DMG_SCRIPT" && printf 'true' || printf 'false')"
build_script_actual_execution_approval_guard="$(grep -Fq 'signed_release_execution_approval_missing: actual release execution is disabled; run --preflight only' "$DMG_SCRIPT" && grep -Fq 'if [[ "$PREFLIGHT_ONLY" != "1" ]]' "$DMG_SCRIPT" && printf 'true' || printf 'false')"
independent_approval_verifier_ready=false
if [[ -f "$RELEASE_APPROVAL_VERIFIER" && ! -L "$RELEASE_APPROVAL_VERIFIER" && -x "$RELEASE_APPROVAL_VERIFIER" ]] \
  && grep -Fq 'hepta-ui-release-execution-approval-verification-v1' "$RELEASE_APPROVAL_VERIFIER"; then
  independent_approval_verifier_ready=true
fi
release_approval_trust_configured=false
if jq -e '
  .schema_version == 1
  and .kind == "hepta-ui-release-execution-approval-trust-v1"
  and .status == "ready"
  and (.signer_id | type == "string" and length > 0)
  and (.public_key_sha256 | type == "string" and test("^[0-9a-f]{64}$"))
  and .signature_algorithm == "rsa-pkcs1-sha256"
  and (.minimum_rsa_bits | type == "number" and . >= 3072)
' "$RELEASE_APPROVAL_TRUST_POLICY" >/dev/null 2>&1; then
  release_approval_trust_configured=true
fi

notary_profile_keychain_lookup_performed=false
notary_profile_keychain_item_found=false
if [[ "$security_available" == "true" && "$notary_profile_present" == "true" ]]; then
  notary_profile_keychain_lookup_performed=true
  {
    printf 'profile keychain lookup: service=com.apple.gke.notary.tool account=<profile-env>\n'
    set +e
    /usr/bin/security find-generic-password \
      -s "com.apple.gke.notary.tool" \
      -a "$notary_profile_name"
    profile_lookup_status_a=$?
    printf 'profile keychain lookup: service=com.apple.gke.notary.tool.<profile-env>\n'
    /usr/bin/security find-generic-password \
      -s "com.apple.gke.notary.tool.$notary_profile_name"
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
bundle_fingerprint_helper_sha="$(file_sha256 "$BUNDLE_FINGERPRINT_HELPER")"
finder_bookmark_resolver_sha="$(file_sha256 "$FINDER_BOOKMARK_RESOLVER")"

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
  --arg bundle_fingerprint_helper "$BUNDLE_FINGERPRINT_HELPER" \
  --arg finder_bookmark_resolver "$FINDER_BOOKMARK_RESOLVER" \
  --arg cargo_sha "$cargo_sha" \
  --arg dmg_script_sha "$dmg_script_sha" \
  --arg info_sha "$info_sha" \
  --arg entitlements_sha "$entitlements_sha" \
  --arg bundle_fingerprint_helper_sha "$bundle_fingerprint_helper_sha" \
  --arg finder_bookmark_resolver_sha "$finder_bookmark_resolver_sha" \
  --arg signing_identity "$signing_identity" \
  --arg configured_team_id "$configured_team_id" \
  --arg trusted_team_id "$trusted_team_id" \
  --argjson trusted_team_id_valid "$trusted_team_id_valid" \
  --argjson configured_team_id_matches_trusted "$configured_team_id_matches_trusted" \
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
  --argjson hdiutil_available "$hdiutil_available" \
  --argjson ditto_available "$ditto_available" \
  --argjson spctl_available "$spctl_available" \
  --argjson xattr_available "$xattr_available" \
  --argjson mount_available "$mount_available" \
  --argjson ruby_available "$ruby_available" \
  --argjson swift_available "$swift_available" \
  --argjson apple_id_present "$apple_id_present" \
  --argjson apple_password_present "$apple_password_present" \
  --argjson apple_team_id_present "$apple_team_id_present" \
  --argjson direct_apple_credentials_present "$direct_apple_credentials_present" \
  --argjson apple_team_id_matches_trusted "$apple_team_id_matches_trusted" \
  --argjson notary_profile_present "$notary_profile_present" \
  --arg notary_profile_lookup_output_sha "$notary_profile_lookup_output_sha" \
  --argjson notary_profile_lookup_output_bytes "$notary_profile_lookup_output_bytes" \
  --argjson notary_profile_keychain_lookup_performed "$notary_profile_keychain_lookup_performed" \
  --argjson notary_profile_keychain_item_found "$notary_profile_keychain_item_found" \
  --argjson build_script_supports_notary_profile "$build_script_supports_notary_profile" \
  --argjson build_script_writes_artifact_receipt "$build_script_writes_artifact_receipt" \
  --argjson build_script_spctl_hard_fail "$build_script_spctl_hard_fail" \
  --argjson build_script_consumes_exact_formal_app "$build_script_consumes_exact_formal_app" \
  --argjson build_script_requires_unsigned_receipt "$build_script_requires_unsigned_receipt" \
  --argjson build_script_requires_accepted_notary_result "$build_script_requires_accepted_notary_result" \
  --argjson build_script_binds_full_bundle_fingerprint "$build_script_binds_full_bundle_fingerprint" \
  --argjson build_script_recomputes_private_copy "$build_script_recomputes_private_copy" \
  --argjson build_script_uses_nonlaunch_release_input "$build_script_uses_nonlaunch_release_input" \
  --argjson build_script_verifies_readonly_dmg_payload "$build_script_verifies_readonly_dmg_payload" \
  --argjson build_script_v3_notary_truth "$build_script_v3_notary_truth" \
  --argjson build_script_actual_execution_approval_guard "$build_script_actual_execution_approval_guard" \
  --argjson independent_approval_verifier_ready "$independent_approval_verifier_ready" \
  --argjson release_approval_trust_configured "$release_approval_trust_configured" \
  '
  def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
  def distribution_tools_ready:
    $codesign_available
    and $security_available
    and $xcrun_available
    and $notarytool_available
    and $stapler_available
    and $hdiutil_available
    and $ditto_available
    and $spctl_available
    and $xattr_available
    and $mount_available
    and $ruby_available
    and $swift_available;
  def bundle_and_release_script_contract_ready:
    $bundle_identifier == "ai.hepta.nativeapp"
    and $bundle_executable == "hepta-native"
    and $bundle_name == "Hepta"
    and $build_script_consumes_exact_formal_app
    and $build_script_requires_unsigned_receipt
    and $build_script_requires_accepted_notary_result
    and $build_script_binds_full_bundle_fingerprint
    and $build_script_recomputes_private_copy
    and $build_script_uses_nonlaunch_release_input
    and $build_script_verifies_readonly_dmg_payload
    and $build_script_v3_notary_truth
    and $build_script_actual_execution_approval_guard;
  def keychain_identity_ready:
    $security_available
    and $trusted_team_id_valid
    and $configured_team_id_matches_trusted
    and $valid_identity_count > 0
    and $matching_identity_count > 0;
  def notary_env_ready:
    false;
  def notary_profile_ready:
    $notary_profile_present
    and ($direct_apple_credentials_present | not)
    and $build_script_supports_notary_profile
    and $notary_profile_keychain_lookup_performed
    and $notary_profile_keychain_item_found;
  def notary_credentials_ready:
    notary_profile_ready;
  (
    distribution_tools_ready
    and bundle_and_release_script_contract_ready
    and sha_ready($cargo_sha)
    and sha_ready($dmg_script_sha)
    and sha_ready($info_sha)
    and sha_ready($entitlements_sha)
    and sha_ready($bundle_fingerprint_helper_sha)
    and sha_ready($finder_bookmark_resolver_sha)
    and sha_ready($identity_output_sha)
    and $identity_output_bytes > 0
  ) as $audit_ready
  # This audit can prove the verifier/trust capability, but it never consumes
  # a per-run signed approval and therefore cannot make execution ready.
  | false as $execution_prerequisites_ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $audit_ready then "ready" else "failed" end),
      audit_status:(if $audit_ready then "ready" else "failed" end),
      capability_status:(if $execution_prerequisites_ready then "ready" else "blocked" end),
      release_signing_capability_gate_ready:$audit_ready,
      capability_kind:"local_release_signing_notary_prerequisite_audit",
      capability_version:2,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      capability_dir:$capability_dir,
      markdown_path:$markdown_path,
      inspected_files:{
        cargo_toml:$cargo_toml,
        dmg_script:$dmg_script,
        info_plist:$info_plist,
        entitlements_plist:$entitlements_plist,
        bundle_fingerprint_helper:$bundle_fingerprint_helper,
        finder_bookmark_resolver:$finder_bookmark_resolver
      },
      inspected_file_sha256:{
        cargo_toml:$cargo_sha,
        dmg_script:$dmg_script_sha,
        info_plist:$info_sha,
        entitlements_plist:$entitlements_sha,
        bundle_fingerprint_helper:$bundle_fingerprint_helper_sha,
        finder_bookmark_resolver:$finder_bookmark_resolver_sha
      },
      package_metadata:{
        signing_identity_configured:(($signing_identity | length) > 0),
        signing_identity:$signing_identity,
        configured_team_id:$configured_team_id,
        trusted_team_id:$trusted_team_id,
        trusted_team_id_source:"HEPTA_EXPECTED_TEAM_ID",
        trusted_team_id_valid:$trusted_team_id_valid,
        configured_team_id_matches_trusted:$configured_team_id_matches_trusted,
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
        hdiutil_available:$hdiutil_available,
        ditto_available:$ditto_available,
        spctl_available:$spctl_available,
        xattr_available:$xattr_available,
        mount_available:$mount_available,
        ruby_available:$ruby_available,
        swift_available:$swift_available
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
        required_env_names:["HEPTA_EXPECTED_TEAM_ID","HEPTA_NOTARY_PROFILE"],
        preferred_keychain_profile_env_name:"HEPTA_NOTARY_PROFILE",
        credential_value_captured:false,
        apple_id_present:$apple_id_present,
        apple_password_present:$apple_password_present,
        apple_team_id_present:$apple_team_id_present,
        apple_team_id_valid:(if $apple_team_id_present then ($apple_team_id_matches_trusted and $trusted_team_id_valid) else false end),
        apple_team_id_matches_trusted_expected_team:$apple_team_id_matches_trusted,
        direct_apple_credentials_present:$direct_apple_credentials_present,
        direct_environment_supported:false,
        all_required_env_present:notary_env_ready,
        optional_keychain_profile_env_name:"HEPTA_NOTARY_PROFILE",
        optional_keychain_profile_present:$notary_profile_present,
        optional_keychain_profile_keychain_lookup_performed:$notary_profile_keychain_lookup_performed,
        optional_keychain_profile_keychain_item_found:$notary_profile_keychain_item_found,
        optional_keychain_profile_lookup_raw_output_persisted:false,
        optional_keychain_profile_lookup_output_sha256:$notary_profile_lookup_output_sha,
        optional_keychain_profile_lookup_output_bytes:$notary_profile_lookup_output_bytes,
        build_script_uses_direct_apple_env:false,
        build_script_keychain_profile_only:true,
        build_script_supports_keychain_profile:$build_script_supports_notary_profile,
        keychain_profile_ready:notary_profile_ready,
        notary_credentials_ready:notary_credentials_ready
      },
      release_script_capabilities:{
        notary_keychain_profile_supported:$build_script_supports_notary_profile,
        release_artifact_receipt_output_supported:$build_script_writes_artifact_receipt,
        spctl_failure_is_hard_failure:$build_script_spctl_hard_fail,
        consumes_exact_formal_app:$build_script_consumes_exact_formal_app,
        canonical_unsigned_receipt_required:$build_script_requires_unsigned_receipt,
        accepted_notary_result_required:$build_script_requires_accepted_notary_result,
        full_bundle_fingerprint_required:$build_script_binds_full_bundle_fingerprint,
        private_copy_recomputed_before_signing:$build_script_recomputes_private_copy,
        default_release_input_does_not_launch:$build_script_uses_nonlaunch_release_input,
        readonly_dmg_payload_readback_required:$build_script_verifies_readonly_dmg_payload,
        v3_notary_uncertainty_truth_required:$build_script_v3_notary_truth,
        actual_execution_blocked_before_release_side_effects:$build_script_actual_execution_approval_guard,
        preflight_only_supported:true,
        actual_release_execution_supported:false,
        builds_second_product_app:false
      },
      release_execution_prerequisites:{
        bundle_and_release_script_contract_ready:bundle_and_release_script_contract_ready,
        keychain_identity_ready:keychain_identity_ready,
        notary_env_ready:notary_env_ready,
        notary_keychain_profile_ready:notary_profile_ready,
        notary_credentials_ready:notary_credentials_ready,
        independent_approval_verifier_ready:$independent_approval_verifier_ready,
        release_approval_trust_configured:$release_approval_trust_configured,
        signed_release_execution_approval_verified:false,
        release_signing_execution_prerequisites_ready:$execution_prerequisites_ready
      },
      blockers:[
        (if $independent_approval_verifier_ready then empty else "independent_release_approval_verifier_unavailable" end),
        (if $release_approval_trust_configured then empty else "release_approval_trust_not_configured" end),
        "signed_release_execution_approval_missing",
        (if keychain_identity_ready then empty else "developer_id_identity_missing_or_not_matching_configured_identity" end),
        (if $direct_apple_credentials_present then "direct_apple_id_password_notary_mode_unsupported" else empty end),
        (if notary_credentials_ready then empty else "apple_notary_keychain_profile_missing_or_unverified" end),
        (if distribution_tools_ready then empty else "local_distribution_tooling_missing" end),
        "signed_notarized_stapled_artifact_missing",
        "public_distribution_artifact_not_written"
      ],
      next_validation_sequence:[
        "install_or_unlock_configured_developer_id_application_identity",
        "provide_apple_notary_keychain_profile_without_direct_password_environment",
        (if $independent_approval_verifier_ready then empty else "provide_independent_release_approval_verifier_bound_to_exact_source_and_action_tuple" end),
        (if $release_approval_trust_configured then empty else "provision_independent_release_approval_trust_root" end),
        "provide_signed_release_execution_approval_bound_to_exact_source_input_and_action",
        "rerun_build_macos_dmg_preflight_only",
        "rerun_hepta_ui_product_readiness_gate"
      ],
      release_execution_handoff:{
        handoff_kind:"operator_supplied_release_signing_notary_artifact_handoff",
        handoff_ready:$audit_ready,
        execution_prerequisites_ready:$execution_prerequisites_ready,
        required_operator_inputs:[
          "configured_developer_id_application_identity_available_in_keychain",
          "trusted_expected_team_id_and_notarytool_keychain_profile",
          "independently_verified_release_approval_bound_to_exact_source_and_action_tuple",
          "release_artifact_receipt_path",
          "signed_notarized_stapled_local_distribution_artifact_receipt"
        ],
        supported_credential_modes:{
          direct_environment:{
            env_names:["APPLE_ID","APPLE_PASSWORD","APPLE_TEAM_ID"],
            supported:false,
            any_present:$direct_apple_credentials_present,
            all_required_env_present:false
          },
          notarytool_keychain_profile:{
            profile_env_name:"HEPTA_NOTARY_PROFILE",
            supported_by_build_script:$build_script_supports_notary_profile,
            profile_env_present:$notary_profile_present,
            keychain_profile_ready:notary_profile_ready
          }
        },
        receipt_contract:{
          receipt_contract_version:3,
          producer_artifact_version:3,
          release_artifact_receipt_output_env_name:"HEPTA_NATIVE_RELEASE_ARTIFACT_RECEIPT_PATH",
          release_artifact_receipt_output_supported:$build_script_writes_artifact_receipt,
          release_artifact_intake_input_env_name:"HEPTA_UI_RELEASE_ARTIFACT_INPUT_PATH",
          release_artifact_intake_gate:"scripts/hepta-ui-release-artifact-intake-gate.sh",
          post_artifact_refresh_gate:"scripts/hepta-ui-product-readiness-gate.sh",
          expected_artifact_kind:"signed_notarized_stapled_artifact",
          required_artifact_evidence_fields:[
            "signed_artifact_path",
            "signed_artifact_sha256",
            "signed_artifact_bytes",
            "notarization_ticket_sha256",
            "codesign_verify_app_sha256",
            "codesign_verify_dmg_sha256",
            "stapler_staple_sha256",
            "stapler_validate_sha256",
            "spctl_assessment_sha256",
            "dmg_mounted_read_only",
            "mounted_app_bundle_fingerprint",
            "mounted_binary_sha256",
            "mounted_bundle_identifier",
            "applications_alias_verified",
            "applications_alias_kind",
            "applications_alias_resolved_target",
            "dmg_readonly_attach_sha256",
            "dmg_readonly_mount_sha256",
            "notarytool_submit_log_sha256",
            "notarytool_submit_log_bytes",
            "notarytool_exit_code",
            "notary_submission_id",
            "notary_submission_state",
            "notary_submission_confirmed",
            "notary_submission_may_have_occurred",
            "notary_auth_mode"
          ],
          required_source_evidence_fields:[
            "source_app_bundle_fingerprint",
            "signed_app_bundle_fingerprint",
            "source_stable_during_unsigned_package_run",
            "private_copy_recomputed_before_signing"
          ],
          required_command_log_paths:[
            "notarytool_submit_log_path",
            "codesign_verify_app_log_path",
            "codesign_verify_dmg_log_path",
            "stapler_staple_log_path",
            "stapler_validate_log_path",
            "spctl_assessment_log_path",
            "dmg_readonly_attach_path",
            "dmg_readonly_mount_log_path"
          ],
          public_upload_performed_must_be_false:true,
          public_distribution_claim_requires_intake_refresh:true
        },
        local_verification_commands:[
          "scripts/hepta-ui-release-signing-capability-gate.sh",
          "apps/hepta-native/packaging/build-macos-dmg.sh",
          "scripts/hepta-ui-release-artifact-intake-gate.sh",
          "scripts/hepta-ui-product-readiness-gate.sh"
        ],
        side_effects_must_remain_false_until_independent_release_approval_verification:[
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
        credential_environment_scrubbed_before_first_external_command:true,
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
  + "- Audit status: `\(.audit_status)`\n"
  + "- Execution capability status: `\(.capability_status)`\n"
  + "- Configured identity: `\(.package_metadata.signing_identity)`\n"
  + "- Configured identity Team ID: `\(.package_metadata.configured_team_id)`\n"
  + "- Trusted Team ID source: `\(.package_metadata.trusted_team_id_source)`\n"
  + "- Trusted Team ID valid and exact-match: \(.package_metadata.trusted_team_id_valid and .package_metadata.configured_team_id_matches_trusted)\n"
  + "- Valid code-signing identities: \(.keychain_identity_lookup.valid_identity_count)\n"
  + "- Matching configured identities: \(.keychain_identity_lookup.matching_configured_identity_count)\n"
  + "- Apple notary env present: \(.notary_credentials.all_required_env_present)\n"
  + "- Apple notary keychain profile present: \(.notary_credentials.optional_keychain_profile_present)\n"
  + "- Apple notary keychain profile item found: \(.notary_credentials.optional_keychain_profile_keychain_item_found)\n"
  + "- Apple notary credentials ready: \(.notary_credentials.notary_credentials_ready)\n"
  + "- Release signing prerequisites ready: \(.release_execution_prerequisites.release_signing_execution_prerequisites_ready)\n"
  + "- Release receipt output supported: \(.release_script_capabilities.release_artifact_receipt_output_supported)\n"
  + "- `spctl` failure is hard failure: \(.release_script_capabilities.spctl_failure_is_hard_failure)\n"
  + "- Canonical unsigned receipt required: \(.release_script_capabilities.canonical_unsigned_receipt_required)\n"
  + "- Accepted notary result required: \(.release_script_capabilities.accepted_notary_result_required)\n"
  + "- Full bundle fingerprint + symlink rejection required: \(.release_script_capabilities.full_bundle_fingerprint_required)\n"
  + "- Private signing copy recomputed: \(.release_script_capabilities.private_copy_recomputed_before_signing)\n"
  + "- Default release input does not launch the app: \(.release_script_capabilities.default_release_input_does_not_launch)\n"
  + "- Final DMG read-only payload readback required: \(.release_script_capabilities.readonly_dmg_payload_readback_required)\n"
  + "- Actual release execution supported: \(.release_script_capabilities.actual_release_execution_supported)\n"
  + "- Independent approval verifier ready: \(.release_execution_prerequisites.independent_approval_verifier_ready)\n"
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
  and .audit_status == "ready"
  and .capability_status == "blocked"
  and .release_signing_capability_gate_ready == true
  and .capability_kind == "local_release_signing_notary_prerequisite_audit"
  and .capability_version == 2
  and (.package_metadata.signing_identity_configured | type) == "boolean"
  and .package_metadata.trusted_team_id_source == "HEPTA_EXPECTED_TEAM_ID"
  and (.package_metadata.trusted_team_id_valid | type) == "boolean"
  and (.package_metadata.configured_team_id_matches_trusted | type) == "boolean"
  and ((.package_metadata.configured_team_id == "") or (.package_metadata.configured_team_id | test("^[A-Z0-9]{10}$")))
  and ((.package_metadata.trusted_team_id == "") or (.package_metadata.trusted_team_id | test("^[A-Z0-9]{10}$")))
  and .package_metadata.bundle_identifier == "ai.hepta.nativeapp"
  and .package_metadata.bundle_executable == "hepta-native"
  and .package_metadata.bundle_name == "Hepta"
  and .local_tooling.distribution_tools_ready == true
  and .keychain_identity_lookup.performed == true
  and (.keychain_identity_lookup.output_sha256 | test("^[0-9a-f]{64}$"))
  and .keychain_identity_lookup.output_bytes > 0
  and .release_execution_prerequisites.bundle_and_release_script_contract_ready == true
  and (.release_execution_prerequisites.keychain_identity_ready | type) == "boolean"
  and (.release_execution_prerequisites.notary_env_ready | type) == "boolean"
  and .release_execution_prerequisites.notary_env_ready == false
  and (.release_execution_prerequisites.notary_keychain_profile_ready | type) == "boolean"
  and (.release_execution_prerequisites.notary_credentials_ready | type) == "boolean"
  and (.release_execution_prerequisites.release_signing_execution_prerequisites_ready | type) == "boolean"
  and .release_execution_prerequisites.independent_approval_verifier_ready == true
  and (.release_execution_prerequisites.release_approval_trust_configured | type) == "boolean"
  and .release_execution_prerequisites.signed_release_execution_approval_verified == false
  and .release_execution_prerequisites.release_signing_execution_prerequisites_ready == false
  and (.blockers | index("independent_release_approval_verifier_unavailable") == null)
  and ((.release_execution_prerequisites.release_approval_trust_configured == true) == (.blockers | index("release_approval_trust_not_configured") == null))
  and (.blockers | index("signed_release_execution_approval_missing") != null)
  and (.notary_credentials.optional_keychain_profile_keychain_lookup_performed | type) == "boolean"
  and (.notary_credentials.optional_keychain_profile_keychain_item_found | type) == "boolean"
  and (.notary_credentials.optional_keychain_profile_lookup_output_sha256 | test("^[0-9a-f]{64}$"))
  and .notary_credentials.optional_keychain_profile_lookup_output_bytes > 0
  and .notary_credentials.optional_keychain_profile_lookup_raw_output_persisted == false
  and .release_script_capabilities.notary_keychain_profile_supported == true
  and .release_script_capabilities.release_artifact_receipt_output_supported == true
  and .release_script_capabilities.spctl_failure_is_hard_failure == true
  and .release_script_capabilities.consumes_exact_formal_app == true
  and .release_script_capabilities.canonical_unsigned_receipt_required == true
  and .release_script_capabilities.accepted_notary_result_required == true
  and .release_script_capabilities.full_bundle_fingerprint_required == true
  and .release_script_capabilities.private_copy_recomputed_before_signing == true
  and .release_script_capabilities.default_release_input_does_not_launch == true
  and .release_script_capabilities.readonly_dmg_payload_readback_required == true
  and .release_script_capabilities.v3_notary_uncertainty_truth_required == true
  and .release_script_capabilities.actual_execution_blocked_before_release_side_effects == true
  and .release_script_capabilities.preflight_only_supported == true
  and .release_script_capabilities.actual_release_execution_supported == false
  and .release_script_capabilities.builds_second_product_app == false
  and .release_execution_handoff.handoff_kind == "operator_supplied_release_signing_notary_artifact_handoff"
  and .release_execution_handoff.handoff_ready == true
  and (.release_execution_handoff.execution_prerequisites_ready | type) == "boolean"
  and (.release_execution_handoff.required_operator_inputs | length) == 5
  and .release_execution_handoff.supported_credential_modes.direct_environment.env_names == ["APPLE_ID","APPLE_PASSWORD","APPLE_TEAM_ID"]
  and .release_execution_handoff.supported_credential_modes.direct_environment.supported == false
  and .release_execution_handoff.supported_credential_modes.direct_environment.all_required_env_present == false
  and .release_execution_handoff.supported_credential_modes.notarytool_keychain_profile.profile_env_name == "HEPTA_NOTARY_PROFILE"
  and .release_execution_handoff.supported_credential_modes.notarytool_keychain_profile.supported_by_build_script == true
  and .release_execution_handoff.receipt_contract.release_artifact_receipt_output_env_name == "HEPTA_NATIVE_RELEASE_ARTIFACT_RECEIPT_PATH"
  and .release_execution_handoff.receipt_contract.release_artifact_receipt_output_supported == true
  and .release_execution_handoff.receipt_contract.release_artifact_intake_input_env_name == "HEPTA_UI_RELEASE_ARTIFACT_INPUT_PATH"
  and .release_execution_handoff.receipt_contract.release_artifact_intake_gate == "scripts/hepta-ui-release-artifact-intake-gate.sh"
  and .release_execution_handoff.receipt_contract.post_artifact_refresh_gate == "scripts/hepta-ui-product-readiness-gate.sh"
  and .release_execution_handoff.receipt_contract.expected_artifact_kind == "signed_notarized_stapled_artifact"
  and .release_execution_handoff.receipt_contract.receipt_contract_version == 3
  and .release_execution_handoff.receipt_contract.producer_artifact_version == 3
  and .release_execution_handoff.receipt_contract.required_artifact_evidence_fields == ["signed_artifact_path","signed_artifact_sha256","signed_artifact_bytes","notarization_ticket_sha256","codesign_verify_app_sha256","codesign_verify_dmg_sha256","stapler_staple_sha256","stapler_validate_sha256","spctl_assessment_sha256","dmg_mounted_read_only","mounted_app_bundle_fingerprint","mounted_binary_sha256","mounted_bundle_identifier","applications_alias_verified","applications_alias_kind","applications_alias_resolved_target","dmg_readonly_attach_sha256","dmg_readonly_mount_sha256","notarytool_submit_log_sha256","notarytool_submit_log_bytes","notarytool_exit_code","notary_submission_id","notary_submission_state","notary_submission_confirmed","notary_submission_may_have_occurred","notary_auth_mode"]
  and .release_execution_handoff.receipt_contract.required_source_evidence_fields == ["source_app_bundle_fingerprint","signed_app_bundle_fingerprint","source_stable_during_unsigned_package_run","private_copy_recomputed_before_signing"]
  and .release_execution_handoff.receipt_contract.required_command_log_paths == ["notarytool_submit_log_path","codesign_verify_app_log_path","codesign_verify_dmg_log_path","stapler_staple_log_path","stapler_validate_log_path","spctl_assessment_log_path","dmg_readonly_attach_path","dmg_readonly_mount_log_path"]
  and .release_execution_handoff.receipt_contract.public_upload_performed_must_be_false == true
  and (.release_execution_handoff.local_verification_commands | index("apps/hepta-native/packaging/build-macos-dmg.sh") != null)
  and (.release_execution_handoff.side_effects_must_remain_false_until_independent_release_approval_verification | index("credential_value_captured") != null)
  and (.release_execution_handoff.side_effects_must_remain_false_until_independent_release_approval_verification | index("notary_submission_performed") != null)
  and (.release_execution_handoff.side_effects_must_remain_false_until_independent_release_approval_verification | index("active_binary_mutation") != null)
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
  and .side_effects.credential_environment_scrubbed_before_first_external_command == true
  and .side_effects.network_call_performed == false
  and .side_effects.notary_submission_performed == false
  and .side_effects.app_signed == false
  and .side_effects.app_notarized == false
  and .side_effects.app_stapled == false
  and .side_effects.public_distribution_artifact_written == false
  and .side_effects.external_mutation == false
' "$REPORT_TMP" >/dev/null

hepta_safe_atomic_replace "$REPORT_TMP" "$REPORT_PATH" signing_capability_report
hepta_safe_atomic_replace "$MARKDOWN_TMP" "$MARKDOWN_PATH" signing_capability_markdown
cat "$REPORT_PATH"
