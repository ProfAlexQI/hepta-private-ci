#!/usr/bin/env bash

set -euo pipefail

impl="$(dirname "${BASH_SOURCE[0]}")/run-bazel-ci-impl.sh"

WINDOWS_GNULLVM_TEST_TAG_FILTERS="-nolinux,-noarm64,-noautodeps,-requires-python,-requires-powershell-parser,-requires-code-mode-powershell,-requires-conpty,-requires-unix-socket-bind,-requires-uds"

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

append_preserving_form() {
  local split="$1"
  local arg="$2"
  local option="$3"
  local value="$4"
  if [[ "$split" == "1" ]]; then
    normalized_bazel_args+=("$option" "$value")
  else
    normalized_bazel_args+=("$arg")
  fi
}

reject_conflicting_option() {
  local option="$1"
  local value="$2"
  local expected="$3"
  local description="$4"
  echo "Credential-free Windows ${description} requires ${option}=${expected}; refusing conflicting option ${option} ${value}." >&2
  exit 1
}

normalize_windows_local_bazel_args() {
  local target_platform="$1"
  local qualification="$2"
  local index arg option value split expected description keep
  normalized_bazel_args=()

  for ((index = 0; index < ${#bazel_args[@]}; index++)); do
    arg="${bazel_args[$index]}"
    option=""
    value=""
    split=0

    case "$arg" in
      --host_platform|--platforms|--repo_env|--extra_execution_platforms|--extra_toolchains|--strategy|--test_env|--test_tag_filters|--config)
        if ((index + 1 >= ${#bazel_args[@]})); then
          echo "Credential-free Windows option ${arg} is missing its value." >&2
          exit 1
        fi
        option="$arg"
        value="${bazel_args[$((index + 1))]}"
        split=1
        index=$((index + 1))
        ;;
      --host_platform=*)
        option="--host_platform"
        value="${arg#--host_platform=}"
        ;;
      --platforms=*)
        option="--platforms"
        value="${arg#--platforms=}"
        ;;
      --repo_env=*)
        option="--repo_env"
        value="${arg#--repo_env=}"
        ;;
      --extra_execution_platforms=*)
        option="--extra_execution_platforms"
        value="${arg#--extra_execution_platforms=}"
        ;;
      --extra_toolchains=*)
        option="--extra_toolchains"
        value="${arg#--extra_toolchains=}"
        ;;
      --strategy=*)
        option="--strategy"
        value="${arg#--strategy=}"
        ;;
      --test_env=*)
        option="--test_env"
        value="${arg#--test_env=}"
        ;;
      --test_tag_filters=*)
        option="--test_tag_filters"
        value="${arg#--test_tag_filters=}"
        ;;
      --config=*)
        option="--config"
        value="${arg#--config=}"
        ;;
    esac

    if [[ -z "$option" ]]; then
      normalized_bazel_args+=("$arg")
      continue
    fi

    expected=""
    description=""
    keep=0

    case "$option" in
      --host_platform)
        expected="//:local_windows_msvc"
        description="gnullvm execution host"
        ;;
      --platforms)
        expected="$target_platform"
        description="target platform"
        ;;
      --repo_env)
        case "$value" in
          BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=*)
            expected="BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=0"
            description="local C++ toolchain discovery"
            ;;
          *)
            keep=1
            ;;
        esac
        ;;
      --extra_execution_platforms)
        if [[ "$qualification" == "1" ]]; then
          expected="//:windows_x86_64_msvc"
          description="gnullvm execution platform"
        else
          keep=1
        fi
        ;;
      --extra_toolchains)
        if [[ "$qualification" == "1" ]]; then
          case "$value" in
            //bazel/toolchains/windows:local_msvc_cc_toolchain|//:windows_gnullvm_tests_on_msvc_host_toolchain)
              expected="$value"
              description="allowlisted qualification toolchain"
              ;;
            *)
              echo "Credential-free Windows qualification forbids competing --extra_toolchains value ${value}." >&2
              exit 1
              ;;
          esac
        elif [[ "$value" == "//bazel/toolchains/windows:local_msvc_cc_toolchain" ]]; then
          expected="$value"
          description="local MSVC C++ toolchain"
        else
          keep=1
        fi
        ;;
      --strategy)
        if [[ "$qualification" == "1" ]]; then
          case "$value" in
            TestRunner=*)
              expected="TestRunner=local"
              description="test execution strategy"
              ;;
            V8Mksnapshot=*)
              expected="V8Mksnapshot=local"
              description="V8 snapshot strategy"
              ;;
            *=local)
              keep=1
              ;;
            *)
              echo "Credential-free Windows qualification forbids non-local --strategy value ${value}." >&2
              exit 1
              ;;
          esac
        else
          keep=1
        fi
        ;;
      --test_env)
        case "$value" in
          RUST_TEST_THREADS=*)
            expected="RUST_TEST_THREADS=1"
            description="Rust test-thread contract"
            ;;
          BAZEL_SH|BAZEL_SH=*)
            expected="BAZEL_SH"
            description="Bazel shell pass-through"
            ;;
          *)
            keep=1
            ;;
        esac
        ;;
      --test_tag_filters)
        if [[ "$qualification" == "1" ]]; then
          expected="$WINDOWS_GNULLVM_TEST_TAG_FILTERS"
          description="gnullvm test-tag filter"
        else
          keep=1
        fi
        ;;
      --config)
        if [[ "$value" == "ci-windows" ]]; then
          expected="ci-windows"
          description="Windows CI config"
        else
          keep=1
        fi
        ;;
    esac

    if [[ "$keep" == "1" ]]; then
      append_preserving_form "$split" "$arg" "$option" "$value"
      continue
    fi

    if [[ "$value" != "$expected" ]]; then
      reject_conflicting_option "$option" "$value" "$expected" "$description"
    fi

    # Canonical protected options are appended exactly once, after every
    # caller-supplied option and config expansion, so no later input can
    # override the qualification tuple.
  done

  bazel_args=("${normalized_bazel_args[@]}")
}

