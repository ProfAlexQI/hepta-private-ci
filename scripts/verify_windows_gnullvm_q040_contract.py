"""Q0.40 linear composition of Q0.39 parser and receipt-truth contracts."""

from __future__ import annotations

import ast
import hashlib
import importlib.util
import stat
import subprocess
import sys
from pathlib import Path
from typing import Final

from hepta_q040_blob_contract import BLOBS, DIRECT, EXECUTABLE, JOB


ROOT = Path(__file__).resolve().parents[1]
BLOB_CONTRACT = ROOT / "scripts" / "hepta_q040_blob_contract.py"
BLOB_CONTRACT_SHA: Final = "57c896107f8eab161a8ccde407e661bc2d9f4ac8"


def fail(message: str) -> None:
    raise SystemExit(message)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def read(relative: str) -> str:
    path = ROOT / relative
    require(path.is_file(), f"missing Q0.40 path: {relative}")
    return path.read_text(encoding="utf-8")


def blob(path: Path) -> str:
    data = path.read_bytes()
    framed = f"blob {len(data)}\0".encode("ascii") + data
    return hashlib.sha1(framed, usedforsecurity=False).hexdigest()


def require_executable(relative: str) -> None:
    path = ROOT / relative
    require(
        bool(path.stat().st_mode & stat.S_IXUSR),
        f"lost filesystem executable mode: {relative}",
    )
    result = subprocess.run(
        ["git", "ls-files", "--stage", "--", relative],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    entries = result.stdout.splitlines()
    require(
        result.returncode == 0
        and len(entries) == 1
        and entries[0].split(maxsplit=1)[0] == "100755",
        f"lost Git executable mode: {relative}",
    )


def require_tokens(text: str, tokens: tuple[str, ...], owner: str) -> None:
    for token in tokens:
        require(token in text, f"{owner} lacks Q0.40 token: {token}")


def assignment_tuple(text: str, name: str) -> tuple[str, ...]:
    tree = ast.parse(text)
    nodes = [
        node
        for node in tree.body
        if isinstance(node, ast.Assign)
        and len(node.targets) == 1
        and isinstance(node.targets[0], ast.Name)
        and node.targets[0].id == name
    ]
    require(
        len(nodes) == 1 and isinstance(nodes[0].value, ast.Tuple),
        f"{name} drifted",
    )
    values: list[str] = []
    for item in nodes[0].value.elts:
        require(
            isinstance(item, ast.Constant) and isinstance(item.value, str),
            f"{name} is not literal",
        )
        values.append(item.value)
    return tuple(values)


def load_source(relative: str, name: str):
    spec = importlib.util.spec_from_file_location(name, ROOT / relative)
    require(spec is not None and spec.loader is not None, f"cannot load {relative}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def run_legacy(owner: str) -> None:
    if owner == "direct-bazel":
        module = load_source(
            "scripts/verify_windows_gnullvm_q038_direct_bazel_base.py",
            "_hepta_q040_direct_base",
        )
        patches = {
            "EXPECTED_Q034_BLOB": BLOBS[
                ".github/scripts/run_bazel_q034_execution_manifest.py"
            ],
            "EXPECTED_EXECUTION_TEST_BLOB": BLOBS[
                ".github/scripts/test_run_bazel_execution_manifest.py"
            ],
            "EXPECTED_BOUNDARY_BLOB": BLOBS[
                ".github/scripts/test_run_bazel_qualification_boundary.sh"
            ],
            "EXPECTED_QUALIFICATION_WORKFLOW_BLOB": BLOBS[
                ".github/workflows/windows-gnullvm-qualification-boundary.yml"
            ],
        }
    else:
        module = load_source(
            "scripts/verify_windows_gnullvm_q038_job_executable_base.py",
            "_hepta_q040_job_base",
        )
        patches = {
            "EXPECTED_Q034_BLOB": BLOBS[
                ".github/scripts/run_bazel_q034_execution_manifest.py"
            ],
            "EXPECTED_EXECUTION_TEST_BLOB": BLOBS[
                ".github/scripts/test_run_bazel_execution_manifest.py"
            ],
            "EXPECTED_FIXTURE_BLOB": BLOBS[
                ".github/scripts/test_run_bazel_qualification_boundary.sh"
            ],
            "EXPECTED_BOUNDARY_BLOB": BLOBS[
                ".github/workflows/windows-gnullvm-qualification-boundary.yml"
            ],
        }
    for key, value in patches.items():
        require(hasattr(module, key), f"legacy {owner} verifier lacks {key}")
        setattr(module, key, value)
    module.main()


def validate_receipt_truth(workflow: str, verifier: str) -> None:
    truthful = (
        '"strict_step_parser_executed_before_setup_action": '
        'runner_os == "Linux",'
    )
    false_positive = (
        '"strict_step_parser_executed_before_setup_action": True,'
    )
    receipt_command = (
        "python3 scripts/"
        "verify-windows-gnullvm-setup-token-receipt-truth.py"
    )
    setup_action = "uses: ./.github/actions/setup-bazel-ci"

    require_tokens(
        workflow,
        (
            "if: runner.os == 'Linux'",
            receipt_command,
            truthful,
            '"cross_platform_verifier_executed_before_setup_action": True,',
            '"runner_os": runner_os,',
        ),
        "selected Q0.39 setup-token workflow",
    )
    require(
        false_positive not in workflow,
        "selected receipt workflow retained a cross-row false-positive claim",
    )
    require(
        workflow.index(receipt_command) < workflow.index(setup_action),
        "receipt-truth verifier must execute before setup-bazel",
    )
    require_tokens(
        verifier,
        (
            "TRUTHFUL_RECEIPT_FIELD",
            "FALSE_POSITIVE_RECEIPT_FIELD",
            "prove_false_positive_rejected",
            "prove_wrong_scope_rejected",
            "prove_truth_gate_order_rejected",
            "PASS_WINDOWS_GNULLVM_Q0_39_RECEIPT_STEP_TRUTH_SOURCE",
        ),
        "selected Q0.39 receipt-truth verifier",
    )


def validate_increment() -> None:
    require(
        blob(BLOB_CONTRACT) == BLOB_CONTRACT_SHA,
        "Q0.40 blob contract drifted",
    )
    for relative, expected in BLOBS.items():
        path = ROOT / relative
        require(path.is_file(), f"missing Q0.40 blob: {relative}")
        require(blob(path) == expected, f"Q0.40 blob drift: {relative}")
    for relative in EXECUTABLE:
        require_executable(relative)

    require(
        read("scripts/verify-windows-gnullvm-direct-bazel.py") == DIRECT,
        "direct verifier compatibility wrapper drifted",
    )
    require(
        read("scripts/verify-windows-gnullvm-job-executable.py") == JOB,
        "job verifier compatibility wrapper drifted",
    )

    manifest = read(".github/scripts/run_bazel_q034_execution_manifest.py")
    require(
        assignment_tuple(manifest, "QUERY_OPTIONS")
        == ("--noshow_progress", "--output=label"),
        "Bazel query vector drifted",
    )
    for token in (
        "--config=ci-windows",
        "--nouse_action_cache",
        "--nouse_analysis_cache",
    ):
        require(
            token not in manifest,
            f"query-incompatible option remains: {token}",
        )
    require(
        manifest.count("_validate_q032(command, env)") == 2,
        "manifest pre/post revalidation count drifted",
    )

    q030 = read(".github/scripts/run_bazel_q030_direct_bazel.py")
    require_tokens(
        q030,
        (
            "consume_setup_bazel_transport_token",
            "_require_transport_token_absent",
            "resolve_verified_bazel_command",
            "validate_keyless_windows_gnullvm_command",
            "verified direct Bazel executable changed before launch",
        ),
        "Q0.32 direct Bazel policy",
    )

    parser = read("scripts/hepta_setup_action_yaml.py")
    require_tokens(
        parser,
        (
            "def parse_yaml_index",
            "duplicate YAML key",
            "YAML merge keys are forbidden",
            "YAML anchors and aliases are forbidden",
        ),
        "setup-action YAML parser",
    )
    setup = read("scripts/verify-windows-gnullvm-setup-action-yaml.py")
    require_tokens(
        setup,
        (
            "validate_setup_action_text",
            "prove_duplicate_forms_fail_closed",
            "PASS_WINDOWS_GNULLVM_Q0_39_DUPLICATE_SAFE_SETUP_ACTION_YAML_SOURCE",
        ),
        "setup-action verifier",
    )
    setup_test = read(".github/scripts/test_run_bazel_setup_action_yaml.py")
    require_tokens(
        setup_test,
        (
            "test_quoted_duplicate_top_level_runs_fails_closed",
            "test_spaced_duplicate_top_level_runs_fails_closed",
            "test_quoted_duplicate_steps_fails_closed",
            "test_quoted_duplicate_uses_in_step_fails_closed",
            "test_merge_key_and_alias_fail_closed",
        ),
        "setup-action regression",
    )

    query = read("scripts/verify-windows-gnullvm-bazel-query-vector.py")
    require_tokens(
        query,
        (
            'BAZEL_VERSION: Final = "9.0.0"',
            "c44a93f25398c68f904fa1d19b61d321de6c0d2f09dca375d7bc0dc9b9428403",
            "def execute_parser_smoke",
            "BAZELISK_VERIFY_SHA256",
            '"//:probe"',
        ),
        "Bazel query verifier",
    )
    query_test = read(".github/scripts/test_run_bazel_query_vector.py")
    require_tokens(
        query_test,
        (
            "test_exact_query_vector_passes_source_parser",
            "test_build_only_and_nonexistent_query_options_fail_closed",
            "test_real_smoke_command_uses_exact_parser_vector",
            "test_transport_token_scrub_is_case_insensitive",
            "test_workflow_must_execute_smoke_after_setup",
        ),
        "Bazel query regression",
    )

    fixture = read(".github/scripts/test_run_bazel_qualification_boundary.sh")
    require_tokens(
        fixture,
        (
            "test_run_bazel_setup_action_yaml.py",
            "test_run_bazel_query_vector.py",
            "verify-windows-gnullvm-setup-action-yaml.py",
            "verify-windows-gnullvm-bazel-query-vector.py",
        ),
        "qualification fixture",
    )
    workflow = read(".github/workflows/windows-gnullvm-qualification-boundary.yml")
    smoke = "verify-windows-gnullvm-bazel-query-vector.py --execute"
    require_tokens(
        workflow,
        (
            "uses: ./.github/actions/setup-bazel-ci",
            smoke,
            '"schema": "hepta_windows_gnullvm_qualification_boundary_v3"',
            '"bazel_query_parser_executed": True',
            '"setup_action_yaml_duplicate_key_parser_executed": True',
        ),
        "Windows gnullvm qualification workflow",
    )
    require(
        workflow.index("uses: ./.github/actions/setup-bazel-ci")
        < workflow.index(smoke),
        "Bazel parser smoke precedes setup-bazel",
    )

    setup_workflow = read(
        ".github/workflows/windows-setup-bazel-token-boundary.yml"
    )
    receipt_verifier = read(
        "scripts/verify-windows-gnullvm-setup-token-receipt-truth.py"
    )
    validate_receipt_truth(setup_workflow, receipt_verifier)

    require_tokens(
        setup_workflow,
        (
            "Enable and verify Windows long paths before checkout",
            "git config --system --type=bool --get core.longpaths",
            "HEPTA_WINDOWS_LONG_PATHS_VERIFIED=true",
            '"schema": "hepta_windows_setup_bazel_token_boundary_v4"',
        ),
        "selected setup-token workflow",
    )
    strict = read("scripts/verify-windows-gnullvm-setup-token-boundary.py")
    require_tokens(
        strict,
        (
            "validate_setup_action",
            "prove_anonymous_step_is_rejected",
            "PASS_WINDOWS_GNULLVM_Q0_34_STRICT_SETUP_TOKEN_BOUNDARY_SOURCE",
        ),
        "strict setup-token verifier",
    )
    cross = read(
        "scripts/verify-windows-gnullvm-setup-token-cross-platform.py"
    )
    require_tokens(
        cross,
        (
            '"SYSTEMROOT"',
            "controlled_subprocess_env",
            "prove_long_paths_required",
            "prove_long_paths_must_precede_checkout",
            "prove_inherited_secrets_rejected",
        ),
        "cross-platform setup-token verifier",
    )

    blocking = read(".github/workflows/blocking-ci.yml")
    job = blocking.split(
        "  windows-setup-bazel-token-boundary:\n",
        1,
    )[1].split("\n  ", 1)[0]
    require(
        "secrets: inherit" not in job,
        "setup-token reusable job inherited secrets",
    )
    require(
        (ROOT / ".bazelversion").read_bytes() == b"9.0.0\n",
        ".bazelversion bytes drifted",
    )

    receipt_module = load_source(
        "scripts/verify-windows-gnullvm-setup-token-receipt-truth.py",
        "_hepta_q040_receipt_truth",
    )
    receipt_module.main()


def main(owner: str = "q040") -> None:
    require(
        owner in {"q040", "direct-bazel", "job-executable"},
        f"unknown Q0.40 owner {owner!r}",
    )
    if owner == "q040":
        run_legacy("job-executable")
        run_legacy("direct-bazel")
    else:
        run_legacy(owner)
    validate_increment()
    suffix = owner.upper().replace("-", "_")
    print(f"PASS_WINDOWS_GNULLVM_Q0_40_{suffix}_SOURCE")


if __name__ == "__main__":
    main()
