# Hepta private CI mirror

This repository is the hosted integration and qualification mirror for the
Hepta local-agent architecture built on the upstream Codex codebase. It is not
a production release channel. Source, test and qualification artifacts cannot
grant runtime, operator, promotion or release authority.

## Current architecture authority

Resolve current development truth in this order:

1. `docs/architecture/HEPTA_CURRENT_PLAN.json`
2. `docs/architecture/HEPTA_ARCHITECTURE_MODEL_V2.json`
3. `docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_PLAN_V5.md`
4. `docs/architecture/HEPTA_ARCHITECTURE_GAP_LEDGER_V5.json`
5. `docs/architecture/HEPTA_PACKAGE_EXECUTION_CONTRACT_V1.md`
6. `docs/architecture/HEPTA_DEPENDENCY_POLICY_V1.json`
7. `docs/architecture/HEPTA_QUALIFICATION_STATUS_V4.json`

The complete classification and supersession rules are in
`docs/architecture/HEPTA_DOCUMENT_AUTHORITY_INDEX_V4.json`. V4 and earlier
plans remain immutable historical provenance.

Generated views are `ARCHITECTURE.md`,
`docs/architecture/DATA_AUTHORITY_MAP.md` and
`docs/architecture/HEPTA_CURRENT_ARCHITECTURE_V1.json`. They are generated from
the V2 architecture model and must not be hand-edited.

## Delivery graph

```text
P0.7a signed runtime-bootstrap closure
  -> P0.7b per-use verified capability boundaries
  -> P0.7c Memory bounded-context extraction
  -> P0.7d common durable fault matrix
  -> P0.7e dependency inversion and wire isolation
  -> P0.8a AST/compiler authority caller ratchet
  -> P0.8b fleet runtime-instance projection
  -> P0.8c runtime resource-budget enforcement
  -> P0.8d exact real-process vertical slice
  -> P0.9 repository/operator/promotion/release gates
```

The audited parent already carries the signed bootstrap implementation and
committed Cargo/Bazel locks. Transport and crash-recovery source are present but
not executable evidence. A temporary source-mutating workflow, exact runner
execution, dependency debt, Memory physical extraction, the common fault
aggregate and the P0.8 product slices remain explicit gaps in the V5 ledger.

## Development rules

Every package follows
`docs/architecture/HEPTA_PACKAGE_EXECUTION_CONTRACT_V1.md`:

- revalidate exact parent commit and tree before each write;
- use one isolated package branch and PR;
- keep touched paths closed and restore unrelated generator/formatter churn;
- keep qualification workflows read-only;
- update current Plan, ledger, status and document index together;
- commit Cargo/Bazel locks and generated projections with their source change;
- never infer execution success from queued, runner-zero or empty-step runs;
- never self-issue repository, operator, promotion or release authority.

Run source checks from the repository root:

```shell
python3 scripts/generate-hepta-architecture-projections.py --check
python3 scripts/verify-hepta-p0-5-gap-closure.py
python3 scripts/verify-hepta-cross-owner-operation-wiring.py
python3 scripts/verify-hepta-p0-6-runtime-authority.py
python3 scripts/verify-hepta-architecture-plan-v5.py
python3 scripts/verify-hepta-dependency-policy.py
```

Rust changes follow `AGENTS.md`: use package-scoped `just test`, run formatting
and strict linting, and leave the candidate clean.

## Authority posture

Local profiles remain closed by default. A signed start bootstrap binds Agent,
release, source, binary, runtime profile, ProductGraph, epochs, generation,
fence, validity window and one-use nonce. It authorizes start identity only.

Every model, provider, tool, network, external filesystem, secret, Matrix or
fleet crossing requires a fresh operation-bound verified-use capability at the
final adapter. Queue admission is not terminal success. An indeterminate
external result remains open until lookup-only reconciliation commits a legal
terminal outcome.

## Repository and external gates

The checked-in repository policy is
`docs/governance/HEPTA_REPOSITORY_RULESET_REQUIRED_V1.json`. Live GitHub
administration, assigned runners, merge-candidate execution, independent
code-owner review, operator acceptance, promotion and release are independently
issued gates. Source and Draft PRs cannot self-issue them.

## Upstream

Hepta retains upstream Codex source, licenses and notices. Upstream Codex
installation and product documentation are not Hepta release or authority
instructions.
