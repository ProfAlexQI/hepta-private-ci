#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

TEST_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-native-mobile-readiness-self-test.XXXXXX")"
trap 'rm -rf "$TEST_DIR"' EXIT

scripts/hepta-native-mobile-readiness-gate.sh --output "$TEST_DIR/report.json" >/dev/null

jq -e '
  .status == "source_contract_ready"
  and .android_emulator_runtime_evidence.scope == null
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
  and .hard_boundaries.mobile_full_product_ready == false
  and .hard_boundaries.mobile_public_ga_ready == false
' "$TEST_DIR/report.json" >/dev/null

grep -Fq -- '.scope == "unauthenticated_android_login_surface_on_arm64_emulator"' scripts/hepta-native-mobile-readiness-gate.sh
grep -Fq -- 'android_emulator_unauthenticated_login_surface_rotation_verified:$android_login_rotation_ready' scripts/hepta-native-mobile-readiness-gate.sh
if grep -Fq -- 'android_emulator_rotation_verified:$android_emulator_receipt_ready' scripts/hepta-native-mobile-readiness-gate.sh; then
  echo "generic Android rotation claim still fans out from the login receipt" >&2
  exit 1
fi
if grep -Fq -- 'claims:{runtime:$ready,visual:$ready,rotation:$ready,ime:$ready}' scripts/hepta-native-mobile-readiness-gate.sh; then
  echo "generic Android mobile summary still overclaims login evidence" >&2
  exit 1
fi

echo "hepta-native mobile readiness scoped-claims self-test: PASS"
