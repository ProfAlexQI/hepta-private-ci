# WEB-C1 immutable receipt-graph manifest assembler

Status: **implemented as a fixture-only packet assembler; no real packet has been assembled and launch remains unauthorized**

## Purpose

The strict receipt-graph verifier deliberately accepts only an explicit graph manifest. Manually authoring that manifest for a real worker packet would be error-prone: a path, array index, schema, or hash edge could point at the wrong source/build/artifact/reproducibility file.

The assembler converts one canonical packet root into the unique v1 graph shape expected by the strict verifier. It is an independent program from the graph verifier. The assembler checks obvious cross-binding errors before producing a manifest; the strict graph verifier must still reopen every file, recalculate every digest, reevaluate every JSON pointer edge, prove graph completeness, and seal the verification receipt.

## Inputs

The caller supplies one canonical absolute packet root and five unique packet-relative paths:

```text
source receipt
worker build manifest
worker artifact receipt
worker reproducibility receipt
worker executable
```

The assembler performs no source fetch, build, network access, process launch, executable loading, or worker execution.

Every path must:

- be relative and platform-unambiguous;
- stay beneath the packet root;
- contain no symlink component;
- name a regular, single-link file;
- not be group/world writable;
- stay inside byte limits.

All four JSON files must be compact canonical JSON with unique keys and exact schema identifiers. They are recursively scanned for positive runtime, launch, network, credential, production, effect, operator, G5, execute, promotion, or release posture.

## Required pre-assembly bindings

Before emitting any manifest, the assembler proves:

1. the source receipt contains the exact pinned Servo commit and tree;
2. the artifact receipt's `source_receipt_sha256` equals the actual source-receipt bytes;
3. the artifact receipt's `build_manifest_sha256` equals the actual build-manifest bytes;
4. the artifact receipt's worker SHA-256 equals the actual worker bytes;
5. the reproducibility receipt binds the same Servo commit/tree;
6. the reproducibility receipt binds the same build-manifest SHA-256;
7. exactly one reproducibility output matches the supplied worker path, type, SHA-256, and byte count.

A mismatch, duplicate worker output, duplicate JSON key, noncanonical JSON, unsafe file, or positive authority produces `FAIL_CLOSED` and no manifest.

## Deterministic output

The assembler emits exactly five sorted nodes:

```text
artifact
build
reproducibility
source
worker
```

and exactly seven sorted semantic edges:

```text
artifact-build
artifact-source
artifact-worker
repro-build
repro-worker
source-commit
source-tree
```

The reproducibility worker edge uses the exact array index of the unique matched worker output. Policy is fixed to:

```text
require_all_edges=true
allow_unknown_nodes=false
allow_unknown_edges=false
launch_authorized=false
runtime_qualified=false
```

All thirteen authority fields are false. Manifest output is create-only, mode `0600`, fsynced, and never overwritten.

## Separation of duties

A manifest assembler PASS means only:

> One packet has the expected files and obvious SHA/source relationships, and a deterministic strict graph manifest was created.

It does not mean:

- the source receipt itself is independently valid;
- the build was correct or reproducible;
- the artifact or SBOM is complete;
- the graph is verified;
- the worker may launch;
- the worker is Servo or safe;
- runtime, operator acceptance, promotion, or release is qualified.

The next mandatory command is the independent strict graph verifier. Only its create-only verification receipt may be consumed by the future graph-bound startup descriptor, which still must keep launch authorization false until a separate qualification admission.

## Fixture coverage

The local fixture suite covers:

1. manifest creation and exact recomputation;
2. create-only manifest output;
3. artifact-to-worker SHA drift rejection;
4. duplicate matching worker outputs rejection;
5. positive authority rejection;
6. duplicate JSON key rejection.

No canonical Servo packet has been assembled. `real_packets_assembled=0` remains release-blocking.
