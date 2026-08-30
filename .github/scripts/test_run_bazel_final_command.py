#!/usr/bin/env python3

import os
import sys
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from unittest.mock import patch

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

import run_bazel_with_buildbuddy as subject


CI_WINDOWS_PATH = r"C:\Program Files\Git\bin;C:\Windows\System32"
CI_TEST_FILTERS = (
    "--test_env=CODEX_BAZEL_TEST_SKIP_FILTERS="
    "command_safety::powershell_parser::tests::,"
    "suite::code_mode::code_mode_can_call_hidden_dynamic_tools,"
    "tests::windows_tests::conpty_ctrl_c_interrupts_powershell_foreground_child"
)


class FinalCommandQualificationTest(unittest.TestCase):
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
            "CODEX_BAZEL_WINDOWS_PATH": CI_WINDOWS_PATH,
            "CODEX_BAZEL_BIN": "bazel-test",
        }
        return temp, bazelrc, env

    @staticmethod
    def exact_args(*extra: str) -> list[str]:
        return [
            "build",
            "--config=clippy",
            "--config=ci-windows",
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
            *extra,
            "--",
            "//codex-rs/utils/rustls-provider:rustls-provider-provider-test",
        ]

    def run_exact(self, args: list[str] | None = None) -> list[str]:
        temp, bazelrc, env = self.fixture()
        self.addCleanup(temp.cleanup)
        expected_blob = subject._git_blob_sha1(bazelrc.read_bytes())
        with patch.object(
            subject,
            "QUALIFICATION_BAZELRC_GIT_BLOB_SHA1",
            expected_blob,
        ):
            return subject.bazel_command(*(args or self.exact_args()), env=env)

    def test_exact_command_is_bound_to_one_reviewed_rc_and_announces_it(self) -> None:
        command = self.run_exact()
        command_idx = command.index("build")
        startup = command[1:command_idx]
        self.assertIn("--nosystem_rc", startup)
        self.assertIn("--nohome_rc", startup)
        self.assertIn("--nomaster_bazelrc", startup)
        self.assertIn("--noworkspace_rc", startup)
        self.assertEqual(
            len([arg for arg in startup if arg.startswith("--bazelrc=")]),
            1,
        )
        self.assertIn("--announce_rc", command)
        self.assertLess(
            command.index("--config=ci-windows"),
            command.index("--host_platform=//:local_windows_msvc"),
        )

    def test_workspace_bazelrc_drift_fails_closed(self) -> None:
        temp, bazelrc, env = self.fixture()
        self.addCleanup(temp.cleanup)
        expected_blob = subject._git_blob_sha1(bazelrc.read_bytes())
        bazelrc.write_text("common --jobs=999\n", encoding="utf-8")
        with patch.object(subject, "QUALIFICATION_BAZELRC_GIT_BLOB_SHA1", expected_blob):
            with self.assertRaisesRegex(ValueError, "Git blob drifted"):
                subject.bazel_command(*self.exact_args(), env=env)

    def test_workspace_user_bazelrc_fails_closed(self) -> None:
        temp, bazelrc, env = self.fixture()
        self.addCleanup(temp.cleanup)
        (Path(temp.name) / "user.bazelrc").write_text(
            "common --jobs=999\n", encoding="utf-8"
        )
        expected_blob = subject._git_blob_sha1(bazelrc.read_bytes())
        with patch.object(subject, "QUALIFICATION_BAZELRC_GIT_BLOB_SHA1", expected_blob):
            with self.assertRaisesRegex(ValueError, "forbids user.bazelrc"):
                subject.bazel_command(*self.exact_args(), env=env)

    def test_split_form_authority_option_fails_closed(self) -> None:
        # The shell wrapper appends the canonical equals-form target but does
        # not remove a caller-supplied split form. The final-command gate must
        # reject the smuggled pair even though the canonical target is present.
        args = self.exact_args("--platforms", "//:attacker_platform")
        with self.assertRaisesRegex(ValueError, "split-form '--platforms'"):
            self.run_exact(args)

    def test_remote_endpoint_injection_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "rejects remote endpoint"):
            self.run_exact(self.exact_args("--remote_executor=grpcs://example.invalid"))

    def test_additional_strategy_injection_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "rejects strategy"):
            self.run_exact(self.exact_args("--strategy=Rustc=remote"))

    def test_additional_action_environment_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "rejects '--action_env=RUSTFLAGS"):
            self.run_exact(
                self.exact_args("--action_env=RUSTFLAGS=-Ctarget-feature=+crt-static")
            )

    def test_duplicate_exact_metadata_fails_closed(self) -> None:
        with self.assertRaisesRegex(ValueError, "requires exactly"):
            self.run_exact(
                self.exact_args("--build_metadata=TAG_windows_gnullvm_local=true")
            )

    def test_caller_rc_control_fails_closed(self) -> None:
        args = ["--bazelrc=attacker.bazelrc", *self.exact_args()]
        with self.assertRaisesRegex(ValueError, "rejects caller rc controls"):
            self.run_exact(args)

    def test_boolean_equals_rc_reenable_fails_closed(self) -> None:
        args = ["--system_rc=true", *self.exact_args()]
        with self.assertRaisesRegex(ValueError, "rejects caller rc controls"):
            self.run_exact(args)

    def test_master_bazelrc_reenable_fails_closed(self) -> None:
        args = ["--master_bazelrc=true", *self.exact_args()]
        with self.assertRaisesRegex(ValueError, "rejects caller rc controls"):
            self.run_exact(args)

    def test_option_smuggling_after_target_separator_fails_closed(self) -> None:
        args = self.exact_args()
        args.append("--remote_executor=grpcs://example.invalid")
        with self.assertRaisesRegex(ValueError, "invalid Bazel target payload"):
            self.run_exact(args)

    def test_release_target_exclusions_remain_canonical_payload(self) -> None:
        args = self.exact_args("--build_metadata=TAG_job=verify-release-build")
        release_targets = [
            "//codex-rs/...",
            "-//codex-rs/core/tests/remote_env_windows:smoke-test",
            "-//codex-rs/v8-poc:all",
        ]
        args[-1:] = release_targets
        command = self.run_exact(args)
        self.assertEqual(command[-len(release_targets) :], release_targets)

    def test_authenticated_windows_path_remains_remote_passthrough(self) -> None:
        args = ["build", "--config=ci-windows-cross", "--", "//codex-rs/cli:codex"]
        env = {
            "RUNNER_OS": "Windows",
            "GITHUB_ACTIONS": "true",
            "BUILDBUDDY_API_KEY": "token",
            "GITHUB_REPOSITORY": "fork/repo",
        }
        command = subject.bazel_command(*args, env=env)
        self.assertIn("--config=buildbuddy-generic-rbe", command)
        self.assertIn("--config=ci-windows-cross", command)
        self.assertNotIn("--noworkspace_rc", command)

    def test_pinned_blob_identity_is_explicit(self) -> None:
        self.assertEqual(
            subject.QUALIFICATION_BAZELRC_GIT_BLOB_SHA1,
            "0736ecbb6e8183b31f0e2739abef901c47235e9d",
        )


if __name__ == "__main__":
    unittest.main()
