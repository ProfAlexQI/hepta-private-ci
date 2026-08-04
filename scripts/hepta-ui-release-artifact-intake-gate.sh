#!/bin/bash -p
set +x
PS4='+ '
set -euo pipefail
unset BASH_ENV ENV CDPATH GLOBIGNORE RUBYOPT RUBYLIB GEM_HOME GEM_PATH BUNDLE_GEMFILE BUNDLE_PATH
SYSTEM_PATH="/usr/bin:/bin:/usr/sbin:/sbin"
PATH="$SYSTEM_PATH"
export PATH

cd "$(/usr/bin/dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_RELEASE_ARTIFACT_INTAKE_REPORT_PATH:-$READINESS_DIR/ui-release-artifact-intake-gate.json}"
INTAKE_DIR="${HEPTA_UI_RELEASE_ARTIFACT_INTAKE_DIR:-$READINESS_DIR/release-artifact-intake}"
ARTIFACT_INPUT_PATH="${HEPTA_UI_RELEASE_ARTIFACT_INPUT_PATH:-}"
TEMPLATE_PATH="$INTAKE_DIR/release-artifact-template.json"
MARKDOWN_PATH="$INTAKE_DIR/release-artifact-intake.md"
ACCEPTED_ARTIFACT_INPUT_PATH="$INTAKE_DIR/release-artifact-input.accepted.json"
CAPTURED_ARTIFACT_INPUT_PATH="$INTAKE_DIR/release-artifact-input.captured.json"
READBACK_REPORT_PATH="$INTAKE_DIR/release-artifact-readback.json"
APP_BUNDLE_FINGERPRINT="apps/hepta-native/packaging/app-bundle-fingerprint-v1.rb"
FINDER_BOOKMARK_RESOLVER="apps/hepta-native/packaging/resolve-finder-bookmark-v1.swift"
SOURCE_FINGERPRINT_HELPER="scripts/hepta-ui-source-fingerprint"

# Artifact-present verification is a trust boundary. These are Apple-owned,
# absolute tool paths so a release receipt cannot be promoted by PATH shims.
SYSTEM_ENV="/usr/bin/env"
SYSTEM_CODESIGN="/usr/bin/codesign"
SYSTEM_DITTO="/usr/bin/ditto"
SYSTEM_FIND="/usr/bin/find"
SYSTEM_GIT="/usr/bin/git"
SYSTEM_HDIUTIL="/usr/bin/hdiutil"
SYSTEM_JQ="/usr/bin/jq"
SYSTEM_MOUNT="/sbin/mount"
SYSTEM_PLUTIL="/usr/bin/plutil"
SYSTEM_RUBY="/usr/bin/ruby"
SYSTEM_SHASUM="/usr/bin/shasum"
SYSTEM_SPCTL="/usr/sbin/spctl"
SYSTEM_SWIFT="/usr/bin/swift"
SYSTEM_XCRUN="/usr/bin/xcrun"
SYSTEM_PATH="/usr/bin:/bin:/usr/sbin:/sbin"

for bootstrap_system_tool in \
  "$SYSTEM_ENV" \
  "$SYSTEM_JQ" \
  "$SYSTEM_RUBY" \
  /bin/cat \
  /bin/cp \
  /bin/mkdir \
  /bin/rm \
  /usr/bin/dirname \
  /usr/bin/mktemp; do
  if [[ ! -x "$bootstrap_system_tool" || -L "$bootstrap_system_tool" ]]; then
    printf 'Missing canonical macOS bootstrap tool: %s\n' "$bootstrap_system_tool" >&2
    exit 2
  fi
done

# Keep control-flow and output writes independent of caller-controlled PATH.
jq() { "$SYSTEM_JQ" "$@"; }
cat() { /bin/cat "$@"; }
cp() { /bin/cp "$@"; }
dirname() { /usr/bin/dirname "$@"; }
mkdir() { /bin/mkdir "$@"; }
mktemp() { /usr/bin/mktemp "$@"; }
rm() { /bin/rm "$@"; }

DISTRIBUTION_PREFLIGHT_REPORT_PATH="$READINESS_DIR/native-distribution-preflight-gate.json"
RELEASE_APPROVAL_INTAKE_REPORT_PATH="$READINESS_DIR/ui-release-approval-intake-gate.json"
RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH="$READINESS_DIR/ui-release-artifact-boundary-gate.json"
EVIDENCE_ARCHIVE_REPORT_PATH="$READINESS_DIR/ui-evidence-archive-gate.json"

HEPTA_UI_GATE_REQUIREMENT_CONTEXT="the Hepta UI release artifact intake gate"
HEPTA_UI_REPORT_INPUT_LABEL="release artifact intake"
source scripts/lib/hepta-ui-gate-common-v1.sh

absolute_path() {
  "$SYSTEM_ENV" -i PATH="$SYSTEM_PATH" HOME="/var/empty" TMPDIR="${TMPDIR:-/tmp}" \
    "$SYSTEM_RUBY" -e 'print File.expand_path(ARGV.fetch(0))' "$1"
}

canonical_path() {
  "$SYSTEM_ENV" -i PATH="$SYSTEM_PATH" HOME="/var/empty" TMPDIR="${TMPDIR:-/tmp}" \
    "$SYSTEM_RUBY" -e '
      cursor = File.expand_path(ARGV.fetch(0))
      suffix = []
      until File.exist?(cursor) || File.dirname(cursor) == cursor
        suffix.unshift(File.basename(cursor))
        cursor = File.dirname(cursor)
      end
      print File.join(File.realpath(cursor), *suffix)
    ' "$1"
}

paths_overlap() {
  "$SYSTEM_ENV" -i PATH="$SYSTEM_PATH" HOME="/var/empty" TMPDIR="${TMPDIR:-/tmp}" \
    "$SYSTEM_RUBY" -e '
      left = File.expand_path(ARGV.fetch(0))
      right = File.expand_path(ARGV.fetch(1))
      overlap = left == right || left.start_with?(right + File::SEPARATOR) || right.start_with?(left + File::SEPARATOR)
      exit(overlap ? 0 : 1)
    ' "$1" "$2"
}

normalize_trust_boundary_path() {
  local label="$1" path="$2" absolute canonical
  absolute="$(absolute_path "$path")"
  canonical="$(canonical_path "$absolute")"
  if [[ "$absolute" != "$canonical" ]]; then
    printf '%s path contains a symlinked component: %s\n' "$label" "$path" >&2
    exit 64
  fi
  printf '%s' "$canonical"
}

require_command jq

if [[ ! -s "$APP_BUNDLE_FINGERPRINT" ]]; then
  printf 'Missing app bundle fingerprint helper: %s\n' "$APP_BUNDLE_FINGERPRINT" >&2
  exit 1
fi
"$SYSTEM_ENV" -i \
  PATH="$SYSTEM_PATH" \
  HOME="/var/empty" \
  TMPDIR="${TMPDIR:-/tmp}" \
  "$SYSTEM_RUBY" -c "$APP_BUNDLE_FINGERPRINT" >/dev/null
if [[ ! -s "$FINDER_BOOKMARK_RESOLVER" ]]; then
  printf 'Missing Finder bookmark resolver: %s\n' "$FINDER_BOOKMARK_RESOLVER" >&2
  exit 1
fi
if [[ ! -s "$SOURCE_FINGERPRINT_HELPER" ]]; then
  printf 'Missing source fingerprint helper: %s\n' "$SOURCE_FINGERPRINT_HELPER" >&2
  exit 1
fi

