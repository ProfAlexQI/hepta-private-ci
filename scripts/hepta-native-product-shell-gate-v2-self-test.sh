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
  abort "remote hygiene is not ready" unless report.dig("checks", "remote_hygiene_ready") == true
  abort "wrong upstream count" unless report.dig("source", "upstream_file_count") == 242
' "$TEST_ROOT/sync.json"

"$PRODUCT_GATE" --output "$TEST_ROOT/product.json"
ruby -rjson -e '
  report = JSON.parse(File.binread(ARGV.fetch(0)))
  abort "Native UI source gate was not ready" unless report["native_ui_ready"] == true
  abort "real Robrix modules were not complete" unless report.dig("product_shell", "real_robrix_modules_ready") == true
  abort "real mobile widget tree was not ready" unless report.dig("product_shell", "mobile_widget_tree_contract", "ready") == true
  abort "rooms header toggle contract was not ready" unless report.dig("product_shell", "rooms_header_toggle_contract_ready") == true
  abort "retired ghost modules were not absent" unless report.dig("product_shell", "retired_ghost_modules_ready") == true
  abort "mobile gate did not bind HomeScreen" unless report.dig("product_shell", "mobile_widget_tree_contract", "path") == "src/home/home_screen.rs"
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

# The post-login semantic tree is authoritative. A login-only tree must not
# satisfy the product-shell source contract.
ruby -e '
  path = ARGV.fetch(0)
  source = File.binread(path)
  needle = "crate::accessibility::publish_home_tree("
  abort "missing post-login accessibility fixture target" unless source.scan(needle).length == 1
  File.binwrite(path, source.sub(needle, "crate::accessibility::disabled_home_tree_for_test("))
' "$TEST_ROOT/app/src/home/home_screen.rs"
set +e
HEPTA_NATIVE_V2_APP_DIR="$TEST_ROOT/app" \
HEPTA_NATIVE_V2_ALLOW_TEST_ROOT=1 \
  "$PRODUCT_GATE" --output "$TEST_ROOT/post-login-accessibility-negative.json"
post_login_accessibility_exit=$?
set -e
if [[ "$post_login_accessibility_exit" -eq 0 ]]; then
  echo "missing post-login accessibility publication unexpectedly passed" >&2
  exit 1
fi
ruby -rjson -e '
  report = JSON.parse(File.binread(ARGV.fetch(0)))
  contract = report.dig("product_shell", "source_contract_checks", "password_safe_accessibility_tree")
  abort "missing post-login accessibility publication was not detected" unless contract.dig("paths", "src/home/home_screen.rs", "ready") == false
  abort "broken post-login accessibility contract remained ready" unless contract["ready"] == false
  abort "broken post-login accessibility contract promoted Native UI" unless report["native_ui_ready"] == false
' "$TEST_ROOT/post-login-accessibility-negative.json"
cp "$APP_DIR/src/home/home_screen.rs" "$TEST_ROOT/app/src/home/home_screen.rs"

printf '\nfn retired_rooms_header_panic() { todo!("Handle other header categories") }\n' \
  >> "$TEST_ROOT/app/src/home/rooms_list.rs"
set +e
HEPTA_NATIVE_V2_APP_DIR="$TEST_ROOT/app" \
HEPTA_NATIVE_V2_ALLOW_TEST_ROOT=1 \
  "$PRODUCT_GATE" --output "$TEST_ROOT/rooms-header-negative.json"
rooms_header_exit=$?
set -e
if [[ "$rooms_header_exit" -eq 0 ]]; then
  echo "rooms header panic negative case unexpectedly passed" >&2
  exit 1
fi
ruby -rjson -e '
  report = JSON.parse(File.binread(ARGV.fetch(0)))
  abort "rooms header panic was not detected" unless report.dig("product_shell", "rooms_header_toggle_contract", "panic_macro_absent") == false
  abort "rooms header panic contract remained ready" unless report.dig("product_shell", "rooms_header_toggle_contract_ready") == false
  abort "rooms header panic promoted Native UI" unless report["native_ui_ready"] == false
' "$TEST_ROOT/rooms-header-negative.json"
cp "$APP_DIR/src/home/rooms_list.rs" "$TEST_ROOT/app/src/home/rooms_list.rs"

# Break the live HomeScreen widget tree, then reintroduce both retired ghost
# modules as decoys. Neither can rescue the authoritative route.
ruby -e '
  path = ARGV.fetch(0)
  source = File.binread(path)
  needle = "view_stack := StackNavigation {"
  abort "missing mobile StackNavigation fixture target" unless source.scan(needle).length == 1
  File.binwrite(path, source.sub(needle, "view_stack := View {"))
