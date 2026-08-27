# WEB-C1 worker build reproducibility contract

Status: **implemented as a fixture-qualified, byte-identical comparison contract; no real Servo worker builds have been compared**

## Purpose

C1-004B-4 requires two independent worker builds to be compared before an artifact may cross into the real startup path. This contract refuses to treat unexplained differences as acceptable evidence.

The comparator consumes:

- two distinct canonical replica roots;
- one compact-canonical reproducibility manifest;
- exact Servo commit/tree;
- exact worker build-manifest SHA-256;
- an explicit sorted list of output files and their semantic type.

It produces a receipt only when every declared output is byte-for-byte identical.

## Compared output types

- `binary` — raw bytes, normally the worker executable or symbol/debug packet;
- `canonical_json` — exact compact canonical JSON, including artifact receipt and SPDX SBOM;
- `text` — strict UTF-8 without NUL, including normalized build/toolchain summaries.

Every output must be:

- beneath its replica root through a canonical non-symlink path;
- a regular file;
- single-link;
- not group/world writable;
- within per-file and total byte bounds.

For canonical JSON outputs, the comparator also recursively rejects positive runtime, network, credential, product, effect, operator, promotion or release claims.

## Strict comparison policy

The manifest fixes:

```text
require_byte_identical=true
allow_missing_optional=false
allow_explained_differences=false
```

The current v1 contract has no “close enough,” normalization, ignored timestamp, section-stripping, or platform-specific exception path. A single byte difference causes `FAIL_CLOSED` and produces no receipt.

This deliberate strictness makes the first result easy to interpret. A future controlled-difference schema would require a successor version, explicit field-level normalization rules, independent review, and its own threat model; it cannot be smuggled into v1.

## Receipt binding

A valid `hepta.servo.worker_reproducibility_receipt.v1` binds:

- exact Servo commit and tree;
- target triple;
- build-manifest SHA-256;
- reproducibility-manifest SHA-256;
- distinct replica IDs;
- sorted file path/type/SHA-256/byte inventory;
- per-replica aggregate digest, file count, and total bytes;
- exact byte equality decision;
- explicit `worker_executed=false` and `servo_runtime_qualified=false`;
- all authority fields false;
- a domain-separated self-binding receipt ID.

The verifier recomputes both replica inventories and the complete receipt. Receipts are create-only and written with mode `0600`.

## What a PASS means

A fixture or real comparison PASS means only:

> The declared output bytes from two distinct replica roots are identical under the exact manifest.

It does not prove:

- the source was canonical unless the linked source receipts separately prove it;
- the build command was correct unless the build manifest separately proves it;
- the SBOM is complete unless the SBOM qualification separately proves it;
- the executable is safe, sandboxed, listener-free, or egress-free;
- the worker is Servo;
- the runtime functions correctly;
- operator acceptance, promotion, or release.

## Current evidence

The standard-library tool, static verifier, schemas, local self-test and unit tests are implemented. The local fixture suite covers:

1. exact receipt recomputation;
2. create-only output;
3. binary drift rejection;
4. positive runtime posture rejection inside canonical JSON;
5. unsorted manifest rejection.

No canonical Servo source bundle, real worker executable, real SPDX SBOM, second independent worker build, or real reproducibility receipt exists yet. GitHub exact-head workflow evidence remains pending while the repository runner fails before steps.

## Next use

After two independent C1-004B worker-build replicas exist, the build workflow must place the same declared output layout under separate roots and run:

```text
create -> independent verify -> artifact receipt cross-check -> startup bridge
```

If the first real builds differ, C1 remains blocked. The difference must be investigated at the toolchain, linker, archive, symbols, generated-resource, environment, or source-date boundary; this v1 contract will not convert a mismatch into success.
