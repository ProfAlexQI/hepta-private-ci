# WEB-C1.4B — Servo worker artifact binding contract

Status: `TOOL_IMPLEMENTED / SYNTHETIC_HEADER_FIXTURES_PENDING / REAL_ARTIFACT_NOT_BUILT`  
Plan relationship: implements the tool and schemas for `C1-004B`; it does not build, execute, or
qualify a real Servo worker.  
Authority: none.

## 1. Purpose

A source receipt does not identify a binary, and a binary SHA-256 alone does not identify its
source, toolchain, feature set, patches, licenses, or SBOM. The artifact binding receipt closes only
that bookkeeping gap. It deliberately leaves every runtime, listener, egress, sandbox, platform,
operator, promotion, and release claim false.

Canonical generator:

```text
scripts/hepta-servo-artifact-receipt.py
```

Canonical contracts:

```text
docs/hepta-vnext/browser/hepta.servo.worker_build_manifest.v1.schema.json
docs/hepta-vnext/browser/hepta.servo.patch_inventory.v1.schema.json
docs/hepta-vnext/browser/hepta.servo.license_packet.v1.schema.json
docs/hepta-vnext/browser/hepta.servo.worker_artifact_receipt.v1.schema.json
```

## 2. Exact input set

The generator requires all of these as existing immutable inputs:

1. compact canonical `hepta.servo.source_receipt.v1` for the fixed Servo pin;
2. compact canonical worker build manifest;
3. one non-symlink, single-hardlink worker executable;
4. compact canonical patch inventory;
5. compact canonical license/source-distribution packet;
6. compact canonical SPDX-2.3 JSON SBOM;
7. create-only output path.

Every supporting input is SHA-256-bound. The build manifest must contain the same hashes; a changed
byte, newline, ordering, or file replacement fails closed.

## 3. Build manifest

The build manifest freezes:

```text
source_receipt_id
source_receipt_sha256
target_triple
build_profile
rustc_verbose_sha256
cargo_version
linker_id
sorted unique Cargo/features list
build_command_sha256
environment_allowlist_sha256
patch_inventory_sha256
license_packet_sha256
sbom_sha256
sbom_format
```

The following must be literal false:

```text
network_access_during_build
worker_tcp_listener
worker_http_surface
worker_external_network
worker_credential_export
worker_production_authority
worker_effect_authority
```

These fields are build inputs and assertions, not runtime proof. Runtime listener/egress/sandbox
qualification remains a later independent gate.

The current allowlisted targets are:

```text
x86_64-unknown-linux-gnu
aarch64-unknown-linux-gnu
x86_64-apple-darwin
aarch64-apple-darwin
x86_64-pc-windows-msvc
aarch64-pc-windows-msvc
```

Adding a target requires a successor schema and its own binary-header, sandbox, packaging, and
platform evidence.

## 4. Binary inspection

The generator never executes the artifact. It performs bounded header inspection:

- ELF: class, endianness, and `e_machine` for x86_64/aarch64;
- Mach-O: 32/64-bit magic and CPU type for x86_64/aarch64;
- PE: DOS header, bounded PE offset/signature, and machine for x86_64/aarch64.

The observed format/architecture must match the target triple. Unknown formats, machines,
truncated headers, artifacts smaller than 64 bytes, symlinks, multiple hard links, and group/world
writable artifacts fail closed.

This inspection does not validate code signing, load commands, imported libraries, runtime search
paths, entitlements, reproducibility, or malicious code. Those require separate C1 build/platform
qualification.

## 5. Patch inventory

Every downstream patch carries:

```text
id
path
sha256
reason
upstream_reference
deletion_condition
```

IDs are sorted and unique. An empty list is valid only when the build used the exact upstream tree
without modifications. The future build pipeline must independently prove that the applied patch
series and resulting source tree match this inventory; the artifact generator only binds the
provided inventory bytes.

## 6. License packet and SBOM

The license packet binds:

- exact upstream repository and commit;
- `MPL-2.0` as primary license;
- exact upstream `LICENSE` digest;
- sorted notice identifiers;
- explicit acknowledgement that source-distribution/source-offer obligations apply.

The SBOM must be compact canonical SPDX-2.3 JSON, use `CC0-1.0` as its data license, carry a UUID URN
document namespace, and contain at least one package. This is a minimum parser gate, not a claim
that package coverage is complete. A later SBOM completeness check compares packages/files against
the actual build dependency and binary inventory.

## 7. Receipt semantics

The artifact receipt claim is exactly:

```text
ARTIFACT_DIGEST_AND_BUILD_INPUTS_ONLY
ARTIFACT_BOUND_RUNTIME_NOT_QUALIFIED
```

It binds:

- source receipt ID and SHA-256;
- fixed repository/commit/tree;
- build manifest SHA-256 and selected build facts;
- artifact SHA-256, byte length, mode, format, architecture, and target;
- patch/license/SBOM hashes and counts.

The following remain false:

```text
artifact_executed
servo_webview_started
listener_scan_passed
egress_scan_passed
sandbox_qualified
platform_matrix_qualified
machine_authority
runtime_authority
production_caller
production_writer
effect_authority
external_effect
external_network_allowed
credential_export_allowed
operator_acceptance
promotion
release_qualified
```

`receipt_id` is a domain-separated SHA-256 over the complete compact canonical payload before the
ID field is inserted.

## 8. Synthetic fixture qualification

Repository CI does not build or download Servo. It creates synthetic bounded executable headers
for ELF, Mach-O, and PE and verifies the artifact-binding parser and negative claims.

Required fixture tests cover:

1. complete artifact/source/build/supporting-input binding;
2. ELF x86_64, Mach-O aarch64, and PE x86_64 header classification;
3. target/format/architecture mismatch rejection;
4. every positive build/network/authority flag rejection;
5. supporting-input byte tamper rejection;
6. group/world writable artifact rejection;
7. hardlink and symlink rejection;
8. positive/tampered source receipt rejection;
9. sorted patch inventory and source-offer acknowledgement.

A synthetic header is not an executable, Servo artifact, or runtime qualification. The workflow
must record `real_worker_artifact=false` and `artifact_executed=false`.

## 9. Real C1-004B completion gates

Before this work item can be marked complete:

- C1-004A canonical source receipt must exist and be independently verified;
- deterministic source archive and patch-applied source tree must be sealed;
- exact Rust/Cargo/linker/native toolchain evidence must exist;
- build must run from a declared environment allowlist with network disabled after dependency
  acquisition;
- a real worker artifact and symbols/debug packet must be produced;
- SPDX SBOM completeness and license packet must be independently reviewed;
- the build must be repeated independently and reproducibility differences explained;
- platform code-signing/notarization policy must be defined without confusing signing with
  operator acceptance;
- artifact receipt must be validated against its JSON schema and stored in a private durable
  evidence root.

Even then, WEB-C1 remains incomplete until the artifact starts through the inherited private
channel, creates exactly one real WebView, passes local-fixture semantic smoke, and receives
listener/egress/sandbox/platform qualification.

## 10. Next implementation sequence

```text
C1-004B-1 fixture-qualify artifact binding tool
C1-004B-2 define real build environment allowlist and command preimage
C1-004B-3 bind deterministic source archive and applied-patch tree
C1-004B-4 produce first Linux worker artifact, symbols, SBOM, and receipt
C1-004B-5 independently repeat build and compare artifacts
C1-004C   implement inherited-channel worker startup against real Servo
```

No step authorizes external browsing, credential use, production/effect behavior, operator
acceptance, G5, promotion, merge, or release.
