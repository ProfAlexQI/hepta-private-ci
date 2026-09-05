# Durable learning cross-crate qualification slice

This extends the existing C1/ART/LRN qualification track, not the production host.
It composes the actual `DurableLedger` and artifact file APIs rather than an
in-memory substitute or separate Python implementation. No dependencies change.

The regression writes complete training decisions and independently named
observer outcomes to a file, closes/reopens against an acknowledgement anchor,
fits a bounded binary fixture policy ONLY from reopened eligible records, compares
it against a separate held-out oracle, persists baseline/candidate payloads and
registry history, reopens both, and creates distinct immutable fixture-run values.
It loads the predecessor through CURRENT registry eligibility to exercise rollback.
Then it persists revocation, rejects the old registry with the new witness,
rejects revoked candidate bytes, and rejects rollback after predecessor revocation.

Run with the repository test entrypoint:

    just test --locked -p codex-hepta-shadow-qualification --test durable_learning_roundtrip

The Bazel target is `//codex-rs/hepta-shadow-qualification:durable-learning-roundtrip`.
The dedicated CI runs exact source and actual-base synthetic merge independently,
plus ledger and artifact regressions. Compilation/execution remains pending until
those exact checks actually pass; no local success is inferred.

This is deterministic integration qualification, NOT longitudinal efficacy,
authenticated independent evaluation, the production C1 retrieval path, NDU
training, cross-fitted OPE or a production selection/rollback transaction. The
small fixture learner and oracle are deliberately labeled as such. Expected
recovery witnesses are retained in the test harness, not a production independent
witness service. Filesystem fixtures do not qualify physical power loss.

Remaining product attachment must provide authenticated observer/evaluator and
operator identities, preregistered evaluation using `learning.eval`, independently
durable current witnesses, normal host/port consumers, dataset-to-artifact
revocation orchestration, cross-store outbox reconciliation, target-host resource
measurements, real future windows and independently governed next-run selection.
A passed roundtrip would close only this integration sub-slice, never these gates.
