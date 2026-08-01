#!/usr/bin/env bash
set -euo pipefail

# Read-only provenance and drift inspection for the upstream-first Native tree.
# This script never fetches, copies, resets, commits, pushes, or edits files.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel)"
export HEPTA_ROBRIX_REPO_ROOT="$REPO_ROOT"

exec ruby - "$@" <<'RUBY'
require "digest"
require "json"
require "open3"
require "pathname"

EXPECTED_REPOSITORY = "https://github.com/project-robius/robrix.git"
EXPECTED_REMOTE = "robrix-upstream"
EXPECTED_COMMIT = "a5a664da569c577ab1a3e5a33f45dcc9364954a0"
EXPECTED_TREE = "e620da0561b6632e85eed31008f811bf94c4c24a"
EXPECTED_RAW_IMPORT_COMMIT = "7ac362f9690aa870591f4edcf533934af18921cb"
EXPECTED_UPSTREAM_FILES = 242
EXPECTED_IMPORTED_FILES = 232
EXPECTED_EXCLUDED_FILES = 10
EXPECTED_EXCLUDES = [".github/**", "AGENTS.md", "packaging/upload-release-secrets.sh"].freeze
EXPECTED_SIGNATURE_SOURCE = "GitHub REST API commit.verification"
EXPECTED_SIGNATURE_REASON = "valid"
EXPECTED_SIGNATURE_VERIFIED_AT = "2026-08-01T04:24:38Z"
DESTINATION_PREFIX = "apps/hepta-native/"
MANIFEST_HEADER = [
  "source_path",
  "decision",
  "local_path",
  "git_mode",
  "git_type",
  "git_object_sha1",
  "upstream_sha256",
  "imported_baseline_sha256",
].join("\t")

repo = Pathname(ENV.fetch("HEPTA_ROBRIX_REPO_ROOT")).realpath
app_dir = repo.join("apps/hepta-native")
lock_path = app_dir.join("UPSTREAM_ROBRIX.lock.json")
manifest_path = app_dir.join("UPSTREAM_ROBRIX_FILES.tsv")
ledger_path = app_dir.join("DOWNSTREAM_PATCHES.md")

format = :human
strict = false
emit_manifest = false
ARGV.each do |arg|
  case arg
  when "--json"
    format = :json
  when "--strict"
    strict = true
  when "--dry-run"
    # The checker is always a dry run. This flag exists to make that intent explicit.
  when "--emit-manifest"
    emit_manifest = true
  when "--help", "-h"
    puts <<~HELP
      usage: #{File.basename($PROGRAM_NAME)} [--json] [--strict] [--dry-run] [--emit-manifest]

      --json           emit a machine-readable report
      --strict         fail on downstream drift not declared in DOWNSTREAM_PATCHES.md
      --dry-run        explicit no-op alias; this checker is always read-only
      --emit-manifest  print the canonical locked manifest to stdout without writing it
    HELP
    exit 0
  else
    warn "unknown argument: #{arg}"
    exit 64
  end
end

def capture(repo, *command)
  stdout, stderr, status = Open3.capture3(*command, chdir: repo.to_s)
  [stdout, stderr, status.success?]
end

def git!(repo, *args)
  stdout, stderr, ok = capture(repo, "git", *args)
  raise "git #{args.join(' ')} failed: #{stderr.strip}" unless ok
  stdout
end

def excluded_path?(path)
  path == "AGENTS.md" || path == "packaging/upload-release-secrets.sh" || path.start_with?(".github/")
end

def upstream_entries(repo, commit)
  raw = git!(repo, "ls-tree", "-r", "-z", commit)
  raw.split("\0", -1).reject(&:empty?).map do |record|
    metadata, path = record.split("\t", 2)
    raise "malformed ls-tree record" unless metadata && path
    mode, type, object = metadata.split(" ", 3)
    raise "unsupported path containing a tab or newline: #{path.inspect}" if path.include?("\t") || path.include?("\n")
    [path, mode, type, object]
  end
end

def blob_sha256(repo, object)
  content, stderr, ok = capture(repo, "git", "cat-file", "blob", object)
  raise "git cat-file blob #{object} failed: #{stderr.strip}" unless ok
  Digest::SHA256.hexdigest(content)
end

def manifest_text(repo, entries)
  rows = entries.map do |path, mode, type, object|
    decision = excluded_path?(path) ? "excluded" : "imported"
    local_path = decision == "imported" ? "#{DESTINATION_PREFIX}#{path}" : "-"
    sha256 = blob_sha256(repo, object)
    baseline = decision == "imported" ? sha256 : "-"
    [path, decision, local_path, mode, type, object, sha256, baseline].join("\t")
  end
  ([MANIFEST_HEADER] + rows).join("\n") + "\n"
end

entries = upstream_entries(repo, EXPECTED_COMMIT)
actual_tree = git!(repo, "rev-parse", "#{EXPECTED_COMMIT}^{tree}").strip

if emit_manifest
  raise "upstream tree mismatch: #{actual_tree}" unless actual_tree == EXPECTED_TREE
  print manifest_text(repo, entries)
  exit 0
