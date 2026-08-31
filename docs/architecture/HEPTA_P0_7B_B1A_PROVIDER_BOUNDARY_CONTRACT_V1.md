# Hepta V5 P0.7b/B1a Provider Verified-Use Boundary Contract V1

## Status

This document freezes the source contract for the provider half of
`P0.7b/B1_model_provider_boundaries` in
`HEPTA-ARCHITECTURE-CONVERGENCE-V5`.

B1a is stacked on the current B0 verified-use kernel. It is source-only and
activation-blocked until P0.7a and B0 have exact-head and merge-candidate
qualification with non-empty runner steps and independent review. It does not
advance `HEPTA_CURRENT_PLAN.json`, which must continue to identify B0 while that
upstream qualification is pending.

B1a does not close the model-invocation half of B1. It does not register a
runtime caller or make a provider available to the product.

## Source layout

The public module remains `checked_provider_operation.rs`. Complete Rust items
are stored in adjacent `checked_provider_operation_parts/*.rs` files and
assembled with module-scope `include!` directives. The focused integration test
uses the same complete-item split. This is source packaging only: it creates no
dynamic loading, generated runtime code, feature bypass, second authority path
or product caller.

## Physical boundary

A physical provider send must use the checked
`ProviderOperationCoordinator`. The lower-level provider operation coordinator
remains crate-private and is not a product API. The raw provider coordinator is
not root-exported. The checked wrapper exposes no public raw-provider accessor,
restoration constructor or extraction method.

The checked boundary requires two distinct externally verified capabilities:

1. `ProviderDispatchCapability` for crossing the provider transport boundary;
2. `ExternalEffectCapability` for the externally observable effect.

The two capabilities must bind the same Agent, generation, authority epoch,
owner epoch and fencing token as the durable operation record. Distinct signed
grant digests are permitted because the scopes are distinct.

The exact final wire payload is built before either B0 token is issued. The
payload SHA-256 must equal the immutable provider intent digest. There is no
public checked no-payload dispatch method.

For both capability kinds the boundary performs:

```text
current broad-capability verification
-> exact operation + final-payload verification
-> trusted-clock verification window
-> final physical re-verification
-> durable single-use claim
-> digest-bound verified-use witness
```

The time source is supplied as a callable owned by the composition root. The
checked API does not accept an ordinary `observed_at_unix_seconds` value.

## Durable claim adapter

B0 claim request types remain internal to the contracts crate in this tranche.
The checked boundary projects each exact request into a public callback using:

```text
physical capability kind
operation-scope SHA-256
payload-bound claim SHA-256
token SHA-256
full claim-request SHA-256
trusted claim time
```

A successful callback must mean that the claim is durably committed and that
one operation scope can never be successfully claimed twice. It returns a
non-zero monotonically increasing claim revision and a store-receipt SHA-256.
The checked boundary converts that result into the B0 claim receipt and rejects
receipt drift.

## Pair persistence and crossing order

After both claims succeed, the caller must persist the two complete
`VerifiedUseWitness` values in one caller-owned transaction. The raw provider
adapter is invoked only after that pair persistence returns success.

The mandatory order is:

```text
final payload
-> provider-dispatch claim
-> external-effect claim
-> atomic witness-pair persistence
-> raw adapter dispatch
```

Witnesses prove committed pre-crossing claims. They deliberately set
`effect_completed=false` and are not provider success receipts.

## Failure and recovery semantics

All failures before raw adapter dispatch are fail-closed and perform no
provider call.

Two bounded partial pre-send states are explicitly represented rather than
hidden:

- provider-dispatch claim committed, external-effect claim failed;
- both claims committed, witness-pair persistence failed.

Both states are quarantine/no-blind-retry outcomes. A later retry is rejected
by the durable claim store. Their crash/reopen adoption and operator-visible
settlement are owned by the common P0.7d fault matrix, especially F07 and F08;
B1a does not pretend those rows are already runtime-qualified.

After the raw adapter is crossed, timeout or acknowledgement loss remains
`Indeterminate`. Recovery is status lookup only. Reconciliation revalidates
both broad capabilities with trusted time but never issues a new token and
never resends the provider request.

## Source qualification

The exact B1a candidate must pass:

```text
python3 scripts/verify-hepta-v5-b1a-provider-boundary.py
python3 scripts/verify-hepta-v5-b1a-format.py --check
cargo test --locked -p codex-hepta-contracts --test provider_verified_use_boundary
cargo test --locked -p codex-hepta-contracts
cargo check --locked -p codex-hepta-contracts --all-targets
cargo clippy --locked -p codex-hepta-contracts --all-targets -- -D warnings
just bazel-lock-check
```

Both the exact source head and the GitHub merge candidate require assigned
runners, non-empty steps and terminal success. Queued, skipped, cancelled,
runner-zero, empty-step, stale-head or source-verifier-only results are not
qualification.

## Permanent negative boundary

This tranche keeps all of the following false:

```text
runtime_registered
production_caller
production_writer
model_invocation
provider_dispatch
external_effect
operator_acceptance
promotion
release
```

Runtime, effect, production, operator, promotion and release authority remain
false. Source qualification does not activate a model, provider, network,
credential, product caller or external effect.
