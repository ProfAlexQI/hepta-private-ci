#!/usr/bin/env python3

import json
import os
import subprocess
import tempfile
import unittest
from pathlib import Path


SCRIPT = Path(__file__).with_name("run-bazel-ci.sh")


class LocalWindowsGnullvmTest(unittest.TestCase):
    def run_fixture(
        self,
        *args: str,
        extra_env: dict[str, str] | None = None,
        expect_success: bool = True,
    ) -> tuple[subprocess.CompletedProcess[str], list[str] | None]:
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            scripts = root / ".github" / "scripts"
            scripts.mkdir(parents=True)
            wrapper = scripts / "run-bazel-ci.sh"
            wrapper.write_bytes(SCRIPT.read_bytes())
            wrapper.chmod(0o755)
            capture = root / "args.json"
            impl = scripts / "run-bazel-ci-impl.sh"
            impl.write_text(
                "#!/usr/bin/env python3\n"
                "import json, os, pathlib, sys\n"
                "pathlib.Path(os.environ['ARG_CAPTURE']).write_text("
                "json.dumps(sys.argv[1:]) + '\\n', encoding='utf-8')\n",
                encoding="utf-8",
            )
            impl.chmod(0o755)
            env = {
                **os.environ,
                "RUNNER_OS": "Windows",
                "ARG_CAPTURE": str(capture),
            }
            env.pop("BUILDBUDDY_API_KEY", None)
            env.pop("ALLOW_WINDOWS_MSVC_FALLBACK", None)
            env.pop("GITHUB_ACTIONS", None)
            if extra_env:
                env.update(extra_env)
            result = subprocess.run(
                [str(wrapper), *args],
                check=False,
                capture_output=True,
                text=True,
                env=env,
            )
            if expect_success:
                self.assertEqual(result.returncode, 0, result.stderr)
                return result, json.loads(capture.read_text(encoding="utf-8"))
            self.assertNotEqual(result.returncode, 0)
            self.assertFalse(capture.exists())
            return result, None

    @staticmethod
    def base_args(*extra: str) -> tuple[str, ...]:
        return (
            "--print-failed-action-summary",
            "--windows-cross-compile",
            "--",
            "test",
            "--skip_incompatible_explicit_targets",
            *extra,
            "--",
            "//codex-rs/uds:uds-unit-tests-bin",
        )

    def test_keyless_cross_uses_real_local_gnullvm_target(self) -> None:
        _, args = self.run_fixture(
            *self.base_args(),
            extra_env={"GITHUB_ACTIONS": "true"},
        )
        assert args is not None
        self.assertIn("--windows-msvc-host-platform", args)
        self.assertNotIn("--windows-cross-compile", args)
        for expected in (
            "--config=ci-windows",
            "--host_platform=//:local_windows_msvc",
            "--platforms=//:windows_x86_64_gnullvm",
            "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=0",
            "--extra_execution_platforms=//:windows_x86_64_msvc",
            "--extra_toolchains=//bazel/toolchains/windows:local_msvc_cc_toolchain",
            "--extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain",
            "--strategy=TestRunner=local",
            "--strategy=V8Mksnapshot=local",
            "--local_test_jobs=8",
            "--test_env=RUST_TEST_THREADS=1",
            "--jobs=8",
        ):
            self.assertIn(expected, args)
        self.assertFalse(
            any("CODEX_BAZEL_TEST_SKIP_FILTERS" in arg for arg in args),
            args,
        )

    def test_conflicting_target_fails_before_bazel(self) -> None:
        result, _ = self.run_fixture(
            *self.base_args("--platforms=//:local_windows_msvc"),
            expect_success=False,
        )
        self.assertIn("requires --platforms=//:windows_x86_64_gnullvm", result.stderr)

    def test_later_conflicting_target_also_fails_before_bazel(self) -> None:
        result, _ = self.run_fixture(
            *self.base_args(
                "--platforms=//:windows_x86_64_gnullvm",
                "--platforms=//:local_windows_msvc",
            ),
            expect_success=False,
        )
        self.assertIn("refusing conflicting option", result.stderr)

    def test_conflicting_test_strategy_fails_before_bazel(self) -> None:
        result, _ = self.run_fixture(
            *self.base_args("--strategy=TestRunner=remote"),
            extra_env={"GITHUB_ACTIONS": "true"},
            expect_success=False,
        )
        self.assertIn("requires --strategy=TestRunner=local", result.stderr)

    def test_explicit_msvc_diagnostic_gets_real_local_cc_toolchain(self) -> None:
        _, args = self.run_fixture(
            *self.base_args(),
            extra_env={"ALLOW_WINDOWS_MSVC_FALLBACK": "1"},
        )
        assert args is not None
        for expected in (
            "--host_platform=//:local_windows_msvc",
            "--platforms=//:local_windows_msvc",
            "--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=0",
            "--extra_toolchains=//bazel/toolchains/windows:local_msvc_cc_toolchain",
            "--config=ci-windows",
            "--jobs=8",
        ):
            self.assertIn(expected, args)
        self.assertNotIn(
            "--extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain",
            args,
        )

    def test_github_actions_rejects_ambient_msvc_diagnostic(self) -> None:
        result, _ = self.run_fixture(
            *self.base_args(),
            extra_env={
                "ALLOW_WINDOWS_MSVC_FALLBACK": "1",
                "GITHUB_ACTIONS": "true",
            },
            expect_success=False,
        )
        self.assertIn("forbidden in GitHub Actions qualification jobs", result.stderr)

    def test_authenticated_cross_path_is_byte_for_byte_passthrough(self) -> None:
        original = list(self.base_args())
        _, args = self.run_fixture(
            *original,
            extra_env={
                "BUILDBUDDY_API_KEY": "fixture",
                "GITHUB_ACTIONS": "true",
            },
        )
        self.assertEqual(args, original)


if __name__ == "__main__":
    unittest.main()
