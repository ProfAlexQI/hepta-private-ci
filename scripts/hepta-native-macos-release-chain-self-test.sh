#!/bin/bash -p
set +x
PS4='+ '
set -euo pipefail
unset BASH_ENV ENV CDPATH GLOBIGNORE RUBYOPT RUBYLIB GEM_HOME GEM_PATH BUNDLE_GEMFILE BUNDLE_PATH
SYSTEM_PATH="/usr/bin:/bin:/usr/sbin:/sbin"
PATH="$SYSTEM_PATH"
export PATH

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
BUILD_SCRIPT="$ROOT_DIR/apps/hepta-native/packaging/build-macos-dmg.sh"
TEST_DIR="$(mktemp -d /private/tmp/hepta-macos-release-chain-self-test.XXXXXX)"
cleanup_test() {
  if [[ "${HEPTA_RELEASE_SELF_TEST_KEEP:-0}" == "1" ]]; then
    printf 'release self-test evidence retained: %s\n' "$TEST_DIR" >&2
  else
    rm -rf "$TEST_DIR"
  fi
}
trap cleanup_test EXIT

[[ "$(head -n 1 "$BUILD_SCRIPT")" == '#!/bin/bash -p' ]]
/bin/bash -n "$BUILD_SCRIPT"

# The production source must place the independent-approval guard immediately
# after option parsing and before platform/tool discovery. This makes the
# release action unreachable before codesign, notarytool, stapler, spctl, or
# hdiutil can run.
/usr/bin/ruby -e '
  source = File.binread(ARGV.fetch(0))
  guard = source.index(%q{if [[ "$PREFLIGHT_ONLY" != "1" ]]}) or abort "approval guard missing"
  blocker = source.index(%q{signed_release_execution_approval_missing: actual release execution is disabled; run --preflight only}) or abort "approval blocker missing"
  discovery = source.index(%q{[[ "$(uname -s)" == "Darwin" ]}) or abort "platform discovery missing"
  abort "approval blocker is not inside the early guard" unless guard < blocker && blocker < discovery
  abort "actual guard does not use dedicated exit" unless source[guard...discovery].include?("exit 77")
' "$BUILD_SCRIPT"

# Instrument a private copy of the exact production script. Only system-tool
# paths are redirected; the approval guard is not modified or bypassed.
FAKE_REPO="$TEST_DIR/repo"
PACKAGING_DIR="$FAKE_REPO/apps/hepta-native/packaging"
SENTINEL_BIN="$TEST_DIR/sentinel-bin"
TOOL_LOG="$TEST_DIR/tool-invocations.log"
HIGH_RISK_LOG="$TEST_DIR/release-side-effect-tool-invocations.log"
STARTUP_HOOK_MARKER="$TEST_DIR/bash-env-startup-hook-ran"
RUBY_HOOK_MARKER="$TEST_DIR/rubyopt-startup-hook-ran"
BASH_ENV_HOOK="$TEST_DIR/hostile-bash-env.sh"
RUBYOPT_HOOK="$TEST_DIR/hostile-rubyopt.rb"
mkdir -p "$PACKAGING_DIR" "$SENTINEL_BIN"
cp "$BUILD_SCRIPT" "$PACKAGING_DIR/build-macos-dmg.sh"
cp "$ROOT_DIR/apps/hepta-native/packaging/Entitlements.plist" "$PACKAGING_DIR/Entitlements.plist"
cp "$ROOT_DIR/apps/hepta-native/packaging/app-bundle-fingerprint-v1.rb" "$PACKAGING_DIR/app-bundle-fingerprint-v1.rb"
cp "$ROOT_DIR/apps/hepta-native/packaging/resolve-finder-bookmark-v1.swift" "$PACKAGING_DIR/resolve-finder-bookmark-v1.swift"
printf 'version = "1.0.0"\n' >"$FAKE_REPO/apps/hepta-native/Cargo.toml"
printf '/usr/bin/touch %q\n' "$STARTUP_HOOK_MARKER" >"$BASH_ENV_HOOK"
printf 'File.write('\''%s'\'', "ran\\n")\n' "$RUBY_HOOK_MARKER" >"$RUBYOPT_HOOK"

