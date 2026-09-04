# Parallel module development and integration specification

**Overlay:** `HEPTA-V8-PRECODING-READINESS` v8.2.0-readiness
**Coverage:** all 40 registered modules
**Purpose:** start parallel implementation without contract drift, path collision or false capability claims

## 1. Scope and authority boundary

This specification assigns every registered module to one primary implementation lane and defines cross-lane checkpoints. It does not grant broad repository ownership, bypass work-package predecessors or authorize merge. Each team receives a bounded `ParallelLaneEnvelopeV1` tied to an exact source receipt and existing `WORK_PACKAGES.json` paths.

## 2. Frozen inputs and lane envelopes

Before coding, all lanes freeze:

```text
CanonicalSourceReceiptV1
MODULES, CONTRACTS, PROTOCOL_SCHEMAS and DATA_AUTHORITY digests
WORK_PACKAGES, PATH_OWNERSHIP and three DAG digests
readiness, algorithm, CNS and HNMF document-set digests
module technical guide digests
qualification profile and mandatory checks
```

A lane envelope names modules, work packages, allowed paths, resource ceiling, stop conditions and expiry. Any change to a frozen semantic input is base drift and requires a new envelope. Mock implementations are allowed only behind the exact typed contract and may not become durable alternative owners.

## 3. Primary implementation lanes

| Lane | Modules | First exit condition |
|---|---|---|
| A Foundation | types, wire, authority, operations, evidence, auth, secrets | shared contracts and fault semantics compile |
| B Runtime | supervisor, fleet, agentd, Codex, inference, taskflow, channels, browser, UI | typed effect boundaries and deterministic fallbacks |
| C Memory | cognitive types/store/read, retrieval, federation, KG, compact, prompt registry, context | coherent reads, single writers and deletion fixtures |
| D Objective/Value | objective compiler, NDU, runtime control | deterministic objective and NDU baseline |
| E Learning | ledger, operator, evaluation, artifacts | immutable episode/evaluation/artifact chain |
| F Adaptive Policy | neuron, intuition, prompt optimizer, plasticity, intelligence façade | shadow-only adaptive path with no current mutation |
| G Engineering | engineering control | bounded envelopes, sandbox and independent evaluator adapter |

The machine registry contains exact membership. A module has one primary lane; integration tracks may add explicit co-ownership without transferring its durable facts.

## 4. Cross-lane integration tracks

`TRACK-1-READ-ONLY-VERTICAL` composes runtime, memory and objective/value into request → retrieval → read-only report with zero external effect. `TRACK-2-ADAPTIVE-SHADOW` adds complete candidate logging, independent outcomes and next-snapshot artifacts. `TRACK-3-EMBODIMENT-AND-ASSIMILATION` wraps digital organs and one authorized external service in simulator or sandbox before any canary.

An integration PR owns adapter and test paths only. Domain changes return to the owning lane or use a declared co-owner package.

## 5. Integration checkpoints

```text
I0 SOURCE: exact source, branch purpose and document digests
I1 TYPES: canonical types/protocols compile and round-trip
I2 STORES: single writers, migrations, crash/reopen and deletion fixtures
I3 VERTICAL: deterministic read-only end-to-end slice
I4 SHADOW: adaptive candidates, complete propensities and independent outcomes
I5 LONGITUDINAL: future windows, retention, unlearning and rollback
I6 ITERATION: sandboxed candidate generation and independent review
I7 EMBODIED/ASSIMILATION: simulator/HIL or external-service qualification
```

`IntegrationCheckpointV1` lists lane receipts, contract digest, tests, open blockers, evaluator and decision. A later checkpoint cannot waive an earlier failure.

## 6. Branch, PR and merge discipline

Branches use the source policy classes and exact base. One PR carries one bounded package or integration checkpoint. The PR body lists source/tree, work envelope, changed paths, contracts, authority delta, tests, resources, rollback and unresolved external gates.

Source-head and synthetic-merge checks both pass. Overlapping write paths require DAG order or a registered lease. Generated code, documentation and status files are checked for clean-worktree parity. A green module test cannot turn a red product matrix green.

## 7. Stop conditions and escalation

All lanes stop on base drift, authority delta, cross-owner write, contract ambiguity, unknown schema, unbounded queue/retry/resource, failed mandatory test, missing rollback, evaluator collision, claim/evidence mismatch or deletion resurrection. Infrastructure failures may be retried only under the existing package policy; semantic failures require a new candidate.

Questions that change hard semantics become a contract decision owned by the relevant primary/deputy pair. Teams do not resolve them by local convention.

## 8. Required evidence per lane

Every lane produces exact source inventory, static verification, focused and package tests, all-target build, strict lint, fault results where stateful, performance/resource measurements, clean worktree, source-head and merge-candidate receipts. Adapters add stale/revoked grant, payload drift, timeout and indeterminate outcome tests. Learning lanes add support, future-window, retention and unlearning evidence only when making those claims.

## 9. Coding-entry checklist

Parallel coding may begin only after all 40 module guides contain the readiness overlay, each module appears in exactly one primary lane, no readiness gap is open, source and lane receipts are current, shared protocol code generation is frozen, path conflicts are ordered, deterministic fallbacks exist and external capability gates remain explicitly unclaimed.

## Appendix A. Closed gap and protocol mapping

This appendix is a closed-world traceability projection. Each identifier is normative in `READINESS.json`, `PROTOCOLS.json` or `GAPS.json`; this Markdown file does not redefine the registry record.

Protocols:

- `ParallelLaneEnvelopeV1`
- `IntegrationCheckpointV1`
- `CanonicalSourceReceiptV1`

Closed documentation gaps:

- `RDY-GAP-PAR-001`
- `RDY-GAP-PAR-002`
- `RDY-GAP-PAR-003`
- `RDY-GAP-PAR-004`

