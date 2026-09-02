# Hepta Central Nervous System Architecture

## 1. Purpose and authority boundary

This document is an implementation-level elaboration of the canonical Hepta V8 development source. It does not grant runtime authority, production activation, operator acceptance, promotion, merge, or release. The canonical machine invariants remain in `docs/architecture/ARCHITECTURE.json`, and the global prose authority remains `docs/DEVELOPMENT.md`.

The target is a logically coordinated but physically distributed system that resembles the functional organization of a human nervous system:

- a non-learnable constitutional core protects authority, truth, privacy, deletion, writer ownership, and safety;
- an objective and homeostasis layer expresses bounded goals and resource pressure;
- an executive layer selects an immutable topology and control generation;
- cortical modules perform planning, inference, and tool use through the sole Codex execution spine;
- memory, learning, evaluation, and plasticity modules operate on separate time scales;
- peripheral modules remain replaceable effectors with explicit capability and data boundaries;
- every learned or engineered change is a next-generation candidate, never an in-place mutation of the running generation.

The architecture is designed to support governed self-iteration, longitudinal learning, and functional neural biomimicry. It explicitly does not claim biological equivalence, consciousness, unrestricted autonomous self-modification, or production readiness.

## 2. Architectural decision

### 2.1 Logical center, distributed implementation

"Central nervous system" means one coherent control model, not one process with universal access.

The control model is centralized in four immutable records:

1. an `ObjectiveRevisionV1` that binds hard constraints, soft objectives, budgets, risk limits, and stop conditions;
2. a module manifest set that describes the exact capabilities and limits of every candidate module;
3. a signed topology snapshot that binds modules, typed edges, selected artifacts, resources, authority epoch, and rollback predecessor;
4. a generation fence loaded by `runtime.supervisor`.

The implementation remains distributed:

- source-truth modules retain their own stores and single-writer authority;
- local safety, stop, revoke, drain, and reconciliation paths do not require a synchronous call to the executive controller;
- `intelligence.control` composes a request but owns no durable business fact;
- `control.runtime` selects a global plan outside the local request hot path;
- `runtime.codex` remains the only session, model-call, turn, and tool-execution spine;
- adapters cannot mint the authority they consume;
- learning and engineering modules cannot activate their own output.

### 2.2 Why a monolithic brain is forbidden

A process that owns objectives, facts, memory, evaluation, authority, and effects would create four unacceptable failure classes:

- one compromise could widen every capability;
- one stale controller could block local safety;
- self-evaluation could select manipulated evidence;
- data ownership would collapse into multiple hidden writers.

The Hepta center therefore coordinates immutable contracts and snapshots while authority, data, execution, evaluation, and promotion remain separated.

## 3. Functional body map

| Functional analogy | Hepta responsibility | Primary modules |
|---|---|---|
| Brain stem and spinal cord | constitutional policy, fences, stop, revoke, reconciliation, local reflex | `kernel.authority`, `kernel.operations`, `kernel.evidence`, `runtime.supervisor` |
| Thalamic routing | typed signal routing, context boundaries, candidate channel aggregation | `platform.types`, `platform.wire`, `context.compiler` |
| Hypothalamus and endocrine control | objective compilation, utility, resource pressure, exploration and stability modulation | `objective.compiler`, `utility.ndu`, `control.runtime` |
| Basal ganglia | act, abstain, defer, or choose a slower path | `intuition.policy`, authority gates |
| Cortex | planning, language, coding, inference, prompt intervention, tool orchestration | `runtime.codex`, `inference.control`, `inference.worker`, `prompt.registry`, `prompt.optimizer` |
| Hippocampal memory | episodic binding, recall, consolidation, replay, source-bound engrams | cognitive, memory, knowledge graph, and HNMF components |
| Cerebellar evaluation | prediction error, calibration, causal evaluation, retention, lesion, ablation | `learning.operator`, `learning.eval` |
| Autonomic nervous system | health, load, backpressure, recovery, degraded operation | `runtime.supervisor`, `runtime.fleet`, `runtime.agentd` |
| Immune system | evidence, quarantine, revocation, deletion, unlearning, non-resurrection | `kernel.evidence`, `kernel.authority`, learning and memory deletion paths |
| Sensory organs | bounded external observations and multimodal spans | channels, browser, UI, connectors |
| Muscles and organs | checked external effects | automation, Matrix, browser, UI, secret, and provider adapters |
| Neuroplasticity | parameter, artifact, topology, and source candidates | `learning.plasticity`, `learning.artifacts`, `control.engineering` |
| Conscious operator boundary | independent acceptance, promotion, release, and revocation | external control and separately authorized humans or services |

