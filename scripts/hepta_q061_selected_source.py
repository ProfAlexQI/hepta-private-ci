#!/usr/bin/env python3
"""Bind an A0-first admission commit to the selected live Q0 source ref."""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
import re
import subprocess
import sys
import urllib.parse
import urllib.request
from pathlib import Path
from types import ModuleType
from typing import Any, Mapping

import hepta_q046_git_context as q046_git

SHA1_RE = re.compile(r"^[0-9a-f]{40}$")
INTEGRATION_BRANCH_RE = re.compile(
    r"^integration/hepta-intelligence-a0-q0-([1-9][0-9]*)-compose-([0-9]{8})$"
)
SOURCE_BRANCH_RE = re.compile(
    r"^codex/hepta-intelligence-q0-([1-9][0-9]*)-[a-z0-9]+"
    r"(?:-[a-z0-9]+)*-([0-9]{8})$"
)
TOKEN_RE = re.compile(r"^[!-~]{1,4096}$")
API_V2 = Path(__file__).with_name(
    "verify-hepta-intelligence-integration-admission-api-v2.py"
)
EXPECTED_ENVIRONMENT_NAMES = frozenset(
    {
        "EXPECTED_HEAD",
        "EXPECTED_HEAD_BRANCH",
        "GH_TOKEN",
        "GITHUB_ACTIONS",
        "GITHUB_REPOSITORY",
        "HEPTA_EXPECTED_Q0_SOURCE_BRANCH",
        "HOME",
        "LANG",
        "LC_ALL",
        "PATH",
        "PYTHONHASHSEED",
        "PYTHONNOUSERSITE",
        "PYTHONPYCACHEPREFIX",
        "PYTHONDONTWRITEBYTECODE",
        "RUNNER_OS",
    }
)


