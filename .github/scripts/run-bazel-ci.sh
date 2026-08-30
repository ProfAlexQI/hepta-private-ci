#!/usr/bin/env bash

set -euo pipefail

impl="$(dirname "${BASH_SOURCE[0]}")/run-bazel-ci-impl.sh"

is_windows_cross=0
for arg in "$@"; do
  if [[ "$arg" == "--windows-cross-compile" ]]; then
    is_windows_cross=1
    break
  fi
  if [[ "$arg" == "--" ]]; then
    break
  fi
done

# A non-qualifying MSVC fallback is a manual diagnostic only. Never permit an
# ambient Actions variable to turn a gnullvm-labelled required check into MSVC
# evidence.
if [[ "${GITHUB_ACTIONS:-}" == "true" && "${ALLOW_WINDOWS_MSVC_FALLBACK:-}" == "1" ]]; then
  echo "ALLOW_WINDOWS_MSVC_FALLBACK is forbidden in GitHub Actions qualification jobs. Use the manual Windows MSVC non-qualifying diagnostic workflow." >&2
  exit 1
fi

# Authenticated Windows cross jobs retain the existing Linux-RBE path. All
# non-Windows and non-cross invocations are delegated byte-for-byte.
if [[ "${RUNNER_OS:-}" != "Windows" || $is_windows_cross -eq 0 || -n "${BUILDBUDDY_API_KEY:-}" ]]; then
  exec "$impl" "$@"
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
      elif [[ "$arg" == "--windows-cross-compile" ]]; then
        # Exec-transition tools run on the hosted Windows/MSVC platform. The
        # actual target ABI is selected independently below.
        wrapper_args+=("--windows-msvc-host-platform")
      else
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
  echo "Expected wrapper options, Bazel args, and targets separated by --" >&2
  exit 1
fi

has_option_prefix() {
  local prefix="$1"
  local arg
  for arg in "${bazel_args[@]}"; do
    if [[ "$arg" == "$prefix"* ]]; then
      return 0
    fi
  done
  return 1
}

has_exact_option() {
  local expected="$1"
  local arg
  for arg in "${bazel_args[@]}"; do
    if [[ "$arg" == "$expected" ]]; then
      return 0
    fi
  done
  return 1
}

require_or_add_single_option() {
  local prefix="$1"
  local expected="$2"
  local description="$3"
  local arg
  for arg in "${bazel_args[@]}"; do
    if [[ "$arg" == "$prefix"* ]]; then
      if [[ "$arg" != "$expected" ]]; then
        echo "Credential-free Windows ${description} requires ${expected}; refusing conflicting option ${arg}." >&2
        exit 1
      fi
      return
    fi
  done
  bazel_args+=("$expected")
}

require_exact_ci_arg() {
  local prefix="$1"
  local expected="$2"
  local arg
  for arg in "${bazel_args[@]}"; do
    if [[ "$arg" == "$prefix"* && "$arg" != "$expected" ]]; then
      echo "GitHub Actions Windows gnullvm qualification rejects conflicting argument '$arg'; expected '$expected'." >&2
      exit 1
    fi
  done
}

require_ci_list_contains() {
  local prefix="$1"
  local required="$2"
  local arg value entry found
  for arg in "${bazel_args[@]}"; do
    if [[ "$arg" != "$prefix"* ]]; then
      continue
    fi
    value="${arg#${prefix}}"
    found=0
    IFS=',' read -r -a entries <<< "$value"
    for entry in "${entries[@]}"; do
      if [[ "$entry" == "$required" ]]; then
        found=1
        break
      fi
    done
    if [[ $found -ne 1 ]]; then
      echo "GitHub Actions Windows gnullvm qualification requires '$required' in '$arg'." >&2
      exit 1
    fi
  done
}

