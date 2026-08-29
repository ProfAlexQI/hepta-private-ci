# Hepta P0.7b Verified-Use Delivery Contract V1

**Package:** `P0.7b_verified_physical_capability_closure`  
**Current slice:** `B0_verified_use_kernel`  
**Plan:** `HEPTA-ARCHITECTURE-CONVERGENCE-V5`  
**Status:** normative source contract; runtime/effect/production authority remains false.  
**Date:** 2026-08-30

## 1. Purpose

The current authority kernel admits typed capability classes and revalidates a broad capability before selected uses. That is necessary but insufficient at an irreversible boundary because the value is not tied to the final operation payload and is reusable. B0 adds the common final-payload gate consumed by all later model, provider, tool, network, filesystem, secret, Matrix, fleet, operator and release adapters.

B0 does not register a caller, open a listener, dispatch a provider, invoke a model, execute a tool, connect a network destination, mutate a filesystem, read a secret, send Matrix traffic, mutate fleet state, accept an operator or promote a release.

## 2. Terms

- **Broad capability:** `Authorized<C>`, already admitted for a capability action. It remains insufficient for an irreversible crossing.
- **Physical capability kind:** a closed enum identifying the actual boundary class.
- **Final payload:** the complete bytes or canonical structure that will cross the boundary, represented at B0 by its SHA-256 digest.
- **Operation ID:** a stable typed ID for exactly one logical boundary operation.
- **Revocation revision:** a non-zero monotonic revision supplied by the external/current authority source.
- **Verified-use token:** a private-constructor, non-cloneable, non-serializable value for one operation and one final payload.
- **Consumption witness:** serializable evidence that the token was rechecked and consumed at the boundary. The witness is not an authority grant and cannot be replayed as a token.

## 3. Trust and issuance boundary

The boundary adapter and the authority verifier are separate roles.

```text
final payload builder
        |
        v
physical-use request
        |
        v
current authority verifier
        |
        v
VerifiedUseToken<C>
        |
        v
separate boundary adapter consumes token by value
```

The verifier must not trust issuer-supplied “current” facts without checking its own authority source. The consumer must not construct a token or implement a local allow-all verifier in production composition. B0 exposes traits and value contracts only; B1–B3 own concrete verifier and adapter placement.

## 4. Closed physical capability kinds

| Kind | Required broad action | Boundary-specific final facts added by later slices |
|---|---|---|
| `ModelInvocation` | `InvokeModel` | model artifact, policy, prompt/request digest, provider route |
| `ProviderDispatch` | `DispatchProvider` | provider namespace, operation key, request digest |
| `ExternalEffect` | `ExternalEffect` | canonical effect intent and idempotency contract |
| `ToolProcessSpawn` | `ExternalEffect` | executable, argv, cwd, environment, sandbox, approval |
| `OutboundNetworkConnect` | `ExternalEffect` | protocol, DNS/IP/destination, proxy and request |
| `ExternalFilesystemMutation` | `ExternalEffect` | canonical path, mount/device, no-follow and prior state |
| `SecretOperation` | `ExternalEffect` | opaque SecretRef, purpose, audience and expected revision |
| `MatrixSend` | `ExternalEffect` | room, event, payload and Matrix generation |
| `FleetMutation` | `MutateFleet` | registry revision, release and process generation |
| `OperatorAcceptance` | `AcceptOperator` | candidate, evidence manifest and reviewer identity |
| `ReleasePromotion` | `PromoteRelease` | accepted candidate, release manifest, SBOM and rollback evidence |

No catch-all, stringly typed or unknown kind is permitted by the Rust API.

## 5. Request contract

The request passed to `verify_physical_capability_use` contains:

```text
kind
operation_id
final_payload_sha256
runtime_authority_context
expected_revocation_revision
verified_at_unix_seconds
requested_expires_at_unix_seconds
```

Required validation order:

1. schema/type invariants;
2. kind-to-action equality;
3. external-lease requirement for irreversible actions;
4. existing `verify_capability_use` checks for subject, generation, grant, epochs, fence and lease expiry;
5. requested time window is non-empty and does not exceed the external lease;
6. current authority verifier checks the final request;
7. returned current revocation revision equals the expected revision;
8. verifier receipt digest is present;
9. verifier validity extends beyond verification time;
10. effective token expiry is the minimum of requested expiry, verifier validity and external lease expiry.

No token is returned after any failed step.

## 6. Token contract

`VerifiedUseToken<C>`:

