# Hepta private CI mirror

This repository is the hosted integration and qualification mirror for the
Hepta local-agent architecture built on the upstream Codex codebase. It is not
a production release channel. Source, test and qualification artifacts cannot
grant runtime, operator, promotion or release authority.

## Current architecture authority

Resolve current architecture documents in this order:

1. `docs/architecture/HEPTA_CURRENT_PLAN.json`
2. `docs/architecture/HEPTA_ARCHITECTURE_MODEL_V2.json`
3. `docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_PLAN_V4.md`
4. `docs/architecture/HEPTA_ARCHITECTURE_GAP_LEDGER_V4.json`
5. `docs/architecture/HEPTA_QUALIFICATION_STATUS_V3.json`

The complete document classification and supersession rules are in
`docs/architecture/HEPTA_DOCUMENT_AUTHORITY_INDEX_V3.json`.

Generated views are:

- `ARCHITECTURE.md`
- `docs/architecture/DATA_AUTHORITY_MAP.md`
- `docs/architecture/HEPTA_CURRENT_ARCHITECTURE_V1.json`

They are regenerated from the V2 architecture model and must not be hand-edited.
V3 remains immutable historical provenance; it is no longer the current plan.

## Current execution package

V4 turns the remaining architecture work into ordered, independently reviewable
packages:

```text
P0.7a signed runtime bootstrap closure
  -> P0.7b per-use physical capability boundaries
  -> P0.7c Memory bounded-context extraction
  -> P0.7d common durable fault matrix execution
  -> P0.7e dependency inversion and wire isolation
  -> P0.8a AST/compiler authority caller ratchet
  -> P0.8b fleet runtime-instance projection
  -> P0.8c runtime resource-budget enforcement
  -> P0.8d exact real-process vertical slice
  -> P0.9 external repository/operator/promotion/release gates
```

The active P0.7a source already contains the signed bootstrap envelope,
Supervisor issuer, immutable trust/provenance registry and Agentd consumer. Its
remaining source-controlled closure items are committed lock coherence,
no-follow/owner-bound fallback transport and explicit crash-window recovery
tests. Exact runner and merge-candidate execution remain separate evidence.

## Development rule

Change the normative model first when architecture facts change, then regenerate
all projections. Change the selected Plan, gap ledger and qualification status
together when package order, exit criteria or observed state changes.

Run the source checks from the repository root:

```shell
python3 scripts/generate-hepta-architecture-projections.py --check
python3 scripts/verify-hepta-p0-5-gap-closure.py
python3 scripts/verify-hepta-cross-owner-operation-wiring.py
python3 scripts/verify-hepta-p0-6-runtime-authority.py
python3 scripts/verify-hepta-architecture-plan-v4.py
```

Rust changes follow `AGENTS.md`: use scoped `just test`, run formatting, keep
Cargo and Bazel locks in the same candidate, and retain a clean worktree.
Qualification workflows may regenerate files for comparison inside an ephemeral
runner, but they must not commit, push, update refs or rewrite the reviewed
candidate.

## Authority posture

Local profiles remain closed by default. A signed start-time bootstrap binds an
Agent, release, source tree, binary, runtime profile, ProductGraph, authority
epochs, generation, fence, validity window and one-use nonce. It does not
authorize a model call, provider dispatch, tool spawn, network connection,
external filesystem write, secret operation, Matrix send or fleet mutation.

Each physical crossing requires a current operation-bound verified-use token.
Neither source presence nor qualification activates authority. Queue admission
is not terminal effect success, and an indeterminate external result remains
open until a current-fence reconciler commits a legal terminal outcome.

## Repository and external gates

The checked-in repository policy is
`docs/governance/HEPTA_REPOSITORY_RULESET_REQUIRED_V1.json`. Live GitHub
administration, independent code-owner review, operator acceptance, promotion
and release are externally issued gates. Source code and Draft pull requests
cannot self-issue them.

Queued runs, runner ID zero, `jobs=[]`, `steps=[]`, cancelled jobs and generated
lock artifacts are not executable qualification.

## Upstream

Hepta retains the upstream Codex source, licenses and notices. Upstream Codex
installation and product documentation are not Hepta release or authority
instructions.