TEST_SENTINEL_BIN="$SENTINEL_BIN" /usr/bin/ruby -pi -e '
  sentinel = ENV.fetch("TEST_SENTINEL_BIN")
  $_ = $_.gsub(%q{SYSTEM_PATH="/usr/bin:/bin:/usr/sbin:/sbin"}, "SYSTEM_PATH=\"#{sentinel}:/usr/bin:/bin:/usr/sbin:/sbin\"")
         .gsub(%q{/usr/bin/security}, "#{sentinel}/security")
         .gsub(%q{/usr/bin/xcrun}, "#{sentinel}/xcrun")
' "$PACKAGING_DIR/build-macos-dmg.sh"

cat >"$SENTINEL_BIN/instrumented-tool" <<'EOF'
#!/bin/bash
set -euo pipefail
tool="${0##*/}"
printf '%s\n' "$tool $*" >>"${HEPTA_RELEASE_TOOL_LOG:?}"
case "$tool" in
  codesign|xcrun|hdiutil|spctl|ditto)
    printf '%s\n' "$tool $*" >>"${HEPTA_RELEASE_HIGH_RISK_LOG:?}"
    ;;
esac
case "$tool" in
  uname)
    if [[ "${1:-}" == "-m" ]]; then printf 'arm64\n'; else printf 'Darwin\n'; fi
    ;;
  security)
    printf '  1) 1111111111111111111111111111111111111111 "Developer ID Application: Hepta Test (TEAMID1234)"\n'
    printf '     1 valid identities found\n'
    ;;
  *) exit 99 ;;
esac
EOF
chmod 755 "$SENTINEL_BIN/instrumented-tool" "$PACKAGING_DIR/build-macos-dmg.sh"
for tool in bash uname security codesign xcrun hdiutil spctl ditto; do
  ln -s instrumented-tool "$SENTINEL_BIN/$tool"
done

TRUST_BOUNDARY_SCRIPTS=(
  apps/hepta-native/packaging/build-macos-dmg.sh
  apps/hepta-native/packaging/create-macos-dmg-from-app.sh
  apps/hepta-native/packaging/fix-dmg-applications-icon.sh
  scripts/hepta-ui-release-approval-intake-gate.sh
  scripts/hepta-ui-release-artifact-boundary-gate.sh
  scripts/hepta-ui-release-artifact-intake-gate.sh
  scripts/hepta-ui-release-artifact-roundtrip-gate.sh
  scripts/hepta-ui-release-signing-capability-gate.sh
  scripts/hepta-ui-current-plan-refresh-gate.sh
  scripts/hepta-ui-blocker-closure-gate.sh
  scripts/hepta-ui-backend-delivery-audit-gate.sh
  scripts/hepta-ui-backend-delivery-receipt-roundtrip-gate.sh
  scripts/hepta-ui-risk-future-plan-gate.sh
  scripts/hepta-ui-root-report-replay-gate.sh
)
for relative_script in "${TRUST_BOUNDARY_SCRIPTS[@]}"; do
  production_script="$ROOT_DIR/$relative_script"
  [[ "$(head -n 1 "$production_script")" == '#!/bin/bash -p' ]]
  startup_case="$TEST_DIR/startup-${relative_script//\//_}"
  mkdir -p "$startup_case/readiness"
  : >"$TOOL_LOG"
  : >"$HIGH_RISK_LOG"
  rm -f "$STARTUP_HOOK_MARKER" "$RUBY_HOOK_MARKER"
  set +e
  /usr/bin/env \
    PATH="$SENTINEL_BIN:$SYSTEM_PATH" \
    BASH_ENV="$BASH_ENV_HOOK" \
    RUBYOPT="-r$RUBYOPT_HOOK" \
    SHELLOPTS=xtrace \
    PS4='startup-secret-sentinel ' \
    HEPTA_RELEASE_TOOL_LOG="$TOOL_LOG" \
    HEPTA_RELEASE_HIGH_RISK_LOG="$HIGH_RISK_LOG" \
    HEPTA_UI_PRODUCT_READINESS_DIR="$startup_case/readiness" \
    HEPTA_NATIVE_RELEASE_EVIDENCE_DIR="$startup_case/evidence" \
    "$production_script" >"$startup_case/stdout" 2>"$startup_case/stderr"
  set -e
  [[ ! -e "$STARTUP_HOOK_MARKER" && ! -e "$RUBY_HOOK_MARKER" ]]
  [[ ! -s "$TOOL_LOG" && ! -s "$HIGH_RISK_LOG" ]]
  ! /usr/bin/grep -F 'startup-secret-sentinel' "$startup_case/stdout" "$startup_case/stderr" >/dev/null
