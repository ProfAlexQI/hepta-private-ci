#!/usr/bin/env python3
"""Normalize and execute the exact P0.7a closure generator.

This wrapper repairs two generator omissions without weakening its exact-anchor
fail-closed contract: two multiline release fixtures and all seven canonical
shell-source fixtures present in the pinned P0.7a source generation.
"""
from __future__ import annotations

from pathlib import Path

SCRIPT = Path(__file__).with_name("hepta-p0-7a-supervisor-gap-closure.py").resolve()


def replace_source_once(source: str, old: str, new: str, label: str) -> str:
    count = source.count(old)
    if count != 1:
        raise SystemExit(f"{label}: expected exactly one generator anchor, found {count}")
    return source.replace(old, new, 1)


def main() -> None:
    source = SCRIPT.read_text()
    source = replace_source_once(
        source,
        '    shell_count = text.count(\'Path::new("/bin/sh")\')\n'
        '    if shell_count != 2:\n'
        '        raise SystemExit(f"paired release fixture: expected two /bin/sh anchors, found {shell_count}")\n',
        '    shell_count = text.count(\'Path::new("/bin/sh")\')\n'
        '    if shell_count != 7:\n'
        '        raise SystemExit(\n'
        '            f"supervisor canonical shell fixtures: expected seven anchors, found {shell_count}"\n'
        '        )\n',
        "canonical shell fixture invariant",
    )

    reject_spawn_anchor = (
        '    text = replace_once(\n'
        '        text,\n'
        '        \'    control.reject_spawn_program("/fake/release-spawn-fails/hepta-agentd");\\n\',\n'
    )
    multiline_release_patch = '''    for identity, program in [
        ("release-spawn-fails", "/fake/release-spawn-fails/hepta-agentd"),
        ("release-health-fails", "/fake/release-health-fails/hepta-agentd"),
    ]:
        text = replace_once(
            text,
            '        release(\\n'
            f'            "{identity}",\\n'
            f'            "{program}",\\n'
            '        )?,',
            '        release(\\n'
            '            &fleet,\\n'
            f'            "{identity}",\\n'
            f'            "{program}",\\n'
            '        )?,',
            f"supervisor multiline release fixture {identity}",
        )
'''
    source = replace_source_once(
        source,
        reject_spawn_anchor,
        multiline_release_patch + reject_spawn_anchor,
        "multiline release fixture normalization",
    )

    scope = {"__file__": str(SCRIPT), "__name__": "__main__"}
    exec(compile(source, str(SCRIPT), "exec"), scope)


if __name__ == "__main__":
    main()
