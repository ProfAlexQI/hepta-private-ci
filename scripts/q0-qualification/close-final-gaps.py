#!/usr/bin/env python3
'''Apply the exact qualification-only Q0 gap closure patch.

This script is a temporary CI carrier. The workflow deletes it before running
the canonical source gates and before committing the verified repair.
'''

from __future__ import annotations

from pathlib import Path
import re


def read(path: str) -> str:
    return Path(path).read_text(encoding='utf-8')


def write(path: str, text: str) -> None:
    Path(path).write_text(text, encoding='utf-8')


def replace_exact(
    path: str,
    old: str,
    new: str,
    *,
    expected: int = 1,
) -> None:
    text = read(path)
    count = text.count(old)
    if count != expected:
        raise AssertionError(
            f'{path}: expected {expected} exact matches, found {count}: {old!r}'
        )
    write(path, text.replace(old, new, expected))


def insert_lint(path: str, marker: str, lint: str, reason: str) -> None:
    text = read(path)
    needle = marker.lstrip()
    pattern = re.compile(rf'(?m)^(?P<indent>[ \t]*){re.escape(needle)}')
    matches = list(pattern.finditer(text))
    if len(matches) != 1:
        raise AssertionError(
            f'{path}: expected one lint target, found {len(matches)}: {needle!r}'
        )
    match = matches[0]
    indent = match.group('indent')
    attribute = f'{indent}#[allow({lint}, reason = "{reason}")]\n'
    updated = text[: match.start()] + attribute + text[match.start() :]
    write(path, updated)


def convert_constant_assertions(path: str) -> int:
    text = read(path)
    pattern = re.compile(
        r'(?m)^(?P<indent>[ \t]*)assert!\('
        r'(?P<expr>!?(?:(?:crate|super)::)?[A-Z][A-Z0-9_]*)'
        r'\);[ \t]*$'
    )

    def repl(match: re.Match[str]) -> str:
        return (
            f"{match.group('indent')}const {{ "
            f"assert!({match.group('expr')}); }}"
        )

    updated, count = pattern.subn(repl, text)
    if count:
        write(path, updated)
    return count


# Keep the read-only qualification seam without an unfulfilled `expect` lint.
replace_exact(
    'codex-rs/hepta-agentd/src/app_runtime.rs',
    '''#[expect(
    dead_code,
    reason = "the read-only constructor remains a qualification and regression seam"
)]
''',
    '''#[allow(
    dead_code,
    reason = "the read-only constructor remains a qualification and regression seam"
)]
''',
)

# Keep verifier markers aligned with the canonical --no-deps matrix.
old_clippy = (
    '"cargo clippy --locked -p codex-hepta-memory '
    '--all-targets -- -D warnings",'
)
new_clippy = (
    '"cargo clippy --locked -p codex-hepta-memory '
    '--all-targets --no-deps -- -D warnings",'
)
for verifier in (
    'scripts/verify-hepta-intelligence-mutation-state.py',
    'scripts/verify-hepta-intelligence-mutation-journal.py',
):
    replace_exact(verifier, old_clippy, new_clippy)

# Close durable grounding module visibility/import drift without widening the
# public crate API.
prepare = (
    'codex-rs/hepta-memory/src/fact_grounding/'
    'durable/grounding/prepare.rs'
)
for function in (
    'validate_source_binding',
    'require_groundable_revision',
    'bind_exact_citation',
    'prepare',
    'validate_canonical_identity_binding',
):
    replace_exact(
        prepare,
        f'pub(super) fn {function}(',
        f'pub(in super::super) fn {function}(',
    )

insert = (
    'codex-rs/hepta-memory/src/fact_grounding/'
    'durable/grounding/ledger/insert.rs'
)
replace_exact(
    insert,
    'use super::*;\n',
    'use super::*;\n'
    'use super::support::durable_receipt_digest;\n'
    'use super::support::to_i64_len;\n',
)
replace_exact(
    insert,
    'pub(super) async fn insert_tx(',
    'pub(in super::super::super) async fn insert_tx(',
)

verify = (
    'codex-rs/hepta-memory/src/fact_grounding/'
    'durable/grounding/ledger/verify.rs'
)
replace_exact(
    verify,
    'use super::*;\n',
    'use super::*;\n'
    'use super::support::durable_receipt_digest;\n'
    'use super::support::limit_plus_one;\n'
    'use super::support::parse_fact_kind;\n'
    'use super::support::stored_fact_supports;\n'
    'use super::support::validate_span_range_corrupt;\n',
)
replace_exact(
    verify,
    'pub(super) async fn verify_receipts(',
    'pub(in super::super::super) async fn verify_receipts(',
)