if [[ "${ALLOW_WINDOWS_MSVC_FALLBACK:-}" == "1" ]]; then
  # Preserve Q0.13's explicit, non-qualifying local diagnostic. Unlike the old
  # fallback, this path also binds a real local MSVC C/C++ toolchain.
  normalize_windows_local_bazel_args "//:local_windows_msvc" 0
  bazel_args+=(
    "--config=ci-windows"
    "--host_platform=//:local_windows_msvc"
    "--platforms=//:local_windows_msvc"
    "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=0"
    "--extra_toolchains=//bazel/toolchains/windows:local_msvc_cc_toolchain"
  )
else
  # Default credential-free execution is genuine gnullvm evidence: target
  # Rust/C/C++ actions use the gnullvm ABI, while only exec-transition tools
  # use the separately constrained local MSVC toolchain.
  normalize_windows_local_bazel_args "//:windows_x86_64_gnullvm" 1
  bazel_args+=(
    "--config=ci-windows"
    "--host_platform=//:local_windows_msvc"
    "--platforms=//:windows_x86_64_gnullvm"
    "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=0"
    "--extra_execution_platforms=//:windows_x86_64_msvc"
    "--extra_toolchains=//bazel/toolchains/windows:local_msvc_cc_toolchain"
    "--extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain"
    "--strategy=TestRunner=local"
    "--strategy=V8Mksnapshot=local"
    "--test_env=RUST_TEST_THREADS=1"
    "--test_env=BAZEL_SH"
    "--test_tag_filters=${WINDOWS_GNULLVM_TEST_TAG_FILTERS}"
  )
  if ! has_option_prefix "--local_test_jobs="; then
    bazel_args+=("--local_test_jobs=8")
  fi
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
