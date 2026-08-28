# frozen_string_literal: true

def write_fixture(path, payload)
  destination = Pathname.new(path)
  destination.dirname.mkpath
  destination.write(JSON.pretty_generate(payload) + "\n")
end

def source_fixture(candidate, tree, authority: false_authority)
  {
    "status" => SOURCE_PASS,
    "candidate" => { "commit" => candidate, "tree" => tree },
    "product_cargo_feature_declared" => false,
    "product_module_registered" => false,
    "product_lifecycle_wired" => false,
    "automatic_binding_allowed" => false,
    "product_host_may_bind" => false,
    "system_material_binding" => false,
    "authority" => authority
  }
end

def compile_fixture(candidate, tree, platform, mode)
  {
    "schema" => "hepta.ui.v4.windows-product-host-candidate-compile.v1",
    "status" => COMPILE_PASS,
    "candidate" => { "commit" => candidate, "tree" => tree },
    "platform" => platform,
    "mode" => mode,
    "feature" => { "name" => FEATURE, "enabled" => mode == "explicit-feature" },
    "qualification" => {
      "fmt" => true,
      "check" => true,
      "tests" => true
    }.merge(false_wiring),
    "authority" => false_authority
  }
end

def review_fixture(candidate, tree)
  {
    "schema" => "hepta.ui.v4.windows-product-host-integration-review-envelope.v1",
    "status" => REVIEW_PASS,
    "candidate" => { "commit" => candidate, "tree" => tree },
    "makepad" => { "revision" => MAKEPAD_REVISION },
    "bindingDigest" => "c" * 64,
    "qualification" => {
      "implementationReviewEligible" => true
    }.merge(false_wiring),
    "authority" => false_authority,
    "failures" => []
  }
end

def device_fixture(candidate, tree)
  {
    "schema" => "hepta.ui.v4.windows-product-host-device-drill.v1",
    "status" => DEVICE_PASS,
    "candidate" => { "commit" => candidate, "tree" => tree },
    "reviewBindingDigest" => "c" * 64,
    "feature" => { "name" => FEATURE, "enabled" => true },
    "device" => {
      "physical" => true,
      "os" => "Windows",
      "architecture" => "X64",
      "runnerLabels" => %w[self-hosted Windows X64 hepta-ui-dwm]
    },
    "identity" => {
      "root" => { "index" => 1, "generation" => 7, "nativeHandle" => "11" },
      "transient" => { "index" => 2, "generation" => 9, "nativeHandle" => "12" }
    },
    "checks" => {
      "activationObserved" => true,
      "rootMicaExact" => true,
      "transientAcrylicExact" => true,
      "explicitRollback" => true,
      "rootNoneExact" => true,
      "transientNoneExact" => true,
      "highContrastFallback" => true,
      "transparencyDisabledFallback" => true,
      "suspendRollback" => true,
      "shutdownRollback" => true,
      "rollbackDrillValidated" => true,
      "physicalDeviceValidated" => true,
      "finalState" => "Unbound"
    },
    "qualification" => {
      "isolatedCandidate" => true,
      "physicalDeviceValidated" => true,
      "rollbackDrillValidated" => true
    }.merge(false_wiring),
    "authority" => false_authority
  }
end

def build_self_test_inputs(root, candidate, tree)
  source_path = root.join("source.json")
  compile_root = root.join("compile")
  review_path = root.join("review.json")
  device_path = root.join("device.json")
  write_fixture(source_path, source_fixture(candidate, tree))
  PLATFORMS.each do |platform|
    MODES.each do |mode|
      write_fixture(
        compile_root.join("#{platform}-#{mode}.json"),
        compile_fixture(candidate, tree, platform, mode)
      )
    end
  end
  write_fixture(review_path, review_fixture("d" * 40, "e" * 40))
  write_fixture(device_path, device_fixture(candidate, tree))
  [source_path, compile_root, review_path, device_path]
end