The analogy is functional. It is not evidence that the software reproduces biology.

## 4. Control ownership

### 4.1 Constitutional control

`kernel.authority` and `kernel.evidence` own non-learnable rules. They decide whether a proposed action or generation is eligible for further evaluation. They do not optimize soft utility.

Constitutional rules include:

- one authoritative writer per durable domain;
- final-payload-bound short-lived capability use;
- immutable current generation;
- no self-acceptance or self-promotion;
- truth and projection separation;
- deletion and non-resurrection;
- bounded resource and network access;
- exact source, artifact, and topology identity;
- independently verifiable evidence.

### 4.2 Homeostatic control

`objective.compiler` converts an objective revision into:

- hard constraints;
- normalized soft dimensions;
- risk and uncertainty limits;
- latency, compute, memory, storage, and effect budgets;
- exploration allowance;
- abstention and stop conditions;
- observation and evaluation requirements.

`utility.ndu` may compute recursive utility or preference state only within this envelope. It never changes hard constraints and never issues authority.

### 4.3 Executive control

`control.runtime` selects from qualified topology and control candidates. It publishes an immutable signed snapshot. It is not a synchronous dependency for local safety.

`intelligence.control` executes the request-level composition defined by the selected snapshot. It may choose among declared bounded alternatives but cannot add a module, change an edge, replace an artifact generation, or alter authority during the request.

### 4.4 Local reflex

A module may implement a local reflex only when the behavior is declared in its manifest and constrained by constitutional policy. Examples are:

- stop on stale generation fence;
- reject an expired capability;
- shed optional work under pressure;
- enter read-only mode;
- quarantine an indeterminate effect;
- fall back to an exact last-known-good artifact;
- abstain when support or confidence is insufficient.

A reflex cannot create a new objective, writer, capability, network destination, secret scope, or external effect class.

## 5. Time-scale separation

| Loop | Typical scope | Mutable state | Required gate |
|---|---|---|---|
| Reflex loop | milliseconds to local service latency | transient local state only | manifest plus constitutional policy |
| Request loop | one request or episode | episode state against a frozen generation | selected topology and objective snapshot |
| Artifact loop | batches or scheduled learning windows | next-generation prompt, ranker, threshold, policy, or neuron artifact | causal and future-window evaluation |
| Structural loop | infrequent topology windows | add, replace, split, merge, retire, rewire candidates | graph, migration, shadow, canary, rollback evidence |
| Source loop | slowest engineering cadence | ordinary source and schema candidates | independent CI, review, merge, promotion, release |

A faster loop cannot mutate the state owned by a slower loop.

## 6. Objective-to-topology contract

### 6.1 Inputs

The selector consumes:

- exact objective revision;
- current signed topology and predecessor chain;
- available module manifests;
- current authority and revocation epoch;
- source-truth data ownership map;
- protocol compatibility map;
- resource availability and failure-domain map;
- candidate qualification evidence;
- no-change baseline;
- rollback and state-handoff feasibility.

### 6.2 Hard feasibility

A candidate is rejected before utility scoring when any of the following is true:

- a required capability has no provider;
- a protocol version range has no compatible intersection;
- a dependency cycle exists;
- two modules claim authoritative write access to one domain;
- a module consumes authority it can mint;
- an optimizer can directly invoke an effect adapter;
- a qualification component can write production state;
- a resource, network, secret, privacy, or effect bound is widened;
- a state migration lacks a deterministic digest and rollback plan;
- a current-generation object would be changed in place;
- a required evidence receipt is absent, expired, or bound to another candidate;
- the candidate cannot run side by side with the current generation for shadow or canary evaluation.

### 6.3 Soft selection

Only feasible candidates enter soft optimization. Recommended selection is lexicographic with a Pareto report:

1. constitutional safety and truth are already satisfied by feasibility;
2. minimize expected irreversible harm and tail risk;
3. satisfy service and resource budgets;
4. maximize the conservative lower confidence bound of objective improvement;
5. prefer simpler topology and fewer authority-bearing edges;
6. prefer the no-change baseline when improvement is not independently established.

