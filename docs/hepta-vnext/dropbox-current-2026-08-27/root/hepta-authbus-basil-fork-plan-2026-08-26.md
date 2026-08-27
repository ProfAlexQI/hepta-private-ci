# Hepta AuthBus — Basil-derived fork implementation plan

Date: 2026-08-26  
Status: `PLANNING_ONLY / IMPLEMENTATION_BACKLOG / NOT_PRODUCTION_AUTHORITY`  
Plan ID: `AUTHBUS-PLAN-2026-08-26`  
Current normative implementation revision: `AUTHBUS.11-v1.3`  
Current normative anchor: `authbus11-execution-closure-v13`  
Canonical parent: `hepta-vnext-development-plan-final-2026-08-23.md` (append section `AUTHBUS-PLAN-2026-08-26`)  
Parent SHA-256 before append: `ca5e28fc5b6a7bac8ca4c156dc7ce389f9320161081910b42f9ec01007ba244c`

## 1. Decision

Hepta should create a downstream fork of [Basil](https://github.com/openbasil/basil), keep an
`upstream` remote, and continuously ingest upstream security and compatibility improvements.
The fork is a thin host-local broker layer, not a replacement for the complete Hepta AuthBus
product and not a fork of OpenBao.

Pinned research baseline:

```text
repository: https://github.com/openbasil/basil
upstream_commit: 1fd29adb8e7356968eacbff9309e056cec9bafd7
workspace_version: 0.7.2 (main snapshot; not a published release)
latest_published_release: v0.7.1
do_not_follow: v0.7.2 branch (divergent 0.8.0-pre.1 experiment)
license: Apache-2.0
upstream_maturity: pre-1.0
primary_runtime_target: Linux x86_64/aarch64
```

The fork keeps Basil's local security boundary (Unix socket, kernel caller identity,
default-deny policy, backend abstraction, audit and zeroization paths) and adds Hepta-owned
crates around stable traits. Hepta does not put quota, marketplace, wallet, TaskFlow or model
execution semantics into `basil-core`.

## 2. Product role in Hepta

AuthBus is the **Capability & Resource Control Plane**. It answers which subject may consume
which auth or compute capacity, for how long, under which quota and data policy, and produces a
short-lived usage permit and receipt. It is not a credential synchronizer, wallet, workflow
engine, fleet scheduler, or model runtime.

The first three product surfaces are:

1. **Local allocation:** one user's agents and nodes share finite provider/auth/compute
   resources. Admission uses request/token rate, per-auth concurrency, remaining quota,
   `Retry-After`, health, deadline and fair-share feedback.
2. **Federated usage rights:** users exchange revocable, bounded capacity leases over HNL. A
   buyer receives a dispatch permit, never the seller's raw provider token. Walletd/TRNM own
   escrow and settlement.
3. **Compute Gateway:** `hepta-gatewayd` exposes owner-authorized local inference or provider
   capacity through a virtual short-lived credential and OpenAI-compatible/Hepta endpoints.
   Upstream credentials remain inside the owner adapter/backend.

Required additions beyond those surfaces are tenant isolation, consent and revocation,
provider-terms/data-residency policy, abuse controls, replay/partition/clock-drift semantics,
metering and dispute handling, fair scheduling, and privacy-preserving observability.

## 3. Thin-fork topology

```text
basil-core/client/proto/keystore/backend  (upstream-compatible, low-diff)
                    │
             hepta-basil-adapter
                    │
        hepta-auth-contracts + authbus-core
             │          │             │
         scheduler    HNL          metering
             │          │             │
         authbusd    node-linkd   gatewayd → inferd/provider
                                      │
                             market-adapter → walletd/TRNM
```

The fork repository should preserve upstream paths and isolate Hepta code:

```text
crates/basil-bin
crates/basil-client
crates/basil-core
crates/basil-proto
crates/basil-keystore-backend
crates/hepta-basil-adapter
crates/hepta-auth-contracts
crates/hepta-authbus-core
crates/hepta-authbus-scheduler
crates/hepta-authbusd
crates/hepta-authbus-hnl
crates/hepta-authbus-metering
crates/hepta-gatewayd
crates/hepta-market-adapter
```

Only a demonstrated upstream defect or platform seam may modify an upstream crate. Every such
patch carries a reason, source issue/commit, compatibility test, and deletion condition. A
Hepta feature must not silently change `basil.broker.v1`; a new `hepta.auth.v1` namespace is
used for Hepta contracts.

## 4. Authority and data boundaries

