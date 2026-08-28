# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"
require "optparse"
require "pathname"
require "tmpdir"

SCHEMA = "hepta.ui.v4.windows-product-host-implementation-qualification.v1"
SOURCE_PASS = "PASS_WINDOWS_PRODUCT_HOST_IMPLEMENTATION_CANDIDATE_SOURCE_ONLY"
COMPILE_PASS = "PASS_WINDOWS_PRODUCT_HOST_CANDIDATE_COMPILE"
REVIEW_PASS = "PASS_WINDOWS_PRODUCT_HOST_INTEGRATION_REVIEW_ENVELOPE"
REVIEW_BLOCKED = "BLOCKED_WINDOWS_RUNTIME_PROVENANCE_REQUIRED"
DEVICE_PASS = "PASS_WINDOWS_PRODUCT_HOST_IMPLEMENTATION_DEVICE_DRILL"
FEATURE = "hepta_ui_windows_system_material_v4"
MAKEPAD_REVISION = "c4335cee10b22aca768510c9d072b0ca1bba15c8"
PLATFORMS = %w[ubuntu-latest windows-latest macos-latest].freeze
MODES = %w[default-off explicit-feature].freeze
STATUSES = %w[
  BLOCKED_WINDOWS_PRODUCT_HOST_REVIEW_ENVELOPE_REQUIRED
  BLOCKED_WINDOWS_PRODUCT_HOST_DEVICE_QUALIFICATION_REQUIRED
  PASS_WINDOWS_PRODUCT_HOST_IMPLEMENTATION_QUALIFICATION
  FAIL_WINDOWS_PRODUCT_HOST_IMPLEMENTATION_QUALIFICATION
].freeze
MAX_FAILURES = 64
MAX_FAILURE_BYTES = 4096

class QualificationError < StandardError; end

Options = Struct.new(
  :candidate_commit,
  :candidate_tree,
  :source_receipt,
  :compile_receipts,
  :review_envelope,
  :device_receipt,
  :output,
  :self_test,
  keyword_init: true
)

def git_object_id?(value)
  value.is_a?(String) && value.match?(/\A[0-9a-f]{40}\z/)
end

def sha256?(value)
  value.is_a?(String) && value.match?(/\A[0-9a-f]{64}\z/)
end

def false_authority
  {
    "network" => false,
    "mutation" => false,
    "effect" => false,
    "liveAdapter" => false,
    "production" => false,
    "operatorAcceptance" => false,
    "promotion" => false,
    "release" => false
  }
end

def false_wiring
  {
    "productCargoFeatureDeclared" => false,
    "productModuleRegistered" => false,
    "productLifecycleWired" => false,
    "automaticBindingAllowed" => false,
    "implementationApproved" => false,
    "productHostMayBind" => false,
    "productBound" => false,
    "transientSystemMaterialBound" => false,
    "completeProfileBound" => false,
    "systemMaterialBound" => false,
    "nativeProductRuntime" => false,
    "deviceValidation" => false
  }
end

def parse_json(path)
  JSON.parse(Pathname.new(path).read)
rescue Errno::ENOENT
  raise QualificationError, "JSON file missing: #{path}"
rescue JSON::ParserError => error
  raise QualificationError, "invalid JSON #{path}: #{error.message}"
end

def digest_file(path)
  Digest::SHA256.file(path).hexdigest
end

def bounded_failure(value)
  text = value.to_s.encode(Encoding::UTF_8, invalid: :replace, undef: :replace, replace: "�")
  return text if text.bytesize <= MAX_FAILURE_BYTES

  bytes = text.byteslice(0, MAX_FAILURE_BYTES)
  bytes = bytes.byteslice(0, bytes.bytesize - 1) until bytes.valid_encoding?
  bytes
end

def write_json_atomic(path, payload)
  destination = Pathname.new(path).expand_path
  destination.dirname.mkpath
  temporary = destination.sub_ext(".tmp")
  temporary.write(JSON.pretty_generate(payload) + "\n")
  File.rename(temporary, destination)
end

def require_false_hash!(value, label)
  raise QualificationError, "#{label} is not an object" unless value.is_a?(Hash)
  escaped = value.select { |_key, entry| entry != false }.keys
  raise QualificationError, "#{label} escaped: #{escaped.join(', ')}" unless escaped.empty?
end

