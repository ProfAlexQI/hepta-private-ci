# Hepta AuthBus Basil upstream policy v1

Status: `PLANNING_ONLY`  
Applies to: `AUTHBUS-PLAN-2026-08-26`  
Upstream: `https://github.com/openbasil/basil`  
Pinned baseline: `1fd29adb8e7356968eacbff9309e056cec9bafd7` (main workspace `0.7.2` snapshot; latest published release `v0.7.1`, Apache-2.0)

## Fork rule

Maintain a thin downstream fork. Keep `upstream` and `origin` remotes, preserve upstream crate
paths, and place Hepta behavior in additive crates behind stable traits. Do not embed quota,
market, wallet, TaskFlow or inferd scheduling into `basil-core`. Do not silently alter
`basil.broker.v1`; use a versioned `hepta.auth.v1` namespace for Hepta contracts.

## Patch inventory

Every downstream patch records:

- patch identifier and affected upstream path;
- motivation and upstream issue/commit, if any;
- security and compatibility impact;
- tests and protocol vectors;
- deletion condition or upstream adoption plan.

Patches touching policy semantics, key handling, crypto envelopes, transport, or default-deny
are security-sensitive and require an explicit compatibility review. A patch must be small
enough to revert independently.

## Sync cadence and triggers

- security advisories: fast lane, immediately fetch and test;
- normal bugfixes and patch releases: scheduled sync window (at least monthly while active);
- minor or breaking API/wire changes: compatibility branch, migration note and rollback window;
- dependency or license changes: source/SBOM/license review before merge.

No upstream branch is auto-promoted to a running Hepta service.

The upstream branch named `v0.7.2` is currently a divergent `0.8.0-pre.1` experiment, not the
published `v0.7.1` line and not the pinned main snapshot. Do not fetch it as an implicit upgrade;
adopt it only through a separately reviewed compatibility branch.

## Peer identity and process topology

`SO_PEERCRED` authenticates only the direct Unix-socket peer. If Basil and `hepta-authbusd`
are separate processes, the Basil socket must trust only the fixed AuthBus service UID; the
AuthBus first hop performs the agent/tenant policy check and passes an explicitly attenuated
capability (subject digest, operation, SecretRef allowlist, epoch, TTL and fence). Never forward
an agent UID as if Basil had kernel-attested it. Prefer embedding Basil broker state in the same
Rust daemon over time; until then keep `hepta.auth.v1` on a separate UDS and document the
trusted-bridge boundary.

<a id="authbus-required-sync-pipeline"></a>
## Required sync pipeline

```text
fetch/pin → source + license + SBOM receipt
→ `hepta-basil-host-minimal-v1` locked Basil build/test/clippy/audit + protocol vectors
→ Hepta adapter/contracts/scheduler/gateway tests
→ secret-byte and unsafe/panic policy scans
→ J3160 Linux smoke → Mac development smoke
→ security/compat review → Hepta tag → canary → promote/rollback
```

Failure stops the sync branch and leaves the current development version usable. The receipt is
provenance, not a final-release approval.

## Version compatibility

Treat the upstream 0.7.x patch-compatibility statement as an expectation, not proof. Pin exact
commits and run local vectors against the exact version. Keep an adapter compatibility window
for any breaking transition; never run JS/Python/Rust dual writers during migration.

## Hepta release boundary

Development and internal test may use synthetic providers, loopback HNL, virtual credits and
crash/reopen tests. Real provider effect contracts, external signer/KMS, physical-media claims,
operator acceptance and public settlement remain E.41 `FINAL_RELEASE` inputs. This policy does
not create a development blocker and does not grant production authority.

<a id="authbus-provenance-capture-minimal-profile"></a>
## AUTHBUS.11 provenance capture and minimal profile

The initial source capture is research provenance for the pinned Basil baseline, not an imported
fork or a passed build. The captured identifiers are:

- repository/commit: `openbasil/basil@1fd29adb8e7356968eacbff9309e056cec9bafd7`;
- tree manifest digest: `a37378a0b8878646db43cbf0037a9b056f65d882527d629af0dc4051fcc962ff`;
- deterministic `git archive --format=tar` digest: `1858ba1bc77e345c119f20270f1cd8953c5dd640efebcbcda29893587a2192bc`;
- `Cargo.lock` digest: `994e6a1331b6fd6c998d377d0eb717f726ed000fce08a9eab4a167939aee2703`;
- `rust-toolchain.toml` digest: `441c9c10157504995d17df44eb7f31738118e3f15c97bd035e606e0e6821bb9d`;
- observed toolchain: `rustc/cargo 1.94.0`, `libprotoc 34.1`; observed Basil MSRV: `1.88`.