end

errors = []
lock = begin
  JSON.parse(lock_path.binread)
rescue StandardError => error
  errors << "lock_parse: #{error.message}"
  {}
end

manifest = manifest_path.file? ? manifest_path.binread : ""
canonical_manifest = manifest_text(repo, entries)
manifest_sha256 = Digest::SHA256.hexdigest(manifest)

source = lock.fetch("source", {})
snapshot = lock.fetch("snapshot", {})
import_policy = lock.fetch("import_policy", {})
sync_policy = lock.fetch("sync_policy", {})
license = lock.fetch("license", {})
verification = lock.fetch("verification", {})

fetch_url, fetch_error, fetch_ok = capture(repo, "git", "remote", "get-url", EXPECTED_REMOTE)
push_url, push_error, push_ok = capture(repo, "git", "remote", "get-url", "--push", EXPECTED_REMOTE)
fetch_url = fetch_url.strip
push_url = push_url.strip

checks = {
  "lock_schema" => lock["schema_version"] == 1 && lock["kind"] == "hepta-native-robrix-upstream-lock",
  "lock_verified" => lock["verified"] == true,
  "verification_method" => verification["method"] == "exact_git_commit_tree_and_per_file_sha256_manifest_plus_github_commit_signature",
  "commit_signature_recorded" => verification["commit_signature_verified"] == true &&
    verification["commit_signature_source"] == EXPECTED_SIGNATURE_SOURCE &&
    verification["commit_signature_reason"] == EXPECTED_SIGNATURE_REASON &&
    verification["commit_signature_verified_at_utc"] == EXPECTED_SIGNATURE_VERIFIED_AT,
  "repository_locked" => source["repository"] == EXPECTED_REPOSITORY,
  "remote_name_locked" => source["remote_name"] == EXPECTED_REMOTE,
  "commit_locked" => source["commit"] == EXPECTED_COMMIT,
  "tree_locked" => source["tree"] == EXPECTED_TREE,
  "raw_import_commit_locked" => source["raw_import_commit"] == EXPECTED_RAW_IMPORT_COMMIT,
  "git_object_tree_matches" => actual_tree == EXPECTED_TREE,
  "upstream_file_count" => entries.length == EXPECTED_UPSTREAM_FILES && snapshot["upstream_file_count"] == EXPECTED_UPSTREAM_FILES,
  "imported_file_count" => entries.count { |path,| !excluded_path?(path) } == EXPECTED_IMPORTED_FILES && snapshot["imported_file_count"] == EXPECTED_IMPORTED_FILES,
  "excluded_file_count" => entries.count { |path,| excluded_path?(path) } == EXPECTED_EXCLUDED_FILES && snapshot["excluded_file_count"] == EXPECTED_EXCLUDED_FILES,
  "include_policy" => import_policy["include"] == ["**"],
  "exclude_policy" => import_policy["exclude"] == EXPECTED_EXCLUDES,
  "destination_prefix" => import_policy["destination_prefix"] == DESTINATION_PREFIX,
  "manifest_path_locked" => snapshot["manifest"] == "UPSTREAM_ROBRIX_FILES.tsv",
  "manifest_exact" => !manifest.empty? && manifest == canonical_manifest,
  "manifest_sha256_locked" => !manifest.empty? && snapshot["manifest_sha256"] == manifest_sha256,
  "license_locked" => license["spdx"] == "MIT" && license["file"] == "LICENSE-MIT",
  "sync_mode_read_only" => sync_policy["mode"] == "read_only_dry_run" && sync_policy["whole_tree_overwrite_allowed"] == false,
  "fetch_url_locked" => sync_policy["fetch_url"] == EXPECTED_REPOSITORY,
  "push_url_locked" => sync_policy["push_url"] == "DISABLED",
  "remote_fetch_url_matches" => fetch_ok && fetch_url == EXPECTED_REPOSITORY,
  "remote_push_disabled" => push_ok && push_url == "DISABLED",
}

errors << "remote_fetch: #{fetch_error.strip}" unless fetch_ok
errors << "remote_push: #{push_error.strip}" unless push_ok

manifest_rows = manifest.lines.drop(1).map { |line| line.chomp.split("\t", -1) }
manifest_rows = [] unless manifest.lines.first&.chomp == MANIFEST_HEADER
declared_patterns = []
if ledger_path.file?
  in_ledger = false
  ledger_path.each_line do |line|
    in_ledger = true if line.include?("DOWNSTREAM_PATCHES_V1_BEGIN")
    in_ledger = false if line.include?("DOWNSTREAM_PATCHES_V1_END")
    next unless in_ledger && line.start_with?("|")
    columns = line.split("|").map(&:strip).reject(&:empty?)
    next if columns.empty? || columns[0] == "Local path or glob" || columns[0].match?(/\A-+\z/)
    declared_patterns << columns[0].delete_prefix("`").delete_suffix("`")
  end
end