- has a private constructor;
- carries a `PhantomData<C>` capability type;
- is not `Clone` or `Copy`;
- is not Serde serializable or deserializable;
- exposes inspection getters only where needed for boundary composition;
- cannot change kind, operation, payload, context, revision or times;
- must be consumed by value.

The Rust type system prevents reuse after a successful consuming call within one safe execution path. B4 adds a static inventory proving that production call sites cannot bypass the constructor or consumption boundary.

## 7. Final boundary check

The consumer supplies a `PhysicalUseFinalCheck` built from the actual operation about to cross the boundary:

```text
kind
operation_id
final_payload_sha256
runtime_authority_context
current_revocation_revision
crossed_at_unix_seconds
```

Consumption rejects:

- capability kind drift;
- operation ID drift;
- final payload drift;
- runtime-context digest drift;
- revocation revision drift;
- crossing before verification;
- crossing at or after expiry.

On success it returns `VerifiedUseWitness` and destroys the token by move.

## 8. Witness contract

The witness binds:

```text
schema_version
kind
action
operation_id
final_payload_sha256
runtime_authority_context_sha256
revocation_revision
verified_at_unix_seconds
expires_at_unix_seconds
crossed_at_unix_seconds
verifier_receipt_sha256
token_sha256
witness_sha256
```

The witness digest uses an explicit domain and length-framed fields. The witness is immutable source evidence only. It cannot mint `Authorized<C>` or `VerifiedUseToken<C>`.

Later adapters persist the witness next to the boundary-specific intent/receipt. Raw prompt text, credentials, secret bytes, Authorization headers and unrestricted file content are prohibited.

## 9. Error taxonomy

B0 distinguishes at least:

- invalid/zero revocation revision;
- invalid verification window;
- physical kind/action mismatch;
- external authority required;
- requested window exceeds authority lease;
- authority verification rejection;
- revocation revision drift;
- verifier validity expired;
- final kind/operation/payload/context/revision drift;
- crossing before verification;
- token expired.

Callers must not collapse these errors into success, terminal failure or automatic retry. B1–B3 map them into their own durable state machines.

## 10. Test matrix

### Positive

- exact external capability, runtime context, operation, payload, revision and bounded time issue one token;
- exact final check consumes it and produces a stable witness;
- witness digest changes when any bound field changes.

### Negative

- kind/action mismatch is rejected before the physical verifier;
- local broad capability is rejected;
- existing subject/generation/epoch/fence/expiry checks remain active;
- requested expiry beyond the lease is rejected;
- verifier revision mismatch is rejected;
- verifier validity at/before verification time is rejected;
- operation, payload, context, revision or kind drift at consumption is rejected;
- consumption before verification or at expiry is rejected.

### Static source checks

- token derives neither clone/copy nor Serde traits;
- token constructor is not public;
- successful boundary function takes `self`;
- request and final check each contain operation, payload, runtime context and revision;
- B0 introduces no product caller or adapter registration;
- all authority flags in the ledger remain false.

## 11. Executable qualification

Required exact-head and merge-candidate steps:

1. print exact commit/tree/ref/event and reject ambiguous checkout;
2. validate V5 current-plan and gap-ledger JSON with duplicate-key rejection;
3. run the V5 source verifier;
4. Rust 1.95 formatting for all touched Rust files;
5. focused verified-use tests through the repository `just test` entrypoint;
6. complete `codex-hepta-contracts` tests;
7. locked all-target check;
8. strict Clippy with `-D warnings`;
9. clean-worktree verification;
10. emit an evidence artifact containing exact identity and step results.

A queued job, empty job or source verifier alone does not advance B0 beyond `source_implemented`/`source_verified`.

## 12. Rollback and compatibility

B0 is additive. Rollback removes the module, exports, workflow, verifier and V5 source documents without migrating durable state because B0 owns no database schema and registers no runtime caller.

Existing `CapabilityUseVerifier` and `verify_capability_use` remain the broad per-use primitive. B0 composes them rather than creating a competing authority system. Later callers migrate one physical boundary at a time and may not remove the old wrapper until B4 proves all bypass paths closed.

## 13. Exit condition

B0 reaches `PACKAGE_CLOSED_CANDIDATE` only when:

- all required source/API/negative facts exist;
- exact-head and merge-candidate executable gates are non-empty and green;
- the PR changes no product call site;
- P0.7a is at least `qualified_exact` before B0 is treated as activatable;
- an independent reviewer confirms the token cannot be cloned, serialized, constructed or consumed outside the intended boundary;
- runtime, effect, production, operator, promotion and release authority remain false.
