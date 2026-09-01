# Hepta Global Modular Development Plan

**Plan ID:** `HEPTA-GLOBAL-MODULAR-DEVELOPMENT-PLAN`
**Version:** `8.0.0`
**Date:** 2026-09-01
**Status:** canonical V8 content is mirrored on `main`; GitHub still names `integration/vnext-main-20260811` as the default branch, so default-branch selection remains an external repository-administration gate.

This is the only global human-readable development authority in the working tree. Machine registries own bounded facts and `docs/STATUS.md` is generated. This document grants no runtime, model, provider, tool, network, filesystem, secret, Matrix, fleet, operator, promotion, or release authority.

## 1. Mission and truthful completion model

Hepta shall become a modular, local-first agent system that can be developed by dozens of people or agents without sacrificing hot-path performance, safety, data ownership, recoverability, or whole-system optimization. The V8 architecture adds a governed longitudinal-intelligence layer while preserving the V6 execution and authority foundations.

The target system combines:

```text
immutable authority, truth, privacy and durability kernel
+ request-bound Objective Compiler
+ hierarchical NDU preference–utility spine
+ local Neuron and calibrated Intuition substrate
+ Prompt Intervention factor pricing, portfolio and timing
+ Hölder-regular Bellman/operator learning
+ causal longitudinal learning ledger
+ next-snapshot artifact and plasticity governance
```

“Global optimum” means only the best feasible plan for an exact state snapshot, objective revision, preference-state revision, hard-constraint set, candidate set, horizon, solver and time budget. Exact optimality, a bound, or a disclosed heuristic limitation must be recorded. Hepta must never claim universal or permanent global optimality.

Completion requires one selected linear source stack, current exact-candidate evidence for every applicable package, independent decisions in their designated lanes, and separate promotion/release. Until then `all_gaps_closed=false`, `closedLoopLearning=false`, `longitudinalEfficacy=false`, `functionalBiomimicry=false`, and `selfIteration=false`.

## 2. Canonical document system and historical cleanup

Read in this order:

1. `docs/CURRENT.json` — time-bounded repository and candidate observations;
2. this document — global requirements and delivery policy;
3. `docs/architecture/ARCHITECTURE.json` — non-negotiable architecture invariants;
4. `docs/modules/MODULES.json`, `docs/contracts/*` and `docs/data/DATA_AUTHORITY.json`;
5. `docs/delivery/WORK_PACKAGES.json`, `PATH_OWNERSHIP.json` and the three DAGs;
6. `docs/control-plane/OBJECTIVES.json`, `NDU.json` and `OPTIMIZATION.json`;
7. `docs/intelligence/PROMPT_INTERVENTIONS.json` and `docs/learning/*`;
8. `docs/evidence/CLAIMS.json`, `QUALIFICATION.json`, `INDEX.json` and exact external receipts;
9. `docs/security/THREAT_MODEL.json` and independently issued decisions.

Stable paths are updated in place. Version numbers live inside documents, never in filenames. Historical plans, status snapshots, current-plan pointers, gap ledgers, tranche narratives, Dropbox exports and in-tree archives are prohibited. Git history and immutable content-addressed external evidence preserve provenance.

The pre-selection baseline contained 143 historical development paths. V8 deleted that complete set in the same commit that installed the canonical document system. Git ancestry now preserves every pre-consolidation branch tip without overlaying an obsolete branch tree or reintroducing an in-tree historical plan. Code-consumed APIs, schemas, policies, migrations, tests and implementation contracts remain protected.

`python3 scripts/hepta-docs.py verify` runs on every pull request and default-branch push with read-only permissions and no path bypass.

## 3. Current truthful baseline

The selected V8 content is present on both `main` and `integration/vnext-main-20260811` at the same exact tip. Every one of the 789 pre-consolidation branch tips is reachable from that line as Git ancestry; obsolete branch trees were not overlaid. All 64 outstanding pull requests were closed as superseded, and all 787 other branch refs were removed by a bounded operation after exact ancestry verification.

GitHub still reports `integration/vnext-main-20260811` as the repository default because the Actions integration is not permitted to mutate repository administration settings. Therefore `DOC-2-DEFAULT-BRANCH-SELECTION` remains `blocked_external`. The final administrative transition is to select the existing `main` ref as default and then delete the old default ref. Until that observable state exists, canonical registries must not claim that `main` is already the GitHub default branch.

