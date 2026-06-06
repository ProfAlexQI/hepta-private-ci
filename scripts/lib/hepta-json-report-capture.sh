#!/usr/bin/env bash

extract_first_json_object() {
  awk '
    BEGIN {
      capture = 0
      depth = 0
    }
    {
      if (!capture && $0 ~ /^[[:space:]]*\{[[:space:]]*$/) {
        capture = 1
      }
      if (capture) {
        print
        line = $0
        open_line = line
        close_line = line
        opens = gsub(/\{/, "", open_line)
        closes = gsub(/\}/, "", close_line)
        depth += opens - closes
        if (depth == 0) {
          exit
        }
      }
    }
  '
}

hepta_emit_capture_tail() {
  local output="$1"
  local lines="${HEPTA_JSON_REPORT_CAPTURE_DIAGNOSTIC_LINES:-60}"

  if [[ -z "$output" ]]; then
    echo "(no command output)" >&2
    return
  fi

  printf '%s\n' "$output" | tail -n "$lines" >&2
}

hepta_json_report_capture_cache_file() {
  local command_name="$1"
  shift

  local cache_dir="${HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR:-}"
  if [[ -z "$cache_dir" ]]; then
    return 1
  fi

  mkdir -p "$cache_dir" 2>/dev/null || return 1

  local key
  key="$(
    {
      printf '%s\0' "$PWD"
      printf '%s\0' "${HEPTA_JSON_REPORT_CAPTURE_CACHE_SALT:-}"
      printf '%s\0' "$command_name"
      printf '%s\0' "$@"
    } | shasum -a 256 | awk '{print $1}'
  )"

  printf '%s/%s.json\n' "$cache_dir" "$key"
}

hepta_json_report_capture_cache_read() {
  local cache_file="$1"

  if [[ -s "$cache_file" ]] && jq -e . >/dev/null <"$cache_file"; then
    cat "$cache_file"
    return 0
  fi

  return 1
}

hepta_json_report_capture_cache_write() {
  local cache_file="$1"
  local report="$2"

  local cache_tmp
  cache_tmp="${cache_file}.tmp.$$"

  umask 077
  printf '%s\n' "$report" >"$cache_tmp" 2>/dev/null \
    && mv "$cache_tmp" "$cache_file" 2>/dev/null \
    || {
      rm -f "$cache_tmp" 2>/dev/null || true
      return 1
    }
}

capture_json_report() {
  local command_name="$1"
  shift

  local cache_file=""
  if cache_file="$(hepta_json_report_capture_cache_file "$command_name" "$@")"; then
    if hepta_json_report_capture_cache_read "$cache_file"; then
      return 0
    fi
  fi

  local output
  local rc=0
  output="$("$@" 2>&1)" || rc=$?

  local report
  report="$(printf '%s\n' "$output" | extract_first_json_object)"

  if [[ "$rc" -ne 0 ]]; then
    echo "$command_name failed with exit code $rc" >&2
    if jq -e . >/dev/null <<<"$report"; then
      echo "$command_name emitted a parseable JSON report before failing:" >&2
      printf '%s\n' "$report" >&2
    else
      echo "$command_name output tail:" >&2
      hepta_emit_capture_tail "$output"
    fi
    exit "$rc"
  fi

  if ! jq -e . >/dev/null <<<"$report"; then
    echo "$command_name did not emit a parseable JSON report" >&2
    echo "$command_name output tail:" >&2
    hepta_emit_capture_tail "$output"
    exit 1
  fi

  if [[ -n "$cache_file" ]]; then
    hepta_json_report_capture_cache_write "$cache_file" "$report" || true
  fi

  printf '%s\n' "$report"
}
