# Hepta Global Modular Development Plan

**Plan ID:** `HEPTA-GLOBAL-MODULAR-DEVELOPMENT-PLAN`
**Version:** `6.0.0`
**Date:** 2026-08-31
**Status:** canonical development source on its exact candidate; executable qualification, independent review, operator acceptance, promotion, and release remain separate.

This is the only global human-readable development authority in the working tree. Machine registries own bounded facts; `docs/STATUS.md` is generated. The document grants no runtime or production authority.

## 1. Mission and completion model

Hepta shall become a modular, local-first agent system that can be developed by dozens of people or agents without sacrificing hot-path performance, safety, data ownership, or whole-system optimization. Each team owns one bounded module. Two central control planes combine module proposals into globally feasible plans:

- the **Runtime Control Plane** selects resource, placement, routing, and degradation plans;
- the **Engineering Control Plane** selects work assignments, integration order, CI allocation, and merge candidates.

Neither control plane may mint capabilities, execute physical effects, self-accept, promote, or release. “Global optimum” always means the best feasible plan for an exact state snapshot, objective-registry version, hard-constraint set, candidate set, planning horizon, and solver budget. Exact optimality, an optimality bound, or a disclosed heuristic limitation must be recorded.

Completion requires all repository-controlled packages to close on one selected linear stack, all applicable evidence and independent gates to be current and exact-candidate bound, and separate promotion/release decisions. Until then `all_gaps_closed=false`.

## 2. Canonical document system

Read in this order:

1. `docs/CURRENT.json` — time-bounded Git and candidate observations;
2. this document — global requirements and delivery policy;
3. `docs/architecture/ARCHITECTURE.json` — architecture invariants;
4. `docs/modules/MODULES.json` — module, owner, source-root, dependency, and writer registry;
5. `docs/delivery/WORK_PACKAGES.json` plus the development, activation, and evidence DAGs;
6. `docs/control-plane/OBJECTIVES.json` — constraints and objective functions;
7. `docs/evidence/INDEX.json` — qualification vocabulary and observed candidates;
8. exact executable evidence and independently issued decisions.

Stable paths are updated in place. Version numbers live inside documents, not filenames. Historical plans, tranche narratives, status snapshots, gap ledgers, current-plan pointers, Dropbox exports, and in-tree archives are prohibited. Git history and immutable content-addressed evidence preserve provenance. Implementation contracts consumed by code remain valid but cannot claim global-plan authority.

`python3 scripts/hepta-docs.py verify` must pass on every PR and default-branch push. The workflow is read-only, has no path bypass, and rejects regenerated history pollution.

## 3. Current truthful baseline

Observed default branch:

```text
repository = TrillionniumFoundation/hepta-private-ci
repository_id = 1320694176
branch = integration/vnext-main-20260811
head = b621768b70a09d56626bb8a2c331e3dc424e6a4d
tree = f2e82fd525d337efae355adf6f19398812d4180c
```

The current source stack extends through:

```text
P0.7a signed runtime bootstrap        PR #83   92d22e241972fd02f2a3a0bf69849b0b4c7a8b7f
B0 verified-use kernel                PR #272  ad7845a8d67390299f86e931bab11d8b0ec13115
B1a provider boundary                 PR #273  537394a0067d204b215db8bee3de533494535481
B1b model boundary                    PR #280  cd6823c94b3fbd1c3845a398206f526b8e4bc85e
B2 tool/network/filesystem boundary   PR #281  24a2c1b733cc1d0f1288b39ffd42057dc6ade8ba
B3 governed boundaries                PR #282  44d4200a0c4721fd277b4a9063eae4c1e675a86b
```

These are source candidates, not activated product authority. AuthBus PR #279, Intelligence PR #277, Inference PR #87, UI PR #78, and Browser PR #1 remain separate candidate lanes described by `CURRENT.json`. Candidate metadata wins over stale PR prose. Queued, cancelled, runnerless, empty-step, source-only, generated, fixture, or self-issued records are not PASS.

## 4. Architecture invariants

### 4.1 Execution and ownership

- Codex App Server remains the sole session, thread, turn, model-call, and tool-execution spine.
- Agentd is a thin composition/lifecycle host and owns no product-domain durable fact.
- Every durable fact has exactly one schema owner and one authoritative writer.
- A module never directly mutates another owner’s store.
- Cross-owner mutation is: local transaction → durable intent → outbox → destination dedupe/apply → acknowledgement → reconciliation.
- Queue or dispatch acknowledgement is not terminal external-effect success.
- `Indeterminate` remains open until a current-fence reconciler records applied, not applied, or quarantined.

### 4.2 Physical authority

A broad admitted capability is insufficient at an irreversible boundary. After the final payload is known and immediately before crossing, the caller obtains one short-lived, non-cloneable, non-serializable `VerifiedUseToken<C>` bound to capability kind, action, operation, final payload digest, runtime context, revocation revision, verification time, and expiry. The token is consumed by value; the caller persists a digest-only witness before adapter entry. The adapter cannot mint what it consumes. Drift in subject, release, epoch, owner, generation, fence, operation, payload, revision, or time fails closed.

