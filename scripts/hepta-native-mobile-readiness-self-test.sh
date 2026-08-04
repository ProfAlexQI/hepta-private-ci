#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

TEST_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-native-mobile-readiness-self-test.XXXXXX")"
trap 'rm -rf "$TEST_DIR"' EXIT

scripts/hepta-native-mobile-readiness-gate.sh --output "$TEST_DIR/report.json" >/dev/null

jq -e '
  .status == "source_contract_ready"
  and .checks.android_secure_credential_source_contract_ready == true
  and .hard_boundaries.android_secure_session_persistence_ready == false
  and (.blockers | index("android_secure_credential_runtime_receipt_missing") != null)
  and (.blockers | index("android_secure_credential_backend_not_supported") == null)
  and .checks.ios_simulator_ui_qualification_source_contract_ready == true
  and .checks.android_emulator_smoke_source_contract_ready == true
  and .checks.android_emulator_live_readback_source_contract_ready == true
  and .checks.android_login_template_contract_ready == true
  and .android_emulator_smoke_source_contract.status == "ready"
  and .android_emulator_smoke_source_contract.receipt.schema_version == 3
  and (.android_emulator_smoke_source_contract.hard_boundaries | to_entries | all(.value == false))
  and .ios_simulator_ui_runtime_evidence.supplied == false
  and .ios_simulator_ui_runtime_evidence.ready == false
  and .ios_simulator_ui_runtime_evidence.claims.unauthenticated_login_surface_software_keyboard == false
  and .ios_simulator_ui_runtime_evidence.claims.unauthenticated_login_surface_safe_area == false
  and .ios_simulator_ui_runtime_evidence.claims.software_keyboard == false
  and .ios_simulator_ui_runtime_evidence.claims.safe_area == false
  and .ios_simulator_ui_runtime_evidence.generic_claims_hard_false == true
  and .android_emulator_runtime_evidence.scope == null
  and .android_emulator_runtime_evidence.live_readback.opt_in == false
  and .android_emulator_runtime_evidence.live_readback.performed == false
  and .android_emulator_runtime_evidence.live_readback.ready == false
  and .android_emulator_runtime_evidence.claims.runtime == false
  and .android_emulator_runtime_evidence.claims.unauthenticated_login_surface_visual == false
  and .android_emulator_runtime_evidence.claims.unauthenticated_login_surface_rotation == false
  and .android_emulator_runtime_evidence.claims.unauthenticated_login_surface_ime == false
  and .android_emulator_runtime_evidence.claims.visual == false
  and .android_emulator_runtime_evidence.claims.rotation == false
  and .android_emulator_runtime_evidence.claims.ime == false
  and .android_emulator_runtime_evidence.deprecated_generic_claims_hard_false == true
  and .hard_boundaries.android_emulator_unauthenticated_login_surface_visual_verified == false
  and .hard_boundaries.android_emulator_unauthenticated_login_surface_rotation_verified == false
  and .hard_boundaries.android_emulator_unauthenticated_login_surface_ime_verified == false
  and .hard_boundaries.android_emulator_visual_verified == false
  and .hard_boundaries.android_emulator_rotation_verified == false
  and .hard_boundaries.android_emulator_ime_verified == false
  and .hard_boundaries.deprecated_generic_android_emulator_claims_hard_false == true
  and .hard_boundaries.ios_simulator_unauthenticated_login_surface_software_keyboard_verified == false
  and .hard_boundaries.ios_simulator_unauthenticated_login_surface_safe_area_verified == false
  and .hard_boundaries.software_keyboard_verified == false
  and .hard_boundaries.safe_area_verified == false
  and .hard_boundaries.mobile_full_product_ready == false
  and .hard_boundaries.mobile_public_ga_ready == false
  and .local_emulator_side_effects_performed == false
' "$TEST_DIR/report.json" >/dev/null

# Opting in without supplying a receipt must remain side-effect free. A fake
# adb sentinel makes an accidental device read or mutation immediately visible.
FAKE_SDK="$TEST_DIR/fake-sdk"
SENTINEL="$TEST_DIR/adb-called"
mkdir -p "$FAKE_SDK/platform-tools"
printf '#!/usr/bin/env bash\nprintf called >%q\nexit 99\n' "$SENTINEL" >"$FAKE_SDK/platform-tools/adb"
chmod 0755 "$FAKE_SDK/platform-tools/adb"
HEPTA_NATIVE_ANDROID_EMULATOR_LIVE_READBACK=1 ANDROID_SDK_ROOT="$FAKE_SDK" \
  scripts/hepta-native-mobile-readiness-gate.sh --output "$TEST_DIR/opt-in-without-receipt.json" >/dev/null
jq -e '
  .status == "source_contract_ready"
  and .android_emulator_runtime_evidence.supplied == false
  and .android_emulator_runtime_evidence.live_readback.opt_in == true
  and .android_emulator_runtime_evidence.live_readback.performed == false
  and .android_emulator_runtime_evidence.live_readback.ready == false
  and .hard_boundaries.android_emulator_runtime_verified == false
  and .local_emulator_side_effects_performed == false
' "$TEST_DIR/opt-in-without-receipt.json" >/dev/null
[[ ! -e "$SENTINEL" ]] || { echo "mobile gate contacted adb without a receipt" >&2; exit 1; }

