#!/usr/bin/env bash
set -euo pipefail

# Source-only v2 gate for the upstream-first Native product shell.
# Runtime, fixture, package, device-lab, and public-release readiness remain
# separate explicit false values unless independently proven by later gates.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel)"
APP_DIR="${HEPTA_NATIVE_V2_APP_DIR:-$REPO_ROOT/apps/hepta-native}"

if [[ "$APP_DIR" != "$REPO_ROOT/apps/hepta-native" && "${HEPTA_NATIVE_V2_ALLOW_TEST_ROOT:-0}" != "1" ]]; then
  echo "refusing alternate app root without HEPTA_NATIVE_V2_ALLOW_TEST_ROOT=1" >&2
  exit 64
fi

export HEPTA_NATIVE_V2_REPO_ROOT="$REPO_ROOT"
export HEPTA_NATIVE_V2_APP_DIR="$APP_DIR"
export HEPTA_NATIVE_V2_SYNC_CHECK="$SCRIPT_DIR/hepta-native-robrix-upstream-sync-check-v2.sh"

exec ruby - "$@" <<'RUBY'
require "digest"
require "fileutils"
require "json"
require "open3"
require "pathname"
require "time"

def capture_source_binding(repo)
  stdout, stderr, status = Open3.capture3(repo.join("scripts/hepta-ui-source-fingerprint").to_s, chdir: repo.to_s)
  raise "source fingerprint failed: #{stderr.strip}" unless status.success?
  JSON.parse(stdout)
end

def binding_equal?(left, right)
  %w[head head_tree source_fingerprint].all? { |key| left[key] == right[key] }
end

repo = Pathname(ENV.fetch("HEPTA_NATIVE_V2_REPO_ROOT")).realpath
app_dir = Pathname(ENV.fetch("HEPTA_NATIVE_V2_APP_DIR")).realpath
sync_check = Pathname(ENV.fetch("HEPTA_NATIVE_V2_SYNC_CHECK")).realpath
canonical_app_dir = repo.join("apps/hepta-native").realpath
test_root_override = app_dir != canonical_app_dir
binding_before = capture_source_binding(repo)

output_path = nil
ARGV.each_with_index do |arg, index|
  case arg
  when "--output"
    output_path = ARGV[index + 1]
    if output_path.nil? || output_path.start_with?("--")
      warn "--output requires a path"
      exit 64
    end
  when "--json"
    # JSON is the only output format; retained for explicit callers.
  when "--help", "-h"
    puts "usage: #{File.basename($PROGRAM_NAME)} [--json] [--output REPORT.json]"
    exit 0
  else
    next if index.positive? && ARGV[index - 1] == "--output"
    warn "unknown argument: #{arg}"
    exit 64
  end
end

sync_stdout, sync_stderr, sync_status = Open3.capture3(sync_check.to_s, "--json", "--strict", chdir: repo.to_s)
sync_report = begin
  JSON.parse(sync_stdout)
rescue JSON::ParserError => error
  {
    "status" => "not_ready",
    "provenance_ready" => false,
    "downstream_overlay_accounted" => false,
    "errors" => ["sync_report_parse: #{error.message}", sync_stderr.strip].reject(&:empty?),
  }
end

requirements = {
  "desktop.main" => ["src/home/main_desktop_ui.rs", %w[MainDesktopUI RoomScreen]],
  "desktop.sidebar" => ["src/home/rooms_sidebar.rs", %w[RoomsSideBar RoomsList]],
  "desktop.dock" => ["src/home/light_themed_dock.rs", %w[Dock]],
  "rooms.list" => ["src/home/rooms_list.rs", %w[RoomsList]],
  "rooms.entry" => ["src/home/rooms_list_entry.rs", %w[RoomsListEntry]],
  "room.screen" => ["src/home/room_screen.rs", %w[RoomScreen RoomInputBar]],
  "room.module" => ["src/room/mod.rs", %w[room_input_bar reply_preview]],
  "room.reply" => ["src/room/reply_preview.rs", %w[CollapsiblePreview]],
  "room.typing" => ["src/room/typing_notice.rs", %w[TypingNotice]],
  "composer.input" => ["src/room/room_input_bar.rs", %w[RoomInputBar]],
  "composer.mentions" => ["src/shared/mention_popup.rs", %w[MentionablePopup]],
  "composer.commands" => ["src/shared/slash_commands.rs", %w[SlashCommand]],
  "composer.menu" => ["src/shared/room_input_popup_menu.rs", %w[RoomInputPopupMenu]],
  "composer.upload" => ["src/shared/file_upload_modal.rs", %w[FileUploadModal]],
  "mobile.main" => ["src/home/main_mobile_ui.rs", %w[MainMobileUI RoomScreen]],
  "navigation.home" => ["src/home/home_screen.rs", %w[HomeScreen NavigationTabBar MainDesktopUI]],
  "navigation.tabs" => ["src/home/navigation_tab_bar.rs", %w[NavigationTabBar]],
  "navigation.spaces" => ["src/home/spaces_bar.rs", %w[SpacesBar]],
  "navigation.space_lobby" => ["src/home/space_lobby.rs", %w[SpaceLobby]],
  "upload.progress" => ["src/home/upload_progress.rs", %w[UploadProgress]],
}.freeze

