# control.engineering technical development guide

**Plan:** `HEPTA-GLOBAL-MODULAR-DEVELOPMENT-PLAN` v8.0.0

**Module:** `control.engineering`

**Owner:** `developer-productivity`

**Deputy:** `architecture`

**Lifecycle:** `target`

**Source status:** `target_unmaterialized`

**Bootstrap work package:** `ECP-1-ENGINEERING-CONTROL-PLANE`

This stable document is the implementation guide for `control.engineering`. Normative identity, ownership, contract, data-authority and delivery facts remain in the canonical JSON registries. This guide explains how those facts are implemented and operated. Documentation readiness is not source implementation, activation, operator acceptance, promotion or release.

## 1. Identity, mission and ownership

Coordinate work envelopes and integration decisions without runtime authority or self-merge.

The primary owner `developer-productivity` controls changes inside the declared target roots and is accountable for correctness, backward compatibility, test evidence and rollback. The deputy `architecture` independently reviews public contracts, authority checks, persistence, migrations, concurrency, resource limits and activation behavior. A work package may narrow this scope but may not widen it. Cross-owner changes require an explicit co-owner or a separate integration package.

Plane `engineering`, kind `orchestrator`, state model `stateful_projection` and architecture role `engineering_control` define placement. The module may optimize locally, but cannot claim global optimality or absorb another module's durable facts.

## 2. Source binding and implementation status

Declared exclusive target roots:

- `tools/hepta-engineering-control`

Existing declared roots at this exact source snapshot:

None.

Non-authoritative implementation evidence roots:

None.

Declared roots not yet present:

- `tools/hepta-engineering-control`

`target_unmaterialized` is a source-location fact. For a target or unbound module, development begins with `ECP-1-ENGINEERING-CONTROL-PLANE` and materializes the declared root before source completion. Aggregate evidence roots are read-only decomposition evidence; they transfer no ownership and cannot become a second permanent implementation. A source move updates `MODULES.json`, `SOURCE_BINDINGS.json` and this guide in one candidate.

## 3. Boundary, responsibilities and non-goals

Direct dependencies:

- `kernel.evidence`

Authoritative write domains:

- `work_assignment_projection`
- `integration_decision`

Explicitly denied capabilities:

- `runtime_capability_issuance`
- `self_merge`

The module accepts only registered, bounded, versioned inputs. It rejects unknown critical fields and treats missing authority, stale revisions, scope mismatch and digest mismatch as hard failures. It never directly writes another owner's store. Cross-owner mutation follows local transaction, durable intent, outbox, destination deduplication, acknowledgement and fenced reconciliation.

Non-goals include becoming a general state store, bypassing the Codex execution spine, interpreting model prose as authority, minting an authority consumed by the same component, or converting qualification evidence into deployment authority. A façade may sequence modules but may not own their facts.

## 4. Internal architecture and component decomposition

The bounded components are:

- `work-envelope scheduler`
- `path-lease arbiter`
- `integration decision engine`
- `audit projection`

Ingress validates identity, version, size, scope and revision before domain logic. The deterministic core receives typed values and is testable without network, filesystem or process-global state unless the module owns that boundary. State-bearing components use one transaction boundary per logical mutation. Publication occurs only after invariants and lineage checks pass.

Adapters translate one registered contract, verify final payload and grant immediately before the boundary, invoke one downstream capability, and map the observed terminal outcome. Queue acceptance or handler completion is never inferred as external success. Component interfaces support deterministic fixtures and fault injection.

Configuration is immutable for one process generation. Changes affecting authority, schema, compatibility, model identity, objective semantics or resource policy create a new revision or generation. Hidden mutable singletons, unbounded queues and implicit store fallback are prohibited.

## 5. Contracts, ports and compatibility

Produced contracts:

- `DomainRead::integration_decisionV1`
- `DomainRead::work_assignment_projectionV1`

Consumed contracts:

- `DomainRead::qualification_evidenceV1`
- `ModulePort::kernel.evidence::control.engineering`
- `TopologyProposalV1`

Critical protocol schemas:

- `TopologyProposalV1`

Every producer validates output before publication and binds semantic fields into the declared digest scope. Every consumer validates version, bounds, producer identity, scope and digest before use. Compatibility is additive only where registered; unknown critical fields are rejected. Contract identifiers, meaning and authority interpretation cannot change in place.

