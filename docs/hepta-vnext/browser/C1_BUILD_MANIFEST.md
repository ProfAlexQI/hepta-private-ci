# WEB-C1 Servo worker build-input manifest

Status: **implemented contract; exact Servo source/build evidence pending**

This slice freezes `hepta.servo.worker_build_manifest.v1` before any real Servo worker build is accepted. The manifest binds one canonical source receipt, governed patch inventory, MPL packet, SPDX-2.3 JSON SBOM, Rust toolchain evidence, direct Cargo argv, bounded environment allowlist, target/profile/linker, and sorted feature set.

## Fail-closed rules

- source receipt must bind the exact pinned Servo commit/tree and remain source-only;
- source receipt authority fields must all be false;
- patch IDs are canonical, sorted, unique, and digest-bound;
- license packet binds MPL-2.0 and acknowledges source-distribution obligations;
- SBOM is SPDX-2.3 JSON with at least one package;
- build command is compact canonical JSON and invokes `cargo build` or `cargo rustc` directly;
- environment is compact canonical JSON using a fixed key allowlist; raw `PATH` and secret-, identity-, proxy-, or credential-bearing variables are rejected;
- output is create-only, mode `0600`, compact canonical JSON, and all worker/network/authority flags are fixed false.

The tool never fetches, builds, links, or executes Servo. A passing contract test proves only deterministic input binding. It does not prove a canonical source checkout, successful compilation, artifact identity, reproducibility, sandboxing, listener/egress posture, WebView behavior, operator acceptance, promotion, or release.

## Commands

```sh
python3 scripts/verify-hepta-servo-build-manifest-contract.py
python3 scripts/tests/test_hepta_servo_build_manifest.py -v
python3 scripts/hepta-servo-build-manifest.py snapshot ... --output manifest.json
python3 scripts/hepta-servo-build-manifest.py verify ... --manifest manifest.json
```

The real `C1-004B-1` evidence may be created only after the two-fetch source bundle, deterministic archive, MPL packet, and patch inventory are sealed for the exact Servo pin.