These are full 64-hex capture digests, but they remain research provenance rather than a release
attestation. A future B0 receipt must also record archive origin, license/SBOM files,
the exact build environment and whether each command passed. Until that receipt exists,
`source_status=research_capture` and `implementation_branch=not-created` remain authoritative.

The Hepta build profile is `hepta-basil-host-minimal-v1`:

- default features are limited to the embedded broker, local transport, OpenBao backend adapter,
  audit, `Sign`, `Verify`, `PublicKey` and `Health`;
- `process_bound_decrypt` is opt-in only, and requires an allowlisted `SecretRef`, declared
  purpose and active permit; plaintext must remain process-bound;
- do not build or expose a standalone/public `basil-bin` host listener;
- `GetSecret`, `SetSecret`, `ImportSecret`, `ListSecret`, `NewKey`, private-key/certificate-key
  return, unscoped mint, NATS/SDS/SPIFFE, Admin, remote invocation and unknown future RPCs are
  denied at compile/registration/runtime layers.

The earlier unqualified command `cargo build --locked --no-default-features` is **historical
v1.2/decode-only notation** and is not a build claim or an acceptable v1.3 proof. Active v1.3
verification must invoke the pinned fork checkout with the explicit
`hepta-basil-host-minimal-v1` profile (the exact manifest/package invocation is emitted and
recorded by B0), followed by Basil unit/integration tests, clippy/audit, protocol golden vectors,
raw-byte scans and negative RPC tests. This document records the commands and policy; it does not
claim that the native build or any external provider/KMS test has passed.

<a id="authbus-contract-dispatch-invariants"></a>
## AUTHBUS.11 contract and dispatch invariants

`codex-hepta-contracts.v1.4_e21`/E21 owns public `CommandEnvelope`, event and `EffectReceipt` bytes;
`hepta-auth-contracts.v1` only extends and re-exports them. Public wire uses
`causal_parent_event_id`; internal event-store/WAL records may use `causal_parent_seq` only with
an explicit immutable `event_seq→event_id` lookup. AuthBus never emits a second parallel enum.

TaskFlow owns canonical `EffectIntent`/`EffectReceipt`; authbusd owns only
`EffectDispatchRef`/`DispatchAttempt` references in its WAL. The dispatch marker is fsynced before
the adapter call. A crash after the call enters `DispatchUnknownRef` and public
`EffectReceipt.Indeterminate`; recovery is lookup-only until an immutable terminal ACK, terminal
failure, cancellation or non-effect proof is verified. `DispatchAccepted` is never terminal.

The sync CI must compare the shared crosswalk projection (owner, namespace, lineage, base mutation
fields and aliases) across the four AuthBus attachments while allowing their explicitly declared
domain-specific keys. It must also reject a non-finite reconcile retry budget, non-terminal ACK
promotion and any Basil raw-secret/admin RPC.

<a id="authbus11-execution-closure-v13"></a>
## AUTHBUS.11 Execution Closure v1.3

This section is the current downstream-sync contract for the Basil-derived AuthBus fork. It
supersedes v1.2 for implementation selection only; v1.2, AUTHBUS.10, E.42 and E.43 remain
immutable decode-only provenance. The active stage and crosswalk selectors are, respectively,
`AUTHBUS_STAGE_MATRIX_v1.yaml#/execution_closure_v1_3/phase_map` and
`AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml#/registry/projection_contract_map`. This is implementation policy, not a
release approval and not a change to E.41 or any runtime authority flag.

### Source and toolchain closure

Every sync receipt must bind the exact source bytes and build inputs, not only a Git commit:

