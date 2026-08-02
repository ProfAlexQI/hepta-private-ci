# Shared Bash 3.2-compatible helpers for canonical, no-follow output paths.

hepta_safe_output_error() {
  printf 'unsafe output path: %s\n' "$1" >&2
  return 64
}

hepta_safe_output_normalize_absolute() {
  local base_dir="$1"
  local input_path="$2"
  local part
  local normalized=""
  local index
  local -a input_parts=()
  local -a output_parts=()

  [[ -n "$input_path" ]] || return 64
  if [[ "$input_path" != /* ]]; then
    input_path="$base_dir/$input_path"
  fi
  IFS='/' read -r -a input_parts <<<"$input_path"
  for part in "${input_parts[@]}"; do
    case "$part" in
      ''|.) ;;
      ..)
        if (( ${#output_parts[@]} > 0 )); then
          index=$((${#output_parts[@]} - 1))
          unset 'output_parts[index]'
        fi
        ;;
      *) output_parts+=("$part") ;;
    esac
  done
  for part in "${output_parts[@]}"; do
    normalized="$normalized/$part"
  done
  [[ -n "$normalized" ]] || normalized="/"
  printf '%s\n' "$normalized"
}

hepta_safe_output_canonicalize_leaf() {
  local lexical_path="$1"
  local parent_path
  local leaf_name
  local probe
  local suffix=""
  local component
  local physical_probe

  [[ "$lexical_path" != "/" ]] || { printf '/\n'; return 0; }
  parent_path="${lexical_path%/*}"
  leaf_name="${lexical_path##*/}"
  [[ -n "$parent_path" ]] || parent_path="/"
  probe="$parent_path"
  while [[ ! -e "$probe" && ! -L "$probe" ]]; do
    [[ "$probe" != "/" ]] || break
    component="${probe##*/}"
    suffix="/$component$suffix"
    probe="${probe%/*}"
    [[ -n "$probe" ]] || probe="/"
  done
  [[ -d "$probe" ]] || return 64
  physical_probe="$(cd "$probe" 2>/dev/null && pwd -P)" || return 64
  hepta_safe_output_normalize_absolute "/" "$physical_probe$suffix/$leaf_name"
}

hepta_safe_output_require_regular_or_absent() {
  local label="$1"
  local path="$2"
  if [[ -L "$path" ]]; then
    hepta_safe_output_error "$label must not be a symlink: $path"
    return 64
  fi
  if [[ -e "$path" && ! -f "$path" ]]; then
    hepta_safe_output_error "$label must be a regular file or absent: $path"
  fi
}

hepta_safe_output_resolve_file() {
  local base_dir="$1"
  local label="$2"
  local input_path="$3"
  local lexical_path
  local canonical_path

  lexical_path="$(hepta_safe_output_normalize_absolute "$base_dir" "$input_path")" \
    || { hepta_safe_output_error "$label is invalid: $input_path"; return 64; }
  hepta_safe_output_require_regular_or_absent "$label" "$lexical_path" || return 64
  canonical_path="$(hepta_safe_output_canonicalize_leaf "$lexical_path")" \
    || { hepta_safe_output_error "$label has a non-directory ancestor: $lexical_path"; return 64; }
  hepta_safe_output_require_regular_or_absent "$label" "$canonical_path" || return 64
  HEPTA_SAFE_OUTPUT_PATH="$canonical_path"
}

hepta_safe_output_resolve_directory() {
  local base_dir="$1"
  local label="$2"
  local input_path="$3"
  local lexical_path
  local canonical_path

  lexical_path="$(hepta_safe_output_normalize_absolute "$base_dir" "$input_path")" \
    || { hepta_safe_output_error "$label is invalid: $input_path"; return 64; }
  [[ "$lexical_path" != "/" ]] || { hepta_safe_output_error "$label must not be the filesystem root"; return 64; }
  [[ ! -L "$lexical_path" ]] || { hepta_safe_output_error "$label must not be a symlink: $lexical_path"; return 64; }
  if [[ -e "$lexical_path" && ! -d "$lexical_path" ]]; then
    hepta_safe_output_error "$label must be a directory or absent: $lexical_path"
    return 64
  fi
  canonical_path="$(hepta_safe_output_canonicalize_leaf "$lexical_path")" \
    || { hepta_safe_output_error "$label has a non-directory ancestor: $lexical_path"; return 64; }
  [[ ! -L "$canonical_path" ]] || { hepta_safe_output_error "$label must not be a symlink: $canonical_path"; return 64; }
  HEPTA_SAFE_OUTPUT_PATH="$canonical_path"
}

hepta_safe_output_path_within() {
  local path="$1"
  local root="$2"
  [[ "$path" == "$root" || "$path" == "$root"/* ]]
}

hepta_safe_output_prepare_parent() {
  local path="$1"
  local parent_path="${path%/*}"
  local physical_parent
  [[ -n "$parent_path" ]] || parent_path="/"
  mkdir -p "$parent_path" || return 1
  [[ -d "$parent_path" && ! -L "$parent_path" ]] || return 64
  physical_parent="$(cd "$parent_path" 2>/dev/null && pwd -P)" || return 64
  [[ "$physical_parent" == "$parent_path" ]] || return 64
}

hepta_safe_output_make_temp() {
  local destination="$1"
  local prefix="${2:-.hepta-output}"
  local parent_path="${destination%/*}"
  [[ -n "$parent_path" ]] || parent_path="/"
  hepta_safe_output_prepare_parent "$destination" || return $?
  hepta_safe_output_require_regular_or_absent "destination" "$destination" || return 64
  HEPTA_SAFE_OUTPUT_TEMP="$(mktemp "$parent_path/${prefix}.XXXXXX")" || return 1
}

hepta_safe_output_install_temp() {
  local temporary="$1"
  local destination="$2"
  [[ -f "$temporary" && ! -L "$temporary" ]] || return 64
  hepta_safe_output_require_regular_or_absent "destination" "$destination" || return 64
  if ! /bin/mv -f "$temporary" "$destination"; then
    return 1
  fi
  [[ -f "$destination" && ! -L "$destination" ]] || return 64
}

hepta_safe_output_atomic_write_text() {
  local destination="$1"
  local content="$2"
  local temporary=""
  hepta_safe_output_make_temp "$destination" ".hepta-output" || return $?
  temporary="$HEPTA_SAFE_OUTPUT_TEMP"
  if ! printf '%s\n' "$content" >"$temporary"; then
    rm -f "$temporary"
    return 1
  fi
  if ! hepta_safe_output_install_temp "$temporary" "$destination"; then
    rm -f "$temporary"
    return 1
  fi
}
