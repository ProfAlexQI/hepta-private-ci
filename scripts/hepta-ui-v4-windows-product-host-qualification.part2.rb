# frozen_string_literal: true

def review_binding_digest(receipt)
  candidates = [
    receipt["bindingDigest"],
    receipt.dig("evidence", "bindingDigest"),
    receipt.dig("binding", "digest")
  ].compact
  value = candidates.find { |entry| sha256?(entry) }
  raise QualificationError, "review binding digest missing" unless value
  value
end

def require_false_if_present!(hash, key, label)
  return unless hash.key?(key)
  raise QualificationError, "#{label} #{key} escaped" unless hash[key] == false
end

def validate_review_envelope(path)
  return {
    "present" => false,
    "qualified" => false,
    "status" => nil,
    "digest" => nil,
    "bindingDigest" => nil,
    "candidate" => nil
  } if path.nil? || path.to_s.empty?

  receipt = parse_json(path)
  status = receipt["status"]
  reject_true_keys!(receipt, "review envelope")
  if status == REVIEW_BLOCKED
    return {
      "present" => true,
      "qualified" => false,
      "status" => status,
      "digest" => digest_file(path),
      "bindingDigest" => nil,
      "candidate" => receipt["candidate"]
    }
  end
  raise QualificationError, "review envelope status=#{status}" unless status == REVIEW_PASS
  qualification = receipt.fetch("qualification")
  raise QualificationError, "review envelope not eligible" unless qualification["implementationReviewEligible"] == true
  %w[
    implementationApproved productHostMayBind productBound
    transientSystemMaterialBound completeProfileBound systemMaterialBound
    nativeProductRuntime deviceValidation
  ].each do |key|
    raise QualificationError, "review envelope #{key} missing" unless qualification.key?(key)
    require_false_if_present!(qualification, key, "review envelope")
  end
  require_false_hash!(receipt.fetch("authority"), "review envelope authority")
  candidate = receipt.fetch("candidate")
  raise QualificationError, "review candidate commit invalid" unless git_object_id?(candidate["commit"])
  raise QualificationError, "review candidate tree invalid" unless git_object_id?(candidate["tree"])
  makepad_revision = receipt.dig("makepad", "revision") || receipt.dig("evidence", "makepadRevision")
  if makepad_revision
    raise QualificationError, "review Makepad revision drift" unless makepad_revision == MAKEPAD_REVISION
  end
  {
    "present" => true,
    "qualified" => true,
    "status" => REVIEW_PASS,
    "digest" => digest_file(path),
    "bindingDigest" => review_binding_digest(receipt),
    "candidate" => candidate
  }
end

def positive_handle?(value)
  value.is_a?(String) && value.match?(/\A[1-9][0-9]*\z/)
end

def valid_window_identity?(value)
  value.is_a?(Hash) &&
    value["index"].is_a?(Integer) && value["index"] >= 0 &&
    value["generation"].is_a?(Integer) && value["generation"] >= 0 &&
    positive_handle?(value["nativeHandle"])
end

def same_window_identity?(left, right)
  %w[index generation nativeHandle].all? { |key| left[key] == right[key] }
end

def validate_device_receipt(path, candidate, tree, review)
  return {
    "present" => false,
    "qualified" => false,
    "status" => nil,
    "digest" => nil
  } if path.nil? || path.to_s.empty?

  raise QualificationError, "device receipt supplied without qualified review envelope" unless review["qualified"]
  receipt = parse_json(path)
  raise QualificationError, "device status=#{receipt['status']}" unless receipt["status"] == DEVICE_PASS
  raise QualificationError, "device commit drift" unless receipt.dig("candidate", "commit") == candidate
  raise QualificationError, "device tree drift" unless receipt.dig("candidate", "tree") == tree
  raise QualificationError, "device review binding drift" unless receipt["reviewBindingDigest"] == review["bindingDigest"]
  raise QualificationError, "device feature name drift" unless receipt.dig("feature", "name") == FEATURE
  raise QualificationError, "device feature was not enabled" unless receipt.dig("feature", "enabled") == true
  raise QualificationError, "device is not physical" unless receipt.dig("device", "physical") == true
  raise QualificationError, "device OS is not Windows" unless receipt.dig("device", "os") == "Windows"
  labels = receipt.dig("device", "runnerLabels")
  required_labels = %w[self-hosted Windows X64 hepta-ui-dwm]
  raise QualificationError, "device runner labels missing" unless labels.is_a?(Array) && (required_labels - labels).empty?

  root = receipt.dig("identity", "root")
  transient = receipt.dig("identity", "transient")
  raise QualificationError, "device root identity invalid" unless valid_window_identity?(root)
  raise QualificationError, "device transient identity invalid" unless valid_window_identity?(transient)
  raise QualificationError, "device root/transient identity reused" if same_window_identity?(root, transient)
  raise QualificationError, "device root/transient HWND reused" if root["nativeHandle"] == transient["nativeHandle"]

  required_checks = %w[
    activationObserved rootMicaExact transientAcrylicExact explicitRollback
    rootNoneExact transientNoneExact highContrastFallback
    transparencyDisabledFallback suspendRollback shutdownRollback
    rollbackDrillValidated physicalDeviceValidated
  ]
  required_checks.each do |key|
    raise QualificationError, "device check #{key} missing" unless receipt.dig("checks", key) == true
  end
  raise QualificationError, "device final state is not Unbound" unless receipt.dig("checks", "finalState") == "Unbound"
  %w[isolatedCandidate physicalDeviceValidated rollbackDrillValidated].each do |key|
    raise QualificationError, "device qualification #{key} missing" unless receipt.dig("qualification", key) == true
  end
  false_wiring.each_key do |key|
    raise QualificationError, "device qualification #{key} missing" unless receipt.fetch("qualification").key?(key)
    raise QualificationError, "device #{key} escaped" unless receipt.dig("qualification", key) == false
  end
  require_false_hash!(receipt.fetch("authority"), "device authority")
  {
    "present" => true,
    "qualified" => true,
    "status" => DEVICE_PASS,
    "digest" => digest_file(path)
  }
