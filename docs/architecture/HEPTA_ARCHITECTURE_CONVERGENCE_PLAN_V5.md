# Hepta Architecture Convergence Plan V5

**Plan ID:** `HEPTA-ARCHITECTURE-CONVERGENCE-V5`  
**Version:** `5.0.0`  
**Date:** 2026-08-30  
**Status:** selected source candidate on the architecture-convergence stack; executable qualification and independent acceptance remain separate.  
**Repository:** `ProfHepta/hepta-private-ci`  
**Immediate lineage:** `f69e5a4a5068a2657f1470da43c26b1410d53c6f` / tree `532307507d2b02a479d3c76042d42cc948b499df`  
**Predecessor:** `HEPTA_ARCHITECTURE_CONVERGENCE_PLAN_V4.md`

V5 retains the V4 authority, ownership, recovery, evidence and release boundaries, but converts the remaining chapter-level backlog into one executable package graph. It also freezes the missing common primitive required by every irreversible boundary: a final-payload, operation-bound, revocation-revision-bound, short-lived and non-serializable verified-use token.

This document is a source contract. Its presence does not establish that a runner executed the candidate, that an independent reviewer accepted it, that an operator accepted it, or that any runtime, production, effect, promotion or release authority exists.

## 1. Current truthful state

The architecture stack has implemented the source portion of the Supervisor-signed runtime bootstrap. The exact P0.7a candidate exposed a deterministic Rust 1.95 formatting failure on an assigned ARM runner. The minimal successor fixes only that emitted formatter delta. Until both exact-head and merge-candidate jobs execute non-empty successful steps, P0.7a remains `source_implemented_executable_qualification_pending`.

V5 starts B0 on top of that candidate because all later physical-boundary work consumes the same verified-use kernel. B0 source may be reviewed and qualified in a stacked Draft, but B1 or later activation may not treat B0 as available until P0.7a and B0 each reach `qualified_exact` on their own exact candidates.

Current claim boundary:

```text
runtime_authority=false
production_caller=false
production_writer=false
model_invocation=false
provider_dispatch=false
tool_execution=false
network_connect=false
external_filesystem_mutation=false
secret_operation=false
matrix_send=false
fleet_mutation=false
operator_acceptance=false
promotion=false
release=false
all_gaps_closed=false
```

## 2. Evidence vocabulary

Every package uses exactly these states:

1. `open` — scope and exit criteria exist; no source is claimed.
2. `source_in_progress` — source is changing; no source PASS is claimed.
3. `source_implemented` — all allowlisted source paths exist and deterministic source verifiers pass.
4. `source_verified` — format/schema/static checks pass on the exact source candidate.
5. `qualified_exact` — an assigned runner executed non-empty required steps on the exact candidate and every package gate passed.
6. `merge_candidate_qualified` — the GitHub merge candidate for that exact head executed the required repository-controlled gates successfully.
7. `blocked_external` — repository-controlled work is closed, but an independently issued physical, human, administrative or operator fact is absent.
8. `closed` — all source, executable, review, operator and release conditions applicable to that package are independently satisfied.

The following are never PASS evidence:

- queued or pending workflow state;
- `runner_id=0`;
- `steps=[]` or missing step records;
- source-only verifier output;
- a PR description, comment or label;
- a synthetic merge ref described as an exact source head;
- a receipt emitted by the component whose authority is under review;
- a qualification fixture standing in for a real model, provider, physical device, human reviewer or operator.

## 3. Non-negotiable architecture invariants

### 3.1 Execution and ownership

1. Codex App Server remains the sole session, thread, turn, model-call and tool-execution spine.
2. Agentd is a thin composition and lifecycle host; it owns no product-domain durable facts.
3. Each durable fact has exactly one schema owner and one authoritative writer.
4. A component must not directly mutate another owner’s store.
5. Cross-owner mutation uses local transaction → durable intent → outbox → destination dedupe/apply → acknowledgement → reconciliation.
6. A queue or dispatch acknowledgement is never terminal external-effect success.
7. An `Indeterminate` result remains open until a current-fence reconciler commits `ReconciledApplied`, `ReconciledNotApplied` or `Quarantined`.

### 3.2 Authority

1. A runtime bootstrap proves only startup identity and a bounded launch context; it is not a reusable physical capability.
2. Broad `Authorized<C>` values identify an admitted capability class. They are insufficient by themselves to cross an irreversible boundary.
3. Immediately after the final operation payload is known and immediately before the crossing, the caller must obtain a one-operation `VerifiedUseToken<C>`.
4. The token binds capability kind, action, operation ID, final payload SHA-256, runtime-authority context digest, revocation revision, verification time and expiry.
5. The token is non-serializable, non-cloneable and consumed by value at the boundary.
6. Subject, release/grant, authority epoch, owner epoch, generation, fence, expiry, revocation revision, operation or payload drift fails closed.
7. No adapter may mint the capability or verified-use token that it consumes.
8. Qualification code, fixture signers and test verifiers cannot be linked into a production artifact.

