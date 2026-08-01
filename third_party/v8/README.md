# `rusty_v8` Consumer Artifacts

This directory wires the `v8` crate to exact-version Bazel inputs.
Bazel consumer builds use sandbox-enabled artifacts throughout:

- Codex release archives on Windows MSVC
- source-built V8 archives on Darwin, GNU Linux, musl Linux, and Windows GNU

Repository Cargo builds should run through `just cargo`, `just check`, or another
Just recipe. Those paths use `scripts/rusty-v8-cargo` to resolve the exact
sandbox archive and binding from the checked SHA-256 manifest before invoking
Cargo. CI and release jobs use the same resolver. Bazel sets the variables
independently in `MODULE.bazel` to select source-built local archives and
bindings or the exact Windows MSVC release archive.

The Bazel `v8` crate feature selection enables V8's in-process sandbox for
every supported target, including Windows MSVC.

Current pinned versions:

- Rust crate: `v8 = =150.4.0`
- Embedded upstream V8 source for Bazel-produced release builds: `15.0.245.2`

## Updating to a new `v8` release

Use this as the maintainer flow for a version bump:

1. Bump the `v8` crate version and refresh `codex-rs/Cargo.lock`.
2. Update the Bazel versioned inputs in `MODULE.bazel`, then refresh the
   matching checksum manifest and generated checksums as described below.
3. Publish matching sandbox archive, binding, and checksum assets under the
   `rusty-v8-v<crate_version>` release tag.
4. Rerun Cargo and Bazel validation against the published assets.

When changing published `rusty_v8` `http_file` inputs, keep the checked-in
checksum manifest and `MODULE.bazel` in sync, then refresh and verify the Bazel
lockfile:

```bash
just bazel-lock-update
just bazel-lock-check
```

The consumer-facing selectors are:

- `//third_party/v8:rusty_v8_archive_for_target`
- `//third_party/v8:rusty_v8_binding_for_target`

Published release assets are expected at the tag:

- `rusty-v8-v<crate_version>`

with these raw asset names:

- `librusty_v8_release_<target>.a.gz`
- `src_binding_release_<target>.rs`

During the sandbox rollout, sandbox-enabled assets are published alongside those
current assets on the same tag, with the Rust crate's sandbox feature suffix in
their raw names:

- `librusty_v8_ptrcomp_sandbox_release_<target>.a.gz`
- `rusty_v8_ptrcomp_sandbox_release_<target>.lib.gz` on Windows MSVC
- `src_binding_ptrcomp_sandbox_release_<target>.rs`

The Bazel graph exposes matching sandbox pair targets for non-MSVC platforms:

- `//third_party/v8:rusty_v8_sandbox_release_pair_x86_64_apple_darwin`
- `//third_party/v8:rusty_v8_sandbox_release_pair_aarch64_apple_darwin`
- `//third_party/v8:rusty_v8_sandbox_release_pair_x86_64_unknown_linux_gnu`
- `//third_party/v8:rusty_v8_sandbox_release_pair_aarch64_unknown_linux_gnu`
- `//third_party/v8:rusty_v8_sandbox_release_pair_x86_64_unknown_linux_musl`
- `//third_party/v8:rusty_v8_sandbox_release_pair_aarch64_unknown_linux_musl`

The Bazel graph pins the same libc++, libc++abi, and llvm-libc source revisions
used by `rusty_v8 v150.4.0`, compiles published artifact targets with
`--config=rusty-v8-upstream-libcxx`, and folds the matching runtime objects into
the final static archive so consumers can link it with the `v8` crate's default
`use_custom_libcxx` feature. The config keeps the object files and the bundled
runtime on Chromium's `std::__Cr` ABI namespace instead of mixing those objects
with the toolchain libc++ default namespace. Bazel consumers use these
source-built targets directly; Cargo release and package builds use the
published copies.

MSVC is not part of the Bazel-produced matrix yet. The repository's current
hermetic Windows C++ platform is `windows-gnullvm`/`x86_64-w64-windows-gnu`, so
it cannot truthfully reproduce upstream's `*-pc-windows-msvc` archives until we
add a real MSVC-targeting C++ toolchain to the artifact graph. Native Windows
CI consumes the exact sandbox-enabled Codex release archive instead.

Repository, release, and CI Cargo builds use `RUSTY_V8_ARCHIVE` plus a
downloaded `RUSTY_V8_SRC_BINDING_PATH` to point at those `openai/codex` release
assets directly. `scripts/rusty-v8-cargo` caches them under
`$XDG_CACHE_HOME/hepta/rusty-v8` (or `~/.cache/hepta/rusty-v8`) and validates
both files against `rusty_v8_150_4_0.sha256`. We do not use
`RUSTY_V8_MIRROR` because the upstream `v8` crate hardcodes a
`v<crate_version>` tag layout, while our artifacts are published under
`rusty-v8-v<crate_version>`.

Do not mix artifacts across crate versions. The archive and binding must match
the exact resolved `v8` crate version in `codex-rs/Cargo.lock`.
