# Architecture and Evidence Document Precedence

This directory contains both normative architecture candidates and historical or generated material. A filename containing `CURRENT`, `FINAL`, `PASS`, or a version number does not grant authority by itself.

## Resolution order

For repository discovery, read:

1. `/docs/CURRENT.json` — default-branch discovery index and active-stack identity;
2. `/docs/STATUS.md` — human projection of the same reviewed facts;
3. the `normativePlanCandidate` branch, commit, tree, and plan path named by `CURRENT.json`;
4. that candidate's architecture model, current-plan pointer, gap ledger, qualification-status contract, and package-specific contracts;
5. exact GitHub Actions evidence and independently issued decisions bound to the same candidate.

The default-branch discovery index does not silently promote a candidate plan into production. A candidate remains a candidate until the repository's separately governed selection and merge process completes.

## Document classes

- **Normative source:** architecture model, selected development plan, protocol contract, migration contract, or fault/resource contract on an exact candidate.
- **Generated projection:** human-readable or compatibility output derived deterministically from normative source. Do not hand-edit it.
- **Observed status:** time-bounded facts read from Git and GitHub Actions. Observations become stale after their declared validity period.
- **Qualification evidence:** exact candidate, runner, steps, tests, artifacts, and digests. It cannot issue independent human or operator decisions.
- **External decision:** independent review, physical evidence, operator acceptance, promotion, or release issued by the designated actor.
- **Historical/archive:** superseded plans, receipts, Dropbox snapshots, old PR descriptions, or provenance retained for audit only.

Unknown classes fail closed.

## Source, execution, and authority are separate

The following implications are forbidden:

```text
source present        != executable qualification
workflow queued       != runner execution
tests green           != semantic completeness
merge candidate green != operator acceptance
operator acceptance   != promotion
promotion             != release
```

An irreversible action requires a final-payload-bound, operation-bound, revocation-revision-bound verified-use token consumed at the physical boundary. A broad capability, runtime bootstrap, queue acknowledgement, generated source package, or source-only receipt is insufficient.

## Dynamic evidence rules

Every observed run must retain:

- source commit and tree;
- merge-candidate identity when applicable;
- workflow path and workflow blob identity;
- run attempt, job ID, runner ID, and non-empty step records;
- terminal GitHub conclusion;
- artifact ID, digest, and expiry when an artifact is required;
- exact package and test inventory;
- all negative authority flags.

Use GitHub's actual terminal vocabulary, including `cancelled`, `timed_out`, `skipped`, `neutral`, and `action_required`. Do not collapse these states into `queued`, `failed`, or `passed`.

## Repository-controlled versus external closure

Repository-controlled work may close source, format, compile, test, fault-injection, migration, and exact-candidate evidence gaps. It may not synthesize:

- an independent reviewer identity;
- physical-device evidence;
- real model/provider/corpus efficacy evidence;
- human accessibility or visual acceptance;
- candidate-bound operator acceptance;
- a production trust-root ceremony;
- promotion or release authority.

When repository-controlled work is complete and one of these facts is absent, the truthful state is `blocked_external`, not `closed` and not `failed`.