READINESS_DIR="$(normalize_trust_boundary_path readiness "$READINESS_DIR")"
REPORT_PATH="$(normalize_trust_boundary_path report "$REPORT_PATH")"
INTAKE_DIR="$(normalize_trust_boundary_path intake "$INTAKE_DIR")"
if [[ -n "$ARTIFACT_INPUT_PATH" ]]; then
  if [[ "$ARTIFACT_INPUT_PATH" != /* ]]; then
    printf 'artifact input path must be absolute: %s\n' "$ARTIFACT_INPUT_PATH" >&2
    exit 64
  fi
  ARTIFACT_INPUT_PATH="$(normalize_trust_boundary_path artifact_input "$ARTIFACT_INPUT_PATH")"
fi
TEMPLATE_PATH="$INTAKE_DIR/release-artifact-template.json"
MARKDOWN_PATH="$INTAKE_DIR/release-artifact-intake.md"
ACCEPTED_ARTIFACT_INPUT_PATH="$INTAKE_DIR/release-artifact-input.accepted.json"
CAPTURED_ARTIFACT_INPUT_PATH="$INTAKE_DIR/release-artifact-input.captured.json"
READBACK_REPORT_PATH="$INTAKE_DIR/release-artifact-readback.json"
DISTRIBUTION_PREFLIGHT_REPORT_PATH="$READINESS_DIR/native-distribution-preflight-gate.json"
RELEASE_APPROVAL_INTAKE_REPORT_PATH="$READINESS_DIR/ui-release-approval-intake-gate.json"
RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH="$READINESS_DIR/ui-release-artifact-boundary-gate.json"
EVIDENCE_ARCHIVE_REPORT_PATH="$READINESS_DIR/ui-evidence-archive-gate.json"

if paths_overlap "$REPORT_PATH" "$INTAKE_DIR"; then
  printf 'release artifact report and intake paths must not overlap\n' >&2
  exit 64
fi
if [[ -n "$ARTIFACT_INPUT_PATH" ]] \
  && { paths_overlap "$ARTIFACT_INPUT_PATH" "$REPORT_PATH" || paths_overlap "$ARTIFACT_INPUT_PATH" "$INTAKE_DIR"; }; then
  printf 'release artifact input must not overlap report or intake paths\n' >&2
  exit 64
fi
for source_report_path in \
  "$DISTRIBUTION_PREFLIGHT_REPORT_PATH" \
  "$RELEASE_APPROVAL_INTAKE_REPORT_PATH" \
  "$RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH" \
  "$EVIDENCE_ARCHIVE_REPORT_PATH"; do
  if paths_overlap "$source_report_path" "$REPORT_PATH" \
    || paths_overlap "$source_report_path" "$INTAKE_DIR" \
    || { [[ -n "$ARTIFACT_INPUT_PATH" ]] && paths_overlap "$source_report_path" "$ARTIFACT_INPUT_PATH"; }; then
    printf 'release artifact source report collides with output or artifact input: %s\n' "$source_report_path" >&2
    exit 64
  fi
done
for fixed_output in \
  "$TEMPLATE_PATH" \
  "$MARKDOWN_PATH" \
  "$ACCEPTED_ARTIFACT_INPUT_PATH" \
  "$CAPTURED_ARTIFACT_INPUT_PATH" \
  "$READBACK_REPORT_PATH"; do
  if paths_overlap "$fixed_output" "$REPORT_PATH" \
    || { [[ -n "$ARTIFACT_INPUT_PATH" ]] && paths_overlap "$fixed_output" "$ARTIFACT_INPUT_PATH"; }; then
    printf 'release artifact fixed output collides with report or input: %s\n' "$fixed_output" >&2
    exit 64
  fi
  if [[ -L "$fixed_output" || ( -e "$fixed_output" && ! -f "$fixed_output" ) ]]; then
    printf 'release artifact fixed output is not a safe regular-file target: %s\n' "$fixed_output" >&2
    exit 64
  fi
done
if [[ -L "$REPORT_PATH" || ( -e "$REPORT_PATH" && ! -f "$REPORT_PATH" ) ]]; then
  printf 'release artifact report is not a safe regular-file target: %s\n' "$REPORT_PATH" >&2
  exit 64
fi
if [[ -e "$INTAKE_DIR" && ! -d "$INTAKE_DIR" ]]; then
  printf 'release artifact intake path is not a directory: %s\n' "$INTAKE_DIR" >&2
  exit 64
fi

require_report "$DISTRIBUTION_PREFLIGHT_REPORT_PATH"
require_report "$RELEASE_APPROVAL_INTAKE_REPORT_PATH"
require_report "$RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH"
require_report "$EVIDENCE_ARCHIVE_REPORT_PATH"

mkdir -p "$INTAKE_DIR"
if [[ "$(canonical_path "$INTAKE_DIR")" != "$INTAKE_DIR" || -L "$INTAKE_DIR" ]]; then
  printf 'release artifact intake path changed during initialization\n' >&2
  exit 64
fi

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-release-artifact-intake.XXXXXX")"
REPORT_TMP="$TMP_DIR/release-artifact-intake-report.json"
ARTIFACT_CAPTURE_PATH="$TMP_DIR/release-artifact-input.json"
READBACK_TMP="$TMP_DIR/release-artifact-readback.json"
TEMPLATE_TMP="$TMP_DIR/release-artifact-template.json"
MARKDOWN_TMP="$TMP_DIR/release-artifact-intake.md"
SNAPSHOT_DIR="$TMP_DIR/evidence-snapshots"
INTAKE_MOUNT_POINT=""
INTAKE_MOUNT_DEVICE_IDS=""
cleanup() {
  local attached_device detach_failed=false
  if [[ -n "$INTAKE_MOUNT_DEVICE_IDS" ]]; then
    while IFS= read -r attached_device; do
      [[ -n "$attached_device" ]] || continue
      "$SYSTEM_HDIUTIL" detach "$attached_device" -force >/dev/null 2>&1 || detach_failed=true
    done <<<"$INTAKE_MOUNT_DEVICE_IDS"
  fi
  if [[ "$detach_failed" == true || ( -z "$INTAKE_MOUNT_DEVICE_IDS" && -n "$INTAKE_MOUNT_POINT" ) ]]; then
    [[ -z "$INTAKE_MOUNT_POINT" ]] || "$SYSTEM_HDIUTIL" detach "$INTAKE_MOUNT_POINT" -force >/dev/null 2>&1 || true
  fi
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT INT TERM
mkdir -p "$SNAPSHOT_DIR"
SYSTEM_HOME="$TMP_DIR/system-home"
mkdir -p "$SYSTEM_HOME"

atomic_replace_from_file() {
  local source_path="$1" destination_path="$2" destination_dir temporary_path
  destination_dir="$(/usr/bin/dirname "$destination_path")"
  mkdir -p "$destination_dir"
  if [[ "$(canonical_path "$destination_dir")" != "$(absolute_path "$destination_dir")" || -L "$destination_dir" ]]; then
    printf 'release artifact output directory contains a symlinked component: %s\n' "$destination_dir" >&2
    return 1
  fi
  if [[ -L "$destination_path" || ( -e "$destination_path" && ! -f "$destination_path" ) ]]; then
    printf 'refusing unsafe release artifact output target: %s\n' "$destination_path" >&2
    return 1
  fi
  temporary_path="$(mktemp "$destination_dir/.hepta-release-artifact-intake.XXXXXX")"
  if ! /bin/cp "$source_path" "$temporary_path" \
    || [[ ! -f "$temporary_path" || -L "$temporary_path" ]]; then
    /bin/rm -f "$temporary_path"
    return 1
  fi
  if ! "$SYSTEM_ENV" -i PATH="$SYSTEM_PATH" HOME="/var/empty" TMPDIR="${TMPDIR:-/tmp}" \
    "$SYSTEM_RUBY" -e '
      source, destination = ARGV
      if File.symlink?(destination) || (File.exist?(destination) && !File.file?(destination))
        abort "unsafe destination"
      end
      File.rename(source, destination)
      abort "unsafe result" unless File.file?(destination) && !File.symlink?(destination)
    ' "$temporary_path" "$destination_path"; then
    /bin/rm -f "$temporary_path"
    return 1
  fi
}

remove_safe_regular_leaf() {
  local path="$1"
  "$SYSTEM_ENV" -i PATH="$SYSTEM_PATH" HOME=/var/empty TMPDIR=/private/tmp \
    "$SYSTEM_RUBY" -e '
      path = ARGV.fetch(0)
      stat = File.lstat(path) rescue nil
      exit 0 unless stat
      abort "unsafe leaf" unless stat.file? && !stat.symlink?
      File.unlink(path)
    ' "$path"
}

for system_tool in \
  "$SYSTEM_ENV" \
  "$SYSTEM_CODESIGN" \
  "$SYSTEM_DITTO" \
  "$SYSTEM_FIND" \
  "$SYSTEM_GIT" \
  "$SYSTEM_HDIUTIL" \
  "$SYSTEM_MOUNT" \
  "$SYSTEM_PLUTIL" \
  "$SYSTEM_RUBY" \
  "$SYSTEM_SHASUM" \
  "$SYSTEM_SPCTL" \
  "$SYSTEM_SWIFT" \
  "$SYSTEM_XCRUN"; do
  if [[ ! -x "$system_tool" || -L "$system_tool" ]]; then
    printf 'Missing canonical macOS system tool: %s\n' "$system_tool" >&2
    exit 2
  fi
done

artifact_present=false
artifact_input_path_json=null
artifact_captured_input_path_json=null
artifact_sha_json=null
artifact_bytes=0

if [[ -n "$ARTIFACT_INPUT_PATH" ]]; then
  require_report "$ARTIFACT_INPUT_PATH"
  /bin/cp "$ARTIFACT_INPUT_PATH" "$ARTIFACT_CAPTURE_PATH"
  atomic_replace_from_file "$ARTIFACT_CAPTURE_PATH" "$CAPTURED_ARTIFACT_INPUT_PATH"
  artifact_present=true
  artifact_input_path_json="$(jq -n --arg path "$ARTIFACT_INPUT_PATH" '$path')"
  artifact_captured_input_path_json="$(jq -n --arg path "$CAPTURED_ARTIFACT_INPUT_PATH" '$path')"
  artifact_sha_json="$(jq -n --arg sha "$(file_sha256 "$ARTIFACT_CAPTURE_PATH")" '$sha')"
  artifact_bytes="$(file_bytes "$ARTIFACT_CAPTURE_PATH")"
else
  jq -n '{present:false}' >"$ARTIFACT_CAPTURE_PATH"
fi

readback_blockers='[]'
add_readback_blocker() {
  readback_blockers="$(jq -c --arg blocker "$1" '. + [$blocker] | unique' <<<"$readback_blockers")"
}

run_isolated_system_tool() {
  "$SYSTEM_ENV" -i \
    PATH="$SYSTEM_PATH" \
    HOME="$SYSTEM_HOME" \
    TMPDIR="$TMP_DIR" \
    "$@"
}

# Reject every symlink and special entry without traversing it.  This check is
# required before ditto/codesign and is repeated on the private staged copy so
# signature removal can never follow attacker-selected bundle structure.
app_tree_no_follow_ready() {
  local root="$1"
  run_isolated_system_tool "$SYSTEM_RUBY" -e '
    root = File.expand_path(ARGV.fetch(0))
    abort "non-canonical app root" unless File.realpath(root) == root
    root_stat = File.lstat(root)
    abort "unsafe app root" unless root_stat.directory? && !root_stat.symlink?
    pending = [root]
    until pending.empty?
      directory = pending.pop
      directory_stat = File.lstat(directory)
      abort "directory changed or became unsafe" unless directory_stat.directory? && !directory_stat.symlink? && File.realpath(directory) == directory
      Dir.children(directory).sort.each do |name|
        abort "unsafe path component" if name == "." || name == ".." || name.include?(File::SEPARATOR)
        path = File.join(directory, name)
        stat = File.lstat(path)
        abort "symlink rejected" if stat.symlink?
        if stat.directory?
          pending << path
        elsif stat.file?
          flags = File::RDONLY
          flags |= File::NOFOLLOW if defined?(File::NOFOLLOW)
          File.open(path, flags) do |file|
            opened = file.stat
            abort "file changed during no-follow scan" unless opened.dev == stat.dev && opened.ino == stat.ino && opened.file?
          end
        else
          abort "unsupported app entry type"
        end
      end
    end
  ' "$root"
}

remove_staged_signature_metadata() {
  local output_app="$1"
  run_isolated_system_tool "$SYSTEM_RUBY" -e '
    root = File.realpath(ARGV.fetch(0))
    contents = File.join(root, "Contents")
    contents_stat = File.lstat(contents)
    abort "unsafe Contents directory" unless contents_stat.directory? && !contents_stat.symlink? && File.realpath(contents) == contents
    target = File.join(contents, "_CodeSignature")
    begin
      target_stat = File.lstat(target)
    rescue Errno::ENOENT
      exit 0
    end
    abort "unsafe signature metadata root" unless target_stat.directory? && !target_stat.symlink?
    pending = [[target, false]]
    until pending.empty?
      path, visited = pending.pop
      stat = File.lstat(path)
      abort "signature metadata symlink rejected" if stat.symlink?
      if stat.directory?
        if visited
          Dir.rmdir(path)
        else
          pending << [path, true]
          Dir.children(path).sort.reverse_each { |name| pending << [File.join(path, name), false] }
        end
      elsif stat.file?
        File.unlink(path)
      else
        abort "unsupported signature metadata entry"
      end
    end
  ' "$output_app"
}

capture_current_source_binding() {
  local output_path="$1"
  if run_isolated_system_tool "$SYSTEM_RUBY" "$SOURCE_FINGERPRINT_HELPER" >"$output_path" 2>"$output_path.stderr" \
    && jq -e '
      .schema_version == 1
      and .kind == "hepta-ui-source-binding"
      and ((.head // "") | test("^[0-9a-f]{40}$"))
      and ((.head_tree // "") | test("^[0-9a-f]{40}$"))
      and ((.source_fingerprint // "") | test("^[0-9a-f]{64}$"))
      and (.worktree_clean | type) == "boolean"
      and (.repository_worktree_clean | type) == "boolean"
    ' "$output_path" >/dev/null; then
    return 0
  fi
  printf '{}\n' >"$output_path"
  return 1
}

system_log_sha256() {
  local path="$1"
  if [[ -f "$path" ]]; then
    file_sha256 "$path"
  else
    printf ''
  fi
}

system_log_bytes() {
  local path="$1"
  if [[ -f "$path" ]]; then
    file_bytes "$path"
  else
    printf '0'
  fi
}

normalize_app_for_unsigned_equivalence() {
  local input_app="$1" output_app="$2" log_prefix="$3"
  local output_parent
  app_tree_no_follow_ready "$input_app" || return 1
  output_parent="$(/usr/bin/dirname "$output_app")"
  mkdir -p "$output_parent"
  "$SYSTEM_DITTO" "$input_app" "$output_app" || return 1
  [[ -d "$output_app" && ! -L "$output_app" ]] || return 1
  app_tree_no_follow_ready "$output_app" || return 1

  # The current Hepta release contract contains exactly one thin 64-bit Mach-O.
  # Unknown/fat/nested code is rejected until it has an equally strict normalizer.
  run_isolated_system_tool "$SYSTEM_RUBY" -e '
    root = File.realpath(ARGV.fetch(0))
    expected = "Contents/MacOS/hepta-native"
    known_macho_magics = [
      "\xcf\xfa\xed\xfe", "\xfe\xed\xfa\xcf",
      "\xce\xfa\xed\xfe", "\xfe\xed\xfa\xce",
      "\xca\xfe\xba\xbe", "\xbe\xba\xfe\xca",
      "\xca\xfe\xba\xbf", "\xbf\xba\xfe\xca"
    ].map(&:b)
    files = Dir.glob(File.join(root, "**", "*"), File::FNM_DOTMATCH).select do |path|
      File.file?(path) && !File.symlink?(path)
    end
    machos = files.select { |path| known_macho_magics.include?(File.binread(path, 4)) }
      .map { |path| path.delete_prefix(root + File::SEPARATOR) }.sort
    executables = files.select { |path| (File.stat(path).mode & 0o111) != 0 }
      .map { |path| path.delete_prefix(root + File::SEPARATOR) }.sort
    abort "unsupported Mach-O set: #{machos.inspect}" unless machos == [expected]
    abort "unsupported executable set: #{executables.inspect}" unless executables == [expected]
    abort "unsupported expected Mach-O format" unless File.binread(File.join(root, expected), 4) == "\xcf\xfa\xed\xfe".b
    signature_dirs = Dir.glob(File.join(root, "**", "_CodeSignature"), File::FNM_DOTMATCH).map do |path|
      abort "symlinked signature metadata" if File.symlink?(path)
      path.delete_prefix(root + File::SEPARATOR)
    end
    abort "unexpected signature metadata" unless (signature_dirs - ["Contents/_CodeSignature"]).empty?
  ' "$output_app" >"$log_prefix.structure.stdout" 2>"$log_prefix.structure.stderr" || return 1

  run_isolated_system_tool "$SYSTEM_CODESIGN" --remove-signature "$output_app" \
    >"$log_prefix.remove-bundle.stdout" 2>"$log_prefix.remove-bundle.stderr" || return 1
  run_isolated_system_tool "$SYSTEM_CODESIGN" --remove-signature "$output_app/Contents/MacOS/hepta-native" \
    >"$log_prefix.remove-binary.stdout" 2>"$log_prefix.remove-binary.stderr" || return 1
  app_tree_no_follow_ready "$output_app" || return 1
  remove_staged_signature_metadata "$output_app" || return 1
  app_tree_no_follow_ready "$output_app" || return 1

  run_isolated_system_tool "$SYSTEM_RUBY" -e '
    path = ARGV.fetch(0)
    bytes = File.binread(path)
    abort "unsupported non-thin-64 Mach-O" unless bytes.byteslice(0, 4) == "\xcf\xfa\xed\xfe".b
    abort "not an executable Mach-O" unless bytes.byteslice(12, 4).unpack1("V") == 2
    ncmds = bytes.byteslice(16, 4).unpack1("V")
    sizeofcmds = bytes.byteslice(20, 4).unpack1("V")
    abort "invalid load command count" unless ncmds.positive? && ncmds < 4096
    cursor = 32
    limit = cursor + sizeofcmds
    abort "invalid load command table" if limit > bytes.bytesize
    linkedit_count = 0
    ncmds.times do
      abort "truncated load command" if cursor + 8 > limit
      command, command_size = bytes.byteslice(cursor, 8).unpack("V2")
      abort "residual LC_CODE_SIGNATURE" if command == 0x1d
      abort "invalid load command size" if command_size < 8 || cursor + command_size > limit
      if command == 0x19 && bytes.byteslice(cursor + 8, 16).delete("\0") == "__LINKEDIT"
        abort "short __LINKEDIT command" if command_size < 72
        bytes[cursor + 32, 8] = "\0" * 8
        bytes[cursor + 48, 8] = "\0" * 8
        linkedit_count += 1
      end
      cursor += command_size
    end
    abort "invalid normalized Mach-O" unless cursor == limit && linkedit_count == 1
    File.binwrite(path, bytes)
  ' "$output_app/Contents/MacOS/hepta-native" \
    >"$log_prefix.normalize-macho.stdout" 2>"$log_prefix.normalize-macho.stderr" || return 1

  if "$SYSTEM_FIND" "$output_app" -name _CodeSignature -print | /usr/bin/grep -q .; then return 1; fi
  if run_isolated_system_tool "$SYSTEM_CODESIGN" -d "$output_app" >/dev/null 2>&1; then return 1; fi
  run_isolated_system_tool "$SYSTEM_RUBY" "$APP_BUNDLE_FINGERPRINT" "$output_app"
}

SNAPSHOT_SOURCE_INODES=""
snapshot_regular_file() {
  local source_path="$1" snapshot_path="$2" blocker="$3"
  local inode_key
  if [[ -z "$source_path" || "$source_path" != /* || ! -f "$source_path" || -L "$source_path" ]]; then
    add_readback_blocker "$blocker"
    return 1
  fi
  if ! inode_key="$(run_isolated_system_tool "$SYSTEM_RUBY" -e '
    source, destination = ARGV
    before = File.lstat(source)
    abort "unsafe source" unless before.file? && !before.symlink? && before.nlink == 1
    File.open(source, File::RDONLY | File::NOFOLLOW) do |input|
      opened = input.stat
      abort "source changed before open" unless opened.file? && opened.nlink == 1 && opened.dev == before.dev && opened.ino == before.ino
      File.open(destination, File::WRONLY | File::CREAT | File::EXCL, 0o600) do |output|
        IO.copy_stream(input, output)
        output.flush
        output.fsync
      end
      after = File.lstat(source)
      abort "source changed during snapshot" unless after.file? && !after.symlink? && after.nlink == 1 && after.dev == opened.dev && after.ino == opened.ino
      print "#{opened.dev}:#{opened.ino}"
    end
  ' "$source_path" "$snapshot_path" 2>/dev/null)" \
    || [[ ! -f "$snapshot_path" || -L "$snapshot_path" ]]; then
    add_readback_blocker "$blocker"
    return 1
  fi
  if printf '%s\n' "$SNAPSHOT_SOURCE_INODES" | /usr/bin/grep -Fxq "$inode_key"; then
    /bin/rm -f "$snapshot_path"
    add_readback_blocker "referenced_evidence_inode_reused"
    return 1
  fi
  SNAPSHOT_SOURCE_INODES="${SNAPSHOT_SOURCE_INODES}${SNAPSHOT_SOURCE_INODES:+$'\n'}${inode_key}"
  return 0
}

COMMAND_LOG_READBACK='{}'
capture_command_log() {
  local key="$1" path_query="$2" hash_query="$3" bytes_query="${4:-}"
  local source_path expected_sha expected_bytes snapshot_path actual_sha="" actual_bytes=0
  local path_ready=false hash_matches=false bytes_positive=false declared_bytes_match=true valid=false
  source_path="$(jq -r "$path_query // empty" "$ARTIFACT_CAPTURE_PATH")"
  expected_sha="$(jq -r "$hash_query // empty" "$ARTIFACT_CAPTURE_PATH")"
  expected_bytes=0
  if [[ -n "$bytes_query" ]]; then
    expected_bytes="$(jq -r "$bytes_query // 0" "$ARTIFACT_CAPTURE_PATH")"
  fi
  snapshot_path="$SNAPSHOT_DIR/$key"
  if snapshot_regular_file "$source_path" "$snapshot_path" "${key}_path_not_absolute_regular_file"; then
    path_ready=true
    actual_sha="$(file_sha256 "$snapshot_path")"
    actual_bytes="$(file_bytes "$snapshot_path")"
    if [[ "$actual_sha" == "$expected_sha" ]]; then hash_matches=true; fi
    if (( actual_bytes > 0 )); then bytes_positive=true; fi
    if [[ -n "$bytes_query" && "$actual_bytes" != "$expected_bytes" ]]; then declared_bytes_match=false; fi
  fi
  if [[ "$path_ready" == true && "$hash_matches" == true && "$bytes_positive" == true && "$declared_bytes_match" == true ]]; then
    valid=true
  else
    add_readback_blocker "${key}_readback_mismatch"
  fi
  COMMAND_LOG_READBACK="$(jq -c \
    --arg key "$key" \
    --arg path "$source_path" \
    --arg expected_sha256 "$expected_sha" \
    --arg actual_sha256 "$actual_sha" \
    --argjson expected_bytes "$expected_bytes" \
    --argjson actual_bytes "$actual_bytes" \
    --argjson path_ready "$path_ready" \
    --argjson hash_matches "$hash_matches" \
    --argjson bytes_positive "$bytes_positive" \
    --argjson declared_bytes_match "$declared_bytes_match" \
    --argjson valid "$valid" \
    '. + {($key):{path:$path,path_ready:$path_ready,expected_sha256:$expected_sha256,actual_sha256:$actual_sha256,expected_bytes:$expected_bytes,actual_bytes:$actual_bytes,hash_matches:$hash_matches,bytes_positive:$bytes_positive,declared_bytes_match:$declared_bytes_match,valid:$valid}}' \
    <<<"$COMMAND_LOG_READBACK")"
}

SOURCE_RECEIPT_SNAPSHOT="$SNAPSHOT_DIR/formal-unsigned-package-receipt.json"
SOURCE_APP_SNAPSHOT="$SNAPSHOT_DIR/source-app/Hepta.app"
SIGNED_DMG_SNAPSHOT="$SNAPSHOT_DIR/signed-artifact.dmg"
SOURCE_APP_FINGERPRINT_ACTUAL='null'
MOUNTED_APP_FINGERPRINT_ACTUAL='null'
SOURCE_BINARY_SHA_ACTUAL=""
MOUNTED_BINARY_SHA_ACTUAL=""
MOUNTED_BUNDLE_IDENTIFIER_ACTUAL=""
APPLICATIONS_ALIAS_KIND_ACTUAL=""
APPLICATIONS_ALIAS_TARGET_ACTUAL=""
APPLICATIONS_ALIAS_RESOLUTION_ACTUAL='null'
SOURCE_RECEIPT_SHA_ACTUAL=""
SOURCE_RECEIPT_BYTES_ACTUAL=0
SIGNED_DMG_SHA_ACTUAL=""
SIGNED_DMG_BYTES_ACTUAL=0
SOURCE_RECEIPT_READY=false
SOURCE_APP_READY=false
SOURCE_APP_TREE_NOFOLLOW_READY=false
MOUNTED_APP_TREE_NOFOLLOW_READY=false
SIGNED_DMG_READY=false
DMG_MOUNTED_READ_ONLY_ACTUAL=false
APPLICATIONS_ALIAS_VERIFIED_ACTUAL=false
REFERENCED_PATHS_UNIQUE=false
NOTARY_LOG_ACCEPTED=false
CURRENT_SOURCE_BINDING_BEFORE_PATH="$SNAPSHOT_DIR/current-source-binding.before.json"
CURRENT_SOURCE_BINDING_AFTER_PATH="$SNAPSHOT_DIR/current-source-binding.after.json"
printf '{}\n' >"$CURRENT_SOURCE_BINDING_BEFORE_PATH"
printf '{}\n' >"$CURRENT_SOURCE_BINDING_AFTER_PATH"
CURRENT_SOURCE_BINDING_CAPTURED=false
CURRENT_SOURCE_BINDING_STABLE=false
CURRENT_SOURCE_BINDING_MATCHES_RECEIPT=false
CURRENT_REPOSITORY_WORKTREE_CLEAN=false
SYSTEM_CODESIGN_DMG_VERIFIED=false
SYSTEM_STAPLER_DMG_VERIFIED=false
SYSTEM_SPCTL_DMG_VERIFIED=false
SYSTEM_CODESIGN_MOUNTED_APP_VERIFIED=false
SYSTEM_DMG_PREMOUNT_TRUSTED=false
SYSTEM_DMG_SIGNATURE_TUPLE_TRUSTED=false
SYSTEM_DEVELOPER_IDENTITY_VERIFIED=false
SYSTEM_VERIFICATION_VALID=false
SYSTEM_CODESIGN_DMG_EXIT_CODE=127
SYSTEM_STAPLER_DMG_EXIT_CODE=127
SYSTEM_SPCTL_DMG_EXIT_CODE=127
SYSTEM_CODESIGN_MOUNTED_APP_EXIT_CODE=127
SYSTEM_DMG_SIGNING_IDENTITY_ACTUAL=""
SYSTEM_MOUNTED_APP_SIGNING_IDENTITY_ACTUAL=""
DECLARED_SIGNING_IDENTITY=""
EXPECTED_SIGNING_IDENTITY=""
EXPECTED_TEAM_IDENTIFIER=""
DECLARED_TEAM_IDENTIFIER=""
SYSTEM_DMG_TEAM_IDENTIFIER_ACTUAL=""
SYSTEM_MOUNTED_APP_TEAM_IDENTIFIER_ACTUAL=""
SYSTEM_DMG_TIMESTAMP_ACTUAL=""
SYSTEM_MOUNTED_APP_TIMESTAMP_ACTUAL=""
SYSTEM_MOUNTED_APP_RUNTIME_VERSION_ACTUAL=""
SYSTEM_MOUNTED_APP_FLAGS_ACTUAL=""
SYSTEM_SIGNING_PROPERTIES_VERIFIED=false
SOURCE_APP_UNSIGNED_VERIFIED=false
NORMALIZED_SOURCE_APP_READY=false
NORMALIZED_MOUNTED_APP_READY=false
NORMALIZED_UNSIGNED_SIGNED_EQUIVALENT=false
NORMALIZED_SOURCE_APP_FINGERPRINT='null'
NORMALIZED_MOUNTED_APP_FINGERPRINT='null'
NORMALIZED_SOURCE_APP="$SNAPSHOT_DIR/normalized-source/Hepta.app"
NORMALIZED_MOUNTED_APP="$SNAPSHOT_DIR/normalized-mounted/Hepta.app"
SYSTEM_CODESIGN_DMG_LOG="$SNAPSHOT_DIR/system-codesign-verify-dmg.log"
SYSTEM_CODESIGN_DMG_DETAILS_LOG="$SNAPSHOT_DIR/system-codesign-details-dmg.log"
SYSTEM_STAPLER_DMG_LOG="$SNAPSHOT_DIR/system-stapler-validate-dmg.log"
SYSTEM_SPCTL_DMG_LOG="$SNAPSHOT_DIR/system-spctl-assess-dmg.log"
SYSTEM_CODESIGN_MOUNTED_APP_LOG="$SNAPSHOT_DIR/system-codesign-verify-mounted-app.log"
SYSTEM_CODESIGN_MOUNTED_APP_DETAILS_LOG="$SNAPSHOT_DIR/system-codesign-details-mounted-app.log"

if [[ "$artifact_present" == true ]]; then
  if capture_current_source_binding "$CURRENT_SOURCE_BINDING_BEFORE_PATH"; then
    CURRENT_SOURCE_BINDING_CAPTURED=true
    if jq -e '.worktree_clean == true and .repository_worktree_clean == true' "$CURRENT_SOURCE_BINDING_BEFORE_PATH" >/dev/null; then
      CURRENT_REPOSITORY_WORKTREE_CLEAN=true
    else
      add_readback_blocker "current_repository_worktree_not_clean"
    fi
  else
    add_readback_blocker "current_source_binding_capture_failed"
  fi

  if [[ "$ARTIFACT_INPUT_PATH" != /* || ! -f "$ARTIFACT_INPUT_PATH" || -L "$ARTIFACT_INPUT_PATH" ]]; then
    add_readback_blocker "artifact_receipt_input_path_not_absolute_regular_file"
  fi

  REFERENCED_PATHS_JSON="$(jq -ce --arg receipt_path "$ARTIFACT_INPUT_PATH" '
      [
        $receipt_path,
        .source_evidence.unsigned_package_receipt_path,
        .artifact_evidence.signed_artifact_path,
        .artifact_evidence.notarytool_submit_log_path,
        .artifact_evidence.codesign_verify_app_log_path,
        .artifact_evidence.codesign_verify_dmg_log_path,
        .artifact_evidence.stapler_staple_log_path,
        .artifact_evidence.stapler_validate_log_path,
        .artifact_evidence.spctl_assessment_log_path,
        .artifact_evidence.dmg_readonly_attach_path,
        .artifact_evidence.dmg_readonly_mount_log_path
      ]
      | if all(.[]; type == "string" and startswith("/") and length > 1) then . else error("invalid evidence path") end
    ' "$ARTIFACT_CAPTURE_PATH" 2>/dev/null || true)"
  if [[ -n "$REFERENCED_PATHS_JSON" ]] \
    && run_isolated_system_tool "$SYSTEM_RUBY" -r json -e '
      paths = JSON.parse(STDIN.read)
      stats = paths.map do |path|
        stat = File.lstat(path)
        abort "unsafe evidence file" unless stat.file? && !stat.symlink? && stat.nlink == 1
        [File.realpath(path), stat.dev, stat.ino]
      end
      realpaths = stats.map(&:first)
      inodes = stats.map { |entry| entry.drop(1) }
      exit(realpaths.uniq.length == realpaths.length && inodes.uniq.length == inodes.length ? 0 : 1)
    ' <<<"$REFERENCED_PATHS_JSON"; then
    REFERENCED_PATHS_UNIQUE=true
  else
    add_readback_blocker "referenced_evidence_paths_or_inodes_not_absolute_unique_single_link"
  fi

  SOURCE_RECEIPT_PATH="$(jq -r '.source_evidence.unsigned_package_receipt_path // empty' "$ARTIFACT_CAPTURE_PATH")"
  if snapshot_regular_file "$SOURCE_RECEIPT_PATH" "$SOURCE_RECEIPT_SNAPSHOT" "source_unsigned_receipt_path_not_absolute_regular_file"; then
    SOURCE_RECEIPT_SHA_ACTUAL="$(file_sha256 "$SOURCE_RECEIPT_SNAPSHOT")"
    SOURCE_RECEIPT_BYTES_ACTUAL="$(file_bytes "$SOURCE_RECEIPT_SNAPSHOT")"
    if jq empty "$SOURCE_RECEIPT_SNAPSHOT" >/dev/null \
      && [[ "$SOURCE_RECEIPT_SHA_ACTUAL" == "$(jq -r '.source_evidence.unsigned_package_receipt_sha256 // empty' "$ARTIFACT_CAPTURE_PATH")" ]]; then
      SOURCE_RECEIPT_READY=true
    else
      add_readback_blocker "source_unsigned_receipt_hash_or_json_mismatch"
    fi
  else
    printf '{}\n' >"$SOURCE_RECEIPT_SNAPSHOT"
  fi

  SOURCE_APP_PATH="$(jq -r '.source_evidence.source_app // empty' "$ARTIFACT_CAPTURE_PATH")"
  if [[ "$SOURCE_APP_PATH" == /* && -d "$SOURCE_APP_PATH" && ! -L "$SOURCE_APP_PATH" ]] \
    && app_tree_no_follow_ready "$SOURCE_APP_PATH"; then
    SOURCE_APP_TREE_NOFOLLOW_READY=true
    mkdir -p "$(dirname "$SOURCE_APP_SNAPSHOT")"
    if "$SYSTEM_DITTO" "$SOURCE_APP_PATH" "$SOURCE_APP_SNAPSHOT" \
      && app_tree_no_follow_ready "$SOURCE_APP_SNAPSHOT" \
      && SOURCE_APP_FINGERPRINT_ACTUAL="$(run_isolated_system_tool "$SYSTEM_RUBY" "$APP_BUNDLE_FINGERPRINT" "$SOURCE_APP_SNAPSHOT")" \
      && jq -e '.symlinks_rejected == true and .supported_entry_types_only == true' <<<"$SOURCE_APP_FINGERPRINT_ACTUAL" >/dev/null \
      && [[ -f "$SOURCE_APP_SNAPSHOT/Contents/MacOS/hepta-native" ]]; then
      SOURCE_BINARY_SHA_ACTUAL="$(file_sha256 "$SOURCE_APP_SNAPSHOT/Contents/MacOS/hepta-native")"
      SOURCE_APP_READY=true
    else
      add_readback_blocker "source_app_snapshot_or_fingerprint_failed"
    fi
  else
    add_readback_blocker "source_app_path_or_tree_not_absolute_canonical_nofollow_safe_directory"
  fi

  if [[ "$SOURCE_APP_READY" == true ]]; then
    SOURCE_SIGNATURE_DIR_COUNT="$("$SYSTEM_FIND" "$SOURCE_APP_SNAPSHOT" -name _CodeSignature -print | /usr/bin/wc -l | /usr/bin/tr -d ' ')"
    if ! run_isolated_system_tool "$SYSTEM_CODESIGN" -d "$SOURCE_APP_SNAPSHOT" >/dev/null 2>&1 \
      && [[ "$SOURCE_SIGNATURE_DIR_COUNT" == 0 ]]; then
      SOURCE_APP_UNSIGNED_VERIFIED=true
    else
      add_readback_blocker "source_app_not_strictly_unsigned"
    fi
    if NORMALIZED_SOURCE_APP_FINGERPRINT="$(normalize_app_for_unsigned_equivalence \
      "$SOURCE_APP_SNAPSHOT" "$NORMALIZED_SOURCE_APP" "$SNAPSHOT_DIR/normalize-source")"; then
      NORMALIZED_SOURCE_APP_READY=true
    else
      NORMALIZED_SOURCE_APP_FINGERPRINT='null'
      add_readback_blocker "source_app_signature_normalization_failed"
    fi
  else
    add_readback_blocker "source_app_signature_normalization_not_performed"
  fi

  SIGNED_DMG_PATH="$(jq -r '.artifact_evidence.signed_artifact_path // empty' "$ARTIFACT_CAPTURE_PATH")"
  if snapshot_regular_file "$SIGNED_DMG_PATH" "$SIGNED_DMG_SNAPSHOT" "signed_dmg_path_not_absolute_regular_file"; then
    SIGNED_DMG_SHA_ACTUAL="$(file_sha256 "$SIGNED_DMG_SNAPSHOT")"
    SIGNED_DMG_BYTES_ACTUAL="$(file_bytes "$SIGNED_DMG_SNAPSHOT")"
    if [[ "$SIGNED_DMG_SHA_ACTUAL" == "$(jq -r '.artifact_evidence.signed_artifact_sha256 // empty' "$ARTIFACT_CAPTURE_PATH")" \
      && "$SIGNED_DMG_BYTES_ACTUAL" == "$(jq -r '.artifact_evidence.signed_artifact_bytes // 0' "$ARTIFACT_CAPTURE_PATH")" \
      && "$SIGNED_DMG_BYTES_ACTUAL" -gt 0 ]]; then
      SIGNED_DMG_READY=true
    else
      add_readback_blocker "signed_dmg_hash_or_bytes_mismatch"
    fi
  fi

  capture_command_log notarytool_submit '.artifact_evidence.notarytool_submit_log_path' '.artifact_evidence.notarytool_submit_log_sha256' '.artifact_evidence.notarytool_submit_log_bytes'
  capture_command_log codesign_verify_app '.artifact_evidence.codesign_verify_app_log_path' '.artifact_evidence.codesign_verify_app_sha256'
  capture_command_log codesign_verify_dmg '.artifact_evidence.codesign_verify_dmg_log_path' '.artifact_evidence.codesign_verify_dmg_sha256'
  capture_command_log stapler_staple '.artifact_evidence.stapler_staple_log_path' '.artifact_evidence.stapler_staple_sha256'
  capture_command_log stapler_validate '.artifact_evidence.stapler_validate_log_path' '.artifact_evidence.stapler_validate_sha256'
  capture_command_log spctl_assessment '.artifact_evidence.spctl_assessment_log_path' '.artifact_evidence.spctl_assessment_sha256'
  capture_command_log dmg_readonly_attach '.artifact_evidence.dmg_readonly_attach_path' '.artifact_evidence.dmg_readonly_attach_sha256'
  capture_command_log dmg_readonly_mount '.artifact_evidence.dmg_readonly_mount_log_path' '.artifact_evidence.dmg_readonly_mount_sha256'

  if jq -e --arg submission_id "$(jq -r '.artifact_evidence.notary_submission_id // empty' "$ARTIFACT_CAPTURE_PATH")" \
    '.status == "Accepted" and (.id // "") == $submission_id and ($submission_id | length) > 0' \
    "$SNAPSHOT_DIR/notarytool_submit" >/dev/null 2>&1; then
    NOTARY_LOG_ACCEPTED=true
  else
    add_readback_blocker "notarytool_submit_log_not_accepted_or_id_mismatch"
  fi

  DECLARED_SIGNING_IDENTITY="$(jq -r '.artifact_evidence.signing_identity // empty' "$ARTIFACT_CAPTURE_PATH")"
  DECLARED_TEAM_IDENTIFIER="$(jq -r '.artifact_evidence.signing_team_identifier // empty' "$ARTIFACT_CAPTURE_PATH")"
  EXPECTED_TEAM_IDENTIFIER="${HEPTA_EXPECTED_TEAM_ID:-}"
  EXPECTED_SIGNING_IDENTITY="${HEPTA_EXPECTED_SIGNING_IDENTITY:-${HEPTA_SIGNING_IDENTITY:-}}"
  if [[ ! "$EXPECTED_TEAM_IDENTIFIER" =~ ^[A-Z0-9]{10}$ ]]; then
    add_readback_blocker "expected_team_identifier_missing_or_invalid"
  fi
  if [[ ! "$EXPECTED_SIGNING_IDENTITY" =~ ^Developer\ ID\ Application:\ .+\ \(([A-Z0-9]{10})\)$ ]] \
    || [[ "${BASH_REMATCH[1]:-}" != "$EXPECTED_TEAM_IDENTIFIER" ]]; then
    add_readback_blocker "expected_signing_identity_missing_invalid_or_team_mismatch"
  fi

  if [[ "$SIGNED_DMG_READY" == true ]]; then
    if run_isolated_system_tool "$SYSTEM_CODESIGN" --verify --strict --verbose=4 "$SIGNED_DMG_SNAPSHOT" >"$SYSTEM_CODESIGN_DMG_LOG" 2>&1; then
      SYSTEM_CODESIGN_DMG_EXIT_CODE=0
      SYSTEM_CODESIGN_DMG_VERIFIED=true
      if ! run_isolated_system_tool "$SYSTEM_CODESIGN" -d --verbose=4 "$SIGNED_DMG_SNAPSHOT" >"$SYSTEM_CODESIGN_DMG_DETAILS_LOG" 2>&1; then
        add_readback_blocker "system_codesign_dmg_identity_read_failed"
      fi
      SYSTEM_DMG_SIGNING_IDENTITY_ACTUAL="$(/usr/bin/awk -F= '/^Authority=Developer ID Application:/ {sub(/^[^=]*=/, ""); print; exit}' "$SYSTEM_CODESIGN_DMG_DETAILS_LOG")"
      SYSTEM_DMG_TEAM_IDENTIFIER_ACTUAL="$(/usr/bin/awk -F= '/^TeamIdentifier=/ {sub(/^[^=]*=/, ""); print; exit}' "$SYSTEM_CODESIGN_DMG_DETAILS_LOG")"
      SYSTEM_DMG_TIMESTAMP_ACTUAL="$(/usr/bin/awk -F= '/^Timestamp=/ {sub(/^[^=]*=/, ""); print; exit}' "$SYSTEM_CODESIGN_DMG_DETAILS_LOG")"
      if [[ "$EXPECTED_TEAM_IDENTIFIER" =~ ^[A-Z0-9]{10}$ \
        && "$EXPECTED_SIGNING_IDENTITY" =~ ^Developer\ ID\ Application:\ .+\ \(([A-Z0-9]{10})\)$ \
        && "${BASH_REMATCH[1]:-}" == "$EXPECTED_TEAM_IDENTIFIER" \
        && "$DECLARED_SIGNING_IDENTITY" == "$EXPECTED_SIGNING_IDENTITY" \
        && "$DECLARED_TEAM_IDENTIFIER" == "$EXPECTED_TEAM_IDENTIFIER" \
        && "$SYSTEM_DMG_SIGNING_IDENTITY_ACTUAL" == "$EXPECTED_SIGNING_IDENTITY" \
        && "$SYSTEM_DMG_TEAM_IDENTIFIER_ACTUAL" == "$EXPECTED_TEAM_IDENTIFIER" \
        && -n "$SYSTEM_DMG_TIMESTAMP_ACTUAL" \
        && "$SYSTEM_DMG_TIMESTAMP_ACTUAL" != "none" \
        && "$(jq -r '.artifact_evidence.codesign_dmg_timestamp // empty' "$ARTIFACT_CAPTURE_PATH")" == "$SYSTEM_DMG_TIMESTAMP_ACTUAL" ]]; then
        SYSTEM_DMG_SIGNATURE_TUPLE_TRUSTED=true
      else
        add_readback_blocker "system_dmg_premount_trusted_identity_team_or_timestamp_mismatch"
      fi
    else
      SYSTEM_CODESIGN_DMG_EXIT_CODE=$?
      add_readback_blocker "system_codesign_verify_dmg_failed"
    fi

    if run_isolated_system_tool "$SYSTEM_XCRUN" stapler validate "$SIGNED_DMG_SNAPSHOT" >"$SYSTEM_STAPLER_DMG_LOG" 2>&1; then
      SYSTEM_STAPLER_DMG_EXIT_CODE=0
      SYSTEM_STAPLER_DMG_VERIFIED=true
    else
      SYSTEM_STAPLER_DMG_EXIT_CODE=$?
      add_readback_blocker "system_stapler_validate_dmg_failed"
    fi

    if run_isolated_system_tool "$SYSTEM_SPCTL" --assess --type open --context context:primary-signature --verbose "$SIGNED_DMG_SNAPSHOT" >"$SYSTEM_SPCTL_DMG_LOG" 2>&1; then
      SYSTEM_SPCTL_DMG_EXIT_CODE=0
      SYSTEM_SPCTL_DMG_VERIFIED=true
    else
      SYSTEM_SPCTL_DMG_EXIT_CODE=$?
      add_readback_blocker "system_spctl_assess_dmg_failed"
    fi
    if [[ "$SYSTEM_DMG_SIGNATURE_TUPLE_TRUSTED" == true \
      && "$SYSTEM_STAPLER_DMG_VERIFIED" == true \
      && "$SYSTEM_SPCTL_DMG_VERIFIED" == true ]]; then
      SYSTEM_DMG_PREMOUNT_TRUSTED=true
    else
      add_readback_blocker "system_dmg_premount_apple_trust_not_established"
    fi
  else
    add_readback_blocker "system_signed_dmg_verification_not_performed"
  fi

  if [[ "$SIGNED_DMG_READY" == true && "$SYSTEM_DMG_PREMOUNT_TRUSTED" == true ]]; then
    ACTUAL_ATTACH_PLIST="$SNAPSHOT_DIR/intake-readonly-attach.plist"
    ACTUAL_ATTACH_STDERR="$SNAPSHOT_DIR/intake-readonly-attach.stderr"
    set +e
    run_isolated_system_tool "$SYSTEM_HDIUTIL" attach -readonly -nobrowse -noautoopen -plist "$SIGNED_DMG_SNAPSHOT" >"$ACTUAL_ATTACH_PLIST" 2>"$ACTUAL_ATTACH_STDERR"
    INTAKE_ATTACH_STATUS=$?
    set -e
    INTAKE_MOUNT_DEVICE_IDS="$(/usr/bin/grep -Eo '/dev/disk[0-9]+' "$ACTUAL_ATTACH_PLIST" | /usr/bin/sort -u || true)"
    if [[ "$INTAKE_ATTACH_STATUS" -eq 0 ]]; then
      INTAKE_MOUNT_POINTS="$(run_isolated_system_tool "$SYSTEM_RUBY" -r rexml/document -e '
        document = REXML::Document.new(File.binread(ARGV.fetch(0)))
        points = REXML::XPath.match(document, "//key").map do |item|
          item.next_element&.text.to_s if item.text == "mount-point"
        end.compact.reject(&:empty?).uniq
        puts points
      ' "$ACTUAL_ATTACH_PLIST" 2>/dev/null || true)"
      INTAKE_MOUNT_POINT="$(printf '%s\n' "$INTAKE_MOUNT_POINTS" | /usr/bin/sed -n '1p')"
      INTAKE_MOUNT_POINT_COUNT="$(printf '%s\n' "$INTAKE_MOUNT_POINTS" | /usr/bin/awk 'NF {count += 1} END {print count + 0}')"
      if [[ -z "$INTAKE_MOUNT_DEVICE_IDS" ]]; then
        add_readback_blocker "independent_dmg_attached_device_identifier_missing"
      fi
      if [[ "$INTAKE_MOUNT_POINT_COUNT" != 1 ]]; then
        add_readback_blocker "independent_dmg_mount_point_missing_or_ambiguous"
      fi
      ACTUAL_MOUNT_LOG="$SNAPSHOT_DIR/intake-readonly-mount.log"
      if [[ -n "$INTAKE_MOUNT_DEVICE_IDS" && "$INTAKE_MOUNT_POINT_COUNT" == 1 && -n "$INTAKE_MOUNT_POINT" && -d "$INTAKE_MOUNT_POINT" ]] \
        && "$SYSTEM_MOUNT" | /usr/bin/awk -v mount_point="$INTAKE_MOUNT_POINT" 'index($0, " on " mount_point " (") {print; found=1} END {exit(found ? 0 : 1)}' >"$ACTUAL_MOUNT_LOG" \
        && /usr/bin/grep -Eq '\(([^)]*,[[:space:]]*)?(read-only|rdonly)(,[^)]*)?\)' "$ACTUAL_MOUNT_LOG"; then
        DMG_MOUNTED_READ_ONLY_ACTUAL=true
      else
        add_readback_blocker "independent_dmg_mount_not_read_only"
      fi

      TOP_LEVEL_APP_COUNT="$("$SYSTEM_FIND" "$INTAKE_MOUNT_POINT" -mindepth 1 -maxdepth 1 -type d -name '*.app' -print 2>/dev/null | /usr/bin/wc -l | /usr/bin/tr -d ' ' || true)"
      if [[ "$TOP_LEVEL_APP_COUNT" == 1 && -d "$INTAKE_MOUNT_POINT/Hepta.app" \
        && -f "$INTAKE_MOUNT_POINT/Hepta.app/Contents/MacOS/hepta-native" \
        && -f "$INTAKE_MOUNT_POINT/Hepta.app/Contents/Info.plist" ]] \
        && app_tree_no_follow_ready "$INTAKE_MOUNT_POINT/Hepta.app"; then
        MOUNTED_APP_TREE_NOFOLLOW_READY=true
        if MOUNTED_APP_FINGERPRINT_ACTUAL="$(run_isolated_system_tool "$SYSTEM_RUBY" "$APP_BUNDLE_FINGERPRINT" "$INTAKE_MOUNT_POINT/Hepta.app" 2>/dev/null)"; then
          if jq -e '.symlinks_rejected == true and .supported_entry_types_only == true' <<<"$MOUNTED_APP_FINGERPRINT_ACTUAL" >/dev/null; then
            MOUNTED_BINARY_SHA_ACTUAL="$(file_sha256 "$INTAKE_MOUNT_POINT/Hepta.app/Contents/MacOS/hepta-native")"
            MOUNTED_BUNDLE_IDENTIFIER_ACTUAL="$(run_isolated_system_tool "$SYSTEM_PLUTIL" -extract CFBundleIdentifier raw "$INTAKE_MOUNT_POINT/Hepta.app/Contents/Info.plist" 2>/dev/null || true)"
          else
            MOUNTED_APP_FINGERPRINT_ACTUAL='null'
            add_readback_blocker "independent_dmg_bundle_tree_contains_symlink_or_unsupported_entry"
          fi
        else
          MOUNTED_APP_FINGERPRINT_ACTUAL='null'
          add_readback_blocker "independent_dmg_bundle_fingerprint_failed"
        fi
      else
        add_readback_blocker "independent_dmg_exact_hepta_app_missing"
      fi

      if [[ "$MOUNTED_APP_TREE_NOFOLLOW_READY" == true ]]; then
        if run_isolated_system_tool "$SYSTEM_CODESIGN" --verify --deep --strict --verbose=4 "$INTAKE_MOUNT_POINT/Hepta.app" >"$SYSTEM_CODESIGN_MOUNTED_APP_LOG" 2>&1; then
          SYSTEM_CODESIGN_MOUNTED_APP_EXIT_CODE=0
          SYSTEM_CODESIGN_MOUNTED_APP_VERIFIED=true
          if ! run_isolated_system_tool "$SYSTEM_CODESIGN" -d --verbose=4 "$INTAKE_MOUNT_POINT/Hepta.app" >"$SYSTEM_CODESIGN_MOUNTED_APP_DETAILS_LOG" 2>&1; then
            add_readback_blocker "system_codesign_mounted_app_identity_read_failed"
          fi
          SYSTEM_MOUNTED_APP_SIGNING_IDENTITY_ACTUAL="$(/usr/bin/awk -F= '/^Authority=Developer ID Application:/ {sub(/^[^=]*=/, ""); print; exit}' "$SYSTEM_CODESIGN_MOUNTED_APP_DETAILS_LOG")"
          SYSTEM_MOUNTED_APP_TEAM_IDENTIFIER_ACTUAL="$(/usr/bin/awk -F= '/^TeamIdentifier=/ {sub(/^[^=]*=/, ""); print; exit}' "$SYSTEM_CODESIGN_MOUNTED_APP_DETAILS_LOG")"
          SYSTEM_MOUNTED_APP_TIMESTAMP_ACTUAL="$(/usr/bin/awk -F= '/^Timestamp=/ {sub(/^[^=]*=/, ""); print; exit}' "$SYSTEM_CODESIGN_MOUNTED_APP_DETAILS_LOG")"
          SYSTEM_MOUNTED_APP_RUNTIME_VERSION_ACTUAL="$(/usr/bin/awk -F= '/^Runtime Version=/ {sub(/^[^=]*=/, ""); print; exit}' "$SYSTEM_CODESIGN_MOUNTED_APP_DETAILS_LOG")"
          SYSTEM_MOUNTED_APP_FLAGS_ACTUAL="$(/usr/bin/sed -n 's/.* flags=\([^ ]*\).*/\1/p' "$SYSTEM_CODESIGN_MOUNTED_APP_DETAILS_LOG" | /usr/bin/head -n 1)"
        else
          SYSTEM_CODESIGN_MOUNTED_APP_EXIT_CODE=$?
          add_readback_blocker "system_codesign_verify_mounted_app_failed"
        fi
      else
        add_readback_blocker "system_codesign_mounted_app_not_performed"
      fi

      if [[ "$MOUNTED_APP_TREE_NOFOLLOW_READY" == true ]]; then
        if NORMALIZED_MOUNTED_APP_FINGERPRINT="$(normalize_app_for_unsigned_equivalence \
          "$INTAKE_MOUNT_POINT/Hepta.app" "$NORMALIZED_MOUNTED_APP" "$SNAPSHOT_DIR/normalize-mounted")"; then
          NORMALIZED_MOUNTED_APP_READY=true
        else
          NORMALIZED_MOUNTED_APP_FINGERPRINT='null'
          add_readback_blocker "mounted_app_signature_normalization_failed"
        fi
      else
        add_readback_blocker "mounted_app_signature_normalization_not_performed"
      fi

      if [[ "$NORMALIZED_SOURCE_APP_READY" == true \
        && "$NORMALIZED_MOUNTED_APP_READY" == true \
        && "$NORMALIZED_SOURCE_APP_FINGERPRINT" == "$NORMALIZED_MOUNTED_APP_FINGERPRINT" ]]; then
        NORMALIZED_UNSIGNED_SIGNED_EQUIVALENT=true
      else
        add_readback_blocker "unsigned_and_signed_bundle_content_mismatch"
      fi

      APPLICATIONS_ENTRY="$INTAKE_MOUNT_POINT/Applications"
      APPLICATIONS_ALIAS_RESOLUTION_PATH="$SNAPSHOT_DIR/applications-alias-resolution.json"
      APPLICATIONS_ALIAS_RESOLUTION_STDERR_PATH="$SNAPSHOT_DIR/applications-alias-resolution.stderr.log"
      if [[ -f "$APPLICATIONS_ENTRY" && ! -L "$APPLICATIONS_ENTRY" ]] \
        && run_isolated_system_tool "$SYSTEM_SWIFT" "$FINDER_BOOKMARK_RESOLVER" "$APPLICATIONS_ENTRY" \
          >"$APPLICATIONS_ALIAS_RESOLUTION_PATH" \
          2>"$APPLICATIONS_ALIAS_RESOLUTION_STDERR_PATH" \
        && jq -e '
          .schema_version == 1
          and .kind == "hepta-finder-bookmark-resolution"
          and .bookmark_data_stale == false
          and .resolved_target == "/Applications"
        ' "$APPLICATIONS_ALIAS_RESOLUTION_PATH" >/dev/null; then
        APPLICATIONS_ALIAS_RESOLUTION_ACTUAL="$(jq -c . "$APPLICATIONS_ALIAS_RESOLUTION_PATH")"
        APPLICATIONS_ALIAS_TARGET_ACTUAL="$(jq -r '.resolved_target' "$APPLICATIONS_ALIAS_RESOLUTION_PATH")"
        APPLICATIONS_ALIAS_KIND_ACTUAL="finder_bookmark_alias"
      else
        add_readback_blocker "independent_dmg_applications_alias_resolution_failed"
      fi
      if [[ "$APPLICATIONS_ALIAS_TARGET_ACTUAL" == "/Applications" ]]; then
        APPLICATIONS_ALIAS_VERIFIED_ACTUAL=true
      else
        add_readback_blocker "independent_dmg_applications_alias_target_mismatch"
      fi

      INTAKE_DETACH_FAILED=false
      while IFS= read -r attached_device; do
        [[ -n "$attached_device" ]] || continue
        "$SYSTEM_HDIUTIL" detach "$attached_device" >/dev/null 2>&1 || INTAKE_DETACH_FAILED=true
      done <<<"$INTAKE_MOUNT_DEVICE_IDS"
      if [[ "$INTAKE_DETACH_FAILED" == false && -n "$INTAKE_MOUNT_DEVICE_IDS" ]]; then
        INTAKE_MOUNT_POINT=""
        INTAKE_MOUNT_DEVICE_IDS=""
      else
        add_readback_blocker "independent_dmg_detach_failed"
      fi
    else
      add_readback_blocker "independent_dmg_readonly_attach_failed"
      add_readback_blocker "system_codesign_mounted_app_not_performed"
    fi
  else
    add_readback_blocker "independent_dmg_readonly_attach_blocked_until_trusted_system_signature"
    add_readback_blocker "system_codesign_mounted_app_not_performed"
  fi

  if [[ "$EXPECTED_SIGNING_IDENTITY" =~ ^Developer\ ID\ Application:\ .+\ \(([A-Z0-9]{10})\)$ \
    && "${BASH_REMATCH[1]:-}" == "$EXPECTED_TEAM_IDENTIFIER" \
    && "$DECLARED_SIGNING_IDENTITY" == "$EXPECTED_SIGNING_IDENTITY" \
    && "$SYSTEM_DMG_SIGNING_IDENTITY_ACTUAL" == "$EXPECTED_SIGNING_IDENTITY" \
    && "$SYSTEM_MOUNTED_APP_SIGNING_IDENTITY_ACTUAL" == "$EXPECTED_SIGNING_IDENTITY" ]]; then
    SYSTEM_DEVELOPER_IDENTITY_VERIFIED=true
  else
    add_readback_blocker "system_developer_id_identity_mismatch"
  fi

  if [[ "$EXPECTED_TEAM_IDENTIFIER" =~ ^[A-Z0-9]{10}$ \
    && "$DECLARED_TEAM_IDENTIFIER" == "$EXPECTED_TEAM_IDENTIFIER" \
    && "$SYSTEM_DMG_TEAM_IDENTIFIER_ACTUAL" == "$EXPECTED_TEAM_IDENTIFIER" \
    && "$SYSTEM_MOUNTED_APP_TEAM_IDENTIFIER_ACTUAL" == "$EXPECTED_TEAM_IDENTIFIER" \
    && -n "$SYSTEM_DMG_TIMESTAMP_ACTUAL" \
    && "$SYSTEM_DMG_TIMESTAMP_ACTUAL" != "none" \
    && -n "$SYSTEM_MOUNTED_APP_TIMESTAMP_ACTUAL" \
    && "$SYSTEM_MOUNTED_APP_TIMESTAMP_ACTUAL" != "none" \
    && -n "$SYSTEM_MOUNTED_APP_RUNTIME_VERSION_ACTUAL" \
    && "$SYSTEM_MOUNTED_APP_FLAGS_ACTUAL" == *runtime* \
    && "$(jq -r '.artifact_evidence.codesign_app_runtime_version // empty' "$ARTIFACT_CAPTURE_PATH")" == "$SYSTEM_MOUNTED_APP_RUNTIME_VERSION_ACTUAL" \
    && "$(jq -r '.artifact_evidence.codesign_app_flags // empty' "$ARTIFACT_CAPTURE_PATH")" == "$SYSTEM_MOUNTED_APP_FLAGS_ACTUAL" \
    && "$(jq -r '.artifact_evidence.codesign_app_timestamp // empty' "$ARTIFACT_CAPTURE_PATH")" == "$SYSTEM_MOUNTED_APP_TIMESTAMP_ACTUAL" \
    && "$(jq -r '.artifact_evidence.codesign_dmg_timestamp // empty' "$ARTIFACT_CAPTURE_PATH")" == "$SYSTEM_DMG_TIMESTAMP_ACTUAL" ]]; then
    SYSTEM_SIGNING_PROPERTIES_VERIFIED=true
  else
    add_readback_blocker "system_codesign_properties_or_team_mismatch"
  fi

  if [[ "$SYSTEM_CODESIGN_DMG_VERIFIED" == true \
    && "$SYSTEM_STAPLER_DMG_VERIFIED" == true \
    && "$SYSTEM_SPCTL_DMG_VERIFIED" == true \
    && "$SYSTEM_CODESIGN_MOUNTED_APP_VERIFIED" == true \
    && "$SYSTEM_DEVELOPER_IDENTITY_VERIFIED" == true \
    && "$SYSTEM_SIGNING_PROPERTIES_VERIFIED" == true \
    && "$SOURCE_APP_UNSIGNED_VERIFIED" == true \
    && "$NORMALIZED_UNSIGNED_SIGNED_EQUIVALENT" == true ]]; then
    SYSTEM_VERIFICATION_VALID=true
  else
    add_readback_blocker "independent_system_verification_failed"
  fi

  if capture_current_source_binding "$CURRENT_SOURCE_BINDING_AFTER_PATH"; then
    if jq -ne --slurpfile before "$CURRENT_SOURCE_BINDING_BEFORE_PATH" --slurpfile after "$CURRENT_SOURCE_BINDING_AFTER_PATH" '
      ($before[0]) as $before_binding
      | ($after[0]) as $after_binding
      | $before_binding.head == $after_binding.head
        and $before_binding.head_tree == $after_binding.head_tree
        and $before_binding.source_fingerprint == $after_binding.source_fingerprint
        and $before_binding.worktree_clean == $after_binding.worktree_clean
        and $before_binding.repository_worktree_clean == $after_binding.repository_worktree_clean
    ' >/dev/null; then
      CURRENT_SOURCE_BINDING_STABLE=true
    else
      add_readback_blocker "current_source_binding_changed_during_readback"
    fi
  else
    add_readback_blocker "current_source_binding_recapture_failed"
  fi

  if [[ "$CURRENT_SOURCE_BINDING_CAPTURED" == true \
    && "$CURRENT_REPOSITORY_WORKTREE_CLEAN" == true \
    && "$CURRENT_SOURCE_BINDING_STABLE" == true \
    && "$SOURCE_RECEIPT_READY" == true ]] \
    && jq -ne \
      --slurpfile current "$CURRENT_SOURCE_BINDING_BEFORE_PATH" \
      --slurpfile source_receipt "$SOURCE_RECEIPT_SNAPSHOT" \
      --slurpfile artifact "$ARTIFACT_CAPTURE_PATH" '
        ($current[0]) as $binding
        | ($source_receipt[0]) as $unsigned
        | ($artifact[0]) as $release
        | $binding.worktree_clean == true
          and $binding.repository_worktree_clean == true
          and $release.source_evidence.source_worktree_clean == true
          and $release.source_evidence.source_head == $binding.head
          and $release.source_evidence.source_tree == $binding.head_tree
          and $release.source_evidence.source_fingerprint == $binding.source_fingerprint
          and $unsigned.source_binding.head == $binding.head
          and $unsigned.source_binding.head_tree == $binding.head_tree
          and $unsigned.source_binding.source_fingerprint == $binding.source_fingerprint
          and $unsigned.source_binding.worktree_clean == true
          and $unsigned.source_binding.repository_worktree_clean == true
          and $unsigned.repository_worktree_clean == true
      ' >/dev/null; then
    CURRENT_SOURCE_BINDING_MATCHES_RECEIPT=true
  else
    add_readback_blocker "source_receipt_not_bound_to_current_repository"
  fi
fi

if [[ "$artifact_present" == false ]]; then
  printf '{}\n' >"$SOURCE_RECEIPT_SNAPSHOT"
fi

jq -n \
  --argjson artifact_present "$artifact_present" \
  --argjson blockers "$readback_blockers" \
  --argjson command_logs "$COMMAND_LOG_READBACK" \
  --arg source_receipt_sha256 "$SOURCE_RECEIPT_SHA_ACTUAL" \
  --argjson source_receipt_bytes "$SOURCE_RECEIPT_BYTES_ACTUAL" \
  --argjson source_receipt_ready "$SOURCE_RECEIPT_READY" \
  --argjson source_app_ready "$SOURCE_APP_READY" \
  --argjson source_app_tree_nofollow_ready "$SOURCE_APP_TREE_NOFOLLOW_READY" \
  --argjson mounted_app_tree_nofollow_ready "$MOUNTED_APP_TREE_NOFOLLOW_READY" \
  --argjson source_app_fingerprint_actual "$SOURCE_APP_FINGERPRINT_ACTUAL" \
  --arg source_binary_sha256_actual "$SOURCE_BINARY_SHA_ACTUAL" \
  --argjson signed_dmg_ready "$SIGNED_DMG_READY" \
  --arg signed_dmg_sha256_actual "$SIGNED_DMG_SHA_ACTUAL" \
  --argjson signed_dmg_bytes_actual "$SIGNED_DMG_BYTES_ACTUAL" \
  --argjson referenced_paths_unique "$REFERENCED_PATHS_UNIQUE" \
  --argjson notary_log_accepted "$NOTARY_LOG_ACCEPTED" \
  --argjson dmg_mounted_read_only_actual "$DMG_MOUNTED_READ_ONLY_ACTUAL" \
  --argjson mounted_app_fingerprint_actual "$MOUNTED_APP_FINGERPRINT_ACTUAL" \
  --arg mounted_binary_sha256_actual "$MOUNTED_BINARY_SHA_ACTUAL" \
  --arg mounted_bundle_identifier_actual "$MOUNTED_BUNDLE_IDENTIFIER_ACTUAL" \
  --argjson applications_alias_verified_actual "$APPLICATIONS_ALIAS_VERIFIED_ACTUAL" \
  --arg applications_alias_kind_actual "$APPLICATIONS_ALIAS_KIND_ACTUAL" \
  --arg applications_alias_target_actual "$APPLICATIONS_ALIAS_TARGET_ACTUAL" \
  --argjson applications_alias_resolution_actual "$APPLICATIONS_ALIAS_RESOLUTION_ACTUAL" \
  --argjson current_source_binding_captured "$CURRENT_SOURCE_BINDING_CAPTURED" \
  --argjson current_source_binding_stable "$CURRENT_SOURCE_BINDING_STABLE" \
  --argjson current_source_binding_matches_receipt "$CURRENT_SOURCE_BINDING_MATCHES_RECEIPT" \
  --argjson current_repository_worktree_clean "$CURRENT_REPOSITORY_WORKTREE_CLEAN" \
  --argjson system_codesign_dmg_verified "$SYSTEM_CODESIGN_DMG_VERIFIED" \
  --argjson system_stapler_dmg_verified "$SYSTEM_STAPLER_DMG_VERIFIED" \
  --argjson system_spctl_dmg_verified "$SYSTEM_SPCTL_DMG_VERIFIED" \
  --argjson system_codesign_mounted_app_verified "$SYSTEM_CODESIGN_MOUNTED_APP_VERIFIED" \
  --argjson system_dmg_premount_trusted "$SYSTEM_DMG_PREMOUNT_TRUSTED" \
  --argjson system_dmg_signature_tuple_trusted "$SYSTEM_DMG_SIGNATURE_TUPLE_TRUSTED" \
  --argjson system_developer_identity_verified "$SYSTEM_DEVELOPER_IDENTITY_VERIFIED" \
  --argjson system_signing_properties_verified "$SYSTEM_SIGNING_PROPERTIES_VERIFIED" \
  --argjson system_verification_valid "$SYSTEM_VERIFICATION_VALID" \
  --argjson source_app_unsigned_verified "$SOURCE_APP_UNSIGNED_VERIFIED" \
  --argjson normalized_source_app_ready "$NORMALIZED_SOURCE_APP_READY" \
  --argjson normalized_mounted_app_ready "$NORMALIZED_MOUNTED_APP_READY" \
  --argjson normalized_unsigned_signed_equivalent "$NORMALIZED_UNSIGNED_SIGNED_EQUIVALENT" \
  --argjson normalized_source_app_fingerprint "$NORMALIZED_SOURCE_APP_FINGERPRINT" \
  --argjson normalized_mounted_app_fingerprint "$NORMALIZED_MOUNTED_APP_FINGERPRINT" \
  --argjson system_codesign_dmg_exit_code "$SYSTEM_CODESIGN_DMG_EXIT_CODE" \
  --argjson system_stapler_dmg_exit_code "$SYSTEM_STAPLER_DMG_EXIT_CODE" \
  --argjson system_spctl_dmg_exit_code "$SYSTEM_SPCTL_DMG_EXIT_CODE" \
  --argjson system_codesign_mounted_app_exit_code "$SYSTEM_CODESIGN_MOUNTED_APP_EXIT_CODE" \
  --arg declared_signing_identity "$DECLARED_SIGNING_IDENTITY" \
  --arg expected_signing_identity "$EXPECTED_SIGNING_IDENTITY" \
  --arg system_dmg_signing_identity_actual "$SYSTEM_DMG_SIGNING_IDENTITY_ACTUAL" \
  --arg system_mounted_app_signing_identity_actual "$SYSTEM_MOUNTED_APP_SIGNING_IDENTITY_ACTUAL" \
  --arg expected_team_identifier "$EXPECTED_TEAM_IDENTIFIER" \
  --arg declared_team_identifier "$DECLARED_TEAM_IDENTIFIER" \
  --arg system_dmg_team_identifier_actual "$SYSTEM_DMG_TEAM_IDENTIFIER_ACTUAL" \
  --arg system_mounted_app_team_identifier_actual "$SYSTEM_MOUNTED_APP_TEAM_IDENTIFIER_ACTUAL" \
  --arg system_dmg_timestamp_actual "$SYSTEM_DMG_TIMESTAMP_ACTUAL" \
  --arg system_mounted_app_timestamp_actual "$SYSTEM_MOUNTED_APP_TIMESTAMP_ACTUAL" \
  --arg system_mounted_app_runtime_version_actual "$SYSTEM_MOUNTED_APP_RUNTIME_VERSION_ACTUAL" \
  --arg system_mounted_app_flags_actual "$SYSTEM_MOUNTED_APP_FLAGS_ACTUAL" \
  --arg system_codesign_dmg_log_sha256 "$(system_log_sha256 "$SYSTEM_CODESIGN_DMG_LOG")" \
  --argjson system_codesign_dmg_log_bytes "$(system_log_bytes "$SYSTEM_CODESIGN_DMG_LOG")" \
  --arg system_stapler_dmg_log_sha256 "$(system_log_sha256 "$SYSTEM_STAPLER_DMG_LOG")" \
  --argjson system_stapler_dmg_log_bytes "$(system_log_bytes "$SYSTEM_STAPLER_DMG_LOG")" \
  --arg system_spctl_dmg_log_sha256 "$(system_log_sha256 "$SYSTEM_SPCTL_DMG_LOG")" \
  --argjson system_spctl_dmg_log_bytes "$(system_log_bytes "$SYSTEM_SPCTL_DMG_LOG")" \
  --arg system_codesign_mounted_app_log_sha256 "$(system_log_sha256 "$SYSTEM_CODESIGN_MOUNTED_APP_LOG")" \
  --argjson system_codesign_mounted_app_log_bytes "$(system_log_bytes "$SYSTEM_CODESIGN_MOUNTED_APP_LOG")" \
  --slurpfile receipt_file "$ARTIFACT_CAPTURE_PATH" \
  --slurpfile source_receipt_file "$SOURCE_RECEIPT_SNAPSHOT" \
  --slurpfile current_source_binding_before_file "$CURRENT_SOURCE_BINDING_BEFORE_PATH" \
  --slurpfile current_source_binding_after_file "$CURRENT_SOURCE_BINDING_AFTER_PATH" \
  '
  ($receipt_file[0]) as $receipt
  | ($source_receipt_file[0]) as $source_receipt
  | ($current_source_binding_before_file[0]) as $current_source_binding_before
  | ($current_source_binding_after_file[0]) as $current_source_binding_after
  | def sha_ready($value): (($value // "") | test("^[0-9a-f]{64}$"));
    def bundle_fingerprint_ready($value):
      $value.schema_version == 1
      and $value.kind == "hepta-app-bundle-fingerprint"
      and ($value.entry_count | type) == "number" and $value.entry_count > 0
      and ($value.file_count | type) == "number" and $value.file_count > 0
      and ($value.directory_count | type) == "number" and $value.directory_count > 0
      and $value.symlink_count == 0
      and $value.unsupported_entry_count == 0
      and ($value.file_bytes | type) == "number" and $value.file_bytes > 0
      and sha_ready($value.manifest_sha256)
      and $value.symlinks_rejected == true
      and $value.supported_entry_types_only == true;
    def every_command_log_valid:
      ($command_logs | length) == 8
      and all($command_logs[]; .valid == true);
    def source_receipt_contract_matches:
      $source_receipt_ready == true
      and $source_receipt.schema_version == 1
      and $source_receipt.kind == "hepta-native-current-package-gate"
      and $source_receipt.status == "ready"
      and $source_receipt.local_package_ready == true
      and $source_receipt.signed == false
      and $source_receipt.notarized == false
      and $source_receipt.stapled == false
      and $source_receipt.artifact.path == $receipt.source_evidence.source_app
      and $source_receipt.artifact.binary_sha256 == $source_binary_sha256_actual
      and $source_receipt.artifact.bundle_fingerprint == $source_app_fingerprint_actual
      and $source_receipt.artifact.full_head_embedded == true
      and $source_receipt.artifact.developer_id_signed == false
      and $source_receipt.source_binding.head == $receipt.source_evidence.source_head
      and $source_receipt.source_binding.head_tree == $receipt.source_evidence.source_tree
      and $source_receipt.source_binding.source_fingerprint == $receipt.source_evidence.source_fingerprint
      and $source_receipt.source_binding.worktree_clean == true
      and $source_receipt.source_binding.repository_worktree_clean == true
      and $source_receipt.repository_worktree_clean == true
      and $source_receipt.source_stable_during_run == true
      and $current_source_binding_captured == true
      and $current_source_binding_stable == true
      and $current_source_binding_matches_receipt == true
      and $current_repository_worktree_clean == true
      and $current_source_binding_before.worktree_clean == true
      and $current_source_binding_before.repository_worktree_clean == true
      and $current_source_binding_before.head == $receipt.source_evidence.source_head
      and $current_source_binding_before.head_tree == $receipt.source_evidence.source_tree
      and $current_source_binding_before.source_fingerprint == $receipt.source_evidence.source_fingerprint
      and $current_source_binding_after.head == $current_source_binding_before.head
      and $current_source_binding_after.head_tree == $current_source_binding_before.head_tree
      and $current_source_binding_after.source_fingerprint == $current_source_binding_before.source_fingerprint;
    (
      $artifact_present == true
      and ($blockers | length) == 0
      and $referenced_paths_unique == true
      and every_command_log_valid
      and $notary_log_accepted == true
      and $command_logs.notarytool_submit.actual_sha256 == $receipt.artifact_evidence.notarization_ticket_sha256
      and $source_receipt_sha256 == $receipt.source_evidence.unsigned_package_receipt_sha256
      and $source_receipt_bytes > 0
      and source_receipt_contract_matches
      and $source_app_ready == true
      and $source_app_tree_nofollow_ready == true
      and bundle_fingerprint_ready($source_app_fingerprint_actual)
      and $source_app_fingerprint_actual == $receipt.source_evidence.source_app_bundle_fingerprint
      and $source_binary_sha256_actual == $receipt.source_evidence.source_binary_sha256
      and $signed_dmg_ready == true
      and $signed_dmg_sha256_actual == $receipt.artifact_evidence.signed_artifact_sha256
      and $signed_dmg_bytes_actual == $receipt.artifact_evidence.signed_artifact_bytes
      and $system_verification_valid == true
      and $system_dmg_signature_tuple_trusted == true
      and $system_dmg_premount_trusted == true
      and $system_codesign_dmg_verified == true
      and $system_stapler_dmg_verified == true
      and $system_spctl_dmg_verified == true
      and $system_codesign_mounted_app_verified == true
      and $system_developer_identity_verified == true
      and $system_signing_properties_verified == true
      and $source_app_unsigned_verified == true
      and $normalized_source_app_ready == true
      and $normalized_mounted_app_ready == true
      and $normalized_unsigned_signed_equivalent == true
      and $normalized_source_app_fingerprint == $normalized_mounted_app_fingerprint
      and $declared_signing_identity == $receipt.artifact_evidence.signing_identity
      and $system_dmg_signing_identity_actual == $declared_signing_identity
      and $system_mounted_app_signing_identity_actual == $declared_signing_identity
      and $declared_signing_identity == $expected_signing_identity
      and $mounted_app_tree_nofollow_ready == true
      and $dmg_mounted_read_only_actual == true
      and $receipt.artifact_evidence.dmg_mounted_read_only == true
      and bundle_fingerprint_ready($mounted_app_fingerprint_actual)
      and $mounted_app_fingerprint_actual == $receipt.source_evidence.signed_app_bundle_fingerprint
      and $mounted_app_fingerprint_actual == $receipt.artifact_evidence.mounted_app_bundle_fingerprint
      and $mounted_binary_sha256_actual == $receipt.source_evidence.signed_binary_sha256
      and $mounted_binary_sha256_actual == $receipt.artifact_evidence.mounted_binary_sha256
      and $mounted_bundle_identifier_actual == "ai.hepta.nativeapp"
      and $mounted_bundle_identifier_actual == $receipt.artifact_evidence.mounted_bundle_identifier
      and $applications_alias_resolution_actual.schema_version == 1
      and $applications_alias_resolution_actual.kind == "hepta-finder-bookmark-resolution"
      and $applications_alias_resolution_actual.bookmark_data_stale == false
      and $applications_alias_resolution_actual.resolved_target == "/Applications"
      and $applications_alias_verified_actual == true
      and $receipt.artifact_evidence.applications_alias_verified == true
      and $applications_alias_kind_actual == $receipt.artifact_evidence.applications_alias_kind
      and $applications_alias_target_actual == "/Applications"
      and $receipt.artifact_evidence.applications_alias_resolved_target == "/Applications"
    ) as $all_evidence_valid
  | {
      schema_version:1,
      kind:"hepta-ui-release-artifact-intake-readback-v3",
      receipt_contract_version:3,
      performed:$artifact_present,
      all_evidence_valid:$all_evidence_valid,
      blockers:$blockers,
      referenced_paths_absolute_and_unique:$referenced_paths_unique,
      signed_artifact:{snapshot_performed:$signed_dmg_ready,actual_sha256:$signed_dmg_sha256_actual,actual_bytes:$signed_dmg_bytes_actual},
      command_logs:$command_logs,
      source_unsigned_receipt:{snapshot_performed:$source_receipt_ready,actual_sha256:$source_receipt_sha256,actual_bytes:$source_receipt_bytes,contract_matches:source_receipt_contract_matches},
      current_source_binding:{captured:$current_source_binding_captured,stable_during_readback:$current_source_binding_stable,current_repository_worktree_clean:$current_repository_worktree_clean,matches_release_and_unsigned_receipts:$current_source_binding_matches_receipt,before:$current_source_binding_before,after:$current_source_binding_after},
      source_app:{snapshot_performed:$source_app_ready,tree_nofollow_safe:$source_app_tree_nofollow_ready,full_bundle_fingerprint:$source_app_fingerprint_actual,binary_sha256:$source_binary_sha256_actual},
      normalized_bundle_equivalence:{
        contract:"thin-64-macho-remove-signature-linkedit-vmsize-filesize-v1",
        source_app_strictly_unsigned:$source_app_unsigned_verified,
        source_normalized:$normalized_source_app_ready,
        mounted_signed_app_normalized:$normalized_mounted_app_ready,
        exact_path_mode_content_equivalent:$normalized_unsigned_signed_equivalent,
        source_full_bundle_fingerprint:$normalized_source_app_fingerprint,
        mounted_full_bundle_fingerprint:$normalized_mounted_app_fingerprint,
        unsupported_or_fat_macho_fail_closed:true
      },
      independent_dmg_readback:{premount_trusted:$system_dmg_premount_trusted,mounted_app_tree_nofollow_safe:$mounted_app_tree_nofollow_ready,mounted_read_only:$dmg_mounted_read_only_actual,full_bundle_fingerprint:$mounted_app_fingerprint_actual,binary_sha256:$mounted_binary_sha256_actual,bundle_identifier:$mounted_bundle_identifier_actual,applications_alias_verified:$applications_alias_verified_actual,applications_alias_kind:$applications_alias_kind_actual,applications_alias_resolved_target:$applications_alias_target_actual,applications_alias_resolution:$applications_alias_resolution_actual},
      notarytool_log:{accepted_status_and_submission_id_match:$notary_log_accepted},
      independent_system_verification:{
        valid:$system_verification_valid,
        tool_paths:{codesign:"/usr/bin/codesign",xcrun:"/usr/bin/xcrun",spctl:"/usr/sbin/spctl",hdiutil:"/usr/bin/hdiutil",mount:"/sbin/mount"},
        codesign_dmg:{verified:$system_codesign_dmg_verified,exit_code:$system_codesign_dmg_exit_code,log_sha256:$system_codesign_dmg_log_sha256,log_bytes:$system_codesign_dmg_log_bytes,developer_id_identity:$system_dmg_signing_identity_actual,team_identifier:$system_dmg_team_identifier_actual,timestamp:$system_dmg_timestamp_actual},
        stapler_validate_dmg:{verified:$system_stapler_dmg_verified,exit_code:$system_stapler_dmg_exit_code,log_sha256:$system_stapler_dmg_log_sha256,log_bytes:$system_stapler_dmg_log_bytes},
        spctl_assess_dmg:{verified:$system_spctl_dmg_verified,exit_code:$system_spctl_dmg_exit_code,log_sha256:$system_spctl_dmg_log_sha256,log_bytes:$system_spctl_dmg_log_bytes},
        codesign_mounted_app:{verified:$system_codesign_mounted_app_verified,exit_code:$system_codesign_mounted_app_exit_code,log_sha256:$system_codesign_mounted_app_log_sha256,log_bytes:$system_codesign_mounted_app_log_bytes,developer_id_identity:$system_mounted_app_signing_identity_actual,team_identifier:$system_mounted_app_team_identifier_actual,timestamp:$system_mounted_app_timestamp_actual,runtime_version:$system_mounted_app_runtime_version_actual,flags:$system_mounted_app_flags_actual,hardened_runtime:(($system_mounted_app_flags_actual | contains("runtime")))},
        declared_developer_id_identity:$declared_signing_identity,
        expected_developer_id_identity:$expected_signing_identity,
        expected_developer_id_identity_source:"HEPTA_EXPECTED_SIGNING_IDENTITY_or_HEPTA_SIGNING_IDENTITY_trusted_configuration",
        dmg_signature_tuple_trusted_before_mount:$system_dmg_signature_tuple_trusted,
        apple_trust_stapler_spctl_ready_before_mount:$system_dmg_premount_trusted,
        developer_id_identity_matches_receipt:$system_developer_identity_verified,
        expected_team_identifier:$expected_team_identifier,
        expected_team_identifier_source:"HEPTA_EXPECTED_TEAM_ID_trusted_configuration",
        artifact_declared_team_cannot_define_expected_team:true,
        declared_team_identifier:$declared_team_identifier,
        signing_properties_match_system_and_receipt:$system_signing_properties_verified
      }
    }
  ' >"$READBACK_TMP"

atomic_replace_from_file "$READBACK_TMP" "$READBACK_REPORT_PATH"

distribution_sha="$(file_sha256 "$DISTRIBUTION_PREFLIGHT_REPORT_PATH")"
approval_sha="$(file_sha256 "$RELEASE_APPROVAL_INTAKE_REPORT_PATH")"
boundary_sha="$(file_sha256 "$RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH")"
evidence_archive_report_sha="$(file_sha256 "$EVIDENCE_ARCHIVE_REPORT_PATH")"
readback_sha="$(file_sha256 "$READBACK_REPORT_PATH")"
readback_bytes="$(file_bytes "$READBACK_REPORT_PATH")"

jq -n \
  --slurpfile distribution_file "$DISTRIBUTION_PREFLIGHT_REPORT_PATH" \
  --slurpfile approval_file "$RELEASE_APPROVAL_INTAKE_REPORT_PATH" \
  --slurpfile boundary_file "$RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH" \
  --slurpfile evidence_archive_file "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  '
  ($distribution_file[0]) as $distribution
  | ($approval_file[0]) as $approval
  | ($boundary_file[0]) as $boundary
  | ($evidence_archive_file[0]) as $archive
  | {
      artifact_kind:"signed_notarized_stapled_artifact",
      artifact_version:3,
      receipt_contract_version:3,
      owner_lane:"release_operator",
      product:"Hepta Native",
      bundle_identifier:$distribution.package_metadata.bundle_identifier,
      bundle_name:$distribution.package_metadata.bundle_name,
      bundle_executable:$distribution.package_metadata.bundle_executable,
      required_state:{
        release_approval_valid_required:true,
        independent_approval_verifier_contract_required:true,
        present_artifact_branch_supported:false,
        signed_app_artifact_required:true,
        notarized_app_artifact_required:true,
        stapled_app_artifact_required:false,
        stapled_dmg_artifact_required:true,
        local_distribution_artifact_required:true,
        public_distribution_policy_required:true,
        public_upload_must_be_false:true,
        no_live_product_claim_from_artifact_alone:true
      },
      expected_source_evidence:{
        release_approval_template_sha256:$approval.template_sha256,
        release_artifact_boundary_markdown_sha256:$boundary.boundary_markdown_sha256,
        evidence_archive_sha256:$archive.archive_sha256,
        unsigned_app_bundle_sha256:$boundary.release_artifact_boundary.unsigned_app_bundle_sha256,
        source_app:"",
        source_binary_sha256:"",
        signed_binary_sha256:"",
        source_app_bundle_fingerprint:null,
        signed_app_bundle_fingerprint:null,
        unsigned_package_receipt_path:"",
        unsigned_package_receipt_sha256:"",
        source_head:"",
        source_tree:"",
        source_fingerprint:"",
        source_stable_during_unsigned_package_run:true,
        private_copy_recomputed_before_signing:true
      },
      artifact_evidence:{
        signed:false,
        notarized:false,
        stapled:false,
        dmg_stapled:false,
        app_stapled:false,
        local_distribution_artifact_written:false,
        public_distribution_artifact_written:false,
        public_upload_performed:false,
        signed_artifact_path:"",
        signed_artifact_sha256:"",
        signed_artifact_bytes:0,
        notarization_ticket_sha256:"",
        notarytool_submit_log_sha256:"",
        notarytool_submit_log_bytes:0,
        notarytool_exit_code:0,
        notary_submission_id:"",
        notary_submission_state:"accepted",
        notary_submission_confirmed:true,
        notary_submission_may_have_occurred:true,
        codesign_verify_app_sha256:"",
        codesign_verify_dmg_sha256:"",
        stapler_staple_sha256:"",
        stapler_validate_sha256:"",
        spctl_assessment_sha256:"",
        dmg_mounted_read_only:true,
        mounted_app_bundle_fingerprint:null,
        mounted_binary_sha256:"",
        mounted_bundle_identifier:"ai.hepta.nativeapp",
        applications_alias_verified:true,
        applications_alias_kind:"finder_bookmark_alias",
        applications_alias_resolved_target:"/Applications",
        dmg_readonly_attach_sha256:"",
        dmg_readonly_mount_sha256:"",
        notarytool_submit_log_path:"",
        codesign_verify_app_log_path:"",
        codesign_verify_dmg_log_path:"",
        stapler_staple_log_path:"",
        stapler_validate_log_path:"",
        spctl_assessment_log_path:"",
        dmg_readonly_attach_path:"",
        dmg_readonly_mount_log_path:"",
        signing_identity:"",
        notary_auth_mode:""
      },
      claim_boundary:{
        release_artifact_claim_ready:false,
        release_execution_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false,
        live_product_claim_ready:false
      },
      side_effects:{
        external_mutation:false,
        credential_value_read:false,
        keychain_identity_lookup_performed:false,
        network_call_performed:false,
        notary_submission_performed:false,
        app_signed:false,
        app_notarized:false,
        app_stapled:false,
        local_distribution_artifact_written:false,
        public_distribution_artifact_written:false,
        public_upload_performed:false
      }
    }' >"$TEMPLATE_TMP"

atomic_replace_from_file "$TEMPLATE_TMP" "$TEMPLATE_PATH"

jq -r '
  "# Hepta UI Release Artifact Intake\n\n"
  + "- Kind: local signed/notarized/stapled artifact intake contract\n"
  + "- Target: \(.product) / \(.bundle_identifier)\n"
  + "- Artifact input env: `HEPTA_UI_RELEASE_ARTIFACT_INPUT_PATH`\n"
  + "- Receipt contract: v3 only; legacy v1 and simulated/fake semantics are rejected.\n"
  + "- Artifact evidence alone does not make live-product, public distribution, or release claims ready.\n\n"
  + "## Required Artifact Evidence\n\n"
  + "- valid release approval\n"
  + "- signed app artifact\n"
  + "- notarized app artifact\n"
  + "- stapled DMG artifact\n"
  + "- codesign verify app/DMG output hashes\n"
  + "- notarytool submit, stapler, and spctl output hashes\n"
  + "- independent file snapshots, byte/hash readback, and unique absolute evidence paths\n"
  + "- source unsigned receipt plus source/signed/mounted full bundle fingerprints\n"
  + "- independent read-only DMG mount, binary/bundle ID, and `/Applications` alias target\n"
  + "- local signed/notarized/stapled DMG artifact-write evidence\n"
  + "- no public upload/public claim from the artifact receipt alone\n"
  + "- post-artifact UI readiness refresh\n"
' "$TEMPLATE_PATH" >"$MARKDOWN_TMP"

atomic_replace_from_file "$MARKDOWN_TMP" "$MARKDOWN_PATH"

template_sha="$(file_sha256 "$TEMPLATE_PATH")"
template_bytes="$(file_bytes "$TEMPLATE_PATH")"
markdown_sha="$(file_sha256 "$MARKDOWN_PATH")"
markdown_bytes="$(file_bytes "$MARKDOWN_PATH")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_release_artifact_intake_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg intake_dir "$INTAKE_DIR" \
  --arg template_path "$TEMPLATE_PATH" \
  --arg markdown_path "$MARKDOWN_PATH" \
  --arg distribution_path "$DISTRIBUTION_PREFLIGHT_REPORT_PATH" \
  --arg approval_path "$RELEASE_APPROVAL_INTAKE_REPORT_PATH" \
  --arg boundary_path "$RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH" \
  --arg evidence_archive_path "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  --arg readback_path "$READBACK_REPORT_PATH" \
  --arg distribution_sha "$distribution_sha" \
  --arg approval_sha "$approval_sha" \
  --arg boundary_sha "$boundary_sha" \
  --arg evidence_archive_report_sha "$evidence_archive_report_sha" \
  --arg readback_sha "$readback_sha" \
  --arg template_sha "$template_sha" \
  --arg markdown_sha "$markdown_sha" \
  --argjson template_bytes "$template_bytes" \
  --argjson markdown_bytes "$markdown_bytes" \
  --argjson artifact_present "$artifact_present" \
  --argjson artifact_input_path "$artifact_input_path_json" \
  --argjson artifact_captured_input_path "$artifact_captured_input_path_json" \
  --argjson artifact_sha "$artifact_sha_json" \
  --argjson artifact_bytes "$artifact_bytes" \
  --argjson readback_bytes "$readback_bytes" \
  --slurpfile distribution_file "$DISTRIBUTION_PREFLIGHT_REPORT_PATH" \
  --slurpfile approval_file "$RELEASE_APPROVAL_INTAKE_REPORT_PATH" \
  --slurpfile boundary_file "$RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH" \
  --slurpfile evidence_archive_file "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  --slurpfile template_file "$TEMPLATE_PATH" \
  --slurpfile artifact_file "$ARTIFACT_CAPTURE_PATH" \
  --slurpfile readback_file "$READBACK_REPORT_PATH" \
  '
  ($distribution_file[0]) as $distribution
  | ($approval_file[0]) as $approval
  | ($boundary_file[0]) as $boundary
  | ($evidence_archive_file[0]) as $archive
  | ($template_file[0]) as $template
  | ($artifact_file[0]) as $artifact
  | ($readback_file[0]) as $readback
  | def sha_ready($sha): (($sha // "") | test("^[0-9a-f]{64}$"));
    def source_chain_ready:
      $distribution.distribution_preflight_gate_ready == true
      and $distribution.distribution_static_contract_ready == true
      and $distribution.public_distribution_ready == false
      and $distribution.app_signed == false
      and $distribution.app_notarized == false
      and $distribution.app_stapled == false
      and $distribution.public_distribution_artifact_written == false
      and $approval.release_approval_intake_gate_ready == true
      and $approval.release_approval_state.waiting_for_release_approval == true
      and $approval.release_approval_state.release_approval_present == false
      and $approval.release_approval_state.release_approval_valid == false
      and $approval.release_approval_state.independent_approval_verifier_ready == false
      and $approval.release_approval_state.self_reported_approval_can_authorize_release == false
      and ($approval.approval_blockers | index("independent_release_approval_verifier_unavailable")) != null
      and $approval.claim_boundary.release_approval_claim_ready == false
      and $approval.release_approval_state.approval_only_can_make_release_claim == false
      and $approval.release_approval_state.signed_notarized_stapled_artifact_present == false
      and $approval.release_approval_state.public_distribution_artifact_written == false
      and $approval.claim_boundary.release_execution_ready == false
      and $approval.claim_boundary.public_distribution_claim_ready == false
      and $approval.claim_boundary.release_claim_ready == false
      and $boundary.release_artifact_boundary_gate_ready == true
      and $boundary.release_artifact_boundary.next_required_artifact_gate == "signed_notarized_stapled_artifact_gate"
      and $boundary.release_artifact_boundary.signed_notarized_stapled_artifact_present == false
      and $boundary.release_artifact_boundary.public_distribution_artifact_written == false
      and $boundary.claim_boundary.release_artifact_claim_ready == false
      and $boundary.claim_boundary.public_distribution_claim_ready == false
      and $boundary.claim_boundary.release_claim_ready == false
      and $archive.evidence_archive_gate_ready == true
      and $archive.claim_boundary.local_evidence_archive_ready == true
      and $archive.all_extracted_items_sha256_match == true
      and $archive.claim_boundary.public_distribution_claim_ready == false
      and $archive.claim_boundary.release_claim_ready == false
      and sha_ready($distribution_sha)
      and sha_ready($approval_sha)
      and sha_ready($boundary_sha)
      and sha_ready($evidence_archive_report_sha);
    def template_ready:
      $template.artifact_kind == "signed_notarized_stapled_artifact"
      and $template.artifact_version == 3
      and $template.receipt_contract_version == 3
      and $template.owner_lane == "release_operator"
      and $template.bundle_identifier == $distribution.package_metadata.bundle_identifier
      and $template.required_state.release_approval_valid_required == true
      and $template.required_state.independent_approval_verifier_contract_required == true
      and $template.required_state.present_artifact_branch_supported == false
      and $template.required_state.signed_app_artifact_required == true
      and $template.required_state.notarized_app_artifact_required == true
      and $template.required_state.stapled_app_artifact_required == false
      and $template.required_state.stapled_dmg_artifact_required == true
      and $template.required_state.local_distribution_artifact_required == true
      and $template.required_state.public_upload_must_be_false == true
      and $template.required_state.no_live_product_claim_from_artifact_alone == true
      and sha_ready($template_sha)
      and $template_bytes > 0
      and sha_ready($markdown_sha)
      and $markdown_bytes > 0;
    def artifact_distribution_semantics:
      ($artifact.artifact_evidence.public_distribution_artifact_semantics // "");
    def artifact_input_valid:
      $artifact_present == true
      and $readback.receipt_contract_version == 3
      and $readback.performed == true
      and $readback.all_evidence_valid == true
      and $readback.independent_system_verification.valid == true
      and $readback.independent_system_verification.codesign_dmg.verified == true
      and $readback.independent_system_verification.stapler_validate_dmg.verified == true
      and $readback.independent_system_verification.spctl_assess_dmg.verified == true
      and $readback.independent_system_verification.codesign_mounted_app.verified == true
      and $readback.independent_system_verification.developer_id_identity_matches_receipt == true
      and $readback.current_source_binding.current_repository_worktree_clean == true
      and $readback.current_source_binding.stable_during_readback == true
      and $readback.current_source_binding.matches_release_and_unsigned_receipts == true
      and $artifact.artifact_kind == "signed_notarized_stapled_artifact"
      and $artifact.artifact_version == 3
      and $artifact.receipt_contract_version == 3
      and $artifact.status == "ready"
      and $artifact.owner_lane == "release_operator"
      and $artifact.bundle_identifier == $distribution.package_metadata.bundle_identifier
      and (($artifact.artifact_mode // "") | test("(?i)(simulat|fake|fixture)") | not)
      and ($artifact.simulated_provenance // null) == null
      and $artifact.release_approval_valid == true
      and $approval.release_approval_state.release_approval_valid == true
      and $artifact.artifact_evidence.signed == true
      and $artifact.artifact_evidence.notarized == true
      and $artifact.artifact_evidence.stapled == true
      and $artifact.artifact_evidence.dmg_stapled == true
      and ($artifact.artifact_evidence.app_stapled // false) == false
      and $artifact.artifact_evidence.local_distribution_artifact_written == true
      and $artifact.artifact_evidence.public_distribution_artifact_written == true
      and ($artifact.artifact_evidence.public_upload_performed // false) == false
      and artifact_distribution_semantics == "local_signed_notarized_stapled_dmg_written_not_public_upload"
      and ($artifact.claim_boundary.release_artifact_claim_ready // false) == false
      and ($artifact.claim_boundary.release_execution_ready // false) == false
      and ($artifact.claim_boundary.public_distribution_claim_ready // false) == false
      and ($artifact.claim_boundary.release_claim_ready // false) == false
      and ($artifact.claim_boundary.live_product_claim_ready // false) == false
      and $artifact.source_evidence.source_stable_during_unsigned_package_run == true
      and $artifact.source_evidence.source_worktree_clean == true
      and $artifact.source_evidence.private_copy_recomputed_before_signing == true
      and $artifact.source_evidence.consumed_exact_formal_app == true
      and $artifact.source_evidence.built_second_product_app == false
      and (($artifact.source_evidence.source_head // "") | test("^[0-9a-f]{40}$"))
      and (($artifact.source_evidence.source_tree // "") | test("^[0-9a-f]{40}$"))
      and (($artifact.source_evidence.source_fingerprint // "") | test("^[0-9a-f]{64}$"))
      and (($artifact.artifact_evidence.signed_artifact_path // "") | length) > 0
      and (($artifact.artifact_evidence.signed_artifact_sha256 // "") | test("^[0-9a-f]{64}$"))
      and ($artifact.artifact_evidence.signed_artifact_bytes // 0) > 0
      and (($artifact.artifact_evidence.notarization_ticket_sha256 // "") | test("^[0-9a-f]{64}$"))
      and (($artifact.artifact_evidence.notarytool_submit_log_sha256 // "") | test("^[0-9a-f]{64}$"))
      and $artifact.artifact_evidence.notarytool_submit_log_sha256 == $artifact.artifact_evidence.notarization_ticket_sha256
      and ($artifact.artifact_evidence.notarytool_submit_log_bytes // 0) > 0
      and $artifact.artifact_evidence.notarytool_exit_code == 0
      and (($artifact.artifact_evidence.notary_submission_id // "") | length) > 0
      and $artifact.artifact_evidence.notary_submission_state == "accepted"
      and $artifact.artifact_evidence.notary_submission_confirmed == true
      and $artifact.artifact_evidence.notary_submission_may_have_occurred == true
      and (($artifact.artifact_evidence.codesign_verify_app_sha256 // "") | test("^[0-9a-f]{64}$"))
      and (($artifact.artifact_evidence.codesign_verify_dmg_sha256 // "") | test("^[0-9a-f]{64}$"))
      and (($artifact.artifact_evidence.stapler_staple_sha256 // "") | test("^[0-9a-f]{64}$"))
      and (($artifact.artifact_evidence.stapler_validate_sha256 // "") | test("^[0-9a-f]{64}$"))
      and (($artifact.artifact_evidence.spctl_assessment_sha256 // "") | test("^[0-9a-f]{64}$"))
      and $artifact.artifact_evidence.dmg_mounted_read_only == true
      and $artifact.artifact_evidence.mounted_bundle_identifier == $artifact.bundle_identifier
      and $artifact.artifact_evidence.applications_alias_verified == true
      and $artifact.artifact_evidence.applications_alias_kind == "finder_bookmark_alias"
      and $artifact.artifact_evidence.applications_alias_resolved_target == "/Applications"
      and (($artifact.artifact_evidence.notarytool_submit_log_path // "") | length) > 0
      and (($artifact.artifact_evidence.codesign_verify_app_log_path // "") | length) > 0
      and (($artifact.artifact_evidence.codesign_verify_dmg_log_path // "") | length) > 0
      and (($artifact.artifact_evidence.stapler_staple_log_path // "") | length) > 0
      and (($artifact.artifact_evidence.stapler_validate_log_path // "") | length) > 0
      and (($artifact.artifact_evidence.spctl_assessment_log_path // "") | length) > 0
      and (($artifact.artifact_evidence.dmg_readonly_attach_path // "") | length) > 0
      and (($artifact.artifact_evidence.dmg_readonly_mount_log_path // "") | length) > 0
      and (($artifact.artifact_evidence.signing_identity // "") | length) > 0
      and (($artifact.artifact_evidence.notary_auth_mode // "") | IN("apple_env", "keychain_profile"))
      and ($artifact.side_effects.credential_value_captured // false) == false
      and ($artifact.side_effects.network_call_performed // false) == true
      and ($artifact.side_effects.notary_submission_performed // false) == true
      and ($artifact.side_effects.app_signed // false) == true
      and ($artifact.side_effects.app_notarized // false) == true
      and ($artifact.side_effects.app_stapled // false) == false
      and ($artifact.side_effects.dmg_stapled // false) == true
      and ($artifact.side_effects.local_distribution_artifact_written // false) == true
      and ($artifact.side_effects.public_distribution_artifact_written // false) == true
      and ($artifact.side_effects.public_upload_performed // false) == false
      and ($artifact.side_effects.external_mutation // false) == true;
    (
      source_chain_ready
      and template_ready
      and $artifact_present == false
    ) as $ready
  | false as $artifact_valid
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      release_artifact_intake_gate_ready:$ready,
      intake_kind:"local_signed_notarized_stapled_artifact_intake_contract",
      intake_version:3,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      intake_dir:$intake_dir,
      template_path:$template_path,
      markdown_path:$markdown_path,
      readback_report_path:$readback_path,
      source_reports:{
        native_distribution_preflight:$distribution_path,
        release_approval_intake:$approval_path,
        release_artifact_boundary:$boundary_path,
        evidence_archive:$evidence_archive_path,
        release_artifact_readback:$readback_path
      },
      source_report_sha256:{
        native_distribution_preflight:$distribution_sha,
        release_approval_intake:$approval_sha,
        release_artifact_boundary:$boundary_sha,
        evidence_archive:$evidence_archive_report_sha,
        release_artifact_readback:$readback_sha
      },
      template_sha256:$template_sha,
      template_bytes:$template_bytes,
      markdown_sha256:$markdown_sha,
      markdown_bytes:$markdown_bytes,
      readback_sha256:$readback_sha,
      readback_bytes:$readback_bytes,
      root_report_replay_required_count_after_intake:37,
      release_artifact_state:{
        waiting_for_release_artifact:($artifact_present == false),
        release_artifact_present:$artifact_present,
        release_artifact_input_path:$artifact_input_path,
        release_artifact_captured_input_path:$artifact_captured_input_path,
        release_artifact_input_sha256:$artifact_sha,
        release_artifact_input_bytes:$artifact_bytes,
        release_artifact_valid:$artifact_valid,
        present_artifact_branch_supported:false,
        independent_approval_verifier_contract_ready:false,
        release_approval_valid:$approval.release_approval_state.release_approval_valid,
        receipt_contract_version:($artifact.receipt_contract_version // 0),
        evidence_readback_valid:$readback.all_evidence_valid,
        evidence_readback_blockers:$readback.blockers,
        referenced_paths_absolute_and_unique:$readback.referenced_paths_absolute_and_unique,
        signed_app_artifact_present:($artifact_valid and ($artifact.artifact_evidence.signed // false)),
        notarized_app_artifact_present:($artifact_valid and ($artifact.artifact_evidence.notarized // false)),
        stapled_app_artifact_present:false,
        stapled_dmg_artifact_present:($artifact_valid and ($artifact.artifact_evidence.dmg_stapled // false)),
        signed_notarized_stapled_artifact_present:$artifact_valid,
        local_distribution_artifact_written:($artifact_valid and ($artifact.artifact_evidence.local_distribution_artifact_written // false)),
        public_distribution_artifact_written:($artifact_valid and ($artifact.artifact_evidence.public_distribution_artifact_written // false)),
        public_upload_performed:($artifact_valid and ($artifact.artifact_evidence.public_upload_performed // false)),
        public_distribution_artifact_semantics:($artifact.artifact_evidence.public_distribution_artifact_semantics // "missing_release_artifact_distribution_semantics"),
        signed_artifact_path_present:($artifact_valid and (($artifact.artifact_evidence.signed_artifact_path // "") | length > 0)),
        signed_artifact_bytes:(if $artifact_valid then ($readback.signed_artifact.actual_bytes // 0) else 0 end),
        signed_artifact_hash_and_bytes_verified:($artifact_valid and $readback.signed_artifact.snapshot_performed),
        source_unsigned_receipt_verified:($artifact_valid and $readback.source_unsigned_receipt.contract_matches),
        current_source_binding_verified:($artifact_valid and $readback.current_source_binding.matches_release_and_unsigned_receipts),
        current_repository_worktree_clean:($readback.current_source_binding.current_repository_worktree_clean // false),
        source_app_full_bundle_fingerprint_verified:($artifact_valid and $readback.source_app.snapshot_performed),
        mounted_app_full_bundle_fingerprint_verified:($artifact_valid and $readback.independent_dmg_readback.mounted_read_only),
        source_app_strictly_unsigned:($artifact_valid and $readback.normalized_bundle_equivalence.source_app_strictly_unsigned),
        normalized_unsigned_signed_bundle_equivalent:($artifact_valid and $readback.normalized_bundle_equivalence.exact_path_mode_content_equivalent),
        applications_alias_target_verified:($artifact_valid and $readback.independent_dmg_readback.applications_alias_verified and $readback.independent_dmg_readback.applications_alias_resolved_target == "/Applications"),
        codesign_verify_app_ready:($artifact_valid and $readback.command_logs.codesign_verify_app.valid),
        codesign_verify_dmg_ready:($artifact_valid and $readback.command_logs.codesign_verify_dmg.valid),
        stapler_staple_ready:($artifact_valid and $readback.command_logs.stapler_staple.valid),
        stapler_validate_ready:($artifact_valid and $readback.command_logs.stapler_validate.valid),
        spctl_assessment_ready:($artifact_valid and $readback.command_logs.spctl_assessment.valid),
        independent_system_verification_ready:($artifact_valid and $readback.independent_system_verification.valid),
        system_codesign_dmg_ready:($artifact_valid and $readback.independent_system_verification.codesign_dmg.verified),
        system_stapler_validate_dmg_ready:($artifact_valid and $readback.independent_system_verification.stapler_validate_dmg.verified),
        system_spctl_assess_dmg_ready:($artifact_valid and $readback.independent_system_verification.spctl_assess_dmg.verified),
        system_codesign_mounted_app_ready:($artifact_valid and $readback.independent_system_verification.codesign_mounted_app.verified),
        system_developer_id_identity_verified:($artifact_valid and $readback.independent_system_verification.developer_id_identity_matches_receipt),
        system_signing_properties_verified:($artifact_valid and $readback.independent_system_verification.signing_properties_match_system_and_receipt),
        notary_auth_mode:($artifact.artifact_evidence.notary_auth_mode // ""),
        next_required_step:"post_artifact_ui_readiness_refresh"
      },
      release_artifact_source_side_effects:{
        credential_value_read:($artifact.side_effects.credential_value_read // false),
        keychain_identity_lookup_performed:($artifact.side_effects.keychain_identity_lookup_performed // false),
        network_call_performed:($artifact.side_effects.network_call_performed // false),
        notary_submission_performed:($artifact.side_effects.notary_submission_performed // false),
        app_signed:($artifact.side_effects.app_signed // false),
        app_notarized:($artifact.side_effects.app_notarized // false),
        app_stapled:($artifact.side_effects.app_stapled // false),
        local_distribution_artifact_written:($artifact.side_effects.local_distribution_artifact_written // false),
        public_distribution_artifact_written:($artifact.side_effects.public_distribution_artifact_written // false),
        public_upload_performed:($artifact.side_effects.public_upload_performed // $artifact.artifact_evidence.public_upload_performed // false),
        external_mutation:($artifact.side_effects.external_mutation // false)
      },
      source_alignment:{
        native_distribution_preflight_ready:$distribution.distribution_preflight_gate_ready,
        release_approval_intake_ready:$approval.release_approval_intake_gate_ready,
        release_approval_waiting_for_approval:$approval.release_approval_state.waiting_for_release_approval,
        release_approval_present:$approval.release_approval_state.release_approval_present,
        release_approval_valid:$approval.release_approval_state.release_approval_valid,
        independent_approval_verifier_ready:$approval.release_approval_state.independent_approval_verifier_ready,
        self_reported_approval_can_authorize_release:$approval.release_approval_state.self_reported_approval_can_authorize_release,
        present_artifact_branch_supported:false,
        independent_approval_verifier_contract_ready:false,
        release_artifact_boundary_ready:$boundary.release_artifact_boundary_gate_ready,
        release_artifact_boundary_root_report_required_count:$boundary.release_artifact_boundary.root_report_replay_required_count_after_boundary,
        release_artifact_boundary_next_required_artifact_gate:$boundary.release_artifact_boundary.next_required_artifact_gate,
        evidence_archive_ready:$archive.evidence_archive_gate_ready,
        real_backend_receipt_claim_ready:($boundary.claim_boundary.real_backend_receipt_claim_ready // false),
        backend_receipt_claim_ready:($boundary.claim_boundary.backend_receipt_claim_ready // false),
        unsigned_app_bundle_codesign_status:$boundary.release_artifact_boundary.unsigned_app_bundle_codesign_status,
        approval_only_can_make_release_claim:$approval.release_approval_state.approval_only_can_make_release_claim,
        boundary_signed_notarized_stapled_artifact_present:$boundary.release_artifact_boundary.signed_notarized_stapled_artifact_present,
        boundary_public_distribution_artifact_written:$boundary.release_artifact_boundary.public_distribution_artifact_written
      },
      release_artifact_blockers:[
        (if $approval.release_approval_state.release_approval_valid then empty else "operator_release_approval_required" end),
        (if $approval.release_approval_state.independent_approval_verifier_ready then empty else "independent_release_approval_verifier_unavailable" end),
        "release_artifact_present_branch_unsupported_without_independent_approval_verifier",
        (if $artifact_valid then empty else "signed_notarized_stapled_artifact_missing" end),
        (if $artifact_valid then empty else "release_artifact_v3_readback_not_verified" end),
        (if $artifact_valid then empty else "public_distribution_artifact_not_written" end),
        "post_artifact_ui_readiness_refresh_required",
        (if ($boundary.claim_boundary.real_backend_receipt_claim_ready // false) then empty else "real_backend_receipt_missing" end)
      ],
      claim_boundary:{
        local_release_artifact_intake_ready:$ready,
        release_approval_claim_ready:$approval.claim_boundary.release_approval_claim_ready,
        release_artifact_claim_ready:false,
        release_execution_ready:false,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false,
        external_actions_allowed:false,
        public_upload_performed:false,
        signing_notarization_performed:false
      },
      side_effects:{
        filesystem_read:true,
        local_template_written:true,
        local_markdown_written:true,
        local_report_written:true,
        credential_value_read:false,
        keychain_identity_lookup_performed:false,
        network_call_performed:false,
        notary_submission_performed:false,
        app_signed:false,
        app_notarized:false,
        app_stapled:false,
        local_distribution_artifact_written:false,
        public_distribution_artifact_written:false,
        public_upload_performed:false,
        matrix_login:false,
        gateway_call:false,
        provider_invoked:false,
        channel_delivery:false,
        external_mutation:false
      }
    }' >"$REPORT_TMP"

jq -e '
  def sha_ready($value): (($value // "") | test("^[0-9a-f]{64}$"));
  .intake_kind == "local_signed_notarized_stapled_artifact_intake_contract"
  and .intake_version == 3
  and sha_ready(.template_sha256)
  and .template_bytes > 0
  and sha_ready(.markdown_sha256)
  and .markdown_bytes > 0
  and sha_ready(.readback_sha256)
  and .readback_bytes > 0
  and .root_report_replay_required_count_after_intake == 37
  and .release_artifact_state.next_required_step == "post_artifact_ui_readiness_refresh"
  and (.release_artifact_state.release_artifact_present | type) == "boolean"
  and (.release_artifact_state.release_artifact_valid | type) == "boolean"
  and .release_artifact_state.present_artifact_branch_supported == false
  and .release_artifact_state.independent_approval_verifier_contract_ready == false
  and (.release_artifact_state.evidence_readback_valid | type) == "boolean"
  and (.release_artifact_state.evidence_readback_blockers | type) == "array"
  and (.release_artifact_state.referenced_paths_absolute_and_unique | type) == "boolean"
  and (.release_artifact_state.signed_artifact_hash_and_bytes_verified | type) == "boolean"
  and (.release_artifact_state.source_unsigned_receipt_verified | type) == "boolean"
  and (.release_artifact_state.current_source_binding_verified | type) == "boolean"
  and (.release_artifact_state.current_repository_worktree_clean | type) == "boolean"
  and (.release_artifact_state.source_app_full_bundle_fingerprint_verified | type) == "boolean"
  and (.release_artifact_state.mounted_app_full_bundle_fingerprint_verified | type) == "boolean"
  and (.release_artifact_state.source_app_strictly_unsigned | type) == "boolean"
  and (.release_artifact_state.normalized_unsigned_signed_bundle_equivalent | type) == "boolean"
  and (.release_artifact_state.applications_alias_target_verified | type) == "boolean"
  and (.release_artifact_state.independent_system_verification_ready | type) == "boolean"
  and (.release_artifact_state.system_codesign_dmg_ready | type) == "boolean"
  and (.release_artifact_state.system_stapler_validate_dmg_ready | type) == "boolean"
  and (.release_artifact_state.system_spctl_assess_dmg_ready | type) == "boolean"
  and (.release_artifact_state.system_codesign_mounted_app_ready | type) == "boolean"
  and (.release_artifact_state.system_developer_id_identity_verified | type) == "boolean"
  and (.release_artifact_state.system_signing_properties_verified | type) == "boolean"
  and .source_alignment.release_approval_intake_ready == true
  and .source_alignment.release_approval_waiting_for_approval == true
  and .source_alignment.release_approval_present == false
  and .source_alignment.release_approval_valid == false
  and .source_alignment.independent_approval_verifier_ready == false
  and .source_alignment.self_reported_approval_can_authorize_release == false
  and .source_alignment.present_artifact_branch_supported == false
  and .source_alignment.independent_approval_verifier_contract_ready == false
  and .source_alignment.release_artifact_boundary_ready == true
  and .source_alignment.release_artifact_boundary_root_report_required_count == 36
  and .source_alignment.release_artifact_boundary_next_required_artifact_gate == "signed_notarized_stapled_artifact_gate"
  and .source_alignment.approval_only_can_make_release_claim == false
  and .source_alignment.boundary_signed_notarized_stapled_artifact_present == false
  and .source_alignment.boundary_public_distribution_artifact_written == false
  and (.release_artifact_blockers | index("operator_release_approval_required") != null)
  and (.release_artifact_blockers | index("independent_release_approval_verifier_unavailable") != null)
  and (.release_artifact_blockers | index("release_artifact_present_branch_unsupported_without_independent_approval_verifier") != null)
  and (.release_artifact_blockers | index("post_artifact_ui_readiness_refresh_required") != null)
  and .claim_boundary.release_artifact_claim_ready == false
  and .claim_boundary.release_execution_ready == false
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and .claim_boundary.public_upload_performed == false
  and .side_effects.network_call_performed == false
  and .side_effects.notary_submission_performed == false
  and .side_effects.public_upload_performed == false
  and .side_effects.external_mutation == false
  and .release_artifact_state.release_artifact_valid == false
  and .release_artifact_state.signed_notarized_stapled_artifact_present == false
  and .release_artifact_state.stapled_app_artifact_present == false
  and .release_artifact_state.stapled_dmg_artifact_present == false
  and .release_artifact_state.local_distribution_artifact_written == false
  and .release_artifact_state.public_distribution_artifact_written == false
  and .release_artifact_state.public_upload_performed == false
  and .release_artifact_state.signed_artifact_hash_and_bytes_verified == false
  and .release_artifact_state.source_unsigned_receipt_verified == false
  and .release_artifact_state.current_source_binding_verified == false
  and .release_artifact_state.source_app_full_bundle_fingerprint_verified == false
  and .release_artifact_state.mounted_app_full_bundle_fingerprint_verified == false
  and .release_artifact_state.normalized_unsigned_signed_bundle_equivalent == false
  and .release_artifact_state.applications_alias_target_verified == false
  and .release_artifact_state.independent_system_verification_ready == false
  and (.release_artifact_blockers | index("signed_notarized_stapled_artifact_missing") != null)
  and (.release_artifact_blockers | index("release_artifact_v3_readback_not_verified") != null)
  and (.release_artifact_blockers | index("public_distribution_artifact_not_written") != null)
  and (
    if .release_artifact_state.release_artifact_present == false then
      .status == "ready"
      and .release_artifact_intake_gate_ready == true
      and .claim_boundary.local_release_artifact_intake_ready == true
      and .release_artifact_state.waiting_for_release_artifact == true
      and .release_artifact_state.receipt_contract_version == 0
      and .release_artifact_state.evidence_readback_valid == false
    else
      .status == "failed"
      and .release_artifact_intake_gate_ready == false
      and .claim_boundary.local_release_artifact_intake_ready == false
      and .release_artifact_state.waiting_for_release_artifact == false
    end
  )
' "$REPORT_TMP" >/dev/null

mkdir -p "$(dirname "$REPORT_PATH")"
atomic_replace_from_file "$REPORT_TMP" "$REPORT_PATH"
remove_safe_regular_leaf "$ACCEPTED_ARTIFACT_INPUT_PATH"
cat "$REPORT_TMP"
if ! jq -e '.release_artifact_intake_gate_ready == true' "$REPORT_TMP" >/dev/null; then
  exit 1
fi
