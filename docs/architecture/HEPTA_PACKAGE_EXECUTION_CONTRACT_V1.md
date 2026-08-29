# Hepta package execution contract V1

**Status:** normative source-development operating contract. It does not grant
runtime, production, operator, promotion or release authority.

This contract applies to every package selected by
`docs/architecture/HEPTA_CURRENT_PLAN.json`.

## 1. Exact base and branch

Before every write, record and revalidate:

- repository full name;
- parent branch, commit and tree;
- package branch;
- current branch head and tree;
- open parent PR and merge-base relationship.

Use one isolated package branch and one PR. Never write directly to the default
branch, never force-update a reviewed package, and never merge your own PR.
Base drift stops the package as `base_drift`; rebase and requalify instead of
reusing old receipts.

## 2. Closed source scope

A package declares a closed touched-path allowlist before implementation.
Generated Cargo/Bazel locks, schemas and projections are included when their
source changes. Unrelated formatting, refactors and adjacent feature work are
forbidden. Any unexpected path is restored or the package is rejected and
respecified before review.

Qualification workflows are read-only. They may create ephemeral comparison
output and upload diagnostics, but may not commit, push, update refs, mutate
repository administration, or rewrite the reviewed candidate.

## 3. Authority posture

The default package authority delta is exactly:

```json
{
  "runtime": false,
  "productionCaller": false,
  "productionWriter": false,
  "modelInvocation": false,
  "providerDispatch": false,
  "toolExecution": false,
  "networkConnect": false,
  "externalEffect": false,
  "fleetMutation": false,
  "operatorAcceptance": false,
  "promotion": false,
  "release": false
}
```

Source cannot issue an external decision. Any package that requires a non-false
value stops at the corresponding external gate.

## 4. Durable ownership

Every durable object declares:

- sole writer;
- schema and migration owner;
- key/idempotency namespace;
- revision, authority epoch, owner epoch, generation and fence;
- transaction boundary;
- outbox/inbox and acknowledgement semantics;
- legal terminal states;
- backup, restore and migration recovery.

A process may coordinate multiple stores but may not claim a cross-database
atomic transaction. Cross-owner work uses a committed intent/outbox and
lookup-only reconciliation.

## 5. Physical boundaries

A package crossing a model, provider, tool, network, filesystem, secret,
Matrix, fleet, operator or release boundary declares the corresponding row from
`HEPTA_PHYSICAL_CAPABILITY_BOUNDARY_MATRIX_V1.md`. The final adapter verifies and
consumes a current operation-bound token immediately before the irreversible
crossing. The adapter cannot mint the token it consumes.

Unknown outcomes are not silently retried or closed. Queue acknowledgement is
not terminal success.

## 6. Fault and resource rows

Every durable package declares applicable F01–F18 rows from
`HEPTA_COMMON_DURABLE_FAULT_MATRIX_V1.md`. Every admission or execution package
declares resource rows from `HEPTA_RESOURCE_BUDGETS_V1.md`.

A missing row is `not_run`. `not_applicable` requires an explicit reviewed
reason. An in-memory or model fault cannot substitute for a required physical
fault.

## 7. Required source verification

At minimum:

```shell
python3 scripts/generate-hepta-architecture-projections.py --check
python3 scripts/verify-hepta-architecture-plan-v5.py
python3 scripts/verify-hepta-dependency-policy.py
cargo metadata --manifest-path codex-rs/Cargo.toml --locked --no-deps
just bazel-lock-check
```

Rust packages additionally run package-scoped formatting, tests, `cargo check
--locked --all-targets` and strict Clippy. Schema, golden and compile-fail
fixtures run when their package changes.

Source verification proves deterministic source consistency only. It does not
substitute for hosted execution.

## 8. Executable evidence

Exact-head and merge-candidate evidence must bind:

- commit and tree;
- synthetic merge identity where applicable;
- runner and job IDs;
- non-empty step list and conclusions;
- toolchain and verifier digests;
- test binary and raw-log digests;
- applicable state, fault, resource and authority assertions.

Queued, skipped, cancelled, empty-job, empty-step and runner-zero observations
remain `not_run`.

## 9. Review and external decisions

A distinct current-head code owner reviews the package and resolves all
conversations. Operator acceptance, promotion and release are separate signed
decisions over the same immutable candidate and evidence manifest. Any source
change invalidates downstream decisions.

## 10. Package outcomes

Allowed outcomes are:

- `source_verified` or `merge_candidate_qualified` when criteria are met;
- `base_drift` when the exact parent changed;
- `blocked_upstream` when a predecessor contract is absent;
- `blocked_external` when a runner, physical platform, repository setting,
  reviewer or signer is unavailable;
- `rejected` when evidence or invariants fail;
- `resume_required` when a durable partial operation must be inspected.

Never convert a blocked or unexecuted requirement into a pass.
