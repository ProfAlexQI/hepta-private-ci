# Hepta private CI mirror

This repository is the hosted qualification and integration mirror for the
Hepta local-agent architecture built on the upstream Codex codebase. It is not
a production release channel.

## Current authority

- Editable architecture facts: `docs/architecture/HEPTA_ARCHITECTURE_CATALOG_V1.json`
- Generated human view: `ARCHITECTURE.md`
- Current execution plan: `docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_PLAN_V2.md`
- Execution status vocabulary: `docs/architecture/HEPTA_ARCHITECTURE_EXECUTION_STATUS_V2.json`
- Required repository governance: `docs/governance/HEPTA_REPOSITORY_RULESET_REQUIRED_V1.json`

Historical plans, Draft pull requests, source receipts and queued Actions runs
are evidence or development inputs. They cannot mint runtime capabilities,
operator acceptance, promotion or release.

## Development rule

Change the architecture catalog, then regenerate and verify its views:

```shell
python3 scripts/generate-hepta-architecture-views.py --write
python3 scripts/generate-hepta-architecture-views.py --check
python3 scripts/verify-hepta-architecture-catalog.py
```

CI is read-only. It must not commit, push, update refs or rewrite the candidate.

## Runtime posture

The current local profiles are closed. They do not grant model invocation,
provider dispatch, external effects, fleet mutation, operator acceptance,
promotion or release. The qualification cognitive-writer profile is
build-time-only and remains non-production.

## Upstream

Hepta retains the upstream Codex source, licenses and notices. Upstream Codex
installation and product documentation are not Hepta release or authority
instructions.
