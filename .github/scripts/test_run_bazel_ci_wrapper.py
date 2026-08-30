#!/usr/bin/env python3

import importlib.util
import json
import os
import shutil
import subprocess
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory


SCRIPT_DIR = Path(__file__).resolve().parent
SCRIPT = SCRIPT_DIR / "run-bazel-ci.sh"
BUILD_BUDDY_WRAPPER = SCRIPT_DIR / "run_bazel_with_buildbuddy.py"

_spec = importlib.util.spec_from_file_location(
    "run_bazel_with_buildbuddy_q014", BUILD_BUDDY_WRAPPER
)
assert _spec is not None and _spec.loader is not None
run_bazel_with_buildbuddy = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(run_bazel_with_buildbuddy)


class RunBazelCiWrapperTest(unittest.TestCase):
    def run_wrapper(
        self,
        args: list[str],
        *,
        runner_os: str = "Windows",
        buildbuddy_key: str | None = None,
        allow_msvc_fallback: bool = False,
    ) -> tuple[subprocess.CompletedProcess[str], list[str]]:
        with TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            wrapper = root / "run-bazel-ci.sh"
            delegate = root / "run-bazel-ci-core.sh"
            capture = root / "capture.json"
            shutil.copy2(SCRIPT, wrapper)
            delegate.write_text(
                "#!/usr/bin/env python3\n"
                "import json, os, sys\n"
                "from pathlib import Path\n"
                "Path(os.environ['CAPTURE']).write_text(\n"
                "    json.dumps(sys.argv[1:]), encoding='utf-8'\n"
                ")\n",
                encoding="utf-8",
            )
            wrapper.chmod(0o755)
            delegate.chmod(0o755)

            env = os.environ.copy()
            env["CAPTURE"] = str(capture)
            env["RUNNER_OS"] = runner_os
            if buildbuddy_key is None:
                env.pop("BUILDBUDDY_API_KEY", None)
            else:
                env["BUILDBUDDY_API_KEY"] = buildbuddy_key
            if allow_msvc_fallback:
                env["ALLOW_WINDOWS_MSVC_FALLBACK"] = "1"
            else:
                env.pop("ALLOW_WINDOWS_MSVC_FALLBACK", None)

            bash = shutil.which("bash")
            if bash is None:
                self.skipTest("bash is required to exercise run-bazel-ci.sh")

            result = subprocess.run(
                [bash, str(wrapper), *args],
                env=env,
                check=False,
                capture_output=True,
                text=True,
            )
            captured = (
                json.loads(capture.read_text(encoding="utf-8"))
                if capture.exists()
                else []
            )
            return result, captured

    @staticmethod
    def base_cross_args(*bazel_options: str) -> list[str]:
        return [
            "--print-failed-action-summary",
            "--windows-cross-compile",
            "--",
            "build",
            *bazel_options,
            "--",
            "//codex-rs/utils/rustls-provider:rustls-provider-provider-test",
        ]

    def test_keyless_windows_cross_uses_exact_hybrid_contract(self) -> None:
        args = self.base_cross_args("--config=clippy")

        result, captured = self.run_wrapper(args)

        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertNotIn("--windows-cross-compile", captured)
        self.assertEqual(captured[0], "--print-failed-action-summary")
        expected = {
            "--config=ci-windows",
            "--host_platform=//:local_windows_msvc",
            "--platforms=//:windows_x86_64_gnullvm",
            "--extra_execution_platforms=//:windows_x86_64_msvc",
            "--extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain",
            "--strategy=TestRunner=local",
            "--strategy=V8Mksnapshot=local",
            "--build_metadata=TAG_windows_gnullvm_local=true",
            "--jobs=8",
            "--local_test_jobs=8",
            "--test_env=RUST_TEST_THREADS=1",
        }
        self.assertTrue(expected.issubset(captured), sorted(expected - set(captured)))
        self.assertLess(
            captured.index("--config=ci-windows"), captured.index("--config=clippy")
        )
        self.assertEqual(captured[-1], args[-1])

    def test_exact_semantic_overrides_are_accepted_without_duplication(self) -> None:
        exact_options = [
            "--host_platform=//:local_windows_msvc",
            "--platforms=//:windows_x86_64_gnullvm",
            "--extra_execution_platforms=//:windows_x86_64_msvc",
            "--extra_toolchains=//:windows_gnullvm_tests_on_msvc_host_toolchain",
            "--strategy=TestRunner=local",
            "--strategy=V8Mksnapshot=local",
        ]

        result, captured = self.run_wrapper(self.base_cross_args(*exact_options))

        self.assertEqual(result.returncode, 0, result.stderr)
        for option in exact_options:
            self.assertEqual(captured.count(option), 1, option)

    def test_incompatible_host_target_and_toolchain_overrides_fail_closed(self) -> None:
        incompatible_options = [
            "--host_platform=//:local_windows",
            "--platforms=//:windows_x86_64_msvc",
            "--extra_execution_platforms=//:custom_exec",
            "--extra_toolchains=//:custom_toolchain",
            "--strategy=TestRunner=remote",
            "--strategy=V8Mksnapshot=remote",
            "--test_env=CODEX_BAZEL_TEST_SKIP_FILTERS=everything",
        ]

        for option in incompatible_options:
            with self.subTest(option=option):
                result, captured = self.run_wrapper(self.base_cross_args(option))
                self.assertEqual(result.returncode, 1)
                self.assertEqual(captured, [])
                self.assertIn("refusing incompatible override", result.stderr)

    def test_split_form_semantic_option_fails_closed(self) -> None:
        result, captured = self.run_wrapper(
            self.base_cross_args(
                "--host_platform",
                "//:local_windows_msvc",
            )
        )

        self.assertEqual(result.returncode, 1)
        self.assertEqual(captured, [])
        self.assertIn("refusing split-form override", result.stderr)

    def test_nonsemantic_concurrency_overrides_remain_authoritative(self) -> None:
        result, captured = self.run_wrapper(
            self.base_cross_args(
                "--jobs=3",
                "--local_test_jobs=2",
                "--test_env=RUST_TEST_THREADS=2",
            )
        )

        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("--jobs=3", captured)
        self.assertIn("--local_test_jobs=2", captured)
        self.assertIn("--test_env=RUST_TEST_THREADS=2", captured)
        self.assertNotIn("--jobs=8", captured)
        self.assertNotIn("--local_test_jobs=8", captured)
        self.assertNotIn("--test_env=RUST_TEST_THREADS=1", captured)

    def test_authenticated_windows_cross_delegates_unchanged(self) -> None:
        args = self.base_cross_args()

        result, captured = self.run_wrapper(args, buildbuddy_key="token")

        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(captured, args)

    def test_explicit_msvc_diagnostic_delegates_unchanged(self) -> None:
        args = self.base_cross_args()

        result, captured = self.run_wrapper(args, allow_msvc_fallback=True)

        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(captured, args)

    def test_non_windows_cross_delegates_unchanged(self) -> None:
        args = self.base_cross_args()

        result, captured = self.run_wrapper(args, runner_os="Linux")

        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(captured, args)

    def test_gnullvm_marker_appends_target_unsets_after_inherited_values(self) -> None:
        sdk_vars = run_bazel_with_buildbuddy.WINDOWS_MSVC_SDK_ENV
        args = [
            "build",
            "--config=ci-windows",
            "--build_metadata=TAG_windows_gnullvm_local=true",
            *(f"--action_env={name}" for name in sdk_vars),
            *(f"--host_action_env={name}" for name in sdk_vars),
            "--",
            "//:fixture",
        ]

        configured = run_bazel_with_buildbuddy.bazel_args_with_remote_config(
            args,
            {"RUNNER_OS": "Windows"},
        )
        separator = configured.index("--")
        options = configured[:separator]
        for name in sdk_vars:
            inherited = f"--action_env={name}"
            target_unset = f"--action_env=={name}"
            host_inherited = f"--host_action_env={name}"
            self.assertIn(inherited, options)
            self.assertIn(host_inherited, options)
            self.assertIn(target_unset, options)
            self.assertGreater(options.index(target_unset), options.index(inherited))

    def test_target_unsets_are_not_added_outside_keyless_windows_hybrid(self) -> None:
        args = [
            "build",
            "--build_metadata=TAG_windows_gnullvm_local=true",
            "--action_env=INCLUDE",
            "--",
            "//:fixture",
        ]
        cases = [
            {"RUNNER_OS": "Linux"},
            {"RUNNER_OS": "Windows", "BUILDBUDDY_API_KEY": "token"},
            {"RUNNER_OS": "Windows"},
        ]
        for env in cases[:2]:
            with self.subTest(env=env):
                configured = run_bazel_with_buildbuddy.bazel_args_with_remote_config(
                    args, env
                )
                self.assertNotIn("--action_env==INCLUDE", configured)

        no_marker = [
            "build",
            "--action_env=INCLUDE",
            "--",
            "//:fixture",
        ]
        configured = run_bazel_with_buildbuddy.bazel_args_with_remote_config(
            no_marker, cases[2]
        )
        self.assertNotIn("--action_env==INCLUDE", configured)


if __name__ == "__main__":
    unittest.main()