def run_self_test
  candidate = "a" * 40
  tree = "b" * 40
  results = {}
  Dir.mktmpdir("hepta-product-host-qualification") do |dir|
    root = Pathname.new(dir)
    source_path, compile_root, review_path, device_path = build_self_test_inputs(root, candidate, tree)
    options = Options.new(
      candidate_commit: candidate,
      candidate_tree: tree,
      source_receipt: source_path,
      compile_receipts: compile_root
    )

    blocked_review = build_qualification(options)
    unless blocked_review["status"] == "BLOCKED_WINDOWS_PRODUCT_HOST_REVIEW_ENVELOPE_REQUIRED"
      raise QualificationError, "review-blocked self-test failed"
    end
    results["reviewBlocked"] = true

    options.review_envelope = review_path
    blocked_device = build_qualification(options)
    unless blocked_device["status"] == "BLOCKED_WINDOWS_PRODUCT_HOST_DEVICE_QUALIFICATION_REQUIRED"
      raise QualificationError, "device-blocked self-test failed"
    end
    results["deviceBlocked"] = true

    options.device_receipt = device_path
    passed = build_qualification(options)
    unless passed["status"] == "PASS_WINDOWS_PRODUCT_HOST_IMPLEMENTATION_QUALIFICATION"
      raise QualificationError, "qualification PASS self-test failed"
    end
    unless passed.dig("qualification", "productHostMayBind") == false && passed.fetch("authority").values.none?
      raise QualificationError, "PASS self-test escaped authority"
    end
    results["qualified"] = true

    drift_path = compile_root.join("windows-latest-explicit-feature.json")
    drift = parse_json(drift_path)
    drift["candidate"]["commit"] = "f" * 40
    write_fixture(drift_path, drift)
    failed = build_qualification(options)
    unless failed["status"] == "FAIL_WINDOWS_PRODUCT_HOST_IMPLEMENTATION_QUALIFICATION"
      raise QualificationError, "candidate-drift self-test did not fail"
    end
    results["candidateDriftRejected"] = true

    write_fixture(drift_path, compile_fixture(candidate, tree, "windows-latest", "explicit-feature"))
    escaped_review = review_fixture("d" * 40, "e" * 40)
    escaped_review["authority"]["production"] = true
    write_fixture(review_path, escaped_review)
    failed = build_qualification(options)
    unless failed["status"] == "FAIL_WINDOWS_PRODUCT_HOST_IMPLEMENTATION_QUALIFICATION"
      raise QualificationError, "authority self-test did not fail"
    end
    results["authorityEscapeRejected"] = true
  end
  puts JSON.pretty_generate(
    "schema" => "hepta.ui.v4.windows-product-host-qualification-self-test.v1",
    "status" => "PASS_WINDOWS_PRODUCT_HOST_QUALIFICATION_SELF_TEST",
    "tests" => results,
    "authority" => false_authority
  )
end

options = Options.new(self_test: false)
parser = OptionParser.new do |opts|
  opts.banner = "usage: #{$PROGRAM_NAME} --candidate-commit SHA --candidate-tree TREE --source-receipt FILE --compile-receipts DIR --output FILE [options]"
  opts.on("--candidate-commit SHA") { |value| options.candidate_commit = value }
  opts.on("--candidate-tree SHA") { |value| options.candidate_tree = value }
  opts.on("--source-receipt PATH") { |value| options.source_receipt = value }
  opts.on("--compile-receipts DIR") { |value| options.compile_receipts = value }
  opts.on("--review-envelope PATH") { |value| options.review_envelope = value }
  opts.on("--device-receipt PATH") { |value| options.device_receipt = value }
  opts.on("--output PATH") { |value| options.output = value }
  opts.on("--self-test") { options.self_test = true }
end
parser.parse!

if options.self_test
  run_self_test
  exit 0
end

%i[candidate_commit candidate_tree source_receipt compile_receipts output].each do |name|
  abort "missing --#{name.to_s.tr('_', '-')}" if options[name].nil?
end

begin
  payload = build_qualification(options)
  write_json_atomic(options.output, payload)
  puts JSON.pretty_generate(payload)
  exit(payload["status"].start_with?("FAIL_") ? 1 : 0)
rescue QualificationError => error
  payload = qualification_payload(
    "FAIL_WINDOWS_PRODUCT_HOST_IMPLEMENTATION_QUALIFICATION",
    options.candidate_commit.to_s,
    options.candidate_tree.to_s,
    empty_evidence,
    [error.message]
  )
  write_json_atomic(options.output, payload)
  warn JSON.pretty_generate(payload)
  exit 1
end