- repository/commit: `openbasil/basil@1fd29adb8e7356968eacbff9309e056cec9bafd7`;
- tree manifest SHA-256: `a37378a0b8878646db43cbf0037a9b056f65d882527d629af0dc4051fcc962ff`;
- deterministic archive SHA-256: `1858ba1bc77e345c119f20270f1cd8953c5dd640efebcbcda29893587a2192bc`;
- `Cargo.lock` SHA-256: `994e6a1331b6fd6c998d377d0eb717f726ed000fce08a9eab4a167939aee2703`;
- `rust-toolchain.toml` SHA-256: `441c9c10157504995d17df44eb7f31738118e3f15c97bd035e606e0e6821bb9d`;
- `.gitmodules` SHA-256 and every initialized submodule commit: captured at B0, or explicitly
  recorded as `not_required` with a reason;
- workspace member include/exclude set, feature set, target triple, license/SBOM inputs and
  descriptor/route digests.

The repository toolchain channel is `1.98.0`; the earlier `rustc/cargo 1.94.0` observation is
research capture only and is not a native-build PASS. Core MSRV is `1.88` and the minimum test
lane is `1.96`. Until all source fields and command results are captured,
`source_status=research_capture`, `source_receipt_status=NOT_CAPTURED` and
`build_claim=NOT_RUN` remain authoritative. A dirty checkout, unresolved submodule or missing
lockfile/toolchain digest fails the sync lane closed.

### Hepta Basil minimal profile and key-generation boundary

The fork build profile is `hepta-basil-host-minimal-v1`. Basil's workspace has no service-group
Cargo feature that is sufficient as an allowlist, so the downstream patch inventory must exclude
`basil-bin`, NATS/courier members and default keystore/unlock/key-generation dependencies; remove
and regenerate forbidden protobuf services/descriptors; and enforce the same deny list at compile,
registration and runtime:

- allowed: `Sign`, `Verify`, `PublicKey`, `Health` and an explicitly process-bound capability;
- optional only under a declared feature, allowlisted `SecretRef`, purpose and active permit:
  process-bound `Decrypt`;
- denied: `GetSecret`, `SetSecret`, `ImportSecret`, `ListSecret`, `NewKey`, private-key or
  certificate-key return, unscoped token mint, NATS/SDS/SPIFFE, Admin, remote invocation and
  unknown future RPCs.

Key generation is forbidden. Patch and test `LocalIdentity::open_or_create`,
`reconcile_catalog.missing_generate` and every keystore generate/import path so a missing
pre-registered backend key returns fail-closed readiness and refuses signing/capability issuance;
it must never generate or silently substitute a software P-256 key. Startup, descriptor, RPC,
binary-symbol and raw-byte negative tests are mandatory. Raw secret bytes are forbidden in wire,
WAL, SQLite projections, logs, receipts and core dumps.

### Required sync and validation pipeline

The exact command, tool version, exit status, fixture digest and timestamp are recorded for every
step. The minimum pipeline is:

```text
fetch/pin + clean-tree check
→ git submodule status --recursive
→ cargo metadata --locked --format-version 1
→ locked minimal-profile build
→ Basil tests/clippy/deny/audit + protocol golden vectors
→ Hepta contract/adapter/scheduler/gateway tests
→ descriptor/raw-secret/key-generation/unsafe scans
→ strict YAML/schema/duplicate-key/crosswalk generation checks
→ historical-terminal and no-blind-retry negative corpus
→ J3160 Linux smoke → macOS development-only smoke
→ security/compatibility review → Hepta tag → canary/rollback
```

The locked build must use the repository's `1.98.0` channel and the exact target/profile. A
failure stops only this sync branch and leaves the current development version usable; it cannot
auto-promote a running service. CI rejects duplicate keys, unresolved stage pointers, a non-finite
reconcile budget, `ManualRequired`/`ReconcileBlocked` terminalization, a blind retry after
`DispatchUnknownRef`, any raw-secret/admin RPC, and any v1.2 pointer selected as normative.

### HNL-GATE0-DECISIONS dependency

Federation-dependent AuthBus stages bind the same external dependency declared by the stage
matrix. `HNL-GATE0-DECISIONS` (short name `HNL-GATE0`) must resolve all three references:

- plan: `OpenClaw/hepta-net-node-link-plan-2026-08-26.md#hnl-gate0-architecture`;
- matrix: `OpenClaw/HNL_STAGE_MATRIX_v1.yaml#/stages/0`, with `expected_stage_id=HNL-0`;
- receipt: `OpenClaw/HNL-PLAN-APPEND-RECEIPT-2026-08-26.json`, with an exact digest.

