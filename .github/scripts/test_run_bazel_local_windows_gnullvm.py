#!/usr/bin/env python3

import json
import os
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path


SCRIPT = Path(__file__).with_name("run-bazel-ci.sh")
IMPLEMENTATION = Path(__file__).with_name("run-bazel-ci-impl.sh")
CANONICAL_SKIP = (
    "command_safety::powershell_parser::tests::,"
    "suite::code_mode::code_mode_can_call_hidden_dynamic_tools,"
    "tests::windows_tests::conpty_ctrl_c_interrupts_powershell_foreground_child"
)


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

    def run_real_impl_fixture(
        self,
        *args: str,
        github_actions: bool,
    ) -> tuple[subprocess.CompletedProcess[str], list[str]]:
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            scripts = root / ".github" / "scripts"
            scripts.mkdir(parents=True)
            wrapper = scripts / "run-bazel-ci.sh"
            impl = scripts / "run-bazel-ci-impl.sh"
            launcher = scripts / "run_bazel_with_buildbuddy.py"
            capture = root / "bazel-args.json"
            shutil.copy2(SCRIPT, wrapper)
            shutil.copy2(IMPLEMENTATION, impl)
            launcher.write_text(
                "#!/usr/bin/env python3\n"
                "import json, os, pathlib, sys\n"
                "pathlib.Path(os.environ['ARG_CAPTURE']).write_text("
                "json.dumps(sys.argv[1:]) + '\\n', encoding='utf-8')\n",
                encoding="utf-8",
            )
            wrapper.chmod(0o755)
            impl.chmod(0o755)
            launcher.chmod(0o755)

            windows_path = r"C:\Tools;C:\Windows\System32"
            env = {
                **os.environ,
                "RUNNER_OS": "Windows",
                "ARG_CAPTURE": str(capture),
                "CODEX_BAZEL_WINDOWS_PATH": windows_path,
                "INCLUDE": r"C:\VS\include",
                "LIB": r"C:\VS\lib",
                "LIBPATH": r"C:\VS\libpath",
                "WindowsSdkDir": r"C:\Windows Kits\10",
            }
            for name in (
                "BUILDBUDDY_API_KEY",
                "ALLOW_WINDOWS_MSVC_FALLBACK",
                "BAZEL_OUTPUT_USER_ROOT",
                "BAZEL_REPO_CONTENTS_CACHE",
                "BAZEL_REPOSITORY_CACHE",
                "CODEX_BAZEL_EXECUTION_LOG_COMPACT_DIR",
            ):
                env.pop(name, None)
            if github_actions:
                env["GITHUB_ACTIONS"] = "true"
            else:
                env.pop("GITHUB_ACTIONS", None)

            result = subprocess.run(
                [str(wrapper), *args],
                check=False,
                capture_output=True,
                text=True,
                env=env,
            )
            self.assertEqual(result.returncode, 0, result.stderr)
            return result, json.loads(capture.read_text(encoding="utf-8"))

    @staticmethod
    def cross_args(*extra: str) -> tuple[str, ...]:
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
        _, args = self.run_fixture(*self.cross_args())
        assert args is not None
        self.assertIn("--windows-local-gnullvm", args)
        self.assertNotIn("--windows-msvc-host-platform", args)
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

    def test_github_keyless_contract_injects_canonical_cross_skip_set(self) -> None:
        _, args = self.run_fixture(
            *self.cross_args(),
            extra_env={"GITHUB_ACTIONS": "true"},
        )
        assert args is not None
        self.assertIn(
            f"--test_env=CODEX_BAZEL_TEST_SKIP_FILTERS={CANONICAL_SKIP}",
            args,
        )

    def test_conflicting_target_fails_before_bazel(self) -> None:
        result, _ = self.run_fixture(
            *self.cross_args("--platforms=//:local_windows_msvc"),
            expect_success=False,
        )
        self.assertIn("requires --platforms=//:windows_x86_64_gnullvm", result.stderr)

    def test_explicit_msvc_diagnostic_gets_real_local_cc_toolchain(self) -> None:
        _, args = self.run_fixture(
            *self.cross_args(),
            extra_env={"ALLOW_WINDOWS_MSVC_FALLBACK": "1"},
        )
        assert args is not None
        self.assertIn("--windows-msvc-host-platform", args)
        self.assertNotIn("--windows-local-gnullvm", args)
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

    def test_authenticated_cross_path_is_byte_for_byte_passthrough(self) -> None:
        original = self.cross_args()
        _, args = self.run_fixture(
            *original,
            extra_env={"BUILDBUDDY_API_KEY": "fixture"},
        )
        self.assertEqual(args, list(original))

    def test_local_gnullvm_impl_keeps_msvc_sdk_host_only(self) -> None:
        _, args = self.run_real_impl_fixture(
            *self.cross_args(),
            github_actions=True,
        )
        for name in ("INCLUDE", "LIB", "LIBPATH", "WindowsSdkDir"):
            self.assertIn(f"--host_action_env={name}", args)
            self.assertNotIn(f"--action_env={name}", args)
        windows_path = r"C:\Tools;C:\Windows\System32"
        self.assertIn(f"--host_action_env=PATH={windows_path}", args)
        self.assertNotIn(f"--action_env=PATH={windows_path}", args)
        self.assertIn(f"--test_env=PATH={windows_path}", args)
        self.assertIn(
            f"--test_env=CODEX_BAZEL_TEST_SKIP_FILTERS={CANONICAL_SKIP}",
            args,
        )

    def test_ordinary_local_windows_impl_preserves_target_and_host_env(self) -> None:
        _, args = self.run_real_impl_fixture(
            "--",
            "build",
            "--",
            "//codex-rs/uds:uds-unit-tests-bin",
            github_actions=True,
        )
        for name in ("INCLUDE", "LIB", "LIBPATH", "WindowsSdkDir"):
            self.assertIn(f"--action_env={name}", args)
            self.assertIn(f"--host_action_env={name}", args)
        windows_path = r"C:\Tools;C:\Windows\System32"
        self.assertIn(f"--action_env=PATH={windows_path}", args)
        self.assertIn(f"--host_action_env=PATH={windows_path}", args)


if __name__ == "__main__":
    unittest.main()