| Component | Sole responsibility | Explicit non-responsibility |
|---|---|---|
| Basil layer | local caller identity, policy, in-place secret/sign operation, audit/backend seam | quota, market, wallet, workflows |
| `hepta-basil-adapter` | typed OpenBao/Vault/KMS mapping, `SecretRef`, health/error classification | returning raw long-lived secrets, replacing OpenBao server |
| `hepta-authbus-core` | resource, lease, permit, epoch, CAS, revoke and reconcile state | model execution, balances, fleet lifecycle |
| scheduler | rate/concurrency/fair admission and cooldown | TaskFlow/fleet scheduling |
| `authbusd` | per-host UDS, SO_PEERCRED, tenant/agent policy, WAL/audit bridge | public listener by default, wallet mutation |
| HNL adapter | encrypted peer session and signed usage-right transport | price discovery, settlement, authority by IP |
| `gatewayd` | virtual credential, ingress, streaming/backpressure, metering | holding upstream token, replacing inferd |
| walletd/TRNM adapter | escrow, nonce/UTXO, settlement, dispute | raw credential brokerage |
| agentd/fleet supervisor | process generation, lifecycle and fencing | resource quota policy internals |
| inferd | model/device execution, cancellation and `ModelReceipt` | provider credential authority |

The single-writer rule is invariant: an `AuthResource` has one owner authority. Remote nodes
request a lease through HNL; an unreachable owner refuses new leases. No implicit multi-primary
CRDT or network-reachability-as-authorization is introduced.

### Peer-credential boundary

Kernel peer credentials identify only the process directly connected to a Unix socket. If Basil
is a separate process, an agent's UID/GID is not forwarded through `hepta-authbusd`; Basil must
not treat a forwarded subject as kernel-attested. The first-hop `authbusd` therefore performs
the complete SubjectRef/tenant policy check. A separate Basil socket is service-UID-only and
accepts a narrowly attenuated capability containing subject digest, operation, allowed
SecretRefs, epoch, TTL and fence. The long-term target is to embed Basil broker state in the
same Rust daemon (or add a tiny upstreamable custom-service hook), while keeping
`hepta.auth.v1` on a distinct UDS. This explicit choice prevents a confused-deputy path and
avoids claiming two independent SO_PEERCRED checks when only one exists.

Raw access/refresh/API tokens and private keys never enter agent SQLite, WAL projections,
audit records, HNL envelopes, gateway logs, market objects or receipts. The only cross-boundary
value is an opaque `SecretRef`, a process-bound operation, or a digest/reference.

## 5. Core contracts

The first version freezes these versioned Rust types with canonical bytes and domain-separated
digests:

```text
Principal, SubjectRef, AuthResource, ComputeResource, SecretRef,
QuotaContract, QuotaSnapshot, AuthRequest, ResourceLease, UsagePermit,
UsageReceipt, VirtualCredential, Revoke, Offer, Quote, Order, UsageRight,
DispatchPermit, SettlementRef, Dispute
```

Every lease/permit binds owner, subject, resource digest, payload/model digest, authority epoch,
generation, nonce, audience, not-before/expiry, maximum usage and policy digest. Mutations use
`expected_revision + owner_epoch + generation + fencing_token`.

Unknown provider quota, sealed/standby backend, 429/timeout, lost response and ambiguous
dispatch are conservative states. `Queued`, `Accepted` and HTTP 202 are not terminal success;
the state is `Indeterminate → reconcile`. An epoch bump or revoke prevents creation of new
dispatches from old permits.

## 6. Local allocation scheduler

Each resource pool combines a per-resource semaphore, request/token bucket or sliding window,
cooldown from provider feedback, health/quarantine state, quota confidence, and owner epoch.
Candidate selection filters scope/model/data policy, generation and quota before applying
tenant→workspace→agent weighted fair queue; EDF orders hard deadlines and deficit round-robin
serves background work. Reservation precedes dispatch; cancellation releases unconsumed budget.

The scheduler only admits auth/compute use. TaskFlow submits an admission request, agentd owns
lifecycle, and inferd owns model execution. Account rotation is never used to evade a provider
limit.

## 7. Federation and Gateway

HNL carries encrypted, versioned signed `Offer`, `UsageRight`, `DispatchPermit`, `UsageReceipt`
and `Revoke` envelopes. Default rights are non-transferable, depth zero, bounded scope and
short expiry. Provider terms must explicitly permit any transferable capacity.

`hepta-gatewayd` authenticates a virtual credential (an ingress `Authorization: Bearer` value),
asks AuthBus for a permit, and calls inferd or an owner-side adapter. The ingress credential is
extracted and verified; callers cannot choose the upstream URL or forward arbitrary upstream
headers, and the adapter constructs the real upstream `Authorization` after the gateway boundary.
The gateway supports streaming/backpressure/cancel and emits `ModelReceipt` plus `UsageReceipt`.
It starts loopback-only; LAN or public bind is an explicit later choice. Model output cannot mint
a grant, API key, wallet asset or governance authority.

## 8. Upstream update contract

