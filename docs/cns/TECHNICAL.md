# Hepta CNS, organ graph and embodied-control technical specification

**Plan:** `HEPTA-GLOBAL-MODULAR-DEVELOPMENT-PLAN` v8.1.0-cns-organ  
**Parent plan:** v8.0.0  
**Status:** repository specification and deterministic reference closure  
**Capability posture:** no physical activation, longitudinal efficacy, biological equivalence, acceptance, promotion or release claim

## 1. Scope and architectural decision

Hepta is organized as a distributed central nervous system rather than a single privileged optimizer. The constitutional kernel defines non-learnable authority, truth, privacy, deletion, ownership and evidence boundaries. The CNS compiles objectives, value, memory, attention, world-state hypotheses and plans. Brainstem, spinal and peripheral layers preserve local liveness, calibration and safety. Organs expose typed ports and bounded resources and may be activated, drained, quarantined or retired only through an immutable body-graph generation.

This design is functional biomimicry, not anatomical identity. Biological names communicate control roles and failure isolation. Every claimed mechanism must remain independently testable and must not be used to infer consciousness, human psychology or biological equivalence.

## 2. Constitutional kernel and immune boundary

The constitutional layer includes `kernel.authority`, `kernel.operations`, `kernel.evidence`, `auth.authbus` and the secret adapter boundary. Its hard fields never enter an optimizer as compensable weights. A feasible plan must first satisfy authority, scope, deletion, truth, single-writer, resource and rollback constraints. Only then may utility compare feasible alternatives.

An organ cannot mint the capability it consumes. Irreversible adapters consume a short-lived operation- and final-payload-bound token immediately before entry. Revocation, payload drift, body-generation drift or deadline expiry rejects the action. Evidence and acceptance remain separate from production writers.

## 3. Distributed CNS, brainstem and spinal separation

Codex App Server remains the sole high-level model, session, turn and tool-execution spine. This does not place language-model latency on physical control paths. `motor.control` and `spinal.reflex-safety` are deterministic, qualified local controllers. They may execute only within a previously authorized envelope and may never originate a new semantic objective or capability.

The brainstem supervises process generations, fences, watchdogs, readiness and degradation. The spinal layer can veto a planned action from current body state without waiting for central cognition. The reflex layer cannot broaden authority or invent an alternative effect; it can only stop, clamp, route to a prequalified fallback or request human takeover.

## 4. Multi-timescale control loops

Five loops are deliberately separated:

```text
reflex:        sub-millisecond to 10 ms, deterministic local veto/clamp
sensorimotor:  1 ms to 100 ms, body-state estimation and feedback control
cognitive:     100 ms to minutes, snapshot-bound planning and tool use
consolidation: minutes to days, replay and next-snapshot candidate creation
development:   days to releases, code/topology proposals and governed rollout
```

No slow loop may block a faster safety loop. A missed consolidation window is observable degradation, not permission to create unbounded catch-up work. Local controllers bind the exact body, calibration, rule and artifact generations and fail closed on mismatch.

## 5. Organ manifest and body graph

`OrganManifestV1` declares identity, version, class, ports, data authority, effect class, resource envelope, health checks, rollback predecessor and retirement policy. `BodyGraphSnapshotV1` contains all manifests, dependency and fallback edges, canonical topological order and a semantic digest.

The body graph is acyclic. Every essential organ except the constitutional kernel and human override has a qualified fallback. A fallback may reduce capability but may not widen scope or effect authority. Local-hot-path organs reject configurations that require synchronous central RPC. Graph publication is atomic by generation; a consumer never mixes manifests from different generations.

## 6. Organ lifecycle, addition, removal and modification

The lifecycle is:

```text
proposed -> built -> simulated -> qualified -> dormant -> canary -> active
                                               |          |       |
                                               +------> quarantined
active -> draining -> retired
```

Activation requires a current qualification receipt and distinct generator/operator identities. Removal begins with `draining`, blocks new work, reconciles outstanding operations, migrates or tombstones owned state, proves fallback readiness and only then retires. Deleting code is not the same as retiring an organ because historical records must remain interpretable.

Runtime may choose among already qualified compatible organs and bounded soft settings. Code, schema, authority, topology and hard-boundary changes create `OrganProposalV1` or `TopologyProposalV1` for the next generation. Structural operations are typed `add`, `split`, `merge`, `rewire` and `retire`; arbitrary graph patches are rejected.

## 7. Objective, value and homeostasis

`objective.compiler` freezes principal scope, success predicates, terminal conditions, legal and forbidden action classes and evidence requirements. `utility.ndu` may adapt bounded preference state, resource allocation, evidence effort, exploration and abstention but cannot replace the objective.

Homeostasis treats compute, memory, disk, network, energy, temperature, time and risk as explicit endowments. Allocation first reserves essential floors, then distributes the remaining budget by bounded priority and need. Sum allocation can never exceed the endowment. Overload selects a declared degradation mode; it does not silently borrow from safety, rollback or evidence budgets.

## 8. Sensory timing, calibration and body schema

Every `SensorObservationV1` carries sensor identity, monotonic time, calibration generation, body generation, payload digest, uncertainty and principal scope. Unknown sensor identity, stale observation, future timestamp beyond skew, expired calibration, body mismatch, unbounded payload or scope escape fails before fusion.

`body.schema` publishes a coherent `BodyStateEstimateV1` for one source range and generation. Pose, velocity, contact, integrity and uncertainty are jointly snapshot-bound. Digital organs such as browser or Matrix use the same semantics: page/session generation, authentication state and remote-effect uncertainty act as digital proprioception.

## 9. Memory, HNMF and attention workspace