### 3.3 Product and release

1. Feature presence is not runtime registration.
2. Runtime registration is not production authority.
3. Executable qualification is not operator acceptance.
4. Operator acceptance is not promotion.
5. Promotion is not release.
6. Source candidates never write their own independent review, operator, promotion or release receipt.
7. A production artifact must bind exact source commit/tree, complete feature set, binary digests, SBOM, migration compatibility, rollback evidence and required external receipts.

## 4. Target architecture

```text
hepta-cli / native UI / control UI
                |
                v
      agentd composition root
 identity | profile | health | routing
        /       |        |        \
       v        v        v         v
 Codex port  Automation  Cognitive  Channel ingress
 adapter      domain       read       adapters
   |           store       ports       stores
   v
upstream-clean Codex App Server

irreversible intent
        |
        v
external authority verifier
        |
  VerifiedUseToken<C>
        |
        v
boundary adapter / dedicated writer or dispatcher
        |
        v
receipt + lookup-only reconciliation

supervisord: process lifecycle, immutable release selection,
generation/owner fencing and runtime-instance projection only
```

Required dependency direction:

```text
hepta-types
    ↑
hepta-wire
    ↑
hepta-domain-*
    ↑
hepta-adapter-*
    ↑
hepta-agentd / hepta-matrixd / hepta-cli
```

Forbidden directions include:

```text
codex-*                    -X-> hepta-* implementation
hepta-wire                 -X-> sqlx / codex-state / daemon crates
hepta-domain-*             -X-> hepta-agentd / hepta-matrixd binaries
hepta-matrix-*             -X-> hepta-agentd implementation
shadow/qualification code  -X-> production writer
boundary adapter           -X-> its own capability issuer
```

## 5. Ordered delivery graph

| Order | Package | Purpose | Hard predecessor | Initial V5 state |
|---:|---|---|---|---|
| 1 | P0.7a | Supervisor-signed runtime bootstrap | V4 source stack | source implemented; executable pending |
| 2 | P0.7b/B0 | common verified-use kernel | P0.7a source; activation waits for P0.7a qualification | source in progress |
| 3 | P0.7b/B1 | model/provider physical boundaries | B0 qualified | open |
| 4 | P0.7b/B2 | tool/network/filesystem physical boundaries | B0 qualified | open |
| 5 | P0.7b/B3 | secret/Matrix/fleet/operator/release boundaries | B0 qualified | open |
| 6 | P0.7b/B4 | complete negative call-site proof | B1–B3 qualified | open |
| 7 | P0.7c | physical Memory bounded-context extraction | B4 qualified | open |
| 8 | P0.7d | common durable fault matrix | P0.7c source implemented | open |
| 9 | P0.7e | dependency inversion and wire isolation | B4 + P0.7c interfaces | open |
| 10 | P0.8a | compiler/AST authority caller ratchet | P0.7e | open |
| 11 | P0.8b | durable runtime-instance/readiness projection | P0.8a | open |
| 12 | P0.8c | executable resource-budget enforcement | P0.8b | open |
| 13 | P0.8d | real-process product vertical slice | P0.7d + P0.8c | open |
| 14 | P0.9 | repository, physical, human, operator, promotion and release gates | P0.8d | open/external |

A package may prepare source while its predecessor’s executable run is pending only when:

- it is a separate stacked Draft;
- it grants no new runtime or effect authority;
- its ledger records the predecessor as blocking activation;
- it does not alter the predecessor’s receipt;
- it can be discarded without changing the predecessor candidate.

## 6. P0.7a — signed runtime bootstrap closure

### Definition of ready

- one exact V4/V5 predecessor chain;
- committed Cargo and Bazel lock coherence;
- no-follow, owner, mode, link-count, inode and digest handoff rules present;
- durable reservation/publication/claim crash-window source present;
- assigned runner available.

### Definition of done

- exact source head and exact merge candidate each execute non-empty format, locked metadata, tests, all-target check, strict Clippy, architecture verifier and clean-worktree steps;
- no failure is hidden by a skipped later step;
- reservation, publication, claim and stale-generation fault tests pass;
- runtime and all production/effect/operator/promotion/release flags remain false;
- independent review is still required before stack integration.

