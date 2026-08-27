# WEB-C1 pre-build stop conditions

A Servo build runner must stop before invoking Cargo when any of these conditions is true:

- the exact source pointer is absent, unreviewed, expired or refers to another Hepta commit/tree;
- the offline verification receipt does not recompute the pinned Servo tree;
- the compressed source archive, source receipt, license packet or patch inventory digest differs;
- the build recipe is not compact canonical JSON or contains unknown fields;
- target, profile, package, manifest, artifact path, jobs or feature ordering differs from the sealed manifest;
- rustc, cargo or linker version/digest differs from the sealed toolchain;
- inherited proxy, credential, HOME, Git authentication, Rust flags or non-allowlisted environment is present;
- Cargo is not simultaneously `--locked`, `--offline` and `--frozen`;
- default features are enabled or the feature set differs;
- source is writable after extraction sealing or contains symlink/hardlink escape;
- the build sandbox cannot prove network denial;
- the runner cannot record process, filesystem, network and resource evidence;
- output artifact, symbols or SBOM destinations are not private and empty;
- any authority field is true.

A stop result is evidence, not an implementation failure to be bypassed. It must record the exact failing gate and preserve the existing source/build candidate unchanged.

The first real build is allowed to produce only an unqualified artifact candidate and its evidence. It cannot launch a WebView, contact the public network, update a production caller, create operator acceptance or promote a release.