done

# Caller-selected managed output directories must stay beneath readiness. A
# rejected path must not mutate the caller-owned directory or its sentinel.
run_managed_path_outside_rejected() {
  local label="$1" relative_script="$2" managed_env_name="$3"
  local case_dir="$TEST_DIR/path-safety-$label"
  local readiness_dir="$case_dir/readiness"
  local operator_dir="$case_dir/operator-owned"
  local sentinel="$operator_dir/sentinel.txt"
  mkdir -p "$readiness_dir" "$operator_dir"
  printf 'operator-owned-%s\n' "$label" >"$sentinel"
  local before_sha status
  before_sha="$(/usr/bin/shasum -a 256 "$sentinel" | /usr/bin/awk '{print $1}')"
  set +e
  /usr/bin/env \
    HEPTA_UI_PRODUCT_READINESS_DIR="$readiness_dir" \
    "$managed_env_name=$operator_dir" \
    "$ROOT_DIR/$relative_script" >"$case_dir/stdout" 2>"$case_dir/stderr"
  status=$?
  set -e
  [[ "$status" -eq 64 ]]
  [[ -f "$sentinel" && ! -L "$sentinel" ]]
  [[ "$(/usr/bin/shasum -a 256 "$sentinel" | /usr/bin/awk '{print $1}')" == "$before_sha" ]]
}

run_managed_path_outside_rejected \
  current-plan scripts/hepta-ui-current-plan-refresh-gate.sh HEPTA_UI_CURRENT_PLAN_REFRESH_DIR
run_managed_path_outside_rejected \
  blocker scripts/hepta-ui-blocker-closure-gate.sh HEPTA_UI_BLOCKER_CLOSURE_DIR
run_managed_path_outside_rejected \
  backend-audit scripts/hepta-ui-backend-delivery-audit-gate.sh HEPTA_UI_BACKEND_DELIVERY_AUDIT_DIR
run_managed_path_outside_rejected \
  delivery-roundtrip scripts/hepta-ui-backend-delivery-receipt-roundtrip-gate.sh HEPTA_UI_BACKEND_DELIVERY_RECEIPT_ROUNDTRIP_DIR
run_managed_path_outside_rejected \
  risk-plan scripts/hepta-ui-risk-future-plan-gate.sh HEPTA_UI_RISK_FUTURE_PLAN_DIR

# A symlinked managed path and a managed path equal to readiness are rejected
# before any caller-owned content is touched.
CURRENT_PATH_CASE="$TEST_DIR/path-safety-current-special"
mkdir -p "$CURRENT_PATH_CASE/readiness" "$CURRENT_PATH_CASE/operator-owned"
printf 'operator-owned-current-special\n' >"$CURRENT_PATH_CASE/operator-owned/sentinel.txt"
ln -s "$CURRENT_PATH_CASE/operator-owned" "$CURRENT_PATH_CASE/readiness/linked-managed"
for rejected_refresh_dir in \
  "$CURRENT_PATH_CASE/readiness/linked-managed" \
  "$CURRENT_PATH_CASE/readiness"; do
  set +e
  /usr/bin/env \
    HEPTA_UI_PRODUCT_READINESS_DIR="$CURRENT_PATH_CASE/readiness" \
    HEPTA_UI_CURRENT_PLAN_REFRESH_DIR="$rejected_refresh_dir" \
    "$ROOT_DIR/scripts/hepta-ui-current-plan-refresh-gate.sh" \
    >"$CURRENT_PATH_CASE/stdout" 2>"$CURRENT_PATH_CASE/stderr"
  PATH_STATUS=$?
  set -e
  [[ "$PATH_STATUS" -eq 64 ]]
  /usr/bin/grep -Fxq 'operator-owned-current-special' "$CURRENT_PATH_CASE/operator-owned/sentinel.txt"
