  COMPILE_JOBS.each_key { |os| Pathname.new(root).join("hepta-ui-v4-exact-compile-#{os}-#{candidate}").mkpath }

  return unless with_runtime
  runtime_dir = Pathname.new(root).join("hepta-ui-v4-exact-windows-profile-#{candidate}")
  runtime_dir.mkpath
  root_identity = { "index" => 1, "generation" => 7, "nativeHandle" => "11" }
  transient_identity = { "index" => 2, "generation" => 9, "nativeHandle" => "12" }
  write_fixture_json(runtime_dir.join("receipt.json"), {
    "status" => "PASS_WINDOWS_MATERIAL_PROFILE_AGGREGATE",
    "candidate" => { "commit" => candidate, "tree" => tree },
    "makepad" => { "revision" => MAKEPAD_REVISION, "vendoredPatchBuild" => true, "defaultDependencySwitched" => false },
    "fixture" => true,
    "root" => {
      "identity" => root_identity, "requestSequence" => 1, "backend" => "WindowsDwm",
      "status" => "VerifiedPersistentChromeWithBackdropReadback", "readbackScope" => "BackdropOnly",
      "requestedBackdrop" => "Mica", "observedBackdrop" => "Mica", "backdropExact" => true
    },
    "transient" => {
      "parent" => root_identity, "identity" => transient_identity, "separateFromRoot" => true,
      "acrylic" => { "status" => "VerifiedAcrylicWithBackdropReadback", "requestSequence" => 2, "requestedBackdrop" => "Acrylic", "observedBackdrop" => "Acrylic", "backdropExact" => true },
      "solidRollback" => { "status" => "VerifiedSolidRollbackWithBackdropReadback", "requestSequence" => 3, "requestedBackdrop" => "None", "observedBackdrop" => "None", "backdropExact" => true },
      "destroyedIdentity" => transient_identity, "destroyedAcknowledged" => true
    },
    "qualification" => {
      "rootRuntimeReceipt" => true, "transientRuntimeReceipt" => true,
      "dualReceiptAggregateRuntime" => true, "eligibleForProductIntegrationReview" => true,
      "productBound" => false, "transientSystemMaterialBound" => false,
      "completeProfileBound" => false, "systemMaterialBound" => false,
      "nativeProductRuntime" => false, "deviceValidation" => false
    },
    "authority" => false_hash
  })
end

def run_self_test
  candidate = "a" * 40
  tree = "b" * 40
  results = {}
  Dir.mktmpdir("hepta-ui-index-self-test") do |dir|
    artifacts = Pathname.new(dir).join("artifacts")
    artifacts.mkpath
    build_self_test_artifacts(artifacts, candidate, tree, with_runtime: false)
    jobs = [
      fake_job(1, SOURCE_JOB, candidate), fake_job(2, PATCH_JOB, candidate),
      *COMPILE_JOBS.values.each_with_index.map { |name, index| fake_job(10 + index, name, candidate) },
      fake_job(20, RUNTIME_JOB, candidate, conclusion: "skipped")
    ]
    jobs_path = Pathname.new(dir).join("jobs.json")
    write_fixture_json(jobs_path, { "jobs" => jobs })
    options = Options.new(jobs: jobs_path, artifacts: artifacts, candidate_commit: candidate, candidate_tree: tree, repository: "owner/repo", run_id: 1, require_windows_runtime: false, allow_in_progress_run: true)
    pre = build_index(options)
    raise QualificationError, "pre-runtime self-test failed" unless pre["status"] == "PASS_EXACT_HEAD_PRE_RUNTIME_QUALIFICATION"
    results["preRuntime"] = true

    FileUtils.rm_rf(artifacts)
    artifacts.mkpath
    build_self_test_artifacts(artifacts, candidate, tree, with_runtime: true)
    jobs[-1] = fake_job(20, RUNTIME_JOB, candidate)
    write_fixture_json(jobs_path, { "jobs" => jobs })
    options.require_windows_runtime = true
    runtime = build_index(options)
    raise QualificationError, "runtime self-test failed" unless runtime["status"] == "PASS_EXACT_HEAD_WINDOWS_RUNTIME_QUALIFICATION"
    results["windowsRuntime"] = true

    jobs[3]["conclusion"] = "failure"
    jobs[3]["steps"][0]["conclusion"] = "failure"
    write_fixture_json(jobs_path, { "jobs" => jobs })
    options.require_windows_runtime = false
    failed = build_index(options)
    raise QualificationError, "failure self-test did not fail" unless failed["status"] == "FAIL_EXACT_HEAD_QUALIFICATION_INDEX" && !failed["failures"].empty?
    results["failClosed"] = true
  end
  puts JSON.pretty_generate(
    "schema" => "hepta.ui.v4.exact-head-qualification-index-self-test.v1",
    "status" => "PASS_EXACT_HEAD_QUALIFICATION_INDEX_SELF_TEST",
    "tests" => results,
    "authority" => false_hash
  )
end

options = Options.new(require_windows_runtime: false, allow_in_progress_run: false, self_test: false)
parser = OptionParser.new do |opts|
  opts.banner = "usage: #{$PROGRAM_NAME} --jobs JOBS_JSON --artifacts DIR --output INDEX_JSON [options]"
  opts.on("--jobs PATH") { |value| options.jobs = value }
  opts.on("--artifacts PATH") { |value| options.artifacts = value }
  opts.on("--candidate-commit SHA") { |value| options.candidate_commit = value }
  opts.on("--candidate-tree SHA") { |value| options.candidate_tree = value }
  opts.on("--repository NAME") { |value| options.repository = value }
  opts.on("--run-id ID", Integer) { |value| options.run_id = value }
  opts.on("--output PATH") { |value| options.output = value }
  opts.on("--require-windows-runtime") { options.require_windows_runtime = true }
  opts.on("--allow-in-progress-run") { options.allow_in_progress_run = true }
  opts.on("--self-test") { options.self_test = true }
end
parser.parse!

if options.self_test
  run_self_test
  exit 0
end

%i[jobs artifacts output].each do |name|
  abort "missing --#{name.to_s.tr('_', '-')}" if options[name].nil?
end

begin
  payload = build_index(options)
  write_json_atomic(options.output, payload)
  puts JSON.pretty_generate(payload)
  exit(payload["status"].start_with?("PASS_") ? 0 : 1)
rescue QualificationError => error
  candidate = options.candidate_commit.to_s
  tree = options.candidate_tree.to_s
  payload = {
    "schema" => SCHEMA,
    "status" => "FAIL_EXACT_HEAD_QUALIFICATION_INDEX",
    "candidate" => { "commit" => candidate, "tree" => tree },
    "sourceRun" => { "repository" => options.repository.to_s, "runId" => options.run_id.to_i, "allowInProgressRun" => options.allow_in_progress_run == true },
    "evidence" => {},
    "qualification" => {
      "source" => false, "materialization" => false,
      "compileUbuntu" => false, "compileWindows" => false, "compileMacos" => false,
      "preRuntime" => false, "windowsRuntime" => false,
      "eligibleForProductIntegrationReview" => false,
      "productBound" => false, "transientSystemMaterialBound" => false,
      "completeProfileBound" => false, "systemMaterialBound" => false,
      "nativeProductRuntime" => false, "deviceValidation" => false
    },
    "authority" => false_hash,
    "failures" => [error.message]
  }
  write_json_atomic(options.output, payload)
  warn JSON.pretty_generate(payload)
  exit 1
end