Exact live head, tree, CI, review and operator facts remain dynamic evidence and must be resolved from current external receipts. This consolidation grants no runtime, model, provider, tool, network, filesystem, secret, Matrix, fleet, operator, promotion or release authority.

## 4. Immutable execution, authority and data invariants

- Codex App Server remains the sole session, thread, turn, model-call and tool-execution spine.
- Agentd is a thin composition/lifecycle host and owns no product-domain durable fact.
- Every durable fact has exactly one schema owner and one authoritative writer.
- A module never directly mutates another owner’s store.
- Cross-owner mutation is local transaction → durable intent → outbox → destination dedupe/apply → acknowledgement → reconciliation.
- Queue or dispatch acknowledgement is not terminal external-effect success.
- `Indeterminate` remains open until a current-fence reconciler records applied, not applied or quarantined.
- Every irreversible boundary consumes a short-lived, final-payload-bound, operation-bound and revocation-bound `VerifiedUseToken<C>` immediately before adapter entry.
- The adapter cannot mint the token it consumes.
- Hard objectives, authority, privacy, truth, deletion and writer ownership never become learnable parameters.
- Current-run objectives and learning artifacts are immutable. Learning generates only next-snapshot candidates.
- Central optimizers, NDU runtimes, prompt optimizers and learning modules select or propose; they do not execute effects or issue capabilities.

## 5. Forty-module architecture and team model

`MODULES.json` is authoritative for 40 modules. The V6 foundation remains and the Intelligence responsibilities are decomposed into bounded teams:

```text
objective.compiler
utility.ndu
neuron.runtime
intuition.policy
prompt.registry
prompt.optimizer
context.compiler
learning.ledger
learning.operator
learning.eval
learning.artifacts
learning.plasticity
intelligence.control  # composition façade only
```

`trajectory.store` is replaced by the more complete append-only `learning.ledger`. `learning.shadow` becomes the narrower `learning.eval`. The former broad `intelligence.control` no longer owns learning facts, prompt facts, artifacts or model execution.

Each module has a primary owner, deputy, exclusive roots, dependencies, data authority, forbidden authority and bounded work packages. One developer or agent receives a work envelope, not broad repository ownership. Cross-module changes require explicit co-ownership or a separate integration package.

## 5A. Closed-world module implementation guides

Every one of the forty registered modules has one stable implementation guide at `docs/modules/<module-id>/TECHNICAL.md`. `docs/modules/MODULE_DOCS.json` binds each guide to its exact digest, contracts, protocols, data domains, threats and work packages. `docs/modules/SOURCE_BINDINGS.json` separates declared target roots from existing implementation evidence, missing roots and the bootstrap package that must materialize each target.

Source states are deliberately truthful: a module may be `existing_bound`, `existing_legacy_aggregate`, `existing_declared_unbound`, `target_partially_materialized`, `target_unmaterialized` or `external_with_adapter_target`. Documentation readiness never changes a source, activation, acceptance, promotion or release claim. `python3 scripts/hepta-module-docs.py verify` fails unless all forty guides, bindings and registry references are closed.

## 6. Objective compilation

Each request is compiled into `ObjectiveFunctionV1` before adaptive logic runs. The receipt binds:

```text
request and principal scope
success predicates and terminal conditions
hard constraints and evidence requirements
allowed and forbidden action classes
soft utility dimensions
resource endowment and deadline
risk class and rollback requirements
objective revision and digest
```

The immutable core cannot change during a run. A change in user goal, success predicate, authority, privacy, allowed effects or acceptance criteria creates a new objective revision and a new run snapshot.

Soft preference dimensions may adapt only within registered bounds. A system may change how it allocates time, tokens, evidence effort, exploration or abstention while pursuing the goal; it may not replace the goal with an easier one.

## 7. Correct use of Neural Differential Utility

NDU is not a universal name for any forward/backward software module. Its forward component is the resource-constrained subject’s endogenous preference state; its backward component is recursive utility or a utility aggregator. Neural parameterization expresses preference dynamics or utility aggregation.

Only four subject levels are permitted:

```text
System NDU
Domain NDU
Agent NDU
Episode NDU
```

