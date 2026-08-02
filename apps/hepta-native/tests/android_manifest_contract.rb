#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "open3"
require "pathname"
require "rexml/document"

ANDROID_NS = "http://schemas.android.com/apk/res/android"
BROAD_MEDIA_PERMISSIONS = %w[
  android.permission.MANAGE_EXTERNAL_STORAGE
  android.permission.READ_EXTERNAL_STORAGE
  android.permission.READ_MEDIA_IMAGES
  android.permission.READ_MEDIA_VIDEO
].freeze
REQUIRED_CONFIG_CHANGES = %w[
  orientation
  screenSize
  keyboardHidden
  fontScale
  locale
  layoutDirection
].freeze
COMPILED_CONFIG_CHANGES = 0x400024a4
PINNED_MAKEPAD_REVISION = "c4335cee10b22aca768510c9d072b0ca1bba15c8"

def usage!
  warn <<~USAGE
    usage: tests/android_manifest_contract.rb [--static-only | --rendered PATH | --apk PATH] [--aapt PATH] [--tools-dir PATH] [--json]

      --static-only    validate the source template without claiming packaging adoption
      --rendered PATH  validate a plaintext manifest rendered by cargo-makepad
      --apk PATH       validate AndroidManifest.xml as compiled into an actual APK
      --aapt PATH      aapt executable used with --apk (or set HEPTA_ANDROID_AAPT)
      --tools-dir PATH pinned cargo-makepad root (or set HEPTA_NATIVE_MOBILE_TOOLS_DIR)
      --json           emit a machine-readable report
  USAGE
  exit 64
end

mode = :packager
artifact_path = nil
aapt_path = nil
tools_dir = ENV["HEPTA_NATIVE_MOBILE_TOOLS_DIR"]
json = false
args = ARGV.dup
until args.empty?
  case (arg = args.shift)
  when "--static-only"
    usage! unless mode == :packager && artifact_path.nil?
    mode = :static
  when "--rendered"
    usage! unless mode == :packager && artifact_path.nil? && !args.empty?
    mode = :rendered
    artifact_path = Pathname(args.shift).expand_path
  when "--apk"
    usage! unless mode == :packager && artifact_path.nil? && !args.empty?
    mode = :apk
    artifact_path = Pathname(args.shift).expand_path
  when "--aapt"
    usage! if args.empty? || aapt_path
    aapt_path = Pathname(args.shift).expand_path
  when "--tools-dir"
    usage! if args.empty?
    tools_dir = args.shift
  when "--json"
    json = true
  when "--help", "-h"
    usage!
  else
    warn "unknown argument: #{arg}"
    usage!
  end
end

app_root = Pathname(__dir__).parent.realpath
repo_root = app_root.parent.parent
template_path = app_root.join("resources/android/AndroidManifest.xml.template")
errors = []

def android_attr(element, name)
  element&.attributes&.get_attribute_ns(ANDROID_NS, name)&.value
end

def child_names(element, child_name, android_attribute)
  element.get_elements(child_name).map { |child| android_attr(child, android_attribute) }.compact
end

