# Hepta architecture convergence plan v2

**Status:** active source-convergence plan for P0.6 and later tranches.  
**Normative architecture:** `HEPTA_ARCHITECTURE_MODEL_V2.json`.  
**Qualification status:** `HEPTA_QUALIFICATION_STATUS_V2.json`.  
**Gap ledger:** `HEPTA_ARCHITECTURE_GAP_LEDGER_V2.json`.  
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

## Current exact source stack

- parent PR: `#53`;
- parent branch: `codex/hepta-architecture-convergence-p0-2-20260828`;
- exact parent head used to create this tranche:
  `58ef2df827b6dcf83ddf1fffa3139339cf73eb1f`;
- active source branch: `codex/hepta-gap-closure-p0-6-20260829`;
- Draft PR: `#71`;
- exact source commit/tree and merge candidate are resolved only by the
  read-only qualification workflow for the current head.

## P0.5 — Architecture authority and runtime-profile binding

Current source state: `source_implemented`; executable qualification remains
external and candidate-bound.

Implemented scope:

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

P0.5 exits executable qualification only after hosted Rust 1.95
fmt/test/check/Clippy jobs have real runners and non-empty successful steps for
both source and merge identities.

## P0.6 — Runtime authority, live revocation and instance readiness

Current source state: `source_implemented_partial`; the completed source portion
does not imply production authority.

Implemented source:

1. `RuntimeAuthorityContext` binds Agent identity, authority epoch, owner epoch,
   generation, fencing token and exact grant digest;
2. Agentd derives its local closed-profile runtime context from the persisted
   release generation and Agent lifecycle generation instead of substituting a
   schema version for an authority epoch;
3. Memory and Automation validate and consume the same runtime authority
   context; Automation operation identities now use that context's real epoch
   and fence fields;
4. `RuntimeInstanceGraph` separates canonical topology from observed service
   state and readiness; the App Server is marked ready only after an exact-home
   physical readiness probe;
5. provider-effect coordination and the production writer host require a
   `CapabilityUseVerifier` immediately before dispatch/reconciliation, including
   revocation, current epoch, generation, fence and expiry checks;
6. the unchecked provider coordinator is no longer the public product export;
7. Hepta-specific README, security policy, CODEOWNERS and a machine-readable
   repository-ruleset contract are present;
8. `.github/workflows/hepta-gap-closure-p0-6.yml` is a read-only exact-source and
   merge-candidate gate and cannot write the candidate.

Remaining P0.6 work:

1. replace normal in-process local grant construction with a separately scoped,
   Supervisor-issued and signed runtime-grant bootstrap;
2. bind that grant to Agent, release/candidate identity, closed runtime profile,
   authority/owner epochs, generation, fence, signer epoch and validity window;
3. carry it through an owner-only file/descriptor or authenticated local
   channel, never through an untrusted request field;
4. apply per-use verification uniformly to every physical model, provider,
   tool, network and external-effect boundary;
5. add compiler/AST-level constructor and callsite control in addition to source
   ratchets;
6. run real multi-process stale-owner, key-rotation, revocation, expiry and
   clock-regression tests;
7. prove production artifacts do not link qualification grant constructors.

P0.6 exits executable qualification only when its source-head and merge-candidate
jobs both have real runners, non-empty successful steps, exact identities and
independently reviewed receipts. Source checks alone are insufficient.

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

## P0.8 — Runtime budgets and physical vertical slice

Required work:

- publish a fleet-queryable, read-only projection of the generation-fenced
  desired, observed and readiness graphs;
- enforce turn, tool-process, model, memory, disk and queue budgets at physical
  admission points;
- prove permit release on cancellation, terminal events, crash and restart;
- run one exact-candidate real-process vertical slice:

```text
Supervisor signed runtime grant/lifecycle
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

1. the live default-branch ruleset matches
   `docs/governance/HEPTA_REPOSITORY_RULESET_REQUIRED_V1.json`;
2. direct/force push and branch deletion are blocked;
3. an independent CODEOWNER reviews the exact candidate and all conversations
   are resolved;
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
