#!/usr/bin/env bash
set -euo pipefail

# Read-only provenance and path-by-path drift inspection for the upstream-first
# Native tree. All anchors are read from the committed lock/manifest. This tool
# never fetches, copies, resets, commits, pushes, or edits the worktree.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel)"
export HEPTA_ROBRIX_REPO_ROOT="$REPO_ROOT"

exec ruby - "$@" <<'RUBY'
require "digest"
require "json"
require "open3"
require "pathname"

DESTINATION_PREFIX = "apps/hepta-native/"
LOCK_BASENAME = "UPSTREAM_ROBRIX.lock.json"
MANIFEST_HEADER = %w[
  source_path decision local_path git_mode git_type git_object_sha1
  upstream_sha256 imported_baseline_sha256
].join("\t")

format = :human
strict = false
emit_manifest = false
ARGV.each do |arg|
  case arg
  when "--json" then format = :json
  when "--strict" then strict = true
  when "--dry-run" then nil
  when "--emit-manifest" then emit_manifest = true
  when "--help", "-h"
    puts <<~HELP
      usage: #{File.basename($PROGRAM_NAME)} [--json] [--strict] [--dry-run] [--emit-manifest]

      --json           emit a machine-readable report
      --strict         require a unique ledger rule for every drift path
      --dry-run        explicit no-op alias; this checker is always read-only
      --emit-manifest  print the canonical locked manifest without writing it
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

def source_binding!(repo)
  script = repo.join("scripts/hepta-ui-source-fingerprint").to_s
  stdout, stderr, ok = capture(repo, script)
  raise "source fingerprint failed: #{stderr.strip}" unless ok
  JSON.parse(stdout)
end

def binding_equal?(left, right)
  %w[head head_tree source_fingerprint].all? { |key| left[key] == right[key] }
end

def safe_relative_basename?(value)
  value.is_a?(String) && !value.empty? && Pathname(value).relative? &&
    !value.split("/").include?("..")
end

def path_matches?(pattern, path)
  if pattern.end_with?("/**")
    prefix = pattern.delete_suffix("/**")
    return true if path == prefix || path.start_with?("#{prefix}/")
  end
  File.fnmatch?(pattern, path, File::FNM_PATHNAME | File::FNM_EXTGLOB)
end

def excluded_path?(path, patterns)
  patterns.any? { |pattern| path_matches?(pattern, path) }
end

def capture_blob(repo, revision, path)
  stdout, _stderr, ok = capture(repo, "git", "show", "#{revision}:#{DESTINATION_PREFIX}#{path}")
  ok ? stdout : nil
end

def local_content_sha256(path, git_mode)
  if git_mode == "120000"
    Digest::SHA256.hexdigest(File.readlink(path))
  else
    Digest::SHA256.file(path).hexdigest
  end
end

repo = Pathname(ENV.fetch("HEPTA_ROBRIX_REPO_ROOT")).realpath
app_dir = repo.join(DESTINATION_PREFIX)
lock_path = app_dir.join(LOCK_BASENAME)
binding_before = source_binding!(repo)

errors = []
lock = begin
  JSON.parse(lock_path.binread)
rescue StandardError => error
  errors << "lock_parse: #{error.message}"
  {}
end

source = lock.fetch("source", {})
snapshot = lock.fetch("snapshot", {})
import_policy = lock.fetch("import_policy", {})
sync_policy = lock.fetch("sync_policy", {})
license = lock.fetch("license", {})
verification = lock.fetch("verification", {})

repository = source["repository"].to_s
remote_name = source["remote_name"].to_s
upstream_commit = source["commit"].to_s
upstream_tree = source["tree"].to_s
raw_import_commit = source["raw_import_commit"].to_s
lineage_import_commit = source["current_lineage_import_commit"].to_s
destination_prefix = import_policy["destination_prefix"].to_s
exclude_patterns = Array(import_policy["exclude"])
manifest_basename = snapshot["manifest"].to_s
ledger_basename = sync_policy["downstream_patch_ledger"].to_s
manifest_path = app_dir.join(manifest_basename)
ledger_path = app_dir.join(ledger_basename)