done

# The root report fixed leaf is not blanket-owned by the gate. Unknown JSON
# and malformed regular files must survive byte-for-byte and return EX_USAGE.
for root_sentinel_kind in unknown-json malformed-json; do
  ROOT_SENTINEL_CASE="$TEST_DIR/path-safety-root-$root_sentinel_kind"
  ROOT_SENTINEL_READINESS="$ROOT_SENTINEL_CASE/readiness"
  ROOT_SENTINEL_REPORT="$ROOT_SENTINEL_READINESS/ui-root-report-replay-gate.json"
  mkdir -p "$ROOT_SENTINEL_READINESS"
  if [[ "$root_sentinel_kind" == "unknown-json" ]]; then
    /usr/bin/jq -n --arg path "$ROOT_SENTINEL_REPORT" \
      '{gate:"operator_owned_report",report_path:$path,operator_sentinel:true}' >"$ROOT_SENTINEL_REPORT"
  else
    printf 'operator-owned malformed report sentinel\n' >"$ROOT_SENTINEL_REPORT"
  fi
  ROOT_SENTINEL_SHA="$(/usr/bin/shasum -a 256 "$ROOT_SENTINEL_REPORT" | /usr/bin/awk '{print $1}')"
  set +e
  /usr/bin/env HEPTA_UI_PRODUCT_READINESS_DIR="$ROOT_SENTINEL_READINESS" \
    "$ROOT_DIR/scripts/hepta-ui-root-report-replay-gate.sh" \
    >"$ROOT_SENTINEL_CASE/stdout" 2>"$ROOT_SENTINEL_CASE/stderr"
  ROOT_SENTINEL_STATUS=$?
  set -e
  [[ "$ROOT_SENTINEL_STATUS" -eq 64 ]]
  [[ "$(/usr/bin/shasum -a 256 "$ROOT_SENTINEL_REPORT" | /usr/bin/awk '{print $1}')" == "$ROOT_SENTINEL_SHA" ]]
done

# The backend audit accepted-receipt leaf has the same ownership rule. A
# caller-owned same-name regular file cannot be cleared as stale evidence.
ACCEPTED_SENTINEL_CASE="$TEST_DIR/path-safety-backend-accepted"
ACCEPTED_SENTINEL_READINESS="$ACCEPTED_SENTINEL_CASE/readiness"
ACCEPTED_SENTINEL_AUDIT="$ACCEPTED_SENTINEL_READINESS/backend-delivery-audit"
ACCEPTED_SENTINEL_REPORT="$ACCEPTED_SENTINEL_AUDIT/backend-delivery-receipt-input.accepted.json"
mkdir -p "$ACCEPTED_SENTINEL_AUDIT"
/usr/bin/jq -n '{delivery_kind:"operator_owned_receipt",delivery_version:1,owner_lane:"operator"}' \
  >"$ACCEPTED_SENTINEL_REPORT"
ACCEPTED_SENTINEL_SHA="$(/usr/bin/shasum -a 256 "$ACCEPTED_SENTINEL_REPORT" | /usr/bin/awk '{print $1}')"
set +e
/usr/bin/env HEPTA_UI_PRODUCT_READINESS_DIR="$ACCEPTED_SENTINEL_READINESS" \
  "$ROOT_DIR/scripts/hepta-ui-backend-delivery-audit-gate.sh" \
  >"$ACCEPTED_SENTINEL_CASE/stdout" 2>"$ACCEPTED_SENTINEL_CASE/stderr"
ACCEPTED_SENTINEL_STATUS=$?
set -e
[[ "$ACCEPTED_SENTINEL_STATUS" -eq 64 ]]
[[ "$(/usr/bin/shasum -a 256 "$ACCEPTED_SENTINEL_REPORT" | /usr/bin/awk '{print $1}')" == "$ACCEPTED_SENTINEL_SHA" ]]

