# P0.7b/B2 Tool, Network, and External-Filesystem Verified-Use Contract V1

**Package:** `P0.7b/B2_tool_network_filesystem_boundaries`  
**Plan:** `HEPTA-ARCHITECTURE-CONVERGENCE-V5`  
**Parent:** `cd6823c94b3fbd1c3845a398206f526b8e4bc85e` / tree `fb9af24eaea3283ede58611d639ea8ab1176f2c4`  
**State:** source implemented; exact-head, merge-candidate and independent qualification pending.  
**Authority:** none.

## 1. Scope

B2 replaces a generic, under-specified external-effect crossing with three explicit final-intent classes built on the existing B0 verified-use kernel:

1. tool process spawn;
2. outbound network connect/request;
3. filesystem mutation outside the Agent-owned root.

The checked seam is `CheckedExternalBoundary::cross_once`. It owns a private adapter and exposes no adapter accessor, raw restoration constructor or `into_parts()` escape. This package registers no product caller and performs no real tool, network or filesystem operation by itself.

## 2. Common crossing order

Every B2 crossing must execute:

1. the owning domain durably records the logical operation intent;
2. all boundary-specific identities and policies are finalized;
3. the exact final adapter payload is built;
4. byte count and SHA-256 are revalidated against the intent;
5. the boundary-specific physical payload digest is derived;
6. externally verified `Authorized<ExternalEffectCapability>` is rebound to the current runtime context;
7. the current revocation revision and trusted time are verified;
8. a non-cloneable, non-serializable B0 token is obtained and consumed;
9. a durable single-use operation claim is committed;
10. the caller durably persists the verified-use witness;
11. the private adapter is entered exactly once.

Any failure through step 10 prevents adapter entry. Once the claim has committed, ordinary replay is forbidden.

## 3. Tool process identity

`ToolProcessIntent` length-delimits and binds:

- operation ID;
- canonical executable identity digest;
- executable file-byte digest;
- argv digest and argument count;
- canonical working-directory identity digest;
- environment policy digest;
- sandbox policy digest;
- approval-envelope digest;
- exact final launch payload digest and byte count.

Raw argv and environment values are not persisted by this contract.

## 4. Outbound network identity

`OutboundNetworkIntent` binds:

- operation ID;
- protocol;
- canonical destination;
- resolved DNS/IP-set digest;
- DNS policy digest;
- proxy policy digest;
- TLS policy digest;
- redacted request-header digest;
- exact final request digest and byte count.

The verified-use token cannot be obtained before destination resolution and proxy/TLS policy are final. Destination, IP-set or request drift requires a new reviewed operation rather than reusing the old token.

## 5. External filesystem identity

`ExternalFilesystemMutationIntent` binds:

- operation ID;
- canonical target identity digest;
- device/mount identity digest;
- mandatory `no_follow=true` policy;
- mutation class;
- expected prior-state digest;
- exact mutation payload digest and byte count.

A caller cannot weaken the symlink policy. Target, mount, prior-state or payload drift fails before adapter entry.

## 6. Failure and recovery

- Claim failure prevents the physical adapter from being called.
- Witness persistence failure prevents adapter entry, but the committed claim remains consumed; reconciliation or quarantine owns recovery.
- An adapter error after crossing is `Indeterminate` with a bounded normalized reason code.
- This API exposes no blind retry function.
- A descriptive `RejectedNoCrossing` does not release a committed claim; any retriable no-effect proof belongs to a separately reviewed recovery protocol.
- B4 must prove that no product path bypasses these checked boundaries.

## 7. Required tests

The exact candidate must execute tests proving:

1. all three boundary kinds persist a witness before adapter entry;
2. argv, DNS/IP resolution and expected prior state alter the physical payload digest;
3. `no_follow=false` is rejected;
4. final payload drift fails before claim and adapter;
5. claim and witness failures block adapter entry;
6. post-crossing transport failure is indeterminate and is not retried;
7. B0 rechecks runtime context, revocation, token time and final payload;
8. the raw adapter cannot be extracted through the public API.

## 8. Qualification gates

The exact source head and GitHub merge candidate must each run non-empty attributable steps for:

- duplicate-key-safe source verification;
- Rust 1.95 formatting;
- focused B2 tests;
- complete `codex-hepta-contracts` tests;
- locked all-target check;
- strict Clippy with `-D warnings`;
- Cargo/Bazel lock coherence;
- clean worktree and retained evidence.

P0.7a, B0, B1a and B1b exact qualification remain activation predecessors. Queued, cancelled, runner-zero, empty-step, source-only or stale evidence is not PASS.

## 9. Authority boundary

```text
runtime_registered=false
production_caller=false
production_writer=false
tool_execution=false
network_connect=false
external_filesystem_mutation=false
external_effect=false
operator_acceptance=false
promotion=false
release=false
```
