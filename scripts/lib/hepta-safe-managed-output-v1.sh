#!/usr/bin/env bash

# Shared no-follow helpers for release gates that write caller-selected paths.
# The caller must validate every path before creating directories or files.

HEPTA_SAFE_ENV=/usr/bin/env
HEPTA_SAFE_RUBY=/usr/bin/ruby
HEPTA_SAFE_PATH=/usr/bin:/bin:/usr/sbin:/sbin

hepta_safe_absolute_path() {
  "$HEPTA_SAFE_ENV" -i PATH="$HEPTA_SAFE_PATH" HOME=/var/empty TMPDIR=/private/tmp \
    "$HEPTA_SAFE_RUBY" -e 'print File.expand_path(ARGV.fetch(0))' "$1"
}

hepta_safe_canonical_path() {
  "$HEPTA_SAFE_ENV" -i PATH="$HEPTA_SAFE_PATH" HOME=/var/empty TMPDIR=/private/tmp \
    "$HEPTA_SAFE_RUBY" -e '
      cursor = File.expand_path(ARGV.fetch(0))
      suffix = []
      until File.exist?(cursor) || File.symlink?(cursor) || File.dirname(cursor) == cursor
        suffix.unshift(File.basename(cursor))
        cursor = File.dirname(cursor)
      end
      abort "path has no existing ancestor" unless File.exist?(cursor) || File.symlink?(cursor)
      print File.join(File.realpath(cursor), *suffix)
    ' "$1"
}

