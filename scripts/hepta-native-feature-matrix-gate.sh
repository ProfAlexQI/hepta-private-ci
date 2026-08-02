#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

REPORT_PATH=""
LOG_DIR=""
TARGET_DIR="${HEPTA_NATIVE_CARGO_TARGET_DIR:-${TMPDIR:-/tmp}/hepta-native-feature-matrix-target}"
while [[ $# -gt 0 ]]; do
  case "$1" in
    --output) REPORT_PATH="${2:-}"; shift 2 ;;
    --log-dir) LOG_DIR="${2:-}"; shift 2 ;;
    --target-dir) TARGET_DIR="${2:-}"; shift 2 ;;
    --help|-h)
      echo "usage: $0 [--output report.json] [--log-dir directory] [--target-dir directory]"
      exit 0
      ;;
    *) echo "unknown argument: $1" >&2; exit 64 ;;
  esac
done

if [[ -z "$LOG_DIR" ]]; then
  if [[ -n "$REPORT_PATH" ]]; then LOG_DIR="$(dirname "$REPORT_PATH")/native-feature-matrix-logs"; else LOG_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-native-feature-matrix.XXXXXX")"; fi
fi
mkdir -p "$LOG_DIR" "$TARGET_DIR"
[[ -z "$REPORT_PATH" ]] || mkdir -p "$(dirname "$REPORT_PATH")"

# shellcheck source=scripts/lib/hepta-ui-rust-toolchain.sh
source scripts/lib/hepta-ui-rust-toolchain.sh
hepta_ui_activate_rust_toolchain

binding_before="$(scripts/hepta-ui-source-fingerprint)"

run_check() {
  local name="$1"
  shift
  local log="$LOG_DIR/$name.log"
  local started ended rc
  started="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  if CARGO_TARGET_DIR="$TARGET_DIR" hepta_ui_cargo check --manifest-path apps/hepta-native/Cargo.toml --locked --offline "$@" >"$log" 2>&1; then rc=0; else rc=$?; fi
  ended="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  jq -n --arg name "$name" --arg log "$log" --arg started "$started" --arg ended "$ended" \
    --argjson exit_code "$rc" '{name:$name,command_exit_code:$exit_code,ready:($exit_code == 0),log_path:$log,started_at_utc:$started,ended_at_utc:$ended}'
}

no_default="$(run_check no-default-features --no-default-features)"
default_features="$(run_check default-features)"
all_features="$(run_check all-features --all-features)"
binding_after="$(scripts/hepta-ui-source-fingerprint)"

stable=false
if [[ "$(jq -r '.head' <<<"$binding_before")" == "$(jq -r '.head' <<<"$binding_after")" \
  && "$(jq -r '.head_tree' <<<"$binding_before")" == "$(jq -r '.head_tree' <<<"$binding_after")" \
  && "$(jq -r '.source_fingerprint' <<<"$binding_before")" == "$(jq -r '.source_fingerprint' <<<"$binding_after")" ]]; then stable=true; fi
matrix_ready=false
if [[ "$stable" == true && "$(jq -r '.ready' <<<"$no_default")" == true \
  && "$(jq -r '.ready' <<<"$default_features")" == true && "$(jq -r '.ready' <<<"$all_features")" == true ]]; then matrix_ready=true; fi
status=not_ready
[[ "$matrix_ready" == true ]] && status=ready

report="$(jq -n --arg status "$status" --arg rust_toolchain "$(hepta_ui_rustc --version)" \
  --argjson before "$binding_before" --argjson after "$binding_after" --argjson stable "$stable" \
  --argjson no_default "$no_default" --argjson default_features "$default_features" --argjson all_features "$all_features" \
  --argjson ready "$matrix_ready" \
  '{schema_version:1,kind:"hepta-native-feature-matrix-gate",status:$status,rust_toolchain:$rust_toolchain,source_binding_before:$before,source_binding:$after,source_stable_during_run:$stable,checks:[$no_default,$default_features,$all_features],feature_matrix_ready:$ready,external_side_effects_performed:false}')"
if [[ -n "$REPORT_PATH" ]]; then printf '%s\n' "$report" >"$REPORT_PATH"; else printf '%s\n' "$report"; fi
[[ "$matrix_ready" == true ]]