sha1 = /\A[0-9a-f]{40}\z/
lock_shape = {
  "lock_schema" => lock["schema_version"] == 1 && lock["kind"] == "hepta-native-robrix-upstream-lock",
  "lock_verified" => lock["verified"] == true,
  "repository_https" => repository.match?(%r{\Ahttps://github\.com/[^/]+/[^/]+(?:\.git)?\z}),
  "remote_name_safe" => remote_name.match?(/\A[a-zA-Z0-9._-]+\z/),
  "commit_shape" => upstream_commit.match?(sha1),
  "tree_shape" => upstream_tree.match?(sha1),
  "raw_import_commit_shape" => raw_import_commit.match?(sha1),
  "lineage_import_commit_shape" => lineage_import_commit.match?(sha1),
  "destination_prefix_locked" => destination_prefix == DESTINATION_PREFIX,
  "manifest_path_safe" => safe_relative_basename?(manifest_basename),
  "ledger_path_safe" => safe_relative_basename?(ledger_basename),
  "include_policy" => import_policy["include"] == ["**"],
  "exclude_policy_nonempty" => !exclude_patterns.empty? && exclude_patterns.all? { |item| safe_relative_basename?(item) },
  "sync_mode_read_only" => sync_policy["mode"] == "read_only_dry_run" && sync_policy["whole_tree_overwrite_allowed"] == false,
  "push_url_disabled_in_lock" => sync_policy["push_url"] == "DISABLED",
  "fetch_url_matches_repository" => sync_policy["fetch_url"] == repository,
  "license_locked" => license["spdx"] == "MIT" && safe_relative_basename?(license["file"]),
  "signature_recorded" => verification["commit_signature_verified"] == true &&
    !verification["commit_signature_source"].to_s.empty? &&
    !verification["commit_signature_reason"].to_s.empty? &&
    !verification["commit_signature_verified_at_utc"].to_s.empty?,
}

unless lock_shape.values.all?
  binding_after = source_binding!(repo)
  report = {
    "schema_version" => 3,
    "kind" => "hepta-native-robrix-upstream-sync-check",
    "status" => "not_ready",
    "mode" => strict ? "strict_path_ledger" : "read_only_provenance",
    "read_only" => true,
    "would_modify_worktree" => false,
    "source_binding_before" => binding_before,
    "source_binding" => binding_after,
    "source_stable_during_run" => binding_equal?(binding_before, binding_after),
    "checks" => lock_shape,
    "errors" => errors + ["lock shape or safety policy is invalid"],
  }
  puts(format == :json ? JSON.pretty_generate(report) : "Robrix upstream sync check: not_ready (invalid lock)")
  exit 1
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

def manifest_text(repo, entries, exclusions)
  rows = entries.map do |path, mode, type, object|
    decision = excluded_path?(path, exclusions) ? "excluded" : "imported"
    local_path = decision == "imported" ? "#{DESTINATION_PREFIX}#{path}" : "-"
    sha256 = blob_sha256(repo, object)
    baseline = decision == "imported" ? sha256 : "-"
    [path, decision, local_path, mode, type, object, sha256, baseline].join("\t")
  end
  ([MANIFEST_HEADER] + rows).join("\n") + "\n"
end

def reconstructed_tree_sha1(entries)
  root = {}
  entries.each do |path, mode, type, object|
    parts = path.split("/")
    node = root
    parts[0...-1].each do |part|
      child = (node[part] ||= { "tree" => {} })
      raise "manifest path collision at #{path}" unless child.key?("tree")
      node = child["tree"]
    end
    leaf = parts.last
    raise "duplicate manifest path: #{path}" if node.key?(leaf)
    node[leaf] = { "mode" => mode, "type" => type, "object" => object }
  end

  hash_tree = lambda do |node|
    content = node.keys.sort_by { |name| name.b }.map do |name|
      entry = node.fetch(name)
      if entry.key?("tree")
        mode = "40000"
        object = hash_tree.call(entry.fetch("tree"))
      else
        mode = entry.fetch("mode").sub(/\A0+/, "")
        object = entry.fetch("object")
      end
      "#{mode} #{name}\0".b + [object].pack("H*")
    end.join
    Digest::SHA1.hexdigest("tree #{content.bytesize}\0".b + content)
  end
  hash_tree.call(root)
end

manifest = manifest_path.file? ? manifest_path.binread : ""
manifest_sha256 = Digest::SHA256.hexdigest(manifest)
manifest_header_valid = manifest.lines.first&.chomp == MANIFEST_HEADER
manifest_rows = manifest.lines.drop(1).map { |line| line.chomp.split("\t", -1) }
manifest_rows = [] unless manifest_header_valid

manifest_schema_valid = manifest_header_valid && !manifest_rows.empty? && manifest_rows.all? do |row|
  source_path, decision, local_path, git_mode, git_type, object, upstream_sha, baseline_sha = row
  row.length == 8 && safe_relative_basename?(source_path) && %w[imported excluded].include?(decision) &&
    git_mode.to_s.match?(/\A(?:100644|100755|120000)\z/) && git_type == "blob" && object.to_s.match?(sha1) &&
    upstream_sha.to_s.match?(/\A[0-9a-f]{64}\z/) &&
    (decision == "imported" ? (local_path == "#{DESTINATION_PREFIX}#{source_path}" && baseline_sha == upstream_sha) : (local_path == "-" && baseline_sha == "-")) &&
    (excluded_path?(source_path, exclude_patterns) == (decision == "excluded"))
end
manifest_paths_unique = manifest_rows.map(&:first).uniq.length == manifest_rows.length
manifest_entries = manifest_rows.map { |row| [row[0], row[3], row[4], row[5]] }
manifest_tree = if manifest_schema_valid && manifest_paths_unique
  reconstructed_tree_sha1(manifest_entries)
end

_object_out, _object_error, upstream_object_available = capture(repo, "git", "cat-file", "-e", "#{upstream_commit}^{commit}")
entries = upstream_object_available ? upstream_entries(repo, upstream_commit) : manifest_entries
actual_tree = upstream_object_available ? git!(repo, "rev-parse", "#{upstream_commit}^{tree}").strip : nil
canonical_manifest = upstream_object_available ? manifest_text(repo, entries, exclude_patterns) : nil

if emit_manifest
  raise "upstream commit object is unavailable; committed manifest remains the offline authority" unless upstream_object_available
  raise "locked upstream tree mismatch: #{actual_tree}" unless actual_tree == upstream_tree
  print canonical_manifest
  exit 0
end

fetch_url, fetch_error, fetch_ok = capture(repo, "git", "remote", "get-url", remote_name)
remote_configured = fetch_ok
if remote_configured
  push_url, push_error, push_ok = capture(repo, "git", "remote", "get-url", "--push", remote_name)
  fetch_url = fetch_url.strip
  push_url = push_url.strip
  errors << "remote_push: #{push_error.strip}" unless push_ok
else
  fetch_url = "NOT_CONFIGURED"
  push_url = "NOT_CONFIGURED"
  push_ok = true
end
remote_hygiene_ready = !remote_configured || (fetch_url == repository && push_ok && push_url == "DISABLED")

head = git!(repo, "rev-parse", "HEAD").strip
branch = git!(repo, "branch", "--show-current").strip
_ancestor_out, ancestor_error, lineage_import_is_ancestor = capture(repo, "git", "merge-base", "--is-ancestor", lineage_import_commit, head)
errors << "lineage_import_ancestor: #{ancestor_error.strip}" unless lineage_import_is_ancestor

raw_import_mismatches = []
lineage_import_mismatches = []
_raw_out, _raw_error, raw_import_object_available = capture(repo, "git", "cat-file", "-e", "#{raw_import_commit}^{commit}")
manifest_rows.each do |source_path, decision, _local_path, _mode, _type, _object, _upstream_sha, baseline_sha|
  next unless source_path
  if raw_import_object_available
    raw_blob = capture_blob(repo, raw_import_commit, source_path)
    if decision == "imported"
      raw_import_mismatches << source_path unless raw_blob && Digest::SHA256.hexdigest(raw_blob) == baseline_sha
    elsif raw_blob
      raw_import_mismatches << source_path
    end
  end
  lineage_blob = capture_blob(repo, lineage_import_commit, source_path)
  if decision == "imported"
    lineage_import_mismatches << source_path unless lineage_blob && Digest::SHA256.hexdigest(lineage_blob) == baseline_sha
  elsif lineage_blob
    lineage_import_mismatches << source_path
  end
end

checks = lock_shape.merge(
  "manifest_schema_valid" => manifest_schema_valid,
  "manifest_paths_unique" => manifest_paths_unique,
  "manifest_reconstructs_locked_tree" => manifest_tree == upstream_tree,
  "upstream_file_count" => manifest_rows.length == snapshot["upstream_file_count"],
  "imported_file_count" => manifest_rows.count { |row| row[1] == "imported" } == snapshot["imported_file_count"],
  "excluded_file_count" => manifest_rows.count { |row| row[1] == "excluded" } == snapshot["excluded_file_count"],
  "manifest_sha256_locked" => !manifest.empty? && snapshot["manifest_sha256"] == manifest_sha256,
  "lineage_import_is_current_ancestor" => lineage_import_is_ancestor,
  "remote_hygiene_ready" => remote_hygiene_ready,
)

ledger_rules = []
ledger_parse_errors = []
if ledger_path.file?
  in_ledger = false
  ledger_path.each_line.with_index(1) do |line, line_number|
    in_ledger = true if line.include?("DOWNSTREAM_PATCHES_V1_BEGIN")
    in_ledger = false if line.include?("DOWNSTREAM_PATCHES_V1_END")
    next unless in_ledger && line.start_with?("|")
    columns = line.split("|").map(&:strip).reject(&:empty?)
    next if columns.empty? || columns[0] == "Local path or glob" || columns[0].match?(/\A-+\z/)
    if columns.length != 4
      ledger_parse_errors << "line #{line_number}: expected four columns"
      next
    end
    pattern = columns[0].delete_prefix("`").delete_suffix("`")
    unless safe_relative_basename?(pattern)
      ledger_parse_errors << "line #{line_number}: unsafe path pattern #{pattern.inspect}"
      next
    end
    ledger_rules << {
      "pattern" => pattern,
      "class" => columns[1],
      "purpose" => columns[2],
      "verification" => columns[3],
      "line" => line_number,
    }
  end
else
  ledger_parse_errors << "missing ledger: #{ledger_path}"
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
  "#{DESTINATION_PREFIX}#{LOCK_BASENAME}",
  "#{DESTINATION_PREFIX}#{manifest_basename}",
  "#{DESTINATION_PREFIX}#{ledger_basename}",
]
all_local_raw = git!(repo, "ls-files", "--cached", "--others", "--exclude-standard", "-z", "--", DESTINATION_PREFIX)
all_local = all_local_raw.split("\0", -1).reject(&:empty?).select do |path|
  absolute = repo.join(path)
  absolute.exist? || absolute.symlink?
end
extras = (all_local - baseline_paths - known_governance).map { |path| path.delete_prefix(DESTINATION_PREFIX) }.sort
drift_paths = (modified + missing + extras).uniq.sort

drift_ledger = drift_paths.map do |path|
  matches = ledger_rules.select { |rule| path_matches?(rule["pattern"], path) }
  {
    "path" => path,
    "kind" => missing.include?(path) ? "removed_upstream" : (extras.include?(path) ? "downstream_addition" : "modified_upstream"),
    "rule_count" => matches.length,
    "rules" => matches,
  }
end
undeclared_drift = drift_ledger.select { |entry| entry["rule_count"].zero? }.map { |entry| entry["path"] }
ambiguous_drift = drift_ledger.select { |entry| entry["rule_count"] > 1 }.map { |entry| entry["path"] }
used_rule_lines = drift_ledger.flat_map { |entry| entry["rules"].map { |rule| rule["line"] } }.uniq
unused_ledger_rules = ledger_rules.reject { |rule| used_rule_lines.include?(rule["line"]) }

binding_after = source_binding!(repo)
source_stable = binding_equal?(binding_before, binding_after)
checks["source_stable_during_run"] = source_stable
provenance_ready = checks.values.all? && errors.empty?
raw_snapshot_exact = raw_import_object_available && raw_import_mismatches.empty?
path_ledger_ready = ledger_parse_errors.empty? && undeclared_drift.empty? && ambiguous_drift.empty? &&
  unused_ledger_rules.empty? && unexpected_excluded_present.empty?
strict_ready = provenance_ready && path_ledger_ready

report = {
  "schema_version" => 3,
  "kind" => "hepta-native-robrix-upstream-sync-check",
  "status" => (strict ? strict_ready : provenance_ready) ? "ready" : "not_ready",
  "mode" => strict ? "strict_path_ledger" : "read_only_provenance",
  "read_only" => true,
  "would_modify_worktree" => false,
  "source_binding_before" => binding_before,
  "source_binding" => binding_after,
  "source_stable_during_run" => source_stable,
  "source" => {
    "repository" => repository,
    "remote_name" => remote_name,
    "commit" => upstream_commit,
    "tree" => upstream_tree,
    "raw_import_commit" => raw_import_commit,
    "current_lineage_import_commit" => lineage_import_commit,
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
  "remote" => { "name" => remote_name, "configured" => remote_configured, "fetch_url" => fetch_url, "push_url" => push_url, "hygiene_ready" => remote_hygiene_ready },
  "advisory" => {
    "upstream_commit_object_available" => upstream_object_available,
    "upstream_object_tree_matches" => upstream_object_available ? actual_tree == upstream_tree : nil,
    "canonical_manifest_matches_upstream_object" => upstream_object_available ? manifest == canonical_manifest : nil,
    "raw_import_commit_object_available" => raw_import_object_available,
    "raw_import_snapshot_exact" => raw_import_object_available ? raw_import_mismatches.empty? : nil,
  },
  "checks" => checks,
  "provenance_ready" => provenance_ready,
  "raw_snapshot_exact" => raw_snapshot_exact,
  "downstream_overlay_accounted" => path_ledger_ready,
  "path_ledger_ready" => path_ledger_ready,
  "local_snapshot" => {
    "baseline_imported_files" => manifest_rows.count { |row| row[1] == "imported" },
    "unchanged_files" => matched.length,
    "modified_files" => modified.sort,
    "missing_files" => missing.sort,
    "extra_files" => extras,
    "unexpected_excluded_files" => unexpected_excluded_present.sort,
    "drift_ledger" => drift_ledger,
    "undeclared_drift" => undeclared_drift,
    "ambiguous_drift" => ambiguous_drift,
    "unused_ledger_rules" => unused_ledger_rules,
    "raw_import_mismatches" => raw_import_mismatches.sort,
    "lineage_import_mismatches" => lineage_import_mismatches.sort,
  },
  "ledger_parse_errors" => ledger_parse_errors,
  "errors" => errors,
}

if format == :json
  puts JSON.pretty_generate(report)
else
  puts "Robrix upstream sync check v2: #{report['status']}"
  puts "  current source: #{branch}@#{head}"
  puts "  upstream: #{upstream_commit} / #{upstream_tree}"
  puts "  exact raw snapshot advisory: #{raw_import_commit} (object=#{raw_import_object_available}, exact=#{report.dig('advisory', 'raw_import_snapshot_exact').inspect})"
  puts "  current lineage import: #{lineage_import_commit} (ancestor=#{checks['lineage_import_is_current_ancestor']}, baseline deviations=#{lineage_import_mismatches.length})"
  puts "  provenance_ready: #{provenance_ready}"
  puts "  remote hygiene: configured=#{remote_configured}, ready=#{remote_hygiene_ready}"
  puts "  current unchanged/modified/missing/extra: #{matched.length}/#{modified.length}/#{missing.length}/#{extras.length}"
  puts "  ledger undeclared/ambiguous/unused: #{undeclared_drift.length}/#{ambiguous_drift.length}/#{unused_ledger_rules.length}"
  puts "  downstream_overlay_accounted: #{path_ledger_ready}"
  puts "  would_modify_worktree: false"
end

exit((strict ? strict_ready : provenance_ready) ? 0 : 1)
RUBY
