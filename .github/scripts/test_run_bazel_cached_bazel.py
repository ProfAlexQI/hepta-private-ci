#!/usr/bin/env python3

import subprocess
import sys
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from unittest.mock import patch

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))

import run_bazel_q029_execution_context as q029
import run_bazel_q030_cached_bazel as subject
import run_bazel_with_buildbuddy as launcher


class CachedBazelExecutionClosureTest(unittest.TestCase):
    def fixture(self, job: str = q029.CLIPPY_JOB):
        temporary = TemporaryDirectory()
        root = Path(temporary.name)
        workspace = root / "workspace"
        home = root / "home"
        workspace.mkdir()
        home.mkdir()
        (workspace / ".bazelversion").write_bytes(q029.BAZELVERSION_BYTES)
        (workspace / ".bazelrc").write_text(
            "common --config=ci\n",
            encoding="utf-8",
        )
        env = {
            "GITHUB_ACTIONS": "true",
            "GITHUB_REPOSITORY": q029.REPOSITORY,
            "GITHUB_JOB": job,
            "GITHUB_SHA": "0123456789abcdef0123456789abcdef01234567",
            "GITHUB_RUN_ID": "42",
            "GITHUB_WORKSPACE": str(workspace),
            "RUNNER_OS": "Windows",
            "RUNNER_ARCH": "X64",
            "RUNNER_ENVIRONMENT": "github-hosted",
            "RUNNER_TEMP": "D:/runner-temp",
            "USERPROFILE": str(home),
            "CI_BUILD_ROOT": "D:/ci",
            "BAZEL_OUTPUT_BASE": "D:/ci/o",
            "BAZEL_OUTPUT_USER_ROOT": "D:/ci/b",
            "BAZEL_REPOSITORY_CACHE": "D:/ci/bazel-repository-cache",
            "BAZEL_REPO_CONTENTS_CACHE": (
                f"D:/ci/bazel-repo-contents-cache-42-{job}"
            ),
            "CODEX_BAZEL_EXECUTION_LOG_COMPACT_DIR": (
                "D:/runner-temp/bazel-execution-logs"
            ),
            "PATH": str(root),
            "BAZELISK_GITHUB_TOKEN": "setup-only-secret",
        }
        if job == q029.TEST_JOB:
            env["BAZEL_TEST_SHARD"] = "2"
            env["BAZEL_TEST_SHARD_COUNT"] = "4"

        bazelisk = root / "toolcache" / "bazel.exe"
        bazelisk.parent.mkdir()
        bazelisk.write_bytes(b"verified Bazelisk fixture")

        real_bazel = (
            root
            / "downloads"
            / "sha256"
            / q029.BAZEL_WINDOWS_X86_64_SHA256
            / "bin"
            / "bazel.exe"
        )
        real_bazel.parent.mkdir(parents=True)
        real_bazel.write_bytes(b"verified cached Bazel fixture")
        return temporary, root, env, bazelisk, real_bazel

    def q029_command(self, env, bazelisk):
        job = env["GITHUB_JOB"]
        command_name = "test" if job == q029.TEST_JOB else "build"
        metadata = [f"{q029.COMMIT_METADATA_PREFIX}{env['GITHUB_SHA']}"]
        if job == q029.TEST_JOB:
            metadata.append(
                f"{q029.TEST_SHARD_METADATA_PREFIX}{env['BAZEL_TEST_SHARD']}"
            )
        elif job == q029.CLIPPY_JOB:
            metadata.append(q029.CLIPPY_JOB_METADATA)
        else:
            metadata.append(q029.RELEASE_JOB_METADATA)

        startup = [
            f"--output_user_root={env['BAZEL_OUTPUT_USER_ROOT']}",
            "--noexperimental_remote_repo_contents_cache",
            "--nomaster_bazelrc",
            "--nosystem_rc",
            "--noworkspace_rc",
            "--nohome_rc",
            f"--bazelrc={Path(env['GITHUB_WORKSPACE']).resolve() / '.bazelrc'}",
        ]
        options = [
            *metadata,
            f"--repo_contents_cache={env['BAZEL_REPO_CONTENTS_CACHE']}",
            f"--repository_cache={env['BAZEL_REPOSITORY_CACHE']}",
            "--execution_log_compact_file="
            f"{env['CODEX_BAZEL_EXECUTION_LOG_COMPACT_DIR']}/"
            f"execution-log-{command_name}-{job}-123.zst",
        ]
        return [
            str(bazelisk.resolve()),
            *startup,
            command_name,
            *options,
            "--",
            "//codex-rs/foo:unit",
        ]

    @staticmethod
    def completed(stdout: str = "", returncode: int = 0):
        return subprocess.CompletedProcess(
            args=["bazelisk", "--print_env"],
            returncode=returncode,
            stdout=stdout,
            stderr="hidden transport diagnostics",
        )

    def prepared(self, env):
        subject.clear_setup_bazel_transport_token(env)
        q029.prepare_bazelisk_environment(env)

    def test_setup_bazel_transport_token_is_consumed(self):
        temporary, _, env, _, _ = self.fixture()
        self.addCleanup(temporary.cleanup)
        self.assertTrue(subject.clear_setup_bazel_transport_token(env))
        self.assertNotIn(subject.SETUP_BAZEL_TRANSPORT_TOKEN, env)
        self.assertFalse(subject.clear_setup_bazel_transport_token(env))

    def test_output_base_is_appended_after_q029_startup(self):
        temporary, _, env, bazelisk, _ = self.fixture()
        self.addCleanup(temporary.cleanup)
        command = subject.bind_output_base_startup(
            self.q029_command(env, bazelisk),
            env,
        )
        command_idx = command.index("build")
        self.assertEqual(
            command[command_idx - 1],
            f"--output_base={env['BAZEL_OUTPUT_BASE']}",
        )

    def test_preexisting_output_base_fails_closed(self):
        temporary, _, env, bazelisk, _ = self.fixture()
        self.addCleanup(temporary.cleanup)
        command = self.q029_command(env, bazelisk)
        command.insert(command.index("build"), "--output_base=C:/attacker")
        with self.assertRaisesRegex(ValueError, "absent before final"):
            subject.bind_output_base_startup(command, env)

    def test_cached_bazel_is_resolved_rehashed_and_bypasses_bazelisk(self):
        temporary, _, env, bazelisk, real_bazel = self.fixture()
        self.addCleanup(temporary.cleanup)
        self.prepared(env)
        child_path = f"{real_bazel.parent};C:/Windows/System32"
        command = subject.resolve_verified_cached_bazel(
            self.q029_command(env, bazelisk),
            env,
            run=lambda *_args, **_kwargs: self.completed(
                stdout=f"PATH={child_path}\n"
            ),
            digest_file=lambda _path: q029.BAZEL_WINDOWS_X86_64_SHA256,
        )
        self.assertEqual(command[0], str(real_bazel.resolve()))
        self.assertEqual(env["PATH"], child_path)
        self.assertNotIn(subject.SETUP_BAZEL_TRANSPORT_TOKEN, env)

    def test_cached_bazel_tamper_fails_even_after_bazelisk_success(self):
        temporary, _, env, bazelisk, real_bazel = self.fixture()
        self.addCleanup(temporary.cleanup)
        self.prepared(env)
        child_path = f"{real_bazel.parent};C:/Windows/System32"
        with self.assertRaisesRegex(ValueError, "cached Bazel executable SHA-256"):
            subject.resolve_verified_cached_bazel(
                self.q029_command(env, bazelisk),
                env,
                run=lambda *_args, **_kwargs: self.completed(
                    stdout=f"PATH={child_path}\n"
                ),
                digest_file=lambda _path: "0" * 64,
            )

    def test_cached_bazel_outside_cas_layout_fails_closed(self):
        temporary, root, env, bazelisk, _ = self.fixture()
        self.addCleanup(temporary.cleanup)
        self.prepared(env)
        real_bazel = root / "attacker" / "bazel.exe"
        real_bazel.parent.mkdir()
        real_bazel.write_bytes(b"same reviewed bytes")
        child_path = f"{real_bazel.parent};C:/Windows/System32"
        with self.assertRaisesRegex(ValueError, "content-addressed layout"):
            subject.resolve_verified_cached_bazel(
                self.q029_command(env, bazelisk),
                env,
                run=lambda *_args, **_kwargs: self.completed(
                    stdout=f"PATH={child_path}\n"
                ),
                digest_file=lambda _path: q029.BAZEL_WINDOWS_X86_64_SHA256,
            )

    def test_duplicate_print_env_path_fails_closed(self):
        temporary, _, env, bazelisk, real_bazel = self.fixture()
        self.addCleanup(temporary.cleanup)
        self.prepared(env)
        child_path = f"{real_bazel.parent};C:/Windows/System32"
        with self.assertRaisesRegex(ValueError, "exactly one PATH"):
            subject.resolve_verified_cached_bazel(
                self.q029_command(env, bazelisk),
                env,
                run=lambda *_args, **_kwargs: self.completed(
                    stdout=f"PATH={child_path}\nPath={child_path}\n"
                ),
            )

    def test_print_env_transport_token_fails_closed(self):
        temporary, _, env, bazelisk, real_bazel = self.fixture()
        self.addCleanup(temporary.cleanup)
        self.prepared(env)
        child_path = f"{real_bazel.parent};C:/Windows/System32"
        with self.assertRaisesRegex(ValueError, "retained the setup-only"):
            subject.resolve_verified_cached_bazel(
                self.q029_command(env, bazelisk),
                env,
                run=lambda *_args, **_kwargs: self.completed(
                    stdout=(
                        f"PATH={child_path}\n"
                        "BAZELISK_GITHUB_TOKEN=should-not-exist\n"
                    )
                ),
            )

    def test_nonzero_print_env_does_not_echo_transport_output(self):
        temporary, _, env, bazelisk, _ = self.fixture()
        self.addCleanup(temporary.cleanup)
        self.prepared(env)
        with self.assertRaisesRegex(ValueError, "exit=17") as error:
            subject.resolve_verified_cached_bazel(
                self.q029_command(env, bazelisk),
                env,
                run=lambda *_args, **_kwargs: self.completed(
                    stdout="BAZELISK_GITHUB_TOKEN=secret\n",
                    returncode=17,
                ),
            )
        self.assertNotIn("secret", str(error.exception))
        self.assertNotIn("hidden transport diagnostics", str(error.exception))

    def final_command(self):
        temporary, _, env, bazelisk, real_bazel = self.fixture()
        self.addCleanup(temporary.cleanup)
        self.prepared(env)
        command = subject.bind_output_base_startup(
            self.q029_command(env, bazelisk),
            env,
        )
        child_path = f"{real_bazel.parent};C:/Windows/System32"
        command = subject.resolve_verified_cached_bazel(
            command,
            env,
            run=lambda *_args, **_kwargs: self.completed(
                stdout=f"PATH={child_path}\n"
            ),
            digest_file=lambda _path: q029.BAZEL_WINDOWS_X86_64_SHA256,
        )
        return env, real_bazel, command

    def test_final_cached_bazel_context_passes(self):
        env, _, command = self.final_command()
        subject.validate_keyless_windows_gnullvm_cached_bazel_context(
            command,
            env,
            digest_file=lambda _path: q029.BAZEL_WINDOWS_X86_64_SHA256,
        )

    def test_final_output_base_drift_fails_closed(self):
        env, _, command = self.final_command()
        env["BAZEL_OUTPUT_BASE"] = "C:/attacker"
        index = next(
            i for i, value in enumerate(command) if value.startswith("--output_base=")
        )
        command[index] = "--output_base=C:/attacker"
        with self.assertRaisesRegex(ValueError, "runner-controlled root"):
            subject.validate_keyless_windows_gnullvm_cached_bazel_context(
                command,
                env,
                digest_file=lambda _path: q029.BAZEL_WINDOWS_X86_64_SHA256,
            )

    def test_final_cached_bazel_is_rehashed_immediately_before_launch(self):
        env, _, command = self.final_command()
        with self.assertRaisesRegex(ValueError, "drifted before launch"):
            subject.validate_keyless_windows_gnullvm_cached_bazel_context(
                command,
                env,
                digest_file=lambda _path: "f" * 64,
            )

    def test_wrapper_orders_q030_after_q029_and_before_return(self):
        env = {"GITHUB_ACTIONS": "true"}
        raw = ["candidate-bazel", "build", "--", "//codex-rs/cli:codex"]
        q029_bound = ["verified-bazelisk", *raw[1:]]
        output_bound = ["verified-bazelisk", "--output_base=D:/o", *raw[1:]]
        final = ["verified-cached-bazel", *output_bound[1:]]
        events = []

        def record(name, result=None):
            def effect(*_args, **_kwargs):
                events.append(name)
                return result

            return effect

        with (
            patch.object(launcher, "bazel_command", return_value=raw),
            patch.object(
                launcher,
                "_is_keyless_windows_gnullvm",
                return_value=True,
            ),
            patch.object(
                launcher,
                "clear_setup_bazel_transport_token",
                side_effect=record("clear"),
            ),
            patch.object(
                launcher,
                "prepare_bazelisk_environment",
                side_effect=record("prepare"),
            ),
            patch.object(
                launcher,
                "bind_verified_bazelisk",
                side_effect=record("bind-q029", q029_bound),
            ),
            patch.object(
                launcher,
                "validate_keyless_windows_gnullvm_execution_context",
                side_effect=record("validate-q029"),
            ),
            patch.object(
                launcher,
                "bind_output_base_startup",
                side_effect=record("bind-output-base", output_bound),
            ),
            patch.object(
                launcher,
                "resolve_verified_cached_bazel",
                side_effect=record("resolve-cached-bazel", final),
            ),
            patch.object(
                launcher,
                "validate_keyless_windows_gnullvm_cached_bazel_context",
                side_effect=record("validate-q030"),
            ),
        ):
            result = launcher.executable_command("build", env=env)

        self.assertEqual(result, final)
        self.assertEqual(
            events,
            [
                "clear",
                "prepare",
                "bind-q029",
                "validate-q029",
                "bind-output-base",
                "resolve-cached-bazel",
                "validate-q030",
            ],
        )


if __name__ == "__main__":
    unittest.main()