def reject_true_keys!(value, label)
  forbidden = %w[
    productCargoFeatureDeclared productModuleRegistered productLifecycleWired
    automaticBindingAllowed implementationApproved productHostMayBind productBound
    transientSystemMaterialBound completeProfileBound systemMaterialBound
    nativeProductRuntime deviceValidation network mutation effect liveAdapter
    production operatorAcceptance promotion release
    product_cargo_feature_declared product_module_registered product_lifecycle_wired
    automatic_binding_allowed implementation_approved product_host_may_bind product_bound
    transient_system_material_bound complete_profile_bound system_material_bound
    native_product_runtime device_validation production_authority effect_authority
    live_adapter_authority operator_acceptance
  ].freeze
  walk = lambda do |entry, path|
    case entry
    when Hash
      entry.each do |key, child|
        if forbidden.include?(key.to_s) && child == true
          raise QualificationError, "#{label} escaped at #{(path + [key]).join('.')}"
        end
        walk.call(child, path + [key])
      end
    when Array
      entry.each_with_index { |child, index| walk.call(child, path + [index]) }
    end
  end
  walk.call(value, [])
end

def false_compile_evidence(mode)
  {
    "qualified" => false,
    "status" => nil,
    "featureEnabled" => mode == "explicit-feature",
    "digest" => nil
  }
end

def empty_compile_matrix
  PLATFORMS.to_h do |platform|
    [platform, MODES.to_h { |mode| [mode, false_compile_evidence(mode)] }]
  end
end

def validate_source!(path, candidate, tree)
  receipt = parse_json(path)
  raise QualificationError, "source status=#{receipt['status']}" unless receipt["status"] == SOURCE_PASS
  reject_true_keys!(receipt, "source receipt")
  if receipt["candidate"].is_a?(Hash)
    raise QualificationError, "source commit drift" unless receipt.dig("candidate", "commit") == candidate
    raise QualificationError, "source tree drift" unless receipt.dig("candidate", "tree") == tree
  end
  {
    "qualified" => true,
    "status" => SOURCE_PASS,
    "digest" => digest_file(path)
  }
end

def compile_receipt_paths(root)
  Pathname.new(root).glob("**/*.json").select(&:file?)
end

def validate_compile_receipts!(root, candidate, tree)
  receipts = {}
  compile_receipt_paths(root).each do |path|
    payload = parse_json(path)
    next unless payload["schema"] == "hepta.ui.v4.windows-product-host-candidate-compile.v1"

    platform = payload["platform"]
    mode = payload["mode"]
    key = [platform, mode]
    raise QualificationError, "duplicate compile receipt #{key.join('/')}" if receipts.key?(key)
    receipts[key] = [path, payload]
  end

  evidence = empty_compile_matrix
  PLATFORMS.each do |platform|
    MODES.each do |mode|
      match = receipts[[platform, mode]]
      raise QualificationError, "compile receipt missing #{platform}/#{mode}" unless match
      path, payload = match
      raise QualificationError, "compile status #{platform}/#{mode}" unless payload["status"] == COMPILE_PASS
      raise QualificationError, "compile commit drift #{platform}/#{mode}" unless payload.dig("candidate", "commit") == candidate
      raise QualificationError, "compile tree drift #{platform}/#{mode}" unless payload.dig("candidate", "tree") == tree
      raise QualificationError, "compile feature name drift #{platform}/#{mode}" unless payload.dig("feature", "name") == FEATURE
      expected_enabled = mode == "explicit-feature"
      unless payload.dig("feature", "enabled") == expected_enabled
        raise QualificationError, "compile feature state drift #{platform}/#{mode}"
      end
      %w[fmt check tests].each do |gate|
        raise QualificationError, "compile #{gate} missing #{platform}/#{mode}" unless payload.dig("qualification", gate) == true
      end
      false_wiring.each_key do |key|
        raise QualificationError, "compile #{key} escaped #{platform}/#{mode}" unless payload.dig("qualification", key) == false
      end
      require_false_hash!(payload.fetch("authority"), "compile authority #{platform}/#{mode}")
      evidence[platform][mode] = {
        "qualified" => true,
        "status" => COMPILE_PASS,
        "featureEnabled" => expected_enabled,
        "digest" => digest_file(path)
      }
    end
  end
  unexpected = receipts.keys - PLATFORMS.product(MODES)
  raise QualificationError, "unexpected compile receipts: #{unexpected.inspect}" unless unexpected.empty?
  evidence
end
