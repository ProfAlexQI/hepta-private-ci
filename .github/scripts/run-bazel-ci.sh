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
# fallback remains a non-qualifying diagnostic. Only the keyless Windows
# gnullvm lane is rewritten below.
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

has_bazel_arg_prefix() {
  local prefix="$1"
  local arg
  for arg in "${bazel_args[@]}"; do
    if [[ "$arg" == "$prefix"* ]]; then
      return 0
    fi
  done
  return 1
}

local_defaults=(
  --config=ci-windows
  --build_metadata=TAG_windows_cross_compile=true
  --build_metadata=TAG_windows_gnullvm_local=true
)

if ! has_bazel_arg_prefix --host_platform=; then
  local_defaults+=(--host_platform=//:local_windows_msvc)
fi
if ! has_bazel_arg_prefix --platforms=; then
  local_defaults+=(--platforms=//:windows_x86_64_gnullvm)
fi
if ! has_bazel_arg_prefix --extra_execution_platforms=; then
  local_defaults+=(--extra_execution_platforms=//:windows_x86_64_msvc)
fi
if ! has_bazel_arg_prefix --extra_toolchains=; then
  local_defaults+=(--extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain)
fi
if ! has_bazel_arg_prefix --strategy=TestRunner=; then
  local_defaults+=(--strategy=TestRunner=local)
fi
if ! has_bazel_arg_prefix --strategy=V8Mksnapshot=; then
  local_defaults+=(--strategy=V8Mksnapshot=local)
fi
if ! has_bazel_arg_prefix --jobs=; then
  local_defaults+=(--jobs=8)
fi
if ! has_bazel_arg_prefix --local_test_jobs=; then
  local_defaults+=(--local_test_jobs=8)
fi
if ! has_bazel_arg_prefix --test_env=RUST_TEST_THREADS=; then
  local_defaults+=(--test_env=RUST_TEST_THREADS=1)
fi
if ! has_bazel_arg_prefix --test_env=CODEX_BAZEL_TEST_SKIP_FILTERS=; then
  local_defaults+=(
    --test_env=CODEX_BAZEL_TEST_SKIP_FILTERS=command_safety::powershell_parser::tests::,suite::code_mode::code_mode_can_call_hidden_dynamic_tools,tests::windows_tests::conpty_ctrl_c_interrupts_powershell_foreground_child
  )
fi

# Put the local CI contract immediately after the Bazel command. Explicit
# caller flags remain later on the command line and therefore stay authoritative.
local_bazel_args=(
  "${bazel_args[@]:0:$((command_index + 1))}"
  "${local_defaults[@]}"
  "${bazel_args[@]:$((command_index + 1))}"
)

echo "BuildBuddy API key is not available; using local Windows gnullvm target execution with an MSVC host platform."
exec "$delegate" \
  "${wrapper_args[@]}" \
  -- \
  "${local_bazel_args[@]}" \
  -- \
  "${bazel_targets[@]}"
