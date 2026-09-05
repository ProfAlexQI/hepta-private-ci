# Hepta module execution dossiers

This qualification companion is subordinate to `docs/DEVELOPMENT.md` (plan 8.0.0) and the V8.2 readiness layer. It is not a second global plan. Canonical module ownership, source roots, contracts, data authority, work packages, the three DAGs, readiness, CNS and HNMF registries remain authoritative.

## Read order

1. [TECHNICAL.md](TECHNICAL.md): shared execution and evolution obligations.
2. [MODULE_DOSSIERS.json](MODULE_DOSSIERS.json): existing forty-module classification projection and eighteen required receipt fields.
3. [EXECUTION_SEMANTICS.md](EXECUTION_SEMANTICS.md): detailed objective, NDU, sequential evaluation, numeric compatibility, handoff, organ, developmental and assimilation semantics.
4. [DETAILS.json](DETAILS.json): exact forty-module design index with SHA-256 bindings, lanes, roots and work-package references.
5. The module-specific detail linked below: public operations, logical records, transaction order, algorithm, capacity, four named product-test designs and rollback.
6. [STATE_HANDOFF.schema.json](STATE_HANDOFF.schema.json): bounded same-owner cardinality-preserving handoff evidence schema with phase rules checked by the companion oracle.
7. [DETAIL_GAPS.json](DETAIL_GAPS.json): enumerated audit/module requirements, unresolved integration tasks and separately preserved external gates.
8. [STATUS.md](STATUS.md): the existing generated classification/depth status, not a new runtime or semantic acceptance record.

The new detail files refine intended implementations without claiming that their proposed operation names are already exported native symbols. Their `specified target` state applies to this design revision, not an assertion that no predecessor source code exists. A library or source directory is not a process, a production caller, a physical store or an independently accepted capability.

## Forty implementation designs

| Module | Primary lane | Detail |
|---|---|---|
| `platform.types` | `LANE-A-FOUNDATION` | [platform.types](detail/platform.types.md) |
| `platform.wire` | `LANE-A-FOUNDATION` | [platform.wire](detail/platform.wire.md) |
| `kernel.authority` | `LANE-A-FOUNDATION` | [kernel.authority](detail/kernel.authority.md) |
| `kernel.operations` | `LANE-A-FOUNDATION` | [kernel.operations](detail/kernel.operations.md) |
| `kernel.evidence` | `LANE-A-FOUNDATION` | [kernel.evidence](detail/kernel.evidence.md) |
| `auth.authbus` | `LANE-A-FOUNDATION` | [auth.authbus](detail/auth.authbus.md) |
| `secrets.heptabao` | `LANE-A-FOUNDATION` | [secrets.heptabao](detail/secrets.heptabao.md) |
| `runtime.supervisor` | `LANE-B-RUNTIME` | [runtime.supervisor](detail/runtime.supervisor.md) |
| `runtime.fleet` | `LANE-B-RUNTIME` | [runtime.fleet](detail/runtime.fleet.md) |
| `runtime.agentd` | `LANE-B-RUNTIME` | [runtime.agentd](detail/runtime.agentd.md) |
| `runtime.codex` | `LANE-B-RUNTIME` | [runtime.codex](detail/runtime.codex.md) |
| `inference.control` | `LANE-B-RUNTIME` | [inference.control](detail/inference.control.md) |
| `inference.worker` | `LANE-B-RUNTIME` | [inference.worker](detail/inference.worker.md) |
| `automation.taskflow` | `LANE-B-RUNTIME` | [automation.taskflow](detail/automation.taskflow.md) |
| `channel.matrix` | `LANE-B-RUNTIME` | [channel.matrix](detail/channel.matrix.md) |
| `browser.servo` | `LANE-B-RUNTIME` | [browser.servo](detail/browser.servo.md) |
| `ui.control` | `LANE-B-RUNTIME` | [ui.control](detail/ui.control.md) |
| `ui.native` | `LANE-B-RUNTIME` | [ui.native](detail/ui.native.md) |
| `cognitive.types` | `LANE-C-MEMORY` | [cognitive.types](detail/cognitive.types.md) |
| `cognitive.store` | `LANE-C-MEMORY` | [cognitive.store](detail/cognitive.store.md) |
| `cognitive.read` | `LANE-C-MEMORY` | [cognitive.read](detail/cognitive.read.md) |
| `memory.retrieval` | `LANE-C-MEMORY` | [memory.retrieval](detail/memory.retrieval.md) |
| `memory.federation` | `LANE-C-MEMORY` | [memory.federation](detail/memory.federation.md) |
| `knowledge.graph` | `LANE-C-MEMORY` | [knowledge.graph](detail/knowledge.graph.md) |
| `compact.engine` | `LANE-C-MEMORY` | [compact.engine](detail/compact.engine.md) |
| `prompt.registry` | `LANE-C-MEMORY` | [prompt.registry](detail/prompt.registry.md) |
| `context.compiler` | `LANE-C-MEMORY` | [context.compiler](detail/context.compiler.md) |
| `objective.compiler` | `LANE-D-OBJECTIVE-VALUE` | [objective.compiler](detail/objective.compiler.md) |
| `utility.ndu` | `LANE-D-OBJECTIVE-VALUE` | [utility.ndu](detail/utility.ndu.md) |
| `control.runtime` | `LANE-D-OBJECTIVE-VALUE` | [control.runtime](detail/control.runtime.md) |
| `learning.ledger` | `LANE-E-LEARNING` | [learning.ledger](detail/learning.ledger.md) |
| `learning.operator` | `LANE-E-LEARNING` | [learning.operator](detail/learning.operator.md) |
| `learning.eval` | `LANE-E-LEARNING` | [learning.eval](detail/learning.eval.md) |
| `learning.artifacts` | `LANE-E-LEARNING` | [learning.artifacts](detail/learning.artifacts.md) |
| `neuron.runtime` | `LANE-F-ADAPTIVE-POLICY` | [neuron.runtime](detail/neuron.runtime.md) |
| `intuition.policy` | `LANE-F-ADAPTIVE-POLICY` | [intuition.policy](detail/intuition.policy.md) |
| `prompt.optimizer` | `LANE-F-ADAPTIVE-POLICY` | [prompt.optimizer](detail/prompt.optimizer.md) |
| `learning.plasticity` | `LANE-F-ADAPTIVE-POLICY` | [learning.plasticity](detail/learning.plasticity.md) |
| `intelligence.control` | `LANE-F-ADAPTIVE-POLICY` | [intelligence.control](detail/intelligence.control.md) |
| `control.engineering` | `LANE-G-ENGINEERING` | [control.engineering](detail/control.engineering.md) |