Rust types and canonical JSON represent identical semantics. Tests cover round trips, maximum bounds, missing fields, unknown fields, invalid enums, canonical ordering and digest stability. Error mapping preserves rejected, unavailable, timed out, indeterminate, quarantined and terminally failed outcomes.

## 6. Data authority, persistence and migrations

Owned authoritative or rebuildable domains:

- `integration_decision`
- `work_assignment_projection`

Read-only data dependencies:

- `qualification_evidence`

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

- `self_review_or_self_merge`

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

- `DOC-0-CANONICAL-DOCUMENT-CONSOLIDATION`
- `DOC-1-V8-SEMANTIC-UPGRADE`
- `DOC-2-DEFAULT-BRANCH-SELECTION`
- `DOC-3A-SOURCE-BINDING-RECONCILIATION`
- `DOC-3B-MODULE-TECHNICAL-DOCUMENTS`
- `DOC-3C-MODULE-DOC-CLOSED-WORLD`
- `DOC-REGISTRY-CLOSED-WORLD`
- `ECP-1-ENGINEERING-CONTROL-PLANE`
- `SELF-1-CODE-CANDIDATE-PIPELINE`

The bootstrap package is `ECP-1-ENGINEERING-CONTROL-PLANE`. Development, activation and evidence predecessor graphs are distinct and all are enforced. Contract-first work may run in parallel only with non-overlapping write paths and frozen semantics. Each PR carries one bounded envelope with contracts, domains, denied authorities, resources, rollback and stop conditions.

Source implementation completes only when the declared target root exists, public surfaces match registries, tests pass and exact-head plus merge-candidate evidence is current. Later planned packages may remain without invalidating documentation closure.

## 14. Activation, compatibility and retirement

Activation composes a named product caller through registered ports and verifies authority, configuration, resource and failure behavior. Shadow and qualification callers are not production callers. Source-complete modules remain inactive until activation predecessors and evidence gates pass.

Compatibility adapters are temporary. Retirement requires all named callers migrated, no old-path use, oracle parity where required, rehearsed rollback and independent acceptance. Retirement preserves historical evidence and durable-record interpretability.

## 15. Definition of module completion

Documentation completion requires this guide, exact registry references and closed-world validation. Source completion requires code in the declared root and candidate tests. Composition requires a named caller. Qualification requires current exact-candidate evidence. Acceptance, selection, promotion and release are separate externally governed states.

For `control.engineering`, this document grants no runtime, production, model, provider, tool, network, filesystem, secret, Matrix, fleet, acceptance, promotion or release authority.

### Work-package execution envelopes

#### `DOC-0-CANONICAL-DOCUMENT-CONSOLIDATION`

- State: `source_implemented`; priority: `0`; parallel class: `serial_governance`.
- Owner/deputy: `developer-productivity` / `architecture`.
- Allowed write paths:
- `README.md`
- `docs/**`
- `scripts/hepta-docs.py`
- `.github/workflows/hepta-development-docs.yml`
- Development predecessors:
None.
- Activation predecessors:
None.
- Required deliverables:
- `single_human_development_document`
- `canonical_machine_registries`
- `legacy_inventory_zero`
- `generated_status_exact`
- `read_only_ci`
- Stop conditions:
- `authority_violation`
- `base_drift`
- `claim_evidence_mismatch`
- `cross_owner_write`
- `unbounded_resource_or_retry`

#### `DOC-1-V8-SEMANTIC-UPGRADE`

- State: `source_implemented`; priority: `0`; parallel class: `serial_governance`.
- Owner/deputy: `developer-productivity` / `architecture`.
- Allowed write paths:
- `README.md`
- `docs/**`
- `scripts/hepta-docs.py`
- `.github/workflows/hepta-development-docs.yml`
- Development predecessors:
- `DOC-0-CANONICAL-DOCUMENT-CONSOLIDATION`
- Activation predecessors:
- `DOC-0-CANONICAL-DOCUMENT-CONSOLIDATION`
- Required deliverables:
- `v8_single_commit_on_observed_default_base`
- `forty_module_registry`
- `ndu_prompt_learning_registries`
- `legacy_removal_count_at_least_139`
- `all_authority_flags_false`
- `exact_head_docs_verifier_pass`
- `merge_candidate_docs_verifier_pass`
- Stop conditions:
- `authority_violation`
- `base_drift`
- `claim_evidence_mismatch`
- `cross_owner_write`
- `unbounded_resource_or_retry`

