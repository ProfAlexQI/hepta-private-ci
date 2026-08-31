# P0.7b/B1b Model Invocation Verified-Use Contract V1

**Package:** `P0.7b/B1b_model_invocation_verified_use_boundary`  
**Plan:** `HEPTA-ARCHITECTURE-CONVERGENCE-V5`  
**Parent:** `537394a0067d204b215db8bee3de533494535481` / tree `fd0f84d73504507078cecfcc1043490ca0856187`  
**State:** source implemented; exact-head, merge-candidate and independent qualification pending.  
**Authority:** none.

## 1. Scope

B1b adds one checked model-submission seam on top of the common B0 verified-use kernel. It does not register a product caller, select a provider, enable a listener, invoke a real model, mutate Memory/KG, grant external-effect authority, or issue operator, promotion or release receipts.

The checked seam is `CheckedModelInvocation::invoke_once`. Its raw adapter is a private field and the public API has no adapter accessor, restoration constructor or `into_parts()` escape.

## 2. Required ordering

For every physical model request, the caller must execute this sequence:

1. append or otherwise durably own the logical operation intent;
2. finish provider, model, endpoint, routing, response, tool and streaming selection;
3. build the exact final wire bytes;
4. validate byte count and wire SHA-256 against `ModelInvocationIntent`;
5. derive `ModelInvocationIntent::physical_payload_sha256`, which binds all final route and payload facts;
6. reverify `Authorized<ModelInvocationCapability>` against the current runtime authority context and revocation revision;
7. obtain and consume a non-cloneable, non-serializable B0 token;
8. commit the durable single-use claim;
9. commit the verified-use witness in caller-owned durable storage;
10. invoke the private model adapter once.

The adapter cannot be called by this boundary before steps 1–9 succeed.

## 3. Bound final facts

The physical payload digest length-delimits and binds:

- operation ID;
- secret-free provider ID;
- model ID;
- canonical endpoint identity digest;
- routing-policy digest;
- exact wire-payload digest and byte count;
- request content type;
- response-contract digest;
- optional tool-contract digest;
- streaming mode.

Raw prompts, tool arguments, images, credentials and model responses are not persisted by this contract. A successful response is represented only by its SHA-256 and bounded byte count.

## 4. Failure and recovery semantics

- Payload, route, context, operation, revision, fence or expiry drift fails before adapter invocation.
- Claim-store failure fails closed and prevents adapter invocation.
- Witness persistence failure prevents adapter invocation. Because the claim has already committed, ordinary retry is prohibited; reconciliation or quarantine owns recovery.
- A transport failure returned after adapter entry becomes `Indeterminate` with a bounded normalized reason code.
- This API exposes no automatic retry method. A later B4 inventory must prove that product code cannot bypass the checked seam.
- `RejectedNoDispatch` is descriptive only. The local claim remains consumed; any later retry requires a separately reviewed recovery protocol rather than replaying this call.

## 5. Required negative tests

The exact candidate must execute tests proving:

1. witness persistence completes before the adapter is called;
2. changed wire bytes fail before claim and adapter;
3. witness persistence failure blocks the adapter after the claim;
4. transport failure is classified `Indeterminate` and is not retried;
5. claim rejection blocks the adapter;
6. current revocation revision, runtime context, token expiry and final payload are rechecked by B0;
7. the raw adapter cannot be extracted through the public API.

## 6. Qualification gates

An exact source head and its GitHub merge candidate must independently run non-empty, attributable steps for:

- duplicate-key-safe source verifier;
- Rust 1.95 formatting;
- focused B1b tests;
- complete `codex-hepta-contracts` tests;
- locked all-target check;
- strict Clippy with `-D warnings`;
- Cargo/Bazel lock coherence;
- clean worktree and retained evidence.

Queued, cancelled, runner-zero, empty-step, source-only or stale evidence is not PASS. Independent exact-candidate review remains separate from executable qualification.

## 7. Authority boundary

```text
runtime_registered=false
production_caller=false
production_writer=false
model_invocation=false
provider_dispatch=false
external_effect=false
operator_acceptance=false
promotion=false
release=false
```
