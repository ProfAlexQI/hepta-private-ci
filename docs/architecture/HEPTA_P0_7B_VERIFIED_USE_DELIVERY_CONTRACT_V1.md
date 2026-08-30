# Hepta P0.7b Verified-Use Delivery Contract V1

**Package:** `P0.7b_verified_physical_capability_closure`  
**Current slice:** `B0_verified_use_kernel`  
**Contract revision:** `1.1.0` / kernel schema `v2`  
**Plan:** `HEPTA-ARCHITECTURE-CONVERGENCE-V5`  
**Status:** normative source contract; runtime/effect/production authority remains false.  
**Date:** 2026-08-30

## 1. Purpose and claim boundary

A broad `Authorized<C>` capability is necessary but insufficient at an irreversible boundary. B0 binds one exact operation and final payload to current authority, trusted time and an atomic durable single-use claim. It does not register a caller, invoke a model, dispatch a provider, execute a tool, connect a network destination, mutate an external filesystem, operate on a secret, send Matrix traffic, mutate fleet state, accept an operator, promote or release.

## 2. Closed capability kinds

`CognitiveStateWrite`, `ModelInvocation`, `ProviderDispatch`, `ExternalEffect`, `ToolProcessSpawn`, `OutboundNetworkConnect`, `ExternalFilesystemMutation`, `SecretOperation`, `MatrixSend`, `FleetMutation`, `OperatorAcceptance` and `ReleasePromotion` form the closed enum. No unknown or stringly typed kind is accepted.

## 3. Trust separation

The final payload builder, authority verifier, trusted clock, durable claim store and physical adapter are separate roles:

```text
final payload -> broad per-use verification -> current physical verifier
             -> VerifiedUseToken<C>
             -> trusted clock + final boundary re-verification
             -> PhysicalUseClaimStore::claim_once
             -> VerifiedUseBoundaryPermit<C>
             -> physical adapter -> boundary witness / later effect receipt
```

No adapter may mint the capability or token it consumes. A local allow-all verifier or in-memory claim store is qualification-only and cannot be production authority.

## 4. Request and trusted clock

The caller supplies `kind`, `operation_id`, `final_payload_sha256`, `runtime_authority_context`, `expected_revocation_revision` and an expiry upper bound. The caller does **not** supply verification time, boundary time or a claimed current revision. `TrustedPhysicalClock` supplies both observed instants. A zero, rejected or rolled-back trusted clock read fails closed.

Issuance validates kind/action equality, external lease presence, the existing `verify_capability_use` subject/generation/grant/epoch/fence/expiry checks, the requested expiry bound, current physical verification, exact revocation revision and verifier validity. Effective expiry is the minimum of caller upper bound, external lease expiry and verifier validity.

## 5. Token and final boundary

`VerifiedUseToken<C>` has a private constructor, is non-cloneable, non-serializable and consumed by value. `PhysicalUseFinalCheck` contains only the exact kind, operation ID, final payload digest and runtime context. It contains no caller-supplied time or current revision.

`consume_at_boundary` performs final boundary re-verification using the same exact request and a fresh trusted clock read. Capability kind drift, operation ID drift, final payload drift, runtime-context drift, revocation revision drift, clock rollback, expiry or verifier denial fails before any durable claim.

## 6. Durable single-use claim

`PhysicalUseClaimStore::claim_once` is an atomic durable uniqueness boundary. Its operation-scope digest binds capability kind plus operation ID; its claim digest additionally binds final payload SHA-256. The store must distinguish:

- exact replay: `AlreadyClaimed`;
- same operation with changed payload: `OperationPayloadConflict`;
- durability or availability failure: no permit;
- malformed store receipt: no permit.

A crash after claim success is indeterminate and lookup or reconciliation only; it must not issue another permit. Rust move semantics alone are not accepted as global single-use evidence.

## 7. Boundary permit and witness

Only a successful current-authority check, trusted time read and durable claim yields `VerifiedUseBoundaryPermit<C>`. The permit is non-cloneable and non-serializable. It exposes the exact operation and payload identity needed by the adapter and is consumed at the physical seam.

`VerifiedUseWitness` binds schema, kind, action, operation, final payload, runtime-context digest, revocation revision, issuance and final verifier receipts, verification/expiry/crossing times, token digest, claim key, claim revision and claim-store receipt. It carries `effect_completed=false`: a committed pre-crossing claim is not proof of external effect completion. Completion requires a separate provider, device or destination acknowledgement and reconciliation contract.

## 8. Error and retry rules

Invalid revision or window, kind/action mismatch, local-only authority, broad-authority denial, trusted-clock failure, issuance or final verifier denial, final-fact drift, expiry, claim replay/conflict/unavailability and receipt-integrity drift are distinct fail-closed outcomes. They may not be collapsed into success or blind retry.

## 9. Required tests

Positive tests prove issuance plus final verifier call, two trusted clock reads, one durable claim and one valid witness. Negative tests prove exact replay rejection, same-operation/different-payload conflict, final revision drift, clock rollback or expiry, operation/payload/context/kind drift, local-authority rejection, claim-store unavailability and corrupt claim-receipt rejection.

Static checks prove the token and permit have no `Clone`, `Copy`, `Serialize` or `Deserialize`; constructors remain private; successful boundary entry consumes `self`; `PhysicalUseFinalCheck` contains no caller current revision or time; and B0 introduces no product caller.

## 10. Executable qualification

Exact-head and merge-candidate jobs must execute non-empty identity, source verifier, Rust 1.95 format, focused and full tests, locked all-target check, strict Clippy, Bazel-lock and clean-worktree steps. Evidence must bind event, source head, merge candidate, workflow identity, run/job/runner identity, step conclusions, toolchain and artifact digest. A queued job, `runner_id=0`, empty steps, source-only verifier or checked-in positive run fact is not qualification.

## 11. Exit condition

B0 becomes a package-closed candidate only after exact-head and merge-candidate gates pass and an independent reviewer accepts the final candidate. P0.7a must be qualified before activation. The package posture is normative: runtime, effect, production, operator, promotion and release authority remain false.