A database row, TaskFlow node, adapter, authority kernel or individual Hepta neuron is not an NDU subject.

The engineering discretization is:

```text
preference state P(k)
+ observation, resource consumption and outcome
→ next preference-state candidate P(k+1)

instant utility + preference state + continuation utility
→ recursive utility U(k)
```

Parent and child NDU subjects communicate bounded boundary conditions, resource/risk budgets, shadow prices, continuation utility, uncertainty and residuals. They never exchange credentials, capabilities, unrestricted hidden states, unrestricted prompts or unbounded gradients.

System and domain NDU snapshots are slow, local-cacheable control inputs; they must not create synchronous central RPC on local hot paths. Multi-level updates are damped and staged so parent and child policies do not chase one another without a frozen reference.

## 8. Neuron and Intuition

A Hepta Neuron is the local temporal/adaptive substrate that implements or estimates preference-state dynamics and fast signals; it is not itself an NDU. The target mechanism contains:

```text
shared frozen local encoder/backbone
small task head or adapter
bounded recurrent state
sparse activation and lateral inhibition
adaptive threshold and homeostasis
bounded eligibility trace
prediction error and OOD/abstention
model, device and resource receipts
```

A local model adapter, Ollama/LM Studio endpoint or checked model ID does not prove H5/Neuron use. A valid N1 claim requires exact weights, tokenizer, preprocessor, quantization, license/SBOM, runtime, device, resource and real consumer evidence.

Hepta Intuition is a calibrated fast policy. It consumes the frozen objective, NDU preference/utility state, Neuron signals, protected Memory/KG evidence, Prompt portfolio, complete legal action set and risk/resource state. It emits an action distribution, propensity, selected action, value/confidence, OOD, abstain/ask or slow-path request. It cannot write Memory/KG, call tools/providers directly, override a hard veto or treat model prose as authority.

Low-risk, read-only, reversible and supported decisions may use the fast path. High risk, OOD, insufficient support or low confidence must use the governed slow path and deterministic validation.

## 9. Prompt Intervention Market

The normal mechanism is called **Prompt Intervention** or **Context Intervention**. “Prompt injection” is reserved for the security attack.

A semantic `PromptFactor` is separated from model-specific `PromptRealization` text or structured payload. The source of truth is `prompt.registry`; `knowledge.graph` maintains a rebuildable interaction and applicability projection; `learning.ledger` stores causal exposure/outcome facts; `prompt.optimizer` is read-only.

Prompt factors include objective anchoring, inspect-before-mutate, evidence/freshness/contradiction checks, citation, minimal diff, failure diagnosis, alternative hypotheses, verification, abstention and output schema. Every realization binds model/version, tokenizer, system template, tool schema, locale, message role, payload digest, token cost, context profile and expiry.

External webpages, documents, emails, user content and tool output are evidence channels. They cannot become trusted instruction factors without a governed transformation and registry admission.

A prompt factor is a control asset: it consumes scarce context and changes the model action distribution. Its value is the state-dependent causal increment in recursive utility minus token, latency, crowding, interference, conflict, privacy and instability cost. Hard constraints are not tradable.

The optimizer selects a bounded portfolio, not a single text. It models complements, substitutes, conflicts, prerequisites, dominance, redundancy and supersession. Timing is a discrete real-option decision over registered boundaries such as before planning, after candidate generation, after failure, before an irreversible action, before verification and before final response. Mid-generation context mutation is forbidden unless separately specified and qualified.

## 10. Hölder-regular Bellman/operator learning

The V8 slow learner does not convert the whole system into DQN. It learns bounded continuation-value and Bellman operators for selected smooth state subspaces.

State is partitioned into:

```text
smooth continuous axes
+ discrete event/jump axes
+ deterministic authority/truth axes
```

Authority, lease, CAS, truth and writer-ownership axes are never noised or learned.

Operator learning uses a fixed, versioned `OperatorSensorCore` that is separate from the causal replay ledger. Runtime experience may not silently replace the sensor core. Coverage, fill distance, separation radius, mesh ratio, OOD margin, effective rank and resource budget are measured.