The fork has three refs: `upstream/basil-main`, `hepta/basil-base`, and
`hepta/authbus-integration`. A sync receipt records upstream commit/tag, lockfile/toolchain,
SBOM, license notices, patch inventory, tests and known gaps.

Update classes:

- security fix: fast lane with review and immediate compatibility tests;
- patch/bugfix/non-breaking feature: scheduled sync window;
- protocol, policy, key-handling or breaking API change: compatibility branch and explicit
  Hepta migration; never auto-promote.

The mandatory pipeline is:

```text
fetch → source/license/SBOM receipt → Basil build/test/clippy/audit
→ protocol golden vectors → Hepta adapter/contracts/scheduler/gateway tests
→ J3160 Linux smoke → Mac development smoke → security/compat review
→ signed Hepta tag → canary → promote or rollback
```

The fork consumes upstream improvements through a small, reviewable patch queue. It does not
blindly track `main`, and an upstream failure affects the sync lane only, not the current
development installation.

The similarly named upstream `v0.7.2` branch is an experimental, divergent `0.8.0-pre.1`
line. It is excluded from the initial sync target until a separate compatibility review and
explicit migration decision.

## 9. Staged implementation plan (normative B0–B10 overlay)

| Stage | Deliverable | Definition of done |
|---|---|---|
| B0 / AUTH-0 | fork provenance, license notices, remote and pinned source | source/SBOM/license receipt and Basil native build |
| B1 / AUTH-1 | upstream base import and feature profile | policy/UDS/backend/audit vectors; no legacy production caller |
| B2 / AUTH-2 | contracts and lease/permit state machine | property/golden/idempotency/CAS/epoch tests |
| B3 / AUTH-3 | typed SecretBackend adapter | 404/401/timeout distinction, fail-closed sealed state, no raw projection |
| B4 / AUTH-4 | local scheduler | synthetic 16-agent/3-resource test, no oversell/starvation bound |
| B5 / AUTH-5 | Rust authbusd and WAL/audit | crash/reopen, fsync, stale-owner and observe bridge tests |
| B6 / AUTH-6 | legacy metadata migration and single-writer canary | no raw projection, rollback and observe bridge |
| B7 / AUTH-7 | LAN/HNL capability channel and quota feedback | 3-node loopback, replay/partition reject, raw-secret scan zero |
| B8 / AUTH-8 | compute gateway and inferd seam | streaming/cancel/backpressure, virtual credential expiry/revoke |
| B9 / AUTH-9 | usage-right market sandbox | virtual escrow, duplicate settlement/double-spend/replay tests |
| B10 / AUTH-10 | release preparation and external-input handoff | final-release checks remain gated; no promotion |

B0–B9/B10 are implementation backlog. Under E.41, DEVELOPMENT, INTERNAL_TEST and RELEASE_PREP
continue with synthetic/loopback fixtures. Real provider contracts, external KMS/HSM,
physical-media evidence, operator acceptance and public settlement are deferred until an
explicit `FINAL_RELEASE`; they are not development blockers.

<a id="authbus-migration-verification"></a>
## 10. Migration and verification

Migration is `observe → shadow → projection → canary → federated sandbox → release`. During
observe/shadow, the existing Node/Python services are read-only observers; JS/Python/Rust never
double-write. A rollback changes authority/adapter flags and preserves historical receipts;
it does not rewrite an indeterminate result into success.

Required checks include upstream parity, canonical serialization, scheduler fairness and quota
limits, UDS caller identity, raw-secret byte scans, rotation/revocation, crash/reopen/reconcile,
HNL replay/partition behavior, gateway stream lifecycle, wallet conservation and source/license
provenance. Metrics contain digests and references rather than prompts or credentials.

## 11. Immediate implementation queue

1. Freeze `hepta-auth-contracts` Resource/Lease/Permit/Receipt vectors.
2. Import the pinned Basil base into an isolated fork and establish the sync CI.
3. Implement `hepta-authbus-core` semaphore/token bucket/WFQ/EDF with fake provider/inferd.
4. Implement the Rust UDS shell and typed OpenBao adapter on J3160; keep current services
   untouched and read-only.
5. Add crash/reopen/reconcile and gateway virtual-credential seams before any federation or
   real external effect.

This plan does not generate production keys, alter the running OpenBao/AuthBus deployment, open
public listeners, or authorize external effects. It is a practical implementation backlog,
not a release approval.

## 12. AUTHBUS.10 implementation clarifications (historical)

<!-- normative: false; execution_input: false; decode_only: true; superseded_by: AUTHBUS.11-v1.3 -->
Machine consumers MUST treat this section as `normative=false`, `execution_input=false` and
`decode_only=true`; it is retained only for provenance and must never supply a stage, writer or
state-machine implementation input.

