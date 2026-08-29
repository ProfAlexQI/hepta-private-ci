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
    """Prove every compile-fix semantic postcondition in the current source."""

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


def rewrite_one_function(
    source: str,
    *,
    name: str,
    following_name: str,
    replacement: str,
) -> str:
    pattern = re.compile(
        rf'(?ms)^def {re.escape(name)}\(.*?(?=^def {re.escape(following_name)}\(|\Z)'
    )
    source, count = pattern.subn(lambda _: replacement.rstrip() + '\n\n', source, count=1)
    if count != 1:
        raise RuntimeError(f'frozen v7 helper boundary drifted: {name}')
    return source


def run_manual_repair_idempotently() -> None:
    """Execute frozen v7 with idempotent, fail-closed mutation helpers."""

    source = MANUAL.read_text(encoding='utf-8')
    source = rewrite_one_function(
        source,
        name='replace_exact',
        following_name='expect_function',
        replacement='''def replace_exact(path: str, old: str, new: str, *, expected: int = 1) -> None:
    file_path, text = load(path)
    old_count = text.count(old)
    if old_count == expected:
        save(file_path, text.replace(old, new, expected))
        return
    if old_count == 0:
        if new == '' or text.count(new) == expected:
            return
    raise AssertionError(
        f'{path}: replacement is partial or drifted; '
        f'old={old_count}, new={text.count(new) if new else "removed"}: {old[:100]!r}'
    )''',
    )
    source = rewrite_one_function(
        source,
        name='expect_function',
        following_name='__NO_NEXT_FUNCTION__',
        replacement='''def expect_function(path: str, name: str, lint: str, reason: str) -> None:
    file_path, text = load(path)
    pattern = re.compile(
        r'(?m)^(?P<indent>[ \\t]*)(?P<signature>'
        rf'(?:(?:pub(?:\\([^\\)]*\\))?)[ \\t]+)?'
        rf'(?:async[ \\t]+)?fn[ \\t]+{re.escape(name)}[ \\t]*\\('
    )
    matches = list(pattern.finditer(text))
    if len(matches) != 1:
        raise AssertionError(f'{path}: expected one function {name}, found {len(matches)}')
    match = matches[0]
    indent = match.group('indent')
    attribute = (
        f'{indent}#[expect(\\n'
        f'{indent}    clippy::{lint},\\n'
        f'{indent}    reason = "{reason}"\\n'
        f'{indent})]\\n'
    )
    prefix = text[: match.start()]
    if prefix.endswith(attribute):
        return
    existing_expect = re.search(
        rf'(?ms){re.escape(indent)}#\\[expect\\(.*?clippy::{re.escape(lint)},.*?\\)\\]\\n$',
        prefix[-1024:],
    )
    if existing_expect is not None:
        return
    lines = prefix.splitlines(keepends=True)
    if lines and re.fullmatch(
        rf'{re.escape(indent)}#\\[allow\\(clippy::{re.escape(lint)}(?:,.*)?\\)\\]\\n',
        lines[-1],
    ):
        prefix = ''.join(lines[:-1])
    save(file_path, prefix + attribute + text[match.start() :])


TOO_MANY_ARGUMENTS =''',
    )
    # The second replacement intentionally supplies the following top-level
    # assignment because the frozen script has no third helper function.
    source = source.replace(
        'TOO_MANY_ARGUMENTS =\nTOO_MANY_ARGUMENTS =',
        'TOO_MANY_ARGUMENTS =',
        1,
    )

    reservation = Path('codex-rs/hepta-memory/src/logical_turn_registry.rs')
    reservation_text = reservation.read_text(encoding='utf-8')
    alternative = (
        '#[derive(Clone, Debug, Eq, PartialEq)]\n'
        '#[allow(clippy::large_enum_variant, reason = "reservation variants preserve a stable explicit qualification API")]\n'
        'pub enum LogicalTurnReservation {'
    )
    original = '#[derive(Clone, Debug, Eq, PartialEq)]\npub enum LogicalTurnReservation {'
    repaired = (
        '#[expect(\n'
        '    clippy::large_enum_variant,\n'
        '    reason = "the frozen Q0 API owns complete verified attempt snapshots without hidden allocation"\n'
        ')]\n'
        '#[derive(Clone, Debug, Eq, PartialEq)]\n'
        'pub enum LogicalTurnReservation {'
    )
    states = (
        reservation_text.count(alternative),
        reservation_text.count(original),
        reservation_text.count(repaired),
    )
    if states == (1, 0, 0):
        reservation.write_text(
            reservation_text.replace(alternative, original, 1),
            encoding='utf-8',
        )
    elif states not in {(0, 1, 0), (0, 0, 1)}:
        raise RuntimeError(f'logical-turn reservation repair is partial or drifted: {states}')

    namespace = {'__name__': '__main__', '__file__': str(MANUAL)}
    exec(compile(source, str(MANUAL), 'exec'), namespace)


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
run_manual_repair_idempotently()