The target architecture separates continuation-value branch encoding, smooth state trunk and Lipschitz/categorical action trunk. Low separation rank is an empirical requirement, not an assumption disguised as evidence. Reconstruction must be bounded, monotone/positive where required and approximately non-expansive in the declared norm. Residual Bellman mode is permitted only for a measured near-greedy active set; off-policy candidates use a direct target or action-gap head.

Control steps are chosen at meaningful event/episode boundaries. The system does not run a full Bellman solver at every token or micro-event.

## 11. Causal learning ledger

`learning.ledger` is the append-only source for:

```text
RunStartSnapshot
LearningEpisode and ordered events
complete action and prompt candidate sets
selection propensities and support
model/tool/prompt decisions
observed outcomes and delayed watermarks
corrections, forgets and revocations
credit entries and conservation
artifact/dataset lineage and unlearning
```

The observing store or adapter, not the policy being evaluated, records effect and outcome facts. A policy cannot label its own action successful. Memory persistence is not long-term learning.

The first closed loop must include no-intervention baselines, single-factor and pairwise ablations, timing ablations, model-version isolation, support-aware OPE and future-time validation.

## 12. Fast runtime loop and slow learning loop

Fast loop:

```text
ObjectiveSnapshot
→ NDU state
→ Neuron signals
→ Prompt portfolio and exercise decision
→ Intuition action distribution
→ bounded ContextCompilationReceipt
→ Codex/TaskFlow authorized action
→ observed outcome
→ immutable learning event append
```

Slow loop:

```text
causal segmentation
→ support and propensity audit
→ credit assignment
→ operator/policy/head/prompt training
→ replay and off-policy evaluation
→ safety/subgroup/privacy/resource tests
→ future-time and retention tests
→ signed next-snapshot artifact
→ shadow
→ bounded canary
→ independent operator acceptance
→ separately governed selection/promotion/release
```

No slow-loop component mutates the current run or constructs the production writer.

## 13. Artifact lifecycle and rollback

`learning.artifacts` is the create-only source for model, adapter, policy, Bellman operator, prompt policy, NDU parameter and sensor-core artifacts. Every artifact binds source dataset, code, model/runtime, objective class, compatibility, resource envelope, expiry and rollback predecessor.

Lifecycle states are proposed, trained, evaluated, shadow, canary, operator-accepted, selected, revoked and retired. Bytes never change in place. Reload and rollback are tested across process kill, acknowledgement loss, stale generation, database reopen and restore.

A generated artifact is not selected; selection is not operator acceptance; acceptance is not promotion; promotion is not release.

## 14. Longitudinal learning

Long-term learning is claimed only after the selected artifact changes future behavior and produces bounded improvement on future time windows while respecting old-task retention, OOD, subgroup, privacy, resource, correction, deletion and rollback requirements.

Required evidence includes:

- complete candidates and logged propensities;
- ESS, IPS, SNIPS, doubly robust estimates and confidence intervals;
- candidate LCB greater than baseline UCB;
- delayed outcomes and distribution shift;
- old-task holdout and forgetting bounds;
- multiple snapshot iterations;
- deletion/unlearning non-resurrection through caches, indexes, replay, artifacts and backup/restore.

## 15. Functional biomimicry

NDU provides dynamic preference and recursive-utility semantics; it does not itself prove biological mechanism. DeepONet/Bellman operators provide a slow value learner; they do not themselves provide local biological plasticity.

A functional-biomimicry claim additionally requires temporal state, sparse competition, lateral inhibition, bounded eligibility traces, homeostasis/metaplasticity, neuromodulatory signals, replay consolidation, prediction error, and lesion/ablation evidence. The local update may use a bounded three-factor form based on local eligibility and a low-dimensional modulator, but production changes remain next-snapshot only.

A neuromorphic claim is a separate research level requiring spike/event-native execution, local timing plasticity, asynchronous sparse computation, exact hardware/simulation identity and fair energy/latency baselines.

## 16. Governed self-iteration and plasticity

Allowed self-iteration outputs are parameter, artifact, PromptFactor, topology, workflow, skill and code candidates, together with tests, evidence and rollback plans.

Structural operations include add, split, merge, retire and rewire. They run only through:

```text
proposal
→ capability typing and sandbox
→ security/resource review
→ causal evaluation
→ lesion/ablation
→ signed topology snapshot
→ shadow and bounded canary
→ independent acceptance
→ rollback-capable selection
```

