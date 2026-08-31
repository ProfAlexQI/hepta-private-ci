# Hepta Private CI

This repository is a verification and qualification mirror for Hepta development. It does not issue production, operator, promotion, or release authority.

## Canonical development entry point

Read [`docs/DEVELOPMENT.md`](docs/DEVELOPMENT.md). It is the only global human-readable development authority in the working tree.

Machine-readable sources:

- [`docs/CURRENT.json`](docs/CURRENT.json) — time-bounded repository and candidate observations;
- [`docs/governance/DOCUMENT_SYSTEM.json`](docs/governance/DOCUMENT_SYSTEM.json) — document ownership, precedence, and anti-pollution rules;
- [`docs/architecture/ARCHITECTURE.json`](docs/architecture/ARCHITECTURE.json) — architecture invariants and control-plane boundaries;
- [`docs/modules/MODULES.json`](docs/modules/MODULES.json) — module ownership and data authority;
- [`docs/delivery/WORK_PACKAGES.json`](docs/delivery/WORK_PACKAGES.json) — delivery scope and package state;
- [`docs/evidence/INDEX.json`](docs/evidence/INDEX.json) — evidence rules and observed candidate identities;
- [`docs/STATUS.md`](docs/STATUS.md) — generated human status projection.

Historical development plans are not retained beside the current plan. Git history and immutable content-addressed evidence preserve provenance.

## Verify

```bash
python3 scripts/hepta-docs.py verify
```

To regenerate the human status projection:

```bash
python3 scripts/hepta-docs.py generate-status
```

All runtime, production, external-effect, operator, promotion, and release authority remains false until separately issued and exact-candidate bound.