#### `DOC-2-DEFAULT-BRANCH-SELECTION`

- State: `blocked_external`; priority: `0`; parallel class: `serial_governance`.
- Owner/deputy: `developer-productivity` / `architecture`.
- Allowed write paths:
None.
- Development predecessors:
- `DOC-1-V8-SEMANTIC-UPGRADE`
- `DOC-3C-MODULE-DOC-CLOSED-WORLD`
- Activation predecessors:
- `DOC-1-V8-SEMANTIC-UPGRADE`
- `DOC-3C-MODULE-DOC-CLOSED-WORLD`
- Required deliverables:
- `exact_head_docs_verifier_pass`
- `merge_candidate_docs_verifier_pass`
- `independent_review`
- `canonical_selection`
- `default_branch_contains_only_v8_document_set`
- Stop conditions:
- `authority_violation`
- `base_drift`
- `claim_evidence_mismatch`
- `cross_owner_write`
- `unbounded_resource_or_retry`

#### `DOC-3A-SOURCE-BINDING-RECONCILIATION`

- State: `source_implemented`; priority: `0`; parallel class: `serial_governance`.
- Owner/deputy: `developer-productivity` / `architecture`.
- Allowed write paths:
- `docs/modules/MODULES.json`
- `docs/modules/SOURCE_BINDINGS.json`
- `docs/delivery/**`
- `docs/governance/DOCUMENT_SYSTEM.json`
- Development predecessors:
- `DOC-1-V8-SEMANTIC-UPGRADE`
- Activation predecessors:
- `DOC-1-V8-SEMANTIC-UPGRADE`
- Required deliverables:
- `forty_module_source_classification`
- `existing_root_reconciliation`
- `target_root_missing_state`
- `bootstrap_package_binding`
- `browser_source_conflict_reclassified`
- Stop conditions:
- `authority_violation`
- `base_drift`
- `claim_evidence_mismatch`
- `cross_owner_write`
- `unbounded_resource_or_retry`

#### `DOC-3B-MODULE-TECHNICAL-DOCUMENTS`

- State: `source_implemented`; priority: `0`; parallel class: `serial_governance`.
- Owner/deputy: `developer-productivity` / `architecture`.
- Allowed write paths:
- `README.md`
- `docs/DEVELOPMENT.md`
- `docs/modules/**`
- Development predecessors:
- `DOC-3A-SOURCE-BINDING-RECONCILIATION`
- Activation predecessors:
- `DOC-3A-SOURCE-BINDING-RECONCILIATION`
- Required deliverables:
- `forty_stable_module_guides`
- `module_specific_contract_inventory`
- `module_specific_data_inventory`
- `module_specific_failure_and_test_model`
- `no_versioned_module_plan_files`
- Stop conditions:
- `authority_violation`
- `base_drift`
- `claim_evidence_mismatch`
- `cross_owner_write`
- `unbounded_resource_or_retry`

#### `DOC-3C-MODULE-DOC-CLOSED-WORLD`

- State: `source_implemented`; priority: `0`; parallel class: `serial_governance`.
- Owner/deputy: `developer-productivity` / `architecture`.
- Allowed write paths:
- `docs/**`
- `scripts/hepta-docs.py`
- `scripts/hepta-module-docs.py`
- `.github/workflows/hepta-development-docs.yml`
- Development predecessors:
- `DOC-3B-MODULE-TECHNICAL-DOCUMENTS`
- Activation predecessors:
- `DOC-3B-MODULE-TECHNICAL-DOCUMENTS`
- Required deliverables:
- `module_document_index`
- `module_source_binding_index`
- `closed_world_module_validator`
- `canonical_path_closure`
- `exact_head_and_merge_candidate_validation`
- Stop conditions:
- `authority_violation`
- `base_drift`
- `claim_evidence_mismatch`
- `cross_owner_write`
- `unbounded_resource_or_retry`

#### `DOC-REGISTRY-CLOSED-WORLD`

