# WEB-C1 immutable worker receipt graph

Status: **implemented as a fixture-only graph verifier; no real worker graph has been verified and launch remains unauthorized**

## Purpose

C1 now has separate source, build-input, build-manifest, artifact, reproducibility and startup contracts. A real worker must not be launched merely because each file looks valid in isolation. They must all describe the same exact source tree, build, executable and qualification candidate.

The receipt graph verifier supplies that cross-binding layer. It treats every immutable receipt or artifact as a node and every required equality as a directed proof edge.

Example graph:

```text
source receipt SHA ───────────────┐
                                 v
build manifest SHA <──── artifact receipt ────> worker executable SHA
        ^                 ^                         ^
        |                 |                         |
        └──── reproducibility receipt ─────────────┘

source.commit == pinned Servo commit
source.tree   == pinned Servo tree
```

Every edge must match. There is no partial graph, optional missing output, explained mismatch, or fallback launch path.

## Node contract

A manifest declares a sorted, unique list of bounded nodes:

- `canonical_json` — strict compact canonical JSON with an exact schema string and mandatory recursive negative-authority scan;
- `binary` — raw immutable bytes, normally the worker executable;
- `text` — strict UTF-8 without NUL for normalized metadata.

All node paths are repository/packet-relative. At verification time each file must be beneath one canonical root, regular, non-symlink, single-link, not group/world writable, and within byte bounds.

The verifier hashes every file itself. Caller-provided file hashes are never accepted as facts without a proof edge to the verifier-computed hash.

## Edge contract

Three edge types are supported:

- `pointer_equals_literal` — a JSON scalar equals an exact frozen literal, including the Servo commit/tree;
- `pointer_equals_file_sha256` — a JSON scalar equals the verifier-computed SHA-256 of another node;
- `pointers_equal` — two JSON scalar values in different nodes are byte-equal in canonical encoding.

JSON pointers use strict RFC6901-style escaping. Unknown fields, missing pointers, invalid array indices, scalar traversal and non-scalar compared values fail closed.

Each matched edge receives a domain-separated proof digest binding the canonical edge definition and both scalar values. The graph receipt records only value digests, not arbitrary secret-bearing scalar contents.

## Negative posture

Every canonical JSON node is recursively scanned. Runtime, launch, network, credential, production, effect, operator, G5, execute, promotion and release-like keys may appear only with value `false`.

The graph manifest and graph verification receipt both require the complete authority posture to be false. The runtime section additionally fixes:

```text
launch_authorized=false
worker_executed=false
servo_runtime_qualified=false
external_network_used=false
```

Therefore a graph verification is evidence of immutable cross-binding only. It is not an execution admission.

## Receipt

A `hepta.servo.worker_receipt_graph_verification.v1` binds:

- exact Servo commit/tree;
- graph manifest SHA-256;
- sorted node path/type/schema/SHA-256/byte inventory;
- all matched edge proof digests;
- exact worker node SHA-256 and byte length;
- node/edge counts and total bytes;
- negative runtime and authority posture;
- decision `RECEIPT_GRAPH_BOUND_LAUNCH_NOT_AUTHORIZED`;
- domain-separated self-binding receipt ID.

Verification reloads every node, reevaluates every edge and recomputes the complete receipt. Receipt output is create-only, mode `0600`, fsynced, and never overwritten.

## Current fixture evidence

The local self-test constructs a synthetic source receipt, build manifest, artifact receipt, reproducibility receipt and worker binary. It covers:

1. complete graph creation and exact recomputation;
2. worker byte drift rejection;
3. positive runtime-authority rejection;
4. edge mismatch rejection;
5. unsorted graph manifest rejection;
6. duplicate JSON key rejection.

This does not prove any real Servo source, build, artifact or runtime claim.

## Real C1 use

After two real worker builds exist, a reviewed graph manifest must cross-bind at least:

- canonical source receipt and source-bundle verification;
- build-input packet and worker build manifest;
- patch inventory, license packet and complete SPDX SBOM;
- both artifact receipts;
- byte-identical reproducibility receipt;
- selected worker executable and symbols;
- exact Servo commit/tree;
- exact target/profile/features/toolchain candidate.

Only after the real graph verifies may the next explicit gate construct a qualification startup descriptor. Even then, launch authority remains a separate action and all production/effect/operator/promotion/release authority remains false.
