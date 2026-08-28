  receipt_path = directory.join("materialization.json")
  raise QualificationError, "materialization receipt missing" unless receipt_path.file?
  receipt = parse_json(receipt_path)
  raise QualificationError, "materialization status=#{receipt['status']}" unless receipt["status"] == "PASS_PATCHED_CHECKOUT_MATERIALIZED_SOURCE_ONLY"
  raise QualificationError, "materialization commit drift" unless receipt.dig("candidate", "commit") == candidate
  raise QualificationError, "materialization tree drift" unless receipt.dig("candidate", "tree") == tree
  raise QualificationError, "materialization candidate not bound" unless receipt.dig("candidate", "bound") == true
  raise QualificationError, "Makepad revision drift" unless receipt.dig("makepad", "revision") == MAKEPAD_REVISION
  changed = receipt.dig("makepad", "changedFiles")
  raise QualificationError, "Makepad changed-file set drift" unless changed.is_a?(Array) && changed.sort == EXPECTED_MAKEPAD_FILES
  raise QualificationError, "default dependency changed" unless receipt.dig("cargoOverride", "defaultDependencyChanged") == false
  raise QualificationError, "patched checkout not materialized" unless receipt.dig("qualification", "patchedCheckoutMaterialized") == true
  raise QualificationError, "patch not applied" unless receipt.dig("qualification", "patchAppliedInWorktree") == true
  %w[patchedMakepadCompile heptaHookCompile windowsRuntime backdropReadback transientSystemMaterial completeProfile systemMaterialBinding nativeProductRuntime deviceValidation].each do |key|
    raise QualificationError, "materialization #{key} escaped" unless receipt.dig("qualification", key) == false
  end
  require_false_hash!(receipt.fetch("authority"), "materialization authority")
  { "qualified" => true, "status" => receipt["status"], "artifact" => directory.basename.to_s }
end

def positive_handle?(value)
  value.is_a?(String) && value.match?(/\A[1-9][0-9]*\z/)
end

def same_identity?(left, right)
  %w[index generation nativeHandle].all? { |key| left[key] == right[key] }
end

def validate_runtime_receipt!(root, candidate, tree)
  directory = artifact_dir!(root, "hepta-ui-v4-exact-windows-profile-#{candidate}")
  receipt_path = directory.join("receipt.json")
  raise QualificationError, "Windows runtime receipt missing" unless receipt_path.file?
  receipt = parse_json(receipt_path)
  raise QualificationError, "runtime status=#{receipt['status']}" unless receipt["status"] == "PASS_WINDOWS_MATERIAL_PROFILE_AGGREGATE"
  raise QualificationError, "runtime commit drift" unless receipt.dig("candidate", "commit") == candidate
  raise QualificationError, "runtime tree drift" unless receipt.dig("candidate", "tree") == tree
  raise QualificationError, "runtime Makepad drift" unless receipt.dig("makepad", "revision") == MAKEPAD_REVISION
  raise QualificationError, "vendored patch runtime not proven" unless receipt.dig("makepad", "vendoredPatchBuild") == true
  raise QualificationError, "default dependency switched" unless receipt.dig("makepad", "defaultDependencySwitched") == false
  raise QualificationError, "runtime is not fixture-bound" unless receipt["fixture"] == true

  root_receipt = receipt.fetch("root")
  root_identity = root_receipt.fetch("identity")
  raise QualificationError, "root HWND invalid" unless positive_handle?(root_identity["nativeHandle"])
  raise QualificationError, "root backend" unless root_receipt["backend"] == "WindowsDwm"
  raise QualificationError, "root status" unless root_receipt["status"] == "VerifiedPersistentChromeWithBackdropReadback"
  raise QualificationError, "root readback scope" unless root_receipt["readbackScope"] == "BackdropOnly"
  raise QualificationError, "root Mica mismatch" unless root_receipt["requestedBackdrop"] == "Mica" && root_receipt["observedBackdrop"] == "Mica" && root_receipt["backdropExact"] == true

  transient = receipt.fetch("transient")
  parent = transient.fetch("parent")
  identity = transient.fetch("identity")
  destroyed = transient.fetch("destroyedIdentity")
  raise QualificationError, "transient parent drift" unless same_identity?(root_identity, parent)
  raise QualificationError, "transient HWND invalid" unless positive_handle?(identity["nativeHandle"])
  raise QualificationError, "root/transient HWND reused" if identity["nativeHandle"] == root_identity["nativeHandle"]
  raise QualificationError, "separateFromRoot missing" unless transient["separateFromRoot"] == true
  raise QualificationError, "Destroyed identity drift" unless same_identity?(identity, destroyed) && transient["destroyedAcknowledged"] == true

  acrylic = transient.fetch("acrylic")
  solid = transient.fetch("solidRollback")
  raise QualificationError, "Acrylic receipt mismatch" unless acrylic["status"] == "VerifiedAcrylicWithBackdropReadback" && acrylic["requestedBackdrop"] == "Acrylic" && acrylic["observedBackdrop"] == "Acrylic" && acrylic["backdropExact"] == true
  raise QualificationError, "solid receipt mismatch" unless solid["status"] == "VerifiedSolidRollbackWithBackdropReadback" && solid["requestedBackdrop"] == "None" && solid["observedBackdrop"] == "None" && solid["backdropExact"] == true
  root_sequence = root_receipt["requestSequence"]
  acrylic_sequence = acrylic["requestSequence"]
  solid_sequence = solid["requestSequence"]
  if [root_sequence, acrylic_sequence, solid_sequence].all? { |value| value.is_a?(Integer) }
    raise QualificationError, "runtime sequence order drift" unless root_sequence < acrylic_sequence && acrylic_sequence < solid_sequence
  end

  qualification = receipt.fetch("qualification")
  %w[rootRuntimeReceipt transientRuntimeReceipt dualReceiptAggregateRuntime eligibleForProductIntegrationReview].each do |key|
    raise QualificationError, "runtime #{key} missing" unless qualification[key] == true
  end
  %w[productBound transientSystemMaterialBound completeProfileBound systemMaterialBound nativeProductRuntime deviceValidation].each do |key|
    raise QualificationError, "runtime #{key} escaped" unless qualification[key] == false
  end
  require_false_hash!(receipt.fetch("authority"), "runtime authority")
  { "qualified" => true, "status" => receipt["status"], "artifact" => directory.basename.to_s }
end

def skipped_runtime?(job)
  job["status"] == "completed" && job["conclusion"] == "skipped"
end

def build_index(options)
  failures = []
  evidence = {
    "source" => { "qualified" => false },
    "materialization" => { "qualified" => false },
    "compile" => {},
    "windowsRuntime" => { "requested" => options.require_windows_runtime, "qualified" => false }
  }
  candidate = options.candidate_commit || read_exact_text!(options.artifacts, "candidate-commit.txt")
  tree = options.candidate_tree || read_exact_text!(options.artifacts, "candidate-tree.txt")
  raise QualificationError, "candidate commit invalid" unless git_object_id?(candidate)
  raise QualificationError, "candidate tree invalid" unless git_object_id?(tree)
  jobs = jobs_from(parse_json(options.jobs))

  begin
