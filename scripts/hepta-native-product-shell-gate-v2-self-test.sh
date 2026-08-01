#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel)"
APP_DIR="$REPO_ROOT/apps/hepta-native"
SYNC_CHECK="$SCRIPT_DIR/hepta-native-robrix-upstream-sync-check-v2.sh"
PRODUCT_GATE="$SCRIPT_DIR/hepta-native-product-shell-gate-v2.sh"
TEST_PARENT="${TMPDIR:-/tmp}"
TEST_PARENT="${TEST_PARENT%/}"
TEST_ROOT="$(mktemp -d "$TEST_PARENT/hepta-native-v2-self-test.XXXXXX")"

cleanup() {
  case "$TEST_ROOT" in
    "$TEST_PARENT"/hepta-native-v2-self-test.*)
      rm -R -- "$TEST_ROOT"
      ;;
    *)
      echo "refusing to clean unexpected self-test path: $TEST_ROOT" >&2
      ;;
  esac
}
trap cleanup EXIT

mkdir -p "$TEST_ROOT/app"
cp -R "$APP_DIR/src" "$TEST_ROOT/app/src"

"$SYNC_CHECK" --strict --json > "$TEST_ROOT/sync.json"
ruby -rjson -e '
  report = JSON.parse(File.binread(ARGV.fetch(0)))
  abort "provenance was not ready" unless report["provenance_ready"] == true
  abort "downstream drift was not accounted" unless report["downstream_overlay_accounted"] == true
  abort "checker is not read-only" unless report["read_only"] == true
  abort "checker claims it would mutate" unless report["would_modify_worktree"] == false
  abort "push URL is not disabled" unless report.dig("checks", "remote_push_disabled") == true
  abort "wrong upstream count" unless report.dig("source", "upstream_file_count") == 242
' "$TEST_ROOT/sync.json"

"$PRODUCT_GATE" --output "$TEST_ROOT/product.json"
ruby -rjson -e '
  report = JSON.parse(File.binread(ARGV.fetch(0)))
  abort "Native UI source gate was not ready" unless report["native_ui_ready"] == true
  abort "real Robrix modules were not complete" unless report.dig("product_shell", "real_robrix_modules_ready") == true
  abort "cockpit was present on the default route" unless report.dig("product_shell", "no_cockpit_default") == true
  abort "downstream source contracts were incomplete" unless report.dig("product_shell", "downstream_source_contracts_ready") == true
  %w[
    offline_fixture_ready
    macos_local_package_ready
    matrix_live_ready
    hepta_live_bridge_ready
    backend_live_adapter_ready
    real_device_lab_ready
    signing_ready
    notarization_ready
    stapling_ready
    public_distribution_ready
    public_ga_ready
    full_product_ready
  ].each do |field|
    abort "#{field} was dishonestly promoted" unless report[field] == false
  end
' "$TEST_ROOT/product.json"

printf '\n// hepta_fixture_cockpit negative self-test marker\n' >> "$TEST_ROOT/app/src/home/home_screen.rs"
set +e
HEPTA_NATIVE_V2_APP_DIR="$TEST_ROOT/app" \
HEPTA_NATIVE_V2_ALLOW_TEST_ROOT=1 \
  "$PRODUCT_GATE" --output "$TEST_ROOT/cockpit-negative.json"
cockpit_exit=$?
set -e
if [[ "$cockpit_exit" -eq 0 ]]; then
  echo "cockpit negative case unexpectedly passed" >&2
  exit 1
fi
ruby -rjson -e '
  report = JSON.parse(File.binread(ARGV.fetch(0)))
  abort "cockpit marker was not detected" unless report.dig("product_shell", "no_cockpit_default") == false
  abort "cockpit case promoted Native UI" unless report["native_ui_ready"] == false
' "$TEST_ROOT/cockpit-negative.json"

cp "$APP_DIR/src/home/home_screen.rs" "$TEST_ROOT/app/src/home/home_screen.rs"
mv "$TEST_ROOT/app/src/room/room_input_bar.rs" "$TEST_ROOT/room_input_bar.rs.hold"
set +e
HEPTA_NATIVE_V2_APP_DIR="$TEST_ROOT/app" \
HEPTA_NATIVE_V2_ALLOW_TEST_ROOT=1 \
  "$PRODUCT_GATE" --output "$TEST_ROOT/module-negative.json"
module_exit=$?
set -e
mv "$TEST_ROOT/room_input_bar.rs.hold" "$TEST_ROOT/app/src/room/room_input_bar.rs"
if [[ "$module_exit" -eq 0 ]]; then
  echo "missing-module negative case unexpectedly passed" >&2
  exit 1
fi
ruby -rjson -e '
  report = JSON.parse(File.binread(ARGV.fetch(0)))
  abort "missing module was not detected" unless report.dig("product_shell", "real_robrix_modules_ready") == false
  abort "missing module case promoted Native UI" unless report["native_ui_ready"] == false
  abort "missing module case promoted GA" unless report["public_ga_ready"] == false
' "$TEST_ROOT/module-negative.json"

echo "hepta-native product-shell gate v2 self-test: PASS"