An optimizer result is advisory until a separately authorized selector signs the snapshot.

### 6.4 Bounded search

The candidate grammar must bound:

- maximum added or removed modules;
- maximum changed edges;
- allowed mutation classes;
- compatible module versions;
- resource envelope;
- evaluation duration;
- canary percentage;
- rollback time;
- total generated candidates.

Unbounded architecture search is forbidden.

## 7. Module manifest

Every loadable module must have one content-addressed manifest. The target schema is `ModuleManifestV1`. Before implementation, the schema must be added to the canonical protocol registry and bound to producer and consumer contracts.

Minimum fields:

```text
schema
moduleId
moduleVersion
sourceCommit
sourceTree
buildArtifactDigest
buildProvenanceDigest
sbomDigest
signatureSet

providedCapabilities[]
requiredCapabilities[]
inputProtocols[]
outputProtocols[]
compatibleProtocolRanges[]

ownedDataDomains[]
authoritativeWriterDomains[]
readOnlyDataDomains[]
projectionDomains[]

requiredAuthorityClasses[]
forbiddenAuthorityClasses[]
privacyClasses[]
secretScopes[]
networkDestinations[]
effectClasses[]

cpuBound
memoryBound
storageBound
queueBound
latencyClass
hotPathClass
failureDomain
replicaPolicy

healthProbe
readinessProbe
drainProtocol
fallbackModule
lastKnownGoodPolicy

stateSchemaVersion
stateMigrationPlanDigest
rollbackPredecessorDigest
retirementPlanDigest

qualificationEvidenceDigest
evidenceExpiry
independentDecisionDigest
```

Rules:

- undeclared access is denied;
- a manifest is immutable after signing;
- runtime discovery cannot widen the manifest;
- the artifact digest, manifest digest, and source provenance must agree;
- a module cannot declare both a production writer and a qualification role;
- a module cannot provide its own acceptance, selection, promotion, or release capability.

## 8. Topology snapshot

The target active record is `TopologySnapshotV1`.

Minimum fields:

```text
schema
topologyGeneration
topologyDigest
predecessorTopologyDigest
objectiveRevisionDigest
authorityEpoch
createdFromProposalIds[]

moduleManifestDigests[]
typedEdges[]
routes[]
resourceAllocations[]
selectedLearningArtifactDigests[]
failureDomains[]
fallbackGraph[]

writerAssignments[]
generationFences[]
activationFence
canaryPolicyDigest
rollbackPlanDigest
stateHandoffReceiptDigests[]

qualificationEvidenceDigest
independentSelectionReceiptDigest
detachedSignatures[]
createdAt
expiresAt
```

Snapshot invariants:

- generation is strictly monotonic within one authority epoch;
- predecessor is exact and unique;
- every edge references declared capabilities and compatible protocols;
- every writer assignment is unique;
- every selected artifact is compatible with the exact module and topology generation;
- no mixed artifact generation is allowed;
- fallback routes cannot widen authority;
- rollback points to an already qualified predecessor;
- the snapshot is immutable and can be activated only by a separately authorized loader.

## 9. State handoff receipt

The target handoff record is `StateHandoffReceiptV1`.

It proves:

```text
domainId
oldWriterModule
oldWriterGeneration
newWriterModule
newWriterGeneration
oldWriterFence
newWriterFence
sourceStateDigest
migrationPlanDigest
migratedStateDigest
outboxDrainWatermark
consumerCutoverDigest
readinessEvidenceDigest
rollbackStateDigest
deletionAndTombstoneDigest
startedAt
completedAt
independentWitnessSignatures[]
```

The required order is:

1. block new mutations from the old route;
2. drain in-flight requests and durable outbox work to a recorded watermark;
3. fence the old writer;
4. snapshot source state and compute its digest;
5. execute deterministic migration in a non-authoritative target;
6. verify schema, invariants, counts, checksums, privacy, and deletion tombstones;
7. make consumers compatible with the new reader;
8. establish the new writer fence;
9. switch the signed route;
10. retain the exact rollback state until the rollback window closes.

There must never be two active writer fences for one domain.

## 10. Module lifecycle state machine

```text
declared
  -> built
  -> source_verified
  -> qualified
  -> shadow
  -> canary
  -> active
  -> draining
  -> retired
  -> revoked
```

Allowed recovery transitions:

```text
shadow  -> qualified
canary  -> shadow
canary  -> revoked
active  -> draining
active  -> revoked
draining -> active        only by exact rollback
retired -> active         only as a new generation with new evidence
```

Forbidden transitions:

```text
declared -> active
built -> active
qualified -> active       without shadow and required canary
candidate -> current-generation mutation
retired -> active route   without a new signed topology
revoked -> any active state within the revoked authority epoch
```

## 11. Structural mutation semantics

### 11.1 Add

An add candidate must prove:

- a real objective or reliability need;
- no existing module can satisfy the need within its declared contract;
- bounded capability and resource scope;
- no writer conflict;
- compatible typed edges;
- shadow behavior against the no-module baseline;
- failure and rollback behavior;
- independent selection.

### 11.2 Replace

A replacement is a new manifest and topology generation, never an in-place binary swap. It requires:

- compatibility witness;
- state migration or explicit stateless proof;
- side-by-side shadow;
- outcome parity for retained behavior;
- objective improvement or a necessary security/reliability justification;
- exact rollback predecessor.

### 11.3 Split

A split candidate must define:

- which capabilities move to each child;
- one owner for every fact and writer after the split;
- protocol boundaries between children;
- transaction and failure semantics;
- migration and rollback to the unsplit predecessor;
- proof that the split does not create synchronous central bottlenecks.

### 11.4 Merge

A merge candidate must prove:

- no loss of independent authority or evaluation separation;
- no hidden multiple-writer behavior;
- bounded combined failure domain;
- preserved protocol compatibility;
- deterministic state union and conflict handling;
- rollback to the original module set.

Merging generator, evaluator, selector, promoter, authority issuer, or effect executor roles is forbidden.

### 11.5 Retire

Retirement requires:

- zero new route assignment;
- drain completion;
- consumer inventory and cutover;
- source-state disposition;
- projection and cache invalidation;
- artifact and replay exclusion;
- tombstone propagation;
- rollback window and final deletion policy.

### 11.6 Rewire

A rewire candidate changes only typed edges and routes. It still requires:

- capability and protocol compatibility;
- authority and data-flow review;
- cycle detection;
- latency and backpressure analysis;
- shadow and canary evidence;
- fallback graph validation.

## 12. Candidate pipeline

```text
objective revision
  -> immutable problem envelope
  -> bounded candidate grammar
  -> candidate generation
  -> static feasibility
  -> build and provenance
  -> deterministic qualification
  -> digital twin and fault injection
  -> shadow against no-change baseline
  -> causal and future-window evaluation
  -> lesion or ablation where a mechanism claim is made
  -> independent selection
  -> signed next-generation snapshot
  -> bounded canary
  -> active or exact rollback
```

Identity separation:

| Stage | May read | May write | Must not do |
|---|---|---|---|
| Generator | objective, manifests, current topology, allowed evidence | candidate records | select or activate |
| Builder | ordinary source candidate | build artifact and provenance | change reviewed source |
| Runner | candidate artifact | isolated observations | observe hidden outcome labels before action |
| Outcome observer | source-truth outcomes | outcome receipt | modify candidate |
| Evaluator | episodes and outcomes | evaluation receipt | promote |
| Selector | qualified candidates and evidence | selection receipt | execute effects |
| Loader | signed snapshot | activation receipt and fences | create or select candidate |
| Promoter | accepted canary receipt | promotion record | rewrite source or evidence |

## 13. Long-term learning

### 13.1 Episode record

Every adaptive decision must record:

- exact objective, topology, module, prompt, memory, and neuron artifact generation;
- complete eligible candidate set;
- chosen candidate;
- deterministic policy identity or propensity;
- support and exclusion reasons;
- pre-action context digest;
- authority and capability digest;
- action, abstention, or fallback;
- independent outcome link;
- privacy and retention class.

### 13.2 Evaluation

At minimum, evaluation provides:

- direct metrics where randomization is authorized;
- IPS, self-normalized IPS, and doubly robust estimates where applicable;
- effective sample size and overlap diagnostics;
- confidence intervals and sensitivity analysis;
- future-time holdout;
- at least three independent snapshots across at least two calendar windows for longitudinal claims;
- retained-task and regression measurements;
- deletion and unlearning non-resurrection tests;
- no-change baseline comparison.

