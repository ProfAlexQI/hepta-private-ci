# Hepta architecture convergence plan v2

**Effective date:** 2026-08-29  
**Parent:** PR #53 / `codex/hepta-architecture-convergence-p0-2-20260828`  
**Active child:** `codex/hepta-architecture-gap-closure-p0-5-20260829`

This plan is subordinate to the sole editable architecture catalog,
`HEPTA_ARCHITECTURE_CATALOG_V1.json`. It cannot grant runtime authority,
operator acceptance, promotion or release.

## State model

Source closure and executable qualification are separate:

```text
source: open | partial | closed
execution: not_run | queued | running | passed | failed | blocked | superseded
external decision: not_run | blocked | accepted | rejected
```

A source file never records itself as executable-qualified. Candidate commit,
tree, merge candidate, workflow run, job, runner and step identities exist only
in runtime-generated evidence.

## P0.5 — Authority, documentation and delivery convergence

Exit criteria:

- one editable architecture catalog;
- byte-generated architecture, data authority and runtime-profile views;
- exact Rust/catalog data-domain and authority-profile parity checks;
- explicit execution-state vocabulary without reversed booleans;
- repository governance contract, Hepta CODEOWNERS, README and security boundary;
- all architecture source-mutating workflows absent;
- source-head and merge-candidate qualification remain distinct.

## P0.6 — Runtime authority and resource safety

Blocking work:

1. create a runtime instance/readiness graph distinct from the canonical design graph;
2. replace schema-version-derived authority epochs and grant-digest pseudo-fences
   with lifecycle-owned epoch/fence inputs;
3. require per-use revocation/epoch validation for external capabilities;
4. enforce `max_concurrent_turns` and `max_tool_processes` at App Server admission;
5. execute stale-generation, revoked-lease, N+1 turn and N+1 tool tests.

No external effect, provider, model or promotion authority may be opened.

## P0.7 — Memory bounded contexts and real fault coverage

Extraction order:

1. `hepta-memory-model`;
2. `hepta-memory-store`;
3. `hepta-kg`;
4. `hepta-retrieval`;
5. `hepta-compact`;
6. `hepta-learning`;
7. reduce `hepta-memory` to compatibility reexports and remove it.

Every move preserves one database/migration owner and existing receipt digests.
The product-store fault matrix must cover Memory, Automation, Evidence and
Matrix with real SQLite transactions, process kill/reopen, pending outbox,
disk-full/write failure, stale generation, corruption and backup/restore.

## P0.8 — Executable and external closure

Repository-controlled completion requires:

- exact source-head fmt/test/check/Clippy;
- exact merge-candidate fmt/test/check/Clippy;
- non-zero runners and non-empty successful steps;
- immutable artifacts bound to the candidate;
- no source-mutating CI path.

The following remain independently issued external decisions:

- live GitHub ruleset matching the checked-in contract;
- independent review;
- physical-device or human evidence for domains that require it;
- operator acceptance;
- promotion and release.

## Permanent authority boundary

```text
production_caller=false
production_writer=false
effect_authority=false
external_effect=false
model_invocation_authority=false
provider_dispatch_authority=false
fleet_mutation_authority=false
operator_acceptance=false
promotion=false
release=false
```
