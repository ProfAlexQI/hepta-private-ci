# neuron.runtime technical development guide

**Plan:** `HEPTA-GLOBAL-MODULAR-DEVELOPMENT-PLAN` v8.0.0

**Module:** `neuron.runtime`

**Owner:** `learning-platform`

**Deputy:** `inference-platform`

**Lifecycle:** `target`

**Source status:** `target_unmaterialized`

**Bootstrap work package:** `BIO-0-NEURON-INTUITION-CONTRACTS`

This stable document is the implementation guide for `neuron.runtime`. Normative identity, ownership, contract, data-authority and delivery facts remain in the canonical JSON registries. This guide explains how those facts are implemented and operated. Documentation readiness is not source implementation, activation, operator acceptance, promotion or release.

## 1. Identity, mission and ownership

Provide bounded temporal adaptive signals, eligibility traces and checkpoints without owning facts or effects.

The primary owner `learning-platform` controls changes inside the declared target roots and is accountable for correctness, backward compatibility, test evidence and rollback. The deputy `inference-platform` independently reviews public contracts, authority checks, persistence, migrations, concurrency, resource limits and activation behavior. A work package may narrow this scope but may not widen it. Cross-owner changes require an explicit co-owner or a separate integration package.

Plane `domain`, kind `adaptive_signal_runtime`, state model `stateful_projection` and architecture role `functional_substrate` define placement. The module may optimize locally, but cannot claim global optimality or absorb another module's durable facts.

## 2. Source binding and implementation status

Declared exclusive target roots:

- `codex-rs/hepta-neuron`

Existing declared roots at this exact source snapshot:

None.

Non-authoritative implementation evidence roots:

None.

Declared roots not yet present:

- `codex-rs/hepta-neuron`

`target_unmaterialized` is a source-location fact. For a target or unbound module, development begins with `BIO-0-NEURON-INTUITION-CONTRACTS` and materializes the declared root before source completion. Aggregate evidence roots are read-only decomposition evidence; they transfer no ownership and cannot become a second permanent implementation. A source move updates `MODULES.json`, `SOURCE_BINDINGS.json` and this guide in one candidate.

## 3. Boundary, responsibilities and non-goals

Direct dependencies:

- `platform.types`
- `cognitive.read`
- `inference.control`
- `learning.artifacts`

Authoritative write domains:

- `neuron_state_checkpoint`
- `eligibility_trace_checkpoint`

Explicitly denied capabilities:

- `fact_authority`
- `physical_effect`
- `runtime_model_install`

The module accepts only registered, bounded, versioned inputs. It rejects unknown critical fields and treats missing authority, stale revisions, scope mismatch and digest mismatch as hard failures. It never directly writes another owner's store. Cross-owner mutation follows local transaction, durable intent, outbox, destination deduplication, acknowledgement and fenced reconciliation.

Non-goals include becoming a general state store, bypassing the Codex execution spine, interpreting model prose as authority, minting an authority consumed by the same component, or converting qualification evidence into deployment authority. A façade may sequence modules but may not own their facts.

## 4. Internal architecture and component decomposition

The bounded components are:

- `frozen encoder adapter`
- `bounded recurrent state`
- `eligibility trace`
- `calibration and checkpoint layer`

Ingress validates identity, version, size, scope and revision before domain logic. The deterministic core receives typed values and is testable without network, filesystem or process-global state unless the module owns that boundary. State-bearing components use one transaction boundary per logical mutation. Publication occurs only after invariants and lineage checks pass.

Adapters translate one registered contract, verify final payload and grant immediately before the boundary, invoke one downstream capability, and map the observed terminal outcome. Queue acceptance or handler completion is never inferred as external success. Component interfaces support deterministic fixtures and fault injection.

Configuration is immutable for one process generation. Changes affecting authority, schema, compatibility, model identity, objective semantics or resource policy create a new revision or generation. Hidden mutable singletons, unbounded queues and implicit store fallback are prohibited.

## 5. Contracts, ports and compatibility