ledger = (
    'codex-rs/hepta-memory/src/fact_grounding/'
    'durable/grounding/ledger.rs'
)
replace_exact(
    ledger,
    'pub(super) use insert::insert_tx;',
    'pub(in super::super) use insert::insert_tx;',
)
replace_exact(
    ledger,
    'pub(super) use verify::verify_receipts;',
    'pub(in super::super) use verify::verify_receipts;',
)

support = (
    'codex-rs/hepta-memory/src/fact_grounding/'
    'durable/grounding/ledger/support.rs'
)
replace_exact(
    support,
    'use super::*;\n',
    'use super::*;\nuse crate::framing::frame_part;\n',
)
replace_exact(
    support,
    'async fn stored_fact_supports(',
    'pub(super) async fn stored_fact_supports(',
)
for function in (
    'durable_receipt_digest',
    'parse_fact_kind',
    'validate_span_range_corrupt',
    'to_i64_len',
    'limit_plus_one',
):
    replace_exact(
        support,
        f'fn {function}(',
        f'pub(super) fn {function}(',
    )
replace_exact(
    support,
    'super::super::frame_part',
    'frame_part',
    expected=16,
)

replace_exact(
    'codex-rs/hepta-memory/src/fact_grounding/durable/schema.rs',
    'use sqlx::Executor;\n',
    '',
)
replace_exact(
    'codex-rs/hepta-memory/src/intelligence_mutation_shadow_host.rs',
    'use super::intelligence_mutation_state::IntelligenceMutationState;\n',
    '',
)

# Avoid shadowing the request helper in the binding-drift regression.
state = 'codex-rs/hepta-memory/src/intelligence_mutation_state.rs'
replace_exact(
    state,
    '        let request = IntelligenceMutationTransitionRequest {\n'
    '            binding: drifted,',
    '        let drifted_request = IntelligenceMutationTransitionRequest {\n'
    '            binding: drifted,',
)
replace_exact(
    state,
    '            state.apply(request),\n'
    '            Err(IntelligenceMutationStateError::BindingDrift)',
    '            state.apply(drifted_request),\n'
    '            Err(IntelligenceMutationStateError::BindingDrift)',
)

# Row mirrors keep chain fields for exact schema/query parity.
for row in ('EventRow', 'OutboxRow'):
    insert_lint(
        'codex-rs/hepta-memory/src/local_lease_outbox.rs',
        f'struct {row} {{',
        'dead_code',
        'row mirrors preserve the complete durable chain shape for verification',
    )

# High-arity constructors and digests encode fixed ordered protocol fields.
TOO_MANY_ARGUMENTS: dict[str, tuple[str, ...]] = {
    'codex-rs/hepta-memory/src/h7_trajectory_store.rs': (
        '    pub fn turn_start(',
        '    pub fn terminal(',
        'async fn verify_historical_lease(',
        'async fn insert_row(',
    ),
    'codex-rs/hepta-memory/src/local_compact_executor.rs': (
        'async fn historical_compact_lease_expiry(',
        'async fn verify_historical_compact_lease_binding(',
    ),
    'codex-rs/hepta-memory/src/local_lease_outbox.rs': (
        '    pub async fn acquire_local_lease_after_head_bound(',
        '    pub async fn acquire_host_bound_lease_after_head(',
        'pub(crate) async fn append_lease(',
        'fn lease_digest(',
        'fn event_digest(',
        'fn outbox_digest(',
    ),
    'codex-rs/hepta-memory/src/local_memory_saga.rs': (
        '    pub async fn tombstone_memory_candidate_saga(',
        '    async fn replay_tombstone(',
    ),
    'codex-rs/hepta-memory/src/logical_turn_registry.rs': (
        'async fn append_attempt(',
        'fn attempt_digest(',
        'fn attempt_digest_without_scope(',
    ),
    'codex-rs/hepta-memory/src/memory_admission.rs': (
        '    pub async fn verify_memory_candidate(',
        '    pub async fn tombstone_memory_candidate(',
    ),
    'codex-rs/hepta-memory/src/local_compact_executor_tests.rs': (
        'fn process_soak_child_command(',
    ),
    'codex-rs/hepta-memory/src/local_lease_outbox_tests.rs': (
        'async fn insert_test_transition(',
        'fn test_event_digest(',
    ),
    'codex-rs/hepta-memory/src/h7_feedback_tests.rs': (
        'fn record(',
    ),
    'codex-rs/ext/hepta-memory/src/extension.rs': (
        'pub fn install_with_turn_writer<C, F>(',
    ),
    'codex-rs/ext/hepta-memory/src/local_replay.rs': (
        'fn replay_binding_digest(',
    ),
    'codex-rs/ext/hepta-memory/src/local_runtime.rs': (
        'fn binding_digest(',
    ),
    'codex-rs/ext/hepta-memory/src/local_witness.rs': (
        '    pub fn with_policy(',
    ),
}
for path, markers in TOO_MANY_ARGUMENTS.items():
    for marker in markers:
        insert_lint(
            path,
            marker,
            'clippy::too_many_arguments',
            'the signature is an explicit ordered protocol or test-harness contract',
        )