The canonical parent plan's `AUTHBUS.10` was the first implementation overlay. It is retained for
provenance only and is superseded by AUTHBUS.11 v1.3; the v1.2 crosswalk blocks are decode-only.
Code generation and CI must consume the canonical registry projection map
(`AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml#/registry/projection_contract_map`) and its declared
`execution_closure_v1_3` projections; each attachment's top-level `contract_crosswalk_v1_3` block is
decode-only. This resolves the historical B2/B3/B9 label ambiguity without changing E.41 or adding a
release gate.
The corrected dependency order is:

```text
B0 provenance → B1 topology/profile/identity → B2 contracts → B3 adapter
→ B4 scheduler → B5 authbusd WAL/reconcile → B6 legacy migration
→ B7 HNL → B8 gateway → B9 market sandbox → B10 release preparation
```

Implementation invariants are:

- `hepta-authbusd` is the per-host single durable writer for resources, quota snapshots, leases,
  permits and dispatch references. TaskFlow remains the sole writer for EffectIntent,
  EffectReceipt, ReconcileDecision and the canonical reconcile queue; Agent SQLite is
  projection/cursor only; Basil audit is a decision projection.
- Embedded Basil is preferred. A separate Basil process may trust only a fixed service UID plus an
  attenuated, nonce/audience/TTL/epoch/fence-bound capability; it must not claim forwarded agent UID.
- The Hepta Basil profile registers only process-bound `Sign`, `Verify`, `PublicKey` and
  `Health` (plus an explicitly Hepta-wrapped capability operation); `Decrypt` is disabled by default
  and requires an explicit process-bound feature, allowlisted SecretRef and active permit. Secret get/set/import/export,
  key creation, certificate private-key return, Admin/NATS/SDS/SPIFFE and unknown future RPCs are
  compile/register/runtime denied.
- Contracts reuse canonical `CommandEnvelope`, `EffectIntent`, `ActivityReceipt`, `EffectReceipt`,
  `FailureClass`, `IndeterminateReason` and `CapabilityId`. Every mutation binds command/idempotency,
  payload/resource digest, revision, owner/generation, authority epoch and fencing token.
- OAuth migration carries only `AuthProfile`/`ProviderSession`/`TokenFamily` metadata and `SecretRef`;
  refresh uses CAS/singleflight and invalid-grant quarantine. Node/Python remain read-only observers
  until a single-writer cutover.
- HNL, gateway and market use bounded usage-rights and virtual credentials. Raw provider tokens are
  never products, receipts, projections or wire data; walletd/TRNM alone owns escrow, nonce and
  settlement.

The four machine-readable implementation inputs are:

1. `AUTHBUS_TRUST_MODE_MATRIX_v1.yaml` — actors, direct-peer limits, Basil service allow/deny and
   DEVELOPMENT/INTERNAL_TEST/RELEASE_PREP/FINAL_RELEASE mode behavior;
2. `AUTHBUS_RESOURCE_QUOTA_METERING_CONTRACT_v1.yaml` — resource taxonomy, quota vectors,
   observations, reservation, metering and legacy profile fields;
3. `AUTHBUS_FAILURE_RECONCILE_STATE_MACHINE_v1.yaml` — fenced transitions, WAL order, failure
   classes and recovery tests;
4. `AUTHBUS_ABUSE_DISPUTE_POLICY_v1.yaml` — usage-right product boundary, gateway abuse controls,
   escrow/dispute and privacy rules.

These files are implementation/test inputs, not authority receipts. Quantitative values in them are
targets to baseline and tune; they do not convert deferred E.41 provider, KMS/HSM, physical-media,
operator or public-settlement inputs into development blockers. No running service, active OpenBao
state or production flag is changed by this clarification.

## 13. AUTHBUS.11 contract crosswalk and implementation binding (v1.2 historical)

> **Decode-only notice:** this section is retained for provenance. It is superseded for all
> new code generation, stage selection and CI by `## 14 / AUTHBUS.11 v1.3`.

This historical v1.2 text is retained for decode/provenance only; it is not the active implementation
overlay. The active public contract owner is
`codex-hepta-contracts.v1.4_e21`/E21; `hepta-auth-contracts.v1` adds only AuthBus resource and quota
types and re-exports the shared envelope/receipt types under `hepta.auth.v1`. Public wire uses
`causal_parent_event_id`; internal WAL/event-store records may use `causal_parent_seq` only after an
immutable `event_seq→event_id` lookup. This is an explicit serialization-context mapping, not a
numeric alias.

The E21 `EffectReceipt` is a closed object. Its exact required set is
`effect_id,effect_intent_id,decision_id,run_id,schema_digest,status,terminal,
reconciliation_required,idempotency_key,attempt,authority_scope,execution_scope,effect_authority,
production_authority,snapshot_digest,graph_digest,policy_digest,provider_id,reconciliation_id,
reconciles_receipt_id,external_state_digest,resolver_attempt,current_revision,cas`. AuthBus fields
are carried in the canonical envelope/payload or the existing `ReconcileDecision` sidecar; they may
not be added as new top-level E21 receipt fields.