printf 'release_downstream_managed_output_path_safety: PASS\n'
printf 'root_and_backend_unknown_output_sentinels_preserved: PASS\n'

run_actual_blocked() {
  local label="$1"
  shift
  local case_dir="$TEST_DIR/$label"
  mkdir -p "$case_dir"
  : >"$TOOL_LOG"
  : >"$HIGH_RISK_LOG"
  rm -f "$STARTUP_HOOK_MARKER" "$RUBY_HOOK_MARKER"
  set +e
  env \
    PATH="$SENTINEL_BIN:/usr/bin:/bin:/usr/sbin:/sbin" \
    HEPTA_RELEASE_TOOL_LOG="$TOOL_LOG" \
    HEPTA_RELEASE_HIGH_RISK_LOG="$HIGH_RISK_LOG" \
    BASH_ENV="$BASH_ENV_HOOK" \
    RUBYOPT="-r$RUBYOPT_HOOK" \
    SHELLOPTS=xtrace \
    PS4='startup-secret-sentinel ' \
    HEPTA_SIGNING_IDENTITY='Developer ID Application: Hepta Test (TEAMID1234)' \
    HEPTA_EXPECTED_TEAM_ID='TEAMID1234' \
    HEPTA_NOTARY_PROFILE='hepta-self-test-profile' \
    HEPTA_NATIVE_RELEASE_EVIDENCE_DIR="$case_dir/evidence" \
    "$@" \
    "$PACKAGING_DIR/build-macos-dmg.sh" \
      --app-path "$case_dir/nonexistent-input/Hepta.app" \
      --app-receipt "$case_dir/nonexistent-input/receipt.json" \
      --output "$case_dir/Hepta.dmg" \
      --receipt "$case_dir/release.json" \
      >"$case_dir/stdout" 2>"$case_dir/stderr"
  local status=$?
  set -e
  [[ "$status" -eq 77 ]]
  grep -Fxq 'signed_release_execution_approval_missing: actual release execution is disabled; run --preflight only' "$case_dir/stderr"
  [[ ! -s "$TOOL_LOG" && ! -s "$HIGH_RISK_LOG" ]]
  [[ ! -e "$STARTUP_HOOK_MARKER" && ! -e "$RUBY_HOOK_MARKER" ]]
  ! /usr/bin/grep -F 'startup-secret-sentinel' "$case_dir/stdout" "$case_dir/stderr" >/dev/null
  [[ ! -e "$case_dir/Hepta.dmg" && ! -e "$case_dir/release.json" && ! -e "$case_dir/evidence" ]]
}

# Complete-looking identity/profile authority still cannot authorize actual
# execution. Direct Apple credentials cannot override the missing verifier.
run_actual_blocked complete_identity_and_profile /usr/bin/env
run_actual_blocked profile_plus_direct_credentials \
  /usr/bin/env APPLE_ID=operator@example.invalid APPLE_PASSWORD=secret-sentinel APPLE_TEAM_ID=TEAMID1234
if /usr/bin/grep -F 'secret-sentinel' "$TEST_DIR/profile_plus_direct_credentials/stdout" "$TEST_DIR/profile_plus_direct_credentials/stderr" >/dev/null; then
  printf 'release guard leaked a direct credential value\n' >&2
  exit 1
fi

