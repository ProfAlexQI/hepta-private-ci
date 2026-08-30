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

has_list_entry() {
  local prefix="$1"
  local required="$2"
  local arg value entry
  local -a entries
  for arg in "${bazel_args[@]}"; do
    if [[ "$arg" != "$prefix"* ]]; then
      continue
    fi
    value="${arg#${prefix}}"
    IFS=',' read -r -a entries <<< "$value"
    for entry in "${entries[@]}"; do
      if [[ "$entry" == "$required" ]]; then
        return 0
      fi
    done
  done
  return 1
}

require_or_add_single_option() {
  local prefix="$1"
  local expected="$2"
  local description="$3"
  local arg
  local seen=0
  for arg in "${bazel_args[@]}"; do
    if [[ "$arg" != "$prefix"* ]]; then
      continue
    fi
    seen=1
    if [[ "$arg" != "$expected" ]]; then
      echo "Credential-free Windows ${description} requires ${expected}; refusing conflicting option ${arg}." >&2
      exit 1
    fi
  done
  if [[ $seen -eq 0 ]]; then
    bazel_args+=("$expected")
  fi
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

require_ci_allowed_configs() {
  local arg config
  for arg in "${bazel_args[@]}"; do
    if [[ "$arg" != --config=* ]]; then
      continue
    fi
    config="${arg#--config=}"
    case "$config" in
      ci | ci-bazel | ci-windows | clippy | argument-comment-lint | ci-v8 | rusty-v8-upstream-libcxx | v8-release-compat | v8-target-x64 | v8-target-arm64)
        ;;
      *)
        echo "GitHub Actions keyless Windows gnullvm qualification rejects non-allowlisted Bazel config '$config'." >&2
        exit 1
        ;;
    esac
  done
}

require_ci_exact_list() {
  local prefix="$1"
  shift
  local -a allowed=("$@")
  local -a observed=()
  local -a entries
  local arg value entry allowed_entry existing
  local seen_option=0
  local valid found

  for arg in "${bazel_args[@]}"; do
    if [[ "$arg" != "$prefix"* ]]; then
      continue
    fi
    seen_option=1
    value="${arg#${prefix}}"
    if [[ -z "$value" ]]; then
      echo "GitHub Actions Windows gnullvm qualification rejects an empty '${prefix}' list." >&2
      exit 1
    fi
    IFS=',' read -r -a entries <<< "$value"
    for entry in "${entries[@]}"; do
      if [[ -z "$entry" ]]; then
        echo "GitHub Actions Windows gnullvm qualification rejects an empty entry in '$arg'." >&2
        exit 1
      fi
      valid=0
      for allowed_entry in "${allowed[@]}"; do
        if [[ "$entry" == "$allowed_entry" ]]; then
          valid=1
          break
        fi
      done
      if [[ $valid -ne 1 ]]; then
        echo "GitHub Actions Windows gnullvm qualification rejects non-canonical entry '$entry' in '$arg'." >&2
        exit 1
      fi
      for existing in "${observed[@]}"; do
        if [[ "$entry" == "$existing" ]]; then
          echo "GitHub Actions Windows gnullvm qualification rejects duplicate entry '$entry' in '${prefix}' arguments." >&2
          exit 1
        fi
      done
      observed+=("$entry")
    done
  done

  if [[ $seen_option -eq 0 ]]; then
    return
  fi

  for allowed_entry in "${allowed[@]}"; do
    found=0
    for existing in "${observed[@]}"; do
      if [[ "$allowed_entry" == "$existing" ]]; then
        found=1
        break
      fi
    done
    if [[ $found -ne 1 ]]; then
      echo "GitHub Actions Windows gnullvm qualification requires exact '${prefix}' set; missing '$allowed_entry'." >&2
      exit 1
    fi
  done
}

canonicalize_ci_option() {
  local prefix="$1"
  local canonical="$2"
  local arg
  local -a filtered=()
  for arg in "${bazel_args[@]}"; do
    if [[ "$arg" != "$prefix"* ]]; then
      filtered+=("$arg")
    fi
  done
  bazel_args=("${filtered[@]}" "$canonical")
}

canonicalize_exact_flag() {
  local canonical="$1"
  local arg
  local -a filtered=()
  for arg in "${bazel_args[@]}"; do
    if [[ "$arg" != "$canonical" ]]; then
      filtered+=("$arg")
    fi
  done
  bazel_args=("${filtered[@]}" "$canonical")
}