### Rollback

Revert only the P0.7a stack. Do not reinterpret a pending reservation or partial claim as a ready runtime grant. Preserve failed evidence for diagnosis.

## 7. P0.7b — verified physical capability closure

### 7.1 B0 common verified-use kernel

**Owned source:**

```text
codex-rs/hepta-contracts/src/verified_use.rs
codex-rs/hepta-contracts/src/verified_use_tests.rs
codex-rs/hepta-contracts/src/lib.rs
docs/architecture/HEPTA_P0_7B_VERIFIED_USE_DELIVERY_CONTRACT_V1.md
docs/architecture/HEPTA_ARCHITECTURE_GAP_LEDGER_V5.json
scripts/verify-hepta-architecture-plan-v5.py
.github/workflows/hepta-architecture-v5-b0-verified-use.yml
```

**Required API facts:**

- closed `PhysicalCapabilityKind` enumeration;
- exact mapping from physical kind to `AuthorityAction`;
- non-zero typed revocation revision;
- bounded verification window;
- `PhysicalUseVerificationRequest` carrying operation ID, final payload digest, runtime context and expected revision;
- verifier response carrying current revision, verifier receipt digest and validity bound;
- `verify_physical_capability_use` composes the existing per-use verifier and returns a private-constructor token;
- `VerifiedUseToken<C>` has no `Clone`, `Copy`, `Serialize` or `Deserialize` implementation;
- boundary entry consumes the token by value and emits a digest-bound witness;
- final operation, payload, context, revision, kind and time are rechecked during consumption.

**Required negative matrix:**

1. action/kind mismatch;
2. local-only broad capability presented to an irreversible boundary;
3. subject or generation mismatch;
4. authority/owner epoch or fence drift;
5. expired lease;
6. request window outside the lease;
7. revocation revision changed during verification;
8. verifier validity already expired;
9. operation ID drift at consumption;
10. payload digest drift at consumption;
11. runtime context drift at consumption;
12. revocation revision drift at consumption;
13. consumption before verification time;
14. consumption at or after expiry;
15. unknown/unsupported capability kind cannot be decoded or selected.

**B0 done:** source verifier, Rust 1.95 formatting, focused tests, full `codex-hepta-contracts` tests, all-target check, strict Clippy and clean-worktree checks pass on exact head and merge candidate. No product call site changes in B0.

### 7.2 B1 model/provider boundary

Every model submission and provider dispatch must construct the final payload first, append the local intent where required, obtain the exact B0 token, consume it at the physical adapter, and persist the returned witness.

For an effectful provider call, `ProviderDispatchCapability` and `ExternalEffectCapability` remain distinct. One capability cannot be cast into the other. Timeout or lost acknowledgement enters `Indeterminate`; retry is lookup-only unless the provider proves no effect or the operation is strongly idempotent under the same key.

B1 cannot close until a static call-site inventory proves no model/provider path bypasses the checked adapter.

### 7.3 B2 tool/network/filesystem boundary

Split one generic “external effect” concept into explicit boundary intents:

- tool process spawn;
- outbound network connect;
- filesystem mutation outside the Agent root.

Each final payload digest includes the boundary-specific identity:

- tool: executable digest, argv, cwd identity, environment policy, sandbox and approval;
- network: protocol, canonical destination, DNS/IP binding, proxy policy and request digest;
- filesystem: canonical target, device/mount identity, no-follow policy, mutation class and expected prior state.

The caller may not obtain a token until all final boundary-specific facts are known.

### 7.4 B3 secret, Matrix, fleet, operator and release boundaries

- Secret operations bind opaque SecretRef, provider/profile/token family, purpose, audience, expected revision and operation deadline. Raw secret bytes never enter a receipt.
- Matrix send binds room, event ID, payload digest, durable dispatch operation and current Matrix identity generation.
- Fleet mutation binds registry revision, release ID, owner epoch, process generation and immutable release identity.
- Operator acceptance binds exact candidate and complete evidence manifest; repository administration cannot synthesize it.
- Release promotion binds candidate acceptance, release manifest, SBOM, migrations, rollback evidence and policy revision.

### 7.5 B4 negative call-site proof

B4 creates a compiler/AST-assisted inventory of every physical adapter and every constructor/consumer of:

- `Authorized<ModelInvocationCapability>`;
- `Authorized<ProviderDispatchCapability>`;
- `Authorized<ExternalEffectCapability>`;
- `Authorized<FleetMutationCapability>`;
- `Authorized<OperatorAcceptanceCapability>`;
- `Authorized<ReleasePromotionCapability>`;
- `VerifiedUseToken<...>`.

