# Hepta Browser C1 worker build-input binding

Status: **implemented as a local-fixture contract; exact Servo source bundle, real build, artifact, and runtime evidence are pending**

## Purpose

`C1-004B-1` freezes every authority-relevant input before a Servo worker build starts. The output is not a build receipt and cannot be used as evidence that a worker exists or runs. It binds one exact source receipt and one exact source-bundle verification to a deterministic command, allowlisted environment, toolchain descriptions, patch packet, license packet, and SPDX-2.3 SBOM bytes.

The canonical entrypoint and implementation are:

```text
scripts/hepta-servo-worker-build-inputs.py
scripts/hepta-servo-worker-build-manifest.py
scripts/tests/test_hepta_servo_worker_build_manifest.py
scripts/tests/test_hepta_servo_worker_build_policy.py
scripts/verify-hepta-servo-worker-build-inputs-contract.py
docs/hepta-vnext/browser/hepta.servo.worker_build_input_packet.v1.schema.json
```

`hepta-servo-worker-build-inputs.py` is the required entrypoint. It narrows command and environment semantics before delegating canonical serialization and independent recomputation to the manifest engine.

## Inputs

The `create` and `verify` commands require:

- canonical `hepta.servo.source_receipt.v1` bytes;
- canonical `hepta.servo.source_bundle_verification.v1` bytes;
- canonical patch inventory and MPL license packet;
- canonical SPDX-2.3 JSON SBOM;
- captured `rustc -vV`, `cargo -V`, and linker identity files;
- compact canonical JSON build command;
- compact canonical JSON allowlisted build environment;
- exact target triple, profile, and sorted unique Cargo features.

The build command is data, not a shell fragment. It must invoke `cargo build` or `cargo rustc` directly and include both `--locked` and `--offline`. Registry/acquisition operations, non-Cargo executors, absolute paths, parent traversal, NUL, CR/LF ambiguity, positive network posture, duplicate features, undeclared features, secret-bearing environment keys, and unknown fields fail closed. Environment values are never copied into the packet; only key, UTF-8 byte length, and digest are retained.

## Outputs

The tool writes two create-only `0600` canonical JSON files:

1. `hepta.servo.worker_build_input_packet.v1`, which binds all supporting bytes and their semantic fields;
2. `hepta.servo.worker_build_manifest.v1`, which is the exact manifest consumed by the existing artifact-receipt validator.

The packet is self-bound by:

```text
servo-worker-build-inputs:v1:<domain-separated-sha256>
```

`verify` independently reloads all supporting inputs and recomputes both outputs byte-for-byte. Reusing an existing output path is rejected.

## Fixed negative posture

Both outputs keep the following false:

```text
worker_artifact_built
runtime_qualified
reproducibility_qualified
network_access_during_build
worker_tcp_listener
worker_http_surface
worker_external_network
worker_credential_export
worker_production_authority
worker_effect_authority
machine/runtime/product/effect/operator/promotion/release authority
```

A passing fixture test means only that the input-freezing contract is deterministic and fail-closed. It does not mean the canonical Servo source bundle exists, a dependency set was acquired, a build completed, an SBOM is complete, or a worker is executable.

## Qualification cases

The engine suite covers:

- exact create/verify recomputation;
- create-only output behavior;
- tampered environment rejection;
- unknown environment key rejection;
- positive build-network rejection;
- noncanonical supporting JSON rejection;
- absolute-path build command rejection.

The strict entrypoint suite additionally covers:

- direct locked/offline Cargo build acceptance;
- non-Cargo executor rejection;
- registry/acquisition command rejection;
- missing `--locked` rejection;
- missing `--offline` rejection;
- duplicate feature rejection;
- newline-bearing command rejection;
- secret or multiline environment rejection.

## Exact next use

After two independent exact Servo fetches and the source-bundle receipt pass, capture build inputs into files, run the strict canonical entrypoint `create`, independently run `verify`, then use the resulting manifest as an immutable input to the first Linux local-fixture-only worker build. Any later change to source, command, feature set, environment, toolchain, patch packet, license packet, or SBOM requires a new packet and manifest.