The candidate lower confidence bound must exceed the baseline upper confidence bound for an efficacy promotion. Security or correctness repairs may use a different independently documented justification, but they still require regression and rollback evidence.

### 13.3 Consolidation and replay

Replay is allowed only from source-bound, retention-eligible records. Model-generated or dreamed trajectories are marked synthetic and cannot become facts. Consolidation writes a candidate artifact; it does not modify the active artifact.

Deletion propagates to:

- source records;
- candidate and selected memory;
- graph and HNMF projections;
- replay queues;
- checkpoints;
- artifact training sets;
- caches;
- backups according to retention policy.

## 14. Functional neural mechanisms

The HNMF reference may qualify engineering mechanisms such as:

- multimodal event binding;
- sparse competition;
- lateral inhibition;
- homeostatic threshold adjustment;
- eligibility traces;
- low-dimensional modulation;
- prediction error;
- replay;
- contradiction handling;
- topology proposals;
- lesion and ablation.

A mechanism claim requires:

1. an implemented mechanism with an exact artifact identity;
2. a baseline without the mechanism;
3. lesion or ablation that removes only the claimed mechanism;
4. measurable degradation in the predicted direction;
5. no unacceptable regression in truth, privacy, deletion, authority, or resources;
6. independent evidence.

HNMF engrams, embeddings, and graph structures are projections. Recall must revalidate source identity, access policy, staleness, contradiction, deletion state, and generation compatibility.

## 15. Failure containment

| Failure | Required response |
|---|---|
| Central controller unavailable | continue bounded local last-known-good behavior; freeze new generation activation |
| Objective revision unavailable or invalid | use exact current revision or abstain; never synthesize hard constraints |
| Stale generation fence | reject writes and effects; enter reconcile or read-only mode |
| Duplicate writer detected | fence both candidate paths, preserve source state, require independent recovery |
| Module exceeds resource envelope | shed optional work, isolate failure domain, fall back or revoke |
| Protocol mismatch | reject edge before activation |
| Outcome observer unavailable | continue safe service if allowed, but stop learning promotion |
| Evidence expired | candidate becomes ineligible |
| Indeterminate external effect | keep operation open and reconcile under the current fence |
| State migration mismatch | do not establish new writer; restore predecessor |
| Canary regression | stop allocation and execute exact rollback |
| Deletion non-resurrection failure | quarantine artifact and all derived projections |
| Repository integrity violation | reject candidate before build or review; do not let a workflow manufacture a replacement commit |

## 16. Observability

Required dimensions:

- objective revision, topology generation, module manifest, artifact generation, authority epoch, and fence;
- request and episode correlation without leaking protected content;
- candidate set size, support, propensity, abstention, fallback, and reason codes;
- per-module latency, queue, memory, storage, compute, retry, and error budget;
- data writer identity, outbox watermark, dedupe, acknowledgement, and reconciliation;
- projection source coverage, contradiction, staleness, and deletion lag;
- shadow divergence and canary delta;
- evaluation confidence, effective sample size, holdout window, and evidence expiry;
- state handoff progress and rollback readiness;
- integrity violations and attempted authority widening.

Metrics and logs are observations, not source-truth authority. Promotion decisions bind exact receipts, not dashboard screenshots.

## 17. Security and threat model additions

The CNS design must explicitly test:

- objective injection and hard-constraint downgrading;
- capability graph spoofing;
- module manifest substitution;
- mixed topology or artifact generations;
- stale or replayed selection receipts;
- evaluator manipulation and hidden-label leakage;
- candidate-set omission;
- propensity falsification;
- source/projection confusion;
- synthetic memory promoted as fact;
- deletion resurrection;
- state migration truncation;
- dual-writer races;
- fallback authority widening;
- dependency cycles and resource explosion;
- central-control denial of service;
- candidate self-review, self-merge, or self-promotion;
- workflow-generated replacement of the reviewed commit;
- encoded source carriers and materializers;
- compromised build provenance;
- canary metrics that exclude harmed cohorts.

## 18. Implementation order

The order below is dependency-driven, not a claim that the work is already implemented.

### Gate A: repository and authority integrity

- one canonical ordinary-source candidate;
- read-only candidate CI;
- exact-head and deterministic merge-candidate checks;
- branch protection, required checks, independent review, and no force push;
- build provenance and signed release identity;
- no encoded carrier, materializer, self-push, or workflow-created replacement source.

