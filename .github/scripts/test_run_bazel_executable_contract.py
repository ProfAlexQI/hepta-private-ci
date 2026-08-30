#!/usr/bin/env python3

import hashlib
from pathlib import Path
from unittest.mock import patch

from _q027_test_support import Completed
from _q027_test_support import Q027TestCase
from _q027_test_support import bazelisk_policy
from _q027_test_support import lane_policy
from _q027_test_support import subject
from _q027_test_support import wrapper


class Q027ExecutableContractTest(Q027TestCase):
    def test_q026_clippy_canonical_negative_target_passes(self) -> None:
        args = (
            "build",
            "--config=clippy",
            "--config=ci-windows",
            lane_policy.CANONICAL_SKIP_INCOMPATIBLE,
            lane_policy.CLIPPY_JOB_METADATA,
            "--",
            "//codex-rs/...",
            lane_policy.CANONICAL_CLIPPY_NEGATIVE_TARGET,
        )
        with patch.object(lane_policy, "_validate_q021"):
            lane_policy.validate_keyless_windows_gnullvm_final_args(args, {})

    def test_q026_clippy_arbitrary_negative_target_fails(self) -> None:
        args = (
            "build",
            "--config=clippy",
            "--config=ci-windows",
            lane_policy.CANONICAL_SKIP_INCOMPATIBLE,
            lane_policy.CLIPPY_JOB_METADATA,
            "--",
            "//codex-rs/...",
            "-//codex-rs/core:all",
        )
        with patch.object(lane_policy, "_validate_q021"):
            with self.assertRaisesRegex(ValueError, "rejects negative targets"):
                lane_policy.validate_keyless_windows_gnullvm_final_args(args, {})

    def test_output_base_is_explicit_startup_input(self) -> None:
        temp, env, bazelrc = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        env["CODEX_BAZEL_BIN"] = "bazel-test"
        args = [
            "build",
            "--config=clippy",
            *subject.CI_EXACT_OPTIONS.values(),
            lane_policy.CANONICAL_SKIP_INCOMPATIBLE,
            lane_policy.CLIPPY_JOB_METADATA,
            "--",
            "//codex-rs/...",
            lane_policy.CANONICAL_CLIPPY_NEGATIVE_TARGET,
        ]
        with (
            patch.object(
                wrapper,
                "QUALIFICATION_BAZELRC_GIT_BLOB_SHA1",
                wrapper._git_blob_sha1(bazelrc.read_bytes()),
            ),
            patch.object(wrapper, "validate_keyless_windows_gnullvm_final_args"),
        ):
            command = wrapper.bazel_command(*args, env=env)
        command_idx = command.index("build")
        self.assertEqual(
            command[1:command_idx],
            [
                f"--output_user_root={env['BAZEL_OUTPUT_USER_ROOT']}",
                "--noexperimental_remote_repo_contents_cache",
                f"--output_base={env['BAZEL_OUTPUT_BASE']}",
                "--nomaster_bazelrc",
                "--nosystem_rc",
                "--noworkspace_rc",
                "--nohome_rc",
                f"--bazelrc={bazelrc.resolve()}",
            ],
        )

    def test_caller_startup_option_fails_closed(self) -> None:
        temp, env, bazelrc = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        env["CODEX_BAZEL_BIN"] = "bazel-test"
        args = [
            "--host_jvm_args=-Xmx32g",
            "build",
            "--platforms=//:windows_x86_64_gnullvm",
            "--",
            "//codex-rs/...",
        ]
        with patch.object(
            wrapper,
            "QUALIFICATION_BAZELRC_GIT_BLOB_SHA1",
            wrapper._git_blob_sha1(bazelrc.read_bytes()),
        ):
            with self.assertRaisesRegex(ValueError, "caller startup options"):
                wrapper.bazel_command(*args, env=env)

    def test_resolver_verifies_bazelisk_and_cached_bazel(self) -> None:
        temp, env, _ = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        root = Path(temp.name)
        bazelisk = root / "toolcache" / "bazel.exe"
        bazelisk.parent.mkdir()
        bazelisk.write_bytes(b"bazelisk fixture")
        real, bazel_digest = self.real_bazel(root)
        child_path = f"{real.parent};C:/Windows/System32"
        with (
            patch.object(
                bazelisk_policy,
                "BAZELISK_WINDOWS_X86_64_SHA256",
                self.digest(bazelisk),
            ),
            patch.object(
                bazelisk_policy,
                "BAZEL_WINDOWS_X86_64_SHA256",
                bazel_digest,
            ),
        ):
            command = subject.resolve_verified_bazel_command(
                ["bazel", "build"],
                env,
                which=lambda *_args, **_kwargs: str(bazelisk),
                run=lambda *_args, **_kwargs: Completed(stdout=f"PATH={child_path}\n"),
            )
        self.assertEqual(command[0], str(real.resolve()))
        self.assertEqual(env["PATH"], child_path)

    def test_cached_bazel_is_rehashed_even_when_bazelisk_succeeds(self) -> None:
        temp, env, _ = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        root = Path(temp.name)
        bazelisk = root / "toolcache" / "bazel.exe"
        bazelisk.parent.mkdir()
        bazelisk.write_bytes(b"bazelisk fixture")
        bad_digest = hashlib.sha256(b"expected bazel").hexdigest()
        real = root / "downloads" / "sha256" / bad_digest / "bin" / "bazel.exe"
        real.parent.mkdir(parents=True)
        real.write_bytes(b"tampered cached bazel")
        child_path = f"{real.parent};C:/Windows/System32"
        with (
            patch.object(
                bazelisk_policy,
                "BAZELISK_WINDOWS_X86_64_SHA256",
                self.digest(bazelisk),
            ),
            patch.object(
                bazelisk_policy,
                "BAZEL_WINDOWS_X86_64_SHA256",
                bad_digest,
            ),
        ):
            with self.assertRaisesRegex(
                ValueError, "cached Bazel executable SHA-256 drifted"
            ):
                subject.resolve_verified_bazel_command(
                    ["bazel", "build"],
                    env,
                    which=lambda *_args, **_kwargs: str(bazelisk),
                    run=lambda *_args, **_kwargs: Completed(
                        stdout=f"PATH={child_path}\n"
                    ),
                )

    def test_bazelisk_redirect_override_fails_closed(self) -> None:
        temp, env, _ = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        env["BAZELISK_BASE_URL"] = "https://example.invalid"
        with self.assertRaisesRegex(ValueError, "BAZELISK_BASE_URL"):
            subject.prepare_bazelisk_environment(env)

    def test_home_bazeliskrc_fails_closed(self) -> None:
        temp, env, _ = self.fixture(subject.CLIPPY_JOB)
        self.addCleanup(temp.cleanup)
        Path(env["USERPROFILE"], ".bazeliskrc").write_text(
            "USE_BAZEL_VERSION=latest\n"
        )
        with self.assertRaisesRegex(ValueError, "Bazelisk config file is forbidden"):
            subject.resolve_verified_bazel_command(["bazel", "build"], env)


if __name__ == "__main__":
    import unittest

    unittest.main()