def local_content_sha256(path, git_mode)
  if git_mode == "120000"
    Digest::SHA256.hexdigest(File.readlink(path))
  else
    Digest::SHA256.file(path).hexdigest
  end
end

def declared?(relative_path, patterns)
  patterns.any? do |pattern|
    File.fnmatch?(pattern, relative_path, File::FNM_PATHNAME | File::FNM_EXTGLOB)
  end
end

matched = []
modified = []
missing = []
unexpected_excluded_present = []
baseline_paths = []
manifest_rows.each do |source_path, decision, local_path, git_mode, _git_type, _object, _upstream_sha, baseline_sha|
  next unless source_path
  if decision == "imported"
    baseline_paths << local_path
    absolute = repo.join(local_path)
    if !absolute.exist? && !absolute.symlink?
      missing << local_path.delete_prefix(DESTINATION_PREFIX)
    elsif local_content_sha256(absolute, git_mode) == baseline_sha
      matched << local_path.delete_prefix(DESTINATION_PREFIX)
    else
      modified << local_path.delete_prefix(DESTINATION_PREFIX)
    end
  elsif decision == "excluded"
    candidate = app_dir.join(source_path)
    unexpected_excluded_present << source_path if candidate.exist? || candidate.symlink?
  end
end

known_governance = [
  "#{DESTINATION_PREFIX}UPSTREAM_ROBRIX.lock.json",
  "#{DESTINATION_PREFIX}UPSTREAM_ROBRIX_FILES.tsv",
  "#{DESTINATION_PREFIX}DOWNSTREAM_PATCHES.md",
]
all_local_raw = git!(repo, "ls-files", "--cached", "--others", "--exclude-standard", "-z", "--", DESTINATION_PREFIX)
all_local = all_local_raw.split("\0", -1).reject(&:empty?)
extras = all_local - baseline_paths - known_governance
extras.map! { |path| path.delete_prefix(DESTINATION_PREFIX) }

drift_paths = (modified + missing + extras).uniq.sort
declared_drift = drift_paths.select { |path| declared?(path, declared_patterns) }
undeclared_drift = drift_paths - declared_drift

provenance_ready = checks.values.all?
raw_snapshot_exact = modified.empty? && missing.empty? && extras.empty? && unexpected_excluded_present.empty?
downstream_overlay_accounted = undeclared_drift.empty? && unexpected_excluded_present.empty?
strict_ready = provenance_ready && downstream_overlay_accounted

report = {
  "schema_version" => 2,
  "kind" => "hepta-native-robrix-upstream-sync-check",
  "status" => (strict ? strict_ready : provenance_ready) ? "ready" : "not_ready",
  "mode" => strict ? "strict_declared_overlay" : "read_only_dry_run",
  "read_only" => true,
  "would_modify_worktree" => false,
  "source" => {
    "repository" => EXPECTED_REPOSITORY,
    "commit" => EXPECTED_COMMIT,
    "tree" => EXPECTED_TREE,
    "upstream_file_count" => entries.length,
    "manifest_sha256" => manifest_sha256,
    "commit_signature" => {
      "verified" => verification["commit_signature_verified"] == true,
      "source" => verification["commit_signature_source"],
      "reason" => verification["commit_signature_reason"],
      "verified_at_utc" => verification["commit_signature_verified_at_utc"],
      "local_gpg_trust_chain_claimed" => false,
    },
  },
  "remote" => {
    "name" => EXPECTED_REMOTE,
    "fetch_url" => fetch_url,
    "push_url" => push_url,
  },
  "checks" => checks,
  "provenance_ready" => provenance_ready,
  "raw_snapshot_exact" => raw_snapshot_exact,
  "downstream_overlay_accounted" => downstream_overlay_accounted,
  "local_snapshot" => {
    "baseline_imported_files" => manifest_rows.count { |row| row[1] == "imported" },
    "unchanged_files" => matched.length,
    "modified_files" => modified,
    "missing_files" => missing,
    "extra_files" => extras.sort,
    "unexpected_excluded_files" => unexpected_excluded_present.sort,
    "declared_patterns" => declared_patterns,
    "declared_drift" => declared_drift,
    "undeclared_drift" => undeclared_drift,
  },
  "errors" => errors,
}

if format == :json
  puts JSON.pretty_generate(report)
else
  puts "Robrix upstream sync check v2: #{report['status']}"
  puts "  source: #{EXPECTED_COMMIT} / #{EXPECTED_TREE}"
  puts "  provenance_ready: #{provenance_ready}"
  puts "  remote_push_disabled: #{checks['remote_push_disabled']}"
  puts "  upstream files: #{entries.length} (imported #{EXPECTED_IMPORTED_FILES}, excluded #{EXPECTED_EXCLUDED_FILES})"
  puts "  current unchanged/modified/missing/extra: #{matched.length}/#{modified.length}/#{missing.length}/#{extras.length}"
  puts "  downstream_overlay_accounted: #{downstream_overlay_accounted}"
  puts "  would_modify_worktree: false"
end

exit((strict ? strict_ready : provenance_ready) ? 0 : 1)
RUBY