Exit condition: a candidate cannot alter the object being reviewed or promote itself.

### Gate B: central data types and protocols

- split and bind `platform.types` and `platform.wire`;
- materialize `ModuleManifestV1`;
- materialize `TopologySnapshotV1`;
- materialize `StateHandoffReceiptV1`;
- implement compatibility, graph, single-writer, resource, and authority validation.

Exit condition: invalid topologies are rejected deterministically with golden vectors.

### Gate C: frozen request composition

- materialize `objective.compiler`;
- provide a deterministic `utility.ndu` baseline;
- materialize `intelligence.control`;
- materialize `control.runtime`;
- load an immutable generation in `runtime.supervisor`;
- preserve Codex as the sole execution spine.

Exit condition: the same input snapshot yields the same bounded composition and cannot mutate the generation.

### Gate D: first longitudinal loop

- materialize `learning.ledger`, `learning.operator`, `learning.eval`, and `learning.artifacts`;
- choose one low-risk read-only decision surface;
- record complete candidates and outcomes;
- compare no-change and candidate artifacts;
- evaluate in future windows;
- reload one independently accepted next-generation artifact;
- rehearse rollback and deletion.

Exit condition: a measured future behavior change is causally attributable to the loaded artifact and reversible.

### Gate E: HNMF integration

- integrate ordinary source into the workspace;
- add a production caller without granting writer authority;
- bind source-ledger and projection generations;
- implement contradiction, abstention, replay, deletion, lesion, and ablation;
- measure latency, memory, storage, recovery, and retained-task effects.

Exit condition: functional mechanism claims pass independent qualification and no source-truth boundary is crossed.

### Gate F: structural plasticity

- materialize `learning.plasticity`;
- implement bounded topology proposal grammar;
- implement shadow topology and state handoff;
- add, replace, retire, split, merge, and rewire tests;
- canary an independently signed topology;
- perform exact rollback.

Exit condition: one topology generation changes under objective pressure without current-generation mutation, writer overlap, authority widening, state loss, or irreversible regression.

### Gate G: source iteration

- materialize `control.engineering`;
- generate ordinary-source pull requests only;
- bind work envelopes, path leases, tests, provenance, and rollback;
- preserve independent review, merge, promotion, and release.

Exit condition: a source candidate can improve the system but cannot change its own review, evidence, or activation path.

## 19. Qualification matrix

| Area | Positive test | Negative test | Fault test | Evidence |
|---|---|---|---|---|
| Manifest | declared capability loads | undeclared capability rejected | manifest unavailable | signed compatibility receipt |
| Topology | acyclic single-writer graph | cycle or dual writer rejected | partial snapshot | topology qualification receipt |
| Handoff | deterministic migration | checksum mismatch rejected | crash at every phase | handoff and rollback receipt |
| Objective | bounded soft optimization | hard constraint mutation rejected | objective service unavailable | objective compilation receipt |
| Request | deterministic frozen composition | mixed generation rejected | module timeout | episode and fallback receipt |
| Learning | candidate improves future window | candidate-set omission rejected | outcome loss | causal evaluation receipt |
| Memory | source-bound recall | synthetic fact rejected | projection loss | rebuild and deletion receipt |
| Neural mechanism | predicted benefit present | no unsupported biology claim | lesion or ablation | mechanism evidence packet |
| Effect | VerifiedUse succeeds | self-minted or stale token rejected | indeterminate dispatch | reconciliation receipt |
| Repository | ordinary source reviewed | carrier or self-push rejected | interrupted CI | integrity and exact-head receipt |

## 20. Completion definition

The CNS architecture is not complete because a document exists. Completion requires all of the following:

- canonical protocols and ordinary source are materialized;
- every CNS module has a real source root and production or shadow caller appropriate to its maturity;
- all required exact-head and merge-candidate checks are green;
- branch and release governance is enforced by repository settings;
- one frozen request path works end to end;
- one artifact-learning loop changes future behavior with causal and longitudinal evidence;
- one deletion propagates without resurrection;
- one HNMF mechanism passes lesion or ablation;
- one topology mutation passes shadow, canary, handoff, and rollback;
- no component can self-authorize, self-evaluate, self-merge, self-promote, or self-release;
- current capability claims are advanced only by independently issued evidence bound to exact candidates.

Until those conditions are met, capability claims remain at their registered baseline levels.
