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

capture_json_report() {
  local command_name="$1"
  shift

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

  printf '%s\n' "$report"
}
