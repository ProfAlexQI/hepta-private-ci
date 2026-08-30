#!/usr/bin/env python3
from __future__ import annotations

import os
import re
import subprocess
import sys
from pathlib import Path
from types import SimpleNamespace
from typing import Mapping

import hepta_q046_git_context as q046_git

VERIFIER = Path(__file__).with_name(
    "verify-hepta-intelligence-integration-admission-api-v2.py"
)
ALLOWED_TOKEN_ENV_NAMES = frozenset({"GH_TOKEN", "GITHUB_TOKEN"})
TOKEN_VALUE_RE = re.compile(r"^[!-~]{1,4096}$")
PROXY_NAMES = frozenset(
    {
        "all_proxy",
        "ftp_proxy",
        "http_proxy",
        "https_proxy",
        "no_proxy",
    }
)
CANONICAL_GIT_NAMES = frozenset(
    {
        "GIT_CONFIG_GLOBAL",
        "GIT_CONFIG_NOSYSTEM",
        "GIT_DIR",
        "GIT_INDEX_FILE",
        "GIT_LITERAL_PATHSPECS",
        "GIT_OPTIONAL_LOCKS",
        "GIT_WORK_TREE",
    }
)
PARENT_ENVIRONMENT_NAMES = frozenset(
    {
        "GITHUB_ACTIONS",
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
CHILD_PYTHON_ENVIRONMENT = {
    "PYTHONDONTWRITEBYTECODE": "1",
    "PYTHONHASHSEED": "0",
    "PYTHONIOENCODING": "utf-8",
    "PYTHONNOUSERSITE": "1",
    "PYTHONSAFEPATH": "1",
    "PYTHONUTF8": "1",
}


class AdmissionRunnerError(RuntimeError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise AdmissionRunnerError(message)


def token_environment_name(args: list[str]) -> str:
    observed: list[str] = []
    index = 0
    while index < len(args):
        argument = args[index]
        if argument == "--token-env":
            require(index + 1 < len(args), "--token-env requires a value")
            observed.append(args[index + 1])
            index += 2
            continue
        if argument.startswith("--token-env="):
            observed.append(argument.split("=", 1)[1])
        index += 1
    require(len(observed) <= 1, "duplicate --token-env arguments")
    name = observed[0] if observed else "GITHUB_TOKEN"
    require(
        name in ALLOWED_TOKEN_ENV_NAMES,
        "token environment variable is outside the fixed admission allowlist",
    )
    return name


def exact_environment_value(source: Mapping[str, str], name: str) -> str:
    folded = name.casefold()
    matches = [
        (key, value)
        for key, value in source.items()
        if key.casefold() == folded
    ]
    require(
        len(matches) == 1,
        f"environment variable must occur exactly once: {name}",
    )
    key, value = matches[0]
    require(key == name, f"environment variable case drifted: {name}")
    return value


def validate_parent_environment(
    source: Mapping[str, str],
    *,
    token_name: str | None,
) -> str | None:
    expected = set(PARENT_ENVIRONMENT_NAMES)
    if token_name is not None:
        expected.add(token_name)
    observed = set(source)
    require(
        observed == expected,
        "admission parent environment is not exact: "
        f"unexpected={sorted(observed - expected)!r} "
        f"missing={sorted(expected - observed)!r}",
    )
    require(source["PATH"] == "/usr/bin:/bin", "admission PATH drifted")
    require(source["LANG"] == "C", "admission LANG drifted")
    require(source["LC_ALL"] == "C", "admission LC_ALL drifted")
    require(
        source["PYTHONHASHSEED"] == "0",
        "admission PYTHONHASHSEED drifted",
    )
    require(
        source["PYTHONNOUSERSITE"] == "1",
        "admission PYTHONNOUSERSITE drifted",
    )
    require(
        source["PYTHONDONTWRITEBYTECODE"] == "1",
        "admission PYTHONDONTWRITEBYTECODE drifted",
    )
    cache_prefix = Path(source["PYTHONPYCACHEPREFIX"])
    require(
        cache_prefix.is_absolute(),
        "admission PYTHONPYCACHEPREFIX must be absolute",
    )
    require(
        source["GITHUB_ACTIONS"] == "true",
        "admission must run in GitHub Actions",
    )
    require(source["RUNNER_OS"] == "Linux", "admission runner must be Linux")

    home = Path(source["HOME"])
    require(home.is_absolute(), "admission HOME must be absolute")
    require(not home.is_symlink(), "admission HOME must not be a symlink")
    try:
        resolved_home = home.resolve(strict=True)
    except OSError as error:
        raise AdmissionRunnerError(f"cannot resolve admission HOME: {error}") from error
    require(resolved_home.is_dir(), "admission HOME must be a directory")
    require(resolved_home == home, "admission HOME traverses a link or alias")

    if token_name is None:
        return None
    token = exact_environment_value(source, token_name)
    require(
        TOKEN_VALUE_RE.fullmatch(token) is not None,
        "admission token must be bounded printable ASCII",
    )
    return token


def path_fingerprint(path: Path) -> tuple[int, int, int, int, int, int]:
    observed = os.stat(path, follow_symlinks=False)
    return (
        observed.st_dev,
        observed.st_ino,
        observed.st_mode,
        observed.st_size,
        observed.st_mtime_ns,
        observed.st_ctime_ns,
    )


def canonical_regular_file(path: Path, owner: str) -> Path:
    try:
        resolved = path.resolve(strict=True)
    except OSError as error:
        raise AdmissionRunnerError(f"cannot resolve {owner}: {error}") from error
    require(resolved.is_file(), f"{owner} is not a regular file")
    return resolved


def context_fingerprints(
    context: q046_git.GitContext,
) -> dict[str, tuple[int, int, int, int, int, int]]:
    return {
        "executable": path_fingerprint(context.executable),
        "git_dir": path_fingerprint(context.git_dir),
        "index": path_fingerprint(context.index),
        "root": path_fingerprint(context.root),
        "python": path_fingerprint(
            canonical_regular_file(Path(sys.executable), "Python executable")
        ),
        "runner": path_fingerprint(
            canonical_regular_file(Path(__file__), "admission runner")
        ),
        "verifier": path_fingerprint(
            canonical_regular_file(VERIFIER, "bounded admission verifier")
        ),
    }


def run_bound_git(
    context: q046_git.GitContext,
    *args: str,
) -> subprocess.CompletedProcess[str]:
    completed = subprocess.run(
        [*context.prefix, *args],
        cwd=context.root,
        env=dict(context.env),
        check=False,
        capture_output=True,
        text=True,
        stdin=subprocess.DEVNULL,
        timeout=30,
    )
    require(
        completed.returncode == 0,
        "bound Git command failed "
        f"({completed.returncode}): {' '.join(args)}: "
        f"{completed.stderr.strip()}",
    )
    require(
        completed.stderr == "",
        f"bound Git command wrote stderr: {' '.join(args)}",
    )
    return completed


def one_line(value: str, owner: str) -> str:
    lines = value.splitlines()
    require(
        len(lines) == 1 and bool(lines[0]),
        f"{owner} did not return one nonempty line",
    )
    require("\x00" not in lines[0], f"{owner} returned a NUL byte")
    return lines[0]


def worktree_status(context: q046_git.GitContext) -> str:
    return run_bound_git(
        context,
        "status",
        "--porcelain=v1",
        "--untracked-files=all",
        "--ignored=matching",
    ).stdout


def path_is_within(path: Path, directory: Path) -> bool:
    try:
        path.relative_to(directory)
    except ValueError:
        return False
    return True


def require_external_parent_paths(
    context: q046_git.GitContext,
    source: Mapping[str, str],
) -> None:
    root = context.root.resolve(strict=True)
    home = Path(source["HOME"]).resolve(strict=True)
    cache_prefix = Path(source["PYTHONPYCACHEPREFIX"]).resolve(strict=False)
    require(
        not path_is_within(home, root),
        "admission HOME must be outside the candidate",
    )
    require(
        not path_is_within(cache_prefix, root),
        "admission Python cache must be outside the candidate",
    )


def child_environment(
    context: q046_git.GitContext,
    *,
    token_name: str,
    source: Mapping[str, str],
) -> dict[str, str]:
    token = validate_parent_environment(source, token_name=token_name)
    assert token is not None

    child = {
        "PATH": str(context.executable.parent),
        "LANG": "C",
        "LC_ALL": "C",
        "GIT_CONFIG_NOSYSTEM": "1",
        "GIT_CONFIG_GLOBAL": os.devnull,
        "GIT_OPTIONAL_LOCKS": "0",
        "GIT_DIR": str(context.git_dir),
        "GIT_WORK_TREE": str(context.root),
        "GIT_INDEX_FILE": str(context.index),
        "GIT_LITERAL_PATHSPECS": "1",
        token_name: token,
        **CHILD_PYTHON_ENVIRONMENT,
    }
    expected = {
        "PATH",
        "LANG",
        "LC_ALL",
        *CANONICAL_GIT_NAMES,
        *CHILD_PYTHON_ENVIRONMENT,
        token_name,
    }
    require(
        set(child) == expected,
        "bounded child environment key set drifted",
    )
    surviving = sorted(
        key
        for key in child
        if key.upper().startswith("GIT_")
        and key not in CANONICAL_GIT_NAMES
    )
    require(
        not surviving,
        f"noncanonical Git environment survived: {surviving}",
    )
    surviving_proxies = sorted(
        key for key in child if key.casefold() in PROXY_NAMES
    )
    require(
        not surviving_proxies,
        f"proxy environment survived: {surviving_proxies}",
    )
    return child


def verify_context_stability(
    context: q046_git.GitContext,
    *,
    expected_head: str,
    expected_fingerprints: dict[
        str, tuple[int, int, int, int, int, int]
    ],
) -> None:
    observed_head = one_line(
        run_bound_git(context, "rev-parse", "HEAD").stdout,
        "bound Git HEAD",
    )
    require(
        observed_head == expected_head,
        "checked-out HEAD changed during admission verification",
    )
    require(
        context_fingerprints(context) == expected_fingerprints,
        "Git, Python, runner, verifier, or repository context changed "
        "during verification",
    )


def trusted_context() -> q046_git.GitContext:
    try:
        return q046_git.trusted_git_context()
    except SystemExit as error:
        raise AdmissionRunnerError(
            f"trusted Git context rejected checkout: {error}"
        ) from error


def run_verifier(args: list[str]) -> int:
    token_name = token_environment_name(args)
    validate_parent_environment(os.environ, token_name=token_name)
    context = trusted_context()
    require_external_parent_paths(context, os.environ)
    q046_git.require_git_executable(VERIFIER)

    expected_head = one_line(
        run_bound_git(context, "rev-parse", "HEAD").stdout,
        "bound Git HEAD",
    )
    require(
        worktree_status(context) == "",
        "candidate worktree is dirty before admission verification",
    )
    fingerprints = context_fingerprints(context)
    child = child_environment(
        context,
        token_name=token_name,
        source=os.environ,
    )

    try:
        completed = subprocess.run(
            [sys.executable, str(VERIFIER), *args],
            cwd=context.root,
            env=child,
            check=False,
            stdin=subprocess.DEVNULL,
            close_fds=True,
            timeout=120,
        )
    except subprocess.TimeoutExpired as error:
        raise AdmissionRunnerError(
            "bounded admission verifier exceeded 120 seconds"
        ) from error
    verify_context_stability(
        context,
        expected_head=expected_head,
        expected_fingerprints=fingerprints,
    )
    require(
        worktree_status(context) == "",
        "candidate worktree is dirty after admission verification",
    )
    return completed.returncode


def check_clean() -> int:
    validate_parent_environment(os.environ, token_name=None)
    context = trusted_context()
    require_external_parent_paths(context, os.environ)
    require(worktree_status(context) == "", "candidate worktree is dirty")
    print("PASS_HEPTA_INTELLIGENCE_ADMISSION_BOUND_GIT_CLEAN")
    return 0


def fixture_parent_environment(*, token_name: str | None) -> dict[str, str]:
    home = Path(__file__).resolve().parent
    value = {
        "PATH": "/usr/bin:/bin",
        "HOME": str(home),
        "LANG": "C",
        "LC_ALL": "C",
        "PYTHONHASHSEED": "0",
        "PYTHONNOUSERSITE": "1",
        "PYTHONPYCACHEPREFIX": str(home / "python-cache"),
        "PYTHONDONTWRITEBYTECODE": "1",
        "GITHUB_ACTIONS": "true",
        "RUNNER_OS": "Linux",
    }
    if token_name is not None:
        value[token_name] = "fixture-token"
    return value


def self_test() -> int:
    fake = SimpleNamespace(
        executable=Path("/usr/bin/git"),
        root=Path("/repo"),
        git_dir=Path("/repo/.git"),
        index=Path("/repo/.git/index"),
        env={
            "PATH": "/attacker:/usr/bin",
            "HOME": "/home/runner",
            "GITHUB_ACTIONS": "true",
            "RUNNER_OS": "Linux",
            "GIT_OBJECT_DIRECTORY": "/attacker/objects",
            "HTTP_PROXY": "http://attacker.invalid",
            "LD_PRELOAD": "/attacker/lib.so",
            "PYTHONPATH": "/attacker/python",
        },
    )
    source = fixture_parent_environment(token_name="GH_TOKEN")
    environment = child_environment(
        fake,
        token_name="GH_TOKEN",
        source=source,
    )
    require(environment["PATH"] == "/usr/bin", "Git PATH was not pinned")
    require(environment["GIT_DIR"] == "/repo/.git", "Git directory was not bound")
    require(environment["GIT_WORK_TREE"] == "/repo", "Git worktree was not bound")
    require(
        environment["GIT_INDEX_FILE"] == "/repo/.git/index",
        "Git index was not bound",
    )
    for forbidden in (
        "GIT_OBJECT_DIRECTORY",
        "HTTP_PROXY",
        "LD_PRELOAD",
        "PYTHONPATH",
        "PYTHONHOME",
        "SSL_CERT_FILE",
        "BASH_ENV",
        "ENV",
        "HOME",
        "GITHUB_ACTIONS",
        "RUNNER_OS",
    ):
        require(
            forbidden not in environment,
            f"unsafe child environment survived: {forbidden}",
        )
    require(
        token_environment_name(["--token-env", "GH_TOKEN"]) == "GH_TOKEN",
        "token environment parsing drifted",
    )
    require(
        token_environment_name(["--token-env=GITHUB_TOKEN"]) == "GITHUB_TOKEN",
        "GITHUB_TOKEN parsing drifted",
    )
    for invalid in (
        ["--token-env"],
        ["--token-env", "GH_TOKEN", "--token-env=GITHUB_TOKEN"],
        ["--token-env", "lowercase"],
        ["--token-env", "GIT_DIR"],
        ["--token-env", "HTTP_PROXY"],
        ["--token-env", "PATH"],
        ["--token-env", "LD_PRELOAD"],
        ["--token-env", "PYTHONPATH"],
        ["--token-env", "SSL_CERT_FILE"],
    ):
        try:
            token_environment_name(invalid)
        except AdmissionRunnerError:
            continue
        raise AdmissionRunnerError(
            f"invalid token argument unexpectedly passed: {invalid!r}"
        )

    polluted = dict(source)
    polluted["LD_PRELOAD"] = "/attacker/lib.so"
    try:
        validate_parent_environment(polluted, token_name="GH_TOKEN")
    except AdmissionRunnerError:
        pass
    else:
        raise AdmissionRunnerError("polluted parent environment unexpectedly passed")

    calls: list[tuple[str, ...]] = []
    original_run_bound_git = globals()["run_bound_git"]

    def fake_run_bound_git(
        _context: object,
        *args: str,
    ) -> SimpleNamespace:
        calls.append(args)
        return SimpleNamespace(stdout="", stderr="", returncode=0)

    globals()["run_bound_git"] = fake_run_bound_git
    try:
        require(worktree_status(fake) == "", "worktree status fixture failed")
    finally:
        globals()["run_bound_git"] = original_run_bound_git
    require(
        calls
        == [
            (
                "status",
                "--porcelain=v1",
                "--untracked-files=all",
                "--ignored=matching",
            )
        ],
        f"worktree status arguments drifted: {calls!r}",
    )

    duplicate_case = dict(source)
    duplicate_case["gh_token"] = "other-token"
    try:
        exact_environment_value(duplicate_case, "GH_TOKEN")
    except AdmissionRunnerError:
        pass
    else:
        raise AdmissionRunnerError(
            "case-colliding token environment unexpectedly passed"
        )

    print("PASS_HEPTA_INTELLIGENCE_ADMISSION_BOUND_GIT_FIXTURES")
    return 0


def main() -> int:
    try:
        if sys.argv[1:] == ["--self-test"]:
            return self_test()
        if sys.argv[1:] == ["--check-clean"]:
            return check_clean()
        return run_verifier(sys.argv[1:])
    except (AdmissionRunnerError, subprocess.TimeoutExpired) as error:
        print(
            f"FAIL_HEPTA_INTELLIGENCE_ADMISSION_BOUND_GIT: {error}",
            file=sys.stderr,
        )
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