insert_lint(
    'codex-rs/hepta-memory/src/logical_turn_registry.rs',
    'pub enum LogicalTurnReservation {',
    'clippy::large_enum_variant',
    'reservation variants preserve a stable explicit qualification API',
)
insert_lint(
    'codex-rs/hepta-memory/src/intelligence_mutation_journal_v3/schema.rs',
    'async fn verify_schema_oracle(',
    'clippy::disallowed_methods',
    'the isolated in-memory schema oracle does not create a runtime state database',
)
insert_lint(
    'codex-rs/hepta-memory/src/h7_feedback.rs',
    '    fn from(key: H7FeedbackKey) -> Self {',
    'clippy::expect_used',
    'serializing a tuple of strings and an integer is infallible',
)

# Propagate impossible prior-row absence as corruption rather than panic.
replace_exact(
    'codex-rs/hepta-memory/src/local_lease_outbox.rs',
    '            let prior = latest.as_ref().expect("lease prior row");',
    '''            let Some(prior) = latest.as_ref() else {
                return Err(corrupt("lease journal is missing its prior row"));
            };''',
)

# End the synchronous lock guard before asynchronous I/O.
replace_exact(
    'codex-rs/ext/hepta-memory/src/local_lifecycle.rs',
    '''        let guard = restarted_state
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        assert!(guard.active.is_none());
        assert!(guard.terminal_started);
        drop(guard);''',
    '''        {
            let guard = restarted_state
                .lock()
                .unwrap_or_else(PoisonError::into_inner);
            assert!(guard.active.is_none());
            assert!(guard.terminal_started);
        }''',
)

# Compile immutable authority-boundary assertions at const-evaluation time.
constant_assertion_files = (
    'codex-rs/hepta-memory/src/local_atomic_witness_tests.rs',
    'codex-rs/hepta-memory/src/compact_persistence_tests.rs',
    'codex-rs/hepta-memory/src/local_compact_executor_tests.rs',
    'codex-rs/hepta-memory/src/local_lease_outbox_tests.rs',
    'codex-rs/hepta-memory/src/logical_turn_registry_tests.rs',
    'codex-rs/hepta-memory/src/h7_feedback_tests.rs',
    'codex-rs/ext/hepta-memory/src/extension_tests.rs',
    'codex-rs/ext/hepta-memory/src/cognitive/grounding_v3.rs',
    'codex-rs/ext/hepta-memory/src/local_replay.rs',
    'codex-rs/ext/hepta-memory/src/local_turn_writer.rs',
)
converted = {path: convert_constant_assertions(path) for path in constant_assertion_files}
if sum(converted.values()) != 26:
    raise AssertionError(
        f'expected 26 constant authority assertions, converted {converted}'
    )

# Iterate witnesses directly while retaining the deterministic operation index.
loop_path = 'codex-rs/hepta-memory/src/local_compact_executor_tests.rs'
replace_exact(
    loop_path,
    '    for operation_index in 0..OPERATIONS {\n'
    '        let operation_id = format!("op:seeded:{operation_index:04}");',
    '    for (operation_index, witness_sequence) in\n'
    '        witness_sequences.iter().enumerate().take(OPERATIONS)\n'
    '    {\n'
    '        let operation_id = format!("op:seeded:{operation_index:04}");',
)
replace_exact(
    loop_path,
    '            witness_sequences[operation_index]\n'
    '        );',
    '            *witness_sequence\n'
    '        );',
)

print('PASS_HEPTA_Q0_FINAL_GAP_PATCH_V1')