# GitHub qualification has one exact target/host/toolchain/effect boundary.
# Local non-Actions callers may retain explicit diagnostic overrides.
if [[ "${GITHUB_ACTIONS:-}" == "true" ]]; then
  require_exact_ci_arg --host_platform= --host_platform=//:local_windows_msvc
  require_exact_ci_arg --platforms= --platforms=//:windows_x86_64_gnullvm
  require_exact_ci_arg \
    --repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN= \
    --repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=0
  require_exact_ci_arg --strategy=TestRunner= --strategy=TestRunner=local
  require_exact_ci_arg --strategy=V8Mksnapshot= --strategy=V8Mksnapshot=local
  require_exact_ci_arg --local_test_jobs= --local_test_jobs=8
  require_exact_ci_arg --jobs= --jobs=8
  require_exact_ci_arg --test_env=RUST_TEST_THREADS= --test_env=RUST_TEST_THREADS=1
  require_exact_ci_arg \
    --test_env=CODEX_BAZEL_TEST_SKIP_FILTERS= \
    --test_env=CODEX_BAZEL_TEST_SKIP_FILTERS=command_safety::powershell_parser::tests::,suite::code_mode::code_mode_can_call_hidden_dynamic_tools,tests::windows_tests::conpty_ctrl_c_interrupts_powershell_foreground_child
  require_ci_list_contains \
    --extra_execution_platforms= \
    //:windows_x86_64_msvc
  require_ci_list_contains \
    --extra_toolchains= \
    //:windows_gnullvm_tests_on_msvc_host_toolchain
  require_ci_list_contains \
    --extra_toolchains= \
    //bazel/toolchains/windows:local_msvc_cc_toolchain
fi

# Bazel's hermetic LLVM toolchain is gnullvm-only after Q0.12. Local execution
# therefore uses detected MSVC only for exec tools and hermetic LLVM for the
# actual gnullvm target.
require_or_add_single_option \
  "--host_platform=" \
  "--host_platform=//:local_windows_msvc" \
  "gnullvm execution host"
require_or_add_single_option \
  "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=" \
  "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=0" \
  "local C++ toolchain discovery"

if [[ "${ALLOW_WINDOWS_MSVC_FALLBACK:-}" == "1" ]]; then
  # Non-GitHub callers may explicitly request the isolated non-qualifying MSVC
  # diagnostic. Both host and target are MSVC and the result is never gnullvm
  # evidence.
  require_or_add_single_option \
    "--platforms=" \
    "--platforms=//:local_windows_msvc" \
    "MSVC diagnostic target"
else
  require_or_add_single_option \
    "--platforms=" \
    "--platforms=//:windows_x86_64_gnullvm" \
    "gnullvm target"

  if ! has_exact_option "--extra_execution_platforms=//:windows_x86_64_msvc"; then
    bazel_args+=("--extra_execution_platforms=//:windows_x86_64_msvc")
  fi
  if ! has_exact_option "--extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain"; then
    bazel_args+=("--extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain")
  fi
  if ! has_exact_option "--strategy=TestRunner=local"; then
    bazel_args+=("--strategy=TestRunner=local")
  fi
  if ! has_exact_option "--strategy=V8Mksnapshot=local"; then
    bazel_args+=("--strategy=V8Mksnapshot=local")
  fi
  if ! has_option_prefix "--local_test_jobs="; then
    bazel_args+=("--local_test_jobs=8")
  fi
  if ! has_exact_option "--test_env=RUST_TEST_THREADS=1"; then
    bazel_args+=("--test_env=RUST_TEST_THREADS=1")
  fi
  if ! has_exact_option "--build_metadata=TAG_windows_gnullvm_local=true"; then
    bazel_args+=("--build_metadata=TAG_windows_gnullvm_local=true")
  fi
fi

if ! has_exact_option "--config=ci-windows"; then
  bazel_args+=("--config=ci-windows")
fi
if ! has_exact_option "--extra_toolchains=//bazel/toolchains/windows:local_msvc_cc_toolchain"; then
  bazel_args+=("--extra_toolchains=//bazel/toolchains/windows:local_msvc_cc_toolchain")
fi
if ! has_option_prefix "--jobs="; then
  bazel_args+=("--jobs=8")
fi

exec "$impl" \
  "${wrapper_args[@]}" \
  -- \
  "${bazel_args[@]}" \
  -- \
  "${bazel_targets[@]}"
