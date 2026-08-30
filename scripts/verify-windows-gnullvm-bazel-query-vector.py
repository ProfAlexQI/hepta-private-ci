#!/usr/bin/env python3

from __future__ import annotations

import argparse
import ast
import hashlib
import os
import shutil
import subprocess
from collections.abc import Callable, Mapping, MutableMapping, Sequence
from pathlib import Path
from tempfile import TemporaryDirectory
from typing import Final


ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / ".github" / "scripts" / "run_bazel_q034_execution_manifest.py"
WORKFLOW = (
    ROOT / ".github" / "workflows" / "windows-gnullvm-qualification-boundary.yml"
)
BAZELVERSION = ROOT / ".bazelversion"

BAZEL_VERSION: Final = "9.0.0"
BAZELVERSION_BYTES: Final = b"9.0.0\n"
BAZEL_LINUX_X86_64_SHA256: Final = (
    "c44a93f25398c68f904fa1d19b61d321de6c0d2f09dca375d7bc0dc9b9428403"
)
EXPECTED_QUERY_OPTIONS: Final = ("--noshow_progress", "--output=label")
FORBIDDEN_QUERY_OPTIONS: Final = (
    "--config=ci-windows",
    "--nouse_action_cache",
    "--nouse_analysis_cache",
)
TRANSPORT_TOKEN: Final = "BAZELISK_GITHUB_TOKEN"
PASS_SOURCE: Final = "PASS_WINDOWS_GNULLVM_Q0_39_BAZEL_QUERY_VECTOR_SOURCE"
PASS_EXECUTED: Final = "PASS_WINDOWS_GNULLVM_Q0_39_BAZEL_QUERY_PARSER_EXECUTED"


def fail(message: str) -> None:
    raise SystemExit(message)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def _assignment_tuple(tree: ast.Module, name: str) -> tuple[str, ...]:
    assignments = [
        node
        for node in tree.body
        if isinstance(node, (ast.Assign, ast.AnnAssign))
        and (
            (
                isinstance(node, ast.Assign)
                and len(node.targets) == 1
                and isinstance(node.targets[0], ast.Name)
                and node.targets[0].id == name
            )
            or (
                isinstance(node, ast.AnnAssign)
                and isinstance(node.target, ast.Name)
                and node.target.id == name
            )
        )
    ]
    require(len(assignments) == 1, f"{name} must have exactly one assignment")
    node = assignments[0]
    value = node.value
    require(isinstance(value, (ast.Tuple, ast.List)), f"{name} must be a literal tuple")
    values: list[str] = []
    for element in value.elts:
        require(
            isinstance(element, ast.Constant) and isinstance(element.value, str),
            f"{name} must contain only literal strings",
        )
        values.append(element.value)
    return tuple(values)


def validate_source(
    manifest_text: str | None = None,
    workflow_text: str | None = None,
) -> None:
    manifest_text = (
        MANIFEST.read_text(encoding="utf-8")
        if manifest_text is None
        else manifest_text
    )
    workflow_text = (
        WORKFLOW.read_text(encoding="utf-8")
        if workflow_text is None
        else workflow_text
    )
    tree = ast.parse(manifest_text, filename=str(MANIFEST))
    observed = _assignment_tuple(tree, "QUERY_OPTIONS")
    require(
        observed == EXPECTED_QUERY_OPTIONS,
        f"manifest query options drifted: expected {EXPECTED_QUERY_OPTIONS!r}, "
        f"observed {observed!r}",
    )
    for forbidden in FORBIDDEN_QUERY_OPTIONS:
        require(
            forbidden not in manifest_text,
            f"manifest retains query-incompatible option {forbidden}",
        )
    for token in (
        '"query"',
        "*QUERY_OPTIONS",
        '"--"',
        "expression",
        "cwd=workspace",
        "env=dict(env)",
        "capture_output=True",
        "target-manifest Bazel query failed",
    ):
        require(token in manifest_text, f"manifest query path lacks token: {token}")

    command = (
        "python3 scripts/verify-windows-gnullvm-bazel-query-vector.py --execute"
    )
    require(
        workflow_text.count(command) == 1,
        "qualification workflow must execute the real Bazel query parser once",
    )
    require(
        "uses: ./.github/actions/setup-bazel-ci" in workflow_text,
        "qualification workflow must install the pinned Bazelisk action",
    )
    require(
        workflow_text.index("uses: ./.github/actions/setup-bazel-ci")
        < workflow_text.index(command),
        "pinned setup-bazel must precede the real parser smoke",
    )
    require(
        '"bazel_query_parser_executed": True' in workflow_text,
        "receipt must record real Bazel query parser execution",
    )
    require(
        '"bazel_query_options": ["--noshow_progress", "--output=label"]'
        in workflow_text,
        "receipt must bind the exact query vector",
    )
    require(
        BAZELVERSION.read_bytes() == BAZELVERSION_BYTES,
        ".bazelversion bytes drifted from Bazel 9.0.0",
    )


