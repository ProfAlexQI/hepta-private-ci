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

# The explicit MSVC branch is a local, non-qualifying diagnostic. Never allow
# an ambient workflow variable to turn a gnullvm-labelled GitHub check into
# MSVC evidence.
if [[ "${GITHUB_ACTIONS:-}" == "true" && "${ALLOW_WINDOWS_MSVC_FALLBACK:-}" == "1" ]]; then
  echo "ALLOW_WINDOWS_MSVC_FALLBACK is forbidden in GitHub Actions qualification jobs." >&2
  exit 1
fi

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
        # Build tools execute locally with the installed MSVC host toolchain.
        # The target ABI is selected explicitly below.
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
  local arg found=0
  for arg in "${bazel_args[@]}"; do
    if [[ "$arg" != "$prefix"* ]]; then
      continue
    fi
    if [[ "$arg" != "$expected" ]]; then
      echo "Credential-free Windows ${description} requires ${expected}; refusing conflicting option ${arg}." >&2
      exit 1
    fi
    found=1
  done
  if [[ $found -eq 0 ]]; then
    bazel_args+=("$expected")
  fi
}

# Bazel's hermetic LLVM toolchain is gnullvm-only after Q0.12. Local execution
# therefore needs two independently constrained C/C++ toolchains: detected
# MSVC for exec tools and hermetic LLVM for the actual gnullvm target.
require_or_add_single_option \
  "--host_platform=" \
  "--host_platform=//:local_windows_msvc" \
  "gnullvm execution host"
require_or_add_single_option \
  "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=" \
  "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=0" \
  "local C++ toolchain discovery"

if [[ "${ALLOW_WINDOWS_MSVC_FALLBACK:-}" == "1" ]]; then
  # Preserve Q0.13's explicit, non-qualifying local diagnostic. Unlike the old
  # fallback, this path also binds a real local MSVC C/C++ toolchain.
  require_or_add_single_option \
    "--platforms=" \
    "--platforms=//:local_windows_msvc" \
    "MSVC diagnostic target"
else
  # Default credential-free execution is genuine gnullvm evidence: target
  # Rust/C/C++ actions use the gnullvm ABI, while only exec-transition tools
  # use the separately constrained local MSVC toolchain.
  require_or_add_single_option \
    "--platforms=" \
    "--platforms=//:windows_x86_64_gnullvm" \
    "gnullvm target"
  require_or_add_single_option \
    "--strategy=TestRunner=" \
    "--strategy=TestRunner=local" \
    "test execution strategy"
  require_or_add_single_option \
    "--strategy=V8Mksnapshot=" \
    "--strategy=V8Mksnapshot=local" \
    "V8 snapshot strategy"
  require_or_add_single_option \
    "--test_env=RUST_TEST_THREADS=" \
    "--test_env=RUST_TEST_THREADS=1" \
    "Rust test-thread contract"

  if ! has_exact_option "--extra_execution_platforms=//:windows_x86_64_msvc"; then
    bazel_args+=("--extra_execution_platforms=//:windows_x86_64_msvc")
  fi
  if ! has_exact_option "--extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain"; then
    bazel_args+=("--extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain")
  fi
  if ! has_option_prefix "--local_test_jobs="; then
    bazel_args+=("--local_test_jobs=8")
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
