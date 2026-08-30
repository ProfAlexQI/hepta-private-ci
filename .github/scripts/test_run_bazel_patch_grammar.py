#!/usr/bin/env python3

from __future__ import annotations

import re
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
PATCH_DIR = ROOT / "patches"
LLVM_MINGW_PATCH = PATCH_DIR / "llvm_windows_mingw_compat.patch"
HUNK_HEADER = re.compile(
    r"^@@ -(?P<old_start>\d+)(?:,(?P<old_count>\d+))? "
    r"\+(?P<new_start>\d+)(?:,(?P<new_count>\d+))? @@"
)


def validate_unified_diff_hunks(path: Path) -> int:
    lines = path.read_text(encoding="utf-8").splitlines()
    hunk_count = 0
    index = 0

    while index < len(lines):
        match = HUNK_HEADER.match(lines[index])
        if match is None:
            index += 1
            continue

        hunk_count += 1
        old_expected = int(match.group("old_count") or "1")
        new_expected = int(match.group("new_count") or "1")
        old_seen = 0
        new_seen = 0
        header = lines[index]
        index += 1

        while old_seen < old_expected or new_seen < new_expected:
            if index >= len(lines):
                raise AssertionError(
                    f"{path}: incomplete hunk {header!r}: expected "
                    f"old/new {old_expected}/{new_expected}, observed "
                    f"{old_seen}/{new_seen}"
                )

            line = lines[index]
            if line.startswith("@@ ") or line.startswith("diff --git "):
                raise AssertionError(
                    f"{path}: incomplete hunk {header!r}: expected "
                    f"old/new {old_expected}/{new_expected}, observed "
                    f"{old_seen}/{new_seen}"
                )
            if line.startswith("\\ No newline at end of file"):
                index += 1
                continue
            if not line or line[0] not in " +-":
                raise AssertionError(
                    f"{path}: invalid unified-diff hunk line "
                    f"{index + 1}: {line!r}"
                )

            if line[0] in " -":
                old_seen += 1
            if line[0] in " +":
                new_seen += 1
            if old_seen > old_expected or new_seen > new_expected:
                raise AssertionError(
                    f"{path}: oversized hunk {header!r}: expected "
                    f"old/new {old_expected}/{new_expected}, observed "
                    f"{old_seen}/{new_seen}"
                )
            index += 1

    if hunk_count == 0:
        text = "\n".join(lines)
        if "GIT binary patch" not in text and "Binary files " not in text:
            raise AssertionError(f"{path}: patch contains no unified-diff hunks")

    return hunk_count


class RepositoryPatchGrammarTest(unittest.TestCase):
    def test_all_repository_patch_hunk_counts_are_complete(self) -> None:
        patches = sorted(PATCH_DIR.glob("*.patch"))
        self.assertTrue(patches, "repository patch inventory is empty")
        for patch in patches:
            with self.subTest(patch=patch.name):
                self.assertGreater(validate_unified_diff_hunks(patch), 0)

    def test_llvm_mingw_patch_retains_trailing_hunk_context(self) -> None:
        content = LLVM_MINGW_PATCH.read_text(encoding="utf-8")
        self.assertIn(
            "\n                 toolchain = cc_toolchain_name,\n",
            content,
        )

    def test_incomplete_hunk_fixture_fails_closed(self) -> None:
        fixture = ROOT / ".github" / "scripts" / "_malformed_patch_fixture.patch"
        fixture.write_text(
            "diff --git a/a b/a\n"
            "--- a/a\n"
            "+++ b/a\n"
            "@@ -1,2 +1,2 @@\n"
            " one\n",
            encoding="utf-8",
        )
        self.addCleanup(fixture.unlink, missing_ok=True)
        with self.assertRaisesRegex(AssertionError, "incomplete hunk"):
            validate_unified_diff_hunks(fixture)


if __name__ == "__main__":
    unittest.main()