The inventory is an allowlist with file, symbol, capability kind, issuer, consumer, final-payload builder and reconciliation owner. A new or moved call site fails CI until reviewed. Text grep is supporting evidence only, not the final ratchet.

## 8. P0.7c — Memory bounded-context extraction

Physical extraction order:

1. `hepta-cognitive-types` — value objects and stable contracts only;
2. `hepta-cognitive-store` — schema/migration owner and transaction kernel;
3. `hepta-memory-retrieval` — read-only lexical/vector/KG candidate planning;
4. `hepta-memory-federation` — capability-bound federated reads;
5. `hepta-compact-engine` — checkpoints, loss report and rehydration;
6. `hepta-trajectory-store` — append-only learning/feedback trajectories;
7. `hepta-learning-shadow` — proposal/evaluation only;
8. `hepta-production-writer` — minimal write authority and outbox boundary;
9. optional thin compatibility facade, with an explicit retirement date.

Rules:

- no table has two schema owners;
- migration lineage remains monotonic and independently verifiable;
- retrieval and federation have no write authority;
- shadow learning cannot depend on or construct the production writer;
- production writer does not link fixture models, fixture signers or qualification-only code;
- every moved public type has a compatibility or migration decision;
- dual writes are prohibited unless a bounded migration protocol explicitly owns them;
- old and new readers are compared from one SQLite snapshot before cutover.

P0.7c is done only after crate graph, migration, API compatibility, backup/restore and fault evidence are exact-candidate qualified.

## 9. P0.7d — common durable fault matrix

The same 18 fault rows apply to Memory, Automation, Matrix, Evidence and any production outbox:

| ID | Fault boundary | Required invariant |
|---|---|---|
| F01 | before intent insert | no durable mutation |
| F02 | after intent insert before commit | transaction rollback |
| F03 | after local commit before return | exact replay adopts existing state |
| F04 | before outbox insert | state and outbox remain atomic |
| F05 | after outbox insert before commit | no partial publication |
| F06 | after outbox commit before wakeup | restart discovers pending row |
| F07 | before delivery claim | retry remains safe |
| F08 | after delivery claim before send | lookup-only if crossing is uncertain |
| F09 | after send before provider acknowledgement | `Indeterminate`, no blind retry |
| F10 | after destination commit before acknowledgement | destination dedupe/adoption |
| F11 | after acknowledgement before source settlement | source adopts exact acknowledgement |
| F12 | stale generation/owner callback | rejected without mutation |
| F13 | permission loss | fail closed; no fallback writer |
| F14 | filesystem full | no success receipt; recoverable transaction state |
| F15 | corrupt row/digest | fail closed and quarantine |
| F16 | nonempty WAL or identity drift on immutable read | reject readiness |
| F17 | backup/restore | complete owner/domain consistency |
| F18 | process kill/reopen | deterministic rebuild and bounded recovery |

Each package must bind executable fault evidence to exact source, toolchain, platform, database schema and test binary digest. A unit-test-only mock does not close a real-process row.

## 10. P0.7e — dependency inversion and wire isolation

Required moves:

- Codex crates expose generic ports and contain no Hepta implementation dependency;
- Hepta adapters implement those ports outside the Codex execution spine;
- `hepta-wire` contains strict versioned DTOs only and has no SQL/domain/daemon dependency;
- domain models map to wire DTOs through explicit adapters;
- `matrixd` depends on a narrow Agent ingress port, not the complete Agentd implementation;
- supervisor is reduced to lifecycle, release selection, fencing and readiness projection;
- signer, authority verification, Robrix projection and production writer are separate trust/domain components.

P0.7e done requires a generated dependency graph with a denylist, package API compatibility tests and an upstream Codex rebase exercise.

## 11. P0.8a — compiler/AST authority ratchet

A repository tool parses Rust items, imports, generic types, constructors and call expressions. It emits a canonical caller manifest and fails on:

- unregistered physical capability consumer;
- direct call to a raw provider/model/tool/network/filesystem/Matrix/fleet adapter;
- token constructor outside the authority kernel;
- token consumption outside an allowed physical boundary;
- production code importing a qualification-only module;
- a Codex crate importing a Hepta implementation crate;
- a wire crate importing domain/storage/runtime code.

The manifest is reviewed like code. Runtime receipts and static call-site proof are both required; neither substitutes for the other.

## 12. P0.8b — runtime-instance and readiness graph

Supervisor owns a durable projection per component instance:

```text
component_id
agent_id
release_id
binary_sha256
profile_sha256
generation
owner_epoch
process_identity
start_reservation
bootstrap_claim
readiness_state
last_observation
fault_state
```