Produced contracts:

- `DomainRead::eligibility_trace_checkpointV1`
- `DomainRead::neuron_state_checkpointV1`
- `LocalModelRuntimeReceiptV1`
- `ModulePort::neuron.runtime::intelligence.control`
- `ModulePort::neuron.runtime::intuition.policy`
- `NeuronSignalReceiptV1`

Consumed contracts:

- `DomainRead::inference_receiptV1`
- `DomainRead::inference_requestV1`
- `DomainRead::inference_reservationV1`
- `DomainRead::learning_artifact_registryV1`
- `DomainRead::operator_sensor_core_registryV1`
- `ModulePort::cognitive.read::neuron.runtime`
- `ModulePort::inference.control::neuron.runtime`
- `ModulePort::learning.artifacts::neuron.runtime`
- `ModulePort::platform.types::neuron.runtime`
- `RunStartSnapshotV1`

Critical protocol schemas:

- `LocalModelRuntimeReceiptV1`
- `NeuronSignalReceiptV1`
- `RunStartSnapshotV1`

Every producer validates output before publication and binds semantic fields into the declared digest scope. Every consumer validates version, bounds, producer identity, scope and digest before use. Compatibility is additive only where registered; unknown critical fields are rejected. Contract identifiers, meaning and authority interpretation cannot change in place.

Rust types and canonical JSON represent identical semantics. Tests cover round trips, maximum bounds, missing fields, unknown fields, invalid enums, canonical ordering and digest stability. Error mapping preserves rejected, unavailable, timed out, indeterminate, quarantined and terminally failed outcomes.

## 6. Data authority, persistence and migrations

Owned authoritative or rebuildable domains:

- `eligibility_trace_checkpoint`
- `neuron_state_checkpoint`

Read-only data dependencies:

- `inference_receipt`
- `inference_request`
- `inference_reservation`
- `learning_artifact_registry`
- `operator_sensor_core_registry`

For every owned domain, this module is the only authoritative writer. Mutations are revision- or generation-bound, idempotent for identical semantics and conflicting for a reused identity with different content. Records bind source identity, schema revision, logical sequence and lineage sufficient for correction, deletion and revocation.

Migrations are deterministic and checksum-bound. Store open verifies required schema objects and integrity constraints before reads or writes. Migration failure leaves a recoverable predecessor. Rollback across a schema boundary restores compatible state with the binary.

Projection domains rebuild from declared sources and publish complete generations atomically. Projections never become sources of truth. Retention and deletion preserve lineage and prevent resurrection through indexes, caches, artifacts or backup restore.

## 7. Runtime, concurrency and transaction model

Central synchronous RPC on the local hot path is `false`. Bounded cached control input is `true`. A fallback is required: `true`.

Ingress enforces queue, payload, concurrency and deadline limits. Cancellation is observed at defined boundaries and cannot relabel a terminal state already being committed. Retries require a stable operation identity and equal semantic digest. Timeout at an external boundary becomes indeterminate absent verified terminal acknowledgement.

State transitions are monotonic within an attempt. A crash between authorization and terminal observation leaves pending or indeterminate state, never invented success. Reconciliation is fenced by authority epoch and predecessor identity. Concurrent writers use transactions or compare-and-swap; last-write-wins is forbidden for authoritative facts.

## 8. Failure semantics, recovery and rollback

Failures are classified as validation rejection, authority rejection, unavailable dependency, bounded timeout, storage failure, conflict, cancellation, indeterminate effect, integrity failure or internal invariant violation. Errors expose safe identifiers and digests, not raw secrets, provider payloads or untrusted content.

Startup validates configuration, schema and integrity, recovers incomplete local transactions, scans outbox state and gates readiness in that order. Integrity uncertainty, unknown schema or conflicting durable identity fails closed or quarantines. Optional context or advisory signals degrade only when fallback cannot widen authority.

Every state-changing package names a rollback predecessor and tests crash/reopen behavior. Rollback restores code, configuration and compatible state. External effects are never rolled back by assumption; they require acknowledgement, compensation or quarantine.

