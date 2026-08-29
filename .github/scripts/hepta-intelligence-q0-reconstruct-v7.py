from __future__ import annotations

from pathlib import Path
import subprocess

PATCH = Path('.github/patches/hepta-intelligence-q0-compile-fix-v1.patch')
MANUAL = Path('.github/scripts/hepta-intelligence-q0-manual-clippy-repair-v7.py')


def patch_applies(*extra: str) -> bool:
    result = subprocess.run(
        ['git', 'apply', *extra, '--check', str(PATCH)],
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    return result.returncode == 0


# The compile repair was historically reconstructed in the qualification
# worktree. Later source-closure commits legitimately absorbed that exact
# patch. Accept precisely either state: apply the forward patch when it is
# absent, or prove the complete reverse patch applies when it is already
# present. A partial or drifted patch fails closed.
if patch_applies():
    subprocess.run(['git', 'apply', str(PATCH)], check=True)
elif not patch_applies('--reverse'):
    raise RuntimeError(
        'Q0 compile repair is neither cleanly applicable nor fully present'
    )

path = Path('codex-rs/hepta-memory/src/fact_grounding/durable/tests.rs')
text = path.read_text(encoding='utf-8')
marker = '#[tokio::test]\nasync fn tampered_evidence_digest_is_rejected_on_reopen() {'
prefix, separator, test = text.partition(marker)
assert separator == marker

old_execute = '.execute(&store.pool)'
new_execute = '.execute(&mut *tamper_connection)'
write_end = '        .await\n        .expect("write");\n\n'
connection = (
    '    let mut tamper_connection = store\n'
    '        .pool\n'
    '        .acquire()\n'
    '        .await\n'
    '        .expect("acquire tamper connection");\n\n'
)
old_drop = '    drop(store);\n'
new_drop = '    drop(tamper_connection);\n    drop(store);\n'

# This repair is also allowed in exactly two complete states so a future
# source-closure commit cannot make the executable matrix fail by attempting
# the same edit twice.
if (
    test.count(old_execute) == 3
    and test.count(new_execute) == 0
    and connection not in test
    and new_drop not in test
):
    assert test.count(write_end) == 1
    assert test.count(old_drop) == 1
    test = test.replace(write_end, write_end + connection, 1)
    test = test.replace(old_execute, new_execute)
    test = test.replace(old_drop, new_drop, 1)
elif not (
    test.count(old_execute) == 0
    and test.count(new_execute) == 3
    and test.count(connection) == 1
    and test.count(new_drop) == 1
):
    raise RuntimeError('tamper-connection repair is partial or drifted')

path.write_text(prefix + separator + test, encoding='utf-8')
subprocess.run(['python3', str(MANUAL)], check=True)