TaskFlow owns canonical `EffectIntent`/`EffectReceipt` and the existing `ReconcileDecision`; per-host `hepta-authbusd` writes resources,
quota, leases, permits and only the `EffectDispatchRef`/`DispatchAttempt` references needed to
reserve and reconcile. The marker is fsynced before an adapter call. A post-call crash records
`DispatchUnknownRef`, projects public `EffectReceipt.Indeterminate`, and performs lookup-only
recovery until an immutable terminal ACK, terminal failure/cancellation, or non-effect proof is
available. AuthBus then writes only a fenced `ReconcileEvidenceRef` proposal; TaskFlow validates it
and commits `ReconcileDecision` next to the closed E21 receipt. No new top-level receipt field,
payload escape hatch, or parallel reconciliation status is introduced.

The Basil profile is `hepta-basil-host-minimal-v1`: default services are Sign/Verify/PublicKey/Health
and the Hepta process-bound capability wrapper; process-bound Decrypt is opt-in. Raw secret get/set,
import/export, key creation, private-key return, unscoped mint, Admin/NATS/SDS/SPIFFE, remote
invocation and unknown future RPCs are compile-, registration- and runtime-denied. A separate Basil
process trusts only the AuthBus service UID and an attenuated nonce/audience/TTL/epoch/fence-bound
capability; it never receives a falsely forwarded agent UID.

The four YAML inputs expose the same shared crosswalk projection (owners, namespace, lineage, base
mutation fields and aliases) and retain domain-specific status, quota, failure, trust and dispute
keys. CI must validate that projection, the B0→B10 stage DAG, canonical E21 fields, terminal ACK
guards, bounded reconcile attempts, no-blind-retry recovery, and quantitative target ownership.
Gateway ingress may carry only `Authorization: Bearer <virtual-credential>` for extraction and
verification; the caller cannot select an upstream URL or forward arbitrary upstream headers. The
adapter constructs the upstream authorization after the gateway boundary. E.41 remains unchanged:
all external provider/KMS/HSM/physical/public-settlement evidence is deferred until an explicit
`FINAL_RELEASE`; this plan does not change running services or authority flags.

The market adapter is the sole durable writer for `UsageRight` and `UsageRightCounter`.
`hepta-authbusd` may only submit fenced `ReserveUse`, `CommitUse`, and `ReleaseUse` commands and keep
the returned `UseReservationRef`. The mutable counter separates `reserved_uses` from
`consumed_uses`; no-effect/cancel releases a reservation, unknown outcomes keep it held, and the
first verified consumption commits it. Every update binds counter revision and before/after digest
to the `UsageReceipt`. HNL mappings are role-qualified (`initiator_node→local_node_id`,
`remote_node→remote_node_id`, `NodeDescriptor.node_id→descriptor_node_id`) and preserve expiry,
epoch, policy, record sequence, key and signature fields without widening scope.

The v1.2 implementation amendment is recorded separately from the immutable original append
receipt: `hepta.authbus_amendment_receipt.v1.schema.json` and
`AUTHBUS-PLAN-AMENDMENT-RECEIPT-2026-08-26.json`. The qualification index records both the
pre-amendment parent digests and the post-amendment file digests; the original E.42 crosswalk and
`AUTHBUS-PLAN-APPEND-RECEIPT-2026-08-26.json` remain immutable historical records. This v1.2
receipt/schema pair is not the active v1.3 execution-closure receipt.

## 14. AUTHBUS.11 execution closure v1.3 (normative)

The v1.3 closure supersedes v1.2 for new code generation, stage resolution and CI. v1.2 and
AUTHBUS.10 remain immutable decode-only history. The closure is implementation work, not a
release approval, and leaves E.41 and all production flags unchanged.

### 14.1 Contract and bytes

`codex-hepta-contracts.v1.4_e21`/E21 remains the sole owner of the closed public envelope and
EffectReceipt. `hepta-auth-contracts.v1` owns only AuthBus resource/quota/lease types and
re-exports the shared enums. Four YAML attachments expose generated `execution_closure_v1_3`
projections; hand-maintained duplicate state/error registries are forbidden.
The active E21 schema is pinned to
`OpenClaw/hepta-vnext-qualification-2026-08-23/e21-contract-hardening-qualification-20260824/schema/canonical_contract_v1_4_e21.schema.json`
(`0e712e91cd188a150b8349391d9043c4ca61cef4798a091f36cd46e675fd6955`); other copies are
historical/decode-only inputs.