## How to use the details

Start with the stable `docs/modules/<module>/TECHNICAL.md` guide and its canonical registry projection, then the readiness overlay and this companion. The seven detail sections do not replace existing module-specific implementation contracts. A package chooses existing compatible APIs and records the mapping from each specified operation to native symbols, actual consumers and tests. New APIs or serialized schemas need canonical versioned registration before wire/runtime admission. A reference schema cannot silently overwrite an existing protocol version.

The eighteen handoff fields in TECHNICAL.md are runtime evidence requirements, not records already populated by these design documents. Physical DDL/byte formats, migrations, host identity, process configuration, target measurements and independently authenticated observers are supplied by implementation/integration packages. Stateless modules prove absence of storage or effects. A separate Python reference is an analytic oracle only, never a second product ledger, authority issuer, scheduler or model spine.

## Refinement and compatibility rules

EXECUTION_SEMANTICS narrows the pilot implementation interpretation: general covariance regression solves Z*C=B; objective conflict extraction is inclusion-minimal and pays oracle cost; sequential policy value uses a separate history-conditioned estimand; numeric profiles have explicit conversion and digest boundaries; applicable thresholds combine by their stricter intersection; writer lease validity is distinct from business admission.

These corrections do not retroactively relabel old artifacts or change serialized bytes. A consumer that cannot prove compatibility returns unavailable or keeps the qualified predecessor. New profile/schema selection remains an independent integration decision. Preserve all existing Development, Activation and Evidence DAG predecessors and mandatory tests; this companion cannot waive them.

## Validation

```bash
python3 scripts/hepta-implementation-dossiers.py self-test
python3 scripts/hepta-implementation-dossiers.py generate-status --check
python3 scripts/hepta-implementation-dossiers.py verify
python3 scripts/hepta-technical-closure.py self-test
python3 scripts/hepta-technical-closure.py verify
```

`verify` additionally checks committed canonical module/lane/root/work-package bindings. For an exported companion without the whole repository, `verify-details --fixture-dir <directory>` checks only local document/index hashes, named cases and analytic/schema fixtures; it explicitly does not check repository integration. Neither mode proves semantic completeness or independently reviews the code it checks. Full native tests and existing source-head/synthetic-merge CI remain mandatory.

## Closure and authority boundary

The forty detail records and nine audit dispositions are a bounded, inspectable specification improvement. They are not a claim that every possible gap is closed. `DETAIL_GAPS.json` retains `allGapsClosed=false`, unresolved native handoff admission, actual deployment bindings, current CI and independent semantic review. Unknown gaps found during implementation are added, not suppressed to preserve a green count.

All nine `RDY-EXT-*` gates remain external and non-self-certifiable. Real models, future-calendar improvement, empirical biomimicry, target hardware, owner consent, independent acceptance and canary/selection/promotion/release require their own exact evidence. No document or oracle grants authority, executes a real effect, authorizes uncontrolled propagation, weakens branch protection or allows generator self-acceptance.
