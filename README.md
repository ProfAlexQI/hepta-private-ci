# Hepta private CI mirror

This repository is the hosted integration and qualification mirror for the
Hepta local-agent architecture built on the upstream Codex codebase. It is not
a production release channel, and a green source check does not grant runtime,
operator, promotion, or release authority.

## Current architecture authority

- Normative editable model: `docs/architecture/HEPTA_ARCHITECTURE_MODEL_V2.json`
- Generated human view: `ARCHITECTURE.md`
- Generated data-authority view: `docs/architecture/DATA_AUTHORITY_MAP.md`
- Compatibility projection: `docs/architecture/HEPTA_CURRENT_ARCHITECTURE_V1.json`
- Active execution plan: `docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_PLAN_V2.md`
- Gap ledger: `docs/architecture/HEPTA_ARCHITECTURE_GAP_LEDGER_V2.json`
- Qualification status: `docs/architecture/HEPTA_QUALIFICATION_STATUS_V2.json`
- Required repository policy: `docs/governance/HEPTA_REPOSITORY_RULESET_REQUIRED_V1.json`

Historical plans, Draft pull requests, captured external documents, generated
receipts, and queued or empty Actions runs are evidence or implementation
inputs only. They cannot mint capabilities or advance a qualification state.

## Development rule

Change the normative model first, then regenerate and verify every projection:

```shell
python3 scripts/generate-hepta-architecture-projections.py
python3 scripts/generate-hepta-architecture-projections.py --check
python3 scripts/verify-hepta-p0-5-gap-closure.py
```

Qualification workflows are read-only. They must not commit, push, update refs,
or rewrite the candidate under review.

## Runtime posture

The current local runtime profiles are closed. They do not grant model
invocation, provider dispatch, external effects, fleet mutation, operator
acceptance, promotion, or release. The cognitive-writer qualification profile
is build-time-only and remains non-production.

Runtime authority is bound to Agent identity, release/lifecycle epochs,
generation, fencing token, and the exact grant digest. Provider and external
effect paths require a current per-use verifier immediately before dispatch or
reconciliation.

## Upstream

Hepta retains the upstream Codex source, licenses, and notices. Upstream Codex
installation and product documentation are not Hepta release or authority
instructions.