def validate_manifest(xml, expected_package:, errors:)
  document = REXML::Document.new(xml)
  manifest = document.root
  unless manifest&.name == "manifest"
    errors << "root element must be <manifest>"
    return
  end

  package_name = manifest.attributes["package"]
  errors << "package must be #{expected_package.inspect}, got #{package_name.inspect}" unless package_name == expected_package

  application = manifest.elements["application"]
  if application.nil?
    errors << "manifest must contain <application>"
    return
  end

  {
    "allowBackup" => "false",
    "fullBackupContent" => "false",
    "usesCleartextTraffic" => "false",
  }.each do |name, expected|
    actual = android_attr(application, name)
    errors << "application android:#{name} must be #{expected.inspect}, got #{actual.inspect}" unless actual == expected
  end

  max_aspect = application.get_elements("meta-data").find do |metadata|
    android_attr(metadata, "name") == "android.max_aspect"
  end
  errors << "android.max_aspect must not constrain tall displays" if max_aspect

  activity = application.elements["activity"]
  if activity.nil?
    errors << "application must contain the Makepad activity"
    return
  end

  config_changes = android_attr(activity, "configChanges").to_s.split("|")
  missing_config_changes = REQUIRED_CONFIG_CHANGES - config_changes
  unless missing_config_changes.empty?
    errors << "activity configChanges is missing: #{missing_config_changes.join(', ')}"
  end

  filters = activity.get_elements("intent-filter")
  launch_filter = filters.find do |filter|
    child_names(filter, "action", "name").include?("android.intent.action.MAIN") &&
      child_names(filter, "category", "name").include?("android.intent.category.LAUNCHER")
  end
  errors << "launcher intent filter is missing" unless launch_filter

  %w[matrix hepta-native].each do |scheme|
    deep_link_filter = filters.find do |filter|
      child_names(filter, "data", "scheme").include?(scheme)
    end
    if deep_link_filter.nil?
      errors << "#{scheme} deep-link intent filter is missing"
      next
    end

    actions = child_names(deep_link_filter, "action", "name")
    categories = child_names(deep_link_filter, "category", "name")
    errors << "#{scheme} deep-link filter must use VIEW" unless actions.include?("android.intent.action.VIEW")
    %w[android.intent.category.DEFAULT android.intent.category.BROWSABLE].each do |category|
      errors << "#{scheme} deep-link filter is missing #{category}" unless categories.include?(category)
    end

    if scheme == "hepta-native"
      callback = deep_link_filter.get_elements("data").find do |data|
        android_attr(data, "scheme") == scheme
      end
      host = android_attr(callback, "host")
      errors << "hepta-native callback must be restricted to host login, got #{host.inspect}" unless host == "login"
    end
  end

  active_permissions = manifest.get_elements("uses-permission").map do |permission|
    android_attr(permission, "name")
  end.compact
  broad_permissions = active_permissions & BROAD_MEDIA_PERMISSIONS
  errors << "broad media/storage permissions must remain absent: #{broad_permissions.join(', ')}" unless broad_permissions.empty?

  %w[android.permission.INTERNET android.permission.ACCESS_NETWORK_STATE].each do |permission|
    errors << "required network permission is missing: #{permission}" unless active_permissions.include?(permission)
  end
rescue REXML::ParseException => error
  errors << "manifest XML is invalid: #{error.message}"
end

def verify_packager_contract(repo_root, tools_dir, errors)
  result = {
    adoption: "hard_false_pinned_tools_dir_missing",
    receipt: nil,
    revision: nil,
    binary_sha256: nil,
  }
  if tools_dir.to_s.empty?
    errors << "pinned cargo-makepad tools directory is required (pass --tools-dir or HEPTA_NATIVE_MOBILE_TOOLS_DIR)"
    return result
  end

  wrapper = repo_root.join("scripts/hepta-native-mobile-cargo")
  unless wrapper.executable?
    errors << "canonical mobile cargo wrapper is missing or not executable: #{wrapper}"
    result[:adoption] = "hard_false_canonical_wrapper_missing"
    return result
  end

  resolved_tools_dir = Pathname(tools_dir).expand_path
  receipt_path = resolved_tools_dir.join("cargo-makepad-receipt.json")
  result[:receipt] = receipt_path.to_s
  command = [wrapper.to_s]
  command.concat(["--tools-dir", resolved_tools_dir.to_s])
  command.concat(["android", "help"])
  stdout, stderr, status = Open3.capture3(*command, chdir: repo_root.to_s)
  help = "#{stdout}\n#{stderr}"
  receipt = if receipt_path.file?
    JSON.parse(receipt_path.binread)
  else
    {}
  end
  result[:revision] = receipt["revision"]
  result[:binary_sha256] = receipt["binary_sha256"]
  receipt_ready = receipt["revision"] == PINNED_MAKEPAD_REVISION &&
    receipt["exact_revision_source_marker_ready"] == true &&
    receipt["custom_android_manifest_help_contract_ready"] == true &&
    receipt["global_cargo_makepad_used"] == false
  supports_template = status.success? && receipt_ready &&
    help.include?("Custom AndroidManifest") &&
    help.include?("resources/android/AndroidManifest.xml.template") &&
    help.include?("{package_id}")
  if supports_template
    result[:adoption] = "pinned_cargo_makepad_template_contract_verified"
  else
    detail = help.lines.map(&:strip).reject(&:empty?).last(3).join(" | ")
    errors << "canonical pinned cargo-makepad did not prove the custom AndroidManifest template contract#{detail.empty? ? '' : ": #{detail}"}"
    errors << "pinned cargo-makepad receipt must bind revision #{PINNED_MAKEPAD_REVISION}" unless receipt_ready
    result[:adoption] = "hard_false_pinned_cargo_makepad_template_contract_unverified"
  end
  result