## 9. Security, privacy and threat controls

Owned threat entries:

- `local_model_identity_drift`
- `model_tokenizer_mismatch`
- `neuron_state_collapse`
- `neuron_state_explosion`

The posture is least authority, bounded input, typed contracts, digest binding and independent evidence. Sensitive values are redacted or represented by digests at evidence boundaries. Credentials never enter general logs, learning datasets, prompt factors or cross-module receipts. Authority is operation-bound, final-payload-bound, short-lived and revocation-aware.

Negative tests cover denied capabilities, cross-owner writes, stale or revoked grants, replay with payload drift, unknown fields, oversize input, scope escape, untrusted instruction escalation and secret/provider leakage. Security review is mandatory for new effect boundaries, persistence, network, model invocation or authority semantics.

## 10. Performance, capacity and hot-path policy

Implementing packages publish measurable latency, throughput, memory, storage growth, queue depth and recovery budgets. Bounds are enforced, not only observed. Backpressure rejects or sheds explicitly and never creates unbounded tasks or retries.

Hot paths avoid global locks, synchronous central control and full-store scans. Expensive verification uses bounded indexes, snapshots or staged slow paths. Caches bind revision and expiry and invalidate on revocation, correction, deletion or generation change. Benchmarks include steady state, cold start, maximum input, contention, degraded dependency and recovery.

## 11. Observability and operations

Structured events include module, operation or attempt identity, source revision, outcome class, duration, bounded resource use and safe digest references. Metrics include ingress, rejection, saturation, transaction conflicts, dependency latency, reconciliation backlog, integrity failures, fallback use and recovery duration.

Readiness means required dependencies, schema and integrity are verified; liveness only means progress is possible. Operator surfaces never expose raw secrets or unbounded payloads. Alerts cover sustained rejection, retry storms, aged pending/indeterminate state, integrity failure, capacity exhaustion, projection lag and rollback failure.

## 12. Verification and qualification

Minimum checks are exact source identity, source inventory, static verification, focused tests, package tests, all-target compilation, strict lint, clean worktree, exact-head execution and synthetic-merge execution. Stateful modules add migration, crash/reopen, corruption, idempotency, conflict and reconciliation. Adapters add revoked/stale grant, payload drift, timeout and indeterminate-outcome tests.

The implementing team cannot issue independent acceptance. Fixture success proves only the tested boundary at the exact candidate; it does not prove a production caller, physical effect, operator acceptance, promotion or release.

## 13. Implementation sequence and work packages

Applicable work packages:

- `BIO-0-NEURON-INTUITION-CONTRACTS`
- `NEU-1-LOCAL-MODEL-BAKEOFF`
- `NEU-2-TEMPORAL-SIGNAL-RUNTIME`
- `BIO-1-ELIGIBILITY-HOMEOSTASIS`

The bootstrap package is `BIO-0-NEURON-INTUITION-CONTRACTS`. Development, activation and evidence predecessor graphs are distinct and all are enforced. Contract-first work may run in parallel only with non-overlapping write paths and frozen semantics. Each PR carries one bounded envelope with contracts, domains, denied authorities, resources, rollback and stop conditions.

Source implementation completes only when the declared target root exists, public surfaces match registries, tests pass and exact-head plus merge-candidate evidence is current. Later planned packages may remain without invalidating documentation closure.

## 14. Activation, compatibility and retirement

Activation composes a named product caller through registered ports and verifies authority, configuration, resource and failure behavior. Shadow and qualification callers are not production callers. Source-complete modules remain inactive until activation predecessors and evidence gates pass.

Compatibility adapters are temporary. Retirement requires all named callers migrated, no old-path use, oracle parity where required, rehearsed rollback and independent acceptance. Retirement preserves historical evidence and durable-record interpretability.

## 15. Definition of module completion