class SelectedSourceError(RuntimeError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SelectedSourceError(message)


def require_sha(value: str, owner: str) -> str:
    require(
        SHA1_RE.fullmatch(value) is not None,
        f"{owner} is not a lowercase SHA-1",
    )
    return value


def load_api_v2() -> ModuleType:
    q046_git.require_git_executable(API_V2)
    spec = importlib.util.spec_from_file_location(
        "hepta_intelligence_integration_admission_api_v2",
        API_V2,
    )
    require(spec is not None and spec.loader is not None, "cannot load API v2")
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def parse_repository(repository: str) -> tuple[str, str]:
    owner, separator, name = repository.partition("/")
    require(
        separator == "/" and bool(owner) and bool(name) and "/" not in name,
        "invalid repository",
    )
    grammar = re.compile(r"^[A-Za-z0-9_.-]+$")
    require(grammar.fullmatch(owner) is not None, "invalid repository owner")
    require(grammar.fullmatch(name) is not None, "invalid repository name")
    return owner, name


def branch_identity(
    branch: str,
    pattern: re.Pattern[str],
    owner: str,
) -> tuple[int, str]:
    match = pattern.fullmatch(branch)
    require(match is not None, f"{owner} is outside the governed Q0 grammar")
    assert match is not None
    return int(match.group(1)), match.group(2)


def validate_branch_pair(integration_branch: str, source_branch: str) -> int:
    integration_q0, integration_date = branch_identity(
        integration_branch,
        INTEGRATION_BRANCH_RE,
        "integration branch",
    )
    source_q0, source_date = branch_identity(
        source_branch,
        SOURCE_BRANCH_RE,
        "source branch",
    )
    require(integration_q0 == source_q0, "integration/source Q0 ordinals differ")
    require(integration_date == source_date, "integration/source dates differ")
    return source_q0


def validate_ref_payload(
    value: dict[str, Any],
    *,
    repository: str,
    source_branch: str,
    expected_sha: str,
) -> None:
    owner, name = parse_repository(repository)
    require(
        value.get("ref") == f"refs/heads/{source_branch}",
        "GitHub selected source ref name drifted",
    )
    selected = value.get("object")
    require(isinstance(selected, dict), "GitHub selected source object is missing")
    require(selected.get("type") == "commit", "selected source ref is not a commit")
    require(
        selected.get("sha") == expected_sha,
        "selected source ref is not the integration second parent",
    )
    require(
        selected.get("url")
        == f"https://api.github.com/repos/{owner}/{name}/git/commits/{expected_sha}",
        "selected source ref commit URL drifted",
    )


def fetch_source_ref(
    *,
    repository: str,
    source_branch: str,
    expected_sha: str,
    token: str,
) -> None:
    api = load_api_v2()
    owner, name = parse_repository(repository)
    branch_path = urllib.parse.quote(source_branch, safe="/")
    url = f"https://api.github.com/repos/{owner}/{name}/git/ref/heads/{branch_path}"
    request = urllib.request.Request(
        url,
        headers={
            "Accept": "application/vnd.github+json",
            "Accept-Encoding": "identity",
            "Authorization": f"Bearer {token}",
            "User-Agent": "hepta-intelligence-selected-q0-source-v1",
            "X-GitHub-Api-Version": "2022-11-28",
        },
        method="GET",
    )
    try:
        with api.URL_OPENER.open(request, timeout=30) as response:
            declared = api.validate_response_envelope(response, url)
            encoded = api.read_bounded(
                response,
                api.MAX_GIT_COMMIT_RESPONSE_BYTES,
            )
    except api.V1.AdmissionError as error:
        raise SelectedSourceError(f"GitHub selected source ref rejected: {error}") from error
    except (OSError, TimeoutError) as error:
        raise SelectedSourceError(f"GitHub selected source ref failed: {error}") from error
    require(
        declared is None or declared == len(encoded),
        "GitHub selected source Content-Length differs from its body",
    )
    try:
        value = api.decode_strict_json(encoded)
    except api.V1.AdmissionError as error:
        raise SelectedSourceError(f"GitHub selected source JSON rejected: {error}") from error
    validate_ref_payload(
        value,
        repository=repository,
        source_branch=source_branch,
        expected_sha=expected_sha,
    )


def exact_environment(source: Mapping[str, str]) -> Mapping[str, str]:
    observed = set(source)
    require(
        observed == set(EXPECTED_ENVIRONMENT_NAMES),
        "selected-source environment is not exact: "
        f"unexpected={sorted(observed - EXPECTED_ENVIRONMENT_NAMES)!r} "
        f"missing={sorted(EXPECTED_ENVIRONMENT_NAMES - observed)!r}",
    )
    require(source["PATH"] == "/usr/bin:/bin", "selected-source PATH drifted")
    require(
        source["LANG"] == "C" and source["LC_ALL"] == "C",
        "selected-source locale drifted",
    )
    require(source["PYTHONHASHSEED"] == "0", "PYTHONHASHSEED drifted")
    require(source["PYTHONNOUSERSITE"] == "1", "PYTHONNOUSERSITE drifted")
    require(
        source["PYTHONDONTWRITEBYTECODE"] == "1",
        "PYTHONDONTWRITEBYTECODE drifted",
    )
    require(source["GITHUB_ACTIONS"] == "true", "GitHub Actions is required")
    require(source["RUNNER_OS"] == "Linux", "Linux runner is required")
    require(Path(source["HOME"]).is_absolute(), "HOME must be absolute")
    require(
        Path(source["PYTHONPYCACHEPREFIX"]).is_absolute(),
        "PYTHONPYCACHEPREFIX must be absolute",
    )
    require(
        TOKEN_RE.fullmatch(source["GH_TOKEN"]) is not None,
        "GH_TOKEN is not bounded printable ASCII",
    )
    return source


def git_environment(context: q046_git.GitContext) -> dict[str, str]:
    return {
        "PATH": str(context.executable.parent),
        "LANG": "C",
        "LC_ALL": "C",
        "GIT_CONFIG_NOSYSTEM": "1",
        "GIT_CONFIG_GLOBAL": os.devnull,
        "GIT_OPTIONAL_LOCKS": "0",
        "GIT_LITERAL_PATHSPECS": "1",
        "GIT_DIR": str(context.git_dir),
        "GIT_WORK_TREE": str(context.root),
        "GIT_INDEX_FILE": str(context.index),
        "GIT_NO_LAZY_FETCH": "1",
        "GIT_NO_REPLACE_OBJECTS": "1",
    }


def run_git(context: q046_git.GitContext, *args: str) -> str:
    completed = subprocess.run(
        [*context.prefix, *args],
        cwd=context.root,
        env=git_environment(context),
        check=False,
        capture_output=True,
        text=True,
        stdin=subprocess.DEVNULL,
        timeout=30,
    )
    require(
        completed.returncode == 0,
        f"bound Git command failed: {' '.join(args)}",
    )
    require(
        completed.stderr == "",
        f"bound Git command wrote stderr: {' '.join(args)}",
    )
    return completed.stdout


def one_line(value: str, owner: str) -> str:
    lines = value.splitlines()
    require(
        len(lines) == 1 and bool(lines[0]),
        f"{owner} did not return one nonempty line",
    )
    require("\x00" not in lines[0], f"{owner} contains NUL")
    return lines[0]


def verify(output: Path) -> dict[str, Any]:
    source = exact_environment(os.environ)
    expected_head = require_sha(source["EXPECTED_HEAD"], "expected integration head")
    integration_branch = source["EXPECTED_HEAD_BRANCH"]
    source_branch = source["HEPTA_EXPECTED_Q0_SOURCE_BRANCH"]
    ordinal = validate_branch_pair(integration_branch, source_branch)
    try:
        context = q046_git.trusted_git_context()
    except SystemExit as error:
        raise SelectedSourceError(f"trusted Git rejected checkout: {error}") from error
    actual_head = require_sha(
        one_line(run_git(context, "rev-parse", "HEAD"), "bound Git HEAD"),
        "actual integration head",
    )
    require(actual_head == expected_head, "checked-out integration head drifted")
    parents = one_line(
        run_git(context, "show", "-s", "--format=%P", actual_head),
        "integration parents",
    ).split()
    require(len(parents) == 2, "integration head must have exactly two parents")
    first_parent, second_parent = [require_sha(item, "integration parent") for item in parents]
    require(first_parent != second_parent, "integration parents must be distinct")

    fetch_source_ref(
        repository=source["GITHUB_REPOSITORY"],
        source_branch=source_branch,
        expected_sha=second_parent,
        token=source["GH_TOKEN"],
    )
    receipt = {
        "schema": "hepta_intelligence_selected_q0_source_binding_v1",
        "status": "PASS_HEPTA_INTELLIGENCE_SELECTED_Q0_SOURCE_BINDING",
        "repository": source["GITHUB_REPOSITORY"],
        "integration_head": actual_head,
        "integration_branch": integration_branch,
        "first_parent_a0": first_parent,
        "second_parent_q0": second_parent,
        "selected_q0_source_branch": source_branch,
        "q0_ordinal": ordinal,
        "binding_sha256": hashlib.sha256(
            "\n".join(
                (
                    source["GITHUB_REPOSITORY"],
                    integration_branch,
                    actual_head,
                    first_parent,
                    source_branch,
                    second_parent,
                    str(ordinal),
                    "",
                )
            ).encode()
        ).hexdigest(),
        "source_writeback": False,
        "a0_candidate_qualified": False,
        "independent_review": False,
        "selected": False,
        "full_repository_merge_green": False,
        "runtime_wired": False,
        "production_authority": False,
        "operator_acceptance": False,
        "promotion": False,
        "release_authority": False,
        "callers_ratchet": False,
    }
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(
        json.dumps(receipt, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    return receipt


def self_test() -> None:
    source_branch = (
        "codex/hepta-intelligence-q0-61-selected-source-binding-20260831"
    )
    integration_branch = (
        "integration/hepta-intelligence-a0-q0-61-compose-20260831"
    )
    require(
        validate_branch_pair(integration_branch, source_branch) == 61,
        "valid branch pair failed",
    )
    for invalid in (
        (
            "integration/hepta-intelligence-a0-q0-60-compose-20260831",
            source_branch,
        ),
        (
            integration_branch,
            "codex/hepta-intelligence-q0-61-selected-source-binding-20260830",
        ),
        (integration_branch, "feature/q0-61"),
    ):
        try:
            validate_branch_pair(*invalid)
        except SelectedSourceError:
            pass
        else:
            raise SelectedSourceError(f"invalid branch pair passed: {invalid!r}")
    repository = "ProfHepta/hepta-private-ci"
    sha = "a" * 40
    good = {
        "ref": f"refs/heads/{source_branch}",
        "object": {
            "type": "commit",
            "sha": sha,
            "url": (
                f"https://api.github.com/repos/{repository}/git/commits/{sha}"
            ),
        },
    }
    validate_ref_payload(
        good,
        repository=repository,
        source_branch=source_branch,
        expected_sha=sha,
    )
    mutations = (
        {**good, "ref": "refs/heads/other"},
        {**good, "object": {**good["object"], "type": "tag"}},
        {**good, "object": {**good["object"], "sha": "b" * 40}},
        {
            **good,
            "object": {**good["object"], "url": "https://example.invalid"},
        },
    )
    for mutation in mutations:
        try:
            validate_ref_payload(
                mutation,
                repository=repository,
                source_branch=source_branch,
                expected_sha=sha,
            )
        except SelectedSourceError:
            pass
        else:
            raise SelectedSourceError(f"invalid ref payload passed: {mutation!r}")
    print("PASS_HEPTA_INTELLIGENCE_Q0_61_SELECTED_SOURCE_FIXTURES")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--self-test", action="store_true")
    mode.add_argument("--check", action="store_true")
    parser.add_argument("--output")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        if args.self_test:
            require(args.output is None, "--output is invalid with --self-test")
            self_test()
            return 0
        require(args.output is not None, "--output is required with --check")
        receipt = verify(Path(args.output))
        print(receipt["status"])
        return 0
    except (SelectedSourceError, subprocess.TimeoutExpired) as error:
        print(
            f"FAIL_HEPTA_INTELLIGENCE_Q0_61_SELECTED_SOURCE: {error}",
            file=sys.stderr,
        )
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