' "$TEST_ROOT/app/src/home/home_screen.rs"
# Authoritative-file decoys must not rescue the broken widget tree. Cover both
# nested block comments and raw string literals, not just // comments in the
# unused legacy module.
printf '%s\n' \
  '/* outer decoy' \
  '  /* nested decoy: view_stack := StackNavigation { */' \
  '  main_adaptive_view := AdaptiveView {' \
  '  Mobile := SolidView {' \
  '  stack_templates: {' \
  '  RoomScreenStackNavigationView := mod.widgets.RobrixStackNavigationView {' \
  '  room_screen := mod.widgets.RoomScreen {}' \
  '  RoomsListAction::Selected(selected_room) if !effective_is_desktop(cx) => {' \
  '  self.push_selected_screen_view(cx, app_state, selected_room);' \
  '  stack_navigation.create_view_from_template(cx, id!(RoomScreenStackNavigationView))' \
  '  .room_screen(cx, ids!(room_screen)) .set_displayed_room(cx, room_name_id, thread_root);' \
  '  stack_navigation.push(cx, view_id);' \
  '*/' \
  'const _HEPTA_MOBILE_MARKER_DECOY: &[u8] = br################"main_adaptive_view := AdaptiveView { Mobile := SolidView { view_stack := StackNavigation { stack_templates: { RoomScreenStackNavigationView := mod.widgets.RobrixStackNavigationView { room_screen := mod.widgets.RoomScreen {} RoomsListAction::Selected(selected_room) if !effective_is_desktop(cx) => { self.push_selected_screen_view(cx, app_state, selected_room); stack_navigation.create_view_from_template(cx, id!(RoomScreenStackNavigationView)) .room_screen(cx, ids!(room_screen)) .set_displayed_room(cx, room_name_id, thread_root); stack_navigation.push(cx, view_id);"################;' \
  >> "$TEST_ROOT/app/src/home/home_screen.rs"
printf '%s\n' \
  '// false-positive fixture: main_adaptive_view := AdaptiveView {' \
  '// false-positive fixture: Mobile := SolidView {' \
  '// false-positive fixture: view_stack := StackNavigation {' \
  '// false-positive fixture: RoomScreenStackNavigationView := mod.widgets.RobrixStackNavigationView {' \
  '// false-positive fixture: room_screen := mod.widgets.RoomScreen {}' \
  '// false-positive fixture: RoomsListAction::Selected(selected_room) if !effective_is_desktop(cx) => {' \
  '// false-positive fixture: self.push_selected_screen_view(cx, app_state, selected_room);' \
  '// false-positive fixture: stack_navigation.create_view_from_template(cx, id!(RoomScreenStackNavigationView))' \
  '// false-positive fixture: .room_screen(cx, ids!(room_screen)) .set_displayed_room(cx, room_name_id, thread_root);' \
  '// false-positive fixture: stack_navigation.push(cx, view_id);' \
  >> "$TEST_ROOT/app/src/home/main_mobile_ui.rs"
printf '%s\n' 'struct SearchMessagesButton;' > "$TEST_ROOT/app/src/home/search_messages.rs"
set +e
HEPTA_NATIVE_V2_APP_DIR="$TEST_ROOT/app" \
HEPTA_NATIVE_V2_ALLOW_TEST_ROOT=1 \
  "$PRODUCT_GATE" --output "$TEST_ROOT/mobile-tree-negative.json"
mobile_tree_exit=$?
set -e
if [[ "$mobile_tree_exit" -eq 0 ]]; then
  echo "broken real mobile widget tree unexpectedly passed" >&2
  exit 1
fi
ruby -rjson -e '
  report = JSON.parse(File.binread(ARGV.fetch(0)))
  abort "broken real mobile tree was not detected" unless report.dig("product_shell", "mobile_widget_tree_contract", "ready") == false
  abort "block-comment/string decoys rescued the widget chain" unless report.dig("product_shell", "mobile_widget_tree_contract", "checks", "adaptive_mobile_stack_room_template_ordered") == false
  abort "dead MainMobileUI markers rescued the mobile contract" unless report.dig("product_shell", "shell_relationships", "mobile_owns_room_screen") == false
  abort "reintroduced ghost modules were accepted" unless report.dig("product_shell", "retired_ghost_modules_ready") == false
  abort "broken mobile tree promoted Native UI" unless report["native_ui_ready"] == false
' "$TEST_ROOT/mobile-tree-negative.json"

cp "$APP_DIR/src/home/home_screen.rs" "$TEST_ROOT/app/src/home/home_screen.rs"
rm -f "$TEST_ROOT/app/src/home/main_mobile_ui.rs" "$TEST_ROOT/app/src/home/search_messages.rs"

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