- State: `source_implemented`; priority: `2`; parallel class: `contract_coordinated`.
- Owner/deputy: `developer-productivity` / `architecture`.
- Allowed write paths:
- `docs/**`
- Development predecessors:
- `DOC-1-V8-SEMANTIC-UPGRADE`
- `DOC-0-CANONICAL-DOCUMENT-CONSOLIDATION`
- Activation predecessors:
- `DOC-1-V8-SEMANTIC-UPGRADE`
- `DOC-0-CANONICAL-DOCUMENT-CONSOLIDATION`
- Required deliverables:
- `exact_source_identity`
- `static_verification`
- `focused_tests`
- `clean_worktree`
- Stop conditions:
- `authority_violation`
- `base_drift`
- `claim_evidence_mismatch`
- `cross_owner_write`
- `unbounded_resource_or_retry`

#### `ECP-1-ENGINEERING-CONTROL-PLANE`

- State: `planned`; priority: `2`; parallel class: `independent_engineering_tooling`.
- Owner/deputy: `developer-productivity` / `architecture`.
- Allowed write paths:
- `tools/hepta-engineering-control/**`
- Development predecessors:
- `DOC-2-DEFAULT-BRANCH-SELECTION`
- Activation predecessors:
- `DOC-2-DEFAULT-BRANCH-SELECTION`
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

#### `SELF-1-CODE-CANDIDATE-PIPELINE`

- State: `planned`; priority: `4`; parallel class: `independent_engineering_tooling`.
- Owner/deputy: `developer-productivity` / `architecture`.
- Allowed write paths:
- `tools/hepta-engineering-control/**`
- `qa/self-iteration/**`
- Development predecessors:
- `ECP-1-ENGINEERING-CONTROL-PLANE`
- `LONG-3-UNLEARNING-NON-RESURRECTION`
- `ART-2-NEXT-SNAPSHOT-RELOAD-ROLLBACK`
- Activation predecessors:
- `ECP-1-ENGINEERING-CONTROL-PLANE`
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
- `patch_and_test_generation`
- `bounded_work_envelope`
- `independent_review_required`
- `no_self_merge`
- `rollback_plan`
- Stop conditions:
- `authority_violation`
- `base_drift`
- `claim_evidence_mismatch`
- `cross_owner_write`
- `unbounded_resource_or_retry`

<!-- BEGIN GENERATED EXACT REGISTRY PROJECTION -->
### Exact closed-world registry projection

This generated projection binds `control.engineering` to the current canonical contract, protocol, data, delivery and threat registries. The registries remain authoritative; this block is a digest-checked documentation projection.

**Produced contracts:**
- `DomainRead::integration_decisionV1`
- `DomainRead::work_assignment_projectionV1`
- `GoldenFixtureManifestV1`
- `IterationEnvelopeV1`

**Consumed contracts:**
- `CandidateEvaluationReceiptV1`
- `DomainRead::qualification_evidenceV1`
- `IndependentDecisionReceiptV1`
- `IterationCandidateV1`
- `ModulePort::kernel.evidence::control.engineering`
- `PlasticityProposalV1`
- `TopologyProposalV1`

**Typed protocols:**
- `CandidateEvaluationReceiptV1`
- `GoldenFixtureManifestV1`
- `IndependentDecisionReceiptV1`
- `IterationCandidateV1`
- `IterationEnvelopeV1`
- `PlasticityProposalV1`
- `TopologyProposalV1`

**Owned data domains:**
- `golden_fixture_manifest_v1`
- `integration_decision`
- `iteration_envelope_v1`
- `work_assignment_projection`

**Read data domains:**
- `candidate_evaluation_receipt_v1`
- `independent_decision_receipt_v1`
- `iteration_candidate_v1`
- `plasticity_proposal_v1`
- `qualification_evidence`
- `topology_proposal_v1`

**Work packages:**
- `DOC-0-CANONICAL-DOCUMENT-CONSOLIDATION`
- `DOC-1-V8-SEMANTIC-UPGRADE`
- `DOC-2-DEFAULT-BRANCH-SELECTION`
- `DOC-3A-SOURCE-BINDING-RECONCILIATION`
- `DOC-3B-MODULE-TECHNICAL-DOCUMENTS`
- `DOC-3C-MODULE-DOC-CLOSED-WORLD`
- `DOC-3D-ADAPTIVE-ALGORITHM-DOC-CLOSED-WORLD`
- `DOC-REGISTRY-CLOSED-WORLD`
- `ECP-1-ENGINEERING-CONTROL-PLANE`
- `SELF-1-CODE-CANDIDATE-PIPELINE`

**Owned threats:**
- `self_review_or_self_merge`

<!-- END GENERATED EXACT REGISTRY PROJECTION -->
