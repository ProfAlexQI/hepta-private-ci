#!/usr/bin/env bash

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
delegate="${script_dir}/run-bazel-ci-core.sh"

windows_cross_compile=0
for arg in "$@"; do
  if [[ "$arg" == "--" ]]; then
    break
  fi
  if [[ "$arg" == "--windows-cross-compile" ]]; then
    windows_cross_compile=1
    break
  fi
done

# Authenticated jobs keep the existing Linux-RBE path. An explicit MSVC
# fallback remains a non-qualifying diagnostic. Only the keyless Windows lane
# is rewritten into an MSVC execution platform plus a truthful gnullvm target.
if [[ "${RUNNER_OS:-}" != "Windows" || $windows_cross_compile -ne 1 || -n "${BUILDBUDDY_API_KEY:-}" || "${ALLOW_WINDOWS_MSVC_FALLBACK:-}" == "1" ]]; then
  exec "$delegate" "$@"
fi

wrapper_args=()
bazel_args=()
bazel_targets=()
phase=wrapper
for arg in "$@"; do
  case "$phase" in
    wrapper)
      if [[ "$arg" == "--" ]]; then
        phase=bazel
      elif [[ "$arg" != "--windows-cross-compile" ]]; then
        wrapper_args+=("$arg")
      fi
      ;;
    bazel)
      if [[ "$arg" == "--" ]]; then
        phase=targets
      else
        bazel_args+=("$arg")
      fi
      ;;
    targets)
      bazel_targets+=("$arg")
      ;;
  esac
done

if [[ "$phase" != "targets" || ${#bazel_args[@]} -eq 0 || ${#bazel_targets[@]} -eq 0 ]]; then
  echo "Expected Bazel args and targets separated by --" >&2
  exit 1
fi

command_index=-1
for index in "${!bazel_args[@]}"; do
  if [[ "${bazel_args[$index]}" != -* ]]; then
    command_index=$index
    break
  fi
done
if [[ $command_index -lt 0 ]]; then
  echo "Expected a Bazel command" >&2
  exit 1
fi

local_defaults=()

contains_exact_bazel_arg() {
  local expected="$1"
  local arg
  for arg in "${bazel_args[@]}"; do
    if [[ "$arg" == "$expected" ]]; then
      return 0
    fi
  done
  return 1
}

ensure_exact_semantic_option() {
  local option_prefix="$1"
  local exact_value="$2"
  local bare_option="${option_prefix%=}"
  local seen=0
  local arg

  for arg in "${bazel_args[@]}"; do
    if [[ "$arg" == "$bare_option" ]]; then
      echo "Keyless Windows gnullvm execution requires ${exact_value}; refusing split-form override: ${arg}" >&2
      exit 1
    fi
    if [[ "$arg" == "$option_prefix"* ]]; then
      if [[ "$arg" != "$exact_value" ]]; then
        echo "Keyless Windows gnullvm execution requires ${exact_value}; refusing incompatible override: ${arg}" >&2
        exit 1
      fi
      seen=$((seen + 1))
    fi
  done

  if [[ $seen -gt 1 ]]; then
    echo "Keyless Windows gnullvm execution requires exactly one ${exact_value}; refusing duplicate semantic options" >&2
    exit 1
  fi
  if [[ $seen -eq 0 ]]; then
    local_defaults+=("$exact_value")
  fi
}

if ! contains_exact_bazel_arg --config=ci-windows; then
  local_defaults+=(--config=ci-windows)
fi
if ! contains_exact_bazel_arg --build_metadata=TAG_windows_cross_compile=true; then
  local_defaults+=(--build_metadata=TAG_windows_cross_compile=true)
fi
if ! contains_exact_bazel_arg --build_metadata=TAG_windows_gnullvm_local=true; then
  local_defaults+=(--build_metadata=TAG_windows_gnullvm_local=true)
fi

ensure_exact_semantic_option \
  --host_platform= \
  --host_platform=//:local_windows_msvc
ensure_exact_semantic_option \
  --platforms= \
  --platforms=//:windows_x86_64_gnullvm
ensure_exact_semantic_option \
  --extra_execution_platforms= \
  --extra_execution_platforms=//:windows_x86_64_msvc
ensure_exact_semantic_option \
  --extra_toolchains= \
  --extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain
ensure_exact_semantic_option \
  --strategy=TestRunner= \
  --strategy=TestRunner=local
ensure_exact_semantic_option \
  --strategy=V8Mksnapshot= \
  --strategy=V8Mksnapshot=local

if ! printf '%s\n' "${bazel_args[@]}" | grep -q '^--jobs='; then
  local_defaults+=(--jobs=8)
fi
if ! printf '%s\n' "${bazel_args[@]}" | grep -q '^--local_test_jobs='; then
  local_defaults+=(--local_test_jobs=8)
fi
if ! printf '%s\n' "${bazel_args[@]}" | grep -q '^--test_env=RUST_TEST_THREADS='; then
  local_defaults+=(--test_env=RUST_TEST_THREADS=1)
fi

expected_skip_filters='--test_env=CODEX_BAZEL_TEST_SKIP_FILTERS=command_safety::powershell_parser::tests::,suite::code_mode::code_mode_can_call_hidden_dynamic_tools,tests::windows_tests::conpty_ctrl_c_interrupts_powershell_foreground_child'
ensure_exact_semantic_option \
  --test_env=CODEX_BAZEL_TEST_SKIP_FILTERS= \
  "$expected_skip_filters"

# Put the local execution contract immediately after the Bazel command. Any
# later caller options are limited to non-semantic controls such as concurrency;
# host, target, execution-platform, toolchain, and strategy overrides were
# validated above and cannot silently change the qualification surface.
local_bazel_args=(
  "${bazel_args[@]:0:$((command_index + 1))}"
  "${local_defaults[@]}"
  "${bazel_args[@]:$((command_index + 1))}"
)

echo "BuildBuddy API key is not available; using local Windows gnullvm target execution with an MSVC host and execution platform."
exec "$delegate" \
  "${wrapper_args[@]}" \
  -- \
  "${local_bazel_args[@]}" \
  -- \
  "${bazel_targets[@]}"