Readiness is a graph, not a single boolean. Required services must be ready on the same release/profile/generation; optional services may degrade only according to the selected runtime profile; disabled services must be absent. A stale process, wrong binary, missing claim, old generation or contradictory readiness row fails closed.

## 13. P0.8c — executable resource budgets

Budgets are enforced at admission and during execution:

- process count and restart rate;
- resident memory and mapped model bytes;
- CPU time and concurrency;
- queue depth and oldest-item age;
- SQLite WAL size and transaction latency;
- open file descriptors and sockets;
- payload/frame size;
- retry, reconcile and dead-letter limits;
- retrieval candidates/results/context tokens;
- per-Agent and fleet-wide model/provider quotas.

Each rejection emits a bounded, secret-safe receipt. Monitoring without admission enforcement is not closure.

## 14. P0.8d — real-process vertical slice

The first complete vertical slice is deliberately narrow:

```text
supervisord starts one Agent generation
→ Agentd verifies and atomically claims signed bootstrap
→ Codex App Server becomes ready
→ one durable Agent-local automation command is admitted
→ one final payload is built
→ one verified-use token is issued and consumed
→ one destination dedupe/apply occurs
→ acknowledgement settles source state
→ evidence can be read
→ forced crash/reopen reconstructs the same terminal state
```

The slice must run with real processes and real SQLite files. It must cover success, duplicate delivery, acknowledgement loss, stale generation, process kill and database reopen. It must not require a production provider, real external effect or operator acceptance.

## 15. P0.9 — external and administrative gates

These facts cannot be manufactured by repository source:

- live GitHub branch/ruleset enforcement and required-check configuration;
- independent exact-candidate review;
- physical platform/device evidence where required;
- real provider/model capability evidence;
- real reviewed corpus and efficacy evidence;
- human accessibility/visual acceptance;
- candidate-bound operator acceptance;
- production trust-root and secret ceremony;
- promotion and release decision.

A missing external fact produces `blocked_external`, not a false source failure and not `closed`.

## 16. Security, privacy and unlearning

All packages must define:

- secret and PII data-flow classification;
- receipt redaction and maximum byte sizes;
- key identity, rotation, revocation and expiry;
- retention, export and deletion behavior;
- exact forget/correct propagation across Memory, KG/index, compact checkpoints, federation caches, trajectories, training caches and signed artifacts;
- corruption quarantine and forensic preservation;
- audit access and operator separation of duties.

A forget request is not complete until every derived artifact either proves deletion/rebuild or is revoked and excluded from loading.

## 17. Observability and SLO contract

At minimum expose bounded metrics for:

- runtime bootstrap reservations/claims/rejections;
- verified-use issue, rejection, expiry and revision drift;
- queue depth, age and backpressure;
- outbox pending, claimed, indeterminate and dead-letter rows;
- reconciliation latency and terminal distribution;
- SQLite busy time, WAL size and reopen recovery;
- process restarts and stale-generation rejection;
- retrieval latency, candidate counts and attached token budget;
- model/provider invocation latency and unknown outcomes;
- resource-budget rejection.

Logs and metrics must use stable IDs and digests, not raw prompts, credentials, secret values or unrestricted source content.

## 18. Completion algorithm

For each package, repeat:

1. revalidate exact repository, branch, commit, tree and clean candidate;
2. confirm the current Plan pointer and package predecessor;
3. inventory existing source and active competing branches/PRs;
4. select one unique successor and freeze changed-path allowlist;
5. implement the smallest coherent source closure;
6. run deterministic source/schema/dependency checks;
7. execute package format, tests, check, Clippy and fault rows on an assigned runner;
8. inspect logs for the first real failure and patch only the owning package;
9. re-run exact-head and merge-candidate gates;
10. bind executable receipts without mutating runtime authority;
11. obtain independent review where required;
12. advance the ledger only to the highest evidenced state;
13. proceed to the next package only when its hard predecessor permits it.

Valid stop outcomes are:

- `PACKAGE_CLOSED_CANDIDATE`;
- `BASE_DRIFT`;
- `BLOCKED_UPSTREAM`;
- `BLOCKED_EXTERNAL`;
- `STOP_CONDITION` for a safety or authority violation;
- `RESUME_REQUIRED` when an exact executable run has not completed.

“All gaps closed” is valid only when the V5 ledger has no repository-controlled open package, every required executable and merge-candidate gate is green on the selected linear stack, every applicable external slot has a valid independent receipt, and promotion/release authority has been issued separately. Until then `all_gaps_closed=false` is mandatory.