The HNL plan now exposes the stable `#hnl-gate0-architecture` anchor, but its matrix and append
receipt remain `STALE_SOURCE_BINDING`. Until the HNL owner publishes a fresh anchored receipt,
federated dependency resolution is `RECEIPT_STALE / NOT_READY_FAIL_CLOSED`. The lane split is
explicit: the local lane is `B0–B6 → B8 → B9 → B10` and may continue with synthetic/loopback
inputs; the federated lane is `B0–B6 → B7 → B8 → B9 → B10` and must remain fail-closed. B7 and
federated B8/B9/B10 may emit design or fixture results only, while local B8/B9/B10 remain
development/loopback work. This dependency is not an E.41 release gate; it cannot be converted
into a PASS by copying a stale hash.

### Historical terminal guard and release boundary

Historical stage arrays, v1.2 crosswalks, legacy `ACKED`/`RECONCILING`/`RELEASED`/`REFUNDED`
aliases and old `DispatchStatus` values are decode-only. A frozen negative corpus must prove that
they cannot be emitted on v1.3 wire, selected as an execution input, or reopened. `ManualRequired`
and `ReconcileBlocked` remain non-terminal holds: they cannot close `Indeterminate`, release or
refund quota/escrow, or create a new dispatch without verified evidence and a current fence.
Stale callbacks after an epoch, generation or fencing change are rejected.

Development, internal test and release-prep retain the E.41 continuity rule. Real provider effect
contracts, external KMS/HSM signer evidence, physical-media/power-loss evidence, operator/legal
acceptance and public settlement remain deferred until an explicit `FINAL_RELEASE`; this policy
does not grant production authority or modify any running AuthBus/OpenBao service.

<a id="authbus-v13-concrete-artifacts-and-route-inventory"></a>
## AUTHBUS.11 v1.3 concrete artifacts and route inventory

The implementation source of truth is the registry
`OpenClaw/AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml#/registry`. Four narrow files are generated
projections: `AUTHBUS_OAUTH_SECRETREF_CONTRACT_v1.yaml`,
`AUTHBUS_RECONCILE_E21_CONTRACT_v1.yaml`, `AUTHBUS_OUTBOX_DISPATCH_CONTRACT_v1.yaml` and
`AUTHBUS_REMOTE_RESERVATION_GATEWAY_CONTRACT_v1.yaml`. The four broad AuthBus attachments are
domain projections only. Their registry digest is intentionally `PENDING_AT_B2`; no document
static check is an implementation test or a provider attestation.

For avoidance of ambiguity, the pinned Basil descriptor uses these method paths. The only native
allowlist is:

```text
/basil.broker.v1.SigningService/Sign
/basil.broker.v1.SigningService/Verify
/basil.broker.v1.SigningService/GetPublicKey
/basil.broker.v1.AdminService/Health
```

`/basil.broker.v1.AeadService/Decrypt` is an opt-in process-bound extension. Deny rules must use
the actual descriptor names: Signing `NewKey`, `Import`, `ImportSet`; Secret `GetSecret`,
`SetSecret`, `RotateSecret`, `ListCatalog`; Invocation `Invoke`, `GetInvocationChallenge`,
`GetInvocationCapabilities`; Minting `MintJwt`, `IssueCertificate`; every Admin method other
than `Health` (including `ListConnections` and `DropConnections`); NATS and NixCache. The SPIFFE
and SDS prefixes are `/SpiffeWorkloadAPI/*` and `/envoy.service.secret.v3.SecretDiscoveryService/*`
respectively. `ImportSecret` and `ListSecret` are non-canonical historical labels, not routes.
Unknown future methods are denied. The fork must enforce this set during proto/descriptor
generation, service registration and runtime dispatch; a policy-only wrapper is insufficient.

The same patch must disable `LocalIdentity::open_or_create`, `missing=generate` and every keystore
auto-generate/import path. A missing pre-registered backend key returns fail-closed readiness;
the profile never creates a software P-256 key. B1 records descriptor, binary-symbol and
raw-secret negative vectors, while B0 records exact source/toolchain/SBOM inputs. Refresh uses
opaque `SecretRef`; E21 reconciliation and outbox ownership follow the registry's generated
contracts. Development and internal-test lanes remain available under E.41; this appendix does
not enable a listener, provider effect, signer, or release authority.
