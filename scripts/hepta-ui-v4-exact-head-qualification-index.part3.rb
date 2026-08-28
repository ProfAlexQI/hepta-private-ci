    evidence["source"] = validate_source_artifact!(options.artifacts, candidate, tree)
    evidence["source"]["job"] = validate_executed_job!(job_by_name!(jobs, SOURCE_JOB), candidate, SOURCE_JOB)
  rescue QualificationError => error
    failures << error.message
  end

  begin
    evidence["materialization"] = validate_materialization!(options.artifacts, candidate, tree)
    evidence["materialization"]["job"] = validate_executed_job!(job_by_name!(jobs, PATCH_JOB), candidate, PATCH_JOB)
  rescue QualificationError => error
    failures << error.message
  end

  COMPILE_JOBS.each do |os, name|
    begin
      directory = artifact_dir!(options.artifacts, "hepta-ui-v4-exact-compile-#{os}-#{candidate}")
      evidence["compile"][os] = {
        "qualified" => true,
        "artifact" => directory.basename.to_s,
        "job" => validate_executed_job!(job_by_name!(jobs, name), candidate, name)
      }
    rescue QualificationError => error
      evidence["compile"][os] = { "qualified" => false }
      failures << error.message
    end
  end

  runtime_job_matches = jobs.select { |job| job["name"] == RUNTIME_JOB }
  if runtime_job_matches.length > 1
    failures << "runtime job count=#{runtime_job_matches.length}"
  elsif runtime_job_matches.empty?
    failures << "runtime job missing" if options.require_windows_runtime
    evidence["windowsRuntime"]["requested"] = false unless options.require_windows_runtime
  else
    runtime_job = runtime_job_matches.first
    if skipped_runtime?(runtime_job)
      failures << "Windows runtime required but job was skipped" if options.require_windows_runtime
      evidence["windowsRuntime"]["requested"] = false unless options.require_windows_runtime
    elsif runtime_job["status"] == "completed" && runtime_job["conclusion"] == "success"
      begin
        runtime_evidence = validate_runtime_receipt!(options.artifacts, candidate, tree)
        runtime_evidence["requested"] = true
        runtime_evidence["job"] = validate_executed_job!(runtime_job, candidate, RUNTIME_JOB)
        evidence["windowsRuntime"] = runtime_evidence
      rescue QualificationError => error
        failures << error.message
      end
    else
      failures << "runtime job status=#{runtime_job['status']} conclusion=#{runtime_job['conclusion']}"
    end
  end

  source_ok = evidence.dig("source", "qualified") == true
  materialization_ok = evidence.dig("materialization", "qualified") == true
  compile_ok = COMPILE_JOBS.keys.all? { |os| evidence.dig("compile", os, "qualified") == true }
  pre_runtime = failures.empty? && source_ok && materialization_ok && compile_ok
  runtime_ok = pre_runtime && evidence.dig("windowsRuntime", "qualified") == true

  status = if failures.any?
             "FAIL_EXACT_HEAD_QUALIFICATION_INDEX"
           elsif runtime_ok
             "PASS_EXACT_HEAD_WINDOWS_RUNTIME_QUALIFICATION"
           else
             "PASS_EXACT_HEAD_PRE_RUNTIME_QUALIFICATION"
           end
  if options.require_windows_runtime && !runtime_ok
    status = "FAIL_EXACT_HEAD_QUALIFICATION_INDEX"
    failures << "Windows runtime qualification is required" unless failures.include?("Windows runtime qualification is required")
  end

  {
    "schema" => SCHEMA,
    "status" => status,
    "candidate" => { "commit" => candidate, "tree" => tree },
    "sourceRun" => {
      "repository" => options.repository.to_s,
      "runId" => options.run_id.to_i,
      "allowInProgressRun" => options.allow_in_progress_run == true
    },
    "evidence" => evidence,
    "qualification" => {
      "source" => source_ok,
      "materialization" => materialization_ok,
      "compileUbuntu" => evidence.dig("compile", "ubuntu-latest", "qualified") == true,
      "compileWindows" => evidence.dig("compile", "windows-latest", "qualified") == true,
      "compileMacos" => evidence.dig("compile", "macos-latest", "qualified") == true,
      "preRuntime" => pre_runtime,
      "windowsRuntime" => runtime_ok,
      "eligibleForProductIntegrationReview" => runtime_ok,
      "productBound" => false,
      "transientSystemMaterialBound" => false,
      "completeProfileBound" => false,
      "systemMaterialBound" => false,
      "nativeProductRuntime" => false,
      "deviceValidation" => false
    },
    "authority" => false_hash,
    "failures" => failures.uniq.first(64)
  }
end

def write_fixture_json(path, payload)
  Pathname.new(path).dirname.mkpath
  Pathname.new(path).write(JSON.pretty_generate(payload) + "\n")
end

def fake_job(id, name, candidate, conclusion: "success", runner_id: nil)
  runner_id ||= 10_000 + id
  steps = required_step_names(name).map.with_index do |step, index|
    { "name" => step, "status" => "completed", "conclusion" => "success", "number" => index + 1 }
  end
  steps << { "name" => "Complete job", "status" => "completed", "conclusion" => "success", "number" => 99 }
  {
    "id" => id,
    "name" => name,
    "head_sha" => candidate,
    "status" => "completed",
    "conclusion" => conclusion,
    "runner_id" => conclusion == "skipped" ? nil : runner_id,
    "runner_name" => conclusion == "skipped" ? nil : "self-test-runner-#{id}",
    "steps" => conclusion == "skipped" ? [] : steps
  }
end

def source_receipt(status)
  {
    "status" => status,
    "production_authority" => false,
    "effect_authority" => false,
    "live_adapter_authority" => false,
    "operator_acceptance" => false,
    "promotion" => false,
    "release" => false
  }
end

def build_self_test_artifacts(root, candidate, tree, with_runtime: false)
  source_dir = Pathname.new(root).join("hepta-ui-v4-exact-source-#{candidate}")
  source_dir.mkpath
  source_dir.join("candidate-commit.txt").write(candidate + "\n")
  source_dir.join("candidate-tree.txt").write(tree + "\n")
  SOURCE_STATUSES.each_with_index { |status, index| write_fixture_json(source_dir.join("source-#{index}.json"), source_receipt(status)) }
  write_fixture_json(source_dir.join("patcher-self-test.json"), { "status" => "PASS_MAKEPAD_PATCHER_SELF_TEST" })

  materialization_dir = Pathname.new(root).join("hepta-ui-v4-exact-materialization-#{candidate}")
  materialization_dir.mkpath
  write_fixture_json(materialization_dir.join("materialization.json"), {
    "status" => "PASS_PATCHED_CHECKOUT_MATERIALIZED_SOURCE_ONLY",
    "candidate" => { "commit" => candidate, "tree" => tree, "bound" => true },
    "makepad" => { "revision" => MAKEPAD_REVISION, "changedFiles" => EXPECTED_MAKEPAD_FILES },
    "cargoOverride" => { "defaultDependencyChanged" => false },
    "qualification" => {
      "patchedCheckoutMaterialized" => true, "patchAppliedInWorktree" => true,
      "patchedMakepadCompile" => false, "heptaHookCompile" => false,
      "windowsRuntime" => false, "backdropReadback" => false,
      "transientSystemMaterial" => false, "completeProfile" => false,
      "systemMaterialBinding" => false, "nativeProductRuntime" => false,
      "deviceValidation" => false
    },
    "authority" => false_hash
  })