ci_host_platform="--host_platform=//:local_windows_msvc"
ci_target_platform="--platforms=//:windows_x86_64_gnullvm"
ci_cc_discovery="--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=0"
ci_execution_platforms="--extra_execution_platforms=//:windows_x86_64_msvc"
ci_toolchains="--extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain,//bazel/toolchains/windows:local_msvc_cc_toolchain"
ci_test_runner_strategy="--strategy=TestRunner=local"
ci_v8_strategy="--strategy=V8Mksnapshot=local"
ci_local_test_jobs="--local_test_jobs=8"
ci_jobs="--jobs=8"
ci_test_threads="--test_env=RUST_TEST_THREADS=1"
ci_test_filters="--test_env=CODEX_BAZEL_TEST_SKIP_FILTERS=command_safety::powershell_parser::tests::,suite::code_mode::code_mode_can_call_hidden_dynamic_tools,tests::windows_tests::conpty_ctrl_c_interrupts_powershell_foreground_child"

# GitHub qualification has one exact target/host/toolchain/effect boundary.
# Local non-Actions callers may retain explicit diagnostic overrides.
if [[ "${GITHUB_ACTIONS:-}" == "true" ]]; then
  require_ci_allowed_configs
  require_exact_ci_arg --host_platform= "$ci_host_platform"
  require_exact_ci_arg --platforms= "$ci_target_platform"
  require_exact_ci_arg \
    --repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN= \
    "$ci_cc_discovery"
  require_exact_ci_arg --strategy=TestRunner= "$ci_test_runner_strategy"
  require_exact_ci_arg --strategy=V8Mksnapshot= "$ci_v8_strategy"
  require_exact_ci_arg --local_test_jobs= "$ci_local_test_jobs"
  require_exact_ci_arg --jobs= "$ci_jobs"
  require_exact_ci_arg --test_env=RUST_TEST_THREADS= "$ci_test_threads"
  require_exact_ci_arg \
    --test_env=CODEX_BAZEL_TEST_SKIP_FILTERS= \
    "$ci_test_filters"
  require_ci_exact_list \
    --extra_execution_platforms= \
    //:windows_x86_64_msvc
  require_ci_exact_list \
    --extra_toolchains= \
    //:windows_gnullvm_tests_on_msvc_host_toolchain \
    //bazel/toolchains/windows:local_msvc_cc_toolchain

  # Put the source-controlled local config before every authority-critical
  # command-line option. Bazel expands rc configs in place; canonical options
  # at the tail therefore override single-valued rc defaults, while exact-set
  # validation prevents additive execution/toolchain injection.
  canonicalize_exact_flag "--config=ci-windows"
  canonicalize_ci_option --host_platform= "$ci_host_platform"
  canonicalize_ci_option --platforms= "$ci_target_platform"
  canonicalize_ci_option \
    --repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN= \
    "$ci_cc_discovery"
  canonicalize_ci_option \
    --extra_execution_platforms= \
    "$ci_execution_platforms"
  canonicalize_ci_option --extra_toolchains= "$ci_toolchains"
  canonicalize_ci_option --strategy=TestRunner= "$ci_test_runner_strategy"
  canonicalize_ci_option --strategy=V8Mksnapshot= "$ci_v8_strategy"
  canonicalize_ci_option --local_test_jobs= "$ci_local_test_jobs"
  canonicalize_ci_option --jobs= "$ci_jobs"
  canonicalize_ci_option --test_env=RUST_TEST_THREADS= "$ci_test_threads"
  canonicalize_ci_option \
    --test_env=CODEX_BAZEL_TEST_SKIP_FILTERS= \
    "$ci_test_filters"
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

  if ! has_list_entry \
    "--extra_execution_platforms=" \
    "//:windows_x86_64_msvc"; then
    bazel_args+=("--extra_execution_platforms=//:windows_x86_64_msvc")
  fi
  if ! has_list_entry \
    "--extra_toolchains=" \
    "//:windows_gnullvm_tests_on_msvc_host_toolchain"; then
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
if ! has_list_entry \
  "--extra_toolchains=" \
  "//bazel/toolchains/windows:local_msvc_cc_toolchain"; then
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
