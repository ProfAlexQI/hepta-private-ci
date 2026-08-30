#!/usr/bin/env bash

set -euo pipefail

impl="$(dirname "${BASH_SOURCE[0]}")/run-bazel-ci-impl.sh"
canonical_skip_filters="command_safety::powershell_parser::tests::,suite::code_mode::code_mode_can_call_hidden_dynamic_tools,tests::windows_tests::conpty_ctrl_c_interrupts_powershell_foreground_child"

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
  local found=0
  local arg
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

reject_ci_owned_prefix() {
  local prefix="$1"
  local arg
  for arg in "${bazel_args[@]}"; do
    if [[ "$arg" == "$prefix"* ]]; then
      echo "GitHub Actions Windows gnullvm qualification owns ${prefix}; refusing caller argument ${arg}." >&2
      exit 1
    fi
  done
}

reject_ci_forbidden_config() {
  local forbidden="$1"
  local arg
  for arg in "${bazel_args[@]}"; do
    if [[ "$arg" == "--config=${forbidden}" ]]; then
      echo "GitHub Actions keyless Windows gnullvm qualification forbids --config=${forbidden}." >&2
      exit 1
    fi
  done
}

if [[ "${GITHUB_ACTIONS:-}" == "true" ]]; then
  # Critical platform, toolchain, effect and environment options are wrapper-
  # owned. Reject even matching caller copies so the canonical options can be
  # appended after all user configs and therefore win Bazel precedence.
  for prefix in \
    "--host_platform=" \
    "--platforms=" \
    "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=" \
    "--extra_execution_platforms=" \
    "--extra_toolchains=" \
    "--strategy=TestRunner=" \
    "--strategy=V8Mksnapshot=" \
    "--local_test_jobs=" \
    "--jobs=" \
    "--test_env=RUST_TEST_THREADS=" \
    "--test_env=CODEX_BAZEL_TEST_SKIP_FILTERS=" \
    "--action_env=" \
    "--host_action_env="; do
    reject_ci_owned_prefix "$prefix"
  done
  for config in \
    ci-windows \
    ci-windows-cross \
    ci-linux \
    ci-macos \
    remote \
    buildbuddy-generic \
    buildbuddy-generic-rbe \
    buildbuddy-openai \
    buildbuddy-openai-rbe; do
    reject_ci_forbidden_config "$config"
  done

  wrapper_args+=("--windows-local-gnullvm")
  bazel_args+=(
    "--config=ci-windows"
    "--host_platform=//:local_windows_msvc"
    "--platforms=//:windows_x86_64_gnullvm"
    "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=0"
    "--extra_execution_platforms=//:windows_x86_64_msvc"
    "--extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain"
    "--extra_toolchains=//bazel/toolchains/windows:local_msvc_cc_toolchain"
    "--strategy=TestRunner=local"
    "--strategy=V8Mksnapshot=local"
    "--local_test_jobs=8"
    "--jobs=8"
    "--test_env=RUST_TEST_THREADS=1"
    "--test_env=CODEX_BAZEL_TEST_SKIP_FILTERS=${canonical_skip_filters}"
    "--build_metadata=TAG_windows_gnullvm_local=true"
  )
else
  # Local non-Actions callers retain the explicit non-qualifying MSVC
  # diagnostic. Ordinary keyless callers still receive a genuine gnullvm
  # target with MSVC restricted to host/exec transitions.
  if ! has_exact_option "--config=ci-windows"; then
    bazel_args+=("--config=ci-windows")
  fi
  require_or_add_single_option \
    "--host_platform=" \
    "--host_platform=//:local_windows_msvc" \
    "gnullvm execution host"
  require_or_add_single_option \
    "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=" \
    "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=0" \
    "local C++ toolchain discovery"

  if [[ "${ALLOW_WINDOWS_MSVC_FALLBACK:-}" == "1" ]]; then
    wrapper_args+=("--windows-msvc-host-platform")
    require_or_add_single_option \
      "--platforms=" \
      "--platforms=//:local_windows_msvc" \
      "MSVC diagnostic target"
  else
    wrapper_args+=("--windows-local-gnullvm")
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

  if ! has_exact_option "--extra_toolchains=//bazel/toolchains/windows:local_msvc_cc_toolchain"; then
    bazel_args+=("--extra_toolchains=//bazel/toolchains/windows:local_msvc_cc_toolchain")
  fi
  if ! has_option_prefix "--jobs="; then
    bazel_args+=("--jobs=8")
  fi
fi

exec "$impl" \
  "${wrapper_args[@]}" \
  -- \
  "${bazel_args[@]}" \
  -- \
  "${bazel_targets[@]}"
