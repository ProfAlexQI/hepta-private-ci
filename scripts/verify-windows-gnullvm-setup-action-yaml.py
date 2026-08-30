#!/usr/bin/env python3
from __future__ import annotations

import hashlib
from pathlib import Path

from hepta_setup_action_yaml import Entry, YamlIndex, fail, parse_yaml_index, require

ROOT = Path(__file__).resolve().parents[1]
ACTION = ROOT / ".github/actions/setup-bazel-ci/action.yml"
EXPECTED_ACTION_BLOB = "890567be46f3fd78c11b89a20950bef2f7af4bf6"
PIN = "bazel-contrib/setup-bazel@c5acdfb288317d0b5c0bbd7a396a3dc868bb0f86"
NAMES = (
    None,
    "Set up Bazel",
    "Scrub setup-only Bazelisk GitHub token",
    "Configure Bazel repository cache",
    "Expose MSVC SDK environment (Windows)",
    "Compute cache-stable Windows Bazel PATH",
)


def blob(path: Path) -> str:
    data = path.read_bytes()
    return hashlib.sha1(f"blob {len(data)}\0".encode() + data, usedforsecurity=False).hexdigest()


def value(index: YamlIndex, scope: int, key: str) -> str:
    return index.entry(scope, key).value


def keys(index: YamlIndex, scope: int, expected: set[str], owner: str) -> None:
    observed = set(index.scopes[scope].entries)
    require(observed == expected, f"{owner} keys drifted: {sorted(observed)!r}")


def validate_setup_action_text(text: str) -> None:
    y = parse_yaml_index(text)
    keys(y, 0, {"name", "description", "inputs", "outputs", "runs"}, "root")
    require(value(y, 0, "name") == "setup-bazel-ci", "action name drifted")
    runs = y.child(0, "runs")
    keys(y, runs, {"using", "steps"}, "runs")
    require(value(y, runs, "using") == "composite", "action is not composite")
    steps = y.child(runs, "steps")
    items = y.scopes[steps].items
    require(len(items) == 6, f"setup-bazel-ci must contain six steps; got {len(items)}")
    observed = tuple(y.scopes[item].entries.get("name", Entry("", "", 0)).value or None for item in items)
    require(observed == NAMES, f"setup action step sequence drifted: {observed!r}")

    first, setup, scrub, cache, msvc, path_step = items
    keys(y, first, {"id", "uses"}, "setup-ci")
    require(value(y, first, "id") == "setup_ci", "setup-ci id drifted")
    require(value(y, first, "uses") == "./.github/actions/setup-ci", "setup-ci action drifted")

    keys(y, setup, {"name", "uses", "with"}, "setup-bazel")
    require(value(y, setup, "uses") == PIN, "setup-bazel pin drifted")
    with_scope = y.child(setup, "with")
    keys(y, with_scope, {"bazelisk-version", "output-base"}, "setup-bazel with")
    require(value(y, with_scope, "bazelisk-version") == "1.28.1", "Bazelisk version drifted")
    require(value(y, with_scope, "output-base") == "${{ steps.setup_ci.outputs.bazel-output-base }}", "output-base drifted")

    keys(y, scrub, {"name", "shell", "run"}, "scrub")
    require(value(y, scrub, "shell") == "bash" and value(y, scrub, "run") == "|", "scrub step drifted")
    keys(y, cache, {"name", "id", "shell", "run"}, "cache")
    require(value(y, cache, "id") == "configure_bazel_repository_cache" and value(y, cache, "shell") == "pwsh", "cache step drifted")
    for item, owner in ((msvc, "MSVC"), (path_step, "PATH")):
        keys(y, item, {"name", "if", "shell", "run"}, owner)
        require(value(y, item, "if") == "runner.os == 'Windows'" and value(y, item, "shell") == "pwsh", f"{owner} step drifted")


def prove_duplicate_forms_fail_closed(text: str) -> None:
    cases = (
        text + '\n"runs":\n  using: composite\n',
        text + "\nruns :\n  using: composite\n",
        text.replace("  steps:\n", '  steps:\n  "steps":\n', 1),
        text.replace("      uses: ./.github/actions/setup-ci\n", '      uses: ./.github/actions/setup-ci\n      "uses": attacker/action@v1\n', 1),
        text.replace("    - name: Scrub setup-only Bazelisk GitHub token", "    - uses: ./.github/actions/untrusted-before-scrub\n\n    - name: Scrub setup-only Bazelisk GitHub token", 1),
    )
    for candidate in cases:
        try:
            validate_setup_action_text(candidate)
        except SystemExit:
            continue
        fail("strict YAML parser accepted an adversarial structure")


def main() -> None:
    require(ACTION.is_file(), "setup-bazel-ci action is missing")
    text = ACTION.read_text(encoding="utf-8")
    require(blob(ACTION) == EXPECTED_ACTION_BLOB, "setup-bazel-ci action blob drifted")
    validate_setup_action_text(text)
    prove_duplicate_forms_fail_closed(text)
    print("PASS_WINDOWS_GNULLVM_Q0_39_DUPLICATE_SAFE_SETUP_ACTION_YAML_SOURCE")


if __name__ == "__main__":
    main()