printf '%s\n' '{}' >"$TEST_DIR/invalid-receipt.json"
if HEPTA_NATIVE_ANDROID_EMULATOR_LIVE_READBACK=1 \
    HEPTA_NATIVE_ANDROID_EMULATOR_RECEIPT="$TEST_DIR/invalid-receipt.json" \
    scripts/hepta-native-mobile-readiness-gate.sh --output "$TEST_DIR/invalid-receipt-report.json" \
      >"$TEST_DIR/invalid-receipt.stdout" 2>"$TEST_DIR/invalid-receipt.stderr"; then
  echo "mobile gate accepted an invalid opted-in Android receipt" >&2
  exit 1
fi
jq -e '
  .android_emulator_runtime_evidence.supplied == true
  and .android_emulator_runtime_evidence.status == "invalid"
  and .android_emulator_runtime_evidence.ready == false
  and .android_emulator_runtime_evidence.live_readback.ready == false
  and .android_emulator_runtime_evidence.claims.runtime == false
' "$TEST_DIR/invalid-receipt-report.json" >/dev/null

if HEPTA_NATIVE_IOS_SIMULATOR_UI_RECEIPT="$TEST_DIR/invalid-receipt.json" \
    scripts/hepta-native-mobile-readiness-gate.sh --output "$TEST_DIR/invalid-ios-ui-report.json" \
      >"$TEST_DIR/invalid-ios-ui.stdout" 2>"$TEST_DIR/invalid-ios-ui.stderr"; then
  echo "mobile gate accepted an invalid iOS UI qualification receipt" >&2
  exit 1
fi
jq -e '
  .ios_simulator_ui_runtime_evidence.supplied == true
  and .ios_simulator_ui_runtime_evidence.status == "invalid"
  and .ios_simulator_ui_runtime_evidence.ready == false
  and .hard_boundaries.ios_simulator_unauthenticated_login_surface_software_keyboard_verified == false
  and .hard_boundaries.ios_simulator_unauthenticated_login_surface_safe_area_verified == false
' "$TEST_DIR/invalid-ios-ui-report.json" >/dev/null

grep -Fq -- 'HEPTA_NATIVE_IOS_SIMULATOR_UI_RECEIPT' scripts/hepta-native-mobile-readiness-gate.sh
grep -Fq -- 'ios_simulator_unauthenticated_login_surface_software_keyboard_verified:$ios_login_keyboard_ready' scripts/hepta-native-mobile-readiness-gate.sh
grep -Fq -- 'ios_simulator_unauthenticated_login_surface_safe_area_verified:$ios_login_safe_area_ready' scripts/hepta-native-mobile-readiness-gate.sh
grep -Fq -- 'generic_software_keyboard_ready == false' scripts/hepta-native-mobile-readiness-gate.sh
grep -Fq -- 'generic_safe_area_ready == false' scripts/hepta-native-mobile-readiness-gate.sh
grep -Fq -- '.scope == "unauthenticated_android_login_surface_on_arm64_emulator"' scripts/hepta-native-mobile-readiness-gate.sh
grep -Fq -- 'android_emulator_unauthenticated_login_surface_rotation_verified:$android_login_rotation_ready' scripts/hepta-native-mobile-readiness-gate.sh
grep -Fq -- 'HEPTA_NATIVE_ANDROID_EMULATOR_LIVE_READBACK' scripts/hepta-native-mobile-readiness-gate.sh
grep -Fq -- 'independently_verified_claims.unauthenticated_login_surface_rotation == false' scripts/hepta-native-mobile-readiness-gate.sh
grep -Fq -- 'env -i PATH=/usr/bin:/bin:/usr/sbin:/sbin "$ANDROID_EMULATOR_LIVE_READBACK_PATH"' scripts/hepta-native-mobile-readiness-gate.sh
grep -Fq -- 'reason:"trusted_live_readback_failed"' scripts/hepta-native-mobile-readiness-gate.sh
grep -Fq -- '"$ANDROID_SYSTEM_BAR_CONTRAST_REPLAY_VERIFY_PATH" --receipt "$ANDROID_EMULATOR_RECEIPT"' scripts/hepta-native-mobile-readiness-gate.sh
grep -Fq -- 'android_emulator_login_visual_ready=true' scripts/hepta-native-mobile-readiness-gate.sh
grep -Fq -- 'android_emulator_login_rotation_ready=true' scripts/hepta-native-mobile-readiness-gate.sh
grep -Fq -- 'android_emulator_login_ime_ready=true' scripts/hepta-native-mobile-readiness-gate.sh
if grep -Fq -- 'android_emulator_rotation_verified:$android_emulator_receipt_ready' scripts/hepta-native-mobile-readiness-gate.sh; then
  echo "generic Android rotation claim still fans out from the login receipt" >&2
  exit 1
fi
if grep -Fq -- 'claims:{runtime:$ready,visual:$ready,rotation:$ready,ime:$ready}' scripts/hepta-native-mobile-readiness-gate.sh; then
  echo "generic Android mobile summary still overclaims login evidence" >&2
  exit 1
fi

echo "hepta-native mobile readiness scoped-claims self-test: PASS"