end

def empty_evidence
  {
    "source" => { "qualified" => false, "status" => nil, "digest" => nil },
    "compile" => empty_compile_matrix,
    "reviewEnvelope" => {
      "present" => false, "qualified" => false, "status" => nil,
      "digest" => nil, "bindingDigest" => nil, "candidate" => nil
    },
    "device" => { "present" => false, "qualified" => false, "status" => nil, "digest" => nil }
  }
end

def compile_lane_qualified?(evidence, mode)
  PLATFORMS.all? { |platform| evidence.dig("compile", platform, mode, "qualified") == true }
end

def qualification_payload(status, candidate, tree, evidence, failures)
  source_ok = evidence.dig("source", "qualified") == true
  default_ok = compile_lane_qualified?(evidence, "default-off")
  explicit_ok = compile_lane_qualified?(evidence, "explicit-feature")
  compile_ok = default_ok && explicit_ok
  review_ok = evidence.dig("reviewEnvelope", "qualified") == true
  device_ok = evidence.dig("device", "qualified") == true
  final_ok = status == "PASS_WINDOWS_PRODUCT_HOST_IMPLEMENTATION_QUALIFICATION"
  {
    "schema" => SCHEMA,
    "status" => status,
    "candidate" => { "commit" => candidate, "tree" => tree },
    "feature" => { "name" => FEATURE, "defaultEnabled" => false },
    "evidence" => evidence,
    "qualification" => {
      "source" => source_ok,
      "defaultOffCompile" => default_ok,
      "explicitFeatureCompile" => explicit_ok,
      "compileMatrix" => compile_ok,
      "reviewEnvelope" => review_ok,
      "rollbackDrill" => device_ok,
      "physicalDevice" => device_ok,
      "implementationCandidateQualified" => final_ok,
      "eligibleForProductWiringReview" => final_ok
    }.merge(false_wiring),
    "authority" => false_authority,
    "failures" => failures.map { |entry| bounded_failure(entry) }.uniq.first(MAX_FAILURES)
  }
end

def build_qualification(options)
  candidate = options.candidate_commit.to_s
  tree = options.candidate_tree.to_s
  raise QualificationError, "candidate commit invalid" unless git_object_id?(candidate)
  raise QualificationError, "candidate tree invalid" unless git_object_id?(tree)

  evidence = empty_evidence
  failures = []
  begin
    evidence["source"] = validate_source!(options.source_receipt, candidate, tree)
  rescue QualificationError => error
    failures << error.message
  end
  begin
    evidence["compile"] = validate_compile_receipts!(options.compile_receipts, candidate, tree)
  rescue QualificationError => error
    failures << error.message
  end
  begin
    evidence["reviewEnvelope"] = validate_review_envelope(options.review_envelope)
  rescue QualificationError => error
    failures << error.message
  end
  begin
    evidence["device"] = validate_device_receipt(
      options.device_receipt,
      candidate,
      tree,
      evidence["reviewEnvelope"]
    )
  rescue QualificationError => error
    failures << error.message
  end

  unless failures.empty?
    return qualification_payload(
      "FAIL_WINDOWS_PRODUCT_HOST_IMPLEMENTATION_QUALIFICATION",
      candidate,
      tree,
      evidence,
      failures
    )
  end
  unless evidence.dig("reviewEnvelope", "qualified") == true
    return qualification_payload(
      "BLOCKED_WINDOWS_PRODUCT_HOST_REVIEW_ENVELOPE_REQUIRED",
      candidate,
      tree,
      evidence,
      []
    )
  end
  unless evidence.dig("device", "qualified") == true
    return qualification_payload(
      "BLOCKED_WINDOWS_PRODUCT_HOST_DEVICE_QUALIFICATION_REQUIRED",
      candidate,
      tree,
      evidence,
      []
    )
  end
  qualification_payload(
    "PASS_WINDOWS_PRODUCT_HOST_IMPLEMENTATION_QUALIFICATION",
    candidate,
    tree,
    evidence,
    []
  )
end
