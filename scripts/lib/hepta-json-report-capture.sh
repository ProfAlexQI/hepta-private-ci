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
  # The first argument is a diagnostic label, not part of the report DAG identity.
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

  local max_entries="${HEPTA_JSON_REPORT_CAPTURE_MAX_ENTRIES:-2048}"
  local max_total_bytes="${HEPTA_JSON_REPORT_CAPTURE_MAX_TOTAL_BYTES:-134217728}"
  local max_report_bytes="${HEPTA_JSON_REPORT_CAPTURE_MAX_REPORT_BYTES:-2097152}"
  local budget_name
  local budget_value

  for budget_name in max_entries max_total_bytes max_report_bytes; do
    budget_value="${!budget_name}"
    if [[ ! "$budget_value" =~ ^[1-9][0-9]*$ ]]; then
      echo "invalid JSON report capture budget ${budget_name}=${budget_value}" >&2
      return 1
    fi
  done

  local report_bytes
  report_bytes="$(printf '%s\n' "$report" | wc -c | tr -d '[:space:]')"
  if (( report_bytes > max_report_bytes )); then
    echo "JSON report capture exceeds per-report budget: ${report_bytes} > ${max_report_bytes} bytes" >&2
    return 1
  fi

  local cache_dir
  cache_dir="$(dirname "$cache_file")"
  local entry_count
  local total_bytes
  local replaced_bytes=0
  entry_count="$(find "$cache_dir" -maxdepth 1 -type f -name '*.json' | wc -l | tr -d '[:space:]')"
  total_bytes="$(find "$cache_dir" -maxdepth 1 -type f -name '*.json' -exec wc -c {} + 2>/dev/null \
    | awk '$2 != "total" { total += $1 } END { print total + 0 }')"

  if [[ -f "$cache_file" ]]; then
    replaced_bytes="$(wc -c <"$cache_file" | tr -d '[:space:]')"
  elif (( entry_count >= max_entries )); then
    echo "JSON report capture exceeds DAG entry budget: ${entry_count} >= ${max_entries}" >&2
    return 1
  fi

  if (( total_bytes - replaced_bytes + report_bytes > max_total_bytes )); then
    echo "JSON report capture exceeds DAG byte budget: $((total_bytes - replaced_bytes + report_bytes)) > ${max_total_bytes}" >&2
    return 1
  fi

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
    if ! hepta_json_report_capture_cache_write "$cache_file" "$report"; then
      echo "$command_name could not enter the bounded JSON report DAG cache" >&2
      exit 1
    fi
  fi

  printf '%s\n' "$report"
}
