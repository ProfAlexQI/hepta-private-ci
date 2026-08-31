# P0.7b/B3 Secret, Matrix, Fleet, Operator, and Release Boundary Contract V1

**Package:** `P0.7b/B3_secret_matrix_fleet_operator_release_boundaries`  
**Plan:** `HEPTA-ARCHITECTURE-CONVERGENCE-V5`  
**Parent:** `24a2c1b733cc1d0f1288b39ffd42057dc6ade8ba` / tree `7ed08fb76eb8f0a30f3be926b66ff0d81fa46336`  
**State:** source implemented; exact-head, merge-candidate and independent qualification pending.  
**Authority:** none.

## 1. Scope and physical layout

B3 is split into one private shared kernel and five boundary-owned modules:

```text
physical_boundaries/governed/core.rs
physical_boundaries/governed/secret.rs
physical_boundaries/governed/matrix.rs
physical_boundaries/governed/fleet.rs
physical_boundaries/governed/operator.rs
physical_boundaries/governed/release.rs
```

Each public wrapper accepts exactly the capability class required by its physical kind. The shared core owns the adapter privately and performs final-payload validation, current authority verification, durable single-use claim, witness persistence and one adapter crossing. No module exposes a raw adapter accessor, restoration constructor or `into_parts()` escape.

B3 records only digest-bound external receipts. Repository source cannot issue an independent review, operator acceptance, promotion or release fact.

## 2. Common crossing order

For every B3 operation:

1. the owning domain records a durable logical operation;
2. all boundary-specific facts are finalized;
3. exact adapter payload bytes are built and rehashed;
4. the boundary-specific physical payload digest is derived;
5. the correct externally verified capability is rebound to the current runtime context;
6. current revocation revision and trusted time are checked;
7. a B0 token is issued and consumed for the exact operation/payload;
8. a durable single-use claim is committed;
9. the caller persists the verified-use witness;
10. the private adapter is crossed once.

Failure before step 10 prevents adapter entry. Failure after a successful claim never authorizes blind retry.

## 3. Secret operation

`SecretOperationIntent` binds:

- operation ID;
- opaque SecretRef digest;
- provider and profile identity;
- token family;
- purpose and audience;
- expected secret revision;
- operation deadline;
- exact final request payload digest and byte count.

Raw secret, private-key and signature bytes are absent from the intent, witness, outcome and error surface. A provider timeout or lost acknowledgement is indeterminate and must use status lookup/reconciliation.

## 4. Matrix send

`MatrixSendIntent` binds:

- operation ID;
- room ID and event ID;
- current Matrix identity generation;
- sender identity digest;
- durable outbox-envelope digest;
- homeserver-route digest;
- exact final event payload digest and byte count.

A queue acknowledgement is not terminal Matrix delivery success. Generation, room, event, route or payload drift fails before the adapter.

## 5. Fleet mutation

`FleetMutationIntent` binds:

- operation ID;
- registry revision;
- immutable release ID and release-identity digest;
- owner epoch;
- process generation;
- expected prior registry digest;
- exact final mutation payload digest and byte count.

Only `Authorized<FleetMutationCapability>` can enter this wrapper. Stale owner epoch, process generation, registry revision or prior-state evidence cannot reuse a token.

## 6. Operator acceptance

`OperatorAcceptanceIntent` is an intent to record an externally issued exact-candidate decision, not a decision issuer. It binds:

- exact candidate commit and tree;
- complete evidence-manifest digest;
- acceptance-policy revision;
- implementer identity digest;
- independent reviewer identity digest;
- review-challenge digest;
- issue and expiry time;
- exact external envelope digest and byte count.

The implementer and reviewer identity digests must differ. Source code, repository administration, a test fixture or the checked adapter cannot manufacture independent acceptance.

## 7. Release promotion

`ReleasePromotionIntent` binds:

- exact candidate commit and tree;
- release ID and promotion target;
- release manifest and artifact-set digests;
- SBOM digest;
- migration-compatibility digest;
- rollback-evidence digest;
- independent-review receipt digest;
- operator-acceptance receipt digest;
- release-policy revision;
- exact external promotion envelope digest and byte count.

A recorded external receipt is not automatically a release. A separate release authority must validate and publish the terminal release decision.

## 8. Recovery semantics

- Claim-store failure blocks the adapter.
- Witness persistence failure blocks the adapter but leaves the operation claimed; reconciliation or quarantine owns recovery.
- Adapter error after crossing is `Indeterminate` with a bounded normalized reason code.
- `RejectedNoCrossing` is descriptive and cannot silently release a claim.
- Secret and Matrix uncertain outcomes use lookup-only reconciliation.
- Fleet, operator and release uncertain outcomes require current-fence external reconciliation and must never be inferred from repository state.

## 9. Required tests

The exact candidate must execute tests proving:

1. all five physical kinds persist a witness before adapter entry;
2. boundary-specific revisions, identities and evidence alter the physical digest;
3. operator reviewer identity differs from the implementer;
4. secret intent serialization contains no raw secret material;
5. claim and witness failures block adapter entry;
6. post-crossing transport failure is indeterminate and not retried;
7. repository source keeps all runtime, writer, operator, promotion and release authority false;
8. capability-specific wrapper types prevent substituting a Fleet, operator or release capability for another class;
9. B0 rechecks runtime context, revocation, token time and final payload.

## 10. Qualification and activation

The exact source head and GitHub merge candidate must independently run non-empty attributable steps for source verification, Rust 1.95 formatting, focused/full tests, all-target check, strict Clippy, Cargo/Bazel lock coherence and clean worktree.

P0.7a, B0, B1a, B1b and B2 exact qualification remain activation predecessors. B4 must prove the complete no-bypass callsite inventory. Independent review, operator acceptance and release remain externally issued gates.

```text
runtime_registered=false
production_caller=false
production_writer=false
secret_operation=false
matrix_send=false
fleet_mutation=false
operator_acceptance=false
promotion=false
release=false
```