rescue JSON::ParserError => error
  errors << "pinned cargo-makepad receipt is invalid JSON: #{error.message}"
  result[:adoption] = "hard_false_pinned_cargo_makepad_receipt_invalid"
  result
end

def find_aapt(explicit_path)
  return explicit_path if explicit_path

  env_path = ENV["HEPTA_ANDROID_AAPT"]
  return Pathname(env_path).expand_path unless env_path.to_s.empty?

  sdk_roots = %w[HEPTA_ANDROID_SDK_DIR ANDROID_SDK_ROOT ANDROID_HOME].each_with_object([]) do |name, roots|
    value = ENV[name]
    roots << Pathname(value).expand_path unless value.to_s.empty?
  end
  sdk_roots << Pathname(Dir.home).join("Library/Android/sdk")
  candidates = sdk_roots.flat_map do |root|
    Dir.glob(root.join("build-tools/*/aapt").to_s).map { |path| Pathname(path) }
  end
  candidates.select(&:executable?).sort_by(&:to_s).last
end

def attribute_value_present?(dump, name, expected)
  dump.lines.any? do |line|
    line.include?("android:#{name}") && line.match?(expected)
  end
end

def validate_apk_manifest_dump(dump, expected_package:, errors:)
  unless dump.lines.any? { |line| line.include?("package=\"#{expected_package}\"") }
    errors << "compiled APK package must be #{expected_package.inspect}"
  end

  {
    "allowBackup" => /(?:\(type 0x12\))?0x0(?:\s|$)/,
    "fullBackupContent" => /(?:\(type 0x12\))?0x0(?:\s|$)/,
    "usesCleartextTraffic" => /(?:\(type 0x12\))?0x0(?:\s|$)/,
  }.each do |name, expected|
    errors << "compiled APK android:#{name} must be false" unless attribute_value_present?(dump, name, expected)
  end

  errors << "compiled APK must not contain android.max_aspect" if dump.include?("android.max_aspect")
  unless attribute_value_present?(dump, "configChanges", /0x#{COMPILED_CONFIG_CHANGES.to_s(16)}(?:\s|$)/i)
    errors << "compiled APK configChanges must include orientation, screenSize, keyboardHidden, fontScale, locale, and layoutDirection"
  end

  %w[
    android.intent.action.MAIN
    android.intent.category.LAUNCHER
    android.intent.action.VIEW
    android.intent.category.DEFAULT
    android.intent.category.BROWSABLE
  ].each do |entry|
    errors << "compiled APK intent contract is missing #{entry}" unless dump.include?(entry)
  end
  %w[matrix hepta-native].each do |scheme|
    errors << "compiled APK is missing #{scheme} deep-link scheme" unless dump.include?("=\"#{scheme}\"")
  end
  errors << "compiled APK hepta-native callback must be restricted to host login" unless dump.include?("=\"login\"")

  BROAD_MEDIA_PERMISSIONS.each do |permission|
    errors << "compiled APK must not request broad media/storage permission #{permission}" if dump.include?(permission)
  end
  %w[android.permission.INTERNET android.permission.ACCESS_NETWORK_STATE].each do |permission|
    errors << "compiled APK is missing required network permission #{permission}" unless dump.include?(permission)
  end
end

substitutions = {
  "{package_id}" => "ai.hepta.nativeapp",
  "{label}" => "Hepta",
  "{class_name}" => "MakepadApp",
  "{min_sdk_version}" => "26",
  "{target_sdk_version}" => "35",
  "{version_code}" => "2026080201",
  "{version_name}" => "1.0.0-alpha.1",
  "{debuggable}" => "false",
}
template_errors = []
template = template_path.binread
unknown_tokens = template.scan(/\{[^}]+\}/).uniq - substitutions.keys
template_errors << "unknown manifest template tokens: #{unknown_tokens.join(', ')}" unless unknown_tokens.empty?
rendered_template = template.gsub(/\{[^}]+\}/) { |token| substitutions.fetch(token, token) }
validate_manifest(rendered_template, expected_package: "ai.hepta.nativeapp", errors: template_errors)
errors.concat(template_errors)

