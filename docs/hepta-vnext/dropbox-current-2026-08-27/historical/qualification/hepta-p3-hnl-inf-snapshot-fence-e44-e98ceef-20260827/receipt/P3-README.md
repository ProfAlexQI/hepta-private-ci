# HNL-0 / INF-0 contract lane

This directory is the first, qualification-only contract slice for the HNL
(network/node-link) and INF (local inference) plans. It contains a deliberately
small backend-neutral envelope projection, deterministic positive/negative
vectors, source-binding metadata, and an offline verifier only. It does **not**
add a runtime crate, start a listener, open a route, load a model, touch an NPU,
or change any authority flag.

`hnl-wire-v1.schema.json` and `inf-wire-v1.schema.json` are qualification
projections, not a claim that canonical CBOR/COSE or the native ABI has been
frozen. The HNL signature fields are explicitly marked
`synthetic_fixture_not_attestation`; no private key, signer, network, model, or
hardware evidence is present here.

The vectors intentionally exercise the fields that INF-0/HNL-0 require before
loopback work: peer/tenant/workspace binding, ALPN/domain separation,
transcript/channel digests, epoch/TTL/replay metadata, the complete inference
admission tuple (backend ABI, compiler/build flags, device/thermal/power
profile, quantization, context/batch, cache policy, shape and supply-chain
digests), generation fencing, and explicit negative authority flags.
`expected_sha256` and the HNL body/INF tuple digests are computed over sorted-key,
compact UTF-8 JSON and checked on every run. Four negative mutations must be
rejected by the same fail-closed validator.

The lane is valid only when its source binding is rechecked against the current
Dropbox plan/index and the exact commit recorded in the generated P3 receipt.
`P3-SOURCE-BINDING.json` records the local Dropbox bytes and the stale external
pointer observed for this qualification run. Any parent digest, HEAD/tree,
dirty state, schema, or artifact drift makes the receipt
`STALE_SOURCE_BINDING`; the tracked binding file is evidence of that state, not
an authority grant.

Run the verifier through the SSD entrypoint from the worktree root:

```sh
./qualification/hnl-inf-0/verify_golden_vectors.sh
```

The separate HNL-1/INF-1 qualification fixture exercises a deterministic
request/response path without creating a filesystem socket or a listener:

```sh
./qualification/hnl-inf-0/verify_uds_loopback.sh
```

`uds_loopback.py` uses an anonymous `AF_UNIX` `socketpair` and a bounded
big-endian-u32 length prefix around the sorted-key JSON projection.  Its state
machine is deliberately small and local: it checks frame and payload digests,
monotonic sequence numbers, session/generation fences, synthetic peer
credentials, tenant/workspace/Agent ACLs, nonce/request replay, cancellation,
delegation/revocation, and snapshot/restart terminal-receipt replay.  The
`uds-loopback-vectors.json` corpus records the positive exchanges and the
sequence/epoch/credential/authority/generation/digest mutations that must be
rejected.  A successful exchange still returns a qualification receipt only;
the INF fixture remains `NotAdmitted`, and no model, NPU, route, listener,
external effect, or production authority is touched.

Snapshots carry a `state_digest` over the canonical state projection.  Reopen
is fail-closed when the digest, state key set, record sequence/cardinality, or
cached record-to-terminal-receipt bindings do not agree.  This protects the
qualification crash/restart fixture from accepting a partially written or
mispaired terminal receipt; it is local integrity evidence, not an external
signature or trust-root attestation.

This is not the HNL-1/INF-1 production implementation.  In particular, the
credential is a deterministic fixture rather than an OS peer-credential or
audit-token attestation, JSON is not the eventual canonical CBOR/COSE security
encoding, state snapshots are in-memory test evidence, and no native backend,
shared-memory descriptor, fuzz target, or external signer is introduced.  Any
such work requires a separate review and feature-gated lane.

The next stage (HNL-1/INF-1) must be a separate review and feature-gated UDS
loopback implementation. These files alone do not satisfy that stage. In
particular, `NotAdmitted` in the INF fixture is intentional: no measured or
signed model tuple exists yet, and no fallback is allowed.
