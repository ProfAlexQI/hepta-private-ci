#!/usr/bin/env bash
# shellcheck shell=bash

if [[ -z "${HEPTA_UI_GATE_REQUIREMENT_CONTEXT:-}" ]]; then
  printf 'HEPTA_UI_GATE_REQUIREMENT_CONTEXT is required before sourcing hepta-ui-gate-common-v1.sh\n' >&2
  return 2 2>/dev/null || exit 2
fi
if [[ -z "${HEPTA_UI_REPORT_INPUT_LABEL:-}" ]]; then
  printf 'HEPTA_UI_REPORT_INPUT_LABEL is required before sourcing hepta-ui-gate-common-v1.sh\n' >&2
  return 2 2>/dev/null || exit 2
fi

require_command() {
  local command_name="$1"
  if ! command -v "$command_name" >/dev/null 2>&1; then
    printf '%s is required for %s\n' "$command_name" "$HEPTA_UI_GATE_REQUIREMENT_CONTEXT" >&2
    exit 2
  fi
}

require_report() {
  local report_path="$1"
  if [[ ! -s "$report_path" ]]; then
    printf 'Missing required %s input: %s\n' "$HEPTA_UI_REPORT_INPUT_LABEL" "$report_path" >&2
    exit 1
  fi
  jq empty "$report_path" >/dev/null
}

file_sha256() {
  shasum -a 256 "$1" | awk '{print $1}'
}

file_bytes() {
  wc -c <"$1" | tr -d ' '
}
