# Hepta Private CI

This repository is the verification-only GitHub mirror for Hepta development and hosted qualification. It is not, by itself, a production release, an operator-acceptance record, or an authority issuer.

## Start here

- [`docs/CURRENT.json`](docs/CURRENT.json) — machine-readable discovery index for the default baseline, the latest architecture-plan candidate, the active stacked candidates, and their observed evidence state.
- [`docs/STATUS.md`](docs/STATUS.md) — human-readable status generated from the same reviewed facts.
- [`docs/architecture/README.md`](docs/architecture/README.md) — document precedence, evidence rules, and the boundary between committed source and externally issued decisions.

The latest reviewed architecture-plan candidate is **Hepta Architecture Convergence Plan V5 v5.0.1** (`HEPTA-ARCHITECTURE-CONVERGENCE-V5`) on branch `codex/hepta-architecture-v5-b0-exact-restack-20260831`, commit `ad7845a8d67390299f86e931bab11d8b0ec13115`. The current source-development focus is the stacked B1b model-invocation verified-use boundary; its final source candidate has not yet been qualified or selected into the default branch.

## Current safety posture

All runtime, production-caller, production-writer, model-invocation, provider-dispatch, tool, network, external-filesystem, secret, Matrix-send, fleet-mutation, external-effect, operator-acceptance, promotion, and release authority is **false** in the discovery index.

A source file, generated artifact, queued or cancelled workflow, Draft pull request, fixture result, or self-issued receipt is never sufficient authority. Exact-head qualification, merge-candidate qualification, independent review, operator acceptance, promotion, and release remain separate facts.

## Development rules

1. Each durable fact has one schema owner and one authoritative writer.
2. Cross-owner mutation uses durable intent, outbox, destination dedupe/apply, acknowledgement, and reconciliation.
3. Irreversible boundaries require a final-payload-bound, operation-bound, revocation-revision-bound verified-use token.
4. `Indeterminate` outcomes are lookup-only until reconciled; blind retry is forbidden.
5. Qualification-only code and fixture signers must not link into production artifacts.
6. Dynamic observations expire; re-read GitHub Actions and exact Git identities before making a release or authority decision.

Run the repository-local baseline verifier with:

```bash
python3 scripts/verify-hepta-current-baseline.py
```
