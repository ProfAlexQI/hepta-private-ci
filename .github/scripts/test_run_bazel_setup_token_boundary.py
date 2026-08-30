#!/usr/bin/env python3

from __future__ import annotations

import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
ACTION = ROOT / ".github" / "actions" / "setup-bazel-ci" / "action.yml"
PINNED_SETUP_BAZEL = (
    "bazel-contrib/setup-bazel@"
    "c5acdfb288317d0b5c0bbd7a396a3dc868bb0f86"
)
SCRUB_STEP = "- name: Scrub setup-only Bazelisk GitHub token"
EMPTY_EXPORT = "printf '%s\\n' 'BAZELISK_GITHUB_TOKEN=' >> \"$GITHUB_ENV\""


class SetupBazelTokenBoundaryTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.text = ACTION.read_text(encoding="utf-8")
        cls.lines = cls.text.splitlines()

    def test_setup_bazel_is_centralized_in_scrubbed_composite_action(self) -> None:
        owners: list[str] = []
        for path in sorted((ROOT / ".github").rglob("*.y*ml")):
            text = path.read_text(encoding="utf-8")
            if "bazel-contrib/setup-bazel@" in text:
                owners.append(path.relative_to(ROOT).as_posix())
        self.assertEqual(
            owners,
            [".github/actions/setup-bazel-ci/action.yml"],
        )
        self.assertIn(PINNED_SETUP_BAZEL, self.text)

    def test_scrub_step_is_immediately_after_setup_bazel(self) -> None:
        step_markers = [
            line.strip()
            for line in self.lines
            if line.startswith("    - id:") or line.startswith("    - name:")
        ]
        self.assertGreaterEqual(len(step_markers), 4)
        self.assertEqual(step_markers[0], "- id: setup_ci")
        self.assertEqual(step_markers[1], "- name: Set up Bazel")
        self.assertEqual(step_markers[2], SCRUB_STEP)
        self.assertEqual(
            step_markers[3],
            "- name: Configure Bazel repository cache",
        )

    def test_scrub_exports_only_an_empty_value_without_reading_secret(self) -> None:
        start = self.text.index(f"    {SCRUB_STEP}")
        end = self.text.index(
            "    - name: Configure Bazel repository cache",
            start,
        )
        block = self.text[start:end]
        self.assertEqual(block.count(EMPTY_EXPORT), 1)
        self.assertEqual(block.count("unset BAZELISK_GITHUB_TOKEN"), 1)
        self.assertNotIn("${BAZELISK_GITHUB_TOKEN", block)
        self.assertNotIn("$BAZELISK_GITHUB_TOKEN", block)
        self.assertNotIn("::debug::", block)
        self.assertNotIn("::notice::", block)
        self.assertNotIn("::warning::", block)

    def test_setup_download_retains_default_token_only_inside_upstream_action(self) -> None:
        start = self.text.index("    - name: Set up Bazel")
        end = self.text.index(f"    {SCRUB_STEP}", start)
        setup_block = self.text[start:end]
        self.assertIn(PINNED_SETUP_BAZEL, setup_block)
        self.assertNotIn("        token:", setup_block)
        self.assertNotIn("BAZELISK_GITHUB_TOKEN", setup_block)


if __name__ == "__main__":
    unittest.main()
