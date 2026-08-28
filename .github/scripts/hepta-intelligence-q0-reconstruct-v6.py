from pathlib import Path
import subprocess

PATCH = Path('.github/patches/hepta-intelligence-q0-compile-fix-v1.patch')
MANUAL = Path('.github/scripts/hepta-intelligence-q0-manual-clippy-repair.py')
subprocess.run(['git', 'apply', '--check', str(PATCH)], check=True)
subprocess.run(['git', 'apply', str(PATCH)], check=True)

path = Path('codex-rs/hepta-memory/src/fact_grounding/durable/tests.rs')
text = path.read_text(encoding='utf-8')
marker = '#[tokio::test]\nasync fn tampered_evidence_digest_is_rejected_on_reopen() {'
prefix, separator, test = text.partition(marker)
assert separator == marker
assert test.count('.execute(&store.pool)') == 3
write_end = '        .await\n        .expect("write");\n\n'
assert test.count(write_end) == 1
connection = (
    '    let mut tamper_connection = store\n'
    '        .pool\n'
    '        .acquire()\n'
    '        .await\n'
    '        .expect("acquire tamper connection");\n\n'
)
test = test.replace(write_end, write_end + connection, 1)
test = test.replace('.execute(&store.pool)', '.execute(&mut *tamper_connection)')
assert test.count('.execute(&mut *tamper_connection)') == 3
assert test.count('    drop(store);\n') == 1
test = test.replace(
    '    drop(store);\n',
    '    drop(tamper_connection);\n    drop(store);\n',
    1,
)
path.write_text(prefix + separator + test, encoding='utf-8')
subprocess.run(['python3', str(MANUAL)], check=True)
