# Hepta runtime-grant bootstrap contract V1

**Status:** normative development contract; source work does not grant production authority.  
**Delivery block:** P0.7a.  
**Owners:** Supervisor trust plane, Agentd composition root, authority-contract crate.

## 1. Problem statement

The current Agentd composition creates a closed local `AuthorityGrant` from the
selected build profile. That is correct for local development, but it cannot be
used as evidence that an externally governed release, lifecycle owner and exact
process generation were authorized. P0.7a introduces a separately signed,
bounded bootstrap object. It does not widen any local profile and it does not
allow Agentd to select its own production actions.

The bootstrap is a **start-time identity and authority binding**, not an
operation capability. Every physical model, provider, tool, network or external
effect still requires a typed capability and current per-use verification.

## 2. Canonical envelope

The only active envelope schema is `hepta.runtime-authority-bootstrap.v1`.
Unknown fields, duplicate JSON keys, non-canonical digest text and unsupported
schema versions fail closed.

```text
schema_version
bootstrap_id
subject_agent_id
release_id
source_commit
source_tree
binary_sha256
runtime_profile
runtime_profile_sha256
authority_grant_sha256
product_graph_sha256
authority_epoch
owner_epoch
generation
fencing_token_sha256
signer_key_id
signer_epoch
issued_at_unix_seconds
not_before_unix_seconds
expires_at_unix_seconds
nonce_sha256
```

The signed message is a length-framed canonical byte sequence, not ambient JSON
serialization. It includes the namespace, schema version and every field above
in the listed order. The detached signature object contains the signature
algorithm, signer key ID, signer epoch, envelope digest and signature bytes.

## 3. Required bindings

The Agentd consumer must compare every bootstrap field to independently loaded
local facts before product composition:

| Bootstrap field | Independent local source |
|---|---|
| `subject_agent_id` | fleet registry record and process environment |
| `release_id` | selected immutable fleet release |
| `source_commit` / `source_tree` | release manifest provenance |
| `binary_sha256` | digest of the executable opened by the supervisor |
| `runtime_profile` / digest | compiled profile contract and generated digest |
| `authority_grant_sha256` | locally reconstructed closed-world grant |
| `product_graph_sha256` | locally reconstructed ProductGraph |
| authority and owner epochs | fleet release and lifecycle records |
| `generation` | process spawn generation |
| fence digest | deterministic release/lifecycle/resource fence |
| signer key and epoch | pinned trust-root registry |
| validity window | monotonicized trusted clock observation |
| nonce | durable single-use bootstrap claim store |

A signature over internally inconsistent fields is invalid. A valid signature
cannot override local identity, release, profile, graph or generation facts.

## 4. Issuance boundary

Only the Supervisor trust plane may issue the bootstrap. Issuance requires:

1. an immutable release manifest whose binary and source identities verify;
2. a fleet record in `Starting` for the exact next generation;
3. a closed runtime profile with no unapproved dangerous action;
4. an authority epoch derived from the selected release lineage;
5. an owner epoch derived from the current Supervisor lifecycle owner;
6. a fresh random nonce whose digest is reserved durably before handoff;
7. a signer key selected by pinned key ID and monotonically increasing epoch;
8. a bounded validity interval no longer than the configured maximum.

The issuer owns no session, turn, model, provider, tool or external-effect
execution. It signs only the bootstrap identity. The signing key must not be
available to Agentd.

## 5. Transport

Allowed transport modes are intentionally narrow:

### 5.1 Inherited read-only descriptor

Preferred. The Supervisor creates a sealed regular file or anonymous descriptor,
writes the bounded envelope, fsyncs it, rewinds it, removes write access, and
passes the descriptor number through a dedicated environment variable. Agentd
must reject descriptors that are writable, seek-unstable, oversized or not a
regular sealed source.

### 5.2 Owner-only bootstrap file

Fallback for platforms without a suitable inherited descriptor. The file must:

- be beneath the exact Agent run root;
- be opened with no-follow semantics;
- be a single-linked regular file;
- have mode `0600` or stricter;
- be owned by the Agent runtime UID;
- have a bounded size;
- produce identical metadata and content digest before and after read;
- be removed or atomically consumed after a successful durable nonce claim.

Environment variables containing the full envelope or signature are forbidden.
A caller-supplied arbitrary filesystem path is forbidden.

## 6. Verification and consumption order

Agentd performs the following order without side effects before composition:

```text
read bounded transport
→ strict decode
→ canonical envelope digest
→ pinned signer/key-epoch verification
→ time-window verification
→ local Agent/release/source/binary/profile/grant/graph/epoch/generation/fence comparison
→ durable nonce compare-and-claim
→ construct RuntimeAuthorityContext
→ compose services
```

No socket, database migration, App Server, model provider, ingress adapter or
background task may start before the nonce claim and complete binding check.
A failed check publishes no successful claim. A crash after nonce claim but
before runtime readiness leaves a consumed bootstrap; the Supervisor must issue
a new nonce for a new generation rather than replaying the previous bootstrap.

## 7. Failure matrix

P0.7a qualification must include at least:

- wrong Agent ID;
- wrong release ID, source commit/tree or binary digest;
- wrong profile or profile digest;
- wrong grant or ProductGraph digest;
- stale authority epoch, owner epoch, generation or fence;
- unknown signer key, stale signer epoch or invalid signature;
- not-yet-valid and expired envelope;
- duplicate key, unknown field, oversized frame and non-canonical digest;
- writable descriptor, symlink, hardlink, wrong owner or wrong mode;
- nonce replay before and after process crash;
- release or lifecycle drift between issue and consume;
- successful bootstrap followed by per-use capability revocation.

## 8. Exit criteria

P0.7a is `source_implemented` only when the typed contract, Supervisor issuer,
Agentd consumer, durable nonce claim, transport checks and negative tests are in
the product source and required workflow. It is `qualified_exact` only after a
real runner executes non-empty successful tests against the exact source head
and merge candidate.

P0.7a never self-issues operator acceptance, promotion or release.
