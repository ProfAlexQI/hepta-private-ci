from __future__ import annotations

import json
from pathlib import Path
import re
import subprocess

PATCH = Path('.github/patches/hepta-intelligence-q0-compile-fix-v1.patch')
MANUAL = Path('.github/scripts/hepta-intelligence-q0-manual-clippy-repair-v7.py')


def patch_applies() -> bool:
    result = subprocess.run(
        ['git', 'apply', '--check', str(PATCH)],
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    return result.returncode == 0


def verify_absorbed_compile_repair() -> None:
    """Prove every compile-fix semantic postcondition in the current source.

    A later source-closure commit may preserve the repair while changing module
    paths, visibility spellings, formatting, or surrounding context. Requiring
    the historical patch to reverse-apply would reject such valid supersets.
    This verifier accepts only the complete repaired state and rejects partial
    application or reintroduction of any old marker.
    """

    failures: list[dict[str, str]] = []

    def text(path: str) -> str:
        return Path(path).read_text(encoding='utf-8')

    def require(path: str, marker: str, label: str) -> None:
        if marker not in text(path):
            failures.append({'path': path, 'check': label, 'missing': marker})

    def forbid(path: str, marker: str, label: str) -> None:
        if marker in text(path):
            failures.append({'path': path, 'check': label, 'forbidden': marker})

    def require_regex(path: str, pattern: str, label: str) -> re.Match[str] | None:
        match = re.search(pattern, text(path), flags=re.MULTILINE)
        if match is None:
            failures.append({'path': path, 'check': label, 'missing_regex': pattern})
        return match

    ledger = 'codex-rs/hepta-memory/src/fact_grounding/durable/grounding/ledger.rs'
    require_regex(
        ledger,
        r'^pub\(in [^)]+\) use insert::insert_tx;$',
        'insert re-export reaches durable grounding parent',
    )
    require_regex(
        ledger,
        r'^pub\(in [^)]+\) use verify::verify_receipts;$',
        'verify re-export reaches durable grounding parent',
    )
    forbid(ledger, 'pub(super) use insert::insert_tx;', 'old insert visibility removed')
    forbid(ledger, 'pub(super) use verify::verify_receipts;', 'old verify visibility removed')

    insert = 'codex-rs/hepta-memory/src/fact_grounding/durable/grounding/ledger/insert.rs'
    require(insert, 'use super::support::durable_receipt_digest;', 'receipt helper imported')
    require(insert, 'use super::support::to_i64_len;', 'length helper imported')
    require_regex(
        insert,
        r'^pub\(in [^)]+\) async fn insert_tx\($',
        'insert transaction visibility widened',
    )
    forbid(insert, 'pub(super) async fn insert_tx(', 'old insert function visibility removed')

    support = 'codex-rs/hepta-memory/src/fact_grounding/durable/grounding/ledger/support.rs'
    require(support, 'use crate::framing::frame_part;', 'canonical frame helper imported')
    for function_name in (
        'stored_fact_supports',
        'durable_receipt_digest',
        'parse_fact_kind',
        'validate_span_range_corrupt',
        'to_i64_len',
        'limit_plus_one',
    ):
        require_regex(
            support,
            rf'^pub\(super\) (?:async )?fn {function_name}\(',
            f'{function_name} exported to ledger siblings',
        )
    forbid(support, 'super::super::frame_part', 'stale frame helper path removed')

    verify = 'codex-rs/hepta-memory/src/fact_grounding/durable/grounding/ledger/verify.rs'
    for function_name in (
        'durable_receipt_digest',
        'limit_plus_one',
        'parse_fact_kind',
        'stored_fact_supports',
        'validate_span_range_corrupt',
    ):
        require(
            verify,
            f'use super::support::{function_name};',
            f'{function_name} imported by verifier',
        )
    require_regex(
        verify,
        r'^pub\(in [^)]+\) async fn verify_receipts\($',
        'receipt verifier visibility widened',
    )
    forbid(verify, 'pub(super) async fn verify_receipts(', 'old verifier visibility removed')

    prepare = 'codex-rs/hepta-memory/src/fact_grounding/durable/grounding/prepare.rs'
    for function_name in (
        'validate_source_binding',
        'require_groundable_revision',
        'bind_exact_citation',
        'prepare',
        'validate_canonical_identity_binding',
    ):
        require_regex(
            prepare,
            rf'^pub\(in [^)]+\) fn {function_name}\(',
            f'{function_name} visibility widened',
        )
        forbid(
            prepare,
            f'pub(super) fn {function_name}(',
            f'old {function_name} visibility removed',
        )

    schema = 'codex-rs/hepta-memory/src/fact_grounding/durable/schema.rs'
    forbid(schema, 'use sqlx::Executor;', 'unused Executor import removed')

    shadow = 'codex-rs/hepta-memory/src/intelligence_mutation_shadow_host.rs'
    forbid(
        shadow,
        'use super::intelligence_mutation_state::IntelligenceMutationState;',
        'unused mutation-state import removed',
    )

    state = 'codex-rs/hepta-memory/src/intelligence_mutation_state.rs'
    state_text = text(state)
    drift_match = re.search(
        r'let (?P<name>drift(?:ed)?_request) = IntelligenceMutationTransitionRequest \{',
        state_text,
    )
    if drift_match is None:
        failures.append(
            {
                'path': state,
                'check': 'drift request avoids generic request shadowing',
                'missing_regex': 'let drift(?:ed)?_request = IntelligenceMutationTransitionRequest {',
            }
        )
    elif f"state.apply({drift_match.group('name')})" not in state_text:
        failures.append(
            {
                'path': state,
                'check': 'renamed drift request is consumed',
                'missing': f"state.apply({drift_match.group('name')})",
            }
        )

    if failures:
        raise RuntimeError(
            'Q0 compile repair is partial or semantically drifted:\n'
            + json.dumps(failures, indent=2, sort_keys=True)
        )


# Apply the historical patch only when the exact pre-repair source is present.
# Otherwise require every semantic postcondition and every old-marker absence.
if patch_applies():
    subprocess.run(['git', 'apply', str(PATCH)], check=True)
else:
    verify_absorbed_compile_repair()

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

# Accept exactly the complete pre-repair or complete post-repair state.
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
