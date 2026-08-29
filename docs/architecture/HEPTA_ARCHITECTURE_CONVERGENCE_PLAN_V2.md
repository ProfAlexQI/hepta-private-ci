# Hepta architecture convergence plan v2

**Status:** active source-convergence plan for P0.5 and later tranches.
**Normative architecture:** `HEPTA_ARCHITECTURE_MODEL_V2.json`.
**Supersession:** this plan supersedes stale execution-status assertions in
`HEPTA_ARCHITECTURE_CONVERGENCE_PLAN_V1.md`; V1 remains immutable provenance.

This plan cannot grant runtime, operator, promotion, or release authority. Every
tranche moves through the same ordered states:

```text
planned
→ source_implemented
→ exact_source_head_qualified
→ merge_candidate_qualified
→ operator_accepted
→ promoted
→ released
```

No state may be inferred from a later-looking filename, source presence, a local
fixture, a queued job, `runner_id=0`, empty steps, or a generated receipt.

## P0.5 — Architecture authority and runtime-profile binding

Source implementation branch:
`codex/hepta-gap-closure-p0-5-20260829`, based on exact parent
`5e6cc0d84a33920e99c9b1adaf7abcba91fddda1`.

Source scope:

1. one normative V2 architecture model;
2. deterministic generated Markdown and V1 compatibility projections;
3. explicit separation of runtime, external-control and qualification-evidence
   data domains;
4. a Rust runtime-profile contract defining required, optional and disabled
   services;
5. composition-time binding of that profile to the exact authority digest and
   ProductGraph;
6. unambiguous qualification status using state enums rather than inverted
   booleans;
7. a read-only source-head and merge-candidate qualification workflow.

P0.5 exits `source_implemented` only when the above files are present and the
source verifier passes. It exits executable qualification only after hosted
Rust 1.95 fmt/test/check/Clippy runs have real runners and non-empty successful
steps for both source and merge identities.

## P0.6 — Signed runtime authority and live revocation

Required work:

1. replace normal in-process grant construction with a Supervisor-issued,
   signed, candidate-bound runtime grant bootstrap;
2. retain closed local profiles only as explicitly non-production test or
   offline utility profiles;
3. persist monotonic authority and owner epochs;
4. validate revocation/current epoch immediately before every model, provider,
   tool and external-effect boundary;
5. make stale generation, stale fence, key rotation, revocation, expiry and
   clock-regression tests mandatory;
6. prohibit production artifacts from linking qualification grant constructors.

Exit evidence must include compiler-level caller control, negative runtime tests
and a real multi-process stale-owner race. Source scans alone are insufficient.

## P0.7 — Physical bounded contexts and common durable fault matrix

Extraction order remains:

1. `hepta-memory-model`;
2. `hepta-memory-store`;
3. `hepta-kg`;
4. `hepta-retrieval`;
5. `hepta-compact`;
6. `hepta-learning`;
7. legacy `hepta-memory` compatibility reexports and deletion.

Each step preserves one database/migration owner and moves callers through the
stable `hepta-memory-runtime` facade before source relocation. No cross-crate
transaction may be described as atomic.

Memory, Automation, Matrix and Evidence must all pass the same product-store
matrix: kill before/after commit, WAL reopen, SQLite full, permission loss,
corruption, stale fence, pending outbox, acknowledgement loss, terminal reopen,
backup, restore and schema compatibility. Matrix-only coverage cannot close the
aggregate gap.

## P0.8 — Runtime instance graph, budgets and physical vertical slice

Required work:

- produce generation-fenced desired, observed and readiness graphs;
- enforce turn, tool-process, model, memory, disk and queue budgets at physical
  admission points;
- prove permit release on cancellation, terminal events, crash and restart;
- run one exact-candidate real-process vertical slice:

```text
Supervisor grant/lifecycle
→ Agentd composition and readiness
→ App Server session
→ approved model/provider boundary
→ Memory read/write and compact
→ Automation occurrence
→ Matrix/UI observation
→ kill/restart/lookup-only reconcile
```

The slice must not substitute fixture signers, source-only receipts or synthetic
physical-device evidence.

## P0.9 — Repository, operator, promotion and release closure

External gates remain separate:

1. default-branch ruleset requires aggregate source and merge checks;
2. direct/force push and branch deletion are blocked;
3. independent CODEOWNER review completes;
4. candidate-bound operator acceptance is signed;
5. SBOM, dependency provenance, release manifest and artifacts are signed;
6. canary and rollback drills complete;
7. promotion and release receipts are independently issued.

Repository configuration and human decisions must never be self-issued by a
source commit or CI fixture.

## Current authority boundary

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
