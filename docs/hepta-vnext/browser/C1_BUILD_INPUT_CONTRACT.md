# WEB-C1 Servo build-input sealing contract

Status: **implemented tooling; no accepted real recipe and no build receipt**  
Stage: `WEB-C1 / C1-004B-1`  
Authority: `build-input-only`

## Purpose

A Servo build is not authorized merely because a source archive exists. Before a runner may invoke a compiler, one canonical manifest must bind the independently verified source tree to one exact target, profile, package, feature set, command, environment and toolchain.

The sealer is:

```text
scripts/hepta-servo-build-input-seal.py
```

It accepts only:

1. a compact canonical `hepta.browser.servo_source_bundle_verification.v1` receipt whose archive bytes independently reconstruct the pinned Servo tree; and
2. a compact canonical `hepta.browser.servo_worker_build_recipe.v1` recipe.

It emits:

```text
hepta.browser.servo_build_input_manifest.v1
```

The sealer never extracts source, invokes Cargo, runs a build script or executes an artifact.

## Frozen source boundary

The source verification must bind:

```text
repository      = servo/servo
commit          = 0a48e298482659817eb50097df23841f2b8e3044
tree            = b04d2f75b3217374d079d579c270177b57fa1389
recomputed_tree = b04d2f75b3217374d079d579c270177b57fa1389
```

It must prove canonical JSON, safe archive paths, a single deterministic gzip member, independent acquisition nonces, Git tree reconstruction and matching MPL-2.0 license bytes. Any source receipt with enabled runtime/release authority is rejected.

## Build recipe

A recipe binds exactly one of the initial targets:

```text
x86_64-unknown-linux-gnu
aarch64-apple-darwin
x86_64-pc-windows-msvc
```

The initial profiles are `release` and `profiling`. Paths are source-relative POSIX paths; absolute paths, backslashes, `.` and `..` are rejected.

Features must be unique and strictly sorted. Default features are always disabled. The recipe supplies only a direct `cargo build` command prefix and must contain:

```text
--locked
--offline
--frozen
```

The sealer owns and appends:

```text
--manifest-path
--package
--target
--profile
--jobs
--no-default-features
--features <sorted-comma-list>   # only when non-empty
```

This prevents a recipe from smuggling conflicting target, manifest, profile or feature flags.

## Environment

The complete environment visible to the future build wrapper is represented by this frozen allowlist:

```text
CARGO_NET_OFFLINE=true
GIT_CONFIG_NOSYSTEM=1
GIT_TERMINAL_PROMPT=0
LANG=C
LC_ALL=C
SOURCE_DATE_EPOCH=0
TZ=UTC
```

`HOME`, inherited `PATH`, proxy variables, credentials, Git authentication, Rust flags and arbitrary build-script variables are not part of the manifest. A future build runner must create a minimal environment, resolve the already verified toolchain binaries before clearing ambient state, and execute the canonical command as an argv array rather than through a shell.

## Toolchain binding

The recipe records:

- exact rustc version and commit hash;
- rustc binary SHA-256;
- exact cargo version and binary SHA-256;
- host and build target;
- linker kind/version and binary SHA-256.

The manifest contains no machine-local binary paths. The later build receipt must prove that the binaries actually executed match these digests.

## Negative authority

A sealed build-input manifest proves only that inputs are syntactically and cryptographically bound. It fixes all of the following to false:

```text
build_run
artifact_created
SBOM_created
Servo runtime qualified
runtime/effect authority
production caller/writer
runtime external network
operator acceptance
promotion
release qualification
```

The first real build cannot begin until a separately reviewed recipe and toolchain receipt bind the accepted canonical source bundle. A successful build must create a successor artifact receipt; it cannot mutate this input manifest in place.