module_checks = {}
requirements.each do |name, (relative_path, markers)|
  path = app_dir.join(relative_path)
  content = path.file? ? path.binread : ""
  marker_results = markers.to_h { |marker| [marker, content.include?(marker)] }
  module_checks[name] = {
    "path" => relative_path,
    "exists" => path.file?,
    "bytes" => content.bytesize,
    "sha256" => content.empty? ? nil : Digest::SHA256.hexdigest(content),
    "markers" => marker_results,
    "ready" => path.file? && content.bytesize.positive? && marker_results.values.all?,
  }
end

default_entry_paths = %w[
  src/main.rs
  src/app.rs
  src/home/mod.rs
  src/home/home_screen.rs
  src/home/main_desktop_ui.rs
  src/home/main_mobile_ui.rs
].freeze
forbidden_default_markers = %w[
  hepta_fixture_cockpit
  hepta_productization
  hepta_inspector
  hepta_mobile_detail
  hepta_mobile_safety
  hepta_telegram_base_contract
  HEPTA_NATIVE_FIXTURE
].freeze

default_marker_hits = []
default_entry_paths.each do |relative_path|
  path = app_dir.join(relative_path)
  next unless path.file?
  content = path.binread
  forbidden_default_markers.each do |marker|
    default_marker_hits << { "path" => relative_path, "marker" => marker } if content.include?(marker)
  end
end

contains_marker = lambda do |relative_path, marker|
  path = app_dir.join(relative_path)
  path.file? && path.binread.include?(marker)
end
shell_relationships = {
  "desktop_owns_room_screen" => contains_marker.call("src/home/main_desktop_ui.rs", "RoomScreen"),
  "mobile_owns_room_screen" => contains_marker.call("src/home/main_mobile_ui.rs", "RoomScreen"),
  "home_owns_navigation" => contains_marker.call("src/home/home_screen.rs", "NavigationTabBar"),
  "room_owns_composer" => contains_marker.call("src/home/room_screen.rs", "RoomInputBar"),
}

source_contract_requirements = {
  "hepta_identity" => {
    "src/lib.rs" => %w[APP_QUALIFIER APP_ORGANIZATION APP_NAME],
    "src/app.rs" => ["Hepta"],
  },
  "hepta_theme" => {
    "src/shared/hepta_theme.rs" => %w[HEPTA_GLASS HEPTA_CONTENT HEPTA_FOCUS],
    "src/shared/styles.rs" => %w[HEPTA_GLASS HEPTA_CONTENT],
  },
  "secure_matrix_session" => {
    "src/persistence/matrix_session_store/mod.rs" => %w[persist_secure_session_with_store load_session_material_with_store],
    "src/persistence/matrix_session_store/credential.rs" => %w[MATRIX_CREDENTIAL_SERVICE SYSTEM_CREDENTIAL_STORE_SUPPORTED],
    "src/persistence/matrix_state.rs" => %w[save_session_material load_session_material clear_session_material],
  },
  "side_effect_free_bridge_contract" => {
    "src/hepta_bridge/contract.rs" => %w[HEPTA_BRIDGE_SCHEMA_VERSION ConversationBinding],
    "src/hepta_bridge/adapter.rs" => %w[DisabledBridgeAdapter BridgeCapabilities],
    "src/hepta_bridge/presenter.rs" => %w[DEFAULT_PRESENTATION_PAYLOAD_CAP_BYTES MAX_PRESENTATION_PAYLOAD_CAP_BYTES],
  },
  "developer_diagnostics_boundary" => {
    "src/settings/developer_diagnostics.rs" => ["cfg!(feature = \"developer-diagnostics\")", "disabled adapter"],
    "src/settings/settings_screen.rs" => ["DeveloperDiagnostics"],
  },
}.freeze

source_contract_checks = {}
source_contract_requirements.each do |name, path_requirements|
  path_checks = {}
  path_requirements.each do |relative_path, markers|
    path = app_dir.join(relative_path)
    content = path.file? ? path.binread : ""
    marker_results = markers.to_h { |marker| [marker, content.include?(marker)] }
    path_checks[relative_path] = {
      "exists" => path.file?,
      "bytes" => content.bytesize,
      "sha256" => content.empty? ? nil : Digest::SHA256.hexdigest(content),
      "markers" => marker_results,
      "ready" => path.file? && content.bytesize.positive? && marker_results.values.all?,
    }
  end
  source_contract_checks[name] = {
    "paths" => path_checks,
    "ready" => path_checks.values.all? { |check| check["ready"] },
  }
end

provenance_ready = sync_report["provenance_ready"] == true
downstream_overlay_accounted = sync_report["downstream_overlay_accounted"] == true
real_robrix_modules_ready = module_checks.values.all? { |check| check["ready"] }
real_shell_relationships_ready = shell_relationships.values.all?
no_cockpit_default = default_marker_hits.empty?
downstream_source_contracts_ready = source_contract_checks.values.all? { |check| check["ready"] }