hepta_safe_normalize_path() {
  local label="$1" path="$2" absolute canonical
  if [[ -z "$path" || "$path" != /* ]]; then
    printf '%s path must be absolute: %s\n' "$label" "$path" >&2
    return 64
  fi
  absolute="$(hepta_safe_absolute_path "$path")"
  canonical="$(hepta_safe_canonical_path "$absolute")"
  if [[ "$absolute" != "$canonical" ]]; then
    printf '%s path contains a symlinked component: %s\n' "$label" "$path" >&2
    return 64
  fi
  printf '%s' "$canonical"
}

hepta_safe_paths_overlap() {
  "$HEPTA_SAFE_ENV" -i PATH="$HEPTA_SAFE_PATH" HOME=/var/empty TMPDIR=/private/tmp \
    "$HEPTA_SAFE_RUBY" -e '
      left = File.expand_path(ARGV.fetch(0))
      right = File.expand_path(ARGV.fetch(1))
      overlap = left == right || left.start_with?(right + File::SEPARATOR) || right.start_with?(left + File::SEPARATOR)
      exit(overlap ? 0 : 1)
    ' "$1" "$2"
}

hepta_safe_is_strict_descendant() {
  "$HEPTA_SAFE_ENV" -i PATH="$HEPTA_SAFE_PATH" HOME=/var/empty TMPDIR=/private/tmp \
    "$HEPTA_SAFE_RUBY" -e '
      child = File.expand_path(ARGV.fetch(0))
      parent = File.expand_path(ARGV.fetch(1))
      exit(child.start_with?(parent + File::SEPARATOR) ? 0 : 1)
    ' "$1" "$2"
}

hepta_safe_require_directory_target() {
  local label="$1" path="$2"
  if [[ -L "$path" || ( -e "$path" && ! -d "$path" ) ]]; then
    printf '%s is not a safe directory target: %s\n' "$label" "$path" >&2
    return 64
  fi
}

hepta_safe_require_regular_target() {
  local label="$1" path="$2"
  if [[ -L "$path" || ( -e "$path" && ! -f "$path" ) ]]; then
    printf '%s is not a safe regular-file target: %s\n' "$label" "$path" >&2
    return 64
  fi
}

hepta_safe_revalidate_directory() {
  local label="$1" path="$2"
  if [[ ! -d "$path" || -L "$path" \
    || "$(hepta_safe_absolute_path "$path")" != "$(hepta_safe_canonical_path "$path")" ]]; then
    printf '%s changed during initialization: %s\n' "$label" "$path" >&2
    return 64
  fi
}

hepta_safe_atomic_replace() {
  local source_path="$1" destination_path="$2" label="${3:-release output}"
  local destination_dir temporary_path
  destination_dir="$(/usr/bin/dirname "$destination_path")"
  hepta_safe_revalidate_directory "$label parent" "$destination_dir"
  hepta_safe_require_regular_target "$label" "$destination_path"
  temporary_path="$(/usr/bin/mktemp "$destination_dir/.hepta-safe-output.XXXXXX")"
  if ! /bin/cp "$source_path" "$temporary_path" || [[ ! -f "$temporary_path" || -L "$temporary_path" ]]; then
    /bin/rm -f "$temporary_path"
    return 1
  fi
  if ! "$HEPTA_SAFE_ENV" -i PATH="$HEPTA_SAFE_PATH" HOME=/var/empty TMPDIR=/private/tmp \
    "$HEPTA_SAFE_RUBY" -e '
      source, destination, expected_parent = ARGV
      parent = File.dirname(destination)
      abort "output parent changed" unless File.realpath(parent) == expected_parent
      stat = File.lstat(destination) rescue nil
      abort "unsafe destination" if stat && (!stat.file? || stat.symlink?)
      File.rename(source, destination)
    ' "$temporary_path" "$destination_path" "$destination_dir"; then
    /bin/rm -f "$temporary_path"
    return 1
  fi
}

hepta_safe_unlink_regular_target() {
  local destination_path="$1" label="${2:-release output}"
  local destination_dir
  destination_dir="$(/usr/bin/dirname "$destination_path")"
  hepta_safe_revalidate_directory "$label parent" "$destination_dir"
  "$HEPTA_SAFE_ENV" -i PATH="$HEPTA_SAFE_PATH" HOME=/var/empty TMPDIR=/private/tmp \
    "$HEPTA_SAFE_RUBY" -e '
      destination, expected_parent = ARGV
      abort "output parent changed" unless File.realpath(File.dirname(destination)) == expected_parent
      stat = File.lstat(destination) rescue nil
      exit 0 unless stat
      abort "unsafe destination" unless stat.file? && !stat.symlink?
      File.unlink(destination)
    ' "$destination_path" "$destination_dir"
}

hepta_safe_require_owned_json_target_or_absent() {
  local destination_path="$1" label="$2"
  shift 2
  if ! "$HEPTA_SAFE_ENV" -i PATH="$HEPTA_SAFE_PATH" HOME=/var/empty TMPDIR=/private/tmp \
    "$HEPTA_SAFE_RUBY" -rjson -e '
      destination, label, *pairs = ARGV
      stat = File.lstat(destination) rescue nil
      exit 0 unless stat
      abort "#{label} is not an owned regular JSON target" unless stat.file? && !stat.symlink? && stat.nlink == 1
      flags = File::RDONLY | (File.const_defined?(:NOFOLLOW) ? File::NOFOLLOW : 0)
      File.open(destination, flags) do |file|
        opened = file.stat
        abort "#{label} changed before ownership validation" unless opened.dev == stat.dev && opened.ino == stat.ino && opened.nlink == 1
        document = JSON.parse(file.read)
        pairs.each_slice(2) do |key, expected|
          abort "#{label} ownership marker mismatch" unless document[key].to_s == expected
        end
      end
      after = File.lstat(destination)
      abort "#{label} changed during ownership validation" unless after.dev == stat.dev && after.ino == stat.ino && after.nlink == 1
    ' "$destination_path" "$label" "$@"; then
    return 64
  fi
}

hepta_safe_unlink_owned_json_target_if_present() {
  local destination_path="$1" label="$2"
  shift 2
  local destination_dir
  destination_dir="$(/usr/bin/dirname "$destination_path")"
  hepta_safe_revalidate_directory "$label parent" "$destination_dir"
  if ! "$HEPTA_SAFE_ENV" -i PATH="$HEPTA_SAFE_PATH" HOME=/var/empty TMPDIR=/private/tmp \
    "$HEPTA_SAFE_RUBY" -rjson -e '
      destination, expected_parent, label, *pairs = ARGV
      abort "#{label} parent changed" unless File.realpath(File.dirname(destination)) == expected_parent
      stat = File.lstat(destination) rescue nil
      exit 0 unless stat
      abort "#{label} is not an owned regular JSON target" unless stat.file? && !stat.symlink? && stat.nlink == 1
      flags = File::RDONLY | (File.const_defined?(:NOFOLLOW) ? File::NOFOLLOW : 0)
      File.open(destination, flags) do |file|
        opened = file.stat
        abort "#{label} changed before ownership validation" unless opened.dev == stat.dev && opened.ino == stat.ino && opened.nlink == 1
        document = JSON.parse(file.read)
        pairs.each_slice(2) do |key, expected|
          abort "#{label} ownership marker mismatch" unless document[key].to_s == expected
        end
      end
      after = File.lstat(destination)
      abort "#{label} changed during ownership validation" unless after.dev == stat.dev && after.ino == stat.ino && after.nlink == 1
      File.unlink(destination)
    ' "$destination_path" "$destination_dir" "$label" "$@"; then
    return 64
  fi
}

hepta_safe_atomic_replace_owned_json() {
  local source_path="$1" destination_path="$2" label="$3"
  shift 3
  local destination_dir temporary_path
  destination_dir="$(/usr/bin/dirname "$destination_path")"
  hepta_safe_revalidate_directory "$label parent" "$destination_dir"
  temporary_path="$(/usr/bin/mktemp "$destination_dir/.hepta-safe-owned-json.XXXXXX")"
  if ! /bin/cp "$source_path" "$temporary_path"; then
    /bin/rm -f "$temporary_path"
    return 1
  fi
  if ! "$HEPTA_SAFE_ENV" -i PATH="$HEPTA_SAFE_PATH" HOME=/var/empty TMPDIR=/private/tmp \
    "$HEPTA_SAFE_RUBY" -rjson -e '
      temporary, destination, expected_parent, label, *pairs = ARGV
      abort "#{label} parent changed" unless File.realpath(File.dirname(destination)) == expected_parent
      temporary_stat = File.lstat(temporary)
      abort "unsafe #{label} temporary" unless temporary_stat.file? && !temporary_stat.symlink? && temporary_stat.nlink == 1
      source_document = JSON.parse(File.binread(temporary))
      pairs.each_slice(2) do |key, expected|
        abort "#{label} source ownership marker mismatch" unless source_document[key].to_s == expected
      end
      destination_stat = File.lstat(destination) rescue nil
      if destination_stat
        abort "#{label} is not an owned regular JSON target" unless destination_stat.file? && !destination_stat.symlink? && destination_stat.nlink == 1
        flags = File::RDONLY | (File.const_defined?(:NOFOLLOW) ? File::NOFOLLOW : 0)
        File.open(destination, flags) do |file|
          opened = file.stat
          abort "#{label} changed before ownership validation" unless opened.dev == destination_stat.dev && opened.ino == destination_stat.ino && opened.nlink == 1
          destination_document = JSON.parse(file.read)
          pairs.each_slice(2) do |key, expected|
            abort "#{label} ownership marker mismatch" unless destination_document[key].to_s == expected
          end
        end
        after = File.lstat(destination)
        abort "#{label} changed before replacement" unless after.dev == destination_stat.dev && after.ino == destination_stat.ino && after.nlink == 1
      end
      File.rename(temporary, destination)
    ' "$temporary_path" "$destination_path" "$destination_dir" "$label" "$@"; then
    /bin/rm -f "$temporary_path"
    return 64
  fi
}
