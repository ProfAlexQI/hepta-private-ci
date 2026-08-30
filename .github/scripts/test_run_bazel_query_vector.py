#!/usr/bin/env python3

from __future__ import annotations

import importlib.util
import sys
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
VERIFIER = ROOT / "scripts" / "verify-windows-gnullvm-bazel-query-vector.py"
SPEC = importlib.util.spec_from_file_location("bazel_query_vector", VERIFIER)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError(f"cannot load {VERIFIER}")
subject = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = subject
SPEC.loader.exec_module(subject)


class BazelQueryVectorTest(unittest.TestCase):
    @staticmethod
    def manifest(options: tuple[str, ...]) -> str:
        values = ",\n".join(f"    {value!r}" for value in options)
        return (
            "QUERY_OPTIONS = (\n"
            f"{values},\n"
            ")\n"
            "query_command = [command[0], *startup, \"query\", "
            "*QUERY_OPTIONS, \"--\", expression]\n"
            "result = run(query_command, cwd=workspace, env=dict(env), "
            "capture_output=True)\n"
            "raise ValueError(\"target-manifest Bazel query failed\")\n"
        )

    @staticmethod
    def workflow() -> str:
        return (
            "uses: ./.github/actions/setup-bazel-ci\n"
            "run: python3 scripts/verify-windows-gnullvm-bazel-query-vector.py "
            "--execute\n"
            'receipt = {"bazel_query_parser_executed": True, '
            '"bazel_query_options": ["--noshow_progress", "--output=label"]}\n'
        )

    def test_exact_query_vector_passes_source_parser(self) -> None:
        subject.validate_source(
            self.manifest(subject.EXPECTED_QUERY_OPTIONS),
            self.workflow(),
        )

    def test_build_only_and_nonexistent_query_options_fail_closed(self) -> None:
        for option in subject.FORBIDDEN_QUERY_OPTIONS:
            with self.subTest(option=option):
                with self.assertRaisesRegex(SystemExit, "query options drifted"):
                    subject.validate_source(
                        self.manifest((*subject.EXPECTED_QUERY_OPTIONS, option)),
                        self.workflow(),
                    )

    def test_real_smoke_command_uses_exact_parser_vector(self) -> None:
        workspace = Path("/tmp/hepta-query-probe")
        command = subject.parser_smoke_command(Path("/opt/bazel"), workspace)
        self.assertEqual(
            command,
            [
                "/opt/bazel",
                "--output_user_root=/tmp/hepta-query-probe/output-user-root",
                "--nomaster_bazelrc",
                "--nosystem_rc",
                "--noworkspace_rc",
                "--nohome_rc",
                "--bazelrc=/tmp/hepta-query-probe/empty.bazelrc",
                "query",
                "--noshow_progress",
                "--output=label",
                "--",
                "//:probe",
            ],
        )

    def test_transport_token_scrub_is_case_insensitive(self) -> None:
        env = {
            "PATH": "/opt/bin",
            "BAZELISK_GITHUB_TOKEN": "one",
            "BaZeLiSk_GiThUb_ToKeN": "two",
        }
        subject._scrub_transport_token(env)
        self.assertEqual(env, {"PATH": "/opt/bin"})

    def test_workflow_must_execute_smoke_after_setup(self) -> None:
        workflow = self.workflow().replace(
            "uses: ./.github/actions/setup-bazel-ci\n",
            "",
            1,
        )
        with self.assertRaisesRegex(SystemExit, "install the pinned"):
            subject.validate_source(
                self.manifest(subject.EXPECTED_QUERY_OPTIONS),
                workflow,
            )


if __name__ == "__main__":
    unittest.main()
