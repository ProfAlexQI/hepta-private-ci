#!/usr/bin/env ruby
# frozen_string_literal: true

require "digest"
require "json"
require "pathname"

root = Pathname.new(ARGV.fetch(0)).expand_path
abort "bundle root is not a directory: #{root}" unless root.directory?

entries = [root] + Dir.glob(root.join("**", "*").to_s, File::FNM_DOTMATCH)
  .map { |path| Pathname.new(path) }
  .reject { |path| [".", ".."].include?(path.basename.to_s) }
entries.sort_by! { |path| path == root ? "" : path.relative_path_from(root).to_s.b }

manifest = []
file_bytes = 0
symlink_count = 0
unsupported_count = 0

entries.each do |path|
  stat = path.lstat
  relative = path == root ? "." : path.relative_path_from(root).to_s
  record = {
    path: relative,
    mode: format("%04o", stat.mode & 0o7777)
  }
  if stat.directory?
    record[:type] = "directory"
  elsif stat.file?
    record[:type] = "file"
    record[:bytes] = stat.size
    record[:sha256] = Digest::SHA256.file(path).hexdigest
    file_bytes += stat.size
  elsif stat.symlink?
    record[:type] = "symlink"
    record[:target] = path.readlink.to_s
    symlink_count += 1
  else
    record[:type] = "unsupported"
    unsupported_count += 1
  end
  manifest << record
end

canonical = manifest.map { |entry| JSON.generate(entry) }.join("\n") + "\n"
puts JSON.generate(
  schema_version: 1,
  kind: "hepta-app-bundle-fingerprint",
  entry_count: manifest.length,
  file_count: manifest.count { |entry| entry[:type] == "file" },
  directory_count: manifest.count { |entry| entry[:type] == "directory" },
  symlink_count: symlink_count,
  unsupported_entry_count: unsupported_count,
  file_bytes: file_bytes,
  manifest_sha256: Digest::SHA256.hexdigest(canonical),
  symlinks_rejected: symlink_count.zero?,
  supported_entry_types_only: unsupported_count.zero?
)