### 4.3 Dependency direction

```text
platform types → wire DTOs → kernel/domain ports → domain modules → checked adapters → composition/UI
```

Forbidden: Codex implementation depending on Hepta implementation; wire crates importing SQL/domain/daemon code; domains importing Agentd/Matrixd/UI binaries; qualification code importing production writers; adapters issuing their own capabilities; central RPC on local hot paths by default.

## 5. Module architecture and team model

`MODULES.json` is the authoritative roster of 30 modules. The major bounded contexts are:

- foundation: `platform.types`, `platform.wire`;
- kernel: authority, operations/outbox, evidence;
- runtime: supervisor, fleet, Agentd, Codex adapter;
- identity/secrets: AuthBus and HeptaBao boundary;
- inference/intelligence: controller, isolated worker, intelligence evaluation;
- cognitive: types, store, read ports, retrieval, federation, Rust KG, compact, trajectory, learning shadow;
- product adapters: automation, Matrix, Browser;
- presentation: Control UI and Native UI;
- orchestration: runtime and engineering control planes.

Each module has a primary owner, deputy, exclusive source roots, declared dependencies, authoritative data domains, forbidden authorities, and current work packages. One person or agent receives a bounded work envelope, not broad repository ownership. Any cross-module change requires explicit co-ownership or a separate integration package.

## 6. Two control planes

### 6.1 Runtime Control Plane

Inputs: exact global state snapshot, health/readiness graph, resource/quota inventory, module Pareto candidates, policies, deadlines, and current fences. It validates hard constraints, solves or approximates the selected objective set, emits a decision receipt and fallback plan, and requests separately governed execution grants. It must remain off the local hot path unless a contract explicitly proves bounded use.

Module proposals contain candidate ID, input snapshot digest, utility vector, resource vector, risk, confidence, compatibility, rollback, and validity window. A single local optimum is never accepted as global truth. The control plane can select a plan but cannot execute it.

### 6.2 Engineering Control Plane

Inputs: work-package DAG, team skills/capacity, source-root conflict graph, review topology, CI capacity, critical path, risk, and expected value. Outputs: bounded assignments, integration order, merge queue proposal, and decision receipt. It cannot self-approve or merge its own work and cannot grant runtime authority.

Optimization targets critical-path reduction and delivered value while minimizing merge conflict, CI pressure, architecture debt, rollback cost, and review imbalance. Replanning is event-driven and bounded; churn is explicitly penalized.

## 7. Three DAGs and work lifecycle

The development DAG permits contract-first parallel source work. The activation DAG is stricter and controls integration/runtime enablement. The evidence DAG prevents source, tests, semantic review, physical facts, operator acceptance, promotion, and release from being collapsed into one boolean.

A package progresses only through evidenced states. Definition of ready requires an exact base, owner/deputy, bounded paths, interface inputs, predecessor classification, tests, resource budget, rollback, and stop conditions. Definition of done requires source inventory, deterministic verifier, format/schema checks, focused and package tests, all-target checks, strict lint, required fault rows, clean tree, exact-head and merge-candidate evidence, and any independent gates.

Valid stop outcomes are `PACKAGE_CLOSED_CANDIDATE`, `BASE_DRIFT`, `BLOCKED_UPSTREAM`, `BLOCKED_EXTERNAL`, `STOP_CONDITION`, and `RESUME_REQUIRED`. “Best effort PASS” is invalid.

## 8. Delivery roadmap

### P0 — document and source identity convergence

- `DOC-0`: install this canonical set and remove the historical Dropbox tree and all competing development guidance.
- `DOC-1`: obtain exact-head and merge-candidate document verification, independent review, and select the candidate into the default branch.
- Restack active implementation PRs on the selected V6 baseline while preserving code, tests, implementation contracts, and immutable evidence but deleting legacy plan/status/tranche files.

### P1 — authority boundaries and current blockers

- Qualify P0.7a, B0, B1a, B1b, B2, and B3 independently; no activation before predecessors close.
- Build B4 compiler/AST caller proof covering every capability constructor, consumer, final-payload builder, adapter, witness store, and reconciler. Text grep is supporting evidence only.
- AuthBus P1.3 V12 must execute window-keyed RPM/TPM/day accounting, per-request context, reservation conservation, state transitions, digest-chain tamper tests, and independent semantic review before product/OpenBao wiring.
- Inference V4 completes CSPRNG identities/grants, deterministic admission, daemon ownership, real cancel/kill facts, bounded provider host, shadow bridge, kill switch, and rollback.
- Intelligence closes exact admission/config graph and selection evidence without gaining model, Memory/KG, effect, or production authority.

### P2 — Memory, KG, dependency, readiness, and control planes

Extract in order: cognitive types; store/migration kernel; retrieval; federation; Rust KG projection; compact/checkpoint engine; trajectory store; learning shadow; minimal production writer. Retrieval/federation are read-only. KG is derived and rebuildable; source facts remain store-owned. Learning shadow cannot construct the production writer. Old/new readers compare one SQLite snapshot before cutover; unbounded dual-write is forbidden.