Every serialization context (E21 envelope, AuthBus object, Basil protobuf, HNL COSE/CBOR and
gateway JWT/COSE) declares its version, domain label, deterministic field ordering, UTF-8 and
integer rules, null/default behavior and unknown-field policy. The Basil length-delimited
transport carries the same canonical preimage and a domain-separated SHA-256 digest. Time aliases
are context-specific: lease RFC3339/monotonic values are never silently encoded as JWT NumericDate.
All digest/id formulas use the length-delimited `preimage_v1` field encoding and shared golden vectors;
unframed string concatenation is forbidden.

### 14.2 SecretRef refresh

The first implementation is an adapter-local, process-bound `RefreshWithSecretRef`/
`RotateSecretRef`; `authbusd` is the TokenFamily writer. An optional `hepta-auth-refreshd` is
allowed only after a review proves an adapter boundary insufficient, and the two modes may never
dual-write. Requests contain only `schema_version,operation_id,refresh_operation_key,command_id,run_id,
profile_id,provider_id,token_family_id,secret_ref,expected_secret_revision,idempotency_key,payload_digest,
policy_digest,scope_digest,authority_epoch,owner_epoch,generation,fencing_token,logical_clock,
causal_parent_event_id,deadline_at,purpose_digest,audience` and no raw credential bytes. The operation
key is durable and non-null before the provider call. Responses contain only opaque SecretRef/version,
operation key, status and digests, never token bytes, provider headers or provider response bodies;
new SecretRefs are non-null only for a successful rotation, while failure/indeterminate responses
forbid them and retain the operation key for lookup. Exact response required/nullable/conditional
fields are generated from the canonical registry
`AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml#/registry/contracts/oauth_secretref`; the
`AUTHBUS_RESOURCE_QUOTA_METERING_CONTRACT_v1.yaml#/execution_closure_v1_3/opaque_secret_operations`
block is a generated projection and may add only explicitly owned adapter conditional guards.
The exact required `RefreshWithSecretRef` response is
`schema_version,response_id,operation_id,provider_id,profile_id,token_family_id,outcome,
access_secret_ref,refresh_secret_ref,secret_revision,refresh_operation_key,provider_status,
response_digest,idempotency_key,payload_digest,expected_secret_revision,authority_epoch,owner_epoch,
generation,fencing_token`; the exact required `RotateSecretRef` response is
`schema_version,response_id,operation_id,provider_id,profile_id,token_family_id,outcome,
new_refresh_secret_ref,secret_revision,refresh_operation_key,response_digest,idempotency_key,
payload_digest,expected_secret_revision,authority_epoch,owner_epoch,generation,fencing_token`.
Outcome-specific nullable/forbidden rules are generated from the canonical registry; the Resource
attachment is a generated projection and may add only explicitly owned adapter guards. They must preserve
the response binding. `deadline_at` is
the only active request time field (`deadline` is decode-only historical input). `INDETERMINATE` refreshes reconcile by key; a verified lookup maps
`ROTATED→SUCCEEDED`, `INVALID_GRANT→QUARANTINED`, `TRANSIENT_FAILURE→BACKOFF` and
`UNKNOWN→MANUAL_REQUIRED`. Claim TTL takeover rejects stale callbacks;
`INVALID_GRANT` is a classification event only and quarantines the token family; it is never a durable
state. `MANUAL_REQUIRED`/`RECONCILE_BLOCKED` are nonterminal
holds and cannot release quota or escrow.

### 14.3 Effect/reconcile and outbox

E21's required receipt fields remain closed. Deterministic reconciliation IDs use the single
canonical `preimage_v1(domain=hepta.auth.reconciliation.v1; effect_id; effect_intent_id; attempt;
authority_epoch; owner_epoch; generation)`; same-attempt retries reuse the ID and a
new attempt or owner epoch creates a new ID. The canonical `ReconcileEvidenceRef` and `NoEffectProof`
field sets are generated from `AUTHBUS_FAILURE_RECONCILE_STATE_MACHINE_v1.yaml#/execution_closure_v1_3/reconcile_evidence`
and must not be hand-diverged. A `ReconcileEvidenceRef`/`NoEffectProof` fixture fills
the required IDs for queued, direct-terminal and cancel-without-dispatch cases. Direct terminal ACK
is legal immediately after a durable `DispatchAttemptStarted`; otherwise the sequence is accepted-ref
→ ACK. A synthetic accepted marker is permitted only in the same writer transaction as the terminal
ACK, must be marked synthetic, and is never publicly projected. `StatusByEffectKey` has fixed
provider namespace, effect key, payload digest, operation reference, terminal state,
signature/audience/key_epoch, execution mode attestation and retention fields. `not_found_kind` is
one of `none|verified|unavailable|expired`; any non-`none` response has `terminal_state=UNKNOWN`,
no operation/ack refs, and only verified not-found can close through a fenced NoEffectProof.
Collision, unknown or expired evidence stays `Indeterminate/ManualRequired` and never creates a new attempt.
Pre-dispatch NoEffectProof uses `dispatch_marker_state=absent`, the deterministic no-marker digest,
and the TaskFlow enqueue `outbox_delivery_seq`; its exact required fields are
`proof_id,proof_stage,reconciliation_id,provider_id,provider_namespace,effect_id,effect_intent_id,effect_key,
attempt,idempotency_key,operation_ref,proof_kind,dispatch_attempted,dispatch_marker_state,
dispatch_marker_digest,outbox_delivery_seq,writer_record_fsync_confirmed,writer_fsync_witness_digest,
provider_query_receipt_digest,external_state_digest,evidence_digest,observed_at,verified_at,decided_at,
taskflow_decision_id,taskflow_decision_digest,authority_epoch,owner_epoch,generation,fencing_token,
payload_digest,current_revision,cas`. The fsync witness resolves to
`source_owner,wal_segment,wal_offset,event_seq,commit_digest,directory_fsync,writer_boot_id,
writer_generation,verified_at`; a boolean alone is invalid. `operation_ref` is the deterministic
provider-namespace/effect-id/effect-key sentinel for pre-dispatch and verified-not-found proofs;
a real provider reference is allowed only for provider-explicit no-effect with the full binding.
TaskFlow alone commits the ReconcileDecision.

