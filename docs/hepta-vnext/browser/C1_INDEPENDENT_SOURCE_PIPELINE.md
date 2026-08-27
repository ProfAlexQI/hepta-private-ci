# WEB-C1 independent Servo source pipeline

Status: **implemented tooling; exact Servo run and receipt are not yet recorded**  
Stage: `WEB-C1 / C1-004A`  
Authority: `source-qualification-only`

## Purpose

The pipeline converts the pinned Servo commit from a reference into a reproducible, independently acquired source bundle. It deliberately stops before compilation or execution.

The canonical input remains:

```text
repository = servo/servo
commit     = 0a48e298482659817eb50097df23841f2b8e3044
tree       = b04d2f75b3217374d079d579c270177b57fa1389
license    = MPL-2.0
```

## Acquisition algorithm

A canonical run performs two fresh Git initializations in different canonical absolute directories. Each fetch uses the pinned HTTPS origin, protocol v2, `--no-tags`, `--depth=1`, and `--filter=blob:none`, then detaches at the exact commit.

For each checkout the tool proves:

- exact `HEAD` and `HEAD^{tree}`;
- SHA-1 Git object format;
- empty porcelain status including untracked files;
- no alternate object database;
- a standalone `.git/objects` directory;
- strictly sorted, safe UTF-8 recursive `ls-tree` paths;
- path, blob, symlink and submodule counts;
- a domain-separated tree-manifest digest;
- exact MPL-2.0 license bytes and digest;
- no machine-local path in any receipt.

The two roots and their object stores must be different filesystem objects. The acquisition nonce digests must differ. Their source projections must be byte-for-byte identical.

## Deterministic distribution archive

Each independent checkout produces:

```text
git archive --format=tar --prefix=servo-<commit>/ <commit>
```

The tar is inspected before acceptance. Absolute paths, `..`, duplicate names, hard links, device entries, FIFOs, unsafe symlink targets, backslashes and paths outside the single prefix are rejected.

The tar is then compressed using gzip level 9, empty filename and `mtime=0`. Both independent checkouts must produce identical tar metadata, tar SHA-256 and gzip SHA-256. One copy is retained; the duplicate is deleted after comparison.

## License and patch binding

The source bundle receipt binds:

- the exact MPL-2.0 LICENSE bytes;
- the canonical patch-inventory digest and patch count;
- the requirement to distribute corresponding source;
- the two acquisition-receipt digests;
- tar and gzip digests and sizes.

It does not claim that the patch series was applied, that Servo was built, or that an artifact is runnable.

## Canonical execution

The network-heavy qualification is manual only:

```text
workflow_dispatch
acknowledge_source_only = SOURCE_ONLY_NO_RUNTIME_AUTHORITY
```

The workflow uploads the compressed archive, the two fetch receipts, the license packet and the bundle receipt. It records acquisition network use separately from browser runtime authority.

## Negative claims

Even after a successful exact-source run, all of these remain false:

```text
Servo built
Servo linked
browser runtime qualified
runtime external network
production caller/writer
effect authority
operator acceptance
promotion
release qualification
```

WEB-C1 cannot advance to a real worker until the canonical source receipt is independently reviewed and the build manifest, SBOM, artifact digest and platform-private transport bind the same source tree.
