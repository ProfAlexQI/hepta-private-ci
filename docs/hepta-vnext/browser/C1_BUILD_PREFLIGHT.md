# WEB-C1 bounded Linux build preflight

Status: **implemented tooling; no real preflight receipt and no build run**  
Stage: `WEB-C1 / C1-004B-2 entry gate`  
Authority: `pre-build-inputs-only`

The canonical entrypoint is:

```text
scripts/hepta-servo-build-preflight-v2.py
```

It performs no compilation and does not execute rustc, Cargo, the linker, build scripts or a Servo worker. It cross-validates the exact bytes immediately before a separately controlled Linux build attempt.

## Required inputs

The preflight accepts:

- the accepted source-bundle verification receipt;
- the compressed deterministic Servo source archive;
- the reviewed worker build recipe;
- the accepted toolchain receipt;
- the sealed build-input manifest v2;
- the exact rustc, Cargo and linker binary files;
- distinct empty private source and artifact directories.

All file paths are operational inputs only. The output receipt contains no machine-local paths.

## Cross-binding

The preflight requires:

```text
repository      = servo/servo
commit          = 0a48e298482659817eb50097df23841f2b8e3044
tree            = b04d2f75b3217374d079d579c270177b57fa1389
recomputed_tree = b04d2f75b3217374d079d579c270177b57fa1389
target          = x86_64-unknown-linux-gnu
```

It recomputes the SHA-256 of the source verification receipt, recipe, toolchain receipt, build-input manifest and compressed source archive. Every digest must match the sealed manifest and source verification receipt.

The recipe is independently normalized again. The direct command prefix must be exactly:

```text
cargo build --locked --offline --frozen
```

The preflight reconstructs all sealer-owned flags and requires the result to equal the build-input manifest. Default features remain disabled; explicit features are unique and sorted.

## Toolchain bytes

The exact rustc, Cargo and linker files must be executable non-symlink regular files with one hard link and no group/world write bit. Their complete bytes are rehashed immediately before the build and must match the toolchain receipt.

The v2 entrypoint additionally requires the receipt to bind exactly:

```text
rustc -vV
cargo -Vv
<linker-kind> --version
```

and the capture environment:

```text
GIT_CONFIG_NOSYSTEM=1
GIT_TERMINAL_PROMPT=0
LANG=C
LC_ALL=C
TZ=UTC
```

Version facts must be bounded printable path-free text. Control characters, absolute paths, backslashes, semicolons and unlisted linker kinds fail closed.

## Filesystem roots

The source extraction root and artifact root must:

- already exist as distinct canonical directories;
- contain no symlink component;
- be owned by the current Unix user;
- grant no group/world permission;
- be empty.

The preflight does not populate either directory.

## Pass boundary

A pass emits `hepta.browser.servo_build_preflight.v1` with status:

```text
READY_FOR_SEPARATE_BOUNDED_BUILD
```

This means only that the supplied bytes and empty roots satisfy the frozen entry conditions. The output fixes all of the following to false:

```text
build run
artifact created
SBOM created
Servo runtime qualified
runtime/effect authority
production caller/writer
runtime external network
operator acceptance
promotion
release qualification
```

A later build runner must consume the exact preflight receipt and still enforce environment clearing, source extraction verification, process/resource limits, build-network denial, output capture and source immutability.