authbusd and TaskFlow each commit locally and bridge through an at-least-once outbox/inbox with the
canonical `hepta.auth.outbox-delivery.v1` fields:
`schema_version,outbox_id,source_owner,destination,event_type,event_id,aggregate_id,command_id,
causal_parent_event_id,delivery_seq,delivery_attempt,provider_attempt,resolver_attempt,payload_ref,
payload_encoding,payload_digest,source_record_digest,idempotency_key,lease_owner,lease_expires_at,
next_retry_at,ack_digest,created_at,updated_at,status,authority_epoch,owner_epoch,generation,
fencing_token,writer_record_fsync_confirmed,writer_fsync_witness_digest,dead_letter_reason` and
nullable `lease_owner,lease_expires_at,next_retry_at,ack_digest,dead_letter_reason`. Bridge retry may reuse the same idempotency/payload; provider effect retry after
DispatchUnknown is forbidden; resolver retries are bounded lookup-only. 
This is not a distributed transaction. The dispatch marker is fsynced before an adapter call;
post-call crash is lookup-only `DispatchUnknownRef`. TaskFlow remains the sole owner of the
canonical EffectIntent/EffectReceipt/ReconcileDecision.

### 14.4 Actual Basil profile and source closure

The pinned Basil source has no service-group Cargo features. B1 therefore carries a patch inventory
that excludes `basil-bin`, NATS/courier members and default keystore/unlock/keygen dependencies,
removes and regenerates forbidden proto services/descriptors, and applies compile/registration/
runtime deny tests. `LocalIdentity::open_or_create`, `missing=generate` and keystore generation
are patched to `key_generation=forbidden`; missing external key registration fails closed. B0
records tree/archive, lockfile, toolchain, cargo metadata, SBOM/license tool versions and command
digests. Research capture of Rust 1.94 is not a native-build pass when upstream requires 1.98.

### 14.5 Quota, federation, gateway and market

Quota observations carry a `window_id`/`window_kind`, start/end/reset timestamps, monotonic
`source_seq`, component-level KNOWN/UNKNOWN state, limit/used/held/remaining vectors, revision,
epoch/generation/fence and stale-age; old observations cannot overwrite newer capacity and a window
rollover always creates a new ID. Remote reservation and its `ReserveRemote` command carry
`window_id,window_kind,window_start,window_end,reset_at,source_seq,window_digest`; the digest is
included in the signed preimage and cannot change under one idempotency key. Remote reservation preserves
`local_held + remote_held + consumed <= limit` and holds unknown outcomes through reconciliation.
`UsageVector=(request_count,rpm,tpm,concurrency,day_budget,context)` declares units and rounding;
a request-count-only market mode must not be advertised as TPM or compute capacity, and legacy
`max_uses` may map only to `request_count` under an explicit request-count-only mode. B7 is
fail-closed until the HNL Gate-0 decisions have a resolvable
source digest and receipt. Gateway algorithms, issuer/JWKS rotation, PoP/replay/TTL/revocation,
API subset, SSE/cancel/error and SSRF policy are frozen before non-loopback bind. Virtual credentials
bind tenant/workspace/agent/service/node, generation/epoch/fence, resource/provider/permit and
payload digest; remote use requires PoP `cnf`, with replay key `(iss,kid,jti,resource_id,permit_id,payload_digest,nonce)`.
Raw tokens remain
non-transferable; only owner/provider-consented bounded delegated rights can enter a market mode,
with explicit price, fee, escrow timeout, refund and appeal conservation. ACTIVE or
PARTIALLY_CONSUMED expiry/cancel/refund enters DISPUTED first; a terminal market state requires an
immutable wallet disposition join including wallet epoch/nonce and effect/usage receipt digests.