binding_after = capture_source_binding(repo)
source_stable = binding_equal?(binding_before, binding_after)
sync_bound_to_source = sync_report["source_stable_during_run"] == true &&
  binding_equal?(sync_report.fetch("source_binding", {}), binding_after)

native_ui_ready = source_stable && sync_bound_to_source && provenance_ready && downstream_overlay_accounted && real_robrix_modules_ready && real_shell_relationships_ready && no_cockpit_default && downstream_source_contracts_ready

# These are intentionally not inferred from source presence or a successful build.
# Each requires a separate, current-source evidence-producing gate or real device run.
offline_fixture_ready = false
macos_local_package_ready = false
matrix_live_ready = false
hepta_live_bridge_ready = false
backend_live_adapter_ready = false
real_device_lab_ready = false
signing_ready = false
notarization_ready = false
stapling_ready = false
public_distribution_ready = false
public_ga_ready = false
full_product_ready = false

report = {
  "schema_version" => 3,
  "kind" => "hepta-native-upstream-first-product-shell-gate",
  "generated_at_utc" => Time.now.utc.iso8601,
  "status" => native_ui_ready ? "ready" : "not_ready",
  "scope" => "source_only_native_ui",
  "source_gate_only" => true,
  "source_binding_before" => binding_before,
  "source_binding" => binding_after,
  "source_stable_during_run" => source_stable,
  "sync_receipt_bound_to_source" => sync_bound_to_source,
  "test_root_override" => test_root_override,
  "upstream" => {
    "repository" => sync_report.dig("source", "repository"),
    "commit" => sync_report.dig("source", "commit"),
    "tree" => sync_report.dig("source", "tree"),
    "raw_import_commit" => sync_report.dig("source", "raw_import_commit"),
    "current_lineage_import_commit" => sync_report.dig("source", "current_lineage_import_commit"),
    "provenance_ready" => provenance_ready,
    "downstream_overlay_accounted" => downstream_overlay_accounted,
    "sync_check_status" => sync_report["status"],
    "sync_check_exit_success" => sync_status.success?,
  },
  "product_shell" => {
    "real_robrix_modules_ready" => real_robrix_modules_ready,
    "real_shell_relationships_ready" => real_shell_relationships_ready,
    "no_cockpit_default" => no_cockpit_default,
    "downstream_source_contracts_ready" => downstream_source_contracts_ready,
    "module_checks" => module_checks,
    "shell_relationships" => shell_relationships,
    "source_contract_checks" => source_contract_checks,
    "forbidden_default_marker_hits" => default_marker_hits,
  },
  "native_ui_ready" => native_ui_ready,
  "offline_fixture_ready" => offline_fixture_ready,
  "macos_local_package_ready" => macos_local_package_ready,
  "matrix_live_ready" => matrix_live_ready,
  "hepta_live_bridge_ready" => hepta_live_bridge_ready,
  "backend_live_adapter_ready" => backend_live_adapter_ready,
  "real_device_lab_ready" => real_device_lab_ready,
  "signing_ready" => signing_ready,
  "notarization_ready" => notarization_ready,
  "stapling_ready" => stapling_ready,
  "public_distribution_ready" => public_distribution_ready,
  "public_ga_ready" => public_ga_ready,
  "full_product_ready" => full_product_ready,
  "boundary_reasons" => {
    "offline_fixture_ready" => "not_run_by_source_only_gate",
    "macos_local_package_ready" => "no_current_source_package_evidence_consumed",
    "matrix_live_ready" => "no_live_homeserver_session_exercised",
    "hepta_live_bridge_ready" => "no_authoritative_runtime_bridge_receipt_exercised",
    "backend_live_adapter_ready" => "the_source_contains_only_a_disabled_adapter_and_contract_types_not_a_live_backend_connection",
    "real_device_lab_ready" => "no_current_real_ios_android_device_lab_evidence",
    "signing_ready" => "no_signing_identity_or_signed_current_source_artifact_consumed",
    "notarization_ready" => "no_Apple_notarization_submission_or_acceptance_consumed",
    "stapling_ready" => "no_notarization_ticket_stapled_to_a_current_source_artifact",
    "public_distribution_ready" => "no_store_upload_or_public_distribution_authority",
    "public_ga_ready" => "signing_notarization_stapling_distribution_and_full_product_gates_are_not_complete",
    "full_product_ready" => "UI source readiness cannot imply backend runtime device lab or release readiness",
  },
  "legacy_v1_gate_reused" => false,
  "sync_check_errors" => Array(sync_report["errors"]) + (sync_stderr.empty? ? [] : [sync_stderr.strip]),
}

json = JSON.pretty_generate(report) + "\n"
if output_path
  destination = Pathname(output_path).expand_path
  FileUtils.mkdir_p(destination.dirname)
  destination.binwrite(json)
else
  print json
end

exit(native_ui_ready ? 0 : 1)
RUBY