packager_adoption = "not_checked"
packager_receipt = nil
aapt_used = nil
apk_manifest_verified = false
case mode
when :static
  packager_adoption = "not_claimed"
when :rendered
  if artifact_path&.file?
    rendered_errors = []
    validate_manifest(artifact_path.binread, expected_package: "ai.hepta.nativeapp", errors: rendered_errors)
    errors.concat(rendered_errors)
    packager_adoption = rendered_errors.empty? ? "rendered_manifest_verified" : "rendered_manifest_rejected"
  else
    errors << "rendered manifest does not exist: #{artifact_path}"
    packager_adoption = "rendered_manifest_missing"
  end
when :apk
  packager = verify_packager_contract(repo_root, tools_dir, errors)
  packager_adoption = packager[:adoption]
  packager_receipt = packager
  if artifact_path&.file?
    resolved_aapt = find_aapt(aapt_path)
    if resolved_aapt&.executable?
      aapt_used = resolved_aapt.to_s
      stdout, stderr, status = Open3.capture3(
        resolved_aapt.to_s,
        "dump",
        "xmltree",
        artifact_path.to_s,
        "AndroidManifest.xml",
      )
      if status.success?
        apk_errors = []
        validate_apk_manifest_dump(stdout, expected_package: "ai.hepta.nativeapp", errors: apk_errors)
        errors.concat(apk_errors)
        apk_manifest_verified = apk_errors.empty?
      else
        errors << "aapt could not read the APK manifest: #{stderr.lines.map(&:strip).reject(&:empty?).last}"
      end
    else
      errors << "aapt executable is required for --apk (pass --aapt or HEPTA_ANDROID_AAPT)"
    end
  else
    errors << "APK does not exist: #{artifact_path}"
  end
when :packager
  packager = verify_packager_contract(repo_root, tools_dir, errors)
  packager_adoption = packager[:adoption]
  packager_receipt = packager
  errors << "no APK manifest was supplied; rerun with --apk PATH before artifact promotion"
end

report = {
  schema_version: 2,
  kind: "hepta-native-android-manifest-contract",
  status: errors.empty? ? "ready" : "not_ready",
  mode: mode.to_s,
  template: template_path.to_s,
  artifact: artifact_path&.to_s,
  aapt: aapt_used,
  pinned_tools_dir: tools_dir.to_s.empty? ? nil : Pathname(tools_dir).expand_path.to_s,
  packager_adoption: packager_adoption,
  packager_receipt: packager_receipt,
  claims: {
    source_template_verified: template_errors.empty?,
    apk_manifest_verified: apk_manifest_verified,
    manifest_contract_ready: mode == :apk && apk_manifest_verified && errors.empty?,
    android_artifact_ready: false,
    android_device_ready: false,
    android_release_ready: false,
    overall_mobile_ready: false,
    rtl_complete: false,
    dynamic_type_complete: false,
    ime_complete: false,
  },
  errors: errors,
}

if json
  puts JSON.pretty_generate(report)
else
  puts "Android manifest contract: #{report[:status]}"
  puts "  mode: #{report[:mode]}"
  puts "  packager adoption: #{packager_adoption}"
  errors.each { |error| puts "  - #{error}" }
end

exit(errors.empty? ? 0 : 1)