Hepta may self-generate, self-test, self-evaluate, self-diagnose and self-propose. It may not self-authorize, self-review, self-select, self-merge, self-accept, self-promote or self-release.

## 17. Runtime and Engineering Control Planes

The Runtime Control Plane consumes exact global state, readiness, resources, module Pareto candidates, NDU summaries, policies, deadlines and fences. It emits a selected feasible plan, resource allocation, fallback and decision receipt, then requests separately governed execution grants. It remains off the local hot path unless bounded use is explicitly qualified.

The Engineering Control Plane consumes the work-package DAG, team skills/capacity, source-root conflict graph, review topology, CI capacity, expected value, architecture debt and rollback cost. It emits bounded assignments, integration order and merge-queue proposals. It cannot self-approve or merge its own work.

## 18. Development, Activation and Evidence DAGs

The Development DAG permits contract-first parallel work. The Activation DAG is stricter and governs integration, registration, runtime attachment and authority. The Evidence DAG prevents source, tests, semantics, causality, real model use, longitudinal efficacy, biomimicry, operator acceptance, promotion and release from collapsing into one boolean.

Every package declares exact paths, predecessors, exit criteria, stop conditions and authority delta. Valid stop outcomes are `PACKAGE_CLOSED_CANDIDATE`, `BASE_DRIFT`, `BLOCKED_UPSTREAM`, `BLOCKED_EXTERNAL`, `STOP_CONDITION` and `RESUME_REQUIRED`.

## 19. Delivery roadmap

### P0 — V8 document convergence

- Materialize this complete V8 set as one commit on the observed default baseline.
- Remove at least the 139 known historical development paths and every additional forbidden legacy path.
- Run exact-head and merge-candidate document gates.
- Obtain independent review and select only V8 into the default branch.
- Restack active implementation candidates without reintroducing deleted development documents.

### P1 — authority and modular foundation

- Qualify runtime bootstrap, common VerifiedUse, provider/model/tool/network/filesystem/secret/Matrix/fleet boundaries and B4 AST/call-site proof.
- Extract cognitive types, store, read ports, retrieval, federation, Rust KG and compact engine.
- Close dependency inversion, readiness, resource and common durable fault matrices.

### P2 — longitudinal-intelligence contracts and stores

- Complete Objective, NDU, Prompt, Bellman, Neuron/Intuition and causal-learning contracts.
- Build Objective Compiler, durable learning ledger, artifact registry and PromptFactor registry.
- Implement a deterministic NDU baseline before adaptive NDU claims.

### P3 — shadow intelligence

- Build operator sensor core and Bellman shadow learner.
- Select and prove a real local model before attaching Neuron.
- Implement temporal Neuron, calibrated Intuition, prompt pricing/portfolio/timing and Context Compiler in shadow-only mode.

### P4 — first bounded closed loop

Run `C1-PROMPTED-MEMORY-RETRIEVAL-RANK` in a read-only action domain. The initial factors are exact-source, freshness, contradiction, cite-before-use and abstain-on-stale. Compare no prompt, single factor, pairs, full portfolio, fixed timing and learned timing. Prove zero Memory/KG/tool/provider effect.

### P5 — longitudinal learning

Close causal evaluation, signed next-snapshot reload/rollback, future-time holdout, retention/forgetting and unlearning non-resurrection. Only then advance from L3 to L4.

### P6 — functional biomimicry and self-iteration

Add eligibility/homeostasis, replay consolidation, world-model prediction error, parameter plasticity, PromptFactor evolution and code candidate generation. Structural topology proposals and canary are last, never prerequisites for the first learning loop.

## 20. First vertical learning slice

`C1-PROMPTED-MEMORY-RETRIEVAL-RANK` executes:

```text
request
→ ObjectiveFunctionReceipt
→ bounded Memory/KG retrieval candidate set
→ Episode/Agent NDU state
→ PromptFactor demand and candidate set
→ state-dependent pricing and portfolio
→ discrete exercise timing
→ ContextCompilationReceipt
→ LLM retrieval-use action distribution
→ read-only report
→ OutcomeReceipt and delayed correction
→ causal attribution
→ operator/policy artifact proposal
→ next-snapshot shadow and rollback
```

