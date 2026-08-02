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

for command in git jq ruby rustup shasum; do
  command -v "$command" >/dev/null 2>&1 || { echo "$command is required" >&2; exit 2; }
done

SOURCE_BEFORE="$(scripts/hepta-ui-source-fingerprint)"
POLICY_PATH="apps/hepta-native/mobile-readiness-policy-v1.json"
MANIFEST_PATH="apps/hepta-native/Cargo.toml"
CREDENTIAL_PATH="apps/hepta-native/src/persistence/matrix_session_store/credential.rs"
TESTFLIGHT_PATH="apps/hepta-native/packaging/build-ios-testflight.sh"

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
      %q{APP_BUNDLE="$BUILD_DIR/${CARGO_PACKAGE}.app"},
      %q{rm -rf "$APP_BUNDLE" "$SCENT"},
      %q{run-device -p "$CARGO_PACKAGE" --locked --release},
      %q{scripts/hepta-native-mobile-cargo},
      %q{https://github.com/ProfAlexQI/Hepta/commit/$SOURCE_HEAD},
      %q{stale_artifact_accepted:false},
    ]
    abort "missing fail-closed TestFlight contract" unless required.all? { |needle| text.include?(needle) }
    abort "TestFlight script still suppresses a command failure" if text.include?("|| true")
  ' "$TESTFLIGHT_PATH" >/dev/null 2>&1; then
  testflight_source_contract_ready=true
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

source_contract_ready=false
if [[ "$source_stable" == true \
  && "$policy_ready" == true \
  && "$makepad_pin_ready" == true \
  && "$android_credential_fail_closed_ready" == true \
  && "$testflight_source_contract_ready" == true \
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
        ios_opaque_canonical_icon_contract_ready:$icons_ready,
        android_credential_fail_closed_contract_ready:$credential_ready,
        ios_pinned_toolchain_targets_installed:$ios_targets,
        android_pinned_toolchain_target_installed:$android_target
      },
      toolchain:$toolchain,
      ios_icons:$icons,
      signing_preflight:{apple_distribution_identity_available:$identity_available,apple_distribution_identity_count:$identity_count,signing_performed:false},
      hard_boundaries:{
        ios_accessibility_update_consumed:false,
        android_accessibility_update_consumed:false,
        android_secure_session_persistence_ready:false,
        plaintext_credential_fallback_allowed:false,
        ios_simulator_runtime_verified:false,
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
      blockers:([if $source_stable then empty else "source_changed_during_mobile_gate" end,if $policy_ready then empty else "mobile_policy_contract_not_ready" end,if $makepad_pin_ready then empty else "makepad_revision_not_pinned" end,if $toolchain_ready then empty else "cargo_makepad_exact_toolchain_wrapper_not_ready" end,if $testflight_ready then empty else "testflight_current_source_fail_closed_contract_not_ready" end,if $icons_ready then empty else "ios_opaque_icon_contract_not_ready" end,if $credential_ready then empty else "android_credential_fail_closed_contract_not_ready" end,if $ios_targets then empty else "ios_1_95_targets_not_installed" end,if $android_target then empty else "android_1_95_target_not_installed" end,if $identity_available then empty else "apple_distribution_identity_not_available" end,"pinned_makepad_ios_accessibility_update_discarded","pinned_makepad_android_accessibility_update_discarded","android_secure_credential_backend_not_supported","ios_simulator_receipt_missing","ios_real_device_receipt_missing","android_real_device_receipt_missing","voiceover_receipt_missing","talkback_receipt_missing","software_keyboard_receipt_missing","safe_area_receipt_missing","rtl_receipt_missing","dynamic_type_or_font_scale_receipt_missing"])
    }
  ')"

if [[ -n "$REPORT_PATH" ]]; then
  mkdir -p "$(dirname "$REPORT_PATH")"
  printf '%s\n' "$report" >"$REPORT_PATH"
fi
printf '%s\n' "$report"
jq -e '.mobile_source_contract_ready == true' <<<"$report" >/dev/null