HNMF supplies a qualification-level multimodal hippocampal substrate: immutable events, modality spans, associative engrams, contradiction edges, sparse competition, eligibility, replay and deletion propagation. It remains a rebuildable projection and cannot replace authoritative source facts.

The attention workspace performs bounded union, deduplication, salience selection and context compilation. It records omitted-count bounds and uncertainty. External text and tool output remain evidence channels and cannot become trusted instructions without governed transformation. High-risk contradiction, insufficient coverage or OOD forces abstention or a slow path.

## 10. World model, affordance and metacognition

The world model predicts typed successor state, outcome, uncertainty and OOD for each legal candidate. It segments discrete events and deterministic hard axes rather than smoothing them into a latent vector. Affordances bind object/body state to legal action templates; they do not grant authority.

Metacognition tracks capability limits, calibration, model and tool identity, failure attribution and evidence sufficiency. A self-model is a scoped uncertain estimate, not a source fact about a person. When the system cannot distinguish model error, stale state or actuator failure, it reports indeterminate and routes to reconciliation or human takeover.

## 11. Action gating, motor planning and reflex safety

The action gate receives the complete generator-relative legal set, propensities and hard vetoes. The no-op/abstain action is always legal. `motor.plan` converts a selected semantic action into `ActuationIntentV1`, binding objective, body generation, target actuator, final payload digest, safety envelope, deadline, idempotency key and authority witness.

Before adapter entry, `spinal.reflex-safety` evaluates current body state and rule generation. Collision, force, speed, temperature, tilt, stale state, human stop or integrity breach produces `ReflexVetoV1`. The veto occurs before dispatch and remains independent of plan utility or model confidence.

## 12. Effect execution and terminal observation

`actuator.gateway` is the only organ class in this reference that crosses an external effect boundary. Queue acknowledgement or receipt by a transport is `accepted` or `dispatched`, never `succeeded`. Success requires `PhysicalOutcomeReceiptV1` from the effect owner or trusted observer.

Acknowledgement loss leaves an operation `indeterminate`. A fenced reconciler records applied, not applied or quarantined. Reusing an idempotency key with a different final-payload digest is a conflict. Compensation is a new authorized effect and is never assumed to undo the original action.

## 13. Long-term learning, sleep and skill consolidation

The causal ledger records state, complete candidate set, chosen propensity, delivered intervention, authorized action, independent outcome, correction, credit and deletion lineage. A policy cannot label itself successful. Missing or delayed outcome is not zero reward.

Sleep consolidation samples immutable eligible episodes with source quotas, surprise, coverage and retention priorities. Revoked rows are excluded before replay. Consolidation may propose semantic prototypes, procedures, predictive transitions, local parameter deltas or topology candidates, but only as immutable next-snapshot artifacts. Future-time holdout, old-task retention, subgroup, OOD, deletion non-resurrection and rollback are mandatory before any longitudinal claim.

## 14. Governed self-iteration and digital twin

`control.engineering` creates a bounded iteration envelope with exact base, allowed paths, denied authorities, candidate budget, mandatory tests and rollback. No-change is always a candidate. Generated code and topology run first in an isolated worktree and digital twin with no credentials or production network.

The generator may reach only `sandbox_tested`. Independent evaluation, review, acceptance, selection, promotion and release require distinct identities and receipts. The candidate cannot modify the evidence, tests or authority policy that judges that same candidate. Base drift invalidates the packet.

## 15. Security, privacy and human override

Threats include sensor spoofing, calibration drift, body-generation replay, activation flooding, prompt escalation, poisoned replay, unsafe topology churn, effect acknowledgement confusion, deleted-data resurrection and self-promotion. Controls are exact digests, bounded inputs, source scopes, generation checks, deterministic ordering, negative authority fields, independent observation and fail-closed state transitions.

Human override supports stop, takeover, consent revision and recovery. It is authenticated, scoped, expiring and auditable. Emergency stop does not require model cooperation. Restart after stop requires a fresh body snapshot, integrity checks, reconciled operations and explicit recovery authority.

## 16. Reference implementation and quantitative gates

The dependency-free reference implements objective immutability, body-graph validation, organ lifecycle, independent qualification, homeostatic allocation, sensor staleness, body generation, deterministic plan selection, reflex veto, idempotent effect ledger, next-snapshot topology and revoked-row exclusion.

Repository gates require 24 organs, 15 protocols, 22 closed reference gaps, zero positive authority flags, an acyclic dependency graph, complete module coverage, deterministic test replay and successful HNMF and paper-byte verification. Physical gates remain separate: target-host p99 timing, hardware-in-loop actuation, emergency stop, collision/force limits, real sensor calibration, future-time learning, ablation/lesion evidence, operator acceptance and production rollout.

## 17. Migration sequence

1. Land the ordinary source specification and deterministic reference without transport or self-push workflows.
2. Integrate HNMF as qualification-only memory and independently replay paper source bytes.
3. Wrap existing browser, UI, Matrix and tool adapters as digital organs using the common intent/outcome semantics.
4. Materialize objective, value, neuron, learning, runtime-control and engineering-control target modules.
5. Add simulator-backed sensor bus, body schema, world model, motor planning, local control and reflex veto.
6. Prove crash/reopen, acknowledgement loss, generation rollover, unlearning and rollback.
7. Run shadow and hardware-in-loop evaluation with no autonomous production effects.
8. Request separately governed canary, operator acceptance, selection, promotion and release.

Repository completion closes design and deterministic-reference gaps only. Evidence that depends on real hardware, future calendar time or an independent decision remains a named external gate and may never be replaced by prose or fixtures.
