#!/usr/bin/env ruby
# frozen_string_literal: true

require "json"
require "optparse"
require "pathname"
require "tmpdir"
require "fileutils"

SCHEMA = "hepta.ui.v4.exact-head-qualification-index.v1"
MAKEPAD_REVISION = "c4335cee10b22aca768510c9d072b0ca1bba15c8"
SOURCE_STATUSES = [
  "PASS_WINDOWS_MATERIAL_PROFILE_AGGREGATE_SOURCE_ONLY",
  "PASS_WINDOWS_MATERIAL_PROFILE_RUNTIME_SOURCE_ONLY",
  "PASS_EXACT_CANDIDATE_MATERIALIZATION_HARDENING_SOURCE_ONLY",
  "PASS_EXACT_HEAD_QUALIFICATION_CLOSURE_SOURCE_ONLY"
].freeze
EXPECTED_MAKEPAD_FILES = [
  "platform/src/cx_api.rs",
  "platform/src/lib.rs",
  "platform/src/os/windows/win32_window.rs",
  "platform/src/os/windows/windows.rs",
  "platform/src/window.rs"
].freeze
SOURCE_JOB = "Exact-head full source chain and patcher self-test"
PATCH_JOB = "Candidate-bound Makepad materialization"
COMPILE_JOBS = {
  "ubuntu-latest" => "Exact-head compile and focused tests (ubuntu-latest)",
  "windows-latest" => "Exact-head compile and focused tests (windows-latest)",
  "macos-latest" => "Exact-head compile and focused tests (macos-latest)"
}.freeze
RUNTIME_JOB = "Governed exact-head Windows Mica/Acrylic producer"
INDEX_JOB = "Canonical exact-head qualification index"
ALLOWED_STEP_CONCLUSIONS = ["success", "skipped"].freeze

class QualificationError < StandardError; end

Options = Struct.new(
  :jobs,
  :artifacts,
  :candidate_commit,
  :candidate_tree,
  :repository,
  :run_id,
  :output,
  :require_windows_runtime,
  :allow_in_progress_run,
  :self_test,
  keyword_init: true
)

def git_object_id?(value)
  value.is_a?(String) && value.match?(/\A[0-9a-f]{40}\z/)
end

def false_hash
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

def parse_json(path)
  JSON.parse(Pathname.new(path).read)
rescue JSON::ParserError => error
  raise QualificationError, "invalid JSON #{path}: #{error.message}"
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

def jobs_from(payload)
  jobs = payload.is_a?(Hash) ? payload.fetch("jobs", nil) : payload
  raise QualificationError, "jobs payload has no jobs array" unless jobs.is_a?(Array)
  jobs
end

def job_by_name!(jobs, name)
  matches = jobs.select { |job| job["name"] == name }
  raise QualificationError, "job #{name.inspect} count=#{matches.length}" unless matches.length == 1
  matches.first
end

def required_step_names(job_name)
  case job_name
  when SOURCE_JOB
    ["Verify exact checkout identity", "Run inherited and canonical source gates"]
  when PATCH_JOB
    ["Verify exact checkout identity", "Materialize exact patched checkout", "Validate materialization receipt"]
  when *COMPILE_JOBS.values
    ["Verify exact checkout identity", "Materialize patched Makepad", "Format, compile, and focused tests"]
  when RUNTIME_JOB
    ["Verify exact checkout identity", "Materialize patched Makepad", "Run governed producer", "Validate exact aggregate PASS receipt"]
  else
    []
  end
end

def validate_executed_job!(job, candidate, expected_name)
  raise QualificationError, "#{expected_name}: head SHA drift" unless job["head_sha"] == candidate
  raise QualificationError, "#{expected_name}: status=#{job['status']}" unless job["status"] == "completed"
  raise QualificationError, "#{expected_name}: conclusion=#{job['conclusion']}" unless job["conclusion"] == "success"
  runner_id = job["runner_id"]
  raise QualificationError, "#{expected_name}: runner_id missing" unless runner_id.is_a?(Integer) && runner_id.positive?
  runner_name = job["runner_name"].to_s
  raise QualificationError, "#{expected_name}: runner_name missing" if runner_name.empty?
  steps = job["steps"]
  raise QualificationError, "#{expected_name}: steps missing" unless steps.is_a?(Array) && !steps.empty?
  invalid = steps.reject { |step| ALLOWED_STEP_CONCLUSIONS.include?(step["conclusion"]) }
  raise QualificationError, "#{expected_name}: bad steps #{invalid.map { |s| s['name'] }.join(', ')}" unless invalid.empty?
  names = steps.map { |step| step["name"] }
  missing = required_step_names(expected_name) - names
  raise QualificationError, "#{expected_name}: missing steps #{missing.join(', ')}" unless missing.empty?
  {
    "name" => expected_name,
    "jobId" => job["id"],
    "runnerId" => runner_id,
    "runnerName" => runner_name,
    "stepCount" => steps.length,
    "qualified" => true
  }
end

def artifact_dir!(root, name)
  path = Pathname.new(root).join(name)
  raise QualificationError, "artifact directory missing: #{name}" unless path.directory?
  path
end

def read_exact_text!(root, basename)
  matches = Pathname.new(root).glob("**/#{basename}").select(&:file?)
  raise QualificationError, "#{basename} count=#{matches.length}" unless matches.length == 1
  matches.first.read.strip
end

def validate_source_artifact!(root, candidate, tree)
  directory = artifact_dir!(root, "hepta-ui-v4-exact-source-#{candidate}")
  commit_text = directory.join("candidate-commit.txt").read.strip
  tree_text = directory.join("candidate-tree.txt").read.strip
  raise QualificationError, "source candidate commit drift" unless commit_text == candidate
  raise QualificationError, "source candidate tree drift" unless tree_text == tree

  statuses = {}
  directory.glob("**/*.json").each do |path|
    payload = parse_json(path)
    status = payload["status"]
    statuses[status] = (statuses[status] || []) << [path, payload] if status
  end
  SOURCE_STATUSES.each do |status|
    matches = statuses.fetch(status, [])
    raise QualificationError, "source status #{status} count=#{matches.length}" unless matches.length == 1
    receipt = matches.first.last
    require_false_hash!(receipt["authority"], "source #{status} authority") if receipt.key?("authority")
    %w[production_authority effect_authority live_adapter_authority operator_acceptance promotion release].each do |key|
      raise QualificationError, "source #{status} #{key} escaped" if receipt[key] == true
    end
  end
  self_test = statuses.fetch("PASS_MAKEPAD_PATCHER_SELF_TEST", [])
  raise QualificationError, "patcher self-test count=#{self_test.length}" unless self_test.length == 1
  { "qualified" => true, "statuses" => SOURCE_STATUSES, "artifact" => directory.basename.to_s }
end

def validate_materialization!(root, candidate, tree)
  directory = artifact_dir!(root, "hepta-ui-v4-exact-materialization-#{candidate}")