### 14.6 Stage and CI closure

The only executable stage registry is resolved from
`AUTHBUS_STAGE_MATRIX_v1.yaml#/execution_closure_v1_3/phase_map`; its active lane selects
`local: B0→B1→B2→B3→B4→B5→B6→B8→B9→B10` or
`federated: B0→B1→B2→B3→B4→B5→B6→B7→B8→B9→B10`.
Local B8/B9/B10 do not wait for HNL-GATE0; the federated lane remains fail-closed until that
dependency resolves. DoD ownership is:
provenance/toolchain (B0), Basil deny/keygen (B1), canonical bytes/status (B2), SecretRef/direct
ACK/status lookup (B3), scheduler fairness (B4), WAL/reconcile (B5), single-writer migration (B6),
HNL Gate-0/replay (B7), gateway stream/security (B8), usage-vector/ledger/dispute (B9), and
reproducible canary/rollback handoff (B10). CI binds every PASS to command, fixture and tool
digests and rejects historical terminal flags, blind retry, duplicate keys, raw-secret bytes,
ManualRequired closure and stage-pointer drift.

The v1.3 closure is recorded by the separate execution-closure receipt
`AUTHBUS-PLAN-AMENDMENT-RECEIPT-2026-08-26-v1.3.json` and schema
`hepta.authbus_amendment_receipt.v1_3.schema.json`, as named in the qualification index.
Development, internal test and release-prep remain unblocked under E.41;
provider, KMS/HSM, physical-media, public settlement and operator/legal evidence are evaluated
only on an explicit `FINAL_RELEASE`. No running service, key, listener or authority flag changes
as part of this document revision.

<a id="authbus-canonical-artifact-closure-v13"></a>
### 14.7 Canonical contract artifacts and implementation handoff (v1.3)

The fork plan does not maintain a second hand-written protocol. B2 consumes the registry at
`OpenClaw/AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml#/registry`; it is the sole normative source
for status/error values, E21 required fields, canonical preimages, identity fences and usage
dimensions. The following files are generated projections and are intentionally
`projection_only: true` until B2 binds their registry digest:

- `AUTHBUS_OAUTH_SECRETREF_CONTRACT_v1.yaml#/contract` — `hepta.auth.opaque-secret-operation.v1`;
- `AUTHBUS_RECONCILE_E21_CONTRACT_v1.yaml#/contract` — `hepta.auth.reconcile-evidence.v1`;
- `AUTHBUS_OUTBOX_DISPATCH_CONTRACT_v1.yaml#/contract` — `hepta.auth.outbox-delivery.v1`;
- `AUTHBUS_REMOTE_RESERVATION_GATEWAY_CONTRACT_v1.yaml#/contract` — the paired
  `hepta.auth.remote-reservation.v1` and `hepta.gateway.credential.v1` contracts.

The four existing trust/quota/failure/abuse YAMLs remain domain projections. A projection may
wrap or group fields and expose decode-only aliases, but may not rename/drop fields, reinterpret
states, replace the six `UsageVector` dimensions, or conflate refresh `operation_id` with a
provider `provider_operation_id`. The B2 generator must fail closed when `source_registry_ref`,
registry revision or projection transform does not match.

The Basil descriptor inventory is also canonicalized here. Native allowlist is only
`SigningService/Sign`, `SigningService/Verify`, `SigningService/GetPublicKey` and
`AdminService/Health`; `AeadService/Decrypt` is opt-in process-bound. Invocation challenge and
capabilities, Secret `ListCatalog`, Signing import/new-key, all other Admin methods (including
`ListConnections`/`DropConnections`), Minting, NATS, NixCache, `/SpiffeWorkloadAPI/*`,
`/envoy.service.secret.v3.SecretDiscoveryService/*`, and unknown routes are denied at descriptor,
registration and runtime layers. The previous `ImportSecret`/`ListSecret` labels are historical
aliases only; they are not Basil method paths. `LocalIdentity.open_or_create`,
`missing=generate` and keystore generation/import are forbidden, so a missing registered key
fails readiness and signing closed.

AUTHBUS.11 v1.3 is implemented in the order `B0→B1→B2→B3→B4→B5→B6→B7→B8→B9→B10` from the
active stage matrix. B7 and federated B8/B9/B10 remain fail-closed on the stale HNL-GATE0
dependency; local loopback work may proceed. The execution-closure receipt
`OpenClaw/AUTHBUS-PLAN-AMENDMENT-RECEIPT-2026-08-26-v1.3.json` binds the exact post-state and
does not claim behavioral implementation evidence. E.41's deferred external gates and all
runtime authority flags remain unchanged.