Then close the common durable fault matrix, dependency inversion, AST ratchet, durable runtime-instance/readiness graph, executable resource budgets, Engineering Control Plane, and Runtime Control Plane. UI/Browser proceed in parallel only behind capability, device, human, and exact-source gates.

### P3/P4 — vertical slice and external gates

Run one real-process slice: supervisor starts one generation; Agentd claims signed bootstrap; Codex becomes ready; one durable local command is admitted; final payload is built; verified-use is consumed; destination dedupes/applies; acknowledgement settles source; crash/reopen reconstructs the same terminal state. Cover success, duplicate, ACK loss, stale generation, kill, and database reopen without requiring a production provider.

Repository rulesets, independent review, physical devices, real providers/models/corpus, human accessibility/visual acceptance, operator acceptance, trust-root ceremony, promotion, and release remain external gates.

## 9. Concurrency and performance contract

Every affected hot path records baseline and candidate throughput, p50/p95/p99 latency, CPU, resident memory, allocations, queue depth/age, SQLite busy/WAL metrics, file descriptors/sockets, payload sizes, and provider cost. Tests use declared hardware/toolchain/workload and confidence bounds. A performance claim without a comparable baseline is not evidence.

Backpressure is enforced at admission and execution. Bounded queues define capacity, age, fairness, cancellation, retry, reconcile, and dead-letter policies. Overload rejects or degrades explicitly; it never grows without bound. Control-plane loss uses a bounded cached plan only while identity, policy, fence, and expiry remain current; otherwise fail closed or use a declared local fallback.

## 10. Durable fault matrix

All stateful modules and outboxes cover: before intent; after intent before commit; after commit before return; before outbox; after outbox before commit; after outbox commit before wakeup; before claim; after claim before send; after send before ACK; destination commit before ACK; ACK before source settlement; stale generation callback; permission loss; filesystem full; corruption; nonempty WAL/identity drift; backup/restore; process kill/reopen. Real-process rows require real processes/files and exact binary/schema/toolchain identity; unit mocks do not close them.

## 11. Security, privacy, deletion, and unlearning

Each module classifies secrets/PII, bounds/redacts receipts, specifies key identity/rotation/revocation/expiry, retention/export/deletion, corruption quarantine, and audit separation. Raw prompts, credentials, secret values, private keys, unrestricted source content, and raw model payloads never enter general receipts. A forget/correct request closes only after Memory, KG/indexes, compact checkpoints, federation caches, trajectories, training caches, and signed artifacts prove deletion/rebuild or are revoked and excluded from loading.

HeptaBao remains a distinct secrets authority. AuthBus references opaque `SecretRef` values and expected revisions; it does not store raw secret bytes. Secret operations require final-purpose/audience/provider/profile/token-family/deadline binding and a current verified-use token.

## 12. Compatibility, migration, rollback, and upstream

All public APIs and wire DTOs are versioned, deny unknown critical fields, and have compatibility tests. Schema migration has one owner, monotonic lineage, backup/restore, old/new reader comparison, and a no-dual-writer cutover. Rollback restores exact source/release identity and fencing state; it never interprets uncertain effects as absent.

Codex stays upstream-clean: Hepta implements generic ports outside the execution spine. Every upstream intake records old/new exact revisions, semantic delta, compatibility impact, test selection, and rollback. Rebase evidence is required before claiming dependency inversion complete.

## 13. Testing, evidence, and review topology

Required layers are static contracts; unit/property/model tests; integration tests; real-process crash/fault tests; performance tests; physical/provider/human tests where applicable; and security/privacy/deletion campaigns. CI is read-only and diagnostic-complete: failures remain attributable, later diagnostics still run when safe, and one final gate requires every mandatory result.

Executable evidence binds commit/tree, merge candidate where applicable, workflow path/blob, run attempt, job/runner identity, non-empty steps, terminal conclusion, artifact digest/expiry, toolchain/platform, package/test inventory, and negative authority posture. Dynamic evidence expires.

Code owner and independent reviewer are distinct for trust boundaries. Security reviews authority/secrets; durability reviews storage/outbox; performance reviews budgets/benchmarks; accessibility reviews UI evidence; release authority alone promotes/releases. Authors, source publishers, workflows, and central optimizers cannot self-issue independent decisions.

## 14. Immediate queue

1. Land and qualify `DOC-0`; complete `DOC-1` selection into the default branch.
2. Restack #83 → #272 → #273 → #280 → #281 → #282 on that baseline and rerun exact-head plus merge-candidate gates.
3. Execute independent semantic review for AuthBus #279.
4. Reconcile Browser #1’s metadata/body identity conflict before accepting any evidence.
5. Close B4 caller proof, then begin contract-first Memory/KG extraction.
6. Close fault, dependency, readiness, and resource packages before activating either central optimizer.
7. Run the real-process vertical slice; then obtain external gates in their designated lanes.

All authority flags remain false in this document set.