Bound work packages:

- `ART-1-LEARNING-ARTIFACT-REGISTRY`
- `ART-2-NEXT-SNAPSHOT-RELOAD-ROLLBACK`
- `ASM-0-EXTERNAL-SYSTEM-CONTRACTS`
- `ASM-1-DISCOVERY-MANIFEST`
- `ASM-2-DEBIAN-BRIDGE-SANDBOX`
- `ASM-3-STATE-MIGRATION-QUALIFICATION`
- `ASM-4-FEDERATED-ORGAN-ENROLLMENT`
- `AUTHBUS-P1.3-V12`
- `BIO-0-NEURON-INTUITION-CONTRACTS`
- `BIO-1-ELIGIBILITY-HOMEOSTASIS`
- `BIO-2-REPLAY-CONSOLIDATION`
- `BIO-3-WORLD-MODEL-PREDICTION-ERROR`
- `BROWSER-WEB-C1`
- `C1-PROMPTED-MEMORY-RETRIEVAL-RANK`
- `CTX-1-CONTEXT-COMPILER`
- `DOC-0-CANONICAL-DOCUMENT-CONSOLIDATION`
- `DOC-1-V8-SEMANTIC-UPGRADE`
- `DOC-2-DEFAULT-BRANCH-SELECTION`
- `DOC-3A-SOURCE-BINDING-RECONCILIATION`
- `DOC-3B-MODULE-TECHNICAL-DOCUMENTS`
- `DOC-3C-MODULE-DOC-CLOSED-WORLD`
- `DOC-3D-ADAPTIVE-ALGORITHM-DOC-CLOSED-WORLD`
- `DOC-3E-PRECODING-READINESS-CLOSED-WORLD`
- `DOC-REGISTRY-CLOSED-WORLD`
- `ECP-1-ENGINEERING-CONTROL-PLANE`
- `EMB-0-EMBODIED-CONTRACTS`
- `EMB-1-SENSOR-BUS-BODY-SCHEMA`
- `EMB-2-REFLEX-MOTOR-ACTUATION`
- `EMB-3-HIL-SIM-TO-REAL-QUALIFICATION`
- `FLEET-1-ALLOCATION-CONTRACT`
- `HBO-0-BELLMAN-OPERATOR-CONTRACTS`
- `HBO-1-OPERATOR-SENSOR-CORE`
- `HBO-2-BELLMAN-OPERATOR-SHADOW`
- `HEPTABAO-1-SECRET-BOUNDARY`
- `INFER-V4-T1`
- `INFER-V4-T2`
- `INFER-V4-T3`
- `INFER-V4-T4`
- `INFER-V4-T5`
- `INT-1-CALIBRATED-INTUITION-POLICY`
- `INT-2-AGENTD-CODEX-COMPOSITION`
- `INTELLIGENCE-A0-Q0.63`
- `LONG-1-TEMPORAL-HOLDOUT`
- `LONG-2-RETENTION-FORGETTING`
- `LONG-3-UNLEARNING-NON-RESURRECTION`
- `LRN-0-CAUSAL-LEARNING-CONTRACTS`
- `LRN-1-DURABLE-EPISODE-LEDGER`
- `LRN-2-CAUSAL-EVALUATION`
- `MATRIX-1-CHANNEL-BOUNDARY`
- `MEM-0-TYPES`
- `MEM-1-STORE`
- `MEM-2-RETRIEVAL`
- `MEM-3-FEDERATION`
- `MEM-4-KG`
- `MEM-5-COMPACT`
- `MEM-8-PRODUCTION-WRITER`
- `MEM-READ-1-SNAPSHOT-PORT`
- `NDU-0-PREFERENCE-UTILITY-CONTRACTS`
- `NDU-1-DETERMINISTIC-UTILITY-BASELINE`
- `NDU-2-AGENT-DOMAIN-HIERARCHY`
- `NEU-1-LOCAL-MODEL-BAKEOFF`
- `NEU-2-TEMPORAL-SIGNAL-RUNTIME`
- `OBJ-0-OBJECTIVE-CONTRACTS`
- `OBJ-1-OBJECTIVE-COMPILER`
- `P0.7A-RUNTIME-BOOTSTRAP`
- `P0.7B-B0-VERIFIED-USE`
- `P0.7B-B1A-PROVIDER-BOUNDARY`
- `P0.7B-B1B-MODEL-BOUNDARY`
- `P0.7B-B2-TOOL-NET-FS`
- `P0.7B-B3-BOUNDARIES`
- `P0.7B-B4-CALLSITE-PROOF`
- `P0.7D-FAULT-MATRIX`
- `P0.7E-DEPENDENCY-INVERSION`
- `P0.8A-AST-RATCHET`
- `P0.8B-READINESS`
- `P0.8C-RESOURCE-BUDGETS`
- `P0.8D-VERTICAL-SLICE`
- `P0.9-EXTERNAL-GATES`
- `PIM-0-PROMPT-INTERVENTION-CONTRACTS`
- `PIM-1-PROMPT-FACTOR-REGISTRY`
- `PIM-2-PROMPT-PRICING-PORTFOLIO-SHADOW`
- `PIM-3-FACTOR-EVOLUTION`
- `PLATFORM-0-TYPE-BOUNDARY`
- `PLS-1-PARAMETER-PLASTICITY`
- `PLS-2-TOPOLOGY-PROPOSAL`
- `PLS-3-BOUNDED-STRUCTURAL-CANARY`
- `RCP-1-RUNTIME-CONTROL-PLANE`
- `SELF-1-CODE-CANDIDATE-PIPELINE`
- `TASKFLOW-1-EXECUTION-BOUNDARY`
- `UI-NATIVE-1-SHELL`
- `UI-V5`