Documentation completion requires this guide, exact registry references and closed-world validation. Source completion requires code in the declared root and candidate tests. Composition requires a named caller. Qualification requires current exact-candidate evidence. Acceptance, selection, promotion and release are separate externally governed states.

For `neuron.runtime`, this document grants no runtime, production, model, provider, tool, network, filesystem, secret, Matrix, fleet, acceptance, promotion or release authority.

### Work-package execution envelopes

#### `BIO-0-NEURON-INTUITION-CONTRACTS`

- State: `planned`; priority: `2`; parallel class: `contract_first_parallel`.
- Owner/deputy: `learning-platform` / `inference-platform`.
- Allowed write paths:
- `codex-rs/hepta-neuron/**`
- `codex-rs/hepta-intuition/**`
- `codex-rs/hepta-types/**`
- Development predecessors:
- `NDU-0-PREFERENCE-UTILITY-CONTRACTS`
- `OBJ-0-OBJECTIVE-CONTRACTS`
- `LRN-0-CAUSAL-LEARNING-CONTRACTS`
- `PIM-0-PROMPT-INTERVENTION-CONTRACTS`
- `HBO-0-BELLMAN-OPERATOR-CONTRACTS`
- Activation predecessors:
- `NDU-0-PREFERENCE-UTILITY-CONTRACTS`
- Required deliverables:
- `exact_source_identity`
- `source_inventory`
- `static_verification`
- `focused_tests`
- `package_tests`
- `all_target_check`
- `strict_lint`
- `clean_worktree`
- `exact_head_execution`
- `merge_candidate_execution`
- Stop conditions:
- `authority_violation`
- `base_drift`
- `claim_evidence_mismatch`
- `cross_owner_write`
- `unbounded_resource_or_retry`

#### `NEU-1-LOCAL-MODEL-BAKEOFF`

- State: `planned`; priority: `2`; parallel class: `external_evidence_coordinated`.
- Owner/deputy: `learning-platform` / `inference-platform`.
- Allowed write paths:
- `codex-rs/hepta-neuron/**`
- `codex-rs/hepta-infer-worker-host/**`
- Development predecessors:
- `BIO-0-NEURON-INTUITION-CONTRACTS`
- `INFER-V4-T5`
- `ART-1-LEARNING-ARTIFACT-REGISTRY`
- `INFER-V4-T4`
- Activation predecessors:
- `INFER-V4-T5`
- `ART-1-LEARNING-ARTIFACT-REGISTRY`
- `BIO-0-NEURON-INTUITION-CONTRACTS`
- Required deliverables:
- `exact_source_identity`
- `source_inventory`
- `static_verification`
- `focused_tests`
- `package_tests`
- `all_target_check`
- `strict_lint`
- `clean_worktree`
- `exact_head_execution`
- `merge_candidate_execution`
- Stop conditions:
- `authority_violation`
- `base_drift`
- `claim_evidence_mismatch`
- `cross_owner_write`
- `unbounded_resource_or_retry`

#### `NEU-2-TEMPORAL-SIGNAL-RUNTIME`

- State: `planned`; priority: `2`; parallel class: `contract_coordinated`.
- Owner/deputy: `learning-platform` / `inference-platform`.
- Allowed write paths:
- `codex-rs/hepta-neuron/**`
- Development predecessors:
- `NEU-1-LOCAL-MODEL-BAKEOFF`
- `LRN-1-DURABLE-EPISODE-LEDGER`
- `BIO-0-NEURON-INTUITION-CONTRACTS`
- Activation predecessors:
- `NEU-1-LOCAL-MODEL-BAKEOFF`
- `LRN-1-DURABLE-EPISODE-LEDGER`
- Required deliverables:
- `exact_source_identity`
- `source_inventory`
- `static_verification`
- `focused_tests`
- `package_tests`
- `all_target_check`
- `strict_lint`
- `clean_worktree`
- `exact_head_execution`
- `merge_candidate_execution`
- Stop conditions:
- `authority_violation`
- `base_drift`
- `claim_evidence_mismatch`
- `cross_owner_write`
- `unbounded_resource_or_retry`

