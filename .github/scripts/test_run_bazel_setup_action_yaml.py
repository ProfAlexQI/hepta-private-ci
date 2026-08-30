#!/usr/bin/env python3

from __future__ import annotations

import importlib.util
import sys
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))
VERIFIER = ROOT / "scripts" / "verify-windows-gnullvm-setup-action-yaml.py"
SPEC = importlib.util.spec_from_file_location("setup_token_boundary", VERIFIER)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError(f"cannot load {VERIFIER}")
subject = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = subject
SPEC.loader.exec_module(subject)

CANONICAL = """\
name: setup-bazel-ci
description: Prepare a Bazel CI runner with shared caches.
inputs:
  target:
    description: Target triple used for cache namespacing.
    required: true
outputs:
  repository-cache-path:
    description: Filesystem path used for the Bazel repository cache.
    value: ${{ steps.configure_bazel_repository_cache.outputs.repository-cache-path }}

runs:
  using: composite
  steps:
    - id: setup_ci
      uses: ./.github/actions/setup-ci

    - name: Set up Bazel
      uses: bazel-contrib/setup-bazel@c5acdfb288317d0b5c0bbd7a396a3dc868bb0f86 # 0.19.0
      with:
        bazelisk-version: 1.28.1
        output-base: ${{ steps.setup_ci.outputs.bazel-output-base }}

    - name: Scrub setup-only Bazelisk GitHub token
      shell: bash
      run: |
        set -euo pipefail
        echo 'literal parser fixtures:'
        echo 'runs:'
        echo '"uses": attacker/action@v1'
        printf '%s\\n' 'BAZELISK_GITHUB_TOKEN=' >> "$GITHUB_ENV"
        unset BAZELISK_GITHUB_TOKEN

    - name: Configure Bazel repository cache
      id: configure_bazel_repository_cache
      shell: pwsh
      run: |
        "repository-cache-path=x" | Out-File "$env:GITHUB_OUTPUT"

    - name: Expose MSVC SDK environment (Windows)
      if: runner.os == 'Windows'
      shell: pwsh
      run: |
        Write-Output "fixture"

    - name: Compute cache-stable Windows Bazel PATH
      if: runner.os == 'Windows'
      shell: pwsh
      run: ./.github/scripts/compute-bazel-windows-path.ps1
"""


class DuplicateSafeSetupActionYamlTest(unittest.TestCase):
    def validate(self, text: str) -> None:
        subject.validate_setup_action_text(text)

    def test_canonical_action_passes_and_block_scalar_tokens_are_ignored(self) -> None:
        self.validate(CANONICAL)

    def test_quoted_duplicate_top_level_runs_fails_closed(self) -> None:
        with self.assertRaisesRegex(SystemExit, "duplicate YAML key 'runs'"):
            self.validate(CANONICAL + '\n"runs":\n  using: composite\n')

    def test_spaced_duplicate_top_level_runs_fails_closed(self) -> None:
        with self.assertRaisesRegex(SystemExit, "duplicate YAML key 'runs'"):
            self.validate(CANONICAL + "\nruns :\n  using: composite\n")

    def test_quoted_duplicate_steps_fails_closed(self) -> None:
        candidate = CANONICAL.replace(
            "  steps:\n",
            '  steps:\n  "steps":\n',
            1,
        )
        with self.assertRaisesRegex(SystemExit, "duplicate YAML key 'steps'"):
            self.validate(candidate)

    def test_quoted_duplicate_uses_in_step_fails_closed(self) -> None:
        candidate = CANONICAL.replace(
            "      uses: ./.github/actions/setup-ci\n",
            '      uses: ./.github/actions/setup-ci\n'
            '      "uses": attacker/action@v1\n',
            1,
        )
        with self.assertRaisesRegex(SystemExit, "duplicate YAML key 'uses'"):
            self.validate(candidate)

    def test_anonymous_step_fails_closed(self) -> None:
        candidate = CANONICAL.replace(
            "    - name: Scrub setup-only Bazelisk GitHub token",
            "    - uses: attacker/action@v1\n\n"
            "    - name: Scrub setup-only Bazelisk GitHub token",
            1,
        )
        with self.assertRaisesRegex(SystemExit, "must contain six steps"):
            self.validate(candidate)

    def test_merge_key_and_alias_fail_closed(self) -> None:
        for injection in (
            "      <<: *attacker\n",
            "      uses: *attacker\n",
        ):
            with self.subTest(injection=injection):
                candidate = CANONICAL.replace(
                    "      uses: ./.github/actions/setup-ci\n",
                    "      uses: ./.github/actions/setup-ci\n" + injection,
                    1,
                )
                with self.assertRaises(SystemExit):
                    self.validate(candidate)


if __name__ == "__main__":
    unittest.main()
