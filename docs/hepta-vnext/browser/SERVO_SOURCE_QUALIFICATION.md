# WEB-C1.4A — pinned Servo source qualification

Status: `TOOL_IMPLEMENTED / CANONICAL_CHECKOUT_NOT_YET_RECEIPTED / BUILD_NOT_QUALIFIED`  
Plan relationship: implements the source-verification tooling for `C1-004A`; it does not import or
build Servo and does not complete `WEB-C1`.  
Authority: none.

## 1. Purpose

A commit string copied into a plan is not source provenance. Before Hepta imports Servo, the
canonical source checkout must be shown to be the exact expected Git commit and tree from the
expected repository, clean, inventory-bound, and licensed as expected. The source receipt remains
strictly weaker than an archive, build, artifact, sandbox, platform, operator, or release receipt.

Canonical tool:

```text
scripts/hepta-servo-source-receipt.py
```

Canonical receipt schema:

```text
docs/hepta-vnext/browser/hepta.servo.source_receipt.v1.schema.json
```

## 2. Fixed canonical expectation

```text
repository: https://github.com/servo/servo
commit: 0a48e298482659817eb50097df23841f2b8e3044
tree: b04d2f75b3217374d079d579c270177b57fa1389
license: MPL-2.0
```

The production CLI has no option to replace this pin. Tests call internal helpers with temporary
Git fixture commits, but fixture receipts are not accepted as canonical Servo evidence.

## 3. Required source facts

The generator independently obtains from Git:

- `HEAD` commit;
- `HEAD^{tree}`;
- `remote.origin.url`, normalized only from recognized GitHub Servo URL forms;
- complete porcelain status including untracked files;
- recursive `git ls-tree` mode, object type, object ID, and UTF-8 path;
- entry, blob, submodule, symlink, and path-byte counts;
- a domain-separated manifest digest over every recursive tree entry;
- exact `LICENSE` byte count and SHA-256;
- presence—not trust validation—of an embedded commit signature.

The generator fails on:

- wrong commit, tree, repository, or license marker;
- dirty/untracked checkout;
- malformed or duplicate tree paths;
- unknown recursive object type;
- invalid object ID or mode;
- empty tree;
- missing Git or missing license;
- output path that already exists.

## 4. Manifest digest

For each recursive Git entry, the canonical field is:

```text
mode NUL object_type NUL object_id NUL UTF-8 path
```

The receipt uses a length-framed SHA-256 domain:

```text
hepta.servo.git-tree-manifest.v1
```

Each field is preceded by an unsigned 64-bit big-endian length. This is an inventory digest over
Git tree metadata, not a replacement for the Git tree object or a source archive digest.

## 5. Receipt semantics

The compact canonical JSON receipt states only:

```text
SOURCE_PIN_AND_TREE_ONLY
SOURCE_PIN_VERIFIED_BUILD_NOT_QUALIFIED
```

It must explicitly keep:

```text
source_archive_created=false
source_archive_sha256=null
worker_artifact_built=false
worker_artifact_sha256=null
sbom_created=false
machine_authority=false
runtime_authority=false
production_caller=false
production_writer=false
effect_authority=false
external_effect=false
operator_acceptance=false
promotion=false
release_qualified=false
```

`receipt_id` is a domain-separated SHA-256 of the complete canonical receipt payload before the ID
is inserted. Verification reconstructs the current source facts and requires exact equality.

The receipt does **not** establish that the GitHub signature is valid. It records only whether an
embedded signature exists. Independent signature/key/trust verification requires a later source
trust policy and evidence.

## 6. Canonical use

On an exact local checkout:

```sh
python3 scripts/hepta-servo-source-receipt.py snapshot \
  --checkout /absolute/private/servo-checkout \
  --output /absolute/private/evidence/servo-source-receipt.json

python3 scripts/hepta-servo-source-receipt.py verify \
  --checkout /absolute/private/servo-checkout \
  --receipt /absolute/private/evidence/servo-source-receipt.json
```

The output path is create-only and mode `0600` where the platform honors Unix permissions. Formal
qualification should place it in an already private, independently inventoried evidence root and
bind the directory durability separately.

## 7. Fixture qualification

Repository CI creates a small temporary Git repository with:

- normalized Servo origin URL;
- MPL-2.0 marker;
- multiple nested source files;
- an exact fixture commit/tree;
- no third-party Python dependencies.

Tests prove:

1. exact clean commit/tree/origin/license collection;
2. stable tree-manifest digest shape;
3. compact canonical receipt generation and verification;
4. dirty/untracked checkout rejection;
5. wrong commit/tree rejection;
6. unexpected origin rejection;
7. pretty/noncanonical JSON rejection;
8. positive authority tamper rejection;
9. receipt-ID payload tamper rejection.

A fixture PASS qualifies the tool contract only. It cannot create a canonical Servo receipt because
the fixture commit/tree differ from the fixed pin.

## 8. Remaining source gates

Before marking C1-004A complete:

- run the tool against a newly fetched exact Servo checkout;
- independently record fetch URL, DNS/TLS/repository context, and Git object transfer method;
- verify the embedded commit signature against a pinned trust policy or record it as untrusted;
- freeze the full canonical receipt and its SHA-256 in Hepta evidence;
- create a deterministic source archive and archive inventory;
- verify symlink/submodule policy for the import topology;
- produce MPL-2.0 notices and source-offer/distribution obligations;
- create a patch inventory, initially empty, with review and deletion rules;
- repeat verification from a second independently fetched checkout.

## 9. Next source/build sequence

```text
C1-004A-1 canonical checkout source receipt
C1-004A-2 second-fetch comparison
C1-004A-3 deterministic source archive + archive SHA-256
C1-004A-4 license/notice/source-distribution packet
C1-004A-5 empty patch inventory and import topology
C1-004B   toolchain/native lock + worker artifact manifest
C1-004C   minimal inherited-channel Servo worker
```

No step enables a browser caller, external network, credential, effect, operator, G5, promotion, or
release authority.