#### `BIO-1-ELIGIBILITY-HOMEOSTASIS`

- State: `planned`; priority: `3`; parallel class: `contract_coordinated`.
- Owner/deputy: `learning-platform` / `inference-platform`.
- Allowed write paths:
- `codex-rs/hepta-neuron/**`
- `qa/learning/biomimicry/**`
- Development predecessors:
- `NEU-2-TEMPORAL-SIGNAL-RUNTIME`
- `C1-PROMPTED-MEMORY-RETRIEVAL-RANK`
- `BIO-0-NEURON-INTUITION-CONTRACTS`
- `NEU-1-LOCAL-MODEL-BAKEOFF`
- Activation predecessors:
- `ART-2-NEXT-SNAPSHOT-RELOAD-ROLLBACK`
- `NEU-2-TEMPORAL-SIGNAL-RUNTIME`
- Required deliverables:
- `exact_source_identity`
- `source_inventory`
- `static_verification`
- `focused_tests`
- `package_tests`
- `all_target_check`
- `strict_lint`
- `clean_worktree`
- `exact_head_execution`
- `merge_candidate_execution`
- `bounded_eligibility_trace`
- `lateral_inhibition`
- `homeostatic_target`
- `adaptive_threshold`
- `stability_under_replay`
- Stop conditions:
- `authority_violation`
- `base_drift`
- `claim_evidence_mismatch`
- `cross_owner_write`
- `unbounded_resource_or_retry`

<!-- BEGIN GENERATED EXACT REGISTRY PROJECTION -->
### Exact closed-world registry projection

This generated projection binds `neuron.runtime` to the current canonical contract, protocol, data, delivery and threat registries. The registries remain authoritative; this block is a digest-checked documentation projection.

**Produced contracts:**
- `DomainRead::eligibility_trace_checkpointV1`
- `DomainRead::neuron_state_checkpointV1`
- `LocalModelRuntimeReceiptV1`
- `ModulePort::neuron.runtime::intelligence.control`
- `ModulePort::neuron.runtime::intuition.policy`
- `NeuronCheckpointV1`
- `NeuronSignalReceiptV1`

**Consumed contracts:**
- `DomainRead::inference_receiptV1`
- `DomainRead::inference_requestV1`
- `DomainRead::inference_reservationV1`
- `DomainRead::learning_artifact_registryV1`
- `DomainRead::operator_sensor_core_registryV1`
- `GoldenFixtureManifestV1`
- `ModulePort::cognitive.read::neuron.runtime`
- `ModulePort::inference.control::neuron.runtime`
- `ModulePort::learning.artifacts::neuron.runtime`
- `ModulePort::platform.types::neuron.runtime`
- `RandomStreamManifestV1`
- `RunStartSnapshotV1`

**Typed protocols:**
- `GoldenFixtureManifestV1`
- `LocalModelRuntimeReceiptV1`
- `NeuronCheckpointV1`
- `NeuronSignalReceiptV1`
- `RandomStreamManifestV1`
- `RunStartSnapshotV1`

**Owned data domains:**
- `eligibility_trace_checkpoint`
- `neuron_checkpoint_v1`
- `neuron_state_checkpoint`

**Read data domains:**
- `golden_fixture_manifest_v1`
- `inference_receipt`
- `inference_request`
- `inference_reservation`
- `learning_artifact_registry`
- `operator_sensor_core_registry`
- `random_stream_manifest_v1`

**Work packages:**
- `BIO-0-NEURON-INTUITION-CONTRACTS`
- `BIO-1-ELIGIBILITY-HOMEOSTASIS`
- `NEU-1-LOCAL-MODEL-BAKEOFF`
- `NEU-2-TEMPORAL-SIGNAL-RUNTIME`

**Owned threats:**
- `local_model_identity_drift`
- `model_tokenizer_mismatch`
- `neuron_state_collapse`
- `neuron_state_explosion`

<!-- END GENERATED EXACT REGISTRY PROJECTION -->