def _scrub_transport_token(env: MutableMapping[str, str]) -> None:
    for name in list(env):
        if name.casefold() == TRANSPORT_TOKEN.casefold():
            env.pop(name, None)


def parser_smoke_command(
    bazel: Path,
    workspace: Path,
) -> list[str]:
    return [
        str(bazel),
        f"--output_user_root={workspace / 'output-user-root'}",
        "--nomaster_bazelrc",
        "--nosystem_rc",
        "--noworkspace_rc",
        "--nohome_rc",
        f"--bazelrc={workspace / 'empty.bazelrc'}",
        "query",
        *EXPECTED_QUERY_OPTIONS,
        "--",
        "//:probe",
    ]


def execute_parser_smoke(
    *,
    base_env: Mapping[str, str] | None = None,
    which: Callable[..., str | None] = shutil.which,
    run: Callable[..., subprocess.CompletedProcess[str]] = subprocess.run,
) -> None:
    env = dict(os.environ if base_env is None else base_env)
    _scrub_transport_token(env)
    env.update(
        {
            "USE_BAZEL_VERSION": BAZEL_VERSION,
            "BAZELISK_VERIFY_SHA256": BAZEL_LINUX_X86_64_SHA256,
            "BAZELISK_SKIP_WRAPPER": "true",
        }
    )
    bazel_value = which("bazel", path=env.get("PATH"))
    require(bool(bazel_value), "pinned Bazelisk executable was not found on PATH")
    bazel = Path(str(bazel_value))
    require(bazel.is_absolute(), "Bazelisk path must be absolute")
    require(not bazel.is_symlink(), "Bazelisk path must not be a symlink")
    require(bazel.is_file(), "Bazelisk path must be a regular file")

    with TemporaryDirectory(prefix="hepta-q039-query-") as temporary:
        workspace = Path(temporary).resolve()
        (workspace / "MODULE.bazel").write_text(
            'module(name = "hepta_q039_query_probe", version = "1.0")\n',
            encoding="utf-8",
        )
        (workspace / "BUILD.bazel").write_text(
            'filegroup(name = "probe", srcs = [])\n',
            encoding="utf-8",
        )
        (workspace / "empty.bazelrc").write_text("", encoding="utf-8")
        command = parser_smoke_command(bazel, workspace)
        result = run(
            command,
            cwd=workspace,
            env=env,
            capture_output=True,
            text=True,
            check=False,
            timeout=300,
        )
        require(
            result.returncode == 0,
            f"pinned Bazel {BAZEL_VERSION} query parser rejected the canonical "
            f"vector: exit={result.returncode}",
        )
        observed = [line for line in result.stdout.splitlines() if line]
        require(
            observed == ["//:probe"],
            f"pinned Bazel query smoke returned a noncanonical label set: {observed!r}",
        )
        require(
            all(
                name.casefold() != TRANSPORT_TOKEN.casefold()
                for name in env
            ),
            "setup-only transport token reached the parser smoke environment",
        )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--execute",
        action="store_true",
        help="execute the pinned Bazel 9 parser smoke after source validation",
    )
    args = parser.parse_args()

    validate_source()
    if args.execute:
        execute_parser_smoke()
        print(PASS_EXECUTED)
    else:
        print(PASS_SOURCE)


if __name__ == "__main__":
    main()