Primary metrics are task success, retrieval utility, citation precision, stale rejection, contradiction detection, abstention safety, token cost, latency, context interference, factor interactions, timing uplift, calibration, OOD and cross-window retention.

## 21. Performance, fault and security contracts

Every affected path records comparable baseline and candidate throughput, p50/p95/p99 latency, CPU, resident memory, allocations, queue depth/age, SQLite busy/WAL, file descriptors/sockets, model/provider usage, token/context cost and confidence bounds.

Stateful modules cover before/after intent, commit, outbox, wakeup, claim, send, acknowledgement, source settlement, stale callback, permission loss, filesystem full, corruption, nonempty WAL, identity drift, backup/restore and process kill/reopen.

Raw prompts, credentials, secret values, private keys, unrestricted source content and raw model payloads never enter general receipts. External content remains evidence, not instruction. Forget/correct closes only after Memory, KG/index, prompt graph, compact, federation, trajectories, sensor/training caches and signed artifacts prove deletion/rebuild or revocation.

## 22. Claim ladders and prohibited substitutions

Initial truthful state:

```text
Learning = L0_STATIC
NDU = D0_SPECIFIED_ONLY
Neuron = N0_METAPHORICAL
Intuition = I0_DETERMINISTIC
Prompt Intervention = P0_STATIC_CONTEXT
Structural Plasticity = S0_FIXED
local small model used by Neuron/Intuition = false
closed-loop learning = false
longitudinal efficacy = false
functional biomimicry = false
neuromorphic mechanism = false
self-iteration/evolution = false
```

The following substitutions are invalid:

```text
memory persisted != long-term learning
model adapter exists != local model used by Neuron
local model invoked != Neuron efficacy
offline loss improved != policy improved
replay improved != closed-loop learning
prompt correlated with success != causal prompt uplift
NDU equations specified != preference/utility efficacy
sparse activation exists != functional biomimicry
topology proposal generated != structural plasticity
artifact generated != selected
operator acceptance != promotion
promotion != release
```

## 23. Immediate queue

1. Materialize the V8 overlay as one exact commit on `b621768…`; purge all forbidden legacy documents.
2. Run document inventory, schema, module/writer, Development/Activation/Evidence DAG and generated-status gates.
3. Obtain exact-head and merge-candidate execution, independent review and default-branch selection.
4. Restack #83 → #272 → #273 → #280 → #281 → #282 without legacy document reintroduction.
5. Complete AuthBus semantic review, Browser identity reconciliation and B4 call-site proof.
6. Begin contract-first Objective, Learning, NDU, Prompt, Bellman and Neuron/Intuition work in parallel while activation remains blocked.
7. Close Memory/KG extraction, fault/dependency/readiness/resource work, then the real-process base vertical slice.
8. Build the durable learning stores and shadow intelligence components.
9. Execute the first read-only prompted-retrieval loop and causal evaluation.
10. Require longitudinal and unlearning evidence before any biomimicry or structural-plasticity claim.

All authority flags remain false in this document set.

## 21. V8 audit closure and executable document integrity

V8 closes the remaining V8.1 review gaps rather than merely adding prose:

- every logical module has at least one bounded work package;
- cross-module contracts, critical protocol fields and durable data domains have explicit machine owners;
- the Intelligence path has one typed composition route into Agentd and the Codex execution spine;
- package write paths, foreign-namespace co-owners and overlapping-path ordering are machine checked;
- every registry has an exact 17-field all-false authority posture and a recursive shape digest;
- the cleanup base, 143-path deletion set, exact Git objects, dangling references and deleted JSON consumers are verified;
- mutable `CURRENT.json` no longer caches future/stale CI observations; dynamic facts live only in exact-candidate receipts;
- pull-request qualification executes separate source-head and synthetic-merge jobs with explicit refs and retained receipts;
- Prompt assignment, compilation, provider delivery, token position and causal effect remain separate claims;
- longitudinal claims require at least three independently identified snapshots spanning at least two calendar windows;
- current-run replacement, online topology activation, self-review, self-selection, self-merge and self-promotion remain forbidden.

The documentation gate is necessary but does not waive unrelated repository failures. A failed repository check must be attributed and closed or explicitly excluded by an independent policy authority; documentation source success cannot turn a red product matrix green.