# Preflight is the sole supported path. It may audit local identity/tool shape,
# but it must not invoke codesign, notarytool/stapler, hdiutil, ditto, or spctl,
# and it cannot promote release execution readiness.
PREFLIGHT_DIR="$TEST_DIR/preflight"
mkdir -p "$PREFLIGHT_DIR"
: >"$TOOL_LOG"
: >"$HIGH_RISK_LOG"
rm -f "$STARTUP_HOOK_MARKER" "$RUBY_HOOK_MARKER"
env \
  PATH="$SENTINEL_BIN:/usr/bin:/bin:/usr/sbin:/sbin" \
  HEPTA_RELEASE_TOOL_LOG="$TOOL_LOG" \
  HEPTA_RELEASE_HIGH_RISK_LOG="$HIGH_RISK_LOG" \
  BASH_ENV="$BASH_ENV_HOOK" RUBYOPT="-r$RUBYOPT_HOOK" SHELLOPTS=xtrace PS4='startup-secret-sentinel ' \
  HEPTA_SIGNING_IDENTITY='Developer ID Application: Hepta Test (TEAMID1234)' \
  HEPTA_EXPECTED_TEAM_ID='TEAMID1234' \
  HEPTA_NOTARY_PROFILE='hepta-self-test-profile' \
  HEPTA_NATIVE_RELEASE_EVIDENCE_DIR="$PREFLIGHT_DIR/evidence" \
  "$PACKAGING_DIR/build-macos-dmg.sh" --preflight \
    --output "$PREFLIGHT_DIR/Hepta.dmg" --receipt "$PREFLIGHT_DIR/release.json" \
    >"$PREFLIGHT_DIR/preflight.json"
jq -e '
  .status == "ready"
  and .preflight_scope == "tools_credentials_and_path_shape_only"
  and .independent_approval_verifier_ready == false
  and .release_execution_ready == false
  and .actual_release_execution_supported == false
  and (.blockers | index("independent_release_approval_verifier_unavailable") != null)
  and .canonical_input_verified == false
  and .receipt_json_validated == false
  and .publishes == false
' "$PREFLIGHT_DIR/preflight.json" >/dev/null
[[ ! -s "$HIGH_RISK_LOG" ]]
[[ ! -e "$STARTUP_HOOK_MARKER" && ! -e "$RUBY_HOOK_MARKER" ]]
[[ ! -e "$PREFLIGHT_DIR/Hepta.dmg" && ! -e "$PREFLIGHT_DIR/release.json" && ! -e "$PREFLIGHT_DIR/evidence" ]]

# Direct password environment is unsupported even during preflight and must
# fail before any high-risk tool executes.
DIRECT_DIR="$TEST_DIR/direct-preflight"
mkdir -p "$DIRECT_DIR"
: >"$HIGH_RISK_LOG"
set +e
env PATH="$SENTINEL_BIN:/usr/bin:/bin:/usr/sbin:/sbin" \
  HEPTA_RELEASE_TOOL_LOG="$TOOL_LOG" HEPTA_RELEASE_HIGH_RISK_LOG="$HIGH_RISK_LOG" \
  HEPTA_SIGNING_IDENTITY='Developer ID Application: Hepta Test (TEAMID1234)' \
  HEPTA_EXPECTED_TEAM_ID='TEAMID1234' HEPTA_NOTARY_PROFILE='hepta-self-test-profile' \
  APPLE_ID=operator@example.invalid APPLE_PASSWORD=secret-sentinel APPLE_TEAM_ID=TEAMID1234 \
  HEPTA_NATIVE_RELEASE_EVIDENCE_DIR="$DIRECT_DIR/evidence" \
  "$PACKAGING_DIR/build-macos-dmg.sh" --preflight \
    --output "$DIRECT_DIR/Hepta.dmg" --receipt "$DIRECT_DIR/release.json" \
    >"$DIRECT_DIR/stdout" 2>"$DIRECT_DIR/stderr"
DIRECT_STATUS=$?
set -e
[[ "$DIRECT_STATUS" -eq 2 ]]
grep -Fq 'release notarization is keychain-profile-only' "$DIRECT_DIR/stderr"
! /usr/bin/grep -F 'secret-sentinel' "$DIRECT_DIR/stdout" "$DIRECT_DIR/stderr" >/dev/null
[[ ! -s "$HIGH_RISK_LOG" && ! -e "$DIRECT_DIR/Hepta.dmg" ]]
jq -e '
  .status == "not_ready"
  and .failure.blocker == "direct_apple_id_password_notary_mode_unsupported"
  and .artifact_evidence.signed == false
  and .artifact_evidence.notary_submission_may_have_occurred == false
  and .side_effects.network_call_may_have_occurred == false
' "$DIRECT_DIR/release.json" >/dev/null

printf 'hepta-native macOS release approval fail-closed self-test: PASS\n'
