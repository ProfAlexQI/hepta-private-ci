#!/usr/bin/env python3

import sys
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from unittest.mock import patch

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

import run_bazel_with_buildbuddy as subject

CI_WINDOWS_PATH = r"C:\Program Files\Git\bin;C:\Windows\System32"
GITHUB_SHA = "a" * 40
CI_TEST_FILTERS = (
    "--test_env=CODEX_BAZEL_TEST_SKIP_FILTERS="
    "command_safety::powershell_parser::tests::,"
    "suite::code_mode::code_mode_can_call_hidden_dynamic_tools,"
    "tests::windows_tests::conpty_ctrl_c_interrupts_powershell_foreground_child"
)


class ClosedWorldCommandQualificationTest(unittest.TestCase):
    def fixture(self) -> tuple[TemporaryDirectory[str], Path, dict[str, str]]:
        temp = TemporaryDirectory()
        workspace = Path(temp.name)
        bazelrc = workspace / ".bazelrc"
        bazelrc.write_text(
            "common --enable_platform_specific_config\n"
            "common:ci-windows --config=ci\n"
            "try-import %workspace%/user.bazelrc\n",
            encoding="utf-8",
        )
        env = {
            "GITHUB_ACTIONS": "true",
            "RUNNER_OS": "Windows",
            "GITHUB_WORKSPACE": str(workspace),
            "GITHUB_SHA": GITHUB_SHA,
            "CODEX_BAZEL_WINDOWS_PATH": CI_WINDOWS_PATH,
            "CODEX_BAZEL_BIN": "bazel-test",
        }
        return temp, bazelrc, env

    @staticmethod
    def common_options() -> list[str]:
        return [
            "--host_platform=//:local_windows_msvc",
            "--platforms=//:windows_x86_64_gnullvm",
            "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=0",
            "--extra_execution_platforms=//:windows_x86_64_msvc",
            "--extra_toolchains="
            "//:windows_gnullvm_tests_on_msvc_host_toolchain,"
            "//bazel/toolchains/windows:local_msvc_cc_toolchain",
            "--strategy=TestRunner=local",
            "--strategy=V8Mksnapshot=local",
            "--local_test_jobs=8",
            "--jobs=8",
            "--test_env=RUST_TEST_THREADS=1",
            CI_TEST_FILTERS,
            "--build_metadata=TAG_windows_gnullvm_local=true",
            f"--action_env=PATH={CI_WINDOWS_PATH}",
            f"--host_action_env=PATH={CI_WINDOWS_PATH}",
            f"--test_env=PATH={CI_WINDOWS_PATH}",
            f"--build_metadata=COMMIT_SHA={GITHUB_SHA}",
        ]

    @classmethod
    def clippy_args(cls, *extra: str) -> list[str]:
        return [
            "build",
            "--config=clippy",
            "--config=ci-windows",
            *cls.common_options(),
            "--skip_incompatible_explicit_targets",
            "--build_metadata=TAG_job=clippy",
            *extra,
            "--",
            "//codex-rs/utils/rustls-provider:rustls-provider-provider-test",
        ]

    @classmethod
    def release_args(cls, *extra: str) -> list[str]:
        return [
            "build",
            "--config=ci-windows",
            *cls.common_options(),
            "--compilation_mode=fastbuild",
            "--@rules_rust//rust/settings:extra_rustc_flag=-Cdebug-assertions=no",
            "--@rules_rust//rust/settings:extra_exec_rustc_flag=-Cdebug-assertions=no",
            "--build_metadata=TAG_job=verify-release-build",
            "--build_metadata=TAG_rust_debug_assertions=off",
            *extra,
            "--",
            "//codex-rs/...",
            "-//codex-rs/core/tests/remote_env_windows:smoke-test",
            "-//codex-rs/v8-poc:all",
        ]

    @classmethod
    def test_args(cls, *extra: str) -> list[str]:
        return [
            "test",
            "--config=ci-windows",
            *cls.common_options(),
            "--test_tag_filters=-argument-comment-lint",
            "--skip_incompatible_explicit_targets",
            "--test_verbose_timeout_warnings",
            "--remote_download_toplevel",
            "--build_metadata=TAG_windows_test_shard=1",
            *extra,
            "--",
            "//codex-rs/uds:uds-unit-tests-bin",
        ]

    def run_exact(
        self,
        args: list[str],
        *,
        startup: tuple[str, ...] = (),
        env_updates: dict[str, str] | None = None,
    ) -> list[str]:
        temp, bazelrc, env = self.fixture()
        self.addCleanup(temp.cleanup)
        if env_updates:
            env.update(env_updates)
        expected_blob = subject._git_blob_sha1(bazelrc.read_bytes())
        with patch.object(
            subject,
            "QUALIFICATION_BAZELRC_GIT_BLOB_SHA1",
            expected_blob,
        ):
            return subject.bazel_command(*startup, *args, env=env)

    def test_canonical_clippy_command_passes(self) -> None:
        self.run_exact(self.clippy_args())

    def test_canonical_release_command_passes(self) -> None:
        self.run_exact(self.release_args())

    def test_canonical_test_command_passes(self) -> None:
        self.run_exact(
            self.test_args(),
            env_updates={"BAZEL_TEST_SHARD": "1", "BAZEL_TEST_SHARD_COUNT": "4"},
        )

    def test_announce_rc_disable_alias_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "final command requires exactly"):
            self.run_exact(self.clippy_args("--announce_rc", "--noannounce_rc"))

    def test_announce_rc_false_form_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "final command requires exactly"):
            self.run_exact(self.clippy_args("--announce_rc=false"))

    def test_platform_specific_config_override_fails_closed(self) -> None:
        for option in (
            "--noenable_platform_specific_config",
            "--enable_platform_specific_config=false",
        ):
            with self.subTest(option=option):
                with self.assertRaisesRegex(
                    ValueError, "unrecognized final Bazel options"
                ):
                    self.run_exact(self.clippy_args(option))

    def test_invocation_policy_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "unrecognized final Bazel options"):
            self.run_exact(self.clippy_args("--invocation_policy={}"))

    def test_dependency_override_families_fail_closed(self) -> None:
        for option in (
            "--override_repository=rules_rust=C:/tmp/rules_rust",
            "--override_module=rules_rust=C:/tmp/rules_rust",
            "--registry=https://example.invalid/bcr",
        ):
            with self.subTest(option=option):
                with self.assertRaisesRegex(
                    ValueError, "unrecognized final Bazel options"
                ):
                    self.run_exact(self.clippy_args(option))

    def test_arbitrary_define_and_starlark_setting_fail_closed(self) -> None:
        for option in (
            "--define=skip_required_tests=true",
            "--@attacker//:setting=enabled",
        ):
            with self.subTest(option=option):
                with self.assertRaisesRegex(
                    ValueError, "unrecognized final Bazel options"
                ):
                    self.run_exact(self.clippy_args(option))

    def test_unrecognized_build_metadata_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "unrecognized build metadata"):
            self.run_exact(self.clippy_args("--build_metadata=ATTACKER=true"))

    def test_commit_metadata_must_match_github_sha(self) -> None:
        args = [
            option
            for option in self.clippy_args()
            if not option.startswith("--build_metadata=COMMIT_SHA=")
        ]
        args.insert(args.index("--"), f"--build_metadata=COMMIT_SHA={'b' * 40}")
        with self.assertRaisesRegex(ValueError, "requires exact commit metadata"):
            self.run_exact(args)

    def test_remote_download_boolean_override_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "download posture permits only"):
            self.run_exact(
                self.test_args("--noremote_download_toplevel"),
                env_updates={"BAZEL_TEST_SHARD": "1", "BAZEL_TEST_SHARD_COUNT": "4"},
            )

    def test_startup_repo_cache_reenable_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "exact startup vector"):
            self.run_exact(
                self.clippy_args(),
                startup=("--experimental_remote_repo_contents_cache",),
            )

    def test_arbitrary_startup_jvm_option_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "exact startup vector"):
            self.run_exact(
                self.clippy_args(),
                startup=("--host_jvm_args=-Xmx4g",),
            )


if __name__ == "__main__":
    unittest.main()
